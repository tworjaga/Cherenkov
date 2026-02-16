export class RadiationGlobe {
  constructor(canvas: HTMLCanvasElement);
  updateSensor(id: string, lat: number, lon: number, value: number): void;
  render(): void;
  setView(lat: number, lon: number, zoom: number): void;
  resize(width: number, height: number): void;
  addFacility(id: string, lat: number, lon: number, status: string): void;
  updatePlume(lat: number, lon: number, particles: Float64Array): void;
  setLayerVisibility(layer: string, visible: boolean): void;
  setTime(time: number): void;
}


export default function init(): Promise<void>;
