<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-8  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_017opeg75Mge37gSMguiXQMD
  agent_id:           a201598fdbc77bf9e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a201598fdbc77bf9e.jsonl
  dispatch_desc:      Review Task 8 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T12:04:21.451Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
✅ Spec compliant. Verified against `/home/senol/Git/Muxsmith/.superpowers/sdd/review-aa045db..3283552.diff` and the underlying source (`planner.rs`, `model.rs`, `matcher.rs`, `discovery.rs`, `identify.rs`).

- **First-match-wins ordering** (`planner.rs:120-137`, new function): the loop walks `profile.attachments.rules` once, per attachment, checking `rule.select` then `rule.drop` inside the *same* iteration, `break`-ing on the first hit. This is not a select-pass/drop-pass split — confirmed via `model.rs:303-320` that `AttachmentRuleShape` validation guarantees exactly one of `select`/`drop`/`add` per rule, so per-rule "check select, then drop" is exactly "check whichever field this rule has," preserving true ordered-list semantics across mixed rule kinds. No ordering bug.
- **Reduction correctness** (`planner.rs:143-150`): `kept.len() == primary_attachments.len()` is checked *before* `kept.is_empty()`, so the zero-attachments case (0==0) correctly falls to `KeepAll`, not `DropAll`. The `Subset` branch unconditionally `sort_unstable()`s before construction, guaranteeing ascending order regardless of iteration order.
- **Adds (D12)**: all hits from every `add` locator are appended (`add_files.extend(hits)`, `planner.rs:165`); emptiness is checked on that rule's own `hits` before extending, so the warning is scoped per-rule, not to the cumulative `add_files`. Confirmed `Diagnostic::warning(...)` (`planner.rs:158`), not `.error(`. Dedup (`planner.rs:167-168`) uses `BTreeSet` purely as a "seen" set inside `Vec::retain`, which preserves original (first-seen) order while removing later duplicates — confirmed correct by construction, not just by test.
- **D10**: `resolve_attachments` only ever receives `&ident.attachments` from the primary's own `id.identify(&primary.path)` result (`planner.rs:52-59`); no donor attachment path touched anywhere in the diff.
- **`report.rs:202`**: `MissingExternal` doc comment now reads "(track rule, chapters, or attachment add)", exact match to the brief. `AmbiguousExternal`'s comment was correctly left untouched — adds have no "ambiguous" concept since multiple hits are valid by design (D12), so there's nothing to update there.
- **Scope**: diff touches only `resolve_file`'s attachment wiring, the new `resolve_attachments` helper, the one doc-comment line, and test/fixture files. No chapters/tags/title/track/collision logic touched.

Independently re-ran (not just trusted the report):
- `cargo check -p muxsmith-core --tests` — clean (also confirms the `if let ... && matches(...)` let-chain syntax compiles on this toolchain, `rustc 1.96.1`, edition 2024).
- `cargo test -p muxsmith-core --test planner_resolution attachment` — all 7 pass.
- `cargo clippy -p muxsmith-core --all-targets -- -D warnings` and `cargo fmt --all --check` — both clean.

### Strengths
- The per-rule single-pass check is the correct, non-obvious way to satisfy first-match-wins across heterogeneous rule kinds without a two-pass bug; validated against the `AttachmentRuleShape` invariant rather than assumed.
- Branch ordering in the reduction (`KeepAll` check before `is_empty`) correctly handles the zero-attachments edge case without a special case, and matches the pre-existing Task-4 default test's expectation.
- Test suite matches the brief's 7 specified cases exactly, uses a real fixture with real attachment ids for the rule tests and real files on disk for the add/dedup/zero-match tests (`with-attachments.json`, `attachment_add_locator_attaches_all_matching_files` writing `b.ttf`/`a.ttf` in reverse order to actually exercise the sort).
- Doc comment on the new private function is precise about *why* (D10/D12 rationale), not just *what*.

### Issues
None found at Critical, Important, or Minor severity.

### Assessment
**Task quality:** Approved
**Reasoning:** All brief requirements verified correct by reading the actual logic (not just the report's narration), including the two specific bug patterns the review was primed to look for (split select/drop passes, wrong branch-order in reduction); independent recompile, targeted test run, clippy, and fmt all confirm the implementer's gate claims.