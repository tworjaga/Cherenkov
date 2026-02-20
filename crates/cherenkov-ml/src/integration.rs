use crate::{InferenceService, TrainingPipeline, TrainingConfig, TrainingResult, Classification};
use crate::inference::BatchRequest;
use cherenkov_core::events::{CherenkovEvent, Severity, Anomaly as CoreAnomaly};
use cherenkov_stream::anomaly::{AnomalyDetector, Reading};

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// ML-enhanced anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlAnomalyResult {
    pub anomaly_id: String,
    pub timestamp: u64,
    pub location: (f64, f64),
    pub radiation_level: f64,
    pub isotope_classification: Option<Classification>,
    pub anomaly_score: f64,
    pub confidence: f64,
    pub recommended_action: RecommendedAction,
    pub contributing_sensors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendedAction {
    Monitor,
    Investigate,
    Alert,
    Evacuate,
    Critical,
}

/// ML integration service for anomaly detection
pub struct MlAnomalyIntegration {
    inference: Arc<InferenceService>,
    anomaly_detector: Arc<RwLock<AnomalyDetector>>,
    classification_threshold: f64,
    min_confidence: f64,
}

impl MlAnomalyIntegration {
    pub fn new(
        inference: Arc<InferenceService>,
        anomaly_detector: Arc<RwLock<AnomalyDetector>>,
    ) -> Self {
        Self {
            inference,
            anomaly_detector,
            classification_threshold: 0.7,
            min_confidence: 0.6,
        }
    }
    
    pub fn with_thresholds(mut self, classification: f64, confidence: f64) -> Self {
        self.classification_threshold = classification;
        self.min_confidence = confidence;
        self
    }
    
    /// Process sensor reading with ML classification
    pub async fn process_reading(
        &self,
        sensor_id: String,
        spectrum: crate::Spectrum,
        location: (f64, f64),
        radiation_level: f64,
    ) -> anyhow::Result<Option<MlAnomalyResult>> {
        // First check if anomaly detector flags this
        let readings = vec![Reading {
            sensor_id: sensor_id.clone(),
            dose_rate: radiation_level,
            timestamp: chrono::Utc::now(),
        }];
        
        let anomaly = {
            let mut detector = self.anomaly_detector.write().await;
            detector.detect(readings)
        };
        
        if anomaly.is_none() {
            return Ok(None);
        }

        
        // Run ML classification
        let classification = self.inference.classify_spectrum(&spectrum).await?;
        
        // Calculate confidence from top prediction
        let confidence = classification.isotopes.first()
            .map(|p| p.confidence as f64)
            .unwrap_or(0.0);
        
        // Determine action based on radiation level and classification
        let action = self.determine_action(radiation_level, confidence);
        
        let result = MlAnomalyResult {
            anomaly_id: uuid::Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            location,
            radiation_level,
            isotope_classification: Some(classification),
            anomaly_score: radiation_level * confidence,
            confidence,
            recommended_action: action,
            contributing_sensors: vec![sensor_id],
        };
        
        info!("ML anomaly detected: {} with confidence {:.2}", result.anomaly_id, confidence);
        
        Ok(Some(result))
    }
    
    /// Process batch of readings for efficiency
    pub async fn process_batch(
        &self,
        readings: Vec<(String, crate::Spectrum, (f64, f64), f64)>,
    ) -> anyhow::Result<Vec<MlAnomalyResult>> {
        let mut results = Vec::new();
        
        // Filter to anomalous readings
        let anomalous: Vec<_> = {
            let mut detector = self.anomaly_detector.write().await;
            readings.into_iter()
                .filter(|(id, _, _, level)| {
                    let r = vec![Reading {
                        sensor_id: id.clone(),
                        dose_rate: *level,
                        timestamp: chrono::Utc::now(),
                    }];
                    detector.detect(r).is_some()
                })
                .collect()
        };

        
        if anomalous.is_empty() {
            return Ok(results);
        }
        
        // Batch classify spectra
        let spectra: Vec<_> = anomalous.iter()
            .map(|(_, spectrum, _, _)| spectrum.clone())
            .collect();
        
        let batch_request = BatchRequest {
            spectra,
            request_id: uuid::Uuid::new_v4().to_string(),
        };
        
        let batch_result = self.inference.classify_batch(batch_request).await?;
        
        // Combine results
        for ((sensor_id, _, location, radiation), classification) in 
            anomalous.into_iter().zip(batch_result.results.into_iter()) {
            
            let confidence = classification.isotopes.first()
                .map(|p| p.confidence as f64)
                .unwrap_or(0.0);
            
            if confidence >= self.min_confidence {
                let action = self.determine_action(radiation, confidence);
                
                results.push(MlAnomalyResult {
                    anomaly_id: uuid::Uuid::new_v4().to_string(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs(),
                    location,
                    radiation_level: radiation,
                    isotope_classification: Some(classification),
                    anomaly_score: radiation * confidence,
                    confidence,
                    recommended_action: action,
                    contributing_sensors: vec![sensor_id],
                });
            }
        }
        
        Ok(results)
    }
    
    fn determine_action(&self, radiation_level: f64, confidence: f64) -> RecommendedAction {
        match (radiation_level, confidence) {
            (r, c) if r > 1000.0 && c > 0.8 => RecommendedAction::Critical,
            (r, c) if r > 500.0 && c > 0.7 => RecommendedAction::Evacuate,
            (r, c) if r > 100.0 && c > 0.6 => RecommendedAction::Alert,
            (r, c) if r > 50.0 || c > 0.5 => RecommendedAction::Investigate,
            _ => RecommendedAction::Monitor,
        }
    }
    
    /// Convert ML result to system event
    pub fn to_event(&self, result: &MlAnomalyResult) -> CherenkovEvent {
        let severity = match result.recommended_action {
            RecommendedAction::Critical => Severity::Critical,
            RecommendedAction::Evacuate => Severity::Critical,
            RecommendedAction::Alert => Severity::Warning,
            RecommendedAction::Investigate => Severity::Warning,
            RecommendedAction::Monitor => Severity::Info,
        };
        
        let isotope_info = result.isotope_classification.as_ref()
            .map(|c| format!("{:?}", c.isotopes))
            .unwrap_or_default();
        
        CherenkovEvent::AnomalyDetected(CoreAnomaly {
            anomaly_id: result.anomaly_id.clone(),
            sensor_id: uuid::Uuid::new_v4(),
            severity,
            z_score: result.anomaly_score,
            detected_at: chrono::DateTime::from_timestamp(result.timestamp as i64, 0).unwrap_or_else(|| chrono::Utc::now()),
            timestamp: chrono::DateTime::from_timestamp(result.timestamp as i64, 0).unwrap_or_else(|| chrono::Utc::now()),
            dose_rate: result.radiation_level,
            baseline: result.radiation_level * 0.5,
            algorithm: "ml_classifier".to_string(),
        })
    }

}

/// Model management API
pub struct ModelManager {
    inference: Arc<InferenceService>,
    training_config: TrainingConfig,
    active_model_version: Arc<RwLock<String>>,
}

impl ModelManager {
    pub fn new(inference: Arc<InferenceService>, config: TrainingConfig) -> Self {
        Self {
            inference,
            training_config: config,
            active_model_version: Arc::new(RwLock::new("default".to_string())),
        }
    }
    
    /// Start new training job
    pub async fn start_training(&self) -> anyhow::Result<TrainingResult> {
        info!("Starting new training job for model: {}", self.training_config.model_name);
        
        let (mut pipeline, _metrics_rx) = TrainingPipeline::new(self.training_config.clone())?;
        let result = pipeline.train().await?;
        
        info!("Training completed: accuracy={:.2}%", result.test_accuracy * 100.0);
        
        Ok(result)
    }
    
    /// Load specific model version
    pub async fn load_version(&self, version_id: &str, path: &str) -> anyhow::Result<()> {
        info!("Loading model version: {}", version_id);
        
        self.inference.load_model(
            "isotope_classifier",
            path,
            version_id
        ).await?;
        
        *self.active_model_version.write().await = version_id.to_string();
        
        info!("Model version {} loaded successfully", version_id);
        Ok(())
    }
    
    /// Hot-swap to new model version
    pub async fn hot_swap(&self, version_id: &str, path: &str) -> anyhow::Result<()> {
        warn!("Initiating model hot-swap to version: {}", version_id);
        
        self.inference.hot_swap_model(
            "isotope_classifier",
            path,
            version_id
        ).await?;
        
        *self.active_model_version.write().await = version_id.to_string();
        
        info!("Model hot-swapped to version {} successfully", version_id);
        Ok(())
    }
    
    /// Get current model info
    pub async fn get_model_info(&self) -> HashMap<String, String> {
        let mut info = self.inference.get_model_info().await;
        info.insert("active_version".to_string(), self.active_model_version.read().await.clone());
        info.insert("model_name".to_string(), self.training_config.model_name.clone());
        info
    }
    
    /// List available model versions
    pub async fn list_versions(&self) -> anyhow::Result<Vec<crate::training::ModelVersion>> {
        crate::training::list_model_versions(&self.training_config.output_path).await
    }
    
    /// Rollback to previous version
    pub async fn rollback(&self) -> anyhow::Result<()> {
        let versions = self.list_versions().await?;
        
        if versions.len() < 2 {
            return Err(anyhow::anyhow!("No previous version available for rollback"));
        }
        
        // Get second most recent version
        let previous = &versions[1];
        let model_path = format!("{}/model.onnx", self.training_config.output_path);
        
        self.hot_swap(&previous.version_id, &model_path).await?;
        
        info!("Rolled back to version: {}", previous.version_id);
        Ok(())
    }
}

/// Training job scheduler
pub struct TrainingScheduler {
    model_manager: Arc<ModelManager>,
    schedule: Arc<RwLock<TrainingSchedule>>,
}

#[derive(Debug, Clone)]
struct TrainingSchedule {
    enabled: bool,
    interval_hours: u64,
    last_run: Option<u64>,
    auto_deploy: bool,
    min_improvement_threshold: f64,
}

impl TrainingScheduler {
    pub fn new(model_manager: Arc<ModelManager>) -> Self {
        Self {
            model_manager,
            schedule: Arc::new(RwLock::new(TrainingSchedule {
                enabled: false,
                interval_hours: 24,
                last_run: None,
                auto_deploy: false,
                min_improvement_threshold: 0.05,
            })),
        }
    }
    
    pub async fn enable(&self, interval_hours: u64, auto_deploy: bool) {
        let mut schedule = self.schedule.write().await;
        schedule.enabled = true;
        schedule.interval_hours = interval_hours;
        schedule.auto_deploy = auto_deploy;
        info!("Training scheduler enabled: interval={}h, auto_deploy={}", interval_hours, auto_deploy);
    }
    
    pub async fn disable(&self) {
        self.schedule.write().await.enabled = false;
        info!("Training scheduler disabled");
    }
    
    /// Check if training should run
    pub async fn should_run(&self) -> bool {
        let schedule = self.schedule.read().await;
        
        if !schedule.enabled {
            return false;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        match schedule.last_run {
            None => true,
            Some(last) => now - last > schedule.interval_hours * 3600,
        }
    }
    
    /// Run scheduled training
    pub async fn run_scheduled(&self) -> anyhow::Result<()> {
        if !self.should_run().await {
            return Ok(());
        }
        
        info!("Running scheduled training job");
        
        let result = self.model_manager.start_training().await?;
        
        // Update last run time
        self.schedule.write().await.last_run = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
        );
        
        // Auto-deploy if enabled and model improved
        if self.schedule.read().await.auto_deploy {
            let threshold = self.schedule.read().await.min_improvement_threshold;
            
            if result.test_accuracy > threshold {
                let model_path = format!("{}/model.onnx", 
                    self.model_manager.training_config.output_path);
                
                let version_id = format!("v_{}", chrono::Utc::now().timestamp());
                
                self.model_manager.hot_swap(&version_id, &model_path).await?;
                info!("Auto-deployed new model version: {}", version_id);
            }
        }
        
        Ok(())
    }
}
