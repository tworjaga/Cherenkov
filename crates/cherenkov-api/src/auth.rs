use axum::{
    extract::{Request, State, Extension},
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm, TokenData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest;
use tracing::{info, warn, error, debug};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub org_id: Option<String>,
    pub tier: ApiTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiTier {
    Free,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub tier: ApiTier,
    pub rate_limit: RateLimitInfo,
}

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub requests_remaining: u32,
    pub window_reset: Instant,
}

pub struct AuthService {
    jwks_cache: Arc<RwLock<JwksCache>>,
    policy_engine: Arc<PolicyEngine>,
    http_client: reqwest::Client,
    jwks_url: String,
    api_keys: Arc<RwLock<HashMap<String, ApiKeyInfo>>>,
}

struct JwksCache {
    keys: HashMap<String, DecodingKey>,
    last_updated: Instant,
    ttl: Duration,
}

struct ApiKeyInfo {
    user_id: String,
    roles: Vec<String>,
    tier: ApiTier,
    created_at: Instant,
}

pub struct PolicyEngine {
    policies: Arc<RwLock<Vec<Policy>>>,
}

#[derive(Debug, Clone)]
struct Policy {
    resource: String,
    action: String,
    conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
enum Condition {
    Role(String),
    Permission(String),
    Tier(ApiTier),
    TimeRange { start: u8, end: u8 },
}

impl AuthService {
    pub fn new(jwks_url: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            jwks_cache: Arc::new(RwLock::new(JwksCache {
                keys: HashMap::new(),
                last_updated: Instant::now() - Duration::from_secs(3600),
                ttl: Duration::from_secs(300),
            })),
            policy_engine: Arc::new(PolicyEngine::new()),
            http_client,
            jwks_url,
            api_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn authenticate(&self, token: &str) -> Result<Claims, AuthError> {
        let header = decode_header(token).map_err(|e| {
            warn!("Failed to decode JWT header: {}", e);
            AuthError::InvalidToken
        })?;
        
        let kid = header.kid.ok_or_else(|| {
            warn!("JWT header missing kid");
            AuthError::InvalidToken
        })?;
        
        let key = self.get_jwks_key(&kid).await?;
        
        let validation = Validation::new(Algorithm::RS256);
        let token_data: TokenData<Claims> = decode(token, &key, &validation)
            .map_err(|e| {
                warn!("JWT validation failed: {}", e);
                AuthError::InvalidToken
            })?;
        
        if self.is_token_revoked(&token_data.claims.sub, token_data.claims.exp).await {
            return Err(AuthError::TokenRevoked);
        }
        
        debug!("Authenticated user: {}", token_data.claims.sub);
        Ok(token_data.claims)
    }
    
    pub async fn authenticate_api_key(&self, api_key: &str) -> Result<Claims, AuthError> {
        let keys = self.api_keys.read().await;
        
        let key_info = keys.get(api_key).ok_or(AuthError::InvalidApiKey)?;
        
        Ok(Claims {
            sub: key_info.user_id.clone(),
            exp: usize::MAX,
            iat: 0,
            roles: key_info.roles.clone(),
            permissions: vec![],
            org_id: None,
            tier: key_info.tier.clone(),
        })
    }
    
    async fn get_jwks_key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        {
            let cache = self.jwks_cache.read().await;
            if let Some(key) = cache.keys.get(kid) {
                return Ok(key.clone());
            }
            if cache.last_updated.elapsed() < cache.ttl {
                return Err(AuthError::UnknownKey);
            }
        }
        
        self.refresh_jwks().await?;
        
        let cache = self.jwks_cache.read().await;
        cache.keys.get(kid).cloned().ok_or(AuthError::UnknownKey)
    }
    
    async fn refresh_jwks(&self) -> Result<(), AuthError> {
        info!("Refreshing JWKS from {}", self.jwks_url);
        
        let response = self.http_client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to fetch JWKS: {}", e);
                AuthError::JwksFetchFailed
            })?;
        
        let jwks: JwksResponse = response.json().await.map_err(|e| {
            error!("Failed to parse JWKS: {}", e);
            AuthError::JwksParseFailed
        })?;
        
        let mut cache = self.jwks_cache.write().await;
        cache.keys.clear();
        
        for key in jwks.keys {
            if key.kty == "RSA" {
                let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
                    .map_err(|_| AuthError::InvalidKey)?;
                cache.keys.insert(key.kid, decoding_key);
            }
        }
        
        cache.last_updated = Instant::now();
        info!("JWKS refreshed with {} keys", cache.keys.len());
        
        Ok(())
    }
    
    async fn is_token_revoked(&self, _sub: &str, _exp: usize) -> bool {
        false
    }
    
    pub async fn authorize(
        &self,
        claims: &Claims,
        resource: &str,
        action: &str,
        context: &RequestContext,
    ) -> Result<bool, AuthError> {
        self.policy_engine.evaluate(claims, resource, action, context).await
    }
    
    pub async fn check_rate_limit(&self, user_id: &str, tier: &ApiTier) -> RateLimitInfo {
        let limits = match tier {
            ApiTier::Free => (100, Duration::from_secs(60)),
            ApiTier::Pro => (1000, Duration::from_secs(60)),
            ApiTier::Enterprise => (10000, Duration::from_secs(60)),
        };
        
        RateLimitInfo {
            requests_remaining: limits.0,
            window_reset: Instant::now() + limits.1,
        }
    }
}

impl PolicyEngine {
    fn new() -> Self {
        let mut policies = Vec::new();
        
        policies.push(Policy {
            resource: "readings".to_string(),
            action: "read".to_string(),
            conditions: vec![Condition::Permission("readings:read".to_string())],
        });
        
        policies.push(Policy {
            resource: "readings".to_string(),
            action: "write".to_string(),
            conditions: vec![
                Condition::Permission("readings:write".to_string()),
                Condition::Tier(ApiTier::Pro),
            ],
        });
        
        policies.push(Policy {
            resource: "plume".to_string(),
            action: "simulate".to_string(),
            conditions: vec![
                Condition::Permission("plume:simulate".to_string()),
                Condition::Tier(ApiTier::Enterprise),
            ],
        });
        
        Self {
            policies: Arc::new(RwLock::new(policies)),
        }
    }
    
    async fn evaluate(
        &self,
        claims: &Claims,
        resource: &str,
        action: &str,
        _context: &RequestContext,
    ) -> Result<bool, AuthError> {
        let policies = self.policies.read().await;
        
        for policy in policies.iter() {
            if policy.resource == resource && policy.action == action {
                let allowed = policy.conditions.iter().all(|condition| match condition {
                    Condition::Role(role) => claims.roles.contains(role),
                    Condition::Permission(perm) => claims.permissions.contains(perm),
                    Condition::Tier(tier) => matches!((&claims.tier, tier), 
                        (ApiTier::Enterprise, _) | 
                        (ApiTier::Pro, ApiTier::Pro) | 
                        (ApiTier::Pro, ApiTier::Free) |
                        (ApiTier::Free, ApiTier::Free)),
                    Condition::TimeRange { start, end } => {
                        let hour = chrono::Local::now().hour() as u8;
                        hour >= *start && hour <= *end
                    }
                });
                
                return Ok(allowed);
            }
        }
        
        Ok(false)
    }
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    InvalidApiKey,
    TokenRevoked,
    UnknownKey,
    JwksFetchFailed,
    JwksParseFailed,
    InvalidKey,
    Unauthorized,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::InvalidApiKey => write!(f, "Invalid API key"),
            AuthError::TokenRevoked => write!(f, "Token has been revoked"),
            AuthError::UnknownKey => write!(f, "Unknown signing key"),
            AuthError::JwksFetchFailed => write!(f, "Failed to fetch JWKS"),
            AuthError::JwksParseFailed => write!(f, "Failed to parse JWKS"),
            AuthError::InvalidKey => write!(f, "Invalid key format"),
            AuthError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl std::error::Error for AuthError {}

pub struct RequestContext {
    pub ip: String,
    pub user_agent: Option<String>,
    pub request_path: String,
}

pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());
    
    let claims = if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            auth_service.authenticate(token).await
                .map_err(|_| StatusCode::UNAUTHORIZED)?
        } else if auth_header.starts_with("ApiKey ") {
            let key = &auth_header[7..];
            auth_service.authenticate_api_key(key).await
                .map_err(|_| StatusCode::UNAUTHORIZED)?
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    
    let rate_limit = auth_service.check_rate_limit(&claims.sub, &claims.tier).await;
    
    let context = RequestContext {
        ip: "unknown".to_string(),
        user_agent: request.headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string()),
        request_path: request.uri().path().to_string(),
    };
    
    let auth_context = AuthContext {
        user_id: claims.sub.clone(),
        roles: claims.roles.clone(),
        permissions: claims.permissions.clone(),
        tier: claims.tier.clone(),
        rate_limit,
    };
    
    request.extensions_mut().insert(auth_context);
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

pub fn require_permission(claims: &Claims, permission: &str) -> Result<(), StatusCode> {
    if claims.permissions.contains(&permission.to_string()) || claims.roles.contains(&"admin".to_string()) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
