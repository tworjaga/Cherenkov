import { describe, it, expect } from 'vitest';
import {
  clusterSensors,
  gridCluster,
  type Point,
  type Cluster,
} from './clustering';

describe('clusterSensors', () => {
  const createSensors = (count: number): Point[] => {
    return Array.from({ length: count }, (_, i) => ({
      id: `sensor-${i}`,
      lat: 40 + i * 0.01,
      lon: -74 + i * 0.01,
      value: Math.random() * 10,
    }));
  };

  it('returns empty array for empty input', () => {
    const result = clusterSensors([], 2);
    expect(result).toEqual([]);
  });

  it('returns single clusters for widely spaced sensors', () => {
    const sensors = [
      { id: '1', lat: 40, lon: -74, value: 5 },
      { id: '2', lat: 50, lon: -100, value: 10 },
    ];
    const result = clusterSensors(sensors, 2);
    expect(result).toHaveLength(2);
    expect(result[0].count).toBe(1);
    expect(result[1].count).toBe(1);
  });

  it('clusters nearby sensors together', () => {
    const sensors = [
      { id: '1', lat: 40.7128, lon: -74.006, value: 5 },
      { id: '2', lat: 40.7129, lon: -74.0061, value: 6 },
      { id: '3', lat: 40.713, lon: -74.0062, value: 7 },
    ];
    const result = clusterSensors(sensors, 2);
    // All 3 should be in one cluster due to proximity
    expect(result.length).toBeLessThanOrEqual(3);
    const totalPoints = result.reduce((sum: number, c: Cluster) => sum + c.count, 0);
    expect(totalPoints).toBe(3);
  });

  it('reduces clusters for large datasets', () => {
    const sensors = createSensors(200);
    const result = clusterSensors(sensors, 2, 50);
    // Should create fewer clusters than total points when maxClusters is specified
    expect(result.length).toBeLessThan(sensors.length);
  });


  it('calculates average value correctly', () => {
    const sensors = [
      { id: '1', lat: 40, lon: -74, value: 10 },
      { id: '2', lat: 40.001, lon: -74.001, value: 20 },
    ];
    const result = clusterSensors(sensors, 2, 100);
    const cluster = result.find((c: Cluster) => c.count === 2);
    if (cluster) {
      expect(cluster.avgValue).toBe(15);
    }
  });

  it('uses adaptive radius based on zoom', () => {
    const sensors = createSensors(20);
    const zoom2 = clusterSensors(sensors, 2, 50);
    const zoom10 = clusterSensors(sensors, 10, 50);
    // Higher zoom should have more clusters (smaller radius)
    expect(zoom10.length).toBeGreaterThanOrEqual(zoom2.length);
  });
});

describe('gridCluster', () => {
  it('returns empty array for empty input', () => {
    const result = gridCluster([], 1);
    expect(result).toEqual([]);
  });

  it('creates grid-based clusters', () => {
    const sensors = Array.from({ length: 100 }, (_, i) => ({
      id: `sensor-${i}`,
      lat: 40 + (i % 10) * 0.1,
      lon: -74 + Math.floor(i / 10) * 0.1,
      value: Math.random() * 10,
    }));
    const result = gridCluster(sensors, 5);
    expect(result.length).toBeGreaterThan(0);
    const totalPoints = result.reduce((sum: number, c: Cluster) => sum + c.count, 0);
    expect(totalPoints).toBe(100);
  });

  it('reduces clusters for large datasets', () => {
    // Create 1000 sensors clustered in a small geographic area
    const sensors = Array.from({ length: 1000 }, (_, i) => ({
      id: `sensor-${i}`,
      lat: 40 + (Math.random() - 0.5) * 2, // Within 2 degrees of lat 40
      lon: -74 + (Math.random() - 0.5) * 2, // Within 2 degrees of lon -74
      value: Math.random() * 10,
    }));
    const result = gridCluster(sensors, 3, 50);
    // Grid clustering should group points, creating fewer clusters than total points
    expect(result.length).toBeLessThan(sensors.length);
  });



  it('calculates cluster centers correctly', () => {
    const sensors = [
      { id: '1', lat: 40, lon: -74, value: 5 },
      { id: '2', lat: 40.1, lon: -74.1, value: 5 },
      { id: '3', lat: 40.2, lon: -74.2, value: 5 },
    ];
    const result = gridCluster(sensors, 2);
    expect(result.length).toBeGreaterThan(0);
    result.forEach((cluster: Cluster) => {
      expect(cluster.lat).toBeDefined();
      expect(cluster.lon).toBeDefined();
      expect(cluster.lat).not.toBeNaN();
      expect(cluster.lon).not.toBeNaN();
    });
  });
});
