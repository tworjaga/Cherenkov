/**
 * Application routes configuration
 */

export const routes = {
  home: '/',
  dashboard: '/',
  globe: '/globe',
  sensors: '/sensors',
  anomalies: '/anomalies',
  plume: '/plume',
  settings: {
    root: '/settings',
    general: '/settings/general',
    notifications: '/settings/notifications',
    dataSources: '/settings/data-sources',
    apiKeys: '/settings/api-keys',
  },
  auth: {
    login: '/login',
    logout: '/logout',
  },
  api: {
    health: '/api/health',
  },
} as const;

export type Routes = typeof routes;
