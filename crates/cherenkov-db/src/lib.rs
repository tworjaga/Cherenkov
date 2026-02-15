pub mod scylla;
pub mod schema;
pub mod query;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationReading {
    pub sensor_id: Uuid,
    pub bucket: i64,
    pub timestamp: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate_microsieverts: f64,
    pub uncertainty: f32,
    pub quality_flag: QualityFlag,
    pub source: String,
    pub cell_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityFlag {
    Valid,
    Suspect,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub aggregate_id: Uuid,
    pub payload: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    AnomalyDetected,
    AlertTriggered,
    IncidentCreated,
    SensorOffline,
    SensorOnline,
}
