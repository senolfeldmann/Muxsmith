/**
 * Task 14 (D57): field-anchored diagnostic markers. Drives the real app
 * (nav -> editor -> open) with mocked `load_profile`/`validate_profile_model`
 * returning a document whose `config_diagnostics` carry the closed
 * `config_path` list -- one entry per anchor mechanism plus the grammar's
 * asymmetries (design section 1, plan Task 14). Every path resolves to a
 * control by EXACT string equality against the paths the widget tree
 * constructs while rendering; a path with no rendered control
 * (`profile_version`) is panel-only, and the diagnostics panel is never
 * filtered. Codes are real `diagnostics.ftl` codes with matching params so
 * the marker `title` (the panel's own render) resolves.
 *
 * Markers are located by a `data-diag-path` attribute the marker span
 * carries (house `data-testid`-for-test-location convention): it names the
 * exact anchored path, so a marker on the wrong control -- or a redundant
 * second marker for a path -- fails a count, not just a presence check.
 */
import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import type { Page } from "@playwright/test";
import { installTauriMocks, resolveWith } from "./mocks";
import { en, name } from "./i18n-en";
import type { Diagnostic, LoadProfileDocument, MkvmergeInfo, ReportDocument } from "../src/ipc";
import type { Profile } from "../src/bindings/profile";

const MKVMERGE_INFO: MkvmergeInfo = { path: "/usr/bin/mkvmerge", version: "90.0.0" };
const PROFILE_PATH = "/profiles/marker-demo.yaml";

// One primary rule (rule 0) carries the structure fixtures 6-10 anchor
// against (a match with `exact` + a one-item `any`, and a `changes` row
// keyed `language`); rule 1 exists only so `tracks[1]` (ProvableOverlap,
// the grid's second row) is a real, rendered row. Rule 0 additionally
// carries a bare `tracks[0]` diagnostic (fixture #17, OverlappingRules):
// its grid row anchors one marker, and opening rule 0's detail panel roots
// a `SectionWidget` at the same `tracks[0]` path -- the D57 single-anchor
// case the panel must not double (F1). `chapters`/`title`/
// `output.filename` stay bare keywords: the keyword-or-block widget renders
// its nested block regardless, so the template/locator sub-widgets those
// paths anchor at exist without populating the block.
const markerProfile: Profile = {
  profile_version: 1,
  meta: { name: "demo" },
  input: { pattern: ".*", extensions: ["mkv"] },
  output: { filename: "keep" },
  tracks: {
    rules: [
      {
        source: "primary",
        match: { exact: { type: "video" }, any: [{}] },
        changes: { language: "xx" },
      },
      { match: { exact: { type: "audio" } } },
    ],
  },
  attachments: { rules: [{ select: {} }] },
  chapters: "keep",
  title: "keep",
};

// The closed path list. `input.pattern` carries TWO diagnostics
// (warning + error) -- the worst-of-severity case: its marker must render
// `--error`. 17 diagnostics over 16 distinct paths.
const diagnostics: Diagnostic[] = [
  { code: "unsupported-profile-version", severity: "error", config_path: "profile_version", params: { found: "2", supported: "1" }, rendered: "unsupported-profile-version" },
  { code: "multiple-identifier-matches", severity: "warning", config_path: "input.pattern", params: { name: "a.mkv" }, rendered: "multiple-identifier-matches" },
  { code: "invalid-regex", severity: "error", config_path: "input.pattern", params: { detail: "bad" }, rendered: "invalid-regex" },
  { code: "unknown-extension", severity: "warning", config_path: "input.extensions", params: { extension: "mvk", known: "mkv, mp4" }, rendered: "unknown-extension" },
  { code: "no-track-rules", severity: "error", config_path: "tracks.rules", params: {}, rendered: "no-track-rules" },
  { code: "provable-overlap", severity: "error", config_path: "tracks[1]", params: { rule_a: "1", rule_b: "2" }, rendered: "provable-overlap" },
  { code: "empty-match-expression", severity: "warning", config_path: "tracks[0].match", params: {}, rendered: "empty-match-expression" },
  { code: "invalid-property-value", severity: "error", config_path: "tracks[0].changes.language", params: { property: "language", value: "xx" }, rendered: "invalid-property-value" },
  { code: "locator-conflict", severity: "error", config_path: "tracks[0].source.external", params: {}, rendered: "locator-conflict" },
  { code: "empty-match-list", severity: "warning", config_path: "tracks[0].match.any", params: {}, rendered: "empty-match-list" },
  { code: "empty-match-expression", severity: "info", config_path: "tracks[0].match.any[0]", params: {}, rendered: "empty-match-expression" },
  { code: "attachment-rule-shape", severity: "error", config_path: "attachments.rules[0]", params: { found: "0" }, rendered: "attachment-rule-shape" },
  { code: "missing-external", severity: "warning", config_path: "attachments.rules[0].add", params: {}, rendered: "missing-external" },
  { code: "invalid-template", severity: "error", config_path: "output.filename.template", params: { kind: "empty-field", pos: "3" }, rendered: "invalid-template" },
  { code: "invalid-keyword", severity: "info", config_path: "chapters", params: { found: "x", allowed: "external" }, rendered: "invalid-keyword" },
  { code: "path-separator-in-template", severity: "error", config_path: "title.template", params: {}, rendered: "path-separator-in-template" },
  // #17 (F1): a bare `tracks[0]` diagnostic on the rule whose detail panel
  // the test opens. Its grid-row marker is the sole anchor for `tracks[0]`;
  // the detail-panel `SectionWidget` roots at the same path and must
  // `suppress-self-anchor` rather than render a redundant second marker.
  { code: "overlapping-rules", severity: "error", config_path: "tracks[0]", params: { rules: "tracks[0], tracks[1]", track: "0" }, rendered: "overlapping-rules" },
];

const loadedDoc: LoadProfileDocument = {
  config_diagnostics: diagnostics,
  batch_diagnostics: [],
  files: [],
  suggestions: [],
  mkvmerge_found: true,
  profile: markerProfile,
};

const validateDoc: ReportDocument = {
  config_diagnostics: diagnostics,
  batch_diagnostics: [],
  files: [],
  suggestions: [],
  mkvmerge_found: true,
};

// Markers rendered while the per-rule detail panel is closed: everything
// top-level and the grid, minus `profile_version` (no rendered control).
const TOP_MARKERS: { path: string; severity: string }[] = [
  { path: "input.pattern", severity: "error" },
  { path: "input.extensions", severity: "warning" },
  { path: "tracks.rules", severity: "error" },
  { path: "tracks[0]", severity: "error" },
  { path: "tracks[1]", severity: "error" },
  { path: "attachments.rules[0]", severity: "error" },
  { path: "attachments.rules[0].add", severity: "warning" },
  { path: "output.filename.template", severity: "error" },
  { path: "chapters", severity: "info" },
  { path: "title.template", severity: "error" },
];

// The additional markers that appear once rule 0's detail panel is open.
const DETAIL_MARKERS: { path: string; severity: string }[] = [
  { path: "tracks[0].match", severity: "warning" },
  { path: "tracks[0].changes.language", severity: "error" },
  { path: "tracks[0].source.external", severity: "error" },
  { path: "tracks[0].match.any", severity: "warning" },
  { path: "tracks[0].match.any[0]", severity: "info" },
];

function marker(page: Page, path: string) {
  return page.locator(`[data-diag-path="${path}"]`);
}

async function assertNoSeriousA11yViolations(page: Page): Promise<void> {
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter(
    (v) => v.impact === "serious" || v.impact === "critical",
  );
  const report = serious
    .map((v) => `${v.id} (${v.impact}): ${v.help}\n  ${v.nodes.map((n) => n.target.join(" ")).join("\n  ")}`)
    .join("\n\n");
  expect(serious, report).toEqual([]);
}

test("field-anchored markers resolve by exact config_path; panel stays complete; worst-of severity; aria-invalid only on error form controls", async ({
  page,
}) => {
  await installTauriMocks(page, {
    commands: {
      detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
      load_profile: [resolveWith(loadedDoc)],
      validate_profile_model: [resolveWith(validateDoc)],
    },
  });

  await page.goto("/");
  await page.getByTestId("nav-editor").click();
  const editor = page.getByTestId("view-editor");
  await editor.getByRole("button", name("batch-profile-pick")).click();
  await expect(editor.getByText(en("batch-profile-current", { path: PROFILE_PATH }))).toBeVisible();

  // The panel is never filtered: all 17 diagnostics list, including the
  // panel-only `profile_version`.
  await expect(
    editor.locator('section[aria-labelledby="editor-diagnostics-heading"] li'),
  ).toHaveCount(17);

  // `profile_version` has no rendered control -> no marker anywhere, panel
  // row still present (asserted by the count above).
  await expect(marker(page, "profile_version")).toHaveCount(0);

  // Top-level + grid markers, each on exactly its own control at its worst
  // severity. The total count is exact: a redundant second marker for any
  // path (e.g. a keyword-or-block root double-rendered by its nested block)
  // would fail it.
  for (const { path, severity } of TOP_MARKERS) {
    await expect(marker(page, path)).toHaveClass(new RegExp(`\\bdiag-marker--${severity}\\b`));
  }
  await expect(editor.locator("[data-testid='diag-marker']")).toHaveCount(TOP_MARKERS.length);

  // `input.pattern` carries warning + error: the marker renders the worst.
  await expect(marker(page, "input.pattern")).toHaveClass(/\bdiag-marker--error\b/);
  await expect(marker(page, "input.pattern")).not.toHaveClass(/\bdiag-marker--warning\b/);

  // The outline class rides the anchored control for every severity; the
  // input carries the severity, error additionally sets aria-invalid.
  const patternInput = editor.getByRole("textbox", name("editor-input-pattern"));
  await expect(patternInput).toHaveClass(/\bdiag-anchored--error\b/);
  await expect(patternInput).toHaveAttribute("aria-invalid", "true");

  // Scoped to the Input group: `Locator.extensions` (attachment add,
  // chapters external) shares the "Extensions" accessible name.
  const extInput = editor
    .getByRole("group", name("editor-profile-input"))
    .getByRole("textbox", name("editor-input-extensions"));
  await expect(extInput).toHaveClass(/\bdiag-anchored--warning\b/);
  expect(await extInput.getAttribute("aria-invalid")).toBeNull();

  const chaptersSelect = editor.getByRole("combobox", name("editor-profile-chapters"));
  await expect(chaptersSelect).toHaveClass(/\bdiag-anchored--info\b/);
  expect(await chaptersSelect.getAttribute("aria-invalid")).toBeNull();

  await assertNoSeriousA11yViolations(page);

  // Open rule 0's detail panel: the tracks[0].* markers now render, at
  // their exact anchor depths and asymmetries.
  await editor.getByTestId("editor-rule-select").first().click();
  const rulePanel = editor.getByTestId("editor-rule-detail");
  await expect(rulePanel).toBeVisible();

  // D57 single-anchor invariant (F1): the detail panel roots a
  // `SectionWidget` at `selectedPath` (`tracks[0]`) but must
  // `suppress-self-anchor`, since the grid row already anchors that path.
  // With rule 0's panel open, `tracks[0]` still resolves to exactly one
  // marker (the grid row); a re-anchoring detail root would make it two.
  await expect(marker(page, "tracks[0]")).toHaveCount(1);

  for (const { path, severity } of DETAIL_MARKERS) {
    await expect(marker(page, path)).toHaveClass(new RegExp(`\\bdiag-marker--${severity}\\b`));
  }
  await expect(editor.locator("[data-testid='diag-marker']")).toHaveCount(
    TOP_MARKERS.length + DETAIL_MARKERS.length,
  );

  // Error at a propertyMap row -> aria-invalid on that row's value input.
  const changesValue = rulePanel
    .getByRole("group", name("editor-track-rule-changes"))
    .getByTestId("property-map-value");
  await expect(changesValue).toHaveAttribute("aria-invalid", "true");

  // The panel is still complete and unfiltered after selection.
  await expect(
    editor.locator('section[aria-labelledby="editor-diagnostics-heading"] li'),
  ).toHaveCount(17);

  await assertNoSeriousA11yViolations(page);
});
