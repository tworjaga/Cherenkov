import React from 'react';
import { useAppStore, View } from '../../stores/useAppStore';

const navItems: { id: View; label: string; icon: string }[] = [
  { id: 'DASHBOARD', label: 'Dashboard', icon: 'D' },
  { id: 'GLOBE', label: 'Globe', icon: 'G' },
  { id: 'SENSORS', label: 'Sensors', icon: 'S' },
  { id: 'ANOMALIES', label: 'Anomalies', icon: 'A' },
  { id: 'PLUME', label: 'Plume', icon: 'P' },
  { id: 'SETTINGS', label: 'Settings', icon: 'C' },
];

export const Sidebar: React.FC = () => {
  const view = useAppStore((state) => state.view);
  const setView = useAppStore((state) => state.setView);
  const collapsed = useAppStore((state) => state.sidebarCollapsed);

  return (
    <nav className={`${collapsed ? 'w-[64px]' : 'w-[200px]'} bg-bg-secondary border-r border-border-subtle flex flex-col py-4 shrink-0 transition-all duration-300`}>
      {navItems.map((item) => (
        <button
          key={item.id}
          onClick={() => setView(item.id)}
          className={`flex items-center gap-3 px-4 py-3 mx-2 rounded-lg transition-all ${
            view === item.id
              ? 'bg-accent-primary/10 text-accent-primary'
              : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'
          }`}
          title={item.label}
        >
          <span className="w-6 h-6 flex items-center justify-center font-semibold text-sm">
            {item.icon}
          </span>
          {!collapsed && (
            <span className="text-sm font-medium">{item.label}</span>
          )}
        </button>
      ))}
    </nav>
  );
};
