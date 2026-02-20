# ONNX Model Loading Fix - Task Tracking

## Tasks
- [ ] Update candle-onnx dependency to 0.6.x
- [ ] Replace stub OnnxModel with proper implementation
- [ ] Add model validation (input/output shape verification, opset compatibility)
- [ ] Enhance error handling with specific error types
- [ ] Test with sample ONNX model
- [ ] Commit and push changes

## Files to Modify
1. `crates/cherenkov-ml/Cargo.toml` - Update dependency
2. `crates/cherenkov-ml/src/inference.rs` - Replace implementation
3. `crates/cherenkov-ml/src/lib.rs` - Update exports if needed

## Current Status
- candle-onnx version: 0.3 (outdated)
- OnnxModel is a stub implementation
- No model validation
- Basic error handling only
