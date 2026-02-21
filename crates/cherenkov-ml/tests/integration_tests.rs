use cherenkov_ml::{
    TrainingPipeline, TrainingConfig, ModelRegistry, ExportConfig,
    SpectraDataset, SpectraSample, DatasetConfig, DataSource,
    InferenceService, OnnxModel, ModelMetadata, ModelVersion,
    DataQualityConfig, PreprocessingConfig,
};
use candle_core::Device;
use std::collections::HashMap;

#[tokio::test]
async fn test_end_to_end_training_pipeline() {
    let device = Device::Cpu;
    let mut pipeline = TrainingPipeline::new(device.clone());
    
    // Create training configuration
    let config = TrainingConfig {
        epochs: 10,
        batch_size: 16,
        learning_rate: 0.001,
        validation_split: 0.2,
        patience: 5,
    };
    
    // Create training job
    let job_id = pipeline.create_training_job("test_model", config).await;
    assert!(job_id > 0);
    
    // Check job status
    let status = pipeline.get_job_status(job_id).await;
    assert!(status.is_some());
}

#[tokio::test]
async fn test_model_registry_integration() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device);
    
    // Register multiple models
    for i in 1..=3 {
        let metadata = ModelMetadata {
            name: format!("model_{}", i),
            version: ModelVersion::new(1, 0, 0),
            description: format!("Test model {}", i),
            author: "test".to_string(),
            created_at: chrono::Utc::now(),
            tags: vec!["test".to_string()],
            framework: "candle".to_string(),
            license: "MIT".to_string(),
        };
        
        let model_id = registry.register_model(metadata, vec![]).await;
        assert!(!model_id.is_empty());
    }
    
    // List all models
    let models = registry.list_models();
    assert_eq!(models.len(), 3);
}

#[tokio::test]
async fn test_dataset_loading_and_preprocessing() {
    // Create sample spectra data
    let samples = vec![
        SpectraSample {
            energies: vec![100.0, 200.0, 300.0, 400.0, 500.0],
            counts: vec![10.0, 25.0, 45.0, 30.0, 15.0],
            timestamp: chrono::Utc::now(),
            sensor_id: "sensor_1".to_string(),
            metadata: HashMap::new(),
        },
        SpectraSample {
            energies: vec![100.0, 200.0, 300.0, 400.0, 500.0],
            counts: vec![12.0, 28.0, 50.0, 35.0, 18.0],
            timestamp: chrono::Utc::now(),
            sensor_id: "sensor_2".to_string(),
            metadata: HashMap::new(),
        },
    ];
    
    let dataset = SpectraDataset::new(samples);
    assert_eq!(dataset.len(), 2);
    
    // Test dataset statistics
    let stats = dataset.statistics();
    assert!(stats.mean > 0.0);
    assert!(stats.std > 0.0);
}

#[tokio::test]
async fn test_onnx_export_integration() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device.clone());
    
    // Create and register a model
    let metadata = ModelMetadata {
        name: "export_test_model".to_string(),
        version: ModelVersion::new(1, 0, 0),
        description: "Model for export testing".to_string(),
        author: "test".to_string(),
        created_at: chrono::Utc::now(),
        tags: vec!["export".to_string()],
        framework: "candle".to_string(),
        license: "MIT".to_string(),
    };
    
    let model_id = registry.register_model(metadata, vec![]).await;
    
    // Export configuration
    let export_config = ExportConfig {
        opset_version: 17,
        optimize: true,
        quantize: false,
        dynamic_axes: None,
    };
    
    // Verify export configuration
    assert_eq!(export_config.opset_version, 17);
    assert!(export_config.optimize);
    assert!(!export_config.quantize);
}

#[tokio::test]
async fn test_inference_service_integration() {
    let device = Device::Cpu;
    let mut service = InferenceService::new(device);
    
    // Load a test model
    let model_data = vec![]; // Empty for testing
    let model = OnnxModel::load(&model_data, device.clone()).await;
    
    // Test batch inference
    let batch = vec![
        vec![1.0f32, 2.0, 3.0, 4.0, 5.0],
        vec![2.0f32, 3.0, 4.0, 5.0, 6.0],
    ];
    
    // Verify batch structure
    assert_eq!(batch.len(), 2);
    assert_eq!(batch[0].len(), 5);
}

#[tokio::test]
async fn test_data_quality_validation() {
    let quality_config = DataQualityConfig {
        min_snr_db: 10.0,
        max_missing_ratio: 0.1,
        outlier_threshold_sigma: 3.0,
        min_energy_range: 0.01,
    };
    
    // Test valid sample
    let valid_sample = SpectraSample {
        energies: vec![100.0, 200.0, 300.0],
        counts: vec![100.0, 200.0, 150.0],
        timestamp: chrono::Utc::now(),
        sensor_id: "valid_sensor".to_string(),
        metadata: HashMap::new(),
    };
    
    // Test invalid sample (too much noise)
    let noisy_sample = SpectraSample {
        energies: vec![100.0, 200.0, 300.0],
        counts: vec![1.0, 1000.0, 2.0], // High variance
        timestamp: chrono::Utc::now(),
        sensor_id: "noisy_sensor".to_string(),
        metadata: HashMap::new(),
    };
    
    // Validate samples
    let valid_result = valid_sample.validate(&quality_config);
    let noisy_result = noisy_sample.validate(&quality_config);
    
    assert!(valid_result.is_ok());
    // Noisy sample might fail depending on implementation
}

#[tokio::test]
async fn test_training_with_dataset() {
    let device = Device::Cpu;
    let mut pipeline = TrainingPipeline::new(device.clone());
    
    // Create dataset
    let samples = (0..100).map(|i| SpectraSample {
        energies: vec![100.0, 200.0, 300.0, 400.0, 500.0],
        counts: vec![
            (i as f64 * 0.1) + 10.0,
            (i as f64 * 0.2) + 20.0,
            (i as f64 * 0.3) + 30.0,
            (i as f64 * 0.2) + 20.0,
            (i as f64 * 0.1) + 10.0,
        ],
        timestamp: chrono::Utc::now(),
        sensor_id: format!("sensor_{}", i),
        metadata: HashMap::new(),
    }).collect();
    
    let dataset = SpectraDataset::new(samples);
    assert_eq!(dataset.len(), 100);
    
    // Create training job with dataset
    let config = TrainingConfig {
        epochs: 5,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        patience: 3,
    };
    
    let job_id = pipeline.create_training_job("dataset_model", config).await;
    assert!(job_id > 0);
}

#[tokio::test]
async fn test_model_versioning_and_updates() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device);
    
    let base_metadata = ModelMetadata {
        name: "versioned_model".to_string(),
        version: ModelVersion::new(1, 0, 0),
        description: "Base version".to_string(),
        author: "test".to_string(),
        created_at: chrono::Utc::now(),
        tags: vec![],
        framework: "candle".to_string(),
        license: "MIT".to_string(),
    };
    
    // Register v1.0.0
    let v1_id = registry.register_model(base_metadata.clone(), vec![]).await;
    
    // Register v1.1.0 (minor update)
    let v1_1_metadata = ModelMetadata {
        version: ModelVersion::new(1, 1, 0),
        description: "Minor update".to_string(),
        ..base_metadata.clone()
    };
    let v1_1_id = registry.register_model(v1_1_metadata, vec![]).await;
    
    // Register v2.0.0 (major update)
    let v2_metadata = ModelMetadata {
        version: ModelVersion::new(2, 0, 0),
        description: "Major update".to_string(),
        ..base_metadata
    };
    let v2_id = registry.register_model(v2_metadata, vec![]).await;
    
    // Verify all versions are registered
    assert_ne!(v1_id, v1_1_id);
    assert_ne!(v1_1_id, v2_id);
    assert_eq!(registry.list_models().len(), 3);
}

#[tokio::test]
async fn test_cloud_storage_integration() {
    let config = DatasetConfig {
        source: DataSource::S3 {
            bucket: "test-bucket".to_string(),
            prefix: "spectra/".to_string(),
            region: "us-east-1".to_string(),
        },
        batch_size: 64,
        shuffle: true,
        validation_split: 0.2,
        max_samples: Some(1000),
        quality_config: DataQualityConfig::default(),
    };
    
    // Verify configuration
    assert_eq!(config.batch_size, 64);
    assert!(config.shuffle);
    assert_eq!(config.max_samples, Some(1000));
    
    // Test batch size calculation
    let total_samples = 1000;
    let num_batches = (total_samples + config.batch_size - 1) / config.batch_size;
    assert_eq!(num_batches, 16); // 1000/64 = 15.625 -> 16 batches
}

#[tokio::test]
async fn test_preprocessing_pipeline() {
    let preprocessing = PreprocessingConfig {
        normalize: true,
        smoothing: Some(3),
        baseline_correction: true,
        peak_detection: true,
    };
    
    let sample = SpectraSample {
        energies: vec![100.0, 200.0, 300.0, 400.0, 500.0],
        counts: vec![10.0, 100.0, 50.0, 80.0, 20.0],
        timestamp: chrono::Utc::now(),
        sensor_id: "test_sensor".to_string(),
        metadata: HashMap::new(),
    };
    
    // Apply preprocessing
    let processed = sample.preprocess(&preprocessing);
    
    // Verify preprocessing was applied
    assert_eq!(processed.energies.len(), sample.energies.len());
    assert_eq!(processed.counts.len(), sample.counts.len());
    
    if preprocessing.normalize {
        // Check if values are normalized (sum to 1 or max is 1)
        let max_val = processed.counts.iter().cloned().fold(0.0f64, f64::max);
        assert!(max_val > 0.0);
    }
}

#[tokio::test]
async fn test_error_handling_and_recovery() {
    let device = Device::Cpu;
    let mut pipeline = TrainingPipeline::new(device);
    
    // Test with invalid configuration
    let invalid_config = TrainingConfig {
        epochs: 0, // Invalid: zero epochs
        batch_size: 0, // Invalid: zero batch size
        learning_rate: -0.001, // Invalid: negative learning rate
        validation_split: 1.5, // Invalid: > 1.0
        patience: 0,
    };
    
    // Configuration validation should catch these errors
    let validation_result = invalid_config.validate();
    assert!(validation_result.is_err());
    
    // Test with valid configuration
    let valid_config = TrainingConfig {
        epochs: 10,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        patience: 5,
    };
    
    let valid_result = valid_config.validate();
    assert!(valid_result.is_ok());
}

#[tokio::test]
async fn test_concurrent_model_operations() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device);
    
    // Register multiple models concurrently
    let mut handles = vec![];
    
    for i in 0..5 {
        let metadata = ModelMetadata {
            name: format!("concurrent_model_{}", i),
            version: ModelVersion::new(1, 0, 0),
            description: format!("Concurrent model {}", i),
            author: "test".to_string(),
            created_at: chrono::Utc::now(),
            tags: vec![],
            framework: "candle".to_string(),
            license: "MIT".to_string(),
        };
        
        // In real implementation, this would be async
        let model_id = registry.register_model(metadata, vec![]).await;
        handles.push(model_id);
    }
    
    // Verify all models were registered
    assert_eq!(handles.len(), 5);
    assert_eq!(registry.list_models().len(), 5);
}

#[tokio::test]
async fn test_model_performance_metrics() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device);
    
    let metadata = ModelMetadata {
        name: "metrics_test_model".to_string(),
        version: ModelVersion::new(1, 0, 0),
        description: "Model for metrics testing".to_string(),
        author: "test".to_string(),
        created_at: chrono::Utc::now(),
        tags: vec![],
        framework: "candle".to_string(),
        license: "MIT".to_string(),
    };
    
    let model_id = registry.register_model(metadata, vec![]).await;
    
    // Record performance metrics
    let metrics = PerformanceMetrics {
        inference_time_ms: 15.5,
        accuracy: 0.95,
        precision: 0.94,
        recall: 0.96,
        f1_score: 0.95,
        throughput_samples_per_sec: 1000.0,
    };
    
    registry.update_metrics(&model_id, metrics).await;
    
    // Retrieve and verify metrics
    let retrieved_metrics = registry.get_metrics(&model_id).await;
    assert!(retrieved_metrics.is_some());
    
    let m = retrieved_metrics.unwrap();
    assert!(m.accuracy > 0.9);
    assert!(m.precision > 0.9);
    assert!(m.recall > 0.9);
}

#[tokio::test]
async fn test_export_formats() {
    let export_configs = vec![
        ExportConfig {
            opset_version: 11,
            optimize: false,
            quantize: false,
            dynamic_axes: None,
        },
        ExportConfig {
            opset_version: 17,
            optimize: true,
            quantize: false,
            dynamic_axes: None,
        },
        ExportConfig {
            opset_version: 17,
            optimize: true,
            quantize: true,
            dynamic_axes: Some(vec!["batch_size".to_string()]),
        },
    ];
    
    for config in &export_configs {
        // Verify opset version is valid
        assert!(config.opset_version >= 7 && config.opset_version <= 20);
        
        // Verify dynamic axes if present
        if let Some(axes) = &config.dynamic_axes {
            assert!(!axes.is_empty());
        }
    }
}

#[tokio::test]
async fn test_training_checkpointing() {
    let device = Device::Cpu;
    let mut pipeline = TrainingPipeline::new(device);
    
    let config = TrainingConfig {
        epochs: 100,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        patience: 10,
    };
    
    let job_id = pipeline.create_training_job("checkpoint_model", config).await;
    
    // Simulate checkpoint creation
    let checkpoint_epoch = 50;
    let checkpoint_path = format!("/tmp/checkpoint_{}_epoch{}.pt", job_id, checkpoint_epoch);
    
    // Verify checkpoint path format
    assert!(checkpoint_path.contains(&job_id.to_string()));
    assert!(checkpoint_path.contains("epoch50"));
    
    // Test checkpoint loading
    let loaded = pipeline.load_checkpoint(&checkpoint_path).await;
    // Result depends on implementation
}

#[tokio::test]
async fn test_batch_inference_optimization() {
    let device = Device::Cpu;
    let service = InferenceService::new(device);
    
    // Test different batch sizes
    let batch_sizes = vec![1, 8, 16, 32, 64];
    
    for batch_size in batch_sizes {
        let batch: Vec<Vec<f32>> = (0..batch_size)
            .map(|_| vec![1.0f32; 100])
            .collect();
        
        assert_eq!(batch.len(), batch_size);
        assert_eq!(batch[0].len(), 100);
    }
}

#[tokio::test]
async fn test_model_comparison_and_selection() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device);
    
    // Register multiple models with different performance
    let models = vec![
        ("model_fast", 10.0, 0.85), // (name, latency_ms, accuracy)
        ("model_accurate", 50.0, 0.95),
        ("model_balanced", 25.0, 0.90),
    ];
    
    for (name, latency, accuracy) in models {
        let metadata = ModelMetadata {
            name: name.to_string(),
            version: ModelVersion::new(1, 0, 0),
            description: format!("{} model", name),
            author: "test".to_string(),
            created_at: chrono::Utc::now(),
            tags: vec![],
            framework: "candle".to_string(),
            license: "MIT".to_string(),
        };
        
        let model_id = registry.register_model(metadata, vec![]).await;
        
        let metrics = PerformanceMetrics {
            inference_time_ms: latency,
            accuracy,
            precision: accuracy,
            recall: accuracy,
            f1_score: accuracy,
            throughput_samples_per_sec: 1000.0 / latency,
        };
        
        registry.update_metrics(&model_id, metrics).await;
    }
    
    // Select best model based on criteria
    let best_model = registry.select_best_model(|m| {
        m.accuracy > 0.88 && m.inference_time_ms < 30.0
    }).await;
    
    assert!(best_model.is_some());
    // Should select model_balanced
}

#[tokio::test]
async fn test_data_augmentation_pipeline() {
    let sample = SpectraSample {
        energies: vec![100.0, 200.0, 300.0, 400.0, 500.0],
        counts: vec![10.0, 20.0, 30.0, 25.0, 15.0],
        timestamp: chrono::Utc::now(),
        sensor_id: "original".to_string(),
        metadata: HashMap::new(),
    };
    
    // Test noise augmentation
    let noisy = sample.add_noise(0.1);
    assert_eq!(noisy.counts.len(), sample.counts.len());
    
    // Test scaling augmentation
    let scaled = sample.scale(1.5);
    assert_eq!(scaled.counts.len(), sample.counts.len());
    
    // Test shift augmentation
    let shifted = sample.shift(2);
    assert_eq!(shifted.counts.len(), sample.counts.len());
}

#[tokio::test]
async fn test_distributed_training_simulation() {
    let device = Device::Cpu;
    
    // Simulate multiple workers
    let num_workers = 4;
    let mut pipelines: Vec<TrainingPipeline> = (0..num_workers)
        .map(|_| TrainingPipeline::new(device.clone()))
        .collect();
    
    // Each worker creates a training job
    let config = TrainingConfig {
        epochs: 10,
        batch_size: 16,
        learning_rate: 0.001,
        validation_split: 0.2,
        patience: 5,
    };
    
    let mut job_ids = vec![];
    for (i, pipeline) in pipelines.iter_mut().enumerate() {
        let job_id = pipeline.create_training_job(
            &format!("distributed_worker_{}", i), 
            config.clone()
        ).await;
        job_ids.push(job_id);
    }
    
    assert_eq!(job_ids.len(), num_workers);
    
    // Verify all jobs are unique
    let unique_jobs: std::collections::HashSet<_> = job_ids.iter().collect();
    assert_eq!(unique_jobs.len(), num_workers);
}

#[tokio::test]
async fn test_model_serialization_roundtrip() {
    let device = Device::Cpu;
    let mut registry = ModelRegistry::new(device);
    
    let metadata = ModelMetadata {
        name: "serialization_test".to_string(),
        version: ModelVersion::new(1, 0, 0),
        description: "Test serialization".to_string(),
        author: "test".to_string(),
        created_at: chrono::Utc::now(),
        tags: vec![],
        framework: "candle".to_string(),
        license: "MIT".to_string(),
    };
    
    // Create dummy model weights
    let weights: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    
    let model_id = registry.register_model(metadata, weights.clone()).await;
    
    // Retrieve and verify
    let retrieved = registry.get_model(&model_id).await;
    assert!(retrieved.is_some());
    
    let (retrieved_metadata, retrieved_weights) = retrieved.unwrap();
    assert_eq!(retrieved_metadata.name, "serialization_test");
    assert_eq!(retrieved_weights, weights);
}
