'use client';

import { useMemo } from 'react';
import { ScatterplotLayer } from 'deck.gl';

import { useGlobeStore } from '@/stores/globe-store';
import { usePlumeSimulation, PlumeParticle } from '@/hooks/use-plume-simulation';

interface PlumeLayerProps {
  visible?: boolean;
  onClick?: (info: { object: PlumeParticle | null; x: number; y: number }) => void;
}

export function PlumeLayer({ 
  visible = true, 
  onClick 
}: PlumeLayerProps): ScatterplotLayer<PlumeParticle> {
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

  return new ScatterplotLayer({
    id: 'plume-layer',
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
  });
}
