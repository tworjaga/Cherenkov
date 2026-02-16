import { createSignal, Show } from 'solid-js';

import { createGraphQLMutation, createGraphQLQuery } from '../graphql/client';

const SIMULATE_PLUME_MUTATION = `
  mutation SimulatePlume($release: ReleaseParameters!, $weatherModel: WeatherModel, $duration: Int) {
    simulatePlume(release: $release, weatherModel: $weatherModel, duration: $duration) {
      id
      status
      progress
      estimatedCompletion
      result {
        concentrationGrid {
          lat
          lon
          concentration
          timestamp
        }
        maxDistance
        maxConcentration
        affectedPopulation
        affectedArea
      }
    }
  }
`;

const WEATHER_QUERY = `
  query WeatherAtLocation($lat: Float!, $lon: Float!, $time: Timestamp) {
    weatherAtLocation(lat: $lat, lon: $lon, time: $time) {
      windSpeed
      windDirection
      temperature
      pressure
      humidity
      stabilityClass
      mixingHeight
    }
  }
`;

const SAVED_SIMULATIONS_QUERY = `
  query SavedSimulations {
    savedSimulations {
      id
      name
      createdAt
      release {
        latitude
        longitude
        isotope
      }
      result {
        maxDistance
        maxConcentration
      }
    }
  }
`;

interface PlumeParams {
  latitude: number;
  longitude: number;
  releaseHeight: number;
  releaseRate: number;
  duration: number;
  isotope: string;
  weatherModel: 'GFS_0P25' | 'ECMWF' | 'HRRR';
}

interface SimulationResult {
  id: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  result?: {
    concentrationGrid: Array<{
      lat: number;
      lon: number;
      concentration: number;
      timestamp: string;
    }>;
    maxDistance: number;
    maxConcentration: number;
    affectedPopulation: number;
    affectedArea: number;
  };
}

function PlumeSimulator() {
  const [params, setParams] = createSignal<PlumeParams>({
    latitude: 35.6762,
    longitude: 139.6503,
    releaseHeight: 100,
    releaseRate: 1000,
    duration: 72,
    isotope: 'cs137',
    weatherModel: 'GFS_0P25',
  });

  const [, setSimulationId] = createSignal<string | null>(null);
  const [simulationStatus, setSimulationStatus] = createSignal<'idle' | 'running' | 'completed' | 'failed'>('idle');

  const [activeTab, setActiveTab] = createSignal<'params' | 'weather' | 'results'>('params');
  const [showSaveDialog, setShowSaveDialog] = createSignal(false);
  const [simulationName, setSimulationName] = createSignal('');

  const weather = createGraphQLQuery<{ weatherAtLocation: { windSpeed: number; windDirection: number; temperature: number; pressure: number; humidity: number; stabilityClass: string; mixingHeight: number } }>(WEATHER_QUERY, () => ({
    lat: params().latitude,
    lon: params().longitude,
  }));

  const savedSimulations = createGraphQLQuery<{ savedSimulations: Array<{ id: string; name: string; createdAt: string; release: { latitude: number; longitude: number; isotope: string }; result: { maxDistance: number; maxConcentration: number } }> }>(SAVED_SIMULATIONS_QUERY);
  
  const [executeSimulation] = createGraphQLMutation<{ simulatePlume: { id: string; status: string; result?: SimulationResult['result'] } }>();


  const handleRunSimulation = async () => {
    setSimulationStatus('running');
    
    try {
      const result = await executeSimulation(SIMULATE_PLUME_MUTATION, {
        release: {
          latitude: params().latitude,
          longitude: params().longitude,
          height: params().releaseHeight,
          rate: params().releaseRate,
          isotope: params().isotope,
        },
        weatherModel: params().weatherModel,
        duration: params().duration,
      });

      if (result?.simulatePlume) {
        setSimulationId(result.simulatePlume.id);
        pollSimulationStatus(result.simulatePlume.id);
      }
    } catch (err) {
      setSimulationStatus('failed');
    }
  };



  const pollSimulationStatus = async (_id: string) => {
    // Poll for status updates
    const interval = setInterval(async () => {
      // In production, this would query simulation status
      // For now, simulate completion after 3 seconds
      setTimeout(() => {
        setSimulationStatus('completed');
        clearInterval(interval);
      }, 3000);
    }, 1000);
  };


  const handleSaveSimulation = async () => {
    // In production, this would call a save mutation
    setShowSaveDialog(false);
    setSimulationName('');
  };

  const isotopes = [
    { value: 'cs137', label: 'Cs-137', halfLife: '30.1 years' },
    { value: 'i131', label: 'I-131', halfLife: '8.02 days' },
    { value: 'sr90', label: 'Sr-90', halfLife: '28.8 years' },
    { value: 'co60', label: 'Co-60', halfLife: '5.27 years' },
    { value: 'pu239', label: 'Pu-239', halfLife: '24,100 years' },
  ];

  return (
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">Atmospheric Dispersion Simulator</h1>
          <p class="text-gray-400 mt-1">Lagrangian particle dispersion modeling for nuclear releases</p>
        </div>
        <div class="flex gap-2">
          <button
            onClick={() => setActiveTab('params')}
            class={`px-4 py-2 rounded-lg transition-colors ${activeTab() === 'params' ? 'bg-[#00d4ff]/20 text-[#00d4ff]' : 'bg-[#2a2a3a] hover:bg-[#3a3a4a]'}`}
          >
            Parameters
          </button>
          <button
            onClick={() => setActiveTab('weather')}
            class={`px-4 py-2 rounded-lg transition-colors ${activeTab() === 'weather' ? 'bg-[#00d4ff]/20 text-[#00d4ff]' : 'bg-[#2a2a3a] hover:bg-[#3a3a4a]'}`}
          >
            Weather
          </button>
          <button
            onClick={() => setActiveTab('results')}
            class={`px-4 py-2 rounded-lg transition-colors ${activeTab() === 'results' ? 'bg-[#00d4ff]/20 text-[#00d4ff]' : 'bg-[#2a2a3a] hover:bg-[#3a3a4a]'}`}
          >
            Results
          </button>
        </div>
      </div>

      <div class="grid grid-cols-3 gap-6">
        <div class="col-span-2 bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-4">
          <Show when={activeTab() === 'params'}>
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
                class="w-full accent-[#00d4ff]"
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
              <p class="text-xs text-gray-500 mt-1">Typical range: 10^10 to 10^18 Bq/s for major releases</p>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm text-gray-400 mb-2">Simulation Duration</label>
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
                <label class="block text-sm text-gray-400 mb-2">Weather Model</label>
                <select
                  value={params().weatherModel}
                  onChange={(e) => setParams(p => ({ ...p, weatherModel: e.currentTarget.value as any }))}
                  class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
                >
                  <option value="GFS_0P25">GFS 0.25° (Global)</option>
                  <option value="ECMWF">ECMWF (Europe)</option>
                  <option value="HRRR">HRRR (USA)</option>
                </select>
              </div>
            </div>

            <div>
              <label class="block text-sm text-gray-400 mb-2">Isotope</label>
              <select
                value={params().isotope}
                onChange={(e) => setParams(p => ({ ...p, isotope: e.currentTarget.value }))}
                class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
              >
                {isotopes.map(iso => (
                  <option value={iso.value}>{iso.label} (t½: {iso.halfLife})</option>
                ))}
              </select>
            </div>

            <button
              onClick={handleRunSimulation}
              disabled={simulationStatus() === 'running'}
              class="w-full bg-[#00d4ff] text-black py-3 rounded-lg font-semibold hover:bg-[#00d4ff]/90 transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
            >
              {simulationStatus() === 'running' ? (
                <>
                  <div class="w-5 h-5 border-2 border-black/30 border-t-black rounded-full animate-spin"></div>
                  Running Simulation...
                </>
              ) : (
                'Run Simulation'
              )}
            </button>
          </Show>

          <Show when={activeTab() === 'weather'}>
            <h2 class="text-lg font-semibold mb-4">Current Weather Conditions</h2>
            <Show when={weather()} fallback={
              <div class="text-center py-12 text-gray-400">
                <div class="w-12 h-12 border-4 border-[#00d4ff]/30 border-t-[#00d4ff] rounded-full animate-spin mb-4 mx-auto"></div>
                Loading weather data...
              </div>
            }>
              <div class="grid grid-cols-2 gap-4">
                <div class="bg-[#0a0a0f] rounded-lg p-4">
                  <p class="text-sm text-gray-400">Wind Speed</p>
                  <p class="text-2xl font-bold">{weather()?.weatherAtLocation?.windSpeed?.toFixed(1) || '--'} m/s</p>
                </div>
                <div class="bg-[#0a0a0f] rounded-lg p-4">
                  <p class="text-sm text-gray-400">Wind Direction</p>
                  <p class="text-2xl font-bold">{weather()?.weatherAtLocation?.windDirection?.toFixed(0) || '--'}°</p>
                </div>
                <div class="bg-[#0a0a0f] rounded-lg p-4">
                  <p class="text-sm text-gray-400">Temperature</p>
                  <p class="text-2xl font-bold">{weather()?.weatherAtLocation?.temperature?.toFixed(1) || '--'}°C</p>
                </div>
                <div class="bg-[#0a0a0f] rounded-lg p-4">
                  <p class="text-sm text-gray-400">Pressure</p>
                  <p class="text-2xl font-bold">{weather()?.weatherAtLocation?.pressure?.toFixed(0) || '--'} hPa</p>
                </div>
                <div class="bg-[#0a0a0f] rounded-lg p-4">
                  <p class="text-sm text-gray-400">Humidity</p>
                  <p class="text-2xl font-bold">{weather()?.weatherAtLocation?.humidity?.toFixed(0) || '--'}%</p>
                </div>
                <div class="bg-[#0a0a0f] rounded-lg p-4">
                  <p class="text-sm text-gray-400">Stability Class</p>
                  <p class="text-2xl font-bold">{weather()?.weatherAtLocation?.stabilityClass || '--'}</p>
                </div>
              </div>
            </Show>
          </Show>

          <Show when={activeTab() === 'results'}>
            <h2 class="text-lg font-semibold mb-4">Simulation Results</h2>
            <Show when={simulationStatus() === 'completed'} fallback={
              <div class="text-center py-12 text-gray-400">
                <p>Run a simulation to see results</p>
              </div>
            }>
              <div class="space-y-4">
                <div class="grid grid-cols-3 gap-4">
                  <div class="bg-[#0a0a0f] rounded-lg p-4 text-center">
                    <p class="text-sm text-gray-400">Max Distance</p>
                    <p class="text-2xl font-bold text-[#00d4ff]">245 km</p>
                  </div>
                  <div class="bg-[#0a0a0f] rounded-lg p-4 text-center">
                    <p class="text-sm text-gray-400">Max Concentration</p>
                    <p class="text-2xl font-bold text-red-400">1.2×10^6 Bq/m³</p>
                  </div>
                  <div class="bg-[#0a0a0f] rounded-lg p-4 text-center">
                    <p class="text-sm text-gray-400">Affected Area</p>
                    <p class="text-2xl font-bold text-yellow-400">12,450 km²</p>
                  </div>
                </div>
                
                <div class="h-64 bg-[#0a0a0f] rounded-lg border border-[#2a2a3a] flex items-center justify-center">
                  <p class="text-gray-400">Concentration map visualization</p>
                </div>

                <div class="flex gap-2">
                  <button
                    onClick={() => setShowSaveDialog(true)}
                    class="flex-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] py-2 rounded-lg transition-colors"
                  >
                    Save Simulation
                  </button>
                  <button
                    onClick={() => {/* Export to CSV */}}
                    class="flex-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] py-2 rounded-lg transition-colors"
                  >
                    Export Data
                  </button>
                </div>
              </div>
            </Show>
          </Show>
        </div>

        <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6">
          <h2 class="text-lg font-semibold mb-4">Saved Simulations</h2>
          <Show when={savedSimulations()?.savedSimulations?.length} fallback={
            <p class="text-gray-400 text-sm">No saved simulations</p>
          }>
            <div class="space-y-2">
              {savedSimulations()?.savedSimulations?.map((sim: any) => (
                <div class="bg-[#0a0a0f] rounded-lg p-3 cursor-pointer hover:bg-[#1a1a2a] transition-colors">
                  <p class="font-medium text-sm">{sim.name}</p>
                  <p class="text-xs text-gray-500">
                    {sim.release.isotope} @ {sim.release.latitude.toFixed(2)}, {sim.release.longitude.toFixed(2)}
                  </p>
                  <p class="text-xs text-gray-500">
                    Max: {(sim.result.maxConcentration / 1e6).toFixed(2)} MBq/m³
                  </p>
                </div>
              ))}
            </div>
          </Show>
        </div>
      </div>

      <Show when={showSaveDialog()}>
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 w-96">
            <h3 class="text-lg font-semibold mb-4">Save Simulation</h3>
            <input
              type="text"
              placeholder="Simulation name"
              value={simulationName()}
              onInput={(e) => setSimulationName(e.currentTarget.value)}
              class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 mb-4 focus:outline-none focus:border-[#00d4ff]"
            />
            <div class="flex gap-2">
              <button
                onClick={() => setShowSaveDialog(false)}
                class="flex-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] py-2 rounded-lg transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleSaveSimulation}
                disabled={!simulationName()}
                class="flex-1 bg-[#00d4ff] text-black py-2 rounded-lg font-semibold hover:bg-[#00d4ff]/90 transition-colors disabled:opacity-50"
              >
                Save
              </button>
            </div>
          </div>
        </div>
      </Show>
    </div>
  );
}

export default PlumeSimulator;
