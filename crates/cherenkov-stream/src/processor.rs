use std::sync::Arc;
use tokio::sync::{mpsc, broadcast};
use tracing::{info, debug};

use cherenkov_db::RadiationDatabase;
use crate::anomaly::Anomaly;

/// Stream processor coordinating anomaly detection pipeline
pub struct StreamProcessor {
    db: Arc<RadiationDatabase>,
    ingest_tx: mpsc::Sender<RadiationReading>,
    ingest_rx: mpsc::Receiver<RadiationReading>,
    reading_tx: mpsc::Sender<RadiationReading>,
    reading_rx: mpsc::Receiver<RadiationReading>,
    anomaly_tx: broadcast::Sender<Anomaly>,
}

use cherenkov_db::RadiationReading;

impl StreamProcessor {
    pub fn new(
        db: Arc<RadiationDatabase>,
        anomaly_tx: broadcast::Sender<Anomaly>,
    ) -> Self {
        let (ingest_tx, ingest_rx) = mpsc::channel(10000);
        let (reading_tx, reading_rx) = mpsc::channel(10000);
        
        Self {
            db,
            ingest_tx,
            ingest_rx,
            reading_tx,
            reading_rx,
            anomaly_tx,
        }
    }

    pub fn get_ingest_tx(&self) -> mpsc::Sender<RadiationReading> {
        self.ingest_tx.clone()
    }

    pub fn get_reading_rx(&self) -> mpsc::Receiver<RadiationReading> {
        self.reading_rx.clone()
    }

    /// Start the processor pipeline
    pub async fn run(mut self) -> anyhow::Result<()> {
        info!("Stream processor starting");
        
        // Bridge ingest channel to reading channel with filtering
        while let Some(reading) = self.ingest_rx.recv().await {
            // Filter invalid readings
            if reading.dose_rate_microsieverts < 0.0 {
                debug!("Dropping negative dose rate reading");
                continue;
            }
            
            // Forward to reading channel
            if self.reading_tx.send(reading).await.is_err() {
                break;
            }
        }
        
        Ok(())
    }
}
