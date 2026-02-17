'use client';

import { useMemo } from 'react';
import { ScatterplotLayer } from '@deck.gl/layers';
import { useDataStore } from '@/stores';
import type { Sensor } from '@/types';

interface SensorLayerProps {
  onSensorClick?: (sensorId: string) => void;
  selectedSensorId?: string | null;
}

export function SensorLayer({ onSensorClick, selectedSensorId }: SensorLayerProps) {
  const { sensors } = useDataStore();

  useMemo(() => {
    return new ScatterplotLayer({
      id: 'sensor-layer',
      data: sensors,
      pickable: true,
      opacity: 0.8,
      stroked: true,
      filled: true,
      radiusMinPixels: 4,
      radiusMaxPixels: 16,
      lineWidthMinPixels: 1,
      getPosition: (d: Sensor) => [d.location.lon, d.location.lat],
      getRadius: (d: Sensor) => {
        if (d.id === selectedSensorId) return 12;
        return d.status === 'active' ? 8 : 6;
      },
      getFillColor: (d: Sensor): [number, number, number, number] => {
        if (d.id === selectedSensorId) {
          return [0, 212, 255, 255]; // accent.primary
        }
        switch (d.status) {
          case 'active':
            return [0, 255, 136, 255]; // alert.normal
          case 'inactive':
            return [160, 160, 176, 255]; // text.secondary
          case 'maintenance':
            return [255, 184, 0, 255]; // alert.medium
          case 'offline':
            return [255, 51, 102, 255]; // alert.critical
          default:
            return [160, 160, 176, 255]; // text.secondary
        }
      },
      getLineColor: (): [number, number, number, number] => [255, 255, 255, 255],
      getLineWidth: 2,
      onClick: (info: { object?: Sensor | null; x: number; y: number }) => {
        if (info.object && onSensorClick) {
          onSensorClick(info.object.id);
        }
      },
      transitions: {
        getFillColor: 300,
        getRadius: 300,
      },
    });
  }, [sensors, selectedSensorId, onSensorClick]);

  return null; // Layer is managed by parent DeckGL component
}

export default SensorLayer;
