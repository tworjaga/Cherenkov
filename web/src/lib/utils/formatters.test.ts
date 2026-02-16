import { describe, it, expect } from 'vitest';
import { formatDoseRate, formatTimestamp, formatCoordinate, formatDuration } from './formatters';

describe('formatDoseRate', () => {
  it('formats dose rate with default unit', () => {
    expect(formatDoseRate(1.5)).toBe('1.500 μSv/h');
  });

  it('formats dose rate with custom unit', () => {
    expect(formatDoseRate(1.5, 'mSv/h')).toBe('1.500 mSv/h');
  });

  it('handles zero dose rate', () => {
    expect(formatDoseRate(0)).toBe('0.000 μSv/h');
  });

  it('handles large numbers', () => {
    expect(formatDoseRate(1000)).toBe('1000.000 μSv/h');
  });
});

describe('formatTimestamp', () => {
  it('formats timestamp to readable string', () => {
    const timestamp = 1609459200;
    const result = formatTimestamp(timestamp);
    expect(result).toBeDefined();
    expect(result.length).toBeGreaterThan(0);
  });

  it('handles current timestamp', () => {
    const now = Math.floor(Date.now() / 1000);
    const result = formatTimestamp(now);
    expect(result).toBeDefined();
    expect(result.length).toBeGreaterThan(0);
  });
});

describe('formatCoordinate', () => {
  it('formats latitude and longitude with degrees', () => {
    expect(formatCoordinate(35.6762, 139.6503)).toBe('35.6762°N, 139.6503°E');
  });

  it('handles negative coordinates as South/West', () => {
    expect(formatCoordinate(-35.6762, -139.6503)).toBe('35.6762°S, 139.6503°W');
  });

  it('handles zero coordinates', () => {
    expect(formatCoordinate(0, 0)).toBe('0.0000°N, 0.0000°E');
  });
});

describe('formatDuration', () => {
  it('formats seconds', () => {
    expect(formatDuration(30000)).toBe('30s');
  });

  it('formats minutes', () => {
    expect(formatDuration(120000)).toBe('2m 0s');
  });


  it('formats hours', () => {
    expect(formatDuration(3661000)).toBe('1h 1m');
  });

  it('handles zero', () => {
    expect(formatDuration(0)).toBe('0s');
  });
});
