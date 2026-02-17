/**
 * Application route definitions
 * Centralized routing configuration for the Cherenkov web application
 */

export const ROUTES = {
  // Public routes
  HOME: '/',
  LOGIN: '/login',
  REGISTER: '/register',
  FORGOT_PASSWORD: '/forgot-password',

  // Dashboard routes
  DASHBOARD: '/dashboard',
  GLOBE: '/globe',
  SENSORS: '/sensors',
  ANOMALIES: '/anomalies',
  PLUME: '/plume',

  // Settings routes
  SETTINGS: {
    ROOT: '/settings',
    GENERAL: '/settings/general',
    NOTIFICATIONS: '/settings/notifications',
    DATA_SOURCES: '/settings/data-sources',
    API_KEYS: '/settings/api-keys',
  },

  // API routes
  API: {
    HEALTH: '/api/health',
    GRAPHQL: '/api/graphql',
  },
} as const;

// Type for route paths
export type RoutePath = typeof ROUTES[keyof typeof ROUTES] | string;

// Helper to check if a path is active
export function isActivePath(currentPath: string, targetPath: string): boolean {
  if (targetPath === '/') {
    return currentPath === '/';
  }
  return currentPath.startsWith(targetPath);
}

// Navigation items for sidebar
export interface NavItem {
  label: string;
  path: string;
  icon: string;
  children?: NavItem[];
}

export const NAV_ITEMS: NavItem[] = [
  { label: 'Dashboard', path: ROUTES.DASHBOARD, icon: 'LayoutDashboard' },
  { label: 'Globe', path: ROUTES.GLOBE, icon: 'Globe' },
  { label: 'Sensors', path: ROUTES.SENSORS, icon: 'Radio' },
  { label: 'Anomalies', path: ROUTES.ANOMALIES, icon: 'AlertTriangle' },
  { label: 'Plume Simulator', path: ROUTES.PLUME, icon: 'Wind' },
];

export const SETTINGS_NAV_ITEMS: NavItem[] = [
  { label: 'General', path: ROUTES.SETTINGS.GENERAL, icon: 'Settings' },
  { label: 'Notifications', path: ROUTES.SETTINGS.NOTIFICATIONS, icon: 'Bell' },
  { label: 'Data Sources', path: ROUTES.SETTINGS.DATA_SOURCES, icon: 'Database' },
  { label: 'API Keys', path: ROUTES.SETTINGS.API_KEYS, icon: 'Key' },
];
