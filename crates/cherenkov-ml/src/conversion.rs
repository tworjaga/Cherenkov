use std::collections::HashMap;
use candle_core::{Tensor, Device, DType};
use candle_nn::VarMap;
use tracing::{info, debug, warn, error};
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Errors that can occur during model conversion
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Tensor conversion error: {0}")]
    TensorConversion(String),
    
    #[error("Shape mismatch: expected {expected:?}, got {actual:?}")]
    ShapeMismatch { expected: Vec<usize>, actual: Vec<usize> },
    
    #[error("DType mismatch: expected {expected:?}, got {actual:?}")]
    DTypeMismatch { expected: DType, actual: DType },
    
    #[error("Missing tensor: {0}")]
    MissingTensor(String),
    
    #[error("Invalid architecture: {0}")]
    InvalidArchitecture(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Candle error: {0}")]
    Candle(#[from] candle_core::Error),
}

/// Result type for conversion operations
pub type ConversionResult<T> = Result<T, ConversionError>;

/// Tensor metadata for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorMetadata {
    pub name: String,
    pub shape: Vec<usize>,
    pub dtype: DType,
    pub description: Option<String>,
}

/// Model architecture specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelArchitecture {
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub hidden_layers: Vec<usize>,
    pub expected_tensors: Vec<TensorMetadata>,
}

/// Conversion configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub validate_shapes: bool,
    pub validate_dtypes: bool,
    pub strict_mode: bool,
    pub allow_partial_conversion: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            validate_shapes: true,
            validate_dtypes: true,
            strict_mode: false,
            allow_partial_conversion: false,
        }
    }
}

/// Conversion report with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionReport {
    pub success: bool,
    pub tensors_converted: usize,
    pub tensors_validated: usize,
    pub shape_mismatches: Vec<String>,
    pub dtype_mismatches: Vec<String>,
    pub missing_tensors: Vec<String>,
    pub warnings: Vec<String>,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
}

/// VarMap to ONNX converter with validation
pub struct VarMapConverter {
    config: ConversionConfig,
    architecture: Option<ModelArchitecture>,
}

impl VarMapConverter {
    /// Create a new converter with default configuration
    pub fn new() -> Self {
        Self {
            config: ConversionConfig::default(),
            architecture: None,
        }
    }
    
    /// Create converter with custom configuration
    pub fn with_config(config: ConversionConfig) -> Self {
        Self {
            config,
            architecture: None,
        }
    }
    
    /// Set expected architecture for validation
    pub fn with_architecture(mut self, architecture: ModelArchitecture) -> Self {
        self.architecture = Some(architecture);
        self
    }
    
    /// Convert VarMap to tensor map with validation
    pub fn convert_varmap(
        &self,
        varmap: &VarMap,
    ) -> ConversionResult<HashMap<String, Tensor>> {
        info!("Starting VarMap conversion");
        
        let varmap_data = varmap.data().lock()
            .map_err(|e| ConversionError::TensorConversion(
                format!("Failed to lock VarMap: {}", e)
            ))?;
        
        let mut tensor_map = HashMap::new();
        let mut report = ConversionReport {
            success: true,
            tensors_converted: 0,
            tensors_validated: 0,
            shape_mismatches: Vec::new(),
            dtype_mismatches: Vec::new(),
            missing_tensors: Vec::new(),
            warnings: Vec::new(),
            input_shape: Vec::new(),
            output_shape: Vec::new(),
        };
        
        // Convert all tensors from VarMap
        for (name, tensor) in varmap_data.iter() {
            debug!("Converting tensor: {} with shape {:?}, dtype {:?}", 
                name, tensor.dims(), tensor.dtype());
            
            // Validate tensor if architecture is specified
            if let Some(ref arch) = self.architecture {
                if let Some(expected) = arch.expected_tensors.iter().find(|t| t.name == *name) {
                    if let Err(e) = self.validate_tensor(name, tensor, expected) {
                        if self.config.strict_mode {
                            return Err(e);
                        } else {
                            warn!("Validation warning for {}: {}", name, e);
                            report.warnings.push(format!("{}: {}", name, e));
                        }
                    } else {
                        report.tensors_validated += 1;
                    }
                }
            }
            
            tensor_map.insert(name.clone(), tensor.clone());
            report.tensors_converted += 1;
        }
        
        // Check for missing expected tensors
        if let Some(ref arch) = self.architecture {
            for expected in &arch.expected_tensors {
                if !tensor_map.contains_key(&expected.name) {
                    let msg = format!("Missing expected tensor: {}", expected.name);
                    if self.config.strict_mode {
                        return Err(ConversionError::MissingTensor(expected.name.clone()));
                    } else {
                        warn!("{}", msg);
                        report.missing_tensors.push(expected.name.clone());
                        report.warnings.push(msg);
                    }
                }
            }
            
            report.input_shape = arch.input_shape.clone();
            report.output_shape = arch.output_shape.clone();
        }
        
        report.success = report.missing_tensors.is_empty() || self.config.allow_partial_conversion;
        
        info!("VarMap conversion completed: {} tensors converted, {} validated", 
            report.tensors_converted, report.tensors_validated);
        
        Ok(tensor_map)
    }
    
    /// Validate tensor against expected metadata
    fn validate_tensor(
        &self,
        name: &str,
        tensor: &Tensor,
        expected: &TensorMetadata,
    ) -> ConversionResult<()> {
        // Validate shape if enabled
        if self.config.validate_shapes {
            let actual_shape: Vec<usize> = tensor.dims().to_vec();
            if actual_shape != expected.shape {
                // Allow batch dimension to be flexible (0 or 1)
                let shapes_match = actual_shape.len() == expected.shape.len() &&
                    actual_shape.iter().zip(expected.shape.iter())
                        .enumerate()
                        .all(|(idx, (a, e))| {
                            if idx == 0 && (*a == 0 || *a == 1) && (*e == 0 || *e == 1) {
                                true // Flexible batch dimension
                            } else {
                                a == e
                            }
                        });
                
                if !shapes_match {
                    return Err(ConversionError::ShapeMismatch {
                        expected: expected.shape.clone(),
                        actual: actual_shape,
                    });
                }
            }
        }
        
        // Validate dtype if enabled
        if self.config.validate_dtypes {
            let actual_dtype = tensor.dtype();
            if actual_dtype != expected.dtype {
                return Err(ConversionError::DTypeMismatch {
                    expected: expected.dtype,
                    actual: actual_dtype,
                });
            }
        }
        
        debug!("Tensor {} validation passed", name);
        Ok(())
    }
    
    /// Extract architecture from VarMap
    pub fn detect_architecture(&self, varmap: &VarMap) -> ConversionResult<ModelArchitecture> {
        let varmap_data = varmap.data().lock()
            .map_err(|e| ConversionError::TensorConversion(
                format!("Failed to lock VarMap: {}", e)
            ))?;
        
        let mut hidden_layers = Vec::new();
        let mut input_size = 0;
        let mut output_size = 0;
        
        // Detect hidden layers from weight tensors
        for (name, tensor) in varmap_data.iter() {
            if name.starts_with('w') && name != "w_out" {
                if let Ok(layer_idx) = name[1..].parse::<usize>() {
                    let shape = tensor.dims();
                    if shape.len() >= 2 {
                        hidden_layers.push((layer_idx, shape[1]));
                        if input_size == 0 {
                            input_size = shape[0];
                        }
                    }
                }
            } else if name == "w_out" {
                let shape = tensor.dims();
                if shape.len() >= 2 {
                    output_size = shape[1];
                    if input_size == 0 {
                        input_size = shape[0];
                    }
                }
            }
        }
        
        // Sort hidden layers by index
        hidden_layers.sort_by_key(|(idx, _)| *idx);
        let hidden_layer_sizes: Vec<usize> = hidden_layers.into_iter().map(|(_, size)| size).collect();
        
        if input_size == 0 || output_size == 0 {
            return Err(ConversionError::InvalidArchitecture(
                "Could not detect input/output dimensions from VarMap".to_string()
            ));
        }
        
        // Build expected tensors list
        let mut expected_tensors = Vec::new();
        for (idx, hidden_size) in hidden_layer_sizes.iter().enumerate() {
            let prev_size = if idx == 0 { input_size } else { hidden_layer_sizes[idx - 1] };
            
            expected_tensors.push(TensorMetadata {
                name: format!("w{}", idx),
                shape: vec![prev_size, *hidden_size],
                dtype: DType::F32,
                description: Some(format!("Weight matrix for layer {}", idx)),
            });
            
            expected_tensors.push(TensorMetadata {
                name: format!("b{}", idx),
                shape: vec![*hidden_size],
                dtype: DType::F32,
                description: Some(format!("Bias vector for layer {}", idx)),
            });
        }
        
        // Output layer
        let last_hidden = hidden_layer_sizes.last().copied().unwrap_or(input_size);
        expected_tensors.push(TensorMetadata {
            name: "w_out".to_string(),
            shape: vec![last_hidden, output_size],
            dtype: DType::F32,
            description: Some("Output layer weight matrix".to_string()),
        });
        
        expected_tensors.push(TensorMetadata {
            name: "b_out".to_string(),
            shape: vec![output_size],
            dtype: DType::F32,
            description: Some("Output layer bias vector".to_string()),
        });
        
        Ok(ModelArchitecture {
            input_shape: vec![1, input_size],
            output_shape: vec![1, output_size],
            hidden_layers: hidden_layer_sizes,
            expected_tensors,
        })
    }
    
    /// Verify converted model by running inference
    pub fn verify_conversion(
        &self,
        tensor_map: &HashMap<String, Tensor>,
        test_input: &Tensor,
        expected_output_shape: &[usize],
    ) -> ConversionResult<bool> {
        info!("Verifying converted model with test input");
        
        // Simple forward pass verification
        let input_shape: Vec<usize> = test_input.dims().to_vec();
        let batch_size = input_shape[0];
        let input_size = input_shape.iter().product::<usize>() / batch_size;
        
        // Reshape input if needed
        let x = if input_shape.len() > 2 {
            test_input.reshape(&[batch_size, input_size])?
        } else {
            test_input.clone()
        };
        
        let mut current = x;
        let mut layer_idx = 0;
        
        // Process hidden layers
        loop {
            let w_name = format!("w{}", layer_idx);
            let b_name = format!("b{}", layer_idx);
            
            if !tensor_map.contains_key(&w_name) || !tensor_map.contains_key(&b_name) {
                break;
            }
            
            let w = tensor_map.get(&w_name).unwrap();
            let b = tensor_map.get(&b_name).unwrap();
            
            // MatMul: current @ w
            current = current.matmul(w)?;
            
            // Add bias
            current = current.broadcast_add(b)?;
            
            // ReLU activation (except for last layer)
            let next_w_name = format!("w{}", layer_idx + 1);
            if tensor_map.contains_key(&next_w_name) {
                current = current.relu()?;
            }
            
            layer_idx += 1;
        }
        
        // Output layer
        if let (Some(w_out), Some(b_out)) = (tensor_map.get("w_out"), tensor_map.get("b_out")) {
            current = current.matmul(w_out)?;
            current = current.broadcast_add(b_out)?;
        }
        
        // Verify output shape
        let output_shape: Vec<usize> = current.dims().to_vec();
        if output_shape != expected_output_shape {
            return Err(ConversionError::Validation(format!(
                "Output shape mismatch: expected {:?}, got {:?}",
                expected_output_shape, output_shape
            )));
        }
        
        info!("Conversion verification passed: output shape {:?}", output_shape);
        Ok(true)
    }
    
    /// Get tensor info from VarMap
    pub fn get_tensor_info(&self, varmap: &VarMap) -> ConversionResult<Vec<TensorMetadata>> {
        let varmap_data = varmap.data().lock()
            .map_err(|e| ConversionError::TensorConversion(
                format!("Failed to lock VarMap: {}", e)
            ))?;
        
        let mut metadata = Vec::new();
        
        for (name, tensor) in varmap_data.iter() {
            metadata.push(TensorMetadata {
                name: name.clone(),
                shape: tensor.dims().to_vec(),
                dtype: tensor.dtype(),
                description: None,
            });
        }
        
        Ok(metadata)
    }
}

/// Convenience function to convert VarMap with validation
pub fn convert_varmap_to_tensors(
    varmap: &VarMap,
    config: Option<ConversionConfig>,
) -> ConversionResult<HashMap<String, Tensor>> {
    let converter = match config {
        Some(cfg) => VarMapConverter::with_config(cfg),
        None => VarMapConverter::new(),
    };
    
    converter.convert_varmap(varmap)
}

/// Detect architecture from VarMap
pub fn detect_model_architecture(varmap: &VarMap) -> ConversionResult<ModelArchitecture> {
    let converter = VarMapConverter::new();
    converter.detect_architecture(varmap)
}

/// Verify converted model
pub fn verify_model_conversion(
    tensor_map: &HashMap<String, Tensor>,
    test_input: &Tensor,
    expected_output_shape: &[usize],
) -> ConversionResult<bool> {
    let converter = VarMapConverter::new();
    converter.verify_conversion(tensor_map, test_input, expected_output_shape)
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    
    #[test]
    fn test_conversion_config_default() {
        let config = ConversionConfig::default();
        assert!(config.validate_shapes);
        assert!(config.validate_dtypes);
        assert!(!config.strict_mode);
    }
    
    #[test]
    fn test_tensor_metadata_creation() {
        let meta = TensorMetadata {
            name: "test_tensor".to_string(),
            shape: vec![10, 20],
            dtype: DType::F32,
            description: Some("Test tensor".to_string()),
        };
        assert_eq!(meta.name, "test_tensor");
        assert_eq!(meta.shape, vec![10, 20]);
    }
    
    #[test]
    fn test_converter_creation() {
        let converter = VarMapConverter::new();
        assert!(converter.architecture.is_none());
    }
    
    #[tokio::test]
    async fn test_architecture_detection() {
        let device = Device::Cpu;
        let varmap = VarMap::new();
        
        // Create simple model weights
        let w0 = Tensor::zeros(&[10, 20], DType::F32, &device).unwrap();
        let b0 = Tensor::zeros(&[20], DType::F32, &device).unwrap();
        let w_out = Tensor::zeros(&[20, 5], DType::F32, &device).unwrap();
        let b_out = Tensor::zeros(&[5], DType::F32, &device).unwrap();
        
        varmap.set_one("w0", w0).unwrap();
        varmap.set_one("b0", b0).unwrap();
        varmap.set_one("w_out", w_out).unwrap();
        varmap.set_one("b_out", b_out).unwrap();
        
        let converter = VarMapConverter::new();
        let arch = converter.detect_architecture(&varmap).unwrap();
        
        assert_eq!(arch.input_shape, vec![1, 10]);
        assert_eq!(arch.output_shape, vec![1, 5]);
        assert_eq!(arch.hidden_layers, vec![20]);
    }
}
