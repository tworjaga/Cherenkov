use crate::renderer::WebGLRenderer;
use nalgebra::{Matrix4, Vector3, Point3};
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;

pub struct RadiationGlobe {
    renderer: WebGLRenderer,
    sensors: HashMap<String, SensorData>,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    camera_position: Point3<f32>,
}

#[derive(Clone)]
pub struct SensorData {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub value: f64,
    pub color: [f32; 4],
}

impl RadiationGlobe {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, wasm_bindgen::JsValue> {
        let renderer = WebGLRenderer::new(canvas)?;
        
        let sensors = HashMap::new();
        let view_matrix = Matrix4::identity();
        let projection_matrix = Matrix4::identity();
        let camera_position = Point3::new(0.0, 0.0, 3.0);
        
        Ok(Self {
            renderer,
            sensors,
            view_matrix,
            projection_matrix,
            camera_position,
        })
    }
    
    pub fn render(&mut self) {
        self.renderer.clear();
        self.renderer.draw_globe(&self.view_matrix, &self.projection_matrix);
        
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
    
    pub fn update_sensor(&mut self, id: &str, value: f64) {
        let color = self.value_to_color(value);
        
        if let Some(sensor) = self.sensors.get_mut(id) {
            sensor.value = value;
            sensor.color = color;
        } else {
            // Parse id to extract lat/lon if embedded
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
        // Color scale: blue (low) -> green -> yellow -> red (high)
        let normalized = (value / 10.0).min(1.0).max(0.0);
        
        if normalized < 0.33 {
            // Blue to green
            let t = normalized / 0.33;
            [0.0, t as f32, 1.0 - t as f32, 0.8]
        } else if normalized < 0.66 {
            // Green to yellow
            let t = (normalized - 0.33) / 0.33;
            [t as f32, 1.0, 0.0, 0.8]
        } else {
            // Yellow to red
            let t = (normalized - 0.66) / 0.34;
            [1.0, 1.0 - t as f32, 0.0, 0.8]
        }
    }
    
    fn parse_sensor_location(&self, id: &str) -> (f64, f64) {
        // Default to 0,0 if parsing fails
        (0.0, 0.0)
    }
}
