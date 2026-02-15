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
pub struct GlobeHandle {
    globe: RadiationGlobe,
}

#[wasm_bindgen]
impl GlobeHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<GlobeHandle, JsValue> {
        let globe = RadiationGlobe::new(canvas)?;
        Ok(Self { globe })
    }
    
    pub fn render(&mut self) {
        self.globe.render();
    }
    
    pub fn update_sensor(&mut self, id: &str, value: f64) {
        self.globe.update_sensor(id, value);
    }
    
    pub fn set_view(&mut self, lat: f64, lon: f64, zoom: f64) {
        self.globe.set_view(lat, lon, zoom);
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.globe.resize(width, height);
    }
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
