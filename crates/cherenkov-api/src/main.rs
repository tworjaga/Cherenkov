use axum::{
    routing::get,
    Router,
    middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;
use tracing::info;

mod graphql;
mod rest;
mod auth;
mod websocket;
mod rate_limit;

use auth::AuthState;
use websocket::{create_websocket_state, create_websocket_router};
use graphql::schema::build_schema;
use cherenkov_db::{RadiationDatabase, DatabaseConfig, scylla::ScyllaConfig};
use cherenkov_observability::init_observability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_observability();
    
    info!("Starting Cherenkov API Server v{}", env!("CARGO_PKG_VERSION"));
    
    // Initialize database
    let scylla_config = ScyllaConfig::default();
    let db = Arc::new(
        RadiationDatabase::new(
            scylla_config,
            "./data/cherenkov_warm.db",
            "redis://127.0.0.1:6379",
            DatabaseConfig::default(),
        ).await?
    );
    
    // Initialize authentication
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "cherenkov-dev-secret-change-in-production".to_string());
    let auth_state = Arc::new(AuthState::new(jwt_secret));
    
    // Build GraphQL schema
    let schema = build_schema(db.clone()).await?;
    
    // Create WebSocket state
    let ws_state = create_websocket_state();
    
    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // GraphQL endpoints
        .route("/graphql", get(graphql::handler).post(graphql::handler))
        
        // REST API v1
        .nest("/v1", rest::create_router(db.clone()))
        
        // WebSocket
        .nest("/ws", create_websocket_router(ws_state.clone()))
        
        // Layers
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            auth::auth_middleware,
        ))
        .layer(rate_limit::create_rate_limit_layer())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        
        // State
        .with_state((ws_state, db, auth_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app
    ).await?;
    
    Ok(())
}

use axum::extract::State;
use serde_json::json;

async fn health_check(State((_, db, _)): State<(Arc<websocket::WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>) -> impl axum::response::IntoResponse {
    let health = db.health_check().await;
    
    let status = if health.is_healthy() {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };
    
    let body = json!({
        "status": if health.is_healthy() { "healthy" } else { "unhealthy" },
        "database": {
            "hot": health.hot,
            "warm": health.warm,
            "cache": health.cache
        }
    });
    
    (status, axum::Json(body))
}

async fn readiness_check(State((_, db, _)): State<(Arc<websocket::WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>) -> impl axum::response::IntoResponse {
    let health = db.health_check().await;
    
    let status = if health.is_healthy() {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };
    
    let body = json!({
        "ready": health.is_healthy(),
        "checks": {
            "scylladb": health.hot,
            "sqlite": health.warm,
            "redis": health.cache
        }
    });
    
    (status, axum::Json(body))
}
