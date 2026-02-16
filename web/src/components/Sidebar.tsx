import { A } from '@solidjs/router';

function Sidebar() {
  const navItems = [
    { path: '/', label: 'Dashboard', icon: '◈' },
    { path: '/globe', label: 'Globe', icon: '◉' },
    { path: '/sensors', label: 'Sensors', icon: '◆' },
    { path: '/anomalies', label: 'Anomalies', icon: '▲' },
    { path: '/plume', label: 'Plume Sim', icon: '◊' },
    { path: '/settings', label: 'Settings', icon: '⚙' },
  ];

  return (
    <aside aria-label="Navigation sidebar" class="w-64 bg-[#12121a] border-r border-[#2a2a3a] flex flex-col">
      <div class="p-6 border-b border-[#2a2a3a]">
        <h1 class="text-xl font-bold text-[#00d4ff]">Cherenkov</h1>
        <p class="text-xs text-gray-500 mt-1">Radiation Intelligence</p>
      </div>
      <nav class="flex-1 p-4 space-y-1">
        {navItems.map((item) => (
          <A
            href={item.path}
            title={`Navigate to ${item.label}`}
            class="flex items-center gap-3 px-4 py-3 rounded-lg text-gray-400 hover:bg-[#2a2a3a] hover:text-white transition-colors"
            activeClass="bg-[#00d4ff]/10 text-[#00d4ff]"
          >
            <span class="text-lg">{item.icon}</span>
            <span class="font-medium">{item.label}</span>
          </A>
        ))}

      </nav>
      <div class="p-4 border-t border-[#2a2a3a] text-xs text-gray-500">
        <p>v0.1.0-alpha</p>
      </div>
    </aside>
  );
}

export default Sidebar;
