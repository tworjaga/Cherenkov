pub mod inference;
pub mod isotope;
pub mod training;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub isotopes: Vec<IsotopePrediction>,
    pub latency_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsotopePrediction {
    pub symbol: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spectrum {
    pub channels: Vec<f32>,
    pub calibration: Calibration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calibration {
    pub slope: f32,
    pub intercept: f32,
    pub quadratic: f32,
}
