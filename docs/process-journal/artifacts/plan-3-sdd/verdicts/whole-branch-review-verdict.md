<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01XqEC2AQGguuCc1YPQNRppx
  agent_id:           a976df077071a93f9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a976df077071a93f9.jsonl
  dispatch_desc:      Whole-branch review Plan 3
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T13:27:30.570Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

I have completed a thorough cross-cutting review, including live validation against the installed mkvmerge v100. Here is the whole-branch review.

---

## Whole-branch review: Muxsmith Plan 3 (62d4956..0dcb116)

I read the full diff, the enriched-`Plan` reference and CANONICAL ARGV CONTRACT in the plan, the design decisions (D7-D12), spec 4.9/6, and the assembled unchanged sources (`resolve_file`, `command`, `capability::settable`, `discovery::resolve_locator`, `validate.rs`, `model.rs`, `dry_run.rs`). Where the golden tests only assert strings, I drove real mkvmerge v100 to close the gaps.

### Strengths

- **The argv is correct against real mkvmerge, beyond what the tests prove.** I verified all 26 flag spellings exist in v100, and ran a rich argv end-to-end (`--title ""`, `--default-track-flag`, `--hearing-impaired-flag`, `--track-name "0:English forced"`, `--language 1:tr`, `--subtitle-tracks 0,1`, `--attachments`, `--no-global-tags`, `--track-order`): exit 0, and every property applied correctly on re-identification.
- **The highest-risk untested path holds.** `PrimaryAttachments::Subset` emits `-J` attachment ids into `--attachments`. I confirmed the `-J` `id` is identical to mkvmerge's `--attachments` selector id (kept exactly the intended attachment for `--attachments 1` and `--attachments 2` when placed per-group). This is the one place a numbering mismatch would have silently selected the wrong files; it is correct.
- **The `Plan`/`command` split is clean.** `command` is pure, format-neutral, consumes only the `Plan`, and keeps mkvmerge's CLI surface in one module (D8). The `Category` table spelling out both select and no-flag (rather than templating the singular/plural inconsistency) is the right call.
- **`Matchable` generalization did not regress tracks.** The blanket `impl<M: Matchable> for &M` is justified precisely (filter hands `&&Track`); direct impl for `Attachment`, and the boolean-absent-false branch correctly yields `false` for attachment properties absent from the track schema. Track and attachment matching share one algebra with no track behavior change.
- **Reuse over new mechanism throughout** (locator machinery, template engine, `LanguageIndex`, existing `DiagCode`s), exactly as D10 intended. Defaults (`unmatched: keep`, `chapters: keep`, `tags: keep/keep`) match spec 4.9, so an omitted section never silently drops data.
- **`add` semantics match D12**: all hits attached, dedup-by-path first-seen, zero-match is a warning that does not suppress the plan. `resolve_attachments` reduction to KeepAll/Subset/DropAll (including the empty-attachments `0==0 -> KeepAll`, emitting no filter) is correct and minimal.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. Every argv concern in the plan's focus list checks out against v100.

#### Minor (Nice to Have)

1. **Live acceptance test covers only the degenerate case.** `tests/command_integration.rs` `LIVE_PROFILE` is a single subtitle rule; the rich argv (property flags, `--title ""`, `--attachments`, multi-id selection, chapters/tags flags, UTF-8 track names) is locked *only* as golden strings and the pure reference-example golden, never executed by mkvmerge. I verified the rich argv against v100 by hand this session, so correctness is currently established, but a future argv refactor could regress the shape and the golden test would just be updated to the new (wrong) string with nobody re-running the binary. Fix: add one gated live case with an attachment + a couple of `changes` so CI-with-mkvmerge guards the real surface, not just string equality.

2. **`command.rs` `input_groups` rationale is empirically false.** The comment (command.rs ~lines 71-75) says an empty input group is one "that mkvmerge may reject." mkvmerge v100 accepts empty groups (I ran `--no-video --no-audio --no-subtitles --no-buttons ( f ) ...` and a fully zero-track single-file mux: both exit 0). The guard (skip donor groups that contribute no track) is still the *right* behavior (don't open a pointless input), but the stated reason is wrong. Fix the comment to the real rationale.

3. **A zero-track plan renders a valid-but-empty MKV with no diagnostic.** If every rule is `optional` and matches nothing, the plan survives (no error), and `command` emits an all-`--no-*` argv; mkvmerge produces an empty MKV (exit 0, verified). Nothing warns that a plan resolved to zero output tracks. Narrow (needs an all-optional profile), and arguably Plan 4 / a planner "empty plan" diagnostic rather than Plan 3. Deferrable, worth recording.

4. **T5 (elevated) — non-string `changes.language` is correct but untested, and double-reports.** `resolve_changes` (planner.rs:567) correctly emits `InvalidPropertyValue` for a non-`Str` language value (brief requirement), but there is no test. Additionally, in the full `dry-run` flow both passes run (`dry_run.rs:46,85`), so `language: true` yields `ValueTypeMismatch` (config, `validate_changes`) *and* `InvalidPropertyValue` (plan) at the same `config_path` — two errors for one mistake. Both accurate; slightly noisy. Add the missing test; the double-report is acceptable for v1.

5. **`with-attachments.json` uses 0-based attachment ids (0,1,2); real mkvmerge numbers from 1.** My v100 run showed attachment ids start at 1. The code is id-agnostic (passes through whatever `-J` reports), so this is functionally harmless, but the fixture models an id space that cannot occur and misleads anyone cross-checking against the binary. Minor fixture-realism nit.

6. **Eager resolution on the discarded-plan path (T6, broader than noted).** In `resolve_file` (planner.rs:515-526) `resolve_chapters` and `resolve_attachments` run unconditionally before `output.map(...)`; on a `render_output` failure they still do filesystem I/O (`resolve_locator`) and push diagnostics that will never accompany a surviving plan. Consistent with the deliberate pre-finalize design (`detect_source_overwrites` needs pre-drop plans), and harmless, but the chapters/attachments locator walks are wasted and can add extra diagnostics to an already-failing file. Low priority.

7. **Test hygiene: `std::mem::forget(dir)` leaks temp directories on disk.** Several `planner_resolution.rs` tests forget the `TempDir` guard; the assertions only compare owned `PathBuf`s and don't need the dir to outlive the guard, so the `forget` is unnecessary *and* permanently leaks `/tmp` dirs across runs. Prefer letting `dir` drop at function end. Also the `FakeIdent`+`lang()` helper is now duplicated across three test files (T12/T7) and the id-join snippet twice in `command.rs` (T11) — both are cheap DRY cleanups, not blockers.

8. **`ident.chapters` is parsed but unused.** `resolve_chapters` follows `profile.chapters` only and never consults the parsed count. Intentional per D9 (presence for future use), harmless; noting so it is not mistaken for a wiring gap.

9. **T9 (donor group order) and T3 (no id/size attachment-match test) are non-issues / trivial.** Donor ordering by first-`Some`-appearance cannot affect output order because `--track-order` explicitly encodes profile order (verified the group-0-empty + `--track-order 1:0` case runs exit 0). T3 is a pure coverage gap; the id/size matching path is exercised indirectly and is correct.

### Recommendations

- Extend the gated live test (Minor 1) — the single most valuable follow-up, since it converts my one-off manual v100 validation into a standing regression guard.
- Fold the batch-level `changes.language` consistency check into `walk_exact_languages` if you want the all-optional-unmatched invalid-language case caught (the T5 deliberate-scope note); low value since it is inert, fine to leave for v1.
- Fix the `input_groups` comment (Minor 2) and correct the fixture ids (Minor 5) while touching these files.
- Consider a planner "plan resolves to zero output tracks" warning (Minor 3), or defer explicitly to Plan 4.

### Assessment

**Ready to merge?** Yes

**Reasoning:** No correctness defects survived review; every item in the plan's focus list (multi-donor grouping, empty-primary group, `--track-order` indices, primary-vs-donor attachments, chapters/`--no-chapters` interaction, attachment resolution, title/language validation, `Matchable` generalization) is correct against real mkvmerge v100, including the previously-untested attachment-id identity. The remaining findings are test-coverage, comment-accuracy, and hygiene items suitable as follow-ups, not merge blockers.