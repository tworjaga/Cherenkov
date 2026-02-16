use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    extract::State,
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeUpdate {
    pub update_type: UpdateType,
    pub sensor_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    Reading,
    Anomaly,
    Alert,
    FacilityStatus,
}

pub struct WebSocketState {
    pub tx: broadcast::Sender<RealTimeUpdate>,
    pub connections: RwLock<u64>,
    /// Track sensor subscriptions per connection (simplified)
    pub sensor_subscriptions: RwLock<std::collections::HashMap<String, Vec<String>>>,
}

impl WebSocketState {
    /// Broadcast message to all connected clients
    pub async fn broadcast_all(&self, message: serde_json::Value) -> Result<(), String> {
        let update = RealTimeUpdate {
            update_type: UpdateType::Alert,
            sensor_id: "all".to_string(),
            timestamp: chrono::Utc::now(),
            data: message,
        };
        
        match self.tx.send(update) {
            Ok(n) => {
                metrics::counter!("cherenkov_websocket_broadcast_all_total").increment(1);
                tracing::debug!("Broadcasted to {} receivers", n);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to broadcast to all: {}", e);
                Err("Broadcast failed".to_string())
            }
        }
    }
    
    /// Broadcast message to clients subscribed to a specific sensor
    pub async fn broadcast(&self, sensor_id: &str, message: serde_json::Value) -> Result<(), String> {
        let update = RealTimeUpdate {
            update_type: UpdateType::Reading,
            sensor_id: sensor_id.to_string(),
            timestamp: chrono::Utc::now(),
            data: message,
        };
        
        match self.tx.send(update) {
            Ok(n) => {
                metrics::counter!("cherenkov_websocket_broadcast_sensor_total", "sensor_id" => sensor_id.to_string()).increment(1);
                tracing::debug!("Broadcasted sensor {} update to {} receivers", sensor_id, n);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to broadcast to sensor {}: {}", sensor_id, e);
                Err("Broadcast failed".to_string())
            }
        }
    }
    
    /// Get current connection count
    pub async fn connection_count(&self) -> u64 {
        *self.connections.read().await
    }
}


pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WebSocketState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<WebSocketState>) {
    let mut rx = state.tx.subscribe();
    
    {
        let mut count = state.connections.write().await;
        *count += 1;
        info!("WebSocket client connected. Total connections: {}", *count);
    }
    
    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(update) => {
                        let json = match serde_json::to_string(&update) {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Failed to serialize update: {}", e);
                                continue;
                            }
                        };
                        
                        if socket.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!("WebSocket client lagged behind by {} messages", n);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            
            result = socket.recv() => {
                match result {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(cmd) = serde_json::from_str::<ClientCommand>(&text) {
                            handle_client_command(&mut socket, cmd).await;
                        }
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    
    {
        let mut count = state.connections.write().await;
        *count -= 1;
        info!("WebSocket client disconnected. Total connections: {}", *count);
    }
}

#[derive(Debug, Deserialize)]
struct ClientCommand {
    action: String,
    sensor_id: Option<String>,
    region: Option<GeoRegion>,
}

#[derive(Debug, Deserialize)]
struct GeoRegion {
    lat_min: f64,
    lat_max: f64,
    lon_min: f64,
    lon_max: f64,
}

async fn handle_client_command(socket: &mut WebSocket, cmd: ClientCommand) {
    match cmd.action.as_str() {
        "subscribe_sensor" => {
            info!("Client subscribed to sensor: {:?}", cmd.sensor_id);
        }
        "subscribe_region" => {
            info!("Client subscribed to region: {:?}", cmd.region);
        }
        "ping" => {
            let _ = socket.send(Message::Text(r#"{"type":"pong"}"#.to_string())).await;
        }
        _ => {
            warn!("Unknown client command: {}", cmd.action);
        }
    }
}

pub fn create_websocket_router(state: Arc<WebSocketState>) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

pub fn create_websocket_state() -> Arc<WebSocketState> {
    let (tx, _rx) = broadcast::channel(10000);
    Arc::new(WebSocketState {
        tx,
        connections: RwLock::new(0),
        sensor_subscriptions: RwLock::new(std::collections::HashMap::new()),
    })
}
