use crate::{ReleaseParameters, PlumeSimulation, ConcentrationGrid, ArrivalTime};
use nalgebra::Vector3;
use tracing::{info, debug, warn};
use std::time::Instant;
use candle_core::{Device, Tensor, DType};


pub struct LagrangianDispersion {
    grid: AtmosphericGrid,
    particles: Vec<Particle>,
    dt_seconds: f64,
    use_gpu: bool,
    device: Device,
    decay_constant: f64,
    dry_deposition_velocity: f64,
    wet_deposition_rate: f64,
    /// GPU buffers for particle data
    gpu_positions: Option<Tensor>,
    gpu_activities: Option<Tensor>,
    gpu_deposited: Option<Tensor>,
    gpu_half_lives: Option<Tensor>,
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
        
        let device = if config.use_gpu {
            Device::cuda_if_available(0).unwrap_or(Device::Cpu)
        } else {
            Device::Cpu
        };
        
        info!("Using device: {:?}", device);
        
        Self {
            grid,
            particles: Vec::with_capacity(config.num_particles),
            dt_seconds: config.dt_seconds,
            use_gpu: config.use_gpu && device.is_cuda(),
            device,
            decay_constant: 0.0,
            dry_deposition_velocity: 0.01,
            wet_deposition_rate: 0.0,
            gpu_positions: None,
            gpu_activities: None,
            gpu_deposited: None,
            gpu_half_lives: None,
        }
    }
    
    /// Initialize GPU buffers for particle data
    fn init_gpu_buffers(&mut self) -> anyhow::Result<()> {
        if !self.use_gpu {
            return Ok(());
        }
        
        let n = self.particles.len();
        
        // Extract particle data into contiguous arrays
        let positions: Vec<f32> = self.particles.iter()
            .flat_map(|p| vec![p.position.x as f32, p.position.y as f32, p.position.z as f32])
            .collect();
        
        let activities: Vec<f32> = self.particles.iter()
            .map(|p| p.activity_bq as f32)
            .collect();
        
        let deposited: Vec<f32> = self.particles.iter()
            .map(|p| if p.deposited { 1.0f32 } else { 0.0f32 })
            .collect();
        
        let half_lives: Vec<f32> = self.particles.iter()
            .map(|p| p.half_life_hours as f32)
            .collect();
        
        // Create tensors on GPU
        self.gpu_positions = Some(Tensor::from_vec(
            positions, 
            (n, 3), 
            &self.device
        )?);
        
        self.gpu_activities = Some(Tensor::from_vec(
            activities,
            (n,),
            &self.device
        )?);
        
        self.gpu_deposited = Some(Tensor::from_vec(
            deposited,
            (n,),
            &self.device
        )?);
        
        self.gpu_half_lives = Some(Tensor::from_vec(
            half_lives,
            (n,),
            &self.device
        )?);
        
        info!("GPU buffers initialized for {} particles", n);
        
        Ok(())
    }
    
    /// Synchronize particle data from GPU to CPU
    fn sync_from_gpu(&mut self) -> anyhow::Result<()> {
        if !self.use_gpu {
            return Ok(());
        }
        
        if let Some(ref positions) = self.gpu_positions {
            let pos_data = positions.to_vec1::<f32>()?;
            for (i, particle) in self.particles.iter_mut().enumerate() {
                particle.position.x = pos_data[i * 3] as f64;
                particle.position.y = pos_data[i * 3 + 1] as f64;
                particle.position.z = pos_data[i * 3 + 2] as f64;
            }
        }
        
        if let Some(ref activities) = self.gpu_activities {
            let act_data = activities.to_vec1::<f32>()?;
            for (i, particle) in self.particles.iter_mut().enumerate() {
                particle.activity_bq = act_data[i] as f64;
            }
        }
        
        if let Some(ref deposited) = self.gpu_deposited {
            let dep_data = deposited.to_vec1::<f32>()?;
            for (i, particle) in self.particles.iter_mut().enumerate() {
                particle.deposited = dep_data[i] > 0.5;
            }
        }
        
        Ok(())
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
                concentration_grid.timestamps.push(elapsed_hours as i64);
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
        }
    }
    
    fn initialize_particles(&mut self, release: &ReleaseParameters) {
        let num_particles = self.particles.capacity();
        let total_activity = release.release_rate_bq_s * release.duration_hours as f64 * 3600.0;
        let activity_per_particle = total_activity / num_particles as f64;
        
        let isotope = release.isotope.clone();
        let half_life = get_half_life_hours(&isotope);
        
        for _i in 0..num_particles {
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
        if !self.use_gpu {
            // Fallback to CPU
            self.apply_decay(elapsed_hours);
            self.apply_dry_deposition();
            self.apply_wet_deposition(elapsed_hours);
            return;
        }
        
        let start = Instant::now();
        
        // Initialize buffers if not already done
        if self.gpu_positions.is_none() {
            if let Err(e) = self.init_gpu_buffers() {
                warn!("Failed to initialize GPU buffers: {}, falling back to CPU", e);
                self.use_gpu = false;
                self.apply_decay(elapsed_hours);
                self.apply_dry_deposition();
                return;
            }
        }
        
        // Apply radioactive decay on GPU
        if let Err(e) = self.apply_decay_gpu(elapsed_hours) {
            warn!("GPU decay failed: {}, falling back to CPU", e);
            self.apply_decay(elapsed_hours);
        }
        
        // Apply dry deposition on GPU
        if let Err(e) = self.apply_dry_deposition_gpu() {
            warn!("GPU dry deposition failed: {}, falling back to CPU", e);
            self.apply_dry_deposition();
        }
        
        // Apply wet deposition on GPU
        if let Err(e) = self.apply_wet_deposition_gpu(elapsed_hours) {
            warn!("GPU wet deposition failed: {}, falling back to CPU", e);
            self.apply_wet_deposition(elapsed_hours);
        }
        
        debug!("GPU physics applied in {:?}", start.elapsed());
    }
    
    /// Apply radioactive decay using GPU
    fn apply_decay_gpu(&mut self, elapsed_hours: f64) -> anyhow::Result<()> {
        let half_lives = self.gpu_half_lives.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Half-life buffer not initialized"))?;
        let activities = self.gpu_activities.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Activity buffer not initialized"))?;
        
        let _n = activities.dims()[0];
        
        // decay_factor = exp(-0.693 * elapsed_hours / half_life)
        let decay_constant = -0.693f32 * elapsed_hours as f32;
        let decay_tensor = Tensor::new(decay_constant, &self.device)?;
        
        // Calculate decay factors: exp(decay_constant / half_life)
        let decay_factors = decay_tensor.broadcast_div(half_lives)?.exp()?;
        
        // Apply decay: activity *= decay_factor
        *activities = activities.broadcast_mul(&decay_factors)?;
        
        Ok(())
    }
    
    /// Apply dry deposition using GPU
    fn apply_dry_deposition_gpu(&mut self) -> anyhow::Result<()> {
        let positions = self.gpu_positions.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Position buffer not initialized"))?;
        let deposited = self.gpu_deposited.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Deposited buffer not initialized"))?;
        let activities = self.gpu_activities.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Activity buffer not initialized"))?;
        
        // Get z-coordinates (height)
        let z_coords = positions.narrow(1, 2, 1)?.squeeze(1)?;
        
        // Create mask for particles near ground (z < 10m) and not yet deposited
        let near_ground = z_coords.lt(10.0f32)?;
        let not_deposited = deposited.lt(0.5f32)?;
        let can_deposit = near_ground.broadcast_mul(&not_deposited)?;
        
        // Deposition probability
        let dep_prob = self.dry_deposition_velocity as f32 * self.dt_seconds as f32 / 10.0f32;
        
        // Random deposition (simplified - use probability threshold)
        // In production, this would use a proper random number generator on GPU
        let random_threshold = Tensor::rand(0.0f32, 1.0f32, can_deposit.shape(), &self.device)?;
        let will_deposit = random_threshold.lt(dep_prob)?.broadcast_mul(&can_deposit)?;
        
        // Update deposited status
        *deposited = deposited.broadcast_add(&will_deposit)?.clamp(0.0f32, 1.0f32)?;
        
        // Set deposited particles to ground level
        let zero_z = Tensor::zeros(z_coords.shape(), DType::F32, &self.device)?;
        let new_z = will_deposit.broadcast_mul(&zero_z)?;
        let keep_z = not_deposited.broadcast_mul(&z_coords)?;
        let _updated_z = keep_z.broadcast_add(&new_z)?;
        
        // Zero out activity of deposited particles (optional - track deposited activity separately)
        let zero_activity = Tensor::zeros(activities.shape(), DType::F32, &self.device)?;
        let keep_activity = not_deposited.broadcast_mul(activities)?;
        let deposited_activity = will_deposit.broadcast_mul(&zero_activity)?;
        *activities = keep_activity.broadcast_add(&deposited_activity)?;
        
        Ok(())
    }
    
    /// Apply wet deposition using GPU
    fn apply_wet_deposition_gpu(&mut self, _elapsed_hours: f64) -> anyhow::Result<()> {
        let rain_rate = self.grid.get_rain_rate(_elapsed_hours) as f32;
        
        if rain_rate <= 0.0 {
            return Ok(());
        }
        
        let deposited = self.gpu_deposited.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Deposited buffer not initialized"))?;
        let activities = self.gpu_activities.as_mut()
            .ok_or_else(|| anyhow::anyhow!("Activity buffer not initialized"))?;
        
        let not_deposited = deposited.lt(0.5f32)?;
        
        // Scavenging coefficient
        let scavenging_coeff = 1e-4f32 * rain_rate;
        let wet_dep_prob = scavenging_coeff * self.dt_seconds as f32;
        
        // Random wet deposition
        let random_threshold = Tensor::rand(0.0f32, 1.0f32, not_deposited.shape(), &self.device)?;
        let will_deposit = random_threshold.lt(wet_dep_prob)?.broadcast_mul(&not_deposited)?;
        
        // Update deposited status
        *deposited = deposited.broadcast_add(&will_deposit)?.clamp(0.0f32, 1.0f32)?;
        
        // Zero out activity of wet-deposited particles
        let zero_activity = Tensor::zeros(activities.shape(), DType::F32, &self.device)?;
        let keep_activity = not_deposited.broadcast_mul(activities)?;
        let deposited_activity = will_deposit.broadcast_mul(&zero_activity)?;
        *activities = keep_activity.broadcast_add(&deposited_activity)?;
        
        Ok(())
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

/// Simplified Gaussian plume model for atmospheric dispersion
/// C(x,y,z) = Q / (2π σ_y σ_z u) * exp(-y²/2σ_y²) * [exp(-(z-H)²/2σ_z²) + exp(-(z+H)²/2σ_z²)]
pub struct GaussianPlumeModel {
    pub weather: WeatherConditions,
    pub release: ReleaseParameters,
}

#[derive(Debug, Clone)]
pub struct WeatherConditions {
    pub wind_speed_ms: f64,
    pub wind_direction_deg: f64,
    pub stability_class: StabilityClass,
    pub temperature_k: f64,
    pub pressure_pa: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum StabilityClass {
    A, // Very unstable
    B, // Unstable
    C, // Slightly unstable
    D, // Neutral
    E, // Slightly stable
    F, // Stable
}

impl StabilityClass {
    /// Get dispersion coefficients for this stability class
    /// Returns (a_y, b_y, a_z, b_z) for σ_y = a_y * x^b_y, σ_z = a_z * x^b_z
    pub fn dispersion_coefficients(&self) -> (f64, f64, f64, f64) {
        match self {
            // Coefficients from Pasquill-Gifford model (x in meters)
            StabilityClass::A => (0.22, 0.90, 0.20, 0.90),
            StabilityClass::B => (0.16, 0.90, 0.12, 0.90),
            StabilityClass::C => (0.11, 0.90, 0.08, 0.90),
            StabilityClass::D => (0.08, 0.90, 0.06, 0.90),
            StabilityClass::E => (0.06, 0.90, 0.04, 0.90),
            StabilityClass::F => (0.04, 0.90, 0.02, 0.90),
        }
    }
}

impl Default for WeatherConditions {
    fn default() -> Self {
        Self {
            wind_speed_ms: 5.0,
            wind_direction_deg: 0.0,
            stability_class: StabilityClass::D,
            temperature_k: 288.15,
            pressure_pa: 101325.0,
        }
    }
}

impl GaussianPlumeModel {
    pub fn new(weather: WeatherConditions, release: ReleaseParameters) -> Self {
        Self { weather, release }
    }

    /// Calculate concentration at a point (x, y, z) downwind from source
    /// x: downwind distance (m)
    /// y: crosswind distance (m)
    /// z: height above ground (m)
    pub fn concentration(&self, x: f64, y: f64, z: f64) -> f64 {
        if x <= 0.0 || self.weather.wind_speed_ms <= 0.0 {
            return 0.0;
        }

        let q = self.release.release_rate_bq_s;
        let u = self.weather.wind_speed_ms;
        let h = self.release.altitude_m;
        
        let (sigma_y, sigma_z) = self.dispersion_parameters(x);
        
        // Gaussian plume equation
        let exp_y = (-y * y / (2.0 * sigma_y * sigma_y)).exp();
        let exp_z1 = (-(z - h).powi(2) / (2.0 * sigma_z * sigma_z)).exp();
        let exp_z2 = (-(z + h).powi(2) / (2.0 * sigma_z * sigma_z)).exp();
        
        let denominator = 2.0 * std::f64::consts::PI * sigma_y * sigma_z * u;
        
        q / denominator * exp_y * (exp_z1 + exp_z2)
    }

    /// Calculate ground-level concentration (z = 0)
    pub fn ground_level_concentration(&self, x: f64, y: f64) -> f64 {
        self.concentration(x, y, 0.0)
    }

    /// Calculate centerline concentration (y = 0, z = 0)
    pub fn centerline_concentration(&self, x: f64) -> f64 {
        self.concentration(x, 0.0, 0.0)
    }

    /// Calculate dose rate at a point (microsieverts per hour)
    pub fn dose_rate(&self, x: f64, y: f64, z: f64) -> f64 {
        let concentration_bq_m3 = self.concentration(x, y, z);
        let dose_factor = self.dose_conversion_factor();
        
        // Dose rate = concentration * dose conversion factor
        concentration_bq_m3 * dose_factor * 1e6 // Convert to microsieverts
    }

    /// Calculate ground-level dose rate
    pub fn ground_level_dose_rate(&self, x: f64, y: f64) -> f64 {
        self.dose_rate(x, y, 0.0)
    }

    /// Generate evacuation zone contour for a given dose threshold
    pub fn contour(&self, dose_threshold_microsv_h: f64, max_distance_m: f64) -> Vec<(f64, f64)> {
        let mut contour_points = Vec::new();
        let step = 100.0; // 100m resolution
        
        // Find contour by scanning in polar coordinates
        for angle_deg in (0..360).step_by(5) {
            let angle_rad = angle_deg as f64 * std::f64::consts::PI / 180.0;
            
            // Search along this angle for the contour
            for dist in (0..(max_distance_m as usize)).step_by(step as usize) {
                let x = dist as f64 * angle_rad.cos();
                let y = dist as f64 * angle_rad.sin();
                
                let dose = self.ground_level_dose_rate(x, y);
                
                if dose >= dose_threshold_microsv_h {
                    contour_points.push((x, y));
                    break;
                }
            }
        }
        
        contour_points
    }

    /// Calculate dispersion parameters σ_y and σ_z
    fn dispersion_parameters(&self, x: f64) -> (f64, f64) {
        let (a_y, b_y, a_z, b_z) = self.weather.stability_class.dispersion_coefficients();
        
        let sigma_y = a_y * x.powf(b_y);
        let sigma_z = a_z * x.powf(b_z);
        
        (sigma_y.max(1.0), sigma_z.max(1.0))
    }

    /// Dose conversion factor (Sv per Bq/m³) - simplified
    fn dose_conversion_factor(&self) -> f64 {
        // Simplified dose conversion based on isotope
        // Typical values range from 1e-15 to 1e-12 Sv per Bq/m³
        match self.release.isotope.as_str() {
            "I-131" => 3.2e-14,
            "Cs-137" => 2.1e-14,
            "Cs-134" => 2.8e-14,
            "Co-60" => 3.5e-14,
            "Am-241" => 1.8e-13,
            _ => 2.0e-14, // Default
        }
    }

    /// Calculate total integrated dose over time
    pub fn total_integrated_dose(&self, x: f64, y: f64, duration_hours: f64) -> f64 {
        let avg_concentration = self.ground_level_concentration(x, y);
        let breathing_rate = 1.2; // m³/hour (adult)
        let dose_coefficient = self.dose_conversion_factor();
        
        // Total dose = concentration * breathing rate * time * dose coefficient
        avg_concentration * breathing_rate * duration_hours * dose_coefficient * 1e6
    }
}
