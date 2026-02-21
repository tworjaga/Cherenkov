'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { getWsClient } from '@/lib/graphql/client';
import { EVACUATION_ZONES } from '@/lib/graphql/subscriptions';

export interface EvacuationZone {
  id: string;
  name: string;
  level: number;
  doseThreshold: number;
  polygon: {
    coordinates: number[][][];
  };
  center: {
    latitude: number;
    longitude: number;
  };
  radius: number;
  population: number;
  timestamp: string;
}

export interface EvacuationZonesUpdate {
  simulationId: string;
  zones: EvacuationZone[];
  timestamp: string;
}

export interface UseEvacuationZonesOptions {
  simulationId: string;
  enabled?: boolean;
  onZonesReceived?: (update: EvacuationZonesUpdate) => void;
}

export interface UseEvacuationZonesReturn {
  zones: EvacuationZone[];
  isConnected: boolean;
  error: Error | null;
  lastUpdateTime: Date | null;
  totalZones: number;
  clearZones: () => void;
}

interface EvacuationZonesSubscriptionResponse {
  evacuationZones: EvacuationZonesUpdate;
}

export function useEvacuationZones(options: UseEvacuationZonesOptions): UseEvacuationZonesReturn {
  const { simulationId, enabled = true, onZonesReceived } = options;

  const [zones, setZones] = useState<EvacuationZone[]>([]);
  const [lastUpdateTime, setLastUpdateTime] = useState<Date | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  const zonesRef = useRef<EvacuationZone[]>([]);
  const unsubscribeRef = useRef<(() => void) | null>(null);

  // Update ref when zones change
  useEffect(() => {
    zonesRef.current = zones;
  }, [zones]);

  const clearZones = useCallback(() => {
    setZones([]);
    setLastUpdateTime(null);
    setError(null);
  }, []);

  // Clear zones when simulation changes
  useEffect(() => {
    clearZones();
  }, [simulationId, clearZones]);

  // Subscribe to evacuation zones via GraphQL WS
  useEffect(() => {
    if (!enabled) {
      setIsConnected(false);
      return;
    }

    setIsConnected(false);

    try {
      const client = getWsClient();

      const unsubscribe = client.subscribe(
        {
          query: EVACUATION_ZONES,
          variables: { simulationId },
        },
        {
          next: (data) => {
            setIsConnected(true);

            if (data.data?.evacuationZones) {
              const update = data.data.evacuationZones as EvacuationZonesUpdate;

              if (!update.zones || !Array.isArray(update.zones)) {
                return;
              }

              setZones(update.zones);
              setLastUpdateTime(new Date());

              if (onZonesReceived) {
                onZonesReceived(update);
              }
            }
          },
          error: (err) => {
            console.error('Evacuation zones subscription error:', err);
            setError(err instanceof Error ? err : new Error('Subscription error'));
            setIsConnected(false);
          },
          complete: () => {
            setIsConnected(false);
          },
        }
      );

      unsubscribeRef.current = unsubscribe;
    } catch (err) {
      console.error('Failed to subscribe to evacuation zones:', err);
      setError(err instanceof Error ? err : new Error('Failed to subscribe'));
      setIsConnected(false);
    }

    return () => {
      if (unsubscribeRef.current) {
        unsubscribeRef.current();
        unsubscribeRef.current = null;
      }
    };
  }, [simulationId, enabled, onZonesReceived]);

  return {
    zones,
    isConnected,
    error,
    lastUpdateTime,
    totalZones: zones.length,
    clearZones,
  };
}

// Utility to convert evacuation zones to deck.gl PolygonLayer format
export function zonesToDeckGlFormat(
  zones: EvacuationZone[],
  options: {
    getFillColor?: (z: EvacuationZone) => [number, number, number, number];
    getLineColor?: (z: EvacuationZone) => [number, number, number, number];
    getLineWidth?: (z: EvacuationZone) => number;
  } = {}
) {
  const {
    getFillColor = (z) => {
      // Color based on evacuation level (red = critical, orange = high, yellow = medium, green = low)
      switch (z.level) {
        case 4: // Critical
          return [220, 20, 60, 150];
        case 3: // High
          return [255, 140, 0, 150];
        case 2: // Medium
          return [255, 215, 0, 150];
        case 1: // Low
          return [50, 205, 50, 150];
        default:
          return [128, 128, 128, 100];
      }
    },
    getLineColor = (z) => {
      // Darker border for visibility
      switch (z.level) {
        case 4:
          return [139, 0, 0, 255];
        case 3:
          return [255, 69, 0, 255];
        case 2:
          return [218, 165, 32, 255];
        case 1:
          return [34, 139, 34, 255];
        default:
          return [64, 64, 64, 255];
      }
    },
    getLineWidth = () => 2,
  } = options;

  return zones.map(z => ({
    contour: z.polygon.coordinates[0].map(coord => [coord[0], coord[1]]),
    fillColor: getFillColor(z),
    lineColor: getLineColor(z),
    lineWidth: getLineWidth(z),
    name: z.name,
    level: z.level,
    doseThreshold: z.doseThreshold,
    population: z.population,
    id: z.id,
  }));
}

// Utility to get zone statistics
export function getZoneStats(zones: EvacuationZone[]) {
  if (zones.length === 0) {
    return {
      totalPopulation: 0,
      criticalZones: 0,
      highZones: 0,
      mediumZones: 0,
      lowZones: 0,
    };
  }

  return {
    totalPopulation: zones.reduce((sum, z) => sum + (z.population || 0), 0),
    criticalZones: zones.filter(z => z.level === 4).length,
    highZones: zones.filter(z => z.level === 3).length,
    mediumZones: zones.filter(z => z.level === 2).length,
    lowZones: zones.filter(z => z.level === 1).length,
  };
}
