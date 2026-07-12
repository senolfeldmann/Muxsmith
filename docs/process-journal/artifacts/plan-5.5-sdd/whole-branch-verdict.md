# Plan 5.5 whole-branch review verdict (T23)

## Close-state verification addendum (master@f25b02d, post fix wave 98e869a/a6976d0/f25b02d)

- (a) C1 VERIFIED: §5.2:283 and §9.2:404 both say info ("SchemaDrift info notice"); docs-wide grep finds no lingering "SchemaDrift ... warning" occurrence.
- (b) §5.2 rows VERIFIED against emitters: UnsupportedSource row exact (D21 predicate `!container_recognized || !container_supported` matches both planner branches; `kind=primary`/`kind=donor`+`donor` params match T9/T9.5 emission; error severity -> plan dropped via finalize_plans, both branches). EmptyPlan row matches `detect_empty_plans` incl. the D20 keep-passthrough exemption. UnknownExtension row matches `validate_extension_values` incl. degrade-on-unavailable and both param names. WorkerPanicked row's channel note is honest and accurate. One residual cosmetic nit, non-blocking: the WorkerPanicked severity cell says "info" while the token only ever accompanies a job reported Failed; no code assigns any severity (no Diagnostic is ever constructed), so nothing can drift against it and the channel note defuses the column - "n/a" would be tighter at next touch.
- (c) T4-m2 fix VERIFIED sound: `if let Some(killer) = ctl.killers().remove(&index) { killer(); }` - atomic remove-then-invoke on the owned Arc; matches `cancel_job`'s existing invoke pattern; Killer is documented idempotent/best-effort (spawn.rs:19-22) and the live impl proves it (sticky `killed` flag, swallowed `kill()` result), so a concurrent `cancel_job` double-invoke is harmless. The killer's own `Arc<Mutex<Child>>` clone survives the panicked worker's unwind (dropping std `Child` does not reap the process), so the kill genuinely reaches the leaked child - exactly the case the fix targets.
- (d) M1/M2/M3 VERIFIED present in the ROADMAP idiomacy named-input list (docs/ROADMAP.md:171-174), alongside the code-adjacent DEFERs; I2 has its own mixed-language `allowed`-param polish entry.

**Final branch verdict: READY** (ready to close; the one residual is a doc-wording nit that cannot drift into code, routed to next-touch).

---

Reviewer: final whole-branch (Fable 5). Range: e8e85d9..befc74e (74 commits, all tasks merged).
Inputs: plan (f0df4d8 state), spec + plan-5.5 memo, progress ledger, full diff package (read in full), five focused worktree checks against master@befc74e (named per finding).

## Cross-task findings

### Critical (Must Fix before close)

- **C1 - Spec records SchemaDrift as `warning`; ruling and code say `info`.**
  `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:280` (§5.2 table) and `:400` (§9.2 prose, "emits a `SchemaDrift` warning") both say warning. Şenol's ruling (D32 addendum RESOLVED; plan Task 16.5: "info severity") and the code say info: `crates/muxsmith-core/src/planner.rs:402` (`Diagnostic::info(DiagCode::SchemaDrift, ...)`), `crates/muxsmith-core/src/report/mod.rs:163` (rustdoc: "info severity"), test `schema_drift_fires_once_per_batch_with_the_max_found_version` asserts `Severity::Info`. Introduced by ce4fae1, the T16.5 review's own spec amendment - a transcription error no task-scoped review could see because each side is locally consistent. Under the "spec wins on conflict" doctrine the spec now instructs a future session to flip the code the wrong way against an explicit owner ruling. Fix: one word in two places. Verified at HEAD (worktree check 1).

### Important (Should Fix)

- **I1 - §5.2 diagnostic catalog missing `EmptyPlan` and `UnknownExtension` rows.**
  `grep -n "EmptyPlan\|UnknownExtension\|WorkerPanicked" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` returns nothing (worktree check 2). The same plan's other new codes (SchemaDrift, SuggestionPartition, RawProperty, RawOnKnownProperty) WERE added to the table, and the table carries config-time siblings (EmptyMatchList, CodecKindExactOnly), so the omission is cross-task spec-maintenance drift, not a scoping decision: T16/T13/T16.5 amended the catalog, T5/T5.9/T6 did not. Two shipped, user-visible diagnostics are absent from the authoritative contract. `WorkerPanicked` is executor-scope; a §6 sentence is optional, but decide it explicitly. Fix: two table rows (+ optional §6 line).

- **I2 - Core emits English prose through the `allowed` param (2 sites); renders mixed-language in de mode.**
  `crates/muxsmith-core/src/planner.rs:428` (`walk_exact_languages`) and `:841` (`resolve_changes`, added by T9 item ix) both set `.with("allowed", "a valid ISO 639/BCP-47 language code")` (worktree check 3). A German user with an invalid `changes.language` reads: "Der Wert ... ist nicht gültig. Zulässige Werte sind unter anderem: a valid ISO 639/BCP-47 language code." This is core-authored English in a param, not third-party pass-through, and it is not on §8.4's exception list. Pre-existing at :428; T9(ix) faithfully duplicated the pattern and T21 turned it user-visible - exactly the merged-whole class. Correct fix is catalog-side (e.g. `invalid-property-value` gains a `kind` selector, or `language` gets its own domain-hint message), too large for the close: **DEFER with a ROADMAP pre-1.0-polish line** (the bilingual launch is a pre-1.0 gate; a mixed-language diagnostic on the marquee language property undercuts it).

- **I3 (ledger items T4-i1 / T4-i2, confirmed at HEAD, triaged below).** Single `eprintln!` in core (`queue.rs:396`, worktree check 4) and `JobOutcome.errors` carrying the core-composed `"worker-panicked: job N"` string that no live surface renders through the catalog. Both DEFER to Plan 6 with ROADMAP lines (see triage).

### Minor

- **M1** - de catalog headers (all six `locales/de/*.ftl`) claim "keys, placeables and selector structure mirror it and are parity-enforced by scripts/check-i18n.mjs". The script enforces **id** parity only; placeable-name and selector-structure drift in de is not machine-checked (the e2e real-parse guard catches syntax, not a wrong `{$param}` name). Overclaiming comment; manual review of all six catalogs found no actual placeable drift. Fix wording at next touch, or accept.
- **M2** - `overlap_conflicts()` (planner.rs) re-parses claimant indices out of the rendered `rules` param string ("tracks[0], tracks[1]"). Core re-consuming its own display-formatted param works today but has no type-level link; a param format change silently kills overlap suggestions. Idiomacy-review candidate.
- **M3** - `report/mod.rs` UnknownExtension rustdoc says "Batch-wide, once per batch"; emission is once per offending list entry (per batch walk). Doc nuance, next touch.

### Cross-task interactions verified clean (the review's named risks)

- **T5/T5.9 extension walks vs T16 `raw:`**: orthogonal by construction - `raw:` exists only as a match-expression property-key prefix; `extensions` are plain string lists with no property namespace. No bypass path exists. Coherent.
- **T13/T18 suggestion engine vs T6 post-finalize EmptyPlan**: the interaction (simulations reject candidates that newly produce empty plans, via the multiset nothing-new guard) is **deliberately pinned**, not accidental: `tc_c_batch_unsafe_overlap_narrowing_is_rejected_by_the_multiset_guard` (crates/muxsmith-core/tests/suggestions.rs) constructs exactly this case and proves both halves (resolves in isolation, EmptyPlan collateral on the batch).
- **T9/T9.5/T16/T16.5/T19 key changes vs T21 de**: every en key that changed did so either before the T21 translation (overlapping-rules `$rules`, lint `tracks[N]`, unsupported-source donor variant, plural selectors) or landed bilingual after it (T21.5 gui-settings, T16.5 schema-drift). `node scripts/check-i18n.mjs` at HEAD: green, 1 locale x 6 catalogs (worktree check 5). Register of the post-gate de additions matches the Şenol-corrected terminology (Stapel, Meldungen, Verweis, du-Imperativ). **No semantically stale de message found.**
- **T22 snapshots vs later merges / mkvmerge 97-vs-100 legs**: no snapshot embeds mkvmerge-version-dependent text (the version/duration filters exist but nothing currently needs them); live-gated snapshots pin only version-independent lines. mkvmerge v100 reports `identification_format_version: 20` == pinned (verified live, worktree check; mkvtoolnix source pins ID_JSON_FORMAT_VERSION=20), so SchemaDrift cannot fire on any CI leg and cannot destabilize snapshots or gated exit-code assertions. Snapshot suite is stable and CI-convergent.
- **T2 skip-marker contract**: all gated tests added after T2 (T5.9, T6, T8, T22 conversions) use the exact `"mkvmerge not found; skipping"` string; the unenforced contract held through the whole plan (T2-m1 defer stands).

## Constraint-sweep results

- Core prose-free (§8.4): **PASS with exceptions** - I2 (`allowed` prose param, 2 sites) new; T4-i1/i2 known and triaged; documented exceptions (clap, `detail` pass-through, IdentifyError Display) all honored, IdentifyError now explicitly spec-listed.
- Fixture-guard completeness (C1 ledger constraint): **PASS** - exhaustive match covers every DiagCode incl. SchemaDrift/RawProperty/RawOnKnownProperty/WorkerPanicked/EmptyPlan/UnknownExtension/SuggestionPartition; fixtures cross-checked faithful to emitter sites; the two single-fixture blind spots are documented in-file and the InvalidPropertyValue one is pinned by a real-emitter test.
- Spec-vs-code coherence after the amendments: **FAIL on one point** (C1 SchemaDrift severity), **incomplete on one** (I1 missing rows); §4.2/§4.3/§4.4/§4.6/§5.3/§5.4/§8.4/§9.2 otherwise mutually consistent and consistent with code (raw: semantics, partition, extension validation, exception list all cross-checked).
- en/de parity at HEAD: **PASS** (check-i18n green; T9.5-era donor variant, T16.5 schema-drift, T21.5 settings keys all present in de, T21-register German).
- C2 (bilingual from T19 on): **PASS** (T21.5 and T16.5 landed en+de in the same commits).
- C3 (nine-part gate from T12): controller-verified per ledger; CI green at merge commits; not re-run here.
- SDD scratch namespaced + verdict files at creation: **PASS** (.superpowers/sdd/plan-5.5/ complete, 30+ verdict/report files).

## Minor triage table (roll-up funnel)

n-in: **37** ledger items. n-out: **3 FIX-NOW / 16 DEFER / 14 DISCARD / 4 RESOLVED in-plan**.

| id | verdict | vehicle / reason |
|---|---|---|
| T2-m1 (skip-marker unenforced contract) | DEFER | idiomacy review: shared marker const in tests/support, ci.yml grep comment points at it |
| T2-m2 (double test run in CI) | DISCARD | repo is public (minutes free); the rerun buys the no-silent-skip guarantee; verdict analysis found no better alternative |
| T2-m3 (apt exact-build pin fragile) | DISCARD | accepted pin-policy cost, documented in ci.yml; fails loudly, bump is one line |
| T2-m4 (97 vs 100 cross-leg divergence) | DISCARD | sanctioned by the per-manager pin decision, recorded in ci.yml |
| T8-m1 (three contains() loose pin) | DISCARD | superseded: T22 converted these to redacted snapshots pinning the composed line |
| T8-m2 (dry-run-summary key serves run too) | DEFER | idiomacy review: rename to batch-summary (touches en+de+allowlist+2 snapshots) |
| T3-m1 (redundant fn-level cfg(unix)) | DEFER | next touch of executor_no_hang_live.rs; cosmetic, file-level gate governs |
| T5-m1 (locator doc-claim false) | RESOLVED | routed in-plan -> Task 5.9, landed |
| T5-m2 (known_extensions default-None idiom inversion) | DEFER | idiomacy review: required method, fakes return None explicitly |
| T14-m1 (prop_assume vs prop_assert) | DEFER | idiomacy review (test-quality; generator regressions currently die opaquely) |
| T14-m2 (D4 separator-injection unexercisable) | DISCARD | inherent v1 surface limit, recorded in test comment |
| T14-m3 (test-side logic mirror) | DEFER | idiomacy review; deliberate basename-scoped mirror, covered by unit tests meanwhile |
| T13-m1 (fixless file dropped from partition) | DEFER | idiomacy review: invariant comment or "unresolvable" group at the best=None skip (unreachable under v1 id-uniqueness) |
| T13-m2 (Fluent {$fix} column-0 indent) | DISCARD | cosmetic, pinned by renderer tests; revisit only on output-polish pass |
| T13-m3 ("batch-wide" phrase misreadable) | DISCARD | report-note only, no code impact |
| T11-m1 (_comment fixture-note ad hoc) | DEFER | trigger: second use -> promote to written convention (BUILDING.md test section) |
| T4-i1 (eprintln in core, queue.rs:396) | DEFER | Plan 6 / v1.x ROADMAP line: core logging facade (single site confirmed at HEAD) |
| T4-i2 (worker-panicked renders on no surface) | DEFER | Plan 6 (jobs-surface rendering): route JobOutcome.errors codes through the catalog; ROADMAP line |
| T4-m1 (lock_active doc overclaim) | DEFER | idiomacy review; doc-precision only (reads/method calls under lock vs "single assignment" claim) |
| T4-m2 (child-process leak on post-spawn panic) | **FIX-NOW** | queue.rs:398 removes the killer without invoking it; invoke-then-remove is one line, leaked mkvmerge keeps writing an output the queue reported Failed |
| T12-m1 ("module-private" phrasing) | DISCARD | documented-inert nit |
| T12-m2 (--workspace deviation) | DISCARD | documented-inert, recorded in verdict |
| T6-m1 (no attachments/chapters-only EmptyPlan test) | DEFER | idiomacy review / Plan 6 test rider (doc value only; D20 keep-path covered) |
| T7-m1 (attachment-donor exposure) | RESOLVED | routed in-plan -> Tasks 7.5+7.6, #7 class closed by construction |
| T7.5-m1 (resolve_file top doc stale re chapters) | **FIX-NOW** | one-line doc fix: the return-doc names track+attachment donors, omits chapters (7.6) - the completeness comment it points at is the class-closure guard, so keep it exact |
| T9-m(iv) (config_diags JSON unsorted vs validate) | DEFER | Plan 6 / idiomacy: sort JSON config_diagnostics for parity; consumers key on codes, ordering cosmetic |
| T9-m(viii) (exception broader than pass-through) | RESOLVED | spec §8.4 now names the IdentifyError Display framing explicitly |
| T9.5-m1 (report overstated no-alternative claim) | DISCARD | report prose, recorded |
| T16-m1 (bare `raw:` empty property accepted) | DEFER | Plan 6: reject empty bare name at validate; currently visible-but-odd, never silent |
| T18-m1 (dead `edited` fixture + `let _ = edited;` in TC-A) | **FIX-NOW** | delete the dead fixture string; trivial, bundle with the fixer dispatch |
| T18-m2 (SuggestionsCapped comment placement) | DISCARD | comment-filing nit |
| T19-m1 (report fixture-value misquote) | DISCARD | report prose, recorded |
| T19-m2 (visibleText strips U+2066-2069, wider than needed) | DISCARD | stripping all four isolate marks is a strict superset and safer |
| T20-m1 (regex-vs-parser divergence on malformed ftl) | RESOLVED | c229400: e2e real-Fluent-parse guard covers every catalog of every locale incl. cli.ftl |
| T20-m2 (no check-i18n.mjs self-test despite grown logic) | DEFER | v1.x test-hardening line: fixture self-test (plan T20's own condition fired; e2e guard covers only the parser-blindness half) |
| T21.5-m1 (live locale switch) | DEFER | already recorded as Plan 6 candidate |
| T22-m1 (regex-wording snapshots break on cargo update) | DISCARD | deliberate, documented decision (breaking on regex bump is intended signal) |

Open owner item (not a funnel entry): Şenol-Veto still open on the T21.5 hint texts (reported in the task summary); surface at close.

## Assessment

**Branch quality: Needs fixes** (small, pre-close)

**Reasoning:** The branch is functionally sound - every named cross-task risk checked out clean or deliberately pinned, en/de parity and snapshot stability hold at HEAD, and the fixture guard is complete. What blocks the close is documentation-of-record drift the task reviews structurally could not see: the spec now contradicts the owner's SchemaDrift severity ruling (C1, two words) and omits two shipped diagnostics from its catalog (I1, two rows), which under spec-wins doctrine is live ammunition for a future wrong "fix". One fixer dispatch covering C1+I1 plus the three FIX-NOW minors (T4-m2 killer invoke, T7.5-m1 doc line, T18-m1 dead fixture), then gate re-run, and the branch is ready to close; I2 and the DEFER set need their ROADMAP/vehicle lines written at close per the funnel.
