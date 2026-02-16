mod globe;
mod renderer;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub use globe::RadiationGlobe;
pub use renderer::WebGLRenderer;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    console_log!("Cherenkov WebGL2 Globe initialized");
}

#[wasm_bindgen]
pub struct RadiationGlobe {
    inner: globe::RadiationGlobe,
}

#[wasm_bindgen]
impl RadiationGlobe {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<RadiationGlobe, JsValue> {
        let inner = globe::RadiationGlobe::new(canvas)?;
        Ok(Self { inner })
    }
    
    pub fn render(&mut self) {
        self.inner.render();
    }
    
    pub fn update_sensor(&mut self, id: &str, lat: f64, lon: f64, value: f64) {
        self.inner.update_sensor_with_location(id, lat, lon, value);
    }
    
    pub fn set_view(&mut self, lat: f64, lon: f64, zoom: f64) {
        self.inner.set_view(lat, lon, zoom);
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.inner.resize(width, height);
    }
    
    pub fn add_facility(&mut self, id: &str, lat: f64, lon: f64, status: &str) {
        self.inner.add_facility(id, lat, lon, status);
    }
    
    pub fn update_plume(&mut self, lat: f64, lon: f64, particles: &[f64]) {
        self.inner.update_plume(lat, lon, particles);
    }
    
    pub fn set_layer_visibility(&mut self, layer: &str, visible: bool) {
        self.inner.set_layer_visibility(layer, visible);
    }
    
    pub fn set_time(&mut self, time: f64) {
        self.inner.set_time(time);
    }

}

#[wasm_bindgen]
pub async fn init() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log!("Cherenkov WASM module initialized");
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
