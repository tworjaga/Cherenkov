'use client';

import { motion } from 'framer-motion';
import {
  LayoutDashboard,
  Globe,
  Radio,
  AlertTriangle,
  Wind,
  Settings,
} from 'lucide-react';
import { useAppStore } from '@/stores';
import { ViewType } from '@/types';

const navItems: { id: ViewType; icon: React.ElementType; label: string }[] = [
  { id: 'dashboard', icon: LayoutDashboard, label: 'Dashboard' },
  { id: 'globe', icon: Globe, label: 'Globe' },
  { id: 'sensors', icon: Radio, label: 'Sensors' },
  { id: 'anomalies', icon: AlertTriangle, label: 'Anomalies' },
  { id: 'plume', icon: Wind, label: 'Plume' },
  { id: 'settings', icon: Settings, label: 'Settings' },
];

export const Sidebar = () => {
  const { view, setView, sidebarCollapsed } = useAppStore();

  return (
    <motion.aside
      data-testid="sidebar"
      initial={{ x: -64, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
      className="fixed left-0 top-header h-[calc(100vh-56px)] bg-bg-secondary border-r border-border-subtle z-40"
      style={{ width: sidebarCollapsed ? 64 : 240 }}
    >

      <nav className="flex flex-col gap-1 p-2">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = view === item.id;

          return (
            <motion.button
              key={item.id}
              data-testid={`nav-${item.id}`}
              onClick={() => setView(item.id)}
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
              className={`
                flex items-center gap-3 px-3 py-3 rounded-md transition-all duration-fast
                ${isActive 
                  ? 'bg-bg-active text-accent-primary border-l-2 border-accent-primary' 
                  : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'
                }
              `}
            >

              <Icon className="w-6 h-6" strokeWidth={1.5} />
              {!sidebarCollapsed && (
                <span className="text-body-sm font-medium">{item.label}</span>
              )}
            </motion.button>
          );
        })}
      </nav>
    </motion.aside>
  );
};
