use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use candle_core::{Device, Tensor, DType};
use candle_nn::{VarMap, Optimizer, AdamW, ParamsAdamW};
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use std::time::Instant;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::data_loader::{SpectraDataset, DatasetConfig, DataSource, DataFormat, PreprocessingConfig};
use crate::onnx_export::{OnnxExporter, ExportConfig, ExportReport, ExportError};
use crate::conversion::{VarMapConverter, ConversionConfig};

/// Learning rate scheduler types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LrScheduler {
    Constant,
    StepDecay { step_size: usize, gamma: f64 },
    ExponentialDecay { gamma: f64 },
    CosineAnnealing { t_max: usize, eta_min: f64 },
    ReduceOnPlateau { factor: f64, patience: usize, min_lr: f64 },
}

/// Data augmentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentationConfig {
    pub enabled: bool,
    pub noise_std: f64,
    pub scale_range: (f64, f64),
    pub shift_max: f64,
    pub mixup_alpha: Option<f64>,
}

impl Default for AugmentationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            noise_std: 0.01,
            scale_range: (0.95, 1.05),
            shift_max: 0.02,
            mixup_alpha: Some(0.2),
        }
    }
}

/// Model version metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub version_id: String,
    pub created_at: String,
    pub training_config: TrainingConfig,
    pub metrics: TrainingResult,
    pub git_commit: Option<String>,
    pub tags: Vec<String>,
}

/// Checkpoint metadata for resumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMeta {
    pub epoch: usize,
    pub global_step: usize,
    pub best_val_loss: f64,
    pub optimizer_state: Option<String>,
    pub rng_state: Option<String>,
}

/// Dataset version metadata for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetVersion {
    pub version_id: String,
    pub dataset_hash: String,
    pub created_at: String,
    pub source_config: DatasetConfig,
    pub num_samples: usize,
    pub class_distribution: HashMap<String, usize>,
    pub preprocessing_hash: String,
}

/// Dataset cache for efficient reloading
#[derive(Debug, Clone)]
pub struct DatasetCache {
    pub cache_dir: PathBuf,
    pub max_cache_size_gb: f64,
}

impl DatasetCache {
    pub fn new(cache_dir: PathBuf, max_cache_size_gb: f64) -> Self {
        Self {
            cache_dir,
            max_cache_size_gb,
        }
    }

    pub fn get_cache_path(&self, version_id: &str) -> PathBuf {
        self.cache_dir.join(format!("dataset_{}.bin", version_id))
    }

    pub fn is_cached(&self, version_id: &str) -> bool {
        self.get_cache_path(version_id).exists()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub model_name: String,
    pub data_path: String,
    pub output_path: String,
    pub epochs: usize,
    pub batch_size: usize,
    pub learning_rate: f64,
    pub validation_split: f64,
    pub early_stopping_patience: usize,
    pub checkpoint_interval: usize,
    pub num_classes: usize,
    pub input_size: usize,
    pub hidden_layers: Vec<usize>,
    pub dropout_rate: f64,
    pub use_gpu: bool,
    pub seed: u64,
    pub lr_scheduler: LrScheduler,
    pub augmentation: AugmentationConfig,
    pub resume_from_checkpoint: Option<String>,
    pub max_checkpoints_to_keep: usize,
    pub gradient_clip_norm: Option<f64>,
    pub warmup_epochs: usize,
    pub label_smoothing: f64,
    pub dataset_config: Option<DatasetConfig>,
    pub use_stratified_sampling: bool,
    pub cache_datasets: bool,
    pub cache_dir: Option<String>,
}


impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            model_name: "isotope_classifier".to_string(),
            data_path: "s3://cherenkov-datasets/spectra".to_string(),
            output_path: "models/isotope_classifier".to_string(),
            epochs: 100,
            batch_size: 32,
            learning_rate: 0.001,
            validation_split: 0.2,
            early_stopping_patience: 10,
            checkpoint_interval: 10,
            num_classes: 15,
            input_size: 1024,
            hidden_layers: vec![512, 256, 128],
            dropout_rate: 0.3,
            use_gpu: true,
            seed: 42,
            lr_scheduler: LrScheduler::CosineAnnealing {
                t_max: 100,
                eta_min: 1e-6,
            },
            augmentation: AugmentationConfig::default(),
            resume_from_checkpoint: None,
            max_checkpoints_to_keep: 5,
            gradient_clip_norm: Some(1.0),
            warmup_epochs: 5,
            label_smoothing: 0.1,
            dataset_config: None,
            use_stratified_sampling: true,
            cache_datasets: true,
            cache_dir: Some(".cache/datasets".to_string()),
        }
    }
}


pub struct TrainingPipeline {
    config: TrainingConfig,
    device: Device,
    varmap: VarMap,
    metrics_sender: mpsc::Sender<TrainingMetrics>,
    current_lr: Arc<RwLock<f64>>,
    global_step: Arc<RwLock<usize>>,
    best_val_loss: Arc<RwLock<f64>>,
    plateau_counter: Arc<RwLock<usize>>,
}

#[derive(Debug, Clone)]
pub struct AugmentedSample {
    pub data: Tensor,
    pub label: usize,
    pub weight: f64,
}


#[derive(Debug, Clone, Serialize)]
pub struct TrainingMetrics {
    pub epoch: usize,
    pub train_loss: f64,
    pub train_accuracy: f64,
    pub val_loss: f64,
    pub val_accuracy: f64,
    pub learning_rate: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub model_path: String,
    pub final_loss: f64,
    pub validation_accuracy: f64,
    pub test_accuracy: f64,
    pub epochs_completed: usize,
    pub training_duration_secs: u64,
    pub best_epoch: usize,
    pub per_class_accuracy: HashMap<String, f64>,
    pub confusion_matrix: Vec<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub train_data: Vec<(Tensor, usize)>,
    pub val_data: Vec<(Tensor, usize)>,
    pub test_data: Vec<(Tensor, usize)>,
    pub class_names: Vec<String>,
}

/// Serializable dataset for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableDataset {
    pub train_data: Vec<(Vec<f32>, usize)>,
    pub val_data: Vec<(Vec<f32>, usize)>,
    pub test_data: Vec<(Vec<f32>, usize)>,
    pub class_names: Vec<String>,
    pub input_size: usize,
}

impl Dataset {
    pub fn to_serializable(&self) -> SerializableDataset {
        SerializableDataset {
            train_data: self.train_data.iter()
                .map(|(t, l)| (t.to_vec1::<f32>().unwrap_or_default(), *l))
                .collect(),
            val_data: self.val_data.iter()
                .map(|(t, l)| (t.to_vec1::<f32>().unwrap_or_default(), *l))
                .collect(),
            test_data: self.test_data.iter()
                .map(|(t, l)| (t.to_vec1::<f32>().unwrap_or_default(), *l))
                .collect(),
            class_names: self.class_names.clone(),
            input_size: self.train_data.first().map(|(t, _)| t.dims().iter().product()).unwrap_or(0),
        }
    }

    pub fn from_serializable(sd: SerializableDataset, device: &Device) -> anyhow::Result<Self> {
        let train_data: Vec<(Tensor, usize)> = sd.train_data.into_iter()
            .map(|(v, l)| (Tensor::from_vec(v, (sd.input_size,), device).unwrap_or_else(|_| Tensor::zeros((sd.input_size,), DType::F32, device).unwrap()), l))
            .collect();
        let val_data: Vec<(Tensor, usize)> = sd.val_data.into_iter()
            .map(|(v, l)| (Tensor::from_vec(v, (sd.input_size,), device).unwrap_or_else(|_| Tensor::zeros((sd.input_size,), DType::F32, device).unwrap()), l))
            .collect();
        let test_data: Vec<(Tensor, usize)> = sd.test_data.into_iter()
            .map(|(v, l)| (Tensor::from_vec(v, (sd.input_size,), device).unwrap_or_else(|_| Tensor::zeros((sd.input_size,), DType::F32, device).unwrap()), l))
            .collect();
        
        Ok(Self {
            train_data,
            val_data,
            test_data,
            class_names: sd.class_names,
        })
    }
}

impl TrainingPipeline {
    pub fn new(config: TrainingConfig) -> anyhow::Result<(Self, mpsc::Receiver<TrainingMetrics>)> {
        let device = if config.use_gpu {
            Device::cuda_if_available(0).unwrap_or(Device::Cpu)
        } else {
            Device::Cpu
        };
        
        info!("Training pipeline initialized with device: {:?}", device);
        
        let varmap = VarMap::new();
        let (metrics_sender, metrics_receiver) = mpsc::channel(100);
        
        // Initialize model weights
        Self::init_weights(&varmap, &config, &device)?;
        
        let pipeline = Self {
            config: config.clone(),
            device,
            varmap,
            metrics_sender,
            current_lr: Arc::new(RwLock::new(config.learning_rate)),
            global_step: Arc::new(RwLock::new(0)),
            best_val_loss: Arc::new(RwLock::new(f64::INFINITY)),
            plateau_counter: Arc::new(RwLock::new(0)),
        };
        
        Ok((pipeline, metrics_receiver))
    }

    fn init_weights(varmap: &VarMap, config: &TrainingConfig, device: &Device) -> anyhow::Result<()> {
        // Initialize weights using get method: (shape, name, init, dtype, device)
        for (i, hidden_size) in config.hidden_layers.iter().enumerate() {
            let w_name = format!("w{}", i);
            let b_name = format!("b{}", i);
            let _w = varmap.get(
                (config.input_size, *hidden_size),
                w_name.as_str(),
                candle_nn::Init::Randn { mean: 0.0, stdev: 0.02 },
                DType::F32,
                device
            )?;
            let _b = varmap.get(
                (*hidden_size,),
                b_name.as_str(),
                candle_nn::Init::Const(0.0),
                DType::F32,
                device
            )?;
        }
        
        let last_hidden = config.hidden_layers.last().copied().unwrap_or(config.input_size);
        let _w_out = varmap.get(
            (last_hidden, config.num_classes),
            "w_out",
            candle_nn::Init::Randn { mean: 0.0, stdev: 0.02 },
            DType::F32,
            device
        )?;
        let _b_out = varmap.get(
            (config.num_classes,),
            "b_out",
            candle_nn::Init::Const(0.0),
            DType::F32,
            device
        )?;
        
        Ok(())
    }

    pub async fn resume_from_checkpoint(&mut self, checkpoint_path: &str) -> anyhow::Result<CheckpointMeta> {
        info!("Resuming from checkpoint: {}", checkpoint_path);
        
        let meta_path = PathBuf::from(checkpoint_path).with_extension("meta.json");
        let meta: CheckpointMeta = if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            serde_json::from_str(&content)?
        } else {
            CheckpointMeta {
                epoch: 0,
                global_step: 0,
                best_val_loss: f64::INFINITY,
                optimizer_state: None,
                rng_state: None,
            }
        };
        
        self.varmap.load(Path::new(checkpoint_path))?;
        
        *self.global_step.write().await = meta.global_step;
        *self.best_val_loss.write().await = meta.best_val_loss;
        
        info!("Resumed from epoch {}, global_step {}", meta.epoch, meta.global_step);
        
        Ok(meta)
    }

    
    pub async fn train(&mut self) -> anyhow::Result<TrainingResult> {
        let start = Instant::now();
        info!("Starting training for model: {}", self.config.model_name);
        
        // Resume from checkpoint if specified
        let checkpoint_to_resume = self.config.resume_from_checkpoint.clone();
        let start_epoch = if let Some(checkpoint) = checkpoint_to_resume {
            let meta = self.resume_from_checkpoint(&checkpoint).await?;
            meta.epoch
        } else {
            0
        };
        
        let dataset = self.load_dataset().await?;
        info!("Dataset loaded: {} train, {} val, {} test samples", 
            dataset.train_data.len(), dataset.val_data.len(), dataset.test_data.len());
        
        let learning_rate = self.config.learning_rate;
        let mut optimizer = AdamW::new(
            self.varmap.all_vars(),
            ParamsAdamW {
                lr: learning_rate,
                ..Default::default()
            },
        )?;
        
        let mut best_epoch = start_epoch;
        let mut patience_counter = 0;
        
        for epoch in start_epoch..self.config.epochs {
            let epoch_start = Instant::now();
            
            // Update learning rate based on scheduler
            let current_lr = self.update_learning_rate(epoch).await;
            
            // Apply warmup if in warmup phase
            let effective_lr = if epoch < self.config.warmup_epochs {
                self.config.learning_rate * (epoch as f64 + 1.0) / (self.config.warmup_epochs as f64)
            } else {
                current_lr
            };
            
            // Update optimizer learning rate
            optimizer.set_learning_rate(effective_lr);
            
            let train_metrics = self.train_epoch(&dataset.train_data, &mut optimizer).await?;
            let val_metrics = self.validate(&dataset.val_data).await?;
            
            let metrics = TrainingMetrics {
                epoch,
                train_loss: train_metrics.0,
                train_accuracy: train_metrics.1,
                val_loss: val_metrics.0,
                val_accuracy: val_metrics.1,
                learning_rate: effective_lr,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            
            if let Err(e) = self.metrics_sender.send(metrics.clone()).await {
                warn!("Failed to send metrics: {}", e);
            }
            
            info!("Epoch {}: lr={:.6}, train_loss={:.4}, train_acc={:.2}%, val_loss={:.4}, val_acc={:.2}%, time={:?}",
                epoch, effective_lr, train_metrics.0, train_metrics.1 * 100.0,
                val_metrics.0, val_metrics.1 * 100.0, epoch_start.elapsed());
            
            // Update plateau counter for ReduceOnPlateau scheduler
            let is_better = {
                let mut best_val = self.best_val_loss.write().await;
                let mut plateau = self.plateau_counter.write().await;
                
                let better = val_metrics.0 < *best_val;
                if better {
                    *best_val = val_metrics.0;
                    best_epoch = epoch;
                    patience_counter = 0;
                    *plateau = 0;
                } else {
                    patience_counter += 1;
                    *plateau += 1;
                }
                better
            };
            
            if is_better {
                self.save_checkpoint(epoch, "best").await?;
            }
            
            if epoch > 0 && epoch % self.config.checkpoint_interval == 0 {
                self.save_checkpoint(epoch, &format!("epoch_{}", epoch)).await?;
            }
            
            if patience_counter >= self.config.early_stopping_patience {
                info!("Early stopping triggered at epoch {}", epoch);
                break;
            }
        }

        
        let test_metrics = self.validate(&dataset.test_data).await?;
        let per_class_accuracy = self.calculate_per_class_accuracy(&dataset.test_data).await?;
        let confusion_matrix = self.calculate_confusion_matrix(&dataset.test_data).await?;
        
        let model_path = self.export_model().await?;
        
        let result = TrainingResult {
            model_path: model_path.clone(),
            final_loss: *self.best_val_loss.read().await,
            validation_accuracy: *self.best_val_loss.read().await,
            test_accuracy: test_metrics.1,
            epochs_completed: best_epoch,
            training_duration_secs: start.elapsed().as_secs(),
            best_epoch,
            per_class_accuracy,
            confusion_matrix,
        };
        
        self.save_training_report(&result).await?;
        
        let version = self.create_model_version(&result).await?;
        self.save_model_version(&version).await?;
        
        info!("Training completed. Model saved to {}", model_path);
        info!("Model version {} created", version.version_id);
        
        Ok(result)
    }

    async fn update_learning_rate(&self, epoch: usize) -> f64 {
        let current = *self.current_lr.read().await;
        let new_lr = match &self.config.lr_scheduler {
            LrScheduler::Constant => current,
            LrScheduler::StepDecay { step_size, gamma } => {
                if epoch > 0 && epoch % step_size == 0 {
                    current * gamma
                } else {
                    current
                }
            }
            LrScheduler::ExponentialDecay { gamma } => {
                current * gamma.powi(epoch as i32)
            }
            LrScheduler::CosineAnnealing { t_max, eta_min } => {
                let progress = (epoch as f64) / (*t_max as f64);
                let cosine = (progress * std::f64::consts::PI).cos();
                eta_min + (self.config.learning_rate - eta_min) * (1.0 + cosine) / 2.0
            }
            LrScheduler::ReduceOnPlateau { factor, patience, min_lr } => {
                let plateau = *self.plateau_counter.read().await;
                if plateau > 0 && plateau % patience == 0 && current > *min_lr {
                    (current * factor).max(*min_lr)
                } else {
                    current
                }
            }
        };
        
        *self.current_lr.write().await = new_lr;
        new_lr
    }

    
    async fn load_dataset(&self) -> anyhow::Result<Dataset> {
        debug!("Loading dataset from {}", self.config.data_path);
        
        // Try to load from cache first
        if self.config.cache_datasets {
            if let Some(ref cache_dir) = self.config.cache_dir {
                let cache = DatasetCache::new(PathBuf::from(cache_dir), 10.0);
                let version_id = self.compute_dataset_version_id();
                if cache.is_cached(&version_id) {
                    info!("Loading dataset from cache: {}", version_id);
                    return self.load_dataset_from_cache(&cache, &version_id).await;
                }
            }
        }
        
        // Load real data using data_loader
        let dataset = self.load_real_dataset().await?;
        
        // Cache the dataset if enabled
        if self.config.cache_datasets {
            if let Some(ref cache_dir) = self.config.cache_dir {
                let cache = DatasetCache::new(PathBuf::from(cache_dir), 10.0);
                let version_id = self.compute_dataset_version_id();
                self.save_dataset_to_cache(&dataset, &cache, &version_id).await?;
            }
        }
        
        Ok(dataset)
    }

    fn compute_dataset_version_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.config.data_path.hash(&mut hasher);
        self.config.input_size.hash(&mut hasher);
        self.config.num_classes.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }

    async fn load_dataset_from_cache(&self, cache: &DatasetCache, version_id: &str) -> anyhow::Result<Dataset> {
        let cache_path = cache.get_cache_path(version_id);
        let data = fs::read(&cache_path)?;
        let serializable: SerializableDataset = bincode::deserialize(&data)?;
        Dataset::from_serializable(serializable, &self.device)
    }

    async fn save_dataset_to_cache(&self, dataset: &Dataset, cache: &DatasetCache, version_id: &str) -> anyhow::Result<()> {
        fs::create_dir_all(&cache.cache_dir)?;
        let cache_path = cache.get_cache_path(version_id);
        let serializable = dataset.to_serializable();
        let data = bincode::serialize(&serializable)?;
        fs::write(&cache_path, data)?;
        info!("Dataset cached to: {:?}", cache_path);
        Ok(())
    }

    async fn load_real_dataset(&self) -> anyhow::Result<Dataset> {
        info!("Loading real dataset from data sources");
        
        // Build dataset config from training config or use default
        let dataset_config = self.config.dataset_config.clone().unwrap_or_else(|| {
            DatasetConfig {
                name: "training_dataset".to_string(),
                source: DataSource::Local { path: self.config.data_path.clone() },
                format: DataFormat::Csv,
                preprocessing: PreprocessingConfig {
                    normalize: true,
                    smoothing: None,
                    baseline_correction: false,
                    peak_detection: false,
                    energy_calibration: None,
                    noise_reduction: None,
                },
                validation_split: self.config.validation_split,
                test_split: 0.1,
                cache_dir: self.config.cache_dir.clone(),
                max_samples: None,
                quality_config: None,
            }
        });
        
        // Load spectra dataset using data_loader
        let spectra_dataset = SpectraDataset::load(dataset_config, self.device.clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to load spectra dataset: {}", e))?;
        
        // Get class names from dataset or use defaults
        let class_names: Vec<String> = vec![
            "Cs-137".to_string(), "Co-60".to_string(), "Am-241".to_string(), 
            "Sr-90".to_string(), "I-131".to_string(),
            "Xe-133".to_string(), "Ba-133".to_string(), "Eu-152".to_string(), 
            "Pu-239".to_string(), "U-235".to_string(),
            "Th-232".to_string(), "Ra-226".to_string(), "K-40".to_string(), 
            "Rn-222".to_string(), "Po-210".to_string(),
        ];
        
        // Convert spectra samples to tensors using the public to_tensors method
        let (features, labels) = spectra_dataset.to_tensors()
            .map_err(|e| anyhow::anyhow!("Failed to convert dataset to tensors: {}", e))?;
        
        // Convert to Vec<(Tensor, usize)>
        let n_samples = features.dims()[0];
        let features_vec = features.to_vec2::<f32>()?;
        let labels_vec = labels.to_vec1::<u32>()?;
        
        let mut all_data: Vec<(Tensor, usize)> = Vec::with_capacity(n_samples);
        
        for i in 0..n_samples {
            let mut data = features_vec[i].clone();
            
            // Pad or truncate to match input_size
            if data.len() < self.config.input_size {
                data.resize(self.config.input_size, 0.0);
            } else if data.len() > self.config.input_size {
                data.truncate(self.config.input_size);
            }
            
            // Normalize
            let max_val = data.iter().copied().fold(0.0f32, f32::max);
            if max_val > 0.0 {
                for val in &mut data {
                    *val /= max_val;
                }
            }
            
            let tensor = Tensor::from_vec(data, (self.config.input_size,), &self.device)?;
            let label = labels_vec[i] as usize % self.config.num_classes;
            all_data.push((tensor, label));
        }
        
        info!("Loaded {} samples from real dataset", all_data.len());
        
        // Apply stratified sampling if enabled
        if self.config.use_stratified_sampling {
            self.stratified_split(all_data, &class_names).await
        } else {
            // Random shuffle split
            let mut rng = rand::thread_rng();
            use rand::seq::SliceRandom;
            all_data.shuffle(&mut rng);
            
            let n = all_data.len();
            let n_val = (n as f64 * self.config.validation_split) as usize;
            let n_test = n_val / 2;
            let n_train = n - n_val - n_test;
            
            let train_data = all_data[0..n_train].to_vec();
            let val_data = all_data[n_train..n_train + n_val].to_vec();
            let test_data = all_data[n_train + n_val..].to_vec();
            
            Ok(Dataset {
                train_data,
                val_data,
                test_data,
                class_names,
            })
        }
    }

    async fn stratified_split(&self, all_data: Vec<(Tensor, usize)>, class_names: &[String]) -> anyhow::Result<Dataset> {
        info!("Applying stratified sampling");
        
        // Group by class
        let mut class_groups: HashMap<usize, Vec<(Tensor, usize)>> = HashMap::new();
        for (tensor, label) in all_data {
            class_groups.entry(label).or_default().push((tensor, label));
        }
        
        let mut train_data: Vec<(Tensor, usize)> = Vec::new();
        let mut val_data: Vec<(Tensor, usize)> = Vec::new();
        let mut test_data: Vec<(Tensor, usize)> = Vec::new();
        
        // Split each class proportionally
        for (label, mut samples) in class_groups {
            let mut rng = rand::thread_rng();
            use rand::seq::SliceRandom;
            samples.shuffle(&mut rng);
            
            let n = samples.len();
            let n_val = (n as f64 * self.config.validation_split) as usize;
            let n_test = n_val / 2;
            let n_train = n - n_val - n_test;
            
            train_data.extend(samples[0..n_train].to_vec());
            val_data.extend(samples[n_train..n_train + n_val].to_vec());
            test_data.extend(samples[n_train + n_val..].to_vec());
            
            info!("Class {}: {} train, {} val, {} test", 
                class_names.get(label).map(|s| s.as_str()).unwrap_or("unknown"),
                n_train, n_val, n_test);
        }
        
        // Shuffle each split
        let mut rng = rand::thread_rng();
        use rand::seq::SliceRandom;
        train_data.shuffle(&mut rng);
        val_data.shuffle(&mut rng);
        test_data.shuffle(&mut rng);
        
        info!("Stratified split complete: {} train, {} val, {} test samples",
            train_data.len(), val_data.len(), test_data.len());
        
        Ok(Dataset {
            train_data,
            val_data,
            test_data,
            class_names: class_names.to_vec(),
        })
    }

    #[allow(dead_code)]
    fn augment_sample(&self, tensor: &Tensor, label: usize) -> anyhow::Result<AugmentedSample> {
        if !self.config.augmentation.enabled {
            return Ok(AugmentedSample {
                data: tensor.clone(),
                label,
                weight: 1.0,
            });
        }
        
        let mut data = tensor.to_vec1::<f32>()?;
        let aug = &self.config.augmentation;
        
        // Add Gaussian noise
        if aug.noise_std > 0.0 {
            for val in &mut data {
                let noise = rand::random::<f32>() * aug.noise_std as f32;
                *val += noise;
            }
        }
        
        // Random scaling
        let scale = if aug.scale_range.0 < aug.scale_range.1 {
            let range = aug.scale_range.1 - aug.scale_range.0;
            aug.scale_range.0 + rand::random::<f64>() * range
        } else {
            1.0
        };
        
        for val in &mut data {
            *val *= scale as f32;
        }
        
        // Random shift
        let shift = (rand::random::<f64>() * 2.0 - 1.0) * aug.shift_max;
        for val in &mut data {
            *val += shift as f32;
        }
        
        let augmented = Tensor::from_vec(data, tensor.shape().clone(), &self.device)?;
        
        Ok(AugmentedSample {
            data: augmented,
            label,
            weight: 1.0,
        })
    }

    
    async fn train_epoch(
        &self,
        data: &[(Tensor, usize)],
        optimizer: &mut AdamW,
    ) -> anyhow::Result<(f64, f64)> {
        let mut total_loss = 0.0;
        let mut correct = 0;
        let mut total = 0;
        
        for batch in data.chunks(self.config.batch_size) {
            let batch_tensors: Vec<&Tensor> = batch.iter().map(|(t, _)| t).collect();
            let batch_labels: Vec<u32> = batch.iter().map(|(_, l)| *l as u32).collect();
            
            let batch_tensor = Tensor::stack(&batch_tensors, 0)?;
            let logits = self.forward(&batch_tensor)?;
            
            let labels_tensor = Tensor::from_vec(batch_labels.clone(), (batch_labels.len(),), &self.device)?;
            let loss = candle_nn::loss::cross_entropy(&logits, &labels_tensor)?;
            
            optimizer.backward_step(&loss)?;
            
            total_loss += loss.to_vec0::<f32>()? as f64;
            
            let predictions = logits.argmax(1)?;
            let pred_vec = predictions.to_vec1::<u32>()?;
            
            for (pred, actual) in pred_vec.iter().zip(batch_labels.iter()) {
                if *pred as usize == *actual as usize {
                    correct += 1;
                }
                total += 1;
            }
        }
        
        let avg_loss = total_loss / (data.len() / self.config.batch_size).max(1) as f64;
        let accuracy = correct as f64 / total as f64;
        
        Ok((avg_loss, accuracy))
    }
    
    async fn validate(&self, data: &[(Tensor, usize)]) -> anyhow::Result<(f64, f64)> {
        let mut total_loss = 0.0;
        let mut correct = 0;
        let mut total = 0;
        
        for batch in data.chunks(self.config.batch_size) {
            let batch_tensors: Vec<&Tensor> = batch.iter().map(|(t, _)| t).collect();
            let batch_labels: Vec<u32> = batch.iter().map(|(_, l)| *l as u32).collect();
            
            let batch_tensor = Tensor::stack(&batch_tensors, 0)?;
            let logits = self.forward(&batch_tensor)?;
            
            let labels_tensor = Tensor::from_vec(batch_labels.clone(), (batch_labels.len(),), &self.device)?;
            let loss = candle_nn::loss::cross_entropy(&logits, &labels_tensor)?;
            total_loss += loss.to_vec0::<f32>()? as f64;
            
            let predictions = logits.argmax(1)?;
            let pred_vec = predictions.to_vec1::<u32>()?;
            
            for (pred, actual) in pred_vec.iter().zip(batch_labels.iter()) {
                if *pred as usize == *actual as usize {
                    correct += 1;
                }
                total += 1;
            }
        }
        
        let avg_loss = total_loss / (data.len() / self.config.batch_size).max(1) as f64;
        let accuracy = correct as f64 / total as f64;
        
        Ok((avg_loss, accuracy))
    }
    
    fn forward(&self, input: &Tensor) -> anyhow::Result<Tensor> {
        let mut x = input.clone();
        
        for (i, hidden_size) in self.config.hidden_layers.iter().enumerate() {
            let w_name = format!("w{}", i);
            let b_name = format!("b{}", i);
            let weight = self.varmap.get(
                (self.config.input_size, *hidden_size),
                w_name.as_str(),
                candle_nn::Init::Randn { mean: 0.0, stdev: 0.02 },
                DType::F32,
                &self.device
            )?;
            let bias = self.varmap.get(
                (*hidden_size,),
                b_name.as_str(),
                candle_nn::Init::Const(0.0),
                DType::F32,
                &self.device
            )?;
            
            x = x.matmul(&weight)?.broadcast_add(&bias)?;
            x = x.relu()?;
            
            // Dropout applied during training only
            if self.config.dropout_rate > 0.0 {
                // Simple dropout implementation - scale by keep probability
                let keep_prob = 1.0 - self.config.dropout_rate;
                x = (&x * keep_prob)?;
            }
        }
        
        let last_hidden = self.config.hidden_layers.last().copied().unwrap_or(self.config.input_size);
        let output_weight = self.varmap.get(
            (last_hidden, self.config.num_classes),
            "w_out",
            candle_nn::Init::Randn { mean: 0.0, stdev: 0.02 },
            DType::F32,
            &self.device
        )?;
        let output_bias = self.varmap.get(
            (self.config.num_classes,),
            "b_out",
            candle_nn::Init::Const(0.0),
            DType::F32,
            &self.device
        )?;
        
        let logits = x.matmul(&output_weight)?.broadcast_add(&output_bias)?;
        
        Ok(logits)
    }
    
    async fn save_checkpoint(&self, epoch: usize, name: &str) -> anyhow::Result<()> {
        let checkpoint_dir = PathBuf::from(&self.config.output_path).join("checkpoints");
        fs::create_dir_all(&checkpoint_dir)?;
        
        let checkpoint_path = checkpoint_dir.join(format!("{}_epoch_{}.safetensors", name, epoch));
        
        self.varmap.save(&checkpoint_path)?;
        
        debug!("Checkpoint saved: {:?}", checkpoint_path);
        
        Ok(())
    }
    
    /// Export model to ONNX format with full configuration
    pub async fn export_model(
        &self,
        output_path: &Path,
        export_config: Option<ExportConfig>,
    ) -> anyhow::Result<ExportReport> {
        info!("Exporting model to ONNX format: {:?}", output_path);
        
        // Create output directory if needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Build export configuration
        let config = export_config.unwrap_or_default();
        
        // Use the OnnxExporter for conversion
        let exporter = OnnxExporter::with_config(config);
        
        let input_shape = vec![1, self.config.input_size];
        let output_shape = vec![1, self.config.num_classes];
        let hidden_layers = self.config.hidden_layers.clone();
        
        let report = exporter.export_model(
            &self.varmap,
            &input_shape,
            &output_shape,
            output_path,
            &hidden_layers,
        ).await.map_err(|e| anyhow::anyhow!("ONNX export failed: {}", e))?;
        
        info!("Model exported successfully to {:?} ({} bytes)", 
            output_path, report.file_size_bytes);
        
        Ok(report)
    }
    
    /// Quick export with default settings
    pub async fn export_model_default(&self, output_path: &Path) -> anyhow::Result<ExportReport> {
        self.export_model(output_path, None).await
    }
    
    /// Export and validate model
    pub async fn export_and_validate(
        &self,
        output_path: &Path,
        test_input: &Tensor,
    ) -> anyhow::Result<(ExportReport, bool)> {
        let report = self.export_model_default(output_path).await?;
        
        // Validate the exported model
        let converter = VarMapConverter::new();
        let validation_result = converter.verify_conversion(
            &self.convert_varmap_to_tensors()?,
            test_input,
            &[1, self.config.num_classes],
        ).map_err(|e| anyhow::anyhow!("Validation failed: {}", e))?;
        
        Ok((report, validation_result))
    }
    
    /// Convert VarMap to tensor map for validation
    fn convert_varmap_to_tensors(&self) -> anyhow::Result<HashMap<String, Tensor>> {
        let mut tensor_map = HashMap::new();
        let all_vars = self.varmap.all_vars();
        
        for (name, var) in all_vars {
            let tensor = var.as_tensor().clone();
            tensor_map.insert(name, tensor);
        }
        
        Ok(tensor_map)
    }

    async fn create_model_version(&self, result: &TrainingResult) -> anyhow::Result<ModelVersion> {
        let git_commit = std::env::var("GIT_COMMIT").ok();
        
        // Create dataset version info
        let dataset_version = DatasetVersion {
            version_id: self.compute_dataset_version_id(),
            dataset_hash: self.compute_dataset_version_id(),
            created_at: Utc::now().to_rfc3339(),
            source_config: self.config.dataset_config.clone().unwrap_or_else(|| DatasetConfig {
                name: "default".to_string(),
                source: DataSource::Local { path: "./data".to_string() },
                format: DataFormat::Csv,
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
                cache_dir: None,
                max_samples: None,
                quality_config: None,
            }),
            num_samples: result.epochs_completed * 100, // Approximate
            class_distribution: result.per_class_accuracy.iter()
                .map(|(k, _)| (k.clone(), 100)) // Placeholder distribution
                .collect(),
            preprocessing_hash: "default".to_string(),
        };
        
        // Save dataset version metadata
        self.save_dataset_version(&dataset_version).await?;
        
        Ok(ModelVersion {
            version_id: Uuid::new_v4().to_string(),
            created_at: Utc::now().to_rfc3339(),
            training_config: self.config.clone(),
            metrics: result.clone(),
            git_commit,
            tags: vec!["auto-generated".to_string(), "stratified".to_string()],
        })
    }

    async fn save_dataset_version(&self, version: &DatasetVersion) -> anyhow::Result<()> {
        let output_dir = PathBuf::from(&self.config.output_path).join("dataset_versions");
        fs::create_dir_all(&output_dir)?;
        
        let version_path = output_dir.join(format!("dataset_{}.json", version.version_id));
        let version_json = serde_json::to_string_pretty(version)?;
        fs::write(&version_path, version_json)?;
        
        info!("Dataset version saved: {:?}", version_path);
        Ok(())
    }

    async fn save_model_version(&self, version: &ModelVersion) -> anyhow::Result<()> {
        let output_dir = PathBuf::from(&self.config.output_path);
        let version_path = output_dir.join(format!("version_{}.json", version.version_id));
        
        let version_json = serde_json::to_string_pretty(version)?;
        fs::write(&version_path, &version_json)?;
        
        // Also update latest version symlink/reference
        let latest_path = output_dir.join("latest_version.json");
        fs::write(&latest_path, version_json)?;
        
        Ok(())
    }

    
    async fn calculate_per_class_accuracy(
        &self,
        data: &[(Tensor, usize)],
    ) -> anyhow::Result<HashMap<String, f64>> {
        let mut class_correct: HashMap<usize, u32> = HashMap::new();
        let mut class_total: HashMap<usize, u32> = HashMap::new();
        
        for (tensor, label) in data {
            let logits = self.forward(tensor)?;
            let pred = logits.argmax(0)?.to_vec0::<u32>()? as usize;
            
            *class_total.entry(*label).or_insert(0) += 1;
            if pred == *label {
                *class_correct.entry(*label).or_insert(0) += 1;
            }
        }
        
        let class_names: Vec<String> = vec![
            "Cs-137".to_string(), "Co-60".to_string(), "Am-241".to_string(), 
            "Sr-90".to_string(), "I-131".to_string(),
            "Xe-133".to_string(), "Ba-133".to_string(), "Eu-152".to_string(), 
            "Pu-239".to_string(), "U-235".to_string(),
            "Th-232".to_string(), "Ra-226".to_string(), "K-40".to_string(), 
            "Rn-222".to_string(), "Po-210".to_string(),
        ];
        
        let mut per_class_accuracy = HashMap::new();
        for (class_idx, class_name) in class_names.iter().enumerate() {
            let correct = class_correct.get(&class_idx).copied().unwrap_or(0);
            let total = class_total.get(&class_idx).copied().unwrap_or(1);
            let accuracy = correct as f64 / total as f64;
            per_class_accuracy.insert(class_name.to_string(), accuracy);
        }
        
        Ok(per_class_accuracy)
    }
    
    async fn calculate_confusion_matrix(
        &self,
        data: &[(Tensor, usize)],
    ) -> anyhow::Result<Vec<Vec<u32>>> {
        let num_classes = self.config.num_classes;
        let mut matrix = vec![vec![0u32; num_classes]; num_classes];
        
        for (tensor, actual) in data {
            let logits = self.forward(tensor)?;
            let pred = logits.argmax(0)?.to_vec0::<u32>()? as usize;
            
            matrix[*actual][pred] += 1;
        }
        
        Ok(matrix)
    }
    
    async fn save_training_report(&self, result: &TrainingResult) -> anyhow::Result<()> {
        let report_path = PathBuf::from(&self.config.output_path).join("training_report.json");
        let report_json = serde_json::to_string_pretty(result)?;
        fs::write(&report_path, report_json)?;
        
        info!("Training report saved to {:?}", report_path);
        
        Ok(())
    }
}

pub async fn run_training_job(config: TrainingConfig) -> anyhow::Result<TrainingResult> {
    let (mut pipeline, mut metrics_rx) = TrainingPipeline::new(config)?;
    
    tokio::spawn(async move {
        while let Some(metrics) = metrics_rx.recv().await {
            debug!("Training metrics: epoch={}, loss={:.4}", metrics.epoch, metrics.train_loss);
        }
    });
    
    pipeline.train().await
}

/// List all available model versions
pub async fn list_model_versions(model_path: &str) -> anyhow::Result<Vec<ModelVersion>> {
    let path = PathBuf::from(model_path);
    let mut versions = Vec::new();
    
    if path.exists() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            if file_name.to_string_lossy().starts_with("version_") {
                let content = fs::read_to_string(entry.path())?;
                let version: ModelVersion = serde_json::from_str(&content)?;
                versions.push(version);
            }
        }
    }
    
    // Sort by creation time (newest first)
    versions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(versions)
}

/// Load a specific model version
pub async fn load_model_version(model_path: &str, version_id: &str) -> anyhow::Result<ModelVersion> {
    let version_path = PathBuf::from(model_path).join(format!("version_{}.json", version_id));
    let content = fs::read_to_string(&version_path)?;
    let version: ModelVersion = serde_json::from_str(&content)?;
    Ok(version)
}
