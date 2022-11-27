/// <reference types="vitest" />

import { defineConfig } from 'vite';
import path from 'path';
import react from '@vitejs/plugin-react';
import { createStyleImportPlugin } from 'vite-plugin-style-import';
import { checker } from 'vite-plugin-checker';

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
    checker({
      typescript: {
        buildMode: true,
      },
      eslint: {
        lintCommand: 'eslint .',
      },
    }),
    react(),
    createStyleImportPlugin({
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
    lib: {
      entry: path.resolve(__dirname, 'src/index.ts'),
      name: 'tidy',
      fileName: 'tidy',
      formats: ['es'],
    },
    rollupOptions: {
      external: ['react'],
    },
  },
  test: {},
});
