import { createSignal, Show, createEffect, For } from 'solid-js';

import { createGraphQLQuery, createGraphQLMutation } from '../graphql/client';

const USER_PREFERENCES_QUERY = `
  query UserPreferences {
    me {
      id
      email
      preferences {
        theme
        language
        timezone
        notifications {
          email
          push
          sms
          alertThreshold
        }
        display {
          defaultView
          globeRotation
          showLabels
          colorScheme
        }
        data {
          defaultTimeRange
          autoRefresh
          refreshInterval
        }
      }
      apiKeys {
        id
        name
        createdAt
        lastUsed
        permissions
      }
    }
  }
`;

const UPDATE_PREFERENCES_MUTATION = `
  mutation UpdatePreferences($preferences: UserPreferencesInput!) {
    updatePreferences(preferences: $preferences) {
      theme
      language
      notifications {
        email
        push
        alertThreshold
      }
    }
  }
`;

const CREATE_API_KEY_MUTATION = `
  mutation CreateApiKey($name: String!, $permissions: [String!]!) {
    createApiKey(name: $name, permissions: $permissions) {
      id
      key
      name
      createdAt
    }
  }
`;

const REVOKE_API_KEY_MUTATION = `
  mutation RevokeApiKey($keyId: UUID!) {
    revokeApiKey(keyId: $keyId) {
      id
      revokedAt
    }
  }
`;

interface UserPreferences {
  theme: 'dark' | 'light' | 'system';
  language: string;
  timezone: string;
  notifications: {
    email: boolean;
    push: boolean;
    sms: boolean;
    alertThreshold: number;
  };
  display: {
    defaultView: 'dashboard' | 'globe' | 'sensors';
    globeRotation: boolean;
    showLabels: boolean;
    colorScheme: 'default' | 'thermal' | 'radiation';
  };
  data: {
    defaultTimeRange: number;
    autoRefresh: boolean;
    refreshInterval: number;
  };
}

function Settings() {
  const [activeSection, setActiveSection] = createSignal<'profile' | 'notifications' | 'display' | 'data' | 'api'>('profile');
  const [showApiKeyDialog, setShowApiKeyDialog] = createSignal(false);
  const [newApiKeyName, setNewApiKeyName] = createSignal('');
  const [newApiKeyPermissions, setNewApiKeyPermissions] = createSignal<string[]>(['read:readings']);
  const [generatedKey, setGeneratedKey] = createSignal<string | null>(null);
  const [saveStatus, setSaveStatus] = createSignal<'idle' | 'saving' | 'saved' | 'error'>('idle');

  const userData = createGraphQLQuery<{ me: { id: string; email: string; preferences: UserPreferences; apiKeys: Array<{ id: string; name: string; createdAt: string; lastUsed?: string; permissions: string[] }> } }>(USER_PREFERENCES_QUERY);
  
  const [updatePreferences] = createGraphQLMutation<{ updatePreferences: Partial<UserPreferences> }>();
  const [createApiKey] = createGraphQLMutation<{ createApiKey: { id: string; key: string; name: string } }>();
  const [revokeApiKey] = createGraphQLMutation<{ revokeApiKey: { id: string } }>();

  const [localPrefs, setLocalPrefs] = createSignal<Partial<UserPreferences>>({});

  // Sync local state with server data
  createEffect(() => {
    if (userData()?.me?.preferences) {
      setLocalPrefs(userData()!.me.preferences);
    }
  });

  const handleSavePreferences = async () => {
    setSaveStatus('saving');
    try {
      await updatePreferences(UPDATE_PREFERENCES_MUTATION, { preferences: localPrefs() });
      setSaveStatus('saved');
      setTimeout(() => setSaveStatus('idle'), 2000);
    } catch (err) {
      setSaveStatus('error');
    }
  };

  const handleCreateApiKey = async () => {
    try {
      const result = await createApiKey(CREATE_API_KEY_MUTATION, {
        name: newApiKeyName(),
        permissions: newApiKeyPermissions(),
      });
      if (result?.createApiKey) {
        setGeneratedKey(result.createApiKey.key);
        userData.refetch();
      }
    } catch (err) {
      console.error('Failed to create API key:', err);
    }
  };

  const handleRevokeKey = async (keyId: string) => {
    if (!confirm('Are you sure you want to revoke this API key?')) return;
    try {
      await revokeApiKey(REVOKE_API_KEY_MUTATION, { keyId });
      userData.refetch();
    } catch (err) {
      console.error('Failed to revoke API key:', err);
    }
  };

  const updateLocalPref = <K extends keyof UserPreferences>(key: K, value: UserPreferences[K]) => {
    setLocalPrefs(prev => ({ ...prev, [key]: value }));
  };

  const updateNotificationPref = <K extends keyof UserPreferences['notifications']>(key: K, value: UserPreferences['notifications'][K]) => {
    setLocalPrefs(prev => ({
      ...prev,
      notifications: { ...prev.notifications, [key]: value } as UserPreferences['notifications'],
    }));
  };

  const updateDisplayPref = <K extends keyof UserPreferences['display']>(key: K, value: UserPreferences['display'][K]) => {
    setLocalPrefs(prev => ({
      ...prev,
      display: { ...prev.display, [key]: value } as UserPreferences['display'],
    }));
  };

  const updateDataPref = <K extends keyof UserPreferences['data']>(key: K, value: UserPreferences['data'][K]) => {
    setLocalPrefs(prev => ({
      ...prev,
      data: { ...prev.data, [key]: value } as UserPreferences['data'],
    }));
  };

  const languages = [
    { code: 'en', name: 'English' },
    { code: 'ja', name: 'Japanese' },
    { code: 'de', name: 'German' },
    { code: 'fr', name: 'French' },
    { code: 'ru', name: 'Russian' },
    { code: 'zh', name: 'Chinese' },
  ];

  const timezones = [
    'UTC',
    'America/New_York',
    'America/Chicago',
    'America/Denver',
    'America/Los_Angeles',
    'Europe/London',
    'Europe/Paris',
    'Europe/Berlin',
    'Asia/Tokyo',
    'Asia/Shanghai',
  ];

  const availablePermissions = [
    { value: 'read:readings', label: 'Read sensor data' },
    { value: 'read:alerts', label: 'Read alerts' },
    { value: 'write:alerts', label: 'Acknowledge alerts' },
    { value: 'read:simulations', label: 'Read simulations' },
    { value: 'write:simulations', label: 'Create simulations' },
  ];

  return (
    <div class="max-w-4xl mx-auto space-y-6">
      <div>
        <h1 class="text-2xl font-bold">Settings</h1>
        <p class="text-gray-400 mt-1">Configure your Cherenkov experience</p>
      </div>

      <div class="flex gap-6">
        {/* Sidebar */}
        <div class="w-48 space-y-1">
          {[
            { id: 'profile', label: 'Profile', icon: '👤' },
            { id: 'notifications', label: 'Notifications', icon: '🔔' },
            { id: 'display', label: 'Display', icon: '🎨' },
            { id: 'data', label: 'Data', icon: '💾' },
            { id: 'api', label: 'API Keys', icon: '🔑' },
          ].map(section => (
            <button
              onClick={() => setActiveSection(section.id as any)}
              class={`w-full text-left px-4 py-2 rounded-lg transition-colors flex items-center gap-2 ${
                activeSection() === section.id 
                  ? 'bg-[#00d4ff]/20 text-[#00d4ff]' 
                  : 'hover:bg-[#2a2a3a] text-gray-400'
              }`}
            >
              <span>{section.icon}</span>
              {section.label}
            </button>
          ))}
        </div>

        {/* Content */}
        <div class="flex-1 space-y-6">
          <Show when={activeSection() === 'profile'}>
            <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
              <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Profile</h2>
              
              <div class="space-y-4">
                <div>
                  <label class="block text-sm text-gray-400 mb-2">Email</label>
                  <input
                    type="email"
                    title="User email address"
                    value={userData()?.me?.email || ''}
                    disabled
                    class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 text-gray-500"
                  />
                </div>

                <div>
                  <label class="block text-sm text-gray-400 mb-2">Language</label>
                  <select
                    value={localPrefs().language || 'en'}
                    title="Select language"
                    onChange={(e) => updateLocalPref('language', e.currentTarget.value)}
                    class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
                  >

                    {languages.map(lang => (
                      <option value={lang.code}>{lang.name}</option>
                    ))}
                  </select>
                </div>

                <div>
                  <label class="block text-sm text-gray-400 mb-2">Timezone</label>
                  <select
                    value={localPrefs().timezone || 'UTC'}
                    title="Select timezone"
                    onChange={(e) => updateLocalPref('timezone', e.currentTarget.value)}
                    class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
                  >

                    {timezones.map(tz => (
                      <option value={tz}>{tz}</option>
                    ))}
                  </select>
                </div>
              </div>
            </div>
          </Show>

          <Show when={activeSection() === 'notifications'}>
            <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
              <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Notifications</h2>
              
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <div>
                    <p class="font-medium">Email Notifications</p>
                    <p class="text-sm text-gray-400">Receive alerts via email</p>
                  </div>
                  <button
                    title={localPrefs().notifications?.email ? 'Disable email notifications' : 'Enable email notifications'}
                    onClick={() => updateNotificationPref('email', !localPrefs().notifications?.email)}
                    class={`w-12 h-6 rounded-full transition-colors ${localPrefs().notifications?.email ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
                  >
                    <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${localPrefs().notifications?.email ? 'translate-x-6' : 'translate-x-1'}`}></span>
                  </button>

                </div>

                <div class="flex items-center justify-between">
                  <div>
                    <p class="font-medium">Push Notifications</p>
                    <p class="text-sm text-gray-400">Browser push notifications</p>
                  </div>
                  <button
                    title={localPrefs().notifications?.push ? 'Disable push notifications' : 'Enable push notifications'}
                    onClick={() => updateNotificationPref('push', !localPrefs().notifications?.push)}
                    class={`w-12 h-6 rounded-full transition-colors ${localPrefs().notifications?.push ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
                  >
                    <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${localPrefs().notifications?.push ? 'translate-x-6' : 'translate-x-1'}`}></span>
                  </button>

                </div>

                <div class="flex items-center justify-between">
                  <div>
                    <p class="font-medium">SMS Notifications</p>
                    <p class="text-sm text-gray-400">Critical alerts via SMS</p>
                  </div>
                  <button
                    title={localPrefs().notifications?.sms ? 'Disable SMS notifications' : 'Enable SMS notifications'}
                    onClick={() => updateNotificationPref('sms', !localPrefs().notifications?.sms)}
                    class={`w-12 h-6 rounded-full transition-colors ${localPrefs().notifications?.sms ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
                  >
                    <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${localPrefs().notifications?.sms ? 'translate-x-6' : 'translate-x-1'}`}></span>
                  </button>

                </div>

                <div class="pt-4 border-t border-[#2a2a3a]">
                  <div class="flex items-center justify-between mb-2">
                    <p class="font-medium">Alert Threshold (μSv/h)</p>
                    <span class="text-[#00d4ff] font-medium">{localPrefs().notifications?.alertThreshold || 0.5}</span>
                  </div>
                  <input
                    type="range"
                    min="0.1"
                    max="5"
                    step="0.1"
                    title="Adjust alert threshold"
                    value={localPrefs().notifications?.alertThreshold || 0.5}
                    onInput={(e) => updateNotificationPref('alertThreshold', parseFloat(e.currentTarget.value))}
                    class="w-full accent-[#00d4ff]"
                  />

                  <p class="text-xs text-gray-500 mt-1">Alerts triggered when dose rate exceeds this threshold</p>
                </div>
              </div>
            </div>
          </Show>

          <Show when={activeSection() === 'display'}>
            <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
              <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Display</h2>
              
              <div class="space-y-4">
                <div>
                  <label class="block text-sm text-gray-400 mb-2">Theme</label>
                  <div class="grid grid-cols-3 gap-2">
                    {['dark', 'light', 'system'].map(theme => (
                      <button
                        onClick={() => updateLocalPref('theme', theme as any)}
                        class={`px-4 py-2 rounded-lg border transition-colors ${
                          localPrefs().theme === theme 
                            ? 'border-[#00d4ff] bg-[#00d4ff]/10 text-[#00d4ff]' 
                            : 'border-[#2a2a3a] hover:border-[#3a3a4a]'
                        }`}
                      >
                        {theme.charAt(0).toUpperCase() + theme.slice(1)}
                      </button>
                    ))}
                  </div>
                </div>

                <div>
                  <label class="block text-sm text-gray-400 mb-2">Default View</label>
                  <select
                    value={localPrefs().display?.defaultView || 'dashboard'}
                    title="Select default view"
                    onChange={(e) => updateDisplayPref('defaultView', e.currentTarget.value as any)}
                    class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
                  >

                    <option value="dashboard">Dashboard</option>
                    <option value="globe">Globe</option>
                    <option value="sensors">Sensors</option>
                  </select>
                </div>

                <div class="flex items-center justify-between">
                  <div>
                    <p class="font-medium">Globe Auto-Rotation</p>
                    <p class="text-sm text-gray-400">Rotate globe when idle</p>
                  </div>
                  <button
                    title={localPrefs().display?.globeRotation ? 'Disable globe auto-rotation' : 'Enable globe auto-rotation'}
                    onClick={() => updateDisplayPref('globeRotation', !localPrefs().display?.globeRotation)}
                    class={`w-12 h-6 rounded-full transition-colors ${localPrefs().display?.globeRotation ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
                  >
                    <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${localPrefs().display?.globeRotation ? 'translate-x-6' : 'translate-x-1'}`}></span>
                  </button>

                </div>

                <div class="flex items-center justify-between">
                  <div>
                    <p class="font-medium">Show Labels</p>
                    <p class="text-sm text-gray-400">Display sensor labels on globe</p>
                  </div>
                  <button
                    title={localPrefs().display?.showLabels ? 'Hide sensor labels' : 'Show sensor labels'}
                    onClick={() => updateDisplayPref('showLabels', !localPrefs().display?.showLabels)}
                    class={`w-12 h-6 rounded-full transition-colors ${localPrefs().display?.showLabels ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
                  >
                    <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${localPrefs().display?.showLabels ? 'translate-x-6' : 'translate-x-1'}`}></span>
                  </button>

                </div>

                <div>
                  <label class="block text-sm text-gray-400 mb-2">Color Scheme</label>
                  <div class="grid grid-cols-3 gap-2">
                    {[
                      { value: 'default', label: 'Default', colors: 'bg-blue-500' },
                      { value: 'thermal', label: 'Thermal', colors: 'bg-gradient-to-r from-blue-500 via-yellow-500 to-red-500' },
                      { value: 'radiation', label: 'Radiation', colors: 'bg-green-500' },
                    ].map(scheme => (
                      <button
                        onClick={() => updateDisplayPref('colorScheme', scheme.value as any)}
                        class={`px-4 py-2 rounded-lg border transition-colors flex items-center gap-2 ${
                          localPrefs().display?.colorScheme === scheme.value 
                            ? 'border-[#00d4ff] bg-[#00d4ff]/10' 
                            : 'border-[#2a2a3a] hover:border-[#3a3a4a]'
                        }`}
                      >
                        <div class={`w-4 h-4 rounded ${scheme.colors}`}></div>
                        {scheme.label}
                      </button>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </Show>

          <Show when={activeSection() === 'data'}>
            <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
              <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Data</h2>
              
              <div class="space-y-4">
                <div>
                  <label class="block text-sm text-gray-400 mb-2">Default Time Range</label>
                  <select
                    value={localPrefs().data?.defaultTimeRange || 24}
                    title="Select default time range"
                    onChange={(e) => updateDataPref('defaultTimeRange', parseInt(e.currentTarget.value))}
                    class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
                  >

                    <option value={1}>Last hour</option>
                    <option value={6}>Last 6 hours</option>
                    <option value={24}>Last 24 hours</option>
                    <option value={168}>Last 7 days</option>
                    <option value={720}>Last 30 days</option>
                  </select>
                </div>

                <div class="flex items-center justify-between">
                  <div>
                    <p class="font-medium">Auto-Refresh</p>
                    <p class="text-sm text-gray-400">Automatically update data</p>
                  </div>
                  <button
                    title={localPrefs().data?.autoRefresh ? 'Disable auto-refresh' : 'Enable auto-refresh'}
                    onClick={() => updateDataPref('autoRefresh', !localPrefs().data?.autoRefresh)}
                    class={`w-12 h-6 rounded-full transition-colors ${localPrefs().data?.autoRefresh ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
                  >
                    <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${localPrefs().data?.autoRefresh ? 'translate-x-6' : 'translate-x-1'}`}></span>
                  </button>

                </div>

                <Show when={localPrefs().data?.autoRefresh}>
                  <div>
                    <label class="block text-sm text-gray-400 mb-2">Refresh Interval (seconds)</label>
                    <select
                      value={localPrefs().data?.refreshInterval || 30}
                      title="Select refresh interval"
                      onChange={(e) => updateDataPref('refreshInterval', parseInt(e.currentTarget.value))}
                      class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
                    >

                      <option value={5}>5 seconds</option>
                      <option value={10}>10 seconds</option>
                      <option value={30}>30 seconds</option>
                      <option value={60}>1 minute</option>
                      <option value={300}>5 minutes</option>
                    </select>
                  </div>
                </Show>

                <div class="pt-4 border-t border-[#2a2a3a]">
                  <button 
                    onClick={() => {
                      localStorage.clear();
                      window.location.reload();
                    }}
                    class="text-red-400 hover:text-red-300 text-sm font-medium"
                  >
                    Clear Local Cache & Reload
                  </button>
                </div>
              </div>
            </div>
          </Show>

          <Show when={activeSection() === 'api'}>
            <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
              <div class="flex items-center justify-between border-b border-[#2a2a3a] pb-4">
                <h2 class="text-lg font-semibold">API Keys</h2>
                <button
                  onClick={() => setShowApiKeyDialog(true)}
                  class="px-4 py-2 bg-[#00d4ff] text-black rounded-lg font-medium hover:bg-[#00d4ff]/90 transition-colors"
                >
                  + New Key
                </button>
              </div>
              
              <div class="space-y-3">
                <For each={userData()?.me?.apiKeys}>
                  {(key: { id: string; name: string; createdAt: string; lastUsed?: string; permissions: string[] }) => (
                    <div class="flex items-center justify-between p-4 bg-[#0a0a0f] rounded-lg">
                      <div>
                        <p class="font-medium">{key.name}</p>
                        <p class="text-xs text-gray-500">
                          Created: {new Date(key.createdAt).toLocaleDateString()}
                          {key.lastUsed && ` • Last used: ${new Date(key.lastUsed).toLocaleDateString()}`}
                        </p>
                        <div class="flex gap-1 mt-1">
                          {key.permissions.map((perm: string) => (
                            <span class="text-xs px-2 py-0.5 bg-[#2a2a3a] rounded">{perm}</span>
                          ))}
                        </div>
                      </div>

                      <button
                        onClick={() => handleRevokeKey(key.id)}
                        class="text-red-400 hover:text-red-300 text-sm"
                      >
                        Revoke
                      </button>
                    </div>
                  )}
                </For>
                
                <Show when={!userData()?.me?.apiKeys?.length}>
                  <p class="text-gray-500 text-center py-8">No API keys created</p>
                </Show>
              </div>

              <div class="bg-[#0a0a0f] rounded-lg p-4 text-sm text-gray-400">
                <p class="font-medium text-gray-300 mb-2">API Documentation</p>
                <p>Access the GraphQL API at <code class="bg-[#2a2a3a] px-1 rounded">/graphql</code></p>
                <p class="mt-1">WebSocket subscriptions at <code class="bg-[#2a2a3a] px-1 rounded">/graphql/ws</code></p>
              </div>
            </div>
          </Show>

          {/* Save Button */}
          <Show when={activeSection() !== 'api'}>
            <div class="flex items-center justify-end gap-4">
              <Show when={saveStatus() === 'saved'}>
                <span class="text-green-400 text-sm">Saved successfully</span>
              </Show>
              <Show when={saveStatus() === 'error'}>
                <span class="text-red-400 text-sm">Failed to save</span>
              </Show>
              <button
                onClick={handleSavePreferences}
                disabled={saveStatus() === 'saving'}
                class="px-6 py-2 bg-[#00d4ff] text-black rounded-lg font-semibold hover:bg-[#00d4ff]/90 transition-colors disabled:opacity-50"
              >
                {saveStatus() === 'saving' ? 'Saving...' : 'Save Changes'}
              </button>
            </div>
          </Show>
        </div>
      </div>

      {/* API Key Dialog */}
      <Show when={showApiKeyDialog()}>
        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 w-96">
            <Show when={!generatedKey()} fallback={
              <div class="space-y-4">
                <h3 class="text-lg font-semibold text-green-400">API Key Created</h3>
                <p class="text-sm text-gray-400">Copy this key now. It won't be shown again.</p>
                <div class="bg-[#0a0a0f] p-3 rounded-lg break-all font-mono text-sm">
                  {generatedKey()}
                </div>
                <button
                  onClick={() => {
                    setShowApiKeyDialog(false);
                    setGeneratedKey(null);
                    setNewApiKeyName('');
                    setNewApiKeyPermissions(['read:readings']);
                  }}
                  class="w-full bg-[#00d4ff] text-black py-2 rounded-lg font-semibold"
                >
                  Done
                </button>
              </div>
            }>
              <h3 class="text-lg font-semibold mb-4">Create API Key</h3>
              <div class="space-y-4">
                <div>
                  <label class="block text-sm text-gray-400 mb-1">Key Name</label>
                  <input
                    type="text"
                    title="Enter API key name"
                    value={newApiKeyName()}
                    onInput={(e) => setNewApiKeyName(e.currentTarget.value)}
                    placeholder="e.g., Production App"
                    class="w-full bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-3 py-2 focus:outline-none focus:border-[#00d4ff]"
                  />
                </div>

                
                <div>
                  <label class="block text-sm text-gray-400 mb-2">Permissions</label>
                  <div class="space-y-2">
                    {availablePermissions.map(perm => (
                      <label class="flex items-center gap-2 text-sm">
                        <input
                          type="checkbox"
                          checked={newApiKeyPermissions().includes(perm.value)}
                          onChange={(e) => {
                            if (e.currentTarget.checked) {
                              setNewApiKeyPermissions([...newApiKeyPermissions(), perm.value]);
                            } else {
                              setNewApiKeyPermissions(newApiKeyPermissions().filter(p => p !== perm.value));
                            }
                          }}
                          class="rounded border-[#2a2a3a]"
                        />
                        {perm.label}
                      </label>
                    ))}
                  </div>
                </div>

                <div class="flex gap-2 pt-2">
                  <button
                    onClick={() => setShowApiKeyDialog(false)}
                    class="flex-1 bg-[#2a2a3a] hover:bg-[#3a3a4a] py-2 rounded-lg"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleCreateApiKey}
                    disabled={!newApiKeyName() || newApiKeyPermissions().length === 0}
                    class="flex-1 bg-[#00d4ff] text-black py-2 rounded-lg font-semibold disabled:opacity-50"
                  >
                    Create
                  </button>
                </div>
              </div>
            </Show>
          </div>
        </div>
      </Show>
    </div>
  );
}

export default Settings;
