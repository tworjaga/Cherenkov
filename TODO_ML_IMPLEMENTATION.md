# ML Pipeline Implementation TODO

## Phase 1: Fix ONNX Export Functionality
- [x] Create implementation tracking document
- [x] Fix placeholder neural network construction in onnx_export.rs
- [x] Implement proper weight extraction from VarMap matching training architecture
- [x] Add proper tensor shape propagation through layers
- [x] Enhance validation with actual inference testing
- [x] Commit changes


## Phase 2: Implement Cloud Data Loading
- [x] Create data_loader.rs with S3/cloud storage client
- [x] Implement CSV, HDF5, JSON format support
- [x] Add spectra normalization pipeline
- [x] Add data quality validation
- [x] Commit changes

## Phase 3: Connect Training to Real Data
- [x] Replace synthetic data with real data loader in training.rs
- [x] Add dataset versioning and caching
- [x] Implement stratified sampling
- [x] Commit chaxges

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
