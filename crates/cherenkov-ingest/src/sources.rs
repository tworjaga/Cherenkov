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
