/**
 * Validation utilities
 */

export const isValidEmail = (email: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
};

export const isValidLatitude = (lat: number): boolean => {
  return lat >= -90 && lat <= 90;
};

export const isValidLongitude = (lon: number): boolean => {
  return lon >= -180 && lon <= 180;
};

export const isValidSensorId = (id: string): boolean => {
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
  return uuidRegex.test(id);
};

export const isValidDoseRate = (value: number): boolean => {
  return !isNaN(value) && value >= 0 && value < 1000000;
};

export const isValidPassword = (password: string): boolean => {
  return password.length >= 8 && /[A-Z]/.test(password) && /[0-9]/.test(password);
};

export const isValidCoordinate = (lat: number, lon: number): boolean => {
  return isValidLatitude(lat) && isValidLongitude(lon);
};
