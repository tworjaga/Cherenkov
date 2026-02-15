export class RadiationGlobe {
  constructor(canvas: HTMLCanvasElement);
  updateSensor(id: string, lat: number, lon: number, value: number): void;
  render(): void;
}

export default function init(): Promise<void>;
