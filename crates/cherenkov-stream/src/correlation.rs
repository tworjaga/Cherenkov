use std::collections::HashMap;

pub struct CorrelationEngine {
    temporal_window_secs: u64,
    spatial_radius_km: f64,
}

impl CorrelationEngine {
    pub fn new(temporal_window_secs: u64, spatial_radius_km: f64) -> Self {
        Self {
            temporal_window_secs,
            spatial_radius_km,
        }
    }
    
    pub fn correlate(&self, events: Vec<CorrelatedEvent>) -> Vec<EventCluster> {
        let mut clusters: Vec<EventCluster> = Vec::new();
        
        for event in events {
            let mut found_cluster = false;
            
            for cluster in &mut clusters {
                if self.is_related(&event, cluster) {
                    cluster.add(event.clone());
                    found_cluster = true;
                    break;
                }
            }
            
            if !found_cluster {
                clusters.push(EventCluster::new(event));
            }
        }
        
        clusters
    }
    
    fn is_related(&self, event: &CorrelatedEvent, cluster: &EventCluster) -> bool {
        let temporal_diff = event.timestamp.timestamp() - cluster.center_time.timestamp();
        if temporal_diff.abs() > self.temporal_window_secs as i64 {
            return false;
        }
        
        let distance = haversine_distance(
            event.location.lat, event.location.lon,
            cluster.center_location.lat, cluster.center_location.lon
        );
        
        distance <= self.spatial_radius_km
    }
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0; // Earth radius in km
    
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    
    let a = (d_lat / 2.0).sin().powi(2) +
            lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    R * c
}

#[derive(Clone)]
pub struct CorrelatedEvent {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub location: GeoPoint,
    pub event_type: String,
}

#[derive(Clone)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

pub struct EventCluster {
    pub events: Vec<CorrelatedEvent>,
    pub center_time: chrono::DateTime<chrono::Utc>,
    pub center_location: GeoPoint,
}

impl EventCluster {
    pub fn new(event: CorrelatedEvent) -> Self {
        Self {
            center_time: event.timestamp,
            center_location: event.location.clone(),
            events: vec![event],
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
        
        self.center_location = GeoPoint {
            lat: avg_lat,
            lon: avg_lon,
        };
    }
}
