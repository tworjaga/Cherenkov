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
    
    pub fn eurdep() -> Self {
        Self {
            config: SourceConfig {
                name: "eurdep",
                url: "https://eurdep.jrc.ec.europa.eu/eurdep/services/getLastMeasurements",
                poll_interval_secs: 600,
            },
            client: Client::new(),
        }
    }
    
    pub fn iaea_pris() -> Self {
        Self {
            config: SourceConfig {
                name: "iaea_pris",
                url: "https://pris.iaea.org/PRIS/home.aspx",
                poll_interval_secs: 86400,
            },
            client: Client::new(),
        }
    }
    
    pub fn usgs_seismic() -> Self {
        Self {
            config: SourceConfig {
                name: "usgs_seismic",
                url: "https://earthquake.usgs.gov/fdsnws/event/1/query",
                poll_interval_secs: 60,
            },
            client: Client::new(),
        }
    }
    
    pub fn nasa_firms() -> Self {
        Self {
            config: SourceConfig {
                name: "nasa_firms",
                url: "https://firms.modaps.eosdis.nasa.gov/api/area/csv/VIIRS_NOAA20_NRT",
                poll_interval_secs: 300,
            },
            client: Client::new(),
        }
    }
    
    pub fn noaa_gfs() -> Self {
        Self {
            config: SourceConfig {
                name: "noaa_gfs",
                url: "https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl",
                poll_interval_secs: 21600,
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
            "eurdep" => self.fetch_eurdep().await,
            "iaea_pris" => self.fetch_iaea_pris().await,
            "usgs_seismic" => self.fetch_usgs_seismic().await,
            "nasa_firms" => self.fetch_nasa_firms().await,
            "noaa_gfs" => self.fetch_noaa_gfs().await,
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
    
    async fn fetch_eurdep(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        #[derive(Deserialize)]
        struct EurdepMeasurement {
            station_id: String,
            timestamp: String,
            latitude: f64,
            longitude: f64,
            dose_rate: f64,
        }

        let response = self.client
            .get(self.config.url)
            .send()
            .await?;

        let measurements: Vec<EurdepMeasurement> = response.json().await?;

        let readings = measurements.into_iter()
            .filter_map(|m| {
                let timestamp = chrono::DateTime::parse_from_rfc3339(&m.timestamp)
                    .ok()?
                    .with_timezone(&chrono::Utc);

                Some(RawReading {
                    sensor_id: format!("eurdep-{}", m.station_id),
                    timestamp,
                    latitude: m.latitude,
                    longitude: m.longitude,
                    dose_rate: m.dose_rate,
                    unit: "μSv/h".to_string(),
                    source: "eurdep".to_string(),
                })
            })
            .collect();

        Ok(readings)
    }
    
    async fn fetch_iaea_pris(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        #[derive(Deserialize)]
        struct ReactorStatus {
            plant_id: String,
            plant_name: String,
            latitude: f64,
            longitude: f64,
            status: String,
            power_output: f64,
            last_updated: String,
        }

        let response = self.client
            .get(self.config.url)
            .send()
            .await?;

        let reactors: Vec<ReactorStatus> = response.json().await?;

        let readings = reactors.into_iter()
            .filter_map(|r| {
                let timestamp = chrono::DateTime::parse_from_rfc3339(&r.last_updated)
                    .ok()?
                    .with_timezone(&chrono::Utc);

                Some(RawReading {
                    sensor_id: format!("iaea-pris-{}", r.plant_id),
                    timestamp,
                    latitude: r.latitude,
                    longitude: r.longitude,
                    dose_rate: 0.0,
                    unit: "status".to_string(),
                    source: "iaea_pris".to_string(),
                })
            })
            .collect();

        Ok(readings)
    }
    
    async fn fetch_usgs_seismic(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        #[derive(Deserialize)]
        struct Earthquake {
            id: String,
            geometry: Geometry,
            properties: Properties,
        }

        #[derive(Deserialize)]
        struct Geometry {
            coordinates: Vec<f64>,
        }

        #[derive(Deserialize)]
        struct Properties {
            mag: f64,
            place: String,
            time: i64,
        }

        let response = self.client
            .get(self.config.url)
            .query(&[
                ("format", "geojson"),
                ("minmagnitude", "4.0"),
                ("limit", "100"),
            ])
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        let features = data["features"].as_array().unwrap_or(&vec![]).clone();

        let readings = features.into_iter()
            .filter_map(|f| {
                let id = f["id"].as_str()?.to_string();
                let coords = f["geometry"]["coordinates"].as_array()?;
                let lon = coords.get(0)?.as_f64()?;
                let lat = coords.get(1)?.as_f64()?;
                let mag = f["properties"]["mag"].as_f64()?;
                let time_ms = f["properties"]["time"].as_i64()?;
                let timestamp = chrono::DateTime::from_timestamp_millis(time_ms)?
                    .with_timezone(&chrono::Utc);

                Some(RawReading {
                    sensor_id: format!("usgs-seismic-{}", id),
                    timestamp,
                    latitude: lat,
                    longitude: lon,
                    dose_rate: mag,
                    unit: "magnitude".to_string(),
                    source: "usgs_seismic".to_string(),
                })
            })
            .collect();

        Ok(readings)
    }
    
    async fn fetch_nasa_firms(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        #[derive(Deserialize)]
        struct ThermalAnomaly {
            latitude: f64,
            longitude: f64,
            brightness: f64,
            acq_date: String,
            acq_time: String,
        }

        let response = self.client
            .get(self.config.url)
            .send()
            .await?;

        let anomalies: Vec<ThermalAnomaly> = response.json().await?;

        let readings = anomalies.into_iter()
            .filter_map(|a| {
                let datetime_str = format!("{} {}", a.acq_date, a.acq_time);
                let timestamp = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H%M")
                    .ok()?
                    .and_local_timezone(chrono::Utc)
                    .single()?;

                Some(RawReading {
                    sensor_id: format!("nasa-firms-{}-{}", a.latitude, a.longitude),
                    timestamp,
                    latitude: a.latitude,
                    longitude: a.longitude,
                    dose_rate: a.brightness,
                    unit: "kelvin".to_string(),
                    source: "nasa_firms".to_string(),
                })
            })
            .collect();

        Ok(readings)
    }
    
    async fn fetch_noaa_gfs(&self) -> Result<Vec<RawReading>, reqwest::Error> {
        Ok(vec![])
    }
}
