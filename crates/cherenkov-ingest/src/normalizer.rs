use crate::sources::RawReading;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NormalizeError {
    #[error("Invalid coordinates")]
    InvalidCoordinates,
    #[error("Invalid dose rate")]
    InvalidDoseRate,
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),
}

pub struct NormalizedReading {
    pub sensor_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub location: GeoPoint,
    pub dose_rate_microsieverts: f64,
    pub source: String,
    pub quality_flag: QualityFlag,
}

pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

pub enum QualityFlag {
    Valid,
    Suspect,
    Invalid,
}

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn normalize(&self, raw: RawReading) -> Result<NormalizedReading, NormalizeError> {
        if raw.latitude < -90.0 || raw.latitude > 90.0 {
            return Err(NormalizeError::InvalidCoordinates);
        }
        if raw.longitude < -180.0 || raw.longitude > 180.0 {
            return Err(NormalizeError::InvalidCoordinates);
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
