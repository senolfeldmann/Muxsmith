# Task 14 (D57) - F1 fix report

**Status: DONE**
**Commit: `ff496582be522c416f86b7ab424588124b2d24f3`** (`ff49658`) on `plan7-f`, parent `18a9801`.
**Test summary:** RED->GREEN fire-verified; `pnpm build` (vue-tsc) exit 0, full e2e **42 passed**, eslint on both files exit 0.

Scope executed exactly as directed: the two-part F1 fix, no more. No fork surfaced, no NEEDS_CONTEXT.

---

## What changed and why

### 1. `src/views/EditorView.vue` (the one-line fix)

The bespoke rule-grid row is the design-named anchor for `tracks[{i}]` (lint's `ProvableOverlap`, `lint.rs:34`; grid-row marker `EditorView.vue:566-575`). The per-rule detail panel mounts `<SectionWidget :path="selectedPath">` with `selectedPath = tracks[{i}]` and, before the fix, **no `suppress-self-anchor`** - so `SectionWidget`'s `useDiagAnchor(() => suppressSelfAnchor ? undefined : path)` (`SectionWidget.vue:45-47`) anchored a second marker at the same path on the panel legend whenever a bare `tracks[{i}]` diagnostic existed for the opened rule.

Fix: added `suppress-self-anchor` to that mount. The detail root now uses `tracks[{i}]` as a **child-path prefix only** (via `childPath`), exactly the mechanism the keywordOrBlock->block precedent already uses (`KeywordOrBlockWidget.vue:80`). Extended the adjacent `selectedPath` comment to document the rationale (grid row owns the anchor; detail root suppresses to avoid the redundant marker), matching the file's documented style.

This closes the second instance of the same-path-collision class `suppressSelfAnchor` was created to prevent - the `thorough_separation` gap the verdict identified. Zero outward surface: an internal prop, markers behave exactly as the fixture enumerates.

### 2. `e2e/editor-markers.spec.ts` (fixture reaches the collision + count-stays-1 guard)

The prior fixture masked the defect: its only bare `tracks[{i}]` diagnostic sat on `tracks[1]` (rule 1) while the test opens **rule 0**'s panel, so the flagged rule's own panel was never opened.

- **Added fixture #17**: a bare `tracks[0]` diagnostic (`overlapping-rules`, severity error, params `{ rules: "tracks[0], tracks[1]", track: "0" }`) on rule 0 - the rule whose panel the test opens. Chosen over reusing `provable-overlap` because `OverlappingRules` is the real runtime lint that anchors at `tracks[0]` (`report/mod.rs:368`); `ProvableOverlap` can only anchor at the second rule of a pair (`b_idx`), never at `tracks[0]`. Both codes resolve in the GUI Fluent catalog (`locales/{en,de}/diagnostics.ftl:30/37`); marker `title` renders via `$t(d.code, ...)`, not the `rendered` field.
- **Inserted at the END of the `diagnostics` array** (not near `tracks[1]`) deliberately, to keep every existing 1-based fixture index stable - the `// structure fixtures 6-10` comment and the verdict's `#N` references stay valid. Only the count statement changed (`16 diagnostics over 15 distinct paths` -> `17 diagnostics over 16 distinct paths`).
- **Added `{ path: "tracks[0]", severity: "error" }` to `TOP_MARKERS`** - the grid row for rule 0 now anchors this marker while the panel is closed. The exact-count guards (`toHaveCount(TOP_MARKERS.length)` closed; `toHaveCount(TOP_MARKERS.length + DETAIL_MARKERS.length)` open) auto-adjust off the array length, so no literal change there.
- **Added the count-stays-1 assertion** `expect(marker(page, "tracks[0]")).toHaveCount(1)` as the FIRST panel-open check, so its RED firing is unambiguous.
- **Updated the two panel-completeness exact counts** `toHaveCount(16)` -> `toHaveCount(17)` (closed and after-selection) plus their comments - the panel is unfiltered and now lists 17 diagnostics.

The kept `tracks[1]` diagnostic still exercises the panel-closed grid-row anchor for a rule whose panel is never opened; the new `tracks[0]` additionally exercises the open-panel single-anchor invariant. Not redundant - two distinct structural sites.

The count guards are exact-count guards throughout; kept exact and adjusted by the computable delta (+1 diagnostic, +1 top marker), never weakened.

---

## Fire-verification (TDD RED -> GREEN)

Fixture extension + count-stays-1 assertion written FIRST, run against the **unfixed** source, watched RED, then the one-line fix applied and watched GREEN.

### RED (unfixed `EditorView.vue`, spec already extended)

`pnpm build` exit 0, then `npx playwright test --grep "field-anchored markers"`:

```
Error: expect(locator).toHaveCount(expected) failed
  Locator:  locator('[data-diag-path="tracks[0]"]')
  Expected: 1
  Received: 2
  ...
  14 x locator resolved to 2 elements - unexpected value "2"
  > 222 |   await expect(marker(page, "tracks[0]")).toHaveCount(1);
1 failed
```

With rule 0's panel open, `tracks[0]` resolved to **2** elements (grid row + un-suppressed detail-panel legend). The new assertion fired first, proving it can fire and the double-marker is real and reachable. This is a control-fired count check per `proc-verification-step-must-be-falsifiable`.

### GREEN (fix applied)

`pnpm build` exit 0, then the same filtered run:

```
  ok  1 [chromium] field-anchored markers resolve by exact config_path; ... (1.3s)
  1 passed (1.7s)
```

`tracks[0]` now resolves to exactly 1 element with the panel open.

---

## Every verification command and its result

| Command | Result |
|---|---|
| `pnpm build` (vue-tsc --noEmit && vite build), unfixed src | exit 0 (built) |
| `npx playwright test --grep "field-anchored markers"`, unfixed src | **1 failed** (RED, count 2, as designed) |
| `pnpm build`, fixed src | exit 0 (built) |
| `npx playwright test --grep "field-anchored markers"`, fixed src | **1 passed** (GREEN) |
| `pnpm test:e2e` (e2e tsc + harness/mount builds + full suite) | **42 passed** (incl. editor-markers, editor-tooltips, all mount-harness smoke, axe) |
| `npx eslint src/views/EditorView.vue e2e/editor-markers.spec.ts` | exit 0 |
| `pnpm build` + filtered run after the comment-wording touch-up | built, 1 passed |

Filtered runs used `npx playwright test --grep` directly (not the pnpm script) per the harness fact. All runs foreground. No background/monitor patterns. The `.focus()`/Tab harness caveat did not apply - the test drives a real mouse `click()` on `editor-rule-select` to open the panel.

---

## Diff scope and surfaced items

- **Exactly two files changed** (`git status --porcelain` clean post-commit; `dist/` gitignored): `src/views/EditorView.vue` (+7/-1), `e2e/editor-markers.spec.ts` (+22/-5) - 29 insertions, 6 deletions total (`git show --numstat`). No `.ftl`, no other file, no new test infra, no weakened/mutated existing assertion (fixture #17 is additive; the two `toHaveCount` bumps are the computable delta of the additive extension).
- **New pattern established:** none. The fix reuses the existing `suppressSelfAnchor` prop and the existing keywordOrBlock suppression pattern verbatim. Covered by the standing structural-conformance grant (zero outward surface, additive pattern-conforming test extension).
- **Deviation from plan premises:** none. Line anchors matched (detail-panel `SectionWidget` at `EditorView.vue:602`, `selectedPath` at `:373-375`, both within the verdict's cited ranges).
- **Commit:** unsigned via `git -c commit.gpgsign=false`, explicit `git add` of the two files only (no `-A`), trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` per the plan's Global Constraints and the branch's existing commits.

## Concerns

None.
