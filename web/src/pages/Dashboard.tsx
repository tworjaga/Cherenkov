import { createSignal, onMount } from 'solid-js';
import StatCard from '../components/StatCard';
import AlertPanel from '../components/AlertPanel';
import SensorChart from '../components/SensorChart';

function Dashboard() {
  const [stats, setStats] = createSignal({
    sensors: 50247,
    readings: 104729,
    anomalies: 3,
    alerts: 0,
  });

  return (
    <div class="space-y-6">
      <div class="grid grid-cols-4 gap-4">
        <StatCard
          title="Active Sensors"
          value={stats().sensors.toLocaleString()}
          change="+12"
          trend="up"
          color="blue"
        />
        <StatCard
          title="Readings/Min"
          value={stats().readings.toLocaleString()}
          change="+5.2%"
          trend="up"
          color="green"
        />
        <StatCard
          title="Anomalies"
          value={stats().anomalies.toString()}
          change="-2"
          trend="down"
          color="yellow"
        />
        <StatCard
          title="Active Alerts"
          value={stats().alerts.toString()}
          change="0"
          trend="neutral"
          color="red"
        />
      </div>

      <div class="grid grid-cols-3 gap-6">
        <div class="col-span-2 bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6">
          <h2 class="text-lg font-semibold mb-4">Global Radiation Levels</h2>
          <SensorChart />
        </div>
        
        <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6">
          <h2 class="text-lg font-semibold mb-4">Recent Alerts</h2>
          <AlertPanel />
        </div>
      </div>

      <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6">
        <h2 class="text-lg font-semibold mb-4">Data Sources</h2>
        <div class="grid grid-cols-5 gap-4">
          {[
            { name: 'Safecast', status: 'online', count: 5234 },
            { name: 'uRADMonitor', status: 'online', count: 10241 },
            { name: 'EPA RadNet', status: 'online', count: 140 },
            { name: 'EURDEP', status: 'online', count: 4892 },
            { name: 'IAEA PRIS', status: 'online', count: 440 },
          ].map(source => (
            <div class="flex items-center justify-between p-4 bg-[#0a0a0f] rounded-lg border border-[#2a2a3a]">
              <div>
                <p class="font-medium">{source.name}</p>
                <p class="text-sm text-gray-500">{source.count.toLocaleString()} sensors</p>
              </div>
              <span class={`w-2 h-2 rounded-full ${source.status === 'online' ? 'bg-green-500' : 'bg-red-500'}`}></span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default Dashboard;
