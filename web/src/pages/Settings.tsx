import { createSignal } from 'solid-js';

function Settings() {
  const [settings, setSettings] = createSignal({
    theme: 'dark',
    language: 'en',
    notifications: true,
    alertThreshold: 0.5,
    dataRetention: 30,
  });

  return (
    <div class="max-w-2xl space-y-6">
      <div>
        <h1 class="text-2xl font-bold">Settings</h1>
        <p class="text-gray-400 mt-1">Configure your Cherenkov experience</p>
      </div>

      <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
        <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Display</h2>
        
        <div class="flex items-center justify-between">
          <div>
            <p class="font-medium">Theme</p>
            <p class="text-sm text-gray-400">Choose your preferred color scheme</p>
          </div>
          <select
            value={settings().theme}
            onChange={(e) => setSettings(s => ({ ...s, theme: e.currentTarget.value }))}
            class="bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
          >
            <option value="dark">Dark</option>
            <option value="light">Light</option>
            <option value="system">System</option>
          </select>
        </div>

        <div class="flex items-center justify-between">
          <div>
            <p class="font-medium">Language</p>
            <p class="text-sm text-gray-400">Interface language</p>
          </div>
          <select
            value={settings().language}
            onChange={(e) => setSettings(s => ({ ...s, language: e.currentTarget.value }))}
            class="bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
          >
            <option value="en">English</option>
            <option value="ja">Japanese</option>
            <option value="de">German</option>
            <option value="fr">French</option>
          </select>
        </div>
      </div>

      <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
        <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Notifications</h2>
        
        <div class="flex items-center justify-between">
          <div>
            <p class="font-medium">Push Notifications</p>
            <p class="text-sm text-gray-400">Receive alerts for anomalies and critical events</p>
          </div>
          <button
            onClick={() => setSettings(s => ({ ...s, notifications: !s.notifications }))}
            class={`w-12 h-6 rounded-full transition-colors ${settings().notifications ? 'bg-[#00d4ff]' : 'bg-gray-600'}`}
          >
            <span class={`block w-5 h-5 bg-white rounded-full transition-transform ${settings().notifications ? 'translate-x-6' : 'translate-x-1'}`}></span>
          </button>
        </div>

        <div>
          <div class="flex items-center justify-between mb-2">
            <p class="font-medium">Alert Threshold (μSv/h)</p>
            <span class="text-[#00d4ff] font-medium">{settings().alertThreshold}</span>
          </div>
          <input
            type="range"
            min="0.1"
            max="5"
            step="0.1"
            value={settings().alertThreshold}
            onInput={(e) => setSettings(s => ({ ...s, alertThreshold: parseFloat(e.currentTarget.value) }))}
            class="w-full"
          />
        </div>
      </div>

      <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6 space-y-6">
        <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4">Data</h2>
        
        <div class="flex items-center justify-between">
          <div>
            <p class="font-medium">Data Retention (days)</p>
            <p class="text-sm text-gray-400">How long to keep historical data cached</p>
          </div>
          <select
            value={settings().dataRetention}
            onChange={(e) => setSettings(s => ({ ...s, dataRetention: parseInt(e.currentTarget.value) }))}
            class="bg-[#0a0a0f] border border-[#2a2a3a] rounded-lg px-4 py-2 focus:outline-none focus:border-[#00d4ff]"
          >
            <option value={7}>7 days</option>
            <option value={30}>30 days</option>
            <option value={90}>90 days</option>
            <option value={365}>1 year</option>
          </select>
        </div>

        <div class="pt-4 border-t border-[#2a2a3a]">
          <button class="text-red-400 hover:text-red-300 text-sm font-medium">
            Clear Local Cache
          </button>
        </div>
      </div>

      <div class="bg-[#12121a] rounded-xl border border-[#2a2a3a] p-6">
        <h2 class="text-lg font-semibold border-b border-[#2a2a3a] pb-4 mb-4">About</h2>
        <div class="space-y-2 text-sm text-gray-400">
          <p>Cherenkov v0.1.0</p>
          <p>Build: 2024.01.15-abc123</p>
          <p>License: AGPL-3.0</p>
          <p class="pt-2">© 2024 tworjaga</p>
        </div>
      </div>
    </div>
  );
}

export default Settings;
