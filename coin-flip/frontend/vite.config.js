import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import inject from '@rollup/plugin-inject';

// https://vitejs.dev/config/
export default defineConfig({ 
  define:{
    "global":{}
  },
  plugins: [react()],
  build:{
    rollupOptions: {
        plugins: [inject({ Buffer: ['Buffer', 'Buffer'] })],
    }
  }
});