<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-1  (round 1 of 1)
  session_uuid:       ffc7c915-ea2a-432b-9ffd-64ac313a2a6f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f.jsonl
  tool_use_id:        toolu_016iTM5asU2D99jmbRo3SDMZ
  agent_id:           a1310c4109764b222
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f/subagents/agent-a1310c4109764b222.jsonl
  dispatch_desc:      Review Task 1 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T16:55:47.731Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance Verdict: PASS

All named global constraints are met, and the exhaustiveness gate holds up under independent re-verification (not just re-stated from the report).

### Verification detail

**Named risk (every non-test `profile.tracks` read updated to `.rules`):** Checked directly in the repo, not just the diff. Grepped all `.tracks` accesses across `crates/*/src/` outside `tests/`:
- `planner.rs:286,364,983,1031,1169`, `validate.rs:59,63`, `lint.rs:21` — all now `profile.tracks.rules` / `p.tracks.rules`.
- Remaining bare `.tracks` sites — `planner.rs:448` (`source_ident.tracks`), `planner.rs:1041` (`ident.tracks`), `identify.rs`, `muxsmith-cli/src/commands/identify.rs` — inspected each in context: all are `Identification.tracks`, the unrelated field the brief says to leave alone. Confirmed correct, not missed.
- No production `Profile` construction site builds `tracks` as a bare list anymore; grep for `tracks:` field-init in `crates/*/src/` (excluding tests) shows only `model.rs:35` (`pub tracks: TracksCfg`) and lint.rs's own `#[cfg(test)]` fixtures (already migrated).

**Exhaustive YAML migration:** Independently scanned every `.rs`/`.yaml` file under `crates/` with a script matching literal and escaped `tracks:\n  -` patterns (both physical-newline and `\n`-escaped forms used in `dry_run_cli.rs`). Zero un-migrated sites found. Remaining un-migrated occurrences in `docs/` are confirmed to be: two frozen `docs/process-journal/artifacts/plan-1-sdd/*` briefs, two completed historical plan documents, and the current plan-3.5 document's own "before" example illustrating the transformation rule itself — none are code, fixtures, or the two named spec sections.

**`#![deny(missing_docs)]`:** Confirmed present at `crates/muxsmith-core/src/lib.rs:1`. `TracksCfg` and both fields (`unmatched`, `rules`) carry doc comments in the diff.

**`unmatched` default / behavior preservation:** `TracksCfg.unmatched` uses `#[serde(default = "drop_policy")]` with `drop_policy() -> KeepDrop::Drop`, matching the exact code the brief specified (including `rules` having no `#[serde(default)]`, which is deliberate: `tracks:` itself stays a mandatory `Profile` field with no default, exactly preserving the old requirement that the list must be present). `KeepDrop` has no `#[derive(Default)]`, so the named-function-default approach is the only option — correctly mirrors `AttachmentsCfg`'s `keep()` pattern.

**Typography:** Scanned every added line in the diff (not just trusting the report) for em/en-dash, figure-dash, horizontal bar, Unicode minus, curly quotes, ellipsis, NBSP — zero hits. `Türkçe` in `reference.yaml`/spec doc is preserved untouched (only re-indented).

**Spec sections 4.1/4.5:** Both updated as required; 4.1's reference example nests rules under `rules:` with a new `unmatched: drop # keep | drop; default drop` line, the "de rules omitted for brevity" comment is preserved (not dropped), and 4.5 gained the `{ unmatched, rules }` block description plus the `tracks.rules` order-note update.

**`ident.tracks` / `source_ident.tracks`:** Untouched, confirmed by direct inspection, correct.

**Ancillary checks (no gaps found):** `cli_schema.rs`'s schema test asserts only `contains("tracks")`, generic enough not to require updating; `xtask/tests/fixtures/mini-schema.json` is an unrelated synthetic schema for the mkvmerge `-J` identify shape, not `Profile`; `TracksCfg` is not referenced anywhere requiring a `Default` impl; no doctest code blocks reference the old bare-list `tracks:` shape.

## Strengths

- The `Assignment.rule_index` doc comment (`planner.rs:43`) was proactively updated from "Index into `profile.tracks`" to "`profile.tracks.rules`" — a real doc-drift catch not explicitly asked for in the brief.
- `validate_semantics.rs`'s `profile()` helper was changed to programmatically re-indent its argument rather than touching all ~12 callers — exactly matching the brief's own instruction and minimizing surface area.
- `validate_structure.rs`'s `unknown_keywords_are_flagged` fix (bumping the hardcoded replacement indent from 4 to 6 spaces) reflects genuine understanding of the YAML nesting, not a mechanical find/replace; independently verified the arithmetic and it is correct.
- Two new tests added verbatim from the brief's Step 1, both present and correctly asserting `KeepDrop::Drop`/`KeepDrop::Keep`.
- Report is candid and specific (names exact files/lines checked, states the JSON-literal test needed structural change beyond the brief's YAML-only transformation rule, explains why `command.rs`/`identify.rs` needed zero changes rather than silently omitting them).

## Issues

**Critical:** none.

**Important:** none.

**Minor:**
- `crates/muxsmith-core/src/profile/model.rs:269-314` — `TracksCfg`'s struct definition and its `drop_policy()` helper are inserted between `AttachmentsCfg`'s struct/`keep()` helper and `impl Default for AttachmentsCfg`, splitting a previously-contiguous, clearly-grouped unit (struct -> default-fn -> `impl Default`) with an unrelated type. The brief's own suggested placement was "after its `impl Default`" (~line 300), which would have kept `AttachmentsCfg`'s three related items together. Purely a locality/readability nit; no functional effect. Trivial to fix by moving the `TracksCfg` block (struct + `drop_policy()`) to after `impl Default for AttachmentsCfg`.
- `validate.rs:60` — the `NoTrackRules` diagnostic keeps the literal config_path `"tracks"` rather than `"tracks.rules"` even though the empty check now tests `.rules`. Not a bug (no test asserts this path string, and "tracks" is still a defensible pointer to the block), and explicitly out of the brief's scope, but worth a mental note for Task 2 or a future pass if diagnostic-path precision matters later.

## Task quality verdict

Solid, disciplined execution. The implementer followed the brief's TDD steps literally, matched the exact code samples given (including the non-obvious choice to leave `rules` without `#[serde(default)]`), and the exhaustiveness claim for the YAML migration holds up under independent re-scanning of the actual repository, not just the diff. The one substantive review risk named in the task (every non-test `.tracks` consumer updated, no stray `Vec` treatment left) checks out completely. Only cosmetic nits found; nothing blocks acceptance.