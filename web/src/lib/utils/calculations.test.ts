import { describe, it, expect } from 'vitest';
import {
  calculateDistance,
  calculateBearing,
  getSeverityColor,
  getDefconColor,
  lerp,
} from './calculations';

describe('calculateDistance', () => {
  it('calculates distance between two points correctly', () => {
    const distance = calculateDistance(0, 0, 0, 1);
    expect(distance).toBeGreaterThan(110000);
    expect(distance).toBeLessThan(111000);
  });

  it('returns 0 for same point', () => {
    const distance = calculateDistance(10, 20, 10, 20);
    expect(distance).toBe(0);
  });
});

describe('calculateBearing', () => {
  it('calculates north bearing correctly', () => {
    const bearing = calculateBearing(0, 0, 1, 0);
    expect(bearing).toBeCloseTo(0, 1);
  });

  it('calculates east bearing correctly', () => {
    const bearing = calculateBearing(0, 0, 0, 1);
    expect(bearing).toBeCloseTo(90, 1);
  });
});

describe('getSeverityColor', () => {
  it('returns normal color for low severity', () => {
    expect(getSeverityColor('normal')).toBe('#00ff88');
  });

  it('returns critical color for critical severity', () => {
    expect(getSeverityColor('critical')).toBe('#ff3366');
  });

  it('returns default color for unknown severity', () => {
    expect(getSeverityColor('unknown')).toBe('#00d4ff');
  });
});

describe('getDefconColor', () => {
  it('returns correct colors for all DEFCON levels', () => {
    expect(getDefconColor(5)).toBe('#00ff88');
    expect(getDefconColor(4)).toBe('#00d4ff');
    expect(getDefconColor(3)).toBe('#ffb800');
    expect(getDefconColor(2)).toBe('#ff6b35');
    expect(getDefconColor(1)).toBe('#ff3366');
  });
});

describe('lerp', () => {
  it('interpolates correctly', () => {
    expect(lerp(0, 10, 0)).toBe(0);
    expect(lerp(0, 10, 1)).toBe(10);
    expect(lerp(0, 10, 0.5)).toBe(5);
  });
});
