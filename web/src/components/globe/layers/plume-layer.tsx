'use client';

import { useMemo, useCallback, useState, useEffect } from 'react';
import { ScatterplotLayer, HeatmapLayer, PolygonLayer } from 'deck.gl';

import { useGlobeStore } from '@/stores/globe-store';
import { 
  usePlumeSimulation, 
  usePlumeParticles, 
  useEvacuationZones,
  zonesToDeckGlFormat,
} from '@/hooks';

import { useToast } from '@/components/ui/toast';
import { Spinner } from '@/components/ui/spinner';
import { Alert } from '@/components/ui/alert';




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
  /** Enable real-time evacuation zone updates via GraphQL subscription */
  enableRealtimeZones?: boolean;
  /** Callback when loading state changes */
  onLoadingChange?: (isLoading: boolean) => void;
  /** Callback when error occurs */
  onError?: (error: Error) => void;
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
  enableRealtimeZones = false,
  onLoadingChange,
  onError,
}: PlumeLayerProps) {

  const [isInitializing, setIsInitializing] = useState(true);
  const [connectionError, setConnectionError] = useState<Error | null>(null);
  
  const { timeRange } = useGlobeStore();
  const { toast } = useToast();
  
  // Use real-time simulation data from WebSocket
  const [simState] = usePlumeSimulation();
  
  // Connect to real-time particle streaming when simulationId is provided
  const { 
    particles: realtimeParticles, 
    isConnected: isRealtimeConnected,
    totalParticles: realtimeTotal,
    error: particlesError,
    clearParticles,
  } = usePlumeParticles({
    simulationId: simulationId || '',
    enabled: enableRealtime && !!simulationId && simState.isPlaying,
    batchSize: 100,
    onBatchReceived: () => {
      setIsInitializing(false);
    },
  });

  // Connect to real-time evacuation zones when simulationId is provided
  const {
    zones: realtimeZones,
    isConnected: isZonesConnected,
    totalZones: realtimeTotalZones,
    error: zonesError,
    clearZones,
  } = useEvacuationZones({
    simulationId: simulationId || '',
    enabled: enableRealtimeZones && !!simulationId && simState.isPlaying,
  });

  // Handle initialization and loading states
  useEffect(() => {
    const isLoading = (enableRealtime || enableRealtimeZones) && 
                      !!simulationId && 
                      simState.isPlaying && 
                      !isRealtimeConnected && 
                      !isZonesConnected &&
                      isInitializing;
    
    onLoadingChange?.(isLoading);
  }, [
    enableRealtime, 
    enableRealtimeZones, 
    simulationId, 
    simState.isPlaying, 
    isRealtimeConnected, 
    isZonesConnected,
    isInitializing,
    onLoadingChange
  ]);

  // Handle errors from subscriptions
  useEffect(() => {
    const error = particlesError || zonesError;
    if (error) {
      setConnectionError(error);
      onError?.(error);
      
      toast({
        title: 'Real-time Connection Error',
        description: error.message,
        variant: 'destructive',
        duration: 5000,
      });
    }
  }, [particlesError, zonesError, onError, toast]);

  // Reset initialization state when simulation starts/stops
  useEffect(() => {
    if (!simState.isPlaying) {
      setIsInitializing(true);
      setConnectionError(null);
      clearParticles?.();
      clearZones?.();
    }
  }, [simState.isPlaying, clearParticles, clearZones]);




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

  // Merge real-time zones with prop-provided zones
  const allZones = useMemo(() => {
    const merged = [...evacuationZones];
    if (enableRealtimeZones && realtimeZones.length > 0) {
      // Convert real-time zones to EvacuationZone format
      const converted = realtimeZones.map(z => ({
        id: z.id,
        name: z.name,
        severity: (['low', 'medium', 'high', 'critical'][z.level - 1] || 'low') as 'critical' | 'high' | 'medium' | 'low',
        contour: z.polygon.coordinates[0].map(coord => [coord[0], coord[1]] as [number, number]),
        doseRate: z.doseThreshold,
        population: z.population,
        instructions: getInstructionsForLevel(['low', 'medium', 'high', 'critical'][z.level - 1] || 'low'),
      }));
      merged.push(...converted);
    }
    return merged;
  }, [evacuationZones, realtimeZones, enableRealtimeZones]);

  // Generate evacuation zones from particles if not provided and no real-time data
  const generatedZones = useMemo((): EvacuationZone[] => {
    if (allZones.length > 0) return allZones;
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
  }, [allParticles, allZones, releaseLat, releaseLng, calculateDoseRate]);



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
    if (enableRealtime && simulationId && !connectionError) {
      if (isRealtimeConnected && !isInitializing) {
        toast({
          title: 'Real-time Streaming Connected',
          description: `Receiving live particle data (${realtimeTotal} particles)`,
          variant: 'default',
          duration: 3000,
        });
      }
    }
  }, [enableRealtime, simulationId, isRealtimeConnected, realtimeTotal, toast, isInitializing, connectionError]);

  // Show connection status toast for real-time zones
  useMemo(() => {
    if (enableRealtimeZones && simulationId && !connectionError) {
      if (isZonesConnected) {
        toast({
          title: 'Evacuation Zones Connected',
          description: `Receiving live zone updates (${realtimeTotalZones} zones)`,
          variant: 'default',
          duration: 3000,
        });
      }
    }
  }, [enableRealtimeZones, simulationId, isZonesConnected, realtimeTotalZones, toast, connectionError]);




  const layers = [];
  
  // Render loading overlay if initializing real-time connection
  if (isInitializing && (enableRealtime || enableRealtimeZones) && simState.isPlaying) {
    layers.push(new ScatterplotLayer({
      id: 'plume-loading-indicator',
      data: [{ position: [releaseLng, releaseLat] }],
      visible: visible,
      getPosition: (d: { position: [number, number] }) => d.position,
      getRadius: () => 50,
      getFillColor: () => [255, 165, 0, 100],
      getLineColor: () => [255, 165, 0, 200],
      lineWidthMinPixels: 2,
      stroked: true,
      filled: true,
      radiusMinPixels: 20,
      radiusMaxPixels: 100,
    }));
  }


  // Add evacuation zone polygons

  if (showEvacuationZones && generatedZones.length > 0) {
    layers.push(new PolygonLayer<EvacuationZone>({
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

  return (
    <>
      {/* Error Alert Overlay */}
      {connectionError && (
        <div className="absolute top-4 left-1/2 transform -translate-x-1/2 z-50 w-96">
          <Alert variant="error">
            <div className="flex items-center gap-2">
              <span className="font-semibold">Connection Error</span>
            </div>
            <p className="text-sm mt-1">{connectionError.message}</p>
            <button 
              onClick={() => setConnectionError(null)}
              className="text-xs underline mt-2 hover:text-red-300"
            >
              Dismiss
            </button>
          </Alert>
        </div>
      )}

      
      {/* Loading Indicator */}
      {isInitializing && (enableRealtime || enableRealtimeZones) && simState.isPlaying && (
        <div className="absolute bottom-4 right-4 z-50 flex items-center gap-2 bg-black/70 text-white px-3 py-2 rounded-md">
          <Spinner size="sm" />
          <span className="text-sm">Connecting to real-time data...</span>
        </div>
      )}
      
      {/* Connection Status Badge */}
      {(enableRealtime || enableRealtimeZones) && simState.isPlaying && !isInitializing && !connectionError && (
        <div className="absolute bottom-4 right-4 z-50 flex items-center gap-2">
          <div className={`w-2 h-2 rounded-full ${isRealtimeConnected || isZonesConnected ? 'bg-green-500' : 'bg-yellow-500'}`} />
          <span className="text-xs text-white bg-black/70 px-2 py-1 rounded">
            {isRealtimeConnected || isZonesConnected ? 'Live' : 'Connecting...'}
          </span>
        </div>
      )}
      
      {layers}
    </>
  );
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
