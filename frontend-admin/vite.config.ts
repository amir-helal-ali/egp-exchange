import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  server: { port: 3007, proxy: { '/api': 'http://localhost:3005', '/ws': { target: 'ws://localhost:3005', ws: true } } }
});
