import { For } from 'solid-js';

interface Alert {
  id: string;
  severity: 'critical' | 'warning' | 'info';
  message: string;
  location: string;
  timestamp: string;
}

function AlertPanel() {
  const alerts: Alert[] = [
    {
      id: '1',
      severity: 'warning',
      message: 'Elevated radiation detected',
      location: 'Tokyo, Japan',
      timestamp: '2 min ago',
    },
    {
      id: '2',
      severity: 'info',
      message: 'Sensor offline',
      location: 'Berlin, Germany',
      timestamp: '15 min ago',
    },
    {
      id: '3',
      severity: 'info',
      message: 'Calibration complete',
      location: 'New York, USA',
      timestamp: '1 hour ago',
    },
  ];

  const severityColors = {
    critical: 'bg-red-500/20 border-red-500/50 text-red-400',
    warning: 'bg-yellow-500/20 border-yellow-500/50 text-yellow-400',
    info: 'bg-blue-500/20 border-blue-500/50 text-blue-400',
  };

  return (
    <div class="space-y-3">
      <For each={alerts}>
        {(alert) => (
          <div class={`p-3 rounded-lg border ${severityColors[alert.severity]}`}>
            <div class="flex items-start justify-between">
              <div>
                <p class="font-medium text-sm">{alert.message}</p>
                <p class="text-xs opacity-80 mt-1">{alert.location}</p>
              </div>
              <span class="text-xs opacity-60">{alert.timestamp}</span>
            </div>
          </div>
        )}
      </For>
      
      {alerts.length === 0 && (
        <p class="text-gray-500 text-center py-8">No active alerts</p>
      )}
    </div>
  );
}

export default AlertPanel;
