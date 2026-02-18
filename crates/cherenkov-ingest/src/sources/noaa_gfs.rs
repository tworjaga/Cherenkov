use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn, instrument};
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use crate::pipeline::DataSource;
use crate::SourceConfig;
use chrono::{DateTime, Utc, Timelike};


/// NOAA GFS (Global Forecast System) weather data source
/// 
/// Fetches global weather model data for atmospheric dispersion modeling.
/// GFS provides wind, temperature, and pressure data on a global grid.
pub struct NoaaGfsSource {
    client: Client,
    config: SourceConfig,
}

/// GFS grid point data
#[derive(Debug, Clone)]
pub struct GfsGridPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub temperature_c: f64,
    pub wind_u: f64,  // U-component of wind (m/s)
    pub wind_v: f64,  // V-component of wind (m/s)
    pub pressure_hpa: f64,
    pub humidity_percent: f64,
    pub timestamp: DateTime<Utc>,
}

impl NoaaGfsSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: SourceConfig {
                name: "noaa_gfs".to_string(),
                url: "https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl".to_string(),
                poll_interval_secs: 21600, // 6 hours - GFS updates every 6 hours
                timeout: std::time::Duration::from_secs(30),
                retries: 3,
            },

        }
    }

    /// Parse GFS GRIB2 index file to find data locations
    fn parse_gfs_index(&self, index_data: &str) -> Vec<(String, u64, u64)> {
        let mut records = Vec::new();
        
        for line in index_data.lines() {
            // Format: record_number|byte_offset|byte_length|variable|level|forecast_time
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 6 {
                if let Ok(offset) = parts[1].parse::<u64>() {
                    if let Ok(length) = parts[2].parse::<u64>() {
                        let variable = parts[3].to_string();
                        records.push((variable, offset, length));
                    }
                }
            }
        }
        
        records
    }

    /// Extract wind speed from U and V components
    fn calculate_wind_speed(&self, u: f64, v: f64) -> f64 {
        (u * u + v * v).sqrt()
    }

    /// Calculate wind direction in degrees from U and V components
    fn calculate_wind_direction(&self, u: f64, v: f64) -> f64 {
        let dir = (270.0 - v.atan2(u).to_degrees()) % 360.0;
        if dir < 0.0 { dir + 360.0 } else { dir }
    }

    /// Fetch GFS data via OpenDAP-style ASCII request
    /// 
    /// Uses NOAA's simplified ASCII grid access for specific regions
    async fn fetch_gfs_ascii_grid(
        &self,
        run_date: &str,
        forecast_hour: u32,
        var: &str,
        level: &str,
        north: f64,
        south: f64,
        east: f64,
        west: f64,
    ) -> anyhow::Result<String> {
        let url = format!(
            "{}/dir?file=gfs.t{}z.pgrb2.0p25.f{:03}&lev_{}=on&var_{}=on&subregion=&north={}&south={}&west={}&east={}&dir=%2Fgfs.{}%2F{}%2Fatmos",
            self.config.url,
            run_date,
            forecast_hour,
            level,
            var,
            north,
            south,
            west,
            east,
            run_date,
            run_date
        );
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Cherenkov/1.0 (Radiation Monitoring)")
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("NOAA GFS returned {}", response.status()));
        }
        
        Ok(response.text().await?)
    }

    /// Parse ASCII grid data from GFS
    fn parse_ascii_grid(&self, ascii_data: &str) -> Vec<GfsGridPoint> {
        let mut points = Vec::new();
        let mut current_lat = 0.0;
        let mut current_lon = 0.0;
        let mut reading_grid = false;
        
        for line in ascii_data.lines() {
            let trimmed = line.trim();
            
            // Look for grid header
            if trimmed.starts_with("lat,lon,") {
                reading_grid = true;
                continue;
            }
            
            if reading_grid && !trimmed.is_empty() {
                let parts: Vec<&str> = trimmed.split(',').collect();
                if parts.len() >= 3 {
                    if let (Ok(lat), Ok(lon), Ok(value)) = (
                        parts[0].parse::<f64>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>()
                    ) {
                        current_lat = lat;
                        current_lon = lon;
                        
                        // Create grid point with placeholder values
                        // Real implementation would parse all variables
                        points.push(GfsGridPoint {
                            latitude: lat,
                            longitude: lon,
                            temperature_c: value,
                            wind_u: 0.0,
                            wind_v: 0.0,
                            pressure_hpa: 1013.25,
                            humidity_percent: 50.0,
                            timestamp: Utc::now(),
                        });
                    }
                }
            }
        }
        
        points
    }

    /// Convert GFS grid point to radiation proxy reading
    /// 
    /// Weather data is used as proxy for atmospheric transport conditions
    /// High wind speeds and specific pressure patterns may indicate
    /// conditions favorable for radiation dispersion
    fn grid_to_radiation_proxy(&self, point: &GfsGridPoint) -> Option<RadiationReading> {
        // Only create readings for significant weather events
        let wind_speed = self.calculate_wind_speed(point.wind_u, point.wind_v);
        
        // High wind conditions (> 15 m/s) may indicate significant transport
        if wind_speed < 15.0 {
            return None;
        }
        
        let sensor_id = format!("gfs-{:.2}-{:.2}-{}", 
            point.latitude, 
            point.longitude,
            point.timestamp.format("%Y%m%d%H")
        );
        
        // Create proxy reading based on wind speed
        // Higher winds = higher proxy value for dispersion modeling
        let proxy_dose = wind_speed / 100.0;
        
        Some(RadiationReading {
            sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
            bucket: point.timestamp.timestamp() / 3600,
            timestamp: point.timestamp.timestamp(),
            latitude: point.latitude,
            longitude: point.longitude,
            dose_rate_microsieverts: proxy_dose,
            uncertainty: 0.3,
            quality_flag: QualityFlag::Suspect,

            source: "noaa_gfs".to_string(),
            cell_id: format!("{:.2},{:.2}", point.latitude, point.longitude),
        })
    }
}

#[async_trait]
impl DataSource for NoaaGfsSource {
    fn name(&self) -> String {
        self.config.name.to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(self.config.poll_interval_secs)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        // GFS runs at 00, 06, 12, 18 UTC
        // Get the most recent run
        let now = Utc::now();
        let run_hour = (now.hour() / 6) * 6;
        let run_date = now.format("%Y%m%d").to_string();
        let run_time = format!("{:02}", run_hour);
        let run_timestamp = format!("{}{}", run_date, run_time);
        
        // Fetch data for key regions around nuclear facilities
        let regions = vec![
            // North America
            (60.0, 20.0, -50.0, -130.0),
            // Europe
            (70.0, 35.0, 40.0, -10.0),
            // East Asia
            (50.0, 20.0, 150.0, 100.0),
            // Japan/Fukushima region
            (45.0, 30.0, 145.0, 135.0),
        ];
        
        let mut all_points = Vec::new();
        
        for (north, south, east, west) in regions {
            // Try to fetch temperature data
            match self.fetch_gfs_ascii_grid(
                &run_timestamp,
                0, // Analysis (0-hour forecast)
                "TMP",
                "2_m_above_ground",
                north, south, east, west
            ).await {
                Ok(data) => {
                    let points = self.parse_ascii_grid(&data);
                    all_points.extend(points);
                }
                Err(e) => {
                    warn!("Failed to fetch GFS data for region ({}, {}, {}, {}): {}", 
                        north, south, east, west, e);
                }
            }
        }
        
        info!("Fetched {} GFS grid points from NOAA", all_points.len());
        
        // Convert to radiation proxy readings
        let readings: Vec<RadiationReading> = all_points
            .iter()
            .filter_map(|p| self.grid_to_radiation_proxy(p))
            .collect();
        
        info!("Converted {} GFS points to radiation proxy readings", readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "noaa_gfs")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}
