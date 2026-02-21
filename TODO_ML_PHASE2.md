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
- [ ] Connect ML classification to stream pipeline
- [ ] Add confidence thresholds
- [ ] Implement classification caching
- [ ] Add multi-class anomaly detection

### 5. Model Management API
- [ ] Create REST/GraphQL endpoints
- [ ] Add model performance metrics
- [ ] Implement retraining triggers
- [ ] Add model explainability

## Current Status
- Phase 1 (Plume Simulation): Complete
- Phase 2 (ML Classification): In Progress
- ONNX Model Loading: Stubbed, needs fix
- Training Pipeline: Implemented, needs real data
