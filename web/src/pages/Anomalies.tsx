import { createSignal, For } from 'solid-js';

interface Anomaly {
  id: string;
  sensorId: string;
  location: string;
  severity: 'critical' | 'warning' | 'info';
  zScore: number;
  doseRate: number;
  baseline: number;
  detectedAt: string;
  status: 'open' | 'investigating' | 'resolved';
}

function Anomalies() {
  const [anomalies] = createSignal<Anomaly[]>([
    {
      id: 'ANM-2024-001',
      sensorId: 'TKY-47',
      location: 'Tokyo, Japan (35.6762, 139.6503)',
      severity: 'warning',
      zScore: 3.2,
      doseRate: 0.48,
      baseline: 0.15,
      detectedAt: '2024-01-15 09:23:47 UTC',
      status: 'investigating',
    },
    {
      id: 'ANM-2024-002',
      sensorId: 'FKS-12',
      location: 'Fukushima, Japan (37.4214, 141.0326)',
      severity: 'info',
      zScore: 2.1,
      doseRate: 0.22,
      baseline: 0.18,
      detectedAt: '2024-01-15 08:15:22 UTC',
      status: 'resolved',
    },
  ]);

  const severityColors = {
    critical: 'bg-red-500/20 border-red-500/50 text-red-400',
    warning: 'bg-yellow-500/20 border-yellow-500/50 text-yellow-400',
    info: 'bg-blue-500/20 border-blue-500/50 text-blue-400',
  };

  const statusColors = {
    open: 'text-red-400',
    investigating: 'text-yellow-400',
    resolved: 'text-green-400',
  };

  return (
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">Anomaly Detection</h1>
          <p class="text-gray-400 mt-1">Statistical outliers and radiation events</p>
        </div>
        <div class="flex items-center gap-4">
          <select title="Filter by severity level" class="bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]">
            <option>All Severities</option>

            <option>Critical</option>
            <option>Warning</option>
            <option>Info</option>
          </select>
          <button class="bg-[#00d4ff] text-black px-4 py-2 rounded-lg font-medium hover:bg-[#00d4ff]/90 transition-colors">
            Export Report
          </button>
        </div>
      </div>

      <div class="space-y-4">
        <For each={anomalies()}>
          {(anomaly) => (
            <div class={`p-6 rounded-xl border ${severityColors[anomaly.severity]}`}>
              <div class="flex items-start justify-between">
                <div class="space-y-2">
                  <div class="flex items-center gap-3">
                    <h3 class="text-lg font-semibold">{anomaly.id}</h3>
                    <span class={`px-2 py-1 rounded text-xs font-medium uppercase ${severityColors[anomaly.severity]}`}>
                      {anomaly.severity}
                    </span>
                    <span class={`text-sm font-medium ${statusColors[anomaly.status]}`}>
                      {anomaly.status}
                    </span>
                  </div>
                  <p class="text-gray-300">{anomaly.location}</p>
                  <p class="text-sm text-gray-400">Sensor: {anomaly.sensorId}</p>
                </div>
                <div class="text-right space-y-1">
                  <p class="text-sm text-gray-400">Detected: {anomaly.detectedAt}</p>
                  <div class="flex items-center gap-4 mt-2">
                    <div>
                      <p class="text-xs text-gray-500">Current</p>
                      <p class="text-xl font-bold">{anomaly.doseRate.toFixed(2)} μSv/h</p>
                    </div>
                    <div>
                      <p class="text-xs text-gray-500">Baseline</p>
                      <p class="text-xl font-bold text-gray-400">{anomaly.baseline.toFixed(2)}</p>
                    </div>
                    <div>
                      <p class="text-xs text-gray-500">Z-Score</p>
                      <p class="text-xl font-bold">{anomaly.zScore.toFixed(1)}σ</p>
                    </div>
                  </div>
                </div>
              </div>
              <div class="mt-4 pt-4 border-t border-white/10 flex items-center gap-3">
                <button class="px-4 py-2 bg-white/10 rounded-lg text-sm hover:bg-white/20 transition-colors">
                  View Details
                </button>
                <button class="px-4 py-2 bg-white/10 rounded-lg text-sm hover:bg-white/20 transition-colors">
                  Run Plume Sim
                </button>
                <button class="px-4 py-2 bg-white/10 rounded-lg text-sm hover:bg-white/20 transition-colors">
                  Acknowledge
                </button>
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  );
}

export default Anomalies;
