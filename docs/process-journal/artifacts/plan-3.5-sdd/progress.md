# Plan 3.5 progress ledger (mkvtoolnix parity fixes)

Plan: docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md
Decisions: docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md (D19-D21)
Base (pre-implementation): b04c4a2
Execution: subagent-driven-development (SI-1). Per-task gate: cargo test --workspace && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check.

(Plan 3's ledger content is archived at docs/process-journal/artifacts/plan-3-sdd/progress.md; this file now tracks Plan 3.5.)

## Tasks

Task 1: complete (commit 91b19eb, review clean spec+quality, controller gate green: 208 tests + fmt/clippy(-D warnings)/deny). TracksCfg mirrors AttachmentsCfg, unmatched default drop via drop_policy(); rules deliberately no serde default (tracks stays mandatory). Migration exhaustive (reviewer re-scanned repo). spec 4.1/4.5 updated.
Task 2: complete (commit b57abf4, review clean spec+quality, controller gate green: 209 tests + fmt/clippy/deny). Plan.keep_unmatched from tracks.unmatched==Keep; command.rs push_track_selection early-returns for the primary group under keep. Reviewer confirmed donor-isolation is STRUCTURAL (input_groups always makes group 0 the primary; guard source==plan.source can never match a donor). 6 pre-existing Plan{} literals patched keep_unmatched:false.
Task 3: complete (commit 342ea42, review clean spec+quality ZERO findings, controller gate green: 210 tests inc. the live test + fmt/clippy/deny). SI-3 CONFIRMED against mkvmerge v100: keep_unmatched ordering only id1 of a 3-track source (ALPHA/BRAVO/CHARLIE) yields BRAVO,ALPHA,CHARLIE (ordered first, unlisted after in source order). D20 assumption holds; NO planner change needed. Test self-skips when mkvmerge absent (CI parity).
Task 4: complete (commit 1676b7c, review clean spec+quality, controller gate green: 211 tests + fmt/clippy/deny). language-tags 0.3.2 added (MIT/Apache-2.0, no transitive deps, deny.toml untouched). is_valid_value = normalize().is_some() || LanguageTag::parse().is_ok(); swapped walk_exact_languages + resolve_changes (monotonic widening, ISO path preserved). DEVIATION (correct+necessary): two regression fixtures zz/zzz were well-formed BCP-47 so newly accepted -> changed to `notalanguage` (len>8 fails well-formedness); reviewer empirically confirmed. pt-BR positive test asserts NO InvalidPropertyValue.
Task 5: complete (commit fba2fbc, review clean spec+quality, controller gate green: workspace 0 failed inc. 75 lib + fmt/clippy/deny). lang_eq: ISO normalize arm unchanged/first, then canonical_tag (parse().canonicalize()) case-insensitive compare, then raw fallback. Reviewer cross-checked crate SOURCE: pt Suppress-Script=Latn so pt-Latn-BR==pt-BR; zh has none so Hans!=Hant; region never suppressed so pt-BR!=pt-PT; iw==he via preferred-value. Doc-comment corrected (xx-YY canonicalizes to itself, not None - brief draft was wrong). spec 4.3 exact-principle paragraph added. mkvmerge --normalize-language-ietf default = canonical (comment).
Task 6: complete (commit 2b08de4, review clean spec+quality ZERO findings, controller gate green: workspace 0 failed inc. catalog_completeness + fmt/clippy/deny). DiagCode::UnsupportedSource + unsupported-source.ftl; planner gate after skew, before rule resolution, mirrors UnidentifiableSource early-return (plan:None). Trigger `!container_recognized || !container_supported` ONLY (NOT is_identifiable); 3 tests: both OR false-branches + decision-#5 confirmatory (recognized+supported+zero-tracks stays MissingTrack). Donor-path gate deferred by decision (primary-only per D21).

=== ALL 6 TASKS COMPLETE at 2b08de4. Full range b04c4a2..2b08de4 (6 commits). Controller gate green (workspace 0 failed, fmt/clippy(-D warnings)/deny). ===

=== WHOLE-BRANCH REVIEW (opus) at 2b08de4: READY TO MERGE = YES. No Critical. ===
Reviewer drove real mkvmerge v100 for the cross-file case. 6 commits compose cleanly (D19 which-tracks-match / D20 how-plan-renders / D21 whether-resolution-runs = disjoint seams). D19 accept-liberal(parse)/match-precise(canonicalize) asymmetry sound; D21 gate predicate correct (not is_identifiable); tracks restructure complete (0 orphaned consumers, schema regenerated at runtime).
ONE Important (reviewer: NOT a defect, NOT a blocker): donor+keep TRACK ORDERING. A keep profile with ONLY a donor rule places the added donor track FIRST (ahead of primary video/audio): verified `( primary[P0,P1] ) --subtitle-tracks 0 ( donor[D] ) --track-order 1:0` -> [D,P0,P1]. This is the approved memo assumption #3 (explicitly-matched ordered first, kept-unmatched primary appended after), recoverable by adding explicit primary rules. Reviewer recommends fast follow-up: (a) one sentence in spec 4.5/keep docs on ordering; (b) a gated cross-file (primary+donor) keep test. PENDING ŞENOL: confirm donor-first ordering is acceptable vs primary-first-donor-appended, before doc+test.
Roll-up triage: #2 NoTrackRules config_path "tracks"->"tracks.rules" = fix-now-cheap (or defer, acceptable); #1 TracksCfg placement = defer; #3/#4/#5 = fine-as-is (reviewer: #3 is a real guard, not redundant; #5's discriminator lang_eq_canonical_forms_match is present).
ŞENOL CALLED B (2026-07-09): donors TRAIL. Reframing adopted: "keep = match to what is already there" so kept-unmatched primary tracks ARE matched -> listing them in --track-order keeps the invariant; primary is primary = first, donors trail. Memo D20 updated (reverses assumption #3). Consequence (not vetoed): explicit primary rules under keep change properties not position; reorder = drop mode.

Task 7 (review-driven, decision B): commit 51567d7, review PASS spec+quality no Critical, controller gate green (workspace 0 failed, fmt/clippy/deny). Under keep --track-order = all primary tracks (0:id source order) then donors (g:id rule order); Plan.primary_track_ids populated from primary ident; push_track_order branches on keep_unmatched, drop path unchanged; push_track_selection/properties untouched. Live test restructured to primary+donor `live_keep_donor_trails_primary`, verified real mkvmerge v100: primary[PA,PB]+donor[DONOR] -> --track-order 0:0,0:1,1:0 -> [PA,PB,DONOR]. config_path #2 fixed (tracks.rules). spec 4.5 updated.
  Task 7 review Important (coverage): keep+donor track-order branch had only the GATED live test (silently skips w/o mkvmerge, e.g. CI) -> no unconditional guard. FIX commit aa75025: deterministic Plan-literal unit test keep_unmatched_donor_trails_primary_track_order asserting `0:0,0:1,1:0`. Implementer verified it's a real guard (scratch-reverted keep branch two ways -> both FAIL). Controller final gate green.

=== PLAN 3.5 COMPLETE at aa75025. 8 commits (91b19eb..aa75025). All 7 tasks + 1 review-fix, each per-task-reviewed; opus whole-branch review READY-TO-MERGE (at 2b08de4, pre-Task-7); Task 7 (its recommended follow-up, decision B) reviewed + guarded. Controller gate green throughout (workspace 0 failed, fmt/clippy(-D warnings)/deny). CLOSE-OUT: SI-2 journal + salvage sdd artifacts + refresh HANDOFF + push. ===

## Minor findings roll-up (for final whole-branch review triage)

Task 1 minors (cosmetic, not defects):
- model.rs ~269-314: TracksCfg struct + drop_policy() inserted between AttachmentsCfg's struct/keep() and its `impl Default`, splitting a contiguous group. Move TracksCfg after `impl Default for AttachmentsCfg` for locality. No functional effect.
- validate.rs:60: NoTrackRules diagnostic config_path is still literal "tracks" though the empty check now tests `.rules`; "tracks.rules" would be more precise. No test asserts it; "tracks" defensible. Consider in a diagnostic-path pass.

Task 2 minor (test hygiene, not a defect):
- tests/command.rs ~72-100: keep golden's negative assertion lists `--audio-tracks` among suppressed flags, but audio is the MATCHED category here so a select-flag was never at risk; asymmetric/redundant but still correctly fails if the guard is removed. Harmless.

Task 4 minor (readability, not a defect):
- planner_resolution.rs:157 & :998: both regression tests reuse the literal `notalanguage`; a future reader can't tell the shared string is deliberate. Consider a named const if touched later.

Task 5 minor (test discrimination, not a defect):
- matcher.rs:276-282 lang_eq_preserves_meaningful_distinctions passes under the OLD raw-fallback too (its pairs are string-unequal anyway), so it doesn't discriminate old-vs-new; it is a valid FORWARD regression guard against a future canonicalization that would wrongly merge them. Inherited from plan Step 2.
