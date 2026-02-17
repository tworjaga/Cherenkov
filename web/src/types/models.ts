export interface GeoLocation {
  lat: number;
  lon: number;
}

export interface Sensor {
  id: string;
  name: string;
  location: GeoLocation;
  status: 'active' | 'inactive' | 'maintenance' | 'offline';
  source: string;
  lastReading: Reading | null;
}

export interface Reading {
  timestamp: number;
  doseRate: number;
  unit: string;
  qualityFlag: 'good' | 'suspect' | 'bad';
  uncertainty?: number;
}

export interface Anomaly {
  id: string;
  sensorId: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  zScore: number;
  detectedAt: number;
  doseRate: number;
  baseline: number;
  algorithm: string;
  acknowledged: boolean;
  location: GeoLocation;
  message: string;
}

export interface Facility {
  id: string;
  name: string;
  type: 'nuclear' | 'research' | 'medical' | 'industrial';
  location: GeoLocation;
  status: 'operating' | 'shutdown' | 'incident' | 'decommissioned';
  reactorType?: string;
  capacity?: number;
  reactorCount?: number;
  currentOutput?: number;
}

export interface PlumeSimulation {
  id: string;
  status: 'running' | 'completed' | 'failed';
  releaseLocation: GeoLocation;
  releaseHeight: number;
  releaseRate: number;
  isotope: string;
  duration: number;
  estimatedArrival?: number;
  concentrationGrid: ConcentrationPoint[];
  maxDose: number;
}

export interface ConcentrationPoint {
  lat: number;
  lon: number;
  concentration: number;
}

export interface GlobalStatus {
  level: number;
  defcon: 1 | 2 | 3 | 4 | 5;
  status: 'NORMAL' | 'ELEVATED' | 'CRITICAL';
  activeAlerts: number;
  activeSensors: number;
  lastUpdate: number;
}

export interface Alert {
  id: string;
  type: 'anomaly' | 'system' | 'facility';
  severity: 'low' | 'medium' | 'high' | 'critical';
  message: string;
  timestamp: number;
  location?: GeoLocation;
  metadata?: Record<string, unknown>;
  acknowledged: boolean;
  acknowledgedAt?: number;
}

export interface TimeSeriesPoint {
  timestamp: number;
  value: number;
  count?: number;
}

export interface RegionalStat {
  region: string;
  averageDose: number;
  sensorCount: number;
  alertCount: number;
}

export interface Viewport {
  latitude: number;
  longitude: number;
  zoom: number;
  pitch: number;
  bearing: number;
}

export interface GlobeViewport {
  latitude: number;
  longitude: number;
  zoom: number;
  pitch: number;
  bearing: number;
}

export type ViewType = 'dashboard' | 'globe' | 'sensors' | 'anomalies' | 'plume' | 'settings';

export type TimeMode = 'live' | 'paused' | 'replay';
export type ConnectionStatus = 'connected' | 'connecting' | 'disconnected';
