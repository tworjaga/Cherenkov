pub mod dispersion;
pub mod weather;
pub mod particle;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseParameters {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_m: f64,
    pub release_rate_bq_s: f64,
    pub duration_hours: u32,
    pub isotope: String,
    pub particle_size_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlumeSimulation {
    pub release: ReleaseParameters,
    pub concentration_grid: ConcentrationGrid,
    pub arrival_times: Vec<ArrivalTime>,
    pub total_integrated_dose: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationGrid {
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub resolution_m: f64,
    pub levels: Vec<Vec<Vec<f64>>>,
    pub timestamps: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrivalTime {
    pub latitude: f64,
    pub longitude: f64,
    pub time_seconds: f64,
    pub concentration: f64,
}
