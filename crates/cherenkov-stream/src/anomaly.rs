use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub sensor_id: String,
    pub severity: Severity,
    pub z_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub dose_rate: f64,
    pub baseline: f64,
    pub algorithm: Algorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    Welford,
    IsolationForest,
}

pub struct AnomalyDetector {
    windows: std::collections::HashMap<String, SensorWindow>,
    isolation_forest: IsolationForest,
}

pub struct IsolationForest {
    trees: Vec<IsolationTree>,
    num_trees: usize,
    subsample_size: usize,
}

struct IsolationTree {
    root: Option<Box<Node>>,
    height_limit: usize,
}

struct Node {
    feature: usize,
    split_value: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    size: usize,
}

impl IsolationForest {
    pub fn new(num_trees: usize, subsample_size: usize) -> Self {
        let height_limit = (subsample_size as f64).log2().ceil() as usize;
        Self {
            trees: (0..num_trees).map(|_| IsolationTree::new(height_limit)).collect(),
            num_trees,
            subsample_size,
        }
    }
    
    pub fn fit(&mut self, data: &[Vec<f64>]) {
        let mut rng = rand::thread_rng();
        for tree in &mut self.trees {
            let sample: Vec<Vec<f64>> = data.choose_multiple(&mut rng, self.subsample_size).cloned().collect();
            tree.root = Some(Box::new(build_tree(&sample, 0, self.subsample_size, &mut rng)));
        }
    }
    
    pub fn anomaly_score(&self, point: &[f64]) -> f64 {
        let path_lengths: Vec<f64> = self.trees.iter()
            .map(|tree| path_length(point, tree.root.as_ref(), 0))
            .collect();
        
        let avg_path_length = path_lengths.iter().sum::<f64>() / path_lengths.len() as f64;
        let expected_length = c(self.subsample_size);
        
        2.0_f64.powf(-avg_path_length / expected_length)
    }
}

impl IsolationTree {
    fn new(height_limit: usize) -> Self {
        Self {
            root: None,
            height_limit,
        }
    }
}

fn build_tree<R: Rng>(data: &[Vec<f64>], current_height: usize, height_limit: usize, rng: &mut R) -> Node {
    if data.len() <= 1 || current_height >= height_limit {
        return Node {
            feature: 0,
            split_value: 0.0,
            left: None,
            right: None,
            size: data.len(),
        };
    }
    
    let num_features = data[0].len();
    let feature = rng.gen_range(0..num_features);
    
    let min_val = data.iter().map(|p| p[feature]).fold(f64::INFINITY, f64::min);
    let max_val = data.iter().map(|p| p[feature]).fold(f64::NEG_INFINITY, f64::max);
    
    let split_value = rng.gen_range(min_val..=max_val);
    
    let (left_data, right_data): (Vec<_>, Vec<_>) = data.iter()
        .cloned()
        .partition(|p| p[feature] < split_value);
    
    Node {
        feature,
        split_value,
        left: if left_data.is_empty() { None } else { Some(Box::new(build_tree(&left_data, current_height + 1, height_limit, rng))) },
        right: if right_data.is_empty() { None } else { Some(Box::new(build_tree(&right_data, current_height + 1, height_limit, rng))) },
        size: data.len(),
    }
}

fn path_length(point: &[f64], node: Option<&Box<Node>>, current_depth: usize) -> f64 {
    match node {
        None => 0.0,
        Some(n) if n.left.is_none() && n.right.is_none() => c(n.size),
        Some(n) => {
            if point[n.feature] < n.split_value {
                path_length(point, n.left.as_ref(), current_depth + 1)
            } else {
                path_length(point, n.right.as_ref(), current_depth + 1)
            }
        }
    }
}

fn c(n: usize) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    2.0 * (n as f64 - 1.0).ln() + 0.5772156649 - 2.0 * (n as f64 - 1.0) / n as f64
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
            isolation_forest: IsolationForest::new(100, 256),
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
            algorithm: Algorithm::Welford,
        })
    }
    
    pub fn detect_isolation_forest(&self, point: &[f64]) -> Option<f64> {
        let score = self.isolation_forest.anomaly_score(point);
        if score > 0.6 {
            Some(score)
        } else {
            None
        }
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
