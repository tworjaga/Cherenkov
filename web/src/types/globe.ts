/**
 * Globe visualization type definitions
 * TypeScript interfaces for 3D globe components
 */

export interface GlobeLayer {

  id: string;
  type: 'sensors' | 'facilities' | 'anomalies' | 'plumes' | 'heatmap';
  visible: boolean;
  opacity: number;
  data: unknown[];
}

export interface SensorMarker {
  id: string;
  latitude: number;
  longitude: number;
  altitude?: number;
  value: number;
  unit: string;
  status: 'normal' | 'warning' | 'critical' | 'offline';
  type: string;
  name: string;
  facilityId?: string;
}

export interface FacilityMarker {
  id: string;
  latitude: number;
  longitude: number;
  name: string;
  type: 'power_plant' | 'research' | 'industrial' | 'medical';
  status: 'operational' | 'maintenance' | 'shutdown' | 'emergency';
  sensorCount: number;
  country: string;
}

export interface AnomalyMarker {
  id: string;
  latitude: number;
  longitude: number;
  radius: number;
  intensity: number;
  type: string;
  confidence: number;
  detectedAt: string;
  sensorId: string;
}

export interface PlumeVisualization {
  id: string;
  sourceLat: number;
  sourceLon: number;
  sourceAltitude: number;
  particles: PlumeParticle[];
  timestamp: string;
  windSpeed: number;
  windDirection: number;
  dispersionModel: 'gaussian' | 'lagrangian';
}

export interface PlumeParticle {
  id: string;
  lat: number;
  lon: number;
  altitude: number;
  concentration: number;
  age: number;
}

export interface HeatmapCell {
  lat: number;
  lon: number;
  value: number;
  count: number;
}

export interface GlobeTooltipData {
  type: 'sensor' | 'facility' | 'anomaly' | 'plume';
  data: SensorMarker | FacilityMarker | AnomalyMarker | PlumeVisualization;
  x: number;
  y: number;
}

export interface GlobeSelection {
  type: 'sensor' | 'facility' | 'region';
  id: string;
  bounds?: {
    north: number;
    south: number;
    east: number;
    west: number;
  };
}
