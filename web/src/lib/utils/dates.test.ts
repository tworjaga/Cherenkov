import { describe, it, expect } from 'vitest';
import {
  toISOString,
  fromISOString,
  formatDate,
  formatTime,
  formatDateTime,
  getRelativeTime,
  addTime,
  startOfDay,
  endOfDay,
  isSameDay,
  isToday,
  isPast,
  isFuture,
  getDurationMs,
  getDurationString,
  clampDate,
} from './dates';

describe('dates', () => {
  describe('toISOString', () => {
    it('formats date to ISO string without milliseconds', () => {
      const date = new Date('2024-01-15T10:30:00.123Z');
      expect(toISOString(date)).toBe('2024-01-15T10:30:00Z');
    });
  });

  describe('fromISOString', () => {
    it('parses ISO string to Date', () => {
      const result = fromISOString('2024-01-15T10:30:00Z');
      expect(result.toISOString()).toBe('2024-01-15T10:30:00.000Z');
    });
  });

  describe('formatDate', () => {
    it('formats date to display string', () => {
      const date = new Date('2024-01-15T10:30:00Z');
      const result = formatDate(date);
      expect(result).toContain('Jan');
      expect(result).toContain('2024');
    });
  });

  describe('formatTime', () => {
    it('formats time to display string', () => {
      const date = new Date('2024-01-15T10:30:00Z');
      const result = formatTime(date);
      expect(result).toMatch(/\d{2}:\d{2}:\d{2}/);
    });
  });

  describe('formatDateTime', () => {
    it('formats date and time together', () => {
      const date = new Date('2024-01-15T10:30:00Z');
      const result = formatDateTime(date);
      expect(result).toContain('Jan');
      expect(result).toMatch(/\d{2}:\d{2}:\d{2}/);
    });
  });

  describe('getRelativeTime', () => {
    it('returns "just now" for recent times', () => {
      const now = new Date();
      expect(getRelativeTime(now)).toBe('just now');
    });

    it('returns minutes ago for recent times', () => {
      const fiveMinutesAgo = new Date(Date.now() - 5 * 60 * 1000);
      expect(getRelativeTime(fiveMinutesAgo)).toBe('5m ago');
    });
  });

  describe('addTime', () => {
    it('adds days to date', () => {
      const date = new Date('2024-01-15T10:30:00Z');
      const result = addTime(date, 5, 'd');
      expect(result.getUTCDate()).toBe(20);
    });

    it('adds hours to date', () => {
      const date = new Date('2024-01-15T10:30:00Z');
      const result = addTime(date, 5, 'h');
      expect(result.getUTCHours()).toBe(15);
    });
  });


  describe('startOfDay', () => {
    it('returns start of day', () => {
      const date = new Date('2024-01-15T10:30:45Z');
      const result = startOfDay(date);
      expect(result.getHours()).toBe(0);
      expect(result.getMinutes()).toBe(0);
      expect(result.getSeconds()).toBe(0);
    });
  });

  describe('endOfDay', () => {
    it('returns end of day', () => {
      const date = new Date('2024-01-15T10:30:00Z');
      const result = endOfDay(date);
      expect(result.getHours()).toBe(23);
      expect(result.getMinutes()).toBe(59);
      expect(result.getSeconds()).toBe(59);
    });
  });


  describe('isSameDay', () => {
    it('returns true for same day', () => {
      const a = new Date('2024-01-15T10:30:00Z');
      const b = new Date('2024-01-15T20:00:00Z');
      expect(isSameDay(a, b)).toBe(true);
    });

    it('returns false for different days', () => {
      const a = new Date('2024-01-15T10:30:00Z');
      const b = new Date('2024-01-16T10:30:00Z');
      expect(isSameDay(a, b)).toBe(false);
    });
  });

  describe('isToday', () => {
    it('returns true for today', () => {
      expect(isToday(new Date())).toBe(true);
    });

    it('returns false for yesterday', () => {
      const yesterday = new Date(Date.now() - 24 * 60 * 60 * 1000);
      expect(isToday(yesterday)).toBe(false);
    });
  });

  describe('isPast', () => {
    it('returns true for past dates', () => {
      const past = new Date(Date.now() - 1000);
      expect(isPast(past)).toBe(true);
    });

    it('returns false for future dates', () => {
      const future = new Date(Date.now() + 1000);
      expect(isPast(future)).toBe(false);
    });
  });

  describe('isFuture', () => {
    it('returns true for future dates', () => {
      const future = new Date(Date.now() + 1000);
      expect(isFuture(future)).toBe(true);
    });

    it('returns false for past dates', () => {
      const past = new Date(Date.now() - 1000);
      expect(isFuture(past)).toBe(false);
    });
  });

  describe('getDurationMs', () => {
    it('calculates duration in milliseconds', () => {
      const start = new Date('2024-01-15T10:00:00Z');
      const end = new Date('2024-01-15T10:00:05Z');
      expect(getDurationMs(start, end)).toBe(5000);
    });
  });

  describe('getDurationString', () => {
    it('formats duration as string', () => {
      const start = new Date('2024-01-15T10:00:00Z');
      const end = new Date('2024-01-15T10:05:30Z');
      expect(getDurationString(start, end)).toBe('5m 30s');
    });

    it('formats hours duration', () => {
      const start = new Date('2024-01-15T10:00:00Z');
      const end = new Date('2024-01-15T12:30:00Z');
      expect(getDurationString(start, end)).toBe('2h 30m');
    });
  });

  describe('clampDate', () => {
    it('clamps date to range', () => {
      const min = new Date('2024-01-15T00:00:00Z');
      const max = new Date('2024-01-20T00:00:00Z');
      const date = new Date('2024-01-25T00:00:00Z');
      const result = clampDate(date, min, max);
      expect(result.getTime()).toBe(max.getTime());
    });
  });
});
