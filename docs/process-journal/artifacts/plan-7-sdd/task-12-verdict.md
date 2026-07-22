# Task 12 reviewer verdict (D52 help mode; model: opus, 2026-07-22)

Commit 906260b on plan7-f, parent c445aa7 (approved Task 11).
Diff: 8 files, +334/-24. Implementer verdict DONE_WITH_CONCERNS (Q1 premise
refutation + Q2/Q3 adaptations); all adjudicated below.

## VERDICT: APPROVED

Full gate rerun green by the reviewer, all foreground:
`pnpm lint` exit 0, zero output; `pnpm check:i18n` ok (210 ids, 17 unused
warnings are the pre-existing dynamically-keyed IpcError codes, not Task 12);
`pnpm build` ok (vue-tsc clean; the global stylesheet now ships as
`dist/assets/index-*.css` 0.72 kB, confirming the App.vue side-effect import
attaches); `pnpm test:e2e` **36 passed** (the expected count), including both
help-mode specs and the axe smoke checks. No weakened verification.

---

## D52/E3 semantics - verified behaviorally and by code

1. **Event wiring matches D52 exactly.** Three capture-phase listeners on
   `<main>` (`mouseover`+`focusin` -> `onHelpHover`; `click` -> `onHelpClick`)
   plus one `keydown` on `document`, all registered `true` (capture) and torn
   down on help-off. `helpTarget` = `event.target.closest("[data-help-id]")`
   fallthrough, null-safe. hover == focusin (both call `onHelpHover`), a11y
   parity satisfied. ✅
2. **Click pins WITHOUT activating; global suppression correct.** `onHelpClick`
   calls `preventDefault()` + `stopPropagation()` UNCONDITIONALLY, then pins
   only when `id !== null` (annotated) - unannotated targets suppress with no
   topic change (owner decision B). This is the E3 ruling verbatim: ALL
   activation inside `<main>` is inert. Proven by e2e: clicking
   `batch-source-browse` in help mode leaves `plugin:dialog|open` uncalled and
   the topic unchanged. ✅
3. **E3 allowlist walked against the ruling - structurally satisfied, not an
   array.** Allowlist = {help toggle, nav-batch/jobs/editor, settings button,
   sidebar interior}. Template places `<nav>` (toggle + 3 tabs + settings) and
   `<HelpSidebar>` (sibling of `<main>`) and `<SettingsDialog>` all OUTSIDE
   `<main>`; the suppressing listeners are scoped to `<main>` and never see
   them. Every one of the six allowlist members is structurally outside
   `<main>` (checked in the diff); nothing inside `<main>` escapes. This is the
   design's own prescribed mechanism (Step 5 note) and is drift-proof (no
   explicit allowlist to fall out of sync). e2e confirms nav-jobs still swaps
   views and open-settings still opens the dialog while help mode is on. ✅
4. **Esc yields to an open settings dialog - defineExpose path verified.**
   `onHelpKeydown` returns early on `settingsDialog.value?.isOpen()`;
   `SettingsDialog` exposes `isOpen: () => dialogEl.value?.open ?? false` over
   its native `<dialog ref="dialogEl">` (line 85, 90). `.open` is the native
   HTMLDialogElement state (true after `showModal()`). e2e test #2 proves the
   full path: dialog open + Escape -> dialog closes, help mode stays active
   (aria-pressed still "true"). Enter/Space on a focused annotated element pins
   with activation suppressed; on unannotated (nav buttons have no
   `data-help-id`) the handler no-ops, so keyboard activation of allowlist
   controls survives. ✅
5. **Pin cleared on view switch.** `watch(activeView, () => pinnedId = null)`,
   with the design's rationale comment. Help-off also clears both refs and
   removes listeners. ✅
6. **Topic resolution / hoverId-null fallback matches amended D54/D52.**
   `helpTopicId = pinnedId ?? hoverId ?? VIEW_TOPICS[activeView]` - pin > hover
   > view-topic. When hover is null and nothing pinned it falls to the active
   view's topic; a pin wins over any hover. Matches D54's amended clause
   ("hovering outside the table sets hoverId null; sidebar shows pinned if
   pinned else the active view's topic"). ✅
7. **Highlight bookkeeping.** `setHelpClass` removes the class from the prior
   element and applies to the current, guarded by `helpMode`; `help-hover`
   (faint) / `help-pinned` (prominent) mapping is closed by D52; outline (not
   border, no layout shift). `src/help/state.ts` is the design's verbatim
   module. ✅

## The v-html license

8. **Exactly ONE v-html site, fire-verified.** `grep -rn v-html src/` yields
   only `src/components/HelpSidebar.vue` (one directive, line 16; the other two
   hits are the eslint comment text). It renders `topicHtml(topicId,
   currentLocale.value)` only - first-party topic HTML, no user string. **Fire-
   verify:** injected a second `v-html` into a scratch control file; the grep
   caught it, confirming the check is not vacuous. No sanitizer added (the
   commit touches neither package.json nor topics.ts; no new dependency). ✅

## Catalog ids

9. **Exactly two new ids, both locales, 38/38 recount confirmed.** gui-common
   went 36 -> 38 in en AND de (recomputed from the message-line regex, matching
   the plan's "final 38/38"). New ids: `help-toggle-label` (with `.tooltip`
   attribute - not a separate id) and `help-sidebar-label`. Only gui-common.ftl
   (en+de) touched; no other catalog. de register per the catalog header:
   `Hilfe` / `Hilfe-Seitenleiste`, du-imperative tooltip ("Fahre ueber ... klicke
   ..."). Both ids are referenced in src/ (absent from the check-i18n unused
   list). ✅

## Adjudications (both directions argued)

### Q1 - src/style.css: refuted "Modify" premise -> Created + App.vue import. RULED: mechanical adaptation, in scope. No NEEDS_CONTEXT owed.

**Premise refutation verified at the tree:** parent c445aa7 has no
`src/style.css`, no CSS import in `main.ts` or anywhere in `src/`, and no
global stylesheet at all - the only styling is two `<style scoped>` blocks
(DiagnosticsPanel, LiveLog). index.html links no stylesheet. So the plan's
"Modify: src/style.css" rested on a false existence premise; the implementer
correctly refuted it and surfaced it (a valid completion per
`brief-drafts-verified-against-tree` / `proc-57-briefs-not-ground-truth`, not
a failed task).

**For "mechanical adaptation" (adopted):**
- The CSS *content* is explicitly enumerated - D52 mandates the two-column
  layout around `<main>` (views not reflowed beyond width, sidebar scrolls
  independently) and the two highlight classes with closed semantics
  (faint-hover vs prominent-pin, outline-not-border); the plan's Step 7 hands
  the exact block over. Explicit enumeration authorizes it directly; the
  presentation-token carve-out covers only the exact colors/widths within
  those constraints. This is not silence the grant had to fill.
- The refutation left exactly two open facts, both mechanical: create-vs-modify
  (a tree fact, not a decision) and the attachment site. App.vue-import vs
  main.ts-import is **zero outward effect**: Vite hoists the CSS into the same
  global bundle regardless of which module's side-effect import pulls it -
  identical runtime, no API/symbol surface, no data format, no user-visible
  delta beyond the already-enumerated CSS. It is internal wiring.
- The choice made *respects* the Files enumeration boundary: App.vue is IN the
  task's file set, main.ts is NOT. Importing from App.vue avoids reaching into
  an undeclared file - the more conservative option, and the one the 8-file
  diff confirms (main.ts untouched).
- The "app's first global stylesheet" was forced by D52 itself: the
  highlight classes are toggled at runtime onto `[data-help-id]` elements deep
  inside child view components, which a Vue `<style scoped>` block provably
  cannot reach. The global-reach stylesheet is mandated by the design, not
  invented at the keyboard; the plan even named the file. The whole-branch
  review sees it in the diff.

**For "structural fork owing NEEDS_CONTEXT" (rejected, but real):** the
standing brief grant explicitly excludes user-visible/layout, and "match the
house pattern" says a lone structural deviation ratifies at whole-branch level;
a purist reading says the implementer should have filed a one-line memo
("style.css absent - creating the app's first global stylesheet, attaching via
App.vue - confirm"). Defensible, but not required: the stylesheet's existence
and content were design/plan-enumerated (not silence), leaving only a
zero-outward-effect wiring micro-choice that resolve-and-surface covers. The
implementer did surface it (DONE_WITH_CONCERNS Q1). No fork was opened at the
keyboard; the design opened it.

### Q2 - eslint suppression form. RULED: within adaptation scope, correct.

**Empirically verified (byte-identical probe, restored via `command cp -f` +
`cmp` IDENTICAL, tree clean):** reverting HelpSidebar to the plan's Step-4
`eslint-disable-next-line` form and running eslint leaves the violation LIVE -
`16:5 warning ... vue/no-v-html`. The directive targets the line after the
comment (the `<aside` opening line 13), but `vue/no-v-html` reports on the
directive's own line (16, `v-html="html"`) inside the multi-line element, so
next-line does not cover it. Exactly the mechanism the implementer described.
The rule is configured `warn`, and `pnpm lint` is bare `eslint .` (no
`--max-warnings 0`), so the broken form would not fail the gate by exit code -
but it would leave a permanent unsuppressed warning at the single licensed
site, defeating the disable comment's whole purpose and the plan's own "don't
carry a dead directive" intent. The implementer's block `eslint-disable ...
eslint-enable` suppresses exactly the one rule at exactly the licensed element
and produces zero warnings (confirmed in the clean full-gate run). This is
`brief-drafts-verified-against-tree`: literal draft didn't hold against the
tree, adapted idiomatically, surfaced. Not a verification weakening (it is a
lint-suppression comment, not a test). ✅

### Q3 - the .app-body wrapper div. RULED: mechanical realization of an enumerated layout, not a fork.

D52 explicitly enumerates a "two-column layout around `<main>`". A flex/grid
container around the two columns is the standard minimal DOM realization of
that layout; `<div class="app-body">` wraps only `<main>` and `<HelpSidebar>`,
leaving `<nav>` and the dialog untouched (which also keeps them outside the
suppression scope - see finding 3). The presentation-token carve-out does NOT
cover "layout structure" (it is called out as semantic-carrying) - but it does
not need to, because the layout is explicitly enumerated in D52, and the exact
widths/gap/max-height that the implementer chose ARE presentation tokens
(correctly cited). Counter-view: the plan's Step-5 template snippet did not
show the wrapper, so a strict reader could call it an unannounced structural
addition; rejected because Step 7's "two-column arrangement" cannot exist
without a container around the two columns - the wrapper is forced by the
enumerated layout, not selected among viable alternatives. Verified no reflow:
`<main>` flex:1 1 auto, min-width:0; help-off -> sidebar absent -> full width.
✅

## Diff vs Files list

10. **8 files, exact match to the plan's Files list under the Q1 ruling.**
    Create state.ts + HelpSidebar.vue; Modify App.vue, SettingsDialog.vue,
    src/style.css (Create in fact - Q1), en+de gui-common.ftl; Test
    help-mode.spec.ts. main.ts NOT in the commit (respecting the file set).
    Commit discipline: single commit, **unsigned** (`%G? = N`), correct
    `Co-Authored-By` trailer, message verbatim from Step 9, exactly the 8 files
    staged. ✅

## a11y

11. Toggle carries `aria-pressed` (bound to helpMode) and the Fluent `.tooltip`
    title; sidebar `<aside>` carries `aria-label="$t('help-sidebar-label')"`;
    hover==focusin gives keyboard-focus parity; Enter/Space pin path is
    keyboard-reachable. The axe smoke helper (`AxeBuilder(...).analyze()`,
    fails on serious+) runs inside the smoke specs and is green in the 36-pass
    run. ✅

## Fire-verification record (dimension 6)

- v-html single-site grep: broke it with a scratch control carrying `v-html`;
  the grep fired. Positive control passed.
- Q2 eslint form: broke HelpSidebar back to the plan's next-line directive and
  watched `vue/no-v-html` fire at 16:5; restored byte-identically (cmp
  IDENTICAL, `git status` clean).

## HARVEST

- **Q1 is grant-boundary / over-restriction-watch calibration data (both
  directions).** The boundary worked: the Files-list-as-enumeration held (the
  implementer stayed inside it by importing from App.vue rather than the
  undeclared main.ts), and the implementer surfaced the refuted premise rather
  than silently resolving it. The gap is on the **plan-authoring** side, not
  the keyboard: a plan that hands over CSS destined for a not-yet-existing file
  should say "**Create** `<file>` and attach via `import` in `<in-set file>`"
  so the create-vs-modify fact and the attachment dimension are closed up front
  - the same tree-verification owed to line anchors is owed to "Modify vs
  Create" premises. No implementer defect; recommend the controller note the
  plan-authoring lesson. This is not evidence to loosen the grant.
- **The structural-allowlist-via-scoping pattern is a keeper.** Scoping the
  suppression to `<main>` and placing allowlist controls outside it is
  drift-proof where an explicit allowlist array would rot; worth carrying into
  any future "inert overlay" work.
- **Q2: the plan's `eslint-disable-next-line` on multi-line elements is a
  recurring trap.** For any single-licensed `v-html` (or similar attribute-line
  rule) inside a multi-attribute element, the block `disable/enable` form is
  the reliable one; the next-line form silently misses because the rule reports
  on the attribute line, not the element's opening line. Cheap plan-text fix
  for future v-html sites.

---

# Fix round (D52 round-6 amendment; same reviewer, 2026-07-22)

Design amendment bd21e85 (round-6 approved): a view switch clears **both**
`pinnedId` and `hoverId`, deliberately narrow (hoverId cleared ONLY on view
switch; the delegation's normal null-on-unannotated-hover stays; section 9
carries the negative guard "nothing else clears hoverId eagerly"). Implementer
commit f8e7d5d on plan7-f atop T13's 97f707f, 2 files +39/-10.

## FIX-ROUND VERDICT: APPROVED (one Minor finding on part 3, comment-only)

The mandated amendment is correctly implemented, non-vacuously tested, and
regression-free. Full suite rebuilt from HEAD and rerun by me: **41 passed**
(the expected count). One Minor documentation inaccuracy in a reworded test
comment (part 3) - zero functional/verification impact, recommend a one-line
correction.

### Part 1 - App.vue clears both refs, narrow scope: CORRECT ✅

`watch(activeView, ...)` now sets `pinnedId=null; hoverId=null` with a comment
recording the amended rationale and the narrow scope. Verified no other eager
hoverId clear crept in: `grep "hoverId.value ="` src/ yields exactly three
writes - line 58 (`onHelpHover`, the delegation's normal `= helpTarget(event)`,
null only when the closest is null; unchanged, stays), line 101 (the help-mode
-OFF teardown branch, pre-existing and mandated by D52's original "exiting
clears both refs"), and line 114 (the NEW view-switch clear). Only line 114 is
a new eager clear; the delegation null-set and the teardown are untouched.
Matches the amended clause and section 9's negative guard exactly.

### Part 2 - the new instant-swap test: CORRECT and non-vacuous ✅

Body: `batchCardInHelpMode` -> `card.hover()` -> assert `batch-suggestion-card`
-> `nav-jobs.click()` -> assert `view-jobs`. **No hidden re-hover**: the only
hover is on the card BEFORE the switch; `expectSidebarTopic` is a pure
`expect.poll` on `innerHTML` (no interaction), so the final assertion measures
the instant post-switch state. The poll cannot mask the bug - pre-fix it would
poll the stale card topic until timeout.

**RED reproduced cheaply (probe, byte-identical restore).** Reverted the one
line `hoverId.value = null` in App.vue, rebuilt `dist/` (the webServer serves
`vite preview` over dist, so a rebuild is required between edits), ran only the
round-6 test: **1 failed** at the final `view-jobs` poll (helper line 166) -
the sidebar stayed on the stale `batch-suggestion-card` topic - while **40
passed**. Notably the sibling pin-clear test (line 263) stayed green because it
hovers `view-jobs` back in; only the new test isolates the hover-clear, so it
is non-vacuous. Restored App.vue via `command cp -f` + `cmp` IDENTICAL, rebuilt
dist to HEAD, tree clean.

### Part 3 - F1 focusin comment rewording: INACCURATE (Minor) ⚠

The reworded comment (both occurrences) asserts: *"a real `.focus()` DOES fire
a focusin that reaches the `<main>` capture listener, but in this headless
harness it does not drive the app's delegated state (the reactive topic
swap)."* This is **contradicted by the harness behavior** and is internally
inconsistent with the app wiring: `onHelpHover` is the sole focusin listener on
`<main>`, and it directly sets `hoverId = event.target.closest("[data-help-id]")`
- if a focusin reached it from the focused annotated element, `hoverId` would
become `batch-suggestion-card` and the sidebar WOULD swap. There is no code
path where focusin reaches the listener yet fails to drive the swap.

**Empirical probe (scratch test, byte-identically restored).** Reused the
in-file `batchCardInHelpMode`, then did a real `copy.focus()` with NO
dispatchEvent: `document.activeElement` moved to `batch-suggestion-copy` (focus
succeeded) yet the sidebar did **not** swap (`swapped-to-card: false |
still-view: true`, after a 300 ms settle). Because the dispatched-focusin path
(test line 246) DOES swap - proving the listener sets `hoverId` from the
annotated ancestor when it receives the event - a real focus not swapping means
its focusin does **not** reach the `<main>` capture listener. The ORIGINAL
comment ("headless Chromium ... no focusin reaches the listener") described the
observable outcome correctly; the rewording replaced an accurate statement with
an inaccurate one.

Correction re the brief's framing ("accurate per your own round-1
verification?"): my Task-12 finding 1 verified that the *code* treats mouseover
and focusin equivalently (both call `onHelpHover`), NOT the headless firing
behavior - I did not verify the real-focus/focusin harness fact in round 1, and
on checking it now the reworded claim is not accurate. **Severity Minor**: this
is a test comment with zero functional impact (the test uses deterministic
direct dispatch and is correct; the technique is unchanged and sound).
**Recommend** restoring the original comment's factual claim, or restating the
verified fact: *a real `.focus()` moves `activeElement` but its focusin does not
reach the `<main>` capture listener in this headless harness, so the event is
dispatched directly.* Not a functional blocker; fold into the next touch of
this file.

### Part 4 - no regression + commit discipline: CLEAN ✅

Full suite 41 passed (rebuilt from HEAD f8e7d5d). help-mode.spec.ts now holds 7
tests: the 2 landed D52 tests (describe "help mode (D52)") and 5 in "help mode
annotations (D54)" - T13's original four plus the new round-6 test - all green
within the 41. Commit f8e7d5d: unsigned (`%G? = N`), correct `Co-Authored-By`
trailer, exactly the 2 files (`e2e/help-mode.spec.ts`, `src/App.vue`), +39/-10.
Probe state fully restored (App.vue + help-mode.spec.ts both `cmp` IDENTICAL,
`git status` clean); dist rebuilt to HEAD.

## Fix-round HARVEST

- **Comment-rewording accuracy is its own review surface.** The functional fix
  was clean, but the accompanying F1 comment rewrite substituted a
  plausible-but-false mechanistic claim ("focusin reaches the listener but
  doesn't drive the swap") for the accurate one it was correcting. A reworded
  explanatory comment is a factual claim about the system and deserves the same
  verify-at-the-artifact discipline as a quoted string or a number - the more
  so when it OVERWRITES a previously-correct "verified" note. Cheap to check
  here (one real-focus probe); the trigger is "a comment's causal claim is
  being changed."
- **The narrow-scope guard held.** The amendment's "hoverId cleared ONLY on
  view switch" is greppable and was greppable-verified (3 writes, 1 new). Good
  example of a section-9 negative guard that a reviewer can mechanically
  confirm rather than eyeball.
