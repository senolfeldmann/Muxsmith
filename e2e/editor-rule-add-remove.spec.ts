/**
 * Plan 7.5 (D65-D70, D72): the Add/Remove affordance on the bespoke track-rule
 * grid. Two harnesses in one file, matching the two questions the design asks:
 *
 * - Cases 1-5 use the component mount harness (`mount.ts`, the Task 11/13b
 *   sibling pattern): the model is injected as a prop and read back via
 *   `readModel`, and `EditorView`'s `watch(model)` revalidation is gated on
 *   `currentPath`, which a bare mount never sets -- so these cases exercise
 *   the grid, the selection and the two mutations with no IPC mock at all.
 * - Cases 6-8 drive the real served app with `installTauriMocks` (the
 *   `editor-markers.spec.ts` pattern): they pin the WIRE truth (what
 *   `validate_profile_model` is actually invoked with) and where core's
 *   verified diagnostics for the skeleton and the zero-rule state land in the
 *   editor (marker anchor + severity class, the diagnostics panel, and the
 *   Save gate's severity behaviour).
 *
 * Mock-queue mechanics (`mocks.ts`): entries are consumed per call and the
 * last one repeats, so each mocked case queues two `validate_profile_model`
 * responses -- the open's own model assignment fires the first, the Add or
 * Remove model swap fires the second.
 *
 * Diagnostic text is asserted through the en catalog binding (`en(code)`),
 * never through a fixture's `rendered` field: that field is wire ballast no
 * frontend surface displays (`e2e-diagnostic-rendered-is-wire-ballast`,
 * `docs/decision-ledger.yaml`; `src/ipc.ts` documents the non-display and
 * `DiagnosticsPanel.vue` renders Fluent over `code`/`params`), present in
 * the fixtures below only because the `Diagnostic` type requires it.
 */
import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";
import { mountComponent, readModel } from "./mount";
import { installTauriMocks, resolveWith } from "./mocks";
import type { RecordedInvoke } from "./mocks";
import { en, name } from "./i18n-en";
import type { Diagnostic, LoadProfileDocument, MkvmergeInfo, ReportDocument } from "../src/ipc";
import type { Profile } from "../src/bindings/profile";

const MKVMERGE_INFO: MkvmergeInfo = { path: "/usr/bin/mkvmerge", version: "90.0.0" };

// The two rules differ by `match.exact.type`, a real matchable property with
// real values -- the Task 11/13b fixture shape, not hand-rolled markers.
const twoRuleProfile: Profile = {
  profile_version: 1,
  input: { pattern: ".*", extensions: ["mkv"] },
  tracks: {
    rules: [{ match: { exact: { type: "video" } } }, { match: { exact: { type: "audio" } } }],
  },
};

const oneRuleProfile: Profile = {
  profile_version: 1,
  input: { pattern: ".*", extensions: ["mkv"] },
  tracks: { rules: [{ match: { exact: { type: "video" } } }] },
};

/** The clean first validate response every mocked case below shares: the
 *  open's own revalidation resolves with nothing to report, so a marker or a
 *  panel row in the assertions can only have come from the case's own
 *  second (post-mutation) response. */
const cleanReport: ReportDocument = {
  config_diagnostics: [],
  batch_diagnostics: [],
  files: [],
  suggestions: [],
  mkvmerge_found: true,
};

function loadedDoc(profile: Profile): LoadProfileDocument {
  return { ...cleanReport, profile };
}

function reportWith(diagnostic: Diagnostic): ReportDocument {
  return { ...cleanReport, config_diagnostics: [diagnostic] };
}

function marker(page: Page, path: string) {
  return page.locator(`[data-diag-path="${path}"]`);
}

/** The profile carried by the LAST recorded `validate_profile_model` call --
 *  the wire truth, not a UI echo. `undefined` while no call is recorded yet,
 *  so it is pollable. */
function lastValidatedProfile(recorded: RecordedInvoke[]): Profile | undefined {
  const calls = recorded.filter((r) => r.cmd === "validate_profile_model");
  const last = calls[calls.length - 1];
  return last === undefined ? undefined : (last.args as { profile: Profile }).profile;
}

test.describe("editor rule add/remove (D65-D70, D72)", () => {
  test("Add appends the empty skeleton as a new last row, in the DOM and in the held model", async ({
    page,
  }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: twoRuleProfile } });

    const rows = page.getByTestId("editor-rule-row");
    await expect(rows).toHaveCount(2);

    await page.getByTestId("editor-rule-add").click();
    await expect(rows).toHaveCount(3);

    // Row-cell layout of the grid: 0 ordinal, 1 the select button (its text
    // is `sourceSummary`), 2 match summary, 3 the disabled optional
    // checkbox, 4 changes summary.
    const newRow = rows.nth(2);
    await expect(newRow.getByRole("cell").nth(0)).toHaveText("3");
    // `sourceSummary`'s fallback for an absent `source`: `SOURCE_KEYWORDS[0]`
    // (`src/bindings/keywords.ts`), a real profile-format token.
    await expect(newRow.getByTestId("editor-rule-select")).toHaveText("primary");
    await expect(newRow.getByRole("cell").nth(2)).toHaveText("");
    await expect(newRow.getByRole("cell").nth(4)).toHaveText("");
    await expect(newRow.getByRole("checkbox")).not.toBeChecked();

    // Anti-vacuity: the held MODEL gained exactly the skeleton, not just a
    // row that renders like one.
    const model = (await readModel(page)) as Profile;
    expect(model.tracks.rules).toHaveLength(3);
    expect(model.tracks.rules[2]).toEqual({ match: {} });
  });

  test("Add auto-selects the new rule and opens its detail panel", async ({ page }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: twoRuleProfile } });

    await page.getByTestId("editor-rule-add").click();

    const newRow = page.getByTestId("editor-rule-row").nth(2);
    await expect(newRow.getByTestId("editor-rule-select")).toHaveAttribute("aria-current", "true");

    // The panel opens purely reactively (`v-if="selectedRule"`); it renders
    // the four `trackRule` fields through the real registry.
    const panel = page.getByTestId("editor-rule-detail");
    await expect(panel).toBeVisible();
    await expect(panel.getByRole("combobox", name("editor-track-rule-source"))).toBeVisible();
    await expect(panel.getByRole("group", name("editor-track-rule-match-expr"))).toBeVisible();
    await expect(panel.getByRole("checkbox", name("editor-track-rule-optional"))).toBeVisible();
    await expect(panel.getByRole("group", name("editor-track-rule-changes"))).toBeVisible();
  });

  test("Remove is disabled without a selection and enabled once a row is selected", async ({
    page,
  }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: twoRuleProfile } });

    const remove = page.getByTestId("editor-rule-remove");
    await expect(remove).toBeDisabled();

    await page.getByTestId("editor-rule-select").first().click();
    await expect(remove).toBeEnabled();
  });

  test("Remove deletes the selected rule and clears the selection", async ({ page }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: twoRuleProfile } });

    const rows = page.getByTestId("editor-rule-row");
    await expect(rows).toHaveCount(2);
    await expect(rows.nth(0)).toContainText("video");

    await page.getByTestId("editor-rule-select").first().click();
    await page.getByTestId("editor-rule-remove").click();

    // The RIGHT rule went: the selected one is gone, the other one stayed.
    await expect(rows).toHaveCount(1);
    await expect(rows.filter({ hasText: "video" })).toHaveCount(0);
    await expect(rows.nth(0)).toContainText("audio");

    // Selection cleared (D66): no row is current, the panel closed.
    await expect(page.locator('[data-testid="editor-rule-select"][aria-current="true"]')).toHaveCount(0);
    await expect(page.getByTestId("editor-rule-detail")).toHaveCount(0);

    const model = (await readModel(page)) as Profile;
    expect(model.tracks.rules).toHaveLength(1);
  });

  test("Remove works down to zero rules and Add recovers from zero in one click", async ({
    page,
  }) => {
    await mountComponent(page, { component: "EditorView", props: { modelValue: oneRuleProfile } });

    const editor = page.getByTestId("view-editor");
    const rows = page.getByTestId("editor-rule-row");
    await expect(rows).toHaveCount(1);

    await page.getByTestId("editor-rule-select").first().click();
    await page.getByTestId("editor-rule-remove").click();

    // The zero-rule state is legal (D69, `core-83-zero-rule-keep-passthrough`):
    // no guard, no floor. The grid keeps its caption and headers, and Add
    // stays there, so the state is recoverable.
    await expect(rows).toHaveCount(0);
    await expect(editor.locator("caption")).toContainText(en("editor-tracks-rules"));
    await expect(page.getByRole("columnheader").first()).toHaveText(en("editor-track-rule-order"));
    const add = page.getByTestId("editor-rule-add");
    await expect(add).toBeVisible();

    await add.click();
    await expect(rows).toHaveCount(1);
    await expect(rows.nth(0).getByTestId("editor-rule-select")).toHaveAttribute(
      "aria-current",
      "true",
    );
    await expect(page.getByTestId("editor-rule-detail")).toBeVisible();
  });

  test("Add wires the skeleton onto the wire and core's empty-match warning lands in the open panel and the diagnostics list, Save stays enabled", async ({
    page,
  }) => {
    const PROFILE_PATH = "/profiles/add-skeleton.yaml";
    // The verified emission for `{ match: {} }` beside an exact-match rule
    // (design section 1, run through the repo's own CLI): exactly one
    // warning-severity `empty-match-expression` at `tracks[1].match`.
    const warningReport = reportWith({
      code: "empty-match-expression",
      severity: "warning",
      config_path: "tracks[1].match",
      params: {},
      rendered: "empty-match-expression",
    });

    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        load_profile: [resolveWith(loadedDoc(oneRuleProfile))],
        validate_profile_model: [resolveWith(cleanReport), resolveWith(warningReport)],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: PROFILE_PATH }))).toBeVisible();
    await expect(editor.getByTestId("editor-rule-row")).toHaveCount(1);

    await editor.getByTestId("editor-rule-add").click();

    // (a) Wire truth: the appended member on the LAST validated payload is
    // the skeleton itself, not a UI echo of one.
    await expect.poll(() => lastValidatedProfile(recorded)?.tracks.rules.length).toBe(2);
    const validatedRules = lastValidatedProfile(recorded)?.tracks.rules;
    expect(validatedRules?.[1]).toEqual({ match: {} });

    // (b) The warning's marker renders at its exact anchor INSIDE the
    // auto-opened detail panel (the nested Match `SectionWidget`'s own
    // legend, D57 exact-string anchoring).
    const panel = editor.getByTestId("editor-rule-detail");
    await expect(panel).toBeVisible();
    await expect(panel.locator('[data-diag-path="tracks[1].match"]')).toHaveClass(
      /\bdiag-marker--warning\b/,
    );

    // (c) The exact-anchoring negative: the grid ROW marker anchors the bare
    // `tracks[1]` and therefore does NOT fire for a `tracks[1].match`
    // diagnostic. (b) above is this assertion's in-test positive control --
    // the same marker layer demonstrably renders for the deeper path.
    await expect(marker(page, "tracks[1]")).toHaveCount(0);

    // (d) The never-filtered diagnostics panel lists the code, rendered
    // through the en catalog (the message is placeable-free, so no args).
    await expect(
      editor
        .locator('section[aria-labelledby="editor-diagnostics-heading"] li')
        .filter({ hasText: en("empty-match-expression") }),
    ).toHaveCount(1);

    // (e) D65's warning-severity consequence: the save gate is
    // error-severity only, so a fresh skeleton is guided, not save-blocked.
    // Pinned deliberately -- a future core severity change fails here.
    await expect(editor.getByTestId("editor-save")).toBeEnabled();
  });

  test("Removing the last rule under tracks.unmatched drop surfaces the no-track-rules error at the caption and disables Save", async ({
    page,
  }) => {
    const PROFILE_PATH = "/profiles/zero-rules-drop.yaml";
    const dropProfile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { unmatched: "drop", rules: [{ match: { exact: { type: "video" } } }] },
    };
    const errorReport = reportWith({
      code: "no-track-rules",
      severity: "error",
      config_path: "tracks.rules",
      params: {},
      rendered: "no-track-rules",
    });

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        load_profile: [resolveWith(loadedDoc(dropProfile))],
        validate_profile_model: [resolveWith(cleanReport), resolveWith(errorReport)],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: PROFILE_PATH }))).toBeVisible();

    // Selection is a pure UI write (`selectRule` touches only
    // `selectedIndex`), so it consumes none of the scripted queue.
    await editor.getByTestId("editor-rule-select").first().click();
    await editor.getByTestId("editor-rule-remove").click();

    await expect(editor.getByTestId("editor-rule-row")).toHaveCount(0);
    await expect(marker(page, "tracks.rules")).toHaveClass(/\bdiag-marker--error\b/);
    await expect(editor.getByTestId("editor-save")).toBeDisabled();
  });

  test("Removing the last rule under tracks.unmatched keep surfaces the passthrough info at the caption and leaves Save enabled", async ({
    page,
  }) => {
    const PROFILE_PATH = "/profiles/zero-rules-keep.yaml";
    const keepProfile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { unmatched: "keep", rules: [{ match: { exact: { type: "video" } } }] },
    };
    const infoReport = reportWith({
      code: "passthrough-profile",
      severity: "info",
      config_path: "tracks.rules",
      params: {},
      rendered: "passthrough-profile",
    });

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        load_profile: [resolveWith(loadedDoc(keepProfile))],
        validate_profile_model: [resolveWith(cleanReport), resolveWith(infoReport)],
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: PROFILE_PATH }))).toBeVisible();

    await editor.getByTestId("editor-rule-select").first().click();
    await editor.getByTestId("editor-rule-remove").click();

    // Zero rules under `keep` is a sanctioned pure-passthrough profile
    // (`core-83-zero-rule-keep-passthrough`): info severity, and saving it
    // stays possible.
    await expect(editor.getByTestId("editor-rule-row")).toHaveCount(0);
    await expect(marker(page, "tracks.rules")).toHaveClass(/\bdiag-marker--info\b/);
    await expect(editor.getByTestId("editor-save")).toBeEnabled();
  });
});
