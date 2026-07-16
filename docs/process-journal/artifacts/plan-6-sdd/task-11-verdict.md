# Verdict round 1 (extracted from the reviewer transcript)

Vue 3.5.39, well within `defineModel` support (stable since 3.4). All checks complete. Compiling the final review.

## Verdict

**Task quality: Needs fixes** (one Important house-pattern gap; core spec compliance, drag-reorder correctness, and mount purity all hold).

### Spec Compliance
- ✅ Only the two files touched (`e2e/smoke.spec.ts`, `src/views/EditorView.vue`) — confirmed via diff stat (188 insertions, 2 files, 0 deletions elsewhere).
- ✅ Reorder emits the model, not a DOM mutation: `EditorView.vue:83-86` builds a new `Profile` object and assigns `model.value = {...}`, which `defineModel` (Vue 3.5.39, macro stable since 3.4) auto-emits as `update:modelValue`.
- ✅ Zero frontend semantic validation: no uniqueness/type/required checks anywhere in the file.
- ✅ Scaffold stays reviewable as one unit: no `App.vue` touch, no IPC import, no save/open, no nav wiring.
- ✅ Mount-purity constraint (Q3): confirmed pure, see below.
- ✅ 43-key invariant: `grep -cE '^[a-zA-Z0-9_-]+ = '` gives 43 for both `locales/en/gui-editor.ftl` and `locales/de/gui-editor.ftl`; `git diff c50866a af9ebc3 -- locales/` is empty.
- ✅ All five `$t()` calls resolve to pre-existing keys (`editor-tracks-rules:68`, `editor-track-rule-source:46`, `-match-expr:47`, `-optional:48`, `-changes:49` in `locales/en/gui-editor.ftl`).
- ⚠️ House view conventions: matches `BatchView.vue`'s root-section/`data-testid`/`defineModel` shape (the `defineModel`-for-a-view pattern is independently established in `JobsView.vue:11` for `runActive`, not novel here) — but see Important finding below on the `<table>` shape specifically.

### Adjudications

**Q1 (row-summary formatting):**
(a) Sanctioned. Walked `sourceSummary`/`matchSummary`/`changesSummary` (`EditorView.vue:34-67`) line by line: every emitted component is either a profile-format keyword (`SOURCE_KEYWORDS[0]` = `"primary"`, verified in `src/bindings/keywords.ts:3`), a raw model value (external path, scalar), a schema property name (`Object.entries(expr.exact/substring/regex)`'s `key`, and `MatchExpr`'s own `any`/`not` field names, verified against `src/bindings/profile.ts:157-182`), or code-authored punctuation (`=`, `~`, `(N)`, `, `). No English UI prose is embedded. The `ResolutionTable.vue:23-24` `resolvedTrackLabel` precedent the report cites is real, verified verbatim. The i18n-08 ledger rule (`docs/decision-ledger.yaml:1840`, "workarounds may relocate only passthrough/ASCII non-prose data, never real user-facing copy") is respected.
(b) Blind, not confirmatory. `eslint.config.js:61-68` scans template text nodes plus four static attributes (`title`/`aria-label`/`placeholder`/`alt`); the row cells are `{{ functionCall(rule) }}` expression interpolations, which the rule's AST-based scan cannot inspect for runtime string content. A lint pass proves no *literal* text node was skipped, not that the interpolated output is prose-free — that's a judgment call the reviewer (or implementer) has to make by reading the string-building code, which is what (a) does.

**Q2 (drag test integrity):** Falsifiable, confirmed non-vacuous. `e2e/smoke.spec.ts:786-822` asserts both the swapped row order (`toContainText`) and `readModel(page).tracks.rules[].match.exact?.type` against `["audio", "video"]` — a genuine swap from the fixture's `["video", "audio"]`, not a no-op-compatible expectation. Traced `onDrop` (`EditorView.vue:75-88`): a no-op, duplicate, or off-by-one reorder bug would leave either the DOM or the model (or both) at the pre-swap/malformed state and fail. The shared inert `DataTransfer` handle is exactly Playwright's documented programmatic-DnD pattern; harmless even though this component's handlers never read it.

**Q3 (mount purity):** Confirmed pure. `EditorView.vue` imports only `computed` (vue), `Profile`/`TrackRule` types, and `SOURCE_KEYWORDS`. No `onMounted`, no `../ipc` import, no `load_profile` call anywhere in the file. `defineModel<Profile>()` is the sole state source.

### Strengths
- Reorder correctness: immutable rebuild (`splice`-out/`splice`-in on a copy, `EditorView.vue:80-86`) with a correct closure-based `dragIndex`, matching `ListWidget.vue`'s established mechanics exactly, including the `dragover.prevent` needed for real (non-synthetic) browser drops to work.
- Type-safe narrowing of `SourceCfg = ExternalBlock | string` in `sourceSummary` (`:34-42`).
- Candid, accurate self-review: the report's two flagged "Concerns" (row-summary format likely superseded by Task 12; inert `DataTransfer`) both check out under adjudication and are appropriately scoped as FYI, not defects.
- Commit discipline: exactly the two files the brief names staged, unsigned, message accurate against the diff.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
- **Missing `<caption>` on the rule grid `<table>` (`EditorView.vue:94`).** Every other `<table>` in the codebase carries a `<caption>` with a dedicated `$t()` accessible name — `ResolutionTable.vue:34`, `JobsView.vue:270`, `RunHistory.vue:193`, 3-for-3, no exceptions. `EditorView.vue`'s table has none, and the preceding `<h2>` is not wired via `aria-labelledby` either, so the table has no accessible name for a user landing on it directly (e.g. via screen-reader table navigation). This is a genuine, avoidable house-pattern deviation, not a plan-mandated one: the fix costs zero new keys (`<caption>{{ $t("editor-tracks-rules") }}</caption>`, reusing the exact key already on the `<h2>`), so it doesn't conflict with the "existing keys only" / "43-key budget" constraint. Not codified in the Tier-2 docs (`docs/conventions.yaml`/`product-boundaries.yaml`/`process-conventions.yaml` — grepped for "caption", no hits), so this is an emergent, unpromoted pattern rather than a written rule; see HARVEST.

#### Minor (Nice to Have)
- **`e2e/smoke.spec.ts:817`**: `const model = (await readModel(page)) as Profile;` reads the model directly, unlike every one of the file's ten other `readModel` call sites (e.g. `:616,627,638,649,662,678,693,710,721`), all of which wrap it in `expect.poll(() => readModel(page))`. Verified this specific case is not actually racy: `e2e/mount-entry.ts:55-56` exposes `__muxsmithModel__` as a closure over the live Vue `ref`, updated synchronously inside the `onUpdate:modelValue` handler the instant `EditorView`'s `onDrop` assigns `model.value`, and the two preceding `toContainText` assertions (`:814-815`) already force a wait for the DOM to reflect that same state change before the direct read happens. So no flakiness risk in practice, but it's a one-line, zero-cost fix to restore consistency with the file's otherwise-unbroken pattern (defensive against a future edit that reorders or removes the preceding DOM assertions).

### HARVEST
- **Candidate pattern for `docs/conventions.yaml` promotion**: "Every `<table>` gets a `<caption>` with a dedicated Fluent key naming what the table lists." Agent-emergent, technical-code nature, already at 3 occurrences pre-Task-11 (`ResolutionTable.vue`, `JobsView.vue`, `RunHistory.vue`) without ever being promoted or tracked as a ledger entry — the recurrence existed but was never written down, which is exactly the gap that let Task 11 miss it. Per the source × nature matrix, agent-emergent × technical-code promotes at count 3; it already qualified before this task and should be promoted now, with Task 11's miss recorded as a `violated` occurrence once fixed.
- No-work-needed check performed: confirmed (not merely trusted) that `gui-editor.ftl` was untouched by the commit via `git diff c50866a af9ebc3 -- locales/` (empty output) and an independent key recount, rather than accepting the report's "confirmed via git diff" claim at face value.

---

# Re-review after the fix round

**Task quality: Approved**

- Important (missing `<caption>`): resolved — `EditorView.vue:95-97` adds `<caption>{{ $t("editor-tracks-rules") }}</caption>` as the table's first child, reusing the existing key, zero new catalog keys (confirmed no CSS anywhere hides captions, so it renders visible like all three precedents).
- Minor (`readModel` without `expect.poll`): resolved — `e2e/smoke.spec.ts:813-818` now wraps the read in `expect.poll(async () => {...}).toEqual(...)`, eliminating the bare direct read; the async-callback shape (needed for the `.map()` transform) is a legitimate, Playwright-documented extension of the same wrap-in-poll pattern, not a new deviation.
- Nothing beyond the two fixes changed: confirmed — `git show 35d844d --stat` shows exactly the two files/hunks in the delta diff, no other lines touched.
