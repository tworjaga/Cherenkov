import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: [
      'src/**/*.test.{ts,tsx}',
      'wasm/tests/**/*.test.ts',
    ],
    exclude: [
      'node_modules',
      'dist',
      'e2e',
    ],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.d.ts',
        '**/*.config.*',
      ],
    },
    benchmark: {
      include: ['src/**/*.bench.ts'],
      outputFile: './benchmark-report.json',
    },
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
      '@wasm': resolve(__dirname, './wasm'),
    },
  },
});
