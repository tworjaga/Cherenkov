// Stub WASM module - will be replaced by actual wasm-bindgen output
export class RadiationGlobe {
  constructor(canvas) {
    this.canvas = canvas;
    this.gl = canvas.getContext('webgl2');
    console.log('RadiationGlobe initialized (stub)');
  }
  
  updateSensor(id, lat, lon, value) {
    // Stub implementation
  }
  
  render() {
    // Stub implementation
  }
}

export default function init() {
  return Promise.resolve();
}
