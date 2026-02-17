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

interface PickingInfo {
  object?: unknown;
  x: number;
  y: number;
}


const _INITIAL_VIEW_STATE: MapViewState = {
  longitude: 0,
  latitude: 20,
  zoom: 2,
  pitch: 30,
  bearing: 0,
};

interface GlobeProps {

  sensors: Sensor[];
  facilities: Facility[];
  anomalies: Anomaly[];
  plumes: PlumeSimulation[];
  selectedSensorId: string | null;
  viewport: Viewport;
  layers: {
    sensors: boolean;
    facilities: boolean;
    anomalies: boolean;
    plumes: boolean;
    heatmap: boolean;
  };
  onViewportChange: (viewport: Viewport) => void;
  onSensorSelect: (sensorId: string) => void;
}

export const Globe = ({ 
  sensors, 
  facilities: _facilities, 
  anomalies, 
  plumes: _plumes,
  selectedSensorId: _selectedSensorId,
  viewport,
  layers: layerVisibility,
  onViewportChange,
  onSensorSelect
}: GlobeProps) => {


  const { setHoveredFeature } = useGlobeStore();
  const [viewState, setViewState] = useState<MapViewState>({
    longitude: viewport.longitude,
    latitude: viewport.latitude,
    zoom: viewport.zoom,
    pitch: viewport.pitch,
    bearing: viewport.bearing,
  });

  const handleViewStateChange = useCallback((params: { viewState: MapViewState }) => {
    setViewState(params.viewState);
    onViewportChange({
      longitude: params.viewState.longitude,
      latitude: params.viewState.latitude,
      zoom: params.viewState.zoom,
      pitch: params.viewState.pitch,
      bearing: params.viewState.bearing,
    });
  }, [onViewportChange]);


  const sensorLayer = useMemo(() => {
    if (!layerVisibility.sensors) return null;

    
    return new ScatterplotLayer({
      id: 'sensor-layer',
      data: sensors,
      getPosition: (d: Sensor) => [d.location.lon, d.location.lat],

      getRadius: (d: Sensor) => {
        const reading = d.lastReading?.doseRate || 0;
        return Math.max(8, Math.min(16, reading * 2));
      },
      getFillColor: (d: Sensor) => {
        const reading = d.lastReading?.doseRate || 0;
        if (reading > 10) return [255, 51, 102]; // critical
        if (reading > 5) return [255, 107, 53]; // high
        if (reading > 2) return [255, 184, 0]; // medium
        if (reading > 0.5) return [0, 212, 255]; // low
        return [0, 255, 136]; // normal
      },
      getLineColor: [0, 212, 255],
      lineWidthMinPixels: 1,
      stroked: true,
      filled: true,
      pickable: true,
      onHover: (info: PickingInfo) => {
        if (info.object) {
          setHoveredFeature({ type: 'sensor', id: (info.object as Sensor).id });
        } else {
          setHoveredFeature(null);
        }
      },
      onClick: (info: PickingInfo) => {
        if (info.object) {
          onSensorSelect((info.object as Sensor).id);
        }
      },


      updateTriggers: {
        getFillColor: [sensors],
        getRadius: [sensors],
      },
      transitions: {
        getFillColor: 300,
        getRadius: 300,
      },
    });
  }, [sensors, layerVisibility.sensors, setHoveredFeature, onSensorSelect]);


  const anomalyLayer = useMemo(() => {
    if (!layerVisibility.anomalies) return null;

    
    return new ScatterplotLayer({
      id: 'anomaly-layer',
      data: anomalies,
      getPosition: (d: Anomaly) => [d.location.lon, d.location.lat],

      getRadius: 20,
      getFillColor: (d: Anomaly) => {
        const color = getSeverityColor(d.severity);
        const hex = color.replace('#', '');
        const r = parseInt(hex.slice(0, 2), 16);
        const g = parseInt(hex.slice(2, 4), 16);
        const b = parseInt(hex.slice(4, 6), 16);
        return [r, g, b, 200];
      },
      getLineColor: [255, 255, 255],
      lineWidthMinPixels: 2,
      stroked: true,
      filled: true,
      pickable: true,
      onHover: (info: PickingInfo) => {
        if (info.object) {
          setHoveredFeature({ type: 'anomaly', id: (info.object as Anomaly).id });
        } else {
          setHoveredFeature(null);
        }
      },
    });
  }, [anomalies, layerVisibility.anomalies, setHoveredFeature]);



  const heatmapLayer = useMemo(() => {
    if (!layerVisibility.heatmap) return null;

    
    return new HeatmapLayer({
      id: 'heatmap-layer',
      data: sensors,
      getPosition: (d: Sensor) => [d.location.lon, d.location.lat],

      getWeight: (d: Sensor) => d.lastReading?.doseRate || 0,
      radiusPixels: 50,
      intensity: 1,
      threshold: 0.05,
      colorRange: [
        [0, 255, 136],
        [0, 212, 255],
        [255, 184, 0],
        [255, 107, 53],
        [255, 51, 102],
      ],
    });
  }, [sensors, layerVisibility.heatmap]);


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
