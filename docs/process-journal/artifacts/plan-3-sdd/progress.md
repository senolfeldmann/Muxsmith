# Plan 3 progress ledger (pure layer: resolution + command)

Plan: docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md
Decisions: docs/superpowers/specs/2026-07-09-plan-3-design-decisions.md (D7-D12)
Base (pre-implementation): 62d4956
Execution: subagent-driven-development (SI-1). Per-task gate: cargo test --workspace && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check.

(Plan 1's ledger content was archived to docs/process-journal/artifacts/plan-1-sdd/; this file now tracks Plan 3.)

## Tasks

Task 1: complete (commit b40c084, review clean, gate green: 167 tests, fmt/clippy/deny)
Task 2: complete (commit 8a2defc, review clean, tests green). Note: blanket impl<M: Matchable> Matchable for &M added (planner .iter().filter passes &&Track -> M=&Track); Task 3 attachment .iter().filter will rely on the same blanket impl.
Task 3: complete (commit 18457c6, tests green). Reviewer spec-flagged the trait DOC-COMMENT truthfulness update (naming Attachment as an implementor) under 'no trait changes'; controller adjudicated ACCEPT - it is a correct doc fix, not a contract/algebra change, and reverting would reintroduce a false comment.
Task 4: complete (commit 61dcf4f, review clean, tests green). Types in planner.rs match reference block; track_kind captured as Some(kind) via Vec<(u64,String)>; defaults-only confirmed.
Task 5: resolve changes + plan-time language validation - pending
Task 6: complete (commit 693c38f, review clean, tests green). resolve_title (keep/clear/template render, no-panic fallback to Keep, empty->Set("")) + resolve_tags; title Ctx adds source_stem to match validate.rs title.template allowed fields (closes latent silently-empty-field bug, reviewer-verified).
Task 7: complete (commit aa045db, review clean, tests green). resolve_chapters keep/drop/external mirrors track-external branch; config_path chapters.external; no id.identify on chapters file; error->no-plan via finalize; reuses Missing/AmbiguousExternal.
Task 8: complete (commit 3283552, review clean ZERO findings, tests green). resolve_attachments: single-pass first-match-wins select/drop, KeepAll(before empty check)/DropAll/Subset(sorted); adds all hits, BTreeSet-seen Vec::retain dedup preserving order, zero-hit->WARNING (plan survives); D10 primary-only; MissingExternal doc updated.
Task 9: complete (commits 78ee2e1 + fix d55f19d, re-review clean, tests green). command module: global section, input-group track selection, track-order. FIX: input_groups excludes track_id:None donor sources (primary always); regression test unmatched_donor_rule_opens_no_input_group. Category table for irregular flag names.
Task 10: complete (commit 61ab07c, review clean, tests green). push_track_properties: option via capability::settable().1, value_str (Bool 1/0, Str raw, else to_string), track_id-asc then property-asc; multi-group golden 0:0,0:1,1:0; Task 9 group logic untouched.
Task 11: complete (commit afa8074, review clean, tests green). push_group_{chapters,tags,attachments} at group start (order a/b/c before selection); --no-chapters every group on Drop|External; --no-*-tags; primary KeepAll/Subset(--attachments ids)/DropAll, donor always --no-attachments. Updated Task 10 golden with donor --no-attachments (contract lock, verified not a regression). COMMAND MODULE COMPLETE.
Task 12: complete (commit 0dcb116, review clean, 203 tests). Pure golden = FULL spec 4.1 reference example (fake Identify, 10 assignments, full-vector assert_eq); live mkvmerge test ran locally v100 exit 0, output re-identified; NO command.rs correction needed (argv accepted as-is); ( file ) grouping verified vs man page.

## Minor findings roll-up (for final whole-branch review triage)

Task 1 minors (edge-case coverage parity, not defects):
- parse_attachment: no test for wrong-typed required field (id as string) dropping the entry (parity with parse_track gap). identify.rs ~226
- chapters: no test for missing/non-numeric num_entries contributing 0. identify.rs ~172
- attachment with no `properties` key (vs empty {}) untested. identify.rs ~238
Task 2 minor (roll-up): matches_is_generic_over_matchable tests only M=Track directly; the &&Track (blanket &M) path is guarded only indirectly by planner tests. matcher.rs ~344.
Task 3 minors (roll-up): (a) no test for Attachment int properties id/size (exact: {size:100}); delegation is trivial passthrough. matcher.rs ~363. (b) redundant `use crate::identify::Attachment` in test (super::* already covers). matcher.rs:364.
Task 4 minor (roll-up): 1=> arm clones tuple out of a soon-dropped Vec; matched.into_iter().next().unwrap() would move. Cosmetic. planner.rs matched branch.
Task 5 minors (roll-up): (a) ELEVATED - non-Str changes.language branch (e.g. language: true) is a brief-REQUIRED behavior but untested; logic independently verified correct. Add a case in final review. planner_resolution.rs. (b) tests assert code+config_path but not the property/value params dict.
Task 6 minors (roll-up): (a) report claimed 7 new tests, diff has 6. (b) resolve_title/resolve_tags run before output.map so they execute even when plan is discarded (pure, negligible). (c) only 1 of 4 tags combos explicitly tested (mapping provably correct).
Task 7 minors (roll-up): (a) unexpected-keyword->Keep defensive branch untested (unreachable via validate; mirrors resolve_title gap). (b) tests use std::mem::forget(dir) leaking tempdirs - pre-existing pattern 7x in file + suggestions.rs; consider a cleanup pass.
Task 9 minor (roll-up): donor group ORDER uses first-appearance-among-Some-assignments; with a donor having mixed None/Some assignments across rules, relative donor order vs literal first-appearance could differ. Defensible reading, no golden arbitrates. Be conscious in Task 12. command.rs input_groups.
Task 10 minor (roll-up): value_str Bool arm `if *b {"1"} else {"0"}.to_string()` reads ambiguously (binds whole expr, correct); parens would clarify. command.rs ~194.
Task 12 minors (roll-up): (a) FakeIdent+lang() now duplicated 3x (planner_resolution, suggestions, command_integration) - past threshold, consider tests/support.rs. (b) live test uses no changes/non-default flags - broader live coverage manual only; natural Plan 4/5 increment.

=== ALL 12 TASKS COMPLETE at 0dcb116. Full gate green (203 tests, fmt/clippy/deny). Whole-branch review (opus) pending. ===

=== WHOLE-BRANCH REVIEW (opus) at 0dcb116: READY TO MERGE = YES. No Critical/Important. ===
Reviewer drove real mkvmerge v100: all 26 flags exist, attachment-id identity (-J id == --attachments selector) CONFIRMED, rich argv end-to-end exit 0, Matchable no track regression, D10/D12 correct.
Fixing now (cheap, worth it): M2 input_groups comment states FALSE reason (mkvmerge DOES accept empty groups; guard still right); M4 add non-Str changes.language test (required behavior, logic already correct); M10 bool to_string parens.
Deferred follow-ups (record in HANDOFF for Plan 4/cleanup): M1 richer gated live test (attachment+changes) - highest-value; M3 zero-track plan renders empty MKV with no diagnostic (planner empty-plan warning?); M5 with-attachments.json 0-based ids (real mkvmerge 1-based; code id-agnostic so harmless); M6 eager chapters/attach resolve on discarded-plan path; M7 FakeIdent+lang() 3x dup -> tests/support.rs, std::mem::forget tempdir leak; T5 batch-level settable-language option.
