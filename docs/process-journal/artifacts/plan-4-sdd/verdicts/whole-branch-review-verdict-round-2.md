<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 2 of 2)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_015wPAmACsHJKzjUUUkHYEFX
  agent_id:           a44acd0735a4896d9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a44acd0735a4896d9.jsonl
  dispatch_desc:      Whole-branch review Plan 4
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-09T23:25:20.858Z
  continuation_trigger: The coordinator sent a message while you were working: The fix wave for your findings has 
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

# Fix-wave verification: 93d1a6b..9009d34 (whole branch now 7aec492..9009d34)

Independently re-verified on HEAD `9009d34`: **269 passed / 0 failed, fmt clean, clippy -D warnings clean, deny clean.** The 5-commit delta touches only the flagged surfaces plus their tests (+5 new tests = the 264→269 delta, exactly accounted for).

### Per-finding verdict

**Important 1 (Windows kill → `Warning` + partial kept) - RESOLVED** (`75c075f`).
The fix is the one I specified: `killed: Arc<AtomicBool>` shared between `LiveJob` and its Killer, set before `kill()`, `wait()` folds through the extracted pure `resolve_wait` (flag wins over raw OS status). The `RunningJob::wait` trait doc now states the cross-platform guarantee explicitly, so the contract `FakeJob` always encoded is binding on implementors. The RED route is legitimate and honestly reported: `Child::kill` is SIGKILL on Unix (empirically confirmed), so a live RED was undemonstrable here; the `resolve_wait` unit tests carry the regression pin, the gated script test proves the wiring. Two observations, both acceptable, neither a regression:
- The flag-wins rule means a job that completes naturally in the same instant as the kill sweep is recorded `Cancelled` and its (complete) output deleted. This is consistent with the newly documented contract ("`None` exactly when the Killer was invoked"), only reachable under cancellation where that job would have been killed anyway, and the window is milliseconds. Accepted.
- The live test's SIGKILLed `sh` orphans its `sleep 30` grandchild for up to 30s. Harmless (nothing waits on it, no hang path); not worth a change.

**Important 2 (`mkvmerge_found: false` on profile-load failure) - RESOLVED** (`db9f559`).
`config_only_json` parameterized with `Option<bool>`; key now absent when the lookup never ran, with RED-confirmed assertions in both CLI test files, and the existing locate-failure `== false` assertions untouched (regression guard for the accurate arm).

**Minor (cli.rs exit-contract doc missing 130) - RESOLVED** (`841db0b`). One line, correct.

**Minor (worker count uncapped by spec count) - RESOLVED** (`4b1ddbf`).
`worker_count` extracted pure and unit-tested; the report's epistemics are correct that `ConcurrencyTracker::max()` cannot prove the thread cap (spec-bounded regardless) - the unit tests are the real guard, and the compile-failure RED conveniently prevented the 100k-thread test from ever running against unfixed code.

**Ledger #10 (found-but-broken arm) - RESOLVED** (`9009d34`).
`Some(true)` on the `list_languages()`-failure arm, RED-confirmed via the two existing stub tests; `config_only_json`'s doc now spells out the honest three-way semantics (`true` = found but query failed, `false` = lookup ran and failed, absent = never checked). Also worth noting: the Fix-2 implementer's scope discipline (flagging this arm instead of silently folding it in) and the coordinator routing it as a separate accepted commit is exactly how that should go.

### New concerns

One trivial, gate-invisible nit: `4b1ddbf`'s `QueueOpts::jobs` doc links `[worker_count]`, a private fn, from a public item - `cargo doc` emits *"public documentation for `jobs` links to private item"* (verified). The 4-command gate doesn't run rustdoc, so nothing fails today; drop the intra-doc brackets (plain `worker_count`) whenever the file is next touched, or it becomes noise if a doc gate is ever added. Not blocking.

Nothing else: no behavioral regressions found, the untouched backlog Minors from my review (non-UTF-8 `read_line`, empty-batch human summary, CLI test-helper duplication, `ConcurrencyTracker` in the public surface, `jobs[].index` doc note) remain correctly ledgered as v1.x.

### Final verdict

**Ready to push (7aec492..9009d34)? Yes - unconditional.**

**Reasoning:** Both Importants and the two accepted Minors are resolved with genuine RED/GREEN evidence and truthful documentation of what each test can and cannot prove; the gate is independently green at 269/0 with fmt/clippy/deny clean. The only residual is a rustdoc-warning nit outside the gate and the analyzed-and-accepted kill-race edge, neither of which affects any platform or path this branch ships. Post-push, confirm in the Actions log that the gated tests ran rather than skipped (T6's deferred verification).