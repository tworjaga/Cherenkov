use std::path::{Path, PathBuf};
use std::fs;
use candle_nn::VarMap;
use prost::Message;
use tracing::{info, debug, warn};
use thiserror::Error;
use chrono::Utc;
use serde::{Serialize, Deserialize};

/// Errors that can occur during ONNX export
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Candle error: {0}")]
    Candle(#[from] candle_core::Error),
    
    #[error("Invalid model architecture: {0}")]
    InvalidArchitecture(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;

/// Dynamic axis configuration for ONNX export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicAxis {
    pub name: String,
    pub dimensions: Vec<Option<i64>>,
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub opset_version: i64,
    pub input_names: Vec<String>,
    pub output_names: Vec<String>,
    pub dynamic_axes: Option<Vec<DynamicAxis>>,
    pub metadata: Option<ModelMetadata>,
    pub optimize: bool,
    pub validate: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            opset_version: 17,
            input_names: vec!["input".to_string()],
            output_names: vec!["output".to_string()],
            dynamic_axes: None,
            metadata: None,
            optimize: true,
            validate: true,
        }
    }
}

/// Model metadata for ONNX export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub domain: String,
    pub model_version: i64,
    pub doc_string: String,
}

impl Default for ModelMetadata {
    fn default() -> Self {
        Self {
            name: "CherenkovRadiationClassifier".to_string(),
            version: "1.0.0".to_string(),
            description: "Neural network for radiation isotope classification".to_string(),
            author: "Cherenkov ML Team".to_string(),
            domain: "radiation-spectroscopy".to_string(),
            model_version: 1,
            doc_string: "Trained model for classifying radiation spectra into isotope categories".to_string(),
        }
    }
}

/// ONNX model exporter
pub struct OnnxExporter {
    config: ExportConfig,
}

impl OnnxExporter {
    /// Create a new exporter with default configuration
    pub fn new() -> Self {
        Self {
            config: ExportConfig::default(),
        }
    }
    
    /// Create exporter with custom configuration
    pub fn with_config(config: ExportConfig) -> Self {
        Self { config }
    }
    
    /// Export a Candle model to ONNX format
    pub async fn export_model(
        &self,
        varmap: &VarMap,
        input_shape: &[usize],
        output_shape: &[usize],
        output_path: &Path,
    ) -> ExportResult<ExportReport> {
        info!("Starting ONNX export to {:?}", output_path);
        
        // Validate input parameters
        self.validate_shapes(input_shape, output_shape)?;
        
        // Create output directory if needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Build ONNX model using candle_onnx types
        let model_proto = self.build_onnx_model(varmap, input_shape, output_shape).await?;
        
        // Serialize to file
        let mut buffer = Vec::new();
        model_proto.encode(&mut buffer)
            .map_err(|e| ExportError::Serialization(format!("Failed to encode ONNX model: {}", e)))?;
        
        fs::write(output_path, &buffer)?;
        
        // Validate if requested
        if self.config.validate {
            self.validate_export(output_path, input_shape, output_shape).await?;
        }
        
        let report = ExportReport {
            output_path: output_path.to_path_buf(),
            input_shape: input_shape.to_vec(),
            output_shape: output_shape.to_vec(),
            opset_version: self.config.opset_version,
            file_size_bytes: buffer.len(),
            exported_at: Utc::now().to_rfc3339(),
        };
        
        info!("ONNX export completed successfully: {} bytes", buffer.len());
        Ok(report)
    }
    
    /// Build ONNX model protobuf using candle_onnx types
    async fn build_onnx_model(
        &self,
        varmap: &VarMap,
        input_shape: &[usize],
        output_shape: &[usize],
    ) -> ExportResult<candle_onnx::onnx::ModelProto> {
        use candle_onnx::onnx::*;
        
        let mut model = ModelProto::default();
        model.ir_version = 8;
        model.opset_import.push(OperatorSetIdProto {
            domain: "".to_string(),
            version: self.config.opset_version,
            ..Default::default()
        });
        
        // Build graph
        let mut graph = GraphProto::default();
        graph.name = self.config.metadata.as_ref()
            .map(|m| m.name.clone())
            .unwrap_or_else(|| "model".to_string());
        
        // Add input
        let input_tensor = ValueInfoProto {
            name: self.config.input_names[0].clone(),
            r#type: Some(TypeProto {
                denotation: "".to_string(),
                value: Some(type_proto::Value::TensorType(type_proto::Tensor {
                    elem_type: tensor_proto::DataType::Float as i32,
                    shape: Some(TensorShapeProto {
                        dim: input_shape.iter()
                            .map(|&d| tensor_shape_proto::Dimension {
                                value: Some(tensor_shape_proto::dimension::Value::DimValue(d as i64)),
                                ..Default::default()
                            })
                            .collect(),
                    }),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        };
        graph.input.push(input_tensor);
        
        // Add output
        let output_tensor = ValueInfoProto {
            name: self.config.output_names[0].clone(),
            r#type: Some(TypeProto {
                denotation: "".to_string(),
                value: Some(type_proto::Value::TensorType(type_proto::Tensor {
                    elem_type: tensor_proto::DataType::Float as i32,
                    shape: Some(TensorShapeProto {
                        dim: output_shape.iter()
                            .map(|&d| tensor_shape_proto::Dimension {
                                value: Some(tensor_shape_proto::dimension::Value::DimValue(d as i64)),
                                ..Default::default()
                            })
                            .collect(),
                    }),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        };
        graph.output.push(output_tensor);
        
        // Extract weights and build nodes
        self.build_neural_network_nodes(varmap, &mut graph).await?;
        
        // Add metadata
        if let Some(metadata) = &self.config.metadata {
            model.metadata_props.push(StringStringEntryProto {
                key: "model_name".to_string(),
                value: metadata.name.clone(),
                ..Default::default()
            });
            model.metadata_props.push(StringStringEntryProto {
                key: "model_version".to_string(),
                value: metadata.version.clone(),
                ..Default::default()
            });
            model.metadata_props.push(StringStringEntryProto {
                key: "description".to_string(),
                value: metadata.description.clone(),
                ..Default::default()
            });
        }
        
        model.graph = Some(graph);
        Ok(model)
    }
    
    /// Build neural network nodes from VarMap
    async fn build_neural_network_nodes(
        &self,
        varmap: &VarMap,
        graph: &mut candle_onnx::onnx::GraphProto,
    ) -> ExportResult<()> {
        let varmap_data = varmap.data().lock().unwrap();
        
        for (name, tensor) in varmap_data.iter() {
            debug!("Processing variable: {}", name);
            
            let tensor_proto = self.candle_tensor_to_onnx(tensor, name)?;
            graph.initializer.push(tensor_proto);
        }
        
        // Add a simple MatMul + Add node as placeholder
        let node = candle_onnx::onnx::NodeProto {
            op_type: "MatMul".to_string(),
            input: vec![self.config.input_names[0].clone(), "weight".to_string()],
            output: vec!["matmul_out".to_string()],
            ..Default::default()
        };
        graph.node.push(node);
        
        let bias_node = candle_onnx::onnx::NodeProto {
            op_type: "Add".to_string(),
            input: vec!["matmul_out".to_string(), "bias".to_string()],
            output: vec![self.config.output_names[0].clone()],
            ..Default::default()
        };
        graph.node.push(bias_node);
        
        Ok(())
    }
    
    /// Convert Candle tensor to ONNX tensor proto
    fn candle_tensor_to_onnx(
        &self,
        tensor: &candle_core::Tensor,
        name: &str,
    ) -> ExportResult<candle_onnx::onnx::TensorProto> {
        use candle_onnx::onnx::*;
        
        let shape: Vec<i64> = tensor.dims().iter().map(|&d| d as i64).collect();
        let data: Vec<f32> = tensor.to_vec1()
            .map_err(ExportError::Candle)?;
        
        Ok(TensorProto {
            name: name.to_string(),
            data_type: tensor_proto::DataType::Float as i32,
            dims: shape,
            float_data: data,
            ..Default::default()
        })
    }
    
    /// Validate input/output shapes
    fn validate_shapes(&self, input: &[usize], output: &[usize]) -> ExportResult<()> {
        if input.is_empty() {
            return Err(ExportError::Validation("Input shape cannot be empty".to_string()));
        }
        if output.is_empty() {
            return Err(ExportError::Validation("Output shape cannot be empty".to_string()));
        }
        if input[0] != 1 && input[0] != 0 {
            warn!("Unusual batch size detected: {}", input[0]);
        }
        Ok(())
    }
    
    /// Validate exported model
    async fn validate_export(
        &self,
        model_path: &Path,
        _input_shape: &[usize],
        _output_shape: &[usize],
    ) -> ExportResult<()> {
        debug!("Validating exported ONNX model");
        
        let model_bytes = fs::read(model_path)?;
        let model: candle_onnx::onnx::ModelProto = Message::decode(&model_bytes[..])
            .map_err(|e| ExportError::Serialization(format!("Failed to decode model: {}", e)))?;
        
        if model.ir_version < 7 {
            return Err(ExportError::Validation(
                format!("IR version {} is too old", model.ir_version)
            ));
        }
        
        if let Some(graph) = model.graph {
            if graph.input.is_empty() {
                return Err(ExportError::Validation("Model has no inputs".to_string()));
            }
            if graph.output.is_empty() {
                return Err(ExportError::Validation("Model has no outputs".to_string()));
            }
        } else {
            return Err(ExportError::Validation("Model has no graph".to_string()));
        }
        
        info!("ONNX model validation passed");
        Ok(())
    }
    
    /// Optimize the exported model
    pub async fn optimize_model(&self, model_path: &Path) -> ExportResult<PathBuf> {
        info!("Optimizing ONNX model at {:?}", model_path);
        
        let model_bytes = fs::read(model_path)?;
        let mut model: candle_onnx::onnx::ModelProto = Message::decode(&model_bytes[..])
            .map_err(|e| ExportError::Serialization(format!("Failed to decode: {}", e)))?;
        
        if let Some(ref mut graph) = model.graph {
            let used_tensors: std::collections::HashSet<String> = graph.node
                .iter()
                .flat_map(|n| n.input.iter().cloned())
                .collect();
            
            graph.initializer.retain(|t| used_tensors.contains(&t.name));
            self.fuse_operations(graph)?;
        }
        
        let optimized_path = model_path.with_extension("optimized.onnx");
        let mut buffer = Vec::new();
        model.encode(&mut buffer)
            .map_err(|e| ExportError::Serialization(format!("Failed to encode: {}", e)))?;
        fs::write(&optimized_path, &buffer)?;
        
        info!("Model optimized and saved to {:?}", optimized_path);
        Ok(optimized_path)
    }
    
    /// Fuse operations for optimization
    fn fuse_operations(&self, graph: &mut candle_onnx::onnx::GraphProto) -> ExportResult<()> {
        let mut i = 0;
        while i < graph.node.len().saturating_sub(1) {
            if graph.node[i].op_type == "MatMul" && graph.node[i + 1].op_type == "Add" {
                if graph.node[i].output[0] == graph.node[i + 1].input[0] {
                    let mut gemm = graph.node[i].clone();
                    gemm.op_type = "Gemm".to_string();
                    gemm.input.push(graph.node[i + 1].input[1].clone());
                    gemm.output = graph.node[i + 1].output.clone();
                    
                    graph.node[i] = gemm;
                    graph.node.remove(i + 1);
                    debug!("Fused MatMul+Add into Gemm");
                    continue;
                }
            }
            i += 1;
        }
        Ok(())
    }
}

/// Export report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportReport {
    pub output_path: PathBuf,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub opset_version: i64,
    pub file_size_bytes: usize,
    pub exported_at: String,
}

/// Convenience function to export a model
pub async fn export_model_to_onnx(
    varmap: &VarMap,
    input_shape: &[usize],
    output_shape: &[usize],
    output_path: &Path,
) -> ExportResult<ExportReport> {
    let exporter = OnnxExporter::new();
    exporter.export_model(varmap, input_shape, output_shape, output_path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_export_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.opset_version, 17);
        assert!(config.optimize);
        assert!(config.validate);
    }
    
    #[tokio::test]
    async fn test_exporter_creation() {
        let exporter = OnnxExporter::new();
        assert_eq!(exporter.config.opset_version, 17);
    }
    
    #[tokio::test]
    async fn test_shape_validation() {
        let exporter = OnnxExporter::new();
        assert!(exporter.validate_shapes(&[1, 10], &[1, 5]).is_ok());
        assert!(exporter.validate_shapes(&[32, 1024], &[32, 10]).is_ok());
        assert!(exporter.validate_shapes(&[], &[1, 5]).is_err());
        assert!(exporter.validate_shapes(&[1, 10], &[]).is_err());
    }
}
