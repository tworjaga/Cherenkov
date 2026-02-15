import { onMount, createSignal } from 'solid-js';
import init, { RadiationGlobe } from '../../wasm/pkg';

function GlobeView() {
  let canvasRef: HTMLCanvasElement;
  const [globe, setGlobe] = createSignal<RadiationGlobe | null>(null);
  const [loading, setLoading] = createSignal(true);

  onMount(async () => {
    await init();
    const g = new RadiationGlobe(canvasRef);
    setGlobe(g);
    setLoading(false);

    // Connect to WebSocket for real-time updates
    const ws = new WebSocket('ws://localhost:8080/ws');
    ws.onmessage = (event) => {
      const update = JSON.parse(event.data);
      g.updateSensor(update.id, update.lat, update.lon, update.value);
    };
  });

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
            <span class="text-sm text-gray-400">Normal (<0.5 μSv/h)</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-yellow-500"></span>
            <span class="text-sm text-gray-400">Elevated (0.5-2 μSv/h)</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-red-500"></span>
            <span class="text-sm text-gray-400">Critical (>2 μSv/h)</span>
          </div>
        </div>
      </div>
      
      <div class="flex-1 relative bg-[#0a0a0f] rounded-xl border border-[#2a2a3a] overflow-hidden">
        {loading() && (
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="w-12 h-12 border-4 border-[#00d4ff]/30 border-t-[#00d4ff] rounded-full animate-spin mb-4"></div>
              <p class="text-gray-400">Initializing WebGL Globe...</p>
            </div>
          </div>
        )}
        <canvas 
          ref={canvasRef} 
          class="w-full h-full cursor-grab active:cursor-grabbing"
        />
      </div>
    </div>
  );
}

export default GlobeView;
