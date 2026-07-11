<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-2  (round 1 of 2)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_019rYCYvnoPqv4DLdDW6fHnf
  agent_id:           a37edc817e9001305
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a37edc817e9001305.jsonl
  dispatch_desc:      Review Task 2 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-09T20:33:01.780Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

I have everything needed: the diff is fully contained, and I've read Task 1's seam (`spawn.rs`) for the FakeSpawner/next_line/wait/SpawnError semantics the named risks depend on. All line references below are to `crates/muxsmith-core/src/executor/job.rs` (mapped from the new-file diff).

### Spec Compliance
- ✅ Spec compliant. All five interfaces match the brief verbatim: `JobSpec` derives + fields (job.rs:20-26), `JobState { Ok, Warning, Failed, Cancelled }` with `Debug, Clone, Copy, PartialEq, Eq, Serialize` + `#[serde(rename_all = "snake_case")]` (job.rs:27-40), `JobOutcome` fields + `Serialize` (job.rs:43-56), `JobProgress { Percent(u8), WarningLine(String), ErrorLine(String) }` non-Serialize (job.rs:59-67), `run_job` signature exact (job.rs:78-83). `pub mod job;` added (mod.rs).
- ✅ All six mandated tests present and faithful to their scenarios: exit_zero/kept (job.rs:183-197), exit_one/warning-captured (job.rs:199-220), exit_two/failed-deleted with pre-created partial (job.rs:222-239), killed-under-cancel/deleted (job.rs:241-255), progress-percent 25/50/100 (job.rs:257-282), parent-dir-created (job.rs:284-295). The kill test scripts `exit: None` rather than invoking the killer, but that yields the identical `wait()==None` + `cancel==true` input to `run_job`, so it is a faithful test of the Cancelled arm.
- ⚠️ Cannot verify from diff: `#![deny(missing_docs)]` is a crate attribute (lib.rs), not in this diff. Moot for trust — every public item here carries a doc comment, so the code satisfies it regardless; controller need not act.

### Named-risk findings

**Risk 1 — spawn-failure deletes a pre-existing file: CONFIRMED (Important).** Trace: `spawner.spawn()` returns `Err(SpawnError(message))` (job.rs:92) → `finish(JobState::Failed, None, …, &spec.output, start)` (job.rs:93-100). `finish` runs `matches!(state, Failed | Cancelled)` → true → `delete_partial(&spec.output)` → `std::fs::remove_file(spec.output)` (job.rs:149-150, 164-166). Note `create_dir_all(parent)` (job.rs:86-88) already ran, but that only creates the directory, not the file. So: the process provably never ran, yet any file sitting at `spec.output` is deleted. Under an on_collision=overwrite plan, that file is a **pre-existing valid output from a prior run** — this silently destroys it on an environment error (missing binary, non-UTF-8 path per `SpawnError`'s own doc). This is *not* defensible under D17: D17 deletes a *partial mkvmerge left behind*; on the spawn-failure path there is no partial, so the delete can only remove a file this job never wrote. The implementer flagged the state-mapping-to-Failed but missed that Failed drags delete-partial along. Fix is trivial and local: the `Err` arm should assemble the outcome without routing through the deleting `finish` (mkvmerge wrote nothing, so nothing to delete).

**Risk 2 — drain-then-wait ordering: SATISFIED.** `while let Some(line) = running.next_line() { … }` (job.rs:107-119) exits only on `next_line()==None` (EOF); `running.wait()` is called only after the loop (job.rs:121). The loop body has no `break`/`continue`/early-return, and the only early return before `wait()` is the spawn-failure arm (job.rs:92-100), which returns without ever calling `wait()` (correct — no process to reap). No path reaches `wait()` with lines pending.

**Risk 3 — percent parser on malformed lines: SKIPS, NEVER PANICS.** `parse_progress` (job.rs:133-137) = `strip_prefix("#GUI#progress ")` → `strip_suffix('%')` → `parse().ok()`. `#GUI#progress` w/o number → no `%` or empty parse → `None` → line falls through the else-if chain and is ignored. Non-numeric (`abc%`) → parse Err → `None` → ignored. No panic on any input. One wart: `parse::<u8>()` accepts 101-255, so `#GUI#progress 200%` surfaces as `Percent(200)` unclamped (256+ overflow to `None`, dropped). Real grammar caps at 100%, so this is a robustness nit, not a live bug (Minor). Matches the brief's literal parser spec ("strip prefix + trailing `%` -> u8").

### Strengths
- Exit-mapping (job.rs:122-127) is total and unambiguous: every `(exit_code, cancel)` pair lands in exactly one state, `_` catch-all covers `Some(2)`, other codes, and `None`-without-cancel (unexplained death → Failed), matching the brief.
- Clean `finish`/`parse_progress`/`delete_partial` factoring; both `finish` call sites share one delete-partial gate.
- Tests are genuinely behavioral, not tautologies: all drive `run_job` through the real `FakeSpawner`, and file-existence assertions hit a real `tempfile::tempdir()` (partial written, then asserted gone/kept through the real fs). RED evidence (all six panic on `todo!()`) corroborates non-vacuity.
- Doc comments on every public and private item; ASCII-only punctuation, no prose strings in core, correct derive set and serde rename.
- Parser prefix handling correct: tag stripped, filename retained in warnings (`'seed.srt': …`), disjoint prefixes so try-order is immaterial.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
- **Spawn-failure path deletes a pre-existing valid file (job.rs:92-100 → 149-150 → 164-166).** Silent data loss of a file this job never wrote, on an environment error, under overwrite semantics. Narrow trigger (spawn failure + file at output) and untested, so not Critical, but it must be fixed before the task is trusted; the human may choose to escalate given "data loss" weight. Fix: don't run delete-partial when the process never spawned.

#### Minor (Nice to Have)
- **`delete_partial` ignores *all* IO errors, not just `NotFound` (job.rs:164-166).** Brief said "ignoring NotFound"; this swallows permission/other errors too. On a failed remove (e.g. EACCES), a broken partial stays in the tree and a later on_collision=skip rerun accepts it as valid — the exact hazard delete-partial exists to prevent. Genuinely unreportable given `JobOutcome` has no channel, so defensible, but broader than the brief's letter.
- **Percent 101-255 pass through unclamped** (job.rs:133-137); see Risk 3.
- **Warning path (exit 1) lacks an output-kept assertion.** `exit_one_is_warning_with_captured_lines` (job.rs:199-220) checks state + captured text but not that the output survives; and no test asserts `WarningLine`/`ErrorLine` reach `on_progress`. Non-deletion on non-Failed states is covered indirectly by exit_zero, but the Warning branch's retention and the on_progress surfacing of warning/error lines are untested.

### Assessment
**Task quality:** Needs fixes
**Reasoning:** Interfaces and all six mandated tests are exact and faithful, ordering and parser risks are clean, but the spawn-failure path routes through delete-partial and can silently destroy a pre-existing valid output — a real, unflagged data-loss bug that should be fixed before trusting the task.