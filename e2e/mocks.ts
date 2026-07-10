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
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Page } from "@playwright/test";

const HARNESS_PATH = resolve(
  dirname(fileURLToPath(import.meta.url)),
  ".generated/tauri-mock-harness.js",
);

export type MockResult =
  | { kind: "resolve"; value: unknown }
  | { kind: "reject"; error: { code: string; params?: Record<string, string> } };

export function resolveWith(value: unknown): MockResult {
  return { kind: "resolve", value };
}

export function rejectWith(code: string, params: Record<string, string> = {}): MockResult {
  return { kind: "reject", error: { code, params } };
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
 */
function installMockIPC(scenario: MockScenario): void {
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

  window.__muxsmithE2E__.mockIPC(
    (cmd, args) => {
      window.__muxsmithRecordInvoke__?.(cmd, JSON.stringify(args ?? null));

      const result = nextResult(cmd);
      if (result) {
        return result.kind === "reject" ? Promise.reject(result.error) : result.value;
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
  await page.exposeFunction("__muxsmithRecordInvoke__", (cmd: string, argsJson: string) => {
    recorded.push({ cmd, args: JSON.parse(argsJson) as unknown });
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
