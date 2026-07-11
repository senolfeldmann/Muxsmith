<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-4  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_016BFVT5crfWhWTwtPzrs1TG
  agent_id:           a488f3ba369d2d7ec
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a488f3ba369d2d7ec.jsonl
  dispatch_desc:      Review Task 4 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T20:09:39.797Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

Confirmed: `RunInputs.on_collision` was already `Option<CollisionPolicy>` with the fallback semantics documented and implemented in core (`planner.rs:249: run.on_collision.unwrap_or(profile.output.on_collision)`), predating this task. This task's job was purely to thread the CLI flag through to that pre-existing plumbing, which it does correctly.

### Spec Compliance
- ✅ Spec compliant. `CollisionArg` is CLI-local (`crates/muxsmith-cli/src/cli.rs:56-74`), core untouched, 1:1 mapping matches core's `CollisionPolicy` exactly (`error|skip|overwrite`, no default arm — verified against `crates/muxsmith-core/src/profile/model.rs:165-172`). `dry_run::run` gains `on_collision: Option<CollisionPolicy>` immediately before `json` (`crates/muxsmith-cli/src/commands/dry_run.rs:111-112`), replacing the hardcoded `None` at the former line 78 (`dry_run.rs:133-134`). `main.rs` threads it at the same position (`main.rs:160-173`), and it is the only call site of `dry_run::run` in the tree (verified: `grep -rn "dry_run::run(" crates/` returns exactly `main.rs:33`). The new test exercises exactly the required scenario: default exits 2, `--on-collision skip` exits 1 with an `output-collision` diagnostic at `warning` severity (`tests/dry_run_cli.rs:204-300`). No core or locale files touched.

### Strengths
- Test is genuine end-to-end (spawns the real binary via `assert_cmd`, real mkvmerge-built fixture, real JSON assertions on diagnostic code/severity), not a mock.
- Caught and correctly fixed a fixture ordering bug (nested output dir being picked up by recursive discovery) rather than papering over it; documented the fix inline in the test comment (`tests/dry_run_cli.rs:209-212`).
- `CollisionArg::policy` match has no wildcard arm, so it will fail to compile (not silently misroute) if core ever adds a `CollisionPolicy` variant — good exhaustiveness discipline for a task explicitly setting up reuse for Task 8.
- Report's "no other call site" and "no forbidden typography" claims both independently verified true.
- Doc comment on the new `on_collision` field/flag correctly states the fallback semantics, and that behavior is genuinely what pre-existing core code does (`planner.rs:249`), not an aspirational claim.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- The test only asserts the exit code (2) for the default/no-flag branch; it does not also assert the `output-collision` diagnostic's severity is `error` in that branch (it does check it for the `skip` branch). Would tighten the test slightly but the exit code alone is sufficient signal for what the task asked.

### Assessment
**Task quality:** Approved
**Reasoning:** All three interface elements match the brief exactly, the only call site is correctly updated, the test exercises real behavior end-to-end and matches the required scenario precisely, and core/CLI separation and typography constraints hold.