import { describe, it, expect } from 'vitest';
import { GraphQLClient } from 'graphql-request';

const client = new GraphQLClient('http://localhost:8080/graphql');

interface Sensor {
  id: string;
  name: string;
  latitude: number;
  longitude: number;
  status: string;
}

interface Facility {
  id: string;
  name: string;
  facilityType: string;
  latitude: number;
  longitude: number;
  status: string;
}

interface Anomaly {
  id: string;
  sensorId: string;
  severity: string;
  zScore: number;
  detectedAt: string;
}

interface GlobalStatus {
  level: number;
  defcon: number;
  status: string;
  activeAlerts: number;
  activeSensors: number;
}

interface SensorsResponse {
  sensors: Sensor[];
}

interface FacilitiesResponse {
  facilities: Facility[];
}

interface AnomaliesResponse {
  anomalies: Anomaly[];
}

interface GlobalStatusResponse {
  globalStatus: GlobalStatus;
}

describe('GraphQL API Integration', () => {
  describe('sensors query', () => {
    it('should return all sensors with required fields', async () => {
      const query = `
        query {
          sensors {
            id
            name
            latitude
            longitude
            status
          }
        }
      `;
      
      const data = await client.request(query) as SensorsResponse;
      
      expect(data.sensors).toBeDefined();
      expect(Array.isArray(data.sensors)).toBe(true);
      expect(data.sensors.length).toBeGreaterThan(0);
      
      const sensor = data.sensors[0];
      expect(sensor.id).toBeDefined();
      expect(sensor.name).toBeDefined();
      expect(typeof sensor.latitude).toBe('number');
      expect(typeof sensor.longitude).toBe('number');
      expect(['active', 'warning', 'critical', 'offline']).toContain(sensor.status);
    });
  });

  describe('facilities query', () => {
    it('should return all facilities with required fields', async () => {
      const query = `
        query {
          facilities {
            id
            name
            facilityType
            latitude
            longitude
            status
          }
        }
      `;
      
      const data = await client.request(query) as FacilitiesResponse;
      
      expect(data.facilities).toBeDefined();
      expect(Array.isArray(data.facilities)).toBe(true);
      expect(data.facilities.length).toBeGreaterThan(0);
      
      const facility = data.facilities[0];
      expect(facility.id).toBeDefined();
      expect(facility.name).toBeDefined();
      expect(typeof facility.latitude).toBe('number');
      expect(typeof facility.longitude).toBe('number');
    });
  });

  describe('anomalies query', () => {
    it('should return anomalies with required fields', async () => {
      const query = `
        query {
          anomalies {
            id
            sensorId
            severity
            zScore
            detectedAt
          }
        }
      `;
      
      const data = await client.request(query) as AnomaliesResponse;
      
      expect(data.anomalies).toBeDefined();
      expect(Array.isArray(data.anomalies)).toBe(true);
      
      if (data.anomalies.length > 0) {
        const anomaly = data.anomalies[0];
        expect(anomaly.id).toBeDefined();
        expect(anomaly.sensorId).toBeDefined();
        expect(['low', 'medium', 'high', 'critical']).toContain(anomaly.severity);
        expect(typeof anomaly.zScore).toBe('number');
      }
    });
  });

  describe('globalStatus query', () => {
    it('should return global status with required fields', async () => {
      const query = `
        query {
          globalStatus {
            level
            defcon
            status
            activeAlerts
            activeSensors
          }
        }
      `;
      
      const data = await client.request(query) as GlobalStatusResponse;
      
      expect(data.globalStatus).toBeDefined();
      expect(typeof data.globalStatus.level).toBe('number');
      expect(typeof data.globalStatus.defcon).toBe('number');
      expect(typeof data.globalStatus.status).toBe('string');
      expect(typeof data.globalStatus.activeAlerts).toBe('number');
      expect(typeof data.globalStatus.activeSensors).toBe('number');
    });
  });
});
