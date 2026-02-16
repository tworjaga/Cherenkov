use crate::{ReleaseParameters, PlumeSimulation, ConcentrationGrid, ArrivalTime};
use nalgebra::{Vector3, DVector};
use tracing::{info, debug, warn};
use std::time::Instant;

pub struct LagrangianDispersion {
    grid: AtmosphericGrid,
    particles: Vec<Particle>,
    dt_seconds: f64,
    use_gpu: bool,
    decay_constant: f64,
    dry_deposition_velocity: f64,
    wet_deposition_rate: f64,
}

struct AtmosphericGrid {
    u: Vec<Vec<Vec<f64>>>,
    v: Vec<Vec<Vec<f64>>>,
    w: Vec<Vec<Vec<f64>>>,
    lat_min: f64,
    lat_max: f64,
    lon_min: f64,
    lon_max: f64,
    levels: Vec<f64>,
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f64,
    dy: f64,
    dz: f64,
}

struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    mass: f64,
    activity_bq: f64,
    deposited: bool,
    deposition_time: Option<f64>,
    isotope: String,
    half_life_hours: f64,
}

pub struct DispersionConfig {
    pub num_particles: usize,
    pub dt_seconds: f64,
    pub use_gpu: bool,
    pub enable_decay: bool,
    pub enable_deposition: bool,
    pub grid_resolution_m: f64,
}

impl Default for DispersionConfig {
    fn default() -> Self {
        Self {
            num_particles: 10000,
            dt_seconds: 60.0,
            use_gpu: false,
            enable_decay: true,
            enable_deposition: true,
            grid_resolution_m: 1000.0,
        }
    }
}

impl LagrangianDispersion {
    pub fn new(grid: AtmosphericGrid, config: DispersionConfig) -> Self {
        info!(
            "Initializing LagrangianDispersion with {} particles, GPU: {}",
            config.num_particles, config.use_gpu
        );
        
        Self {
            grid,
            particles: Vec::with_capacity(config.num_particles),
            dt_seconds: config.dt_seconds,
            use_gpu: config.use_gpu,
            decay_constant: 0.0,
            dry_deposition_velocity: 0.01,
            wet_deposition_rate: 0.0,
        }
    }
    
    pub fn simulate(&mut self, release: &ReleaseParameters, hours: u32) -> PlumeSimulation {
        let start = Instant::now();
        let timesteps = (hours as f64 * 3600.0 / self.dt_seconds) as usize;
        
        info!(
            "Starting plume simulation: {} hours, {} timesteps, {} particles",
            hours, timesteps, self.particles.capacity()
        );
        
        self.initialize_particles(release);
        
        let mut concentration_grid = ConcentrationGrid {
            lat_min: self.grid.lat_min,
            lat_max: self.grid.lat_max,
            lon_min: self.grid.lon_min,
            lon_max: self.grid.lon_max,
            resolution_m: self.grid.dx,
            levels: vec![vec![vec![0.0; self.grid.ny]; self.grid.nx]; timesteps / 60 + 1],
            timestamps: Vec::new(),
        };
        
        for t in 0..timesteps {
            let elapsed_hours = t as f64 * self.dt_seconds / 3600.0;
            
            self.advect_particles();
            self.apply_turbulence();
            
            if self.use_gpu {
                self.apply_physics_gpu(elapsed_hours);
            } else {
                self.apply_decay(elapsed_hours);
                self.apply_dry_deposition();
                self.apply_wet_deposition(elapsed_hours);
            }
            
            if t % 60 == 0 {
                let snapshot = t / 60;
                self.deposit_concentration(&mut concentration_grid, snapshot);
                concentration_grid.timestamps.push(elapsed_hours);
            }
        }
        
        let arrival_times = self.calculate_arrival_times();
        let total_dose = self.calculate_integrated_dose();
        let simulation_time = start.elapsed().as_millis() as u32;
        
        info!(
            "Plume simulation complete: {} ms, {} arrival points, total dose: {:.2e} Bq",
            simulation_time, arrival_times.len(), total_dose
        );
        
        PlumeSimulation {
            release: release.clone(),
            concentration_grid,
            arrival_times,
            total_integrated_dose: total_dose,
            simulation_time_ms: simulation_time,
        }
    }
    
    fn initialize_particles(&mut self, release: &ReleaseParameters) {
        let num_particles = self.particles.capacity();
        let total_activity = release.release_rate_bq_s * release.duration_hours as f64 * 3600.0;
        let activity_per_particle = total_activity / num_particles as f64;
        
        let isotope = release.isotope.clone();
        let half_life = get_half_life_hours(&isotope);
        
        for i in 0..num_particles {
            let random_offset = Vector3::new(
                (rand::random::<f64>() - 0.5) * 0.01,
                (rand::random::<f64>() - 0.5) * 0.01,
                (rand::random::<f64>() - 0.5) * 100.0,
            );
            
            self.particles.push(Particle {
                position: Vector3::new(
                    release.longitude + random_offset.x,
                    release.latitude + random_offset.y,
                    release.altitude_m + random_offset.z.max(0.0),
                ),
                velocity: Vector3::zeros(),
                mass: 1e-9,
                activity_bq: activity_per_particle,
                deposited: false,
                deposition_time: None,
                isotope: isotope.clone(),
                half_life_hours: half_life,
            });
        }
        
        debug!("Initialized {} particles for {}", num_particles, isotope);
    }
    
    fn advect_particles(&mut self) {
        for particle in &mut self.particles {
            if particle.deposited {
                continue;
            }
            
            let wind = self.grid.interpolate_wind(particle.position);
            particle.velocity = wind;
            particle.position += particle.velocity * self.dt_seconds;
            
            if particle.position.z < 0.0 {
                particle.position.z = 0.0;
            }
        }
    }
    
    fn apply_turbulence(&mut self) {
        let k_z = 0.5;
        let sigma_w = (2.0 * k_z / self.dt_seconds).sqrt();
        
        for particle in &mut self.particles {
            if particle.deposited {
                continue;
            }
            
            let random_dispersion = Vector3::new(
                (rand::random::<f64>() - 0.5) * sigma_w * 0.1,
                (rand::random::<f64>() - 0.5) * sigma_w * 0.1,
                (rand::random::<f64>() - 0.5) * sigma_w,
            );
            
            particle.position += random_dispersion * self.dt_seconds;
        }
    }
    
    fn apply_decay(&mut self, elapsed_hours: f64) {
        for particle in &mut self.particles {
            if particle.deposited {
                continue;
            }
            
            let decay_factor = (-0.693 * elapsed_hours / particle.half_life_hours).exp();
            particle.activity_bq *= decay_factor;
        }
    }
    
    fn apply_dry_deposition(&mut self) {
        for particle in &mut self.particles {
            if particle.deposited || particle.position.z > 10.0 {
                continue;
            }
            
            let deposition_prob = self.dry_deposition_velocity * self.dt_seconds / 10.0;
            
            if rand::random::<f64>() < deposition_prob {
                particle.deposited = true;
                particle.deposition_time = Some(particle.position.z / self.dry_deposition_velocity);
                particle.position.z = 0.0;
            }
        }
    }
    
    fn apply_wet_deposition(&mut self, elapsed_hours: f64) {
        let rain_rate = self.grid.get_rain_rate(elapsed_hours);
        
        if rain_rate <= 0.0 {
            return;
        }
        
        let scavenging_coeff = 1e-4 * rain_rate;
        
        for particle in &mut self.particles {
            if particle.deposited {
                continue;
            }
            
            let wet_dep_prob = scavenging_coeff * self.dt_seconds;
            
            if rand::random::<f64>() < wet_dep_prob {
                particle.deposited = true;
                particle.deposition_time = Some(elapsed_hours);
                particle.position.z = 0.0;
            }
        }
    }
    
    fn apply_physics_gpu(&mut self, elapsed_hours: f64) {
        warn!("GPU acceleration not yet implemented, using CPU fallback");
        self.apply_decay(elapsed_hours);
        self.apply_dry_deposition();
    }
    
    fn deposit_concentration(&self, grid: &mut ConcentrationGrid, timestep: usize) {
        if timestep >= grid.levels.len() {
            return;
        }
        
        for particle in &self.particles {
            if particle.deposited && particle.activity_bq < 1e-10 {
                continue;
            }
            
            let i = ((particle.position.x - grid.lon_min) / (grid.lon_max - grid.lon_min) * self.grid.nx as f64) as usize;
            let j = ((particle.position.y - grid.lat_min) / (grid.lat_max - grid.lat_min) * self.grid.ny as f64) as usize;
            
            if i < self.grid.nx && j < self.grid.ny {
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
                time_seconds: p.deposition_time.unwrap_or(0.0) * 3600.0,
                concentration: p.activity_bq,
                isotope: p.isotope.clone(),
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
        let x_frac = (position.x - self.lon_min) / (self.lon_max - self.lon_min);
        let y_frac = (position.y - self.lat_min) / (self.lat_max - self.lat_min);
        let z_frac = position.z / 10000.0;
        
        let i = (x_frac * (self.nx - 1) as f64).clamp(0.0, (self.nx - 1) as f64) as usize;
        let j = (y_frac * (self.ny - 1) as f64).clamp(0.0, (self.ny - 1) as f64) as usize;
        let k = (z_frac * (self.nz - 1) as f64).clamp(0.0, (self.nz - 1) as f64) as usize;
        
        Vector3::new(
            self.u[k][j][i],
            self.v[k][j][i],
            self.w[k][j][i],
        )
    }
    
    fn get_rain_rate(&self, _elapsed_hours: f64) -> f64 {
        0.0
    }
}

fn get_half_life_hours(isotope: &str) -> f64 {
    match isotope {
        "I-131" => 192.0,
        "Cs-137" => 10920.0 * 24.0,
        "Cs-134" => 753.6,
        "Xe-133" => 125.0 / 60.0,
        "Kr-85" => 4530.0,
        "Sr-90" => 105120.0,
        "Co-60" => 46240.0,
        "Am-241" => 157800.0 * 24.0,
        "Pu-239" => 87600.0 * 24.0 * 24100.0,
        _ => 1000.0,
    }
}
