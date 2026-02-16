import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';

export default defineConfig(({ mode }) => ({
  plugins: mode === 'test' ? [react()] : [react(), wasm()],

  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test/setup.ts'],
    pool: 'vmThreads',
    poolOptions: {
      threads: {
        singleThread: true,
      },
    },
  },

  server: {
    port: 3000,
    proxy: {
      '/graphql': 'http://localhost:8080',
      '/ws': {
        target: 'ws://localhost:8080',
        ws: true,
      },
    },
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
  },
}));
