'use client';

import { useMemo } from 'react';
import { ScatterplotLayer, HeatmapLayer } from 'deck.gl';

import { useGlobeStore } from '@/stores/globe-store';
import { usePlumeSimulation, PlumeParticle } from '@/hooks/use-plume-simulation';

interface PlumeLayerProps {
  visible?: boolean;
  onClick?: (info: { object: PlumeParticle | null; x: number; y: number }) => void;
  showHeatmap?: boolean;
  showParticles?: boolean;
}

export function PlumeLayer({ 
  visible = true, 
  onClick,
  showHeatmap = true,
  showParticles = true,
}: PlumeLayerProps): (ScatterplotLayer<PlumeParticle> | HeatmapLayer<{ position: [number, number]; weight: number }>)[] {
  const { timeRange } = useGlobeStore();
  
  // Use real-time simulation data from WebSocket
  const { state } = usePlumeSimulation();

  // Filter particles by time range if set
  const filteredData = useMemo(() => {
    if (!timeRange || !state.particles.length) return state.particles;
    
    return state.particles.filter((particle: PlumeParticle) => {
      const timestamp = particle.timestamp;
      return timestamp >= timeRange[0] && timestamp <= timeRange[1];
    });
  }, [state.particles, timeRange]);

  // Prepare heatmap data from particle concentrations
  const heatmapData = useMemo(() => {
    return filteredData.map((particle: PlumeParticle) => ({
      position: [particle.lng, particle.lat] as [number, number],
      weight: particle.concentration,
    }));
  }, [filteredData]);

  // Calculate color based on dose rate (higher = more red)
  const getFillColor = (d: PlumeParticle): [number, number, number, number] => {
    // Color by dose rate (yellow to red scale)
    const intensity = Math.min(d.doseRate / 100, 1);
    return [
      255, 
      Math.round(255 - intensity * 155), 
      Math.round(100 - intensity * 100), 
      Math.round(150 + intensity * 105)
    ];
  };

  const layers: (ScatterplotLayer<PlumeParticle> | HeatmapLayer<{ position: [number, number]; weight: number }>)[] = [];

  // Add heatmap layer for concentration visualization
  if (showHeatmap) {
    layers.push(new HeatmapLayer({
      id: 'plume-heatmap-layer',
      data: heatmapData,
      visible: visible && state.isRunning,
      getPosition: (d: { position: [number, number] }) => d.position,
      getWeight: (d: { weight: number }) => d.weight,
      intensity: 2,
      radiusPixels: 60,
      colorRange: [
        [255, 255, 178],
        [254, 204, 92],
        [253, 141, 60],
        [240, 59, 32],
        [189, 0, 38],
      ],
    }));
  }

  // Add particle scatterplot layer
  if (showParticles) {
    layers.push(new ScatterplotLayer({
      id: 'plume-particles-layer',
      data: filteredData,
      visible: visible && state.isRunning,
      getPosition: (d: PlumeParticle) => [d.lng, d.lat, d.altitude],
      getRadius: (d: PlumeParticle) => Math.max(50, d.concentration * 10),
      getFillColor,
      getLineColor: () => [255, 200, 50, 200],
      lineWidthMinPixels: 1,
      stroked: true,
      filled: true,
      pickable: true,
      onClick: onClick as (info: { object: PlumeParticle | null; x: number; y: number }) => void,
      radiusMinPixels: 3,
      radiusMaxPixels: 50,
      billboard: false, // 3D particles
    }));
  }

  return layers;
}
