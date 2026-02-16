import { useEffect } from 'react';
import { useAppStore, Alert, Sensor, GlobalStatus } from '../stores/useAppStore';

const generateMockSensors = (): Record<string, Sensor> => {
  const sensors: Record<string, Sensor> = {};
  const locations = [
    { lat: 51.5074, lon: -0.1278, name: 'London' },
    { lat: 40.7128, lon: -74.0060, name: 'New York' },
    { lat: 35.6762, lon: 139.6503, name: 'Tokyo' },
    { lat: 55.7558, lon: 37.6173, name: 'Moscow' },
    { lat: 48.8566, lon: 2.3522, name: 'Paris' },
    { lat: 39.9042, lon: 116.4074, name: 'Beijing' },
    { lat: -33.8688, lon: 151.2093, name: 'Sydney' },
    { lat: 19.4326, lon: -99.1332, name: 'Mexico City' },
    { lat: 28.6139, lon: 77.2090, name: 'New Delhi' },
    { lat: -23.5505, lon: -46.6333, name: 'Sao Paulo' },
    { lat: 52.5200, lon: 13.4050, name: 'Berlin' },
    { lat: 41.9028, lon: 12.4964, name: 'Rome' },
    { lat: 37.7749, lon: -122.4194, name: 'San Francisco' },
    { lat: 34.0522, lon: -118.2437, name: 'Los Angeles' },
    { lat: 43.6532, lon: -79.3832, name: 'Toronto' },
  ];

  locations.forEach((loc, i) => {
    const id = `sensor-${i + 1}`;
    const baseDose = Math.random() * 2 + 0.1;
    sensors[id] = {
      id,
      location: { lat: loc.lat, lon: loc.lon },
      doseRate: baseDose,
      unit: 'μSv/h',
      lastReading: new Date(),
      status: Math.random() > 0.9 ? 'error' : 'active',
    };
  });

  return sensors;
};

const generateMockAlerts = (): Alert[] => {
  return [
    {
      id: 'alert-1',
      severity: 'CRITICAL',
      type: 'ANOMALY',
      title: 'Radiation Spike Detected',
      description: 'Sensor in Tokyo showing 5x normal levels',
      location: { lat: 35.6762, lon: 139.6503 },
      timestamp: new Date(Date.now() - 1000 * 60 * 5),
      acknowledged: false,
      sensorId: 'sensor-3',
    },
    {
      id: 'alert-2',
      severity: 'HIGH',
      type: 'FACILITY_STATUS',
      title: 'Facility Alert',
      description: 'Fukushima Daiichi monitoring elevated',
      location: { lat: 37.4213, lon: 141.0326 },
      timestamp: new Date(Date.now() - 1000 * 60 * 30),
      acknowledged: false,
    },
    {
      id: 'alert-3',
      severity: 'MEDIUM',
      type: 'SEISMIC',
      title: 'Seismic Event',
      description: 'Magnitude 4.2 near monitoring station',
      location: { lat: 35.0, lon: 138.0 },
      timestamp: new Date(Date.now() - 1000 * 60 * 60),
      acknowledged: true,
    },
    {
      id: 'alert-4',
      severity: 'LOW',
      type: 'ANOMALY',
      title: 'Calibration Drift',
      description: 'Sensor calibration requires attention',
      location: { lat: 51.5074, lon: -0.1278 },
      timestamp: new Date(Date.now() - 1000 * 60 * 60 * 2),
      acknowledged: false,
      sensorId: 'sensor-1',
    },
  ];
};

const generateMockStatus = (): GlobalStatus => ({
  level: 'ELEVATED',
  defcon: 4,
  lastUpdate: new Date(),
  activeAlerts: 3,
});

export const useMockData = () => {
  const setSensors = useAppStore((state) => state.setSensors);
  const setGlobalStatus = useAppStore((state) => state.setGlobalStatus);
  const addAlert = useAppStore((state) => state.addAlert);

  useEffect(() => {
    // Initialize with mock data
    setSensors(generateMockSensors());
    setGlobalStatus(generateMockStatus());
    
    const alerts = generateMockAlerts();
    alerts.forEach((alert) => addAlert(alert));

    // Simulate live updates
    const interval = setInterval(() => {
      const sensors = generateMockSensors();
      Object.keys(sensors).forEach((id) => {
        sensors[id].doseRate += (Math.random() - 0.5) * 0.1;
        sensors[id].lastReading = new Date();
      });
      setSensors(sensors);
    }, 5000);

    return () => clearInterval(interval);
  }, [setSensors, setGlobalStatus, addAlert]);
};
