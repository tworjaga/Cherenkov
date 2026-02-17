/**
 * Settings type definitions
 * TypeScript interfaces for application settings
 */

export interface UserSettings {
  id: string;
  userId: string;
  theme: 'light' | 'dark' | 'system';
  language: string;
  timezone: string;
  dateFormat: string;
  timeFormat: '12h' | '24h';
  notifications: NotificationSettings;
  dashboard: DashboardSettings;
  globe: GlobeSettings;
  createdAt: string;
  updatedAt: string;
}

export interface NotificationSettings {
  email: boolean;
  push: boolean;
  sms: boolean;
  desktop: boolean;
  alertThreshold: 'all' | 'warning' | 'critical' | 'none';
  quietHours: {
    enabled: boolean;
    start: string;
    end: string;
  };
}

export interface DashboardSettings {
  defaultView: 'overview' | 'sensors' | 'anomalies' | 'globe';
  refreshInterval: number;
  showWelcomeMessage: boolean;
  compactMode: boolean;
  widgets: WidgetConfig[];
}

export interface WidgetConfig {
  id: string;
  type: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  config: Record<string, unknown>;
}

export interface GlobeSettings {
  defaultZoom: number;
  defaultCenter: { lat: number; lng: number };
  showLabels: boolean;
  showBorders: boolean;
  terrainEnabled: boolean;
  atmosphereEnabled: boolean;
  layers: {
    sensors: boolean;
    facilities: boolean;
    anomalies: boolean;
    plumes: boolean;
    heatmap: boolean;
  };
}

export interface DataSourceConfig {
  id: string;
  name: string;
  type: 'sensor' | 'api' | 'file' | 'stream';
  url: string;
  authType: 'none' | 'basic' | 'bearer' | 'apikey';
  authConfig?: Record<string, string>;
  refreshInterval: number;
  enabled: boolean;
  lastSync?: string;
  status: 'connected' | 'disconnected' | 'error';
}

export interface ApiKey {
  id: string;
  name: string;
  key: string;
  scopes: string[];
  createdAt: string;
  expiresAt?: string;
  lastUsed?: string;
  enabled: boolean;
}
