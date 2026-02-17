import { describe, it, expect } from 'vitest';
import { isValidEmail, isValidPassword, isValidCoordinate, isValidSensorId } from './validators';

describe('validators', () => {
  describe('isValidEmail', () => {
    it('returns true for valid email addresses', () => {
      expect(isValidEmail('user@example.com')).toBe(true);
      expect(isValidEmail('test.user@domain.co.uk')).toBe(true);
    });

    it('returns false for invalid email addresses', () => {
      expect(isValidEmail('invalid')).toBe(false);
      expect(isValidEmail('@example.com')).toBe(false);
      expect(isValidEmail('user@')).toBe(false);
    });
  });

  describe('isValidPassword', () => {
    it('returns true for valid passwords', () => {
      expect(isValidPassword('SecurePass123!')).toBe(true);
      expect(isValidPassword('MyP@ssw0rd')).toBe(true);
    });

    it('returns false for weak passwords', () => {
      expect(isValidPassword('short')).toBe(false);
      expect(isValidPassword('password')).toBe(false);
    });
  });

  describe('isValidCoordinate', () => {
    it('returns true for valid coordinates', () => {
      expect(isValidCoordinate(40.7128, -74.006)).toBe(true);
      expect(isValidCoordinate(0, 0)).toBe(true);
    });

    it('returns false for out of range coordinates', () => {
      expect(isValidCoordinate(91, 0)).toBe(false);
      expect(isValidCoordinate(0, 181)).toBe(false);
    });
  });

  describe('isValidSensorId', () => {
    it('returns true for valid UUID format', () => {
      expect(isValidSensorId('550e8400-e29b-41d4-a716-446655440000')).toBe(true);
    });

    it('returns false for invalid format', () => {
      expect(isValidSensorId('invalid-id')).toBe(false);
    });
  });
});
