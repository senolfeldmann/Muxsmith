// Ambient typings for the mock harness's browser-side globals (Task 12).
// Shared by `tauri-mock-entry.ts` (defines `window.__muxsmithE2E__`) and
// `mocks.ts` (the `installMockIPC` function that runs inside the page via
// `page.addInitScript`, see that file's own doc for why it cannot import
// these instead).
export {};

// Type-only: erased at compile time, so importing the real pinned
// signatures here does not pull `@tauri-apps/api` into the
// `page.addInitScript`-serialized function bodies below. Pins the
// `__muxsmithE2E__` shape to the actual exports lockstep by construction,
// rather than a hand-mirrored signature that can silently drift from them.
import type { clearMocks, mockIPC, mockWindows } from "@tauri-apps/api/mocks";
import type { emit } from "@tauri-apps/api/event";

declare global {
  interface Window {
    /** Set by `tauri-mock-entry.ts` (bundled by `vite.harness.config.ts`):
     * the real `@tauri-apps/api/mocks` functions plus `emit` from
     * `@tauri-apps/api/event`, re-exported onto `window` so a plain
     * `page.addInitScript`/`page.evaluate` call can reach them without a
     * bare-specifier import (browsers cannot resolve those natively). */
    __muxsmithE2E__: {
      mockIPC: typeof mockIPC;
      mockWindows: typeof mockWindows;
      clearMocks: typeof clearMocks;
      emit: typeof emit;
    };
    /** `@tauri-apps/plugin-os`'s `platform()` reads this global directly
     * (verified: node_modules/@tauri-apps/plugin-os/dist-js/index.js has
     * no `invoke` call at all for `platform()`) -- neither `mockIPC` nor
     * `mockWindows` ever sets it, so `mocks.ts`'s `installMockIPC` does. */
    __TAURI_OS_PLUGIN_INTERNALS__?: Record<string, unknown>;
    /** Bound via `page.exposeFunction` in `mocks.ts`: every mocked
     * `invoke()` call is forwarded here so tests can assert on real
     * invocation evidence instead of a UI echo. */
    __muxsmithRecordInvoke__?: (cmd: string, args: unknown) => void;
    /** Set by `mount-entry.ts` (bundled by `vite.mount.config.ts`, wave-3
     * amendment): mounts one editor component in isolation onto `#mount`,
     * round-tripping its `modelValue`/`update:modelValue` v-model. See
     * `mount.ts` for the Playwright-side driver. */
    __muxsmithMount__: (spec: { component: string; props?: Record<string, unknown>; locale?: string }) => void;
    /** Set by `mount-entry.ts` (D104): merges `partial` into the props the
     * mounted component is rendered with, so a spec can deliver a second
     * value of a prop after mount (`__muxsmithMount__` passes `spec.props`
     * once). Reset with the props on every `__muxsmithMount__` call. */
    __muxsmithSetProps__: (partial: Record<string, unknown>) => void;
    /** Reads the mounted component's current model value (the live
     * `modelValue` the harness's wrapper root holds). */
    __muxsmithModel__: () => unknown;
    /** Every `update:modelValue` event the mounted component has emitted,
     * in order; reset on each `__muxsmithMount__` call. */
    __muxsmithEmitted__: Array<{ event: string; payload: unknown }>;
    /** Set by `mount-entry.ts`: renders a help topic to HTML through
     * `src/help/topics.ts` (D50/D51). Exposed so `help-topics.spec.ts`
     * can exercise the loader's fallback chain from the page without an
     * editor UI to open. */
    __muxsmithTopicHtml__: (helpId: string, locale: string) => string;
  }
}
