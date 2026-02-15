use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherModel {
    pub source: WeatherSource,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub grid: WeatherGrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherSource {
    Gfs025,    // NOAA GFS 0.25 degree
    Ecmwf,     // ECMWF HRES
    Icon,      // DWD ICON
    Wrf,       // Custom WRF run
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherGrid {
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub resolution_degrees: f64,
    pub levels_hpa: Vec<f64>,
    pub u_wind: Vec<Vec<Vec<f64>>>,
    pub v_wind: Vec<Vec<Vec<f64>>>,
    pub temperature: Vec<Vec<Vec<f64>>>,
    pub humidity: Vec<Vec<Vec<f64>>>,
    pub precipitation: Vec<Vec<f64>>,
}

pub struct WeatherIngest;

impl WeatherIngest {
    pub async fn fetch_gfs(run_date: chrono::NaiveDate, hour: u8) -> anyhow::Result<WeatherModel> {
        // Placeholder for GFS data fetching
        // In production, this would:
        // 1. Query NOAA NOMADS server
        // 2. Download GRIB2 files
        // 3. Parse with g2clib or eccodes
        // 4. Extract relevant levels and variables
        
        Ok(WeatherModel {
            source: WeatherSource::Gfs025,
            timestamp: chrono::Utc::now(),
            grid: WeatherGrid {
                lat_min: -90.0,
                lat_max: 90.0,
                lon_min: 0.0,
                lon_max: 360.0,
                resolution_degrees: 0.25,
                levels_hpa: vec![1000.0, 950.0, 900.0, 850.0, 800.0, 750.0, 700.0, 650.0, 600.0, 550.0, 500.0],
                u_wind: vec![],
                v_wind: vec![],
                temperature: vec![],
                humidity: vec![],
                precipitation: vec![],
            },
        })
    }
}
