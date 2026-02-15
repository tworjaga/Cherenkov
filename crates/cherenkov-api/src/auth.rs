use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: Vec<String>,
}

pub struct AuthMiddleware;

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());
    
    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            
            let validation = Validation::new(Algorithm::RS256);
            let _token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(b"secret"),
                &validation,
            ).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
            
            return Ok(next.run(request).await);
        }
    }
    
    Err(axum::http::StatusCode::UNAUTHORIZED)
}

pub fn check_permission(claims: &Claims, resource: &str, action: &str) -> bool {
    let required_role = format!("{}:{}", resource, action);
    claims.roles.contains(&required_role) || claims.roles.contains(&"admin".to_string())
}
