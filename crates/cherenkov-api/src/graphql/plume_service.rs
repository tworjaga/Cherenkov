use async_graphql::ID;
use cherenkov_plume::ReleaseParameters;
use cherenkov_plume::dispersion::{GaussianPlumeModel, WeatherConditions, StabilityClass};
use cherenkov_plume::weather::{WeatherDataProvider, CompositeWeatherProvider};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, debug, warn};

use super::subscription::{EvacuationZone, DoseContour, ContourPoint, PlumeParticle};


/// Plume simulation service that manages active simulations
pub struct PlumeService {
    simulations: Arc<RwLock<HashMap<String, SimulationState>>>,
    weather_provider: Arc<dyn WeatherDataProvider>,
}

struct SimulationState {
    release: ReleaseParameters,
    weather: WeatherConditions,
    particle_positions: Vec<ParticlePosition>,
    last_update: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug)]
pub struct ParticlePosition {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub concentration: f64,
    pub deposited: bool,
}

impl PlumeService {
    pub fn new(weather_provider: Arc<dyn WeatherDataProvider>) -> Self {
        Self {
            simulations: Arc::new(RwLock::new(HashMap::new())),
            weather_provider,
        }
    }
    
    /// Start a new plume simulation
    pub async fn start_simulation(&self, simulation_id: String, release: ReleaseParameters) {
        let weather = WeatherConditions::default();
        
        let state = SimulationState {
            release: release.clone(),
            weather,
            particle_positions: Vec::new(),
            last_update: chrono::Utc::now(),
        };
        
        self.simulations.write().await.insert(simulation_id.clone(), state);
        info!("Started plume simulation: {}", simulation_id);
    }

    
    /// Update weather conditions for a simulation
    pub async fn update_weather(&self, simulation_id: &str, weather: WeatherConditions) {
        let mut simulations = self.simulations.write().await;
        if let Some(state) = simulations.get_mut(simulation_id) {
            state.weather = weather;
            debug!("Updated weather for simulation: {}", simulation_id);
        }
    }

    
    /// Generate evacuation zones with dose contours using Gaussian plume model
    pub async fn generate_evacuation_zones(
        &self,
        simulation_id: &str,
    ) -> Option<(Vec<EvacuationZone>, Vec<DoseContour>)> {
        let simulations = self.simulations.read().await;
        let state = simulations.get(simulation_id)?;
        let release = &state.release;
        let weather = &state.weather;
        
        // Create Gaussian plume model with correct API
        let model = GaussianPlumeModel::new(
            weather.clone(),
            release.clone(),
        );
        
        // Generate evacuation zones based on dose thresholds
        let mut zones = Vec::new();
        let mut contours = Vec::new();
        
        // Define dose thresholds for different evacuation zones (in Sieverts)
        let thresholds = vec![
            (0.0001, "immediate", "Immediate evacuation required", 3600), // 0.1 mSv/h
            (0.00001, "extended", "Extended evacuation zone", 86400),     // 0.01 mSv/h
            (0.000001, "monitoring", "Monitoring zone", 604800),            // 0.001 mSv/h
        ];
        
        for (threshold, zone_type, description, evac_time) in thresholds {
            // Find the distance where concentration drops below threshold
            let mut max_distance = 0.0;
            let mut contour_points = Vec::new();
            
            for distance_m in (100..10000).step_by(100) {
                let distance = distance_m as f64;
                let angle = state.weather.wind_direction_deg.to_radians();
                
                // Calculate position downwind
                let x = distance * angle.cos();
                let y = distance * angle.sin();
                
                // Get concentration at ground level
                let concentration = model.ground_level_concentration(x, y);
                
                // Convert concentration to dose rate (simplified conversion)
                let dose_rate = concentration * 1.0e6; // Convert to appropriate units
                
                if dose_rate > threshold {
                    max_distance = distance;
                    
                    // Convert to lat/lon
                    let lat = release.latitude + y / 111000.0;
                    let lon = release.longitude + x / (111000.0 * release.latitude.to_radians().cos());
                    
                    contour_points.push(ContourPoint {
                        latitude: lat,
                        longitude: lon,
                        dose_rate,
                    });
                } else {
                    break;
                }
            }
            
            if max_distance > 0.0 {
                let zone = EvacuationZone {
                    id: ID::from(format!("{}-{}", simulation_id, zone_type)),
                    simulation_id: ID::from(simulation_id.to_string()),
                    zone_type: zone_type.to_string(),
                    radius_meters: max_distance,
                    center_lat: release.latitude,
                    center_lon: release.longitude,
                    recommended_evacuation_time: evac_time,
                    dose_threshold: threshold,
                    affected_population_estimate: None,
                    timestamp: chrono::Utc::now(),
                };
                
                zones.push(zone);
                
                let contour = DoseContour {
                    level: threshold,
                    threshold_sieverts: threshold,
                    coordinates: contour_points,
                    label: description.to_string(),
                };
                
                contours.push(contour);
            }
        }
        
        Some((zones, contours))
    }

    /// Get particle positions for visualization
    pub async fn get_particles(&self, simulation_id: &str, count: usize) -> Vec<ParticlePosition> {
        let simulations = self.simulations.read().await;
        let state = match simulations.get(simulation_id) {
            Some(s) => s,
            None => return Vec::new(),
        };
        
        let release = &state.release;
        let weather = &state.weather;
        
        // Create Gaussian plume model with correct API
        let model = GaussianPlumeModel::new(
            weather.clone(),
            release.clone(),
        );
        
        // Generate particles along the plume centerline
        (0..count)
            .map(|i| {
                let distance = (i as f64) * 100.0; // 100m spacing
                let angle = state.weather.wind_direction_deg.to_radians();
                
                // Calculate position
                let x = distance * angle.cos();
                let y = distance * angle.sin();
                
                // Get concentration at this point
                let concentration = model.ground_level_concentration(x, y);
                
                // Convert to lat/lon
                let lat = release.latitude + y / 111000.0;
                let lon = release.longitude + x / (111000.0 * release.latitude.to_radians().cos());
                
                ParticlePosition {
                    id: format!("{}-{}", simulation_id, i),
                    latitude: lat,
                    longitude: lon,
                    altitude: 50.0 + (i as f64) * 2.0, // Rising plume
                    concentration,
                    deposited: false,
                }
            })
            .collect()
    }

    
    /// Stop a simulation
    pub async fn stop_simulation(&self, simulation_id: &str) {
        self.simulations.write().await.remove(simulation_id);
        info!("Stopped plume simulation: {}", simulation_id);
    }
    
    /// Update weather from provider
    pub async fn refresh_weather(&self, simulation_id: &str) -> anyhow::Result<()> {
        let mut simulations = self.simulations.write().await;
        let state = simulations.get_mut(simulation_id)
            .ok_or_else(|| anyhow::anyhow!("Simulation not found: {}", simulation_id))?;
        
        let local_weather = self.weather_provider
            .fetch_weather(state.release.latitude, state.release.longitude, state.release.altitude_m)
            .await?;
        
        let weather = WeatherConditions::from_local_weather(&local_weather);
        state.weather = weather;
        
        info!("Refreshed weather for simulation: {}", simulation_id);
        Ok(())
    }
}

impl Default for PlumeService {
    fn default() -> Self {
        // Create a dummy weather provider for default implementation
        // In production, this should be properly initialized with a real provider
        let weather_provider = Arc::new(CompositeWeatherProvider::new());
        Self::new(weather_provider)
    }
}
