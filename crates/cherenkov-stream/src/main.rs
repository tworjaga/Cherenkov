use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, broadcast, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use futures::StreamExt;
use tracing::{info, warn, error, instrument};
use chrono::Utc;
use uuid::Uuid;

mod anomaly;
mod window;
mod correlation;
mod processor;

use anomaly::{AnomalyDetector, Anomaly, Severity};
use window::SlidingWindow;
use correlation::CorrelationEngine;
use processor::StreamProcessor;
use cherenkov_db::{RadiationDatabase, RadiationReading, DatabaseConfig, scylla::ScyllaConfig};
use cherenkov_observability::init_observability;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_observability();
    
    info!("Starting Cherenkov Stream Processor v{}", env!("CARGO_PKG_VERSION"));
    
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
    
    // Create broadcast channel for real-time anomaly alerts
    let (anomaly_tx, _) = broadcast::channel(1000);
    
    // Create processor
    let processor = StreamProcessor::new(
        db.clone(),
        anomaly_tx.clone(),
    );
    
    // Start Redis pub/sub listener for new readings
    let redis_listener = tokio::spawn(redis_listener(db.clone(), processor.get_ingest_tx()));
    
    // Start anomaly detection worker
    let detection_worker = tokio::spawn(anomaly_detection_worker(
        processor.get_reading_rx(),
        anomaly_tx.clone(),
        db.clone(),
    ));
    
    // Start WebSocket broadcaster
    let ws_broadcaster = tokio::spawn(websocket_broadcaster(anomaly_tx.subscribe()));
    
    // Start correlation engine
    let correlation_worker = tokio::spawn(correlation_engine_worker(
        anomaly_tx.subscribe(),
        db.clone(),
    ));
    
    // Start health check server
    let health_server = tokio::spawn(health_check_server(db.clone()));
    
    // Wait for shutdown signal
    tokio::select! {
        _ = redis_listener => warn!("Redis listener exited"),
        _ = detection_worker => warn!("Detection worker exited"),
        _ = ws_broadcaster => warn!("WebSocket broadcaster exited"),
        _ = correlation_worker => warn!("Correlation worker exited"),
        _ = health_server => warn!("Health server exited"),
        _ = tokio::signal::ctrl_c() => info!("Shutdown signal received"),
    }
    
    info!("Cherenkov Stream Processor shutting down");
    Ok(())
}

/// Listen for new readings from Redis pub/sub
async fn redis_listener(
    db: Arc<RadiationDatabase>,
    ingest_tx: mpsc::Sender<RadiationReading>,
) {
    let client = match redis::Client::open("redis://127.0.0.1:6379") {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to connect to Redis: {}", e);
            return;
        }
    };
    
    let mut pubsub = match client.get_async_connection().await {
        Ok(conn) => conn.into_pubsub(),
        Err(e) => {
            error!("Failed to get Redis connection: {}", e);
            return;
        }
    };
    
    if let Err(e) = pubsub.subscribe("cherenkov:readings").await {
        error!("Failed to subscribe to readings channel: {}", e);
        return;
    }
    
    info!("Redis pub/sub listener started");
    
    let mut stream = pubsub.on_message();
    
    while let Some(msg) = stream.next().await {
        let payload: String = match msg.get_payload() {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to get message payload: {}", e);
                continue;
            }
        };
        
        let reading: RadiationReading = match serde_json::from_str(&payload) {
            Ok(r) => r,
            Err(e) => {
                warn!("Failed to deserialize reading: {}", e);
                continue;
            }
        };
        
        if let Err(e) = ingest_tx.send(reading).await {
            warn!("Failed to send reading to processor: {}", e);
            break;
        }
    }
}

/// Anomaly detection worker with sliding windows
async fn anomaly_detection_worker(
    mut reading_rx: mpsc::Receiver<RadiationReading>,
    anomaly_tx: broadcast::Sender<Anomaly>,
    db: Arc<RadiationDatabase>,
) {
    let mut detector = AnomalyDetector::new();
    let mut windows: SlidingWindow = SlidingWindow::new(
        Duration::from_secs(3600), // 1 hour window
        Duration::from_secs(60),   // 1 minute slide
    );
    
    info!("Anomaly detection worker started");
    
    while let Some(reading) = reading_rx.recv().await {
        // Add to sliding window
        windows.add(reading.clone());
        
        // Get window for this sensor
        let sensor_window = windows.get_window(&reading.sensor_id.to_string());
        
        // Run anomaly detection
        if let Some(anomaly) = detector.detect(sensor_window) {
            info!("Anomaly detected: {:?}", anomaly);
            
            // Store anomaly in database
            if let Err(e) = store_anomaly(&db, &anomaly).await {
                error!("Failed to store anomaly: {}", e);
            }
            
            // Broadcast to subscribers
            if let Err(e) = anomaly_tx.send(anomaly) {
                warn!("Failed to broadcast anomaly: {}", e);
            }
            
            metrics::counter!("cherenkov_anomalies_detected_total", 
                "severity" => format!("{:?}", anomaly.severity)
            ).increment(1);
        }
    }
}

/// Store anomaly in database
#[instrument(skip(db, anomaly))]
async fn store_anomaly(
    db: &Arc<RadiationDatabase>,
    anomaly: &Anomaly,
) -> anyhow::Result<()> {
    // Convert anomaly to domain event
    let event = cherenkov_db::DomainEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: cherenkov_db::EventType::AnomalyDetected,
        aggregate_id: Uuid::parse_str(&anomaly.sensor_id)?,
        payload: serde_json::to_value(anomaly)?,
        timestamp: anomaly.timestamp.timestamp(),
    };
    
    // TODO: Implement event store in database
    info!("Stored anomaly event: {}", event.event_id);
    
    Ok(())
}

/// WebSocket broadcaster for real-time updates
async fn websocket_broadcaster(
    mut anomaly_rx: broadcast::Receiver<Anomaly>,
) {
    info!("WebSocket broadcaster started");
    
    // TODO: Implement WebSocket server for real-time anomaly streaming
    // For now, just log received anomalies
    
    while let Ok(anomaly) = anomaly_rx.recv().await {
        debug!("Broadcasting anomaly to WebSocket clients: {:?}", anomaly);
        metrics::gauge!("cherenkov_websocket_subscribers").set(0.0); // Placeholder
    }
}

/// Correlation engine for cross-sensor analysis
async fn correlation_engine_worker(
    mut anomaly_rx: broadcast::Receiver<Anomaly>,
    db: Arc<RadiationDatabase>,
) {
    let mut correlation_engine = CorrelationEngine::new(db.clone());
    
    info!("Correlation engine worker started");
    
    while let Ok(anomaly) = anomaly_rx.recv().await {
        // Check for correlated events
        let correlated = correlation_engine.check_correlation(&anomaly).await;
        
        if !correlated.is_empty() {
            warn!("Correlated anomalies detected: {} events", correlated.len());
            
            // Create correlated event alert
            let correlated_event = CorrelatedEvent {
                primary_anomaly: anomaly,
                related_anomalies: correlated,
                correlation_score: 0.95,
                detected_at: Utc::now(),
            };
            
            // TODO: Store and alert on correlated events
            info!("Correlated event: {:?}", correlated_event);
        }
    }
}

/// Health check server
async fn health_check_server(db: Arc<RadiationDatabase>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        let health = db.health_check().await;
        if !health.is_healthy() {
            warn!("Database health check failed in stream processor");
        }
        
        // TODO: Expose health metrics for monitoring
    }
}

#[derive(Debug, Clone)]
pub struct CorrelatedEvent {
    pub primary_anomaly: Anomaly,
    pub related_anomalies: Vec<Anomaly>,
    pub correlation_score: f64,
    pub detected_at: chrono::DateTime<Utc>,
}
