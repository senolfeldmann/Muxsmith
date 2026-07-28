# Task 1 verdict: the planner seam `pipeline::plan_pipeline` + four call-site migrations (D91-D95, S-4/S-8)

**Graded:** commit `9bbe53d` (8 files changed, 288 insertions(+), 246 deletions(-)) against the plan's Task 1 and Global Constraints, design D91-D95 (incl. D91's divergence table, the four call-site mappings, the amended D93), design section 5's seam bullets, design section 7 observable 1, spec sections 5.5 and 7, and the four house-knowledge YAMLs.
**Reviewer:** independent (did not implement, did not dispatch). Model tier: mid (Opus 5), per `proc-03-model-assignment`.
**Verdict: APPROVED.** Behavior preservation holds on all seven enumerated divergences; every fence is met; both verification checks reproduce on my own instruments with a fired red and a controlled green; the retained mpsc block is byte-identical (md5-proven). Three findings, none behavioral, none user-visible, none gate-failing: two LOW one-line prose corrections in files this task owns, and one MEDIUM tree-level ripple that the fences correctly stopped this task from carrying (adjudication 3).

**Instrument independence.** Every measurement below was taken in `/tmp/claude-1000/-home-senol-agents-peter/6556b0df-2581-4c8d-8a85-e1f1b567eb55/scratchpad/rev-t1-p9`, a path minted in this pass. No probe, extraction or scratch file of the implementer's was read or re-run. Pre-state evidence comes from read-only `git grep`/`git show` against `9bbe53d^`; the tree was never mutated (`git status --porcelain` empty before and after every run). Toolchain: `rustc 1.96.1 (31fca3adb 2026-06-26)`.

---

## 1. Behavior preservation - the seven divergences, individually

Walked per divergence against the post-state code, not against the report.

| # | What D91 fixes | Post-state | Verdict |
|---|---|---|---|
| D-1 | resolver is a parameter; CLI `locate`, GUI `detect(override)` | `dry_run.rs:32` and `run.rs:57` pass `Mkvmerge::locate`; `lib.rs:205-207` passes `\|\| Mkvmerge::detect(mkvmerge_override)`; `run.rs:256` passes `\|\| Mkvmerge::detect(mkvmerge_override.as_deref().map(Path::new))` - the same expression the pre-state called inline | preserved |
| D-2 | `mkvmerge_found` unified meaning, no new wire field | `MkvmergeUnavailable` carries only `config_diags`; all four sites still map to `Some(false)` / `Some(true)` / key-absent exactly as before; `config_only_document`'s signature is untouched (its diff is doc lines only) | preserved |
| D-3 | `on_collision` becomes a parameter | CLI passes its flag value at both sites; both GUI sites pass `None` | preserved |
| D-4 | human-vs-JSON ordering stays per-surface | `severity_sorted` remains a CLI import and is applied at the same three human branches; core does no sorting | preserved |
| D-5 | return shape stays per-surface | `i32` (dry_run), `i32` with fall-through (run), `serde_json::Value` (dry_run_body), `Result<PlanOutcome, IpcError>` (plan_run) - all unchanged | preserved |
| D-6 | specs gate on copies 2+4 only | `pipeline::job_specs` has exactly two consumers: `crates/muxsmith-cli/src/commands/run.rs:142`, `src-tauri/src/run.rs:287`. No dry-run surface calls it | preserved |
| D-7 | `plan_run`'s settings-read-first stays caller-side | `run.rs:248` `let mkvmerge_override = crate::load_settings_from(settings_path.as_deref())?.mkvmerge_path;` is still the first statement, above the seam call; the wrapper-vs-body asymmetry with `dry_run` is untouched | preserved |

**Printed bytes / stderr / exit codes.** Every `println!`/`eprintln!` argument expression in the four migrated bodies is byte-identical to the pre-state modulo the binding name (`d` -> `diagnostic`, `run_inputs.source` -> `source_dir`, `batch` -> `planned.batch`). All three CLI pre-planning branches still `return 2`; `Planned` still ends in `diag_exit_code(&config_diags, &batch)`. Both GUI failure shapes still return `PlanOutcome::Soft(run_document(config_only_document(..), &[], &[]))` with the same `Some(false)`/`Some(true)`/`None` third argument.

**The funnel substitution is an identity.** `validate::config_diagnostics` (`validate.rs:193-197`) is exactly `validate(profile)` followed by `diags.extend(lint::provable_overlaps(profile))` - the same two calls in the same order the four copies inlined. No diagnostic can be added, dropped or reordered by the migration.

**Event ordering.** The CLI run's event stream is produced entirely inside the retained mpsc block (section 4), untouched. The GUI emits no event inside the planning stretch; `plan_run`'s post-planning stretch (`run_id`, `QueueControl`, `RunLogger::create`, `base`, `outputs`, `mkv_path`) is unchanged in content and order.

**Emitters.** `cargo test --workspace`: **494 passed, 0 failed, 0 ignored**, tree clean afterwards, no `.snap.new` anywhere. `mkvmerge` is present on this machine (`/home/linuxbrew/.linuxbrew/bin/mkvmerge`, `v100.0`) and the run produced **0** occurrences of the `mkvmerge not found; skipping` marker, so `run_live.rs`, `run_cli.rs` and `dry_run_cli.rs` really executed the subprocess paths rather than self-skipping. No test file appears in the diff, which the plan names as the required state.

## 2. Fidelity to the fences

- **Module, signature, types, fields.** `crates/muxsmith-core/src/pipeline.rs` carries `PipelineOutcome` (4 variants, `Planned(Box<PlannedPipeline>)`), `PlannedPipeline` (5 fields in the fence's order), the five-parameter `plan_pipeline`, and `job_specs` - character for character where the fences write text. `job_specs`' body and doc comment are the fence verbatim.
- **Step order S1-S7.** `load::from_file` (:105), `validate::config_diagnostics` (:110), `resolve_mkvmerge()` (:112), `mkv.list_languages()` (:116), `RunInputs` with the single `unwrap_or_else` (:121-125), `LiveIdentifier { cache: IdentifyCache::new(), mkv: &mkv }` (:127-130), `plan_batch` (:131). Exactly S1-S7, exactly D91's order.
- **D93 (amended).** No cache parameter; the cache is constructed at S6 and dropped on return. `LiveIdentifier` and `IdentifyCache` are not in the diff; `crates/muxsmith-core/tests/command_integration.rs` is not in the diff; `AppState` gains no field (the `lib.rs` diff is imports plus `dry_run_body`).
- **D92.** `report/json.rs` diff is the doc block only; the wording carries all three contract clauses (`false`/`true`/key-absent) with the CLI and GUI qualifications.
- **S-4 / S-8.** Both spec hunks are the design's fences verbatim; the spec diff contains exactly those two hunks and nothing else.
- **Files.** The 8 touched files are precisely the exhaustive list, each within its stated scope (`lib.rs` module declaration only; `report/json.rs` doc only; `src-tauri/src/lib.rs` mapping 3 only; `src-tauri/src/run.rs` mapping 4 only).
- **Consolidation duty.** The near-verbatim CLI rationale comments are gone from both copies and re-homed: the superset-of-`validate` rationale onto `plan_pipeline`'s doc, the S3e/S4e per-branch rationale onto the variant docs (the fence's own text), the specs-gate comment onto `job_specs`, the D95 note onto the source-default doc **verbatim including its closing clause** ("via its dir picker before calling this command"). Nothing was dropped without a new home.
- **Typography.** No non-ASCII byte in any added line; no em/en dash, smart quote or Unicode ellipsis in `pipeline.rs`.

## 3. The two verification checks - re-run by me

**Check A (design acceptance observable 1).**

Fire, read-only against the pre-state:

```
$ git grep -n "config_diags.extend(lint::provable_overlaps" 9bbe53d^ -- crates src-tauri
9bbe53d^:crates/muxsmith-cli/src/commands/dry_run.rs:60:    config_diags.extend(lint::provable_overlaps(&profile));
9bbe53d^:crates/muxsmith-cli/src/commands/run.rs:85:    config_diags.extend(lint::provable_overlaps(&profile));
9bbe53d^:src-tauri/src/lib.rs:211:    config_diags.extend(lint::provable_overlaps(&profile));
9bbe53d^:src-tauri/src/run.rs:263:    config_diags.extend(lint::provable_overlaps(&profile));
```

Exactly the four sites the design names, at the lines it names. Green on the post-state: `grep -rn "config_diags.extend(lint::provable_overlaps" crates src-tauri` -> no output, exit 1. Green-reachability confirmed member-by-member rather than assumed: the unqualified `lint::provable_overlaps` survivor set is **2** hits, both in `crates/muxsmith-core/src/profile/validate.rs` (`:189` a doc mention, `:195` the funnel's own `diags.extend(...)` call), neither matchable by the qualified `config_diags.extend(` pattern. `pipeline.rs`'s consolidated docs do not restate the qualified call text, as the plan required.

**Check B (D95 single default).**

Fire: `git grep -n 'PathBuf::from(".")' 9bbe53d^ -- crates/muxsmith-cli/src src-tauri/src` -> 4 hits, `dry_run.rs:109`, CLI `run.rs:143`, `lib.rs:230`, src-tauri `run.rs:291`. Green: the same grep on the post-state -> no output, exit 1. Presence control `grep -c 'PathBuf::from(".")' crates/muxsmith-core/src/pipeline.rs` -> **1**. Whole-core control `grep -rn 'PathBuf::from(".")' crates/muxsmith-core/src` -> **1** hit, `pipeline.rs:122`, a code line - no doc-prose instance anywhere, so the count of 1 is the code default itself and nothing else.

Both checks therefore have a demonstrated red and a demonstrated green, on separate trees, with the survivor set enumerated rather than asserted.

## 4. The retained block - proven byte-unchanged

Independent proof, not a re-run of anything the implementer left:

```
$ git show 9bbe53d^:crates/muxsmith-cli/src/commands/run.rs | sed -n '241,272p' > queue_old.txt   # pre-state block
$ sed -n '205,236p' crates/muxsmith-cli/src/commands/run.rs   > queue_new.txt                     # post-state block
$ diff -u queue_old.txt queue_new.txt && echo IDENTICAL
IDENTICAL_205_236
$ md5sum queue_old.txt queue_new.txt
f4dc6ebe6312022a00ce9e1ed902af53  queue_old.txt
f4dc6ebe6312022a00ce9e1ed902af53  queue_new.txt
```

32 lines each, from `let (tx, rx) = mpsc::channel();` through `handle.join().expect("queue worker thread panicked")` - the exact region the plan carries verbatim into Task 2. Instrument control (the diff must be able to fire): the same comparison against the off-by-one range `205,235` reports the files differ, so the identity above is a measurement and not a vacuous pass.

Independent corroboration from the diff itself: the `run.rs` hunk set ends at `if specs.is_empty() {`. Everything from there to end of file, the empty-specs presentation and the whole queue block included, has no diff hunk at all.

## 5. House dimension and latitude

Checked against `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`, `docs/decision-ledger.yaml`.

Conforming:

- `core-37-prose-free-core` / spec 5.2: the seam returns data; no `println!`/`eprintln!`/document/exit code in `pipeline.rs`.
- `cli-02-dryrun-config-diags`: config-time diagnostics still travel on all three pre-planning paths and in both renderers - now structurally guaranteed by the seam carrying them on `MkvmergeUnavailable`/`QueryFailed` rather than by four copies remembering to.
- `core-derive-dont-restate`: the four-copy duplication becomes one definition; the D95 note exists once.
- `cli-09-collision-parity`: `--on-collision` still reaches `RunInputs.on_collision` on both CLI commands.
- `proc-05-commit-signing`: commit carries no `gpgsig` header (verified via `git cat-file commit`).
- `agent-commit-trailer-set`: exactly one trailer, `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`; no `Claude-Session`; no context-window suffix; the model name matches the mid tier `proc-03-model-assignment` assigns to Task 1.
- `proc-verification-step-must-be-falsifiable` / `proc-check-green-state-reachable`: both absence checks have a fired red and an argued-then-observed green (section 3).
- `latitude-carveout-zero-content-structural-forks`: the Files list was treated as the enumeration boundary it declares itself to be; the one out-of-list ripple was surfaced, not resolved at the keyboard (adjudication 3).

Deviations: see findings LOW-1 (`proc-normative-count-recomputed`) and LOW-2 (`code-comment-line-citations-drift`).

Nothing in the diff resolves a fork the implementer was not licensed to resolve. The only design-silent space it entered is rustdoc prose (adjudication 1), which is grant territory, and it stayed inside it except for the one count-word of LOW-1.

## 6. The gate

Run by me, foreground, in this repo:

| step | result |
|---|---|
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished` - no warnings, no errors |
| `cargo test --workspace` | **494 passed, 0 failed, 0 ignored**; tree clean afterwards |

Beyond the required three, because the new module adds rustdoc that only the plan-close gate would otherwise exercise: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` exits 0 with all three crates actually re-documented (forced by a fingerprint-changing flag, so the pass is not a replayed cache), and a from-scratch build of `muxsmith-core` in a private `CARGO_TARGET_DIR` passes both `cargo doc` and `cargo clippy` under `-D warnings`. That from-scratch run is the independent proof that `#![deny(missing_docs)]` is satisfied and that the new intra-doc links resolve.

---

## Findings

### MEDIUM-1 (controller item, not an execution defect): `identify.rs:388` now contradicts the spec this commit amended

`crates/muxsmith-core/src/identify.rs:388` reads `/// The per-session identification cache (spec 5.5).` on `LiveIdentifier.cache`. Spec 5.5 as replaced by this commit's S-8 says the cache is "constructed per planning call and dropped with it". A tree-wide sweep confirms it is the only surviving `per-session` claim in code; the spec's own remaining occurrence of the phrase is inside S-8's replacement text, where it correctly describes the ruled-out alternative.

Not a defect of this task's execution - the file is outside the exhaustive Files list and design section 5 states "no `LiveIdentifier` change" without qualification, so surfacing rather than fixing was the correct move under `proc-latitude-clause-boundary`. Routing and the minimal vehicle: adjudication 3.

### LOW-1: `pipeline.rs` asserts a count of 1 over a set D91 enumerates as more than 1

Two sentences the fences do not contain claim a uniqueness that D91's divergence table contradicts:

- `pipeline.rs:10-13`: "The one place the surfaces deliberately differ inside the pipeline itself -- how mkvmerge is resolved ... -- is a parameter, not a branch."
- `pipeline.rs:73-74`: "`resolve_mkvmerge` is the one deliberate per-surface divergence, injected rather than branched on".

D91's table lists **two** divergences that "become a parameter" on this seam: D-1 (the resolver) and D-3 (`on_collision`, where "CLI passes its `--on-collision` flag value; both GUI call sites pass `None`"). The claim is defensible under a narrow reading (only D-1 concerns behavior the pipeline performs, the rest are inputs), but it is written as an absolute and it obscures design trigger 3, whose whole content is that the `on_collision` seam parameter is already wired and needs no pipeline change when a GUI control arrives.

`proc-normative-count-recomputed`, whose occurrence log explicitly reaches source-file charters ("check-i18n.mjs's charter still opens 'Three independent checks' after rule 3 added a fourth").

**Change to:** drop the count from both sentences. Module doc: "How mkvmerge is resolved ... is a parameter, not a branch." Function doc: "`resolve_mkvmerge` is the divergence that is injected rather than branched on: ...". No other word needs to move.

### LOW-2: the mapping-4 import removal left a dangling intra-doc link at `src-tauri/src/run.rs:359`

The line is:

```
/// shells out to mkvmerge once per source file via [`plan_batch`]'s
```

It links a symbol this commit removed from the module's scope (the deleted line `use muxsmith_core::planner::{RunInputs, plan_batch};`). `plan_batch` now appears nowhere else in `src-tauri`, so the link cannot resolve.

Verified, with the instrument fired against a known-present case first: a standalone rustdoc probe carrying exactly this shape errors under `-D warnings` with "unresolved link to `plan_batch`", so an empty result from my grep is evidence rather than decoration. `RUSTDOCFLAGS="--document-private-items" cargo doc --workspace --no-deps` then reports **exactly one** unresolved link in the whole workspace, at `src-tauri/src/run.rs:359:55`, "no item named `plan_batch` in scope".

The default gate step does **not** see it, because `mod run;` is private (`src-tauri/src/lib.rs:23`) and rustdoc does not process private items' docs without `--document-private-items`. So no later task and no plan-close gate run would catch this.

The prose remains true (planning still reaches `plan_batch`, now through the seam); only the link target is out of scope. Repairing a reference the change itself falsified is inside `latitude-carveout-zero-content-structural-forks`'s sweep duty in a file this task already owns (the plan-7 T3 precedent recorded on that entry), so it was catchable here.

**Change to** the fully qualified form, which resolves with no import - one line:

```
/// shells out to mkvmerge once per source file via
/// [`muxsmith_core::planner::plan_batch`]'s
```

**Routing for LOW-1 + LOW-2.** Both live in files no later task's Files list reopens for prose (`pipeline.rs` is in no other task's list; `src-tauri/src/run.rs` is reopened by Task 2 but only for the D96/D97 regions). Cheapest correct vehicle is one doc-only fix round on Task 1, which can carry MEDIUM-1 in the same dispatch once the controller licenses the third file. Neither blocks Task 2 from starting.

---

## Adjudication

### Q1: the added doc comments - in scope, and does any of them change meaning?

**Verdict: in scope, and the wording is acceptable with exactly one exception (LOW-1).**

The lint premise is real and I verified it independently rather than borrowing it. `#![deny(missing_docs)]` sits at `crates/muxsmith-core/src/lib.rs:1`. A standalone probe compiled in my scratch shows `missing_docs` firing on all three constructs the fences leave bare - a named field of an enum variant, a public struct field, and a public function:

```
error: missing documentation for a struct field   --> docprobe.rs:8:9    (enum variant field)
error: missing documentation for a struct field   --> docprobe.rs:14:5   (pub struct field)
error: missing documentation for a function       --> docprobe.rs:17:1
error: aborting due to 3 previous errors
```

So the D91 fence as literally written does not compile in this crate. The seven additions are: `LoadFailed.diagnostic`, `MkvmergeUnavailable.config_diags`, `QueryFailed.config_diags`, `PlannedPipeline.config_diags`, `PlannedPipeline.batch`, the `PlannedPipeline` struct doc, the `plan_pipeline` doc. The module doc is separately mandated by D91 ("the near-verbatim rationale comments ... consolidate into this module's docs"), not by the lint.

Scope: `latitude-carveout-zero-content-structural-forks` covers this squarely. All four zero-outward-effect conditions hold - no API or symbol surface (a doc comment adds no item), no data format, no verification weakened, nothing user-visible (rustdoc is developer-facing; no string, no catalog key). The grant "fills SILENCE only", and the fences are silent here in the strong sense: not merely unenumerated but uncompilable as written. Routing this as NEEDS_CONTEXT would have been over-restriction, not diligence.

Content, checked claim by claim rather than skimmed:

- `LoadFailed.diagnostic`: "The single `ParseError` diagnostic the load produced" - accurate. `load::from_file` has exactly two error paths and both build `DiagCode::ParseError` (`load.rs:62` for the read failure, `load.rs:71` via `parse_error`).
- The two `config_diags` variant docs and both `PlannedPipeline` field docs restate what the step order already establishes; no new claim.
- `plan_pipeline`'s superset-of-`validate` paragraph is a faithful consolidation of the two deleted CLI comments and, on one point, strictly more accurate than what it replaced: the old `dry_run.rs` doc said "Exception: if mkvmerge cannot be located", which never covered the query-failed branch; the new text says "the pre-planning failures", which does.
- The cache paragraph matches D93 and S-8 exactly, including the per-call scope and the within-call sharing.
- The module doc's "It emits no prose, no documents and no exit codes (spec 5.2, 7, 8.4)" mirrors the crate doc at `lib.rs:3-6`, which cites the same sections; not invented.

The one exception is LOW-1: "the one deliberate per-surface divergence" is a count claim over a set the design enumerates, and it is the only added sentence that asserts something the design does not support. Everything else satisfies the lint without asserting anything new.

### Q2: the D95 note under the presence control - faithful, or distorted?

**Verdict: faithful. The check did not distort the documentation, and the wording is not a workaround.**

What D95 mandates is two things: the default is applied exactly once inside the seam, and the `lib.rs` note moves onto the seam's doc so that "after the hoist there is exactly one fallback and one note". Both hold literally. The note moved **verbatim, including the closing clause the design's own quotation truncates** ("via its dir picker before calling this command"), and the sentence that precedes it - "`source` falls back to the current directory here, once for every surface (D95)" - states the single-application property in prose.

The premise of the concern does not actually bite: the original `lib.rs` comment never named `PathBuf::from(".")` either, so no naming was suppressed to satisfy the count. And "falls back to the current directory" is not a euphemism for the literal; it is the semantics, sitting six lines above the code line that shows the literal. A reader of the doc reaches the mechanism in the signature and the body without a detour.

The count is also honest rather than tuned: `grep -rn 'PathBuf::from(".")' crates/muxsmith-core/src` returns exactly one hit tree-wide in core, `pipeline.rs:122`, which is the code default. There is no doc-prose instance anywhere in the crate that the control would have had to avoid counting.

### Q3: `identify.rs:388` - a defect of Task 1, and what is the minimal vehicle?

**Verdict: it is a real defect now live in the tree and it is a ripple of Task 1's own spec amendment - but it is not a defect of Task 1's execution, and Task 1 could not correctly have carried it. Minimal vehicle: a controller-authored amendment adding the one file plus its exact replacement text, executed as a doc-only fix round on Task 1 (which can carry LOW-1 and LOW-2 in the same dispatch).**

Three parts.

*Is it a real contradiction?* Yes. Verified by sweep: `crates/muxsmith-core/src/identify.rs:388` is the only surviving `per-session` claim in `crates`, `src-tauri`, `src` and `e2e`; the spec's only remaining occurrence of the phrase is inside S-8's own replacement sentence describing the alternative that was ruled out. The comment also cites the exact spec section this commit rewrote, which is `code-comment-line-citations-drift`'s named trigger: "whenever the SAME INITIATIVE rewrites a file it cites elsewhere, the citing document has re-staled its own span". It is a LIVE DESCRIPTIVE claim under that entry's two-class rule, so the correct treatment is re-pointing, not historical marking.

*Should Task 1 have carried it?* No, and the implementer's stop was correct on two independent grounds. The Files list is marked EXHAUSTIVE, and `latitude-carveout-zero-content-structural-forks` states that an enumeration "always wins over" the house-pattern grant. Independently, design section 5 says "no `LiveIdentifier` change" with no qualifier; read as written that covers a doc comment on a `LiveIdentifier` field. Under `proc-latitude-clause-boundary` the required response to a fork found on code contact is to surface it and let the controller route it, which is what happened. Note for accuracy: the D93 *entry* text ("no borrow change, no public interface change") would not have forbidden a doc fix; it is section 5's blanket phrasing plus the exhaustive list that did the stopping.

*Where does the defect actually originate?* Not with the implementer, and not with the plan. The design's own contradiction sweep for S-8 (section 3) searched the **spec** for neighbouring contradictions and checked the adjacent run bullet. It never searched the **code** for comments citing spec 5.5. That is the gap - a spec-section citation in a code comment is a citation, and the sweep that a spec amendment owes should cover it.

*Vehicle, weighed.* A later task's file list is the wrong home: no remaining task in Plan 9 opens `identify.rs`, so routing it forward requires a file-list amendment anyway, and Task 3 (the next core+spec task) would then carry a correction with no relation to its own subject matter. A controller one-line commit outside the task structure is cheaper still but leaves the correction unreviewed, which is exactly the property this plan's four-eyes structure exists to prevent. The doc-only fix round on Task 1 costs one dispatch and one short re-review, keeps the amendment and its ripple auditable in one place, and lands before the plan-close gate so the tree is never pushed carrying the contradiction. The amendment must write out the replacement text (`/// The identification cache (spec 5.5), constructed per planning call and dropped with it.` or the controller's own wording) rather than describing it, so the fix round carries no latitude.

---

## HARVEST

**Dominant pattern of this task: the gate cannot see prose, and prose is where both defects landed.** Every machine-checkable property held on the first pass - fmt, clippy, 494 tests, rustdoc, both greps, the byte-identical block. Both LOW findings are sentences: a count-word in a new doc, a link target in an old one. Neither is reachable by any check in the ten-part gate. A hoist that deletes 246 lines and re-homes their rationale is, by construction, mostly a prose migration, and the review effort should be weighted accordingly on tasks of this shape.

**Repeated shape across all three findings: a claim the change itself falsified.** LOW-1 (a uniqueness claim written while two parameters were being added), LOW-2 (a doc link written while its import was being deleted), MEDIUM-1 (a comment citing a spec sentence the same commit replaced). Same class, three layers, one initiative. The generalizable handle already exists for two of them (`code-comment-line-citations-drift`, `proc-normative-count-recomputed`); what is missing is the third, below.

**Proposed rule (new), with a readable trigger and an executable handle.** *You are replacing a sentence in the spec -> grep the code for comments citing that spec section, in the same change.* The existing citation entry covers `file:line` drift; a **spec-section citation is a citation too**, and it drifts in the opposite direction (the cited document changes under a stable coordinate, so nothing about the citing line looks stale). The handle is one command per amendment: for S-8, `grep -rn "spec 5.5" crates src-tauri src` would have found `identify.rs:388` at design time, when adding the file to a task's list was still free. This belongs to the amendment author (design or plan), not to the implementer, because only they know which sentence is being replaced.

**Standing gate gap, measured.** `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - gate part 4 - does not link-check private modules, and `src-tauri` puts `run`, `error` and `settings` behind private `mod` declarations. Measurement: with `--document-private-items` the whole workspace holds **exactly one** unresolved intra-doc link, the one LOW-2 names. So adding `--document-private-items` to the gate's doc step would cost nothing today beyond fixing that single line, and would close the blind spot permanently. Recommended as a ledger/BUILDING.md item for the controller; not a Plan-9 task's business.

**Over-restriction watch (`latitude-carveout-zero-content-structural-forks` explicitly asks for these).** One stop I judge the boundary should have covered: MEDIUM-1. The correction is a single doc comment; it satisfies all four zero-outward-effect conditions; its content is fully determined by the spec sentence the same commit wrote, so there is no fork to resolve and nothing to invent. What blocked it is that an EXHAUSTIVE Files list plus a blanket "no X change" bullet cannot express "and the one-line ripples of your own amendment". This is calibration data in the over-restriction direction, and it is cheap to fix without loosening the grant: the amendment sweep proposed above puts such ripples **into** the file list at design time, so the boundary never has to be crossed at the keyboard.

**Calibration in the other direction, worth recording as interior:** the missing-docs additions of adjudication 1. The fence was not merely silent, it was uncompilable as written, and the implementer wrote the minimum accurate prose rather than routing it. That is the grant working exactly as intended, and a NEEDS_CONTEXT there would have cost a round for nothing.

---

# Fix-round delta review (commit `fed55be`)

**Scope:** delta only. Behavior preservation, the fences, the two greps, the byte-identical queue block and the gate were cleared above and are untouched by a doc-only round; none was re-run. Graded: the three doc edits, the doc-only claim, LOW-2's proof, and the `--document-private-items` cost question.
**Reviewer:** same judge, independent of the fix. Fresh scratch: `/tmp/claude-1000/-home-senol-agents-peter/6556b0df-2581-4c8d-8a85-e1f1b567eb55/scratchpad/rev-t1-p9-delta`. No instrument from the first pass or from the implementer was re-run; the tree was never mutated (`git status --porcelain` empty throughout).
**Delta verdict: APPROVED. All three findings CLOSED.**

## Correction to the review brief (`proc-57-briefs-not-ground-truth`)

The brief says `fed55be` sits "on top of `9bbe53d`" and asks me to diff them. It does not. Measured:

```
1a5a32b roadmap+house: correct the flag's cost; record my own concurrent-writer race
fed55be core+gui: Task 1 doc fix round - drop the divergence count, requalify the plan_batch link, re-point the cache doc at S-8
2856730 house+roadmap: mine the Task 1 harvest, track the rustdoc gate blind spot
```

`fed55be`'s parent is `2856730`, a controller commit (`docs/ROADMAP.md`, `docs/decision-ledger.yaml`, 36 insertions), and `1a5a32b` landed after it. So `git diff 9bbe53d fed55be` also carries the controller's ROADMAP and ledger writes, which are not the fix round and are not graded here. The fix round proper is `git show fed55be`: exactly the 3 files and the +11/-10 the brief quotes. No ruling is affected; recorded because a delta review that diffed the pair the brief named would have graded two commits as one.

Commit hygiene, checked in passing: no `gpgsig` header, exactly one trailer `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session` line - `proc-05-commit-signing` and `agent-commit-trailer-set` both satisfied.

## 1. Do the three edits close their findings?

### LOW-1 - CLOSED

Both sentences, as specified. The module doc drops the claim entirely ("How mkvmerge is resolved (the CLI's PATH-only lookup versus the GUI's override-aware detect ladder, D28) is a parameter, not a branch."). The function doc reads "`resolve_mkvmerge` is the divergence that is injected rather than branched on".

I am grading my own prescription here, so the test I applied is not "does it match what I wrote" but "does the surviving sentence still assert something D91 denies". It does not. The definite article now restricts to divergences that are *injected behavior*; D-3's `on_collision` is a value the caller forwards into `RunInputs`, not behavior the seam executes, so D91's table no longer contradicts the sentence. Design trigger 3's mechanism is no longer obscured, because nothing claims the resolver is the seam's only per-surface parameter.

### LOW-2 - CLOSED

The link is requalified to `muxsmith_core::planner::plan_batch` and, correctly, kept whole on a single doc line rather than split across the `///` wrap. Resolution verified in section 3.

### MEDIUM-1 - CLOSED, and the wording matches S-8 rather than paraphrasing it

The new comment, unwrapped: `The identification cache (spec 5.5), constructed per planning call and dropped with it.`

Checked against the spec as it now stands (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:311`, S-8's replacement sentence) by substring test rather than by eye: the clause `constructed per planning call and dropped with it` is present verbatim in both. Removing that clause from the comment leaves `The identification cache (spec 5.5)` - the original comment's own subject and its own citation, minus the falsified `per-session`. So the edit contributes no new prose at all: it deletes one adjective and grafts on a verbatim clause of the authoritative sentence.

It says strictly *less* than S-8 (it omits the keying, the within-call sharing and the CLI/GUI split), which is the right shape for a field doc and cannot contradict the spec: a subset of an authoritative sentence is not a new claim. The licence scope - "that one doc comment on `LiveIdentifier.cache`, no struct, field, type or visibility change" - is proven honored by the code-identity test in section 2, not merely asserted.

## 2. Is the round genuinely doc-only? Verified independently, two instruments, both fired

I did not re-run the implementer's filtered `git diff -U0`. Two instruments of my own, on the correct range `2856730..fed55be`:

**Instrument A - diff-line filter.** Every added/removed line of the fix round, minus file headers, minus lines whose trimmed form opens with `///`, `//!` or `//`:

| run | non-comment diff lines |
|---|---|
| fix round (`2856730..fed55be`) | **0** |
| fired control (`9bbe53d^..9bbe53d`) | **349** |

**Instrument B - whole-file comment-strip, a different cut.** A removes comment lines *from the diff*; B strips them *from both whole files* and compares what is left, which additionally catches a line moving between comment and code - something A structurally cannot see.

| file | non-comment lines | result |
|---|---|---|
| `crates/muxsmith-core/src/identify.rs` | 483 | byte-identical |
| `crates/muxsmith-core/src/pipeline.rs` | 88 | byte-identical |
| `src-tauri/src/run.rs` | 1169 | byte-identical |

Fired control for B, same method across `9bbe53d`: `crates/muxsmith-core/src/lib.rs` and `src-tauri/src/run.rs` both report CODE-DIFFERS, so the comparison can distinguish states.

Instrument B is also the proof of MEDIUM-1's licence scope: `identify.rs`'s entire non-comment content is unchanged, so no struct, field, type or visibility change is representable in this commit.

Sanity guard, not a gate re-run (a doc-only round can still break `cargo doc`, which is the whole content of LOW-2): `cargo fmt --all --check` clean, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` exits 0.

## 3. LOW-2's proof, both halves re-run

**Fire, diagnostic class - fresh probe, fresh path.** A standalone rustdoc probe carrying the pre-fix doc line verbatim errors under `-D warnings`:

```
error: unresolved link to `plan_batch`
 --> .../lowtwo_probe.rs:5:55
  |                       ^^^^^^^^^^ no item named `plan_batch` in scope
```

**Fire, exact location.** `src-tauri/src/run.rs:359:55` rests on my own live measurement of the pre-fix tree in the first pass, taken before any fix existed. I cannot re-take it now: reproducing it requires the old line in the tree, and this pass is read-only. Stated as provenance, not re-asserted as a fresh measurement.

**Green - the real workspace, post-fix.** `RUSTDOCFLAGS="--document-private-items" cargo doc --workspace --no-deps`, fingerprint-forced so all four crates were actually re-documented (`xtask`, `muxsmith-core`, `muxsmith-cli`, `muxsmith-gui`): **0** occurrences of `unresolved link`, exit 0.

**Control that the green is genuine resolution and not an unchecked pattern.** My probe's second case is the requalified form, and in the isolated probe crate it *also* errors - `no item named muxsmith_core in scope` - because that crate has no such dependency. So rustdoc does check the long path form rather than skipping it; the zero in the real workspace, where `src-tauri` does depend on `muxsmith-core`, is therefore a resolution and not silence.

## 4. The `--document-private-items` cost - correction accepted, three dimensions added

**Your correction stands and mine was the narrower question.** My first-pass number counted `unresolved link` occurrences; it answered that question correctly and does not bound the flag's cost under `-D warnings`, where ambiguity is a different diagnostic in the same lint. Complete enumeration on the post-fix tree, every warning headline, not a filtered count:

```
      1 warning: `muxsmith-gui` (lib doc) generated 2 warnings
      2 warning: `run` is both a function and a module
```

Two, both the `[`run`]` ambiguity between the function and the private module, matching your reproduction. Three lines total with the one LOW-2 removed. The corrected ROADMAP figure is right.

Three dimensions neither of us had measured. Two are now measured; the third is a scope gap in the entry itself.

**(a) The change has two consuming sites and the entry names one - the only one I would call blocking for the adopting pass.** The doc step exists twice:

- `BUILDING.md:76` - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
- `.github/workflows/ci.yml:94-98` - the same command with `env: RUSTDOCFLAGS: "-D warnings"`

The ROADMAP entry says "in BUILDING.md's gate block". Landing it there only leaves CI running the unflagged form, so the blind spot survives in the place that actually enforces it on every push, and the two artifacts silently diverge. This is `proc-normative-count-recomputed` trigger 2's recorded multi-set shape - its plan-8 T5 occurrence is literally "the new check joined two sets and only one was swept", with `BUILDING.md` as the missed one; here the direction is reversed. The `ci.yml` step's own comment supplies the corroborating precedent: "a broken intra-doc link (queue.rs:73, linking a private item) rotted silently since Plan 4 until this task". Recommendation: the entry's vehicle sentence should name both sites explicitly.

**(b) Cross-platform residual - measured, empty.** `ci.yml` runs the doc step on all legs and the repo's own gate carries a cross-target lint precisely because cfg-gated items differ per platform, so a Linux-only measurement does not bound the flag's cost for private cfg-gated items. Measured: `RUSTDOCFLAGS="--document-private-items" cargo doc --workspace --no-deps --target x86_64-pc-windows-msvc` exits 0 with the same two ambiguity warnings and nothing else (the two `GNU compiler is not supported for this target` lines are pre-existing build-script noise, unrelated to rustdoc). The Windows leg costs no more than the Linux one; worth pasting into the entry so the adopting pass does not re-derive it.

**(c) Doctest exposure - bounded to zero, with a placement caveat.** If the flag were ever set as a global `RUSTDOCFLAGS` export rather than on the doc step, private-item doc examples would start being collected as doctests. Measured over `crates` and `src-tauri`, hunting a code fence inside any doc comment:

```
grep -rn --include="*.rs" -E '^[[:space:]]*(///|//!).*```' crates src-tauri | wc -l
0
```

So the class is empty today. Keep it on the doc step regardless; the entry already implies that, and now the reason is recorded.

**Lint classes the flag newly evaluates on private docs**, for completeness, since "what else could it cost" is a class question and not only a count: `invalid_html_tags`, `bare_urls`, `invalid_rust_codeblocks` and `redundant_explicit_links` all begin applying to private items' docs. All are silent today - the enumeration above accounts for every diagnostic the workspace emits, so nothing hides behind the two. One class moves the other way and can only help: `private_intra_doc_links`, which fires when a *public* item links a private one, is suppressed by the flag because the target becomes documented.

## Delta findings

None. No new defect, no regression, nothing carried forward. The three closures are complete and the round introduced nothing.

## Delta HARVEST

**The fix round is the clean counter-example to the first pass's dominant pattern.** Round 1's finding was that the gate cannot see prose; this round changed nothing *but* prose and proved it with two mutually independent instruments plus fired controls, which is the discipline `proc-verification-step-must-be-falsifiable` asks for and rarely gets on a change this small. Worth naming as interior calibration: a doc-only claim is an absence claim ("no code changed"), and it was treated as one.

**One process observation for the controller, not a finding.** The brief's parentage slip (`fed55be` "on top of `9bbe53d`") would have been invisible to any reviewer who took the brief's diff command at face value: the command runs, produces a plausible diff, and silently includes a second commit's ROADMAP and ledger writes. `proc-57-briefs-not-ground-truth` covers it; the cheap standing handle is that a delta dispatch names the range as `<parent>..<commit>` or simply as `git show <commit>`, never as a pair of SHAs the reader must assume are adjacent.

**Method note worth keeping for the next private-items question.** A count of one diagnostic class is not a cost measurement for a lint flag: my `unresolved link` count was right and still under-answered the question, because a lint's diagnostics are a family (unresolved, ambiguous, and the four sibling rustdoc lints the flag also switches on for private items). The handle is readable: when you measure the cost of enabling a lint or a lint-widening flag, enumerate every emitted diagnostic headline and count the classes, never grep for the one you already have in mind.
