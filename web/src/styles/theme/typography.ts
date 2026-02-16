export const fonts = {
  sans: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
  mono: 'JetBrains Mono, "Fira Code", "SF Mono", monospace',
} as const;

export const typography = {
  'display-lg': {
    fontSize: '2rem',
    fontWeight: 700,
    lineHeight: 1.2,
    letterSpacing: '-0.02em',
    fontFamily: fonts.sans,
  },
  'display-md': {
    fontSize: '1.5rem',
    fontWeight: 600,
    lineHeight: 1.3,
    letterSpacing: '-0.01em',
    fontFamily: fonts.sans,
  },
  'heading-xs': {
    fontSize: '0.75rem',
    fontWeight: 600,
    lineHeight: 1.4,
    letterSpacing: '0.05em',
    textTransform: 'uppercase',
    fontFamily: fonts.sans,
  },
  'body-sm': {
    fontSize: '0.8125rem',
    fontWeight: 400,
    lineHeight: 1.5,
    fontFamily: fonts.sans,
  },
  'body-xs': {
    fontSize: '0.6875rem',
    fontWeight: 400,
    lineHeight: 1.4,
    fontFamily: fonts.sans,
  },
  'mono-sm': {
    fontSize: '0.8125rem',
    fontWeight: 400,
    lineHeight: 1.4,
    fontFamily: fonts.mono,
    fontVariantNumeric: 'tabular-nums',
  },
  'mono-xs': {
    fontSize: '0.6875rem',
    fontWeight: 500,
    lineHeight: 1.3,
    fontFamily: fonts.mono,
    fontVariantNumeric: 'tabular-nums',
  },
} as const;

export type Typography = typeof typography;
