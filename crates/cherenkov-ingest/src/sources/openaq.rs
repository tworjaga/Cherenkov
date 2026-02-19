use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use crate::pipeline::DataSource;
use crate::SourceConfig;


use chrono::{DateTime, Utc};
use serde::Deserialize;

/// OpenAQ air quality data source
/// 
/// Fetches air quality measurements from the OpenAQ API.
/// Air quality data provides correlation with radiation transport patterns.
pub struct OpenAqSource {
    client: Client,
    config: SourceConfig,
}

/// OpenAQ API response
#[derive(Debug, Deserialize)]
struct OpenAqResponse {
    results: Vec<OpenAqMeasurement>,
}

/// OpenAQ measurement record
#[derive(Debug, Deserialize)]
struct OpenAqMeasurement {
    location: String,
    #[allow(dead_code)]
    city: Option<String>,
    #[allow(dead_code)]
    country: Option<String>,
    coordinates: Option<OpenAqCoordinates>,
    measurements: Vec<OpenAqParameter>,
    #[serde(rename = "date")]
    timestamp: OpenAqTimestamp,
}


/// OpenAQ coordinates
#[derive(Debug, Deserialize)]
struct OpenAqCoordinates {
    latitude: f64,
    longitude: f64,
}

/// OpenAQ parameter measurement
#[derive(Debug, Deserialize)]
struct OpenAqParameter {
    parameter: String,
    value: f64,
    #[allow(dead_code)]
    unit: String,
}


/// OpenAQ timestamp
#[derive(Debug, Deserialize)]
struct OpenAqTimestamp {
    utc: String,
}

/// Air quality reading with radiation correlation
#[derive(Debug, Clone)]
pub struct AirQualityReading {
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub pm25: Option<f64>,
    pub pm10: Option<f64>,
    pub o3: Option<f64>,
    pub no2: Option<f64>,
    pub so2: Option<f64>,
    pub co: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

impl OpenAqSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "openaq".to_string(),
                url: "https://api.openaq.org/v2/latest".to_string(),
                poll_interval_secs: 1800,
                timeout: Duration::from_secs(30),
                retries: 3,
            },
        }
    }


    /// Parse OpenAQ measurement into structured air quality reading
    fn parse_measurement(&self, measurement: &OpenAqMeasurement) -> Option<AirQualityReading> {
        let coords = measurement.coordinates.as_ref()?;
        
        let timestamp = chrono::DateTime::parse_from_rfc3339(&measurement.timestamp.utc)
            .ok()?
            .with_timezone(&Utc);
        
        let mut reading = AirQualityReading {
            location: measurement.location.clone(),
            latitude: coords.latitude,
            longitude: coords.longitude,
            pm25: None,
            pm10: None,
            o3: None,
            no2: None,
            so2: None,
            co: None,
            timestamp,
        };
        
        // Extract parameters
        for param in &measurement.measurements {
            match param.parameter.to_lowercase().as_str() {
                "pm25" => reading.pm25 = Some(param.value),
                "pm10" => reading.pm10 = Some(param.value),
                "o3" => reading.o3 = Some(param.value),
                "no2" => reading.no2 = Some(param.value),
                "so2" => reading.so2 = Some(param.value),
                "co" => reading.co = Some(param.value),
                _ => {}
            }
        }
        
        Some(reading)
    }

    /// Convert air quality reading to radiation proxy
    /// 
    /// High particulate matter may correlate with atmospheric conditions
    /// that affect radiation dispersion
    fn air_quality_to_radiation_proxy(&self, aq: &AirQualityReading) -> Option<RadiationReading> {
        // Only create readings for significant air quality events
        let pm25 = aq.pm25.unwrap_or(0.0);
        let pm10 = aq.pm10.unwrap_or(0.0);
        
        // High particulate matter (> 100 μg/m³) may indicate conditions
        // favorable for radiation particle transport
        if pm25 < 100.0 && pm10 < 150.0 {
            return None;
        }
        
        let sensor_id = format!("openaq-{}-{}", 
            aq.location.to_lowercase().replace(" ", "_"),
            aq.timestamp.format("%Y%m%d%H")
        );
        
        // Create proxy reading based on particulate matter
        // Higher PM = higher proxy value for transport modeling
        let proxy_dose = (pm25.max(pm10)) / 1000.0;
        
        Some(RadiationReading {
            sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
            bucket: aq.timestamp.timestamp() / 3600,
            timestamp: aq.timestamp.timestamp(),
            latitude: aq.latitude,
            longitude: aq.longitude,
            dose_rate_microsieverts: proxy_dose,
            uncertainty: 0.4,
            quality_flag: QualityFlag::Suspect,

            source: "openaq".to_string(),
            cell_id: format!("{:.2},{:.2}", aq.latitude, aq.longitude),
        })
    }

    /// Log significant air quality events
    fn log_significant_events(&self, aq: &AirQualityReading) {
        if let Some(pm25) = aq.pm25 {
            if pm25 > 100.0 {
                warn!("High PM2.5 at {}: {:.1} μg/m³", aq.location, pm25);
            }
        }
        
        if let Some(pm10) = aq.pm10 {
            if pm10 > 150.0 {
                warn!("High PM10 at {}: {:.1} μg/m³", aq.location, pm10);
            }
        }
        
        if let Some(o3) = aq.o3 {
            if o3 > 200.0 {
                warn!("High O3 at {}: {:.1} ppb", aq.location, o3);
            }
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

    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // Fetch latest measurements from OpenAQ
        let url = self.config.url.clone();
        let response = self.client
            .get(&url)
            .query(&[

                ("limit", "1000"),
                ("sort", "desc"),
            ])
            .header("User-Agent", "Cherenkov/1.0 (Radiation Monitoring)")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAQ API returned {}", response.status()));
        }

        let data: OpenAqResponse = response.json().await?;
        info!("Fetched {} locations from OpenAQ", data.results.len());
        
        // Parse measurements
        let mut air_quality_readings = Vec::new();
        let mut radiation_proxies = Vec::new();
        
        for measurement in &data.results {
            if let Some(aq) = self.parse_measurement(measurement) {
                self.log_significant_events(&aq);
                air_quality_readings.push(aq.clone());
                
                // Convert to radiation proxy if significant
                if let Some(proxy) = self.air_quality_to_radiation_proxy(&aq) {
                    radiation_proxies.push(proxy);
                }
            }
        }
        
        info!("Parsed {} air quality readings, {} radiation proxies", 
            air_quality_readings.len(), 
            radiation_proxies.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "openaq")
            .increment(radiation_proxies.len() as u64);

        Ok(radiation_proxies)
    }
}
