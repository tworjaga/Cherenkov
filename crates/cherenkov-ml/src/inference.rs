use candle_core::{Device, Tensor, DType};
use candle_onnx::onnx;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use std::time::Instant;

use crate::{Classification, IsotopePrediction, Spectrum};

/// ONNX model wrapper for isotope classification
pub struct OnnxModel {
    model: onnx::ModelProto,
    device: Device,
    input_name: String,
    output_name: String,
}

impl OnnxModel {
    pub fn load(path: &str, device: &Device) -> anyhow::Result<Self> {
        info!("Loading ONNX model from: {}", path);
        
        // Read model bytes from file
        let model_bytes = std::fs::read(path)
            .map_err(|e| {
                error!("Failed to read ONNX model file: {}", e);
                anyhow::anyhow!("File read error: {}", e)
            })?;
        
        // Parse the model using prost
        let model = onnx::ModelProto::decode(&*model_bytes)
            .map_err(|e| {
                error!("Failed to parse ONNX model: {}", e);
                anyhow::anyhow!("ONNX parse error: {}", e)
            })?;
        
        // Extract input/output names from model graph
        let graph = model.graph.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Model has no graph"))?;
        
        let input_name = graph.input.first()
            .and_then(|i| i.name.clone())
            .unwrap_or_else(|| "input".to_string());
        
        let output_name = graph.output.first()
            .and_then(|o| o.name.clone())
            .unwrap_or_else(|| "output".to_string());
        
        info!("ONNX model loaded - input: {}, output: {}", input_name, output_name);
        
        Ok(Self {
            model,
            device: device.clone(),
            input_name,
            output_name,
        })
    }
    
    pub fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor> {
        // Create input map for the model
        let mut inputs = HashMap::new();
        inputs.insert(self.input_name.clone(), input.clone());
        
        // Run inference using candle_onnx::simple_eval
        let outputs = candle_onnx::simple_eval(&self.model, inputs)
            .map_err(|e| {
                error!("ONNX inference failed: {}", e);
                anyhow::anyhow!("Inference error: {}", e)
            })?;
        
        // Extract output tensor
        let output = outputs.get(&self.output_name)
            .or_else(|| outputs.values().next())
            .ok_or_else(|| anyhow::anyhow!("No output tensor found"))?;
        
        Ok(output.clone())
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

fn extract_top_k(probs: &[f32], k: usize) -> Vec<IsotopePrediction> {
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

fn id_to_isotope(id: usize) -> String {
    let isotopes = vec![
        "Cs-137", "Co-60", "Am-241", "Sr-90", "I-131",
        "Xe-133", "Ba-133", "Eu-152", "Pu-239", "U-235",
        "Th-232", "Ra-226", "K-40", "Rn-222", "Po-210",
    ];
    
    isotopes.get(id).map(|&s| s.to_string())
        .unwrap_or_else(|| format!("Unknown-{}", id))
}
