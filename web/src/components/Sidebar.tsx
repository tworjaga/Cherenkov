import { A, useLocation } from '@solidjs/router';

function Sidebar() {
  const location = useLocation();
  
  const navItems = [
    { path: '/', label: 'Dashboard', icon: 'dashboard' },
    { path: '/globe', label: 'Globe View', icon: 'globe' },
    { path: '/sensors', label: 'Sensors', icon: 'sensors' },
    { path: '/anomalies', label: 'Anomalies', icon: 'warning' },
    { path: '/plume', label: 'Plume Sim', icon: 'cloud' },
    { path: '/settings', label: 'Settings', icon: 'settings' },
  ];

  const isActive = (path: string) => location.pathname === path;

  return (
    <aside class="w-64 bg-[#12121a] border-r border-[#2a2a3a] flex flex-col">
      <div class="p-6 border-b border-[#2a2a3a]">
        <h1 class="text-xl font-bold text-[#00d4ff] tracking-tight">
          CHERENKOV
        </h1>
        <p class="text-xs text-gray-500 mt-1">Radiological Intelligence</p>
      </div>
      
      <nav class="flex-1 p-4">
        <ul class="space-y-2">
          {navItems.map(item => (
            <li>
              <A
                href={item.path}
                class={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                  isActive(item.path)
                    ? 'bg-[#00d4ff]/10 text-[#00d4ff] border border-[#00d4ff]/30'
                    : 'text-gray-400 hover:bg-[#1a1a25] hover:text-gray-200'
                }`}
              >
                <span class="text-lg">{getIcon(item.icon)}</span>
                <span class="font-medium">{item.label}</span>
              </A>
            </li>
          ))}
        </ul>
      </nav>
      
      <div class="p-4 border-t border-[#2a2a3a]">
        <div class="flex items-center gap-3 px-4 py-3 bg-[#1a1a25] rounded-lg">
          <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
          <span class="text-sm text-gray-400">50,247 sensors online</span>
        </div>
      </div>
    </aside>
  );
}

function getIcon(name: string): string {
  const icons: Record<string, string> = {
    dashboard: '◼',
    globe: '◐',
    sensors: '◎',
    warning: '⚠',
    cloud: '☁',
    settings: '⚙',
  };
  return icons[name] || '•';
}

export default Sidebar;
