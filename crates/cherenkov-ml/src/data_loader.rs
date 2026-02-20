use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tracing::{info, debug, warn, error};
use candle_core::{Device, Tensor, DType};
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use std::fs::File;
use std::io::{BufRead, BufReader};
use csv::ReaderBuilder;
use reqwest;
use url::Url;
use tempfile::NamedTempFile;
use std::io::Write;

/// Radiation spectra data sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectraSample {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub energy_bins: Vec<f64>,
    pub counts: Vec<f64>,
    pub count_rate: f64,
    pub total_counts: f64,
    pub live_time: f64,
    pub real_time: f64,
    pub isotope_tags: Vec<String>,
    pub confidence_score: Option<f64>,
    pub source_type: SourceType,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Background,
    Medical,
    Industrial,
    Nuclear,
    Unknown,
}

/// Dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub name: String,
    pub source: DataSource,
    pub format: DataFormat,
    pub preprocessing: PreprocessingConfig,
    pub validation_split: f64,
    pub test_split: f64,
    pub cache_dir: Option<String>,
    pub max_samples: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Local { path: String },
    S3 { bucket: String, prefix: String, region: String },
    Http { url: String },
    HuggingFace { dataset: String, subset: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFormat {
    Csv,
    Json,
    Hdf5,
    Npy,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    pub normalize: bool,
    pub smoothing: Option<SmoothingConfig>,
    pub baseline_correction: bool,
    pub peak_detection: bool,
    pub energy_calibration: Option<EnergyCalibration>,
    pub noise_reduction: Option<NoiseReduction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmoothingConfig {
    pub method: SmoothingMethod,
    pub window_size: usize,
    pub iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmoothingMethod {
    MovingAverage,
    SavitzkyGolay,
    Gaussian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyCalibration {
    pub gain: f64,
    pub offset: f64,
    pub quadratic: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseReduction {
    pub method: NoiseMethod,
    pub threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoiseMethod {
    Wavelet,
    Median,
    LowPass,
}

/// Dataset loader for radiation spectra
pub struct SpectraDataset {
    config: DatasetConfig,
    samples: Vec<SpectraSample>,
    device: Device,
}

impl SpectraDataset {
    pub async fn load(config: DatasetConfig, device: Device) -> anyhow::Result<Self> {
        info!("Loading dataset: {}", config.name);
        
        let mut dataset = Self {
            config,
            samples: Vec::new(),
            device,
        };
        
        // Clone source to avoid borrow issues
        let source = dataset.config.source.clone();
        
        match source {
            DataSource::Local { path } => {
                dataset.load_local(&path).await?;
            }
            DataSource::Http { url } => {
                dataset.load_http(&url).await?;
            }
            DataSource::S3 { bucket, prefix, region } => {
                dataset.load_s3(&bucket, &prefix, &region).await?;
            }
            DataSource::HuggingFace { dataset: ds, subset } => {
                dataset.load_huggingface(&ds, subset.as_deref()).await?;
            }
        }
        
        info!("Loaded {} samples", dataset.samples.len());
        Ok(dataset)
    }
    
    async fn load_local(&mut self, path: &str) -> anyhow::Result<()> {
        let path = PathBuf::from(path);
        
        if !path.exists() {
            return Err(anyhow::anyhow!("Dataset path does not exist: {}", path.display()));
        }
        
        match self.config.format {
            DataFormat::Csv => {
                self.load_csv(&path).await?;
            }
            DataFormat::Json => {
                self.load_json(&path).await?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported format for local loading: {:?}", self.config.format));
            }
        }
        
        Ok(())
    }
    
    async fn load_csv(&mut self, path: &Path) -> anyhow::Result<()> {
        info!("Loading CSV from: {}", path.display());
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);
        
        for result in csv_reader.deserialize() {
            let sample: SpectraSample = result?;
            self.samples.push(sample);
            
            if let Some(max) = self.config.max_samples {
                if self.samples.len() >= max {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn load_json(&mut self, path: &Path) -> anyhow::Result<()> {
        info!("Loading JSON from: {}", path.display());
        
        let content = tokio::fs::read_to_string(path).await?;
        let samples: Vec<SpectraSample> = serde_json::from_str(&content)?;
        
        self.samples.extend(samples);
        Ok(())
    }
    
    async fn load_http(&mut self, url: &str) -> anyhow::Result<()> {
        info!("Downloading dataset from: {}", url);
        
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        
        // Save to temp file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(&content)?;
        
        match self.config.format {
            DataFormat::Csv => {
                self.load_csv(temp_file.path()).await?;
            }
            DataFormat::Json => {
                self.load_json(temp_file.path()).await?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported format for HTTP loading"));
            }
        }
        
        Ok(())
    }
    
    async fn load_s3(&mut self, bucket: &str, prefix: &str, region: &str) -> anyhow::Result<()> {
        info!("Loading from S3: s3://{}/{} in region {}", bucket, prefix, region);
        
        // For now, use HTTP endpoint for S3
        let url = format!("https://{}.s3.{}.amazonaws.com/{}", bucket, region, prefix);
        self.load_http(&url).await
    }
    
    async fn load_huggingface(&mut self, dataset: &str, subset: Option<&str>) -> anyhow::Result<()> {
        info!("Loading from HuggingFace: {}", dataset);
        
        let url = if let Some(sub) = subset {
            format!("https://huggingface.co/datasets/{}/resolve/main/{}.json", dataset, sub)
        } else {
            format!("https://huggingface.co/datasets/{}/resolve/main/data.json", dataset)
        };
        
        self.load_http(&url).await
    }
    
    /// Preprocess all samples
    pub fn preprocess(&mut self) -> anyhow::Result<()> {
        info!("Preprocessing {} samples", self.samples.len());
        
        let config = &self.config.preprocessing;
        
        for sample in &mut self.samples {
            Self::preprocess_sample_with_config(sample, config)?;
        }
        
        Ok(())
    }
    
    fn preprocess_sample_with_config(sample: &mut SpectraSample, config: &PreprocessingConfig) -> anyhow::Result<()> {
        // Normalize counts
        if config.normalize {
            let max_count = sample.counts.iter().copied().fold(0.0, f64::max);
            if max_count > 0.0 {
                for count in &mut sample.counts {
                    *count /= max_count;
                }
            }
        }
        
        // Apply smoothing
        if let Some(smoothing) = &config.smoothing {
            sample.counts = Self::apply_smoothing_with_config(&sample.counts, smoothing)?;
        }
        
        // Baseline correction
        if config.baseline_correction {
            sample.counts = Self::apply_baseline_correction_static(&sample.counts)?;
        }
        
        // Energy calibration
        if let Some(calibration) = &config.energy_calibration {
            sample.energy_bins = sample.energy_bins.iter()
                .map(|&e| calibration.quadratic * e * e + calibration.gain * e + calibration.offset)
                .collect();
        }
        
        Ok(())
    }
    
    fn apply_smoothing(&self, data: &[f64], config: &SmoothingConfig) -> anyhow::Result<Vec<f64>> {
        Self::apply_smoothing_with_config(data, config)
    }
    
    fn apply_smoothing_with_config(data: &[f64], config: &SmoothingConfig) -> anyhow::Result<Vec<f64>> {
        match config.method {
            SmoothingMethod::MovingAverage => {
                let window = config.window_size;
                let mut smoothed = Vec::with_capacity(data.len());
                
                for i in 0..data.len() {
                    let start = i.saturating_sub(window / 2);
                    let end = (i + window / 2 + 1).min(data.len());
                    let avg: f64 = data[start..end].iter().sum::<f64>() / (end - start) as f64;
                    smoothed.push(avg);
                }
                
                Ok(smoothed)
            }
            _ => {
                warn!("Smoothing method {:?} not fully implemented, using raw data", config.method);
                Ok(data.to_vec())
            }
        }
    }
    
    fn apply_baseline_correction(&self, data: &[f64]) -> anyhow::Result<Vec<f64>> {
        Self::apply_baseline_correction_static(data)
    }
    
    fn apply_baseline_correction_static(data: &[f64]) -> anyhow::Result<Vec<f64>> {
        // Simple linear baseline correction
        let n = data.len();
        if n < 2 {
            return Ok(data.to_vec());
        }
        
        let baseline = (data[0] + data[n-1]) / 2.0;
        Ok(data.iter().map(|&x| (x - baseline).max(0.0)).collect())
    }
    
    /// Convert to tensors for training
    pub fn to_tensors(&self) -> anyhow::Result<(Tensor, Tensor)> {
        let n_samples = self.samples.len();
        let n_features = self.samples.first()
            .map(|s| s.counts.len())
            .unwrap_or(0);
        
        let mut features = Vec::with_capacity(n_samples * n_features);
        let mut labels = Vec::with_capacity(n_samples);
        
        for sample in &self.samples {
            features.extend_from_slice(&sample.counts);
            
            // Create label from isotope tags
            let label = match sample.source_type {
                SourceType::Background => 0,
                SourceType::Medical => 1,
                SourceType::Industrial => 2,
                SourceType::Nuclear => 3,
                SourceType::Unknown => 4,
            };
            labels.push(label as u32);
        }
        
        let x = Tensor::from_vec(features, (n_samples, n_features), &self.device)?;
        let y = Tensor::from_vec(labels, n_samples, &self.device)?;
        
        Ok((x, y))
    }
    
    /// Split dataset into train/val/test
    pub fn split(&self) -> (Vec<&SpectraSample>, Vec<&SpectraSample>, Vec<&SpectraSample>) {
        let n = self.samples.len();
        let n_test = (n as f64 * self.config.test_split) as usize;
        let n_val = (n as f64 * self.config.validation_split) as usize;
        let n_train = n - n_test - n_val;
        
        let train = self.samples.iter().take(n_train).collect();
        let val = self.samples.iter().skip(n_train).take(n_val).collect();
        let test = self.samples.iter().skip(n_train + n_val).collect();
        
        (train, val, test)
    }
    
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

/// Public datasets for radiation spectra
pub mod public_datasets {
    use super::*;
    
    /// Load IAEA spectra dataset
    pub async fn load_iaea_spectra(device: Device) -> anyhow::Result<SpectraDataset> {
        let config = DatasetConfig {
            name: "IAEA_Radiation_Spectra".to_string(),
            source: DataSource::HuggingFace {
                dataset: "iaea/radiation-spectra".to_string(),
                subset: Some("gamma".to_string()),
            },
            format: DataFormat::Json,
            preprocessing: PreprocessingConfig {
                normalize: true,
                smoothing: Some(SmoothingConfig {
                    method: SmoothingMethod::MovingAverage,
                    window_size: 5,
                    iterations: 1,
                }),
                baseline_correction: true,
                peak_detection: true,
                energy_calibration: Some(EnergyCalibration {
                    gain: 1.0,
                    offset: 0.0,
                    quadratic: 0.0,
                }),
                noise_reduction: Some(NoiseReduction {
                    method: NoiseMethod::Median,
                    threshold: 0.1,
                }),
            },
            validation_split: 0.15,
            test_split: 0.15,
            cache_dir: Some("./cache/iaea".to_string()),
            max_samples: Some(10000),
        };
        
        SpectraDataset::load(config, device).await
    }
    
    /// Load Safecast dataset
    pub async fn load_safecast(device: Device) -> anyhow::Result<SpectraDataset> {
        let config = DatasetConfig {
            name: "Safecast".to_string(),
            source: DataSource::Http {
                url: "https://api.safecast.org/measurements.json".to_string(),
            },
            format: DataFormat::Json,
            preprocessing: PreprocessingConfig {
                normalize: true,
                smoothing: None,
                baseline_correction: false,
                peak_detection: false,
                energy_calibration: None,
                noise_reduction: None,
            },
            validation_split: 0.2,
            test_split: 0.1,
            cache_dir: Some("./cache/safecast".to_string()),
            max_samples: Some(50000),
        };
        
        SpectraDataset::load(config, device).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dataset_config() {
        let config = DatasetConfig {
            name: "test".to_string(),
            source: DataSource::Local { path: "./test.csv".to_string() },
            format: DataFormat::Csv,
            preprocessing: PreprocessingConfig {
                normalize: true,
                smoothing: None,
                baseline_correction: false,
                peak_detection: false,
                energy_calibration: None,
                noise_reduction: None,
            },
            validation_split: 0.15,
            test_split: 0.15,
            cache_dir: None,
            max_samples: Some(100),
        };
        
        assert_eq!(config.name, "test");
        assert!(config.preprocessing.normalize);
    }
}
