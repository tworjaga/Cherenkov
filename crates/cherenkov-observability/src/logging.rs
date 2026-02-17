use serde_json::{json, Value};
use chrono::Utc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{info, warn, error, debug};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub struct StructuredLogger {
    sender: mpsc::Sender<LogEntry>,
    #[allow(dead_code)]
    config: LogConfig,
}

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
    pub output_path: Option<PathBuf>,
    pub enable_console: bool,
    pub enable_file: bool,
    pub max_file_size_mb: u64,
    pub retention_days: u32,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            service_name: "cherenkov".to_string(),
            service_version: "0.1.0".to_string(),
            environment: "development".to_string(),
            output_path: Some(PathBuf::from("logs")),
            enable_console: true,
            enable_file: true,
            max_file_size_mb: 100,
            retention_days: 30,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, Value>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub source: LogSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub enum LogSource {
    Ingest,
    Stream,
    Database,
    Ml,
    Api,
    Plume,
    Auth,
    System,
}

impl StructuredLogger {
    pub fn new(config: LogConfig) -> Self {
        let (sender, mut receiver) = mpsc::channel::<LogEntry>(10000);
        
        let cfg = config.clone();
        tokio::spawn(async move {
            let mut file_writer = if cfg.enable_file {
                cfg.output_path.as_ref().map(|path| {
                    std::fs::create_dir_all(path).ok();
                    let file_path = path.join(format!("{}.log", cfg.service_name));
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path)
                        .ok()
                }).flatten()
            } else {
                None
            };
            
            while let Some(entry) = receiver.recv().await {
                let json_line = entry.to_json();
                
                if cfg.enable_console {
                    match entry.level {
                        LogLevel::Debug => debug!("{}", json_line),
                        LogLevel::Info => info!("{}", json_line),
                        LogLevel::Warn => warn!("{}", json_line),
                        LogLevel::Error | LogLevel::Fatal => error!("{}", json_line),
                    }
                }
                
                if let Some(ref mut file) = file_writer {
                    writeln!(file, "{}", json_line).ok();
                }
            }
        });
        
        Self { sender, config }
    }
    
    pub async fn log(
        &self,
        level: LogLevel,
        target: &str,
        message: &str,
        source: LogSource,
        fields: HashMap<String, Value>,
    ) {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level,
            target: target.to_string(),
            message: message.to_string(),
            fields,
            trace_id: None,
            span_id: None,
            source,
        };
        
        if let Err(e) = self.sender.send(entry).await {
            eprintln!("Failed to send log entry: {}", e);
        }
    }
    
    pub async fn debug(&self, target: &str, message: &str, source: LogSource) {
        self.log(LogLevel::Debug, target, message, source, HashMap::new()).await;
    }
    
    pub async fn info(&self, target: &str, message: &str, source: LogSource) {
        self.log(LogLevel::Info, target, message, source, HashMap::new()).await;
    }
    
    pub async fn warn(&self, target: &str, message: &str, source: LogSource) {
        self.log(LogLevel::Warn, target, message, source, HashMap::new()).await;
    }
    
    pub async fn error(&self, target: &str, message: &str, source: LogSource) {
        self.log(LogLevel::Error, target, message, source, HashMap::new()).await;
    }
    
    pub async fn fatal(&self, target: &str, message: &str, source: LogSource) {
        self.log(LogLevel::Fatal, target, message, source, HashMap::new()).await;
    }
    
    pub async fn log_with_fields(
        &self,
        level: LogLevel,
        target: &str,
        message: &str,
        source: LogSource,
        fields: Vec<(&str, Value)>,
    ) {
        let field_map: HashMap<String, Value> = fields.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self.log(level, target, message, source, field_map).await;
    }
    
    pub fn builder(&self) -> LogEntryBuilder {
        LogEntryBuilder::new(self.sender.clone(), self.config.clone())
    }
}

pub struct LogEntryBuilder {
    sender: mpsc::Sender<LogEntry>,
    config: LogConfig,
    level: Option<LogLevel>,
    target: Option<String>,
    message: Option<String>,
    source: Option<LogSource>,
    fields: HashMap<String, Value>,
    trace_id: Option<String>,
    span_id: Option<String>,
}

impl LogEntryBuilder {
    fn new(sender: mpsc::Sender<LogEntry>, config: LogConfig) -> Self {
        Self {
            sender,
            config,
            level: None,
            target: None,
            message: None,
            source: None,
            fields: HashMap::new(),
            trace_id: None,
            span_id: None,
        }
    }
    
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }
    
    pub fn target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }
    
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
    
    pub fn source(mut self, source: LogSource) -> Self {
        self.source = Some(source);
        self
    }
    
    pub fn field(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.fields.insert(key.to_string(), value.into());
        self
    }
    
    pub fn trace_context(mut self, trace_id: &str, span_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self.span_id = Some(span_id.to_string());
        self
    }
    
    pub async fn send(self) {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: self.level.unwrap_or(LogLevel::Info),
            target: self.target.unwrap_or_else(|| "unknown".to_string()),
            message: self.message.unwrap_or_else(|| "no message".to_string()),
            fields: self.fields,
            trace_id: self.trace_id,
            span_id: self.span_id,
            source: self.source.unwrap_or(LogSource::System),
        };
        
        if let Err(e) = self.sender.send(entry).await {
            eprintln!("Failed to send log entry: {}", e);
        }
    }
}

impl LogEntry {
    pub fn to_json(&self) -> String {
        let mut log = json!({
            "timestamp": self.timestamp,
            "level": format!("{:?}", self.level).to_uppercase(),
            "target": self.target,
            "message": self.message,
            "service": {
                "name": "cherenkov",
                "version": "0.1.0",
            },
            "source": format!("{:?}", self.source).to_lowercase(),
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
    let log = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "level": "WARN",
        "target": "cherenkov_stream_anomaly",
        "message": "Anomaly detected",
        "fields": {
            "sensor_id": sensor_id,
            "location": {"lat": location.0, "lon": location.1},
            "dose_rate": dose_rate,
            "baseline": baseline,
            "z_score": z_score,
            "cell": cell,
        },
    });
    
    println!("{}", log.to_string());
}

pub fn log_ingest_batch(
    source: &str,
    count: usize,
    latency_ms: u64,
    bytes_received: u64,
) {
    let log = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "level": "INFO",
        "target": "cherenkov_ingest",
        "message": "Batch ingested",
        "fields": {
            "source": source,
            "count": count,
            "latency_ms": latency_ms,
            "bytes_received": bytes_received,
        },
    });
    
    println!("{}", log.to_string());
}

pub fn log_ml_inference(
    model: &str,
    batch_size: usize,
    latency_ms: u64,
    confidence: f64,
) {
    let log = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "level": "INFO",
        "target": "cherenkov_ml_inference",
        "message": "Inference completed",
        "fields": {
            "model": model,
            "batch_size": batch_size,
            "latency_ms": latency_ms,
            "confidence": confidence,
        },
    });
    
    println!("{}", log.to_string());
}

pub fn log_db_query(
    operation: &str,
    table: &str,
    latency_ms: u64,
    rows_affected: Option<usize>,
) {
    let mut fields = json!({
        "operation": operation,
        "table": table,
        "latency_ms": latency_ms,
    });
    
    if let Some(rows) = rows_affected {
        fields["rows_affected"] = json!(rows);
    }
    
    let log = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "level": "DEBUG",
        "target": "cherenkov_db",
        "message": "Query executed",
        "fields": fields,
    });
    
    println!("{}", log.to_string());
}

pub fn log_auth_event(
    user_id: &str,
    action: &str,
    resource: &str,
    success: bool,
) {
    let log = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "level": if success { "INFO" } else { "WARN" },
        "target": "cherenkov_auth",
        "message": format!("Auth {}: {} on {}", if success { "success" } else { "failure" }, action, resource),
        "fields": {
            "user_id": user_id,
            "action": action,
            "resource": resource,
            "success": success,
        },
    });
    
    println!("{}", log.to_string());
}

pub fn log_plume_simulation(
    release_id: &str,
    isotope: &str,
    duration_hours: u32,
    particles: usize,
    simulation_time_ms: u64,
) {
    let log = json!({
        "timestamp": Utc::now().to_rfc3339(),
        "level": "INFO",
        "target": "cherenkov_plume",
        "message": "Plume simulation completed",
        "fields": {
            "release_id": release_id,
            "isotope": isotope,
            "duration_hours": duration_hours,
            "particles": particles,
            "simulation_time_ms": simulation_time_ms,
        },
    });
    
    println!("{}", log.to_string());
}
