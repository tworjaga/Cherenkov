import { describe, it, expect, beforeEach } from 'vitest';
import { useDataStore } from '../../../src/stores/data-store';
import type { Sensor, GlobalStatus, Alert } from '../../../src/types/models';

describe('Data Store Integration', () => {
  beforeEach(() => {
    // Reset store state
    const store = useDataStore.getState();
    store.setSensors([]);
    store.setAnomalies([]);
    store.setFacilities([]);
    store.setAlerts([]);
  });

  it('should initialize with empty state', () => {
    const state = useDataStore.getState();
    expect(state.sensors).toEqual([]);
    expect(state.anomalies).toEqual([]);
    expect(state.facilities).toEqual([]);
    expect(state.alerts).toEqual([]);
    expect(state.globalStatus).toBeDefined();
    expect(state.globalStatus?.defcon).toBe(5);
  });


  it('should update sensors', () => {
    const store = useDataStore.getState();
    const mockSensors: Sensor[] = [
      {
        id: 'sensor-1',
        name: 'Test Sensor',
        location: { lat: 51.5074, lon: -0.1278 },
        longitude: -0.1278,
        latitude: 51.5074,
        status: 'active',
        source: 'test',
        type: 'gamma',
        unit: 'uSv/h',
        lastReading: null
      }
    ];
    
    store.setSensors(mockSensors);
    expect(useDataStore.getState().sensors).toEqual(mockSensors);
  });

  it('should update global status', () => {
    const store = useDataStore.getState();
    const mockStatus: GlobalStatus = {
      level: 3,
      defcon: 3,
      status: 'ELEVATED',
      activeAlerts: 5,
      activeSensors: 150,
      lastUpdate: Date.now()
    };
    
    store.setGlobalStatus(mockStatus);
    expect(useDataStore.getState().globalStatus).toEqual(mockStatus);
  });

  it('should add alerts', () => {
    const store = useDataStore.getState();
    const mockAlert: Alert = {
      id: 'alert-1',
      type: 'anomaly',
      severity: 'high',
      message: 'Test alert',
      timestamp: Date.now(),
      acknowledged: false
    };
    
    store.addAlert(mockAlert);
    expect(useDataStore.getState().alerts).toContainEqual(mockAlert);
  });
});
