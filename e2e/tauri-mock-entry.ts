/**
 * Browser-side Tauri mock harness (Task 12). Bundled standalone by
 * `vite.harness.config.ts` into a dependency-free IIFE
 * (`e2e/.generated/tauri-mock-harness.js`) and injected into every test
 * page BEFORE any app script runs, via `page.addInitScript({ path })` in
 * `mocks.ts`. Re-exports the REAL `@tauri-apps/api/mocks` functions plus
 * `emit` from `@tauri-apps/api/event` -- the same pinned `@tauri-apps/api`
 * version `src/` itself bundles -- on `window.__muxsmithE2E__`, instead of
 * hand-reimplementing the `plugin:event|listen`/`plugin:event|emit` wire
 * contract those two modules already define (verified against their
 * shipped source, node_modules/@tauri-apps/api/{mocks,event}.js:
 * `mockIPC`'s `shouldMockEvents` option intercepts exactly those two
 * invoke names internally and replays a registered `listen()` handler on
 * `emit()`, which is exactly the mechanism `src-tauri/src/run.rs` uses in
 * production).
 */
import { clearMocks, mockIPC, mockWindows } from "@tauri-apps/api/mocks";
import { emit } from "@tauri-apps/api/event";

window.__muxsmithE2E__ = { mockIPC, mockWindows, clearMocks, emit };
