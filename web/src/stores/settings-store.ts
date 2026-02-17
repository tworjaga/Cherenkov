/**
 * Settings store
 * Manages user preferences and application settings
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { UserSettings, NotificationSettings, DashboardSettings, GlobeSettings } from '@/types';

interface SettingsState {
  settings: UserSettings | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  setSettings: (settings: UserSettings) => void;
  updateTheme: (theme: UserSettings['theme']) => void;
  updateNotifications: (notifications: Partial<NotificationSettings>) => void;
  updateDashboard: (dashboard: Partial<DashboardSettings>) => void;
  updateGlobe: (globe: Partial<GlobeSettings>) => void;
  resetSettings: () => void;
}

const defaultSettings: Partial<UserSettings> = {
  theme: 'system',
  language: 'en',
  timezone: 'UTC',
  dateFormat: 'YYYY-MM-DD',
  timeFormat: '24h',
  notifications: {
    email: true,
    push: true,
    sms: false,
    desktop: true,
    alertThreshold: 'warning',
    quietHours: {
      enabled: false,
      start: '22:00',
      end: '08:00',
    },
  },
  dashboard: {
    defaultView: 'overview',
    refreshInterval: 30,
    showWelcomeMessage: true,
    compactMode: false,
    widgets: [],
  },
  globe: {
    defaultZoom: 3,
    defaultCenter: { lat: 20, lng: 0 },
    showLabels: true,
    showBorders: true,
    terrainEnabled: true,
    atmosphereEnabled: true,
    layers: {
      sensors: true,
      facilities: true,
      anomalies: true,
      plumes: false,
      heatmap: false,
    },
  },
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set, get) => ({
      settings: null,
      isLoading: false,
      error: null,

      setSettings: (settings) => set({ settings, error: null }),

      updateTheme: (theme) => {
        const { settings } = get();
        if (settings) {
          set({ settings: { ...settings, theme } });
        }
      },

      updateNotifications: (notifications) => {
        const { settings } = get();
        if (settings) {
          set({
            settings: {
              ...settings,
              notifications: { ...settings.notifications, ...notifications },
            },
          });
        }
      },

      updateDashboard: (dashboard) => {
        const { settings } = get();
        if (settings) {
          set({
            settings: {
              ...settings,
              dashboard: { ...settings.dashboard, ...dashboard },
            },
          });
        }
      },

      updateGlobe: (globe) => {
        const { settings } = get();
        if (settings) {
          set({
            settings: {
              ...settings,
              globe: { ...settings.globe, ...globe },
            },
          });
        }
      },

      resetSettings: () => set({ settings: defaultSettings as UserSettings, error: null }),

    }),
    {
      name: 'cherenkov-settings',
      partialize: (state) => ({ settings: state.settings }),
    }
  )
);
