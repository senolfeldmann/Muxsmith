# Task 4 review verdict: core integration tests (Stream C)

Base `0b3149a` -> Head `486effe`, worktree `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-c` (read-only).

## Spec Compliance

1. `command.rs` (10 golden-argv sites) - vec![...].into_iter().map(String::from).collect() -> array literal `[...]`, relying on `Vec<T>: PartialEq<[U; N]>` + `String: PartialEq<&str>`. All 10 sites checked; every element sequence is byte-identical to its pre-diff form. ✅
2. `command_integration.rs:121` - `expected: Vec<String>` binding removed, `assert_eq!` now compares directly against a mixed `&'static str`/`&String` array (LUB deref-coercion). Content identical line-for-line to the old `expected` vec. ✅
3. `planner_resolution.rs` fully-qualified-path collapse - `use` extended with `AttachmentPlan, ChapterSource, FileReport, PrimaryAttachments, TagFlags, TitleAction` (planner) and `Diagnostic` (report); every qualified reference in the diff uses the short name afterward, no `muxsmith_core::planner::`/`::report::` residue left in the file. Count reconciliation (line 24 + line 2285 vs. the brief's literal "20") verified: brief's anchor list re-expands to 19 distinct occurrences (the `74-87` range is 5 sites, line 137 is 2 sites on one line), the full file has 21 real occurrences, and the diff shows **both** line 24 (`plan_one`'s return type) and line 2285 (`PrimaryAttachments::Subset(vec![1,2])`) converted - i.e. all 21 were collapsed, not a selected 20. Same mechanical, zero-risk fix class as the rest (target names already bare-imported); exceeding the brief's approximate count by one is not a defect. ✅ (see House dimension)
4. `planner_resolution.rs:619` `is_some_and` - `!d.params.get("detail").unwrap_or(&String::new()).is_empty()` -> `d.params.get("detail").is_some_and(|s| !s.is_empty())`. Truth-table-equivalent (both mean "present and non-empty"; `None` -> false either way). ✅
5. NEW test `empty_plan_fires_when_only_attachments_and_chapters_resolve` (seed T6-m1) - placed directly after the keep-mode test as required; profile carries the optional non-matching `audio/de` rule (same shape as sibling EmptyPlan tests: `assignments.len()==1`, `track_id==None`) plus a real on-disk `attachments.rules[0].add` donor (`donors/Font.ttf`). Asserts `plan.attachments.add_files` non-empty, `plan.chapters == ChapterSource::Keep`, and exactly one `Severity::Warning` `DiagCode::EmptyPlan`. One-line spec-5.2 rationale present in the doc comment. Report includes a first-run isolated-test transcript showing `ok` (pin, not a bugfix). ✅ (first-run-pass claim itself: ⚠️ cannot verify from diff alone, no CI log in the diff)
6. `profile_load.rs:131-158` - both tests switched to the line-1-5 imports (`from_str`, `Format::Yaml`, `KeepDrop::{Drop,Keep}`). ✅
7. `suggestions.rs:194` yagni - `no_clobber_batch()` deleted; `let files = [...]` bound once and passed (`&files`) to both the initial `plan_multi` call and the per-suggestion-loop call; array content unchanged. ✅
8. `suggestions.rs:591` yagni - `partition_diags` deleted; its filter inlined at the one caller as a single combined predicate (`code == SuggestionPartition && kind == "group"`), same item set as before. `overlap_diags` (4 callers) untouched except idiom. ✅
9. `suggestions.rs` report/Diagnostic/Severity idiom - `Diagnostic, Severity` added to the report import; all remaining qualified sites (`Severity::Info` comparison, `overlap_diags` return type, and `plan()`'s `planner::Batch` return type) collapsed to short names. ✅
10. `validate_structure.rs:123` yagni - `for (snippet, _section) in [...]` -> `for snippet in [...]`, dead tuple element dropped, loop body untouched. ✅
11. `executor_no_hang_live.rs:30` dup - redundant `#[cfg(unix)]` removed from the one test; file-level `#![cfg(unix)]` (line 20) still gates the binary. ✅

Scope: `git diff --stat` (top of diff file) lists exactly the seven owned test files, nothing under `src/` or `prop_*.rs`. ✅
Unsigned commit / explicit staging: claimed in the report (`git log -1 --format=%G?` -> `N`, explicit `git add`); not observable from a unified diff. ⚠️ Cannot verify from diff alone.
Full nine-part gate: claimed green with per-suite counts; not re-run per instructions. ⚠️ Cannot verify from diff alone (trusted per task instructions, not re-executed).

## Strengths

- Every "idiom" collapse was checked element-by-element against the diff and is byte-identical to its pre-diff array/expression; no golden-argv content drifted and the `is_some_and` rewrite is a faithful truth-table match, not a silent weakening.
- The new boundary-pin test (item 5) is precise: it asserts the *mechanism* (donor genuinely resolved via a real on-disk file, not a mocked one) as well as the *outcome* (exactly one warning-severity `EmptyPlan`), closing off the two ways this could have been a weaker or vaguer pin.
- The anchor-list reconciliation for `planner_resolution.rs` was surfaced explicitly rather than silently resolved either way, with the actual arithmetic shown (19 literal + line 24 + line 2285 = 21) - this is exactly the "flag the open dimension, don't decide silently" behavior the process wants, and the chosen action (collapse all 21) is the safer, more-complete one, not a corner cut.
- Scope discipline is clean: diff touches only the seven owned files, no `src/` or `prop_*.rs` bleed.

## Issues

### Critical (Must Fix)
None.

### Important (Should Fix)
None. No weakened assertion, no behavior change, no scope violation found.

### Minor (Nice to Have)
- Report's own reconciliation prose is internally inconsistent: it computes "excluding line 24 ... and including line 2285 ... reconciles to exactly 20," then in the next sentence says both line 24 and line 2285 were collapsed (21 total) - which is what the diff actually shows. The action taken is fine (see Strengths); only the write-up contradicts itself about which set was implemented. Worth a one-line correction if the report is kept as a durable artifact.
- `suggestions.rs`'s pre-existing trailing-comma style inside the reformatted `assert!(...,)` (the `Severity::Info` comparison) is unchanged from before the diff - not introduced here, not a defect, noting only because it stood out on read-through.

## House dimension

No deviation from `docs/conventions.yaml` / `docs/process-conventions.yaml` found; nothing here rises to a Tier-2 pattern (single technical-code occurrence, `agent-emergent` promotion needs count 3 per the doctrine matrix, correctly self-assessed as "no new house pattern" in the report).

Harvested for the ledger (process observation, not yet a Tier-2 entry):
- **Task-brief anchor/count claims are not ground truth.** This brief's own prose ("the 20 fully-qualified paths") didn't match its own enumerated line list (19 by literal expansion, once `74-87`'s 5 sites and line 137's 2-on-one-line are unpacked), and the real file had 21. The implementer caught this by re-scanning the file rather than trusting the brief's count - consistent with `proc-07-verify-against-source`'s spirit (verify load-bearing claims against the actual artifact), but that entry is scoped to "tooling and dependency behavior." A brief's own line-count/anchor-list claims are a distinct, recurring failure surface (planner authoring over a large diff can miscount ranges and multi-symbol lines) worth watching for a second occurrence before promoting.

## Assessment

**Task quality:** Approved
**Reasoning:** All 11 brief items verified against the diff with no weakened or altered assertion; the one open judgment call (converting 21 vs. the brief's literal 20 fully-qualified-path occurrences in `planner_resolution.rs`) was transparently surfaced, arithmetically sound, and stayed within the same zero-risk mechanical fix class the brief specified. Scope, gate claims, and commit discipline are all consistent with the task's constraints.
