'use client';

import { useMemo } from 'react';
import { HeatmapLayer } from 'deck.gl';


interface HeatmapData {
  coordinates: [number, number];
  weight: number;
}

interface HeatmapLayerProps {
  data?: HeatmapData[];
  visible?: boolean;
  intensity?: number;
  radius?: number;
}

export function HeatmapLayerComponent({
  data = [],
  visible = true,
  intensity = 1,
  radius = 50,
}: HeatmapLayerProps): HeatmapLayer<{ position: [number, number]; weight: number }> {

  const processedData = useMemo(() => {
    return data.map((point) => ({
      position: point.coordinates,
      weight: point.weight,
    }));
  }, [data]);

  return new HeatmapLayer({
    id: 'heatmap-layer',
    data: processedData,
    visible,
    getPosition: (d: { position: [number, number] }) => d.position,
    getWeight: (d: { weight: number }) => d.weight,
    intensity,
    radiusPixels: radius,
    colorRange: [
      [255, 255, 178],
      [254, 204, 92],
      [253, 141, 60],
      [240, 59, 32],
      [189, 0, 38],
    ],
  });
}
