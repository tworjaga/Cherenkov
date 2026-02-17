export const calculateBearing = (

  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number
): number => {
  const dLon = ((lon2 - lon1) * Math.PI) / 180;
  const y = Math.sin(dLon) * Math.cos((lat2 * Math.PI) / 180);
  const x =
    Math.cos((lat1 * Math.PI) / 180) * Math.sin((lat2 * Math.PI) / 180) -
    Math.sin((lat1 * Math.PI) / 180) *
      Math.cos((lat2 * Math.PI) / 180) *
      Math.cos(dLon);
  const bearing = (Math.atan2(y, x) * 180) / Math.PI;
  return (bearing + 360) % 360;
};

export const interpolatePosition = (
  start: { lat: number; lon: number },
  end: { lat: number; lon: number },
  fraction: number
): { lat: number; lon: number } => {
  return {
    lat: start.lat + (end.lat - start.lat) * fraction,
    lon: start.lon + (end.lon - start.lon) * fraction,
  };
};

export const clamp = (value: number, min: number, max: number): number => {
  return Math.min(Math.max(value, min), max);
};

export const lerp = (start: number, end: number, t: number): number => {
  return start + (end - start) * t;
};

export const getSeverityColor = (severity: string): string => {
  switch (severity) {
    case 'critical':
      return '#ff3366';
    case 'high':
      return '#ff6b35';
    case 'medium':
      return '#ffb800';
    case 'low':
      return '#00d4ff';
    default:
      return '#00ff88';
  }
};

export const getDefconColor = (level: number): string => {
  switch (level) {
    case 1:
      return '#ff3366';
    case 2:
      return '#ff6b35';
    case 3:
      return '#ffb800';
    case 4:
      return '#00d4ff';
    case 5:
    default:
      return '#00ff88';
  }
};

export const calculateBoundingBox = (
  points: { lat: number; lon: number }[]
): { minLat: number; maxLat: number; minLon: number; maxLon: number } => {
  if (points.length === 0) {
    return { minLat: 0, maxLat: 0, minLon: 0, maxLon: 0 };
  }
  
  const lats = points.map(p => p.lat);
  const lons = points.map(p => p.lon);
  
  return {
    minLat: Math.min(...lats),
    maxLat: Math.max(...lats),
    minLon: Math.min(...lons),
    maxLon: Math.max(...lons),
  };
};

export const calculateCentroid = (
  points: { lat: number; lon: number }[]
): { lat: number; lon: number } => {
  if (points.length === 0) {
    return { lat: 0, lon: 0 };
  }
  
  const sumLat = points.reduce((sum, p) => sum + p.lat, 0);
  const sumLon = points.reduce((sum, p) => sum + p.lon, 0);
  
  return {
    lat: sumLat / points.length,
    lon: sumLon / points.length,
  };
};
