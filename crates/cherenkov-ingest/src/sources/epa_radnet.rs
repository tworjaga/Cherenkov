use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::info;
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use crate::pipeline::DataSource;
use crate::SourceConfig;
use scraper::{Html, Selector};
use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};


/// EPA RadNet radiation monitoring source
/// 
/// Fetches radiation monitoring data from EPA RadNet via HTML scraping.
/// RadNet provides near real-time environmental radiation monitoring data
/// from stations across the United States.
pub struct EpaRadnetSource {
    client: Client,
    config: SourceConfig,
}

/// EPA station information
#[derive(Debug, Clone)]
pub struct EpaStation {
    pub station_id: String,
    pub station_name: String,
    pub city: String,
    pub state: String,
    pub latitude: f64,
    pub longitude: f64,
    pub readings: Vec<EpaReading>,
}

/// EPA radiation reading
#[derive(Debug, Clone)]
pub struct EpaReading {
    pub datetime: DateTime<Utc>,
    pub gamma_gross_count_rate: f64,
    pub gamma_energy: f64,
    pub beta_gross_count_rate: Option<f64>,
    pub unit: String,
    pub quality_flag: String,
}

impl EpaRadnetSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "epa_radnet".to_string(),
                url: "https://www.epa.gov/radnet/radnet-data".to_string(),
                poll_interval_secs: 900,
                timeout: Duration::from_secs(30),
                retries: 3,
            },
        }
    }


    /// Parse EPA location text format: "City, State (Lat, Lon)"
    fn parse_epa_location(&self, text: &str) -> anyhow::Result<(String, String, f64, f64)> {
        let re = regex::Regex::new(r"([\w\s]+),\s*(\w{2})\s*\(([-\d.]+),\s*([-\d.]+)\)")?;
        
        let caps = re.captures(text)
            .ok_or_else(|| anyhow::anyhow!("Invalid EPA location format: {}", text))?;
            
        Ok((
            caps[1].trim().to_string(),
            caps[2].to_string(),
            caps[3].parse()?,
            caps[4].parse()?,
        ))
    }

    /// Parse EPA reading value from text like "8.2 μR/h" or "< 8.2"
    fn parse_epa_reading(&self, text: &str) -> anyhow::Result<f64> {
        let re = regex::Regex::new(r"<?\s*([\d.]+)")?;
        
        let caps = re.captures(text)
            .ok_or_else(|| anyhow::anyhow!("Invalid EPA reading format: {}", text))?;
            
        Ok(caps[1].parse()?)
    }

    /// Parse EPA datetime format: "01/15/2024 14:30" (Eastern Time)
    fn parse_epa_datetime(&self, text: &str) -> anyhow::Result<DateTime<Utc>> {
        let naive = NaiveDateTime::parse_from_str(text, "%m/%d/%Y %H:%M")?;
        let eastern = chrono_tz::America::New_York;
        let dt = eastern.from_local_datetime(&naive).single()
            .ok_or_else(|| anyhow::anyhow!("Invalid datetime"))?;
        Ok(dt.with_timezone(&Utc))
    }

    /// Approximate coordinates for US cities (fallback)
    fn approximate_coordinates(&self, city: &str, state: &str) -> (f64, f64) {
        let coords = match (city.to_lowercase().as_str(), state.to_lowercase().as_str()) {
            ("new york", "ny") => (40.7128, -74.0060),
            ("los angeles", "ca") => (34.0522, -118.2437),
            ("chicago", "il") => (41.8781, -87.6298),
            ("houston", "tx") => (29.7604, -95.3698),
            ("phoenix", "az") => (33.4484, -112.0740),
            ("philadelphia", "pa") => (39.9526, -75.1652),
            ("san antonio", "tx") => (29.4241, -98.4936),
            ("san diego", "ca") => (32.7157, -117.1611),
            ("dallas", "tx") => (32.7767, -96.7970),
            ("san jose", "ca") => (37.3382, -121.8863),
            ("austin", "tx") => (30.2672, -97.7431),
            ("jacksonville", "fl") => (30.3322, -81.6557),
            ("san francisco", "ca") => (37.7749, -122.4194),
            ("columbus", "oh") => (39.9612, -82.9988),
            ("charlotte", "nc") => (35.2271, -80.8431),
            ("fort worth", "tx") => (32.7555, -97.3308),
            ("indianapolis", "in") => (39.7684, -86.1581),
            ("seattle", "wa") => (47.6062, -122.3321),
            ("denver", "co") => (39.7392, -104.9903),
            ("washington", "dc") => (38.9072, -77.0369),
            ("boston", "ma") => (42.3601, -71.0589),
            ("el paso", "tx") => (31.7619, -106.4850),
            ("nashville", "tn") => (36.1627, -86.7816),
            ("detroit", "mi") => (42.3314, -83.0458),
            ("oklahoma city", "ok") => (35.4676, -97.5164),
            ("portland", "or") => (45.5152, -122.6784),
            ("las vegas", "nv") => (36.1699, -115.1398),
            ("louisville", "ky") => (38.2527, -85.7585),
            ("baltimore", "md") => (39.2904, -76.6122),
            ("milwaukee", "wi") => (43.0389, -87.9065),
            ("albuquerque", "nm") => (35.0844, -106.6504),
            ("tucson", "az") => (32.2226, -110.9747),
            ("fresno", "ca") => (36.7378, -119.7871),
            ("sacramento", "ca") => (38.5816, -121.4944),
            ("kansas city", "mo") => (39.0997, -94.5786),
            ("mesa", "az") => (33.4152, -111.8315),
            ("atlanta", "ga") => (33.7490, -84.3880),
            ("omaha", "ne") => (41.2565, -95.9345),
            ("colorado springs", "co") => (38.8339, -104.8214),
            ("raleigh", "nc") => (35.7796, -78.6382),
            ("miami", "fl") => (25.7617, -80.1918),
            ("virginia beach", "va") => (36.8529, -75.9780),
            ("oakland", "ca") => (37.8044, -122.2712),
            ("minneapolis", "mn") => (44.9778, -93.2650),
            ("tulsa", "ok") => (36.1540, -95.9928),
            ("arlington", "tx") => (32.7357, -97.1081),
            ("wichita", "ks") => (37.6872, -97.3301),
            ("bakersfield", "ca") => (35.3733, -119.0187),
            ("tampa", "fl") => (27.9506, -82.4572),
            ("anaheim", "ca") => (33.8366, -117.9143),
            ("honolulu", "hi") => (21.3069, -157.8583),
            ("anchorage", "ak") => (61.2181, -149.9003),
            _ => (39.8283, -98.5795),
        };
        coords
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

    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        let url = "https://www.epa.gov/radnet/radnet-data";
        
        let response = self.client
            .get(url)
            .header("User-Agent", "Cherenkov/1.0 (Radiation Monitoring)")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("EPA RadNet returned {}", response.status()));
        }

        let html = response.text().await?;
        
        // Parse HTML synchronously to avoid Send issues with scraper::Html
        let readings = self.parse_html_sync(&html);
        
        // Fallback: Try CSV endpoint if HTML scraping returns no data
        let mut final_readings = readings;
        if final_readings.is_empty() {
            let csv_url = "https://www.epa.gov/sites/default/files/2023-06/radnet-near-real-time-data.csv";
            let csv_response = self.client.get(csv_url).send().await;
            
            if let Ok(resp) = csv_response {
                if resp.status().is_success() {
                    let csv_data = resp.text().await?;
                    let csv_readings = self.parse_radnet_csv(&csv_data)?;
                    final_readings.extend(csv_readings);
                }
            }
        }
        
        info!("Fetched {} readings from EPA RadNet", final_readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "epa_radnet")
            .increment(final_readings.len() as u64);

        Ok(final_readings)
    }
}

impl EpaRadnetSource {
    /// Parse HTML data synchronously to avoid Send issues
    fn parse_html_sync(&self, html: &str) -> Vec<RadiationReading> {
        let document = Html::parse_document(html);
        
        // Try to find data table
        let table_selector = match Selector::parse("table.radnet-data-table tbody tr") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let station_selector = match Selector::parse("td.station-name") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let location_selector = match Selector::parse("td.location") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let reading_selector = match Selector::parse("td.gamma-reading") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let time_selector = match Selector::parse("td.sample-time") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        
        let mut readings = Vec::new();
        
        for row in document.select(&table_selector) {
            let station_name = row.select(&station_selector)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
                
            let location_text = row.select(&location_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
                
            let (city, _state, lat, lon) = match self.parse_epa_location(&location_text) {
                Ok(coords) => coords,
                Err(_) => {
                    let (c, s) = location_text.split_once(',').unwrap_or(("Unknown", "US"));
                    let (la, lo) = self.approximate_coordinates(c.trim(), s.trim());
                    (c.trim().to_string(), s.trim().to_string(), la, lo)
                }
            };
            
            let gamma_text = row.select(&reading_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
                
            let gamma_value = match self.parse_epa_reading(&gamma_text) {
                Ok(v) => v,
                Err(_) => continue,
            };
            
            let time_text = row.select(&time_selector)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
                
            let timestamp = match self.parse_epa_datetime(&time_text) {
                Ok(t) => t,
                Err(_) => Utc::now(),
            };
            
            // Convert μR/h to μSv/h (1 μR ≈ 0.00877 μSv)
            let dose_rate = gamma_value * 0.00877;
            
            let sensor_id = format!("epa:{}", station_name.to_lowercase().replace(" ", "_"));
            
            readings.push(RadiationReading {
                sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                bucket: timestamp.timestamp() / 3600,
                timestamp: timestamp.timestamp(),
                latitude: lat,
                longitude: lon,
                dose_rate_microsieverts: dose_rate,
                uncertainty: (dose_rate * 0.1) as f32,
                quality_flag: QualityFlag::Valid,
                source: "epa_radnet".to_string(),
                cell_id: format!("{:.2},{:.2}", lat, lon),
            });
        }
        
        readings
    }

    /// Parse RadNet CSV data as fallback
    fn parse_radnet_csv(&self, csv_data: &str) -> anyhow::Result<Vec<RadiationReading>> {
        let mut readings = Vec::new();
        let mut lines = csv_data.lines();
        
        let header = lines.next().ok_or_else(|| anyhow::anyhow!("Empty CSV"))?;
        let headers: Vec<&str> = header.split(',').map(|h| h.trim()).collect();
        
        let location_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("Location"));
        let city_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("City"));
        let state_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("State"));
        let date_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("Date"));
        let time_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("Time"));
        let gamma_idx = headers.iter().position(|&h| h.contains("Gamma") || h.contains("CPM"));
        
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = line.split(',').map(|f| f.trim()).collect();
            if fields.len() < 3 {
                continue;
            }
            
            let location = location_idx.and_then(|i| fields.get(i)).unwrap_or(&"Unknown");
            let city = city_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            let state = state_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            
            let (lat, lon) = self.approximate_coordinates(city, state);
            
            let date_str = date_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            let time_str = time_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            
            let timestamp = if !date_str.is_empty() {
                let datetime_str = format!("{} {}", date_str, time_str);
                NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M")
                    .or_else(|_| NaiveDateTime::parse_from_str(&datetime_str, "%m/%d/%Y %H:%M"))
                    .ok()
                    .map(|dt| dt.and_local_timezone(Utc).single())
                    .flatten()
            } else {
                None
            }.unwrap_or_else(Utc::now);
            
            let dose_rate = if let Some(idx) = gamma_idx {
                fields.get(idx)
                    .and_then(|f| f.parse::<f64>().ok())
                    .map(|cpm| cpm * 0.0057)
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            
            if dose_rate <= 0.0 {
                continue;
            }
            
            let sensor_id = format!("epa-radnet-{}-{}-{}", location, city, state)
                .to_lowercase()
                .replace(" ", "-");
            
            readings.push(RadiationReading {
                sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                bucket: timestamp.timestamp() / 3600,
                timestamp: timestamp.timestamp(),
                latitude: lat,
                longitude: lon,
                dose_rate_microsieverts: dose_rate,
                uncertainty: 0.2,
                quality_flag: QualityFlag::Valid,
                source: "epa_radnet".to_string(),
                cell_id: format!("{:.2},{:.2}", lat, lon),
            });
        }
        
        Ok(readings)
    }
}
