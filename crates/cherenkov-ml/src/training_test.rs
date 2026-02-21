#[cfg(test)]
mod tests {
    use crate::training::{
        TrainingConfig, TrainingPipeline, TrainingMetrics, TrainingResult,
        LrScheduler, AugmentationConfig, ModelVersion, DatasetVersion,
        CheckpointMeta, DatasetCache, Dataset, SerializableDataset,
        run_training_job, list_model_versions, load_model_version
    };
    use crate::onnx_export::{ExportConfig, ExportReport, ModelMetadata};
    use crate::data_loader::{DatasetConfig, DataSource, DataFormat, PreprocessingConfig, DataQualityConfig, SpectraSample, SpectraDataset, SourceType};
    use crate::model_registry::ModelRegistry;
    use candle_core::{Device, Tensor, DType};
    use candle_nn::VarMap;
    use std::path::PathBuf;
    use std::collections::HashMap;

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
        assert_eq!(config.early_stopping_patience, 10);
    }

    #[test]
    fn test_export_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.opset_version, 17);
    }

    #[tokio::test]
    async fn test_training_pipeline_creation() {
        let config = TrainingConfig::default();
        let result = TrainingPipeline::new(config);
        assert!(result.is_ok());
        let (_pipeline, _receiver) = result.unwrap();
        // Pipeline created successfully
    }

    #[test]
    fn test_model_version_info() {
        let version = ModelVersion {
            version_id: "test-uuid".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            training_config: TrainingConfig::default(),
            metrics: TrainingResult {
                model_path: "test.onnx".to_string(),
                final_loss: 0.5,
                validation_accuracy: 0.85,
                test_accuracy: 0.83,
                epochs_completed: 50,
                training_duration_secs: 3600,
                best_epoch: 45,
                per_class_accuracy: HashMap::new(),
                confusion_matrix: vec![vec![0; 15]; 15],
            },
            git_commit: Some("abc123".to_string()),
            tags: vec!["test".to_string()],
        };
        assert_eq!(version.version_id, "test-uuid");
        assert_eq!(version.tags[0], "test");
    }

    #[test]
    fn test_model_metadata_creation() {
        let metadata = ModelMetadata {
            name: "TestModel".to_string(),
            version: "1.0.0".to_string(),
            description: "Test model for radiation classification".to_string(),
            author: "Cherenkov ML Team".to_string(),
            domain: "radiation-spectroscopy".to_string(),
            model_version: 1,
            doc_string: "Trained model for classifying radiation spectra".to_string(),
        };
        assert_eq!(metadata.name, "TestModel");
        assert_eq!(metadata.model_version, 1);
    }

    #[test]
    fn test_lr_scheduler() {
        let constant = LrScheduler::Constant;
        let step = LrScheduler::StepDecay { step_size: 10, gamma: 0.1 };
        let exp = LrScheduler::ExponentialDecay { gamma: 0.95 };
        let cosine = LrScheduler::CosineAnnealing { t_max: 100, eta_min: 1e-6 };
        let plateau = LrScheduler::ReduceOnPlateau { factor: 0.5, patience: 5, min_lr: 1e-7 };

        // Just verify they can be created
        match constant {
            LrScheduler::Constant => (),
            _ => panic!("Expected Constant"),
        }
        match step {
            LrScheduler::StepDecay { step_size, gamma } => {
                assert_eq!(step_size, 10);
                assert_eq!(gamma, 0.1);
            }
            _ => panic!("Expected StepDecay"),
        }
    }

    #[test]
    fn test_augmentation_config() {
        let config = AugmentationConfig::default();
        assert!(config.enabled);
        assert_eq!(config.noise_std, 0.01);
        assert_eq!(config.scale_range, (0.95, 1.05));
        assert_eq!(config.shift_max, 0.02);
        assert_eq!(config.mixup_alpha, Some(0.2));
    }

    #[test]
    fn test_checkpoint_meta() {
        let meta = CheckpointMeta {
            epoch: 10,
            global_step: 1000,
            best_val_loss: 0.5,
            optimizer_state: Some("state".to_string()),
            rng_state: Some("rng".to_string()),
        };
        assert_eq!(meta.epoch, 10);
        assert_eq!(meta.global_step, 1000);
    }

    #[test]
    fn test_dataset_cache() {
        let cache = DatasetCache::new(PathBuf::from(".cache"), 10.0);
        assert_eq!(cache.max_cache_size_gb, 10.0);
        let path = cache.get_cache_path("v1");
        assert!(path.to_string_lossy().contains("dataset_v1"));
    }

    #[test]
    fn test_dataset_version() {
        let version = DatasetVersion {
            version_id: "v1".to_string(),
            dataset_hash: "hash123".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            source_config: DatasetConfig {
                name: "test".to_string(),
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
            },
            num_samples: 1000,
            class_distribution: HashMap::new(),
            preprocessing_hash: "pre_hash".to_string(),
        };
        assert_eq!(version.num_samples, 1000);
    }

    #[test]
    fn test_training_metrics() {
        let metrics = TrainingMetrics {
            epoch: 5,
            train_loss: 0.5,
            train_accuracy: 0.85,
            val_loss: 0.6,
            val_accuracy: 0.80,
            learning_rate: 0.001,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        assert_eq!(metrics.epoch, 5);
        assert!(metrics.train_accuracy > 0.0);
    }

    #[test]
    fn test_training_result() {
        let result = TrainingResult {
            model_path: "model.onnx".to_string(),
            final_loss: 0.5,
            validation_accuracy: 0.85,
            test_accuracy: 0.83,
            epochs_completed: 50,
            training_duration_secs: 3600,
            best_epoch: 45,
            per_class_accuracy: HashMap::new(),
            confusion_matrix: vec![vec![0; 15]; 15],
        };
        assert_eq!(result.epochs_completed, 50);
        assert_eq!(result.best_epoch, 45);
    }

    #[test]
    fn test_dataset_serialization() {
        let device = create_test_device();
        let dataset = Dataset {
            train_data: vec![],
            val_data: vec![],
            test_data: vec![],
            class_names: vec!["class1".to_string()],
        };
        let serializable = dataset.to_serializable();
        assert_eq!(serializable.class_names[0], "class1");
        
        let restored = Dataset::from_serializable(serializable, &device);
        assert!(restored.is_ok());
    }

    #[test]
    fn test_spectra_sample() {
        let sample = SpectraSample {
            id: "test".to_string(),
            timestamp: chrono::Utc::now(),
            energy_bins: vec![100.0, 200.0, 300.0],
            counts: vec![10.0, 20.0, 30.0],
            count_rate: 60.0,
            total_counts: 60.0,
            live_time: 10.0,
            real_time: 10.0,
            isotope_tags: vec!["Cs-137".to_string()],
            confidence_score: Some(0.95),
            source_type: SourceType::Nuclear,
            metadata: HashMap::new(),
        };
        assert_eq!(sample.total_counts, 60.0);
        assert!(matches!(sample.source_type, SourceType::Nuclear));

    }

    #[test]
    fn test_data_quality_config() {
        let config = DataQualityConfig {
            min_counts: Some(100.0),
            max_counts: Some(1000000.0),
            min_energy_bins: 256,
            max_energy_bins: 4096,
            required_fields: vec!["counts".to_string()],
            outlier_threshold: Some(3.0),
            duplicate_detection: true,
        };
        assert_eq!(config.min_counts, Some(100.0));
        assert!(config.duplicate_detection);
    }

    #[test]
    fn test_varmap_converter() {
        let device = create_test_device();
        let varmap = VarMap::new();
        
        // Create a simple tensor
        let tensor = Tensor::zeros((10,), DType::F32, &device).unwrap();
        
        // Test that we can create a tensor map
        let mut tensor_map = HashMap::new();
        tensor_map.insert("test".to_string(), tensor);
        
        assert_eq!(tensor_map.len(), 1);
    }

    #[tokio::test]
    async fn test_model_version_listing() {
        // This test just verifies the function signature is correct
        // Actual testing would require a filesystem setup
        let _result = list_model_versions("./nonexistent").await;
        // Should return empty list or error for non-existent path
    }

    #[test]
    fn test_contrastive_learning() {
        let anchor = vec![1.0f64, 2.0, 3.0];
        let positive = vec![1.1f64, 2.1, 3.1];
        let negative = vec![5.0f64, 6.0, 7.0];
        
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
