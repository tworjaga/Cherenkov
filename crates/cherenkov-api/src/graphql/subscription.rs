use async_graphql::{Subscription, ID};
use futures_util::stream::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;

#[allow(dead_code)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn reading_updates(&self, sensor_id: ID) -> impl Stream<Item = ReadingUpdate> {
        let interval = tokio::time::interval(Duration::from_secs(1));
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(move |_| ReadingUpdate {
                sensor_id: sensor_id.clone(),
                timestamp: chrono::Utc::now(),
                dose_rate: 0.15,
            })
    }
    
    async fn global_alerts(&self) -> impl Stream<Item = Alert> {
        let interval = tokio::time::interval(Duration::from_secs(5));
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(|_| Alert {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                severity: "info".to_string(),
                message: "System operational".to_string(),
            })
    }
    
    async fn plume_particles(
        &self,
        simulation_id: ID,
        #[graphql(default = 100)] batch_size: i32,
    ) -> impl Stream<Item = PlumeParticleBatch> {
        let interval = tokio::time::interval(Duration::from_millis(100));
        let simulation_id_clone = simulation_id.clone();
        
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(move |_| {
                // Generate simulated particle positions for the plume
                // In production, this would fetch from the running simulation
                let particles = (0..batch_size.min(500))
                    .map(|i| {
                        let angle = (i as f64) * 0.1;
                        let distance = (i as f64) * 2.0;
                        PlumeParticle {
                            id: format!("{}-{}", simulation_id_clone.to_string(), i),
                            x: distance * angle.cos(),
                            y: distance * angle.sin(),
                            z: (i as f64) * 0.5,
                            concentration: 1000.0 / (distance + 1.0),
                            timestamp: chrono::Utc::now(),
                        }
                    })
                    .collect();
                
                PlumeParticleBatch {
                    simulation_id: simulation_id_clone.clone(),
                    particles,
                    timestamp: chrono::Utc::now(),
                }
            })
    }
    
    async fn evacuation_zones(
        &self,
        simulation_id: ID,
    ) -> impl Stream<Item = EvacuationZoneUpdate> {
        let interval = tokio::time::interval(Duration::from_secs(5));
        let simulation_id_clone = simulation_id.clone();
        
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(move |_| {
                let zones = vec![
                    EvacuationZone {
                        id: ID::from(format!("{}-immediate", simulation_id_clone.to_string())),
                        simulation_id: simulation_id_clone.clone(),
                        zone_type: "immediate".to_string(),
                        radius_meters: 1000.0,
                        center_lat: 35.6762,
                        center_lon: 139.6503,
                        recommended_evacuation_time: 1,
                        dose_threshold: 0.1,
                        affected_population_estimate: Some(5000),
                        timestamp: chrono::Utc::now(),
                    },
                    EvacuationZone {
                        id: ID::from(format!("{}-extended", simulation_id_clone.to_string())),
                        simulation_id: simulation_id_clone.clone(),
                        zone_type: "extended".to_string(),
                        radius_meters: 5000.0,
                        center_lat: 35.6762,
                        center_lon: 139.6503,
                        recommended_evacuation_time: 24,
                        dose_threshold: 0.01,
                        affected_population_estimate: Some(50000),
                        timestamp: chrono::Utc::now(),
                    },
                ];
                
                let contours = vec![
                    DoseContour {
                        level: 1.0,
                        threshold_sieverts: 0.1,
                        coordinates: vec![
                            ContourPoint { latitude: 35.6862, longitude: 139.6403, dose_rate: 0.15 },
                            ContourPoint { latitude: 35.6662, longitude: 139.6603, dose_rate: 0.12 },
                        ],
                        label: "Immediate Evacuation".to_string(),
                    },
                    DoseContour {
                        level: 2.0,
                        threshold_sieverts: 0.01,
                        coordinates: vec![
                            ContourPoint { latitude: 35.6962, longitude: 139.6303, dose_rate: 0.02 },
                            ContourPoint { latitude: 35.6562, longitude: 139.6703, dose_rate: 0.018 },
                        ],
                        label: "Extended Monitoring".to_string(),
                    },
                ];
                
                EvacuationZoneUpdate {
                    simulation_id: simulation_id_clone.clone(),
                    zones,
                    contours,
                    timestamp: chrono::Utc::now(),
                }
            })
    }
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct ReadingUpdate {
    pub sensor_id: ID,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub dose_rate: f64,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct Alert {
    pub id: ID,
    pub severity: String,
    pub message: String,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct PlumeParticle {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub concentration: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct PlumeParticleBatch {
    pub simulation_id: ID,
    pub particles: Vec<PlumeParticle>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct DoseContour {
    pub level: f64,
    pub threshold_sieverts: f64,
    pub coordinates: Vec<ContourPoint>,
    pub label: String,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct ContourPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate: f64,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct EvacuationZone {
    pub id: ID,
    pub simulation_id: ID,
    pub zone_type: String,
    pub radius_meters: f64,
    pub center_lat: f64,
    pub center_lon: f64,
    pub recommended_evacuation_time: i32,
    pub dose_threshold: f64,
    pub affected_population_estimate: Option<i32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct EvacuationZoneUpdate {
    pub simulation_id: ID,
    pub zones: Vec<EvacuationZone>,
    pub contours: Vec<DoseContour>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
