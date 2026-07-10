import { defineConfig, devices } from "@playwright/test";

// Task 12 (spec 10, thin smoke): drives the BUILT frontend (`vite preview`
// over `dist/`) in a plain Chromium browser with `@tauri-apps/api/mocks`
// standing in for the Tauri IPC bridge -- no tauri-driver, no real webview
// window (see e2e/mocks.ts). Chromium only: this smoke checks the Vue
// app's own behavior against the documented IPC/event contract, not
// cross-engine rendering. Precondition: `dist/` must already exist
// (`pnpm build`), which the per-commit gate runs before `pnpm test:e2e`.
export default defineConfig({
  testDir: "./e2e",
  testMatch: "**/*.spec.ts",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:4173",
    locale: "en-US",
    trace: "retain-on-failure",
  },
  webServer: {
    // `--host 127.0.0.1` forced explicitly: vite preview's default
    // `localhost` bind resolved to `[::1]` only on the machine this was
    // authored on, so Playwright's own IPv4 health check against
    // `127.0.0.1` timed out against a server that was, in fact, already
    // up (verified: curl to `[::1]:4173` succeeded, `127.0.0.1:4173`
    // connection-refused, `ss -tlnp` showed `[::1]:4173` only). Pinning
    // the bind address removes that ambiguity instead of hoping every
    // runner's `localhost` resolution order happens to match.
    command: "vite preview --host 127.0.0.1 --port 4173 --strictPort",
    url: "http://127.0.0.1:4173",
    reuseExistingServer: !process.env.CI,
  },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
});
