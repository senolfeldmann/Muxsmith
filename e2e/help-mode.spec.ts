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
    // element's topic. Dispatched directly, not via a real focus: headless
    // Chromium does not fire focus events for programmatic OR Tab focus here
    // (verified: activeElement moves and document.hasFocus() is true, yet no
    // focusin reaches the listener), so a real-focus action cannot exercise
    // this listener in this harness -- the dispatched event drives the exact
    // same capture-phase listener a real focusin would.
    await copy.dispatchEvent("focusin");
    await expectSidebarTopic(page, "batch-suggestion-card");

    // Enter pins the focused annotated element (activation suppressed). The
    // keydown listener is on document and fires on the REAL key event;
    // focus() sets activeElement (the keydown target) even though it does not
    // itself fire focusin in this harness.
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
