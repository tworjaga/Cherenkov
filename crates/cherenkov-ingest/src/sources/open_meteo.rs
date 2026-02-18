use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn, instrument};
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use crate::pipeline::DataSource;
use crate::SourceConfig;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Open-Meteo weather data source
/// 
/// Fetches global weather data from the Open-Meteo API.
/// Weather data provides atmospheric conditions for radiation dispersion modeling.
pub struct OpenMeteoSource {
    client: Client,
    config: SourceConfig,
}

/// Open-Meteo API response
#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    latitude: f64,
    longitude: f64,
    hourly: OpenMeteoHourly,
}

/// Open-Meteo hourly data
#[derive(Debug, Deserialize)]
struct OpenMeteoHourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    relative_humidity_2m: Vec<f64>,
    wind_speed_10m: Vec<f64>,
    wind_direction_10m: Vec<f64>,
    pressure_msl: Vec<f64>,
}

/// Weather reading with radiation correlation
#[derive(Debug, Clone)]
pub struct WeatherReading {
    pub latitude: f64,
    pub longitude: f64,
    pub temperature_c: f64,
    pub humidity_percent: f64,
    pub wind_speed_ms: f64,
    pub wind_direction_deg: f64,
    pub pressure_hpa: f64,
    pub timestamp: DateTime<Utc>,
}

impl OpenMeteoSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "open_meteo".to_string(),
                url: "https://api.open-meteo.com/v1/forecast".to_string(),
                poll_interval_secs: 3600,
                timeout: Duration::from_secs(30),
                retries: 3,
            },
        }
    }


    /// Parse Open-Meteo response into weather readings
    fn parse_response(&self, response: &OpenMeteoResponse) -> Vec<WeatherReading> {
        let mut readings = Vec::new();
        
        for (i, time_str) in response.hourly.time.iter().enumerate() {
            if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(time_str) {
                let timestamp = timestamp.with_timezone(&Utc);
                
                let reading = WeatherReading {
                    latitude: response.latitude,
                    longitude: response.longitude,
                    temperature_c: *response.hourly.temperature_2m.get(i).unwrap_or(&0.0),
                    humidity_percent: *response.hourly.relative_humidity_2m.get(i).unwrap_or(&0.0),
                    wind_speed_ms: *response.hourly.wind_speed_10m.get(i).unwrap_or(&0.0),
                    wind_direction_deg: *response.hourly.wind_direction_10m.get(i).unwrap_or(&0.0),
                    pressure_hpa: *response.hourly.pressure_msl.get(i).unwrap_or(&1013.25),
                    timestamp,
                };
                
                readings.push(reading);
            }
        }
        
        readings
    }

    /// Convert weather reading to radiation proxy
    /// 
    /// High wind speeds and specific pressure patterns may indicate
    /// conditions favorable for radiation dispersion
    fn weather_to_radiation_proxy(&self, weather: &WeatherReading) -> Option<RadiationReading> {
        // Only create readings for significant weather events
        // High wind speeds (> 10 m/s) may indicate significant transport
        if weather.wind_speed_ms < 10.0 {
            return None;
        }
        
        let sensor_id = format!("openmeteo-{:.2}-{:.2}-{}", 
            weather.latitude, 
            weather.longitude,
            weather.timestamp.format("%Y%m%d%H")
        );
        
        // Create proxy reading based on wind speed
        // Higher winds = higher proxy value for dispersion modeling
        let proxy_dose = weather.wind_speed_ms / 100.0;
        
        Some(RadiationReading {
            sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
            bucket: weather.timestamp.timestamp() / 3600,
            timestamp: weather.timestamp.timestamp(),
            latitude: weather.latitude,
            longitude: weather.longitude,
            dose_rate_microsieverts: proxy_dose,
            uncertainty: 0.3,
            quality_flag: QualityFlag::Suspect,

            source: "open_meteo".to_string(),
            cell_id: format!("{:.2},{:.2}", weather.latitude, weather.longitude),
        })
    }

    /// Log significant weather events
    fn log_significant_events(&self, weather: &WeatherReading) {
        if weather.wind_speed_ms > 20.0 {
            warn!("High wind speed at ({:.2}, {:.2}): {:.1} m/s", 
                weather.latitude, weather.longitude, weather.wind_speed_ms);
        }
        
        if weather.pressure_hpa < 980.0 {
            warn!("Low pressure at ({:.2}, {:.2}): {:.1} hPa", 
                weather.latitude, weather.longitude, weather.pressure_hpa);
        }
        
        if weather.temperature_c > 35.0 {
            warn!("High temperature at ({:.2}, {:.2}): {:.1}°C", 
                weather.latitude, weather.longitude, weather.temperature_c);
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
        // Fetch weather data for key regions around nuclear facilities
        let locations = vec![
            // North America - multiple points
            (40.7128, -74.0060),  // New York
            (34.0522, -118.2437), // Los Angeles
            (41.8781, -87.6298),  // Chicago
            // Europe
            (51.5074, -0.1278),   // London
            (48.8566, 2.3522),    // Paris
            (52.5200, 13.4050),   // Berlin
            // East Asia
            (35.6762, 139.6503),  // Tokyo
            (37.5665, 126.9780),  // Seoul
            (39.9042, 116.4074),  // Beijing
            // Fukushima region
            (37.4214, 141.0328),  // Fukushima
        ];
        
        let mut all_readings = Vec::new();
        
        for (lat, lon) in locations {
            let url = format!(
                "{}?latitude={}&longitude={}&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m,wind_direction_10m,pressure_msl&timezone=UTC",
                self.config.url,
                lat,
                lon
            );
            
            let response = self.client
                .get(&url)
                .header("User-Agent", "Cherenkov/1.0 (Radiation Monitoring)")
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    match resp.json::<OpenMeteoResponse>().await {
                        Ok(data) => {
                            let weather_readings = self.parse_response(&data);
                            info!("Fetched {} hourly readings for ({}, {})", 
                                weather_readings.len(), lat, lon);
                            
                            for weather in &weather_readings {
                                self.log_significant_events(weather);
                                
                                if let Some(proxy) = self.weather_to_radiation_proxy(weather) {
                                    all_readings.push(proxy);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse Open-Meteo response for ({}, {}): {}", 
                                lat, lon, e);
                        }
                    }
                }
                Ok(resp) => {
                    warn!("Open-Meteo returned {} for ({}, {})", resp.status(), lat, lon);
                }
                Err(e) => {
                    warn!("Failed to fetch Open-Meteo for ({}, {}): {}", lat, lon, e);
                }
            }
        }
        
        info!("Converted {} weather readings to radiation proxies", all_readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "open_meteo")
            .increment(all_readings.len() as u64);

        Ok(all_readings)
    }
}
