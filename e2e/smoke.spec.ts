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
import { emitEvent, installMockIPC, installTauriMocks, rejectWith, resolveWith } from "./mocks";
import { mountComponent, readModel } from "./mount";
import { en, enAttr } from "./i18n-en";
import type { FluentVariable } from "@fluent/bundle";
import {
  JOB_EVENT_CHANNEL,
  RUN_FINISHED_CHANNEL,
} from "../src/ipc";
import type {
  AppSettings,
  Diagnostic,
  JobEvent,
  LoadProfileDocument,
  MkvmergeInfo,
  ReportDocument,
  RunFinishedEvent,
  StartedRun,
} from "../src/ipc";
import {
  COLLISION_POLICIES,
  inputFields,
  locatorFields,
  matchExprFields,
  metaFields,
  outputFields,
  profileFields,
  trackRuleFields,
} from "../src/editor/registries";
import { CHAPTERS_KEYWORDS } from "../src/bindings/keywords";
import type { Profile, StructuredEdit } from "../src/bindings/profile";

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

/** Strips the U+2068 (FIRST STRONG ISOLATE) / U+2069 (POP DIRECTIONAL
 * ISOLATE) marks Fluent wraps around every placeable substitution when a
 * bundle's `useIsolating` option is on -- the GUI catalog's default (unlike
 * the CLI's `Renderer`, which explicitly turns it off for plain-text
 * stdout). Invisible to a user but present in `textContent`, so a test
 * pinning literal rendered text (not derived via `en()`) needs to strip
 * them before comparing. */
function visibleText(text: string | null): string {
  return (text ?? "").replace(/[\u2066-\u2069]/g, "");
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

  // T19 (#17 step 1): `batch-diagnostics-summary`'s three counts each got
  // their own Fluent plural selector, replacing the provisional "error(s)"
  // wording; `suggestions-capped` (rendered per-diagnostic by
  // DiagnosticsPanel) got one too, and needs its own numeric-promotion fix
  // on the frontend (`diagnosticFluentParams.ts`) since `Diagnostic.params`
  // arrives over IPC as `Record<string, string>` -- a plain string
  // argument never matches a Fluent `[one]` selector. Unlike every other
  // assertion in this file, the expected summary/diagnostic text below is
  // a LITERAL, not `en(id, args)`: the grammar (singular vs. plural) is
  // exactly what this test pins, so deriving the expectation through the
  // same catalog/mechanism under test would prove nothing. `visibleText`
  // strips the U+2068/U+2069 directional-isolate marks the GUI's
  // `FluentBundle` wraps around every placeable substitution (unlike the
  // CLI's, which turns `useIsolating` off) -- invisible to a user, but
  // present in `textContent`, and pre-existing/out of scope for T19 to
  // touch (a GUI-wide catalog-loader setting, not a plural-selector one).
  const pluralReport: ReportDocument = {
    config_diagnostics: [
      {
        code: "unsupported-source",
        severity: "error",
        config_path: "tracks[0].match",
        params: { kind: "primary" },
        rendered: "unsupported-source",
      },
      {
        code: "empty-match-expression",
        severity: "warning",
        config_path: "tracks[0].match",
        params: {},
        rendered: "empty-match-expression",
      },
      {
        code: "empty-match-expression",
        severity: "warning",
        config_path: "tracks[1].match",
        params: {},
        rendered: "empty-match-expression",
      },
      {
        code: "suggestions-capped",
        severity: "info",
        config_path: "tracks[0].match",
        params: { dropped: "1" },
        rendered: "suggestions-capped",
      },
    ],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  test("diagnostics summary and suggestions-capped pluralize their counts (1 error singular, 2 warnings plural, 1 dropped suggestion singular)", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(emptyReport)],
        dry_run: [resolveWith(pluralReport)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    await batch.getByRole("button", name("batch-profile-pick")).click();
    await batch.getByRole("button", name("batch-dry-run")).click();

    const status = batch.getByRole("status");
    await expect(status).toBeVisible();
    expect(visibleText(await status.textContent())).toBe(
      "1 error, 2 warnings, 1 info notice.",
    );

    const diagnosticsSection = batch.getByRole("region", name("batch-diagnostics-heading"));
    await expect(diagnosticsSection).toContainText("further suggestion");
    expect(visibleText(await diagnosticsSection.textContent())).toContain(
      "info: 1 further suggestion for this rule was capped at 3 and not shown.",
    );

    await assertNoSeriousA11yViolations(page);
  });

  // D101 (amendment 1, ruling A): the Run gate is the GUI consequence of
  // giving a bare `raw:` error severity, so it ships with this feature. What
  // is new here is the ASSERTION, not the behavior: `hasErrors` gating exists
  // today and the plural-counts fixture above already feeds BatchView an
  // error-severity document, but nothing anywhere asserted `batch-run`
  // disabled, so this scenario passes on the pre-D101 tree and carries no
  // red-today claim. The red half of D101 is the core/CLI exit-code flip.
  // The paired negative of the enabled assertion in the `jobs view: live run`
  // flow below -- paired by assertion, not by location. Asserting the `title`
  // (not just `disabled`) is what discriminates the errors reason from the
  // other three `runDisabledReason` branches: the document's
  // `mkvmerge_found: true` and the completed pick close the no-profile and
  // mkvmerge-missing branches by construction, and no run is active.
  const emptyRawReport: ReportDocument = {
    config_diagnostics: [
      {
        code: "empty-raw-property",
        severity: "error",
        config_path: "tracks[0].match.exact.raw:",
        params: {},
        rendered: "empty-raw-property",
      },
    ],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  test("an error-severity config diagnostic disables Run with the errors tooltip (D101's Run gate)", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(emptyRawReport)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    // Picking the profile validates it (T7 flow); no dry-run click needed.
    await batch.getByRole("button", name("batch-profile-pick")).click();
    await expect(
      batch.getByText(en("batch-profile-current", { path: PROFILE_PATH })),
    ).toBeVisible();

    const runButton = batch.getByTestId("batch-run");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toHaveAttribute("title", enAttr("batch-run", "tooltip-errors"));
  });

  // Task 14 (D43, D49, apply-wiring routing): one-click apply. Two
  // DISTINCT fixture values on purpose: `PROFILE_PATH` is the picked
  // profile FILE (what `load_profile`/`save_profile` take), and
  // `SUGGESTION_CONFIG_PATH` is a config-field LOCATOR (`tracks[<N>].
  // match`, parsed core-side by `rule_index_of`) -- what the suggestion
  // carries and what `apply_suggestion` takes. Equal values would let a
  // locator-as-path swap pass silently (exactly the bug an earlier draft
  // of this task had, caught by controller review, not by this echo
  // mock, because that draft's fixture set them equal); distinct values
  // make a swap in either direction fail an assertion below. `APPLY_EDIT`
  // is a real `StructuredEdit` (unlike the copy-only fixture above, whose
  // `edit: null` was never read pre-Task-14 and stays that way -- it is
  // not this test's concern) so the echo assertion is meaningful.
  // `core-109-two-required-no-fix`'s no-fix/partition diagnostic
  // (`suggestion-partition`) sits in the SAME report to pair the apply
  // button's presence with its absence on the identical
  // `getByRole("button", name("batch-suggestion-apply"))` selector, so a
  // typo'd selector cannot make the negative pass vacuously (house
  // paired-control template, falsifiability occurrence 5).
  const SUGGESTION_CONFIG_PATH = "tracks[0].match";
  const APPLY_EDIT: StructuredEdit = { kind: "add_exact", property: "codec_kind", value: "srt" };

  const loadedProfile: Profile = {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: { rules: [] },
  };

  const appliedProfile: Profile = {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: { rules: [{ source: "primary", match: { exact: { codec_kind: "srt" } }, changes: {} }] },
  };

  const loadedForApply: LoadProfileDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
    profile: loadedProfile,
  };

  const applyReport: ReportDocument = {
    config_diagnostics: [
      {
        code: "suggestion-partition",
        severity: "warning",
        config_path: "tracks[1].match",
        params: { kind: "overflow", dropped: "1" },
        rendered: "suggestion-partition",
      },
    ],
    batch_diagnostics: [],
    files: [],
    suggestions: [
      {
        resolves: "unknown-property",
        config_path: SUGGESTION_CONFIG_PATH,
        edit: APPLY_EDIT,
        yaml_fragment: SUGGESTION_YAML,
      },
    ],
    mkvmerge_found: true,
  };

  test("a suggestion card's apply button drives load_profile -> apply_suggestion -> save_profile, config_path/edit unmodified and never confused with the profile path; a no-fix diagnostic renders no apply button", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(emptyReport)],
        dry_run: [resolveWith(applyReport)],
        load_profile: [resolveWith(loadedForApply)],
        apply_suggestion: [resolveWith(appliedProfile)],
        save_profile: [resolveWith(null)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    await batch.getByRole("button", name("batch-profile-pick")).click();
    await batch.getByRole("button", name("batch-dry-run")).click();

    const suggestion = batch.getByRole("article");
    const applyButton = suggestion.getByRole("button", name("batch-suggestion-apply"));
    await expect(applyButton).toBeVisible();

    // Paired negative: the no-fix/partition diagnostic (rendered by
    // DiagnosticsPanel, unchanged by Task 14) carries no `Suggestion` and
    // so gets no apply control, on the identical selector the positive
    // assertion above just proved resolves.
    const diagnosticsSection = batch.getByRole("region", name("batch-diagnostics-heading"));
    await expect(diagnosticsSection).toContainText("further resolution group");
    await expect(
      diagnosticsSection.getByRole("button", name("batch-suggestion-apply")),
    ).toHaveCount(0);

    await assertNoSeriousA11yViolations(page);

    await applyButton.click();
    // The round trip (load, apply, save) completed once `applying` drops
    // aria-busy -- a robust wait for three chained mocked IPC calls
    // instead of racing the recorded-call assertions below against them.
    await expect(applyButton).not.toHaveAttribute("aria-busy", "true");

    const loadCalls = recorded.filter((r) => r.cmd === "load_profile");
    expect(loadCalls).toHaveLength(1);
    expect((loadCalls[0].args as { path: string }).path).toBe(PROFILE_PATH);

    const applyCalls = recorded.filter((r) => r.cmd === "apply_suggestion");
    expect(applyCalls).toHaveLength(1);
    const applyArgs = applyCalls[0].args as {
      profile: Profile;
      configPath: string;
      edit: StructuredEdit;
    };
    // The locator, not the file path -- the assertion this task's earlier
    // draft got backwards.
    expect(applyArgs.configPath).toBe(SUGGESTION_CONFIG_PATH);
    expect(applyArgs.configPath).not.toBe(PROFILE_PATH);
    expect(applyArgs.edit).toEqual(APPLY_EDIT);
    expect(applyArgs.profile).toEqual(loadedProfile);

    const saveCalls = recorded.filter((r) => r.cmd === "save_profile");
    expect(saveCalls).toHaveLength(1);
    const saveArgs = saveCalls[0].args as { path: string; profile: Profile };
    // The file path, not the locator.
    expect(saveArgs.path).toBe(PROFILE_PATH);
    expect(saveArgs.path).not.toBe(SUGGESTION_CONFIG_PATH);
    expect(saveArgs.profile).toEqual(appliedProfile);
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
      outcome: { state: "ok", exit_code: 0, warnings: [], errors: [], duration_ms: 1234, panic: null },
    } satisfies JobEvent);
    await emitEvent(page, JOB_EVENT_CHANNEL, {
      event: "finished",
      index: 1,
      outcome: {
        state: "cancelled",
        exit_code: null,
        warnings: [],
        errors: [],
        duration_ms: 50,
        panic: null,
      },
    } satisfies JobEvent);

    const runFinished: RunFinishedEvent = {
      config_diagnostics: [],
      batch_diagnostics: [],
      files: [],
      suggestions: [],
      mkvmerge_found: true,
      jobs: [
        {
          index: 0,
          output: JOB0_OUTPUT,
          state: "ok",
          exit_code: 0,
          warnings: [],
          errors: [],
          duration_ms: 1234,
          panic: null,
        },
        {
          index: 1,
          output: JOB1_OUTPUT,
          state: "cancelled",
          exit_code: null,
          warnings: [],
          errors: [],
          duration_ms: 50,
          panic: null,
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

// Task 21 (#17 step 3): the German catalog renders end-to-end. The locale
// is pinned via the mocked `get_settings.locale = "de-AT"` (the same
// channel `main.ts`'s locale bootstrap reads before mount) -- a
// region-qualified tag, deliberately not the bare "de" primary subtag, so
// this exercises `buildBundles`'s `primarySubtag` normalization (S15,
// T21 review) rather than an accidental exact-string match: without the
// normalization, "de-AT" would miss the `locales/de/` directory entirely
// and silently fall through to the en-only bundle, which the assertions
// below would then fail against. Assertions are LITERAL German strings,
// not `en(id)`/a `de()` helper: the point is proving the app actually
// loaded the de bundle rather than falling back to en, so each asserted
// string is one that ONLY exists in de (en renders "Batch"/"Selected
// profile:"/"Dry run"). `visibleText` strips the U+2066-2069
// directional-isolate marks the GUI bundle wraps around the `{ $path }`
// placeable -- without stripping, the exact-equality check on
// `batch-profile-current` would fail on invisible marks (same mechanism
// the T19 plural assertions document).
test.describe("german locale", () => {
  const PROFILE_PATH = "/profiles/demo.yaml";

  const DE_AT_SETTINGS: AppSettings = {
    mkvmerge_path: null,
    default_jobs: 1,
    locale: "de-AT",
    recent_profiles: [],
    dir_memory: {},
  };

  // T21.5: the bare "de" tag the settings dialog's new option actually
  // saves (`<option value="de">`), as opposed to DE_AT_SETTINGS's
  // region-qualified "de-AT" (T21's own normalization case above).
  const DE_SETTINGS: AppSettings = {
    mkvmerge_path: null,
    default_jobs: 1,
    locale: "de",
    recent_profiles: [],
    dir_memory: {},
  };

  const emptyReport: ReportDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  test("a de-AT-locale settings value normalizes to the German catalog, not the en fallback", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        get_settings: [resolveWith(DE_AT_SETTINGS)],
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(emptyReport)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    // batch-view-heading: "Batch" (en) vs "Stapel" (de).
    await expect(batch.getByRole("heading", { name: "Stapel", exact: true })).toBeVisible();
    // batch-dry-run: "Dry run" (en) vs "Probelauf" (de).
    await expect(batch.getByRole("button", { name: "Probelauf", exact: true })).toBeVisible();

    // batch-profile-current: a placeable-bearing de string. Picking a
    // profile fills "Ausgewähltes Profil: { $path }"; the exact-equality
    // check only passes with the de catalog AND the BiDi marks stripped.
    await batch.getByTestId("batch-profile-pick").click();
    const profileLine = batch.locator("p", { hasText: "Ausgewähltes Profil:" });
    await expect(profileLine).toBeVisible();
    expect(visibleText(await profileLine.textContent())).toBe(
      `Ausgewähltes Profil: ${PROFILE_PATH}`,
    );
  });

  // T21.5 (settings gate follow-up): German reachable from the settings UI
  // itself, not just via a pre-seeded settings value (the de-AT case
  // above). This case covers the RESTART path: `main.ts` resolves the locale
  // exactly once, before mount, so a saved locale change is guaranteed to
  // take effect on the next start, same as a restart. (The app ALSO swaps the
  // live `FluentBundle`s in place when settings are saved -- D56,
  // `SettingsDialog.save()` -> `applyLocale`; that live in-session path is
  // covered separately in `e2e/locale-switch.spec.ts`.) This is simulated
  // here rather than invented: `set_settings`
  // is asserted as the real, load-bearing evidence that the UI action
  // saved "de" (not a UI echo), and `page.reload()` plus a second,
  // layered `installMockIPC` registration (see that function's own doc in
  // mocks.ts) stand in for the app actually restarting with that saved
  // value. The post-reload assertion is a literal German string
  // ("Einstellungen"), not `en(id)`/a `de()` helper -- same rationale as
  // the de-AT case: proving the de bundle is genuinely active, not merely
  // that a translation function was called correctly.
  test("selecting German in the settings dialog saves it, and it renders the German catalog on the next start", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });

    await page.goto("/");

    await page.getByTestId("open-settings").click();
    const dialog = page.getByTestId("settings-dialog");
    await expect(dialog).toBeVisible();

    const localeSelect = dialog.getByRole("combobox", name("settings-locale-label"));
    await expect(localeSelect).toHaveValue("en");
    await localeSelect.selectOption("de");

    await dialog.getByRole("button", name("settings-save")).click();
    await expect(dialog).toBeHidden();

    const settingsWrites = recorded.filter((r) => r.cmd === "set_settings");
    expect(settingsWrites).toHaveLength(1);
    expect((settingsWrites[0].args as { settings: AppSettings }).settings.locale).toBe("de");

    await page.addInitScript(installMockIPC, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        get_settings: [resolveWith(DE_SETTINGS)],
      },
    });
    await page.reload();

    await page.getByTestId("open-settings").click();
    const reloadedDialog = page.getByTestId("settings-dialog");
    await expect(reloadedDialog.getByRole("heading", { name: "Einstellungen", exact: true })).toBeVisible();
    const reloadedLocaleSelect = reloadedDialog.getByRole("combobox", { name: "Sprache", exact: true });
    await expect(reloadedLocaleSelect).toHaveValue("de");
  });
});

// Task 10 (D45, wave-3 mount-harness amendment): per-widget rendering
// assertions for the ten FieldWidget variants, mounted in isolation via
// `mount.ts` -- `page.goto("/")` reaches no widget (no editor mount point
// exists in the running app before Task 13). Every `spec` fixture below is
// a REAL entry pulled from Task 9's registries (`src/editor/registries.ts`),
// not a hand-rolled FieldSpec literal, so a test failure here can never be
// "the fixture drifted from the real registry shape". Accessible names come
// from `en(id)` over the real labelKey exactly as the rest of this file
// does; SELECT/keywordOrBlock option tokens are asserted as the RAW domain
// values (`COLLISION_POLICIES`/`CHAPTERS_KEYWORDS`), never translated --
// D45's own rule that widget option tokens render as profile-format
// tokens, not via new Fluent keys. `list`/`propertyMap`'s generic
// add/remove-row buttons use the two dedicated `editor-action-add`/
// `-remove` keys ("Add"/"Remove", owner Ruling 1, amended 2026-07-16) --
// NOT `editor-attachment-rule-add`/`-drop` any more, which now caption only
// the AttachmentRule fields they are the registry labels for. Catalog
// budget is 45 (42 labels + 1 save-surface note + 2 generic action keys).
test.describe("editor widgets: mount-harness rendering", () => {
  test("the widget dispatcher renders the widget matching a field's kind", async ({ page }) => {
    await mountComponent(page, {
      component: "FieldWidgetDispatcher",
      props: { spec: inputFields.recursive, modelValue: true },
    });
    const field = page.getByRole("checkbox", name("editor-input-recursive"));
    await expect(field).toBeChecked();
    await field.uncheck();
    await expect.poll(() => readModel(page)).toBe(false);
  });

  test("text widget (single-line) renders a textbox and edits update the held model", async ({ page }) => {
    await mountComponent(page, {
      component: "TextWidget",
      props: { spec: inputFields.pattern, modelValue: "^S[0-9]+E[0-9]+" },
    });
    const field = page.getByRole("textbox", name("editor-input-pattern"));
    await expect(field).toHaveValue("^S[0-9]+E[0-9]+");
    await field.fill(".*");
    await expect.poll(() => readModel(page)).toBe(".*");
  });

  test("text widget (multiline) renders a textbox for a multiline field", async ({ page }) => {
    await mountComponent(page, {
      component: "TextWidget",
      props: { spec: metaFields.description, modelValue: "note" },
    });
    const field = page.getByRole("textbox", name("editor-meta-description"));
    await expect(field).toHaveValue("note");
    // The "textbox" role alone doesn't discriminate: a single-line
    // `<input type="text">` carries the same role as a `<textarea>`, so
    // this test would pass unchanged even if `multiline: true` rendered
    // the single-line branch. Pin the actual element too.
    expect(await field.evaluate((el) => el.tagName)).toBe("TEXTAREA");
    await field.fill("longer note");
    await expect.poll(() => readModel(page)).toBe("longer note");
  });

  test("bool widget renders a checkbox and toggling updates the held model", async ({ page }) => {
    await mountComponent(page, {
      component: "BoolWidget",
      props: { spec: inputFields.recursive, modelValue: false },
    });
    const field = page.getByRole("checkbox", name("editor-input-recursive"));
    await expect(field).not.toBeChecked();
    await field.check();
    await expect.poll(() => readModel(page)).toBe(true);
  });

  test("optionalFlag widget's off state is absence, not false (validate.rs rejects Some(false))", async ({
    page,
  }) => {
    await mountComponent(page, {
      component: "OptionalFlagWidget",
      props: { spec: locatorFields.match_to_source, modelValue: undefined },
    });
    const field = page.getByRole("checkbox", name("editor-locator-match-to-source"));
    await expect(field).not.toBeChecked();
    await field.check();
    await expect.poll(() => readModel(page)).toBe(true);
    await field.uncheck();
    await expect.poll(() => readModel(page)).toBeUndefined();
  });

  test("select widget renders a combobox of its raw (untranslated) domain tokens", async ({ page }) => {
    await mountComponent(page, {
      component: "SelectWidget",
      props: { spec: outputFields.on_collision, modelValue: "error" },
    });
    const field = page.getByRole("combobox", name("editor-output-on-collision"));
    await expect(field).toHaveValue("error");
    for (const token of COLLISION_POLICIES) {
      await expect(field.getByRole("option", { name: token, exact: true })).toBeAttached();
    }
    await field.selectOption("overwrite");
    await expect.poll(() => readModel(page)).toBe("overwrite");
  });

  test("keywordOrBlock widget offers its keyword tokens in a combobox plus a nested block section", async ({
    page,
  }) => {
    await mountComponent(page, {
      component: "KeywordOrBlockWidget",
      props: { spec: profileFields.chapters, modelValue: "keep" },
    });
    const field = page.getByRole("combobox", name("editor-profile-chapters"));
    for (const token of CHAPTERS_KEYWORDS) {
      await expect(field.getByRole("option", { name: token, exact: true })).toBeAttached();
    }
    await field.selectOption("drop");
    await expect.poll(() => readModel(page)).toBe("drop");

    // The nested block section (ExternalBlock, per profileFields.chapters'
    // `block: "externalBlock"`) is always present too -- AttachmentRule's
    // one-of precedent: no mode toggle, core diagnoses an over-set model.
    const blockGroup = page.getByRole("group", name("editor-external-block-external"));
    await expect(blockGroup).toBeVisible();
  });

  test("directoryPath widget renders a plain path textbox (text-entry only; directory picker out of scope for Plan 6, D45 widgets are prop-fed/zero-IPC)", async ({ page }) => {
    await mountComponent(page, {
      component: "DirectoryPathWidget",
      props: { spec: outputFields.directory, modelValue: "/out" },
    });
    const field = page.getByRole("textbox", name("editor-output-directory"));
    await expect(field).toHaveValue("/out");
    await field.fill("/other");
    await expect.poll(() => readModel(page)).toBe("/other");
  });

  test("stringList widget round-trips a comma-separated list of strings", async ({ page }) => {
    await mountComponent(page, {
      component: "StringListWidget",
      props: { spec: inputFields.extensions, modelValue: ["mkv", "mp4"] },
    });
    const field = page.getByRole("textbox", name("editor-input-extensions"));
    await expect(field).toHaveValue("mkv, mp4");
    await field.fill("mkv, mp4, avi");
    await expect.poll(() => readModel(page)).toEqual(["mkv", "mp4", "avi"]);
  });

  test("propertyMap widget: add/remove rows edit a key-value map", async ({ page }) => {
    await mountComponent(page, {
      component: "PropertyMapWidget",
      props: { spec: trackRuleFields.changes, modelValue: { forced: "true" } },
    });
    await expect(page.getByTestId("property-map-key").first()).toHaveValue("forced");
    await expect(page.getByTestId("property-map-value").first()).toHaveValue("true");

    await page.getByRole("button", name("editor-action-add")).click();
    await expect(page.getByTestId("property-map-key")).toHaveCount(2);
    await page.getByTestId("property-map-key").nth(1).fill("default");
    await page.getByTestId("property-map-value").nth(1).fill("true");
    await expect.poll(() => readModel(page)).toEqual({ forced: "true", default: "true" });

    await page.getByRole("button", name("editor-action-remove")).first().click();
    await expect.poll(() => readModel(page)).toEqual({ default: "true" });
  });

  test("list widget: add/remove nested items (matchExpr.any, item: matchExpr)", async ({ page }) => {
    await mountComponent(page, {
      component: "ListWidget",
      props: { spec: matchExprFields.any, modelValue: [] },
    });
    await expect.poll(() => readModel(page)).toEqual([]);

    await page.getByRole("button", name("editor-action-add")).click();
    await expect.poll(() => readModel(page)).toEqual([{}]);
    // Each item renders its own nested MatchExpr fields via SectionWidget.
    await expect(page.getByRole("group", name("editor-match-expr-exact"))).toBeVisible();

    await page.getByRole("button", name("editor-action-remove")).first().click();
    await expect.poll(() => readModel(page)).toEqual([]);
  });

  test("section widget renders a fieldset/legend group of its sub-fields, created implicitly on first edit", async ({
    page,
  }) => {
    await mountComponent(page, {
      component: "SectionWidget",
      props: { spec: profileFields.input, modelValue: undefined },
    });
    const group = page.getByRole("group", name("editor-profile-input"));
    await expect(group).toBeVisible();
    const pattern = group.getByRole("textbox", name("editor-input-pattern"));
    await pattern.fill("^S");
    await expect.poll(() => readModel(page)).toMatchObject({ pattern: "^S" });
  });
});

// Task 11 (D45, wave-3 mount-harness amendment): the profile editor's
// top-level rule grid (spec 8.2's "track-rule grid ... drag to reorder"),
// mounted in isolation via `mount.ts` exactly like Task 10's widgets --
// `EditorView` has no nav entry to reach via `page.goto("/")` until Task 13
// wires it into App.vue. `EditorView` takes a `Profile` as its `modelValue`
// and emits `update:modelValue` on reorder (the natural pre-IPC v-model
// shape); that is the only behaviour under test here -- no sections, no
// widget dispatch, no save (Tasks 12-13). The fixture's two rules differ by
// `match.exact.type`, a REAL matchable property (`capability/mod.rs`'s
// `TYPE_VALUES = ["audio", "buttons", "subtitles", "video"]`), not a
// hand-rolled marker.
test.describe("editor view: rule grid + drag-reorder (Task 11, D45)", () => {
  const twoRuleProfile: Profile = {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: {
      rules: [{ match: { exact: { type: "video" } } }, { match: { exact: { type: "audio" } } }],
    },
  };

  test("renders tracks.rules in order; a drag-reorder swaps the rows and updates the held model", async ({
    page,
  }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: twoRuleProfile } });

    const rows = page.getByTestId("editor-rule-row");
    await expect(rows).toHaveCount(2);
    await expect(rows.nth(0)).toContainText("video");
    await expect(rows.nth(1)).toContainText("audio");

    // D59: the leading ordinal column -- a first "Order" columnheader, and
    // 1-based cells rendering each row's array position.
    await expect(page.getByRole("columnheader").first()).toHaveText(en("editor-track-rule-order"));
    await expect(rows.nth(0).getByRole("cell").first()).toHaveText("1");
    await expect(rows.nth(1).getByRole("cell").first()).toHaveText("2");

    // Reorder is a semantic model edit, not a DOM mutation (binding note):
    // `EditorView`'s drag handlers never read `dataTransfer`, only a
    // closure index (mirroring `ListWidget.vue`'s identical mechanics), so
    // dispatching bare `dragstart`/`drop` suffices -- following Playwright's
    // documented programmatic-DnD pattern (a shared DataTransfer JSHandle
    // across both events) regardless, since it is the correct cross-browser
    // way to fire these two event types synthetically.
    const dataTransfer = await page.evaluateHandle(() => new DataTransfer());
    await rows.nth(0).dispatchEvent("dragstart", { dataTransfer });
    await rows.nth(1).dispatchEvent("drop", { dataTransfer });

    await expect(rows.nth(0)).toContainText("audio");
    await expect(rows.nth(1)).toContainText("video");

    // D59: the ordinal re-renders 1..n by position, so ordinal 1 now sits
    // with the row that moved into position 0 (audio) -- it tracks array
    // position, not rule identity.
    await expect(rows.nth(0).getByRole("cell").first()).toHaveText("1");
    await expect(rows.nth(1).getByRole("cell").first()).toHaveText("2");

    await expect
      .poll(async () => {
        const model = (await readModel(page)) as Profile;
        return model.tracks.rules.map((r) => (r.match.exact as Record<string, unknown> | null | undefined)?.type);
      })
      .toEqual(["audio", "video"]);
  });
});

// Task 12 (D45, amended 2026-07-16, owner-rulings routing): EditorView's
// section composition (driven by the 13 registries, not hand-listed
// fields -- adding a field to the model + registry surfaces it here with
// no view edit) plus the two typed-value-cell anti-vacuity cases Ruling 2
// requires (`gui-typed-scalar-needs-typed-input`): a settable Boolean
// round-trips a real `true`, and a matchable Boolean/Float likewise. The
// generic add/remove action captions ("Add"/"Remove", Ruling 1) are
// asserted by the repointed Task-10 propertyMap/list specs above, not
// re-asserted here.
test.describe("editor view: section composition and typed value cells (Task 12, D45)", () => {
  test("EditorView composes every profile section from the registries, dispatching each field to its widget", async ({
    page,
  }) => {
    const profile: Profile = {
      profile_version: 1,
      meta: { name: "demo", description: "a note" },
      input: { pattern: "^S", extensions: ["mkv"], recursive: true },
      output: { directory: "/out", filename: "keep", on_collision: "error" },
      tracks: { unmatched: "drop", rules: [] },
      attachments: { unmatched: "keep", rules: [] },
      chapters: "keep",
      tags: { global: "keep", track: "keep" },
      title: "keep",
    };
    await mountComponent(page, { component: "EditorView", props: { modelValue: profile } });

    // Every top-level `section`-kind field renders as a labeled group,
    // dispatched generically off `profileFields` -- `tracks` is the one
    // hand-built exception (Task 11's own rule grid), asserted below.
    for (const id of [
      "editor-profile-meta",
      "editor-profile-input",
      "editor-profile-output",
      "editor-profile-tracks",
      "editor-profile-attachments",
      "editor-profile-tags",
    ]) {
      await expect(page.getByRole("group", name(id))).toBeVisible();
    }

    // select -> combobox of its raw (untranslated) domain tokens (D45).
    const collision = page.getByRole("combobox", name("editor-output-on-collision"));
    await expect(collision).toHaveValue("error");
    for (const token of COLLISION_POLICIES) {
      await expect(collision.getByRole("option", { name: token, exact: true })).toBeAttached();
    }

    // keywordOrBlock -> combobox of its keyword tokens (D45).
    const chaptersCombo = page.getByRole("combobox", name("editor-profile-chapters"));
    await expect(chaptersCombo).toHaveValue("keep");
    for (const token of CHAPTERS_KEYWORDS) {
      await expect(chaptersCombo.getByRole("option", { name: token, exact: true })).toBeAttached();
    }

    // optionalFlag -> checkbox, reached through `chapters`' always-present
    // nested ExternalBlock -> Locator (KeywordOrBlockWidget's own
    // precedent) -- not a hand-listed lookup, just the registry recursing.
    const matchToSource = page.getByRole("checkbox", name("editor-locator-match-to-source"));
    await expect(matchToSource).not.toBeChecked();

    // `tracks`: `unmatched` dispatches generically through the registry;
    // `rules` keeps Task 11's own bespoke drag-reorder grid unchanged.
    // Scoped to the "Tracks" group: `editor-tracks-unmatched` and
    // `editor-attachments-unmatched` share the same en/de rendered text
    // ("Unmatched"), a pre-existing catalog coincidence unrelated to this
    // task, so an unscoped role+name lookup is ambiguous.
    const tracksGroup = page.getByRole("group", name("editor-profile-tracks"));
    const tracksUnmatched = tracksGroup.getByRole("combobox", name("editor-tracks-unmatched"));
    await expect(tracksUnmatched).toHaveValue("drop");
    await expect(tracksGroup.getByRole("heading", name("editor-tracks-rules"))).toBeVisible();
  });

  test("propertyMap typed value cells: the settable Boolean/String anti-vacuity round trip (Ruling 2)", async ({
    page,
  }) => {
    await mountComponent(page, {
      component: "PropertyMapWidget",
      props: { spec: trackRuleFields.changes, modelValue: { forced_track: false } },
    });
    const checkbox = page.getByRole("checkbox");
    await expect(checkbox).not.toBeChecked();
    await checkbox.check();
    const afterCheck = (await readModel(page)) as Record<string, unknown>;
    expect(afterCheck).toEqual({ forced_track: true });
    expect(afterCheck.forced_track).toBe(true); // a real boolean, not the string "true"

    await page.getByRole("button", name("editor-action-add")).click();
    await page.getByTestId("property-map-key").nth(1).fill("language");
    await page.getByTestId("property-map-value").nth(1).fill("eng");
    await expect.poll(() => readModel(page)).toEqual({ forced_track: true, language: "eng" });
  });

  test("propertyMap typed value cells: the matchable Boolean/Float/Integer anti-vacuity round trip (Ruling 2)", async ({
    page,
  }) => {
    // Boolean: matchable_type("forced_track") == Boolean.
    await mountComponent(page, {
      component: "PropertyMapWidget",
      props: { spec: matchExprFields.exact, modelValue: { forced_track: false } },
    });
    const checkbox = page.getByRole("checkbox");
    await expect(checkbox).not.toBeChecked();
    await checkbox.check();
    const boolModel = (await readModel(page)) as Record<string, unknown>;
    expect(boolModel).toEqual({ forced_track: true });
    expect(boolModel.forced_track).toBe(true);

    // Float: matchable_type("min_luminance") == Float, the one new input
    // variant (step="any").
    await mountComponent(page, {
      component: "PropertyMapWidget",
      props: { spec: matchExprFields.exact, modelValue: { min_luminance: 1 } },
    });
    const floatCell = page.getByRole("spinbutton");
    await expect(floatCell).toHaveAttribute("step", "any");
    await floatCell.fill("1.5");
    const floatModel = (await readModel(page)) as Record<string, unknown>;
    expect(floatModel).toEqual({ min_luminance: 1.5 });
    expect(floatModel.min_luminance).toBe(1.5);
    expect(typeof floatModel.min_luminance).toBe("number");

    // Integer: audio_channels rides the same <input type="number"> branch
    // minus step="any" (the exhaustive switch's remaining arm; not a
    // separately required fixture per the brief, added because it is cheap).
    await mountComponent(page, {
      component: "PropertyMapWidget",
      props: { spec: matchExprFields.exact, modelValue: { audio_channels: 2 } },
    });
    const intCell = page.getByRole("spinbutton");
    await expect(intCell).not.toHaveAttribute("step", "any");
    await intCell.fill("6");
    const intModel = (await readModel(page)) as Record<string, unknown>;
    expect(intModel).toEqual({ audio_channels: 6 });
    expect(typeof intModel.audio_channels).toBe("number");
  });
});

// Task 13 (D45, D41, D42): open/save wiring, the save-surface standing
// note, the nav entry, and validate-on-edit -- through the real, served
// app (`page.goto("/")`), unlike Tasks 10-12's mount-harness specs above:
// the editor is reachable from the nav for the first time. `load_profile`'s
// mocked response is `LoadProfileDocument`-shaped (`ReportDocument` plus
// `profile`); `validate_profile_model`'s scripted queue covers both the
// auto-revalidate right after Open (spec 7: "every profile edit" covers
// the load itself, `EditorView.vue`'s own watcher fires on the model
// assignment `pickAndOpen` makes) and the deliberate edits below --
// `mocks.ts`'s own queue semantics repeat the last entry once exhausted,
// so a single trailing "clean" entry covers every subsequent clean
// re-validation without an exact call count.
test.describe("editor view: open/save (Task 13, D45/D41)", () => {
  const PROFILE_PATH = "/profiles/editor-demo.yaml";

  // Two rules, not empty (wave item 1, whole-branch agenda-a): the served
  // app never rendered the populated grid + row selection + detail panel
  // before this fixture, the same gap the T13b mount-harness fixture below
  // does not close -- a mount-only render is not a served-app render.
  const editorProfile: Profile = {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: {
      rules: [{ match: { exact: { type: "video" } } }, { match: { exact: { type: "audio" } } }],
    },
  };

  const loadedDoc: LoadProfileDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
    profile: editorProfile,
  };

  const errorDiagnostic: Diagnostic = {
    code: "unsupported-source",
    severity: "error",
    config_path: "tracks[0].match",
    params: { kind: "primary" },
    rendered: "unsupported-source",
  };

  const errorDoc: ReportDocument = {
    config_diagnostics: [errorDiagnostic],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  const cleanDoc: ReportDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
  };

  test("the nav opens the editor; opening a profile shows the save note; Save is disabled while an error diagnostic exists and enabled when clean; saving calls save_profile", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        load_profile: [resolveWith(loadedDoc)],
        validate_profile_model: [resolveWith(cleanDoc), resolveWith(errorDoc), resolveWith(cleanDoc)],
        save_profile: [resolveWith(null)],
      },
    });

    await page.goto("/");

    // Post-close fix (owner ruling 2026-07-17, plan-6 surface pass):
    // dedicated `nav-editor` key, not the reused `batch-profile-heading`
    // Task 13 originally rendered here -- pin the tab's accessible name so
    // a regression back to the reused key fails loudly.
    const editorTab = page.getByTestId("nav-editor");
    await expect(editorTab).toHaveAccessibleName(en("nav-editor"));
    await editorTab.click();
    const editor = page.getByTestId("view-editor");
    await expect(editor).toBeVisible();

    await editor.getByRole("button", name("batch-profile-pick")).click();
    await expect(
      editor.getByText(en("batch-profile-current", { path: PROFILE_PATH })),
    ).toBeVisible();
    await expect(editor.getByText(en("editor-save-note"))).toBeVisible();

    const saveButton = editor.getByRole("button", name("settings-save"));
    // The auto-revalidate right after Open resolved clean (first queued
    // response) -- Save starts enabled.
    await expect(saveButton).toBeEnabled();

    // Wave item 1 (whole-branch agenda-a, `fixture-reachable-states-need-
    // one-served-render`): selecting a row and opening the detail panel is
    // a pure UI selection, not a model edit (`selectRule` only writes
    // `selectedIndex`), so it consumes none of the scripted
    // `validate_profile_model` queue above -- the error/clean sequence
    // below is unaffected. This is the same fixture/selection/panel the
    // T13b mount-harness proves further down, now exercised through the
    // real served app and scanned for a11y with the grid, the selection
    // button, and the panel all rendered at once -- states the mount
    // harness alone never puts in front of an actual browser page.
    await editor.getByTestId("editor-rule-select").first().click();
    const rulePanel = editor.getByTestId("editor-rule-detail");
    await expect(rulePanel).toBeVisible();
    await expect(rulePanel.getByRole("combobox", name("editor-track-rule-source"))).toBeVisible();
    await expect(rulePanel.getByRole("checkbox", name("editor-track-rule-optional"))).toBeVisible();
    await expect(rulePanel.getByRole("group", name("editor-track-rule-changes"))).toBeVisible();
    await expect(
      rulePanel.getByRole("group", name("editor-track-rule-match-expr")),
    ).toBeVisible();
    await assertNoSeriousA11yViolations(page);

    // An edit triggers the second queued (error) validate_profile_model
    // response.
    const patternField = editor.getByRole("textbox", name("editor-input-pattern"));
    await patternField.fill("^S[0-9]+E[0-9]+");
    await expect(saveButton).toBeDisabled();
    await expect(
      editor.getByText(
        en("batch-diagnostic-line", {
          severity: en("severity-error"),
          message: en("unsupported-source", { kind: "primary" }),
        }),
      ),
    ).toBeVisible();

    await assertNoSeriousA11yViolations(page);

    // A further edit triggers the third queued (clean) response, clearing
    // the error.
    await patternField.fill(".*");
    await expect(saveButton).toBeEnabled();

    await saveButton.click();
    const saveCalls = recorded.filter((r) => r.cmd === "save_profile");
    expect(saveCalls).toHaveLength(1);
    const saveArgs = saveCalls[0].args as { path: string; profile: Profile };
    expect(saveArgs.path).toBe(PROFILE_PATH);
    expect(saveArgs.profile.input.pattern).toBe(".*");
  });

  test("the editor tab stays mounted across a switch to Jobs and back (v-show, not v-if)", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        load_profile: [resolveWith(loadedDoc)],
        validate_profile_model: [resolveWith(cleanDoc)],
      },
    });

    await page.goto("/");

    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await editor.getByRole("button", name("batch-profile-pick")).click();
    await expect(
      editor.getByText(en("batch-profile-current", { path: PROFILE_PATH })),
    ).toBeVisible();

    const patternField = editor.getByRole("textbox", name("editor-input-pattern"));
    await patternField.fill("kept-through-switch");
    await expect(patternField).toHaveValue("kept-through-switch");

    await page.getByTestId("nav-jobs").click();
    await expect(page.getByTestId("view-jobs")).toBeVisible();
    await expect(editor).toBeHidden();

    await page.getByTestId("nav-editor").click();
    await expect(editor).toBeVisible();
    // Still mounted, not recreated: the field value and the open path
    // survived the round trip through Jobs.
    await expect(patternField).toHaveValue("kept-through-switch");
    await expect(
      editor.getByText(en("batch-profile-current", { path: PROFILE_PATH })),
    ).toBeVisible();
  });
});

// Task 13b (D45, spec 8.2, amended 2026-07-16, detail-editor routing): the
// per-rule detail panel beneath Task 11's read-only summary grid, closing
// the confirmed plan-coverage gap (`registry-slot-capability-delta`,
// `docs/decision-ledger.yaml`) between spec 8.2's "detail editor per rule"
// promise and the read-only grid Task 11 built for it. Selection is a
// native `<button data-testid="editor-rule-select">` with `:aria-current`
// (the `RunHistory.vue:168-173` house precedent, not a hand-rolled
// interactive `<tr>`); the panel renders the selected rule through
// `SectionWidget` over the `trackRule` registry -- byte-for-byte the
// machinery `ListWidget` already uses for AttachmentRule items
// (`attachments.rules`), so track-rule editing becomes the same code path,
// adding zero new catalog keys and zero new components. Mounted through
// the Task-10 harness (`mount.ts`), not the served app, per Tasks 11-13's
// own precedent above.
test.describe("editor view: rule detail editor (Task 13b, D45 / spec 8.2)", () => {
  const twoRuleProfile: Profile = {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: {
      rules: [{ match: { exact: { type: "video" } } }, { match: { exact: { type: "audio" } } }],
    },
  };

  test("no selection renders no panel; selecting a row opens it with the four trackRule fields; editing optional writes a real boolean and the grid summary follows (anti-vacuity)", async ({
    page,
  }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: twoRuleProfile } });

    // 1. No selection, no panel. Made non-vacuous by assertion 2 below,
    // which asserts the panel DOES appear on selection: the RED run
    // exercises that presence branch, so this pair cannot both pass for a
    // never-renders reason.
    await expect(page.getByTestId("editor-rule-detail")).toHaveCount(0);

    // 2. Select opens the panel with the four trackRule fields, dispatched
    // through the real registry -- never a hand-typed literal.
    await page.getByTestId("editor-rule-select").first().click();
    const panel = page.getByTestId("editor-rule-detail");
    await expect(panel).toBeVisible();
    await expect(panel.getByRole("combobox", name("editor-track-rule-source"))).toBeVisible();
    await expect(panel.getByRole("checkbox", name("editor-track-rule-optional"))).toBeVisible();
    await expect(panel.getByRole("group", name("editor-track-rule-changes"))).toBeVisible();
    await expect(panel.getByRole("group", name("editor-track-rule-match-expr"))).toBeVisible();

    // 3. Edit `optional` (row 0 starts unset -- a real state change, not a
    // vacuous re-assert): the model AND the grid's own summary checkbox
    // both update from the same write, proving the panel and the grid
    // share the one model (`setRuleValue`'s immutable rebuild feeding
    // back into the same `rules` the grid renders).
    const optionalCheckbox = panel.getByRole("checkbox", name("editor-track-rule-optional"));
    await expect(optionalCheckbox).not.toBeChecked();
    await optionalCheckbox.check();
    const model = (await readModel(page)) as Profile;
    expect(model.tracks.rules[0].optional).toBe(true); // a real boolean, not the string "true"
    await expect(page.getByTestId("editor-rule-row").first().getByRole("checkbox")).toBeChecked();
  });
});

// Task 13c (spec 8.2, amended 2026-07-16, recents routing): closes
// whole-branch Finding 1 -- the editor as-built had only a pick button and
// never fed or rendered the shared `AppSettings.recent_profiles` MRU
// memory BatchView maintains. The fix extracts BatchView's
// `rememberRecentProfile` round trip into `src/recentProfiles.ts` (a
// behavior-identical refactor there) and routes the editor's pick button
// and its new recents list through one `openPath` funnel, remembering the
// opened profile the same never-clobber way. Served-app tests (nav to the
// editor), not the mount harness, matching Task 13's own precedent above.
// Two distinct fixture paths (`echo-mock-distinct-fixture-values`) so an
// identity assertion cannot pass on a shared value: RECENT_PATH is
// pre-seeded in the mocked `recent_profiles`, OPENED_PATH is what the
// dialog returns.
test.describe("editor view: recent profiles (Task 13c, spec 8.2 / recents routing)", () => {
  const RECENT_PATH = "/profiles/seeded-recent.yaml";
  const OPENED_PATH = "/profiles/freshly-opened.yaml";

  const editorProfile: Profile = {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: { rules: [] },
  };

  const loadedDoc: LoadProfileDocument = {
    config_diagnostics: [],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    mkvmerge_found: true,
    profile: editorProfile,
  };

  function settingsWith(recentProfiles: string[]): AppSettings {
    return {
      mkvmerge_path: null,
      default_jobs: 1,
      locale: "en",
      recent_profiles: recentProfiles,
      dir_memory: {},
    };
  }

  test("the pre-Open surface renders a seeded recent profile", async ({ page }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        get_settings: [resolveWith(settingsWith([RECENT_PATH]))],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await expect(editor).toBeVisible();

    const recentButtons = editor.getByTestId("editor-recent-profile");
    await expect(recentButtons).toHaveCount(1);
    await expect(recentButtons.first()).toContainText(RECENT_PATH);
  });

  // Paired absence control (same selector, non-vacuous per assertion 1
  // above): an empty memory renders no recent-profile buttons at all, so
  // the "renders a seeded recent profile" test above cannot pass for a
  // reason unrelated to the seeded fixture (e.g. a hard-coded literal).
  test("the paired absence control: an empty recents memory renders no recent-profile buttons", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        get_settings: [resolveWith(settingsWith([]))],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await expect(editor).toBeVisible();
    await expect(editor.getByTestId("editor-recent-profile")).toHaveCount(0);
  });

  test("clicking a recent opens through the same load_profile funnel as pick", async ({ page }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        get_settings: [resolveWith(settingsWith([RECENT_PATH]))],
        load_profile: [resolveWith(loadedDoc)],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await editor.getByTestId("editor-recent-profile").first().click();

    await expect(
      editor.getByText(en("batch-profile-current", { path: RECENT_PATH })),
    ).toBeVisible();

    const loadCalls = recorded.filter((r) => r.cmd === "load_profile");
    expect(loadCalls).toHaveLength(1);
    expect((loadCalls[0].args as { path: string }).path).toBe(RECENT_PATH);
  });

  test("opening a profile writes it to the front of the shared recents memory (echo, distinct values)", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        get_settings: [resolveWith(settingsWith([RECENT_PATH]))],
        "plugin:dialog|open": [resolveWith(OPENED_PATH)],
        load_profile: [resolveWith(loadedDoc)],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");

    await editor.getByRole("button", name("batch-profile-pick")).click();
    // Wait for the open to settle: `opening.value` only flips back to
    // false in `openPath`'s `finally`, which runs after the awaited
    // `rememberRecentProfile` round trip -- so the Open button re-enabling
    // is the real "the recents write already happened" signal, unlike the
    // `batch-profile-current` text (set earlier in the same try block).
    await expect(editor.getByTestId("editor-open")).toBeEnabled();
    await expect(
      editor.getByText(en("batch-profile-current", { path: OPENED_PATH })),
    ).toBeVisible();

    const writes = recorded.filter((r) => r.cmd === "set_settings");
    expect(writes).toHaveLength(1);
    const written = (writes[0].args as { settings: AppSettings }).settings;
    expect(written.recent_profiles[0]).toBe(OPENED_PATH);
    expect(written.recent_profiles).toContain(RECENT_PATH);
  });
});
