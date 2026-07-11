<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-8  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01NhXy83v9LdUqUwUz5ySjcg
  agent_id:           a86ef0097fe04f273
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a86ef0097fe04f273.jsonl
  dispatch_desc:      Review Task 8 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T21:59:23.594Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

I have everything needed. All sanctioned out-of-diff files verified (dry_run.rs parity, queue.rs/job.rs signatures, cli.ftl completeness, catalog_completeness.rs, i18n.rs). No suite re-run required.

### Spec Compliance
✅ Spec compliant. Every brief step (1-6) is present and faithful; all deferred work (Task 9 JSON document, Task 10 SIGINT, Task 11 e2e, NDJSON) is correctly absent.

- Step 1 clap `Run` variant matches the brief field-for-field (`cli.rs:57-83`), doc comments added to satisfy the crate's `missing_docs` lint - correct, not scope creep.
- Step 2 flow is identical to dry-run through `plan_batch` and builds `specs` via the exact `filter_map(|f| f.plan.as_ref()).map(|p| JobSpec{...})` the brief mandates (`run.rs:442-450`).
- Planning report prints FIRST via the shared `print_batch_human` (`run.rs:433-438`), byte-identical to dry-run's order (`dry_run.rs:92-97`).
- Empty-specs path folds and returns `diag_exit_code` without touching the queue (`run.rs:452-457`) - same exit as dry-run.
- Deferrals verified: `--json` only suppresses human lines (`run.rs:485-489, 498-500`); no `ctrlc` import/dep (Cargo.toml untouched); `run_cli.rs` tests only planning-failure paths; no NDJSON. The two `TODO(Task 9)` markers (`run.rs:400-403, 454`) are pure comments, not half-built features.

### Named-risk findings

1. **Event-drain threading** - No deadlock, drain terminates deterministically. `run_queue` takes `events: &Sender` and never clones it (`queue.rs:80-85, 134-163`); the sole `tx` is *moved into* the scoped queue closure (`run.rs:481`), so it drops when that closure returns after the batch completes, closing the channel and ending `for event in rx` (`run.rs:484-493`). Std mpsc is unbounded, so sends never block even if the main thread renders slowly. `handle.join()` then returns the already-finished result. A queue panic unwinds -> drops `tx` -> drain ends -> `.expect()` re-panics on the main thread (surfaces, not hangs). Verdict: correct.

2. **Exit fold** - `max(diag_exit_code, job_exit_code)` with the 130 override structurally ahead of it (`run.rs:502-509`); `job_exit_code` is 2-any-Failed / 1-any-Warning / else 0 (`run.rs:515-523`). fail_fast produces `Cancelled` queued jobs alongside the triggering `Failed` (`queue.rs:157-158`), so the fold yields 2 (not 130) since the caller `cancel` flag is untouched by fail_fast - correct. Empty-specs folds via `diag_exit_code` exactly like dry-run. Verdict: correct, Task-10-ready.

3. **Milestone thresholding** - Per-index state via `last_milestone: Vec<u8>` indexed by job index (`run.rs:546-559, 579`); interleaved jobs cannot cross-contaminate (test `thresholds_track_independently_per_job_index`). 0->90 jump renders 25/50/75 and a following 100 renders nothing (test `a_jump_past_several_thresholds...`); exact-25 hits print (sequence includes 25/50/75); repeated/regressing percent does not reprint (test `repeated_or_regressing...`); 100% never prints a progress line since `MILESTONES=[25,50,75]` - Finished covers completion. Verdict: correct.

4. **Planning-path parity** - Shared helpers `all_diags`/`diag_exit_code`/`print_batch_human` extracted into `commands/mod.rs:48-88`; dry_run.rs now calls them (`dry_run.rs:14, 96-98`), so there is a single source, no drift, no verbatim duplication. locate/list_languages error handling matches dry-run (`run.rs:397-419` vs `dry_run.rs:52-76`). Verdict: correct.

5. **Fluent keys** - All 8 keys present in `cli.ftl` and byte-for-byte identical to the brief's texts (verified against the full file); every key is consumed by `run.rs`; no orphans, no extras. Params align (`$count`/`$seconds` for warning, `$code` for failed). Verdict: correct. See caveat under Issues re: which test actually guards them.

### Judgment calls
1. **1-based index in `[i/n]`** - Correct. `JobEvent.index` is 0-based (`queue.rs:23`); rendering `index+1` gives `[1/N]..[N/N]`, the natural CLI progress convention (`[0/3]` would read as "0 of 3 done" at start). The brief's key text leaves the base unspecified, so this is a legitimate call resolved the right way.
2. **Progress renders the threshold value (25/50/75), not the raw percent** - Correct. The brief says "Progress printed ONLY at 25/50/75 threshold crossings"; rendering the threshold makes output deterministic regardless of mkvmerge's reporting granularity, which is what "milestone lines" means. The multi-line-per-jump behavior (one line per crossed threshold) is defensible and documented; it never silently drops a milestone. Both calls sound.

### Strengths
- Threading model is the correct reading of "drain events on the main thread": queue on a side thread (it blocks), live rendering on the calling thread. Sender ownership is reasoned about explicitly and is actually correct.
- Milestone renderer is cleanly separated from I/O (returns `Vec<String>`, caller does `println!`), making it genuinely unit-testable; the 13 tests hit the real edge cases (multi-threshold jump, repeat, regression, per-index isolation, `exit_code: None` -> "n/a", all four terminal states, exact summary string), not just the happy path.
- Shared-helper extraction is the right dedup call and leaves dry-run's behavior and tests unchanged.
- Deferrals are disciplined and the Task-10 130-branch is placed to match Task 10's own snippet, so it drops in without restructuring.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
- **`run-job-warning` renders "1 warnings" (plan-mandated).** `cli.ftl` `run-job-warning = ... warning ({ $count } warnings, ...)` has no plural form, so a single-warning job prints "1 warnings" (`cli.ftl`, verbatim from the brief). The implementer correctly did *not* introduce Fluent pluralization, since that would deviate from the brief's locked text (`report` "Design decisions" bullet). Flagging per the rubric: the defect is in the brief's mandated text, not the code; cosmetic, v1 CLI; the human decides whether to amend the spec.

#### Minor (Nice to Have)
- **The `catalog_completeness` guard named in the brief does not actually cover these keys.** `catalog_completeness.rs:10-15` iterates only `DiagCode::ALL`, not general `cli.ftl` message keys, so it does not guard `run-*`/`run-summary` (the brief's "catalog_completeness guards new keys" is inaccurate for them). Coverage is not actually missing: every one of the 8 keys is exercised by a milestone unit test that asserts on rendered text, and `renderer.msg` falls back to the raw id on a miss (`i18n.rs:45-53`), so a missing/misspelled key fails those tests. The implementer documented this precisely (`report` lines 99-103). Residual risk is only for *future* keys added without a rendered-text assertion; a completeness test over all `cli.ftl` keys would close it, but that is beyond this task's scope. Controller should be aware the named guard is not the thing protecting these keys.
- **`JobEvent::Started.output` is unused** by the renderer (`run.rs:568-575` looks up `self.outputs[*index]` instead). Not a defect - it is deliberate so all event variants share one lookup path (only `Started` carries `output` on the event), and both values derive from the same `specs` so they cannot drift. Noted only so it does not read as an oversight.

### Assessment
**Task quality:** Approved
**Reasoning:** The implementation is correct on every named risk, faithful to the brief, disciplined about deferrals, and well-tested at the right seam; the only findings are a plan-mandated cosmetic pluralization and a documentation/guard-mechanism caveat the implementer already surfaced, neither of which undermines trust in the code.