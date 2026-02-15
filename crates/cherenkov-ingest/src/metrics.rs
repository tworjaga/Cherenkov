use metrics::{counter, histogram, gauge};

pub fn record_ingest(source: &str) {
    counter!("cherenkov_ingest_total", "source" => source.to_string()).increment(1);
}

pub fn record_ingest_latency(source: &str, latency_ms: f64) {
    histogram!("cherenkov_ingest_latency_ms", "source" => source.to_string()).record(latency_ms);
}

pub fn record_active_sensors(count: usize) {
    gauge!("cherenkov_active_sensors").set(count as f64);
}

pub fn record_anomaly_detected(severity: &str) {
    counter!("cherenkov_anomalies_detected", "severity" => severity.to_string()).increment(1);
}
