# ONNX Model Loading Fix - Task Tracking

## Tasks
- [x] Update candle-onnx dependency to 0.8.x (aligned with candle-core 0.8)
- [x] Replace stub OnnxModel with proper implementation
- [x] Add model validation (input/output shape verification, opset compatibility)
- [x] Enhance error handling with specific error types
- [ ] Test with sample ONNX model
- [x] Commit and push changes

## Files Modified
1. `crates/cherenkov-ml/Cargo.toml` - Updated candle-onnx to 0.8
2. `crates/cherenkov-ml/src/inference.rs` - Full OnnxModel implementation with validation
3. `crates/cherenkov-ml/src/lib.rs` - Exports OnnxModel, OnnxError, ModelMetadata

## Current Status
- candle-onnx version: 0.8 (current, aligned with candle-core 0.8)
- OnnxModel: Full implementation with metadata extraction and validation
- Model validation: Input/output shape verification, opset compatibility (7-21)
- Error handling: Comprehensive OnnxError enum with specific error types
- InferenceService: Batch inference support with model hot-swapping

## Implementation Details

### OnnxModel Features
- Load and parse ONNX models from file
- Extract metadata (inputs, outputs, shapes, opset version)
- Validate input tensor shapes against model requirements
- Run inference with candle-onnx simple_eval
- Support batch inference detection

### Error Types (OnnxError)
- FileRead: IO errors during model loading
- ParseError: Protobuf decode failures
- MissingGraph/Inputs/Outputs: Model structure validation
- UnsupportedOpset: Version compatibility check
- InputShapeMismatch: Dimension validation
- InputNotFound: Input name verification
- InferenceFailed: Runtime errors
- NoOutput: Output extraction failures
- ValidationFailed: Custom validation errors

### Next Steps
- Create unit tests for OnnxModel validation
- Add sample ONNX model fixture for testing
- Verify batch inference with real data
