<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01FVTdAKeqZP7j29znaoZsvW
  agent_id:           a7a56e63404aefa55
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a7a56e63404aefa55.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T11:47:28.768Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
✅ Spec compliant.

Verified against `/home/senol/Git/Muxsmith/.superpowers/sdd/task-7-brief.md` and the actual source (not just the diff, for the "mirrors existing branch" check, since the existing track-external branch isn't touched by this diff and so doesn't appear in it):

- Keyword mapping: `planner.rs:740-741` — `Keyword(k) if k == "drop" => Drop`, `Keyword(_) => Keep`. Correct for keep, drop, and defensive unexpected-keyword-> Keep, matching the brief's global constraint exactly.
- External mapping: `planner.rs:742-761` — `resolve_locator(&block.external, primary_dir, &primary.identifier)` then `hits.len()` match: `1 -> External(path)` (744-746), `0 -> Diagnostic::error(MissingExternal, "chapters.external").for_file(&primary.path)` + return `Keep` (748-753), `n -> Diagnostic::error(AmbiguousExternal, "chapters.external").for_file(&primary.path).with("count", n.to_string())` + return `Keep` (755-761). Byte-for-byte match to the brief's spec, including argument order (`for_file` before `with`).
- `config_path` is the literal `"chapters.external"` in both branches, not `tracks[i]`-scoped — confirmed at 749 and 757.
- No mkvmerge identification of the chapters file: the `1`-hit arm (`planner.rs:746`) only extracts the path from `hits`, no `id.identify(...)` call. Contrast with the pre-existing track-external branch at `planner.rs:403` (`match id.identify(&donor) {...}`), which does identify its donor — confirms the chapters path deliberately omits it per the brief.
- No new `DiagCode`: diff touches only `planner.rs` and the test file; `report.rs` untouched. `MissingExternal`/`AmbiguousExternal` doc comments already say "track rule or chapters" (`report.rs:115,117`), pre-dating this task.
- Error-path placeholder never surfaces: `finalize_plans` (`planner.rs:845-851`) sets `plan = None` for any file with a `Severity::Error` diagnostic, and `Diagnostic::error` (`report.rs:185-187`) always sets `Severity::Error`. Verified from source, not asserted. Tests `chapters_external_zero_matches_yields_missing_external_and_no_plan` and `..._two_matches_yields_ambiguous_external` assert `fr.plan.is_none()` (diff lines 263, 304) — both confirm the placeholder never leaks.
- Named consistency check: the external branch mirrors the pre-existing track-external branch's `resolve_locator` + `hits.len()` handling in structure and diagnostic-construction style (`planner.rs:366-443`), correctly diverging exactly where the brief requires: no `optional` escape on 0 hits (tracks gate on `!rule.optional` at line 373; chapters always errors), and no `id.identify()` on the 1-hit path. `primary_dir` is the identical variable computed once at `planner.rs:334` and threaded unchanged to both the track loop (370) and `resolve_chapters` (518) — not recomputed.
- Scope: diff stat is `planner.rs` (+50/-2) and the test file (+160) only. No changes to `discovery.rs`, `report.rs`, `validate.rs`, or attachments/tags/title/changes logic. The only other line touched in `resolve_file` is replacing the `ChapterSource::Keep` literal with the `chapters` variable (diff lines 50-61).

### Strengths
- Faithful reuse of an established codebase pattern: `resolve_chapters`'s keyword-guard shape (`Keyword(k) if k == X => ...`, `Keyword(_) => default`) is the same idiom as the pre-existing `resolve_title` (`planner.rs:696-699`), including the same "defensive only, never a panic" rationale documented at `planner.rs:694-695`. Not a one-off invention.
- Genuine TDD: report shows RED (4/5 failing, 1 trivially passing) before implementation, GREEN after; consistent with the diff's shape.
- Self-review in the report is accurate where checked: config_path string, ASCII-only diff, private-fn `missing_docs` exemption (confirmed `#![deny(missing_docs)]` is crate-level in `lib.rs:1` and only fires on public items), scope confinement.
- Zero new dependency surface (no `Cargo.toml`/`Cargo.lock` changes), so the `cargo deny check` claim is low-risk to trust.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- No explicit test for the "unexpected keyword -> Keep" defensive branch (`planner.rs:741`). Not required by the brief's Step-1 test list, and `validate.rs:125-131` rejects any `chapters` keyword other than keep/drop at config time, so the branch is unreachable through the normal CLI validate-then-plan path — but note `dry_run.rs` runs `plan_batch` unconditionally regardless of `config_diags` (spec 5.5, decoupled by design), so the branch is technically reachable via `dry-run`/direct `plan_batch` calls with an already-invalid profile. This mirrors the identical, equally-untested gap in `resolve_title`, so it's pre-existing codebase practice, not a regression introduced here.
- `std::mem::forget(dir)` in the three new external-locator tests (diff lines 221, 260, 301) leaks the tempdir on disk since the assertions run after all disk access is done and `expected`/`batch` are already owned values. Not new: this exact pattern appears 7 times earlier in the same test file (verified via grep) plus in `suggestions.rs`, so the new tests are following an established (if questionable) file convention, not introducing a fresh smell.

### Assessment
**Task quality:** Approved
**Reasoning:** Every mapping, diagnostic code, config_path, and scope boundary from the brief is implemented literally and correctly, verified against source (not just the diff) for the two checks that required seeing pre-existing unchanged code (the mirrored track-external branch, `primary_dir` identity, the `resolve_title` precedent). No issues rise above minor, and both minor notes are pre-existing codebase conventions rather than defects introduced by this diff.