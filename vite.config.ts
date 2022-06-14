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
    lib: {
      entry: path.resolve(__dirname, 'src/index.ts'),
      name: 'tidy',
      fileName: (format) => `tidy.${format}.js`,
    },
    rollupOptions: {
      external: ['react'],
    },
  },
  // @ts-ignore
  test: {},
});
