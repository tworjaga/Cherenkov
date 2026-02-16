use std::sync::Arc;
use tokio::sync::{mpsc, broadcast, RwLock};
use tracing::{info, debug, warn, error, instrument};
use std::collections::HashMap;

use cherenkov_db::{RadiationDatabase, RadiationReading};
use crate::anomaly::{Anomaly, AnomalyDetector, Severity};
use crate::window::SlidingWindow;

/// Stream processor coordinating anomaly detection pipeline
pub struct StreamProcessor {
    db: Arc<RadiationDatabase>,
    ingest_tx: mpsc::Sender<RadiationReading>,
    ingest_rx: mpsc::Receiver<RadiationReading>,
    anomaly_tx: broadcast::Sender<Anomaly>,
    detector: Arc<RwLock<AnomalyDetector>>,
    windows: Arc<RwLock<HashMap<String, SlidingWindow>>>,
}

impl StreamProcessor {
    pub fn new(
        db: Arc<RadiationDatabase>,
        anomaly_tx: broadcast::Sender<Anomaly>,
    ) -> Self {
        let (ingest_tx, ingest_rx) = mpsc::channel(10000);
        
        Self {
            db,
            ingest_tx,
            ingest_rx,
            anomaly_tx,
            detector: Arc::new(RwLock::new(AnomalyDetector::new())),
            windows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_ingest_tx(&self) -> mpsc::Sender<RadiationReading> {
        self.ingest_tx.clone()
    }

    pub fn subscribe_anomalies(&self) -> broadcast::Receiver<Anomaly> {
        self.anomaly_tx.subscribe()
    }

    /// Start the processor pipeline with multiple workers
    pub async fn run(mut self) -> anyhow::Result<()> {
        info!("Stream processor starting with anomaly detection");

        let db = self.db.clone();
        let anomaly_tx = self.anomaly_tx.clone();
        let detector = self.detector.clone();
        let windows = self.windows.clone();

        // Spawn anomaly detection worker
        let detection_handle = tokio::spawn(async move {
            Self::anomaly_detection_worker(
                self.ingest_rx,
                db,
                anomaly_tx,
                detector,
                windows,
            ).await;
        });

        // Wait for workers
        tokio::select! {
            _ = detection_handle => warn!("Anomaly detection worker exited"),
            _ = tokio::signal::ctrl_c() => info!("Shutdown signal received"),
        }

        info!("Stream processor shutting down");
        Ok(())
    }

    #[instrument(skip(rx, db, anomaly_tx, detector, windows))]
    async fn anomaly_detection_worker(
        mut rx: mpsc::Receiver<RadiationReading>,
        db: Arc<RadiationDatabase>,
        anomaly_tx: broadcast::Sender<Anomaly>,
        detector: Arc<RwLock<AnomalyDetector>>,
        windows: Arc<RwLock<HashMap<String, SlidingWindow>>>,
    ) {
        info!("Anomaly detection worker started");

        while let Some(reading) = rx.recv().await {
            // Filter invalid readings
            if reading.dose_rate_microsieverts < 0.0 {
                debug!("Dropping negative dose rate reading");
                continue;
            }

            let sensor_id = reading.sensor_id.to_string();

            // Get or create sliding window for this sensor
            let mut windows_guard = windows.write().await;
            let window = windows_guard.entry(sensor_id.clone()).or_insert_with(|| {
                SlidingWindow::new(3600, 100) // 1 hour window, max 100 readings
            });

            // Add reading to window
            window.add(crate::window::Reading {
                sensor_id: sensor_id.clone(),
                dose_rate: reading.dose_rate_microsieverts,
                timestamp: chrono::DateTime::from_timestamp(reading.timestamp, 0)
                    .unwrap_or_else(|| chrono::Utc::now()),
            });

            // Run anomaly detection
            let window_data = window.get_window();
            if window_data.len() >= 10 { // Need minimum samples
                let mut detector_guard = detector.write().await;
                
                if let Some(anomaly) = detector_guard.detect(window_data.to_vec()) {
                    info!("Anomaly detected for sensor {}: z_score={:.2}", 
                        sensor_id, anomaly.z_score);

                    // Store anomaly in database
                    if let Err(e) = store_anomaly(&db, &anomaly).await {
                        warn!("Failed to store anomaly: {}", e);
                    }

                    // Broadcast to subscribers
                    if let Err(e) = anomaly_tx.send(anomaly) {
                        warn!("Failed to broadcast anomaly: {}", e);
                    }
                }
            }

            // Cleanup old windows periodically
            if windows_guard.len() > 10000 {
                windows_guard.retain(|_, w| !w.is_stale(3600));
            }
        }

        info!("Anomaly detection worker stopped");
    }
}

/// Store anomaly in database
#[instrument(skip(db, anomaly))]
async fn store_anomaly(
    db: &Arc<RadiationDatabase>,
    anomaly: &Anomaly,
) -> anyhow::Result<()> {
    // For now, just log the anomaly
    // In production, this would write to an anomalies table
    info!(
        "Storing anomaly: sensor={}, severity={:?}, z_score={:.2}",
        anomaly.sensor_id, anomaly.severity, anomaly.z_score
    );
    
    // TODO: Implement actual database write to anomalies table
    // db.warm.store_anomaly(anomaly).await?;
    
    Ok(())
}
