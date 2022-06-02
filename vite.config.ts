import { defineConfig } from 'vite';
import path from 'path';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
  ],
  build: {
    rollupOptions: {
      input: {
        'lib': path.resolve(__dirname, 'src/main.tsx'),
        'wasm': path.resolve(__dirname, 'wasm_dist/wasm.js'),
      },
    },
  },
  // @ts-ignore
  test: {}
});
