use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawReading {
    pub sensor_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate: f64,
    pub unit: String,
    pub source: String,
}

pub struct SourceConfig {
    pub name: &'static str,
    pub url: &'static str,
    pub poll_interval_secs: u64,
}

pub struct DataSource {
    config: SourceConfig,
}

impl DataSource {
    pub fn safecast() -> Self {
        Self {
            config: SourceConfig {
                name: "safecast",
                url: "https://api.safecast.org/measurements.json",
                poll_interval_secs: 60,
            }
        }
    }
    
    pub fn uradmonitor() -> Self {
        Self {
            config: SourceConfig {
                name: "uradmonitor",
                url: "https://data.uradmonitor.com/api/v1/devices",
                poll_interval_secs: 30,
            }
        }
    }
    
    pub fn epa_radnet() -> Self {
        Self {
            config: SourceConfig {
                name: "epa_radnet",
                url: "https://www.epa.gov/radnet",
                poll_interval_secs: 300,
            }
        }
    }
    
    pub async fn run(&self, tx: Sender<RawReading>) {
        loop {
            match self.fetch().await {
                Ok(readings) => {
                    for reading in readings {
                        if tx.send(reading).await.is_err() {
                            return;
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("{} fetch failed: {}", self.config.name, e);
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(self.config.poll_interval_secs)).await;
        }
    }
    
    async fn fetch(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        // Placeholder implementation
        Ok(vec![])
    }
}
