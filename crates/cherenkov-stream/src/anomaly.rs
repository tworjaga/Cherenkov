use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub sensor_id: String,
    pub severity: Severity,
    pub z_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub dose_rate: f64,
    pub baseline: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

pub struct AnomalyDetector {
    windows: std::collections::HashMap<String, SensorWindow>,
}

struct SensorWindow {
    readings: VecDeque<f64>,
    capacity: usize,
    mean: f64,
    variance: f64,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            windows: std::collections::HashMap::new(),
        }
    }
    
    pub fn detect(&mut self, window: Vec<Reading>) -> Option<Anomaly> {
        if window.is_empty() {
            return None;
        }
        
        let sensor_id = window[0].sensor_id.clone();
        let current = window.last().unwrap().dose_rate;
        
        let sensor_window = self.windows.entry(sensor_id.clone()).or_insert_with(|| {
            SensorWindow {
                readings: VecDeque::with_capacity(1000),
                capacity: 1000,
                mean: 0.0,
                variance: 0.0,
            }
        });
        
        // Welford's online algorithm for mean and variance
        for reading in &window {
            sensor_window.update(reading.dose_rate);
        }
        
        let std_dev = sensor_window.variance.sqrt();
        if std_dev == 0.0 {
            return None;
        }
        
        let z_score = (current - sensor_window.mean) / std_dev;
        
        let severity = if z_score > 3.0 {
            Severity::Critical
        } else if z_score > 2.0 {
            Severity::Warning
        } else {
            return None;
        };
        
        Some(Anomaly {
            sensor_id,
            severity,
            z_score,
            timestamp: chrono::Utc::now(),
            dose_rate: current,
            baseline: sensor_window.mean,
        })
    }
}

impl SensorWindow {
    fn update(&mut self, value: f64) {
        if self.readings.len() >= self.capacity {
            self.readings.pop_front();
        }
        self.readings.push_back(value);
        
        let n = self.readings.len() as f64;
        let old_mean = self.mean;
        self.mean = old_mean + (value - old_mean) / n;
        self.variance = ((n - 1.0) * self.variance + (value - old_mean) * (value - self.mean)) / n;
    }
}

#[derive(Clone)]
pub struct Reading {
    pub sensor_id: String,
    pub dose_rate: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
