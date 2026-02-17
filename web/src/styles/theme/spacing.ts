/**
 * Spacing design tokens - 4pt grid system
 */

export const spacing = {
  '0': '0',
  '0.5': '2px',
  '1': '4px',
  '2': '8px',
  '3': '12px',
  '4': '16px',
  '5': '20px',
  '6': '24px',
  '8': '32px',
  '10': '40px',
  '12': '48px',
  '16': '64px',
  '20': '80px',
  '24': '96px',
} as const;

export type Spacing = keyof typeof spacing;

// Panel dimensions
export const layout = {
  header: { height: '56px' },
  sidebar: { width: '64px', widthExpanded: '240px' },
  rightPanel: { width: '320px', widthLg: '280px', widthMd: '240px' },
  bottomPanel: { height: '200px' },
  gap: '16px',
} as const;
