export interface Sensor {
  id: string;
  name: string;
  location: {
    latitude: number;
    longitude: number;
    elevation?: number;
  };
  type: 'gamma' | 'neutron' | 'alpha' | 'beta' | 'multi';
  status: 'online' | 'offline' | 'maintenance' | 'error';
  lastReading?: Reading;
  facilityId?: string;
}

export interface Reading {
  id: string;
  sensorId: string;
  timestamp: string;
  doseRate: number;
  unit: 'μSv/h' | 'mSv/h' | 'Sv/h';
  isAlert: boolean;
  alertLevel?: 'low' | 'medium' | 'high' | 'critical';
}

export interface Alert {
  id: string;
  title: string;
  message: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  timestamp: string;
  sensorId?: string;
  facilityId?: string;
  acknowledged: boolean;
  location?: {
    latitude: number;
    longitude: number;
  };
}

export interface Facility {
  id: string;
  name: string;
  type: 'nuclear_plant' | 'research' | 'medical' | 'industrial';
  location: {
    latitude: number;
    longitude: number;
  };
  status: 'operational' | 'maintenance' | 'incident' | 'shutdown';
  reactorCount?: number;
}

export interface User {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'operator' | 'viewer';
  preferences: UserPreferences;
}

export interface UserPreferences {
  pushNotifications: boolean;
  alertThreshold: number;
  darkMode: boolean;
  language: string;
}

export interface EvacuationZone {
  id: string;
  name: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  coordinates: Array<{ latitude: number; longitude: number }>;
  center: {
    latitude: number;
    longitude: number;
  };
  radius: number;
  estimatedPopulation: number;
  doseRate: number;
  createdAt: string;
  expiresAt?: string;
}

export type DefconLevel = 1 | 2 | 3 | 4 | 5;

export interface SystemStatus {
  defconLevel: DefconLevel;
  activeAlerts: number;
  offlineSensors: number;
  lastUpdate: string;
  globalDoseRate: number;
}
