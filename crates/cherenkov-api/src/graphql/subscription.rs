use async_graphql::{Subscription, ID};
use futures_util::stream::Stream;
use std::time::Duration;
use std::sync::Arc;
use tokio_stream::StreamExt;

use super::plume_service::{PlumeService, ParticlePosition};

/// GraphQL subscription root with plume service integration
pub struct SubscriptionRoot {
    plume_service: Arc<PlumeService>,
}

impl SubscriptionRoot {
    pub fn new(plume_service: Arc<PlumeService>) -> Self {
        Self { plume_service }
    }
}

impl Default for SubscriptionRoot {
    fn default() -> Self {
        Self {
            plume_service: Arc::new(PlumeService::default()),
        }
    }
}

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
    
    /// Real-time plume particle updates from dispersion calculations
    async fn plume_particles(
        &self,
        simulation_id: ID,
        #[graphql(default = 100)] batch_size: i32,
    ) -> impl Stream<Item = PlumeParticleBatch> {
        let plume_service = self.plume_service.clone();
        let simulation_id_str = simulation_id.to_string();
        let interval = tokio::time::interval(Duration::from_millis(500));
        
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(move |_| {
                // Fetch real particle positions from the plume service
                let particles = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        plume_service.get_particles(&simulation_id_str, batch_size.min(500) as usize).await
                    })
                });
                
                // Convert ParticlePosition to PlumeParticle
                let plume_particles: Vec<PlumeParticle> = particles
                    .into_iter()
                    .map(|p| PlumeParticle {
                        id: p.id,
                        x: (p.longitude - 139.6503) * 111000.0 * (35.6762_f64.to_radians().cos()),
                        y: (p.latitude - 35.6762) * 111000.0,
                        z: p.altitude,
                        concentration: p.concentration,
                        timestamp: chrono::Utc::now(),
                    })
                    .collect();
                
                PlumeParticleBatch {
                    simulation_id: simulation_id.clone(),
                    particles: plume_particles,
                    timestamp: chrono::Utc::now(),
                }
            })
    }
    
    /// Real-time evacuation zone updates from dispersion calculations
    async fn evacuation_zone_updates(
        &self,
        simulation_id: ID,
    ) -> impl Stream<Item = EvacuationZoneUpdate> {
        let plume_service = self.plume_service.clone();
        let simulation_id_str = simulation_id.to_string();
        let interval = tokio::time::interval(Duration::from_secs(5));
        
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(move |_| {
                // Fetch real evacuation zones from the plume service
                let (zones, contours) = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        plume_service.generate_evacuation_zones(&simulation_id_str).await
                            .unwrap_or_else(|| (Vec::new(), Vec::new()))
                    })
                });
                
                EvacuationZoneUpdate {
                    simulation_id: simulation_id.clone(),
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
