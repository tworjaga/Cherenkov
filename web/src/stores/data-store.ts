import { create } from 'zustand';
import { Sensor, Anomaly, Facility, Alert, GlobalStatus, Reading, TimeSeriesPoint } from '@/types';


interface DataState {
  sensors: Sensor[];
  anomalies: Anomaly[];
  facilities: Facility[];
  alerts: Alert[];
  globalStatus: GlobalStatus | null;
  recentReadings: Map<string, Reading>;
  readings: Record<string, Reading[]>;
  globalTimeSeries: TimeSeriesPoint[];
  unreadAlertCount: number;



  setSensors: (sensors: Sensor[]) => void;
  updateSensor: (sensor: Sensor) => void;
  setAnomalies: (anomalies: Anomaly[]) => void;
  addAnomaly: (anomaly: Anomaly) => void;
  setFacilities: (facilities: Facility[]) => void;
  setAlerts: (alerts: Alert[]) => void;
  addAlert: (alert: Alert) => void;
  acknowledgeAlert: (id: string) => void;
  setGlobalStatus: (status: GlobalStatus) => void;
  setGlobalTimeSeries: (data: TimeSeriesPoint[]) => void;
  updateReading: (sensorId: string, reading: Reading) => void;

  addHistoricalReading: (sensorId: string, reading: Reading) => void;
  getSensorById: (id: string) => Sensor | undefined;

  getFacilityById: (id: string) => Facility | undefined;
  getAnomalyById: (id: string) => Anomaly | undefined;
  getUnreadAlerts: () => Alert[];
  getAlertsBySeverity: (severity: Alert['severity']) => Alert[];
}

const defaultGlobalStatus: GlobalStatus = {
  level: 0,
  defcon: 5,
  status: 'NORMAL',
  activeAlerts: 0,
  activeSensors: 0,
  lastUpdate: Date.now(),
};

export const useDataStore = create<DataState>()((set, get) => ({
  sensors: [],
  anomalies: [],
  facilities: [],
  alerts: [],
  globalStatus: defaultGlobalStatus,
  recentReadings: new Map(),
  readings: {},
  globalTimeSeries: [],
  unreadAlertCount: 0,



  setSensors: (sensors) => set({ sensors }),

  updateSensor: (sensor) =>
    set((state) => ({
      sensors: state.sensors.map((s) => (s.id === sensor.id ? sensor : s)),
    })),

  setAnomalies: (anomalies) => set({ anomalies }),

  addAnomaly: (anomaly) =>
    set((state) => ({
      anomalies: [anomaly, ...state.anomalies],
      alerts: [
        {
          id: anomaly.id,
          type: 'anomaly',
          severity: anomaly.severity,
          message: anomaly.message,
          timestamp: anomaly.detectedAt,
          location: anomaly.location,
          metadata: {
            sensorId: anomaly.sensorId,
            zScore: anomaly.zScore,
            doseRate: anomaly.doseRate,
            baseline: anomaly.baseline,
          },
          acknowledged: false,
        },
        ...state.alerts,
      ],
      unreadAlertCount: state.unreadAlertCount + 1,
    })),

  setFacilities: (facilities) => set({ facilities }),

  setAlerts: (alerts) =>
    set({
      alerts,
      unreadAlertCount: alerts.filter((a) => !a.acknowledged).length,
    }),

  addAlert: (alert) =>
    set((state) => ({
      alerts: [alert, ...state.alerts],
      unreadAlertCount: state.unreadAlertCount + 1,
    })),

  acknowledgeAlert: (id) =>
    set((state) => {
      const updatedAlerts = state.alerts.map((alert) =>
        alert.id === id ? { ...alert, acknowledged: true, acknowledgedAt: Date.now() } : alert
      );
      return {
        alerts: updatedAlerts,
        unreadAlertCount: updatedAlerts.filter((a) => !a.acknowledged).length,
      };
    }),

  setGlobalStatus: (status) => set({ globalStatus: status }),

  setGlobalTimeSeries: (data) => set({ globalTimeSeries: data }),

  updateReading: (sensorId, reading) =>

    set((state) => {
      const newReadings = new Map(state.recentReadings);
      newReadings.set(sensorId, reading);
      return { recentReadings: newReadings };
    }),

  addHistoricalReading: (sensorId, reading) =>
    set((state) => {
      const currentReadings = state.readings[sensorId] || [];
      return {
        readings: {
          ...state.readings,
          [sensorId]: [...currentReadings, reading].slice(-100),
        },
      };
    }),


  getSensorById: (id) => get().sensors.find((s) => s.id === id),

  getFacilityById: (id) => get().facilities.find((f) => f.id === id),

  getAnomalyById: (id) => get().anomalies.find((a) => a.id === id),

  getUnreadAlerts: () => get().alerts.filter((a) => !a.acknowledged),

  getAlertsBySeverity: (severity) => get().alerts.filter((a) => a.severity === severity),
}));
