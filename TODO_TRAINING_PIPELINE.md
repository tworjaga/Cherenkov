# Training Pipeline Integration TODO

## Tasks
- [ ] Add ONNX Export Functionality
  - [ ] Implement export_to_onnx() method in TrainingPipeline
  - [ ] Add ONNX export dependencies and utilities
  - [ ] Integrate with existing model export flow
- [ ] Connect to Real Radiation Spectra Datasets
  - [ ] Implement data loading from S3/cloud storage
  - [ ] Add support for common radiation data formats (CSV, HDF5, etc.)
  - [ ] Create data preprocessing pipeline for spectra normalization
- [ ] Implement Model Conversion from Trained Weights
  - [ ] Add conversion utilities from Candle VarMap to ONNX format
  - [ ] Ensure proper tensor shape and type mapping
  - [ ] Add validation of converted models
- [ ] Add Training Data Pipeline
  - [ ] Implement data ingestion from multiple sources
  - [ ] Add data quality validation and filtering
  - [ ] Create dataset versioning and caching

## Files to Modify
- crates/cherenkov-ml/src/training.rs
- crates/cherenkov-ml/Cargo.toml
- crates/cherenkov-ml/src/lib.rs
- crates/cherenkov-ml/src/data_loader.rs (new)
