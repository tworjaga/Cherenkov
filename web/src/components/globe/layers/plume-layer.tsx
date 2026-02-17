'use client';

import { useMemo } from 'react';
import { ScatterplotLayer } from 'deck.gl';

import { useGlobeStore } from '@/stores/globe-store';

interface PlumeData {
  id: string;
  coordinates: [number, number];
  radius: number;
  concentration: number;
  timestamp: Date;
}

interface PlumeLayerProps {
  data?: PlumeData[];
  visible?: boolean;
  onClick?: (info: { object?: PlumeData; x: number; y: number }) => void;
}


export function PlumeLayer({ data = [], visible = true, onClick }: PlumeLayerProps): ScatterplotLayer<PlumeData> {

  const { timeRange } = useGlobeStore();

  const filteredData = useMemo(() => {
    if (!timeRange) return data;
    return data.filter((plume) => {
      const timestamp = new Date(plume.timestamp).getTime();
      return timestamp >= timeRange[0] && timestamp <= timeRange[1];
    });
  }, [data, timeRange]);

  return new ScatterplotLayer({
    id: 'plume-layer',
    data: filteredData,
    visible,
    getPosition: (d: PlumeData) => d.coordinates,
    getRadius: (d: PlumeData) => d.radius,
    getFillColor: (d: PlumeData) => {
      const intensity = Math.min(d.concentration / 100, 1);
      return [255, 100 + intensity * 100, 50, 150 + intensity * 105];
    },
    getLineColor: () => [255, 200, 50, 200],


    lineWidthMinPixels: 1,
    stroked: true,
    filled: true,
    pickable: true,
    onClick,
    radiusMinPixels: 5,
    radiusMaxPixels: 100,
  });

}
