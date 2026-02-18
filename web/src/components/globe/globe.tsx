'use client';

import { useState, useCallback, useMemo } from 'react';
import { DeckGL } from '@deck.gl/react';
import { ScatterplotLayer } from '@deck.gl/layers';
import { HeatmapLayer } from '@deck.gl/aggregation-layers';
import type { MapViewState } from '@deck.gl/core';
import { useGlobeStore } from '@/stores';

import { Sensor, Anomaly, Facility, PlumeSimulation, Viewport } from '@/types';
import { getSeverityColor } from '@/lib/utils/calculations';

// Type declarations for deck.gl modules
declare module '@deck.gl/react';
declare module '@deck.gl/layers';
declare module '@deck.gl/aggregation-layers';
declare module '@deck.gl/core';

interface GlobeProps {

  sensors?: Sensor[];
  facilities?: Facility[];
  anomalies?: Anomaly[];
  plumes?: PlumeSimulation[];
  selectedSensorId?: string | null;
  viewport?: Viewport;
  layers?: {
    sensors: boolean;
    facilities: boolean;
    anomalies: boolean;
    plumes: boolean;
    heatmap: boolean;
  };
  onViewportChange?: (viewport: Viewport) => void;
  onSensorSelect?: (sensorId: string) => void;
}

const DEFAULT_VIEWPORT: Viewport = {
  longitude: 0,
  latitude: 20,
  zoom: 2,
  pitch: 30,
  bearing: 0,
};

const DEFAULT_LAYERS = {
  sensors: true,
  facilities: false,
  anomalies: true,
  plumes: false,
  heatmap: false,
};

export const Globe = ({ 
  sensors = [], 
  facilities: _facilities = [], 
  anomalies = [], 
  plumes: _plumes = [],
  selectedSensorId: _selectedSensorId = null,
  viewport = DEFAULT_VIEWPORT,
  layers: layerVisibility = DEFAULT_LAYERS,
  onViewportChange = () => {},
  onSensorSelect = () => {}
}: GlobeProps) => {

  const [viewState, setViewState] = useState<MapViewState>({

    longitude: viewport.longitude,
    latitude: viewport.latitude,
    zoom: viewport.zoom,
    pitch: viewport.pitch ?? 30,
    bearing: viewport.bearing ?? 0,
  });

  const handleViewStateChange = useCallback((params: { viewState: MapViewState }) => {
    setViewState(params.viewState);
    onViewportChange({
      longitude: params.viewState.longitude,
      latitude: params.viewState.latitude,
      zoom: params.viewState.zoom,
      pitch: params.viewState.pitch ?? 30,
      bearing: params.viewState.bearing ?? 0,
    });
  }, [onViewportChange]);



  const heatmapLayer = useMemo(() => {
    if (!layerVisibility.heatmap || sensors.length === 0) return null;
    
    return new HeatmapLayer({
      id: 'heatmap-layer',
      data: sensors,
      getPosition: (d: Sensor) => [d.location.lon, d.location.lat] as [number, number],
      getWeight: (d: Sensor) => d.lastReading?.doseRate ?? 0,
      radiusPixels: 50,
      intensity: 1,
      threshold: 0.05,
    });

  }, [sensors, layerVisibility.heatmap]);

  const sensorLayer = useMemo(() => {
    if (!layerVisibility.sensors) return null;

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
      getPosition: (d: Sensor) => [d.location.lon, d.location.lat] as [number, number],

      getRadius: (d: Sensor) => {
        if (d.id === _selectedSensorId) return 12;
        return d.status === 'active' ? 8 : 6;
      },
      getFillColor: (d: Sensor): [number, number, number, number] => {
        if (d.id === _selectedSensorId) {
          return [0, 212, 255, 255];
        }
        switch (d.status) {
          case 'active':
            return [0, 255, 136, 255];
          case 'inactive':
            return [160, 160, 176, 255];
          case 'maintenance':
            return [255, 184, 0, 255];
          case 'offline':
            return [255, 51, 102, 255];
          default:
            return [160, 160, 176, 255];
        }
      },
      getLineColor: (): [number, number, number, number] => [255, 255, 255, 255],
      getLineWidth: 2,
      onClick: (info: { object: Sensor | null; x: number; y: number }) => {
        if (info.object && onSensorSelect) {
          onSensorSelect(info.object.id);
        }
      },

      updateTriggers: {
        getFillColor: [_selectedSensorId],
        getRadius: [_selectedSensorId],
      },
    });
  }, [sensors, _selectedSensorId, onSensorSelect, layerVisibility.sensors]);

  const anomalyLayer = useMemo(() => {
    if (!layerVisibility.anomalies) return null;

    return new ScatterplotLayer({
      id: 'anomaly-layer',
      data: anomalies,
      pickable: true,
      opacity: 0.9,
      stroked: false,
      filled: true,
      radiusMinPixels: 6,
      radiusMaxPixels: 20,
      getPosition: (d: Anomaly) => [d.location.lon, d.location.lat] as [number, number],

      getRadius: (d: Anomaly) => {
        switch (d.severity) {
          case 'critical':
            return 16;
          case 'high':
            return 12;
          case 'medium':
            return 10;
          case 'low':
          default:
            return 8;
        }
      },
      getFillColor: (d: Anomaly): [number, number, number, number] => {
        const color = getSeverityColor(d.severity);
        const hex = color.replace('#', '');
        const r = parseInt(hex.substring(0, 2), 16);
        const g = parseInt(hex.substring(2, 4), 16);
        const b = parseInt(hex.substring(4, 6), 16);
        return [r, g, b, 200];
      },
    });
  }, [anomalies, layerVisibility.anomalies]);

  const deckLayers = [heatmapLayer, sensorLayer, anomalyLayer].filter(Boolean);

  return (
    <div data-testid="globe-container" className="relative w-full h-full bg-bg-primary">
      <DeckGL
        viewState={viewState}
        onViewStateChange={handleViewStateChange}
        controller={true}
        layers={deckLayers}
        getTooltip={({ object }: { object?: unknown }) => {
          if (!object) return null;
          if (object && typeof object === 'object' && 'lastReading' in object) {
            const sensor = object as Sensor;
            return {
              text: `${sensor.name}\nDose: ${sensor.lastReading?.doseRate.toFixed(3)} μSv/h`,
            };
          }
          if (object && typeof object === 'object' && 'severity' in object) {
            const anomaly = object as Anomaly;
            return {
              text: `Anomaly: ${anomaly.severity}\n${anomaly.message}`,
            };
          }
          return null;
        }}
        style={{ background: '#050508' }}
      />
      
      {/* Layer Controls */}
      <div className="absolute top-4 left-4 flex flex-col gap-2 bg-bg-secondary/90 backdrop-blur-md p-3 rounded-lg border border-border-subtle">
        <span className="text-heading-xs text-text-secondary mb-2">LAYERS</span>
        {Object.entries(layerVisibility).map(([key, enabled]) => (
          <button
            key={key}
            onClick={() => useGlobeStore.getState().toggleLayer(key as keyof typeof layerVisibility)}
            className={`flex items-center gap-2 px-3 py-2 rounded-md text-body-sm transition-all ${
              enabled 
                ? 'bg-accent-primary/20 text-accent-primary border border-accent-primary/30' 
                : 'text-text-secondary hover:bg-bg-hover'
            }`}
          >
            <div 
              className={`w-2 h-2 rounded-full ${enabled ? 'bg-accent-primary' : 'bg-text-tertiary'}`} 
            />
            {key.charAt(0).toUpperCase() + key.slice(1)}
          </button>
        ))}
      </div>

      {/* Viewport Info */}
      <div className="absolute bottom-4 left-4 bg-bg-secondary/90 backdrop-blur-md px-3 py-2 rounded-md border border-border-subtle">
        <span className="text-mono-xs text-text-tertiary">
          {viewState.latitude.toFixed(4)}°N, {viewState.longitude.toFixed(4)}°E | Zoom: {viewState.zoom.toFixed(1)}
        </span>
      </div>
    </div>
  );
};
