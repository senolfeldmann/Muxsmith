# Task 3 review verdict: .tooltip attributes on all 42 editor label messages (en+de)

Reviewer: independent SDD task reviewer (fresh session). Commit under review: 923b049 on plan7-b (`/home/senol/Git/Muxsmith/.worktrees/plan7-b`); worktree clean at that commit, diff touches exactly `locales/en/gui-editor.ftl` and `locales/de/gui-editor.ftl`. Ground truth read: plan Task 3 + Global Constraints, design D53/D54/D55 (incl. D54's exclusion parentheticals and D55's closed attribute-name set), v1 spec 4.2-4.9 + 8.3, Tier-2 house files (`docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/decision-ledger.yaml` i18n family), planner/model source for load-bearing semantics. Implementer verdict was DONE_WITH_CONCERNS; the six surfaced items are adjudicated as Q1-Q6 below.

## VERDICT: APPROVED

All structural gates pass, the set is exact in both locales, and the content is accurate against spec 4.2-4.9. The findings below are owner-pass wording items and out-of-scope doc corrections; none blocks this task's artifact.

## Evidence run (all foreground, this session)

- **Set-diff (run independently):** 42 `labelKey` ids extracted from `src/editor/registries.ts` == the 42 ids carrying `.tooltip` in `locales/en/gui-editor.ftl` == the 42 in `locales/de/gui-editor.ftl`; three-way `diff` empty. Attribute-free ids per locale: exactly `editor-save-note`, `editor-action-add`, `editor-action-remove` (`comm -23` over full id set vs tooltip set, both locales). `editor-track-rule-order` correctly does not exist yet (arrives Task 16).
- **Id budget recount:** 45 message ids in each catalog both at `923b049^` and at `923b049` - the attributes added zero ids. Matches the design's catalog end-state table (design :1464): 45 now, 46 only after D59/Task 16 adds `editor-track-rule-order`. The 46-key end-state budget is untouched.
- **No valueless attributes:** `grep -E '^\s*\.tooltip *=\s*$'` empty in both files; pattern fire-verified against a scratch control containing a valueless `.tooltip =` (fires).
- **No braces/placeables in tooltip values:** brace grep empty; fire-verified against a scratch control containing `{ $brace }` (fires).
- **`pnpm check:i18n`:** ok (36 source files, 234 catalog ids, 17 pre-existing unused warnings - the dynamically rendered error-code ids, unrelated to this task; exit 0). Attributes invisible to the current parser, as the plan states for this stage.
- **Catalogs-parse e2e:** green both ways - plan's literal form `pnpm test:e2e -- --grep "catalogs parse"` (31/31 passed, full suite; see Q2) and the actually-filtering form `pnpm test:e2e --grep "catalogs parse"` (1/1: `all Fluent catalogs parse cleanly`).
- **Fire-verification reproduced** without touching the worktree: replicated `parseOrThrow`'s exact logic (`e2e/i18n-en.ts` - `addResource` errors + droppedIds check against `MESSAGE_ID_RE`-scanned ids) in a scratch script against a copy of the en catalog. Unmutated copy: CLEAN. Copy with valueless `.tooltip =` lines: THROW droppedIds (the parent messages junk away). The implementer's claim that a valueless `.tooltip` breaks the parse guard is reproducible.
- **Typography:** no smart quotes, en/em dashes, Unicode ellipsis, or NBSP in either catalog (grep -P scan, fire-verified against a known-bad control); de umlauts/ß present as real prose; straight quotes throughout.
- **Tier-2/ledger conformance:** `i18n-closed-attribute-name-set` - only `.tooltip` introduced, within D55's closed set (`tooltip`, `hint`, `tooltip-<state>`). `i18n-16` - both locales in the same commit. Config keywords literal in de (keep, drop, error, skip, overwrite, primary, clear, match_to_source, language/track_name/default_track) per the catalog header rule.

## Content spot-verification against spec 4.2-4.9 (16 of 42 verified in depth, incl. all tricky domains)

Accurate, keyword-correct, no id-echo, en/de semantically equivalent with de as real prose:

- `editor-input-pattern`: unanchored search on the basename, first match = identifier, capture groups become template fields - matches 4.2 exactly.
- `editor-input-extensions`: case-insensitive, validated against the local mkvmerge - 4.2 (`--list-types`).
- `editor-output-filename` (keep = source basename + `.mkv`, template renders), `editor-output-on-collision` (error refuses / skip omits with warning / overwrite replaces) - 4.8, including the warning-severity nuance.
- `editor-track-rule-optional`: exactly the zero-candidate case, two candidates remain an error - 4.5 verbatim semantics.
- `editor-tracks-unmatched` (primary-file tracks, keep/drop), `editor-tracks-rules` (exactly-one-track uniqueness, list order = output track order) - 4.5.
- `editor-locator-match-to-source`: donor basename must contain the captured identifier, shorthand for a match pattern of exactly that identifier - 4.6's `sugar for match_pattern: '{match}'`, correctly rendered as containment (regex-mode search).
- `editor-locator-match-pattern`: template as regex, interpolated values literal, mutually exclusive with match_to_source - 4.6/4.7.
- `editor-attachments-unmatched`: keep/drop + the ASS-font warning sentence - 4.9's own rationale, correctly carried.
- `editor-attachments-rules`: list order, first match wins, one rule may match several attachments - 4.9's non-uniqueness, exactly right.
- `editor-profile-chapters` (keep/drop/external locator), `editor-profile-title` (keep/clear/template), `editor-tags-*` (keep/drop) - 4.9.
- `editor-match-expr-exact`: typed equality, numeric numbers, language spellings normalized - 4.3/4.4. `raw:` and false-when-absent booleans are absent from the tooltip - correct: field #10 is D54-annotated, that content belongs to its Task-9/10 topic, not the tooltip.
- `editor-match-expr-substring`/`-regex`/`-any`/`-not`: case behavior and AND-of-entries all match 4.3.

Tooltip depth honors the D54 line: the 24 non-annotated controls' tooltips are self-sufficient; the 18 annotated ones stay at tooltip depth without leaking topic content.

## Findings

### 1. MINOR - `editor-output-directory` tooltip omits the empty-case semantics, and the true fact differs from D54's parenthetical (both locales)

D54's exclusion reason for `OutputCfg.directory` is "tooltip covers it (empty = profile's own directory)" - the exclusion is justified *by* the tooltip covering the empty case, which the shipped tooltip does not mention. And the parenthetical's own fact is wrong: an absent profile `output.directory` falls back to the run's **source directory** (`planner.rs:285`: `run.output.or_else(profile.output.directory).unwrap_or(run.source)`; doc comment :33-35 "ultimately to the source dir"). "Profile's own directory" is not a concept in the chain; the parenthetical apparently transposes `batch-output-hint`'s statement about the *batch field* being empty. Fix direction for the owner pass: add one clause, e.g. "Empty: outputs land beside their source files." Exact wording rides the pass; see Q3 for the routing of the design-side correction.

### 2. MINOR - `editor-template-block-template` says "the source basename"; the field is `{source_stem}` (both locales)

Spec 4.7: `{source_stem}` is the primary basename **without extension**, literal mode only. "Source basename" (de "Quell-Dateiname") overstates by the extension. One-word precision item for the owner pass (e.g. "basename without extension" / "Dateiname ohne Erweiterung").

### 3. NOTE (process, plan-side) - plan Step 3's command form does not filter; step nonetheless satisfied (see Q2)

Not a Task-3 defect; recorded for the plan-authoring harvest.

### 4. NOTE - de header's new register/keyword clause encodes the Q4 register resolution as header law

Correct on the merits (see Q1/Q4), but if the owner's rendered-surface pass overrules the tooltip register, the header clause must be swept in the same change - it now states the register as settled.

## Adjudications Q1-Q6

### Q1: catalog header comment edits under the structural-conformance grant - WITHIN THE GRANT (de edit at the boundary's edge, acceptable)

Against: the brief's steps enumerate only attribute additions and a mirror; headers are in neither step, and "repo-visible" comments shape future editors' behavior - the de edit in particular *adds* a normative clause (register + keyword enumeration), which is closer to legislating than conforming.

For: the grant (`latitude-carveout-zero-content-structural-forks`) requires all four zero-outward-effect conditions, and both edits satisfy them - no API/symbol surface, no data-format change (Fluent `#` comments are not part of the runtime message model), verification untouched, and nothing user-visible under the grant's **own definition**: "nothing a user or screen reader perceives". A Fluent comment is never rendered; repo-visible is what every code comment is, and comment-conformance is the core case the grant exists for. The grant fills silence, and the brief is silent on headers; both Files entries name the touched files. Decisively for the en edit: the change itself falsified the header sentence "the editor ships without tooltips (Plan 7)" - leaving it would be a stale-claim defect under the house's own discipline (cf. spec 4.9's corrected stale parenthetical, `proc-normative-count-recomputed`'s sweep logic).

Ruling: en edit squarely within (a sweep obligation, not latitude). De edit within, but it is the grant's outer edge: it records a decision (register for a new content class), not just a repair. Acceptable here because the decision itself is correct (Q4) and wording rides the owner pass; Finding 4 carries the sweep caveat. Both directions logged as calibration data for the over-restriction watch - this is a case the boundary correctly *permitted*.

### Q2: the `--grep` command form - NON-FILTERING CONFIRMED; PLAN STEP SATISFIED AS WRITTEN

Empirics (this session): (a) `pnpm test:e2e -- --grep "catalogs parse"` ran 31 tests; `npx playwright test --list` shows the full suite is exactly 31 tests in 2 files - the form runs everything. (b) Mechanism: pnpm 11.10.0 forwards the `--` *literally* (scratch package probe: script received `-- --grep 'catalogs parse'`), so playwright got the tokens after a `--` terminator; (c) `npx playwright test --list -- --grep "catalogs parse"` still selects all 31 - playwright discards the post-`--` tokens rather than applying them as grep or file filters.

Against satisfaction: the step's evident intent was a targeted parse-guard check; the written command does not deliver the filter, so the step's letter encodes a false belief. For satisfaction: the step's *expected observable* is "the all-locales Fluent parse test green", and the full suite strictly contains that test - the run performed is a superset of the run intended, a stronger check, and in the spirit of Global Constraint "no subsets" for gates. The implementer additionally ran the actually-filtering form (`pnpm test:e2e --grep "catalogs parse"`, no `--`), which I reproduced: the named test runs and passes in isolation.

Ruling: satisfied as written. The command form is a plan-authoring defect (cost: runtime, not coverage); harvest item, with the correct form being `pnpm test:e2e --grep "..."` - pnpm needs no `--`.

### Q3: `editor-output-directory` follows spec 4.8 over D54's parenthetical - CORRECT PRECEDENCE; ONE-LINE DESIGN CORRECTION ROUTES FORWARD, NO TASK-LEVEL NEEDS_CONTEXT WAS REQUIRED

Against the implementer's reading: the plan names "D54's editor tables (the per-control judgments are the content brief)" - the parenthetical sits in that table, so following the spec over it looks like keyboard-resolving a design conflict. For: the plan's own precedence clause is explicit ("the v1 spec is authoritative above it on conflict; flag conflicts, do not improvise"), the implementer flagged rather than improvised, and the parenthetical is verifiably wrong against both spec 4.8 and the code (Finding 1: the fallback is the source directory; "profile's own directory" exists nowhere in the chain). Crucially, the *judgment* the table carries - this field needs no help topic - survives under either reading: the true semantics are exactly as tooltip-sized as the garbled ones. No fork with decision content was open, so NEEDS_CONTEXT was not owed.

Ruling: correct precedence reading. Two residues route forward: a one-line D54 correction through the plan-close amendment channel (a false parenthetical in a design table is a stale claim), and Finding 1's empty-case clause at the owner pass.

### Q4: register - THE PER-CATALOG HEADER GOVERNS; THE TOOLTIPS CONFORM

For du-imperative: it appears twice in the plan (Global Constraint 19's parenthetical and Task 3 Step 2's), and a step-level directive is the most specific instruction. For the header: both plan mentions are *glosses on a delegation* - the operative clause each time is "follows the de catalog header rules", and Task 3's intro explicitly sends the implementer to "the de catalog header rules in `locales/de/gui-editor.ftl`". That header rules "declarative register throughout", with stated rationale; the du-imperative gloss matches **no** de header in the tree (gui-batch declares "GUI imperative in infinitive form"; the others declare nothing). A summarizing parenthetical cannot override the named source it summarizes - the same defect shape as Q3's parenthetical, twice in one task.

Ruling: the gui-editor de header governs. Conformance verified: all 42 de tooltips are declarative sentences or elliptical infinitives (checkbox style, e.g. "Auch die Unterverzeichnisse ... durchsuchen"); zero du-imperative openers (grep + direct read). Register remains owner-pass territory in any case. Harvest: the plan's parenthetical should be corrected so later tasks (5-7 touch other catalogs) don't inherit the wrong gloss.

### Q5: owner-pass flag list - APPROPRIATELY ROUTED; FOUR ADDITIONS

The flagged five are legitimate (Capture-Gruppen and "übersteuert" genuinely new; Stapel-Ansicht pre-exists in gui-jobs, Spender-Datei in diagnostics - conservative flags, new to this catalog; ASS sentence is spec-backed content whose de word choice deserves eyes). Additions I would put on the same list:

1. **"Richtlinie"** (policy, `editor-profile-tracks`/`-attachments` de) - new de term, no catalog precedent.
2. **"gemuxt"/"Muxen"** as verb - new; de diagnostics says "Multiplex-Quelle", so a mux-vs-Multiplex terminology fork now exists.
3. **basename -> "Dateiname"** - en distinguishes basename; de flattens it (also interacts with Finding 2's stem/basename precision).
4. **"erfassten Feldern"** (`editor-output-filename` de) vs **"Capture-Gruppen ... Vorlagenfeldern"** (`editor-input-pattern` de) - the same concept pair named inconsistently between two tooltips.

### Q6: environment setup (pnpm install + one build) - NON-DEVIATION, CORRECT

Fresh worktrees share no `node_modules`; `playwright.config.ts` documents the precondition "dist/ must already exist (pnpm build)". Both actions are environment provisioning: no tracked file changed (worktree clean at 923b049, lockfile untouched), no dependency added (Global Constraint 20 governs manifests, not installs). Required to run the mandated foreground gates at all.

## HARVEST

- **Pattern (new, keep):** editor tooltip register - en declarative one-to-two-sentence guidance, de declarative per the catalog header, config keywords literal in both, annotated-field tooltips stay at tooltip depth and leave topic content to their topics. The de header's new clause is the written form; candidate for the i18n cluster when tooltips extend beyond gui-editor (Tasks 5-7 territory).
- **Grant calibration (over-restriction watch, both directions):** first post-ruling instance of comment-only edits under the structural-conformance grant - correctly permitted; the boundary neither over- nor under-fired. Sub-signal: "repair of a sentence the change falsified" (en) is the grant's clearest interior; "documenting a fresh decision in a header" (de) is its edge and worked here only because the decision was independently correct. If a narrower wording is ever wanted: repair-of-falsified-text is distinguishable from decision-recording.
- **Repeated defect shape (2x in this one task, 3x with plan-6 lineage): a parenthetical gloss contradicting the source it summarizes.** Q3 (D54's "(empty = profile's own directory)" vs spec 4.8/planner) and Q4 (the plan's "(du-imperative)" vs the header it delegates to). Both times the operative clause was right and the gloss wrong. Candidate rule with trigger and handgrip: a parenthetical that *restates* a delegated source is a drift surface - either cite without summarizing or recompute the summary from the source at authoring time (the `proc-normative-count-recomputed` logic extended from counts to glosses).
- **Plan-authoring note:** `pnpm <script> -- --grep X` does not filter Playwright (pnpm forwards `--` literally; playwright drops post-`--` tokens - both empirically confirmed). Correct form: `pnpm <script> --grep X`. Worth a line in the house files before the next plan copies the idiom.
- **Rejections:** none - no fix round was needed.

## Route-forward summary (for the controller)

1. Owner-pass additions: Findings 1-2 wording, Q5's four terminology additions.
2. Plan-close amendment channel: one-line D54 parenthetical correction (Q3); plan Global-Constraint/Step-2 "du-imperative" gloss correction (Q4).
3. House-files: the two harvest candidates (gloss-drift rule occurrence; pnpm/playwright grep idiom).
