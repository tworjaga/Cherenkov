use serde_json::json;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StructuredLog {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

impl StructuredLog {
    pub fn new(level: &str, target: &str, message: &str) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            level: level.to_string(),
            target: target.to_string(),
            message: message.to_string(),
            fields: HashMap::new(),
            trace_id: None,
            span_id: None,
        }
    }
    
    pub fn with_field(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        self.fields.insert(key.to_string(), value.into());
        self
    }
    
    pub fn with_trace(mut self, trace_id: &str, span_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self.span_id = Some(span_id.to_string());
        self
    }
    
    pub fn to_json(&self) -> String {
        let mut log = json!({
            "timestamp": self.timestamp,
            "level": self.level,
            "target": self.target,
            "message": self.message,
            "fields": self.fields,
        });
        
        if let Some(ref trace_id) = self.trace_id {
            log["trace_id"] = json!(trace_id);
        }
        if let Some(ref span_id) = self.span_id {
            log["span_id"] = json!(span_id);
        }
        
        log.to_string()
    }
}

pub fn log_anomaly_detected(
    sensor_id: &str,
    location: (f64, f64),
    dose_rate: f64,
    baseline: f64,
    z_score: f64,
    cell: &str,
) {
    let log = StructuredLog::new(
        "WARN",
        "cherenkov_stream_anomaly",
        "Anomaly detected",
    )
    .with_field("sensor_id", sensor_id)
    .with_field("location", json!({"lat": location.0, "lon": location.1}))
    .with_field("dose_rate", dose_rate)
    .with_field("baseline", baseline)
    .with_field("z_score", z_score)
    .with_field("cell", cell);
    
    println!("{}", log.to_json());
}
