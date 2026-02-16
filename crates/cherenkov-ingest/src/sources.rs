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
/// 
/// Fetches radiation monitoring data from EPA RadNet via CSV data files.
/// RadNet provides near real-time environmental radiation monitoring data
/// from stations across the United States.
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

    /// Parse RadNet CSV data
    /// 
    /// Expected CSV format:
    /// Location, City, State, Date, Time, Gamma (CPM), Beta (CPM), etc.
    fn parse_radnet_csv(&self, csv_data: &str) -> anyhow::Result<Vec<RadiationReading>> {
        let mut readings = Vec::new();
        let mut lines = csv_data.lines();
        
        // Skip header line
        let header = lines.next().ok_or_else(|| anyhow::anyhow!("Empty CSV data"))?;
        let headers: Vec<&str> = header.split(',').map(|h| h.trim()).collect();
        
        // Find column indices
        let location_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("Location"));
        let city_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("City"));
        let state_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("State"));
        let date_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("Date"));
        let time_idx = headers.iter().position(|&h| h.eq_ignore_ascii_case("Time"));
        let gamma_idx = headers.iter().position(|&h| h.contains("Gamma") || h.contains("CPM"));
        
        for (line_num, line) in lines.enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = line.split(',').map(|f| f.trim()).collect();
            if fields.len() < 3 {
                warn!("Skipping malformed CSV line {}: insufficient fields", line_num + 2);
                continue;
            }
            
            // Extract location
            let location = location_idx.and_then(|i| fields.get(i)).unwrap_or(&"Unknown");
            let city = city_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            let state = state_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            
            // Parse date and time
            let date_str = date_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            let time_str = time_idx.and_then(|i| fields.get(i)).unwrap_or(&"");
            
            let timestamp = if !date_str.is_empty() {
                let datetime_str = format!("{} {}", date_str, time_str);
                chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M")
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(&datetime_str, "%m/%d/%Y %H:%M"))
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S"))
                    .ok()
                    .map(|dt| dt.and_local_timezone(chrono::Utc).single())
                    .flatten()
            } else {
                None
            };
            
            let timestamp = timestamp.unwrap_or_else(chrono::Utc::now);
            
            // Parse gamma radiation (CPM to μSv/h conversion)
            let dose_rate = if let Some(idx) = gamma_idx {
                fields.get(idx)
                    .and_then(|f| f.parse::<f64>().ok())
                    .map(|cpm| cpm * 0.0057) // Convert CPM to μSv/h
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            
            if dose_rate <= 0.0 {
                continue; // Skip readings with no valid dose rate
            }
            
            // Generate sensor ID from location
            let sensor_id = format!("epa-radnet-{}-{}-{}", location, city, state)
                .to_lowercase()
                .replace(" ", "-");
            
            // Approximate coordinates (would need geocoding in production)
            // Using placeholder coordinates - real implementation would use a geocoding service
            let (latitude, longitude) = self.approximate_coordinates(city, state);
            
            readings.push(RadiationReading {
                sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                bucket: timestamp.timestamp() / 3600,
                timestamp: timestamp.timestamp(),
                latitude,
                longitude,
                dose_rate_microsieverts: dose_rate,
                uncertainty: 0.2, // EPA instruments typically have 10-20% uncertainty
                quality_flag: cherenkov_db::QualityFlag::Valid,
                source: "epa_radnet".to_string(),
                cell_id: format!("{:.2},{:.2}", latitude, longitude),
            });
        }
        
        Ok(readings)
    }
    
    /// Approximate coordinates for US cities
    /// In production, this should use a proper geocoding service
    fn approximate_coordinates(&self, city: &str, state: &str) -> (f64, f64) {
        // Major US cities with approximate coordinates
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
            _ => {
                // Default to continental US center for unknown locations
                (39.8283, -98.5795)
            }
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

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // EPA RadNet provides CSV data files
        // The actual URL pattern may vary - this is a representative implementation
        
        // Try to fetch the latest CSV data
        // In production, this would need to handle:
        // 1. Dynamic CSV file URLs from EPA website
        // 2. Authentication if required
        // 3. Multiple CSV files for different time periods
        
        let csv_url = "https://www.epa.gov/sites/default/files/2023-06/radnet-near-real-time-data.csv";
        
        let response = self.client
            .get(csv_url)
            .send()
            .await;

        let csv_data = match response {
            Ok(resp) if resp.status().is_success() => resp.text().await?,
            Ok(resp) => {
                warn!("EPA RadNet CSV endpoint returned {} - using fallback", resp.status());
                // Return empty for now - EPA data access may require different approach
                return Ok(vec![]);
            }
            Err(e) => {
                warn!("Failed to fetch EPA RadNet data: {} - using fallback", e);
                return Ok(vec![]);
            }
        };

        if csv_data.trim().is_empty() {
            warn!("EPA RadNet CSV data is empty");
            return Ok(vec![]);
        }

        // Parse the CSV data
        let readings = self.parse_radnet_csv(&csv_data)?;
        
        info!("Parsed {} radiation readings from EPA RadNet", readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "epa_radnet")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}


/// OpenAQ air quality correlation source
/// 
/// Fetches air quality data (PM2.5, PM10, O3, NO2, SO2, CO) for correlation
/// with radiation transport patterns and atmospheric dispersion modeling.
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

#[derive(Debug, Deserialize)]
struct OpenAqResponse {
    results: Vec<OpenAqMeasurement>,
}

#[derive(Debug, Deserialize)]
struct OpenAqMeasurement {
    location: String,
    city: Option<String>,
    country: Option<String>,
    coordinates: Option<OpenAqCoordinates>,
    measurements: Vec<OpenAqParameter>,
    #[serde(rename = "date")]
    timestamp: OpenAqTimestamp,
}

#[derive(Debug, Deserialize)]
struct OpenAqCoordinates {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize)]
struct OpenAqParameter {
    parameter: String,
    value: f64,
    unit: String,
}

#[derive(Debug, Deserialize)]
struct OpenAqTimestamp {
    utc: String,
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
        // OpenAQ provides air quality data for correlation with radiation transport
        // Parameters: PM2.5, PM10, O3, NO2, SO2, CO
        
        let response = self.client
            .get(self.config.url)
            .query(&[
                ("limit", "100"),
                ("sort", "desc"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAQ API returned {}", response.status()));
        }

        let data: OpenAqResponse = response.json().await?;
        info!("Fetched {} locations from OpenAQ", data.results.len());

        // Log air quality data for correlation analysis
        // Note: OpenAQ provides air quality, not radiation data directly
        // This data is used for atmospheric correlation and plume modeling
        let mut air_quality_readings = 0;
        
        for measurement in &data.results {
            for param in &measurement.measurements {
                air_quality_readings += 1;
                
                // Log significant air quality events that might correlate with radiation
                match param.parameter.as_str() {
                    "pm25" if param.value > 100.0 => {
                        warn!("High PM2.5 detected at {}: {} μg/m³", 
                            measurement.location, param.value);
                    }
                    "pm10" if param.value > 150.0 => {
                        warn!("High PM10 detected at {}: {} μg/m³",
                            measurement.location, param.value);
                    }
                    "o3" if param.value > 200.0 => {
                        warn!("High O3 detected at {}: {} ppb",
                            measurement.location, param.value);
                    }
                    _ => {}
                }
            }
        }

        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "openaq")
            .increment(air_quality_readings);

        // Return empty vec - air quality data is logged but not stored as radiation readings
        // Future: Extend data model to store air quality metrics for correlation analysis
        Ok(vec![])
    }
}


/// Open-Meteo weather data source for plume modeling
/// 
/// Fetches weather data (wind speed, direction, temperature, humidity) 
/// for atmospheric dispersion modeling and radiation plume calculations.
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

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    latitude: f64,
    longitude: f64,
    hourly: OpenMeteoHourly,
    current_weather: Option<OpenMeteoCurrentWeather>,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoHourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    relative_humidity_2m: Vec<f64>,
    wind_speed_10m: Vec<f64>,
    wind_direction_10m: Vec<f64>,
    pressure_msl: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoCurrentWeather {
    temperature: f64,
    windspeed: f64,
    winddirection: f64,
    weathercode: i32,
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
        // Fetch weather data for key monitoring locations
        // Using coordinates of major radiation monitoring networks
        
        let locations = vec![
            // Tokyo area (Safecast origin)
            (35.6762, 139.6503),
            // Chernobyl area
            (51.2763, 30.2219),
            // Fukushima area
            (37.7608, 140.4748),
            // New York area
            (40.7128, -74.0060),
            // London area
            (51.5074, -0.1278),
            // Paris area
            (48.8566, 2.3522),
            // Berlin area
            (52.5200, 13.4050),
        ];

        let mut total_weather_readings = 0;

        for (lat, lon) in locations {
            let response = self.client
                .get(self.config.url)
                .query(&[
                    ("latitude", lat.to_string()),
                    ("longitude", lon.to_string()),
                    ("hourly", "temperature_2m,relative_humidity_2m,wind_speed_10m,wind_direction_10m,pressure_msl"),
                    ("current_weather", "true"),
                    ("timezone", "auto"),
                    ("forecast_days", "1"),
                ])
                .send()
                .await?;

            if !response.status().is_success() {
                warn!("Open-Meteo API returned {} for location ({}, {})", 
                    response.status(), lat, lon);
                continue;
            }

            let data: OpenMeteoResponse = response.json().await?;
            
            // Log current weather conditions
            if let Some(current) = data.current_weather {
                info!(
                    "Weather at ({}, {}): {:.1}°C, wind {:.1} m/s from {}°, code {}",
                    lat, lon, current.temperature, current.windspeed, 
                    current.winddirection, current.weathercode
                );
                
                // Log high wind conditions (relevant for plume dispersion)
                if current.windspeed > 15.0 {
                    warn!("High wind speed at ({}, {}): {:.1} m/s - significant plume dispersion",
                        lat, lon, current.windspeed);
                }
            }

            // Count hourly forecast data points
            let hourly_count = data.hourly.time.len();
            total_weather_readings += hourly_count;

            // Log atmospheric stability indicators
            if hourly_count > 0 {
                let avg_temp = data.hourly.temperature_2m.iter().sum::<f64>() / hourly_count as f64;
                let avg_wind = data.hourly.wind_speed_10m.iter().sum::<f64>() / hourly_count as f64;
                
                info!(
                    "Hourly forecast for ({}, {}): {} hours, avg temp {:.1}°C, avg wind {:.1} m/s",
                    lat, lon, hourly_count, avg_temp, avg_wind
                );
            }
        }

        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "openmeteo")
            .increment(total_weather_readings as u64);

        metrics::gauge!("cherenkov_openmeteo_locations_monitored")
            .set(locations.len() as f64);

        // Weather data is logged for plume modeling but not stored as radiation readings
        // Future: Store weather data in separate table for dispersion calculations
        Ok(vec![])
    }
}
