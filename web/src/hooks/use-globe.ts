'use client';

import { useCallback, useState } from 'react';

interface GlobeState {
  isRotating: boolean;
  zoom: number;
  center: [number, number];
}

export function useGlobe() {
  const [state, setState] = useState<GlobeState>({
    isRotating: false,
    zoom: 1,
    center: [0, 0],
  });

  const toggleRotation = useCallback(() => {
    setState((prev) => ({ ...prev, isRotating: !prev.isRotating }));
  }, []);

  const resetView = useCallback(() => {
    setState({
      isRotating: false,
      zoom: 1,
      center: [0, 0],
    });
  }, []);

  const setZoom = useCallback((zoom: number) => {
    setState((prev) => ({ ...prev, zoom }));
  }, []);

  const setCenter = useCallback((center: [number, number]) => {
    setState((prev) => ({ ...prev, center }));
  }, []);

  return {
    isRotating: state.isRotating,
    zoom: state.zoom,
    center: state.center,
    toggleRotation,
    resetView,
    setZoom,
    setCenter,
  };
}
