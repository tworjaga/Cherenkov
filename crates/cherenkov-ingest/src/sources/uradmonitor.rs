use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, error};
use std::time::Duration;
use uuid::Uuid;

use cherenkov_db::{RadiationReading, QualityFlag};

use crate::pipeline::DataSource;


const URADMONITOR_API_URL: &str = "https://data.uradmonitor.com/api/v1/devices";

#[derive(Debug, Clone)]
pub struct UradmonitorSource {
    client: Client,
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct UradDevice {
    id: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    device_type: String,
    latitude: f64,
    longitude: f64,
    #[serde(rename = "time")]
    last_seen: u64,
    #[serde(default)]
    radiation: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    temperature: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    pressure: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    humidity: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    voc: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    co2: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    pm25: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    pm10: Option<f64>,
}


impl UradmonitorSource {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            api_key: std::env::var("URADMONITOR_API_KEY").unwrap_or_default(),
        }
    }

    fn convert_to_usv(cpm: f64) -> f64 {
        cpm * 0.00294
    }
}


#[async_trait]
impl DataSource for UradmonitorSource {
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        let mut request = self.client.get(URADMONITOR_API_URL);
        
        if !self.api_key.is_empty() {
            request = request.header("X-API-KEY", &self.api_key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Fetch failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            error!("Uradmonitor API returned error status: {}", status);
            return Err(anyhow::anyhow!("HTTP {}", status));
        }

        let devices: Vec<UradDevice> = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

        let readings: Vec<RadiationReading> = devices
            .into_iter()
            .filter_map(|d| {
                let radiation_cpm = d.radiation?;
                let usv = Self::convert_to_usv(radiation_cpm);
                let timestamp = d.last_seen as i64;
                let sensor_uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("urad_{}", d.id).as_bytes());

                Some(RadiationReading {
                    sensor_id: sensor_uuid,
                    bucket: timestamp / 86400,
                    timestamp,
                    latitude: d.latitude,
                    longitude: d.longitude,
                    dose_rate_microsieverts: usv,
                    uncertainty: if usv > 10.0 { 0.5 } else { 0.1 },
                    quality_flag: if usv > 10.0 { QualityFlag::Suspect } else { QualityFlag::Valid },
                    source: "uradmonitor".to_string(),
                    cell_id: format!("{:04x}", (d.latitude as i32 + 90) * 180 + (d.longitude as i32 + 180)),
                })
            })
            .collect();

        info!("Fetched {} readings from Uradmonitor", readings.len());
        
        Ok(readings)
    }

    fn name(&self) -> String {
        "uradmonitor".to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(300)
    }
}


impl Default for UradmonitorSource {
    fn default() -> Self {
        Self::new()
    }
}
