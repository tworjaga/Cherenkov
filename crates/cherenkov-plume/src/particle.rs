use nalgebra::Vector3;
use rand;

pub struct RadioactiveParticle {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub diameter_um: f64,
    pub density_kg_m3: f64,
    pub activity_bq: f64,
    pub half_life_seconds: f64,
    pub isotope: String,
    pub deposited: bool,
    pub decayed: bool,
}

impl RadioactiveParticle {
    pub fn new(
        position: Vector3<f64>,
        diameter_um: f64,
        activity_bq: f64,
        half_life_seconds: f64,
        isotope: String,
    ) -> Self {
        Self {
            position,
            velocity: Vector3::zeros(),
            diameter_um,
            density_kg_m3: 1000.0,
            activity_bq,
            half_life_seconds,
            isotope,
            deposited: false,
            decayed: false,
        }
    }
    
    pub fn mass_kg(&self) -> f64 {
        let radius_m = self.diameter_um * 1e-6 / 2.0;
        let volume_m3 = 4.0 / 3.0 * std::f64::consts::PI * radius_m.powi(3);
        volume_m3 * self.density_kg_m3
    }
    
    pub fn settling_velocity(&self, air_density_kg_m3: f64, viscosity_pa_s: f64) -> f64 {
        // Stokes law for settling velocity
        let g = 9.81;
        let radius_m = self.diameter_um * 1e-6 / 2.0;
        
        let delta_rho = self.density_kg_m3 - air_density_kg_m3;
        let v_stokes = 2.0 / 9.0 * g * radius_m.powi(2) * delta_rho / viscosity_pa_s;
        
        // Cunningham slip correction for small particles
        let lambda = 68e-9; // Mean free path in air
        let kn = lambda / radius_m;
        let cc = 1.0 + kn * (1.257 + 0.4 * (-1.1 / kn).exp());
        
        v_stokes * cc
    }
    
    pub fn apply_decay(&mut self, dt_seconds: f64) {
        let decay_constant = 0.693 / self.half_life_seconds;
        let survival_prob = (-decay_constant * dt_seconds).exp();
        
        if rand::random::<f64>() > survival_prob {
            self.decayed = true;
            self.activity_bq = 0.0;
        } else {
            self.activity_bq *= survival_prob;
        }
    }
    
    pub fn dry_deposition_velocity(&self, surface_type: SurfaceType) -> f64 {
        let v_g = self.settling_velocity(1.225, 1.81e-5);
        
        let r_a = match surface_type {
            SurfaceType::Water => 100.0,
            SurfaceType::Grass => 50.0,
            SurfaceType::Forest => 20.0,
            SurfaceType::Urban => 200.0,
        };
        
        let r_b = 10.0; // Quasi-laminar layer resistance
        
        v_g + 1.0 / (r_a + r_b + r_a * r_b * v_g)
    }
}

pub enum SurfaceType {
    Water,
    Grass,
    Forest,
    Urban,
}
