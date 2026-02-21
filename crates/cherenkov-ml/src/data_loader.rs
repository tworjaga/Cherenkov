use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tracing::{info, warn};
use candle_core::{Device, Tensor};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, Read};
use csv::ReaderBuilder;
use reqwest;
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
    pub quality_config: Option<DataQualityConfig>,
}

/// Data quality validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityConfig {
    pub min_counts: Option<f64>,
    pub max_counts: Option<f64>,
    pub min_energy_bins: usize,
    pub max_energy_bins: usize,
    pub required_fields: Vec<String>,
    pub outlier_threshold: Option<f64>,
    pub duplicate_detection: bool,
}

impl Default for DataQualityConfig {
    fn default() -> Self {
        Self {
            min_counts: Some(0.0),
            max_counts: None,
            min_energy_bins: 128,
            max_energy_bins: 8192,
            required_fields: vec![
                "id".to_string(),
                "energy_bins".to_string(),
                "counts".to_string(),
            ],
            outlier_threshold: Some(3.0),
            duplicate_detection: true,
        }
    }
}

/// Data quality validation result
#[derive(Debug, Clone)]
pub struct QualityReport {
    pub total_samples: usize,
    pub valid_samples: usize,
    pub invalid_samples: usize,
    pub duplicates_removed: usize,
    pub outliers_removed: usize,
    pub errors: Vec<QualityError>,
}

#[derive(Debug, Clone)]
pub struct QualityError {
    pub sample_id: String,
    pub error_type: ErrorType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    MissingField,
    InvalidRange,
    WrongDimensions,
    Duplicate,
    Outlier,
    ParseError,
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

/// Cloud storage client for S3 operations
pub struct CloudStorageClient {
    client: Option<aws_sdk_s3::Client>,
    region: String,
}

impl CloudStorageClient {
    pub async fn new(region: &str) -> Self {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);
        
        Self {
            client: Some(client),
            region: region.to_string(),
        }
    }
    
    pub async fn download_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> anyhow::Result<Vec<u8>> {
        let client = self.client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("S3 client not initialized"))?;
        
        let response = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        
        let data = response.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }
    
    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> anyhow::Result<Vec<String>> {
        let client = self.client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("S3 client not initialized"))?;
        
        let response = client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .send()
            .await?;
        
        let contents = response.contents();
        let mut keys: Vec<String> = Vec::new();
        for obj in contents {
            if let Some(key) = obj.key() {
                keys.push(key.to_string());
            }
        }





















        
        Ok(keys)
    }
}

/// Dataset loader for radiation spectra
pub struct SpectraDataset {
    config: DatasetConfig,
    samples: Vec<SpectraSample>,
    device: Device,
    quality_report: Option<QualityReport>,
}


impl SpectraDataset {
    pub async fn load(config: DatasetConfig, device: Device) -> anyhow::Result<Self> {
        info!("Loading dataset: {}", config.name);
        
        let mut dataset = Self {
            config,
            samples: Vec::new(),
            device,
            quality_report: None,
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
        
        let client = CloudStorageClient::new(region).await;
        
        // List all objects with the given prefix
        let keys = client.list_objects(bucket, prefix).await?;
        info!("Found {} objects in S3", keys.len());
        
        for key in keys {
            let data = client.download_object(bucket, &key).await?;
            
            // Save to temp file
            let mut temp_file = NamedTempFile::new()?;
            temp_file.write_all(&data)?;
            
            // Determine format from key extension
            let format = if key.ends_with(".csv") {
                DataFormat::Csv
            } else if key.ends_with(".json") {
                DataFormat::Json
            } else if key.ends_with(".h5") || key.ends_with(".hdf5") {
                DataFormat::Hdf5
            } else {
                continue; // Skip unknown formats
            };
            
            match format {
                DataFormat::Csv => {
                    self.load_csv(temp_file.path()).await?;
                }
                DataFormat::Json => {
                    self.load_json(temp_file.path()).await?;
                }
                DataFormat::Hdf5 => {
                    self.load_hdf5(temp_file.path()).await?;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    async fn load_hdf5(&mut self, path: &Path) -> anyhow::Result<()> {
        info!("Loading HDF5 from: {}", path.display());
        
        #[cfg(feature = "hdf5")]
        {
            use hdf5::File as Hdf5File;
            use ndarray::Array1;
            
            let file = Hdf5File::open(path)?;
            
            // Try to load spectra dataset
            if let Ok(dataset) = file.dataset("spectra") {
                let data: Array1<f64> = dataset.read_1d()?;
                
                // Create sample from HDF5 data
                let sample = SpectraSample {
                    id: format!("hdf5_{}", self.samples.len()),
                    timestamp: chrono::Utc::now(),
                    energy_bins: (0..data.len()).map(|i| i as f64).collect(),
                    counts: data.to_vec(),
                    count_rate: 0.0,
                    total_counts: data.sum(),
                    live_time: 1.0,
                    real_time: 1.0,
                    isotope_tags: vec![],
                    confidence_score: None,
                    source_type: SourceType::Unknown,
                    metadata: HashMap::new(),
                };
                
                self.samples.push(sample);
            }
            
            // Try to load labels if available
            if let Ok(labels_dataset) = file.dataset("labels") {
                let labels: Array1<i32> = labels_dataset.read_1d()?;
                // Apply labels to samples
                for (i, sample) in self.samples.iter_mut().enumerate() {
                    if i < labels.len() {
                        sample.source_type = match labels[i] {
                            0 => SourceType::Background,
                            1 => SourceType::Medical,
                            2 => SourceType::Industrial,
                            3 => SourceType::Nuclear,
                            _ => SourceType::Unknown,
                        };
                    }
                }
            }
        }
        
        #[cfg(not(feature = "hdf5"))]
        {
            return Err(anyhow::anyhow!("HDF5 support not enabled. Compile with --features hdf5"));
        }
        
        Ok(())
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
    
    /// Validate data quality based on quality configuration
    pub fn validate_quality(&mut self) -> anyhow::Result<QualityReport> {
        let config = self.config.quality_config.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No quality configuration set"))?;
        
        let mut valid_samples = Vec::new();
        let mut errors = Vec::new();
        let mut duplicates_removed = 0;
        let mut outliers_removed = 0;
        
        // Track seen IDs for duplicate detection
        let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        for sample in &self.samples {
            let mut is_valid = true;
            
            // Check required fields
            for field in &config.required_fields {
                let field_valid = match field.as_str() {
                    "id" => !sample.id.is_empty(),
                    "energy_bins" => !sample.energy_bins.is_empty(),
                    "counts" => !sample.counts.is_empty(),
                    _ => true,
                };
                if !field_valid {
                    errors.push(QualityError {
                        sample_id: sample.id.clone(),
                        error_type: ErrorType::MissingField,
                        message: format!("Missing or empty field: {}", field),
                    });
                    is_valid = false;
                }
            }
            
            // Check energy bins count
            let n_bins = sample.energy_bins.len();
            if n_bins < config.min_energy_bins || n_bins > config.max_energy_bins {
                errors.push(QualityError {
                    sample_id: sample.id.clone(),
                    error_type: ErrorType::WrongDimensions,
                    message: format!("Energy bins count {} out of range [{}-{}]", 
                        n_bins, config.min_energy_bins, config.max_energy_bins),
                });
                is_valid = false;
            }
            
            // Check count range
            let total_counts: f64 = sample.counts.iter().sum();
            if let Some(min) = config.min_counts {
                if total_counts < min {
                    errors.push(QualityError {
                        sample_id: sample.id.clone(),
                        error_type: ErrorType::InvalidRange,
                        message: format!("Total counts {} below minimum {}", total_counts, min),
                    });
                    is_valid = false;
                }
            }
            if let Some(max) = config.max_counts {
                if total_counts > max {
                    errors.push(QualityError {
                        sample_id: sample.id.clone(),
                        error_type: ErrorType::InvalidRange,
                        message: format!("Total counts {} above maximum {}", total_counts, max),
                    });
                    is_valid = false;
                }
            }
            
            // Check for duplicates
            if config.duplicate_detection {
                if seen_ids.contains(&sample.id) {
                    errors.push(QualityError {
                        sample_id: sample.id.clone(),
                        error_type: ErrorType::Duplicate,
                        message: "Duplicate sample ID".to_string(),
                    });
                    duplicates_removed += 1;
                    is_valid = false;
                } else {
                    seen_ids.insert(sample.id.clone());
                }
            }
            
            // Check for outliers
            if let Some(threshold) = config.outlier_threshold {
                let mean = total_counts / n_bins as f64;
                let variance: f64 = sample.counts.iter()
                    .map(|&c| (c - mean).powi(2))
                    .sum::<f64>() / n_bins as f64;
                let std_dev = variance.sqrt();
                
                let max_count = sample.counts.iter().copied().fold(0.0f64, f64::max);
                if max_count > mean + threshold * std_dev {
                    errors.push(QualityError {
                        sample_id: sample.id.clone(),
                        error_type: ErrorType::Outlier,
                        message: format!("Outlier detected: max count {} exceeds threshold", max_count),
                    });
                    outliers_removed += 1;
                    is_valid = false;
                }
            }
            
            if is_valid {
                valid_samples.push(sample.clone());
            }
        }
        
        let total_samples = self.samples.len();
        let valid_count = valid_samples.len();
        let invalid_count = total_samples - valid_count;
        
        // Replace samples with validated ones
        self.samples = valid_samples;
        
        let report = QualityReport {
            total_samples,
            valid_samples: valid_count,
            invalid_samples: invalid_count,
            duplicates_removed,
            outliers_removed,
            errors,
        };
        
        self.quality_report = Some(report.clone());
        info!("Quality validation complete: {}/{} samples valid", valid_count, total_samples);
        
        Ok(report)
    }
    
    /// Get dataset statistics
    pub fn statistics(&self) -> DatasetStatistics {

        let n_samples = self.samples.len();
        if n_samples == 0 {
            return DatasetStatistics::default();
        }
        
        let n_bins = self.samples.first().map(|s| s.counts.len()).unwrap_or(0);
        let total_counts: f64 = self.samples.iter()
            .map(|s| s.counts.iter().sum::<f64>())
            .sum();
        
        let avg_counts_per_sample = total_counts / n_samples as f64;
        
        let source_type_counts: HashMap<String, usize> = self.samples.iter()
            .fold(HashMap::new(), |mut acc, s| {
                let key = format!("{:?}", s.source_type);
                *acc.entry(key).or_insert(0) += 1;
                acc
            });
        
        DatasetStatistics {
            n_samples,
            n_bins,
            total_counts,
            avg_counts_per_sample,
            source_type_distribution: source_type_counts,
        }
    }
}

/// Dataset statistics
#[derive(Debug, Clone, Default)]
pub struct DatasetStatistics {
    pub n_samples: usize,
    pub n_bins: usize,
    pub total_counts: f64,
    pub avg_counts_per_sample: f64,
    pub source_type_distribution: HashMap<String, usize>,
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
            quality_config: Some(DataQualityConfig::default()),
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
            quality_config: Some(DataQualityConfig::default()),
        };
        
        let mut dataset = SpectraDataset::load(config, device).await?;
        dataset.validate_quality()?;
        Ok(dataset)
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
            quality_config: Some(DataQualityConfig::default()),
        };
        
        assert_eq!(config.name, "test");
        assert!(config.preprocessing.normalize);
    }
    
    #[test]
    fn test_quality_config_default() {
        let config = DataQualityConfig::default();
        assert_eq!(config.min_energy_bins, 128);
        assert_eq!(config.max_energy_bins, 8192);
        assert!(config.duplicate_detection);
    }
    
    #[test]
    fn test_dataset_statistics() {
        let stats = DatasetStatistics {
            n_samples: 100,
            n_bins: 1024,
            total_counts: 10000.0,
            avg_counts_per_sample: 100.0,
            source_type_distribution: HashMap::new(),
        };
        
        assert_eq!(stats.n_samples, 100);
        assert_eq!(stats.n_bins, 1024);
    }
}
