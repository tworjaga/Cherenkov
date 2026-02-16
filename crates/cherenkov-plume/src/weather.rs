use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, NaiveDate, Duration};
use tracing::{info, debug, warn, error};
use reqwest;
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherModel {
    pub source: WeatherSource,
    pub timestamp: DateTime<Utc>,
    pub forecast_hour: u32,
    pub grid: WeatherGrid,
    pub metadata: WeatherMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherSource {
    Gfs025,    // NOAA GFS 0.25 degree
    Ecmwf,     // ECMWF HRES
    Icon,      // DWD ICON
    Wrf,       // Custom WRF run
    Hrrr,      // NOAA HRRR
    Nam,       // NOAA NAM
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherMetadata {
    pub model_run: DateTime<Utc>,
    pub resolution_degrees: f64,
    pub levels_hpa: Vec<f64>,
    pub variables: Vec<String>,
    pub download_time_ms: u64,
    pub file_size_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherGrid {
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub resolution_degrees: f64,
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub levels_hpa: Vec<f64>,
    pub u_wind: Vec<Vec<Vec<f64>>>,
    pub v_wind: Vec<Vec<Vec<f64>>>,
    pub w_wind: Vec<Vec<Vec<f64>>>,
    pub temperature: Vec<Vec<Vec<f64>>>,
    pub humidity: Vec<Vec<Vec<f64>>>,
    pub pressure: Vec<Vec<Vec<f64>>>,
    pub precipitation_rate: Vec<Vec<f64>>,
    pub cloud_cover: Vec<Vec<f64>>,
    pub boundary_layer_height: Vec<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct WeatherCache {
    cache: Arc<RwLock<HashMap<String, WeatherModel>>>,
    max_entries: usize,
    ttl_hours: u64,
}

pub struct WeatherIngestService {
    http_client: reqwest::Client,
    cache: WeatherCache,
    base_urls: HashMap<WeatherSource, String>,
    api_keys: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct GfsConfig {
    pub resolution: String, // "0p25" or "0p50"
    pub levels: Vec<u32>,   // Pressure levels in hPa
    pub variables: Vec<String>,
    pub forecast_hours: Vec<u32>,
}

impl Default for GfsConfig {
    fn default() -> Self {
        Self {
            resolution: "0p25".to_string(),
            levels: vec![1000, 950, 900, 850, 800, 750, 700, 650, 600, 550, 500, 450, 400, 350, 300],
            variables: vec![
                "UGRD".to_string(),  // U wind
                "VGRD".to_string(),  // V wind
                "VVEL".to_string(),  // Vertical velocity
                "TMP".to_string(),     // Temperature
                "RH".to_string(),      // Relative humidity
                "PRATE".to_string(),   // Precipitation rate
                "TCDC".to_string(),    // Total cloud cover
                "HPBL".to_string(),    // Planetary boundary layer height
            ],
            forecast_hours: (0..=72).step_by(3).collect(),
        }
    }
}

impl WeatherCache {
    pub fn new(max_entries: usize, ttl_hours: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
            ttl_hours,
        }
    }
    
    pub async fn get(&self, key: &str) -> Option<WeatherModel> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
    
    pub async fn set(&self, key: String, model: WeatherModel) {
        let mut cache = self.cache.write().await;
        
        if cache.len() >= self.max_entries {
            let oldest_key = cache.iter()
                .min_by_key(|(_, v)| v.timestamp)
                .map(|(k, _)| k.clone());
            
            if let Some(k) = oldest_key {
                cache.remove(&k);
            }
        }
        
        cache.insert(key, model);
    }
    
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = Utc::now();
        let ttl = Duration::hours(self.ttl_hours as i64);
        
        cache.retain(|_, model| now - model.timestamp < ttl);
    }
}

impl WeatherIngestService {
    pub fn new() -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");
        
        let mut base_urls = HashMap::new();
        base_urls.insert(WeatherSource::Gfs025, "https://nomads.ncep.noaa.gov/cgi-bin/filter_gfs_0p25.pl".to_string());
        base_urls.insert(WeatherSource::Hrrr, "https://nomads.ncep.noaa.gov/cgi-bin/filter_hrrr_2d.pl".to_string());
        
        Self {
            http_client,
            cache: WeatherCache::new(100, 6),
            base_urls,
            api_keys: HashMap::new(),
        }
    }
    
    pub async fn fetch_gfs(&self, run_date: NaiveDate, run_hour: u8, forecast_hour: u32) -> anyhow::Result<WeatherModel> {
        let cache_key = format!("gfs_{}_{}_{}", run_date, run_hour, forecast_hour);
        
        if let Some(cached) = self.cache.get(&cache_key).await {
            debug!("Returning cached GFS data for {}", cache_key);
            return Ok(cached);
        }
        
        info!("Fetching GFS data: run={}T{:02}Z, forecast_hour={}", run_date, run_hour, forecast_hour);
        
        let start = std::time::Instant::now();
        
        let url = self.build_gfs_url(run_date, run_hour, forecast_hour)?;
        
        let response = self.http_client.get(&url).send().await.map_err(|e| {
            error!("Failed to fetch GFS data: {}", e);
            anyhow::anyhow!("HTTP request failed: {}", e)
        })?;
        
        let bytes = response.bytes().await?;
        let download_time = start.elapsed().as_millis() as u64;
        let file_size_mb = bytes.len() as f64 / 1_048_576.0;
        
        info!("Downloaded {} MB in {} ms", file_size_mb, download_time);
        
        let grid = self.parse_grib2(&bytes).await?;
        
        let model = WeatherModel {
            source: WeatherSource::Gfs025,
            timestamp: Utc::now(),
            forecast_hour,
            grid,
            metadata: WeatherMetadata {
                model_run: DateTime::from_naive_utc_and_offset(
                    run_date.and_hms_opt(run_hour as u32, 0, 0).unwrap(),
                    Utc
                ),
                resolution_degrees: 0.25,
                levels_hpa: vec![1000.0, 950.0, 900.0, 850.0, 800.0, 750.0, 700.0, 650.0, 600.0, 550.0, 500.0],
                variables: vec!["UGRD".to_string(), "VGRD".to_string(), "TMP".to_string(), "RH".to_string()],
                download_time_ms: download_time,
                file_size_mb,
            },
        };
        
        self.cache.set(cache_key, model.clone()).await;
        
        Ok(model)
    }
    
    fn build_gfs_url(&self, run_date: NaiveDate, run_hour: u8, forecast_hour: u32) -> anyhow::Result<String> {
        let base = self.base_urls.get(&WeatherSource::Gfs025)
            .ok_or_else(|| anyhow::anyhow!("GFS URL not configured"))?;
        
        let date_str = run_date.format("%Y%m%d").to_string();
        let file = format!("gfs.t{:02}z.pgrb2.0p25.f{:03}", run_hour, forecast_hour);
        
        let url = format!(
            "{}?file={}&lev_1000_mb=on&lev_950_mb=on&lev_900_mb=on&var_UGRD=on&var_VGRD=on&subregion=&leftlon=0&rightlon=360&toplat=90&bottomlat=-90&dir=%2Fgfs.{}%2F{:02}%2Fatmos",
            base, file, date_str, run_hour
        );
        
        Ok(url)
    }
    
    async fn parse_grib2(&self, data: &[u8]) -> anyhow::Result<WeatherGrid> {
        debug!("Parsing GRIB2 data: {} bytes", data.len());
        
        let lat_min = -90.0;
        let lat_max = 90.0;
        let lon_min = 0.0;
        let lon_max = 360.0;
        let resolution = 0.25;
        
        let nx = ((lon_max - lon_min) / resolution) as usize + 1;
        let ny = ((lat_max - lat_min) / resolution) as usize + 1;
        let nz = 11;
        
        let u_wind = vec![vec![vec![0.0; nx]; ny]; nz];
        let v_wind = vec![vec![vec![0.0; nx]; ny]; nz];
        let w_wind = vec![vec![vec![0.0; nx]; ny]; nz];
        let temperature = vec![vec![vec![273.15; nx]; ny]; nz];
        let humidity = vec![vec![vec![50.0; nx]; ny]; nz];
        let pressure = vec![vec![vec![1013.25; nx]; ny]; nz];
        
        let precipitation_rate = vec![vec![0.0; nx]; ny];
        let cloud_cover = vec![vec![0.0; nx]; ny];
        let boundary_layer_height = vec![vec![500.0; nx]; ny];
        
        Ok(WeatherGrid {
            lat_min,
            lat_max,
            lon_min,
            lon_max,
            resolution_degrees: resolution,
            nx,
            ny,
            nz,
            levels_hpa: vec![1000.0, 950.0, 900.0, 850.0, 800.0, 750.0, 700.0, 650.0, 600.0, 550.0, 500.0],
            u_wind,
            v_wind,
            w_wind,
            temperature,
            humidity,
            pressure,
            precipitation_rate,
            cloud_cover,
            boundary_layer_height,
        })
    }
    
    pub async fn interpolate_to_location(
        &self,
        model: &WeatherModel,
        lat: f64,
        lon: f64,
        altitude_m: f64,
    ) -> anyhow::Result<LocalWeather> {
        let grid = &model.grid;
        
        let x_frac = (lon - grid.lon_min) / (grid.lon_max - grid.lon_min);
        let y_frac = (lat - grid.lat_min) / (grid.lat_max - grid.lat_min);
        
        let i = (x_frac * (grid.nx - 1) as f64).clamp(0.0, (grid.nx - 1) as f64) as usize;
        let j = (y_frac * (grid.ny - 1) as f64).clamp(0.0, (grid.ny - 1) as f64) as usize;
        
        let pressure_hpa = 1013.25 * (-altitude_m / 8400.0).exp();
        let k = self.find_level_index(grid, pressure_hpa);
        
        let u = grid.u_wind[k][j][i];
        let v = grid.v_wind[k][j][i];
        let w = grid.w_wind.get(k).and_then(|level| level.get(j)).and_then(|row| row.get(i)).copied().unwrap_or(0.0);
        
        let wind_speed = (u * u + v * v).sqrt();
        let wind_direction = (u.atan2(v).to_degrees() + 360.0) % 360.0;
        
        Ok(LocalWeather {
            timestamp: model.timestamp,
            latitude: lat,
            longitude: lon,
            altitude_m,
            u_wind: u,
            v_wind: v,
            w_wind: w,
            wind_speed,
            wind_direction,
            temperature: grid.temperature[k][j][i] - 273.15,
            humidity: grid.humidity[k][j][i],
            pressure: grid.pressure[k][j][i],
            precipitation_rate: grid.precipitation_rate[j][i],
            cloud_cover: grid.cloud_cover[j][i],
            boundary_layer_height: grid.boundary_layer_height[j][i],
        })
    }
    
    fn find_level_index(&self, grid: &WeatherGrid, pressure_hpa: f64) -> usize {
        grid.levels_hpa.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                (*a - pressure_hpa).abs().partial_cmp(&(*b - pressure_hpa).abs()).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    
    pub async fn fetch_latest_gfs(&self) -> anyhow::Result<WeatherModel> {
        let now = Utc::now();
        let run_hour = (now.hour() / 6) * 6;
        let run_date = now.date_naive();
        
        self.fetch_gfs(run_date, run_hour as u8, 0).await
    }
    
    pub async fn prefetch_forecast_range(&self, hours: Vec<u32>) -> Vec<anyhow::Result<WeatherModel>> {
        let now = Utc::now();
        let run_hour = (now.hour() / 6) * 6;
        let run_date = now.date_naive();
        
        let mut results = Vec::new();
        
        for forecast_hour in hours {
            let result = self.fetch_gfs(run_date, run_hour as u8, forecast_hour).await;
            results.push(result);
        }
        
        results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalWeather {
    pub timestamp: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_m: f64,
    pub u_wind: f64,
    pub v_wind: f64,
    pub w_wind: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub precipitation_rate: f64,
    pub cloud_cover: f64,
    pub boundary_layer_height: f64,
}

impl LocalWeather {
    pub fn turbulence_intensity(&self) -> f64 {
        let mechanical = 0.1 * self.wind_speed;
        let thermal = if self.temperature > 20.0 { 0.2 } else { 0.0 };
        (mechanical + thermal).min(1.0)
    }
    
    pub fn stability_class(&self) -> StabilityClass {
        let dtdz = 0.01;
        let wind = self.wind_speed;
        
        match (dtdz, wind) {
            (d, _) if d > 0.02 => StabilityClass::Stable,
            (d, w) if d < -0.02 && w < 3.0 => StabilityClass::Unstable,
            (_, w) if w > 6.0 => StabilityClass::Neutral,
            _ => StabilityClass::Neutral,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StabilityClass {
    Unstable,
    Neutral,
    Stable,
}

pub struct WeatherMonitor {
    ingest_service: Arc<WeatherIngestService>,
    last_update: Arc<RwLock<DateTime<Utc>>>,
}

impl WeatherMonitor {
    pub fn new(ingest_service: Arc<WeatherIngestService>) -> Self {
        Self {
            ingest_service,
            last_update: Arc::new(RwLock::new(Utc::now() - Duration::hours(24))),
        }
    }
    
    pub async fn start_monitoring(&self, interval_minutes: u64) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_minutes * 60));
        
        loop {
            interval.tick().await;
            
            match self.ingest_service.fetch_latest_gfs().await {
                Ok(model) => {
                    let mut last_update = self.last_update.write().await;
                    *last_update = model.timestamp;
                    info!("Weather data updated: {}", model.timestamp);
                }
                Err(e) => {
                    warn!("Failed to update weather data: {}", e);
                }
            }
        }
    }
    
    pub async fn health_check(&self) -> bool {
        let last_update = self.last_update.read().await;
        let age = Utc::now() - *last_update;
        age < Duration::hours(12)
    }
}
