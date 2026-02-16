use crate::renderer::WebGLRenderer;
use nalgebra::{Matrix4, Vector3, Point3};
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;

pub struct RadiationGlobe {
    renderer: WebGLRenderer,
    sensors: HashMap<String, SensorData>,
    facilities: HashMap<String, FacilityData>,
    plume_particles: Vec<PlumeParticle>,
    layer_visibility: HashMap<String, bool>,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    camera_position: Point3<f32>,
    current_time: f64,
    plume_origin: (f64, f64),
}

#[derive(Clone)]
pub struct SensorData {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub value: f64,
    pub color: [f32; 4],
}

#[derive(Clone)]
pub struct FacilityData {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub status: String,
    pub color: [f32; 4],
}

#[derive(Clone)]
pub struct PlumeParticle {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub intensity: f32,
    pub age: f32,
    pub max_age: f32,
    pub birth_time: f64,
}

impl RadiationGlobe {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, wasm_bindgen::JsValue> {
        let renderer = WebGLRenderer::new(canvas)?;
        
        let sensors = HashMap::new();
        let facilities = HashMap::new();
        let plume_particles = Vec::new();
        let mut layer_visibility = HashMap::new();
        
        // Initialize default layer visibility
        layer_visibility.insert("sensor-heatmap".to_string(), true);
        layer_visibility.insert("sensor-points".to_string(), false);
        layer_visibility.insert("facilities".to_string(), true);
        layer_visibility.insert("plume-simulation".to_string(), false);
        layer_visibility.insert("anomalies".to_string(), true);
        layer_visibility.insert("seismic".to_string(), false);
        
        let view_matrix = Matrix4::identity();
        let projection_matrix = Matrix4::identity();
        let camera_position = Point3::new(0.0, 0.0, 3.0);
        
        Ok(Self {
            renderer,
            sensors,
            facilities,
            plume_particles,
            layer_visibility,
            view_matrix,
            projection_matrix,
            camera_position,
            current_time: 0.0,
            plume_origin: (0.0, 0.0),
        })
    }

    pub fn render(&mut self) {
        self.renderer.clear();
        self.renderer.draw_globe(&self.view_matrix, &self.projection_matrix);
        
        // Render sensors if layer is visible
        if *self.layer_visibility.get("sensor-points").unwrap_or(&true) {
            for sensor in self.sensors.values() {
                let position = self.lat_lon_to_xyz(sensor.lat, sensor.lon);
                self.renderer.draw_sensor(
                    &position,
                    sensor.value as f32,
                    &sensor.color,
                    &self.view_matrix,
                    &self.projection_matrix,
                );
            }
        }
        
        // Render facilities if layer is visible
        if *self.layer_visibility.get("facilities").unwrap_or(&true) {
            for facility in self.facilities.values() {
                let position = self.lat_lon_to_xyz(facility.lat, facility.lon);
                self.renderer.draw_facility(
                    &position,
                    &facility.color,
                    &self.view_matrix,
                    &self.projection_matrix,
                );
            }
        }
        
        // Render plume if layer is visible
        if *self.layer_visibility.get("plume-simulation").unwrap_or(&false) {
            self.update_plume_animation();
            for particle in &self.plume_particles {
                // Fade out based on age
                let age_ratio = particle.age / particle.max_age;
                let fade = 1.0 - age_ratio * age_ratio;
                let intensity = particle.intensity * fade;
                
                self.renderer.draw_plume_particle(
                    &particle.position,
                    intensity,
                    &self.view_matrix,
                    &self.projection_matrix,
                );
            }
        }
    }

    fn update_plume_animation(&mut self) {
        // Remove dead particles
        self.plume_particles.retain(|p| {
            let age = (self.current_time - p.birth_time) as f32;
            age < p.max_age
        });
        
        // Update particle positions based on velocity and time
        for particle in &mut self.plume_particles {
            let dt = 0.016f32; // Assume 60fps
            particle.position += particle.velocity * dt;
            particle.age = (self.current_time - particle.birth_time) as f32;
            
            // Add some turbulence
            let turbulence = Vector3::new(
                (self.current_time.sin() * 0.01) as f32,
                (self.current_time.cos() * 0.005) as f32,
                ((self.current_time * 0.7).sin() * 0.01) as f32,
            );
            particle.velocity += turbulence * dt;
        }
    }

    pub fn update_sensor(&mut self, id: &str, value: f64) {
        let color = self.value_to_color(value);
        
        if let Some(sensor) = self.sensors.get_mut(id) {
            sensor.value = value;
            sensor.color = color;
        } else {
            let (lat, lon) = self.parse_sensor_location(id);
            self.sensors.insert(id.to_string(), SensorData {
                id: id.to_string(),
                lat,
                lon,
                value,
                color,
            });
        }
    }
    
    pub fn update_sensor_with_location(&mut self, id: &str, lat: f64, lon: f64, value: f64) {
        let color = self.value_to_color(value);
        
        if let Some(sensor) = self.sensors.get_mut(id) {
            sensor.value = value;
            sensor.color = color;
        } else {
            self.sensors.insert(id.to_string(), SensorData {
                id: id.to_string(),
                lat,
                lon,
                value,
                color,
            });
        }
    }
    
    pub fn add_facility(&mut self, id: &str, lat: f64, lon: f64, status: &str) {
        let color = self.status_to_color(status);
        
        self.facilities.insert(id.to_string(), FacilityData {
            id: id.to_string(),
            lat,
            lon,
            status: status.to_string(),
            color,
        });
    }
    
    pub fn update_plume(&mut self, lat: f64, lon: f64, particles: &[f64]) {
        self.plume_origin = (lat, lon);
        self.plume_particles.clear();
        
        // Parse particle data (x, y, z, vx, vy, vz, intensity, max_age octuples)
        for chunk in particles.chunks(8) {
            if chunk.len() == 8 {
                self.plume_particles.push(PlumeParticle {
                    position: Vector3::new(chunk[0] as f32, chunk[1] as f32, chunk[2] as f32),
                    velocity: Vector3::new(chunk[3] as f32, chunk[4] as f32, chunk[5] as f32),
                    intensity: chunk[6] as f32,
                    age: 0.0,
                    max_age: chunk[7] as f32,
                    birth_time: self.current_time,
                });
            }
        }
    }
    
    pub fn set_time(&mut self, time: f64) {
        self.current_time = time;
    }
    
    pub fn set_layer_visibility(&mut self, layer: &str, visible: bool) {
        self.layer_visibility.insert(layer.to_string(), visible);
    }

    pub fn set_view(&mut self, lat: f64, lon: f64, zoom: f64) {
        let target = self.lat_lon_to_xyz(lat, lon);
        self.camera_position = Point3::new(
            target.x + zoom as f32 * target.x.signum(),
            target.y + zoom as f32 * target.y.signum(),
            target.z + zoom as f32 * target.z.signum(),
        );
        
        self.view_matrix = Matrix4::look_at_rh(
            &self.camera_position,
            &target,
            &Vector3::y_axis(),
        );
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
        let aspect = width as f32 / height as f32;
        self.projection_matrix = Matrix4::new_perspective(aspect, 45.0f32.to_radians(), 0.1, 100.0);
    }
    
    fn lat_lon_to_xyz(&self, lat: f64, lon: f64) -> Vector3<f32> {
        let lat_rad = lat.to_radians();
        let lon_rad = lon.to_radians();
        let r = 1.0f32;
        
        Vector3::new(
            (r * lat_rad.cos() * lon_rad.cos()) as f32,
            (r * lat_rad.sin()) as f32,
            (r * lat_rad.cos() * lon_rad.sin()) as f32,
        )
    }
    
    fn value_to_color(&self, value: f64) -> [f32; 4] {
        let normalized = (value / 10.0).min(1.0).max(0.0);
        
        if normalized < 0.33 {
            let t = normalized / 0.33;
            [0.0, t as f32, 1.0 - t as f32, 0.8]
        } else if normalized < 0.66 {
            let t = (normalized - 0.33) / 0.33;
            [t as f32, 1.0, 0.0, 0.8]
        } else {
            let t = (normalized - 0.66) / 0.34;
            [1.0, 1.0 - t as f32, 0.0, 0.8]
        }
    }
    
    fn status_to_color(&self, status: &str) -> [f32; 4] {
        match status {
            "operational" => [0.0, 1.0, 0.0, 0.8],
            "maintenance" => [1.0, 0.65, 0.0, 0.8],
            "shutdown" => [0.5, 0.5, 0.5, 0.8],
            "emergency" => [1.0, 0.0, 0.0, 0.8],
            _ => [0.0, 0.84, 1.0, 0.8],
        }
    }

    fn parse_sensor_location(&self, id: &str) -> (f64, f64) {
        (0.0, 0.0)
    }
}
