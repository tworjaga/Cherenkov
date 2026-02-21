use candle_core::{Device, Tensor, DType, Shape};
use candle_onnx::onnx;
use prost::Message;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use std::time::Instant;
use thiserror::Error;

use crate::{Classification, IsotopePrediction, Spectrum};

/// ONNX model loading and inference errors
#[derive(Error, Debug)]
pub enum OnnxError {
    #[error("Failed to read model file: {0}")]
    FileRead(#[source] std::io::Error),
    
    #[error("Failed to parse ONNX model: {0}")]
    ParseError(#[source] prost::DecodeError),
    
    #[error("Model has no graph")]
    MissingGraph,
    
    #[error("Model has no inputs")]
    MissingInputs,
    
    #[error("Model has no outputs")]
    MissingOutputs,
    
    #[error("Unsupported ONNX opset version: {0}. Supported: {1}")]
    UnsupportedOpset(i64, String),
    
    #[error("Input shape mismatch: expected {expected:?}, got {actual:?}")]
    InputShapeMismatch { expected: Vec<usize>, actual: Vec<usize> },
    
    #[error("Input name not found: {0}")]
    InputNotFound(String),
    
    #[error("Inference failed: {0}")]
    InferenceFailed(#[source] candle_core::Error),
    
    #[error("No output tensor found")]
    NoOutput,
    
    #[error("Model validation failed: {0}")]
    ValidationFailed(String),
}

/// ONNX model metadata for validation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelMetadata {
    pub input_names: Vec<String>,
    pub output_names: Vec<String>,
    pub input_shapes: Vec<Vec<usize>>,
    pub output_shapes: Vec<Vec<usize>>,
    pub opset_version: i64,
    pub producer_name: String,
    pub producer_version: String,
}

/// ONNX model wrapper for isotope classification with validation
pub struct OnnxModel {
    pub(crate) model: onnx::ModelProto,
    device: Device,
    input_name: String,
    output_name: String,
    metadata: ModelMetadata,
}

impl OnnxModel {
    /// Load and validate an ONNX model from file
    pub fn load(path: &str, device: &Device) -> Result<Self, OnnxError> {
        info!("Loading ONNX model from: {}", path);
        
        let model_bytes = std::fs::read(path)
            .map_err(OnnxError::FileRead)?;
        
        let model = onnx::ModelProto::decode(&model_bytes[..])
            .map_err(OnnxError::ParseError)?;
        
        let metadata = Self::extract_metadata(&model)?;
        
        // Validate opset version (support 7-21)
        if metadata.opset_version < 7 || metadata.opset_version > 21 {
            return Err(OnnxError::UnsupportedOpset(
                metadata.opset_version,
                "7-21".to_string()
            ));
        }
        
        info!(
            "ONNX model loaded - producer: {} {}, opset: {}, inputs: {:?}, outputs: {:?}",
            metadata.producer_name,
            metadata.producer_version,
            metadata.opset_version,
            metadata.input_names,
            metadata.output_names
        );
        
        let input_name = metadata.input_names[0].clone();
        let output_name = metadata.output_names[0].clone();
        
        Ok(Self {
            model,
            device: device.clone(),
            input_name,
            output_name,
            metadata,
        })
    }
    
    /// Extract metadata from model for validation
    fn extract_metadata(model: &onnx::ModelProto) -> Result<ModelMetadata, OnnxError> {
        let opset_version = model.opset_import.first()
            .map(|opset| opset.version)
            .unwrap_or(0);
        
        let graph = model.graph.as_ref()
            .ok_or(OnnxError::MissingGraph)?;
        
        if graph.input.is_empty() {
            return Err(OnnxError::MissingInputs);
        }
        
        if graph.output.is_empty() {
            return Err(OnnxError::MissingOutputs);
        }
        
        let input_names: Vec<String> = graph.input.iter()
            .map(|i| i.name.clone())
            .collect();
        
        let output_names: Vec<String> = graph.output.iter()
            .map(|o| o.name.clone())
            .collect();
        
        let input_shapes: Vec<Vec<usize>> = graph.input.iter()
            .map(|i| Self::extract_shape(&i.r#type))
            .collect();
        
        let output_shapes: Vec<Vec<usize>> = graph.output.iter()
            .map(|o| Self::extract_shape(&o.r#type))
            .collect();
        
        Ok(ModelMetadata {
            input_names,
            output_names,
            input_shapes,
            output_shapes,
            opset_version,
            producer_name: model.producer_name.clone(),
            producer_version: model.producer_version.clone(),
        })
    }
    
    /// Extract tensor shape from ONNX type
    fn extract_shape(tensor_type: &Option<onnx::TypeProto>) -> Vec<usize> {
        tensor_type.as_ref()
            .and_then(|t| match &t.value {
                Some(onnx::type_proto::Value::TensorType(tt)) => Some(tt),
                _ => None,
            })
            .and_then(|tt| tt.shape.as_ref())
            .map(|s| s.dim.iter()
                .filter_map(|d| d.value.as_ref())
                .filter_map(|v| match v {
                    onnx::tensor_shape_proto::dimension::Value::DimValue(val) if *val > 0 => Some(*val as usize),
                    _ => None,
                })
                .collect())
            .unwrap_or_default()
    }
    
    /// Validate input tensor shape against model requirements
    pub fn validate_input(&self, input: &Tensor) -> Result<(), OnnxError> {
        let actual_shape: Vec<usize> = input.dims().iter().map(|&d| d).collect();
        
        // Get expected shape (handle dynamic dimensions marked as 0)
        let expected_shape = &self.metadata.input_shapes[0];
        
        // Check rank matches
        if actual_shape.len() != expected_shape.len() {
            return Err(OnnxError::InputShapeMismatch {
                expected: expected_shape.clone(),
                actual: actual_shape,
            });
        }
        
        // Check each dimension (0 in expected means dynamic)
        for (i, (actual, expected)) in actual_shape.iter().zip(expected_shape.iter()).enumerate() {
            if *expected > 0 && actual != expected {
                return Err(OnnxError::InputShapeMismatch {
                    expected: expected_shape.clone(),
                    actual: actual_shape,
                });
            }
        }
        
        Ok(())
    }
    
    /// Run inference with validation
    pub fn forward(&self, input: &Tensor) -> Result<Tensor, OnnxError> {
        // Validate input before inference
        self.validate_input(input)?;
        
        let mut inputs = HashMap::new();
        inputs.insert(self.input_name.clone(), input.clone());
        
        let outputs = candle_onnx::simple_eval(&self.model, inputs)
            .map_err(OnnxError::InferenceFailed)?;
        
        let output = outputs.get(&self.output_name)
            .or_else(|| outputs.values().next())
            .ok_or(OnnxError::NoOutput)?;
        
        Ok(output.clone())
    }
    
    /// Get model metadata
    pub fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }
    
    /// Check if model supports batch inference
    pub fn supports_batch(&self) -> bool {
        self.metadata.input_shapes.get(0)
            .map(|shape| shape.first() == Some(&0) || shape.is_empty())
            .unwrap_or(false)
    }
}

pub struct InferenceService {
    models: Arc<RwLock<ModelCache>>,
    device: Device,
    batch_size: usize,
    #[allow(dead_code)]
    max_batch_wait_ms: u64,
}

struct ModelCache {
    isotope_classifier: Option<ModelVersion>,
    models: HashMap<String, ModelVersion>,
}

struct ModelVersion {
    model: Arc<OnnxModel>,
    version: String,
    loaded_at: Instant,
}

pub struct BatchRequest {
    pub spectra: Vec<Spectrum>,
    pub request_id: String,
}

pub struct BatchResult {
    pub results: Vec<Classification>,
    pub batch_latency_ms: u32,
    pub throughput: f32,
}

impl InferenceService {
    pub fn new(batch_size: usize, max_batch_wait_ms: u64) -> anyhow::Result<Self> {
        let device = Device::cuda_if_available(0)
            .unwrap_or_else(|_| Device::Cpu);
        
        info!("ML Inference Service using device: {:?}", device);
        
        Ok(Self {
            models: Arc::new(RwLock::new(ModelCache {
                isotope_classifier: None,
                models: HashMap::new(),
            })),
            device,
            batch_size,
            max_batch_wait_ms,
        })
    }
    
    pub async fn load_model(&self, name: &str, path: &str, version: &str) -> anyhow::Result<()> {
        let mut cache = self.models.write().await;
        
        let model = OnnxModel::load(path, &self.device)
            .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;
        
        let model_arc = Arc::new(model);
        let version_string = version.to_string();
        let loaded_at = Instant::now();
        
        if name == "isotope_classifier" {
            cache.isotope_classifier = Some(ModelVersion {
                model: Arc::clone(&model_arc),
                version: version_string.clone(),
                loaded_at,
            });
        }
        
        cache.models.insert(name.to_string(), ModelVersion {
            model: model_arc,
            version: version_string,
            loaded_at,
        });
        
        info!("Loaded model '{}' version {} from {}", name, version, path);
        
        Ok(())
    }
    
    pub async fn hot_swap_model(&self, name: &str, path: &str, version: &str) -> anyhow::Result<()> {
        info!("Hot-swapping model '{}' to version {}", name, version);
        
        self.load_model(name, path, version).await?;
        
        info!("Model '{}' hot-swapped successfully", name);
        
        Ok(())
    }
    
    pub async fn classify_spectrum(&self, spectrum: &Spectrum) -> anyhow::Result<Classification> {
        let start = Instant::now();
        
        let cache = self.models.read().await;
        let model_version = cache.isotope_classifier.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Isotope classifier not loaded"))?;
        
        let tensor = Tensor::from_vec(
            spectrum.channels.clone(),
            (1, spectrum.channels.len()),
            &self.device,
        )?;
        
        let logits = model_version.model.forward(&tensor)?;
        let probs = candle_nn::ops::softmax(&logits, 1)?;
        
        let probs_vec = probs.to_vec1::<f32>()?;
        let top5 = extract_top_k(&probs_vec, 5);
        
        let latency = start.elapsed().as_millis() as u32;
        
        Ok(Classification {
            isotopes: top5,
            latency_ms: latency,
        })
    }
    
    pub async fn classify_batch(&self, batch: BatchRequest) -> anyhow::Result<BatchResult> {
        let start = Instant::now();
        
        if batch.spectra.is_empty() {
            return Ok(BatchResult {
                results: vec![],
                batch_latency_ms: 0,
                throughput: 0.0,
            });
        }
        
        let cache = self.models.read().await;
        let model_version = cache.isotope_classifier.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Isotope classifier not loaded"))?;
        
        let batch_size = batch.spectra.len().min(self.batch_size);
        let channels = batch.spectra[0].channels.len();
        
        let mut batch_data = Vec::with_capacity(batch_size * channels);
        for spectrum in batch.spectra.iter().take(batch_size) {
            batch_data.extend_from_slice(&spectrum.channels);
        }
        
        let tensor = Tensor::from_vec(
            batch_data,
            (batch_size, channels),
            &self.device,
        )?;
        
        let logits = model_version.model.forward(&tensor)?;
        let probs = candle_nn::ops::softmax(&logits, 1)?;
        
        let probs_matrix = probs.to_vec2::<f32>()?;
        
        let results: Vec<Classification> = probs_matrix.iter()
            .map(|probs_vec| {
                let top5 = extract_top_k(probs_vec, 5);
                Classification {
                    isotopes: top5,
                    latency_ms: 0,
                }
            })
            .collect();
        
        let batch_latency = start.elapsed().as_millis() as u32;
        let throughput = batch_size as f32 / (batch_latency as f32 / 1000.0);
        
        Ok(BatchResult {
            results,
            batch_latency_ms: batch_latency,
            throughput,
        })
    }
    
    pub async fn get_model_info(&self) -> HashMap<String, String> {
        let cache = self.models.read().await;
        let mut info = HashMap::new();
        
        for (name, model_version) in &cache.models {
            info.insert(
                format!("{}_version", name),
                model_version.version.clone(),
            );
            info.insert(
                format!("{}_loaded_at", name),
                format!("{:?}", model_version.loaded_at.elapsed()),
            );
        }
        
        info
    }
    
    pub fn get_device(&self) -> &Device {
        &self.device
    }
}

pub fn extract_top_k(probs: &[f32], k: usize) -> Vec<IsotopePrediction> {
    let mut indexed: Vec<(usize, f32)> = probs.iter()
        .enumerate()
        .map(|(i, &p)| (i, p))
        .collect();
    
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    indexed.iter()
        .take(k)
        .map(|(id, conf)| IsotopePrediction {
            symbol: id_to_isotope(*id),
            confidence: *conf,
        })
        .collect()
}

pub fn id_to_isotope(id: usize) -> String {
    let isotopes = vec![
        "Cs-137", "Co-60", "Am-241", "Sr-90", "I-131",
        "Xe-133", "Ba-133", "Eu-152", "Pu-239", "U-235",
        "Th-232", "Ra-226", "K-40", "Rn-222", "Po-210",
    ];
    
    isotopes.get(id).map(|&s| s.to_string())
        .unwrap_or_else(|| format!("Unknown-{}", id))
}
