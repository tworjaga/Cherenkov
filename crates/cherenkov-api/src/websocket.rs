use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    extract::State,
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use cherenkov_db::RadiationDatabase;
use crate::auth::AuthState;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, RwLock, mpsc};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error, debug};
use futures_util::stream::StreamExt;

/// Maximum messages per second per connection (rate limiting)
const MAX_MESSAGES_PER_SECOND: u32 = 100;
/// Heartbeat interval in seconds
const HEARTBEAT_INTERVAL: u64 = 30;
/// Connection timeout in seconds
const CONNECTION_TIMEOUT: u64 = 120;

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
    SystemStatus,
    BatchUpdate,
}

/// Connection metadata for tracking and rate limiting
#[derive(Debug)]
pub struct ConnectionMeta {
    pub id: String,
    pub connected_at: Instant,
    pub last_activity: Instant,
    pub message_count: u32,
    pub last_message_time: Instant,
    pub subscribed_sensors: Vec<String>,
    pub subscribed_regions: Vec<GeoRegion>,
    pub client_info: Option<String>,
}

impl ConnectionMeta {
    pub fn new(id: String) -> Self {
        let now = Instant::now();
        Self {
            id,
            connected_at: now,
            last_activity: now,
            message_count: 0,
            last_message_time: now,
            subscribed_sensors: Vec::new(),
            subscribed_regions: Vec::new(),
            client_info: None,
        }
    }

    /// Check if connection has exceeded rate limit
    pub fn check_rate_limit(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_message_time);
        
        if elapsed >= Duration::from_secs(1) {
            // Reset counter after 1 second
            self.message_count = 0;
            self.last_message_time = now;
        }
        
        if self.message_count >= MAX_MESSAGES_PER_SECOND {
            return false;
        }
        
        self.message_count += 1;
        true
    }

    /// Update last activity timestamp
    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }

    /// Check if connection has timed out
    pub fn is_timed_out(&self) -> bool {
        Instant::now().duration_since(self.last_activity) > Duration::from_secs(CONNECTION_TIMEOUT)
    }
}

pub struct WebSocketState {
    pub tx: broadcast::Sender<RealTimeUpdate>,
    pub connections: RwLock<u64>,
    /// Track sensor subscriptions per connection
    pub sensor_subscriptions: RwLock<std::collections::HashMap<String, Vec<String>>>,
    /// Active connection metadata
    pub connection_meta: RwLock<std::collections::HashMap<String, ConnectionMeta>>,
    /// Batch message sender for high-throughput scenarios
    pub batch_tx: mpsc::Sender<Vec<RealTimeUpdate>>,
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
                debug!("Broadcasted to {} receivers", n);
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
                debug!("Broadcasted sensor {} update to {} receivers", sensor_id, n);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to broadcast to sensor {}: {}", sensor_id, e);
                Err("Broadcast failed".to_string())
            }
        }
    }

    /// Broadcast batch update for high-throughput scenarios
    pub async fn broadcast_batch(&self, updates: Vec<RealTimeUpdate>) -> Result<(), String> {
        for update in updates {
            if let Err(e) = self.tx.send(update) {
                warn!("Failed to send batch update: {}", e);
            }
        }
        metrics::counter!("cherenkov_websocket_broadcast_batch_total").increment(1);
        Ok(())
    }
    
    /// Get current connection count
    pub async fn connection_count(&self) -> u64 {
        *self.connections.read().await
    }

    /// Register new connection with metadata
    pub async fn register_connection(&self, conn_id: String, client_info: Option<String>) {
        let mut meta = self.connection_meta.write().await;
        let mut conn = ConnectionMeta::new(conn_id.clone());
        conn.client_info = client_info;
        meta.insert(conn_id, conn);
        debug!("Registered WebSocket connection: {}", conn_id);
    }

    /// Unregister connection
    pub async fn unregister_connection(&self, conn_id: &str) {
        let mut meta = self.connection_meta.write().await;
        meta.remove(conn_id);
        debug!("Unregistered WebSocket connection: {}", conn_id);
    }

    /// Update connection subscriptions
    pub async fn update_subscriptions(&self, conn_id: &str, sensors: Vec<String>, regions: Vec<GeoRegion>) {
        let mut meta = self.connection_meta.write().await;
        if let Some(conn) = meta.get_mut(conn_id) {
            conn.subscribed_sensors = sensors;
            conn.subscribed_regions = regions;
            conn.touch();
        }
    }

    /// Check if update should be sent to connection based on subscriptions
    pub async fn should_send_to_connection(&self, conn_id: &str, update: &RealTimeUpdate) -> bool {
        let meta = self.connection_meta.read().await;
        
        if let Some(conn) = meta.get(conn_id) {
            // Check sensor subscription
            if conn.subscribed_sensors.contains(&update.sensor_id) {
                return true;
            }
            
            // Check region subscription if coordinates available
            if let (Some(lat), Some(lon)) = (
                update.data.get("latitude").and_then(|v| v.as_f64()),
                update.data.get("longitude").and_then(|v| v.as_f64())
            ) {
                for region in &conn.subscribed_regions {
                    if region.contains(lat, lon) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}



pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State((ws_state, _db, _auth)): State<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, ws_state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<WebSocketState>) {
    let conn_id = uuid::Uuid::new_v4().to_string();
    let mut rx = state.tx.subscribe();
    
    // Register connection
    {
        let mut count = state.connections.write().await;
        *count += 1;
        state.register_connection(conn_id.clone(), None).await;
        info!("WebSocket client {} connected. Total connections: {}", conn_id, *count);
    }

    // Start heartbeat task
    let heartbeat_interval = tokio::time::interval(Duration::from_secs(HEARTBEAT_INTERVAL));
    let mut heartbeat = heartbeat_interval;
    
    loop {
        tokio::select! {
            _ = heartbeat.tick() => {
                // Send heartbeat ping
                if socket.send(Message::Ping(vec![])).await.is_err() {
                    break;
                }
                
                // Check for connection timeout
                let meta = state.connection_meta.read().await;
                if let Some(conn) = meta.get(&conn_id) {
                    if conn.is_timed_out() {
                        warn!("Connection {} timed out due to inactivity", conn_id);
                        break;
                    }
                }
            }
            
            msg = rx.recv() => {
                match msg {
                    Ok(update) => {
                        // Check if this connection should receive this update
                        if !state.should_send_to_connection(&conn_id, &update).await {
                            continue;
                        }

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

                        // Update activity timestamp
                        let mut meta = state.connection_meta.write().await;
                        if let Some(conn) = meta.get_mut(&conn_id) {
                            conn.touch();
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!("WebSocket client {} lagged behind by {} messages", conn_id, n);
                        // Send lag notification to client
                        let _ = socket.send(Message::Text(format!(
                            r#"{{"type":"lag_notification","missed_messages":{}}}"#, n
                        ))).await;
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
                        // Update activity
                        let mut meta = state.connection_meta.write().await;
                        if let Some(conn) = meta.get_mut(&conn_id) {
                            conn.touch();
                        }
                    }
                    Some(Ok(Message::Pong(_))) => {
                        // Client responded to our ping
                        let mut meta = state.connection_meta.write().await;
                        if let Some(conn) = meta.get_mut(&conn_id) {
                            conn.touch();
                        }
                    }
                    Some(Ok(Message::Text(text))) => {
                        // Check rate limit
                        let mut meta = state.connection_meta.write().await;
                        if let Some(conn) = meta.get_mut(&conn_id) {
                            if !conn.check_rate_limit() {
                                warn!("Rate limit exceeded for connection {}", conn_id);
                                let _ = socket.send(Message::Text(
                                    r#"{"type":"error","message":"Rate limit exceeded"}"#.to_string()
                                )).await;
                                continue;
                            }
                            conn.touch();
                        }
                        drop(meta);

                        if let Ok(cmd) = serde_json::from_str::<ClientCommand>(&text) {
                            handle_client_command(&mut socket, &conn_id, cmd, &state).await;
                        } else {
                            let _ = socket.send(Message::Text(
                                r#"{"type":"error","message":"Invalid command format"}"#.to_string()
                            )).await;
                        }
                    }
                    Some(Ok(Message::Binary(data))) => {
                        debug!("Received binary message from {} ({} bytes)", conn_id, data.len());
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error on {}: {}", conn_id, e);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    
    // Cleanup
    {
        let mut count = state.connections.write().await;
        *count -= 1;
        state.unregister_connection(&conn_id).await;
        info!("WebSocket client {} disconnected. Total connections: {}", conn_id, *count);
    }
}


#[derive(Debug, Deserialize)]
struct ClientCommand {
    action: String,
    sensor_id: Option<String>,
    sensor_ids: Option<Vec<String>>,
    region: Option<GeoRegion>,
    regions: Option<Vec<GeoRegion>>,
    client_info: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeoRegion {
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
}

impl GeoRegion {
    pub fn contains(&self, lat: f64, lon: f64) -> bool {
        lat >= self.lat_min && lat <= self.lat_max &&
        lon >= self.lon_min && lon <= self.lon_max
    }
}

async fn handle_client_command(
    socket: &mut WebSocket, 
    conn_id: &str, 
    cmd: ClientCommand,
    state: &Arc<WebSocketState>
) {
    match cmd.action.as_str() {
        "subscribe_sensor" => {
            if let Some(sensor_id) = cmd.sensor_id {
                info!("Client {} subscribed to sensor: {}", conn_id, sensor_id);
                let mut meta = state.connection_meta.write().await;
                if let Some(conn) = meta.get_mut(conn_id) {
                    if !conn.subscribed_sensors.contains(&sensor_id) {
                        conn.subscribed_sensors.push(sensor_id.clone());
                    }
                }
                
                let response = format!(
                    r#"{{"type":"subscribed","sensor_id":"{}","status":"ok"}}"#,
                    sensor_id
                );
                let _ = socket.send(Message::Text(response)).await;
            }
        }
        "subscribe_sensors" => {
            if let Some(sensor_ids) = cmd.sensor_ids {
                info!("Client {} subscribed to {} sensors", conn_id, sensor_ids.len());
                let mut meta = state.connection_meta.write().await;
                if let Some(conn) = meta.get_mut(conn_id) {
                    for sensor_id in &sensor_ids {
                        if !conn.subscribed_sensors.contains(sensor_id) {
                            conn.subscribed_sensors.push(sensor_id.clone());
                        }
                    }
                }
                
                let response = format!(
                    r#"{{"type":"subscribed","sensor_count":{},"status":"ok"}}"#,
                    sensor_ids.len()
                );
                let _ = socket.send(Message::Text(response)).await;
            }
        }
        "subscribe_region" => {
            if let Some(region) = cmd.region {
                info!("Client {} subscribed to region: [{}, {}] x [{}, {}]", 
                    conn_id, region.lat_min, region.lat_max, region.lon_min, region.lon_max);
                let mut meta = state.connection_meta.write().await;
                if let Some(conn) = meta.get_mut(conn_id) {
                    conn.subscribed_regions.push(region);
                }
                
                let _ = socket.send(Message::Text(
                    r#"{"type":"subscribed","region":"active","status":"ok"}"#.to_string()
                )).await;
            }
        }
        "unsubscribe_sensor" => {
            if let Some(sensor_id) = cmd.sensor_id {
                let mut meta = state.connection_meta.write().await;
                if let Some(conn) = meta.get_mut(conn_id) {
                    conn.subscribed_sensors.retain(|s| s != &sensor_id);
                }
                info!("Client {} unsubscribed from sensor: {}", conn_id, sensor_id);
                
                let _ = socket.send(Message::Text(
                    format!(r#"{{"type":"unsubscribed","sensor_id":"{}","status":"ok"}}"#, sensor_id)
                )).await;
            }
        }
        "get_subscriptions" => {
            let meta = state.connection_meta.read().await;
            if let Some(conn) = meta.get(conn_id) {
                let response = serde_json::json!({
                    "type": "subscriptions",
                    "sensors": conn.subscribed_sensors,
                    "sensor_count": conn.subscribed_sensors.len(),
                    "region_count": conn.subscribed_regions.len(),
                });
                let _ = socket.send(Message::Text(response.to_string())).await;
            }
        }
        "ping" => {
            let _ = socket.send(Message::Text(r#"{"type":"pong","timestamp":""#.to_string() + 
                &chrono::Utc::now().timestamp_millis().to_string() + r#""}"#)).await;
        }
        "get_status" => {
            let conn_count = state.connection_count().await;
            let response = serde_json::json!({
                "type": "status",
                "connections": conn_count,
                "server_time": chrono::Utc::now().to_rfc3339(),
            });
            let _ = socket.send(Message::Text(response.to_string())).await;
        }
        _ => {
            warn!("Unknown client command from {}: {}", conn_id, cmd.action);
            let _ = socket.send(Message::Text(
                format!(r#"{{"type":"error","message":"Unknown command: {}"}}"#, cmd.action)
            )).await;
        }
    }
}


pub fn create_websocket_router(state: (Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)) -> Router<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)> {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

pub fn create_websocket_state() -> Arc<WebSocketState> {
    let (tx, _rx) = broadcast::channel(10000);
    let (batch_tx, mut batch_rx) = mpsc::channel(1000);
    
    let state = Arc::new(WebSocketState {
        tx,
        connections: RwLock::new(0),
        sensor_subscriptions: RwLock::new(std::collections::HashMap::new()),
        connection_meta: RwLock::new(std::collections::HashMap::new()),
        batch_tx,
    });

    // Spawn batch processing task
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut batch = Vec::with_capacity(100);
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        
        loop {
            tokio::select! {
                Some(updates) = batch_rx.recv() => {
                    batch.extend(updates);
                    if batch.len() >= 100 {
                        // Process batch
                        if let Err(e) = state_clone.broadcast_batch(batch.clone()).await {
                            warn!("Batch broadcast failed: {}", e);
                        }
                        batch.clear();
                    }
                }
                _ = interval.tick() => {
                    if !batch.is_empty() {
                        if let Err(e) = state_clone.broadcast_batch(batch.clone()).await {
                            warn!("Batch broadcast failed: {}", e);
                        }
                        batch.clear();
                    }
                }
            }
        }
    });

    state
}
