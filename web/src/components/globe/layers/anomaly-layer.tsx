'use client';

import { useMemo } from 'react';
import { ScatterplotLayer } from '@deck.gl/layers';
import { useDataStore } from '@/stores';
import type { Anomaly } from '@/types';

interface AnomalyLayerProps {
  onAnomalyClick?: (anomalyId: string) => void;
}

export function AnomalyLayer({ onAnomalyClick }: AnomalyLayerProps) {
  const { anomalies } = useDataStore();

  useMemo(() => {
    return new ScatterplotLayer({
      id: 'anomaly-layer',
      data: anomalies,
      pickable: true,
      opacity: 0.9,
      stroked: true,
      filled: true,
      radiusMinPixels: 8,
      radiusMaxPixels: 32,
      lineWidthMinPixels: 2,
      getPosition: (d: Anomaly) => [d.location.lon, d.location.lat],
      getRadius: (d: Anomaly) => {
        // Size based on severity
        switch (d.severity) {
          case 'critical':
            return 24;
          case 'high':
            return 20;
          case 'medium':
            return 16;
          case 'low':
            return 12;
          default:
            return 12;
        }
      },
      getFillColor: (d: Anomaly): [number, number, number, number] => {
        switch (d.severity) {
          case 'critical':
            return [255, 51, 102, 200]; // alert.critical with alpha
          case 'high':
            return [255, 107, 53, 200]; // alert.high with alpha
          case 'medium':
            return [255, 184, 0, 200]; // alert.medium with alpha
          case 'low':
            return [0, 212, 255, 200]; // alert.low with alpha
          default:
            return [160, 160, 176, 200];
        }
      },
      getLineColor: (): [number, number, number, number] => [255, 255, 255, 255],
      getLineWidth: 2,
      onClick: (info: { object?: Anomaly | null; x: number; y: number }) => {
        if (info.object && onAnomalyClick) {
          onAnomalyClick(info.object.id);
        }
      },
    });
  }, [anomalies, onAnomalyClick]);

  return null;
}

export default AnomalyLayer;
