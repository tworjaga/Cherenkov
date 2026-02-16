import { createSignal, createEffect, Show, For } from 'solid-js';
import { createGraphQLQuery, createGraphQLSubscription, type Sensor, type Anomaly, type Alert } from '../graphql/client';
import StatCard from '../components/StatCard';
import AlertPanel from '../components/AlertPanel';
import SensorChart from '../components/SensorChart';


const DASHBOARD_STATS_QUERY = `
  query DashboardStats {
    sensors {
      totalCount
      activeCount
    }
    readings {
      lastMinuteCount
    }
    anomalies {
      unacknowledgedCount
    }
    alerts {
      activeCount
    }
    dataSources {
      name
      status
      sensorCount
    }
  }
`;

const SENSOR_SUBSCRIPTION = `
  subscription OnSensorUpdate {
    sensorUpdated {
      id
      doseRate
      lastReading
      status
    }
  }
`;

const ANOMALY_SUBSCRIPTION = `
  subscription OnAnomalyDetected {
    anomalyDetected {
      id
      sensorId
      severity
      zScore
      doseRate
      timestamp
    }
  }
`;

const ALERT_SUBSCRIPTION = `
  subscription OnAlert {
    alert {
      id
      type
      severity
      message
      timestamp
    }
  }
`;

interface DashboardStats {
  sensors: {
    totalCount: number;
    activeCount: number;
  };
  readings: {
    lastMinuteCount: number;
  };
  anomalies: {
    unacknowledgedCount: number;
  };
  alerts: {
    activeCount: number;
  };
  dataSources: Array<{
    name: string;
    status: string;
    sensorCount: number;
  }>;
}

function Dashboard() {
  const stats = createGraphQLQuery<DashboardStats>(DASHBOARD_STATS_QUERY);
  
  const sensorUpdate = createGraphQLSubscription<Sensor>(SENSOR_SUBSCRIPTION);

  const anomalyUpdate = createGraphQLSubscription<Anomaly>(ANOMALY_SUBSCRIPTION);
  const alertUpdate = createGraphQLSubscription<Alert>(ALERT_SUBSCRIPTION);
  
  const [liveStats, setLiveStats] = createSignal({
    sensors: 0,
    readings: 0,
    anomalies: 0,
    alerts: 0,
  });

  createEffect(() => {
    const data = stats();
    if (data) {
      setLiveStats({
        sensors: data.sensors.activeCount,
        readings: data.readings.lastMinuteCount,
        anomalies: data.anomalies.unacknowledgedCount,
        alerts: data.alerts.activeCount,
      });
    }
  });


  createEffect(() => {
    const update = sensorUpdate();
    if (update) {
      setLiveStats(prev => ({
        ...prev,
        readings: prev.readings + 1,
      }));
    }
  });

  createEffect(() => {
    const anomaly = anomalyUpdate();
    if (anomaly) {
      setLiveStats(prev => ({
        ...prev,
        anomalies: prev.anomalies + 1,
      }));
    }
  });

  createEffect(() => {
    const alert = alertUpdate();
    if (alert) {
      setLiveStats(prev => ({
        ...prev,
        alerts: prev.alerts + 1,
      }));
    }
  });

  const calculateTrend = (current: number, previous: number): { change: string; trend: 'up' | 'down' | 'neutral' } => {
    if (previous === 0) return { change: '0', trend: 'neutral' };
    const diff = current - previous;
    const percent = ((diff / previous) * 100).toFixed(1);
    return {
      change: diff > 0 ? `+${percent}%` : `${percent}%`,
      trend: diff > 0 ? 'up' : diff < 0 ? 'down' : 'neutral',
    };
  };

  return (
    <div class="space-y-6">
      <div class="grid grid-cols-4 gap-4">
        <StatCard
          title="Active Sensors"
          value={liveStats().sensors.toLocaleString()}
          change={calculateTrend(liveStats().sensors, (stats()?.sensors.activeCount || 0) - 10).change}
          trend={calculateTrend(liveStats().sensors, (stats()?.sensors.activeCount || 0) - 10).trend}
          color="blue"
        />
        <StatCard
          title="Readings/Min"
          value={liveStats().readings.toLocaleString()}
          change={calculateTrend(liveStats().readings, (stats()?.readings.lastMinuteCount || 0) - 100).change}
          trend={calculateTrend(liveStats().readings, (stats()?.readings.lastMinuteCount || 0) - 100).trend}
          color="green"
        />
        <StatCard
          title="Anomalies"
          value={liveStats().anomalies.toString()}
          change={calculateTrend(liveStats().anomalies, stats()?.anomalies.unacknowledgedCount || 0).change}
          trend={calculateTrend(liveStats().anomalies, stats()?.anomalies.unacknowledgedCount || 0).trend}
          color="yellow"
        />
        <StatCard
          title="Active Alerts"
          value={liveStats().alerts.toString()}
          change={calculateTrend(liveStats().alerts, stats()?.alerts.activeCount || 0).change}
          trend={calculateTrend(liveStats().alerts, stats()?.alerts.activeCount || 0).trend}
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
        <Show when={stats()} fallback={<div class="text-gray-500">Loading data sources...</div>}>
          <div class="grid grid-cols-5 gap-4">
            <For each={stats()?.dataSources}>
              {(source) => (
                <div class="flex items-center justify-between p-4 bg-[#0a0a0f] rounded-lg border border-[#2a2a3a]">
                  <div>
                    <p class="font-medium">{source.name}</p>
                    <p class="text-sm text-gray-500">{source.sensorCount.toLocaleString()} sensors</p>
                  </div>
                  <span class={`w-2 h-2 rounded-full ${source.status === 'online' ? 'bg-green-500' : 'bg-red-500'}`}></span>
                </div>
              )}
            </For>
          </div>
        </Show>
      </div>

      <Show when={anomalyUpdate()}>
        <div class="fixed bottom-4 right-4 bg-red-900/90 text-white p-4 rounded-lg shadow-lg border border-red-500 animate-pulse">
          <p class="font-semibold">New Anomaly Detected</p>
          <p class="text-sm">Sensor: {(anomalyUpdate() as Anomaly)?.sensorId}</p>
          <p class="text-sm">Severity: {(anomalyUpdate() as Anomaly)?.severity}</p>
        </div>
      </Show>
    </div>
  );
}

export default Dashboard;
