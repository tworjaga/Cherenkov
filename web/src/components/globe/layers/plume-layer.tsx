'use client';

import { useMemo, useCallback } from 'react';
import { ScatterplotLayer, HeatmapLayer, PolygonLayer } from 'deck.gl';

import { useGlobeStore } from '@/stores/globe-store';
import { usePlumeSimulation, usePlumeParticles, particlesToDeckGlFormat } from '@/hooks';
import { useToast } from '@/components/ui/toast';


// Dose thresholds in μSv/h for evacuation zones
const DOSE_THRESHOLDS = {
  CRITICAL: 1000,    // 1 mSv/h - Immediate evacuation
  HIGH: 100,         // 0.1 mSv/h - Shelter in place
  MEDIUM: 10,        // 0.01 mSv/h - Monitoring zone
  LOW: 1,            // 0.001 mSv/h - Precautionary zone
} as const;

interface PlumeParticle {
  x: number;
  y: number;
  z: number;
  concentration: number;
  timestamp: number;
}

interface EvacuationZone {
  id: string;
  name: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
  contour: Array<[number, number]>;
  doseRate: number;
  population: number;
  instructions: string;
}

interface PlumeLayerProps {
  visible?: boolean;
  onClick?: (info: { object: PlumeParticle | null; x: number; y: number }) => void;
  showHeatmap?: boolean;
  showParticles?: boolean;
  showEvacuationZones?: boolean;
  particles?: PlumeParticle[];
  evacuationZones?: EvacuationZone[];
  releaseLat?: number;
  releaseLng?: number;
  isotope?: string;
  onZoneClick?: (zone: EvacuationZone) => void;
  /** Simulation ID for real-time particle streaming */
  simulationId?: string;
  /** Enable real-time particle streaming via WebSocket */
  enableRealtime?: boolean;
}



export function PlumeLayer({ 
  visible = true, 
  onClick,
  showHeatmap = true,
  showParticles = true,
  showEvacuationZones = true,
  particles = [],
  evacuationZones = [],
  releaseLat = 0,
  releaseLng = 0,
  isotope = 'Cs-137',
  onZoneClick,
  simulationId,
  enableRealtime = false,
}: PlumeLayerProps): (ScatterplotLayer<PlumeParticle> | HeatmapLayer<{ position: [number, number]; weight: number }> | PolygonLayer<EvacuationZone>)[] {
  const { timeRange } = useGlobeStore();
  const { toast } = useToast();
  
  // Use real-time simulation data from WebSocket
  const [simState] = usePlumeSimulation();
  
  // Connect to real-time particle streaming when simulationId is provided
  const { 
    particles: realtimeParticles, 
    isConnected: isRealtimeConnected,
    totalParticles: realtimeTotal 
  } = usePlumeParticles({
    simulationId: simulationId || '',
    enabled: enableRealtime && !!simulationId && simState.isPlaying,
    batchSize: 100,
  });


  // Calculate dose rate from concentration
  const calculateDoseRate = useCallback((concentration: number): number => {
    const conversionFactors: Record<string, number> = {
      'Cs-137': 0.0001,
      'I-131': 0.0002,
      'Co-60': 0.0003,
      'Sr-90': 0.00015,
    };
    const factor = conversionFactors[isotope] || 0.0001;
    return concentration * factor;
  }, [isotope]);


  // Merge real-time particles with prop-provided particles
  const allParticles = useMemo(() => {
    const merged = [...particles];
    if (enableRealtime && realtimeParticles.length > 0) {
      // Convert real-time particles to PlumeParticle format
      const converted = realtimeParticles.map(p => ({
        x: p.x,
        y: p.y,
        z: p.z,
        concentration: p.concentration,
        timestamp: new Date(p.timestamp).getTime(),
      }));
      merged.push(...converted);
    }
    return merged;
  }, [particles, realtimeParticles, enableRealtime]);

  // Filter particles by time range if set
  const filteredData = useMemo(() => {
    if (!timeRange || !allParticles.length) return allParticles;
    
    return allParticles.filter((particle: PlumeParticle) => {
      const timestamp = particle.timestamp;
      return timestamp >= timeRange[0] && timestamp <= timeRange[1];
    });
  }, [allParticles, timeRange]);


  // Prepare heatmap data from particle concentrations
  // Note: x, y are in meters from release point, need to convert to lat/lng for visualization
  const heatmapData = useMemo(() => {
    return filteredData.map((particle: PlumeParticle) => ({
      // Convert local coordinates (meters) to approximate lat/lng offset
      // 1 degree latitude ≈ 111km, 1 degree longitude varies by latitude
      position: [
        particle.x / 111000, 
        particle.y / 111000
      ] as [number, number],
      weight: particle.concentration,
    }));
  }, [filteredData]);

  // Calculate color based on concentration (higher = more red)
  const getFillColor = (d: PlumeParticle): [number, number, number, number] => {
    // Color by concentration (yellow to red scale)
    const intensity = Math.min(d.concentration / 1000, 1);
    return [
      255, 
      Math.round(255 - intensity * 155), 
      Math.round(100 - intensity * 100), 
      Math.round(150 + intensity * 105)
    ];
  };

  // Generate evacuation zones from particles if not provided
  const generatedZones = useMemo((): EvacuationZone[] => {
    if (evacuationZones.length > 0) return evacuationZones;
    if (!allParticles.length || !releaseLat || !releaseLng) return [];


    const zones: EvacuationZone[] = [];
    const thresholds = [
      { level: 'critical' as const, threshold: DOSE_THRESHOLDS.CRITICAL, name: 'Immediate Evacuation Zone', color: [189, 0, 38] },
      { level: 'high' as const, threshold: DOSE_THRESHOLDS.HIGH, name: 'Shelter in Place Zone', color: [240, 59, 32] },
      { level: 'medium' as const, threshold: DOSE_THRESHOLDS.MEDIUM, name: 'Monitoring Zone', color: [253, 141, 60] },
      { level: 'low' as const, threshold: DOSE_THRESHOLDS.LOW, name: 'Precautionary Zone', color: [254, 204, 92] },
    ];

    for (const { level, threshold, name, color } of thresholds) {
      const exceedingParticles = allParticles.filter(p => {

        const doseRate = calculateDoseRate(p.concentration);
        return doseRate >= threshold;
      });

      if (exceedingParticles.length < 3) continue;

      // Calculate centroid
      const centroid = exceedingParticles.reduce(
        (acc, p) => ({ 
          x: acc.x + p.x / exceedingParticles.length, 
          y: acc.y + p.y / exceedingParticles.length 
        }),
        { x: 0, y: 0 }
      );

      // Sort by angle for convex hull
      const sorted = exceedingParticles
        .map(p => ({
          x: p.x,
          y: p.y,
          angle: Math.atan2(p.y - centroid.y, p.x - centroid.x),
        }))
        .sort((a, b) => a.angle - b.angle);

      // Convert local coordinates to lat/lng
      const contour: Array<[number, number]> = sorted.map(p => [
        releaseLng + (p.x / 111000) / Math.cos(releaseLat * Math.PI / 180),
        releaseLat + p.y / 111000,
      ]);

      // Close the polygon
      if (contour.length > 0) {
        contour.push(contour[0]);
      }

      const maxRadius = Math.max(
        ...exceedingParticles.map(p => 
          Math.sqrt(Math.pow(p.x / 111000, 2) + Math.pow(p.y / 111000, 2))
        )
      );

      const area = Math.PI * Math.pow(maxRadius, 2);
      const population = Math.round(area * 2000);

      const avgDoseRate = exceedingParticles.reduce((sum, p) => 
        sum + calculateDoseRate(p.concentration), 0
      ) / exceedingParticles.length;

      zones.push({
        id: `zone-${level}`,
        name,
        severity: level,
        contour,
        doseRate: Math.round(avgDoseRate * 100) / 100,
        population,
        instructions: getInstructionsForLevel(level),
      });
    }

    return zones;
  }, [allParticles, evacuationZones, releaseLat, releaseLng, calculateDoseRate]);


  // Trigger alert for critical zones
  useMemo(() => {
    const criticalZones = generatedZones.filter(z => z.severity === 'critical');
    if (criticalZones.length > 0 && simState.isPlaying) {
      toast({
        title: 'Critical Evacuation Alert',
        description: `${criticalZones.length} critical zone(s) detected. Immediate evacuation required.`,
        variant: 'destructive',
        duration: 10000,
      });
      
      // Notify external system if callback provided
      criticalZones.forEach(zone => {
        if (onZoneClick) onZoneClick(zone);
      });
    }
  }, [generatedZones, simState.isPlaying, toast, onZoneClick]);

  // Show connection status toast for real-time streaming
  useMemo(() => {
    if (enableRealtime && simulationId) {
      if (isRealtimeConnected) {
        toast({
          title: 'Real-time Streaming Connected',
          description: `Receiving live particle data (${realtimeTotal} particles)`,
          variant: 'default',
          duration: 3000,
        });
      }
    }
  }, [enableRealtime, simulationId, isRealtimeConnected, realtimeTotal, toast]);


  const layers: (ScatterplotLayer<PlumeParticle> | HeatmapLayer<{ position: [number, number]; weight: number }> | PolygonLayer<EvacuationZone>)[] = [];

  // Add evacuation zone polygons
  if (showEvacuationZones && generatedZones.length > 0) {
    layers.push(new PolygonLayer({
      id: 'evacuation-zones-layer',
      data: generatedZones,
      visible: visible && simState.isPlaying,
      getPolygon: (d: EvacuationZone) => d.contour,
      getFillColor: (d: EvacuationZone) => {
        const colors = {
          critical: [189, 0, 38, 128],
          high: [240, 59, 32, 100],
          medium: [253, 141, 60, 80],
          low: [254, 204, 92, 60],
        };
        return colors[d.severity] as [number, number, number, number];
      },
      getLineColor: (d: EvacuationZone) => {
        const colors = {
          critical: [189, 0, 38, 255],
          high: [240, 59, 32, 255],
          medium: [253, 141, 60, 255],
          low: [254, 204, 92, 255],
        };
        return colors[d.severity] as [number, number, number, number];
      },
      getLineWidth: 3,
      lineWidthMinPixels: 2,
      filled: true,
      stroked: true,
      pickable: true,
      onClick: (info: { object: EvacuationZone | null }) => {
        if (info.object && onZoneClick) {
          onZoneClick(info.object);
        }
      },
      autoHighlight: true,
      highlightColor: [255, 255, 255, 100],
    }));
  }

  // Add heatmap layer for concentration visualization
  if (showHeatmap) {

    layers.push(new HeatmapLayer({
      id: 'plume-heatmap-layer',
      data: heatmapData,
      visible: visible && simState.isPlaying,
      getPosition: (d: { position: [number, number] }) => d.position,
      getWeight: (d: { position: [number, number]; weight: number }) => d.weight,
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
      visible: visible && simState.isPlaying,

      // x, y, z are in meters from release point
      getPosition: (d: PlumeParticle) => [
        d.x / 111000,  // Convert to approximate degrees
        d.y / 111000, 
        d.z
      ],
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

function getInstructionsForLevel(level: string): string {
  const instructions: Record<string, string> = {
    critical: 'Evacuate immediately. Move perpendicular to wind direction. Seek medical attention if exposed.',
    high: 'Close all windows and doors. Turn off ventilation. Move to interior rooms.',
    medium: 'Stay alert for updates. Prepare for potential evacuation. Limit outdoor activities.',
    low: 'Monitor local news. Follow official guidance. No immediate action required.',
  };
  return instructions[level] || 'Follow official guidance.';
}
