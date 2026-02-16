use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use reqwest::Client;

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
    client: Client,
}

impl DataSource {
    pub fn safecast() -> Self {
        Self {
            config: SourceConfig {
                name: "safecast",
                url: "https://api.safecast.org/measurements.json",
                poll_interval_secs: 60,
            },
            client: Client::new(),
        }
    }
    
    pub fn uradmonitor() -> Self {
        Self {
            config: SourceConfig {
                name: "uradmonitor",
                url: "https://data.uradmonitor.com/api/v1/devices",
                poll_interval_secs: 30,
            },
            client: Client::new(),
        }
    }
    
    pub fn epa_radnet() -> Self {
        Self {
            config: SourceConfig {
                name: "epa_radnet",
                url: "https://www.epa.gov/radnet",
                poll_interval_secs: 300,
            },
            client: Client::new(),
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
        match self.config.name {
            "safecast" => self.fetch_safecast().await,
            "uradmonitor" => self.fetch_uradmonitor().await,
            "epa_radnet" => self.fetch_epa_radnet().await,
            _ => Ok(vec![]),
        }
    }

    async fn fetch_safecast(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        #[derive(Deserialize)]
        struct SafecastMeasurement {
            id: u64,
            captured_at: String,
            latitude: f64,
            longitude: f64,
            value: f64,
            unit: String,
            device_id: String,
        }

        let response = self.client
            .get(self.config.url)
            .query(&[("limit", "100")])
            .send()
            .await?;

        let measurements: Vec<SafecastMeasurement> = response.json().await?;

        let readings = measurements.into_iter()
            .filter_map(|m| {
                let timestamp = chrono::DateTime::parse_from_rfc3339(&m.captured_at)
                    .ok()?
                    .with_timezone(&chrono::Utc);

                let dose_rate = match m.unit.as_str() {
                    "cpm" => m.value * 0.0057,
                    "usv" => m.value,
                    _ => m.value,
                };

                Some(RawReading {
                    sensor_id: format!("safecast-{}", m.device_id),
                    timestamp,
                    latitude: m.latitude,
                    longitude: m.longitude,
                    dose_rate,
                    unit: "μSv/h".to_string(),
                    source: "safecast".to_string(),
                })
            })
            .collect();

        Ok(readings)
    }

    async fn fetch_uradmonitor(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        #[derive(Deserialize)]
        struct UradDevice {
            id: String,
            latitude: f64,
            longitude: f64,
            radiation: f64,
            timestamp: String,
        }

        let response = self.client
            .get(self.config.url)
            .send()
            .await?;

        let devices: Vec<UradDevice> = response.json().await?;

        let readings = devices.into_iter()
            .filter_map(|d| {
                let timestamp = chrono::DateTime::parse_from_rfc3339(&d.timestamp)
                    .ok()?
                    .with_timezone(&chrono::Utc);

                Some(RawReading {
                    sensor_id: format!("uradmonitor-{}", d.id),
                    timestamp,
                    latitude: d.latitude,
                    longitude: d.longitude,
                    dose_rate: d.radiation,
                    unit: "μSv/h".to_string(),
                    source: "uradmonitor".to_string(),
                })
            })
            .collect();

        Ok(readings)
    }

    async fn fetch_epa_radnet(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        Ok(vec![])
    }
}
