/**
 * D104 (the test half of the ruled D23 item): `JobsView`'s
 * reset-before-invoke, gated on its own `runActive` flag (the
 * `startingFresh` check), driven through the component mount harness.
 * Three of the five orderings the plan-5 whole-branch round-2 verdict
 * traced, plus D100's panic render:
 *
 *   1. a fresh dispatch whose soft outcome emits `run-finished` BEFORE
 *      `start_run` resolves -- the ordering that verdict proved the literal
 *      "reset after resolve" reading breaks;
 *   2. a fresh dispatch that rejects, clearing `runActive` again;
 *   3. a second dispatch against an ACTIVE run -- the divergent branch,
 *      unreachable from the UI (the same flag disables the Run button),
 *      which is the whole reason the ruled item exists;
 *   4. a `finished` job-event whose outcome carries a panic payload.
 *
 * Orderings 1 (a fresh real run) and 5 (interleaved rapid dispatches) are
 * deliberately NOT duplicated here: the first is already exercised
 * end-to-end by the smoke's `jobs view: live run` describe, and the second
 * is a same-continuation-segment scheduling property whose observable core
 * -- the second dispatch sees `runActive === true` -- is exactly what test
 * 3 asserts; a timing-race harness for it would be nondeterministic
 * decoration.
 *
 * Composition, spec-local by design: `mount.ts` is deliberately not
 * touched (its no-IPC-mock stance belongs to the editor-widget specs) and
 * `mocks.ts`'s `installTauriMocks` drives the served app through
 * `page.goto` rather than a mount. This file therefore assembles the two
 * existing pieces itself -- blank page, the Tauri mock bundle, this test's
 * IPC handler, the mount bundle, mount -- reusing `installMockIPC`, which
 * `mocks.ts` already exports for exactly this kind of second, layered
 * registration and which is self-contained by its own documented
 * constraint, so running it through `evaluate` on the prepared page is the
 * same in-page execution `addInitScript` would perform on navigation.
 *
 * `runActive` is deliberately not passed as a prop: an absent prop makes
 * `defineModel` fall back to local-ref semantics, which is the view's real
 * standalone behavior, and it keeps the internal transitions assertable
 * through the cancel-batch button's own disabled state -- for tests 1 and
 * 3 -- instead of through a value the test itself supplied. Test 2's
 * transition is asserted at the same gating condition from its other side;
 * D104's amendment-5 rider is the single home for that vehicle and its
 * reason.
 */
import { resolve } from "node:path";
import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";
import { emitEvent, installMockIPC, rejectWith, resolveWith } from "./mocks";
import type { MockScenario } from "./mocks";
import { en } from "./i18n-en";
import { JOB_EVENT_CHANNEL, RUN_FINISHED_CHANNEL } from "../src/ipc";
import type { JobEvent, RunFinishedEvent, RunRequest, StartedRun } from "../src/ipc";

const MOCK_HARNESS_PATH = resolve(import.meta.dirname, ".generated/tauri-mock-harness.js");
const MOUNT_HARNESS_PATH = resolve(import.meta.dirname, ".generated/mount-harness.js");

const RUN_ID = "20260728-120000Z";
const JOB0_OUTPUT = "output/episode1.mkv";

/** The two dispatches of test 3 must be DIFFERENT objects for the view's
 *  `watch(() => props.pendingRun)` to fire a second time; they differ in a
 *  real field rather than by identity alone. */
const FIRST_RUN: RunRequest = {
  profile: "/profiles/run-demo.yaml",
  source: null,
  output: null,
  jobs: null,
};
const SECOND_RUN: RunRequest = {
  profile: "/profiles/run-demo.yaml",
  source: "/media/second",
  output: null,
  jobs: null,
};

/** The zero-jobs soft outcome (a profile load failure, a missing mkvmerge,
 *  nothing planned): the run finished before it ever started. */
const SOFT_OUTCOME: RunFinishedEvent = {
  config_diagnostics: [],
  batch_diagnostics: [],
  files: [],
  suggestions: [],
  jobs: [],
  summary: { ok: 0, warning: 0, failed: 0, cancelled: 0 },
  joblog_status: "unavailable",
};

/**
 * Runs INSIDE the page (`page.evaluate`), under the same constraint
 * `mocks.ts`'s `installMockIPC` documents: no closure over this module's
 * scope beyond its own argument. Reproduces the Rust command's documented
 * emit-before-resolve ordering for a soft outcome -- `start_run` emits
 * `muxsmith://run-finished` synchronously through the real bundled
 * `emit()` and only then returns its `StartedRun`, which is precisely the
 * ordering a post-resolve reset would clobber. A scripted `MockResult` is
 * a static value and cannot express it, which is why this case installs
 * its own handler instead of going through a `MockScenario`.
 *
 * Relative to `installMockIPC` it deliberately answers a narrower surface:
 * no `__TAURI_OS_PLUGIN_INTERNALS__.platform` global, no forwarding to the
 * Node-side `__muxsmithRecordInvoke__` log, and no `get_settings` /
 * `set_settings` / `plugin:fs|write_text_file` answers. That is safe today
 * because no mount in this spec reaches any of them: it mounts `JobsView`
 * alone on a blank page instead of driving the served app, `platform()` is
 * `FirstRun.vue`'s, the settings pair is read across the app but by no
 * component this spec mounts (`main.ts`'s locale bootstrap,
 * `SettingsDialog.vue`, `BatchView.vue`, `EditorView.vue`, `FirstRun.vue`,
 * `recentProfiles.ts`), the file write sits behind `RunHistory.vue`'s
 * user-triggered log export, and every test in this file asserts DOM state
 * rather than a recorded call log -- with the unmocked-command throw below
 * as the backstop the day one of those stops holding.
 */
function installSoftOutcomeIPC(arg: {
  channel: string;
  started: StartedRun;
  finished: RunFinishedEvent;
}): void {
  window.__muxsmithE2E__.mockWindows("main");
  window.__muxsmithE2E__.mockIPC(
    (cmd: string) => {
      if (cmd === "start_run") {
        void window.__muxsmithE2E__.emit(arg.channel, arg.finished);
        return arg.started;
      }
      // `RunHistory.vue`'s own `onMounted(refresh)`, which every JobsView
      // mount fires regardless of what the test is actually about.
      if (cmd === "list_runs") {
        return [];
      }
      throw new Error(`jobsview-reset mock: unmocked command "${cmd}"`);
    },
    { shouldMockEvents: true },
  );
}

/** Blank page plus the Tauri mock bundle: everything the page needs before
 *  an IPC handler can be installed into it. */
async function preparePage(page: Page): Promise<void> {
  await page.setContent('<!doctype html><div id="mount"></div>');
  await page.addScriptTag({ path: MOCK_HARNESS_PATH });
}

/** The mount bundle plus the mount call itself. Called AFTER the IPC
 *  handler is installed: `JobsView`'s `pendingRun` watcher is `immediate`,
 *  so mounting is already the first dispatch. */
async function mountJobsView(page: Page, pendingRun: RunRequest): Promise<void> {
  await page.addScriptTag({ path: MOUNT_HARNESS_PATH });
  await page.evaluate(
    (req) => window.__muxsmithMount__({ component: "JobsView", props: { pendingRun: req } }),
    pendingRun,
  );
}

test.describe("jobs view: dispatch reset and run-active gating (D23/D104)", () => {
  test("fresh dispatch with a soft outcome keeps the finished summary", async ({ page }) => {
    await preparePage(page);
    await page.evaluate(installSoftOutcomeIPC, {
      channel: RUN_FINISHED_CHANNEL,
      started: { run_id: RUN_ID, total_jobs: 0, run_dir: null },
      finished: SOFT_OUTCOME,
    });
    await mountJobsView(page, FIRST_RUN);

    const jobs = page.getByTestId("view-jobs");
    const summary = jobs.getByTestId("jobs-run-summary");
    const summaryLine = en("jobs-summary-line", { ok: 0, warning: 0, failed: 0, cancelled: 0 });
    await expect(summary).toContainText(summaryLine);

    // "After the promise resolves", stated as a barrier rather than
    // assumed: the dispatch's own post-await continuation and Vue's flush
    // both run in microtasks queued while the mocked `invoke` returned, so
    // one round trip through the page is strictly later than both. The
    // re-assertion therefore reads the settled DOM, not a value the first
    // assertion could have caught in flight.
    await page.evaluate(() => document.readyState);
    await expect(summary).toContainText(summaryLine);
    await expect(summary).toContainText(en("jobs-joblog-unavailable"));

    // `runActive` false: `onRunFinished` cleared it and nothing after the
    // resolve set it again.
    await expect(jobs.getByTestId("cancel-batch")).toBeDisabled();
  });

  test("fresh dispatch rejection renders the error and clears runActive", async ({ page }) => {
    await preparePage(page);
    await page.evaluate(installMockIPC, {
      commands: { start_run: [rejectWith("run-already-active")] },
    } satisfies MockScenario);
    await mountJobsView(page, FIRST_RUN);

    const jobs = page.getByTestId("view-jobs");
    await expect(jobs.getByRole("alert")).toHaveText(en("run-already-active"));
    await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
    await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
  });

  test("double dispatch against an active run does not wipe the live row", async ({ page }) => {
    await preparePage(page);
    await page.evaluate(installMockIPC, {
      commands: {
        start_run: [
          resolveWith({ run_id: RUN_ID, total_jobs: 2, run_dir: `/runs/${RUN_ID}` } satisfies StartedRun),
          rejectWith("run-already-active"),
        ],
      },
    } satisfies MockScenario);
    await mountJobsView(page, FIRST_RUN);

    const jobs = page.getByTestId("view-jobs");
    const rows = jobs.getByTestId("job-row");
    // Reaching two rows means `start_run` resolved, which the view awaits
    // its `listen()` registrations before invoking -- so the emit below
    // cannot race the listeners.
    await expect(rows).toHaveCount(2);
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "started",
      index: 0,
      output: JOB0_OUTPUT,
    } satisfies JobEvent);
    const row0 = rows.nth(0);
    await expect(row0).toContainText(JOB0_OUTPUT);

    await page.evaluate(
      (req) => window.__muxsmithSetProps__({ pendingRun: req }),
      SECOND_RUN,
    );

    // The rejection landing is what proves the second dispatch ran at all.
    await expect(jobs.getByRole("alert")).toHaveText(en("run-already-active"));
    await expect(rows).toHaveCount(2);
    await expect(row0).toContainText(JOB0_OUTPUT);
    // `runActive` stayed true: the catch arm clears it only for a fresh
    // start, so the first run remains cancellable.
    await expect(jobs.getByTestId("cancel-batch")).toBeEnabled();
  });

  test("a finished event with a panic renders the worker-panicked message", async ({ page }) => {
    await preparePage(page);
    await page.evaluate(installMockIPC, {
      commands: {
        start_run: [
          resolveWith({ run_id: RUN_ID, total_jobs: 1, run_dir: `/runs/${RUN_ID}` } satisfies StartedRun),
        ],
      },
    } satisfies MockScenario);
    await mountJobsView(page, FIRST_RUN);

    const jobs = page.getByTestId("view-jobs");
    const rows = jobs.getByTestId("job-row");
    await expect(rows).toHaveCount(1);

    // The shape `recover_panicked_worker` actually emits (D98): failed, no
    // exit code, no measurable duration, the `worker-panicked` token in
    // `errors`, and the downcast payload in the typed `panic` field.
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "finished",
      index: 0,
      outcome: {
        state: "failed",
        exit_code: null,
        warnings: [],
        errors: ["worker-panicked: job 0"],
        duration_ms: 0,
        panic: "boom",
      },
    } satisfies JobEvent);

    await expect(rows.nth(0).getByTestId("job-panic")).toHaveText(
      en("worker-panicked", { detail: "boom" }),
    );
  });
});
