use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use tracing::info;
use serde::{Serialize, Deserialize};

pub struct CorrelationEngine {
    temporal_window_secs: u64,
    spatial_radius_km: f64,
    event_buffer: Arc<RwLock<Vec<CorrelatedEvent>>>,
    facility_db: Arc<RwLock<HashMap<String, NuclearFacility>>>,
    seismic_threshold: f64,
    radiation_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelatedEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub location: GeoPoint,
    pub event_type: EventType,
    pub magnitude: f64,
    pub confidence: f64,
    pub source: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Seismic,
    RadiationAnomaly,
    IsotopeDetection,
    FacilityAlert,
    PlumeDetection,
    NewsMention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
    pub altitude_m: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearFacility {
    pub id: String,
    pub name: String,
    pub location: GeoPoint,
    pub facility_type: FacilityType,
    pub reactor_type: String,
    pub capacity_mw: u32,
    pub operational_status: OperationalStatus,
    pub radiation_baseline: f64,
    pub seismic_zone: String,
    pub last_inspection: DateTime<Utc>,
    pub anomaly_history: Vec<FacilityAnomaly>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FacilityType {
    PowerReactor,
    ResearchReactor,
    FuelFabrication,
    Reprocessing,
    WasteStorage,
    MedicalIsotope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationalStatus {
    Operational,
    Shutdown,
    Maintenance,
    Decommissioning,
    Emergency,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityAnomaly {
    pub timestamp: DateTime<Utc>,
    pub anomaly_type: String,
    pub severity: f64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCluster {
    pub id: String,
    pub events: Vec<CorrelatedEvent>,
    pub center_time: DateTime<Utc>,
    pub center_location: GeoPoint,
    pub cluster_type: ClusterType,
    pub severity_score: f64,
    pub confidence: f64,
    pub related_facilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterType {
    SeismicRadiation,
    FacilityIncident,
    EnvironmentalRelease,
    NaturalBackground,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicRadiationCorrelation {
    pub seismic_event: CorrelatedEvent,
    pub radiation_events: Vec<CorrelatedEvent>,
    pub time_delay_seconds: i64,
    pub distance_km: f64,
    pub correlation_score: f64,
    pub possible_cause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityStatusInference {
    pub facility_id: String,
    pub inferred_status: OperationalStatus,
    pub confidence: f64,
    pub contributing_factors: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub last_updated: DateTime<Utc>,
}

impl CorrelationEngine {
    pub fn new(
        temporal_window_secs: u64,
        spatial_radius_km: f64,
        seismic_threshold: f64,
        radiation_threshold: f64,
    ) -> Self {
        Self {
            temporal_window_secs,
            spatial_radius_km,
            event_buffer: Arc::new(RwLock::new(Vec::new())),
            facility_db: Arc::new(RwLock::new(HashMap::new())),
            seismic_threshold,
            radiation_threshold,
        }
    }
    
    /// Simplified constructor with defaults for stream processor
    pub fn new_with_db(_db: Arc<cherenkov_db::RadiationDatabase>) -> Self {
        Self {
            temporal_window_secs: 3600, // 1 hour
            spatial_radius_km: 100.0,     // 100 km
            event_buffer: Arc::new(RwLock::new(Vec::new())),
            facility_db: Arc::new(RwLock::new(HashMap::new())),
            seismic_threshold: 4.0,
            radiation_threshold: 2.0,
        }
    }
    
    /// Check for correlated events by sensor_id
    pub async fn check_correlation(&self, sensor_id: &str) -> Vec<super::anomaly::Anomaly> {
        // For now, return empty - this is a simplified implementation
        // In production, this would query the database for related anomalies
        let _ = sensor_id;
        Vec::new()
    }
    
    pub async fn add_event(&self, event: CorrelatedEvent) {
        let mut buffer = self.event_buffer.write().await;
        
        buffer.push(event);
        
        let cutoff = Utc::now() - Duration::seconds(self.temporal_window_secs as i64 * 2);
        buffer.retain(|e| e.timestamp > cutoff);
    }
    
    pub async fn correlate(&self) -> Vec<EventCluster> {
        let buffer = self.event_buffer.read().await;
        let mut clusters: Vec<EventCluster> = Vec::new();
        
        for event in buffer.iter() {
            let mut found_cluster = false;
            
            for cluster in &mut clusters {
                if self.is_related(event, cluster) {
                    cluster.add(event.clone());
                    found_cluster = true;
                    break;
                }
            }
            
            if !found_cluster {
                clusters.push(EventCluster::new(event.clone()));
            }
        }
        
        for cluster in &mut clusters {
            self.classify_cluster(cluster).await;
        }
        
        clusters.sort_by(|a, b| b.severity_score.partial_cmp(&a.severity_score).unwrap());
        
        clusters
    }
    
    fn is_related(&self, event: &CorrelatedEvent, cluster: &EventCluster) -> bool {
        let temporal_diff = (event.timestamp - cluster.center_time).num_seconds().abs();
        if temporal_diff > self.temporal_window_secs as i64 {
            return false;
        }
        
        let distance = haversine_distance(
            event.location.lat, event.location.lon,
            cluster.center_location.lat, cluster.center_location.lon
        );
        
        distance <= self.spatial_radius_km
    }
    
    async fn classify_cluster(&self, cluster: &mut EventCluster) {
        let has_seismic = cluster.events.iter().any(|e| matches!(e.event_type, EventType::Seismic));
        let has_radiation = cluster.events.iter().any(|e| matches!(e.event_type, EventType::RadiationAnomaly));
        let has_facility = cluster.events.iter().any(|e| matches!(e.event_type, EventType::FacilityAlert));
        
        cluster.cluster_type = if has_seismic && has_radiation {
            ClusterType::SeismicRadiation
        } else if has_facility {
            ClusterType::FacilityIncident
        } else if has_radiation {
            ClusterType::EnvironmentalRelease
        } else {
            ClusterType::Unknown
        };
        
        cluster.severity_score = self.calculate_severity(cluster);
        cluster.confidence = self.calculate_confidence(cluster);
        cluster.related_facilities = self.find_nearby_facilities(&cluster.center_location).await;
    }
    
    fn calculate_severity(&self, cluster: &EventCluster) -> f64 {
        let magnitude_score: f64 = cluster.events.iter()
            .map(|e| e.magnitude.min(10.0) / 10.0)
            .sum();
        
        let type_multiplier = match cluster.cluster_type {
            ClusterType::SeismicRadiation => 2.0,
            ClusterType::FacilityIncident => 1.5,
            ClusterType::EnvironmentalRelease => 1.3,
            _ => 1.0,
        };
        
        (magnitude_score * type_multiplier).min(10.0)
    }
    
    fn calculate_confidence(&self, cluster: &EventCluster) -> f64 {
        let event_count_factor = (cluster.events.len() as f64 / 5.0).min(1.0);
        let temporal_consistency = self.calculate_temporal_consistency(cluster);
        let spatial_consistency = self.calculate_spatial_consistency(cluster);
        
        (event_count_factor * 0.4 + temporal_consistency * 0.3 + spatial_consistency * 0.3).min(1.0)
    }
    
    fn calculate_temporal_consistency(&self, cluster: &EventCluster) -> f64 {
        if cluster.events.len() < 2 {
            return 1.0;
        }
        
        let times: Vec<i64> = cluster.events.iter()
            .map(|e| e.timestamp.timestamp())
            .collect();
        
        let min_time = times.iter().min().unwrap();
        let max_time = times.iter().max().unwrap();
        let range = max_time - min_time;
        
        if range == 0 {
            return 1.0;
        }
        
        let avg_interval = range as f64 / (times.len() - 1) as f64;
        let expected_interval = self.temporal_window_secs as f64 / 2.0;
        
        1.0 - (avg_interval / expected_interval).min(1.0)
    }
    
    fn calculate_spatial_consistency(&self, cluster: &EventCluster) -> f64 {
        if cluster.events.len() < 2 {
            return 1.0;
        }
        
        let distances: Vec<f64> = cluster.events.iter()
            .map(|e| haversine_distance(
                e.location.lat, e.location.lon,
                cluster.center_location.lat, cluster.center_location.lon
            ))
            .collect();
        
        let avg_distance = distances.iter().sum::<f64>() / distances.len() as f64;
        
        1.0 - (avg_distance / self.spatial_radius_km).min(1.0)
    }
    
    async fn find_nearby_facilities(&self, location: &GeoPoint) -> Vec<String> {
        let facilities = self.facility_db.read().await;
        
        facilities.iter()
            .filter(|(_, f)| {
                let distance = haversine_distance(
                    location.lat, location.lon,
                    f.location.lat, f.location.lon
                );
                distance < self.spatial_radius_km * 2.0
            })
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    pub async fn correlate_seismic_radiation(&self) -> Vec<SeismicRadiationCorrelation> {
        let buffer = self.event_buffer.read().await;
        
        let seismic_events: Vec<&CorrelatedEvent> = buffer.iter()
            .filter(|e| matches!(e.event_type, EventType::Seismic))
            .filter(|e| e.magnitude >= self.seismic_threshold)
            .collect();
        
        let radiation_events: Vec<&CorrelatedEvent> = buffer.iter()
            .filter(|e| matches!(e.event_type, EventType::RadiationAnomaly))
            .filter(|e| e.magnitude >= self.radiation_threshold)
            .collect();
        
        let mut correlations = Vec::new();
        
        for seismic in seismic_events {
            let related_radiation: Vec<CorrelatedEvent> = radiation_events.iter()
                .filter(|rad| {
                    let time_diff = (rad.timestamp - seismic.timestamp).num_seconds().abs();
                    let distance = haversine_distance(
                        seismic.location.lat, seismic.location.lon,
                        rad.location.lat, rad.location.lon
                    );
                    
                    time_diff < self.temporal_window_secs as i64 && distance < self.spatial_radius_km
                })
                .map(|&e| e.clone())
                .collect();
            
            if !related_radiation.is_empty() {
                let time_delay = related_radiation.first()
                    .map(|r| (r.timestamp - seismic.timestamp).num_seconds())
                    .unwrap_or(0);
                
                let distance = related_radiation.first()
                    .map(|r| haversine_distance(
                        seismic.location.lat, seismic.location.lon,
                        r.location.lat, r.location.lon
                    ))
                    .unwrap_or(0.0);
                
                let correlation_score = self.calculate_correlation_score(seismic, &related_radiation);
                
                let possible_cause = if seismic.magnitude > 6.0 {
                    "Major seismic event may have damaged containment".to_string()
                } else if seismic.magnitude > 4.0 {
                    "Moderate seismic event - possible sensor disruption".to_string()
                } else {
                    "Minor seismic activity - likely natural background correlation".to_string()
                };
                
                correlations.push(SeismicRadiationCorrelation {
                    seismic_event: seismic.clone(),
                    radiation_events: related_radiation,
                    time_delay_seconds: time_delay,
                    distance_km: distance,
                    correlation_score,
                    possible_cause,
                });
            }
        }
        
        correlations.sort_by(|a, b| b.correlation_score.partial_cmp(&a.correlation_score).unwrap());
        correlations
    }
    
    fn calculate_correlation_score(&self, seismic: &CorrelatedEvent, radiation: &[CorrelatedEvent]) -> f64 {
        let magnitude_factor = (seismic.magnitude / 10.0).min(1.0);
        let count_factor = (radiation.len() as f64 / 3.0).min(1.0);
        let temporal_proximity = 1.0 - ((radiation.first().unwrap().timestamp - seismic.timestamp).num_seconds().abs() as f64 / self.temporal_window_secs as f64).min(1.0);
        
        magnitude_factor * 0.4 + count_factor * 0.3 + temporal_proximity * 0.3
    }
    
    pub async fn infer_facility_status(&self, facility_id: &str) -> Option<FacilityStatusInference> {
        let facilities = self.facility_db.read().await;
        let facility = facilities.get(facility_id)?;
        
        let buffer = self.event_buffer.read().await;
        
        let facility_events: Vec<&CorrelatedEvent> = buffer.iter()
            .filter(|e| {
                let distance = haversine_distance(
                    e.location.lat, e.location.lon,
                    facility.location.lat, facility.location.lon
                );
                distance < 50.0
            })
            .collect();
        
        let recent_anomalies = facility.anomaly_history.iter()
            .filter(|a| a.timestamp > Utc::now() - Duration::hours(24))
            .count();
        
        let radiation_spikes = facility_events.iter()
            .filter(|e| matches!(e.event_type, EventType::RadiationAnomaly))
            .filter(|e| e.magnitude > facility.radiation_baseline * 3.0)
            .count();
        
        let seismic_events = facility_events.iter()
            .filter(|e| matches!(e.event_type, EventType::Seismic))
            .filter(|e| e.magnitude > 4.0)
            .count();
        
        let (inferred_status, confidence, factors, actions) = if radiation_spikes > 5 && recent_anomalies > 3 {
            (
                OperationalStatus::Emergency,
                0.9,
                vec![
                    format!("{} radiation spikes detected", radiation_spikes),
                    format!("{} recent anomalies", recent_anomalies),
                ],
                vec![
                    "Activate emergency response team".to_string(),
                    "Notify regulatory authorities".to_string(),
                    "Initiate public alert system".to_string(),
                ]
            )
        } else if radiation_spikes > 2 || recent_anomalies > 2 {
            (
                OperationalStatus::Maintenance,
                0.7,
                vec![
                    format!("{} radiation spikes detected", radiation_spikes),
                    "Elevated anomaly rate".to_string(),
                ],
                vec![
                    "Schedule inspection".to_string(),
                    "Review sensor calibration".to_string(),
                ]
            )
        } else if seismic_events > 0 {
            (
                OperationalStatus::Maintenance,
                0.6,
                vec![
                    format!("{} seismic events detected", seismic_events),
                    "Precautionary measures recommended".to_string(),
                ],
                vec![
                    "Conduct structural assessment".to_string(),
                    "Review seismic safety systems".to_string(),
                ]
            )
        } else {
            (
                facility.operational_status.clone(),
                0.95,
                vec!["Normal operations".to_string()],
                vec!["Continue routine monitoring".to_string()],
            )
        };
        
        Some(FacilityStatusInference {
            facility_id: facility_id.to_string(),
            inferred_status,
            confidence,
            contributing_factors: factors,
            recommended_actions: actions,
            last_updated: Utc::now(),
        })
    }
    
    pub async fn load_facilities(&self, facilities: Vec<NuclearFacility>) {
        let mut db = self.facility_db.write().await;
        for facility in facilities {
            db.insert(facility.id.clone(), facility);
        }
        info!("Loaded {} nuclear facilities", db.len());
    }
    
    pub async fn update_facility_status(&self, facility_id: &str, status: OperationalStatus) {
        let mut db = self.facility_db.write().await;
        if let Some(facility) = db.get_mut(facility_id) {
            facility.operational_status = status;
            facility.last_inspection = Utc::now();
        }
    }
}

impl EventCluster {
    pub fn new(event: CorrelatedEvent) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            center_time: event.timestamp,
            center_location: event.location.clone(),
            events: vec![event],
            cluster_type: ClusterType::Unknown,
            severity_score: 0.0,
            confidence: 1.0,
            related_facilities: Vec::new(),
        }
    }
    
    pub fn add(&mut self, event: CorrelatedEvent) {
        self.events.push(event);
        self.recalculate_center();
    }
    
    fn recalculate_center(&mut self) {
        if self.events.is_empty() {
            return;
        }
        
        let avg_lat = self.events.iter().map(|e| e.location.lat).sum::<f64>() / self.events.len() as f64;
        let avg_lon = self.events.iter().map(|e| e.location.lon).sum::<f64>() / self.events.len() as f64;
        
        let timestamps: Vec<i64> = self.events.iter()
            .map(|e| e.timestamp.timestamp())
            .collect();
        let avg_timestamp = timestamps.iter().sum::<i64>() / timestamps.len() as i64;
        
        self.center_location = GeoPoint {
            lat: avg_lat,
            lon: avg_lon,
            altitude_m: None,
        };
        self.center_time = DateTime::from_timestamp(avg_timestamp, 0).unwrap_or(self.center_time);
    }
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0;
    
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    
    let a = (d_lat / 2.0).sin().powi(2) +
            lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    R * c
}
