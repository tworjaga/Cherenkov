pub mod sources;
pub mod pipeline;
pub mod normalizer;
pub mod metrics;

pub mod sources_extra;


use std::time::Duration;

/// Configuration for a data source
#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub url: String,
    pub poll_interval_secs: u64,
    pub timeout: Duration,
    pub retries: u32,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
            poll_interval_secs: 60,
            timeout: Duration::from_secs(30),
            retries: 3,
        }
    }
}


/// Raw reading from a data source before normalization
#[derive(Debug, Clone)]
pub struct RawReading {
    pub sensor_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate: f64,
    pub unit: String,
    pub source: String,
}
