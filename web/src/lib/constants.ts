/**
 * Application constants
 */

export const APP_NAME = 'Cherenkov';

export const API_ENDPOINTS = {
  GRAPHQL: process.env.NEXT_PUBLIC_GRAPHQL_URL || 'http://localhost:8080/graphql',
  WEBSOCKET: process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws',
  REST: process.env.NEXT_PUBLIC_REST_URL || 'http://localhost:8080',
} as const;

export const DEFAULT_VIEWPORT = {
  latitude: 20,
  longitude: 0,
  zoom: 2,
  pitch: 0,
  bearing: 0,
} as const;

export const TIME_RANGES = {
  '1h': { label: '1 Hour', seconds: 3600 },
  '6h': { label: '6 Hours', seconds: 21600 },
  '24h': { label: '24 Hours', seconds: 86400 },
  '7d': { label: '7 Days', seconds: 604800 },
  '30d': { label: '30 Days', seconds: 2592000 },
} as const;

export const PLAYBACK_SPEEDS = [0.5, 1, 2, 5, 10] as const;

export const MAX_ALERTS = 100;

export const REFRESH_INTERVALS = {
  sensors: 30000,
  readings: 5000,
  status: 1000,
} as const;
