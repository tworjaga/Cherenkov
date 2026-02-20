import { create } from 'zustand';
import type { Sensor, Alert, Facility, SystemStatus, EvacuationZone } from '@types';

interface DataState {
  sensors: Sensor[];
  alerts: Alert[];
  facilities: Facility[];
  evacuationZones: EvacuationZone[];
  systemStatus: SystemStatus | null;
  selectedSensorId: string | null;
  selectedFacilityId: string | null;
  isLoading: boolean;
  error: string | null;
  lastSync: Date | null;

  // Actions
  setSensors: (sensors: Sensor[]) => void;
  setAlerts: (alerts: Alert[]) => void;
  setFacilities: (facilities: Facility[]) => void;
  setEvacuationZones: (zones: EvacuationZone[]) => void;
  setSystemStatus: (status: SystemStatus | null) => void;
  selectSensor: (id: string | null) => void;
  selectFacility: (id: string | null) => void;
  acknowledgeAlert: (alertId: string) => void;
  addAlert: (alert: Alert) => void;
  updateSensorReading: (sensorId: string, reading: Sensor['lastReading']) => void;
  setLoading: (value: boolean) => void;
  setError: (error: string | null) => void;
  setLastSync: (date: Date) => void;
}

export const useDataStore = create<DataState>((set, get) => ({
  sensors: [],
  alerts: [],
  facilities: [],
  evacuationZones: [],
  systemStatus: null,
  selectedSensorId: null,
  selectedFacilityId: null,
  isLoading: false,
  error: null,
  lastSync: null,

  setSensors: (sensors) => set({ sensors }),
  setAlerts: (alerts) => set({ alerts }),
  setFacilities: (facilities) => set({ facilities }),
  setEvacuationZones: (zones) => set({ evacuationZones: zones }),
  setSystemStatus: (status) => set({ systemStatus: status }),
  selectSensor: (id) => set({ selectedSensorId: id }),
  selectFacility: (id) => set({ selectedFacilityId: id }),
  
  acknowledgeAlert: (alertId) => {
    const { alerts } = get();
    set({
      alerts: alerts.map(a => 
        a.id === alertId ? { ...a, acknowledged: true } : a
      )
    });
  },
  
  addAlert: (alert) => {
    const { alerts } = get();
    set({ alerts: [alert, ...alerts].slice(0, 100) });
  },
  
  updateSensorReading: (sensorId, reading) => {
    const { sensors } = get();
    set({
      sensors: sensors.map(s => 
        s.id === sensorId ? { ...s, lastReading: reading } : s
      )
    });
  },
  
  setLoading: (value) => set({ isLoading: value }),
  setError: (error) => set({ error }),
  setLastSync: (date) => set({ lastSync: date }),
}));
