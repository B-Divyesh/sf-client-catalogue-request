import { defineConfig } from 'vitest/config';

export default defineConfig({
  build: { target: 'es2022', sourcemap: false, outDir: 'dist' },
  server: { proxy: { '/api': 'http://localhost:8080', '/health': 'http://localhost:8080' } },
  test: { exclude: ['tests/e2e/**', 'node_modules/**'] }
});
