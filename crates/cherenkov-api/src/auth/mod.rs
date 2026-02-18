use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

/// Authentication claims for JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub tier: RateLimitTier,
    pub iat: u64,
    pub exp: u64,
}

/// Rate limiting tiers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RateLimitTier {
    Anonymous,
    ApiKey,
    Premium,
}

impl RateLimitTier {
    pub fn requests_per_minute(&self) -> u64 {
        match self {
            RateLimitTier::Anonymous => 60,
            RateLimitTier::ApiKey => 600,
            RateLimitTier::Premium => 6000,
        }
    }

    #[allow(dead_code)]
    pub fn websocket_connections(&self) -> u32 {
        match self {
            RateLimitTier::Anonymous => 1,
            RateLimitTier::ApiKey => 10,
            RateLimitTier::Premium => 100,
        }
    }
}

/// API key structure
#[derive(Debug, Clone)]
pub struct ApiKey {
    #[allow(dead_code)]
    pub key: String,
    pub tier: RateLimitTier,
    #[allow(dead_code)]
    pub owner: String,
}

/// Authentication state
pub struct AuthState {
    jwt_secret: String,
    api_keys: dashmap::DashMap<String, ApiKey>,
}

impl AuthState {
    pub fn new(jwt_secret: String) -> Self {
        let api_keys = dashmap::DashMap::new();
        
        // Add default API keys for testing
        api_keys.insert(
            "test-api-key-001".to_string(),
            ApiKey {
                key: "test-api-key-001".to_string(),
                tier: RateLimitTier::ApiKey,
                owner: "test@example.com".to_string(),
            },
        );

        Self {
            jwt_secret,
            api_keys,
        }
    }

    /// Validate API key
    pub fn validate_api_key(&self, key: &str) -> Option<RateLimitTier> {
        self.api_keys
            .get(key)
            .map(|k| k.tier)
    }

    /// Generate JWT token
    #[allow(dead_code)]
    pub fn generate_token(&self, user_id: &str, tier: RateLimitTier) -> anyhow::Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            tier,
            iat: now,
            exp: now + 86400, // 24 hours
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> anyhow::Result<Claims> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}

/// Extract authentication from request headers
pub async fn auth_middleware(
    State(state): State<Arc<AuthState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check for Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let tier = if let Some(auth) = auth_header {
        if auth.starts_with("Bearer ") {
            // JWT token
            let token = &auth[7..];
            match state.validate_token(token) {
                Ok(claims) => {
                    debug!("Authenticated user: {}", claims.sub);
                    claims.tier
                }
                Err(e) => {
                    warn!("Invalid JWT token: {}", e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        } else if auth.starts_with("ApiKey ") {
            // API key
            let key = &auth[7..];
            match state.validate_api_key(key) {
                Some(tier) => {
                    debug!("Authenticated with API key");
                    tier
                }
                None => {
                    warn!("Invalid API key");
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        } else {
            RateLimitTier::Anonymous
        }
    } else {
        RateLimitTier::Anonymous
    };

    // Add tier to request extensions for rate limiting
    request.extensions_mut().insert(tier);

    Ok(next.run(request).await)
}

/// Extract tier from request extensions
pub fn get_tier_from_request(request: &Request) -> RateLimitTier {
    request
        .extensions()
        .get::<RateLimitTier>()
        .copied()
        .unwrap_or(RateLimitTier::Anonymous)
}
