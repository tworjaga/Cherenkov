use async_graphql::ID;
use cherenkov_plume::ReleaseParameters;
use cherenkov_plume::dispersion::{GaussianPlumeModel, WeatherConditions, StabilityClass};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tracing::{info, debug};

use super::subscription::{EvacuationZone, DoseContour, ContourPoint};


/// Plume simulation service that manages active simulations
pub struct PlumeService {
    simulations: Arc<Mutex<HashMap<String, SimulationState>>>,
}

#[derive(Clone)]
struct SimulationState {
    release: ReleaseParameters,
    weather: WeatherConditions,
    model: GaussianPlumeModel,
}

impl PlumeService {
    pub fn new() -> Self {
        Self {
            simulations: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Start a new plume simulation
    pub fn start_simulation(&self, simulation_id: String, release: ReleaseParameters) {
        let weather = WeatherConditions::default();
        let model = GaussianPlumeModel::new(weather.clone(), release.clone());
        
        let state = SimulationState {
            release,
            weather,
            model,
        };
        
        self.simulations.lock().unwrap().insert(simulation_id.clone(), state);
        info!("Started plume simulation: {}", simulation_id);

    }
    
    /// Update weather conditions for a simulation
    pub fn update_weather(&self, simulation_id: &str, weather: WeatherConditions) {
        if let Some(state) = self.simulations.lock().unwrap().get_mut(simulation_id) {
            state.weather = weather.clone();
            state.model = GaussianPlumeModel::new(weather, state.release.clone());
            debug!("Updated weather for simulation: {}", simulation_id);
        }
    }
    
    /// Generate evacuation zones with dose contours for a simulation
    pub fn generate_evacuation_zones(
        &self,
        simulation_id: &str,
    ) -> Option<(Vec<EvacuationZone>, Vec<DoseContour>)> {
        let simulations = self.simulations.lock().unwrap();
        let state = simulations.get(simulation_id)?;
        
        let release = &state.release;
        let model = &state.model;
        
        // Define dose thresholds (in microsieverts per hour)
        let thresholds: Vec<(f64, &str, i32, f64)> = vec![
            (0.1, "immediate", 1, 1000.0),    // 0.1 uSv/h - immediate evacuation
            (0.01, "extended", 24, 5000.0),   // 0.01 uSv/h - extended monitoring
            (0.001, "monitoring", 72, 10000.0), // 0.001 uSv/h - monitoring zone
        ];
        
        let mut zones = Vec::new();
        let mut contours = Vec::new();
        
        for (threshold, zone_type, evac_time, radius) in &thresholds {

            // Generate contour for this threshold
            let contour_points = model.contour(*threshold, *radius);

            
            if contour_points.is_empty() {
                continue;
            }
            
            // Calculate center from release point
            let center_lat = release.latitude;
            let center_lon = release.longitude;
            
            // Estimate affected population (simplified)
            let affected_pop = if zone_type == &"immediate" {
                Some(5000)
            } else if zone_type == &"extended" {
                Some(50000)
            } else {
                Some(100000)
            };
            
            let zone = EvacuationZone {
                id: ID::from(format!("{}-{}", simulation_id, zone_type)),
                simulation_id: ID::from(simulation_id),
                zone_type: zone_type.to_string(),
                radius_meters: *radius,
                center_lat,
                center_lon,
                recommended_evacuation_time: *evac_time,
                dose_threshold: *threshold,
                affected_population_estimate: affected_pop,
                timestamp: chrono::Utc::now(),
            };

            
            zones.push(zone);
            
            // Convert contour points to GraphQL format
            let coordinates: Vec<ContourPoint> = contour_points
                .iter()
                .map(|(x, y)| {
                    // Convert local coordinates back to lat/lon
                    // Approximate: 1 degree = 111km
                    let lat = center_lat + y / 111000.0;
                    let lon = center_lon + x / (111000.0 * lat.to_radians().cos());
                    
                    // Calculate dose at this point
                    let dose = model.ground_level_dose_rate(*x, *y);
                    
                    ContourPoint {
                        latitude: lat,
                        longitude: lon,
                        dose_rate: dose,
                    }
                })
                .collect();
            
            let contour = DoseContour {
                level: if zone_type == &"immediate" {
                    1.0
                } else if zone_type == &"extended" {
                    2.0
                } else {
                    3.0
                },
                threshold_sieverts: *threshold / 1_000_000.0, // Convert to sieverts
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
    pub fn get_particles(&self, simulation_id: &str, count: usize) -> Vec<ParticlePosition> {
        let simulations = self.simulations.lock().unwrap();
        
        if let Some(state) = simulations.get(simulation_id) {
            let model = &state.model;
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
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Stop a simulation
    pub fn stop_simulation(&self, simulation_id: &str) {
        self.simulations.lock().unwrap().remove(simulation_id);
        info!("Stopped plume simulation: {}", simulation_id);
    }
}

#[derive(Clone, Debug)]
pub struct ParticlePosition {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub concentration: f64,
}

impl Default for PlumeService {
    fn default() -> Self {
        Self::new()
    }
}
