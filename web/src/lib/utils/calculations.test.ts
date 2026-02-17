import { describe, it, expect } from 'vitest';
import {
  calculateDistance,
  calculateBearing,
  interpolatePosition,
  calculateBoundingBox,
  calculateCentroid,
} from './calculations';

describe('calculations', () => {
  describe('calculateDistance', () => {
    it('calculates distance between two points', () => {
      const d = calculateDistance(0, 0, 0, 1);
      expect(d).toBeGreaterThan(0);
      expect(d).toBeLessThan(120000);
    });

    it('returns 0 for same point', () => {
      expect(calculateDistance(40, -74, 40, -74)).toBe(0);
    });
  });

  describe('calculateBearing', () => {
    it('calculates bearing between two points', () => {
      const bearing = calculateBearing(0, 0, 1, 0);
      expect(bearing).toBeGreaterThanOrEqual(0);
      expect(bearing).toBeLessThan(360);
    });
  });

  describe('interpolatePosition', () => {
    it('interpolates between two positions', () => {
      const pos = interpolatePosition({ lat: 0, lon: 0 }, { lat: 1, lon: 1 }, 0.5);
      expect(pos.lat).toBeGreaterThan(0);
      expect(pos.lat).toBeLessThan(1);
      expect(pos.lon).toBeGreaterThan(0);
      expect(pos.lon).toBeLessThan(1);
    });
  });


  describe('calculateBoundingBox', () => {
    it('calculates bounding box for points', () => {
      const points = [{ lat: 0, lon: 0 }, { lat: 1, lon: 1 }];
      const bbox = calculateBoundingBox(points);
      expect(bbox.minLat).toBe(0);
      expect(bbox.maxLat).toBe(1);
      expect(bbox.minLon).toBe(0);
      expect(bbox.maxLon).toBe(1);
    });
  });

  describe('calculateCentroid', () => {
    it('calculates centroid of points', () => {
      const points = [{ lat: 0, lon: 0 }, { lat: 2, lon: 2 }];
      const centroid = calculateCentroid(points);
      expect(centroid.lat).toBe(1);
      expect(centroid.lon).toBe(1);
    });
  });
});
