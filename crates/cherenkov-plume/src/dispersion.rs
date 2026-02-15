use crate::{ReleaseParameters, PlumeSimulation, ConcentrationGrid, ArrivalTime};
use nalgebra::{Vector3, DVector};

pub struct LagrangianDispersion {
    grid: AtmosphericGrid,
    particles: Vec<Particle>,
    dt_seconds: f64,
}

struct AtmosphericGrid {
    u: Vec<Vec<Vec<f64>>>, // Wind u-component
    v: Vec<Vec<Vec<f64>>>, // Wind v-component
    w: Vec<Vec<Vec<f64>>>, // Wind w-component
    lat_min: f64,
    lat_max: f64,
    lon_min: f64,
    lon_max: f64,
    levels: Vec<f64>,
}

struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    mass: f64,
    activity_bq: f64,
    deposited: bool,
}

impl LagrangianDispersion {
    pub fn new(grid: AtmosphericGrid, dt_seconds: f64) -> Self {
        Self {
            grid,
            particles: Vec::new(),
            dt_seconds,
        }
    }
    
    pub fn simulate(&mut self, release: &ReleaseParameters, hours: u32) -> PlumeSimulation {
        let timesteps = (hours as f64 * 3600.0 / self.dt_seconds) as usize;
        
        // Initialize particles at release point
        self.initialize_particles(release);
        
        let mut concentration_grid = ConcentrationGrid {
            lat_min: self.grid.lat_min,
            lat_max: self.grid.lat_max,
            lon_min: self.grid.lon_min,
            lon_max: self.grid.lon_max,
            resolution_m: 1000.0,
            levels: vec![vec![vec![0.0; 100]; 100]; timesteps / 60],
            timestamps: Vec::new(),
        };
        
        for t in 0..timesteps {
            self.advect_particles();
            self.apply_turbulence();
            self.apply_deposition();
            
            if t % 60 == 0 {
                self.deposit_concentration(&mut concentration_grid, t / 60);
            }
        }
        
        let arrival_times = self.calculate_arrival_times();
        let total_dose = self.calculate_integrated_dose();
        
        PlumeSimulation {
            release: release.clone(),
            concentration_grid,
            arrival_times,
            total_integrated_dose: total_dose,
        }
    }
    
    fn initialize_particles(&mut self, release: &ReleaseParameters) {
        let num_particles = 10000;
        let activity_per_particle = release.release_rate_bq_s * release.duration_hours as f64 * 3600.0 / num_particles as f64;
        
        for _ in 0..num_particles {
            self.particles.push(Particle {
                position: Vector3::new(release.longitude, release.latitude, release.altitude_m),
                velocity: Vector3::zeros(),
                mass: 1e-9, // 1 microgram
                activity_bq: activity_per_particle,
                deposited: false,
            });
        }
    }
    
    fn advect_particles(&mut self) {
        for particle in &mut self.particles {
            if particle.deposited {
                continue;
            }
            
            let wind = self.grid.interpolate_wind(particle.position);
            particle.velocity = wind;
            particle.position += particle.velocity * self.dt_seconds;
        }
    }
    
    fn apply_turbulence(&mut self) {
        let diffusion_coefficient = 0.1;
        
        for particle in &mut self.particles {
            if particle.deposited {
                continue;
            }
            
            let random_dispersion = Vector3::new(
                rand::random::<f64>() - 0.5,
                rand::random::<f64>() - 0.5,
                rand::random::<f64>() - 0.5,
            ) * (2.0 * diffusion_coefficient * self.dt_seconds).sqrt();
            
            particle.position += random_dispersion;
        }
    }
    
    fn apply_deposition(&mut self) {
        for particle in &mut self.particles {
            if particle.position.z <= 0.0 {
                particle.deposited = true;
                particle.position.z = 0.0;
            }
        }
    }
    
    fn deposit_concentration(&self, grid: &mut ConcentrationGrid, timestep: usize) {
        // Aggregate particle positions into grid cells
        for particle in &self.particles {
            if particle.deposited {
                continue;
            }
            
            let i = ((particle.position.y - grid.lat_min) / (grid.lat_max - grid.lat_min) * 100.0) as usize;
            let j = ((particle.position.x - grid.lon_min) / (grid.lon_max - grid.lon_min) * 100.0) as usize;
            
            if i < 100 && j < 100 && timestep < grid.levels.len() {
                grid.levels[timestep][i][j] += particle.activity_bq;
            }
        }
    }
    
    fn calculate_arrival_times(&self) -> Vec<ArrivalTime> {
        self.particles.iter()
            .filter(|p| p.deposited)
            .map(|p| ArrivalTime {
                latitude: p.position.y,
                longitude: p.position.x,
                time_seconds: 0.0, // Would track actual time
                concentration: p.activity_bq,
            })
            .collect()
    }
    
    fn calculate_integrated_dose(&self) -> f64 {
        self.particles.iter()
            .filter(|p| p.deposited)
            .map(|p| p.activity_bq)
            .sum()
    }
}

impl AtmosphericGrid {
    fn interpolate_wind(&self, position: Vector3<f64>) -> Vector3<f64> {
        // Trilinear interpolation of wind field
        Vector3::new(5.0, 2.0, 0.1) // Placeholder
    }
}
