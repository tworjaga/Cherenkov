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
        let model = GaussianPlumeModel::new(state.weather.clone(), release.clone());
        
        // Define dose thresholds (in microsieverts per hour)
        let thresholds: Vec<(f64, &str, i32, f64)> = vec![
            (0.1, "immediate", 1, 1000.0),    // 0.1 uSv/h - immediate evacuation
            (0.01, "extended", 24, 5000.0),   // 0.01 uSv/h - extended monitoring
            (0.001, "monitoring", 72, 10000.0), // 0.001 uSv/h - monitoring zone
        ];
        
        let mut zones = Vec::new();
        let mut contours = Vec::new();
        
        for (threshold, zone_type, evac_time, max_radius) in &thresholds {
            // Generate contour using Gaussian model
            let contour_points = model.contour(*threshold, *max_radius);
            
            if contour_points.is_empty() {
                continue;
            }
            
            let center_lat = release.latitude;
            let center_lon = release.longitude;
            
            // Estimate affected population based on zone type
            let affected_pop = match *zone_type {
                "immediate" => Some(5000),
                "extended" => Some(50000),
                _ => Some(100000),
            };
            
            let zone = EvacuationZone {
                id: ID::from(format!("{}-{}", simulation_id, zone_type)),
                simulation_id: ID::from(simulation_id),
                zone_type: zone_type.to_string(),
                radius_meters: *max_radius,
                center_lat,
                center_lon,
                recommended_evacuation_time: *evac_time,
                dose_threshold: *threshold,
                affected_population_estimate: affected_pop,
                timestamp: chrono::Utc::now(),
            };
            
            zones.push(zone);
            
            // Convert contour points to lat/lon and calculate dose rates
            let coordinates: Vec<ContourPoint> = contour_points
                .iter()
                .map(|(x, y)| {
                    let lat = center_lat + y / 111000.0;
                    let lon = center_lon + x / (111000.0 * center_lat.to_radians().cos());
                    let dose = model.ground_level_dose_rate(*x, *y);
                    
                    ContourPoint {
                        latitude: lat,
                        longitude: lon,
                        dose_rate: dose,
                    }
                })
                .collect();
            
            let contour = DoseContour {
                level: match *zone_type {
                    "immediate" => 1.0,
                    "extended" => 2.0,
                    _ => 3.0,
                },
                threshold_sieverts: *threshold / 1_000_000.0,
                coordinates,
                label: format!("{} Zone", zone_type.to_uppercase()),
            };
            
            contours.push(contour);
        }
        
        debug!(
            "Generated {} zones and {} contours for simulation: {}",
            zones.len(),
            contours.len(),
            simulation_id
        );
        
        Some((zones, contours))
    }

    
    /// Get particle positions for visualization
    pub async fn get_particles(&self, simulation_id: &str, count: usize) -> Vec<ParticlePosition> {
        let simulations = self.simulations.read().await;
        let state = match simulations.get(simulation_id) {
            Some(s) => s,
            None => return Vec::new(),
        };
        
        let model = GaussianPlumeModel::new(state.weather.clone(), state.release.clone());
        let release = &state.release;
        
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
