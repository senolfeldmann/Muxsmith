<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-12  (round 2 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_016rdFwF7cM98Rj2T7BYYSuw
  agent_id:           aebfc18cc8ad04358
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-aebfc18cc8ad04358.jsonl
  dispatch_desc:      Review Task 12 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-08T00:36:53.392Z
  continuation_trigger: The coordinator sent a message while you were working: Your finding #1 (JSON sort order) a
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

All three in-scope items verified against `/home/senol/Git/Muxsmith/.superpowers/sdd/review-51540e1..ad841b0.diff`:

1. **Single sort before branching — correct.** `crates/muxsmith-cli/src/commands/validate.rs` now sorts `diagnostics` by `Reverse(d.severity)` immediately after `collect(...)`, before the `json` branch. Both output modes iterate the same sorted vector; the old branch-local `sorted` clone is deleted. Order-independent consumers are unaffected: `worst_severity` is a max, the summary `count` closure is a filter-count, so moving the sort ahead of them changes nothing. `Vec::sort_by_key` is stable, so within-severity insertion order is preserved, matching the "stable order otherwise" requirement. The controller's live observation (`['error','error','warning']` on `bad.yaml`) is consistent with what the code now guarantees.

3. **Renderer fallback unit tests — present and load-bearing.** `unknown_message_id_falls_back_to_raw_id` pins the `get_message` -> raw-id path. `invalid_locale_falls_back_to_en_and_renders` uses `"zz-ZZ-invalid!"`, which cannot parse as a `LanguageIdentifier` (the `!` guarantees a parse error, unlike a bare `zz-ZZ-invalid` where `invalid` could pass as a variant subtag), so it genuinely exercises the `unwrap_or_else(|_| "en".parse()...)` branch: replace that fallback with a plain `.unwrap()` and this test panics. Both tests assert observable output, not internals.

4. **Nothing else changed — confirmed.** Three files, 24 insertions / 4 deletions, all accounted for: the sort relocation in `validate.rs`, the appended `#[cfg(test)]` module in `i18n.rs` (no production code touched in that file), and 3 lines in `cli_validate.rs` (comment + `assert_eq!(first["severity"], "error")`). No catalog, `Cargo.toml`, `Cargo.lock`, or `main.rs` changes.

The strengthened JSON assertion (item 2, controller-verified) is also visible in the diff and is a real regression guard: `bad.yaml` produces both errors and a warning, so an unsorted JSON array could lead with the warning and fail the test.

### Strengths

- The fix removed code rather than adding it in `validate.rs` (clone + local sort gone); the shared-sort comment states the contract in one line.
- The invalid-locale test input was chosen deliberately to force the parse-failure path rather than accidentally testing the happy path.

### Issues

#### Minor

- `invalid_locale_falls_back_to_en_and_renders` cannot distinguish "parse fell back to en" from "bundle only contains en resources anyway" by output alone; its real regression value is the panic guard on the `.parse()` fallback. That is sufficient for the spec's requirement (no panic, en output), so this is an observation, not a demanded change.

### Assessment

**Task quality:** Approved

**Reasoning:** Findings #1 and Minor #4 are fixed exactly as specified — one sort feeding both output modes with a test pinning JSON order, and both renderer safety-net paths now under unit test — with no unrelated changes in the diff. The Task 13 deferral of the catalog-completeness guard is a controller decision and outside this gate; nothing in this fix diff conflicts with it.