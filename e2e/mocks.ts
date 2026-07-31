/**
 * Playwright-side Tauri IPC/event mocking for the smoke (Task 12, spec 10):
 * installs the real `@tauri-apps/api/mocks` harness (`tauri-mock-entry.ts`,
 * bundled by `vite.harness.config.ts`) into the page before any app script
 * runs, wires a per-command scripted response queue, and exposes a
 * Node-side call log plus an `emitEvent` helper so tests can drive
 * `muxsmith://job-event`/`muxsmith://run-finished` exactly like
 * `src-tauri/src/run.rs` does.
 *
 * Binding app contracts this mock respects (read directly from
 * `crates/muxsmith-core/src/executor/queue.rs` and `src-tauri/src/run.rs`,
 * not assumed): the app registers both window-event listeners BEFORE
 * invoking `start_run` (`JobsView.vue`'s `ensureListeners()`), and a soft
 * `start_run` outcome emits `run-finished` synchronously, before the
 * command's own promise resolves. `shouldMockEvents` preserves that
 * ordering here too, since `emit()`/`listen()` both go through the same
 * mocked `invoke()` the app's own bundled `@tauri-apps/api/event` uses --
 * nothing about event delivery is reimplemented, only scripted responses
 * for plain commands are.
 */
import { resolve } from "node:path";
import type { Page } from "@playwright/test";

const HARNESS_PATH = resolve(import.meta.dirname, ".generated/tauri-mock-harness.js");

export type MockResult =
  | { kind: "resolve"; value: unknown }
  | { kind: "reject"; error: { code: string; params?: Record<string, string | number> } }
  | { kind: "gated"; value: unknown; gate: string };

export function resolveWith(value: unknown): MockResult {
  return { kind: "resolve", value };
}

export function rejectWith(code: string, params: Record<string, string | number> = {}): MockResult {
  return { kind: "reject", error: { code, params } };
}

/** A response that does not resolve until the test calls `releaseGate`
 *  with the same `gate` name (below). Widens the window between a
 *  command's call and its resolution wide enough to interleave a real,
 *  driven Playwright action (e.g. an edit made while a save is in
 *  flight) -- the same-tick microtask gap an immediately-resolving mock
 *  leaves is too narrow for any driven UI action to land inside, and this
 *  repo's test suite otherwise has no timing-based wait (`waitForTimeout`)
 *  to fall back on, deliberately: a fixed real-world sleep is exactly the
 *  kind of flaky, hardware-speed-dependent mechanism this gate replaces
 *  with a deterministic one. */
export function gatedWith(value: unknown, gate: string): MockResult {
  return { kind: "gated", value, gate };
}

export interface MockScenario {
  /** `window.__TAURI_OS_PLUGIN_INTERNALS__.platform` (D28's per-OS install
   * guidance): `@tauri-apps/plugin-os`'s `platform()` reads this global
   * directly rather than going through `invoke` (see `global.d.ts`), so
   * `installMockIPC` sets it explicitly instead of the mock IPC handler
   * below ever seeing a `plugin:os|platform` command. */
  platform?: string;
  /** Per-command response queue, consumed in call order; once exhausted,
   * the last entry repeats for any further call (covers an incidental
   * extra call -- e.g. `RunHistory.vue`'s own `list_runs` refresh after a
   * run finishes -- without needing an exact call count per scenario). An
   * unmocked command throws in the page instead of silently hanging the
   * app's `invoke()` promise forever. */
  commands: Record<string, MockResult[]>;
}

export interface RecordedInvoke {
  cmd: string;
  args: unknown;
}

/**
 * Runs INSIDE the page: Playwright serializes this whole function body via
 * `page.addInitScript(fn, arg)` and re-evaluates it fresh on every
 * navigation, so it must not close over anything from this module's scope
 * beyond the `scenario` argument itself (see `global.d.ts` for the ambient
 * `window.__muxsmithE2E__`/`__TAURI_OS_PLUGIN_INTERNALS__`/
 * `__muxsmithRecordInvoke__` typings it relies on).
 *
 * Exported so a test can register a SECOND scenario mid-test via its own
 * `page.addInitScript(installMockIPC, otherScenario)` call, layered on top
 * of `installTauriMocks`'s own registration: `@tauri-apps/api/mocks`'
 * `mockIPC` reassigns `window.__TAURI_INTERNALS__.invoke` outright (last
 * registration wins), so a script added after the first navigation has no
 * effect until the next one -- exactly what a "settings change takes
 * effect after the app restarts" case needs. This models the RESTART path
 * specifically: `main.ts` resolves the locale once, before mount, so
 * swapping the mock and reloading stands in for a restart with the new
 * locale. The app DOES also swap the live `FluentBundle`s in place when
 * settings are saved (D56, `SettingsDialog.save()` -> `applyLocale`); that
 * live path is a separate scenario, covered in `e2e/locale-switch.spec.ts`.
 * Calling `installTauriMocks` a second time on the same page cannot do this:
 * its
 * own `page.exposeFunction("__muxsmithRecordInvoke__", ...)` throws on a
 * second registration for the same name.
 */
export function installMockIPC(scenario: MockScenario): void {
  const queues = new Map<string, MockResult[]>(
    Object.entries(scenario.commands).map(([cmd, results]) => [cmd, [...results]]),
  );

  function nextResult(cmd: string): MockResult | undefined {
    const q = queues.get(cmd);
    if (!q || q.length === 0) {
      return undefined;
    }
    return q.length > 1 ? q.shift() : q[0];
  }

  window.__muxsmithE2E__.mockWindows("main");
  window.__TAURI_OS_PLUGIN_INTERNALS__ = { platform: scenario.platform ?? "linux" };

  // Resolvers for any `gatedWith` response this scenario queued, keyed by
  // gate name; `releaseGate` (Node side, below) triggers one via
  // `window.__muxsmithReleaseGate__`.
  const gateResolvers = new Map<string, () => void>();
  window.__muxsmithReleaseGate__ = (gate: string) => {
    gateResolvers.get(gate)?.();
    gateResolvers.delete(gate);
  };

  window.__muxsmithE2E__.mockIPC(
    (cmd, args) => {
      window.__muxsmithRecordInvoke__?.(cmd, args ?? null);

      const result = nextResult(cmd);
      if (result) {
        if (result.kind === "reject") {
          return Promise.reject(result.error);
        }
        if (result.kind === "gated") {
          return new Promise((resolve) => {
            gateResolvers.set(result.gate, () => resolve(result.value));
          });
        }
        return result.value;
      }
      // Incidental background commands every scenario touches regardless
      // of what it is actually testing: both views mount eagerly at
      // startup (T10's `v-show` contract, App.vue), so `RunHistory.vue`'s
      // own `onMounted(refresh)` always fires `list_runs`, and
      // `main.ts`'s locale bootstrap plus every view's own settings reads
      // always fire `get_settings`.
      if (cmd === "list_runs") {
        return [];
      }
      if (cmd === "get_settings") {
        return {
          mkvmerge_path: null,
          default_jobs: 1,
          locale: "en",
          recent_profiles: [],
          dir_memory: {},
        };
      }
      if (cmd === "set_settings" || cmd === "plugin:fs|write_text_file") {
        return null;
      }
      throw new Error(`e2e mock: unmocked command "${cmd}"`);
    },
    { shouldMockEvents: true },
  );
}

/**
 * Installs the harness bundle plus this scenario's IPC responses, and
 * wires a Node-side call log fed from the page via `page.exposeFunction`:
 * real invocation evidence, not a UI echo (Task 12's self-review
 * requirement) -- e.g. a clipboard-copy assertion checks that
 * `plugin:clipboard-manager|write_text` was actually called with the
 * expected text, not just that a local "Copied" flag flipped in the DOM.
 * Call once per test, before `page.goto`.
 */
export async function installTauriMocks(
  page: Page,
  scenario: MockScenario,
): Promise<RecordedInvoke[]> {
  const recorded: RecordedInvoke[] = [];
  await page.exposeFunction("__muxsmithRecordInvoke__", (cmd: string, args: unknown) => {
    recorded.push({ cmd, args });
  });
  await page.addInitScript({ path: HARNESS_PATH });
  await page.addInitScript(installMockIPC, scenario);
  return recorded;
}

/**
 * Drives `muxsmith://job-event`/`muxsmith://run-finished` (or any other
 * window event) exactly like `src-tauri/src/run.rs` emits them: through
 * the real bundled `emit()` (`window.__muxsmithE2E__.emit`), which goes
 * through the same mocked `invoke("plugin:event|emit", ...)` the app's own
 * `listen()` registrations are recorded against.
 */
export async function emitEvent(page: Page, channel: string, payload: unknown): Promise<void> {
  await page.evaluate(
    ({ channel, payload }) => window.__muxsmithE2E__.emit(channel, payload),
    { channel, payload },
  );
}

/** Resolves a `gatedWith(value, gate)` response queued for this `gate`,
 *  letting the awaited `invoke()` call it stands in for finally settle.
 *  Call it after driving whatever real UI action needed to happen while
 *  that call was still in flight. */
export async function releaseGate(page: Page, gate: string): Promise<void> {
  await page.evaluate((g) => window.__muxsmithReleaseGate__?.(g), gate);
}
