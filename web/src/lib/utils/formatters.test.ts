import { describe, it, expect } from 'vitest';
import {
  formatDoseRate,
  formatTimestamp,
  formatCoordinate,
  formatDuration,
} from './formatters';

describe('formatDoseRate', () => {
  it('formats dose rate with default unit', () => {
    expect(formatDoseRate(1.5)).toBe('1.50 μSv/h');
  });

  it('formats dose rate with custom unit', () => {
    expect(formatDoseRate(1.5, 'mSv/h')).toBe('1.50 mSv/h');
  });

  it('handles zero dose rate', () => {
    expect(formatDoseRate(0)).toBe('0.00 μSv/h');
  });

  it('handles large numbers', () => {
    expect(formatDoseRate(1000)).toBe('1000.00 μSv/h');
  });
});

describe('formatTimestamp', () => {
  it('formats timestamp to ISO string', () => {
    const timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
    const result = formatTimestamp(timestamp);
    expect(result).toContain('2021');
  });

  it('handles current timestamp', () => {
    const now = Math.floor(Date.now() / 1000);
    const result = formatTimestamp(now);
    expect(result).toBeDefined();
  });
});

describe('formatCoordinate', () => {
  it('formats latitude and longitude', () => {
    expect(formatCoordinate(35.6762, 139.6503)).toBe('35.68, 139.65');
  });

  it('handles negative coordinates', () => {
    expect(formatCoordinate(-35.6762, -139.6503)).toBe('-35.68, -139.65');
  });

  it('handles zero coordinates', () => {
    expect(formatCoordinate(0, 0)).toBe('0.00, 0.00');
  });
});


describe('formatDuration', () => {
  it('formats seconds', () => {
    expect(formatDuration(30)).toBe('30s');
  });

  it('formats minutes', () => {
    expect(formatDuration(120)).toBe('2m 0s');
  });

  it('formats hours', () => {
    expect(formatDuration(3661)).toBe('1h 1m 1s');
  });

  it('handles zero', () => {
    expect(formatDuration(0)).toBe('0s');
  });
});
