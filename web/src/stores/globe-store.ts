import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { Viewport } from '@/types';

interface GlobeLayers {
  sensors: boolean;
  facilities: boolean;
  anomalies: boolean;
  plumes: boolean;
  heatmap: boolean;
}

interface HoveredFeature {
  type: 'sensor' | 'facility' | 'anomaly';
  id: string;
}

interface GlobeState {
  viewport: Viewport;
  layers: GlobeLayers;
  hoveredFeature: HoveredFeature | null;
  isFlying: boolean;
  timeRange: [number, number] | null;

  setViewport: (viewport: Partial<Viewport>) => void;
  setViewportComplete: (viewport: Viewport) => void;
  toggleLayer: (layer: keyof GlobeLayers) => void;
  setHoveredFeature: (feature: HoveredFeature | null) => void;
  flyTo: (location: { lat: number; lon: number }, zoom?: number) => void;
  resetView: () => void;
  setTimeRange: (range: [number, number] | null) => void;
}


const defaultViewport: Viewport = {
  latitude: 20,
  longitude: 0,
  zoom: 2,
  pitch: 0,
  bearing: 0,
};

export const useGlobeStore = create<GlobeState>()(
  persist(
    (set) => ({
      viewport: defaultViewport,
      layers: {
        sensors: true,
        facilities: true,
        anomalies: true,
        plumes: false,
        heatmap: false,
      },
      hoveredFeature: null,
      isFlying: false,
      timeRange: null,

      setViewport: (viewport) =>

        set((state) => ({
          viewport: { ...state.viewport, ...viewport },
        })),

      setViewportComplete: (viewport) =>
        set({
          viewport,
          isFlying: false,
        }),

      toggleLayer: (layer) =>
        set((state) => ({
          layers: {
            ...state.layers,
            [layer]: !state.layers[layer],
          },
        })),

      setHoveredFeature: (feature) => set({ hoveredFeature: feature }),

      flyTo: (location, zoom = 8) => {
        set({
          isFlying: true,
          viewport: {
            latitude: location.lat,
            longitude: location.lon,
            zoom,
            pitch: 45,
            bearing: 0,
          },
        });
      },

      resetView: () =>
        set({
          viewport: defaultViewport,
          isFlying: false,
        }),

      setTimeRange: (range) => set({ timeRange: range }),
    }),

    {
      name: 'cherenkov-globe-storage',
      partialize: (state) => ({
        layers: state.layers,
      }),
    }
  )
);
