//! Cherenkov Stream Processing Library
//! 
//! Real-time stream processing for anomaly detection and correlation analysis.

pub mod anomaly;
pub mod correlation;
pub mod processor;
pub mod window;

pub use anomaly::{AnomalyDetector, AnomalyResult, DetectionConfig};
pub use correlation::{CorrelationEngine, CorrelationResult, CorrelationConfig};
pub use processor::{StreamProcessor, ProcessingConfig};
pub use window::{TimeWindow, WindowConfig, WindowedData};

use thiserror::Error;

/// Stream processing errors
#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Processing error: {0}")]
    Processing(String),
    
    #[error("Window error: {0}")]
    Window(String),
    
    #[error("Correlation error: {0}")]
    Correlation(String),
    
    #[error("Anomaly detection error: {0}")]
    Anomaly(String),
}

/// Result type for stream operations
pub type Result<T> = std::result::Result<T, StreamError>;
