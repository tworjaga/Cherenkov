/**
 * Color scales for data visualization on the globe
 */

export const ALERT_COLORS = {
  normal: '#00ff88',
  low: '#00d4ff',
  medium: '#ffb800',
  high: '#ff6b35',
  critical: '#ff3366',
} as const;

export const getAlertColor = (level: number): string => {
  if (level <= 1) return ALERT_COLORS.normal;
  if (level <= 2) return ALERT_COLORS.low;
  if (level <= 3) return ALERT_COLORS.medium;
  if (level <= 4) return ALERT_COLORS.high;
  return ALERT_COLORS.critical;
};

export const getDoseRateColor = (doseRate: number): string => {
  // Microsieverts per hour
  if (doseRate < 0.1) return ALERT_COLORS.normal;
  if (doseRate < 1.0) return ALERT_COLORS.low;
  if (doseRate < 10.0) return ALERT_COLORS.medium;
  if (doseRate < 100.0) return ALERT_COLORS.high;
  return ALERT_COLORS.critical;
};

export const interpolateColor = (
  value: number,
  min: number,
  max: number,
  colorStart: string,
  colorEnd: string
): string => {
  const ratio = Math.max(0, Math.min(1, (value - min) / (max - min)));
  
  const hexToRgb = (hex: string) => {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? {
      r: parseInt(result[1], 16),
      g: parseInt(result[2], 16),
      b: parseInt(result[3], 16),
    } : { r: 0, g: 0, b: 0 };
  };
  
  const start = hexToRgb(colorStart);
  const end = hexToRgb(colorEnd);
  
  const r = Math.round(start.r + (end.r - start.r) * ratio);
  const g = Math.round(start.g + (end.g - start.g) * ratio);
  const b = Math.round(start.b + (end.b - start.b) * ratio);
  
  return `rgb(${r}, ${g}, ${b})`;
};
