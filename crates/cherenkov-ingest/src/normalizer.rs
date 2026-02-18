use crate::RawReading;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum NormalizeError {
    #[error("Invalid coordinates")]
    InvalidCoordinates,
    #[error("Invalid dose rate")]
    InvalidDoseRate,
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),
}

#[allow(dead_code)]
pub struct NormalizedReading {
    pub sensor_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub location: GeoPoint,
    pub dose_rate_microsieverts: f64,
    pub source: String,
    pub quality_flag: QualityFlag,
}

#[allow(dead_code)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

#[allow(dead_code)]
pub enum QualityFlag {
    Valid,
    Suspect,
    Invalid,
}

#[allow(dead_code)]
pub struct Normalizer;

impl Normalizer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
    
    #[allow(dead_code)]
    pub async fn normalize(&self, raw: RawReading) -> Result<NormalizedReading, NormalizeError> {
        if raw.latitude < -90.0 || raw.latitude > 90.0 {
            return Err(NormalizeError::InvalidCoordinates);
        }
        if raw.longitude < -180.0 || raw.longitude > 180.0 {
            return Err(NormalizeError::InvalidCoordinates);
        }
        
        // Validate dose rate is not negative or NaN
        if raw.dose_rate < 0.0 || raw.dose_rate.is_nan() {
            return Err(NormalizeError::InvalidDoseRate);
        }
        
        let dose_rate_microsieverts = match raw.unit.as_str() {
            "uSv/h" | "microsieverts/hour" => raw.dose_rate,
            "mSv/h" => raw.dose_rate * 1000.0,
            "Sv/h" => raw.dose_rate * 1_000_000.0,
            "cpm" => raw.dose_rate * 0.0057,
            _ => return Err(NormalizeError::UnknownUnit(raw.unit.clone())),
        };
        
        Ok(NormalizedReading {
            sensor_id: raw.sensor_id,
            timestamp: raw.timestamp,
            location: GeoPoint {
                lat: raw.latitude,
                lon: raw.longitude,
            },
            dose_rate_microsieverts,
            source: raw.source,
            quality_flag: QualityFlag::Valid,
        })
    }
}
