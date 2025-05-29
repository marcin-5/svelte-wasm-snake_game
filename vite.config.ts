import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [svelte(), wasm(), topLevelAwait()],
  css: {
    postcss: './postcss.config.cjs',
  },
  build: {
    target: 'esnext',
  },
  optimizeDeps: {
    exclude: ['@wasm/pkg'],
  },
});
