# Task 5 (D55) verdict - independent SDD review

**VERDICT: APPROVED**

Commit `3fab82f` on `plan7-b` (parent `d194588` = approved Task 4). D55 attribute
migration: 27 sibling ids folded + 1 renamed across 8 catalogs (en+de), 26 render
sites re-pointed to `$ta`, plus the controller-ruled `parseOrThrow` guard correction.
17 files, all gate items green. No fixes required.

Every load-bearing claim below was re-derived from the tree, not borrowed from the
commit message or the plan's self-report. Absence checks were fire-verified against a
control.

---

## Findings (all pass)

### 1. Migration completeness against the D55 table - COMPLETE, verbatim
- All 16 tooltip base rows + the 4 `batch-run` state variants + the baseless rename +
  all 6 hint folds are applied in **both** en and de, one commit. Cross-checked the
  full `git show` diff row by row against the design table (design :675-736).
- **Values verbatim**: the parent is `d194588`, so every `-` line in the diff *is* the
  pre-migration value. Spot-diffed >8 rows (settings-open, browse-button `.tooltip` +
  cross-file `.tooltip-directory`, firstrun-retry, batch-run all 5 variants,
  jobs-history-save, settings-locale-hint de, batch-recents-select de) - each new
  attribute value byte-equals the deleted sibling's value. No wording drift.
- **Rename correct**: `batch-recents-select-tooltip` -> value-less `batch-recents-select`
  with `.tooltip`, exactly the design shape (:702-704), en+de.
- **Cross-file move verbatim**: `batch-browse-dir-tooltip` (gui-batch) -> `browse-button`
  `.tooltip-directory` (gui-common). en "Choose the directory with a folder picker." /
  de "Das Verzeichnis über einen Ordnerdialog auswählen." moved unchanged; the source id
  deleted from gui-batch.
- **Four batch-run variants** -> `.tooltip-no-profile`, `.tooltip-errors`,
  `.tooltip-mkvmerge-missing`, `.tooltip-run-active`. Correct.
- **No id outside the table touched**: diff confirms only enumerated ids changed. The
  `close-abort-*` hard constraint holds - 4 keys each locale, single-line valueful
  attribute-free, **untouched by this commit** (`git show | grep close-abort` -> none).
- **Closed attribute-name set**: the only distinct attribute names across all 8 migrated
  catalogs are `.hint .tooltip .tooltip-directory .tooltip-errors .tooltip-mkvmerge-missing
  .tooltip-no-profile .tooltip-run-active` - all design-enumerated; nothing outside
  {tooltip, hint, tooltip-<state>} + the design's own `tooltip-directory`.
- Counts recompute exactly: **gui-common 36/36, gui-settings 8/8, gui-batch 27/27,
  gui-jobs 41/41**. Attribute tally 28 (6+12+5+5), matching the enumeration.

### 2. Render sites - all 26 re-pointed, access idiom correct
- 20 `:title` + 6 hint = 26, every one re-pointed. Counted from the diff: App 1, JobRow 1,
  RunHistory 3, SettingsDialog 6 (3 tooltip + 3 hint), SuggestionCard 2, BatchView 7
  explicit + `batch-run` via the `runTooltip` computed, FirstRun 4, JobsView 1 = 26.
- The `batch-run` button still binds `:title="runTooltip"` (`BatchView.vue:511`,
  `data-testid="batch-run":508`) and `const fluent = useFluent()` is present (`:38`) - the
  run-gate site rides the rewritten computed, correctly absent from the `:title` diff.
- **Dot vs bracket consistent with the ecosystem/house rule**: dot for valid-identifier
  attrs (`$ta(id).tooltip`, `.hint`), bracket where required - `$ta('browse-button')['tooltip-directory']`
  (hyphen is not a JS identifier) and `fluent.$ta("batch-run")[runDisabledReason.value ?? "tooltip"]`
  (dynamic key). Correct, not a stylistic mix.
- `runDisabledReason`/`runTooltip` rewritten **exactly** per the plan block (:632-651):
  bare attribute names returned, null falls back to base `tooltip`.
- **No orphaned `$t('...-tooltip'|'...-hint')` anywhere** in src/e2e (`--exclude-dir=.generated`).
  Pattern fire-verified against a control line (`:title="$t('settings-open-tooltip')"`) -> the
  regex fires (exit 0), so the empty result is a verified negative, not a malformed grep.

### 3. Completion check (plan Step-3 scoped grep) - matches the authored prediction
- Scoped grep **with** the `grep -v` filter: **empty** (exit 1). Clean.
- **Without** the filter: exactly the **six** enumerated legitimate survivors, all in
  `SettingsDialog.vue` - `settings-mkvmerge-path-hint`, `settings-default-jobs-hint`,
  `settings-locale-hint`, each once as `id="..."` and once as `aria-describedby="..."`.
  Nothing else. FirstRun/BatchView use different DOM ids (`firstrun-path-hint`,
  `batch-source-dir-hint`, `batch-output-dir-hint`) that do not collide with catalog names,
  so only SettingsDialog's three coincide - exactly the plan's member-by-member reachability
  demonstration (:653-662).

### 4. The ruled guard correction (controller Option A, 2026-07-22) - EXACT and verified
- Discriminator is `!message || (message.value == null && Object.keys(message.attributes).length === 0)`
  (`i18n-en.ts:103-106`) - byte-for-byte the ruling's `!m || (value null && zero attributes)`.
- **Three-way verification re-run by me** (ran the *real* `assertAllCatalogsParseCleanly`
  via node type-stripping; catalog mutations restored byte-identically with `command cp -f`
  + `cmp`, git clean confirmed after each):
  - **(a) green reachable**: clean tree -> PASS; full `pnpm test:e2e` **32 passed incl.
    `catalogs.spec.ts`** - the value-less `batch-recents-select` (value null, attrs `[tooltip]`)
    is not flagged.
  - **(b) truly-dropped entry still fires**: bare-id mutation (`batch-view-heading =`) ->
    guard THROWS via the `!message` branch (getMessage undefined).
  - **(c) empty `.tooltip =` still fires**: mutation -> THROW naming `batch-recents-select`,
    "produced no message value for it", via `!message` (empty attribute makes the whole
    message Junk). `addResource` stays silent (0 errors) on all Junk cases - so only the
    `droppedIds` filter catches them, which is the guard's whole reason to exist.
- **Doc touch-ups honest**: both the module doc (:21-24) and the `parseOrThrow` doc (:84-90)
  now say "a value OR at least one attribute", explicitly naming D55's value-less shape as a
  valid, fully-captured case. No overclaim.
- Commit body's ruled-correction clause is accurate and cites "Controller-ruled Option A on
  the stream-B NEEDS_CONTEXT".

### 5. Quality
- **Gate**: `pnpm lint` exit 0, `pnpm build` (vue-tsc typecheck) clean - proving the `$ta`
  typings hold incl. bracket access, `pnpm check:i18n` exit 0 (17 pre-existing dynamic-key
  warnings, not failures; parity ok), `pnpm test:e2e` 32/32.
- **Commit discipline**: 17 files staged explicitly (8 catalogs + 8 src components +
  `e2e/i18n-en.ts`), trailer present. `e2e/smoke.spec.ts` correctly **not** touched - it holds
  no folded-id assertions (the e2e orphan sweep is empty), so the plan's conditional
  "only if assertions there name folded ids" resolves to no-change; 18 planned - 1 conditional
  drop = 17, matching the addendum's "17 files total".

---

## Adjudications - the three comment-level items (structural-conformance grant, doctrine §7)

1. **gui-batch en header rewrite** (`locales/en/gui-batch.ftl:1-6`): the browse-button-reuse
   comment was rewritten from "`batch-browse-dir-tooltip` below" to "`browse-button`'s
   `.tooltip-directory` attribute (gui-common.ftl)". **LEGITIMATE** - the old text named a
   now-deleted id; leaving it would be a stale dangling reference. de gui-batch carries only a
   generic translation header (no such detail), so nothing stale there. Verdict: correct,
   forced by the migration.

2. **D23 comment relocation** (`locales/en/gui-batch.ftl`): the D23 comment moved from
   *between* `batch-run`'s (former) sibling ids to *above* the `batch-run` message.
   **EMPIRICALLY VERIFIED NECESSARY**: I parsed both shapes through the installed
   `@fluent/bundle` 0.19.1. A `#` comment sitting between attribute lines **silently
   terminates the message** - `batch-run` came back with attrs `[tooltip, tooltip-mkvmerge-missing]`,
   the trailing `.tooltip-run-active` **dropped**, and `addResource` reported **zero errors**.
   The relocated (comment-above) form returns all three attrs. So the relocation is not
   cosmetic - keeping the comment inline would have silently deleted a run-gate tooltip that
   `runDisabledReason` depends on. Verdict: correct and required.

3. **`runDisabledReason` doc update** (`BatchView.vue:281-`): rewritten to describe the
   `batch-run` attribute-name return and `$ta("batch-run")[name]` rendering with base-`tooltip`
   fallback. **LEGITIMATE** - honest reflection of the changed return contract; the old doc
   described the removed sibling-id scheme. Verdict: correct.

All three are structure-forced consequences of the fold, not scope creep. Under the doctrine §7
structural-conformance grant they are in-bounds; none touches user-facing wording (owner
surface pass) or design latitude.

---

## HARVEST

- **Guard-discriminator = a green-state-reachability defect
  (`proc-check-green-state-reachable`).** The pre-plan-7 `parseOrThrow` discriminator
  (`?.value == null`) had **no reachable green state** for the shape D55 *mandates*: a
  value-less message carrying attributes always has `value == null`, so the guard would
  false-positive on `batch-recents-select` the instant D55 landed. This is a second concrete
  instance of the class the house record already carries (a check must have a reachable
  passing state given the intended end state) - here the check *predated* the design shape
  that made its green state unreachable. It was handled correctly: surfaced on code contact
  as NEEDS_CONTEXT, controller-ruled (Option A), **not decided at the keyboard**. Harvest hook:
  when a design introduces a new-but-valid artifact shape, sweep pre-existing verification
  guards for green-state reachability against that shape *before* relying on them - the guard
  that was right yesterday can be structurally unable to pass today. Pair it with
  `proc-verification-step-must-be-falsifiable`: this guard now needs both a green control
  (value-less-with-attr passes) and a red control (Junk drop fires) - the review ran both.

- **Fluent fact worth a reference note (non-obvious, silent):** a `#` comment placed
  *between* a message's attribute lines terminates the message and **silently drops** every
  following attribute; `addResource` returns zero errors. Comments must precede a
  multi-attribute message, never sit inside its attribute block. Empirically confirmed against
  `@fluent/bundle` 0.19.1. This is exactly why check-i18n/`parseOrThrow` is line-based *and*
  cross-checks the real parser: `addResource`'s silence on Junk is the gap both close.

- **Over-restriction watch - clean, and a positive model.** No over-restriction found. The
  completion grep deliberately enumerates the 24 concrete names instead of a
  `-tooltip`/`-hint` suffix pattern (which would false-positive on DOM ids like
  `firstrun-path-hint`), and pairs it with a scoped `grep -v id=|aria-describedby=` that
  exempts the six legitimate DOM-attribute survivors. That is the correct calibration of the
  house rule "enumerate concrete targets, exempt legitimate cases" - a suffix ban would have
  killed wanted usage. `tooltip-directory` (a design-enumerated attribute, not a batch-run
  "state") is correctly treated as in-set rather than flagged as an unauthorized name.

- **Minor nit (not a finding, no fix owed):** the `parseOrThrow` *throw-message* hint still
  reads "a malformed multiline continuation or nested-selector indent, most likely" - it was
  not extended to mention the value-less-and-attribute-less / empty-attribute drop causes,
  though the two doc-comment blocks were. It is a "most likely" heuristic string, not a false
  claim, and not decision-relevant. Flagged only for the whole-branch pass to consider.

- **Idiom note:** hyphenated Fluent attribute names force bracket access in TS
  (`$ta(id)['tooltip-directory']`); dot access is a syntax error there. The implementer split
  dot/bracket correctly per name; worth remembering when the closed attribute set grows a
  hyphenated member.
