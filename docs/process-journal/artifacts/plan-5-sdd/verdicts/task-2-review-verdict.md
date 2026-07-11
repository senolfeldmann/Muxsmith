<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-2  (round 1 of 1)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01UXnnVExvaPzLoNmQAxzwhz
  agent_id:           a60face1e2402a9fb
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a60face1e2402a9fb.jsonl
  dispatch_desc:      Review Task 2 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-10T11:48:59.067Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance
✅ Spec compliant

## Strengths

- **Verbatim hoist confirmed by direct comparison.** I compared each moved function body against what was deleted from `dry_run.rs`/`run.rs` line by line: `batch_document`, `config_only_document`, `run_document`, and the private `rendered_diags` helper are character-identical except for the renamed function/param and the `&Renderer` → `&dyn DiagnosticRenderer` type change. No logic, field, or key-order drift — the byte-identical output claim holds structurally, not just by test-passing assertion.
- **Minimal `run.rs` footprint honored.** The diff there touches only the import block and the four call sites plus the removed function/tests — nothing in the surrounding match-arm structure the parallel task needs is disturbed.
- **Rustdoc note matches the brief's exact ask**, `crates/muxsmith-core/src/report/json.rs:644-649`: states `jobs[].index` indexes the queue (`run_queue`'s spec slice), not `batch.files`, and explains why a skipped file has no queue entry.
- **No stale references anywhere in the tree.** `grep -rn "batch_json(\|config_only_json(\|run_json_document("` across the whole worktree returns nothing — every call site was updated, nothing left half-migrated.
- **No dead code from the test relocation.** The `outcome()`/`outputs()` helpers in `run.rs`'s test module remain used by ~15 other tests after the two relocated tests were removed; no orphaned helper, consistent with the clean clippy run.
- **Crate boundary respected.** `crates/muxsmith-core/Cargo.toml` carries no `fluent-bundle` or `muxsmith-cli` dependency (checked directly) — the `DiagnosticRenderer` trait genuinely keeps core decoupled rather than just claiming to.
- **`#![deny(missing_docs)]` verified present** at `crates/muxsmith-core/src/lib.rs:1`, and every new public item (`DiagnosticRenderer`, its method, all three document functions) carries doc comments.
- **ASCII punctuation clean** — grepped all six changed files for em/en dash, smart quotes, ellipsis: no hits.
- **Test additions match the brief exactly**: the new `run_document_jobs_carry_index_output_state_and_summary_carries_all_four_counts` is a field-presence test as Step 2 specifies (not more, not less), and the two relocated tests add stronger full-value coverage on top without inflating scope beyond what the brief asked.
- Commit message matches the brief's Step 3 verbatim.

## Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- `crates/muxsmith-core/src/report/mod.rs`'s crate-level doc ("Core never produces user-facing prose; renderers... map `DiagCode::key()` + params to Fluent messages") predates this task and now sits one module above `report::json`, whose documents *do* carry a `"rendered"` human-language field once a caller supplies a renderer. `json.rs`'s own module doc already disambiguates this ("filled by an injected `DiagnosticRenderer`, never synthesized here"), which is sufficient, but a one-line cross-reference from `mod.rs`'s doc to `json`'s exception would preempt a future reader taking the crate-level claim too literally.
- `&dyn DiagnosticRenderer` (dynamic dispatch) vs. a generic `<R: DiagnosticRenderer>` (static dispatch) is a style choice with only one call site today; either is fine, not worth changing.

## Design deviation evaluation (brief's explicit ask)

**(a) Was a renderer dependency avoidable?** No. The brief names `batch_document`/`config_only_document`/`run_document` as the exact functions to live in core, and pins "byte-identical documents" as a hard constraint — and the pre-existing (pre-Task-2) JSON contract already embeds a `"rendered"` field per diagnostic. Dropping that field to keep core "pure" would break byte-identical output; pushing a post-hoc tree-walk into the CLI to bolt the field back on afterward would just relocate the "who owns document shape" duplication spec 7 is trying to kill. Some externally-fed rendering hook is structurally required once these three functions move as-is.

**(b) Is the trait minimal and well-placed?** Yes. One method, no default impl, no Fluent/i18n knowledge, defined in `report::json` next to its only consumers (`run_document` correctly needs no such thing, since it only re-serializes already-built values). `Cargo.toml` confirms zero new core dependency.

**(c) Does human-language text leak into core output paths?** Functionally yes: the `serde_json::Value` a core function returns contains rendered natural-language strings once a `DiagnosticRenderer` is supplied. But core never originates, hardcodes, or has i18n/localization knowledge of that text — it only reserves a named slot (`"rendered"`) for content the caller's adapter supplies, the standard hexagonal port/adapter shape. I judge this compliant with the spirit of "core emits no prose" (which is about core *deciding what to say*, not about whether a value it returns can ever contain injected text) rather than a defect. It is, however, exactly the kind of boundary call worth a plan-owner's explicit sign-off given the brief called it out by name — flagged above accordingly, not treated as something requiring rework.

## Assessment
**Task quality:** Approved
**Reasoning:** The hoist is verifiably byte-identical, the footprint constraints (minimal `run.rs` touch, no stale call sites, no core dependency creep) all check out against the actual tree, and the one non-mechanical design decision (`DiagnosticRenderer` trait) is the correct and essentially forced resolution of a real constraint conflict, not a shortcut.