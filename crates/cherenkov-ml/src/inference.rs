use candle::{Device, Tensor};
use candle_onnx::OnnxModel;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::{Classification, IsotopePrediction, Spectrum};

pub struct InferenceService {
    models: Arc<RwLock<ModelCache>>,
    device: Device,
}

struct ModelCache {
    isotope_classifier: Option<OnnxModel>,
}

impl InferenceService {
    pub fn new() -> anyhow::Result<Self> {
        let device = Device::cuda_if_available(0)?;
        
        Ok(Self {
            models: Arc::new(RwLock::new(ModelCache {
                isotope_classifier: None,
            })),
            device,
        })
    }
    
    pub async fn load_model(&self, name: &str, path: &str) -> anyhow::Result<()> {
        let mut cache = self.models.write().await;
        
        match name {
            "isotope_classifier" => {
                let model = OnnxModel::load(path)?;
                cache.isotope_classifier = Some(model);
                info!("Loaded isotope classifier from {}", path);
            }
            _ => return Err(anyhow::anyhow!("Unknown model: {}", name)),
        }
        
        Ok(())
    }
    
    pub async fn classify_spectrum(&self, spectrum: &Spectrum) -> anyhow::Result<Classification> {
        let start = std::time::Instant::now();
        
        let cache = self.models.read().await;
        let model = cache.isotope_classifier.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Isotope classifier not loaded"))?;
        
        // Convert spectrum to tensor
        let tensor = Tensor::from_vec(
            spectrum.channels.clone(),
            (1, spectrum.channels.len()),
            &self.device,
        )?;
        
        // Run inference
        let logits = model.forward(&tensor)?;
        let probs = candle_nn::ops::softmax(&logits, 1)?;
        
        // Extract top-5 predictions
        let probs_vec = probs.to_vec1::<f32>()?;
        let mut indexed: Vec<(usize, f32)> = probs_vec.iter().enumerate()
            .map(|(i, &p)| (i, p))
            .collect();
        
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let top5: Vec<IsotopePrediction> = indexed.iter()
            .take(5)
            .map(|(id, conf)| IsotopePrediction {
                symbol: id_to_isotope(*id),
                confidence: *conf,
            })
            .collect();
        
        let latency = start.elapsed().as_millis() as u32;
        
        Ok(Classification {
            isotopes: top5,
            latency_ms: latency,
        })
    }
}

fn id_to_isotope(id: usize) -> String {
    // Mapping from model output IDs to isotope symbols
    let isotopes = vec![
        "Cs-137", "Co-60", "Am-241", "Sr-90", "I-131",
        "Xe-133", "Ba-133", "Eu-152", "Pu-239", "U-235",
    ];
    
    isotopes.get(id).map(|&s| s.to_string())
        .unwrap_or_else(|| format!("Unknown-{}", id))
}
