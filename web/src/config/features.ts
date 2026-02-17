/**
 * Feature flags configuration
 */

export const features = {
  // Globe visualization
  enableGlobe: true,
  enableHeatmap: true,
  enablePlumeSimulation: true,
  
  // Real-time features
  enableWebSocket: true,
  enableNotifications: true,
  
  // Data sources
  enableMockData: process.env.NEXT_PUBLIC_ENABLE_MOCK_DATA === 'true',
  enableAnalytics: process.env.NEXT_PUBLIC_ENABLE_ANALYTICS === 'true',
  
  // UI features
  enableKeyboardShortcuts: true,
  enableTooltips: true,
  enableAnimations: true,
  
  // Development features
  enableDebugMode: process.env.NODE_ENV === 'development',
  enableStorybook: process.env.NODE_ENV === 'development',
} as const;

export type Features = typeof features;
