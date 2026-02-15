use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn, error};

mod sources;
mod normalizer;
mod metrics;

use sources::{DataSource, SourceConfig};
use normalizer::Normalizer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("Starting Cherenkov Ingest Daemon");
    
    let (tx, mut rx) = mpsc::channel(10000);
    
    let sources = vec![
        DataSource::safecast(),
        DataSource::uradmonitor(),
        DataSource::epa_radnet(),
    ];
    
    for source in sources {
        let tx = tx.clone();
        tokio::spawn(async move {
            source.run(tx).await;
        });
    }
    
    let normalizer = Normalizer::new();
    
    while let Some(reading) = rx.recv().await {
        match normalizer.normalize(reading).await {
            Ok(normalized) => {
                metrics::record_ingest(&normalized.source);
            }
            Err(e) => {
                warn!("Normalization failed: {}", e);
            }
        }
    }
    
    Ok(())
}
