use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use candle_onnx::onnx::ModelProto;

use crate::{OnnxModel, OnnxError, ModelMetadata};


/// Model registry errors
#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Version not found: {0}@{1}")]
    VersionNotFound(String, String),
    
    #[error("Hot swap failed: {0}")]
    HotSwapFailed(String),
    
    #[error("Rollback failed: {0}")]
    RollbackFailed(String),
    
    #[error("Performance validation failed: {0}")]
    PerformanceValidationFailed(String),
}

/// Model version information with ONNX metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersionInfo {
    pub version: String,
    pub path: String,
    pub metadata: ModelMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub onnx_metadata: OnnxModelMetadata,
    pub performance_baseline: Option<PerformanceMetrics>,
}

/// ONNX-specific metadata extracted from model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnnxModelMetadata {
    pub ir_version: i64,
    pub opset_version: i64,
    pub producer_name: String,
    pub producer_version: String,
    pub domain: String,
    pub model_version: i64,
    pub doc_string: String,
    pub graph_name: String,
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub num_nodes: usize,
    pub num_initializers: usize,
}

impl From<&ModelProto> for OnnxModelMetadata {
    fn from(model: &ModelProto) -> Self {

        let graph = model.graph.as_ref();
        
        Self {
            ir_version: model.ir_version,
            opset_version: model.opset_import.first()
                .map(|o| o.version)
                .unwrap_or(0),
            producer_name: model.producer_name.clone(),
            producer_version: model.producer_version.clone(),
            domain: model.domain.clone(),
            model_version: model.model_version,
            doc_string: model.doc_string.clone(),
            graph_name: graph.map(|g| g.name.clone()).unwrap_or_default(),
            num_inputs: graph.map(|g| g.input.len()).unwrap_or(0),
            num_outputs: graph.map(|g| g.output.len()).unwrap_or(0),
            num_nodes: graph.map(|g| g.node.len()).unwrap_or(0),
            num_initializers: graph.map(|g| g.initializer.len()).unwrap_or(0),
        }
    }
}

/// Performance metrics for model monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub throughput_per_sec: f64,
    pub error_rate: f64,
    pub total_inferences: u64,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            avg_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
            throughput_per_sec: 0.0,
            error_rate: 0.0,
            total_inferences: 0,
            recorded_at: chrono::Utc::now(),
        }
    }
    
    pub fn meets_baseline(&self, baseline: &PerformanceMetrics, tolerance: f64) -> bool {
        let latency_ok = self.avg_latency_ms <= baseline.avg_latency_ms * (1.0 + tolerance);
        let error_rate_ok = self.error_rate <= baseline.error_rate * (1.0 + tolerance);
        let throughput_ok = self.throughput_per_sec >= baseline.throughput_per_sec * (1.0 - tolerance);
        
        latency_ok && error_rate_ok && throughput_ok
    }
}

/// Active model entry with runtime statistics
struct ActiveModel {
    model: Arc<OnnxModel>,
    version_info: ModelVersionInfo,
    loaded_at: Instant,
    inference_count: std::sync::atomic::AtomicU64,
    error_count: std::sync::atomic::AtomicU64,
    total_latency_ms: std::sync::atomic::AtomicU64,
}

/// Model registry with versioning and hot-swap support
pub struct ModelRegistry {
    models: Arc<RwLock<HashMap<String, Vec<ModelVersionInfo>>>>,
    active_models: Arc<RwLock<HashMap<String, ActiveModel>>>,
    previous_versions: Arc<RwLock<HashMap<String, ModelVersionInfo>>>,
    device: candle_core::Device,
    performance_tolerance: f64,
}

impl ModelRegistry {
    pub fn new(device: candle_core::Device) -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            active_models: Arc::new(RwLock::new(HashMap::new())),
            previous_versions: Arc::new(RwLock::new(HashMap::new())),
            device,
            performance_tolerance: 0.1, // 10% tolerance
        }
    }
    
    /// Register a new model version with enhanced ONNX metadata
    pub async fn register_version(
        &self,
        name: &str,
        path: &str,
        version: &str,
    ) -> Result<ModelVersionInfo, RegistryError> {
        info!("Registering model {} version {} from {}", name, version, path);
        
        // Load model to extract metadata
        let model = OnnxModel::load(path, &self.device)
            .map_err(|e| RegistryError::HotSwapFailed(format!("Failed to load model: {}", e)))?;
        
        let onnx_metadata = OnnxModelMetadata::from(&model.model);
        let metadata = model.metadata().clone();
        
        let version_info = ModelVersionInfo {
            version: version.to_string(),
            path: path.to_string(),
            metadata,
            created_at: chrono::Utc::now(),
            onnx_metadata,
            performance_baseline: None,
        };
        
        let mut models = self.models.write().await;
        let versions = models.entry(name.to_string()).or_insert_with(Vec::new);
        
        // Check if version already exists
        if versions.iter().any(|v| v.version == version) {
            return Err(RegistryError::HotSwapFailed(
                format!("Version {} already exists for model {}", version, name)
            ));
        }
        
        versions.push(version_info.clone());
        versions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        info!("Registered model {} version {}", name, version);
        Ok(version_info)
    }
    
    /// Atomically hot-swap to a new model version with rollback support
    pub async fn hot_swap(
        &self,
        name: &str,
        version: &str,
    ) -> Result<ModelVersionInfo, RegistryError> {
        info!("Initiating hot-swap for model {} to version {}", name, version);
        
        // Get version info
        let version_info = {
            let models = self.models.read().await;
            let versions = models.get(name)
                .ok_or_else(|| RegistryError::ModelNotFound(name.to_string()))?;
            
            versions.iter()
                .find(|v| v.version == version)
                .cloned()
                .ok_or_else(|| RegistryError::VersionNotFound(name.to_string(), version.to_string()))?
        };
        
        // Store current version for potential rollback
        let mut previous_versions = self.previous_versions.write().await;
        if let Some(active) = self.active_models.read().await.get(name) {
            previous_versions.insert(name.to_string(), active.version_info.clone());
        }
        
        // Load new model
        let model = OnnxModel::load(&version_info.path, &self.device)
            .map_err(|e| RegistryError::HotSwapFailed(format!("Failed to load new model: {}", e)))?;
        
        // Validate model compatibility
        self.validate_compatibility(name, &model).await?;
        
        // Perform atomic swap
        let mut active_models = self.active_models.write().await;
        let new_active = ActiveModel {
            model: Arc::new(model),
            version_info: version_info.clone(),
            loaded_at: Instant::now(),
            inference_count: std::sync::atomic::AtomicU64::new(0),
            error_count: std::sync::atomic::AtomicU64::new(0),
            total_latency_ms: std::sync::atomic::AtomicU64::new(0),
        };
        
        active_models.insert(name.to_string(), new_active);
        
        info!("Hot-swap completed for model {} to version {}", name, version);
        Ok(version_info)
    }
    
    /// Rollback to previous version if hot-swap fails
    pub async fn rollback(&self, name: &str) -> Result<ModelVersionInfo, RegistryError> {
        info!("Initiating rollback for model {}", name);
        
        let previous = {
            let mut previous_versions = self.previous_versions.write().await;
            previous_versions.remove(name)
        };
        
        if let Some(version_info) = previous {
            let model = OnnxModel::load(&version_info.path, &self.device)
                .map_err(|e| RegistryError::RollbackFailed(format!("Failed to load previous version: {}", e)))?;
            
            let mut active_models = self.active_models.write().await;
            let new_active = ActiveModel {
                model: Arc::new(model),
                version_info: version_info.clone(),
                loaded_at: Instant::now(),
                inference_count: std::sync::atomic::AtomicU64::new(0),
                error_count: std::sync::atomic::AtomicU64::new(0),
                total_latency_ms: std::sync::atomic::AtomicU64::new(0),
            };
            
            active_models.insert(name.to_string(), new_active);
            
            info!("Rollback completed for model {} to version {}", name, version_info.version);
            Ok(version_info)
        } else {
            Err(RegistryError::RollbackFailed("No previous version available".to_string()))
        }
    }
    
    /// Validate model compatibility before hot-swap
    async fn validate_compatibility(
        &self,
        name: &str,
        new_model: &OnnxModel,
    ) -> Result<(), RegistryError> {
        if let Some(active) = self.active_models.read().await.get(name) {
            let current_metadata = &active.version_info.metadata;
            let new_metadata = new_model.metadata();
            
            // Check input/output compatibility
            if current_metadata.input_shapes.len() != new_metadata.input_shapes.len() {
                return Err(RegistryError::HotSwapFailed(
                    "Input count mismatch".to_string()
                ));
            }
            
            if current_metadata.output_shapes.len() != new_metadata.output_shapes.len() {
                return Err(RegistryError::HotSwapFailed(
                    "Output count mismatch".to_string()
                ));
            }
            
            info!("Compatibility check passed for model {}", name);
        }
        
        Ok(())
    }
    
    /// Get active model
    pub async fn get_active(&self, name: &str) -> Option<(Arc<OnnxModel>, ModelVersionInfo)> {
        let active_models = self.active_models.read().await;
        active_models.get(name).map(|m| (Arc::clone(&m.model), m.version_info.clone()))
    }
    
    /// Record inference metrics for performance monitoring
    pub async fn record_inference(
        &self,
        name: &str,
        latency_ms: u64,
        success: bool,
    ) {
        if let Some(active) = self.active_models.write().await.get_mut(name) {
            active.inference_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            active.total_latency_ms.fetch_add(latency_ms, std::sync::atomic::Ordering::Relaxed);
            
            if !success {
                active.error_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
    
    /// Get current performance metrics
    pub async fn get_performance_metrics(&self, name: &str) -> Option<PerformanceMetrics> {
        let active_models = self.active_models.read().await;
        let active = active_models.get(name)?;
        
        let total_inferences = active.inference_count.load(std::sync::atomic::Ordering::Relaxed);
        let total_errors = active.error_count.load(std::sync::atomic::Ordering::Relaxed);
        let total_latency = active.total_latency_ms.load(std::sync::atomic::Ordering::Relaxed);
        
        let uptime_secs = active.loaded_at.elapsed().as_secs_f64();
        
        let avg_latency = if total_inferences > 0 {
            total_latency as f64 / total_inferences as f64
        } else {
            0.0
        };
        
        let throughput = if uptime_secs > 0.0 {
            total_inferences as f64 / uptime_secs
        } else {
            0.0
        };
        
        let error_rate = if total_inferences > 0 {
            total_errors as f64 / total_inferences as f64
        } else {
            0.0
        };
        
        Some(PerformanceMetrics {
            avg_latency_ms: avg_latency,
            p95_latency_ms: avg_latency * 1.5, // Simplified estimation
            p99_latency_ms: avg_latency * 2.0,
            throughput_per_sec: throughput,
            error_rate,
            total_inferences,
            recorded_at: chrono::Utc::now(),
        })
    }
    
    /// Validate current performance against baseline
    pub async fn validate_performance(&self, name: &str) -> Result<bool, RegistryError> {
        let metrics = self.get_performance_metrics(name).await
            .ok_or_else(|| RegistryError::ModelNotFound(name.to_string()))?;
        
        let active_models = self.active_models.read().await;
        let active = active_models.get(name)
            .ok_or_else(|| RegistryError::ModelNotFound(name.to_string()))?;
        
        if let Some(baseline) = &active.version_info.performance_baseline {
            let valid = metrics.meets_baseline(baseline, self.performance_tolerance);
            
            if !valid {
                warn!("Performance validation failed for model {}", name);
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Set performance baseline for current version
    pub async fn set_performance_baseline(&self, name: &str) -> Result<(), RegistryError> {
        let metrics = self.get_performance_metrics(name).await
            .ok_or_else(|| RegistryError::ModelNotFound(name.to_string()))?;
        
        let mut models = self.models.write().await;
        let versions = models.get_mut(name)
            .ok_or_else(|| RegistryError::ModelNotFound(name.to_string()))?;
        
        let active = self.active_models.read().await;
        let current_version = &active.get(name).unwrap().version_info.version;
        
        for version in versions.iter_mut() {
            if version.version == *current_version {
                version.performance_baseline = Some(metrics.clone());
                info!("Set performance baseline for {}@{}", name, current_version);
                break;
            }
        }
        
        Ok(())
    }
    
    /// List all registered models and versions
    pub async fn list_models(&self) -> HashMap<String, Vec<ModelVersionInfo>> {
        self.models.read().await.clone()
    }
    
    /// Get active model info
    pub async fn get_active_info(&self, name: &str) -> Option<ModelVersionInfo> {
        self.active_models.read().await.get(name).map(|m| m.version_info.clone())
    }
    
    /// Get model uptime
    pub async fn get_uptime(&self, name: &str) -> Option<Duration> {
        self.active_models.read().await.get(name).map(|m| m.loaded_at.elapsed())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_performance_metrics_meets_baseline() {
        let baseline = PerformanceMetrics {
            avg_latency_ms: 100.0,
            p95_latency_ms: 150.0,
            p99_latency_ms: 200.0,
            throughput_per_sec: 1000.0,
            error_rate: 0.01,
            total_inferences: 10000,
            recorded_at: chrono::Utc::now(),
        };
        
        // Within tolerance
        let current = PerformanceMetrics {
            avg_latency_ms: 105.0, // 5% increase, within 10% tolerance
            p95_latency_ms: 155.0,
            p99_latency_ms: 205.0,
            throughput_per_sec: 950.0, // 5% decrease, within 10% tolerance
            error_rate: 0.01,
            total_inferences: 10000,
            recorded_at: chrono::Utc::now(),
        };
        
        assert!(current.meets_baseline(&baseline, 0.1));
        
        // Exceeds tolerance
        let bad = PerformanceMetrics {
            avg_latency_ms: 120.0, // 20% increase, exceeds 10% tolerance
            p95_latency_ms: 150.0,
            p99_latency_ms: 200.0,
            throughput_per_sec: 1000.0,
            error_rate: 0.01,
            total_inferences: 10000,
            recorded_at: chrono::Utc::now(),
        };
        
        assert!(!bad.meets_baseline(&baseline, 0.1));
    }
}
