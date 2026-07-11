<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-9  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_018pAEej4Mjj3JWyHdza3BGH
  agent_id:           a7f2d74d3b440450a
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a7f2d74d3b440450a.jsonl
  dispatch_desc:      Review Task 9 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T22:24:43.094Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

Now I have all evidence needed. Here is the review.

### Spec Compliance
- ✅ Spec compliant on the brief's literal scope: document shape exact (`crates/muxsmith-cli/src/commands/run.rs:253-277` `run_json_document`, field names/nesting match the brief verbatim — verified against test literal transcription at `run.rs:742-767`), builder unit tests use `serde_json::Value` equality throughout (`run.rs:720-767`, no string comparison), both `TODO(Task 9)` sites resolved (`run.rs:68-88`, `run.rs:128-144`), no NDJSON anywhere in the diff (`git diff 2478520..77317a0 | grep -in ndjson` → empty), ASCII-only punctuation in all added text (`grep -P '[\x{2013}\x{2014}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}]'` over added lines → empty).
- ❌ Issues found: the "json callers ALWAYS get a document" global constraint is not actually satisfied by `run --json` as it stands after this task — see Named-risk finding 2 below. Pre-existing, not introduced by this diff, but real.

### Named-risk findings

**1. Keep-mode expectation fix — confirmed correct, not a planner-bug cover-up.**
`FilenameCfg::Keyword("keep")` in `crates/muxsmith-core/src/planner.rs:642-648` renders via `primary.path.file_stem()` (Rust's `Path::file_stem()`: whole filename minus final extension), never touching `primary.identifier` (the parsed struct, used only by the `Template` variant). `.mkv` is unconditionally appended (`planner.rs:675-676`). The design spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:224`, section 4.8) states: "`filename`: `keep` (source basename, `.mkv` extension enforced)". `planner.rs:669-672`'s own comment: "keep is file_stem + `.mkv` unconditionally, since the source's own extension carries no meaning to preserve." So `Show.S01E01.mkv` → `Show.S01E01.mkv` under keep mode is specified behavior, not a bug. The test-expectation fix in `run_cli.rs:484-487` is correct; nothing was papered over.

**2. Document always emitted — two real gaps, both pre-existing and out of this diff's touched lines.**
Tracing every early-return in `run.rs`'s json path:
- `run.rs:57-63` (profile `load::from_file` failure): `println!("{}", renderer.diagnostic(&d))` fires **unconditionally**, not gated on `json` at all, then `return 2`. Under `--json`, this puts a human-formatted diagnostic line on stdout instead of a JSON document — breaking "the final document is the only stdout output" and "ALWAYS get a document" simultaneously. Any `--json` caller parsing stdout with `serde_json::from_slice` gets a parse error on this path.
- `run.rs:89-95` (`mkv.list_languages()` failure): `eprintln!(...)` fires unconditionally, `return 2`, no document at all on stdout in json mode. Less severe (stderr, not stdout) but still violates "ALWAYS get a document."

Both are byte-identical to code already present at base `2478520` (verified via `git show 2478520:crates/muxsmith-cli/src/commands/run.rs`) and untouched by this diff; `grep -n TODO` on the base file shows exactly two `TODO(Task 9)` markers, neither on these two arms, so the brief's own scope (2 TODO sites) never asked the implementer to fix them. The sibling `dry_run.rs` has the identical bug on both arms (`dry_run.rs:38-44`, `dry_run.rs:70-75`), unmodified by this diff too — so it is "consistent with dry-run's json behavior," but that parity standard is itself broken relative to the stated global constraint. Notably, the codebase already has the *correct* pattern for exactly this: `validate.rs:64-73`'s `collect()` folds a profile-load failure into the diagnostics vector (`Err(d) => vec![d]`) so `--json` mode emits a proper `{"diagnostics": [...]}` document even on load failure. `dry_run.rs`/`run.rs` never adopted that pattern.
Not a regression introduced by Task 9, and not something the brief asked this task to fix — flagging because the named risk explicitly asked me to trace this, and because it means the task's own stated deliverable ("json callers ALWAYS get a document") is not fully true of `run --json` after this commit. See Important/1 below.

**3. Summary counting — sound, no drift risk.**
`run_json_document` (`run.rs:253-277`) builds both `jobs` (via `outcomes.iter().zip(outputs).enumerate()`) and `summary` (via `outcomes.iter().filter(...)`) from the **same** `outcomes` parameter in one function — not two independently maintained tallies. The only way these could disagree is if `outputs.len() != outcomes.len()` (the `zip` would silently truncate `jobs` while `summary`'s count stays over the full `outcomes`). That's guarded by contract: `run_queue`'s doc comment (`crates/muxsmith-core/src/executor/queue.rs:71-74`, "Returns one outcome per spec, index-aligned") guarantees `outcomes.len() == specs.len()`, and `run.rs:147-155` builds `outputs`/`json_outputs` from that same `specs` slice, so the lengths are equal by construction at every call site. The main-path call (`run.rs:193-200`) and `render_summary` (`run.rs:190-191`) both consume the identical `outcomes` binding — no separate fold, no drift possible.

### Strengths
- `run_json_document` is a pure function (`Value` in as `base`, owned `outcomes`/`outputs` slices in, `Value` out), no I/O — clean to unit test in isolation.
- Reuse of `dry_run::batch_json`/`config_only_json` via `pub(crate)` (not full `pub`) is correctly scoped and avoids duplicating the diagnostics-rendering logic.
- The 4 new `run_cli.rs` integration tests parse stdout via `serde_json::from_slice` and assert on `Value` field access (`run_cli.rs:471-491`, `540-547`, `590-595`, `644-652`), not string-contains, exactly where shape matters; string-contains is used only for the legitimate "no human line leaked" absence checks.
- The unit test `run_json_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary` (`run.rs:742-767`) transcribes the brief's own example JSON literally (down to `duration_ms: 12400`), asserting against the spec text rather than against whatever the implementation happened to produce.
- The document is printed before the `cancel.load` → 130 check (`run.rs:190-205` precedes `run.rs:207-209`), so even a cancelled batch gets a full document with `Cancelled` job entries reflected — the implementer didn't have to special-case this and didn't miss it.
- Self-reported gate (full workspace tests, fmt, clippy `-D warnings`, `cargo deny`) all green per the report; I did not re-run it (no specific doubt raised beyond the named risks, all resolved by static inspection).

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
1. **Two exit paths still break the "json callers ALWAYS get a document" contract**: `run.rs:57-63` (profile load failure — worse, pollutes stdout with a non-JSON line) and `run.rs:89-95` (`list_languages` failure — no document at all). Pre-existing, not introduced by this diff, not asked for by this task's brief (only 2 TODO markers existed pre-Task-9, confirmed against base commit `2478520`), and mirrored identically in `dry_run.rs`. Not a defect *in* this task's diff, but a real, confirmed gap in the deliverable this task's own global constraints describe. The correct fix pattern already exists in this codebase (`validate.rs:64-73`'s diagnostic-folding `collect()`). Recommend a follow-up task (or an amendment to this one) to close both arms in `run.rs` and `dry_run.rs`; flagging now because a `--json` caller hitting a malformed profile file today gets unparseable stdout, which defeats the entire purpose of `--json`.

#### Minor (Nice to Have)
- `run.rs:78` (`config_only_json(&config_diags, renderer), &[], &[],)`) has a stray trailing comma before the closing paren on an otherwise single-line call; harmless (rustfmt-clean per the report) but slightly odd formatting compared to the other two call sites' multi-line style.

### Assessment
**Task quality:** Approved
**Reasoning:** Everything the brief actually asked for (both `TODO(Task 9)` sites, document shape, builder unit tests, NDJSON exclusion, ASCII typography) is correctly and cleanly implemented, and the keep-mode test-expectation fix is verified correct against the planner and spec, not a cover-up. The one Important finding is a real gap against this task's own stated global constraint, but it lives in code this diff never touched and was outside the brief's explicit scope (confirmed via the base commit's TODO markers) — it should be tracked as a fast-follow, not treated as a defect in this task's execution.