use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use cherenkov_db::RadiationReading;

/// Sliding window for time-series analysis per sensor
pub struct SlidingWindow {
    window_size: Duration,
    slide_interval: Duration,
    sensor_windows: HashMap<String, SensorWindow>,
}

pub struct SensorWindow {
    readings: VecDeque<TimestampedReading>,
    window_size: Duration,
}

#[derive(Clone, Debug)]
pub struct TimestampedReading {
    pub timestamp: DateTime<Utc>,
    pub dose_rate: f64,
    pub sensor_id: String,
}

/// Reading struct for processor compatibility
#[derive(Clone, Debug)]
pub struct Reading {
    pub timestamp: DateTime<Utc>,
    pub dose_rate: f64,
    pub sensor_id: String,
}

impl SlidingWindow {
    pub fn new(window_size: Duration, slide_interval: Duration) -> Self {
        Self {
            window_size,
            slide_interval,
            sensor_windows: HashMap::new(),
        }
    }

    /// Add a reading to the appropriate sensor window
    pub fn add(&mut self, reading: RadiationReading) {
        let sensor_id = reading.sensor_id.to_string();
        
        let window = self.sensor_windows.entry(sensor_id.clone()).or_insert_with(|| {
            SensorWindow::new(self.window_size)
        });
        
        window.add(TimestampedReading {
            timestamp: DateTime::from_timestamp(reading.timestamp, 0)
                .unwrap_or_else(|| Utc::now()),
            dose_rate: reading.dose_rate_microsieverts,
            sensor_id,
        });
    }

    /// Get window contents for a specific sensor
    pub fn get_window(&self, sensor_id: &str) -> Vec<TimestampedReading> {
        self.sensor_windows
            .get(sensor_id)
            .map(|w| w.get_readings())
            .unwrap_or_default()
    }

    /// Get all sensor IDs in the window
    pub fn sensor_ids(&self) -> Vec<String> {
        self.sensor_windows.keys().cloned().collect()
    }

    /// Clean up expired windows
    pub fn cleanup(&mut self) {
        let now = Utc::now();
        for window in self.sensor_windows.values_mut() {
            window.cleanup(now, self.window_size);
        }
        
        // Remove empty windows
        self.sensor_windows.retain(|_, w| !w.is_empty());
    }

    /// Add a Reading to the window (for processor compatibility)
    pub fn add_reading(&mut self, reading: Reading) {
        let sensor_id = reading.sensor_id.clone();
        
        let window = self.sensor_windows.entry(sensor_id).or_insert_with(|| {
            SensorWindow::new(self.window_size)
        });
        
        window.add(TimestampedReading {
            timestamp: reading.timestamp,
            dose_rate: reading.dose_rate,
            sensor_id: reading.sensor_id,
        });
    }

    /// Check if window is stale (no recent data)
    pub fn is_stale(&self, max_age_secs: i64) -> bool {
        let now = Utc::now();
        let cutoff = now - chrono::Duration::seconds(max_age_secs);
        
        self.sensor_windows.values().all(|w| {
            w.readings.back().map_or(true, |r| r.timestamp < cutoff)
        })
    }
}

impl SensorWindow {
    fn new(window_size: Duration) -> Self {
        Self {
            readings: VecDeque::new(),
            window_size,
        }
    }

    fn add(&mut self, reading: TimestampedReading) {
        self.readings.push_back(reading);
        self.cleanup(Utc::now(), self.window_size);
    }

    fn get_readings(&self) -> Vec<TimestampedReading> {
        self.readings.iter().cloned().collect()
    }

    fn cleanup(&mut self, now: DateTime<Utc>, window_size: Duration) {
        let cutoff = now - chrono::Duration::from_std(window_size).unwrap_or_else(|_| chrono::Duration::hours(1));
        
        while let Some(front) = self.readings.front() {
            if front.timestamp < cutoff {
                self.readings.pop_front();
            } else {
                break;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }
}

/// Thread-safe sliding window for concurrent access
pub struct ConcurrentSlidingWindow {
    inner: Arc<RwLock<SlidingWindow>>,
}

impl ConcurrentSlidingWindow {
    pub fn new(window_size: Duration, slide_interval: Duration) -> Self {
        Self {
            inner: Arc::new(RwLock::new(SlidingWindow::new(window_size, slide_interval))),
        }
    }

    pub async fn add(&self, reading: RadiationReading) {
        let mut guard = self.inner.write().await;
        guard.add(reading);
    }

    pub async fn get_window(&self, sensor_id: &str) -> Vec<TimestampedReading> {
        let guard = self.inner.read().await;
        guard.get_window(sensor_id)
    }

    pub async fn cleanup(&self) {
        let mut guard = self.inner.write().await;
        guard.cleanup();
    }
}

/// Tumbling window for fixed-size batching
pub struct TumblingWindow {
    size_secs: u64,
}

impl TumblingWindow {
    pub fn new(size_secs: u64) -> Self {
        Self { size_secs }
    }
    
    pub fn size(&self) -> Duration {
        Duration::from_secs(self.size_secs)
    }
}

pub struct WindowedStream<T> {
    buffer: Vec<T>,
    window_size: Duration,
    last_emit: std::time::Instant,
}

impl<T> WindowedStream<T> {
    pub fn new(window_size: Duration) -> Self {
        Self {
            buffer: Vec::new(),
            window_size,
            last_emit: std::time::Instant::now(),
        }
    }
    
    pub fn push(&mut self, item: T) -> Option<Vec<T>> {
        self.buffer.push(item);
        
        if self.last_emit.elapsed() >= self.window_size {
            self.last_emit = std::time::Instant::now();
            Some(std::mem::take(&mut self.buffer))
        } else {
            None
        }
    }
}
