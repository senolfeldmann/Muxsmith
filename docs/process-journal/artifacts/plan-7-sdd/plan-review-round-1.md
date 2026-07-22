# Plan 7 plan review, round 1

Artifact: `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md` (1976 lines, uncommitted)
Reviewer: independent plan reviewer (fresh session), 2026-07-21
Ground truth used, in order: v1 spec > round-4-amended design (D50-D64) > plan brief > Tier-2 files + real tree. Every count and claim below recomputed from the tree; nothing taken from the plan or the author's report.

## VERDICT: NEEDS FIXES

Six findings; one major, one moderate, four minor. All fixes are small and localized; no structural rework, no coverage gap. Re-review of the delta should be fast.

---

## Findings (severity-ranked)

### 1. MAJOR - Task 5 Step 3's completion grep can never pass as written (verification soundness)

The step's completion check greps 24 folded/renamed id names over `src/ e2e/` and expects "no output" after the migration. Two independent causes make that expectation unsatisfiable:

- **Three hint names double as DOM `id=`/`aria-describedby=` attribute values that the same task explicitly preserves.** Task 5 states "The `aria-describedby` wiring at those sites is unchanged" (matching D55). Verified in the tree: `settings-mkvmerge-path-hint` (`src/components/SettingsDialog.vue:102,104`), `settings-default-jobs-hint` (`:124,126`), `settings-locale-hint` (`:136,145`) each appear as an element `id` and an `aria-describedby` value. Those six lines survive the migration by the plan's own instruction, so the grep hits them forever. The plan even anticipated this exact false-positive class (its note rejects a suffix pattern "which would false-positive on DOM ids like `firstrun-path-hint`") and then enumerated three names that ARE DOM ids.
- **`e2e/.generated/mount-harness.js`** (gitignored build artifact, present on disk - verified with `git check-ignore`) contains the folded ids from the bundled BatchView code and is inside the grep's `e2e/` scope. It stays stale until the harness rebuilds; ran the grep today: it is the ninth hit file.

Consequence: the implementer lands in a red completion check that the plan says must be worked "to zero", with no routed way out - either they rename DOM ids (unplanned, a behavior change with no design mandate) or silently narrow the grep (a keyboard-resolved fork, which Global Constraint 2 forbids). Fix: exclude the three DOM-id collision lines from the pattern's scope (e.g. grep only `$t(`/`$ta(`-adjacent usages, or enumerate the three names with a `grep -v` for `id=`/`aria-describedby=` lines) and scope the sweep to `src/ e2e/*.ts` or exclude `e2e/.generated/`.

### 2. MODERATE - Task 11 consumes a nonexistent harness helper presented as existing

Task 11 Step 2's spec code: `import { loadHarness } from "./mount"; // the existing harness bootstrap helper` and `await loadHarness(page);`. No such export exists: `e2e/mount.ts` exports exactly `mountComponent`, `readModel`, `readEmitted` (verified, `mount.ts:21-33`); the bootstrap lives inline in `mountComponent` (`page.setContent` + `addScriptTag(MOUNT_HARNESS_PATH)`). writing-plans forbids consuming interfaces that do not exist without saying so; Task 4 handles its analogous case correctly ("if the harness helper of that exact name does not exist yet, add it to `e2e/mount.ts` beside the existing mount helper, same pattern") - Task 11 needs the same clause, or a concrete `loadHarness` addition to `e2e/mount.ts` in its Files/steps.

### 3. MINOR - Task 1's helper call-site counts are off by one, in all three files

Plan: `cli_validate.rs` "6 helper call sites", `run_cli.rs` "12 call sites", `run_live.rs` "7 call sites". Recomputed (grep `muxsmith()` minus the `fn muxsmith()` definition line): **5, 11, 6**. The stated numbers are the total occurrence counts including the definition each time. `dry_run_cli.rs` 13 direct sites, `cli_schema.rs` 2 sites, and the 3+3+4+1=11 snapshot split all verify exactly. Mitigated by "re-verify by text at dispatch" and the Step 6 post-sweep grep, but Global Constraint 12 asserts "every count in this plan was recomputed from the table it summarizes at plan-authoring" (`proc-normative-count-recomputed`) - these three provably were not.

### 4. MINOR - Task 5 Step 4: internal 28-vs-27 contradiction in one step

The step's prose recount says "gui-batch 28" while its own command comment expects `27/27` and correctly explains 39 - 12 = 27 with Task 6's `batch-resolved-track` bringing it to the final 28. The gui-common entry carries its at-this-point qualifier ("reaches its final 38 only after Task 12"); gui-batch's is missing from the prose. One line fix: "gui-batch 27 (28 after Task 6)".

### 5. MINOR - Commit snippets omit the trailer the plan's own Global Constraint mandates

Global Constraints: trailer `Co-Authored-By: <your model name> <noreply@anthropic.com>` on commits. Every one of the plan's per-task commit blocks is a single `git -c commit.gpgsign=false commit -m "..."` with no trailer. The house record shows the trailer on 20/20 recent commits (verified in `git log`). An implementer copying the snippet verbatim - which is what complete-code steps are for - produces non-conformant commits. Fix: add the trailer (second `-m` or heredoc) to the snippets, or one explicit line in the constraints stating snippets show the message only and the trailer is always appended.

### 6. MINOR - "The design is 1792 lines" is a stale measurement

"How this plan cites the design" states 1792 lines; `wc -l` on `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md` gives **1822** (the round-4 amendments grew the file after this sentence was written). A concrete number in a normative preamble that a recount refutes; update or drop the number.

### Notes (no fix required to approve, listed for the record)

- **Task 14 per-row marker mechanics left to inference**: `useDiagAnchor(path)` fits the one-path-per-widget case; the per-row anchors (PropertyMapWidget's `.{rowKey}` rows, the grid's `tracks[{i}]` rows) need inject-once-plus-per-row-computation inside a single component, which Step 5 does not spell out. The e2e fixture table (cases 5, 7, 10) pins the observable contract, so this is under-specification of mechanics, not latitude.
- **Task 16's test location is unpinned**: "e2e/smoke.spec.ts (or the existing editor spec file that asserts the grid columns - match at dispatch)". The grid assertions live in `e2e/smoke.spec.ts` today (verified, "editor view: rule grid + drag-reorder" describe at `:926`); the "(or ...)" hedge is resolvable now.
- **Speculative Modify entries**: Tasks 5/6 list `e2e/smoke.spec.ts` / `e2e/i18n-en.ts` as modified for folded-id assertions; zero folded-id references exist in any `e2e/*.ts` today (verified by the same grep that found the 8 src files). Harmless - the steps are conditioned - but the Files blocks overstate the diff.

---

## Coverage walk (independent, ADR by ADR)

Result: **no coverage gap found.** My walk agrees with the plan's own mapping line.

- **D50** -> Task 11 Step 1 (pin `"marked": "18.0.7"`, registry re-verify), Task 12 Step 4 (the single `v-html` site + license comment), Global Constraint 9. No sanitizer added anywhere. ✓
- **D51** -> Tasks 8-10 (44 files; recount 8+18+18 = 44, id split 4+9+9 = 22 matches D54's tables row for row), Task 11 (`topics.ts`: glob byte-identical to the design's, per-topic en fallback, raw-id fallback). ✓
- **D52** -> Task 12. Verified against the tree: nav (with `nav-batch`/`nav-jobs`/`nav-editor`/`open-settings`) closes before `<main>` opens (`App.vue:72-116`), so the structural-allowlist claim is real; `activeView`/`settingsDialog` refs exist under those names; capture-phase mouseover/focusin/click on main + document keydown, Esc-yields-to-dialog, Enter/Space pin, view-switch clears pin, exit clears refs and listeners, two-class outline highlight with enumerated semantics and carve-out-scoped tokens - all present. ✓
- **D53** -> Task 4 (field + `$ta` bindings on the complete 10-widget set - kind names verified against `fieldSpec.ts:59-70`), Task 3 (the 42 attributes). ✓
- **D54** -> Task 13 (the 18 registry entries match D54's table 1:1 - checked name by name against `registries.ts`'s 13 exports; the 5 template literals match the real hosts: three `<section data-testid="view-*">` roots, the existing grid `<caption>`, SuggestionCard's root `<article>` - all verified present), Tasks 8-10 (topics). 18 + 24 = 42 = the registry's editable set (recounted: 42 `labelKey:` lines; Task 3's 42-id enumeration diffs empty against the extracted labelKey set). ✓
- **D55** -> Task 5 (migration table transcribed - compared against the design row for row, identical, including `batch-recents-select`'s value-less shape and the enumerated non-migrations; the 20 `:title` sites recounted: exactly 20 in `src/`; all spot-checked line anchors - `App.vue:110`, `BatchView.vue:370/392/403/415/426/446/507` - hit), Task 3 (42 new attrs), Task 12 (`help-toggle-label.tooltip`), Task 17 (rules 1-5, carve-out list exact). ✓
- **D56** -> Task 7. `primarySubtag` exists unexported at `src/i18n/index.ts:42` (the export-don't-duplicate instruction is correct); `buildBundles` already exported; `SettingsDialog.save`/`baseline` real. ✓
- **D57** -> Task 14. Grammar table compared against design section 1: all 18 path rows and all three sub-grammars present, asymmetries stated. Emission sites spot-verified at source: `profile_version` :28, `input.pattern` :38, `tracks.rules` :66/:70-73, `{base}.match` :87-91, `{base}.source.external` :112-118, AttachmentRuleShape :244, `.drop`/`.add` :250-255, bare-locator LocatorConflict, `lint.rs` `tracks[{b_idx}]` - all as tabled (file is `crates/muxsmith-core/src/profile/validate.rs`; the design cites it as `validate.rs`, consistent). Marker UX complete (worst-of severity, `severity-*` keys, joined title, `aria-invalid`, panel never filtered - e2e case 1 + the all-diagnostics assertion). ✓
- **D58** -> Task 15. `TYPE_VALUES` (4) and `CODEC_KINDS` (17 entries, recounted) verified in `capability/mod.rs`; `CODEC_KIND_NAMES` is `LazyLock<Vec<&'static str>>` so the emitter loop's `.iter()` is right; `emit_settables_ts` at `ts_export.rs:52` with the `out.push_str` style confirmed; the four dropdown conditions and the 8-case matrix carry the decree's boundary exactly. ✓
- **D59** -> Task 16 (one key, `index + 1`, no data/drag change; the 46th-id budget revision correctly routed as controller trigger 10, not a task edit). ✓
- **D60** -> Task 6 (key en+de, `"-"` stays code-side, comment update). ✓
- **D61** -> Task 18 (gate: `#[cfg(test)]` cutoff, `knownIds` = gui-*+diagnostics - verified that is exactly `check-i18n.mjs:149-151`'s filter; feeds `usedIds`; residual-comment update; 19 codes recounted from `src-tauri/src` outside test modules: 19 ✓) + Task 20 (promotion: the four `.to_string()` sites verified at source in `error.rs`'s `From<ApplyError>` and `run.rs`'s `job-log-not-found` closure; `ParamValue` matches the design's derive/impl set; three ref pairs verified as `Record<string, string>` today; 5+3 = 8 render sites; the one plural selector; mocks sweep). ✓
- **D62** -> Task 19 (all four checks; the (c) `VIEW_TOPICS` scan with the corrected fix-round-1 rationale; `fileTexts` covers `src/**/*.{vue,ts}` - verified the glob filter; group indices `m[2] ?? m[1]` are correct for the two regex shapes; the empty-state forcing order is satisfied by wave sequencing). ✓
- **D63** -> Task 2 (embed table, per-locale-langid chain, adjacent dedup correct, `set_use_isolating(false)`, value-presence walk, raw-id fallback, the enumerated four tests, no de snapshots, amendment 3 as the rustdoc rewrite). `cargo test -p muxsmith-cli --lib` verified valid (the crate has `src/lib.rs`; ran the command, it executes). ✓
- **D64** -> Task 1 (funnel appends after args, bare exception one-caller-closed, per-file grep invariant, e2e-invokes-no-CLI, renderer tests unfunneled, `--locale`-not-env). ✓
- **Section 2** -> end-state table transcribed identically; the "today" column recounted against the tree with the plan's own regex: 41/13/39/46/45/50/26 en=de, 18+4 tooltip family, zero attributes anywhere - all exact. Per-task waypoint counts arithmetically consistent (modulo finding 4). ✓
- **Section 6** -> Task 21 (amendments 1/2/4/5/6; 3 rides Task 2). All five replace-from texts verified verbatim at the spec's actual lines (:388-393, :401, :402, :416, :431). Land-together honored: amendments land in wave 4, after D63 (wave 1) and D52/D54 (wave 2). ✓
- **Section 8** -> plan-close trigger list 2-11 complete, trigger 1 marked consumed. ✓
- **Sections 5/7/9** -> out-of-scope respected (nothing touches `close-abort-*`, i18n-17, the attachment-flaw fix, auto-refresh, F1/search/cross-links, a code-enum); E1/E2/E3 + finding-5 rulings folded; section 9 bound via Global Constraints. ✓

Reverse direction (task elements without design mandate): the e2e spec files, test ids, `diagAnchor.ts` as the injection-key home, and the `parseCatalogIds` wrapper are implementation/TDD structure with stated purpose - justified. The content rules added in Tasks 8-10 are flagged under the over-restriction watch below.

## Amended-design fidelity

All three probe points match the ROUND-4 state: Task 1 carries `muxsmith_bare()` with the closed one-caller exception verbatim; Task 13 Step 1 asserts the amended hover semantics ("sets no hover topic ... pinned topic if one is pinned, else the active view's topic", both branches); Task 21 Step 5(a) carries the amended 6(a) wording. Task 12's suppression spec matches E3. No pre-amendment transcription found anywhere (searched for the old "does nothing and keeps its topic" paraphrase: absent).

## Latitude (both forms)

No implementer-choice clause found (Task 16's "(or ... match at dispatch)" file hedge is the closest thing - noted above). All design enumerations appear complete: D55 migration table (transcribed), D57 grammar (transcribed), D54 annotation table (18 entries + 5 literals), D61 promotion sites (4) and render sites (8), D64 surface (5 files + exception), D55 rule 5 carve-out list, Task 5's non-migrations, Task 3's 42-id set (diffed against the registry: identical). Unmarked lists check out as exhaustive where I recounted them.

## Stream/wave soundness

Wave 1: A touches only `crates/muxsmith-cli/**`; B's Files blocks share nothing with A; C/D/E create disjoint new-file sets (8/18/18, id ranges non-overlapping). Wave 3: G touches only `scripts/check-i18n.mjs`; H's file set (src-tauri, `src/ipc.ts`, three components/views, gui-common, mocks) is disjoint from G. Verified against every task's Files block; no shared file between parallel streams. Serialization rationales verified: Tasks 4/14 both touch all 10 widget files (wave-separated), Tasks 13/16 both touch EditorView's grid region (serial in wave 2), Task 15 consumes Task 14's `path` prop (serial). The wave-2-before-H ordering correctly avoids the EditorView collision.

## Verification soundness (spot-executions)

- **Task 5 Step 3's grep** executed against the pre-migration tree: 44 hits across exactly the 8 src files Task 5 modifies plus `e2e/.generated/mount-harness.js` - pattern validated against a known-present control, and the finding-1 defect surfaced by the same run.
- **Task 2 Step 4's grep** executed: hits exactly the two stale rustdoc lines (`i18n.rs:12,19`) the fire-verify predicts.
- **Task 1 Step 6's grep** executed against the pre-state: 5 files today (known-present control for the post-sweep expected-1-file form).
- **Task 5 Step 4's count command** executed (today's tree): matches the design's "today" column exactly.
- **`cargo test -p muxsmith-cli --lib`** executed: compiles and runs (lib target exists); `-p muxsmith-gui` confirmed as the real shell package name.

Fire-once discipline is present on every absence-expectation step I checked (Tasks 1, 2, 3, 5, 8, 17, 18, 19, 20, 21) - the plan is exemplary on this dimension; finding 1 is the one check whose green state is unreachable, which fire-verification of the red state cannot catch.

## House conformance

Nine-part gate matches BUILDING.md's list exactly (9 parts recounted). Bilingual-in-same-commit stated globally and per task. `editor-generic-action-keys` 45->46 handled as trigger 10 with the tooltip-attribute note - matches the entry at `product-boundaries.yaml:404`. `gui-closed-domain-dropdowns` boundary respected (exact-match cells only, path-gated, `raw:` preserved). Presentation carve-out correctly scoped (semantic mappings enumerated; only colors/widths delegated). Tier-2 line refs re-measured correctly against the current files (:449, :420, :455, :457 - all verified; the plan's numbers supersede the design's stale ones). Unsigned commits everywhere; trailer defect is finding 5.

## HARVEST

- **Dominant pattern (positive)**: transcription fidelity. The two hardest-won enumerations (D55 migration table, D57 grammar) are byte-faithful to the design, and the plan re-measured line anchors rather than copying the design's (three Tier-2 refs corrected by 1 line each). Fire-verification is attached to essentially every absence check without being prompted per instance.
- **Repeated defect class: counts.** Three of the four content findings (1 excepted) are count/measurement defects: call-site counts including the definition line (x3 files), a waypoint count stated as its end-state value, a stale design line count. Same class the plan-6 record names as commonest. The plan's own Global Constraint claims universal recomputation - the claim itself is what these contradict. Recommendation for the fix round: re-run the recount pass mechanically (occurrences minus definitions; waypoint vs end-state labeled explicitly).
- **Over-restriction watch**: Tasks 8-10 add content rules the design does not mandate - markdown restricted to headings/paragraphs/lists/emphasis/inline code, a required `# <h1>` opener, "realistic length 1-3 kB" (descriptive cost estimate in D51, normative in the plan). Each carries a rationale and none conflicts with an ADR, but they bind topic authors beyond the contract; the controller should ratify or relax them explicitly rather than let them bind silently. Conversely, the plan correctly resisted over-broad restriction in its grep design (rejecting the suffix pattern with a named false-positive) - the finding-1 residue is a missed collision, not over-restriction.
- **Verification-check blind spot worth a process note**: a completion grep was fire-verified for its red state but not checked for a *reachable green* state - the DOM-id collision means red is permanent. "Fire the check once" catches malformed patterns; it does not catch checks whose pass condition conflicts with a preserved invariant in the same task. A cheap guard: for every worked-to-zero sweep, ask which enumerated names legitimately survive in non-target positions.

## Whole-plan justification

The plan is a faithful, complete, well-structured implementation of the round-4 design: my independent ADR-by-ADR walk found every mandated element owned by a named task and no unmandated element without justification; the hardest enumerations are transcribed without loss; stream disjointness, serialization rationales, TDD ordering, amended-design fidelity, and house constraints all verify against the real tree, and its measured claims are overwhelmingly accurate (registry sets, catalog counts, line anchors, identifier names, crate/package names all recomputed clean). It fails this round on execution-surface defects, not design coverage: one completion check that cannot go green as written (finding 1), one nonexistent helper consumed as existing (finding 2), and a cluster of small count/snippet inconsistencies (findings 3-6) that contradict the plan's own recomputation constraint. All are mechanical fixes confined to individual steps; on their correction I expect to approve without a further full walk.
