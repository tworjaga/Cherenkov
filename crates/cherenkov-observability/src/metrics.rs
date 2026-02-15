use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

pub fn init_prometheus_exporter() {
    let builder = PrometheusBuilder::new();
    builder
        .install_recorder()
        .expect("Failed to install Prometheus recorder");
}

pub fn start_metrics_server(addr: SocketAddr) {
    let builder = PrometheusBuilder::new();
    builder
        .with_http_listener(addr)
        .install_recorder()
        .expect("Failed to start metrics server");
}

#[derive(Clone)]
pub struct IngestMetrics;

impl IngestMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_reading(&self, source: &str) {
        metrics::counter!("cherenkov_ingest_readings_total", "source" => source.to_string()).increment(1);
    }
    
    pub fn record_latency(&self, source: &str, latency_ms: u64) {
        metrics::histogram!("cherenkov_ingest_latency_ms", "source" => source.to_string()).record(latency_ms as f64);
    }
    
    pub fn record_error(&self, source: &str, error_type: &str) {
        metrics::counter!("cherenkov_ingest_errors_total", 
            "source" => source.to_string(),
            "error_type" => error_type.to_string()
        ).increment(1);
    }
}

#[derive(Clone)]
pub struct AnomalyMetrics;

impl AnomalyMetrics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn record_detection(&self, severity: &str) {
        metrics::counter!("cherenkov_anomalies_detected_total", "severity" => severity.to_string()).increment(1);
    }
    
    pub fn record_processing_time(&self, duration_ms: u64) {
        metrics::histogram!("cherenkov_anomaly_processing_ms").record(duration_ms as f64);
    }
}
