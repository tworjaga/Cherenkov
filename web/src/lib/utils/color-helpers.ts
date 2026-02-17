/**
 * Color helper functions
 * Utilities for color manipulation and conversion
 */

/**
 * Convert hex color to RGB
 */
export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : null;
}

/**
 * Convert RGB to hex color
 */
export function rgbToHex(r: number, g: number, b: number): string {
  return '#' + [r, g, b].map((x) => {
    const hex = x.toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  }).join('');
}

/**
 * Interpolate between two colors
 */
export function interpolateColor(
  color1: string,
  color2: string,
  factor: number
): string {
  const c1 = hexToRgb(color1);
  const c2 = hexToRgb(color2);
  
  if (!c1 || !c2) return color1;
  
  const r = Math.round(c1.r + (c2.r - c1.r) * factor);
  const g = Math.round(c1.g + (c2.g - c1.g) * factor);
  const b = Math.round(c1.b + (c2.b - c1.b) * factor);
  
  return rgbToHex(r, g, b);
}

/**
 * Get color from a value within a range (heatmap style)
 */
export function getHeatmapColor(
  value: number,
  min: number,
  max: number,
  colors: string[] = ['#00ff00', '#ffff00', '#ff0000']
): string {
  if (value <= min) return colors[0];
  if (value >= max) return colors[colors.length - 1];
  
  const range = max - min;
  const normalized = (value - min) / range;
  const index = normalized * (colors.length - 1);
  const lowerIndex = Math.floor(index);
  const upperIndex = Math.ceil(index);
  const factor = index - lowerIndex;
  
  return interpolateColor(colors[lowerIndex], colors[upperIndex], factor);
}

/**
 * Adjust color brightness
 */
export function adjustBrightness(hex: string, percent: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;
  
  const r = Math.min(255, Math.max(0, rgb.r + (rgb.r * percent / 100)));
  const g = Math.min(255, Math.max(0, rgb.g + (rgb.g * percent / 100)));
  const b = Math.min(255, Math.max(0, rgb.b + (rgb.b * percent / 100)));
  
  return rgbToHex(Math.round(r), Math.round(g), Math.round(b));
}

/**
 * Convert hex to rgba with alpha
 */
export function hexToRgba(hex: string, alpha: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) return hex;
  return `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, ${alpha})`;
}
