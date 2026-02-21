# ML Pipeline Integration TODO

## Tasks
- [ ] Add ONNX dependencies to Cargo.toml
- [ ] Update TrainingPipeline with integrated ONNX export
- [ ] Update lib.rs exports
- [ ] Create comprehensive integration tests
- [ ] Add documentation
- [ ] Commit and push changes

## Implementation Steps

### Step 1: Dependencies
Add to `crates/cherenkov-ml/Cargo.toml`:
- candle-onnx = "0.6"
- prost = "0.12"

### Step 2: Training Pipeline Integration
Update `export_model()` method in `training.rs` to:
- Use OnnxExporter properly
- Add export configuration options
- Handle export errors gracefully
- Return export report

### Step 3: Library Exports
Update `lib.rs` to export:
- OnnxExporter
- ExportConfig
- ExportReport
- Conversion utilities

### Step 4: Testing
Create tests for:
- End-to-end training and export
- Model conversion validation
- Data pipeline integration
