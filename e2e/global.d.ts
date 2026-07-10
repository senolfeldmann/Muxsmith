// Ambient typings for the mock harness's browser-side globals (Task 12).
// Shared by `tauri-mock-entry.ts` (defines `window.__muxsmithE2E__`) and
// `mocks.ts` (the `installMockIPC` function that runs inside the page via
// `page.addInitScript`, see that file's own doc for why it cannot import
// these instead).
export {};

declare global {
  interface Window {
    /** Set by `tauri-mock-entry.ts` (bundled by `vite.harness.config.ts`):
     * the real `@tauri-apps/api/mocks` functions plus `emit` from
     * `@tauri-apps/api/event`, re-exported onto `window` so a plain
     * `page.addInitScript`/`page.evaluate` call can reach them without a
     * bare-specifier import (browsers cannot resolve those natively). */
    __muxsmithE2E__: {
      mockIPC: (
        cb: (cmd: string, args?: unknown) => unknown,
        options?: { shouldMockEvents?: boolean },
      ) => void;
      mockWindows: (current: string, ...rest: string[]) => void;
      clearMocks: () => void;
      emit: (event: string, payload?: unknown) => Promise<void>;
    };
    /** `@tauri-apps/plugin-os`'s `platform()` reads this global directly
     * (verified: node_modules/@tauri-apps/plugin-os/dist-js/index.js has
     * no `invoke` call at all for `platform()`) -- neither `mockIPC` nor
     * `mockWindows` ever sets it, so `mocks.ts`'s `installMockIPC` does. */
    __TAURI_OS_PLUGIN_INTERNALS__?: Record<string, unknown>;
    /** Bound via `page.exposeFunction` in `mocks.ts`: every mocked
     * `invoke()` call is forwarded here so tests can assert on real
     * invocation evidence instead of a UI echo. */
    __muxsmithRecordInvoke__?: (cmd: string, argsJson: string) => void;
  }
}
