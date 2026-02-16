import { For, Show, createSignal } from 'solid-js';
import { createGraphQLQuery, createGraphQLSubscription, createGraphQLMutation } from '../graphql/client';

interface Alert {
  id: string;
  type: string;
  severity: 'critical' | 'warning' | 'info';
  message: string;
  timestamp: string;
  acknowledged: boolean;
  metadata?: {
    sensorId?: string;
    location?: {
      lat: number;
      lon: number;
    };
    doseRate?: number;
    facilityId?: string;
  };
}


const ALERTS_QUERY = `
  query Alerts($severity: [Severity!], $acknowledged: Boolean, $limit: Int) {
    alerts(severity: $severity, acknowledged: $acknowledged, limit: $limit) {
      id
      type
      severity
      message
      timestamp
      acknowledged
      metadata {
        sensorId
        location {
          lat
          lon
        }
        doseRate
        facilityId
      }
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
      acknowledged
      metadata {
        sensorId
        location {
          lat
          lon
        }
        doseRate
      }
    }
  }
`;

const ACKNOWLEDGE_ALERT_MUTATION = `
  mutation AcknowledgeAlert($alertId: UUID!) {
    acknowledgeAlert(alertId: $alertId) {
      id
      acknowledged
      acknowledgedAt
    }
  }
`;

const CREATE_ALERT_RULE_MUTATION = `
  mutation CreateAlertRule($rule: AlertRuleInput!) {
    createAlertRule(rule: $rule) {
      id
      name
      enabled
    }
  }
`;

interface AlertRuleInput {
  name: string;
  sensorIds?: string[];
  severityThreshold: 'low' | 'medium' | 'high' | 'critical';
  doseRateThreshold?: number;
  emailNotifications: boolean;
  pushNotifications: boolean;
}

function AlertPanel() {
  const [filter, setFilter] = createSignal<'all' | 'critical' | 'warning' | 'info'>('all');
  const [showAcknowledged, setShowAcknowledged] = createSignal(false);
  const [selectedAlert, setSelectedAlert] = createSignal<Alert | null>(null);
  const [showRuleDialog, setShowRuleDialog] = createSignal(false);

  const alerts = createGraphQLQuery<{ alerts: Alert[] }>(ALERTS_QUERY, () => ({
    severity: filter() === 'all' ? null : [filter().toUpperCase()],
    acknowledged: showAcknowledged(),
    limit: 50,
  }));

  // Real-time alert subscription
  createGraphQLSubscription<Alert>(ALERT_SUBSCRIPTION);
  
  const [acknowledgeAlert] = createGraphQLMutation<{ acknowledgeAlert: { id: string; acknowledged: boolean } }>();

  const [createAlertRule] = createGraphQLMutation<{ createAlertRule: { id: string; name: string } }>();

  const handleAcknowledge = async (alertId: string) => {
    try {
      await acknowledgeAlert(ACKNOWLEDGE_ALERT_MUTATION, { alertId });
      // Refetch alerts
      alerts.refetch();
    } catch (err) {
      console.error('Failed to acknowledge alert:', err);
    }
  };

  const handleCreateRule = async (rule: AlertRuleInput) => {
    try {
      await createAlertRule(CREATE_ALERT_RULE_MUTATION, { rule });
      setShowRuleDialog(false);
    } catch (err) {
      console.error('Failed to create alert rule:', err);
    }
  };

  const severityColors = {
    critical: 'bg-red-500/20 border-red-500/50 text-red-400',
    warning: 'bg-yellow-500/20 border-yellow-500/50 text-yellow-400',
    info: 'bg-blue-500/20 border-blue-500/50 text-blue-400',
  };

  const severityIcons = {
    critical: 'C',
    warning: 'W',
    info: 'I',
  };


  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    
    if (minutes < 1) return 'Just now';
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return date.toLocaleDateString();
  };

  return (
    <div class="space-y-4">
      {/* Filters */}
      <div class="flex items-center justify-between">
        <div class="flex gap-2">
          {(['all', 'critical', 'warning', 'info'] as const).map((f) => (
            <button
              title={`Filter alerts by ${f} severity`}
              onClick={() => setFilter(f)}
              class={`px-3 py-1 rounded-full text-xs font-medium transition-colors ${
                filter() === f 
                  ? 'bg-[#00d4ff]/20 text-[#00d4ff]' 
                  : 'bg-[#2a2a3a] text-gray-400 hover:bg-[#3a3a4a]'
              }`}
            >
              {f.charAt(0).toUpperCase() + f.slice(1)}
            </button>
          ))}

        </div>
        <div class="flex items-center gap-2">
          <label title="Toggle display of acknowledged alerts" class="flex items-center gap-2 text-sm text-gray-400 cursor-pointer">
            <input
              type="checkbox"
              checked={showAcknowledged()}
              onChange={(e) => setShowAcknowledged(e.currentTarget.checked)}
              class="rounded border-[#2a2a3a] bg-[#0a0a0f] text-[#00d4ff] focus:ring-[#00d4ff]"
            />
            Show acknowledged
          </label>

          <button
            title="Create new alert rule"
            onClick={() => setShowRuleDialog(true)}
            class="px-3 py-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] rounded text-xs transition-colors"
          >
            + Rule
          </button>

        </div>
      </div>

      {/* Alert List */}
      <div class="space-y-3 max-h-96 overflow-y-auto">
        <For each={alerts()?.alerts}>
          {(alert) => (
            <div 
              class={`p-3 rounded-lg border ${severityColors[alert.severity]} ${alert.acknowledged ? 'opacity-50' : ''} cursor-pointer hover:opacity-80 transition-opacity`}
              onClick={() => setSelectedAlert(alert)}
            >
              <div class="flex items-start justify-between">
                <div class="flex items-start gap-2">
                  <span class="text-lg">{severityIcons[alert.severity]}</span>
                  <div>
                    <p class="font-medium text-sm">{alert.message}</p>
                <Show when={alert.metadata?.location}>
                  {(loc) => (
                    <p class="text-xs opacity-80 mt-1">
                      {loc().lat.toFixed(4)}°, {loc().lon.toFixed(4)}°
                    </p>
                  )}
                </Show>
                <Show when={alert.metadata?.doseRate}>
                  {(rate) => (
                    <p class="text-xs opacity-80">
                      Dose rate: {rate().toFixed(3)} μSv/h
                    </p>
                  )}
                </Show>


                  </div>
                </div>
                <div class="flex flex-col items-end gap-1">
                  <span class="text-xs opacity-60">{formatTimestamp(alert.timestamp)}</span>
                  <Show when={!alert.acknowledged}>
                    <button
                      title="Acknowledge this alert"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleAcknowledge(alert.id);
                      }}
                      class="text-xs px-2 py-1 bg-white/10 hover:bg-white/20 rounded transition-colors"
                    >
                      Ack
                    </button>
                  </Show>

                </div>
              </div>
            </div>
          )}
        </For>
        
        <Show when={!alerts()?.alerts?.length}>
          <p class="text-gray-500 text-center py-8">No active alerts</p>
        </Show>
      </div>

      {/* Alert Detail Modal */}
      <Show when={selectedAlert()}>
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={() => setSelectedAlert(null)}>
          <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 w-96" onClick={(e) => e.stopPropagation()}>
            <div class="flex items-center gap-2 mb-4">
              <span class="text-2xl">{severityIcons[selectedAlert()!.severity]}</span>
              <h3 class="text-lg font-semibold capitalize">{selectedAlert()!.severity} Alert</h3>
            </div>
            
            <p class="text-gray-300 mb-4">{selectedAlert()!.message}</p>
            
            <div class="space-y-2 text-sm mb-4">
              <div class="flex justify-between">
                <span class="text-gray-400">Type:</span>
                <span class="capitalize">{selectedAlert()!.type}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-gray-400">Time:</span>
                <span>{new Date(selectedAlert()!.timestamp).toLocaleString()}</span>
              </div>
              <Show when={selectedAlert()!.metadata?.sensorId}>
                <div class="flex justify-between">
                  <span class="text-gray-400">Sensor:</span>
                  <span>{selectedAlert()!.metadata?.sensorId}</span>
                </div>
              </Show>
              <Show when={selectedAlert()!.metadata?.doseRate}>
                {(rate) => (
                  <div class="flex justify-between">
                    <span class="text-gray-400">Dose Rate:</span>
                    <span class="text-red-400">{rate().toFixed(3)} μSv/h</span>
                  </div>
                )}
              </Show>


              <Show when={selectedAlert()!.acknowledged}>
                <div class="flex justify-between">
                  <span class="text-gray-400">Status:</span>
                  <span class="text-green-400">Acknowledged</span>
                </div>
              </Show>
            </div>

            <div class="flex gap-2">
              <button
                title="Close alert details"
                onClick={() => setSelectedAlert(null)}
                class="flex-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] py-2 rounded-lg transition-colors"
              >
                Close
              </button>

              <Show when={!selectedAlert()!.acknowledged}>
                <button
                  title="Acknowledge this alert"
                  onClick={() => {
                    handleAcknowledge(selectedAlert()!.id);
                    setSelectedAlert(null);
                  }}
                  class="flex-1 bg-[#00d4ff] text-black py-2 rounded-lg font-semibold hover:bg-[#00d4ff]/90 transition-colors"
                >
                  Acknowledge
                </button>
              </Show>

            </div>
          </div>
        </div>
      </Show>

      {/* Create Rule Dialog */}
      <Show when={showRuleDialog()}>
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 w-96">
            <h3 class="text-lg font-semibold mb-4">Create Alert Rule</h3>
            <form 
              onSubmit={(e) => {
                e.preventDefault();
                const formData = new FormData(e.currentTarget);
                handleCreateRule({
                  name: formData.get('name') as string,
                  severityThreshold: formData.get('severity') as AlertRuleInput['severityThreshold'],
                  doseRateThreshold: parseFloat(formData.get('doseRate') as string) || undefined,
                  emailNotifications: formData.get('email') === 'on',
                  pushNotifications: formData.get('push') === 'on',
                });
              }}
              class="space-y-4"
            >
              <div>
                <label class="block text-sm text-gray-400 mb-1">Rule Name</label>
                <input
                  name="name"
                  type="text"
                  required
                  title="Enter rule name"
                  placeholder="e.g., High Radiation Alert"
                  class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-3 py-2 focus:outline-none focus:border-[#00d4ff]"
                />
              </div>

              
              <div>
                <label class="block text-sm text-gray-400 mb-1">Minimum Severity</label>
                <select
                  name="severity"
                  title="Select minimum severity level"
                  class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-3 py-2 focus:outline-none focus:border-[#00d4ff]"
                >

                  <option value="low">Low</option>
                  <option value="medium">Medium</option>
                  <option value="high">High</option>
                  <option value="critical">Critical</option>
                </select>
              </div>

              <div>
                <label class="block text-sm text-gray-400 mb-1">Dose Rate Threshold (μSv/h)</label>
                <input
                  name="doseRate"
                  type="number"
                  step="0.01"
                  title="Enter dose rate threshold"
                  placeholder="e.g., 2.5"
                  class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-3 py-2 focus:outline-none focus:border-[#00d4ff]"
                />
              </div>


              <div class="flex gap-4">
                <label title="Enable email notifications" class="flex items-center gap-2 text-sm text-gray-400">
                  <input name="email" type="checkbox" class="rounded border-[#2a2a3a]" />
                  Email
                </label>
                <label title="Enable push notifications" class="flex items-center gap-2 text-sm text-gray-400">
                  <input name="push" type="checkbox" class="rounded border-[#2a2a3a]" />
                  Push
                </label>
              </div>


              <div class="flex gap-2 pt-2">
                <button
                  title="Cancel rule creation"
                  type="button"
                  onClick={() => setShowRuleDialog(false)}
                  class="flex-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] py-2 rounded-lg transition-colors"
                >
                  Cancel
                </button>
                <button
                  title="Create alert rule"
                  type="submit"
                  class="flex-1 bg-[#00d4ff] text-black py-2 rounded-lg font-semibold hover:bg-[#00d4ff]/90 transition-colors"
                >
                  Create
                </button>
              </div>

            </form>
          </div>
        </div>
      </Show>
    </div>
  );
}

export default AlertPanel;
