use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

mod graphql;
mod rest;
mod auth;

use graphql::schema::build_schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("Starting Cherenkov API Server");
    
    let schema = build_schema().await?;
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/graphql", get(graphql::handler).post(graphql::handler))
        .route("/ws", get(graphql::subscription_handler))
        .layer(CorsLayer::permissive());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
