# ML Pipeline Implementation TODO

## Phase 1: Fix ONNX Export Functionality
- [x] Create implementation tracking document
- [ ] Fix placeholder neural network construction in onnx_export.rs
- [ ] Implement proper weight extraction from VarMap matching training architecture
- [ ] Add proper tensor shape propagation through layers
- [ ] Enhance validation with actual inference testing
- [ ] Commit changes

## Phase 2: Implement Cloud Data Loading
- [ ] Create data_loader.rs with S3/cloud storage client
- [ ] Implement CSV, HDF5, JSON format support
- [ ] Add spectra normalization pipeline
- [ ] Add data quality validation
- [ ] Commit changes

## Phase 3: Connect Training to Real Data
- [ ] Replace synthetic data with real data loader in training.rs
- [ ] Add dataset versioning and caching
- [ ] Implement stratified sampling
- [ ] Commit changes

## Phase 4: Model Conversion Utilities
- [ ] Create conversion.rs with VarMap to ONNX conversion
- [ ] Add tensor shape and dtype validation
- [ ] Implement conversion verification
- [ ] Commit changes

## Phase 5: API Integration
- [ ] Add export_to_onnx() mutation to model_management.rs
- [ ] Add data source configuration endpoints
- [ ] Add training job monitoring
- [ ] Commit changes

## Phase 6: Dependencies and Testing
- [ ] Update Cargo.toml with new dependencies
- [ ] Add comprehensive tests
- [ ] End-to-end integration testing
- [ ] Commit changes
