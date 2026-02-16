import React, { useEffect, useRef, useState } from 'react';
import { DeckGL } from '@deck.gl/react';
import { MapViewState } from '@deck.gl/core';
import { ScatterplotLayer } from '@deck.gl/layers';
import { useAppStore } from '../../stores/useAppStore';

const INITIAL_VIEW_STATE: MapViewState = {
  latitude: 20,
  longitude: 0,
  zoom: 2,
  pitch: 30,
  bearing: 0,
};

export const GlobeContainer: React.FC = () => {
  const [viewState, setViewState] = useState<MapViewState>(INITIAL_VIEW_STATE);
  const sensors = useAppStore((state) => state.sensors);
  const selectSensor = useAppStore((state) => state.selectSensor);
  const layers = useAppStore((state) => state.globe.layers);

  const sensorData = Object.values(sensors).map((sensor) => ({
    position: [sensor.location.lon, sensor.location.lat],
    radius: 50000,
    color: sensor.status === 'active' ? [0, 212, 255] : [160, 160, 176],
    sensorId: sensor.id,
  }));

  const sensorLayer = layers['sensor-points']
    ? new ScatterplotLayer({
        id: 'sensors',
        data: sensorData,
        getPosition: (d) => d.position,
        getRadius: (d) => d.radius,
        getFillColor: (d) => d.color,
        pickable: true,
        onClick: (info) => {
          if (info.object) {
            selectSensor(info.object.sensorId);
          }
        },
      })
    : null;

  return (
    <div className="absolute inset-0 bg-bg-primary">
      <DeckGL
        initialViewState={viewState}
        onViewStateChange={(params) => setViewState(params.viewState)}
        controller={true}
        layers={sensorLayer ? [sensorLayer] : []}
        getTooltip={({ object }) =>
          object ? `Sensor: ${object.sensorId}` : null
        }
      />
      
      {/* Overlay controls */}
      <div className="absolute bottom-4 left-4 z-10 flex flex-col gap-2">
        <button
          onClick={() => setViewState(INITIAL_VIEW_STATE)}
          className="p-2 bg-bg-secondary/80 backdrop-blur border border-border-subtle rounded-lg hover:bg-bg-hover transition-colors"
          title="Reset view"
        >
          <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </button>
      </div>

      {/* Layer toggles */}
      <div className="absolute top-4 right-4 z-10 flex flex-col gap-2">
        {Object.entries(layers).map(([key, enabled]) => (
          <button
            key={key}
            onClick={() => useAppStore.getState().toggleLayer(key)}
            className={`px-3 py-1.5 text-xs font-medium rounded-lg border transition-colors ${
              enabled
                ? 'bg-accent-primary/20 border-accent-primary/50 text-accent-primary'
                : 'bg-bg-secondary/80 border-border-subtle text-text-secondary'
            }`}
          >
            {key.replace(/-/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
          </button>
        ))}
      </div>
    </div>
  );
};
