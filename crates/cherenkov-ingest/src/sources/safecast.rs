use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, error};
use std::time::Duration;
use uuid::Uuid;

use cherenkov_db::{RadiationReading, QualityFlag};

use crate::pipeline::DataSource;

const SAFECAST_API_URL: &str = "https://api.safecast.org/measurements.json";

#[derive(Debug, Clone)]
pub struct SafecastSource {
    client: Client,
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct SafecastMeasurement {
    #[allow(dead_code)]
    id: u64,
    value: f64,
    unit: String,
    latitude: f64,
    longitude: f64,
    captured_at: String,
    device_id: Option<u64>,
    #[allow(dead_code)]
    location_name: Option<String>,
}


impl SafecastSource {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            api_key: std::env::var("SAFECAST_API_KEY").unwrap_or_default(),
        }
    }

    fn convert_to_usv(value: f64, unit: &str) -> f64 {
        match unit.to_lowercase().as_str() {
            "cpm" => value * 0.00294,
            "usv" => value,
            "msv" => value * 1000.0,
            _ => value * 0.00294,
        }
    }
}

#[async_trait]
impl DataSource for SafecastSource {
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        let mut request = self.client.get(SAFECAST_API_URL);
        
        if !self.api_key.is_empty() {
            request = request.header("X-API-KEY", &self.api_key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Fetch failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            error!("Safecast API returned error status: {}", status);
            return Err(anyhow::anyhow!("HTTP {}", status));
        }

        let measurements: Vec<SafecastMeasurement> = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

        let readings: Vec<RadiationReading> = measurements
            .into_iter()
            .filter_map(|m| {
                let timestamp = chrono::DateTime::parse_from_rfc3339(&m.captured_at)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc).timestamp())?;

                let usv = Self::convert_to_usv(m.value, &m.unit);
                let sensor_uuid = m.device_id
                    .map(|id| Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("safecast_{}", id).as_bytes()))
                    .unwrap_or_else(|| Uuid::new_v4());

                Some(RadiationReading {
                    sensor_id: sensor_uuid,
                    bucket: timestamp / 86400,
                    timestamp,
                    latitude: m.latitude,
                    longitude: m.longitude,
                    dose_rate_microsieverts: usv,
                    uncertainty: if usv > 10.0 { 0.5 } else { 0.1 },
                    quality_flag: if usv > 10.0 { QualityFlag::Suspect } else { QualityFlag::Valid },
                    source: "safecast".to_string(),
                    cell_id: format!("{:04x}", (m.latitude as i32 + 90) * 180 + (m.longitude as i32 + 180)),
                })
            })
            .collect();

        info!("Fetched {} readings from Safecast", readings.len());
        
        Ok(readings)
    }

    fn name(&self) -> String {
        "safecast".to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(300)
    }
}

impl Default for SafecastSource {
    fn default() -> Self {
        Self::new()
    }
}
