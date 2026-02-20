#[cfg(test)]
mod tests {
    use super::super::*;
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
    fn test_id_to_isotope() {
        assert_eq!(id_to_isotope(0), "Cs-137");
        assert_eq!(id_to_isotope(1), "Co-60");
        assert_eq!(id_to_isotope(14), "Po-210");
        assert_eq!(id_to_isotope(100), "Unknown-100");
    }

    #[test]
    fn test_extract_top_k() {
        let probs = vec![0.1, 0.5, 0.3, 0.05, 0.05];
        let top3 = extract_top_k(&probs, 3);

        assert_eq!(top3.len(), 3);
        assert_eq!(top3[0].symbol, "Co-60");
        assert!(top3[0].confidence > top3[1].confidence);
        assert!(top3[1].confidence > top3[2].confidence);
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

        let request = BatchRequest { spectra };
        assert_eq!(request.spectra.len(), 2);
    }

    #[tokio::test]
    async fn test_inference_service_creation() {
        let device = Device::Cpu;
        let service = InferenceService::new(device);

        assert!(service.get_model_info().await.is_empty());
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
}
