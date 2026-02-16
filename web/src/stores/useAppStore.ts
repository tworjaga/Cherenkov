import { create } from 'zustand';

export type View = 'DASHBOARD' | 'GLOBE' | 'SENSORS' | 'ANOMALIES' | 'PLUME' | 'SETTINGS';

export interface TimeControl {
  mode: 'LIVE' | 'PAUSED' | 'REPLAY';
  currentTime: Date;
  windowStart: Date;
  windowEnd: Date;
  playbackSpeed: 1 | 2 | 5 | 10 | 50;
}

export interface GlobeViewport {
  latitude: number;
  longitude: number;
  zoom: number;
  pitch: number;
  bearing: number;
}

export interface Alert {
  id: string;
  severity: 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW';
  type: 'ANOMALY' | 'FACILITY_STATUS' | 'SEISMIC' | 'PLUME';
  title: string;
  description: string;
  location: { lat: number; lon: number };
  timestamp: Date;
  acknowledged: boolean;
  sensorId?: string;
  facilityId?: string;
}

export interface Sensor {
  id: string;
  location: { lat: number; lon: number };
  doseRate: number;
  unit: string;
  lastReading: Date;
  status: 'active' | 'inactive' | 'error';
}

export interface GlobalStatus {
  level: 'NORMAL' | 'ELEVATED' | 'HIGH' | 'CRITICAL';
  defcon: 5 | 4 | 3 | 2 | 1;
  lastUpdate: Date;
  activeAlerts: number;
}

interface AppState {
  view: View;
  globe: {
    viewport: GlobeViewport;
    layers: Record<string, boolean>;
    time: TimeControl;
    selectedSensor: string | null;
    selectedFacility: string | null;
  };
  sensors: Record<string, Sensor>;
  alerts: Alert[];
  globalStatus: GlobalStatus | null;
  sidebarCollapsed: boolean;
  rightPanelOpen: boolean;
  bottomPanelOpen: boolean;
  theme: 'DARK' | 'LIGHT';
  connection: 'CONNECTED' | 'CONNECTING' | 'DISCONNECTED' | 'ERROR';
  lastPing: Date | null;


  setView: (view: View) => void;
  setGlobeViewport: (viewport: Partial<GlobeViewport>) => void;
  toggleLayer: (layer: string) => void;
  setTimeControl: (time: Partial<TimeControl>) => void;
  selectSensor: (sensorId: string | null) => void;
  selectFacility: (facilityId: string | null) => void;
  setSensors: (sensors: Record<string, Sensor>) => void;
  addAlert: (alert: Alert) => void;
  acknowledgeAlert: (alertId: string) => void;
  setGlobalStatus: (status: GlobalStatus) => void;
  toggleSidebar: () => void;
  toggleRightPanel: () => void;
  toggleBottomPanel: () => void;
  setTheme: (theme: 'DARK' | 'LIGHT') => void;
  setConnection: (connection: AppState['connection']) => void;
  setLastPing: (date: Date) => void;

}

const initialTimeControl: TimeControl = {
  mode: 'LIVE',
  currentTime: new Date(),
  windowStart: new Date(Date.now() - 24 * 60 * 60 * 1000),
  windowEnd: new Date(),
  playbackSpeed: 1,
};

const initialViewport: GlobeViewport = {
  latitude: 20,
  longitude: 0,
  zoom: 2,
  pitch: 30,
  bearing: 0,
};

export const useAppStore = create<AppState>((set) => ({
  view: 'GLOBE',
  globe: {
    viewport: initialViewport,
    layers: {
      'sensor-heatmap': true,
      'sensor-points': false,
      'facilities': true,
      'plume-simulation': false,
      'anomalies': true,
      'seismic': false,
    },
    time: initialTimeControl,
    selectedSensor: null,
    selectedFacility: null,
  },
  sensors: {},
  alerts: [],
  globalStatus: null,
  sidebarCollapsed: false,
  rightPanelOpen: true,
  bottomPanelOpen: true,
  theme: 'DARK',
  connection: 'DISCONNECTED',

  lastPing: null,

  setView: (view) => set({ view }),
  
  setGlobeViewport: (viewport) => set((state) => ({
    globe: {
      ...state.globe,
      viewport: { ...state.globe.viewport, ...viewport },
    },
  })),
  
  toggleLayer: (layer) => set((state) => ({
    globe: {
      ...state.globe,
      layers: {
        ...state.globe.layers,
        [layer]: !state.globe.layers[layer],
      },
    },
  })),
  
  setTimeControl: (time) => set((state) => ({
    globe: {
      ...state.globe,
      time: { ...state.globe.time, ...time },
    },
  })),
  
  selectSensor: (sensorId) => set((state) => ({
    globe: {
      ...state.globe,
      selectedSensor: sensorId,
      selectedFacility: sensorId ? null : state.globe.selectedFacility,
    },
  })),
  
  selectFacility: (facilityId) => set((state) => ({
    globe: {
      ...state.globe,
      selectedFacility: facilityId,
      selectedSensor: facilityId ? null : state.globe.selectedSensor,
    },
  })),
  
  setSensors: (sensors) => set({ sensors }),
  
  addAlert: (alert) => set((state) => ({
    alerts: [alert, ...state.alerts].slice(0, 100),
  })),
  
  acknowledgeAlert: (alertId) => set((state) => ({
    alerts: state.alerts.map((a) =>
      a.id === alertId ? { ...a, acknowledged: true } : a
    ),
  })),
  
  setGlobalStatus: (status) => set({ globalStatus: status }),
  
  toggleSidebar: () => set((state) => ({
    sidebarCollapsed: !state.sidebarCollapsed,
  })),
  
  toggleRightPanel: () => set((state) => ({
    rightPanelOpen: !state.rightPanelOpen,
  })),
  
  toggleBottomPanel: () => set((state) => ({
    bottomPanelOpen: !state.bottomPanelOpen,
  })),
  
  setTheme: (theme) => set({ theme }),
  
  setConnection: (connection) => set({ connection }),

  
  setLastPing: (date) => set({ lastPing: date }),
}));
