import type { Config } from 'tailwindcss';

const config: Config = {
  darkMode: 'class',
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  safelist: [
    'bg-alert-normal',
    'bg-alert-low',
    'bg-alert-medium',
    'bg-alert-high',
    'bg-alert-critical',
    'text-alert-normal',
    'text-alert-low',
    'text-alert-medium',
    'text-alert-high',
    'text-alert-critical',
    'border-alert-normal',
    'border-alert-low',
    'border-alert-medium',
    'border-alert-high',
    'border-alert-critical',
  ],
  theme: {

    extend: {
      colors: {
        bg: {
          primary: '#050508',
          secondary: '#0a0a10',
          tertiary: '#12121a',
          hover: '#1a1a25',
          active: '#252535',
        },
        border: {
          subtle: '#1f1f2e',
          DEFAULT: '#2a2a3d',
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
      },
      fontFamily: {
        sans: ['Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'SF Mono', 'monospace'],
      },
      fontSize: {
        'display-lg': ['2rem', { lineHeight: '1.2', letterSpacing: '-0.02em', fontWeight: '700' }],
        'display-md': ['1.5rem', { lineHeight: '1.3', letterSpacing: '-0.01em', fontWeight: '600' }],
        'heading-xs': ['0.75rem', { lineHeight: '1.4', letterSpacing: '0.05em', fontWeight: '600' }],
        'body-sm': ['0.8125rem', { lineHeight: '1.5', fontWeight: '400' }],
        'body-xs': ['0.6875rem', { lineHeight: '1.4', fontWeight: '400' }],
        'mono-sm': ['0.8125rem', { lineHeight: '1.4', fontWeight: '400' }],
        'mono-xs': ['0.6875rem', { lineHeight: '1.3', fontWeight: '500' }],
      },
      spacing: {
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
      },
      height: {
        header: '56px',
        sidebar: '64px',
        'bottom-panel': '200px',
      },
      width: {
        sidebar: '64px',
        'right-panel': '320px',
      },
      transitionTimingFunction: {
        'ease-out': 'cubic-bezier(0.4, 0, 0.2, 1)',
        bounce: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
        smooth: 'cubic-bezier(0.45, 0, 0.55, 1)',
        snap: 'cubic-bezier(0, 0, 0.2, 1)',
      },
      transitionDuration: {
        fast: '150ms',
        normal: '300ms',
        slow: '500ms',
        slower: '800ms',
      },
      animation: {
        'pulse-slow': 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'pulse-fast': 'pulse 1s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite',
      },
      keyframes: {
        glow: {
          '0%, 100%': { boxShadow: '0 0 20px rgba(0, 212, 255, 0.15)' },
          '50%': { boxShadow: '0 0 40px rgba(0, 212, 255, 0.3)' },
        },
      },
      backdropBlur: {
        xs: '2px',
      },
    },
  },
  plugins: [],
};

export default config;
