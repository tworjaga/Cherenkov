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
use cherenkov_core::{EventBus, CherenkovEvent, NormalizedReading, Anomaly as CoreAnomaly, Severity as CoreSeverity};


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
    
    // Initialize EventBus for inter-crate communication
    let event_bus = Arc::new(EventBus::new(10000));
    info!("EventBus initialized for stream processing");
    
    // Subscribe to NewReading events from ingest
    let mut reading_rx = event_bus.subscribe();
    info!("Subscribed to NewReading events from EventBus");
    
    // Create broadcast channel for real-time anomaly alerts (internal)
    let (anomaly_tx, _) = broadcast::channel(1000);
    
    // Create processor
    let processor = StreamProcessor::new(
        db.clone(),
        anomaly_tx.clone(),
    );
    
    // Start EventBus listener for new readings
    let eventbus_listener = tokio::spawn(eventbus_listener(
        reading_rx,
        processor.get_ingest_tx(),
    ));
    
    // Start anomaly detection worker
    let detection_worker = tokio::spawn(anomaly_detection_worker(
        processor.get_reading_rx(),
        anomaly_tx.clone(),
        event_bus.clone(),
        db.clone(),
    ));
    
    // Start WebSocket broadcaster
    let ws_broadcaster = tokio::spawn(websocket_broadcaster(
        anomaly_tx.subscribe(),
        event_bus.clone(),
    ));

    
    // Start correlation engine
    let correlation_worker = tokio::spawn(correlation_engine_worker(
        anomaly_tx.subscribe(),
        event_bus.clone(),
        db.clone(),
    ));

    
    // Start health check server
    let health_server = tokio::spawn(health_check_server(db.clone()));
    
    // Wait for shutdown signal
    tokio::select! {
        _ = eventbus_listener => warn!("EventBus listener exited"),
        _ = detection_worker => warn!("Detection worker exited"),
        _ = ws_broadcaster => warn!("WebSocket broadcaster exited"),
        _ = correlation_worker => warn!("Correlation worker exited"),
        _ = health_server => warn!("Health server exited"),
        _ = tokio::signal::ctrl_c() => info!("Shutdown signal received"),
    }

    
    info!("Cherenkov Stream Processor shutting down");
    Ok(())
}

/// Listen for new readings from EventBus
async fn eventbus_listener(
    mut reading_rx: tokio::sync::broadcast::Receiver<CherenkovEvent>,
    ingest_tx: mpsc::Sender<RadiationReading>,
) {
    info!("EventBus listener started for NewReading events");
    
    while let Ok(event) = reading_rx.recv().await {
        match event {
            CherenkovEvent::NewReading(reading) => {
                // Convert NormalizedReading to RadiationReading
                let radiation_reading = RadiationReading {
                    sensor_id: reading.sensor_id,
                    bucket: reading.timestamp.timestamp() / 3600,
                    timestamp: reading.timestamp.timestamp(),
                    latitude: reading.latitude,
                    longitude: reading.longitude,
                    dose_rate_microsieverts: reading.dose_rate_microsieverts,
                    uncertainty: reading.uncertainty,
                    quality_flag: match reading.quality_flag {
                        cherenkov_core::QualityFlag::Valid => cherenkov_db::QualityFlag::Valid,
                        cherenkov_core::QualityFlag::Suspect => cherenkov_db::QualityFlag::Suspect,
                        cherenkov_core::QualityFlag::Invalid => cherenkov_db::QualityFlag::Invalid,
                    },
                    source: reading.source,
                    cell_id: reading.cell_id,
                };
                
                if let Err(e) = ingest_tx.send(radiation_reading).await {
                    warn!("Failed to send reading to processor: {}", e);
                    break;
                }
                
                metrics::counter!("cherenkov_stream_events_received_total").increment(1);
            }
            _ => {
                // Ignore other event types
            }
        }
    }
}


/// Anomaly detection worker with sliding windows
async fn anomaly_detection_worker(
    mut reading_rx: mpsc::Receiver<RadiationReading>,
    anomaly_tx: broadcast::Sender<Anomaly>,
    event_bus: Arc<EventBus>,
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
            
            // Broadcast to internal subscribers
            if let Err(e) = anomaly_tx.send(anomaly.clone()) {
                warn!("Failed to broadcast anomaly internally: {}", e);
            }
            
            // Publish to EventBus for API and other consumers
            let core_anomaly = CoreAnomaly {
                sensor_id: anomaly.sensor_id.clone(),
                severity: match anomaly.severity {
                    Severity::Critical => CoreSeverity::Critical,
                    Severity::Warning => CoreSeverity::Warning,
                    Severity::Info => CoreSeverity::Info,
                },
                z_score: anomaly.z_score,
                timestamp: anomaly.timestamp,
                dose_rate: anomaly.dose_rate,
                baseline: anomaly.baseline,
                algorithm: match anomaly.algorithm {
                    anomaly::Algorithm::Welford => cherenkov_core::Algorithm::Welford,
                    anomaly::Algorithm::IsolationForest => cherenkov_core::Algorithm::IsolationForest,
                },
            };
            
            let event = CherenkovEvent::AnomalyDetected(core_anomaly);
            if let Err(e) = event_bus.publish(event).await {
                warn!("Failed to publish AnomalyDetected to EventBus: {}", e);
            } else {
                metrics::counter!("cherenkov_stream_anomaly_events_published_total").increment(1);
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
    
    // Store to database
    db.store_event(&event).await?;
    info!("Stored anomaly event: {}", event.event_id);
    
    Ok(())
}


/// WebSocket broadcaster for real-time updates
/// Note: Actual WebSocket broadcasting happens via EventBus to API crate
async fn websocket_broadcaster(
    mut anomaly_rx: broadcast::Receiver<Anomaly>,
    event_bus: Arc<EventBus>,
) {
    info!("WebSocket broadcaster started (via EventBus)");
    
    while let Ok(anomaly) = anomaly_rx.recv().await {
        // Convert to core anomaly and publish to EventBus
        // API crate will receive and broadcast to WebSocket clients
        let core_anomaly = CoreAnomaly {
            sensor_id: anomaly.sensor_id.clone(),
            severity: match anomaly.severity {
                Severity::Critical => CoreSeverity::Critical,
                Severity::Warning => CoreSeverity::Warning,
                Severity::Info => CoreSeverity::Info,
            },
            z_score: anomaly.z_score,
            timestamp: anomaly.timestamp,
            dose_rate: anomaly.dose_rate,
            baseline: anomaly.baseline,
            algorithm: match anomaly.algorithm {
                anomaly::Algorithm::Welford => cherenkov_core::Algorithm::Welford,
                anomaly::Algorithm::IsolationForest => cherenkov_core::Algorithm::IsolationForest,
            },
        };
        
        let event = CherenkovEvent::AnomalyDetected(core_anomaly);
        if let Err(e) = event_bus.publish(event).await {
            warn!("Failed to publish anomaly to EventBus: {}", e);
        } else {
            metrics::counter!("cherenkov_stream_anomaly_broadcasts_total").increment(1);
        }
    }
}


/// Correlation engine for cross-sensor analysis
async fn correlation_engine_worker(
    mut anomaly_rx: broadcast::Receiver<Anomaly>,
    event_bus: Arc<EventBus>,
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
                primary_anomaly: anomaly.clone(),
                related_anomalies: correlated.clone(),
                correlation_score: 0.95,
                detected_at: Utc::now(),
            };
            
            // Publish correlated event to EventBus
            let core_anomaly = CoreAnomaly {
                sensor_id: anomaly.sensor_id.clone(),
                severity: match anomaly.severity {
                    Severity::Critical => CoreSeverity::Critical,
                    Severity::Warning => CoreSeverity::Warning,
                    Severity::Info => CoreSeverity::Info,
                },
                z_score: anomaly.z_score,
                timestamp: anomaly.timestamp,
                dose_rate: anomaly.dose_rate,
                baseline: anomaly.baseline,
                algorithm: match anomaly.algorithm {
                    anomaly::Algorithm::Welford => cherenkov_core::Algorithm::Welford,
                    anomaly::Algorithm::IsolationForest => cherenkov_core::Algorithm::IsolationForest,
                },
            };
            
            let event = CherenkovEvent::CorrelatedEventDetected {
                primary: core_anomaly,
                correlated_count: correlated.len(),
                correlation_score: 0.95,
            };
            
            if let Err(e) = event_bus.publish(event).await {
                warn!("Failed to publish CorrelatedEvent to EventBus: {}", e);
            }
            
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
