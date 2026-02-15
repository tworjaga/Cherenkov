import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [solid(), wasm()],
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
});
