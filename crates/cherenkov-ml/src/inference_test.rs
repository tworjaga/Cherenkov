#[cfg(test)]
mod tests {
    use crate::{
        OnnxError, ModelMetadata, BatchRequest, BatchResult, Classification, IsotopePrediction,
        Spectrum, Calibration, id_to_isotope, extract_top_k, InferenceService, OnnxModel
    };

    use candle_core::Device;


    #[test]
    fn test_onnx_error_display() {
        let err = OnnxError::MissingGraph;
        assert_eq!(err.to_string(), "Model has no graph");

        let err = OnnxError::UnsupportedOpset(5, "7-21".to_string());
        assert!(err.to_string().contains("Unsupported ONNX opset version: 5"));

        let err = OnnxError::InputShapeMismatch {
            expected: vec![1, 3, 224, 224],
            actual: vec![1, 3, 256, 256],
        };
        assert!(err.to_string().contains("Input shape mismatch"));
    }

    #[test]
    fn test_model_metadata_creation() {
        let metadata = ModelMetadata {
            input_names: vec!["input".to_string()],
            output_names: vec!["output".to_string()],
            input_shapes: vec![vec![1, 3, 224, 224]],
            output_shapes: vec![vec![1, 1000]],
            opset_version: 13,
            producer_name: "test".to_string(),
            producer_version: "1.0".to_string(),
        };

        assert_eq!(metadata.input_names.len(), 1);
        assert_eq!(metadata.output_names.len(), 1);
        assert_eq!(metadata.opset_version, 13);
    }

    #[test]
    fn test_batch_request_creation() {
        let spectra = vec![
            Spectrum {
                channels: vec![1.0, 2.0, 3.0],
                calibration: Calibration {
                    slope: 1.0,
                    intercept: 0.0,
                    quadratic: 0.0,
                },
            },
            Spectrum {
                channels: vec![4.0, 5.0, 6.0],
                calibration: Calibration {
                    slope: 1.0,
                    intercept: 0.0,
                    quadratic: 0.0,
                },
            },
        ];

        let request = BatchRequest {
            spectra: spectra.clone(),
            request_id: "test-123".to_string(),
        };
        assert_eq!(request.spectra.len(), 2);
        assert_eq!(request.request_id, "test-123");
    }

    #[tokio::test]
    async fn test_inference_service_creation() {
        let service = InferenceService::new(32, 100).expect("Failed to create service");
        let model_info = service.get_model_info().await;
        assert!(model_info.is_empty());
    }

    #[test]
    fn test_onnx_model_load_nonexistent_file() {
        let device = Device::Cpu;
        let result = OnnxModel::load("nonexistent_model.onnx", &device);

        assert!(result.is_err());
        match result {
            Err(OnnxError::FileRead(_)) => (),
            _ => panic!("Expected FileRead error"),
        }
    }

    #[test]
    fn test_classification_creation() {
        let prediction = IsotopePrediction {
            symbol: "Cs-137".to_string(),
            confidence: 0.95,
        };
        
        let classification = Classification {
            isotopes: vec![prediction],
            latency_ms: 10,
        };
        
        assert_eq!(classification.isotopes.len(), 1);
        assert_eq!(classification.isotopes[0].symbol, "Cs-137");
        assert_eq!(classification.latency_ms, 10);
    }

    #[test]
    fn test_batch_result_creation() {
        let results = vec![
            Classification {
                isotopes: vec![IsotopePrediction {
                    symbol: "Cs-137".to_string(),
                    confidence: 0.9,
                }],
                latency_ms: 5,
            },
        ];
        
        let batch_result = BatchResult {
            results,
            batch_latency_ms: 10,
            throughput: 100.0,
        };
        
        assert_eq!(batch_result.results.len(), 1);
        assert_eq!(batch_result.batch_latency_ms, 10);
        assert_eq!(batch_result.throughput, 100.0);
    }
}
