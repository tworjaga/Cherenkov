/**
 * Environment configuration
 */

export const env = {
  NODE_ENV: process.env.NODE_ENV || 'development',
  
  // API URLs
  GRAPHQL_URL: process.env.NEXT_PUBLIC_GRAPHQL_URL || 'http://localhost:8080/graphql',
  WS_URL: process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws',
  REST_URL: process.env.NEXT_PUBLIC_REST_URL || 'http://localhost:8080',
  
  // Features
  ENABLE_MOCK_DATA: process.env.NEXT_PUBLIC_ENABLE_MOCK_DATA === 'true',
  ENABLE_ANALYTICS: process.env.NEXT_PUBLIC_ENABLE_ANALYTICS === 'true',
  
  // Auth
  AUTH_TOKEN_KEY: 'cherenkov_auth_token',
  
  // Version
  APP_VERSION: process.env.NEXT_PUBLIC_APP_VERSION || '1.0.0',
} as const;

export type Env = typeof env;
