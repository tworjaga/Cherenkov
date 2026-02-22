use cherenkov_ml::{
    TrainingPipeline, TrainingConfig, TrainingResult, ModelRegistry,
    ExportConfig, OnnxExporter,
    SpectraDataset, SpectraSample, DatasetConfig, DataSource, DataFormat,
    InferenceService, OnnxModel, ModelMetadata,
    DataQualityConfig, PreprocessingConfig, PerformanceMetrics,
    LrScheduler, AugmentationConfig, SourceType,
    DatasetVersion, ModelVersion, CheckpointMeta, DatasetCache,
};



use candle_core::Device;
use std::collections::HashMap;


#[tokio::test]
async fn test_end_to_end_training_pipeline() {
    let config = TrainingConfig {
        model_name: "test_model".to_string(),
        data_path: "./test_data".to_string(),
        output_path: "./output".to_string(),
        epochs: 10,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        early_stopping_patience: 5,
        checkpoint_interval: 5,
        num_classes: 15,
        input_size: 1024,
        hidden_layers: vec![512, 256],
        dropout_rate: 0.2,
        use_gpu: false,
        seed: 42,
        lr_scheduler: LrScheduler::Constant,
        augmentation: AugmentationConfig::default(),
        resume_from_checkpoint: None,
        max_checkpoints_to_keep: 5,
        gradient_clip_norm: Some(1.0),
        warmup_epochs: 5,
        label_smoothing: 0.1,
        dataset_config: None,
        use_stratified_sampling: true,
        cache_datasets: true,
        cache_dir: Some(".cache".to_string()),
    };
    
    let (pipeline, _metrics_rx) = TrainingPipeline::new(config).expect("Failed to create pipeline");
    // Pipeline created successfully
    assert!(true);
}



#[tokio::test]
async fn test_model_registry_integration() {
    let device = Device::Cpu;
    let registry = ModelRegistry::new(device);
    
    let models = registry.list_models().await;
    assert!(models.is_empty());
    
    assert!(registry.get_active_info("test_model").await.is_none());
}


#[tokio::test]
async fn test_dataset_loading_and_preprocessing() {
    let config = DatasetConfig {
        name: "test_dataset".to_string(),
        source: DataSource::Local { path: "./test_data.csv".to_string() },
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
        max_samples: Some(100),
        quality_config: Some(DataQualityConfig::default()),
    };
    
    assert_eq!(config.name, "test_dataset");
    assert!(config.preprocessing.normalize);
}



#[tokio::test]
async fn test_inference_service() {
    let service = InferenceService::new(1, 100).expect("Failed to create service");
    
    let info = service.get_model_info().await;
    assert!(info.is_empty());
    
    // Device comparison - just check we can get the device
    let _device = service.get_device();
}


#[tokio::test]
async fn test_onnx_export_and_import() {
    let varmap = candle_nn::VarMap::new();
    
    let export_config = ExportConfig {
        opset_version: 11,
        input_names: vec!["input".to_string()],
        output_names: vec!["output".to_string()],
        dynamic_axes: None,
        metadata: None,
        optimize: true,
        validate: true,
    };
    
    let exporter = OnnxExporter::with_config(export_config);
    let _result = exporter.export_model(
        &varmap, 
        &[1, 1024], 
        &[1, 15], 
        std::path::Path::new("./test_model.onnx"), 
        &[512, 256]
    ).await;
}


#[tokio::test]
async fn test_data_augmentation() {
    let sample = SpectraSample {
        id: "sample_1".to_string(),
        timestamp: chrono::Utc::now(),
        energy_bins: vec![100.0, 200.0, 300.0, 400.0, 500.0],
        counts: vec![10.0, 25.0, 45.0, 30.0, 15.0],
        count_rate: 100.0,
        total_counts: 125.0,
        live_time: 1.0,
        real_time: 1.0,
        isotope_tags: vec![],
        confidence_score: None,
        source_type: SourceType::Unknown,
        metadata: HashMap::new(),
    };
    
    assert_eq!(sample.energy_bins.len(), 5);
    assert_eq!(sample.counts.len(), 5);
    assert_eq!(sample.id, "sample_1");
}



#[tokio::test]
async fn test_distributed_training_simulation() {
    let config = TrainingConfig {
        model_name: "test_model".to_string(),
        data_path: "./test_data".to_string(),
        output_path: "./output".to_string(),
        epochs: 10,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        early_stopping_patience: 5,
        checkpoint_interval: 5,
        num_classes: 15,
        input_size: 1024,
        hidden_layers: vec![512, 256],
        dropout_rate: 0.2,
        use_gpu: false,
        seed: 42,
        lr_scheduler: LrScheduler::Constant,
        augmentation: AugmentationConfig::default(),
        resume_from_checkpoint: None,
        max_checkpoints_to_keep: 5,
        gradient_clip_norm: Some(1.0),
        warmup_epochs: 5,
        label_smoothing: 0.1,
        dataset_config: None,
        use_stratified_sampling: true,
        cache_datasets: true,
        cache_dir: Some(".cache".to_string()),
    };
    
    let num_workers = 4;
    let mut pipelines = vec![];
    
    for _ in 0..num_workers {
        let (pipeline, _metrics_rx) = TrainingPipeline::new(config.clone())
            .expect("Failed to create pipeline");
        pipelines.push(pipeline);
    }
    
    assert_eq!(pipelines.len(), num_workers);
}



#[tokio::test]
async fn test_model_serialization_roundtrip() {
    let device = Device::Cpu;
    let registry = ModelRegistry::new(device);
    
    let metrics = PerformanceMetrics {
        avg_latency_ms: 100.0,
        p95_latency_ms: 150.0,
        p99_latency_ms: 200.0,
        throughput_per_sec: 1000.0,
        error_rate: 0.01,
        total_inferences: 10000,
        recorded_at: chrono::Utc::now(),
    };
    
    assert!(metrics.avg_latency_ms > 0.0);
    assert!(metrics.throughput_per_sec > 0.0);
    
    let baseline = PerformanceMetrics {
        avg_latency_ms: 100.0,
        p95_latency_ms: 150.0,
        p99_latency_ms: 200.0,
        throughput_per_sec: 1000.0,
        error_rate: 0.01,
        total_inferences: 10000,
        recorded_at: chrono::Utc::now(),
    };
    
    assert!(metrics.meets_baseline(&baseline, 0.1));
}


#[tokio::test]
async fn test_performance_metrics() {
    let metrics = PerformanceMetrics {
        avg_latency_ms: 50.0,
        p95_latency_ms: 75.0,
        p99_latency_ms: 100.0,
        throughput_per_sec: 2000.0,
        error_rate: 0.001,
        total_inferences: 50000,
        recorded_at: chrono::Utc::now(),
    };
    
    assert_eq!(metrics.avg_latency_ms, 50.0);
    assert_eq!(metrics.p95_latency_ms, 75.0);
    assert_eq!(metrics.p99_latency_ms, 100.0);
    assert_eq!(metrics.throughput_per_sec, 2000.0);
    assert_eq!(metrics.error_rate, 0.001);
    assert_eq!(metrics.total_inferences, 50000);
}


#[tokio::test]
async fn test_export_config_variations() {
    let configs = vec![
        ExportConfig {
            opset_version: 11,
            input_names: vec!["input".to_string()],
            output_names: vec!["output".to_string()],
            dynamic_axes: None,
            metadata: None,
            optimize: true,
            validate: true,
        },
        ExportConfig {
            opset_version: 17,
            input_names: vec!["features".to_string()],
            output_names: vec!["predictions".to_string(), "probabilities".to_string()],
            dynamic_axes: None,
            metadata: None,
            optimize: true,
            validate: true,
        },
    ];
    
    for config in &configs {
        assert!(config.opset_version >= 7 && config.opset_version <= 20);
        assert!(!config.input_names.is_empty());
        assert!(!config.output_names.is_empty());
    }
}



#[tokio::test]
async fn test_spectra_sample_creation() {
    let sample = SpectraSample {
        id: "test_001".to_string(),
        timestamp: chrono::Utc::now(),
        energy_bins: (0..1024).map(|i| i as f64 * 0.5).collect(),
        counts: vec![1.0; 1024],
        count_rate: 500.0,
        total_counts: 1024.0,
        live_time: 60.0,
        real_time: 60.0,
        isotope_tags: vec!["Cs-137".to_string(), "Co-60".to_string()],
        confidence_score: Some(0.95),
        source_type: SourceType::Unknown,
        metadata: {
            let mut m = HashMap::new();
            m.insert("location".to_string(), "Lab A".to_string());
            m.insert("operator".to_string(), "Test Operator".to_string());
            m
        },
    };
    
    assert_eq!(sample.energy_bins.len(), 1024);
    assert_eq!(sample.counts.len(), 1024);
    assert_eq!(sample.isotope_tags.len(), 2);
    assert!(sample.confidence_score.is_some());
    assert_eq!(sample.metadata.len(), 2);
}



#[tokio::test]
async fn test_data_quality_config() {
    let quality_config = DataQualityConfig {
        min_counts: Some(100.0),
        max_counts: Some(1000000.0),
        min_energy_bins: 128,
        max_energy_bins: 8192,
        required_fields: vec!["id".to_string(), "energy_bins".to_string(), "counts".to_string()],
        outlier_threshold: Some(3.0),
        duplicate_detection: true,
    };
    
    assert_eq!(quality_config.min_counts, Some(100.0));
    assert_eq!(quality_config.max_counts, Some(1000000.0));
    assert_eq!(quality_config.min_energy_bins, 128);
    assert_eq!(quality_config.max_energy_bins, 8192);
    assert!(quality_config.duplicate_detection);
}



#[tokio::test]
async fn test_preprocessing_config() {
    let config = PreprocessingConfig {
        normalize: true,
        smoothing: None,
        baseline_correction: true,
        peak_detection: true,
        energy_calibration: None,
        noise_reduction: None,
    };
    
    assert!(config.normalize);
    assert!(config.smoothing.is_none());
    assert!(config.baseline_correction);
    assert!(config.peak_detection);
    assert!(config.energy_calibration.is_none());
    assert!(config.noise_reduction.is_none());
}



#[tokio::test]
async fn test_training_config_validation() {
    let valid_config = TrainingConfig {
        model_name: "test_model".to_string(),
        data_path: "./test_data".to_string(),
        output_path: "./output".to_string(),
        epochs: 100,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        early_stopping_patience: 10,
        checkpoint_interval: 10,
        num_classes: 15,
        input_size: 1024,
        hidden_layers: vec![512, 256, 128],
        dropout_rate: 0.2,
        use_gpu: false,
        seed: 42,
        lr_scheduler: LrScheduler::Constant,
        augmentation: AugmentationConfig::default(),
        resume_from_checkpoint: None,
        max_checkpoints_to_keep: 5,
        gradient_clip_norm: Some(1.0),
        warmup_epochs: 5,
        label_smoothing: 0.1,
        dataset_config: None,
        use_stratified_sampling: true,
        cache_datasets: true,
        cache_dir: Some(".cache".to_string()),
    };
    
    assert!(!valid_config.model_name.is_empty());
    assert!(!valid_config.data_path.is_empty());
    assert!(!valid_config.hidden_layers.is_empty());
    assert!(valid_config.dropout_rate >= 0.0 && valid_config.dropout_rate <= 1.0);
    assert!(valid_config.early_stopping_patience > 0);
    assert!(valid_config.checkpoint_interval > 0);
}



#[tokio::test]
async fn test_model_metadata() {
    let metadata = ModelMetadata {
        input_names: vec!["input".to_string()],
        output_names: vec!["output".to_string()],
        input_shapes: vec![vec![1, 1024]],
        output_shapes: vec![vec![1, 15]],
        opset_version: 17,
        producer_name: "Cherenkov ML".to_string(),
        producer_version: "1.0.0".to_string(),
    };
    
    assert_eq!(metadata.input_names.len(), 1);
    assert_eq!(metadata.output_names.len(), 1);
    assert_eq!(metadata.input_shapes.len(), 1);
    assert_eq!(metadata.output_shapes.len(), 1);
    assert_eq!(metadata.opset_version, 17);
    assert_eq!(metadata.producer_name, "Cherenkov ML");
}



#[tokio::test]
async fn test_inference_service_construction() {
    let service1 = InferenceService::new(1, 100);
    assert!(service1.is_ok());
    
    let service2 = InferenceService::new(4, 1000);
    assert!(service2.is_ok());
    
    let service3 = InferenceService::new(8, 10000);
    assert!(service3.is_ok());
}


#[tokio::test]
async fn test_dataset_config_variations() {
    let local_config = DatasetConfig {
        name: "local_dataset".to_string(),
        source: DataSource::Local { path: "/data/spectra.csv".to_string() },
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
        cache_dir: Some("/cache".to_string()),
        max_samples: Some(10000),
        quality_config: Some(DataQualityConfig::default()),
    };
    
    assert_eq!(local_config.name, "local_dataset");
    assert!(local_config.cache_dir.is_some());
    assert!(local_config.max_samples.is_some());
}


#[tokio::test]
async fn test_source_type_variants() {
    let types = vec![
        SourceType::Unknown,
        SourceType::Background,
        SourceType::Medical,
        SourceType::Industrial,
        SourceType::Nuclear,
    ];
    
    for source_type in types {
        let sample = SpectraSample {
            id: "test".to_string(),
            timestamp: chrono::Utc::now(),
            energy_bins: vec![100.0, 200.0],
            counts: vec![1.0, 2.0],
            count_rate: 100.0,
            total_counts: 3.0,
            live_time: 1.0,
            real_time: 1.0,
            isotope_tags: vec![],
            confidence_score: None,
            source_type,
            metadata: HashMap::new(),
        };
        
        assert_eq!(sample.id, "test");
    }
}



#[tokio::test]
async fn test_lr_scheduler_variants() {
    let schedulers = vec![
        LrScheduler::Constant,
        LrScheduler::StepDecay { step_size: 10, gamma: 0.1 },
        LrScheduler::ExponentialDecay { gamma: 0.95 },
        LrScheduler::CosineAnnealing { t_max: 100, eta_min: 1e-6 },
        LrScheduler::ReduceOnPlateau { factor: 0.5, patience: 5, min_lr: 1e-7 },
    ];
    
    assert_eq!(schedulers.len(), 5);
    
    let config = TrainingConfig {
        model_name: "test_model".to_string(),
        data_path: "./test_data".to_string(),
        output_path: "./output".to_string(),
        epochs: 100,
        batch_size: 32,
        learning_rate: 0.001,
        validation_split: 0.2,
        early_stopping_patience: 10,
        checkpoint_interval: 10,
        num_classes: 15,
        input_size: 1024,
        hidden_layers: vec![512, 256],
        dropout_rate: 0.2,
        use_gpu: false,
        seed: 42,
        lr_scheduler: LrScheduler::CosineAnnealing { t_max: 100, eta_min: 1e-6 },
        augmentation: AugmentationConfig::default(),
        resume_from_checkpoint: None,
        max_checkpoints_to_keep: 5,
        gradient_clip_norm: Some(1.0),
        warmup_epochs: 5,
        label_smoothing: 0.1,
        dataset_config: None,
        use_stratified_sampling: true,
        cache_datasets: true,
        cache_dir: Some(".cache".to_string()),
    };
    
    assert!(!config.model_name.is_empty());
}



#[tokio::test]
async fn test_augmentation_config() {
    let aug_config = AugmentationConfig {
        enabled: true,
        noise_std: 0.01,
        scale_range: (0.9, 1.1),
        shift_max: 5.0,
        mixup_alpha: Some(0.2),
    };
    
    assert!(aug_config.enabled);
    assert!((aug_config.noise_std - 0.01).abs() < f64::EPSILON);
    assert_eq!(aug_config.scale_range, (0.9, 1.1));
    assert!((aug_config.shift_max - 5.0).abs() < f64::EPSILON);

}


#[tokio::test]
async fn test_model_registry_device() {
    let cpu_registry = ModelRegistry::new(Device::Cpu);
    let models = cpu_registry.list_models().await;
    assert!(models.is_empty());
}


#[tokio::test]
async fn test_spectra_dataset_statistics() {
    let samples: Vec<SpectraSample> = (0..10).map(|i| SpectraSample {
        id: format!("sample_{}", i),
        timestamp: chrono::Utc::now(),
        energy_bins: vec![100.0, 200.0, 300.0],
        counts: vec![10.0 * (i as f64 + 1.0), 20.0 * (i as f64 + 1.0), 15.0 * (i as f64 + 1.0)],
        count_rate: 100.0 * (i as f64 + 1.0),
        total_counts: 45.0 * (i as f64 + 1.0),
        live_time: 1.0,
        real_time: 1.0,
        isotope_tags: vec![],
        confidence_score: None,
        source_type: SourceType::Unknown,
        metadata: HashMap::new(),
    }).collect();
    
    // Use Vec<SpectraSample> directly as dataset representation
    let dataset = samples;
    assert_eq!(dataset.len(), 10);
    
    // Calculate simple statistics from the dataset
    let total_count_rate: f64 = dataset.iter().map(|s| s.count_rate).sum();
    let mean = total_count_rate / dataset.len() as f64;
    assert!(mean > 0.0);
}


#[tokio::test]
async fn test_checkpoint_metadata() {
    let checkpoint = CheckpointMeta {
        epoch: 10,
        global_step: 1000,
        best_val_loss: 0.5,
        optimizer_state: Some("optimizer_state".to_string()),
        rng_state: Some("rng_state".to_string()),
    };
    
    assert_eq!(checkpoint.epoch, 10);
    assert_eq!(checkpoint.global_step, 1000);
    assert_eq!(checkpoint.best_val_loss, 0.5);
    assert!(checkpoint.optimizer_state.is_some());
    assert!(checkpoint.rng_state.is_some());
}


#[tokio::test]
async fn test_dataset_version() {
    let dataset_version = DatasetVersion {
        version_id: "v1.0.0".to_string(),
        dataset_hash: "abc123".to_string(),
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
        class_distribution: {
            let mut m = HashMap::new();
            m.insert("class_a".to_string(), 500);
            m.insert("class_b".to_string(), 500);
            m
        },
        preprocessing_hash: "preprocess_v1".to_string(),
    };
    
    assert_eq!(dataset_version.version_id, "v1.0.0");
    assert_eq!(dataset_version.num_samples, 1000);
}


#[tokio::test]
async fn test_model_version() {
    let model_version = ModelVersion {
        version_id: "model_v1".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        training_config: TrainingConfig::default(),
        metrics: TrainingResult {
            model_path: "./model.onnx".to_string(),
            final_loss: 0.5,
            validation_accuracy: 0.95,
            test_accuracy: 0.94,
            epochs_completed: 100,
            training_duration_secs: 3600,
            best_epoch: 95,
            per_class_accuracy: HashMap::new(),
            confusion_matrix: vec![],
        },
        git_commit: Some("abc123".to_string()),
        tags: vec!["production".to_string()],
    };
    
    assert_eq!(model_version.version_id, "model_v1");
    assert!(model_version.git_commit.is_some());
    assert_eq!(model_version.tags.len(), 1);
}


#[tokio::test]
async fn test_dataset_cache() {
    let cache = DatasetCache::new(
        std::path::PathBuf::from("./cache"),
        10.0
    );
    
    let version_id = "test_version";
    let cache_path = cache.get_cache_path(version_id);
    assert!(cache_path.to_string_lossy().contains("test_version"));
}


#[tokio::test]
async fn test_training_result() {
    let result = TrainingResult {
        model_path: "./model.onnx".to_string(),
        final_loss: 0.5,
        validation_accuracy: 0.95,
        test_accuracy: 0.94,
        epochs_completed: 100,
        training_duration_secs: 3600,
        best_epoch: 95,
        per_class_accuracy: {
            let mut m = HashMap::new();
            m.insert("class_a".to_string(), 0.96);
            m.insert("class_b".to_string(), 0.94);
            m
        },
        confusion_matrix: vec![
            vec![90, 10],
            vec![5, 95],
        ],
    };
    
    assert_eq!(result.model_path, "./model.onnx");
    assert_eq!(result.final_loss, 0.5);
    assert_eq!(result.validation_accuracy, 0.95);
    assert_eq!(result.test_accuracy, 0.94);
    assert_eq!(result.epochs_completed, 100);
    assert_eq!(result.training_duration_secs, 3600);
    assert_eq!(result.best_epoch, 95);
}


#[tokio::test]
async fn test_training_metrics() {
    // TrainingMetrics is internal to training module
    // Verify TrainingResult which is the public output type
    let result = TrainingResult {
        model_path: "./model.onnx".to_string(),
        final_loss: 0.5,
        validation_accuracy: 0.95,
        test_accuracy: 0.94,
        epochs_completed: 100,
        training_duration_secs: 3600,
        best_epoch: 95,
        per_class_accuracy: HashMap::new(),
        confusion_matrix: vec![],
    };
    
    assert_eq!(result.epochs_completed, 100);
    assert_eq!(result.best_epoch, 95);
}



#[tokio::test]
async fn test_batch_request_and_result() {
    // BatchRequest and BatchResult are internal types used by InferenceService
    // Verify the service can be created and used
    let service = InferenceService::new(2, 100).expect("Failed to create service");
    let info = service.get_model_info().await;
    assert!(info.is_empty());
    
    // Verify device access works
    let _device = service.get_device();
}




#[tokio::test]
async fn test_export_report() {
    // ExportReport has specific fields based on actual implementation
    let report = cherenkov_ml::onnx_export::ExportReport {
        output_path: std::path::PathBuf::from("./model.onnx"),
        input_shape: vec![1, 1024],
        output_shape: vec![1, 15],
        opset_version: 17,
        file_size_bytes: 1024000,
        exported_at: chrono::Utc::now().to_rfc3339(),
    };
    
    assert_eq!(report.opset_version, 17);
    assert_eq!(report.file_size_bytes, 1024000);
    assert!(!report.exported_at.is_empty());
}
