import React, { useEffect, useRef, useCallback } from 'react';
import { useAppStore } from '../../stores/useAppStore';
import type { RadiationGlobe } from '../../../wasm/pkg/cherenkov_web';
import { TimeSlider } from './TimeSlider';



const INITIAL_VIEW = {
  lat: 20,
  lon: 0,
  zoom: 3.0,
};

export const GlobeContainer: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const globeRef = useRef<RadiationGlobe | null>(null);
  const animationRef = useRef<number>(0);

  
  const sensors = useAppStore((state) => state.sensors);
  const layers = useAppStore((state) => state.globe.layers);
  const viewport = useAppStore((state) => state.globe.viewport);
  const setGlobeViewport = useAppStore((state) => state.setGlobeViewport);
  const selectSensor = useAppStore((state) => state.selectSensor);

  // Initialize WASM and WebGL
  useEffect(() => {
    let isMounted = true;
    
    const initGlobe = async () => {
      if (!canvasRef.current) return;
      
      try {
        // Dynamic import of WASM module
        const wasmModule = await import('../../../wasm/pkg');
        if (!isMounted) return;
        
        // Initialize WASM
        await wasmModule.default();
        if (!isMounted) return;
        
        // Create globe instance
        const globe = new wasmModule.RadiationGlobe(canvasRef.current);
        globeRef.current = globe;

        
        // Set initial view
        globe.setView(INITIAL_VIEW.lat, INITIAL_VIEW.lon, INITIAL_VIEW.zoom);
        
        // Start render loop
        const renderLoop = () => {
          if (globeRef.current) {
            globeRef.current.render();
          }
          animationRef.current = requestAnimationFrame(renderLoop);
        };
        renderLoop();
        
      } catch (error) {
        console.error('Failed to initialize WebGL globe:', error);
      }
    };
    
    initGlobe();
    
    return () => {
      isMounted = false;
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
      globeRef.current = null;
    };
  }, []);

  // Handle resize
  useEffect(() => {
    const handleResize = () => {
      if (canvasRef.current && globeRef.current) {
        const { width, height } = canvasRef.current.getBoundingClientRect();
        canvasRef.current.width = width;
        canvasRef.current.height = height;
        globeRef.current.resize(width, height);
      }
    };
    
    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  // Update sensors in WASM
  useEffect(() => {
    if (!globeRef.current) return;
    
    Object.values(sensors).forEach((sensor) => {
      globeRef.current?.updateSensor(
        sensor.id,
        sensor.location.lat,
        sensor.location.lon,
        sensor.doseRate
      );
    });
  }, [sensors]);

  // Update layer visibility
  useEffect(() => {
    if (!globeRef.current) return;
    
    Object.entries(layers).forEach(([layer, visible]) => {
      globeRef.current?.setLayerVisibility(layer, visible);
    });
  }, [layers]);

  // Handle viewport changes from store
  useEffect(() => {
    if (!globeRef.current) return;
    globeRef.current.setView(viewport.latitude, viewport.longitude, viewport.zoom);
  }, [viewport]);

  // Mouse interaction handlers
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    const startX = e.clientX;
    const startY = e.clientY;
    const startLat = viewport.latitude;
    const startLon = viewport.longitude;
    
    const handleMouseMove = (moveEvent: MouseEvent) => {
      const deltaX = moveEvent.clientX - startX;
      const deltaY = moveEvent.clientY - startY;
      
      // Convert pixel delta to lat/lon delta
      const sensitivity = 0.1;
      const newLon = startLon - deltaX * sensitivity;
      const newLat = Math.max(-85, Math.min(85, startLat + deltaY * sensitivity));
      
      setGlobeViewport({
        latitude: newLat,
        longitude: newLon,
      });
    };
    
    const handleMouseUp = () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
    
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }, [viewport, setGlobeViewport]);

  const handleWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.1 : -0.1;
    const newZoom = Math.max(1.5, Math.min(10, viewport.zoom + delta));
    setGlobeViewport({ zoom: newZoom });
  }, [viewport.zoom, setGlobeViewport]);

  const resetView = useCallback(() => {
    setGlobeViewport({
      latitude: INITIAL_VIEW.lat,
      longitude: INITIAL_VIEW.lon,
      zoom: INITIAL_VIEW.zoom,
    });
  }, [setGlobeViewport]);

  return (
    <div className="absolute inset-0 bg-bg-primary overflow-hidden">
      <canvas
        ref={canvasRef}
        className="w-full h-full cursor-grab active:cursor-grabbing"
        onMouseDown={handleMouseDown}
        onWheel={handleWheel}
        aria-label="3D radiation monitoring globe"
      />
      
      {/* Overlay controls */}
      <div className="absolute bottom-4 left-4 z-10 flex flex-col gap-2">
        <button
          onClick={resetView}
          className="p-2 bg-bg-secondary/80 backdrop-blur border border-border-subtle rounded-lg hover:bg-bg-hover transition-colors"
          title="Reset view"
          aria-label="Reset globe view"
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
            aria-pressed={enabled ? "true" : "false"}

            aria-label={`Toggle ${key.replace(/-/g, ' ')} layer`}
          >
            {key.replace(/-/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
          </button>
        ))}
      </div>
      
      {/* Time Slider */}
      <TimeSlider />

      {/* Coordinates display */}
      <div className="absolute bottom-4 right-4 z-10 px-3 py-2 bg-bg-secondary/80 backdrop-blur border border-border-subtle rounded-lg">
        <div className="text-xs text-text-secondary font-mono">
          <div>Lat: {viewport.latitude.toFixed(4)}</div>
          <div>Lon: {viewport.longitude.toFixed(4)}</div>
          <div>Zoom: {viewport.zoom.toFixed(2)}</div>
        </div>
      </div>
    </div>
  );
};
