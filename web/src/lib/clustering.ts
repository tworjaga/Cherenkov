export interface Point {
  lat: number;
  lon: number;
  value: number;
  id: string;
}

export interface Cluster {
  lat: number;
  lon: number;
  count: number;
  points: Point[];
  avgValue: number;
  id: string;
}

const EARTH_RADIUS = 6371000; // meters

const haversineDistance = (lat1: number, lon1: number, lat2: number, lon2: number): number => {
  const toRad = (deg: number) => (deg * Math.PI) / 180;
  const dLat = toRad(lat2 - lat1);
  const dLon = toRad(lon2 - lon1);
  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(toRad(lat1)) * Math.cos(toRad(lat2)) * Math.sin(dLon / 2) * Math.sin(dLon / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  return EARTH_RADIUS * c;
};

export const clusterSensors = (
  points: Point[],
  zoom: number,
  maxClusters: number = 100
): Cluster[] => {
  if (points.length <= maxClusters) {
    return points.map((p) => ({
      lat: p.lat,
      lon: p.lon,
      count: 1,
      points: [p],
      avgValue: p.value,
      id: p.id,
    }));
  }

  // Adaptive clustering radius based on zoom level
  const baseRadius = 50000; // 50km at zoom 0
  const radius = baseRadius / Math.pow(2, zoom);

  const clusters: Cluster[] = [];
  const visited = new Set<string>();

  for (const point of points) {
    if (visited.has(point.id)) continue;

    const cluster: Cluster = {
      lat: point.lat,
      lon: point.lon,
      count: 0,
      points: [],
      avgValue: 0,
      id: `cluster-${point.id}`,
    };

    // Find all points within radius
    for (const other of points) {
      if (visited.has(other.id)) continue;

      const distance = haversineDistance(point.lat, point.lon, other.lat, other.lon);
      if (distance <= radius) {
        cluster.points.push(other);
        visited.add(other.id);
      }
    }

    if (cluster.points.length > 0) {
      // Calculate cluster center and average value
      cluster.count = cluster.points.length;
      cluster.lat = cluster.points.reduce((sum, p) => sum + p.lat, 0) / cluster.count;
      cluster.lon = cluster.points.reduce((sum, p) => sum + p.lon, 0) / cluster.count;
      cluster.avgValue = cluster.points.reduce((sum, p) => sum + p.value, 0) / cluster.count;
      clusters.push(cluster);
    }
  }

  return clusters;
};

// Grid-based clustering for very large datasets
export const gridCluster = (
  points: Point[],
  zoom: number,
  maxClusters: number = 100
): Cluster[] => {
  if (points.length <= maxClusters) {
    return points.map((p) => ({
      lat: p.lat,
      lon: p.lon,
      count: 1,
      points: [p],
      avgValue: p.value,
      id: p.id,
    }));
  }

  // Grid size in degrees, adaptive to zoom
  const gridSize = 2 / Math.pow(2, zoom);

  const grid = new Map<string, Point[]>();

  for (const point of points) {
    const gridX = Math.floor(point.lon / gridSize);
    const gridY = Math.floor(point.lat / gridSize);
    const key = `${gridX},${gridY}`;

    if (!grid.has(key)) {
      grid.set(key, []);
    }
    grid.get(key)!.push(point);
  }

  const clusters: Cluster[] = [];
  for (const [key, gridPoints] of grid) {
    const cluster: Cluster = {
      lat: gridPoints.reduce((sum, p) => sum + p.lat, 0) / gridPoints.length,
      lon: gridPoints.reduce((sum, p) => sum + p.lon, 0) / gridPoints.length,
      count: gridPoints.length,
      points: gridPoints,
      avgValue: gridPoints.reduce((sum, p) => sum + p.value, 0) / gridPoints.length,
      id: `grid-${key}`,
    };
    clusters.push(cluster);
  }

  // If still too many clusters, recursively merge
  if (clusters.length > maxClusters) {
    return clusterSensors(
      clusters.map((c) => ({ lat: c.lat, lon: c.lon, value: c.avgValue, id: c.id })),
      zoom - 1,
      maxClusters
    );
  }

  return clusters;
};
