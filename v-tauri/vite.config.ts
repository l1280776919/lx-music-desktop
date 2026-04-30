import path from 'node:path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { nodePolyfills } from 'vite-plugin-node-polyfills'

const host = process.env.TAURI_DEV_HOST
const rootDir = __dirname
const srcDir = path.resolve(rootDir, 'src')

export default defineConfig(async() => ({
  plugins: [
    vue(),
    nodePolyfills({
      include: ['buffer', 'process', 'crypto', 'stream', 'util', 'path', 'os'],
      globals: {
        Buffer: true,
        global: true,
        process: true,
      },
      protocolImports: true,
    }),
  ],
  resolve: {
    alias: [
      { find: '@renderer/utils/ipc', replacement: path.resolve(srcDir, 'tauri/ipc.ts') },
      { find: '@renderer/utils/music', replacement: path.resolve(srcDir, 'tauri/music.ts') },
      { find: '@renderer/worker/utils', replacement: path.resolve(srcDir, 'tauri/worker-utils.ts') },
      { find: '@renderer/worker', replacement: path.resolve(srcDir, 'tauri/worker-index.ts') },
      { find: '@common/theme/utils', replacement: path.resolve(srcDir, 'tauri/theme-utils.ts') },
      { find: '@renderer/utils/musicSdk/kg/vendors/infSign.min', replacement: path.resolve(srcDir, 'tauri/shims/infSign.ts') },
      { find: '@common/rendererIpc', replacement: path.resolve(srcDir, 'tauri/rendererIpc.ts') },
      { find: '@common/utils/electron', replacement: path.resolve(srcDir, 'tauri/electron.ts') },
      { find: '@common/utils/nodejs', replacement: path.resolve(srcDir, 'tauri/nodejs.ts') },
      { find: 'needle', replacement: path.resolve(srcDir, 'tauri/shims/needle.ts') },
      { find: 'zlib', replacement: path.resolve(srcDir, 'tauri/shims/zlib.ts') },
      { find: 'tunnel', replacement: path.resolve(srcDir, 'tauri/shims/tunnel.ts') },
      { find: 'electron-log/node', replacement: path.resolve(srcDir, 'tauri/shims/electron-log.ts') },
      { find: '@renderer', replacement: path.resolve(srcDir, 'lx-app/renderer') },
      { find: '@common', replacement: path.resolve(srcDir, 'lx-app/common') },
      { find: '@root', replacement: path.resolve(srcDir, 'lx-app') },
    ],
  },
  define: {
    'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV ?? 'development'),
    'process.env.ELECTRON_DISABLE_SECURITY_WARNINGS': JSON.stringify('true'),
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    fs: {
      allow: [rootDir],
    },
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}))
