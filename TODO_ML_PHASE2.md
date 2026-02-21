# Phase 2: ML-based Anomaly Classification - Implementation Tracking

## Tasks

### 1. Fix ONNX Model Loading
- [x] Update candle-onnx dependency to current version
- [x] Replace stub OnnxModel with proper implementation
- [x] Add model validation and error handling
- [x] Test with sample ONNX model

### 2. Training Pipeline Integration
- [x] Add ONNX export functionality
- [x] Connect to real radiation spectra datasets
- [x] Implement model conversion from trained weights
- [x] Add training data pipeline

### 3. Model Versioning and Hot-Swapping
- [x] Enhance model versioning with ONNX metadata
- [x] Implement atomic hot-swapping with rollback
- [x] Add performance monitoring
- [x] Create model registry

### 4. Anomaly Detection Integration
- [x] Connect ML classification to stream pipeline
- [x] Add confidence thresholds
- [x] Implement classification caching
- [x] Add multi-class anomaly detection

### 5. Model Management API
- [x] Create REST/GraphQL endpoints
- [x] Add model performance metrics
- [x] Implement retraining triggers
- [x] Add model explainability

## Current Status
- Phase 1 (Plume Simulation): Complete
- Phase 2 (ML Classification): In Progress
- ONNX Model Loading: Stubbed, needs fix
- Training Pipeline: Implemented, needs real data
