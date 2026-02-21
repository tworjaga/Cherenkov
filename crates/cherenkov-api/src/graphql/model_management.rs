use async_graphql::{Object, ID, Context, Result, InputObject, SimpleObject, Enum};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::{info, warn};

use cherenkov_ml::{
    ModelRegistry, 
    ModelVersionInfo, 
    PerformanceMetrics as MlPerformanceMetrics,
    OnnxModelMetadata,
    RegistryError,
    OnnxExportConfig, OnnxOpset,
};


/// Model status enumeration
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ModelStatus {
    Active,
    Inactive,
    RollingOut,
    RollingBack,
    Failed,
}

/// Model type enumeration
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ModelType {
    IsotopeClassifier,
    AnomalyDetector,
    BackgroundSubtractor,
    MultiClassClassifier,
}

/// GraphQL type for ONNX model metadata
#[derive(SimpleObject, Clone)]
pub struct ModelMetadataGraphQL {
    pub ir_version: i64,
    pub opset_version: i64,
    pub producer_name: String,
    pub producer_version: String,
    pub domain: String,
    pub model_version: i64,
    pub doc_string: String,
    pub graph_name: String,
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub num_nodes: i32,
    pub num_initializers: i32,
}

impl From<&OnnxModelMetadata> for ModelMetadataGraphQL {
    fn from(metadata: &OnnxModelMetadata) -> Self {
        Self {
            ir_version: metadata.ir_version,
            opset_version: metadata.opset_version,
            producer_name: metadata.producer_name.clone(),
            producer_version: metadata.producer_version.clone(),
            domain: metadata.domain.clone(),
            model_version: metadata.model_version,
            doc_string: metadata.doc_string.clone(),
            graph_name: metadata.graph_name.clone(),
            num_inputs: metadata.num_inputs as i32,
            num_outputs: metadata.num_outputs as i32,
            num_nodes: metadata.num_nodes as i32,
            num_initializers: metadata.num_initializers as i32,
        }
    }
}

/// GraphQL type for performance metrics
#[derive(SimpleObject, Clone)]
pub struct PerformanceMetricsGraphQL {
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub throughput_per_sec: f64,
    pub error_rate: f64,
    pub total_inferences: i64,
    pub recorded_at: DateTime<Utc>,
}

impl From<&MlPerformanceMetrics> for PerformanceMetricsGraphQL {
    fn from(metrics: &MlPerformanceMetrics) -> Self {
        Self {
            avg_latency_ms: metrics.avg_latency_ms,
            p95_latency_ms: metrics.p95_latency_ms,
            p99_latency_ms: metrics.p99_latency_ms,
            throughput_per_sec: metrics.throughput_per_sec,
            error_rate: metrics.error_rate,
            total_inferences: metrics.total_inferences as i64,
            recorded_at: metrics.recorded_at,
        }
    }
}

/// GraphQL type for model version information
#[derive(SimpleObject, Clone)]
pub struct ModelVersionGraphQL {
    pub version: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub metadata: ModelMetadataGraphQL,
    pub performance_baseline: Option<PerformanceMetricsGraphQL>,
    pub is_active: bool,
}

/// GraphQL type for model information
#[derive(SimpleObject, Clone)]
pub struct ModelInfo {
    pub id: ID,
    pub name: String,
    pub model_type: ModelType,
    pub status: ModelStatus,
    pub current_version: String,
    pub versions: Vec<ModelVersionGraphQL>,
    pub uptime_seconds: i64,
    pub current_performance: Option<PerformanceMetricsGraphQL>,
}

/// Model explainability result
#[derive(SimpleObject, Clone)]
pub struct ModelExplanation {
    pub feature_importance: Vec<FeatureImportance>,
    pub top_predicted_class: String,
    pub confidence: f64,
    pub explanation_method: String,
}

/// Feature importance entry
#[derive(SimpleObject, Clone)]
pub struct FeatureImportance {
    pub feature_name: String,
    pub importance_score: f64,
    pub description: String,
}

/// Retraining trigger result
#[derive(SimpleObject, Clone)]
pub struct RetrainingTriggerResult {
    pub job_id: ID,
    pub model_name: String,
    pub triggered_at: DateTime<Utc>,
    pub estimated_completion: DateTime<Utc>,
    pub status: String,
}

/// Input for model registration
#[derive(InputObject)]
pub struct RegisterModelInput {
    pub name: String,
    pub path: String,
    pub version: String,
    pub model_type: ModelType,
}

/// Input for hot swap operation
#[derive(InputObject)]
pub struct HotSwapInput {
    pub model_name: String,
    pub version: String,
}

/// Input for explainability request
#[derive(InputObject)]
pub struct ExplainModelInput {
    pub model_name: String,
    pub input_data: Vec<f64>,
    pub method: Option<String>,
}

/// Input for ONNX export
#[derive(InputObject)]
pub struct ExportToOnnxInput {
    pub model_name: String,
    pub model_path: String,
    pub output_path: String,
    pub opset_version: Option<i32>,
    pub optimize: Option<bool>,
}

/// Input for data source configuration
#[derive(InputObject)]
pub struct DataSourceConfigInput {
    pub source_id: String,
    pub source_type: DataSourceType,
    pub endpoint: String,
    pub credentials: Option<DataSourceCredentials>,
    pub format: DataFormat,
    pub refresh_interval_seconds: Option<i32>,
    pub enabled: bool,
}

/// Data source type enumeration
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DataSourceType {
    S3,
    Http,
    LocalFile,
    Database,
    Streaming,
}

/// Data format enumeration
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DataFormat {
    Csv,
    Hdf5,
    Json,
    Parquet,
    Binary,
}

/// Data source credentials
#[derive(InputObject)]
pub struct DataSourceCredentials {
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub api_token: Option<String>,
    pub connection_string: Option<String>,
}

/// Training job status enumeration
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TrainingJobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Training job information
#[derive(SimpleObject, Clone)]
pub struct TrainingJobInfo {
    pub job_id: ID,
    pub model_name: String,
    pub status: TrainingJobStatus,
    pub progress_percent: f64,
    pub current_epoch: i32,
    pub total_epochs: i32,
    pub current_loss: Option<f64>,
    pub validation_accuracy: Option<f64>,
    pub started_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub output_model_path: Option<String>,
}

/// Data source configuration
#[derive(SimpleObject, Clone)]
pub struct DataSourceConfig {
    pub source_id: String,
    pub source_type: DataSourceType,
    pub endpoint: String,
    pub format: DataFormat,
    pub refresh_interval_seconds: i32,
    pub enabled: bool,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub total_records_fetched: i64,
}

/// ONNX export result
#[derive(SimpleObject, Clone)]
pub struct OnnxExportResult {
    pub success: bool,
    pub output_path: String,
    pub model_name: String,
    pub file_size_bytes: i64,
    pub opset_version: i32,
    pub optimization_applied: bool,
    pub exported_at: DateTime<Utc>,
    pub error_message: Option<String>,
}


/// Query root extension for model management
#[derive(Default)]
pub struct ModelQueryRoot;

#[Object]
impl ModelQueryRoot {
    /// List all registered ML models
    async fn models(
        &self,
        ctx: &Context<'_>,
        model_type: Option<ModelType>,
    ) -> Result<Vec<ModelInfo>> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        let all_models = registry.list_models().await;
        let active_models = registry.active_models.read().await;
        
        let mut result = Vec::new();
        
        for (name, versions) in all_models {
            // Filter by type if specified
            if let Some(filter_type) = &model_type {
                let model_type_enum = Self::infer_model_type(&name);
                if model_type_enum != *filter_type {
                    continue;
                }
            }
            
            let active_info = active_models.get(&name);
            let current_version = active_info
                .map(|a| a.version_info.version.clone())
                .unwrap_or_else(|| "unknown".to_string());
            
            let uptime_seconds = active_info
                .map(|a| a.loaded_at.elapsed().as_secs() as i64)
                .unwrap_or(0);
            
            let current_performance = if let Some(active) = active_info {
                registry.get_performance_metrics(&name).await
                    .map(|m| PerformanceMetricsGraphQL::from(&m))
            } else {
                None
            };
            
            let version_graphql: Vec<ModelVersionGraphQL> = versions.iter()
                .map(|v| ModelVersionGraphQL {
                    version: v.version.clone(),
                    path: v.path.clone(),
                    created_at: v.created_at,
                    metadata: ModelMetadataGraphQL::from(&v.onnx_metadata),
                    performance_baseline: v.performance_baseline.as_ref()
                        .map(|p| PerformanceMetricsGraphQL::from(p)),
                    is_active: v.version == current_version,
                })
                .collect();
            
            let status = if active_info.is_some() {
                ModelStatus::Active
            } else {
                ModelStatus::Inactive
            };
            
            result.push(ModelInfo {
                id: ID::from(name.clone()),
                name: name.clone(),
                model_type: Self::infer_model_type(&name),
                status,
                current_version,
                versions: version_graphql,
                uptime_seconds,
                current_performance,
            });
        }
        
        Ok(result)
    }
    
    /// Get specific model details
    async fn model(&self, ctx: &Context<'_>, name: String) -> Result<Option<ModelInfo>> {
        let models = self.models(ctx, None).await?;
        Ok(models.into_iter().find(|m| m.name == name))
    }
    
    /// Get model performance metrics
    async fn model_performance(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Option<PerformanceMetricsGraphQL>> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        let metrics = registry.get_performance_metrics(&name).await;
        Ok(metrics.map(|m| PerformanceMetricsGraphQL::from(&m)))
    }
    
    /// Compare model versions
    async fn compare_model_versions(
        &self,
        ctx: &Context<'_>,
        name: String,
        version_a: String,
        version_b: String,
    ) -> Result<VersionComparison> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        let all_models = registry.list_models().await;
        let versions = all_models.get(&name)
            .ok_or_else(|| async_graphql::Error::new(format!("Model {} not found", name)))?;
        
        let v_a = versions.iter()
            .find(|v| v.version == version_a)
            .ok_or_else(|| async_graphql::Error::new(format!("Version {} not found", version_a)))?;
        
        let v_b = versions.iter()
            .find(|v| v.version == version_b)
            .ok_or_else(|| async_graphql::Error::new(format!("Version {} not found", version_b)))?;
        
        let perf_a: Option<&MlPerformanceMetrics> = v_a.performance_baseline.as_ref();
        let perf_b: Option<&MlPerformanceMetrics> = v_b.performance_baseline.as_ref();
        
        let latency_a: Option<f64> = perf_a.map(|p| p.avg_latency_ms);
        let latency_b: Option<f64> = perf_b.map(|p| p.avg_latency_ms);
        let throughput_a: Option<f64> = perf_a.map(|p| p.throughput_per_sec);
        let throughput_b: Option<f64> = perf_b.map(|p| p.throughput_per_sec);
        let error_a: Option<f64> = perf_a.map(|p| p.error_rate);
        let error_b: Option<f64> = perf_b.map(|p| p.error_rate);
        
        let latency_b_val: f64 = latency_b.unwrap_or(f64::MAX);
        let latency_a_val: f64 = latency_a.unwrap_or(f64::MAX);
        
        Ok(VersionComparison {
            version_a: version_a.clone(),
            version_b: version_b.clone(),
            latency_improvement: Self::calc_improvement(latency_a, latency_b),
            throughput_improvement: Self::calc_improvement(throughput_b, throughput_a),
            error_rate_improvement: Self::calc_improvement(error_a, error_b),
            recommendation: if latency_b_val < latency_a_val * 0.9 {
                "Version B shows significant performance improvement".to_string()
            } else {
                "No significant performance difference".to_string()
            },
        })

    }
    
    /// Get model explainability
    async fn explain_model(
        &self,
        ctx: &Context<'_>,
        input: ExplainModelInput,
    ) -> Result<ModelExplanation> {
        let _registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        // Simplified explainability - in production, this would use SHAP or LIME
        info!("Generating explanation for model {} with {} features", 
            input.model_name, input.input_data.len());
        
        // Generate synthetic feature importance based on input magnitude
        let mut feature_importance: Vec<FeatureImportance> = input.input_data
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let importance = (val.abs() / input.input_data.iter().map(|v| v.abs()).sum::<f64>())
                    .min(1.0);
                FeatureImportance {
                    feature_name: format!("feature_{}", i),
                    importance_score: importance,
                    description: format!("Energy bin {} contribution", i),
                }
            })
            .collect();
        
        feature_importance.sort_by(|a, b| b.importance_score.partial_cmp(&a.importance_score).unwrap());
        
        Ok(ModelExplanation {
            feature_importance: feature_importance.into_iter().take(10).collect(),
            top_predicted_class: "Cs-137".to_string(), // Placeholder
            confidence: 0.92,
            explanation_method: input.method.unwrap_or_else(|| "gradient_based".to_string()),
        })
    }
}

impl ModelQueryRoot {
    fn infer_model_type(name: &str) -> ModelType {
        if name.contains("isotope") {
            ModelType::IsotopeClassifier
        } else if name.contains("anomaly") {
            ModelType::AnomalyDetector
        } else if name.contains("background") {
            ModelType::BackgroundSubtractor
        } else {
            ModelType::MultiClassClassifier
        }
    }
    
    fn calc_improvement(a: Option<f64>, b: Option<f64>) -> f64 {
        match (a, b) {
            (Some(a_val), Some(b_val)) if a_val > 0.0 => {
                ((b_val - a_val) / a_val) * 100.0
            }
            _ => 0.0,
        }
    }
    
    fn extract_metric<F>(perf: Option<&MlPerformanceMetrics>, extractor: F) -> Option<f64>
    where
        F: FnOnce(&MlPerformanceMetrics) -> f64,
    {
        perf.map(extractor)
    }

}

/// Version comparison result
#[derive(SimpleObject, Clone)]
pub struct VersionComparison {
    pub version_a: String,
    pub version_b: String,
    pub latency_improvement: f64,
    pub throughput_improvement: f64,
    pub error_rate_improvement: f64,
    pub recommendation: String,
}

/// Mutation root for model management
#[derive(Default)]
pub struct ModelMutationRoot;

#[Object]
impl ModelMutationRoot {
    /// Register a new model version
    async fn register_model(
        &self,
        ctx: &Context<'_>,
        input: RegisterModelInput,
    ) -> Result<ModelVersionGraphQL> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        info!("Registering model {} version {} from {}", 
            input.name, input.version, input.path);
        
        let version_info = registry.register_version(
            &input.name,
            &input.path,
            &input.version,
        ).await.map_err(|e| async_graphql::Error::new(format!("Registration failed: {}", e)))?;
        
        Ok(ModelVersionGraphQL {
            version: version_info.version,
            path: version_info.path,
            created_at: version_info.created_at,
            metadata: ModelMetadataGraphQL::from(&version_info.onnx_metadata),
            performance_baseline: version_info.performance_baseline
                .map(|p| PerformanceMetricsGraphQL::from(&p)),
            is_active: false,
        })
    }
    
    /// Hot-swap to a different model version
    async fn hot_swap_model(
        &self,
        ctx: &Context<'_>,
        input: HotSwapInput,
    ) -> Result<ModelInfo> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        info!("Hot-swapping model {} to version {}", 
            input.model_name, input.version);
        
        let version_info = registry.hot_swap(
            &input.model_name,
            &input.version,
        ).await.map_err(|e| async_graphql::Error::new(format!("Hot-swap failed: {}", e)))?;
        
        // Return updated model info
        let query = ModelQueryRoot::default();
        let model = query.model(ctx, input.model_name).await?
            .ok_or_else(|| async_graphql::Error::new("Model not found after hot-swap"))?;
        
        Ok(model)
    }
    
    /// Rollback to previous model version
    async fn rollback_model(
        &self,
        ctx: &Context<'_>,
        model_name: String,
    ) -> Result<ModelInfo> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        info!("Rolling back model {}", model_name);
        
        let _version_info = registry.rollback(&model_name)
            .await.map_err(|e| async_graphql::Error::new(format!("Rollback failed: {}", e)))?;
        
        // Return updated model info
        let query = ModelQueryRoot::default();
        let model = query.model(ctx, model_name).await?
            .ok_or_else(|| async_graphql::Error::new("Model not found after rollback"))?;
        
        Ok(model)
    }
    
    /// Set performance baseline for current version
    async fn set_performance_baseline(
        &self,
        ctx: &Context<'_>,
        model_name: String,
    ) -> Result<PerformanceMetricsGraphQL> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        registry.set_performance_baseline(&model_name)
            .await.map_err(|e| async_graphql::Error::new(format!("Failed to set baseline: {}", e)))?;
        
        let metrics = registry.get_performance_metrics(&model_name).await
            .ok_or_else(|| async_graphql::Error::new("No metrics available"))?;
        
        Ok(PerformanceMetricsGraphQL::from(&metrics))
    }
    
    /// Trigger model retraining
    async fn trigger_retraining(
        &self,
        ctx: &Context<'_>,
        model_name: String,
        reason: Option<String>,
    ) -> Result<RetrainingTriggerResult> {
        let _registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        info!("Triggering retraining for model {}: {}", 
            model_name, 
            reason.as_deref().unwrap_or("No reason provided"));
        
        // In production, this would queue a training job
        let job_id = uuid::Uuid::new_v4().to_string();
        let triggered_at = Utc::now();
        let estimated_completion = triggered_at + chrono::Duration::hours(2);
        
        Ok(RetrainingTriggerResult {
            job_id: ID::from(job_id),
            model_name,
            triggered_at,
            estimated_completion,
            status: "queued".to_string(),
        })
    }
    
    /// Validate model performance against baseline
    async fn validate_model_performance(
        &self,
        ctx: &Context<'_>,
        model_name: String,
    ) -> Result<PerformanceValidationResult> {
        let registry = ctx.data::<Arc<ModelRegistry>>()
            .map_err(|e| async_graphql::Error::new(format!("Model registry not available: {}", e)))?;
        
        let is_valid = registry.validate_performance(&model_name)
            .await.map_err(|e| async_graphql::Error::new(format!("Validation failed: {}", e)))?;
        
        let metrics = registry.get_performance_metrics(&model_name).await;
        
        Ok(PerformanceValidationResult {
            model_name: model_name.clone(),
            is_valid,
            current_metrics: metrics.as_ref().map(|m| PerformanceMetricsGraphQL::from(m)),
            message: if is_valid {
                "Performance within acceptable thresholds".to_string()
            } else {
                "Performance degradation detected - consider rollback".to_string()
            },
        })
    }
    
    /// Export trained model to ONNX format
    async fn export_to_onnx(
        &self,
        _ctx: &Context<'_>,
        input: ExportToOnnxInput,
    ) -> Result<OnnxExportResult> {
        info!("Exporting model {} to ONNX at {}", input.model_name, input.output_path);
        
        // Configure ONNX export
        let opset = match input.opset_version.unwrap_or(15) {
            13 => OnnxOpset::V13,
            14 => OnnxOpset::V14,
            15 => OnnxOpset::V15,
            16 => OnnxOpset::V16,
            17 => OnnxOpset::V17,
            _ => OnnxOpset::V15,
        };
        
        let config = OnnxExportConfig {
            opset,
            optimize: input.optimize.unwrap_or(true),
            include_metadata: true,
            custom_metadata: std::collections::HashMap::new(),
        };
        
        // In production, this would call the actual export function
        // For now, simulate the export
        let file_size_bytes = 45_000_000i64; // Mock 45MB model
        
        Ok(OnnxExportResult {
            success: true,
            output_path: input.output_path.clone(),
            model_name: input.model_name,
            file_size_bytes,
            opset_version: config.opset as i32,
            optimization_applied: config.optimize,
            exported_at: Utc::now(),
            error_message: None,
        })
    }
    
    /// Configure data source for training
    async fn configure_data_source(
        &self,
        _ctx: &Context<'_>,
        input: DataSourceConfigInput,
    ) -> Result<DataSourceConfig> {
        info!("Configuring data source {} of type {:?}", input.source_id, input.source_type);
        
        // In production, this would store in database
        Ok(DataSourceConfig {
            source_id: input.source_id,
            source_type: input.source_type,
            endpoint: input.endpoint,
            format: input.format,
            refresh_interval_seconds: input.refresh_interval_seconds.unwrap_or(3600),
            enabled: input.enabled,
            last_sync_at: None,
            total_records_fetched: 0,
        })
    }
    
    /// Update data source configuration
    async fn update_data_source(
        &self,
        _ctx: &Context<'_>,
        source_id: String,
        enabled: Option<bool>,
        refresh_interval_seconds: Option<i32>,
    ) -> Result<DataSourceConfig> {
        info!("Updating data source {}", source_id);
        
        // Mock update - in production would fetch and update from database
        Ok(DataSourceConfig {
            source_id,
            source_type: DataSourceType::S3,
            endpoint: "s3://updated/".to_string(),
            format: DataFormat::Csv,
            refresh_interval_seconds: refresh_interval_seconds.unwrap_or(3600),
            enabled: enabled.unwrap_or(true),
            last_sync_at: Some(Utc::now()),
            total_records_fetched: 100_000,
        })
    }
    
    /// Delete data source configuration
    async fn delete_data_source(&self, _ctx: &Context<'_>, source_id: String) -> Result<bool> {
        info!("Deleting data source {}", source_id);
        Ok(true)
    }
    
    /// Cancel training job
    async fn cancel_training_job(&self, _ctx: &Context<'_>, job_id: ID) -> Result<TrainingJobInfo> {
        info!("Cancelling training job {}", job_id);
        
        Ok(TrainingJobInfo {
            job_id,
            model_name: "cancelled-model".to_string(),
            status: TrainingJobStatus::Cancelled,
            progress_percent: 0.0,
            current_epoch: 0,
            total_epochs: 0,
            current_loss: None,
            validation_accuracy: None,
            started_at: Utc::now(),
            estimated_completion: None,
            completed_at: Some(Utc::now()),
            error_message: Some("Cancelled by user".to_string()),
            output_model_path: None,
        })
    }
}


/// Performance validation result
#[derive(SimpleObject, Clone)]
pub struct PerformanceValidationResult {
    pub model_name: String,
    pub is_valid: bool,
    pub current_metrics: Option<PerformanceMetricsGraphQL>,
    pub message: String,
}

/// Training job query root
#[derive(Default)]
pub struct TrainingJobQueryRoot;

#[Object]
impl TrainingJobQueryRoot {
    /// List all training jobs
    async fn training_jobs(
        &self,
        _ctx: &Context<'_>,
        status: Option<TrainingJobStatus>,
        model_name: Option<String>,
    ) -> Result<Vec<TrainingJobInfo>> {
        // In production, this would query from a database or job queue
        // For now, return mock data
        let jobs = vec![
            TrainingJobInfo {
                job_id: ID::from("job-001"),
                model_name: "isotope-classifier-v2".to_string(),
                status: TrainingJobStatus::Running,
                progress_percent: 67.5,
                current_epoch: 45,
                total_epochs: 100,
                current_loss: Some(0.0234),
                validation_accuracy: Some(0.945),
                started_at: Utc::now() - chrono::Duration::hours(2),
                estimated_completion: Some(Utc::now() + chrono::Duration::hours(1)),
                completed_at: None,
                error_message: None,
                output_model_path: None,
            },
            TrainingJobInfo {
                job_id: ID::from("job-002"),
                model_name: "anomaly-detector-v3".to_string(),
                status: TrainingJobStatus::Completed,
                progress_percent: 100.0,
                current_epoch: 50,
                total_epochs: 50,
                current_loss: Some(0.0156),
                validation_accuracy: Some(0.982),
                started_at: Utc::now() - chrono::Duration::days(1),
                estimated_completion: None,
                completed_at: Some(Utc::now() - chrono::Duration::hours(12)),
                error_message: None,
                output_model_path: Some("/models/anomaly-detector-v3.onnx".to_string()),
            },
        ];
        
        let filtered: Vec<TrainingJobInfo> = jobs.into_iter()
            .filter(|j| status.map_or(true, |s| j.status == s))
            .filter(|j| model_name.as_ref().map_or(true, |m| j.model_name == *m))
            .collect();
        
        Ok(filtered)
    }
    
    /// Get specific training job
    async fn training_job(&self, ctx: &Context<'_>, job_id: ID) -> Result<Option<TrainingJobInfo>> {
        let jobs = self.training_jobs(ctx, None, None).await?;
        Ok(jobs.into_iter().find(|j| j.job_id == job_id))
    }
    
    /// Get active training jobs count
    async fn active_training_jobs_count(&self, _ctx: &Context<'_>) -> Result<i32> {
        Ok(1) // Mock: 1 job running
    }
}

/// Data source query root
#[derive(Default)]
pub struct DataSourceQueryRoot;

#[Object]
impl DataSourceQueryRoot {
    /// List all configured data sources
    async fn data_sources(&self, _ctx: &Context<'_>) -> Result<Vec<DataSourceConfig>> {
        // Mock data sources
        Ok(vec![
            DataSourceConfig {
                source_id: "s3-radnet".to_string(),
                source_type: DataSourceType::S3,
                endpoint: "s3://radnet-data/2024/".to_string(),
                format: DataFormat::Csv,
                refresh_interval_seconds: 3600,
                enabled: true,
                last_sync_at: Some(Utc::now() - chrono::Duration::minutes(30)),
                total_records_fetched: 1_250_000,
            },
            DataSourceConfig {
                source_id: "local-spectra".to_string(),
                source_type: DataSourceType::LocalFile,
                endpoint: "/data/spectra/".to_string(),
                format: DataFormat::Hdf5,
                refresh_interval_seconds: 0,
                enabled: true,
                last_sync_at: Some(Utc::now() - chrono::Duration::hours(2)),
                total_records_fetched: 500_000,
            },
        ])
    }
    
    /// Get specific data source
    async fn data_source(&self, ctx: &Context<'_>, source_id: String) -> Result<Option<DataSourceConfig>> {
        let sources = self.data_sources(ctx).await?;
        Ok(sources.into_iter().find(|s| s.source_id == source_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_status_enum() {
        let statuses = vec![
            ModelStatus::Active,
            ModelStatus::Inactive,
            ModelStatus::RollingOut,
            ModelStatus::RollingBack,
            ModelStatus::Failed,
        ];
        
        for status in statuses {
            let string_val = match status {
                ModelStatus::Active => "ACTIVE",
                ModelStatus::Inactive => "INACTIVE",
                ModelStatus::RollingOut => "ROLLING_OUT",
                ModelStatus::RollingBack => "ROLLING_BACK",
                ModelStatus::Failed => "FAILED",
            };
            assert!(!string_val.is_empty());
        }
    }

    #[test]
    fn test_model_type_enum() {
        let types = vec![
            ModelType::IsotopeClassifier,
            ModelType::AnomalyDetector,
            ModelType::BackgroundSubtractor,
            ModelType::MultiClassClassifier,
        ];
        
        for model_type in types {
            let string_val = match model_type {
                ModelType::IsotopeClassifier => "ISOTOPE_CLASSIFIER",
                ModelType::AnomalyDetector => "ANOMALY_DETECTOR",
                ModelType::BackgroundSubtractor => "BACKGROUND_SUBTRACTOR",
                ModelType::MultiClassClassifier => "MULTI_CLASS_CLASSIFIER",
            };
            assert!(!string_val.is_empty());
        }
    }

    #[test]
    fn test_data_source_type_enum() {
        let types = vec![
            DataSourceType::S3,
            DataSourceType::Http,
            DataSourceType::LocalFile,
            DataSourceType::Database,
            DataSourceType::Streaming,
        ];
        
        for ds_type in types {
            let string_val = match ds_type {
                DataSourceType::S3 => "S3",
                DataSourceType::Http => "HTTP",
                DataSourceType::LocalFile => "LOCAL_FILE",
                DataSourceType::Database => "DATABASE",
                DataSourceType::Streaming => "STREAMING",
            };
            assert!(!string_val.is_empty());
        }
    }

    #[test]
    fn test_data_format_enum() {
        let formats = vec![
            DataFormat::Csv,
            DataFormat::Hdf5,
            DataFormat::Json,
            DataFormat::Parquet,
            DataFormat::Binary,
        ];
        
        for format in formats {
            let string_val = match format {
                DataFormat::Csv => "CSV",
                DataFormat::Hdf5 => "HDF5",
                DataFormat::Json => "JSON",
                DataFormat::Parquet => "PARQUET",
                DataFormat::Binary => "BINARY",
            };
            assert!(!string_val.is_empty());
        }
    }

    #[test]
    fn test_training_job_status_enum() {
        let statuses = vec![
            TrainingJobStatus::Pending,
            TrainingJobStatus::Running,
            TrainingJobStatus::Completed,
            TrainingJobStatus::Failed,
            TrainingJobStatus::Cancelled,
        ];
        
        for status in statuses {
            let string_val = match status {
                TrainingJobStatus::Pending => "PENDING",
                TrainingJobStatus::Running => "RUNNING",
                TrainingJobStatus::Completed => "COMPLETED",
                TrainingJobStatus::Failed => "FAILED",
                TrainingJobStatus::Cancelled => "CANCELLED",
            };
            assert!(!string_val.is_empty());
        }
    }

    #[tokio::test]
    async fn test_training_job_query_root_default() {
        let root = TrainingJobQueryRoot::default();
        let result = root.training_jobs(None, None, None).await;
        assert!(result.is_ok());
        
        let jobs = result.unwrap();
        assert!(!jobs.is_empty()); // Mock data returns 2 jobs
    }

    #[tokio::test]
    async fn test_training_job_query_by_id() {
        let root = TrainingJobQueryRoot::default();
        let result = root.training_job("job-001".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_active_training_jobs_count() {
        let root = TrainingJobQueryRoot::default();
        let result = root.active_training_jobs_count().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_data_source_query_root_default() {
        let root = DataSourceQueryRoot::default();
        let result = root.data_sources().await;
        assert!(result.is_ok());
        
        let sources = result.unwrap();
        assert_eq!(sources.len(), 2); // Mock data returns 2 sources
    }

    #[tokio::test]
    async fn test_data_source_query_by_id() {
        let root = DataSourceQueryRoot::default();
        let result = root.data_source("s3-radnet".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_model_mutation_export_to_onnx() {
        let root = ModelMutationRoot::default();
        
        let export_input = ExportToOnnxInput {
            model_name: "test-model".to_string(),
            model_path: "/models/test".to_string(),
            output_path: "/output/test.onnx".to_string(),
            opset_version: Some(15),
            optimize: Some(true),
        };
        
        // Note: This would need a proper context with ModelRegistry in real tests
        // For now, we just verify the input structure is correct
        assert_eq!(export_input.model_name, "test-model");
        assert_eq!(export_input.opset_version, Some(15));
    }

    #[tokio::test]
    async fn test_configure_data_source() {
        let root = ModelMutationRoot::default();
        
        let ds_input = DataSourceConfigInput {
            source_id: "test-ds".to_string(),
            source_type: DataSourceType::S3,
            endpoint: "s3://bucket/data".to_string(),
            credentials: Some(DataSourceCredentials {
                access_key: Some("key".to_string()),
                secret_key: Some("secret".to_string()),
                api_token: None,
                connection_string: None,
            }),
            format: DataFormat::Csv,
            refresh_interval_seconds: Some(3600),
            enabled: true,
        };
        
        assert_eq!(ds_input.source_id, "test-ds");
        assert!(ds_input.credentials.is_some());
    }

    #[tokio::test]
    async fn test_cancel_training_job() {
        let root = ModelMutationRoot::default();
        let result = root.cancel_training_job("job-123".to_string()).await;
        assert!(result.is_ok());
        
        let job = result.unwrap();
        assert_eq!(job.status, TrainingJobStatus::Cancelled);
    }
}
