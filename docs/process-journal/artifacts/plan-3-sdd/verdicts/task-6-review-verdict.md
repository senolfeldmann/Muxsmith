<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-6  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01V6pENuWAq339s3r7gPU9Xy
  agent_id:           a30b19c00911480e5
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a30b19c00911480e5.jsonl
  dispatch_desc:      Review Task 6 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T11:36:17.974Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

This confirms `render_literal` is doc-documented as "used for output filenames and titles" — the title use case was already anticipated by the template module itself. Everything checks out. I have sufficient evidence for the full report.

## Spec Compliance

- ✅ Spec compliant

**Named check (source_stem):** Verified directly.
- (a) `crates/muxsmith-core/src/profile/validate.rs:149-159` — the `title.template` branch clones `template_fields` and pushes `"source_stem"` before calling `validate_template`, identically to `output.filename.template` at `validate.rs:112-122`. Confirmed: a validated `{source_stem}` reference in `title.template` is legal.
- (b) `resolve_title` (`planner.rs:695-710`) builds its Ctx as `primary.identifier.to_ctx()` + a conditional `ctx.set("source_stem", stem)` from `primary.path.file_stem()` — the exact idiom `render_output` uses (`planner.rs:615-619`). `Identifier::to_ctx()` (`discovery.rs:30-37`) populates every `g{i}`/named-group key from `Identifier.groups`, which `discovery::scan_primaries` (`discovery.rs:90-100`) populates identically to how `validate.rs`'s `template_fields` is built from the same regex's capture names (`validate.rs:33-49`). Field lists match on both ends. No missing-field gap; the implementer's fix is correct and necessary (without it, `{source_stem}` would validate but silently render empty).
- validate.rs *does* validate the title template (`validate.rs:142-160`), so the ⚠️ contingency in the task instructions doesn't apply.

Additional verified points:
- Keyword/template/fallback mapping (`planner.rs:697-706`): `"clear"` -> `Clear`, everything else keyword-shaped (including `"keep"` and any unexpected string) -> `Keep`, template parse `Err` -> `Keep`. No `unwrap`/`expect`/panic on the `Result`. Matches brief exactly.
- Empty rendered title -> `Set(String::new())`, never `Clear` — `TitleAction::Set(t.render_literal(&ctx))` unconditionally wraps in `Set`; confirmed by test `title_template_rendering_empty_is_a_legitimate_set` (`tests/planner_resolution.rs:213-229`).
- `resolve_tags` (`planner.rs:721-726`): `global_keep`/`track_keep` are direct `== KeepDrop::Keep` equality checks. `KeepDrop` is a closed two-variant enum (`Keep`/`Drop`, `model.rs:179-186`), so this is provably correct for all four combinations by construction, not just the one tested combination.
- Both wired into `Plan { .. }` at `planner.rs:52-53,69-70`, replacing the Task-4 hardcoded `TitleAction::Keep` / `TagFlags { global_keep: true, track_keep: true }`.
- Scope: diff touches only `planner.rs` and `tests/planner_resolution.rs`. No chapters/attachments/`changes` logic added; `chapters: ChapterSource::Keep` and the attachment plan are untouched literals in the same struct literal.
- ASCII typography: no non-ASCII bytes anywhere in the diff (checked with a raw byte grep).

## Strengths

- The `source_stem` fix is real engineering, not gold-plating: it closes an actual latent bug (a validated-but-silently-wrong template field) that the brief's literal wording would have produced. Cross-file evidence (validate.rs's field-list construction) is correctly identified and correctly mirrored.
- Conservative on abstraction: correctly declined to extract a shared `primary_ctx()` helper between `render_output` and `resolve_title` for a ~4-line duplication, citing scope discipline. Right call under the codebase's own "three similar lines beat a premature abstraction" bar — two instances of a short idiom don't clear it.
- Fallback-to-`Keep` on template parse failure is not just "matches the brief" but is live-reachable: `plan_batch` (`planner.rs:270-283`) runs unconditionally regardless of `validate::validate`'s outcome (confirmed via `dry_run.rs:46-85`, which calls `validate::validate` then `plan_batch` with no gate in between). So a profile with an invalid title template genuinely reaches `resolve_title` at plan time in `dry-run`, and the code handles it correctly rather than by luck.
- Test coverage hits the exact rendered string in every case (`Set("Show S03")`, `Set(String::new())`, `Set("Show.S01E01")`), not just variant shape.

## Issues

#### Important (Should Fix)
None.

#### Minor (Nice to Have)

- **`tests/planner_resolution.rs` test count vs. report**, `.superpowers/sdd/task-6-report.md:47-59`: report claims "7 new tests" ("4 mandated by the brief, 3 additional regression tests") and lists them, but the diff adds exactly 6 test functions (`title_clear_resolves_to_clear`, `title_keep_resolves_to_keep`, `title_template_renders_raw_capture_into_set`, `title_template_rendering_empty_is_a_legitimate_set`, `title_template_supports_source_stem_field`, `tags_global_drop_track_keep_resolves_to_flags`) — 4 mandated + 2 additional, not 3. Grep-verified: `grep -c '^#\[test\]' tests/planner_resolution.rs` = 32 total, and the diff's added `+#[test]` lines count to 6. Harmless (doesn't affect correctness or the "32 passed" total, which is internally consistent with 6 new + 26 pre-existing), but it's exactly the kind of self-report inaccuracy the "don't trust the report" instruction exists to catch.
- **Brief's Step-3 signature vs. implementation**, `task-6-brief.md:14` vs. `planner.rs:695`: the brief's helper header literally reads `resolve_title(profile, primary, diags) -> TitleAction`, but the implementation's signature has no `diags` parameter at all, and none of the brief's own detailed bullets under Step 3 ever call for a title diagnostic (title has no path-separator/empty-name invariant per the global constraints). Omitting the parameter is the correct call — a `&mut Vec<Diagnostic>` that's never written to would be dead plumbing — but it is a literal deviation from the written signature that the implementer's self-review didn't surface, unlike the `source_stem` deviation which they did flag. No functional impact.
- **Eager resolution on the discarded-plan path**, `planner.rs:51-53`: `resolve_title`/`resolve_tags` run before `output.map(...)`, so both execute (including a real `Template::parse` call for a title template) even when `render_output` returns `None` and the whole `Plan` is discarded. Pure functions, no side effects, negligible cost — not worth blocking on, but moving the two calls inside the `map` closure would avoid the wasted work on the rejected-plan path.
- **Tags combo coverage**, `tests/planner_resolution.rs:256-275`: only one of the four `global`/`track` combinations is exercised by an explicit test. Not a real risk given the mapping is a direct enum-equality with no per-case branching (provably correct for all four by inspection), but a second combo test (e.g. `global: keep, track: drop`) would remove any doubt for a future reader without re-deriving the equality argument.

## Assessment

**Task quality:** Approved

**Reasoning:** The title/tags resolution logic is exactly to spec, the panic-safety requirement is verified and actually load-bearing (not just defensive theater), and the `source_stem` design decision is a correct, well-evidenced fix to a real latent bug rather than scope creep. Findings are all Minor and non-blocking: a report/diff test-count mismatch, an unflagged (but correct) signature deviation, and two coverage/efficiency nits.