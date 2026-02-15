import { createSignal } from 'solid-js';

function PlumeSimulator() {
  const [params, setParams] = createSignal({
    latitude: 35.6762,
    longitude: 139.6503,
    releaseHeight: 100,
    releaseRate: 1000,
    duration: 72,
    isotope: 'cs137',
  });

  const [simulating, setSimulating] = createSignal(false);

  const runSimulation = () => {
    setSimulating(true);
    // In production, this would call the GraphQL API
    setTimeout(() => setSimulating(false), 3000);
  };

  return (
    <div class="space-y-6">
      <div>
        <h1 class="text-2xl font-bold">Atmospheric Dispersion Simulator</h1>
        <p class="text-gray-400 mt-1">Lagrangian particle dispersion modeling for nuclear releases</p>
      </div>

      <div class="grid grid-cols-2 gap-6">
        <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-4">
          <h2 class="text-lg font-semibold mb-4">Release Parameters</h2>
          
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm text-gray-400 mb-2">Latitude</label>
              <input
                type="number"
                step="0.0001"
                value={params().latitude}
                onInput={(e) => setParams(p => ({ ...p, latitude: parseFloat(e.currentTarget.value) }))}
                class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
              />
            </div>
            <div>
              <label class="block text-sm text-gray-400 mb-2">Longitude</label>
              <input
                type="number"
                step="0.0001"
                value={params().longitude}
                onInput={(e) => setParams(p => ({ ...p, longitude: parseFloat(e.currentTarget.value) }))}
                class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm text-gray-400 mb-2">Release Height (m)</label>
            <input
              type="range"
              min="0"
              max="1000"
              value={params().releaseHeight}
              onInput={(e) => setParams(p => ({ ...p, releaseHeight: parseInt(e.currentTarget.value) }))}
              class="w-full"
            />
            <div class="flex justify-between text-sm text-gray-500 mt-1">
              <span>0m</span>
              <span class="text-[#00d4ff]">{params().releaseHeight}m</span>
              <span>1000m</span>
            </div>
          </div>

          <div>
            <label class="block text-sm text-gray-400 mb-2">Release Rate (Bq/s)</label>
            <input
              type="number"
              value={params().releaseRate}
              onInput={(e) => setParams(p => ({ ...p, releaseRate: parseInt(e.currentTarget.value) }))}
              class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
            />
          </div>

          <div>
            <label class="block text-sm text-gray-400 mb-2">Simulation Duration (hours)</label>
            <select
              value={params().duration}
              onChange={(e) => setParams(p => ({ ...p, duration: parseInt(e.currentTarget.value) }))}
              class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
            >
              <option value={24}>24 hours</option>
              <option value={48}>48 hours</option>
              <option value={72}>72 hours</option>
              <option value={168}>7 days</option>
            </select>
          </div>

          <div>
            <label class="block text-sm text-gray-400 mb-2">Isotope</label>
            <select
              value={params().isotope}
              onChange={(e) => setParams(p => ({ ...p, isotope: e.currentTarget.value }))}
              class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
            >
              <option value="cs137">Cs-137 (30.1 y)</option>
              <option value="i131">I-131 (8.02 d)</option>
              <option value="sr90">Sr-90 (28.8 y)</option>
              <option value="co60">Co-60 (5.27 y)</option>
            </select>
          </div>

          <button
            onClick={runSimulation}
            disabled={simulating()}
            class="w-full bg-[#00d4ff] text-black py-3 rounded-lg font-semibold hover:bg-[#00d4ff]/90 transition-colors disabled:opacity-50"
          >
            {simulating() ? 'Running Simulation...' : 'Run Simulation'}
          </button>
        </div>

        <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6">
          <h2 class="text-lg font-semibold mb-4">Visualization</h2>
          <div class="h-96 bg-[#0a0a0f] rounded-lg border border-[#2a2a3a] flex items-center justify-center">
            <div class="text-center">
              <div class="w-16 h-16 border-4 border-[#00d4ff]/30 border-t-[#00d4ff] rounded-full animate-spin mb-4 mx-auto"></div>
              <p class="text-gray-400">Simulation results will appear here</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default PlumeSimulator;
