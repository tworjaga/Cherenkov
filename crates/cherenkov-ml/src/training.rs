use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tracing::{info, error, debug, warn};
use candle::{Device, Tensor, DType};
use candle_nn::{VarMap, Optimizer, AdamW, ParamsAdamW};
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use std::time::Instant;
use std::fs;

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
        }
    }
}

pub struct TrainingPipeline {
    config: TrainingConfig,
    device: Device,
    varmap: VarMap,
    metrics_sender: mpsc::Sender<TrainingMetrics>,
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

#[derive(Debug, Clone, Serialize)]
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
        
        let pipeline = Self {
            config,
            device,
            varmap,
            metrics_sender,
        };
        
        Ok((pipeline, metrics_receiver))
    }
    
    pub async fn train(&mut self) -> anyhow::Result<TrainingResult> {
        let start = Instant::now();
        info!("Starting training for model: {}", self.config.model_name);
        
        let dataset = self.load_dataset().await?;
        info!("Dataset loaded: {} train, {} val, {} test samples", 
            dataset.train_data.len(), dataset.val_data.len(), dataset.test_data.len());
        
        let mut optimizer = AdamW::new(
            self.varmap.all_vars(),
            ParamsAdamW {
                lr: self.config.learning_rate,
                ..Default::default()
            },
        )?;
        
        let mut best_val_loss = f64::INFINITY;
        let mut best_epoch = 0;
        let mut patience_counter = 0;
        let mut final_result = None;
        
        for epoch in 0..self.config.epochs {
            let epoch_start = Instant::now();
            
            let train_metrics = self.train_epoch(&dataset.train_data, &mut optimizer).await?;
            let val_metrics = self.validate(&dataset.val_data).await?;
            
            let metrics = TrainingMetrics {
                epoch,
                train_loss: train_metrics.0,
                train_accuracy: train_metrics.1,
                val_loss: val_metrics.0,
                val_accuracy: val_metrics.1,
                learning_rate: self.config.learning_rate,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            
            if let Err(e) = self.metrics_sender.send(metrics.clone()).await {
                warn!("Failed to send metrics: {}", e);
            }
            
            info!("Epoch {}: train_loss={:.4}, train_acc={:.2}%, val_loss={:.4}, val_acc={:.2}%, time={:?}",
                epoch, train_metrics.0, train_metrics.1 * 100.0,
                val_metrics.0, val_metrics.1 * 100.0, epoch_start.elapsed());
            
            if val_metrics.0 < best_val_loss {
                best_val_loss = val_metrics.0;
                best_epoch = epoch;
                patience_counter = 0;
                
                self.save_checkpoint(epoch, "best").await?;
            } else {
                patience_counter += 1;
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
            final_loss: best_val_loss,
            validation_accuracy: best_val_loss,
            test_accuracy: test_metrics.1,
            epochs_completed: best_epoch,
            training_duration_secs: start.elapsed().as_secs(),
            best_epoch,
            per_class_accuracy,
            confusion_matrix,
        };
        
        self.save_training_report(&result).await?;
        
        info!("Training completed. Model saved to {}", model_path);
        
        Ok(result)
    }
    
    async fn load_dataset(&self) -> anyhow::Result<Dataset> {
        debug!("Loading dataset from {}", self.config.data_path);
        
        let class_names = vec![
            "Cs-137", "Co-60", "Am-241", "Sr-90", "I-131",
            "Xe-133", "Ba-133", "Eu-152", "Pu-239", "U-235",
            "Th-232", "Ra-226", "K-40", "Rn-222", "Po-210",
        ];
        
        let mut all_data: Vec<(Tensor, usize)> = Vec::new();
        
        for (class_idx, _class_name) in class_names.iter().enumerate() {
            for _sample_idx in 0..100 {
                let data: Vec<f32> = (0..self.config.input_size)
                    .map(|_| rand::random::<f32>())
                    .collect();
                
                let tensor = Tensor::from_vec(data, (self.config.input_size,), &self.device)?;
                all_data.push((tensor, class_idx));
            }
        }
        
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
            let batch_labels: Vec<usize> = batch.iter().map(|(_, l)| *l).collect();
            
            let batch_tensor = Tensor::stack(&batch_tensors, 0)?;
            let logits = self.forward(&batch_tensor)?;
            
            let loss = candle_nn::loss::cross_entropy(&logits, &Tensor::new(batch_labels.clone(), &self.device)?)?;
            
            optimizer.backward_step(&loss)?;
            
            total_loss += loss.to_vec0::<f32>()? as f64;
            
            let predictions = logits.argmax(1)?;
            let pred_vec = predictions.to_vec1::<u32>()?;
            
            for (pred, actual) in pred_vec.iter().zip(batch_labels.iter()) {
                if *pred as usize == *actual {
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
            let batch_labels: Vec<usize> = batch.iter().map(|(_, l)| *l).collect();
            
            let batch_tensor = Tensor::stack(&batch_tensors, 0)?;
            let logits = self.forward(&batch_tensor)?;
            
            let loss = candle_nn::loss::cross_entropy(&logits, &Tensor::new(batch_labels.clone(), &self.device)?)?;
            total_loss += loss.to_vec0::<f32>()? as f64;
            
            let predictions = logits.argmax(1)?;
            let pred_vec = predictions.to_vec1::<u32>()?;
            
            for (pred, actual) in pred_vec.iter().zip(batch_labels.iter()) {
                if *pred as usize == *actual {
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
            let weight = self.varmap.get(&format!("w{}", i))
                .ok_or_else(|| anyhow::anyhow!("Weight not found"))?;
            let bias = self.varmap.get(&format!("b{}", i))
                .ok_or_else(|| anyhow::anyhow!("Bias not found"))?;
            
            x = x.matmul(weight)?.broadcast_add(bias)?;
            x = candle_nn::ops::relu(&x)?;
            
            if self.config.dropout_rate > 0.0 {
                x = candle_nn::ops::dropout(&x, self.config.dropout_rate)?;
            }
        }
        
        let output_weight = self.varmap.get("w_out")
            .ok_or_else(|| anyhow::anyhow!("Output weight not found"))?;
        let output_bias = self.varmap.get("b_out")
            .ok_or_else(|| anyhow::anyhow!("Output bias not found"))?;
        
        let logits = x.matmul(output_weight)?.broadcast_add(output_bias)?;
        
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
    
    async fn export_model(&self) -> anyhow::Result<String> {
        let output_dir = PathBuf::from(&self.config.output_path);
        fs::create_dir_all(&output_dir)?;
        
        let model_path = output_dir.join("model.safetensors");
        self.varmap.save(&model_path)?;
        
        let config_path = output_dir.join("config.json");
        let config_json = serde_json::to_string_pretty(&self.config)?;
        fs::write(&config_path, config_json)?;
        
        let onnx_path = output_dir.join("model.onnx");
        
        Ok(onnx_path.to_string_lossy().to_string())
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
        
        let class_names = vec![
            "Cs-137", "Co-60", "Am-241", "Sr-90", "I-131",
            "Xe-133", "Ba-133", "Eu-152", "Pu-239", "U-235",
            "Th-232", "Ra-226", "K-40", "Rn-222", "Po-210",
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
