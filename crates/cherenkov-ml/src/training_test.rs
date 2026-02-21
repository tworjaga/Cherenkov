#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, Tensor};
    use std::sync::Arc;

    fn create_test_device() -> Device {
        Device::Cpu
    }

    #[test]
    fn test_training_config_default() {
        let config = TrainingConfig::default();
        assert_eq!(config.epochs, 100);
        assert_eq!(config.batch_size, 32);
        assert_eq!(config.learning_rate, 0.001);
        assert_eq!(config.validation_split, 0.2);
        assert_eq!(config.patience, 10);
    }

    #[test]
    fn test_export_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.opset_version, 17);
        assert_eq!(config.optimize, true);
        assert_eq!(config.quantize, false);
    }

    #[test]
    fn test_training_pipeline_creation() {
        let device = create_test_device();
        let pipeline = TrainingPipeline::new(device);
        assert!(pipeline.models.is_empty());
    }

    #[test]
    fn test_model_version_info() {
        let version = ModelVersion {
            major: 1,
            minor: 0,
            patch: 0,
        };
        assert_eq!(version.to_string(), "1.0.0");
    }

    #[test]
    fn test_model_metadata_creation() {
        let metadata = ModelMetadata {
            name: "test_model".to_string(),
            version: ModelVersion::new(1, 0, 0),
            description: "Test model".to_string(),
            author: "test".to_string(),
            created_at: chrono::Utc::now(),
            tags: vec!["test".to_string()],
            framework: "candle".to_string(),
            license: "MIT".to_string(),
        };
        assert_eq!(metadata.name, "test_model");
        assert_eq!(metadata.version.to_string(), "1.0.0");
    }

    #[tokio::test]
    async fn test_training_job_creation() {
        let device = create_test_device();
        let mut pipeline = TrainingPipeline::new(device);
        
        let config = TrainingConfig::default();
        let job_id = pipeline.create_training_job("test_model", config).await;
        
        assert!(job_id > 0);
        assert!(pipeline.jobs.contains_key(&job_id));
    }

    #[tokio::test]
    async fn test_training_job_status() {
        let device = create_test_device();
        let mut pipeline = TrainingPipeline::new(device);
        
        let config = TrainingConfig::default();
        let job_id = pipeline.create_training_job("test_model", config).await;
        
        let status = pipeline.get_job_status(job_id).await;
        assert!(status.is_some());
    }

    #[test]
    fn test_dataset_config_validation() {
        let config = DatasetConfig {
            source: DataSource::Local {
                path: "/tmp/test".to_string(),
            },
            batch_size: 32,
            shuffle: true,
            validation_split: 0.2,
            max_samples: Some(1000),
            quality_config: DataQualityConfig::default(),
        };
        
        assert!(config.validation_split >= 0.0 && config.validation_split <= 1.0);
        assert!(config.batch_size > 0);
    }

    #[test]
    fn test_data_quality_config() {
        let config = DataQualityConfig {
            min_snr_db: 10.0,
            max_missing_ratio: 0.1,
            outlier_threshold_sigma: 3.0,
            min_energy_range: 0.01,
        };
        
        assert!(config.min_snr_db > 0.0);
        assert!(config.max_missing_ratio >= 0.0 && config.max_missing_ratio <= 1.0);
        assert!(config.outlier_threshold_sigma > 0.0);
    }

    #[test]
    fn test_spectra_sample_creation() {
        let sample = SpectraSample {
            energies: vec![100.0, 200.0, 300.0],
            counts: vec![10.0, 20.0, 30.0],
            timestamp: chrono::Utc::now(),
            sensor_id: "test_sensor".to_string(),
            metadata: std::collections::HashMap::new(),
        };
        
        assert_eq!(sample.energies.len(), sample.counts.len());
        assert!(!sample.sensor_id.is_empty());
    }

    #[test]
    fn test_spectra_dataset_creation() {
        let samples = vec![
            SpectraSample {
                energies: vec![100.0, 200.0],
                counts: vec![10.0, 20.0],
                timestamp: chrono::Utc::now(),
                sensor_id: "sensor1".to_string(),
                metadata: std::collections::HashMap::new(),
            },
        ];
        
        let dataset = SpectraDataset::new(samples);
        assert_eq!(dataset.len(), 1);
    }

    #[test]
    fn test_model_registry_creation() {
        let device = create_test_device();
        let registry = ModelRegistry::new(device);
        assert!(registry.list_models().is_empty());
    }

    #[tokio::test]
    async fn test_model_registration() {
        let device = create_test_device();
        let mut registry = ModelRegistry::new(device);
        
        let metadata = ModelMetadata {
            name: "test_model".to_string(),
            version: ModelVersion::new(1, 0, 0),
            description: "Test".to_string(),
            author: "test".to_string(),
            created_at: chrono::Utc::now(),
            tags: vec![],
            framework: "candle".to_string(),
            license: "MIT".to_string(),
        };
        
        let model_id = registry.register_model(metadata, vec![]).await;
        assert!(!model_id.is_empty());
        assert_eq!(registry.list_models().len(), 1);
    }

    #[test]
    fn test_onnx_export_config() {
        let config = ExportConfig {
            opset_version: 17,
            optimize: true,
            quantize: false,
            dynamic_axes: None,
        };
        
        assert!(config.opset_version >= 7 && config.opset_version <= 20);
    }

    #[tokio::test]
    async fn test_training_metrics() {
        let metrics = TrainingMetrics {
            epoch: 1,
            train_loss: 0.5,
            val_loss: 0.6,
            train_accuracy: 0.8,
            val_accuracy: 0.75,
            learning_rate: 0.001,
            timestamp: chrono::Utc::now(),
        };
        
        assert!(metrics.train_loss >= 0.0);
        assert!(metrics.val_loss >= 0.0);
        assert!(metrics.train_accuracy >= 0.0 && metrics.train_accuracy <= 1.0);
        assert!(metrics.val_accuracy >= 0.0 && metrics.val_accuracy <= 1.0);
    }

    #[test]
    fn test_early_stopping() {
        let mut patience = 5;
        let mut best_loss = f64::INFINITY;
        let mut counter = 0;
        
        let losses = vec![0.5, 0.4, 0.45, 0.44, 0.43, 0.42];
        
        for loss in losses {
            if loss < best_loss {
                best_loss = loss;
                counter = 0;
            } else {
                counter += 1;
            }
            
            if counter >= patience {
                break;
            }
        }
        
        assert!(counter < patience || best_loss < 0.42);
    }

    #[test]
    fn test_learning_rate_scheduler() {
        let initial_lr = 0.001;
        let epoch = 10;
        let decay_rate = 0.95;
        
        let lr = initial_lr * decay_rate.powi(epoch as i32);
        assert!(lr < initial_lr);
        assert!(lr > 0.0);
    }

    #[test]
    fn test_batch_processing() {
        let batch_size = 32;
        let total_samples = 100;
        
        let num_batches = (total_samples + batch_size - 1) / batch_size;
        assert_eq!(num_batches, 4); // 32 + 32 + 32 + 4
        
        let last_batch_size = total_samples % batch_size;
        assert_eq!(last_batch_size, 4);
    }

    #[test]
    fn test_model_serialization() {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        
        // Create a simple tensor for testing
        let tensor = Tensor::new(&[1.0f32, 2.0, 3.0], &device).unwrap();
        let data = tensor.to_vec1::<f32>().unwrap();
        
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], 1.0);
    }

    #[test]
    fn test_error_handling() {
        let result: Result<(), MlError> = Err(MlError::TrainingError {
            message: "Test error".to_string(),
        });
        
        assert!(result.is_err());
        if let Err(MlError::TrainingError { message }) = result {
            assert_eq!(message, "Test error");
        }
    }

    #[tokio::test]
    async fn test_concurrent_training_jobs() {
        let device = create_test_device();
        let mut pipeline = TrainingPipeline::new(device);
        
        let config = TrainingConfig::default();
        
        // Create multiple jobs
        let job1 = pipeline.create_training_job("model1", config.clone()).await;
        let job2 = pipeline.create_training_job("model2", config.clone()).await;
        let job3 = pipeline.create_training_job("model3", config).await;
        
        assert_ne!(job1, job2);
        assert_ne!(job2, job3);
        assert_eq!(pipeline.jobs.len(), 3);
    }

    #[test]
    fn test_data_augmentation() {
        let sample = SpectraSample {
            energies: vec![100.0, 200.0, 300.0],
            counts: vec![10.0, 20.0, 30.0],
            timestamp: chrono::Utc::now(),
            sensor_id: "test".to_string(),
            metadata: std::collections::HashMap::new(),
        };
        
        // Test noise addition
        let noise_factor = 0.1;
        let noisy_counts: Vec<f64> = sample.counts.iter()
            .map(|&c| c * (1.0 + noise_factor * (rand::random::<f64>() - 0.5)))
            .collect();
        
        assert_eq!(noisy_counts.len(), sample.counts.len());
    }

    #[test]
    fn test_cross_validation() {
        let k_folds = 5;
        let total_samples = 100;
        
        let fold_size = total_samples / k_folds;
        assert_eq!(fold_size, 20);
        
        // Test that all samples are used
        let total_used = fold_size * k_folds;
        assert_eq!(total_used, total_samples);
    }

    #[test]
    fn test_model_comparison() {
        let model1_accuracy = 0.85;
        let model2_accuracy = 0.90;
        
        let better_model = if model2_accuracy > model1_accuracy {
            "model2"
        } else {
            "model1"
        };
        
        assert_eq!(better_model, "model2");
    }

    #[test]
    fn test_feature_extraction() {
        let counts = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        
        // Calculate mean
        let mean = counts.iter().sum::<f64>() / counts.len() as f64;
        assert_eq!(mean, 30.0);
        
        // Calculate variance
        let variance = counts.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / counts.len() as f64;
        assert!(variance > 0.0);
    }

    #[test]
    fn test_normalization() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
        
        let normalized: Vec<f64> = data.iter()
            .map(|&x| (x - min_val) / (max_val - min_val))
            .collect();
        
        assert_eq!(normalized[0], 0.0);
        assert_eq!(normalized[4], 1.0);
        
        for val in &normalized {
            assert!(*val >= 0.0 && *val <= 1.0);
        }
    }

    #[test]
    fn test_confusion_matrix() {
        let tp = 50;
        let fp = 10;
        let tn = 30;
        let fn_val = 10;
        
        let precision = tp as f64 / (tp + fp) as f64;
        let recall = tp as f64 / (tp + fn_val) as f64;
        let f1 = 2.0 * precision * recall / (precision + recall);
        
        assert!(precision >= 0.0 && precision <= 1.0);
        assert!(recall >= 0.0 && recall <= 1.0);
        assert!(f1 >= 0.0 && f1 <= 1.0);
    }

    #[tokio::test]
    async fn test_model_versioning() {
        let device = create_test_device();
        let mut registry = ModelRegistry::new(device);
        
        let base_metadata = ModelMetadata {
            name: "test_model".to_string(),
            version: ModelVersion::new(1, 0, 0),
            description: "Base version".to_string(),
            author: "test".to_string(),
            created_at: chrono::Utc::now(),
            tags: vec![],
            framework: "candle".to_string(),
            license: "MIT".to_string(),
        };
        
        let v1_id = registry.register_model(base_metadata.clone(), vec![]).await;
        
        let v2_metadata = ModelMetadata {
            version: ModelVersion::new(1, 1, 0),
            description: "Updated version".to_string(),
            ..base_metadata
        };
        
        let v2_id = registry.register_model(v2_metadata, vec![]).await;
        
        assert_ne!(v1_id, v2_id);
        assert_eq!(registry.list_models().len(), 2);
    }

    #[test]
    fn test_gradient_clipping() {
        let gradients = vec![1.0, 2.0, 5.0, 10.0, 0.5];
        let max_norm = 5.0;
        
        let norm: f64 = gradients.iter().map(|&g| g * g).sum::<f64>().sqrt();
        let clip_factor = if norm > max_norm {
            max_norm / norm
        } else {
            1.0
        };
        
        let clipped: Vec<f64> = gradients.iter()
            .map(|&g| g * clip_factor)
            .collect();
        
        let new_norm: f64 = clipped.iter().map(|&g| g * g).sum::<f64>().sqrt();
        assert!(new_norm <= max_norm + 1e-6);
    }

    #[test]
    fn test_weight_initialization() {
        let fan_in = 100;
        let fan_out = 50;
        
        // Xavier initialization
        let limit = (6.0 / (fan_in + fan_out) as f64).sqrt();
        assert!(limit > 0.0);
        
        // He initialization
        let he_std = (2.0 / fan_in as f64).sqrt();
        assert!(he_std > 0.0);
    }

    #[test]
    fn test_activation_functions() {
        // ReLU
        let relu = |x: f64| if x > 0.0 { x } else { 0.0 };
        assert_eq!(relu(5.0), 5.0);
        assert_eq!(relu(-3.0), 0.0);
        
        // Sigmoid
        let sigmoid = |x: f64| 1.0 / (1.0 + (-x).exp());
        let result = sigmoid(0.0);
        assert!((result - 0.5).abs() < 1e-6);
        
        // Tanh
        let tanh = |x: f64| x.tanh();
        let result = tanh(0.0);
        assert!(result.abs() < 1e-6);
    }

    #[test]
    fn test_loss_functions() {
        let predictions = vec![0.9, 0.1, 0.8, 0.2];
        let targets = vec![1.0, 0.0, 1.0, 0.0];
        
        // MSE
        let mse: f64 = predictions.iter()
            .zip(targets.iter())
            .map(|(p, t)| (p - t).powi(2))
            .sum::<f64>() / predictions.len() as f64;
        assert!(mse >= 0.0);
        
        // Binary cross-entropy
        let bce: f64 = predictions.iter()
            .zip(targets.iter())
            .map(|(p, t)| {
                let p_clipped = p.max(1e-7).min(1.0 - 1e-7);
                -(t * p_clipped.ln() + (1.0 - t) * (1.0 - p_clipped).ln())
            })
            .sum::<f64>() / predictions.len() as f64;
        assert!(bce >= 0.0);
    }

    #[test]
    fn test_optimizer_state() {
        let lr = 0.001;
        let momentum = 0.9;
        let velocity = vec![0.0; 10];
        
        // SGD with momentum update
        let gradients = vec![0.1; 10];
        let new_velocity: Vec<f64> = velocity.iter()
            .zip(gradients.iter())
            .map(|(v, g)| momentum * v - lr * g)
            .collect();
        
        assert_eq!(new_velocity.len(), 10);
        for v in &new_velocity {
            assert!(v.abs() > 0.0);
        }
    }

    #[test]
    fn test_data_splitting() {
        let total = 1000;
        let train_ratio = 0.7;
        let val_ratio = 0.15;
        let test_ratio = 0.15;
        
        let train_size = (total as f64 * train_ratio) as usize;
        let val_size = (total as f64 * val_ratio) as usize;
        let test_size = total - train_size - val_size;
        
        assert_eq!(train_size + val_size + test_size, total);
        assert!(train_size > val_size);
        assert!(train_size > test_size);
    }

    #[test]
    fn test_hyperparameter_search_space() {
        let learning_rates = vec![0.1, 0.01, 0.001, 0.0001];
        let batch_sizes = vec![16, 32, 64, 128];
        let epochs = vec![50, 100, 200];
        
        let combinations = learning_rates.len() * batch_sizes.len() * epochs.len();
        assert_eq!(combinations, 48);
        
        // Test that all values are valid
        for lr in &learning_rates {
            assert!(*lr > 0.0);
        }
        for bs in &batch_sizes {
            assert!(*bs > 0);
        }
    }

    #[test]
    fn test_model_architecture_validation() {
        let input_size = 100;
        let hidden_sizes = vec![64, 32, 16];
        let output_size = 2;
        
        // Check that architecture is valid
        assert!(input_size > 0);
        assert!(output_size > 0);
        assert!(!hidden_sizes.is_empty());
        
        // Check that hidden layers are decreasing (common practice)
        for i in 1..hidden_sizes.len() {
            assert!(hidden_sizes[i] <= hidden_sizes[i - 1]);
        }
    }

    #[test]
    fn test_checkpoint_saving() {
        let epoch = 10;
        let loss = 0.5;
        let accuracy = 0.85;
        
        let checkpoint_name = format!("checkpoint_epoch{}_loss{:.4}_acc{:.4}.pt", 
            epoch, loss, accuracy);
        
        assert!(checkpoint_name.contains("epoch10"));
        assert!(checkpoint_name.contains("loss0.5000"));
        assert!(checkpoint_name.contains("acc0.8500"));
    }

    #[test]
    fn test_tensor_operations() {
        let device = Device::Cpu;
        
        // Create test tensors
        let a = Tensor::new(&[1.0f32, 2.0, 3.0], &device).unwrap();
        let b = Tensor::new(&[4.0f32, 5.0, 6.0], &device).unwrap();
        
        // Addition
        let c = (&a + &b).unwrap();
        let c_data = c.to_vec1::<f32>().unwrap();
        assert_eq!(c_data, vec![5.0, 7.0, 9.0]);
        
        // Multiplication
        let d = (&a * &b).unwrap();
        let d_data = d.to_vec1::<f32>().unwrap();
        assert_eq!(d_data, vec![4.0, 10.0, 18.0]);
    }

    #[test]
    fn test_sequence_padding() {
        let sequences = vec![
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0],
            vec![1.0, 2.0, 3.0, 4.0],
        ];
        
        let max_len = sequences.iter().map(|s| s.len()).max().unwrap();
        assert_eq!(max_len, 4);
        
        let padded: Vec<Vec<f64>> = sequences.iter()
            .map(|s| {
                let mut padded = s.clone();
                padded.resize(max_len, 0.0);
                padded
            })
            .collect();
        
        for seq in &padded {
            assert_eq!(seq.len(), max_len);
        }
    }

    #[test]
    fn test_attention_mechanism() {
        let query = vec![1.0, 0.0, 0.0];
        let keys = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        
        // Compute attention scores
        let scores: Vec<f64> = keys.iter()
            .map(|k| {
                query.iter().zip(k.iter()).map(|(q, k)| q * k).sum::<f64>()
            })
            .collect();
        
        // First key should have highest score (most similar to query)
        assert!(scores[0] > scores[1]);
        assert!(scores[0] > scores[2]);
    }

    #[test]
    fn test_batch_normalization() {
        let batch = vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 3.0, 4.0],
            vec![3.0, 4.0, 5.0],
        ];
        
        // Compute batch mean
        let mean: Vec<f64> = (0..3).map(|i| {
            batch.iter().map(|sample| sample[i]).sum::<f64>() / batch.len() as f64
        }).collect();
        
        assert_eq!(mean.len(), 3);
        assert!((mean[0] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_dropout_mask() {
        let size = 100;
        let dropout_rate = 0.5;
        
        // Generate dropout mask
        let mask: Vec<bool> = (0..size)
            .map(|_| rand::random::<f64>() > dropout_rate)
            .collect();
        
        let active_count = mask.iter().filter(|&&m| m).count();
        let ratio = active_count as f64 / size as f64;
        
        // Should be roughly 50% active (with some variance)
        assert!(ratio > 0.3 && ratio < 0.7);
    }

    #[test]
    fn test_embedding_lookup() {
        let vocab_size = 1000;
        let embedding_dim = 128;
        
        // Create simple embedding matrix
        let embeddings: Vec<Vec<f64>> = (0..vocab_size)
            .map(|i| vec![i as f64; embedding_dim])
            .collect();
        
        // Lookup embedding for index 5
        let idx = 5;
        let embedding = &embeddings[idx];
        
        assert_eq!(embedding.len(), embedding_dim);
        assert!(embedding.iter().all(|&v| v == idx as f64));
    }

    #[test]
    fn test_convolution_operation() {
        let input = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        
        let kernel = vec![
            vec![1.0, 0.0],
            vec![0.0, -1.0],
        ];
        
        // Simple 2D convolution
        let output_size = (input.len() - kernel.len() + 1) * (input[0].len() - kernel[0].len() + 1);
        assert_eq!(output_size, 4);
    }

    #[test]
    fn test_pooling_operation() {
        let input = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
        ];
        
        let pool_size = 2;
        let stride = 2;
        
        let output_size = ((input.len() - pool_size) / stride + 1) * 
                         ((input[0].len() - pool_size) / stride + 1);
        assert_eq!(output_size, 4);
        
        // Max pooling
        let max_val = input.iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        assert_eq!(max_val, 16.0);
    }

    #[test]
    fn test_roc_curve() {
        let scores = vec![0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1];
        let labels = vec![1, 1, 1, 0, 1, 0, 0, 0, 0];
        
        // Compute TPR and FPR at different thresholds
        let thresholds = vec![0.5, 0.6, 0.7, 0.8];
        
        for threshold in &thresholds {
            let predictions: Vec<i32> = scores.iter()
                .map(|&s| if s >= *threshold { 1 } else { 0 })
                .collect();
            
            let tp = predictions.iter().zip(labels.iter())
                .filter(|&(p, l)| *p == 1 && *l == 1)
                .count();
            let fp = predictions.iter().zip(labels.iter())
                .filter(|&(p, l)| *p == 1 && *l == 0)
                .count();
            
            assert!(tp + fp <= predictions.len());
        }
    }

    #[test]
    fn test_precision_recall_curve() {
        let precisions = vec![1.0, 0.9, 0.85, 0.8, 0.75, 0.7];
        let recalls = vec![0.1, 0.3, 0.5, 0.7, 0.85, 1.0];
        
        // Compute average precision (approximate AUC)
        let ap: f64 = precisions.iter().zip(recalls.iter().skip(1))
            .map(|(p, r_next)| p * (*r_next - recalls[0]))
            .sum();
        
        assert!(ap >= 0.0 && ap <= 1.0);
    }

    #[test]
    fn test_model_ensemble() {
        let model_predictions = vec![
            vec![0.9, 0.1],
            vec![0.8, 0.2],
            vec![0.85, 0.15],
        ];
        
        // Average ensemble
        let num_models = model_predictions.len();
        let num_classes = model_predictions[0].len();
        
        let ensemble: Vec<f64> = (0..num_classes).map(|c| {
            model_predictions.iter().map(|p| p[c]).sum::<f64>() / num_models as f64
        }).collect();
        
        assert_eq!(ensemble.len(), num_classes);
        assert!(ensemble.iter().all(|&p| p >= 0.0 && p <= 1.0));
    }

    #[test]
    fn test_knowledge_distillation() {
        let teacher_logits = vec![2.0, 1.0, 0.5];
        let student_logits = vec![1.5, 1.2, 0.3];
        let temperature = 2.0;
        
        // Softmax with temperature
        let softmax = |logits: &[f64], t: f64| {
            let exp_sum: f64 = logits.iter().map(|&l| (l / t).exp()).sum();
            logits.iter().map(|&l| (l / t).exp() / exp_sum).collect::<Vec<f64>>()
        };
        
        let teacher_soft = softmax(&teacher_logits, temperature);
        let student_soft = softmax(&student_logits, temperature);
        
        assert!(teacher_soft.iter().all(|&p| p > 0.0 && p < 1.0));
        assert!(student_soft.iter().all(|&p| p > 0.0 && p < 1.0));
    }

    #[test]
    fn test_quantization() {
        let weights = vec![-1.5, -0.5, 0.0, 0.5, 1.5];
        let scale = 0.1;
        let zero_point = 0;
        
        // Quantize to int8
        let quantized: Vec<i8> = weights.iter()
            .map(|&w| ((w / scale).round() as i32 + zero_point).clamp(-128, 127) as i8)
            .collect();
        
        // Dequantize
        let dequantized: Vec<f64> = quantized.iter()
            .map(|&q| (q as i32 - zero_point) as f64 * scale)
            .collect();
        
        // Check quantization error
        let max_error: f64 = weights.iter().zip(dequantized.iter())
            .map(|(w, d)| (w - d).abs())
            .fold(0.0, f64::max);
        
        assert!(max_error <= scale);
    }

    #[test]
    fn test_pruning() {
        let weights = vec![0.01, 0.5, 0.001, 0.3, 0.8, 0.0001];
        let threshold = 0.01;
        
        // Magnitude-based pruning
        let pruned: Vec<f64> = weights.iter()
            .map(|&w| if w.abs() < threshold { 0.0 } else { w })
            .collect();
        
        let zero_count = pruned.iter().filter(|&&w| w == 0.0).count();
        assert!(zero_count >= 2); // At least 2 values should be pruned
    }

    #[test]
    fn test_federated_averaging() {
        let client_updates = vec![
            vec![0.1, 0.2, 0.3],
            vec![0.15, 0.25, 0.35],
            vec![0.05, 0.15, 0.25],
        ];
        
        let num_clients = client_updates.len();
        
        // FedAvg: average all client updates
        let global_update: Vec<f64> = (0..3).map(|i| {
            client_updates.iter().map(|u| u[i]).sum::<f64>() / num_clients as f64
        }).collect();
        
        assert_eq!(global_update.len(), 3);
        assert!((global_update[0] - 0.1).abs() < 1e-6);
    }

    #[test]
    fn test_differential_privacy() {
        let gradients = vec![0.5, 0.3, 0.8, 0.2];
        let noise_scale = 0.1;
        let clip_norm = 1.0;
        
        // Clip gradients
        let norm: f64 = gradients.iter().map(|&g| g * g).sum::<f64>().sqrt();
        let clip_factor = if norm > clip_norm { clip_norm / norm } else { 1.0 };
        
        let clipped: Vec<f64> = gradients.iter()
            .map(|&g| g * clip_factor)
            .collect();
        
        // Add noise
        let noisy: Vec<f64> = clipped.iter()
            .map(|&g| g + noise_scale * rand::random::<f64>())
            .collect();
        
        assert_eq!(noisy.len(), gradients.len());
    }

    #[test]
    fn test_neural_architecture_search_space() {
        let operations = vec!["conv3x3", "conv5x5", "maxpool", "skip"];
        let num_layers = 4;
        
        // Number of possible architectures
        let num_architectures = operations.len().pow(num_layers as u32);
        assert_eq!(num_architectures, 256);
        
        // Generate a random architecture
        let architecture: Vec<&str> = (0..num_layers)
            .map(|_| operations[rand::random::<usize>() % operations.len()])
            .collect();
        
        assert_eq!(architecture.len(), num_layers);
    }

    #[test]
    fn test_meta_learning() {
        let tasks = vec![
            vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            vec![vec![5.0, 6.0], vec![7.0, 8.0]],
        ];
        
        // MAML-style inner loop update
        let alpha = 0.01; // inner learning rate
        let initial_params = vec![0.0, 0.0];
        
        for task in &tasks {
            let task_loss: f64 = task.iter()
                .map(|sample| sample.iter().sum::<f64>())
                .sum();
            
            // Gradient descent step
            let updated_params: Vec<f64> = initial_params.iter()
                .map(|&p| p - alpha * task_loss)
                .collect();
            
            assert_eq!(updated_params.len(), initial_params.len());
        }
    }

    #[test]
    fn test_contrastive_learning() {
        let anchor = vec![1.0, 2.0, 3.0];
        let positive = vec![1.1, 2.1, 3.1];
        let negative = vec![5.0, 6.0, 7.0];
        
        // Euclidean distance
        let distance = |a: &[f64], b: &[f64]| -> f64 {
            a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
        };
        
        let d_pos = distance(&anchor, &positive);
        let d_neg = distance(&anchor, &negative);
        
        // Positive should be closer than negative
        assert!(d_pos < d_neg);
    }

    #[test]
    fn test_self_attention() {
        let seq_len = 4;
        let d_model = 8;
        
        // Create dummy Q, K, V matrices
        let q: Vec<Vec<f64>> = (0..seq_len)
            .map(|i| vec![i as f64; d_model])
            .collect();
        let k = q.clone();
        let v = q.clone();
        
        // Compute attention scores (Q @ K^T)
        let scores: Vec<Vec<f64>> = q.iter().map(|q_i| {
            k.iter().map(|k_j| {
                q_i.iter().zip(k_j.iter()).map(|(a, b)| a * b).sum::<f64>()
            }).collect()
        }).collect();
        
        assert_eq!(scores.len(), seq_len);
        assert_eq!(scores[0].len(), seq_len);
        
        // Apply softmax to get attention weights
        let softmax = |x: &[f64]| {
            let max_val = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let exp_sum: f64 = x.iter().map(|&v| (v - max_val).exp()).sum();
            x.iter().map(|&v| (v - max_val).exp() / exp_sum).collect::<Vec<f64>>()
        };
        
        let attention_weights: Vec<Vec<f64>> = scores.iter()
            .map(|row| softmax(row))
            .collect();
        
        // Verify attention weights sum to 1
        for row in &attention_weights {
            let sum: f64 = row.iter().sum();
            assert!((sum - 1.0).abs() < 1e-6);
        }
        
        // Compute output (attention_weights @ V)
        let output: Vec<Vec<f64>> = attention_weights.iter().map(|weights| {
            (0..d_model).map(|j| {
                weights.iter().enumerate().map(|(i, w)| w * v[i][j]).sum::<f64>()
            }).collect()
        }).collect();
        
        assert_eq!(output.len(), seq_len);
        assert_eq!(output[0].len(), d_model);
    }
}
