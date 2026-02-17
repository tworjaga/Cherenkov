/**
 * Geographic helper functions
 * Utilities for coordinate calculations and geo operations
 */

/**
 * Calculate distance between two coordinates using Haversine formula
 */
export function calculateDistance(
  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number
): number {
  const R = 6371; // Earth's radius in km
  const dLat = toRad(lat2 - lat1);
  const dLon = toRad(lon2 - lon1);
  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) *
    Math.sin(dLon / 2) * Math.sin(dLon / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  return R * c;
}

function toRad(deg: number): number {
  return deg * (Math.PI / 180);
}

/**
 * Check if a point is within a bounding box
 */
export function isPointInBounds(
  lat: number,
  lon: number,
  bounds: { north: number; south: number; east: number; west: number }
): boolean {
  return (
    lat <= bounds.north &&
    lat >= bounds.south &&
    lon <= bounds.east &&
    lon >= bounds.west
  );
}

/**
 * Format coordinates for display
 */
export function formatCoordinates(lat: number, lon: number): string {
  const latDir = lat >= 0 ? 'N' : 'S';
  const lonDir = lon >= 0 ? 'E' : 'W';
  return `${Math.abs(lat).toFixed(4)}${latDir}, ${Math.abs(lon).toFixed(4)}${lonDir}`;
}

/**
 * Parse coordinate string to numbers
 */
export function parseCoordinates(coordString: string): { lat: number; lon: number } | null {
  const match = coordString.match(/(-?\d+\.?\d*)\s*[,;]\s*(-?\d+\.?\d*)/);
  if (!match) return null;
  return {
    lat: parseFloat(match[1]),
    lon: parseFloat(match[2]),
  };
}

/**
 * Calculate bounding box from center point and radius
 */
export function calculateBounds(
  centerLat: number,
  centerLon: number,
  radiusKm: number
): { north: number; south: number; east: number; west: number } {
  const latDelta = radiusKm / 111;
  const lonDelta = radiusKm / (111 * Math.cos(toRad(centerLat)));
  
  return {
    north: centerLat + latDelta,
    south: centerLat - latDelta,
    east: centerLon + lonDelta,
    west: centerLon - lonDelta,
  };
}
