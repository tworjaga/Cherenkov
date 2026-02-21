pub mod conversion;
pub mod data_loader;
pub mod inference;
pub mod integration;
pub mod isotope;
pub mod model_registry;
pub mod onnx_export;
pub mod training;

#[cfg(test)]
mod inference_test;

#[cfg(test)]
mod training_test;

pub use conversion::{
    VarMapConverter, ConversionConfig, ConversionReport, ModelArchitecture, TensorMetadata,
    convert_varmap_to_tensors, detect_model_architecture, verify_model_conversion,
    ConversionError, ConversionResult
};
pub use data_loader::{
    SpectraDataset, SpectraSample, DatasetConfig, DataSource, DataFormat,
    PreprocessingConfig, SourceType, public_datasets,
    CloudStorageClient, DataQualityConfig, QualityReport, QualityError, ErrorType,
    DatasetStatistics
};
pub use inference::{InferenceService, OnnxModel, OnnxError, ModelMetadata, BatchRequest, BatchResult, id_to_isotope, extract_top_k};
pub use model_registry::{ModelRegistry, ModelVersionInfo, OnnxModelMetadata, PerformanceMetrics, RegistryError};
pub use onnx_export::{OnnxExporter, ExportConfig, export_model_to_onnx, export_model_with_architecture};
pub use training::{
    TrainingPipeline, TrainingConfig, TrainingResult, ModelVersion, 
    LrScheduler, AugmentationConfig, CheckpointMeta, DatasetVersion, DatasetCache,
    run_training_job, list_model_versions, load_model_version
};
pub use integration::{
    MlAnomalyIntegration, ModelManager, TrainingScheduler, MlAnomalyResult, 
    RecommendedAction, AnomalyClass, ClassificationCache, CacheStats, 
    ConfidenceThresholds, StreamPipelineConnector
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub isotopes: Vec<IsotopePrediction>,
    pub latency_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsotopePrediction {
    pub symbol: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spectrum {
    pub channels: Vec<f32>,
    pub calibration: Calibration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calibration {
    pub slope: f32,
    pub intercept: f32,
    pub quadratic: f32,
}
