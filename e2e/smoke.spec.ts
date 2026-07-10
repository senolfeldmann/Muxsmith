/**
 * Task 12 GUI smoke: the built frontend in a plain Chromium browser,
 * mocked IPC (`mocks.ts`), locale pinned to "en" (playwright.config.ts's
 * `use.locale` plus the mocked `get_settings.locale`). Three scenarios per
 * the task brief:
 *   (a) mkvmerge detection fails -> first-run guidance -> manual path
 *       recovers it.
 *   (b) a mocked `dry_run` document renders the resolution table and
 *       diagnostics, and copying a suggestion hits the clipboard mock.
 *   (c) a mocked `start_run` plus scripted `job-event`s progress the job
 *       rows and fill the live log; per-row cancel invokes `cancel_job`;
 *       `run-finished` announces the summary.
 * Every user-facing string asserted against comes from `e2e/i18n-en.ts`
 * (the real en Fluent catalog), never a hand-duplicated literal (Task 12's
 * binding test-discipline convention). Locators prefer `getByRole`;
 * `data-testid` is the fallback where no distinct accessible role/name
 * exists.
 */
import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import type { Page } from "@playwright/test";
import { emitEvent, installTauriMocks, rejectWith, resolveWith } from "./mocks";
import { en } from "./i18n-en";
import type { FluentVariable } from "@fluent/bundle";
import {
  JOB_EVENT_CHANNEL,
  RUN_FINISHED_CHANNEL,
} from "../src/ipc";
import type {
  AppSettings,
  JobEvent,
  MkvmergeInfo,
  ReportDocument,
  RunFinishedEvent,
  StartedRun,
} from "../src/ipc";

/** `getByRole(role, name(id))` -- `exact: true` throughout: Playwright's
 * default role-name matching is a case-insensitive SUBSTRING match, which
 * collides here for real (not hypothetically): "Run" (the batch-run
 * button) is a substring of "Dry run" (batch-dry-run) and of any fixture
 * path containing "run" (e.g. "run-demo.yaml" in the recent-profiles
 * list), so an un-exact match resolves to 2-3 elements and Playwright's
 * strict mode rejects the locator. */
function name(id: string, args?: Record<string, FluentVariable>): { name: string; exact: true } {
  return { name: en(id, args), exact: true };
}

/** Fails the test with every axe violation of impact "serious" or
 * "critical" listed (id, impact, help text, offending selectors) -- Task
 * 12 step 2: one scan per view state the smoke actually reaches. */
async function assertNoSeriousA11yViolations(page: Page): Promise<void> {
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter(
    (v) => v.impact === "serious" || v.impact === "critical",
  );
  const report = serious
    .map(
      (v) =>
        `${v.id} (${v.impact}): ${v.help}\n  ${v.nodes.map((n) => n.target.join(" ")).join("\n  ")}`,
    )
    .join("\n\n");
  expect(serious, report).toEqual([]);
}

const MKVMERGE_INFO: MkvmergeInfo = {
  path: "/usr/bin/mkvmerge",
  version: "90.0.0",
  meets_minimum: true,
};

test.describe("first-run gate", () => {
  test("detect failure shows guidance; a manual path recovers it", async ({ page }) => {
    const MANUAL_PATH = "/opt/mkvtoolnix/mkvmerge";

    const recorded = await installTauriMocks(page, {
      platform: "linux",
      commands: {
        detect_mkvmerge: [rejectWith("mkvmerge-not-found"), resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(MANUAL_PATH)],
      },
    });

    await page.goto("/");

    const firstRun = page.getByRole("main");
    await expect(
      firstRun.getByRole("heading", name("firstrun-missing-heading")),
    ).toBeVisible();
    await expect(page.getByText(en("mkvmerge-not-found"))).toBeVisible();
    await expect(page.getByText(en("firstrun-guidance-linux"))).toBeVisible();

    await assertNoSeriousA11yViolations(page);

    await firstRun.getByRole("button", name("browse-button")).click();
    const pathInput = firstRun.getByRole("textbox", name("firstrun-picker-label"));
    await expect(pathInput).toHaveValue(MANUAL_PATH);

    await firstRun.getByRole("button", name("firstrun-use-path")).click();

    await expect(page.getByRole("heading", name("app-title"))).toBeVisible();
    await expect(page.getByRole("navigation", name("nav-label"))).toBeVisible();

    // Real invocation evidence, not a UI echo: the retry actually happened
    // (two detect_mkvmerge calls) and the manual path was actually
    // persisted via set_settings before the retry.
    expect(recorded.filter((r) => r.cmd === "detect_mkvmerge")).toHaveLength(2);
    const settingsWrite = recorded.find((r) => r.cmd === "set_settings");
    expect((settingsWrite?.args as { settings: AppSettings } | undefined)?.settings.mkvmerge_path).toBe(
      MANUAL_PATH,
    );
  });
});

test.describe("batch view: dry run", () => {
  const PROFILE_PATH = "/profiles/demo.yaml";
  const SOURCE_FILE = "movie.mkv";
  const SOURCE_IDENTIFIER = "movie";
  const OUTPUT_FILE = "output/movie.mkv";
  const SUGGESTION_YAML = "track_rules:\n  - match:\n      type: audio\n";

  const emptyReport: ReportDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  const dryRunReport: ReportDocument = {
    config_diagnostics: [],
    batch_diagnostics: [
      {
        code: "ignored-file",
        severity: "info",
        config_path: PROFILE_PATH,
        params: {},
        rendered: "ignored-file",
      },
    ],
    files: [
      {
        source: SOURCE_FILE,
        identifier: SOURCE_IDENTIFIER,
        plan: {
          output: OUTPUT_FILE,
          assignments: [
            { rule_index: 0, track_id: 0, track_kind: "video" },
            { rule_index: 1, track_id: 1, track_kind: "audio" },
          ],
        },
        diagnostics: [
          {
            code: "unknown-property",
            severity: "warning",
            config_path: PROFILE_PATH,
            params: { property: "codec_kind_extra" },
            rendered: "unknown-property",
          },
        ],
      },
    ],
    suggestions: [
      {
        resolves: "unknown-property",
        config_path: PROFILE_PATH,
        edit: null,
        yaml_fragment: SUGGESTION_YAML,
      },
    ],
    mkvmerge_found: true,
  };

  test("dry-run document renders the resolution table + diagnostics; suggestion copy hits the clipboard mock", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(emptyReport)],
        dry_run: [resolveWith(dryRunReport)],
        "plugin:clipboard-manager|write_text": [resolveWith(null)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    await expect(batch.getByRole("heading", name("batch-view-heading"))).toBeVisible();

    await batch.getByRole("button", name("batch-profile-pick")).click();
    await expect(
      batch.getByText(en("batch-profile-current", { path: PROFILE_PATH })),
    ).toBeVisible();

    await batch.getByRole("button", name("batch-dry-run")).click();

    const table = batch.getByRole(
      "table",
      name("batch-file-caption", {
        source: SOURCE_FILE,
        identifier: SOURCE_IDENTIFIER,
        output: OUTPUT_FILE,
      }),
    );
    await expect(table).toBeVisible();
    await expect(table.getByRole("row")).toHaveCount(3); // header + 2 assignments

    const expectedDiagnosticLine = en("batch-diagnostic-line", {
      severity: en("severity-warning"),
      message: en("unknown-property", { property: "codec_kind_extra" }),
    });
    await expect(batch.getByText(expectedDiagnosticLine)).toBeVisible();

    const suggestion = batch.getByRole("article");
    await expect(suggestion.locator("pre code")).toContainText("track_rules");

    await assertNoSeriousA11yViolations(page);

    await suggestion.getByRole("button", name("batch-suggestion-copy")).click();
    await expect(suggestion.getByText(en("batch-suggestion-copied"))).toBeVisible();

    const clipboardCalls = recorded.filter((r) => r.cmd === "plugin:clipboard-manager|write_text");
    expect(clipboardCalls).toHaveLength(1);
    expect((clipboardCalls[0].args as { text: string }).text).toBe(SUGGESTION_YAML);
  });
});

test.describe("jobs view: live run", () => {
  const PROFILE_PATH = "/profiles/run-demo.yaml";
  const JOB0_OUTPUT = "output/episode1.mkv";
  const JOB1_OUTPUT = "output/episode2.mkv";
  const RUN_ID = "20260710-190000Z";

  const runnableReport: ReportDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  test("scripted job-events progress rows and fill the log; per-row cancel invokes cancel_job; run-finished announces the summary", async ({
    page,
  }) => {
    const startedRun: StartedRun = { run_id: RUN_ID, total_jobs: 2, run_dir: `/runs/${RUN_ID}` };

    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(runnableReport)],
        start_run: [resolveWith(startedRun)],
        cancel_job: [resolveWith(null)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    await batch.getByRole("button", name("batch-profile-pick")).click();
    const runButton = batch.getByRole("button", name("batch-run"));
    await expect(runButton).toBeEnabled();
    await runButton.click();

    const jobs = page.getByTestId("view-jobs");
    await expect(jobs.getByTestId("job-row")).toHaveCount(2);

    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "started",
      index: 0,
      output: JOB0_OUTPUT,
    } satisfies JobEvent);
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "progress",
      index: 0,
      percent: 42,
    } satisfies JobEvent);
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "output",
      index: 0,
      line: "mkvmerge output line 1",
    } satisfies JobEvent);
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "started",
      index: 1,
      output: JOB1_OUTPUT,
    } satisfies JobEvent);

    const row0 = jobs.getByTestId("job-row").nth(0);
    const row1 = jobs.getByTestId("job-row").nth(1);

    await expect(row0.getByTestId("job-progress")).toHaveJSProperty("value", 42);
    await expect(jobs.getByTestId("live-log")).toContainText("mkvmerge output line 1");

    await assertNoSeriousA11yViolations(page);

    await row1.getByRole("button", name("jobs-row-cancel-label")).click();
    expect(
      recorded.some((r) => r.cmd === "cancel_job" && (r.args as { index: number }).index === 1),
    ).toBe(true);

    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "finished",
      index: 0,
      outcome: { state: "ok", exit_code: 0, warnings: [], errors: [], duration_ms: 1234 },
    } satisfies JobEvent);
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "finished",
      index: 1,
      outcome: { state: "cancelled", exit_code: null, warnings: [], errors: [], duration_ms: 50 },
    } satisfies JobEvent);

    const runFinished: RunFinishedEvent = {
      config_diagnostics: [],
      batch_diagnostics: [],
      files: [],
      suggestions: [],
      mkvmerge_found: true,
      jobs: [
        { index: 0, output: JOB0_OUTPUT, state: "ok", exit_code: 0, warnings: [], errors: [], duration_ms: 1234 },
        {
          index: 1,
          output: JOB1_OUTPUT,
          state: "cancelled",
          exit_code: null,
          warnings: [],
          errors: [],
          duration_ms: 50,
        },
      ],
      summary: { ok: 1, warning: 0, failed: 0, cancelled: 1 },
      joblog_status: "complete",
    };
    await emitEvent(page, RUN_FINISHED_CHANNEL, runFinished);

    await expect(jobs.getByTestId("jobs-run-summary")).toHaveText(
      en("jobs-summary-line", { ok: 1, warning: 0, failed: 0, cancelled: 1 }),
    );
    await expect(row0.getByText(en("jobs-state-ok"))).toBeVisible();
    await expect(row1.getByText(en("jobs-state-cancelled"))).toBeVisible();
  });
});
