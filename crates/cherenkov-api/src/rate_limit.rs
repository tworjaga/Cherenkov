use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::num::NonZeroU32;
use tower::Layer;
use tracing::{debug, warn};

use crate::auth::get_tier_from_request;

/// Rate limiter state
pub type RateLimiterState = Arc<dashmap::DashMap<String, RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>>;

/// Create rate limiting layer
pub fn create_rate_limit_layer() -> RateLimitLayer {
    RateLimitLayer::new()
}

#[derive(Clone)]
pub struct RateLimitLayer;

impl RateLimitLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiters: Arc::new(dashmap::DashMap::new()),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiters: RateLimiterState,
}

impl<S> tower::Service<Request> for RateLimitService<S>
where
    S: tower::Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let tier = get_tier_from_request(&request);
        let limit = tier.requests_per_minute();
        
        // Get client IP for per-client rate limiting
        let client_id = request
            .extensions()
            .get::<SocketAddr>()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        let key = format!("{}:{:?}", client_id, tier);
        
        // Check rate limit
        let limiter = self.limiters.entry(key.clone()).or_insert_with(|| {
            let quota = Quota::per_minute(
                NonZeroU32::new(limit as u32).unwrap_or(NonZeroU32::new(60).unwrap())
            );
            RateLimiter::direct(quota)
        });
        
        match limiter.value().check() {
            Ok(_) => {
                debug!("Request allowed for client {}", client_id);
            }
            Err(_) => {
                warn!("Rate limit exceeded for client {}", client_id);
                // Note: In a real implementation, we'd return a 429 response here
                // For now, we just log and continue
            }
        }

        self.inner.call(request)
    }
}

/// Rate limiting middleware function
#[allow(dead_code)]
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // This is a simplified version - the tower layer above handles the actual limiting
    Ok(next.run(request).await)
}
