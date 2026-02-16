use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Instant;

pub struct MetricsRegistry {
    handle: PrometheusHandle,
    custom_metrics: Arc<RwLock<HashMap<String, MetricValue>>>,
}

#[derive(Clone, Debug)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
}

pub fn init_prometheus_exporter() -> PrometheusHandle {
    let builder = PrometheusBuilder::new();
    builder
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
}

pub fn start_metrics_server(addr: SocketAddr) -> PrometheusHandle {
    let builder = PrometheusBuilder::new();
    builder
        .with_http_listener(addr)
        .install_recorder()
        .expect("Failed to start metrics server")
}

impl MetricsRegistry {
    pub fn new(handle: PrometheusHandle) -> Self {
        Self {
            handle,
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn render(&self) -> String {
        self.handle.render()
    }
    
    pub async fn record_custom(&self, name: &str, value: MetricValue) {
        let mut metrics = self.custom_metrics.write().await;
        metrics.insert(name.to_string(), value);
    }
}

#[derive(Clone)]
pub struct IngestMetrics {
    source: String,
}

impl IngestMetrics {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
    
    pub fn record_reading(&self) {
        metrics::counter!("cherenkov_ingest_readings_total", "source" => self.source.clone()).increment(1);
    }
    
    pub fn record_batch(&self, count: u64) {
        metrics::counter!("cherenkov_ingest_readings_total", "source" => self.source.clone()).increment(count);
        metrics::histogram!("cherenkov_ingest_batch_size").record(count as f64);
    }
    
    pub fn record_latency(&self, latency_ms: u64) {
        metrics::histogram!("cherenkov_ingest_latency_ms", "source" => self.source.clone()).record(latency_ms as f64);
    }
    
    pub fn record_error(&self, error_type: &str) {
        metrics::counter!("cherenkov_ingest_errors_total", 
            "source" => self.source.clone(),
            "error_type" => error_type.to_string()
        ).increment(1);
    }
    
    pub fn record_bytes_received(&self, bytes: u64) {
        metrics::counter!("cherenkov_ingest_bytes_total", "source" => self.source.clone()).increment(bytes);
    }
}

#[derive(Clone)]
pub struct AnomalyMetrics;

impl AnomalyMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_detection(&self, severity: &str, algorithm: &str) {
        metrics::counter!("cherenkov_anomalies_detected_total", 
            "severity" => severity.to_string(),
            "algorithm" => algorithm.to_string()
        ).increment(1);
    }
    
    pub fn record_processing_time(&self, duration_ms: u64) {
        metrics::histogram!("cherenkov_anomaly_processing_ms").record(duration_ms as f64);
    }
    
    pub fn record_false_positive(&self) {
        metrics::counter!("cherenkov_anomaly_false_positives_total").increment(1);
    }
}

#[derive(Clone)]
pub struct ApiMetrics;

impl ApiMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_request(&self, endpoint: &str, method: &str, status: u16) {
        metrics::counter!("cherenkov_api_requests_total",
            "endpoint" => endpoint.to_string(),
            "method" => method.to_string(),
            "status" => status.to_string()
        ).increment(1);
    }
    
    pub fn record_latency(&self, endpoint: &str, latency_ms: u64) {
        metrics::histogram!("cherenkov_api_latency_ms",
            "endpoint" => endpoint.to_string()
        ).record(latency_ms as f64);
    }
    
    pub fn record_active_connections(&self, count: i64) {
        metrics::gauge!("cherenkov_api_active_connections").set(count as f64);
    }
    
    pub fn record_websocket_message(&self, direction: &str) {
        metrics::counter!("cherenkov_websocket_messages_total",
            "direction" => direction.to_string()
        ).increment(1);
    }
}

#[derive(Clone)]
pub struct DatabaseMetrics;

impl DatabaseMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_query(&self, operation: &str, table: &str, latency_ms: u64) {
        metrics::histogram!("cherenkov_db_query_latency_ms",
            "operation" => operation.to_string(),
            "table" => table.to_string()
        ).record(latency_ms as f64);
    }
    
    pub fn record_write(&self, table: &str, rows: u64) {
        metrics::counter!("cherenkov_db_writes_total",
            "table" => table.to_string()
        ).increment(rows);
    }
    
    pub fn record_connection_pool(&self, active: i64, idle: i64) {
        metrics::gauge!("cherenkov_db_connections_active").set(active as f64);
        metrics::gauge!("cherenkov_db_connections_idle").set(idle as f64);
    }
    
    pub fn record_error(&self, error_type: &str) {
        metrics::counter!("cherenkov_db_errors_total",
            "error_type" => error_type.to_string()
        ).increment(1);
    }
}

#[derive(Clone)]
pub struct MlMetrics;

impl MlMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_inference(&self, model: &str, latency_ms: u64, batch_size: usize) {
        metrics::histogram!("cherenkov_ml_inference_latency_ms",
            "model" => model.to_string()
        ).record(latency_ms as f64);
        
        metrics::histogram!("cherenkov_ml_batch_size",
            "model" => model.to_string()
        ).record(batch_size as f64);
    }
    
    pub fn record_prediction(&self, model: &str, confidence: f64) {
        metrics::histogram!("cherenkov_ml_confidence",
            "model" => model.to_string()
        ).record(confidence);
    }
    
    pub fn record_gpu_utilization(&self, device: &str, utilization: f64) {
        metrics::gauge!("cherenkov_ml_gpu_utilization",
            "device" => device.to_string()
        ).set(utilization);
    }
}

#[derive(Clone)]
pub struct PlumeMetrics;

impl PlumeMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_simulation(&self, duration_ms: u64, particles: usize, hours: u32) {
        metrics::histogram!("cherenkov_plume_simulation_duration_ms").record(duration_ms as f64);
        metrics::histogram!("cherenkov_plume_particles").record(particles as f64);
        metrics::histogram!("cherenkov_plume_simulation_hours").record(hours as f64);
    }
    
    pub fn record_active_simulations(&self, count: i64) {
        metrics::gauge!("cherenkov_plume_active_simulations").set(count as f64);
    }
}

pub struct Timer {
    start: Instant,
    name: String,
    labels: Vec<(String, String)>,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
            labels: Vec::new(),
        }
    }
    
    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.push((key.to_string(), value.to_string()));
        self
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.elapsed_ms() as f64;
        if self.labels.is_empty() {
            metrics::histogram!(self.name.clone()).record(elapsed);
        } else {
            let labels: Vec<(&str, &str)> = self.labels
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            metrics::histogram!(self.name.clone(), labels).record(elapsed);
        }
    }
}
