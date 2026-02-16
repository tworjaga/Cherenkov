import { onMount, createSignal, createEffect, Show } from 'solid-js';
import init, { RadiationGlobe } from '../../wasm/pkg';
import { createGraphQLQuery, createGraphQLSubscription, type Sensor } from '../graphql/client';

const SENSORS_QUERY = `
  query SensorsInRegion($region: GeoPolygon!) {
    sensorsInRegion(region: $region) {
      id
      name
      location {
        lat
        lon
      }
      doseRate
      unit
      lastReading
      status
      source
    }
  }
`;

const SENSOR_SUBSCRIPTION = `
  subscription OnSensorUpdate {
    sensorUpdated {
      id
      location {
        lat
        lon
      }
      doseRate
      status
    }
  }
`;

interface GlobeState {
  zoom: number;
  rotationX: number;
  rotationY: number;
  isDragging: boolean;
  lastMouseX: number;
  lastMouseY: number;
  lastPinchDistance?: number;
}

interface SelectedSensor extends Sensor {
  distance?: number;
}

function GlobeView() {
  let canvasRef: HTMLCanvasElement | undefined;
  let globe: RadiationGlobe | undefined;
  
  const [loading, setLoading] = createSignal(true);
  const [selectedSensor, setSelectedSensor] = createSignal<SelectedSensor | null>(null);
  const [hoveredSensor, setHoveredSensor] = createSignal<string | null>(null);
  const [globeState, setGlobeState] = createSignal<GlobeState>({
    zoom: 1.0,
    rotationX: 0,
    rotationY: 0,
    isDragging: false,
    lastMouseX: 0,
    lastMouseY: 0,
  });
  
  // Plume simulation state (used in double-click handler and sensor panel)
  const [, setShowPlume] = createSignal(false);
  const [, setPlumeLocation] = createSignal<{lat: number; lon: number} | null>(null);

  const sensors = createGraphQLQuery<{ sensorsInRegion: Sensor[] }>(SENSORS_QUERY, () => ({
    region: {
      coordinates: [[-90, -180], [90, 180]]
    }
  }));
  
  const sensorUpdate = createGraphQLSubscription<Sensor>(SENSOR_SUBSCRIPTION);

  onMount(async () => {
    await init();
    
    if (canvasRef) {
      globe = new RadiationGlobe(canvasRef);
      setLoading(false);
      
      setupInteractionHandlers();
    }
  });

  createEffect(() => {
    const data = sensors();
    if (data && globe) {
      data.sensorsInRegion.forEach(sensor => {
        // @ts-ignore - WASM method
        globe?.updateSensor?.(
          sensor.id,
          sensor.location.lat,
          sensor.location.lon,
          sensor.doseRate
        );
      });
    }
  });

  createEffect(() => {
    const update = sensorUpdate();
    if (update && globe) {
      // @ts-ignore - WASM method
      globe.updateSensor?.(
        update.id,
        update.location.lat,
        update.location.lon,
        update.doseRate
      );
    }
  });

  const setupInteractionHandlers = () => {
    if (!canvasRef) return;

    canvasRef.addEventListener('mousedown', handleMouseDown);
    canvasRef.addEventListener('mousemove', handleMouseMove);
    canvasRef.addEventListener('mouseup', handleMouseUp);
    canvasRef.addEventListener('mouseleave', handleMouseUp);
    canvasRef.addEventListener('wheel', handleWheel, { passive: false });
    canvasRef.addEventListener('click', handleClick);
    canvasRef.addEventListener('dblclick', handleDoubleClick);
    
    canvasRef.addEventListener('touchstart', handleTouchStart, { passive: false });
    canvasRef.addEventListener('touchmove', handleTouchMove, { passive: false });
    canvasRef.addEventListener('touchend', handleMouseUp);
  };

  const handleMouseDown = (e: MouseEvent) => {
    setGlobeState(prev => ({
      ...prev,
      isDragging: true,
      lastMouseX: e.clientX,
      lastMouseY: e.clientY,
    }));
    canvasRef?.classList.add('cursor-grabbing');
  };

  const handleMouseMove = (e: MouseEvent) => {
    const state = globeState();
    
    if (state.isDragging && globe) {
      const deltaX = e.clientX - state.lastMouseX;
      const deltaY = e.clientY - state.lastMouseY;
      
      const newRotationY = state.rotationY + deltaX * 0.5;
      const newRotationX = Math.max(-90, Math.min(90, state.rotationX - deltaY * 0.5));
      
      setGlobeState(prev => ({
        ...prev,
        rotationX: newRotationX,
        rotationY: newRotationY,
        lastMouseX: e.clientX,
        lastMouseY: e.clientY,
      }));
      
      // @ts-ignore - WASM method
      globe?.setRotation?.(newRotationX, newRotationY);
    } else {
      const rect = canvasRef!.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      
      // @ts-ignore - WASM method
      const sensorId = globe?.pickSensor?.(x, y);
      setHoveredSensor(sensorId || null);
      canvasRef!.style.cursor = sensorId ? 'pointer' : state.isDragging ? 'grabbing' : 'grab';
    }
  };

  const handleMouseUp = () => {
    setGlobeState(prev => ({ ...prev, isDragging: false }));
    canvasRef?.classList.remove('cursor-grabbing');
  };

  const handleWheel = (e: WheelEvent) => {
    e.preventDefault();
    
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    setGlobeState(prev => {
      const newZoom = Math.max(0.5, Math.min(5.0, prev.zoom * delta));
      // @ts-ignore - WASM method
      globe?.setZoom?.(newZoom);
      return { ...prev, zoom: newZoom };
    });
  };

  const handleClick = (e: MouseEvent) => {
    if (globeState().isDragging) return;
    
    const rect = canvasRef!.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    // @ts-ignore - WASM method
    const sensorId = globe?.pickSensor?.(x, y);
    if (sensorId) {
      const sensorData = sensors()?.sensorsInRegion.find(s => s.id === sensorId);
      if (sensorData) {
        setSelectedSensor(sensorData);
      }
    } else {
      setSelectedSensor(null);
    }
  };

  const handleDoubleClick = (e: MouseEvent) => {
    const rect = canvasRef!.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    // @ts-ignore - WASM method
    const coords = globe?.screenToLatLon?.(x, y);
    if (coords) {
      setPlumeLocation({ lat: coords[0], lon: coords[1] });
      setShowPlume(true);
    }
  };

  const handleTouchStart = (e: TouchEvent) => {
    if (e.touches.length === 1) {
      const touch = e.touches[0];
      setGlobeState(prev => ({
        ...prev,
        isDragging: true,
        lastMouseX: touch.clientX,
        lastMouseY: touch.clientY,
      }));
    } else if (e.touches.length === 2) {
      const touch1 = e.touches[0];
      const touch2 = e.touches[1];
      const distance = Math.hypot(
        touch2.clientX - touch1.clientX,
        touch2.clientY - touch1.clientY
      );
      setGlobeState(prev => ({ ...prev, lastPinchDistance: distance }));
    }
  };

  const handleTouchMove = (e: TouchEvent) => {
    e.preventDefault();
    const state = globeState();
    
    if (e.touches.length === 1 && state.isDragging && globe) {
      const touch = e.touches[0];
      const deltaX = touch.clientX - state.lastMouseX;
      const deltaY = touch.clientY - state.lastMouseY;
      
      const newRotationY = state.rotationY + deltaX * 0.5;
      const newRotationX = Math.max(-90, Math.min(90, state.rotationX - deltaY * 0.5));
      
      setGlobeState(prev => ({
        ...prev,
        rotationX: newRotationX,
        rotationY: newRotationY,
        lastMouseX: touch.clientX,
        lastMouseY: touch.clientY,
      }));
      
      // @ts-ignore - WASM method
      globe?.setRotation?.(newRotationX, newRotationY);
    } else if (e.touches.length === 2) {
      const touch1 = e.touches[0];
      const touch2 = e.touches[1];
      const distance = Math.hypot(
        touch2.clientX - touch1.clientX,
        touch2.clientY - touch1.clientY
      );
      
      if (state.lastPinchDistance) {
        const scale = distance / state.lastPinchDistance;
        setGlobeState(prev => {
          const newZoom = Math.max(0.5, Math.min(5.0, prev.zoom * scale));
          // @ts-ignore - WASM method
          globe?.setZoom?.(newZoom);
          return { ...prev, zoom: newZoom, lastPinchDistance: distance };
        });
      }
    }
  };

  const resetView = () => {
    setGlobeState({
      zoom: 1.0,
      rotationX: 0,
      rotationY: 0,
      isDragging: false,
      lastMouseX: 0,
      lastMouseY: 0,
    });
    // @ts-ignore - WASM method
    globe?.setZoom?.(1.0);
    // @ts-ignore - WASM method
    globe?.setRotation?.(0, 0);
  };

  const zoomToSensor = (sensor: Sensor) => {
    if (!globe) return;
    
    // @ts-ignore - WASM method
    globe?.setRotation?.(sensor.location.lat, sensor.location.lon);
    setGlobeState(prev => ({
      ...prev,
      rotationX: sensor.location.lat,
      rotationY: sensor.location.lon,
      zoom: 2.0,
    }));
    // @ts-ignore - WASM method
    globe?.setZoom?.(2.0);
  };

  return (
    <div class="h-full flex flex-col">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h1 class="text-2xl font-bold">Global Radiation Map</h1>
          <p class="text-gray-400">Real-time sensor data from 50,000+ locations</p>
        </div>
        <div class="flex items-center gap-4">
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-green-500"></span>
            <span class="text-sm text-gray-400">{'<0.5 μSv/h Normal'}</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-yellow-500"></span>
            <span class="text-sm text-gray-400">0.5-2 μSv/h Elevated</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-red-500"></span>
            <span class="text-sm text-gray-400">{'2+ μSv/h Critical'}</span>
          </div>
          <button
            onClick={resetView}
            class="px-3 py-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] rounded text-sm transition-colors"
          >
            Reset View
          </button>
        </div>
      </div>
      
      <div class="flex-1 relative bg-[#0a0a0f] rounded-xl border border-[#2a2a3a] overflow-hidden">
        <Show when={loading()}>
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="w-12 h-12 border-4 border-[#00d4ff]/30 border-t-[#00d4ff] rounded-full animate-spin mb-4"></div>
              <p class="text-gray-400">Initializing WebGL Globe...</p>
            </div>
          </div>
        </Show>
        
        <canvas 
          ref={canvasRef} 
          class="w-full h-full cursor-grab active:cursor-grabbing"
        />
        
        <Show when={selectedSensor()}>
          <div class="absolute top-4 right-4 w-80 bg-[#12121a]/95 backdrop-blur rounded-xl border border-[#2a2a3a] p-4 shadow-xl">
            <div class="flex items-center justify-between mb-3">
              <h3 class="font-semibold text-lg">{(selectedSensor() as Sensor).name}</h3>
              <button 
                onClick={() => setSelectedSensor(null)}
                class="text-gray-400 hover:text-white"
              >
                ✕
              </button>
            </div>
            
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-gray-400">ID:</span>
                <span>{(selectedSensor() as Sensor).id}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Location:</span>
                <span>
                  {(selectedSensor() as Sensor).location.lat.toFixed(4)}°, {(selectedSensor() as Sensor).location.lon.toFixed(4)}°
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Dose Rate:</span>
                <span class={(selectedSensor() as Sensor).doseRate > 2 ? 'text-red-400' : (selectedSensor() as Sensor).doseRate > 0.5 ? 'text-yellow-400' : 'text-green-400'}>
                  {(selectedSensor() as Sensor).doseRate.toFixed(3)} {(selectedSensor() as Sensor).unit}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Status:</span>
                <span class={`capitalize ${(selectedSensor() as Sensor).status === 'alert' ? 'text-red-400' : (selectedSensor() as Sensor).status === 'inactive' ? 'text-gray-400' : 'text-green-400'}`}>
                  {(selectedSensor() as Sensor).status}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Source:</span>
                <span>{(selectedSensor() as Sensor).source}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Last Reading:</span>
                <span>{new Date((selectedSensor() as Sensor).lastReading).toLocaleString()}</span>
              </div>
            </div>
            
            <div class="mt-4 flex gap-2">
              <button
                onClick={() => zoomToSensor(selectedSensor() as Sensor)}
                class="flex-1 px-3 py-2 bg-[#00d4ff]/20 hover:bg-[#00d4ff]/30 text-[#00d4ff] rounded text-sm transition-colors"
              >
                Zoom to Sensor
              </button>
              <button
                onClick={() => {
                  setPlumeLocation({
                    lat: (selectedSensor() as Sensor).location.lat,
                    lon: (selectedSensor() as Sensor).location.lon
                  });
                  setShowPlume(true);
                }}
                class="flex-1 px-3 py-2 bg-[#2a2a3a] hover:bg-[#3a3a4a] rounded text-sm transition-colors"
              >
                Simulate Plume
              </button>
            </div>
          </div>
        </Show>
        
        <Show when={hoveredSensor() && !selectedSensor()}>
          <div class="absolute bottom-4 left-4 bg-[#12121a]/95 backdrop-blur px-3 py-2 rounded-lg border border-[#2a2a3a] text-sm">
            Click to view sensor details
          </div>
        </Show>
        
        <div class="absolute bottom-4 right-4 flex flex-col gap-2">
          <button
            onClick={() => setGlobeState(prev => {
              const newZoom = Math.min(5.0, prev.zoom * 1.2);
              // @ts-ignore - WASM method
              globe?.setZoom?.(newZoom);
              return { ...prev, zoom: newZoom };
            })}
            class="w-10 h-10 bg-[#12121a]/95 backdrop-blur rounded-lg border border-[#2a2a3a] flex items-center justify-center hover:bg-[#2a2a3a] transition-colors"
          >
            +
          </button>
          <button
            onClick={() => setGlobeState(prev => {
              const newZoom = Math.max(0.5, prev.zoom / 1.2);
              // @ts-ignore - WASM method
              globe?.setZoom?.(newZoom);
              return { ...prev, zoom: newZoom };
            })}
            class="w-10 h-10 bg-[#12121a]/95 backdrop-blur rounded-lg border border-[#2a2a3a] flex items-center justify-center hover:bg-[#2a2a3a] transition-colors"
          >
            −
          </button>
        </div>
        
        <div class="absolute bottom-4 left-4 text-xs text-gray-500 bg-[#0a0a0f]/80 px-3 py-2 rounded">
          <p>Drag to rotate • Scroll to zoom • Click sensor for details • Double-click to simulate plume</p>
        </div>
      </div>
    </div>
  );
}

export default GlobeView;
