import { createSignal, createResource, For } from 'solid-js';

interface Sensor {
  id: string;
  name: string;
  location: string;
  country: string;
  doseRate: number;
  status: 'online' | 'offline' | 'maintenance';
  lastReading: string;
}

function Sensors() {
  const [filter, setFilter] = createSignal('');
  
  const fetchSensors = async (): Promise<Sensor[]> => {
    // In production, this would fetch from the GraphQL API
    return [
      { id: '1', name: 'Tokyo-01', location: 'Shinjuku', country: 'Japan', doseRate: 0.12, status: 'online', lastReading: '2 min ago' },
      { id: '2', name: 'LA-05', location: 'Downtown', country: 'USA', doseRate: 0.08, status: 'online', lastReading: '1 min ago' },
      { id: '3', name: 'Berlin-12', location: 'Mitte', country: 'Germany', doseRate: 0.15, status: 'offline', lastReading: '2 hours ago' },
      { id: '4', name: 'Paris-03', location: 'Eiffel Tower', country: 'France', doseRate: 0.11, status: 'online', lastReading: '5 min ago' },
      { id: '5', name: 'London-08', location: 'Westminster', country: 'UK', doseRate: 0.09, status: 'maintenance', lastReading: '1 day ago' },
    ];
  };

  const [sensors] = createResource(fetchSensors);

  const filteredSensors = () => {
    const data = sensors() || [];
    if (!filter()) return data;
    return data.filter(s => 
      s.name.toLowerCase().includes(filter().toLowerCase()) ||
      s.location.toLowerCase().includes(filter().toLowerCase()) ||
      s.country.toLowerCase().includes(filter().toLowerCase())
    );
  };

  const statusColors = {
    online: 'bg-green-500/20 text-green-400 border-green-500/30',
    offline: 'bg-red-500/20 text-red-400 border-red-500/30',
    maintenance: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  };

  return (
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">Sensor Network</h1>
          <p class="text-gray-400 mt-1">Manage and monitor 50,000+ radiation sensors worldwide</p>
        </div>
        <div class="flex items-center gap-4">
          <input
            type="text"
            title="Filter sensors by name, location, or country"
            placeholder="Filter sensors..."
            value={filter()}
            onInput={(e) => setFilter(e.currentTarget.value)}
            class="bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 w-64 focus:outline-none focus:border-[#00d4ff]"
          />

          <button title="Export sensor data to CSV" class="bg-[#00d4ff] text-black px-4 py-2 rounded-lg font-medium hover:bg-[#00d4ff]/90 transition-colors">
            Export CSV
          </button>

        </div>
      </div>

      <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] overflow-hidden">
        <table class="w-full">
          <thead class="bg-[#0a0a0f] border-b border-[#2a2a3a]">
            <tr>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Sensor ID</th>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Location</th>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Country</th>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Dose Rate (μSv/h)</th>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Status</th>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Last Reading</th>
              <th class="text-left px-6 py-4 text-sm font-medium text-gray-400">Actions</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-[#2a2a3a]">
            <For each={filteredSensors()}>
              {(sensor) => (
                <tr class="hover:bg-[#0a0a0f]/50 transition-colors">
                  <td class="px-6 py-4 font-mono text-sm">{sensor.id}</td>
                  <td class="px-6 py-4">{sensor.name} - {sensor.location}</td>
                  <td class="px-6 py-4">{sensor.country}</td>
                  <td class="px-6 py-4">
                    <span class={sensor.doseRate > 0.5 ? 'text-red-400' : 'text-green-400'}>
                      {sensor.doseRate.toFixed(2)}
                    </span>
                  </td>
                  <td class="px-6 py-4">
                    <span class={`px-3 py-1 rounded-full text-xs border ${statusColors[sensor.status]}`}>
                      {sensor.status}
                    </span>
                  </td>
                  <td class="px-6 py-4 text-gray-400 text-sm">{sensor.lastReading}</td>
                  <td class="px-6 py-4">
                    <button title={`View details for sensor ${sensor.id}`} class="text-[#00d4ff] hover:underline text-sm">View Details</button>
                  </td>

                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default Sensors;
