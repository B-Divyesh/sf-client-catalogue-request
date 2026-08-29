import { defineConfig, devices } from '@playwright/test';

const liveBaseUrl=process.env.PLAYWRIGHT_BASE_URL;

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 30_000,
  use: { baseURL: liveBaseUrl||'http://127.0.0.1:4173', trace: 'retain-on-failure' },
  webServer: liveBaseUrl?undefined:[
    { command: 'node tests/license-fixture.mjs', url: 'http://127.0.0.1:8181/verify?license=health', reuseExistingServer: true },
    { command: 'DATA_DIR=.test-data/e2e WEB_DIST=dist PORT=8080 SOCIOBOT_LICENSE_VERIFY_BASE=http://127.0.0.1:8181/verify?license= cargo run', url: 'http://127.0.0.1:8080/health', reuseExistingServer: true, timeout:120_000 },
    { command: 'npm run dev -- --port 4173', url: 'http://127.0.0.1:4173', reuseExistingServer: true }
  ],
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'mobile', use: { ...devices['iPhone 13'], browserName: 'chromium', viewport: { width:390, height:844 } } }
  ]
});
