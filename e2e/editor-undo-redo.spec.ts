/**
 * Task 4 (D108, D112, plan 12 W3): undo/redo over the editor's single
 * mutation funnel (`watch(model)` in `EditorView.vue`), and the derived
 * save state (`dirty`, consumed by Tasks 5-6, not asserted here). Served
 * app (`page.goto("/")`), mocked IPC (`mocks.ts`) -- undo/redo only runs
 * once `sessionActive` is true, which only `openPath`/`createBlank` set,
 * so these cases open a profile first, matching Task 13's own precedent
 * rather than the bare mount harness (see `e2e/editor-rule-add-remove.
 * spec.ts`'s corrected header doc for why the mount harness never reaches
 * this watcher at all).
 *
 * Step 5's mutation-path table (D108 decision 1's own authoring
 * measurement, re-derived and pasted into `task-4-report.md`): the six
 * functions the whole-value-assignment sweep returned, one row and one
 * `test()` each, closed -- a seventh path found here is a finding, not an
 * invented row.
 *
 *   | function              | reached through                              |
 *   |------------------------|----------------------------------------------|
 *   | `setFieldValue`        | a top-level field (the pattern text field)    |
 *   | `setTracksUnmatched`   | `tracks.unmatched`'s own select                |
 *   | `setRuleValue`         | the selected rule's detail panel               |
 *   | `onDrop`                | a drag-reorder of two rows                     |
 *   | `addRule`               | the Add button                                 |
 *   | `removeSelectedRule`    | the Remove button, after a selection           |
 */
import { expect, test } from "@playwright/test";
import type { Locator, Page } from "@playwright/test";
import { installTauriMocks, resolveWith } from "./mocks";
import type { MockResult, RecordedInvoke } from "./mocks";
import { en, name } from "./i18n-en";
import type { AppSettings, Diagnostic, LoadProfileDocument, MkvmergeInfo, ReportDocument } from "../src/ipc";
import type { Profile } from "../src/bindings/profile";

const MKVMERGE_INFO: MkvmergeInfo = { path: "/usr/bin/mkvmerge", version: "90.0.0" };

/** The clean, diagnostic-free `validate_profile_model` response every case
 *  below shares: undo/redo cares about history and rendering, not
 *  diagnostics, and the mocked queue's own "last entry repeats" behaviour
 *  (`mocks.ts`) means one entry covers the open's own revalidation and
 *  every mutation/undo/redo-triggered one after it. */
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

/** The `load_profile` response for a path that resolves but fails to
 *  parse (D108 decision 9's own failed-load branch): `profile: null` plus
 *  one `parse-error` diagnostic, the same fixture shape `e2e/smoke.spec.ts`
 *  already uses for the D103 apply-flow case. */
const parseErrorDiagnostic: Diagnostic = {
  code: "parse-error",
  severity: "error",
  config_path: "",
  params: { detail: "unknown field", at: "" },
  rendered: "parse-error",
};

function failedLoadDoc(): LoadProfileDocument {
  return {
    config_diagnostics: [parseErrorDiagnostic],
    batch_diagnostics: [],
    files: [],
    suggestions: [],
    profile: null,
  };
}

/** Opens `profile` at `path` through the real Open dialog on the served
 *  app -- the shared setup every case below starts from. `extraCommands`
 *  layers additional mocked commands (e.g. `save_profile`) onto the
 *  standard four; passing a command this function already sets overrides
 *  it (object spread order). */
async function openProfile(
  page: Page,
  path: string,
  profile: Profile,
  extraCommands: Record<string, MockResult[]> = {},
): Promise<{ editor: Locator; recorded: RecordedInvoke[] }> {
  const recorded = await installTauriMocks(page, {
    commands: {
      detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      "plugin:dialog|open": [resolveWith(path)],
      load_profile: [resolveWith(loadedDoc(profile))],
      validate_profile_model: [resolveWith(cleanReport)],
      ...extraCommands,
    },
  });
  await page.goto("/");
  await page.getByTestId("nav-editor").click();
  const editor = page.getByTestId("view-editor");
  await editor.getByTestId("editor-open").click();
  await expect(editor.getByText(en("batch-profile-current", { path }))).toBeVisible();
  return { editor, recorded };
}

const emptyRulesProfile: Profile = {
  profile_version: 1,
  input: { pattern: ".*", extensions: ["mkv"] },
  tracks: { rules: [] },
};

test.describe("editor undo/redo: mutation-path coverage (Task 4, D108 decision 1)", () => {
  test("setFieldValue: a top-level field edit is one undo step", async ({ page }) => {
    const { editor } = await openProfile(page, "/profiles/mutation-set-field-value.yaml", emptyRulesProfile);

    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await expect(pattern).toHaveValue(".*");
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();

    await pattern.fill("^changed$");
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-undo").click();
    await expect(pattern).toHaveValue(".*");

    await editor.getByTestId("editor-redo").click();
    await expect(pattern).toHaveValue("^changed$");
  });

  test("setTracksUnmatched: a tracks.unmatched change is one undo step", async ({ page }) => {
    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { unmatched: "keep", rules: [] },
    };
    const { editor } = await openProfile(page, "/profiles/mutation-set-tracks-unmatched.yaml", profile);

    // Scoped to the Tracks group: `editor-tracks-unmatched` and
    // `editor-attachments-unmatched` share the same rendered text
    // ("Unmatched"), the same ambiguity the Task 12 composition test
    // already documents and scopes around.
    const tracksGroup = editor.getByRole("group", name("editor-profile-tracks"));
    const unmatched = tracksGroup.getByRole("combobox", name("editor-tracks-unmatched"));
    await expect(unmatched).toHaveValue("keep");
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();

    await unmatched.selectOption("drop");
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-undo").click();
    await expect(unmatched).toHaveValue("keep");

    await editor.getByTestId("editor-redo").click();
    await expect(unmatched).toHaveValue("drop");
  });

  test("setRuleValue: a per-rule detail edit is one undo step", async ({ page }) => {
    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { rules: [{ match: { exact: { type: "video" } } }] },
    };
    const { editor } = await openProfile(page, "/profiles/mutation-set-rule-value.yaml", profile);

    // The grid's own disabled checkbox (`rule.optional === true`) is what
    // stays assertable across an Undo: applying a history entry clears
    // `selectedIndex` (D108 decision 10's sibling rule in `undo`/`redo`),
    // which closes the detail panel.
    const gridCheckbox = editor.getByTestId("editor-rule-row").first().getByRole("checkbox");
    await expect(gridCheckbox).not.toBeChecked();
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();

    await editor.getByTestId("editor-rule-select").first().click();
    const panelCheckbox = editor
      .getByTestId("editor-rule-detail")
      .getByRole("checkbox", name("editor-track-rule-optional"));
    await panelCheckbox.check();
    await expect(gridCheckbox).toBeChecked();
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-undo").click();
    await expect(gridCheckbox).not.toBeChecked();
    await expect(editor.getByTestId("editor-rule-detail")).toHaveCount(0);

    await editor.getByTestId("editor-redo").click();
    await expect(gridCheckbox).toBeChecked();
  });

  test("onDrop: a drag-reorder is one undo step", async ({ page }) => {
    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: {
        rules: [{ match: { exact: { type: "video" } } }, { match: { exact: { type: "audio" } } }],
      },
    };
    const { editor } = await openProfile(page, "/profiles/mutation-on-drop.yaml", profile);

    const rows = editor.getByTestId("editor-rule-row");
    await expect(rows.nth(0)).toContainText("video");
    await expect(rows.nth(1)).toContainText("audio");
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();

    // Same programmatic drag-and-drop as the Task 11 rule-grid spec: no
    // `dataTransfer` read on either side, so a shared handle across the
    // two events suffices.
    const dataTransfer = await page.evaluateHandle(() => new DataTransfer());
    await rows.nth(0).dispatchEvent("dragstart", { dataTransfer });
    await rows.nth(1).dispatchEvent("drop", { dataTransfer });

    await expect(rows.nth(0)).toContainText("audio");
    await expect(rows.nth(1)).toContainText("video");
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-undo").click();
    await expect(rows.nth(0)).toContainText("video");
    await expect(rows.nth(1)).toContainText("audio");

    await editor.getByTestId("editor-redo").click();
    await expect(rows.nth(0)).toContainText("audio");
    await expect(rows.nth(1)).toContainText("video");
  });

  test("addRule: Add is one undo step", async ({ page }) => {
    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { rules: [{ match: { exact: { type: "video" } } }] },
    };
    const { editor } = await openProfile(page, "/profiles/mutation-add-rule.yaml", profile);

    const rows = editor.getByTestId("editor-rule-row");
    await expect(rows).toHaveCount(1);
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();

    await editor.getByTestId("editor-rule-add").click();
    await expect(rows).toHaveCount(2);
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-undo").click();
    await expect(rows).toHaveCount(1);

    await editor.getByTestId("editor-redo").click();
    await expect(rows).toHaveCount(2);
  });

  test("removeSelectedRule: Remove is one undo step", async ({ page }) => {
    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: {
        rules: [{ match: { exact: { type: "video" } } }, { match: { exact: { type: "audio" } } }],
      },
    };
    const { editor } = await openProfile(page, "/profiles/mutation-remove-selected-rule.yaml", profile);

    const rows = editor.getByTestId("editor-rule-row");
    await expect(rows).toHaveCount(2);
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();

    await editor.getByTestId("editor-rule-select").first().click();
    await editor.getByTestId("editor-rule-remove").click();
    await expect(rows).toHaveCount(1);
    await expect(rows.filter({ hasText: "video" })).toHaveCount(0);
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-undo").click();
    await expect(rows).toHaveCount(2);
    await expect(rows.nth(0)).toContainText("video");

    await editor.getByTestId("editor-redo").click();
    await expect(rows).toHaveCount(1);
    await expect(rows.filter({ hasText: "video" })).toHaveCount(0);
  });
});

test.describe("editor undo/redo: granularity, truncation, save/open, the depth cap (Task 4, D108)", () => {
  test("granularity: one entry per editing burst -- a single burst, two bursts split by a focus change, and two discrete Add clicks", async ({
    page,
  }) => {
    const { editor } = await openProfile(page, "/profiles/granularity.yaml", emptyRulesProfile);
    const undoButton = editor.getByTestId("editor-undo");
    const redoButton = editor.getByTestId("editor-redo");

    // Half 1: several keystrokes in one continuous focus session are ONE
    // burst. `pressSequentially` (not `fill`) dispatches a real per-
    // character `input` event each keystroke -- `fill()` dispatches a
    // single input event and its own change (measured), which would make
    // this half pass even without the coalescing rule under test.
    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await pattern.fill("");
    await pattern.pressSequentially("XYZ");
    await expect(pattern).toHaveValue("XYZ");
    await expect(undoButton).toBeEnabled();
    await undoButton.click();
    await expect(pattern).toHaveValue(".*");
    await redoButton.click();
    await expect(pattern).toHaveValue("XYZ");

    // Half 2: two edits on two different fields, separated by an explicit
    // focus change (clicking into a different control fires `@focusout`
    // on the one that had focus), are TWO bursts -- two Undos are needed
    // to reach back through both, one at a time.
    const metaName = editor.getByRole("textbox", name("editor-meta-name"));
    await metaName.fill("N1");
    const metaDescription = editor.getByRole("textbox", name("editor-meta-description"));
    await metaDescription.fill("D1");

    await editor.getByTestId("editor-undo").click();
    await expect(metaDescription).toHaveValue("");
    await expect(metaName).toHaveValue("N1");
    await editor.getByTestId("editor-undo").click();
    await expect(metaName).toHaveValue("");
    // Half 1's own edit is untouched: only half 2's two edits were undone.
    await expect(pattern).toHaveValue("XYZ");

    await editor.getByTestId("editor-redo").click();
    await editor.getByTestId("editor-redo").click();
    await expect(metaName).toHaveValue("N1");
    await expect(metaDescription).toHaveValue("D1");

    // Half 3: two clicks of the SAME button, which never moves focus
    // between them -- `addRule`'s own explicit `coalesce = false` (D108
    // decision 2), not `@focusout`, is what keeps these two discrete.
    const rows = editor.getByTestId("editor-rule-row");
    const add = editor.getByTestId("editor-rule-add");
    await add.click();
    await add.click();
    await expect(rows).toHaveCount(2);

    await editor.getByTestId("editor-undo").click();
    await expect(rows).toHaveCount(1);
    await editor.getByTestId("editor-undo").click();
    await expect(rows).toHaveCount(0);
  });

  test("truncation: undo once, then make a new edit; Redo is disabled again", async ({ page }) => {
    const { editor } = await openProfile(page, "/profiles/truncation.yaml", emptyRulesProfile);
    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));

    await pattern.fill("first");
    await editor.getByTestId("editor-undo").click();
    await expect(pattern).toHaveValue(".*");
    await expect(editor.getByTestId("editor-redo")).toBeEnabled();

    await pattern.fill("second");
    await expect(editor.getByTestId("editor-redo")).toBeDisabled();
    await expect(pattern).toHaveValue("second");
  });

  test("save marks rather than clears: after a save, Undo is still enabled and one Undo restores the pre-edit state", async ({
    page,
  }) => {
    const { editor, recorded } = await openProfile(
      page,
      "/profiles/save-marks.yaml",
      emptyRulesProfile,
      { save_profile: [resolveWith(null)] },
    );
    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await pattern.fill("edited");

    await editor.getByTestId("editor-save").click();
    await expect.poll(() => recorded.filter((r) => r.cmd === "save_profile").length).toBe(1);

    await expect(editor.getByTestId("editor-undo")).toBeEnabled();
    await editor.getByTestId("editor-undo").click();
    await expect(pattern).toHaveValue(".*");
  });

  test("open resets: opening a second profile clears both Undo and Redo", async ({ page }) => {
    const PATH_A = "/profiles/open-resets-a.yaml";
    const PATH_B = "/profiles/open-resets-b.yaml";
    const profileB: Profile = {
      profile_version: 1,
      input: { pattern: "^B$", extensions: ["mkv"] },
      tracks: { rules: [] },
    };

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PATH_A), resolveWith(PATH_B)],
        load_profile: [resolveWith(loadedDoc(emptyRulesProfile)), resolveWith(loadedDoc(profileB))],
        validate_profile_model: [resolveWith(cleanReport)],
      },
    });
    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");

    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: PATH_A }))).toBeVisible();
    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await pattern.fill("edited-a");
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: PATH_B }))).toBeVisible();
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();
    await expect(editor.getByTestId("editor-redo")).toBeDisabled();
  });

  // task-4-verdict.md Finding 3 (low-moderate): `createBlank` is the eighth
  // whole-value-assignment site the re-derived sweep found (see
  // task-4-report.md), correctly excluded from the six-function mutation
  // table as a session-start funnel like `openPath` -- but nothing in this
  // file exercised it together with PRE-BUILT undo/redo state, so a
  // regression dropping its own `resetHistory(profile)` call went uncaught
  // by every one of this file's other cases (a fresh session with no prior
  // history already has `canUndo` false, so a bare "click New, assert Undo
  // disabled" case would not discriminate the same regression -- it takes
  // the "open resets" shape, mirrored through the OTHER funnel). Unlike the
  // failed-open case above (Finding 2), `createBlank` always ends with a
  // truthy `model.value` (a freshly seeded Profile), so `:disabled="!model
  // || !canUndo"` never short-circuits on `!model` here -- checked, not
  // assumed: `canUndo`/`canRedo`'s real values are what these asserts read.
  test("createBlank resets: New after edited history clears both Undo and Redo", async ({ page }) => {
    const { editor } = await openProfile(page, "/profiles/create-blank-resets.yaml", emptyRulesProfile);

    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await pattern.fill("edited");
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-new").click();
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();
    await expect(editor.getByTestId("editor-redo")).toBeDisabled();
  });

  // task-4-verdict.md Finding 2 (moderate): this case's name and its two
  // `:disabled` asserts below used to read as proving D108 decision 9
  // clears `history`/`position` on a failed load. They do not, and cannot:
  // `:disabled="!model || !canUndo"` short-circuits on `!model` once the
  // failed load clears `model.value`, so `canUndo`'s real value is never
  // consulted here. Reproduced (scratch mutation, not just quoted from the
  // review): mutating `resetHistory`'s `profile === undefined` branch to
  // leave `history`/`position` standing, only nulling `savedSnapshot`,
  // does NOT fail this test -- it still passes unchanged. No surface in
  // the shipped product reads `canUndo`/`canRedo`/`history`/`position`
  // while `model` is falsy: `undo()`/`redo()` carry the identical
  // `!model.value` short-circuit (D108 decision 10), no other template
  // binding consumes them, and the mount harness exposes only `model`
  // (`__muxsmithModel__`) -- never these internal refs. So the actual
  // history-clearing this case is named for is UNOBSERVABLE by this
  // suite's existing surfaces, and per the controller's ruling this is not
  // invented around (no `defineExpose`, no internal-state reflection): the
  // name and the assertions below are scoped down to what they actually
  // prove -- the OBSERVABLE consequence (the buttons read disabled via the
  // model gate, the editing surface is gone, the diagnostic explains why).
  test("a failed open hides the editing surface and the Undo/Redo buttons read disabled through the model gate -- whether history/position were actually cleared is unobservable here (D108 decisions 9-10)", async ({
    page,
  }) => {
    const PATH_A = "/profiles/failed-open-a.yaml";
    const FAILING_PATH = "/profiles/failed-open-b.yaml";

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PATH_A), resolveWith(FAILING_PATH)],
        load_profile: [resolveWith(loadedDoc(emptyRulesProfile)), resolveWith(failedLoadDoc())],
        validate_profile_model: [resolveWith(cleanReport)],
      },
    });
    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");

    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: PATH_A }))).toBeVisible();
    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await pattern.fill("edited");
    // The state before the failed open: Undo enabled, in the SAME test, so
    // a run that passes because Undo is never enabled anywhere cannot be
    // mistaken for this one passing.
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    await editor.getByTestId("editor-open").click();

    // In this order, per the brief: the diagnostic still renders (the
    // panel explains a failed open), then Undo/Redo read disabled (via the
    // model gate -- see the header comment above for why this does not
    // discriminate `canUndo`/`canRedo`'s real cleared value), then the
    // editing surface is gone.
    await expect(
      editor.getByText(
        en("batch-diagnostic-line", {
          severity: en("severity-error"),
          message: en("parse-error", { detail: "unknown field" }),
        }),
      ),
    ).toBeVisible();
    await expect(editor.getByTestId("editor-undo")).toBeDisabled();
    await expect(editor.getByTestId("editor-redo")).toBeDisabled();
    await expect(editor.getByTestId("editor-save")).toHaveCount(0);
  });

  test("the depth cap: more than HISTORY_DEPTH discrete mutations, and Undo cannot reach the original state", async ({
    page,
  }) => {
    test.setTimeout(90_000);
    // Mirrors `EditorView.vue`'s own D108 decision 5 constant; the click
    // count below is derived from it, not hardcoded a second time.
    const HISTORY_DEPTH = 100;

    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { rules: [{ match: { exact: { type: "video" } } }] },
    };
    const { editor } = await openProfile(page, "/profiles/depth-cap.yaml", profile);

    const rows = editor.getByTestId("editor-rule-row");
    const add = editor.getByTestId("editor-rule-add");
    await expect(rows).toHaveCount(1);

    const totalClicks = HISTORY_DEPTH + 1;
    for (let i = 0; i < totalClicks; i++) {
      await add.click();
    }
    await expect(rows).toHaveCount(1 + totalClicks);

    const undo = editor.getByTestId("editor-undo");
    while (await undo.isEnabled()) {
      await undo.click();
    }

    // Growing from the 1-entry baseline to the HISTORY_DEPTH cap costs
    // `HISTORY_DEPTH - 1` drop-free pushes; every push past that point
    // grows the history to `HISTORY_DEPTH + 1` and immediately drops the
    // then-oldest entry back down to the cap, so it is not only the FIRST
    // push past the cap that drops one -- every one of them does. With
    // `totalClicks` pushes, `totalClicks - (HISTORY_DEPTH - 1)` of them
    // are drop-triggering, and each dropped entry is one Add click's worth
    // of rows the original baseline can no longer be reached through.
    const droppedEntries = totalClicks - (HISTORY_DEPTH - 1);
    await expect(rows).toHaveCount(1 + droppedEntries);
  });

  test("U1: the text-entry exemption -- Ctrl+Z in the pattern field does not undo, the identical combination on a button does", async ({
    page,
  }) => {
    const profile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { rules: [{ match: { exact: { type: "video" } } }] },
    };
    const { editor } = await openProfile(page, "/profiles/text-entry-exemption.yaml", profile);

    const rows = editor.getByTestId("editor-rule-row");
    await editor.getByTestId("editor-rule-add").click();
    await expect(rows).toHaveCount(2);
    await expect(editor.getByTestId("editor-undo")).toBeEnabled();

    // The negative: focus in the pattern field (a text-entry INPUT) --
    // Ctrl+Z there must leave the model untouched, the browser's own
    // character-level undo being what should react instead.
    const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
    await pattern.click();
    await pattern.press("Control+z");
    await expect(rows).toHaveCount(2);

    // The positive control, in the SAME test: the identical combination,
    // focus moved to a plain button (not a text-entry control), DOES undo.
    const selectButton = editor.getByTestId("editor-rule-select").first();
    await selectButton.click();
    await selectButton.press("Control+z");
    await expect(rows).toHaveCount(1);
  });
});

// D112 (owner ruling 2026-07-31): the pre-session gate `nothingOpenedOrCreated`
// hides `editor-empty`/`editor-recents` before anything has been opened or
// created, and keeps them hidden -- not re-shown -- once a load has failed
// to parse, because the path line and the parse error already explain that
// state on screen.
test.describe("editor undo/redo: D112, a failed load hides both pre-session surfaces", () => {
  function settingsWith(recentProfiles: string[]): AppSettings {
    return {
      mkvmerge_path: null,
      default_jobs: 1,
      locale: "en",
      recent_profiles: recentProfiles,
      dir_memory: {},
    };
  }

  test("three legs in one flow: nothing opened yet, a successful open, then a failing open", async ({
    page,
  }) => {
    const RECENT_PATH = "/profiles/d112-recent-and-first-open.yaml";
    const FAILING_PATH = "/profiles/d112-failing-open.yaml";

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        get_settings: [resolveWith(settingsWith([RECENT_PATH]))],
        "plugin:dialog|open": [resolveWith(RECENT_PATH), resolveWith(FAILING_PATH)],
        load_profile: [resolveWith(loadedDoc(emptyRulesProfile)), resolveWith(failedLoadDoc())],
        validate_profile_model: [resolveWith(cleanReport)],
        // `set_settings` deliberately NOT mocked: `e2e/mocks.ts`'s own
        // fallback answers it, the same fixture shape the shipped recents
        // cases in `e2e/smoke.spec.ts` already rely on. The model is never
        // edited in this flow, so `dirty` stays false throughout.
      },
    });

    await page.goto("/");
    await page.getByTestId("nav-editor").click();
    const editor = page.getByTestId("view-editor");
    await expect(editor).toBeVisible();

    // Leg 1 (absence check P2's FIRE): nothing opened or created yet.
    await expect(editor.getByTestId("editor-empty")).toBeVisible();
    await expect(editor.getByTestId("editor-recents")).toHaveCount(1);

    // Leg 2: a successful open closes the gate (Task 3's own shipped
    // behaviour), asserted so leg 3 below cannot be mistaken for covering
    // it.
    await editor.getByTestId("editor-open").click();
    await expect(editor.getByText(en("batch-profile-current", { path: RECENT_PATH }))).toBeVisible();
    await expect(editor.getByTestId("editor-empty")).toHaveCount(0);
    await expect(editor.getByTestId("editor-recents")).toHaveCount(0);

    // Leg 3 (P2's zero): a failing open keeps the gate closed even though
    // the load failed and the model is gone -- the path line names the
    // failing file and the panel carries the parse error instead.
    await editor.getByTestId("editor-open").click();
    await expect(editor.getByTestId("editor-empty")).toHaveCount(0);
    await expect(editor.getByTestId("editor-recents")).toHaveCount(0);
    await expect(editor.getByText(en("batch-profile-current", { path: FAILING_PATH }))).toBeVisible();
    await expect(
      editor.getByText(
        en("batch-diagnostic-line", {
          severity: en("severity-error"),
          message: en("parse-error", { detail: "unknown field" }),
        }),
      ),
    ).toBeVisible();
  });
});
