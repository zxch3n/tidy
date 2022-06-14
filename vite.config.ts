import { defineConfig } from 'vite';
import path from 'path';
import react from '@vitejs/plugin-react';
import styleImport from 'vite-plugin-style-import';

// https://vitejs.dev/config/
export default defineConfig({
  css: {
    preprocessorOptions: {
      less: {
        javascriptEnabled: true,
      },
    },
  },
  plugins: [
    react(),
    styleImport({
      libs: [
        {
          libraryName: 'antd',
          esModule: true,
          resolveStyle: (name) => {
            return `antd/es/${name}/style/index`;
          },
        },
      ],
    }),
  ],
  build: {
    rollupOptions: {
      input: {
        lib: path.resolve(__dirname, 'src/main.tsx'),
        wasm: path.resolve(__dirname, 'wasm_dist/wasm.js'),
      },
    },
  },
  // @ts-ignore
  test: {},
});
