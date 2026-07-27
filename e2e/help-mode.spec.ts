/**
 * Task 12 (D52): help mode - the sidebar, the always-visible toggle, the
 * three delegated capture listeners on <main> (hover/focus -> topic, click
 * -> pin with ALL activation suppressed per the E3 ruling), topic
 * resolution per active view, and Esc exiting help unless the settings
 * dialog is open. Drives the real served app with mocked IPC (`mocks.ts`),
 * exactly like `smoke.spec.ts`; annotations (`data-help-id`) arrive in Task
 * 13, so this spec exercises only what exists without them - which is
 * precisely the E3 global-suppression path (an unannotated control is
 * inert, no pin).
 *
 * The sidebar's rendered markup is compared against `marked` over the en
 * topic file (the `topicHtml` mechanism Task 11 landed), normalized once
 * through the page's own DOM so the check is serialization-identical to
 * what `v-html` produced - a raw-string compare would trip over the
 * browser's HTML re-serialization. The toggle's tooltip and the topic
 * files are the source of truth, never a hand-duplicated literal (the
 * binding test-discipline convention).
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";
import { marked } from "marked";
import type { Page } from "@playwright/test";
import { installTauriMocks, resolveWith } from "./mocks";
import type { RecordedInvoke } from "./mocks";
import { en, enAttr } from "./i18n-en";
import type { LoadProfileDocument, MkvmergeInfo, ReportDocument } from "../src/ipc";
import type { Profile } from "../src/bindings/profile";

const MKVMERGE_INFO: MkvmergeInfo = { path: "/usr/bin/mkvmerge", version: "90.0.0" };

/** `marked` over the en topic file - the same string `topicHtml` produces
 *  and `v-html` assigns to the sidebar. */
function topicMarkup(helpId: string): string {
  return marked.parse(readFileSync(join("help", "en", `${helpId}.md`), "utf8"), { async: false });
}

/** Re-serializes `html` through the page DOM so it matches the sidebar's
 *  own `innerHTML` byte-for-byte (both pass through the browser's parser). */
function normalizeInPage(page: Page, html: string): Promise<string> {
  return page.evaluate((h) => {
    const div = document.createElement("div");
    div.innerHTML = h;
    return div.innerHTML;
  }, html);
}

/** The rule grid's source-cell summaries in row order -- the observable a
 *  drag-reorder permutes (`sourceSummary`, EditorView.vue). */
function readRuleOrder(page: Page): Promise<string[]> {
  return page
    .getByTestId("editor-rule-select")
    .evaluateAll((els) => els.map((el) => (el.textContent ?? "").trim()));
}

/**
 * Fires a synthetic HTML5 drag of rule row `from` onto row `to` and reports
 * what the app did with it. Real pointer-driven drag is unreliable headless
 * (the whole-branch review's own note), so the drag is dispatched as events
 * and the browser's drag state machine is modelled faithfully: a `dragstart`
 * whose default the app prevents ABORTS the drag -- no `drop` follows, so
 * `onDrop` never reorders. The drop is therefore dispatched only when the app
 * left `dragstart` un-prevented, exactly as a real webview would. Two rAFs
 * flush Vue's scheduler so the caller reads the settled grid, not a
 * pre-render snapshot.
 */
function attemptDrag(
  page: Page,
  from: number,
  to: number,
): Promise<{ dragstartPrevented: boolean; dropDispatched: boolean }> {
  return page.evaluate(
    async ({ from, to }) => {
      const rows = document.querySelectorAll('[data-testid="editor-rule-row"]');
      const source = rows[from];
      const target = rows[to];
      const dt = new DataTransfer();
      const dragstart = new DragEvent("dragstart", { bubbles: true, cancelable: true, dataTransfer: dt });
      source.dispatchEvent(dragstart);
      let dropDispatched = false;
      if (!dragstart.defaultPrevented) {
        target.dispatchEvent(new DragEvent("dragover", { bubbles: true, cancelable: true, dataTransfer: dt }));
        target.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer: dt }));
        source.dispatchEvent(new DragEvent("dragend", { bubbles: true, cancelable: true, dataTransfer: dt }));
        dropDispatched = true;
      }
      await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(() => r(undefined))));
      return { dragstartPrevented: dragstart.defaultPrevented, dropDispatched };
    },
    { from, to },
  );
}

test.describe("help mode (D52)", () => {
  test("toggle opens the sidebar on the active view's topic; the nav stays live and swaps it; a click in <main> is suppressed; Esc exits", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });

    await page.goto("/");

    const toggle = page.getByTestId("help-toggle");
    const sidebar = page.getByTestId("help-sidebar");

    // The always-visible toggle carries aria-pressed and the Fluent tooltip.
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
    await expect(toggle).toHaveAttribute("title", enAttr("help-toggle-label", "tooltip"));

    // Toggling on shows the sidebar rendering the active (batch) view topic.
    await expect(sidebar).toBeHidden();
    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
    await expect(sidebar).toBeVisible();
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-batch")));

    // The nav stays live in help mode: switching to Jobs swaps the topic.
    await page.getByTestId("nav-jobs").click();
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-jobs")));

    // Back to batch to check activation suppression there.
    await page.getByTestId("nav-batch").click();
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-batch")));

    // A click inside <main> (the batch source-dir browse button) does NOT
    // fire its action: the mocked directory-pick IPC is never invoked, and
    // an unannotated target changes no topic (owner decision B / E3).
    await page.getByTestId("batch-source-browse").click();
    expect(recorded.some((r) => r.cmd === "plugin:dialog|open")).toBe(false);
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-batch")));

    // Escape (no dialog open) exits help mode - the sidebar detaches.
    await page.keyboard.press("Escape");
    await expect(sidebar).toBeHidden();
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
  });

  test("with the settings dialog open, Escape closes the dialog and help mode stays active", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });

    await page.goto("/");

    const toggle = page.getByTestId("help-toggle");
    const sidebar = page.getByTestId("help-sidebar");

    await toggle.click();
    await expect(sidebar).toBeVisible();

    await page.getByTestId("open-settings").click();
    const dialog = page.getByTestId("settings-dialog");
    await expect(dialog).toBeVisible();

    // The native modal's own cancel semantics win: Escape closes the dialog
    // and help mode is untouched (it sits below the top layer).
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(sidebar).toBeVisible();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
  });

  /**
   * Plan 7.5 (D71): the rule-grid Add/Remove buttons are unannotated
   * activation controls in the content area, so help mode covers them BY
   * CONSTRUCTION -- the shipped capture-phase delegation (`onHelpClick`,
   * `onHelpKeydown` in App.vue) closes both mutation channels with no new
   * listener and no button-side condition. This case is that conformance
   * claim's proof, in the I1 sibling's shape below: the mutation controls
   * (help OFF) and the suppression assertions (help ON) share one test and
   * one harness, so a broken fixture cannot let the suppression halves pass
   * vacuously.
   *
   * Add, not Remove, carries the assertions deliberately: Remove is disabled
   * without a selection, so a suppression check against it could pass
   * vacuously -- a disabled button mutates nothing in either mode.
   */
  test("the rule-grid Add button mutates outside help mode; both activation channels are suppressed inside it", async ({
    page,
  }) => {
    const EDITOR_PROFILE_PATH = "/profiles/add-to-me.yaml";
    const report: ReportDocument = {
      config_diagnostics: [],
      batch_diagnostics: [],
      files: [],
      suggestions: [],
      mkvmerge_found: true,
    };
    const oneRuleProfile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { rules: [{ match: { exact: { type: "video" } } }] },
    };

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(EDITOR_PROFILE_PATH)],
        load_profile: [resolveWith({ ...report, profile: oneRuleProfile } satisfies LoadProfileDocument)],
        validate_profile_model: [resolveWith(report)],
      },
    });
    await page.goto("/");

    await page.getByTestId("nav-editor").click();
    await page.getByTestId("editor-open").click();
    const rows = page.getByTestId("editor-rule-row");
    const add = page.getByTestId("editor-rule-add");
    await expect(rows).toHaveCount(1);

    // Controls (help mode OFF): BOTH activation channels genuinely append a
    // rule here, so neither unchanged-count assertion below can pass for the
    // trivial reason that this fixture's Add never mutates at all.
    await add.click();
    await expect(rows).toHaveCount(2);
    await add.focus();
    await page.keyboard.press("Enter");
    await expect(rows).toHaveCount(3);

    const sidebar = page.getByTestId("help-sidebar");
    await page.getByTestId("help-toggle").click();
    await expect(sidebar).toBeVisible();

    // Help mode ON, pointer channel: the capture-phase click listener
    // preventDefaults and stopPropagations before `@click="addRule"` can run,
    // and pins the nearest annotated ancestor -- the view root's
    // `view-editor`, since the buttons themselves carry no `data-help-id`
    // (D71's fallthrough). The pinned topic is this half's evidence that the
    // listener actually handled the click, so the unchanged row count is a
    // suppression rather than an event that never arrived.
    await add.click();
    await expect(rows).toHaveCount(3);
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-editor")));

    // Help mode ON, keyboard channel: `onHelpKeydown` intercepts Enter while
    // `helpTarget()` resolves through that same ancestor walk and
    // preventDefaults it, so the browser never synthesizes the button's
    // activation click. The channels `help-mode-suppression-pointer-scope`
    // deliberately keeps live are typing and keyboard select changes, neither
    // of which a button has.
    await add.focus();
    await page.keyboard.press("Enter");
    await expect(rows).toHaveCount(3);
  });
});

/**
 * Task 13 (D54): the annotated set drives the sidebar. These cases exercise
 * the annotations Task 13 adds (18 registry helpIds via the dispatcher
 * fallthrough, plus the 5 hand-written `data-help-id` template literals) on
 * top of Task 12's landed interaction machinery. The hover/pin resolution
 * priority (`pinnedId ?? hoverId ?? VIEW_TOPICS[activeView]`) is unchanged;
 * what is new is that `event.target.closest("[data-help-id]")` now resolves
 * to a real id instead of always null. Every annotated view carries its own
 * `data-help-id` on its root section, so an "unannotated" hover inside a
 * view resolves to that view's topic (the amended D54 semantics: no card
 * hover topic, the sidebar falls to pinned-else-view) -- the observable is
 * the sidebar content, asserted against the same `topicHtml` mechanism the
 * D52 spec above uses.
 */
test.describe("help mode annotations (D54)", () => {
  const PROFILE_PATH = "/profiles/demo.yaml";
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
    batch_diagnostics: [],
    files: [],
    suggestions: [
      { resolves: "unknown-property", config_path: PROFILE_PATH, edit: null, yaml_fragment: SUGGESTION_YAML },
    ],
    mkvmerge_found: true,
  };

  /** Polls the sidebar's rendered markup against `helpId`'s en topic, so the
   *  reactive re-render after a hover/pin/nav settles before the assertion. */
  async function expectSidebarTopic(page: Page, helpId: string): Promise<void> {
    const expected = await normalizeInPage(page, topicMarkup(helpId));
    await expect.poll(() => page.getByTestId("help-sidebar").innerHTML()).toBe(expected);
  }

  /** Batch view with a rendered suggestion card, help mode ON. The profile
   *  pick and dry-run run BEFORE help mode is entered: both are `<main>`
   *  clicks the E3 global suppression would otherwise eat. Returns the
   *  Node-side invoke log so a caller can prove an action did (not) fire. */
  async function batchCardInHelpMode(page: Page): Promise<RecordedInvoke[]> {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(PROFILE_PATH)],
        validate_profile: [resolveWith(emptyReport)],
        dry_run: [resolveWith(dryRunReport)],
        load_profile: [resolveWith({ ...emptyReport, profile: null } satisfies LoadProfileDocument)],
        apply_suggestion: [resolveWith(null)],
      },
    });
    await page.goto("/");
    await page.getByTestId("batch-profile-pick").click();
    await expect(page.getByText(en("batch-profile-current", { path: PROFILE_PATH }))).toBeVisible();
    await page.getByTestId("batch-dry-run").click();
    await expect(page.getByTestId("batch-suggestion-card")).toBeVisible();
    await page.getByTestId("help-toggle").click();
    await expect(page.getByTestId("help-sidebar")).toBeVisible();
    return recorded;
  }

  test("Batch: the suggestion card hover swaps the sidebar topic; an unannotated hover falls to the view topic (unpinned) but holds the pin (pinned); apply inside the card is suppressed", async ({
    page,
  }) => {
    const recorded = await batchCardInHelpMode(page);
    const card = page.getByTestId("batch-suggestion-card");

    // No hover, no pin: the active (batch) view's own topic.
    await expectSidebarTopic(page, "view-batch");

    // Hovering the annotated card swaps to the card's topic.
    await card.hover();
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Unpinned branch: hovering an unannotated control (the profile pick,
    // no data-help-id) resolves closest() to the view root, so the sidebar
    // falls back to the view topic.
    await page.getByTestId("batch-profile-pick").hover();
    await expectSidebarTopic(page, "view-batch");

    // Clicking the card pins it (activation suppressed, topic pinned).
    await card.locator("pre").click();
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Pinned branch: the same unannotated hover now HOLDS the pinned card
    // topic instead of falling back (pin outranks hover).
    await page.getByTestId("batch-profile-pick").hover();
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Apply inside the card is suppressed: the capture click listener eats
    // it (re-pinning the card), so no apply round trip fires.
    await card.getByTestId("batch-suggestion-apply").click();
    await expectSidebarTopic(page, "batch-suggestion-card");
    expect(recorded.some((r) => r.cmd === "load_profile")).toBe(false);
    expect(recorded.some((r) => r.cmd === "apply_suggestion")).toBe(false);
  });

  test("keyboard: focusin swaps the topic (focusin equivalence); Enter on a focused annotated element pins it", async ({
    page,
  }) => {
    await batchCardInHelpMode(page);
    const copy = page.getByTestId("batch-suggestion-copy");

    await expectSidebarTopic(page, "view-batch");

    // Focusin is the keyboard-equivalent trigger of hover (the a11y pair of
    // capture listeners on <main>): it swaps the sidebar to the focused
    // element's topic. The event is dispatched directly rather than via a
    // real element.focus(): a real .focus() DOES fire a focusin that reaches
    // the <main> capture listener, but in this headless harness it does not
    // drive the app's delegated state (the reactive topic swap), so the
    // direct dispatch exercises the exact same capture-phase listener
    // deterministically.
    await copy.dispatchEvent("focusin");
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Enter pins the focused annotated element (activation suppressed). The
    // keydown listener is on document and fires on the REAL key event;
    // focus() sets activeElement (the keydown target). Its focusin does not
    // drive the delegated topic swap in this harness, but that is immaterial
    // here -- the pin comes from the Enter keydown, not from focusin.
    await copy.focus();
    await page.keyboard.press("Enter");
    // The pin holds as the hover moves to an unannotated control -- only
    // reachable if Enter set the pin (a bare hover would fall back to the
    // view topic).
    await page.getByTestId("batch-profile-pick").hover();
    await expectSidebarTopic(page, "batch-suggestion-card");
  });

  test("switching views clears the pin; the newly active view's topic then shows", async ({
    page,
  }) => {
    await batchCardInHelpMode(page);

    await page.getByTestId("batch-suggestion-card").locator("pre").click();
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Nav stays live in help mode; switching to Jobs clears the pin. Moving
    // the hover into the now-active Jobs view (as a real pointer would on the
    // way in) refreshes the hover topic to view-jobs; the sidebar shows it,
    // which it could not if the batch-card pin had survived the switch (a
    // live pin outranks any hover).
    await page.getByTestId("nav-jobs").click();
    await page.getByTestId("view-jobs").hover();
    await expectSidebarTopic(page, "view-jobs");
  });

  test("a view switch instantly clears a hover topic: hover the card, switch via nav without re-entering <main>, the new view's topic shows immediately (D52 round-6)", async ({
    page,
  }) => {
    await batchCardInHelpMode(page);
    const card = page.getByTestId("batch-suggestion-card");

    // Hover (not pin) the annotated card: the sidebar shows the card topic.
    await card.hover();
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Switch views via the nav, which sits OUTSIDE <main>: Playwright moves
    // the pointer straight to the tab, firing no hover event on any
    // annotated element en route, so nothing refreshes hoverId. Without the
    // round-6 hover-clear, hoverId would still point at the card and the
    // sidebar would keep the stale batch-suggestion-card topic; with it, the
    // pin > hover > view-topic chain lands on view-jobs immediately -- no
    // re-hover into the new view needed (contrast the pin case above, which
    // hovers view-jobs back in).
    await page.getByTestId("nav-jobs").click();
    await expectSidebarTopic(page, "view-jobs");
  });

  test("Editor: hovering the pattern field's widget shows its topic (dispatcher helpId fallthrough)", async ({
    page,
  }) => {
    const EDITOR_PROFILE_PATH = "/profiles/edit-me.yaml";
    const loadedProfile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: { rules: [] },
    };

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(EDITOR_PROFILE_PATH)],
        load_profile: [resolveWith({ ...emptyReport, profile: loadedProfile } satisfies LoadProfileDocument)],
        validate_profile_model: [resolveWith(emptyReport)],
      },
    });
    await page.goto("/");

    // Reach the editor and load a profile before help mode (both are <main>
    // clicks the E3 suppression would otherwise eat).
    await page.getByTestId("nav-editor").click();
    await page.getByTestId("editor-open").click();
    const patternWidget = page.locator('[data-help-id="editor-input-pattern"]');
    await expect(patternWidget).toBeVisible();

    await page.getByTestId("help-toggle").click();
    await expect(page.getByTestId("help-sidebar")).toBeVisible();

    // The registry's helpId (=== labelKey) reaches the DOM only through the
    // dispatcher's :data-help-id fallthrough onto the widget root.
    await patternWidget.hover();
    await expectSidebarTopic(page, "editor-input-pattern");
  });
});

/**
 * I1 (whole-branch review): help mode is a "safe inspection overlay" (E3),
 * but the click-capture suppression never covered HTML5 drag -- a
 * drag-reorder of the rule grid mutated the in-memory profile silently, and a
 * later legitimate Save would persist it. The fix suppresses `dragstart` in
 * the same capture set. This case pins both halves: the drag machinery is
 * live (control, help OFF -> the grid reorders) and help mode aborts it
 * (help ON -> the grid is untouched).
 */
test.describe("help mode drag suppression (I1)", () => {
  test("a drag-reorder mutates the rule grid outside help mode but is suppressed inside it", async ({
    page,
  }) => {
    const EDITOR_PROFILE_PATH = "/profiles/reorder-me.yaml";
    const report: ReportDocument = {
      config_diagnostics: [],
      batch_diagnostics: [],
      files: [],
      suggestions: [],
      mkvmerge_found: true,
    };
    const threeRuleProfile: Profile = {
      profile_version: 1,
      input: { pattern: ".*", extensions: ["mkv"] },
      tracks: {
        rules: [
          { source: "alpha", match: { exact: { type: "video" } } },
          { source: "beta", match: { exact: { type: "audio" } } },
          { source: "gamma", match: { exact: { type: "subtitles" } } },
        ],
      },
    };

    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
        "plugin:dialog|open": [resolveWith(EDITOR_PROFILE_PATH)],
        load_profile: [resolveWith({ ...report, profile: threeRuleProfile } satisfies LoadProfileDocument)],
        validate_profile_model: [resolveWith(report)],
      },
    });
    await page.goto("/");

    await page.getByTestId("nav-editor").click();
    await page.getByTestId("editor-open").click();
    await expect(page.getByTestId("editor-rule-row")).toHaveCount(3);
    expect(await readRuleOrder(page)).toEqual(["alpha", "beta", "gamma"]);

    // Control (help mode OFF): the synthetic drag genuinely reorders, so the
    // help-mode assertion below is not a vacuous pass. Row 0 onto row 2 ->
    // [beta, gamma, alpha] (onDrop splices the moved rule into the target slot).
    const outside = await attemptDrag(page, 0, 2);
    expect(outside.dragstartPrevented).toBe(false);
    expect(outside.dropDispatched).toBe(true);
    expect(await readRuleOrder(page)).toEqual(["beta", "gamma", "alpha"]);

    // Help mode ON: the capture-phase dragstart listener preventDefaults, so
    // the browser aborts the drag before any drop and the grid stays put --
    // the E3 overlay guarantee, closing the silent-reorder-then-save leak.
    await page.getByTestId("help-toggle").click();
    await expect(page.getByTestId("help-sidebar")).toBeVisible();

    const inside = await attemptDrag(page, 0, 2);
    expect(inside.dragstartPrevented).toBe(true);
    expect(inside.dropDispatched).toBe(false);
    expect(await readRuleOrder(page)).toEqual(["beta", "gamma", "alpha"]);
  });
});
