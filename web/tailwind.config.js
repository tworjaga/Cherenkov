/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'bg-primary': '#050508',
        'bg-secondary': '#0a0a10',
        'bg-tertiary': '#12121a',
        'bg-hover': '#1a1a25',
        'border-subtle': '#1f1f2e',
        'border-active': '#2a2a3d',
        'accent-primary': '#00d4ff',
        'accent-secondary': '#00a8cc',
        'accent-glow': 'rgba(0, 212, 255, 0.15)',
        'alert-critical': '#ff3366',
        'alert-high': '#ff6b35',
        'alert-medium': '#ffb800',
        'alert-low': '#00d4ff',
        'alert-normal': '#00ff88',
        'text-primary': '#ffffff',
        'text-secondary': '#a0a0b0',
        'text-tertiary': '#606070',
        'data-line': '#00d4ff',
        'data-area': 'rgba(0, 212, 255, 0.1)',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      spacing: {
        '1': '4px',
        '2': '8px',
        '3': '12px',
        '4': '16px',
        '5': '20px',
        '6': '24px',
        '8': '32px',
        '10': '40px',
        '12': '48px',
      },
      animation: {
        'alert-pulse': 'alert-pulse 2s ease-in-out infinite',
        'glow': 'glow 2s ease-in-out infinite',
      },
      keyframes: {
        'alert-pulse': {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.6' },
        },
        'glow': {
          '0%, 100%': { boxShadow: '0 0 20px rgba(0, 212, 255, 0.3)' },
          '50%': { boxShadow: '0 0 30px rgba(0, 212, 255, 0.5)' },
        },
      },
      transitionTimingFunction: {
        'panel': 'cubic-bezier(0.4, 0, 0.2, 1)',
        'fly-to': 'cubic-bezier(0.45, 0, 0.55, 1)',
        'alert': 'cubic-bezier(0.34, 1.56, 0.64, 1)',
      },
    },
  },
  plugins: [],
};
