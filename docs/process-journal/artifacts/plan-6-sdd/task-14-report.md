# Task 14 report: D43 - one-click apply in the batch view

**Final status: DONE.** Resolved and committed as `bf46932` on `plan6-e`.
The NEEDS_CONTEXT below was routed; the controller's amendment 4 (commit
1f16a8e, four-eyes delta review APPROVED) confirmed the premise-refutation
in my decision memo (DiagnosticsPanel.vue is documentation-only, not in the
apply chain) and corrected the wiring itself: `SuggestionCard` emits, it
does not orchestrate; `BatchView` (not the card) owns the round trip
because it owns the picked profile FILE path. Controller review also found
a real defect in my original draft that the amendment's correction
happens to fix as a side effect: my draft's `apply()` called
`loadProfile(props.suggestion.config_path)` / `saveProfile(props.suggestion.
config_path, ...)`, treating `config_path` (a config-field LOCATOR,
`tracks[<N>].match`) as a file path. It would have failed at runtime; my
own echo-mock test fixture hid the bug because it happened to set the
suggestion's `config_path` equal to the profile path, so the wrong value
and the right value were indistinguishable. See "Refit (amendment 4)"
below for the corrected implementation, the hardened test, and gate
evidence. **The original decision memo is kept below verbatim** as the
record of what was routed and why; it is superseded by the refit section
where the two disagree, not edited in place.

---

## Original decision memo (superseded by the refit below; kept verbatim)

**Status at the time: NEEDS_CONTEXT.** Implementation is complete, tested (TDD RED/GREEN
verified) and the full nine-part gate is green, but it is **held uncommitted**
pending confirmation on one point where the brief's Step-3 instruction
("wire the emit through `DiagnosticsPanel.vue`") appears to conflict with
the brief's own exhaustive Files list and with several other explicit,
verifiable constraints in the design corpus. Per Global Constraints
(`docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md:15`) and this
task's own binding instructions, a fork discovered on code contact is routed
to the controller **before** being resolved, not decided at the keyboard.
I have a strong, evidenced candidate resolution (below) and implemented it
so the controller can evaluate a concrete artifact rather than an abstract
question, but I am not committing until this is confirmed.

## The fork

Brief Step 3: "Wire the apply button through `SuggestionCard.vue` ->
`DiagnosticsPanel.vue` -> the `apply_suggestion` command." Taken literally,
this describes SuggestionCard's click emitting an event that DiagnosticsPanel
forwards ("wire the emit through") to some further consumer.

I could not find an implementation of that literal reading that survives
contact with the rest of the brief:

- **The Files list is stated exhaustive**: `SuggestionCard.vue`,
  `DiagnosticsPanel.vue`, `locales/{en,de}/gui-batch.ftl`, `e2e/smoke.spec.ts`.
  `BatchView.vue` is the only component that currently instantiates both
  `SuggestionCard` and `DiagnosticsPanel`, and today they are **siblings**,
  not parent/child (`BatchView.vue:405,429`: `<DiagnosticsPanel
  :diagnostics="generalDiagnostics" />` ... `<SuggestionCard v-for="(s, i)
  in report.suggestions" ... />`). Making SuggestionCard a child of
  DiagnosticsPanel (the only reading under which an emit would need
  "wiring through" DiagnosticsPanel to reach anything) requires a
  `BatchView.vue` template change to thread a new prop/slot - contradicting
  Files-list exhaustiveness.
- **The key budget is exact and names only SuggestionCard.** Design
  `:1779` / plan `:1601-1607`: "D43 | apply button label + tooltip |
  `gui-batch.ftl` | 2", "beside `SuggestionCard.vue`'s current copy-button
  keys ... because the apply control lives in the batch view." One button,
  one owner, already accounted for without DiagnosticsPanel.
- **DiagnosticsPanel's no-fix rendering is declared unchanged**: "the
  diagnostics panel renders it [the no-fix/partition case] as it does
  today" (brief, binding points). `core-109-two-required-no-fix`
  (`docs/conventions.yaml:741`) records the no-fix case never produces a
  `Suggestion` at all, so there is nothing for a per-diagnostic apply
  control to attach to even if one were built.
- **DiagnosticsPanel's own doc comment is a genericity contract**: "Reused
  for the batch/config-level diagnostics (BatchView) and, once per file,
  for `ReportFile.diagnostics` (ResolutionTable): same shape, same
  rendering, no per-caller variant." A `Suggestion`-typed prop threaded in
  for one caller's use case breaks that contract for the other two
  callers (`ResolutionTable.vue`, `EditorView.vue`), neither of which is
  in the Files list either.
- **D49 rules out DiagnosticsPanel as the error-rendering path** I also
  considered: "`diagnosticFluentParams` (`DiagnosticsPanel.vue:34`)...is
  keyed by diagnostic code...so an `IpcError` code never reaches it" - and
  names the actually-established pattern instead: "`IpcError.params`
  reaches Fluent as `Record<string, string>` at every call site
  (`RunHistory.vue:155`, `:241`, `JobsView.vue:246`, `:252`,
  `FirstRun.vue:94`), which pass `error.params` straight into `$t`."
  That is the pattern I used directly inside `SuggestionCard.vue` instead.

Given these, my candidate resolution treats "wire the emit through
`DiagnosticsPanel.vue`" as most likely describing an earlier-drafted
intent that did not survive contact with how Tasks 9-13 actually built the
component tree (the same kind of drift the brief's own D22-comment-update
instruction asks this task to correct elsewhere) - and I made the file's
"Modify" instruction real via a documentation change: a comment recording,
at the point a future reader would look for it, that Task 14 deliberately
does not add per-diagnostic apply wiring here, why not, and where the
control actually lives instead. Zero behavior change, zero new mechanism,
zero risk to the "no per-caller variant" contract.

**What I am asking:** confirm this reading, or, if real functional wiring
into `DiagnosticsPanel.vue` was intended, tell me what it should consume
and from where - since every concrete version I could construct needed a
`BatchView.vue` (or `App.vue`) change the brief withholds, that would
itself need routing as its own fork.

## What I implemented

### `SuggestionCard.vue`

- Updated the stale D22 comment (`:6-13` originally) per the brief: it
  falsely claimed `edit` "is deliberately never read" and suggestions are
  "never applied." The new comment records that D41 supersedes D22's
  stated reason (comment-preserving YAML mutation is dead machinery -
  D41 settled that saving always rewrites canonically and never preserves
  comments) and that the editor+apply pairing survives on D41's actual,
  stronger reason: shared model ownership - here realized as this card's
  own one-shot load/apply/save round trip rather than a live shared Vue
  ref with the editor (there is none: `App.vue:133` mounts `EditorView`
  with no `v-model`, confirmed deliberate by Task 13's own doc comment,
  "no v-model, no shared editor state there").
- Added `apply()`: `loadProfile(config_path)` -> guard on `doc.profile ===
  null` (a `ParseError` since the suggestion was computed; `load_profile`
  folds that into `config_diagnostics[0]` rather than throwing, per D42 -
  reused directly as the error to show, no bespoke fallback code) ->
  `applySuggestion(profile, config_path, edit as StructuredEdit)` ->
  `saveProfile(config_path, updated)`. `config_path`/`edit` are forwarded
  exactly as received, never parsed or interpreted (D43's binding point).
  IPC failures at any step populate `ipcErrorCode`/`ipcErrorParams`,
  rendered via the same `<p role="alert">{{ $t(ipcErrorCode,
  ipcErrorParams) }}</p>` idiom `BatchView.vue`/`EditorView.vue` already
  use (no new keys needed - the three `ApplyError` codes and two
  `SaveError` codes already exist in `gui-common.ftl` from Task 8).
- Added the apply button: `data-testid="batch-suggestion-apply"`,
  `:disabled="applying"`, `:aria-busy="applying"` (mirrors
  `BatchView.vue`'s `:aria-busy="dryRunning"` / `EditorView.vue`'s
  `:aria-busy="saving"` idiom rather than inventing a third confirmation
  string - the 2-key budget has no room for one, and none of the other
  fire-and-forget buttons in this app show a transient "done" text
  either).

### `DiagnosticsPanel.vue`

Documentation-only addition (see "The fork" above): records that Task 14's
apply control lives entirely in `SuggestionCard.vue`, that the no-fix/
partition diagnostic renders here unchanged, and why a per-diagnostic
apply affordance was not added (would need a second, caller-specific prop,
breaking the component's own no-per-caller-variant contract for its other
two callers). No template, prop, or emit change.

### `locales/en/gui-batch.ftl` / `locales/de/gui-batch.ftl`

Two new keys beside the copy-button pair, plus fixed the header comment's
own stale "D22: ... never applied" claim (same root fact the brief's D22
comment fix addresses, in the same file I was already touching):

```ftl
# en
batch-suggestion-apply = Apply
batch-suggestion-apply-tooltip = Apply this fix to the profile and save it.

# de
batch-suggestion-apply = Anwenden
batch-suggestion-apply-tooltip = Diese Korrektur auf das Profil anwenden und speichern.
```

**Flagged for the owner's plan-close rendered-surface pass** (per this
task's constraints - the brief does not fix exact wording): both strings
follow the sibling `batch-suggestion-copy`/`-tooltip` pair's register
(imperative label, infinitive-ending tooltip sentence) and the de header's
own stated convention (du-imperative infinitive form), but the exact
wording is my proposal, not brief-mandated.

### `e2e/smoke.spec.ts`

One new test in the existing `"batch view: dry run"` describe block: a
`dry_run` report carries one suggestion with a real `StructuredEdit`
(`{ kind: "add_exact", property: "codec_kind", value: "srt" }` - unlike
the pre-existing copy-test fixture's `edit: null`, which stays untouched
and is not this task's concern) plus one `suggestion-partition`
(no-fix/partition) diagnostic. Asserts:

- the suggestion card's apply button is visible;
- the SAME selector (`getByRole("button", name("batch-suggestion-apply"))`)
  scoped to the diagnostics region has count 0 - the paired-control
  template (falsifiability occurrence 5, `docs/process-conventions.yaml`
  amendment 779376c) so the negative cannot pass vacuously;
- clicking invokes `apply_suggestion` exactly once with `configPath` and
  `edit` echoed unmodified (`toEqual`, not `toBe` - IPC args cross a JSON
  boundary);
- `save_profile` is invoked once with the same path.

Extended `mocks.ts`'s existing generic `commands: {...}` scenario map
(`load_profile`, `apply_suggestion`, `save_profile`) - no change to
`e2e/mocks.ts` itself was needed; its command-response mechanism is
already fully generic (confirmed by reading it - `load_profile`/
`save_profile` mocking is already an established pattern from Task 13's
editor open/save test at `:1018-1075`).

## TDD evidence

1. Wrote the e2e test with the implementation already staged, then
   verified RED by `git stash push` on the four implementation files
   (keeping only the test) and running the new test in isolation:
   ```
   Error: e2e/i18n-en: no message "batch-suggestion-apply" in the en catalog
   ```
   (fails upstream of "no apply button," for the correct underlying
   reason - the feature does not exist yet.)
2. `git stash pop` restored the implementation.
3. Full `pnpm test:e2e` (27 tests, including the new one): **27 passed**.

## Gate results (all nine, foreground, no subsets)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| 3 | `cargo test --workspace` | PASS (81 passed) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | PASS |
| 5 | `cargo deny check` | PASS (advisories/bans/licenses/sources ok) |
| 6 | `pnpm lint` | PASS |
| 7 | `pnpm build` | PASS (`vue-tsc --noEmit && vite build`) |
| 8 | `pnpm check:i18n` | PASS (233 catalog ids, only pre-existing unused-key warnings, unaffected by this change) |
| 9 | `pnpm test:e2e` | PASS (27/27) |

No Rust files were touched by this task; the first five items are
whole-workspace regression checks, run per "no subsets."

## Files changed (exactly the brief's list, verified via `git status`)

- `src/components/SuggestionCard.vue`
- `src/components/DiagnosticsPanel.vue`
- `locales/en/gui-batch.ftl`
- `locales/de/gui-batch.ftl`
- `e2e/smoke.spec.ts`

No `BatchView.vue`, `EditorView.vue`, `App.vue`, or `e2e/mocks.ts` changes.

## Self-review

- `config_path`/`edit` forwarded untouched: yes, echo-asserted via
  `toEqual(APPLY_EDIT)` and `toBe(PROFILE_PATH)` against the recorded
  `apply_suggestion` invocation, not a UI echo.
- No-fix case buttonless with paired control: yes, same-selector
  presence/absence pair in one test.
- D22 comment updated with the D41 supersession: yes, in
  `SuggestionCard.vue`.
- Keys bilingual: yes, both `.ftl` files, same commit-to-be.
- `gui-editor.ftl` untouched: confirmed (`git status` shows no change to
  it; still 45 keys).
- Nothing user-visible beyond the brief's mandated two keys: confirmed -
  the DiagnosticsPanel comment is not rendered, and I also fixed the
  gui-batch.ftl file header's own stale "never applied" line (a doc
  comment, no key, no rendered text) since it restates the exact same
  fact the brief's D22-comment instruction targets, in a file I was
  already modifying.

## Concerns

1. **The DiagnosticsPanel.vue fork above - the reason for NEEDS_CONTEXT.**
   Everything else in this report is submitted with high confidence; this
   is the one point I want confirmed (or corrected) before committing.
2. **Apply-vs-editor concurrency is a real, if narrow, edge case, not
   solved here and not flagged elsewhere in the plan's open-items list.**
   `App.vue` keeps all views mounted (`v-show`, not `v-if`) and
   `EditorView`'s own `currentPath`/`model` survive a tab switch. If a
   user has the same profile open in the Editor tab with unsaved edits,
   then applies a suggestion for that same file from the Batch tab, the
   self-contained load/apply/save here writes to disk independently of
   the editor's in-memory state; a subsequent Editor Save would overwrite
   the just-applied fix with the editor's (now stale-relative-to-disk)
   model. This is exactly the hazard design D41 names as the reason apply
   "must go through the same model the editor holds" - a property the
   as-built app (Tasks 9-13, independent views, no shared v-model per
   Task 13's own doc comment) does not actually provide for any caller,
   editor included. Worth a whole-branch-review agenda item; out of this
   task's Files-list-bounded scope to fix.
3. Per this task's constraints, the two new key strings' wording is my
   proposal, not brief-fixed - flagged above for the owner's rendered-
   surface pass.

---

## Refit (amendment 4)

The coordinator's routed response confirmed the DiagnosticsPanel.vue
premise-refutation above and delivered the amended, re-extracted brief
(`task-14-brief.md`, "amended 2026-07-16, apply-wiring routing"). Two
merge commits (`4a8916f` harvest, `1f16a8e` amendment 4, `a2252bf`
integrate) landed on `plan6-e` before I resumed; both are docs-only
(`docs/superpowers/plans/...`, `docs/ROADMAP.md`, house files) - no
product or test file changed under me, confirmed by re-reading
`BatchView.vue`/`SuggestionCard.vue` against my original read before
editing further.

### The corrected wiring vs. my draft

| | My original draft | Amendment 4 (implemented) |
|---|---|---|
| Who calls `apply_suggestion` | `SuggestionCard.vue` itself | `BatchView.vue`, via a `SuggestionCard` `apply` emit |
| Path for `load_profile`/`save_profile` | `props.suggestion.config_path` (**wrong** - a locator, not a path) | `selectedProfile` (the picked profile file, `BatchView`'s own state) |
| `SuggestionCard`'s IPC imports | `loadProfile`, `applySuggestion`, `saveProfile`, `IpcError` | none - pure emit, no IPC |
| Error surface | a new per-card `ipcErrorCode`/`ipcErrorParams` pair | `BatchView`'s existing shared alert line (`:373-378` unchanged) |
| Busy state | per-card local `applying` ref | `BatchView`'s `applyingIndex`, folded into its existing `busy` computed, passed down as a prop |

The defect: `Suggestion.config_path` is a config-field **locator**
(`tracks[<N>].match`, parsed core-side by `rule_index_of`), never a file
path (design D43, `:480-486`, re-confirmed by the amended brief's binding
points). My draft called `loadProfile(props.suggestion.config_path)` and
`saveProfile(props.suggestion.config_path, updated)` - both would have
been Tauri IPC calls to open/write a file literally named `"tracks[0].
match"`, which does not exist. This would have failed at runtime against
the real backend. It passed my own e2e test only because my test fixture
(the earlier "batch view: dry run" describe block) set the suggestion's
`config_path` equal to `PROFILE_PATH`, so the wrong value and the
intended value were the same string - an echo-mock blindness the amended
brief's Step 1 explicitly targets ("The fixture must make the picked
profile path and the suggestion's `config_path` two different values so
a swap of locator and path cannot pass").

### Implementation changes

- **`SuggestionCard.vue`**: reverted to a pure render+emit component.
  Removed the `loadProfile`/`applySuggestion`/`saveProfile`/`IpcError`
  imports, the local `applying`/`ipcErrorCode`/`ipcErrorParams` refs, and
  the `apply()` orchestration function. Added `applying?: boolean` prop
  (parent-controlled busy reflection) and `defineEmits<{ apply:
  [payload: { config_path: string; edit: unknown }] }>()`. The click
  handler (`requestApply`) does one thing: `emit("apply", {
  config_path: props.suggestion.config_path, edit: props.suggestion.edit
  })` - both fields forwarded exactly as received. Removed the per-card
  error paragraph (error now lives in `BatchView`'s shared alert). The
  apply button markup (`data-testid`, `:aria-busy`) is unchanged from the
  original draft, now wired to `applying` (a prop) instead of a local
  ref, and to `requestApply` (an emit) instead of an async IPC call.
  Rewrote the D22 comment again to describe the emit, not a self-
  contained round trip.
- **`BatchView.vue`** (new to the Files list): added `loadProfile`,
  `applySuggestion`, `saveProfile` to the existing `../ipc` import and
  `StructuredEdit` from `../bindings/profile`. Added `applyingIndex =
  ref<number | null>(null)`, folded into `busy` alongside
  `validating`/`dryRunning`. Added `onApplySuggestion(payload, index)`:
  guards on `busy`, sets `applyingIndex`, `loadProfile(selectedProfile)`
  -> guards `doc.profile === null` (reusing `config_diagnostics[0]` as
  the error, same as the draft) -> `applySuggestion(doc.profile,
  payload.config_path, payload.edit as StructuredEdit)` ->
  `saveProfile(selectedProfile, updated)`, errors caught into the
  existing `ipcErrorCode`/`ipcErrorParams` refs, `finally` clears
  `applyingIndex`. Template: `<SuggestionCard ... :applying="applyingIndex
  === i" @apply="onApplySuggestion($event, i)" />`. Also fixed the same
  stale "No profile mutation anywhere in this view (D22...)" claim in
  this file's own header comment (Task 14 adds exactly one narrow
  mutation path) while already touching the file, for the same
  same-root-fact reason the D22 comment fix and the gui-batch.ftl header
  fix apply elsewhere.
- **`DiagnosticsPanel.vue`**: refit the comment's "entirely inside
  SuggestionCard.vue (a sibling...)" phrasing to the amended "renders and
  emits / parent handles" split, per the amended brief's explicit
  instruction ("refit its phrasing to the sibling-emits/parent-handles
  split"). Still documentation-only, still zero behavior change.
- **`locales/{en,de}/gui-batch.ftl`**: unchanged from the original draft
  (both new keys and the header-comment fix survive the refit untouched -
  the key budget and their content were never part of the fork).
- **`e2e/smoke.spec.ts`**: hardened per the amended brief's Step 1.
  `SUGGESTION_CONFIG_PATH = "tracks[0].match"` is now a distinct constant
  from `PROFILE_PATH = "/profiles/demo.yaml"` (previously equal - the bug
  that hid the defect). The no-fix diagnostic's own `config_path` was
  also moved off the shared locator string (`"tracks[1].match"`) to avoid
  any incidental collision with the suggestion's locator. New assertions,
  in both directions:
  - `load_profile` invoked once with `path === PROFILE_PATH`;
  - `apply_suggestion` invoked once with `configPath === SUGGESTION_CONFIG_PATH`
    **and** `configPath !== PROFILE_PATH` (explicit negative);
  - `save_profile` invoked once with `path === PROFILE_PATH` **and**
    `path !== SUGGESTION_CONFIG_PATH` (explicit negative);
  - `apply_suggestion`'s `profile` arg deep-equals the `load_profile`
    mock's returned model; `save_profile`'s `profile` arg deep-equals
    `apply_suggestion`'s mocked return value (the two profile fixtures,
    `loadedProfile`/`appliedProfile`, are now distinct objects so these
    checks are non-vacuous too).
  - The paired no-fix/apply-button-absence assertion from the original
    draft is unchanged.

### TDD evidence (refit)

1. `git stash push` on the six implementation files (kept only the
   hardened test) -> ran the hardened test in isolation -> RED:
   ```
   Error: e2e/i18n-en: no message "batch-suggestion-apply" in the en catalog
   ```
   (same class of RED as the original draft's RED - fails upstream,
   for the correct reason: the feature does not exist pre-refit.)
2. `git stash pop` restored the refit.
3. Full `pnpm test:e2e`: **27/27 passed**, including the hardened test.

### Gate results (refit, all nine, foreground, no subsets)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| 3 | `cargo test --workspace` | PASS (81 passed) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | PASS |
| 5 | `cargo deny check` | PASS |
| 6 | `pnpm lint` | PASS |
| 7 | `pnpm build` | PASS |
| 8 | `pnpm check:i18n` | PASS (233 catalog ids, same pre-existing unused-key warnings, unaffected) |
| 9 | `pnpm test:e2e` | PASS (27/27) |

No Rust files were touched by the refit either (BatchView.vue's new IPC
calls are all pre-existing `#[tauri::command]`s from Task 8); the cargo
items are whole-workspace regression checks, re-run in full after the
refit per "no subsets."

### Files changed (final commit `bf46932`)

Exactly the amended Files list: `src/components/SuggestionCard.vue`,
`src/views/BatchView.vue`, `src/components/DiagnosticsPanel.vue`,
`locales/en/gui-batch.ftl`, `locales/de/gui-batch.ftl`,
`e2e/smoke.spec.ts`. No `EditorView.vue`, `App.vue`, or `e2e/mocks.ts`
changes. `git diff --stat`: 6 files changed, 282 insertions(+), 16
deletions(-).

### Self-review (refit)

- `config_path`/`edit` forwarded untouched end-to-end (card -> BatchView
  -> `apply_suggestion`): yes, echo-asserted with explicit negatives in
  both directions against the recorded IPC call, not a UI echo.
- `config_path` never used as a load/save path, and the profile path
  never used as `apply_suggestion`'s locator: yes, both directions
  asserted explicitly (`.not.toBe(...)`), the exact class of bug this
  refit exists to close.
- No-fix case buttonless with paired control: yes, unchanged from the
  original draft, still on the same selector.
- D22 comment reflects the emit, not a self-contained round trip: yes.
- Keys bilingual, unchanged from the approved draft: yes.
- `gui-editor.ftl` untouched: confirmed, still 45 keys.
- `DiagnosticsPanel.vue` stays documentation-only, comment refit to the
  sibling-emits/parent-handles split per the amended brief's explicit
  instruction: yes.
- Nothing user-visible beyond the two budgeted keys: confirmed - the
  `applying` prop/emit plumbing and the two doc-comment fixes
  (`BatchView.vue`, `gui-batch.ftl` header) carry no rendered strings.

### Remaining concerns (unchanged in substance, now explicitly owned by the plan)

The amended brief's own "Out of scope, routed by the controller" section
now explicitly names both items my original draft flagged as ad hoc
concerns - apply-vs-editor concurrency and no-auto-refresh-after-apply -
as controller-routed ROADMAP candidates, not implementer TODOs. Nothing
further for me to flag beyond what the brief itself now records; the two
new key strings' exact wording remains my proposal (unchanged from the
original draft), still flagged for the owner's rendered-surface pass.

## Report file

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-6/task-14-report.md` (this file).
