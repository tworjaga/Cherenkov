/**
 * API endpoint configuration
 * Centralized location for all API endpoints
 */

export const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export const apiEndpoints = {

  // Health
  health: '/api/health',

  // GraphQL
  graphql: '/graphql',
  subscriptions: '/subscriptions',

  // REST API
  sensors: '/api/v1/sensors',
  facilities: '/api/v1/facilities',
  anomalies: '/api/v1/anomalies',
  readings: '/api/v1/readings',
  alerts: '/api/v1/alerts',
  plume: '/api/v1/plume',

  // Auth
  login: '/api/v1/auth/login',
  logout: '/api/v1/auth/logout',
  refresh: '/api/v1/auth/refresh',
  me: '/api/v1/auth/me',

  // Settings
  settings: '/api/v1/settings',
  notifications: '/api/v1/settings/notifications',
  dataSources: '/api/v1/settings/data-sources',
  apiKeys: '/api/v1/settings/api-keys',
} as const;

export type ApiEndpoint = typeof apiEndpoints[keyof typeof apiEndpoints];

// Backward compatibility alias
export { apiEndpoints as API_ENDPOINTS };
