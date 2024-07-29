import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

const __dirname = new URL(import.meta.url).pathname

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: [
      { find: '@types/', replacement: `${__dirname}/src/types` },
    ],
  },
})
