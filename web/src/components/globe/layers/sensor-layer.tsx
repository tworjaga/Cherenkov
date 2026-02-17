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

  const _layer = useMemo(() => {
    return new ScatterplotLayer({
      id: 'sensor-layer',
      data: sensors,
      pickable: true,
      opacity: 0.8,
      stroked: true,
      filled: true,
      radiusScale: 1,
      radiusMinPixels: 4,
      radiusMaxPixels: 16,
      lineWidthMinPixels: 1,
      getPosition: (d: Sensor) => [d.longitude, d.latitude],
      getRadius: (d: Sensor) => {
        if (d.id === selectedSensorId) return 12;
        return d.status === 'active' ? 8 : 6;
      },
      getFillColor: (d: Sensor) => {
        if (d.id === selectedSensorId) {
          return [0, 212, 255]; // accent.primary
        }
        switch (d.status) {
          case 'active':
            return [0, 255, 136]; // alert.normal
          case 'warning':
            return [255, 184, 0]; // alert.medium
          case 'critical':
            return [255, 51, 102]; // alert.critical
          default:
            return [160, 160, 176]; // text.secondary
        }
      },
      getLineColor: [255, 255, 255],
      getLineWidth: 2,
      onClick: (info: { object: Sensor | null }) => {
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
