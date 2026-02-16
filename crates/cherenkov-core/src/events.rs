use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core event types for inter-crate communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CherenkovEvent {
    /// New radiation reading from ingest
    NewReading(NormalizedReading),
    
    /// Anomaly detected by stream processor
    AnomalyDetected(Anomaly),
    
    /// Alert triggered for significant event
    AlertTriggered(Alert),
    
    /// Sensor status change
    SensorStatusChange {
        sensor_id: Uuid,
        status: SensorStatus,
        timestamp: DateTime<Utc>,
    },
    
    /// System health update
    HealthUpdate {
        component: String,
        healthy: bool,
        message: Option<String>,
    },
    
    /// Correlated event detected
    CorrelatedEventDetected {
        primary: Anomaly,
        correlated_count: usize,
        correlation_score: f64,
    },
}

/// Normalized radiation reading from any source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedReading {
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate_microsieverts: f64,
    pub uncertainty: f64,
    pub source: String,
    pub quality_flag: QualityFlag,
}

/// Quality classification for readings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityFlag {
    Valid,
    Suspect,
    Invalid,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub anomaly_id: String,
    pub sensor_id: Uuid,
    pub severity: Severity,
    pub z_score: f64,
    pub detected_at: DateTime<Utc>,
    pub dose_rate: f64,
    pub baseline: f64,
    pub algorithm: String,
}

/// Severity levels for anomalies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

/// Alert for significant radiation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: String,
    pub anomaly_ids: Vec<String>,
    pub message: String,
    pub severity: Severity,
    pub created_at: DateTime<Utc>,
    pub acknowledged: bool,
}

/// Sensor operational status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
}

/// Algorithm types for ML processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    IsolationForest,
    LSTM,
    Autoencoder,
    Statistical,
    Ensemble,
}
