use axum::{
    routing::get,
    Router,
    middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;
use tracing::{info, warn};

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
use cherenkov_core::{EventBus, CherenkovEvent, Anomaly, Alert, SensorStatus};


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
    
    // Initialize EventBus for inter-crate communication
    let event_bus = Arc::new(EventBus::new(10000));
    info!("EventBus initialized for API WebSocket broadcasting");
    
    // Subscribe to events from ingest and stream
    let mut event_rx = event_bus.subscribe();
    
    // Create WebSocket state
    let ws_state = create_websocket_state();
    
    // Start EventBus listener for broadcasting to WebSocket clients
    let ws_state_clone = ws_state.clone();
    let eventbus_listener = tokio::spawn(async move {
        eventbus_listener(event_rx, ws_state_clone).await
    });
    
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
    
    // Start server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("API server starting on {}", addr);
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    // Shutdown EventBus listener
    eventbus_listener.abort();
    info!("EventBus listener stopped");
    
    info!("Cherenkov API Server shutting down");
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

/// EventBus listener for broadcasting events to WebSocket clients
async fn eventbus_listener(
    mut event_rx: tokio::sync::broadcast::Receiver<CherenkovEvent>,
    ws_state: Arc<websocket::WebSocketState>,
) {
    use tracing::debug;
    
    info!("EventBus listener started for WebSocket broadcasting");
    
    while let Ok(event) = event_rx.recv().await {
        match event {
            CherenkovEvent::NewReading(reading) => {
                debug!("Received NewReading event for sensor {}", reading.sensor_id);
                
                // Broadcast to WebSocket clients subscribed to this sensor
                let message = serde_json::json!({
                    "type": "new_reading",
                    "data": reading
                });
                
                if let Err(e) = ws_state.broadcast(&reading.sensor_id.to_string(), message).await {
                    warn!("Failed to broadcast NewReading to WebSocket: {}", e);
                } else {
                    metrics::counter!("cherenkov_api_websocket_broadcasts_total", "event_type" => "new_reading").increment(1);
                }
            }
            CherenkovEvent::AnomalyDetected(anomaly) => {
                info!("Received AnomalyDetected event for sensor {}", anomaly.sensor_id);
                
                // Broadcast to all WebSocket clients (anomalies are public)
                let message = serde_json::json!({
                    "type": "anomaly",
                    "data": anomaly
                });
                
                if let Err(e) = ws_state.broadcast_all(message).await {
                    warn!("Failed to broadcast Anomaly to WebSocket: {}", e);
                } else {
                    metrics::counter!("cherenkov_api_websocket_broadcasts_total", "event_type" => "anomaly").increment(1);
                }
            }
            CherenkovEvent::AlertTriggered(alert) => {
                info!("Received AlertTriggered: {}", alert.message);
                
                let message = serde_json::json!({
                    "type": "alert",
                    "data": alert
                });
                
                if let Err(e) = ws_state.broadcast_all(message).await {
                    warn!("Failed to broadcast Alert to WebSocket: {}", e);
                } else {
                    metrics::counter!("cherenkov_api_websocket_broadcasts_total", "event_type" => "alert").increment(1);
                }
            }
            CherenkovEvent::CorrelatedEventDetected { primary, correlated_count, correlation_score } => {
                info!("Received CorrelatedEventDetected: {} correlated anomalies", correlated_count);
                
                let message = serde_json::json!({
                    "type": "correlated_event",
                    "data": {
                        "primary": primary,
                        "correlated_count": correlated_count,
                        "correlation_score": correlation_score
                    }
                });
                
                if let Err(e) = ws_state.broadcast_all(message).await {
                    warn!("Failed to broadcast CorrelatedEvent to WebSocket: {}", e);
                } else {
                    metrics::counter!("cherenkov_api_websocket_broadcasts_total", "event_type" => "correlated").increment(1);
                }
            }
            _ => {
                // Handle other event types as needed
            }
        }
    }
    
    info!("EventBus listener stopped");
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C signal"),
        _ = terminate => info!("Received SIGTERM signal"),
    }
}
