use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn, instrument};
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use crate::pipeline::DataSource;
use crate::SourceConfig;
use chrono::{Utc, NaiveDateTime};



/// NASA FIRMS (Fire Information for Resource Management System) source
/// 
/// Fetches active fire data from MODIS and VIIRS satellites.
/// Fire data can correlate with radiation releases from nuclear incidents.
pub struct NasaFirmsSource {
    client: Client,
    config: SourceConfig,
    api_key: String,
}

/// FIRMS fire detection record
#[derive(Debug, Clone)]
pub struct FirmsFire {
    pub latitude: f64,
    pub longitude: f64,
    pub brightness: f64,
    pub scan: f64,
    pub track: f64,
    pub acq_date: String,
    pub acq_time: String,
    pub satellite: String,
    pub confidence: String,
    pub version: String,
    pub bright_t31: f64,
    pub frp: f64,
    pub daynight: String,
}

impl NasaFirmsSource {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "nasa_firms".to_string(),
                url: "https://firms.modaps.eosdis.nasa.gov/api/area/csv".to_string(),
                poll_interval_secs: 1800,
                timeout: Duration::from_secs(30),
                retries: 3,
            },
            api_key,
        }
    }


    /// Parse FIRMS CSV format
    fn parse_firms_csv(&self, csv_data: &str) -> anyhow::Result<Vec<FirmsFire>> {
        let mut fires = Vec::new();
        let mut lines = csv_data.lines();
        
        // Parse header
        let header = lines.next()
            .ok_or_else(|| anyhow::anyhow!("Empty CSV data"))?;
        let headers: Vec<&str> = header.split(',').collect();
        
        // Find column indices
        let lat_idx = headers.iter().position(|&h| h == "latitude");
        let lon_idx = headers.iter().position(|&h| h == "longitude");
        let bright_idx = headers.iter().position(|&h| h == "brightness");
        let scan_idx = headers.iter().position(|&h| h == "scan");
        let track_idx = headers.iter().position(|&h| h == "track");
        let date_idx = headers.iter().position(|&h| h == "acq_date");
        let time_idx = headers.iter().position(|&h| h == "acq_time");
        let sat_idx = headers.iter().position(|&h| h == "satellite");
        let conf_idx = headers.iter().position(|&h| h == "confidence");
        let ver_idx = headers.iter().position(|&h| h == "version");
        let t31_idx = headers.iter().position(|&h| h == "bright_t31");
        let frp_idx = headers.iter().position(|&h| h == "frp");
        let dn_idx = headers.iter().position(|&h| h == "daynight");
        
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 10 {
                continue;
            }
            
            let fire = FirmsFire {
                latitude: lat_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                longitude: lon_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                brightness: bright_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                scan: scan_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                track: track_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                acq_date: date_idx.and_then(|i| fields.get(i)).unwrap_or(&"").to_string(),
                acq_time: time_idx.and_then(|i| fields.get(i)).unwrap_or(&"").to_string(),
                satellite: sat_idx.and_then(|i| fields.get(i)).unwrap_or(&"Unknown").to_string(),
                confidence: conf_idx.and_then(|i| fields.get(i)).unwrap_or(&"n").to_string(),
                version: ver_idx.and_then(|i| fields.get(i)).unwrap_or(&"").to_string(),
                bright_t31: t31_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                frp: frp_idx.and_then(|i| fields.get(i)).and_then(|f| f.parse().ok()).unwrap_or(0.0),
                daynight: dn_idx.and_then(|i| fields.get(i)).unwrap_or(&"D").to_string(),
            };
            
            fires.push(fire);
        }
        
        Ok(fires)
    }

    /// Convert fire brightness to approximate radiation proxy
    /// High temperature events may indicate nuclear incidents
    fn fire_to_radiation_proxy(&self, fire: &FirmsFire) -> Option<RadiationReading> {
        // Only high-confidence fires
        let confidence_val = match fire.confidence.as_str() {
            "h" | "high" => 90,
            "n" | "nominal" => 60,
            "l" | "low" => 30,
            _ => 50,
        };
        
        if confidence_val < 50 {
            return None;
        }
        
        // Parse acquisition datetime
        let datetime_str = format!("{} {}", fire.acq_date, fire.acq_time);
        let timestamp = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H%M")
            .ok()
            .map(|dt| dt.and_local_timezone(Utc).single())
            .flatten()
            .unwrap_or_else(Utc::now);
        
        // Create proxy reading based on fire radiative power
        // FRP > 1000 MW may indicate significant thermal anomaly
        let proxy_dose = if fire.frp > 1000.0 {
            fire.frp / 10000.0 // Proxy conversion
        } else {
            fire.frp / 50000.0
        };
        
        let sensor_id = format!("firms-{}-{}-{:.4}-{:.4}", 
            fire.satellite.to_lowercase(),
            fire.acq_date.replace("-", ""),
            fire.latitude,
            fire.longitude
        );
        
        Some(RadiationReading {
            sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
            bucket: timestamp.timestamp() / 3600,
            timestamp: timestamp.timestamp(),
            latitude: fire.latitude,
            longitude: fire.longitude,
            dose_rate_microsieverts: proxy_dose,
            uncertainty: 0.5,
            quality_flag: QualityFlag::Suspect,

            source: "nasa_firms".to_string(),
            cell_id: format!("{:.2},{:.2}", fire.latitude, fire.longitude),
        })
    }
}

#[async_trait]
impl DataSource for NasaFirmsSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // Fetch global fire data for last 24 hours
        // Using MODIS NRT (Near Real Time) data
        let url = format!(
            "{}/{}/MODIS_NRT/-180,-90,180,90/1/{}",
            self.config.url,
            self.api_key,
            chrono::Local::now().format("%Y-%m-%d")
        );
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Cherenkov/1.0 (Radiation Monitoring)")
            .send()
            .await;

        let csv_data = match response {
            Ok(resp) if resp.status().is_success() => resp.text().await?,
            Ok(resp) => {
                warn!("NASA FIRMS returned {} - trying VIIRS", resp.status());
                // Fallback to VIIRS
                let viirs_url = format!(
                    "{}/{}/VIIRS_NOAA20_NRT/-180,-90,180,90/1/{}",
                    self.config.url,
                    self.api_key,
                    chrono::Local::now().format("%Y-%m-%d")
                );
                
                let viirs_resp = self.client.get(&viirs_url).send().await;
                match viirs_resp {
                    Ok(r) if r.status().is_success() => r.text().await?,
                    _ => return Ok(vec![]),
                }
            }
            Err(e) => {
                warn!("Failed to fetch NASA FIRMS: {}", e);
                return Ok(vec![]);
            }
        };

        if csv_data.trim().is_empty() {
            warn!("NASA FIRMS returned empty data");
            return Ok(vec![]);
        }

        let fires = self.parse_firms_csv(&csv_data)?;
        info!("Fetched {} fire detections from NASA FIRMS", fires.len());
        
        // Convert fires to radiation proxy readings
        let readings: Vec<RadiationReading> = fires
            .iter()
            .filter_map(|f| self.fire_to_radiation_proxy(f))
            .collect();
        
        info!("Converted {} fires to radiation proxy readings", readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "nasa_firms")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}
