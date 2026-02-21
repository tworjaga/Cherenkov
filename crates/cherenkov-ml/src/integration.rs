use crate::{InferenceService, TrainingPipeline, TrainingConfig, TrainingResult, Classification};
use crate::inference::BatchRequest;
use cherenkov_core::events::{CherenkovEvent, Severity, Anomaly as CoreAnomaly};
use cherenkov_stream::anomaly::{AnomalyDetector, Reading};

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use lru::LruCache;
use std::num::NonZeroUsize;


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
    pub anomaly_classes: Vec<AnomalyClass>,
    pub class_confidences: HashMap<String, f64>,
}

/// Multi-class anomaly detection types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AnomalyClass {
    NaturalBackground,
    MedicalIsotope,
    IndustrialSource,
    NuclearMaterial,
    CosmicRay,
    Unknown,
}

impl AnomalyClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnomalyClass::NaturalBackground => "natural_background",
            AnomalyClass::MedicalIsotope => "medical_isotope",
            AnomalyClass::IndustrialSource => "industrial_source",
            AnomalyClass::NuclearMaterial => "nuclear_material",
            AnomalyClass::CosmicRay => "cosmic_ray",
            AnomalyClass::Unknown => "unknown",
        }
    }
    
    pub fn severity(&self) -> Severity {
        match self {
            AnomalyClass::NaturalBackground => Severity::Info,
            AnomalyClass::MedicalIsotope => Severity::Warning,
            AnomalyClass::IndustrialSource => Severity::Warning,
            AnomalyClass::NuclearMaterial => Severity::Critical,
            AnomalyClass::CosmicRay => Severity::Info,
            AnomalyClass::Unknown => Severity::Warning,
        }
    }
}

/// Cached classification result with TTL
#[derive(Debug, Clone)]
struct CachedClassification {
    classification: Classification,
    timestamp: Instant,
    ttl: Duration,
}

impl CachedClassification {
    fn new(classification: Classification, ttl: Duration) -> Self {
        Self {
            classification,
            timestamp: Instant::now(),
            ttl,
        }
    }
    
    fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
}

/// Classification cache with LRU eviction and TTL
pub struct ClassificationCache {
    cache: LruCache<String, CachedClassification>,
    default_ttl: Duration,
    hit_count: u64,
    miss_count: u64,
}

impl ClassificationCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1000).unwrap())),
            default_ttl: ttl,
            hit_count: 0,
            miss_count: 0,
        }
    }
    
    pub fn get(&mut self, key: &str) -> Option<Classification> {
        if let Some(cached) = self.cache.get(key) {
            if !cached.is_expired() {
                self.hit_count += 1;
                return Some(cached.classification.clone());
            }
        }
        self.miss_count += 1;
        None
    }
    
    pub fn put(&mut self, key: String, classification: Classification) {
        let cached = CachedClassification::new(classification, self.default_ttl);
        self.cache.put(key, cached);
    }
    
    pub fn get_stats(&self) -> CacheStats {
        let total = self.hit_count + self.miss_count;
        let hit_rate = if total > 0 {
            self.hit_count as f64 / total as f64
        } else {
            0.0
        };
        
        CacheStats {
            size: self.cache.len(),
            capacity: self.cache.cap().get(),
            hit_count: self.hit_count,
            miss_count: self.miss_count,
            hit_rate,
        }
    }
    
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hit_count = 0;
        self.miss_count = 0;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheStats {
    pub size: usize,
    pub capacity: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
}

/// Confidence thresholds for different anomaly classes
#[derive(Debug, Clone)]
pub struct ConfidenceThresholds {
    pub natural_background: f64,
    pub medical_isotope: f64,
    pub industrial_source: f64,
    pub nuclear_material: f64,
    pub cosmic_ray: f64,
    pub default: f64,
}

impl Default for ConfidenceThresholds {
    fn default() -> Self {
        Self {
            natural_background: 0.5,
            medical_isotope: 0.7,
            industrial_source: 0.75,
            nuclear_material: 0.8,
            cosmic_ray: 0.6,
            default: 0.6,
        }
    }
}

impl ConfidenceThresholds {
    pub fn get(&self, class: &AnomalyClass) -> f64 {
        match class {
            AnomalyClass::NaturalBackground => self.natural_background,
            AnomalyClass::MedicalIsotope => self.medical_isotope,
            AnomalyClass::IndustrialSource => self.industrial_source,
            AnomalyClass::NuclearMaterial => self.nuclear_material,
            AnomalyClass::CosmicRay => self.cosmic_ray,
            AnomalyClass::Unknown => self.default,
        }
    }
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
    cache: Arc<RwLock<ClassificationCache>>,
    thresholds: Arc<RwLock<ConfidenceThresholds>>,
    multi_class_enabled: bool,
}

/// Stream pipeline connector for real-time processing
pub struct StreamPipelineConnector {
    integration: Arc<MlAnomalyIntegration>,
    event_bus: Arc<cherenkov_core::bus::EventBus>,
    processing_stats: Arc<RwLock<ProcessingStats>>,
}

#[derive(Debug, Default)]
struct ProcessingStats {
    pub total_processed: u64,
    pub anomalies_detected: u64,
    pub classifications_cached: u64,
    pub avg_processing_time_ms: f64,
}


impl MlAnomalyIntegration {
    pub fn new(
        inference: Arc<InferenceService>,
        anomaly_detector: Arc<RwLock<AnomalyDetector>>,
    ) -> Self {
        let cache = ClassificationCache::new(1000, Duration::from_secs(300));
        
        Self {
            inference,
            anomaly_detector,
            classification_threshold: 0.7,
            min_confidence: 0.6,
            cache: Arc::new(RwLock::new(cache)),
            thresholds: Arc::new(RwLock::new(ConfidenceThresholds::default())),
            multi_class_enabled: true,
        }
    }
    
    pub fn with_thresholds(mut self, classification: f64, confidence: f64) -> Self {
        self.classification_threshold = classification;
        self.min_confidence = confidence;
        self
    }
    
    pub fn with_cache(mut self, capacity: usize, ttl_secs: u64) -> Self {
        let cache = ClassificationCache::new(capacity, Duration::from_secs(ttl_secs));
        self.cache = Arc::new(RwLock::new(cache));
        self
    }
    
    pub fn with_multi_class(mut self, enabled: bool) -> Self {
        self.multi_class_enabled = enabled;
        self
    }
    
    pub async fn set_thresholds(&self, thresholds: ConfidenceThresholds) {
        *self.thresholds.write().await = thresholds;
    }
    
    pub async fn get_cache_stats(&self) -> CacheStats {
        self.cache.write().await.get_stats()
    }
    
    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
        info!("Classification cache cleared");
    }
    
    /// Generate cache key from spectrum
    fn generate_cache_key(spectrum: &crate::Spectrum) -> String {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        spectrum.energies.hash(&mut hasher);
        spectrum.counts.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// Detect anomaly classes from classification
    fn detect_anomaly_classes(&self, classification: &Classification) -> Vec<AnomalyClass> {
        let mut classes = Vec::new();
        
        for pred in &classification.isotopes {
            let class = match pred.isotope.as_str() {
                "Cs-137" | "Co-60" | "Ir-192" => AnomalyClass::IndustrialSource,
                "Tc-99m" | "I-131" | "F-18" => AnomalyClass::MedicalIsotope,
                "U-235" | "Pu-239" | "Am-241" => AnomalyClass::NuclearMaterial,
                "K-40" | "Th-232" | "U-238" => AnomalyClass::NaturalBackground,
                _ => AnomalyClass::Unknown,
            };
            
            if !classes.contains(&class) {
                classes.push(class);
            }
        }
        
        if classes.is_empty() {
            classes.push(AnomalyClass::Unknown);
        }
        
        classes
    }
    
    /// Get class confidences from classification
    fn get_class_confidences(&self, classification: &Classification) -> HashMap<String, f64> {
        let mut confidences = HashMap::new();
        
        for pred in &classification.isotopes {
            let class = match pred.isotope.as_str() {
                "Cs-137" | "Co-60" | "Ir-192" => "industrial_source",
                "Tc-99m" | "I-131" | "F-18" => "medical_isotope",
                "U-235" | "Pu-239" | "Am-241" => "nuclear_material",
                "K-40" | "Th-232" | "U-238" => "natural_background",
                _ => "unknown",
            };
            
            let entry = confidences.entry(class.to_string()).or_insert(0.0);
            *entry = (*entry).max(pred.confidence as f64);
        }
        
        confidences
    }

    
    /// Process sensor reading with ML classification and caching
    pub async fn process_reading(
        &self,
        sensor_id: String,
        spectrum: crate::Spectrum,
        location: (f64, f64),
        radiation_level: f64,
    ) -> anyhow::Result<Option<MlAnomalyResult>> {
        let start_time = Instant::now();
        
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

        // Check cache first
        let cache_key = Self::generate_cache_key(&spectrum);
        let cached_result = self.cache.write().await.get(&cache_key);
        
        let classification = if let Some(cached) = cached_result {
            info!("Cache hit for sensor {} classification", sensor_id);
            cached
        } else {
            // Run ML classification
            let classification = self.inference.classify_spectrum(&spectrum).await?;
            
            // Store in cache
            self.cache.write().await.put(cache_key, classification.clone());
            classification
        };
        
        // Calculate confidence from top prediction
        let confidence = classification.isotopes.first()
            .map(|p| p.confidence as f64)
            .unwrap_or(0.0);
        
        // Check against thresholds
        let thresholds = self.thresholds.read().await;
        let min_conf = thresholds.default;
        drop(thresholds);
        
        if confidence < min_conf {
            info!("Classification confidence {:.2} below threshold {:.2}", confidence, min_conf);
            return Ok(None);
        }
        
        // Multi-class detection
        let anomaly_classes = if self.multi_class_enabled {
            self.detect_anomaly_classes(&classification)
        } else {
            vec![AnomalyClass::Unknown]
        };
        
        let class_confidences = if self.multi_class_enabled {
            self.get_class_confidences(&classification)
        } else {
            let mut hm = HashMap::new();
            hm.insert("unknown".to_string(), confidence);
            hm
        };
        
        // Determine action based on radiation level, confidence, and classes
        let action = self.determine_action_multi_class(radiation_level, confidence, &anomaly_classes);
        
        let processing_time = start_time.elapsed().as_millis() as f64;
        
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
            anomaly_classes,
            class_confidences,
        };
        
        info!(
            "ML anomaly detected: {} with confidence {:.2}, classes: {:?}, processing_time: {:.2}ms",
            result.anomaly_id, confidence, result.anomaly_classes, processing_time
        );
        
        Ok(Some(result))
    }

    
    /// Process batch of readings for efficiency with caching
    pub async fn process_batch(
        &self,
        readings: Vec<(String, crate::Spectrum, (f64, f64), f64)>,
    ) -> anyhow::Result<Vec<MlAnomalyResult>> {
        let mut results = Vec::new();
        let start_time = Instant::now();
        
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
        
        // Check cache for each spectrum and collect uncached
        let mut uncached: Vec<(String, crate::Spectrum, (f64, f64), f64)> = Vec::new();
        let mut cached_results: Vec<(String, Classification, (f64, f64), f64)> = Vec::new();
        
        for (sensor_id, spectrum, location, radiation) in anomalous {
            let cache_key = Self::generate_cache_key(&spectrum);
            
            if let Some(cached) = self.cache.write().await.get(&cache_key) {
                cached_results.push((sensor_id, cached, location, radiation));
            } else {
                uncached.push((sensor_id, spectrum, location, radiation));
            }
        }
        
        // Batch classify uncached spectra
        if !uncached.is_empty() {
            let spectra: Vec<_> = uncached.iter()
                .map(|(_, spectrum, _, _)| spectrum.clone())
                .collect();
            
            let batch_request = BatchRequest {
                spectra,
                request_id: uuid::Uuid::new_v4().to_string(),
            };
            
            let batch_result = self.inference.classify_batch(batch_request).await?;
            
            // Cache results and combine with cached
            for ((sensor_id, spectrum, location, radiation), classification) in 
                uncached.into_iter().zip(batch_result.results.into_iter()) {
                
                let cache_key = Self::generate_cache_key(&spectrum);
                self.cache.write().await.put(cache_key, classification.clone());
                
                cached_results.push((sensor_id, classification, location, radiation));
            }
        }
        
        // Process all results with threshold checks
        let thresholds = self.thresholds.read().await;
        
        for (sensor_id, classification, location, radiation) in cached_results {
            let confidence = classification.isotopes.first()
                .map(|p| p.confidence as f64)
                .unwrap_or(0.0);
            
            if confidence >= self.min_confidence {
                let anomaly_classes = if self.multi_class_enabled {
                    self.detect_anomaly_classes(&classification)
                } else {
                    vec![AnomalyClass::Unknown]
                };
                
                let class_confidences = if self.multi_class_enabled {
                    self.get_class_confidences(&classification)
                } else {
                    let mut hm = HashMap::new();
                    hm.insert("unknown".to_string(), confidence);
                    hm
                };
                
                let action = self.determine_action_multi_class(radiation, confidence, &anomaly_classes);
                
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
                    anomaly_classes,
                    class_confidences,
                });
            }
        }
        
        let total_time = start_time.elapsed().as_millis() as f64;
        info!(
            "Batch processing complete: {} anomalies from {} readings, time: {:.2}ms",
            results.len(),
            readings.len(),
            total_time
        );
        
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
    
    /// Determine action based on radiation level, confidence, and anomaly classes
    fn determine_action_multi_class(
        &self,
        radiation_level: f64,
        confidence: f64,
        classes: &[AnomalyClass],
    ) -> RecommendedAction {
        // Check for critical classes first
        if classes.contains(&AnomalyClass::NuclearMaterial) && confidence > 0.7 {
            return if radiation_level > 1000.0 {
                RecommendedAction::Critical
            } else {
                RecommendedAction::Evacuate
            };
        }
        
        // Check for industrial sources at high levels
        if classes.contains(&AnomalyClass::IndustrialSource) && radiation_level > 500.0 {
            return RecommendedAction::Alert;
        }
        
        // Default to standard action determination
        self.determine_action(radiation_level, confidence)
    }

    
    /// Convert ML result to system event with multi-class info
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
        
        let class_info = result.anomaly_classes.iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>()
            .join(",");
        
        let algorithm = format!("ml_classifier:{}", class_info);
        
        CherenkovEvent::AnomalyDetected(CoreAnomaly {
            anomaly_id: result.anomaly_id.clone(),
            sensor_id: uuid::Uuid::new_v4(),
            severity,
            z_score: result.anomaly_score,
            detected_at: chrono::DateTime::from_timestamp(result.timestamp as i64, 0).unwrap_or_else(|| chrono::Utc::now()),
            timestamp: chrono::DateTime::from_timestamp(result.timestamp as i64, 0).unwrap_or_else(|| chrono::Utc::now()),
            dose_rate: result.radiation_level,
            baseline: result.radiation_level * 0.5,
            algorithm,
        })
    }
}

impl StreamPipelineConnector {
    pub fn new(
        integration: Arc<MlAnomalyIntegration>,
        event_bus: Arc<cherenkov_core::bus::EventBus>,
    ) -> Self {
        Self {
            integration,
            event_bus,
            processing_stats: Arc::new(RwLock::new(ProcessingStats::default())),
        }
    }
    
    /// Connect to stream and process readings in real-time
    pub async fn connect(&self, readings_rx: tokio::sync::mpsc::Receiver<(String, crate::Spectrum, (f64, f64), f64)>) {
        let mut rx = readings_rx;
        
        while let Some((sensor_id, spectrum, location, radiation_level)) = rx.recv().await {
            let start = Instant::now();
            
            match self.integration.process_reading(
                sensor_id.clone(),
                spectrum,
                location,
                radiation_level,
            ).await {
                Ok(Some(result)) => {
                    // Publish event to bus
                    let event = self.integration.to_event(&result);
                    if let Err(e) = self.event_bus.publish(event).await {
                        error!("Failed to publish anomaly event: {}", e);
                    }
                    
                    // Update stats
                    let mut stats = self.processing_stats.write().await;
                    stats.total_processed += 1;
                    stats.anomalies_detected += 1;
                    let elapsed = start.elapsed().as_millis() as f64;
                    stats.avg_processing_time_ms = (stats.avg_processing_time_ms * (stats.total_processed - 1) as f64 + elapsed) 
                        / stats.total_processed as f64;
                }
                Ok(None) => {
                    let mut stats = self.processing_stats.write().await;
                    stats.total_processed += 1;
                }
                Err(e) => {
                    error!("Error processing reading from {}: {}", sensor_id, e);
                }
            }
        }
        
        info!("Stream pipeline connector shutting down");
    }
    
    /// Get processing statistics
    pub async fn get_stats(&self) -> ProcessingStats {
        let stats = self.processing_stats.read().await;
        ProcessingStats {
            total_processed: stats.total_processed,
            anomalies_detected: stats.anomalies_detected,
            classifications_cached: stats.classifications_cached,
            avg_processing_time_ms: stats.avg_processing_time_ms,
        }
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
