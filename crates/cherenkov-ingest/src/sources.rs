use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn, instrument};
use cherenkov_db::RadiationReading;
use uuid::Uuid;

use crate::pipeline::DataSource;

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

/// Safecast radiation monitoring source
pub struct SafecastSource {
    client: Client,
    config: SourceConfig,
}

impl SafecastSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "safecast",
                url: "https://api.safecast.org/measurements.json",
                poll_interval_secs: 60,
            },
        }
    }
}

#[async_trait]
impl DataSource for SafecastSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        #[derive(Deserialize, Debug)]
        struct SafecastMeasurement {
            id: u64,
            captured_at: String,
            latitude: f64,
            longitude: f64,
            value: f64,
            unit: String,
            device_id: Option<u64>,
        }

        let response = self.client
            .get(self.config.url)
            .query(&[("limit", "100")])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Safecast API returned {}", response.status()));
        }

        let measurements: Vec<SafecastMeasurement> = response.json().await?;
        info!("Fetched {} measurements from Safecast", measurements.len());

        let readings: Vec<RadiationReading> = measurements
            .into_iter()
            .filter_map(|m| {
                let timestamp = chrono::DateTime::parse_from_rfc3339(&m.captured_at)
                    .ok()?
                    .with_timezone(&chrono::Utc);

                // Convert CPM to μSv/h if needed
                // Safecast uses 0.0057 μSv/h per CPM for their sensors
                let dose_rate = match m.unit.to_lowercase().as_str() {
                    "cpm" => m.value * 0.0057,
                    "usv" | "μsv" | "microsieverts" => m.value,
                    _ => {
                        warn!("Unknown unit '{}', assuming μSv/h", m.unit);
                        m.value
                    }
                };

                let device_id = m.device_id.unwrap_or(m.id);
                let sensor_id = format!("safecast-{}", device_id);

                Some(RadiationReading {
                    sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                    bucket: timestamp.timestamp() / 3600,
                    timestamp: timestamp.timestamp(),
                    latitude: m.latitude,
                    longitude: m.longitude,
                    dose_rate_microsieverts: dose_rate,
                    uncertainty: 0.1,
                    quality_flag: cherenkov_db::QualityFlag::Valid,
                    source: "safecast".to_string(),
                    cell_id: format!("{:.2},{:.2}", m.latitude, m.longitude),
                })
            })
            .collect();

        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "safecast")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}

/// uRADMonitor radiation monitoring source
pub struct UradmonitorSource {
    client: Client,
    config: SourceConfig,
}

impl UradmonitorSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "uradmonitor",
                url: "https://data.uradmonitor.com/api/v1/devices",
                poll_interval_secs: 30,
            },
        }
    }
}

#[async_trait]
impl DataSource for UradmonitorSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        #[derive(Deserialize, Debug)]
        struct UradDevice {
            id: String,
            latitude: f64,
            longitude: f64,
            #[serde(rename = "cpm")]
            radiation: Option<f64>,
            #[serde(rename = "dose")]
            dose_rate: Option<f64>,
            timestamp: String,
            status: Option<String>,
        }

        let response = self.client
            .get(self.config.url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("uRADMonitor API returned {}", response.status()));
        }

        let devices: Vec<UradDevice> = response.json().await?;
        info!("Fetched {} devices from uRADMonitor", devices.len());

        let readings: Vec<RadiationReading> = devices
            .into_iter()
            .filter_map(|d| {
                // Skip offline devices
                if let Some(ref status) = d.status {
                    if status == "offline" {
                        return None;
                    }
                }

                let timestamp = chrono::DateTime::parse_from_rfc3339(&d.timestamp)
                    .ok()?
                    .with_timezone(&chrono::Utc);

                // Use dose_rate if available, otherwise convert from CPM
                let dose_rate = d.dose_rate.unwrap_or_else(|| {
                    d.radiation.map(|cpm| cpm * 0.0057).unwrap_or(0.0)
                });

                if dose_rate <= 0.0 {
                    return None;
                }

                let sensor_id = format!("uradmonitor-{}", d.id);

                Some(RadiationReading {
                    sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                    bucket: timestamp.timestamp() / 3600,
                    timestamp: timestamp.timestamp(),
                    latitude: d.latitude,
                    longitude: d.longitude,
                    dose_rate_microsieverts: dose_rate,
                    uncertainty: 0.15,
                    quality_flag: cherenkov_db::QualityFlag::Valid,
                    source: "uradmonitor".to_string(),
                    cell_id: format!("{:.2},{:.2}", d.latitude, d.longitude),
                })
            })
            .collect();

        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "uradmonitor")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}

/// EPA RadNet radiation monitoring source
pub struct EpaRadnetSource {
    client: Client,
    config: SourceConfig,
}

impl EpaRadnetSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "epa_radnet",
                url: "https://www.epa.gov/radnet/radnet-data",
                poll_interval_secs: 300,
            },
        }
    }
}

#[async_trait]
impl DataSource for EpaRadnetSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // EPA RadNet requires parsing HTML or using their data files
        // This is a stub implementation
        warn!("EPA RadNet source not fully implemented - requires HTML parsing");
        Ok(vec![])
    }
}

/// OpenAQ air quality correlation source
pub struct OpenAqSource {
    client: Client,
    config: SourceConfig,
}

impl OpenAqSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "openaq",
                url: "https://api.openaq.org/v2/latest",
                poll_interval_secs: 300,
            },
        }
    }
}

#[async_trait]
impl DataSource for OpenAqSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // OpenAQ provides air quality data that can correlate with radiation transport
        // This is a stub for future implementation
        Ok(vec![])
    }
}

/// Open-Meteo weather data source for plume modeling
pub struct OpenMeteoSource {
    client: Client,
    config: SourceConfig,
}

impl OpenMeteoSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "openmeteo",
                url: "https://api.open-meteo.com/v1/forecast",
                poll_interval_secs: 3600,
            },
        }
    }
}

#[async_trait]
impl DataSource for OpenMeteoSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // Weather data for plume modeling
        // This is a stub for future implementation
        Ok(vec![])
    }
}
