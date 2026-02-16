export const colors = {
  bg: {
    primary: '#050508',
    secondary: '#0a0a10',
    tertiary: '#12121a',
    hover: '#1a1a25',
    active: '#252535',
  },

  border: {
    subtle: '#1f1f2e',
    default: '#2a2a3d',
    active: '#3a3a50',
    accent: 'rgba(0, 212, 255, 0.25)',
  },

  accent: {
    primary: '#00d4ff',
    secondary: '#00a8cc',
    muted: '#006680',
    glow: 'rgba(0, 212, 255, 0.15)',
    pulse: 'rgba(0, 212, 255, 0.4)',
  },

  alert: {
    normal: '#00ff88',
    low: '#00d4ff',
    medium: '#ffb800',
    high: '#ff6b35',
    critical: '#ff3366',
  },

  text: {
    primary: '#ffffff',
    secondary: '#a0a0b0',
    tertiary: '#606070',
    disabled: '#404050',
  },

  data: {
    line: '#00d4ff',
    area: 'rgba(0, 212, 255, 0.1)',
    grid: '#1f1f2e',
    axis: '#606070',
  },
} as const;

export type Colors = typeof colors;
