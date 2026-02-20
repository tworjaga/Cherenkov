use async_graphql::{Subscription, ID};
use futures_util::stream::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;

use cherenkov_plume::{ReleaseParameters, GaussianPlumeModel, StabilityClass};

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
                            id: format!("{}-{}", simulation_id_clone, i),
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
