# Whole-branch verdict: Plan 5.7 (cd5e917..HEAD, 9 commits)

- Reviewer: whole-branch (T5), independent of all four streams; every claim below re-verified on the merged master, not taken from the T*-verdicts.
- Ground truth read: plan 5.7, ADR D37, spec 5.2 (row at spec:271), Tier-2 house files, the four T*-verdicts (context), the full range diff.

## VERDICT: READY

No blockers, no majors, no fix-requiring minors. Three informational notes (I1-I3), routed to the funnel/journal, none gating the push.

## 1. The textual auto-merge: catalog_completeness.rs - CLEAN

Read the merged file end to end. Both streams' additions landed complete and disjoint:

- T3: `run-signal-handler-unavailable` in `ALLOWLISTED_CLI_KEYS` (:205, after the joblog keys, catalog order preserved) AND in the zero-arg multi-pattern arm of `allowlisted_cli_key_args` (:221). Bidirectional wiring (orphan check + stale-entry check) covers it.
- T4: `DiagCode::NonUtf8Path` fixture arm (:118, `role: donor` + U+FFFD-carrying path - the non-default selector branch, the stronger choice) with the value-set tripwire comment.
- No shadowing/duplication possible by construction and verified empirically: `fixture_args` matches on enum variants (duplicate arm = compile error), the allowlist has an explicit dedup assert, `allowlisted_cli_key_args` string arms are clippy-guarded against unreachable patterns (clippy `-D warnings` green, below).
- Foreground run by this reviewer on merged HEAD: `cargo test -p muxsmith-cli --test catalog_completeness` -> **4 passed, 0 failed**. Also `cargo test -p muxsmith-core --test planner_non_utf8_path` -> **2 passed, 0 failed**.

## 2. Cross-stream semantic interaction (T3 x T4) - COMPOSES CORRECTLY, E2E-PROVEN

Interaction surface: T4 drops plans at finalize; T3's dry-run rendering (commands/mod.rs:101-115) prints the newly indented `dry-run-assignment`/`dry-run-output` lines only inside `if let Some(plan)`. A T4-dropped file therefore renders its header + diagnostic and **no** indent lines - no assumption of T3's is violated; the two changes touch the same output stream but disjoint branches.

**Targeted E2E (this reviewer, real binary, real mkvmerge v100.0, Unix):** two genuine SRT-track MKVs; attachment `add` locator (`recursive: true, match_to_source: true`); E01's jpg placed under a Latin-1-named directory (`extra\xe4s`), E02's jpg clean. This exercises the one CLI-reachable per-file vector: `resolve_locator` (discovery.rs:159-162) skips non-UTF-8 *filenames* but `walk_files` descends into non-UTF-8 *directories*, exactly verdict item 2's residual case 1. One `dry-run` invocation rendered:

```
.../Show.S01E01.mkv (identifier: S01E01)
[error] attachments: The attached file .../extra<U+FFFD>s/Show.S01E01.jpg is not valid UTF-8 and cannot be passed to mkvmerge without corruption.
.../Show.S01E02.mkv (identifier: S01E02)
  rule 0 -> track 0
  output: .../out/Show.S01E02.mkv
```

Exit 2. `sed -n l` over the E02 lines: exactly two literal spaces, no FSI/PDI bytes. **Indent and NonUtf8Path error coexist in one rendered output**; the new diagnostic renders correctly through the CLI diagnostic path (severity tag, config_path `attachments`, `[attachment]` selector arm, lossy U+FFFD path); dropped plan produced no plan lines and no job. One error only (dedup holds through the real path).

## 3. Plan-fidelity sweep (Tasks 1-4 vs the range diff) - ALL LANDED

- **T1** ✓ workflow-level `permissions: contents: read` after `on:`, 3-line rationale comment (re-wrapped per T1-F1). Note: merged commit `6c0a720` differs from the reviewed `7c75f00`; I diffed the two trees - the delta is **exactly** the F1 comment re-wrap, content byte-identical otherwise. Post-review fix correctly applied, nothing snuck in.
- **T2** ✓ `File::create` + `write_all` + `sync_all` + explicit `drop` + `fs::rename`, all three new fallible steps mapped via the file's existing `SettingsError::Io` closure; rustdoc claim kept and extended; no directory fsync; no joblog changes.
- **T3** ✓ both en/de cli.ftl pairs use the `{"  "}` placeable; run.rs `is_err()` guard renders `run-signal-handler-unavailable` via `renderer.msg` (joblog degradation shape); handler closure byte-identical to base; catalog registration complete (section 1).
- **T4** ✓ `detect_non_utf8_paths` before the first `finalize_plans`; enumeration mirrors `command`'s argv sites (output, chapters, attach-files, primary, track-carrying donors != primary); bilingual diagnostics.ftl entries with 5-arm selector; `DiagCode::NonUtf8Path` + rustdoc; spec 5.2 row at spec:271 accurate against the implementation; Unix-only integration test (2 tests) cfg-gated at file level. Review minors M1/M2 (`8a8aabb`) verified applied: collision-pass comment now names both drop sources; de says "Die anzuhängende Datei".
- **Rendered-output blast radius** ✓ the whole-range diff touches exactly 12 files; the only locale/renderer changes are the two indent lines (x2 languages), the new `run-signal-handler-unavailable` key (x2), and the new `non-utf8-path` entry (x2 - the planned D37 wire element). commands/mod.rs, i18n.rs, GUI rendering untouched. The two named output changes plus the planned new diagnostic are the only rendered-output changes.

## 4. Commit hygiene - PASS with one informational note

- All 9 commits unsigned (`%G?` = N) per proc-05. ✓
- `Co-Authored-By` trailer on all 9 - the trailer the tier-1 ledger entry actually requires. ✓
- Staging explicit: every commit's file list is exactly its task scope (verified per-commit `--stat`); no stray file anywhere in the range; worktree clean (only ignored `.superpowers/`, salvage pending at T5 per plan). ✓
- **I1 (informational):** `Claude-Session` trailer present on 9143866 + both merges, absent on the five subagent-authored commits (6c0a720, 17ae87c, 1cf10f9, 6a056eb, 8a8aabb). The documented convention requires only Co-Authored-By, so no violation; but T2's verdict adjudicated Claude-Session-addition as "conformance to house style", and the range now ships both styles. Pick one (ledger text or both-trailers) at the next convention touch.
- **I2 (informational, process):** `8a8aabb` (T4 minors) edits product artifacts (planner.rs comment, de catalog) on the stream branch pre-merge. Content matches the T4 reviewer's prescribed fixes exactly (verified in the range diff); if the controller applied it directly rather than via a fixer agent, that grazes "controller edits no product artifact" - comment/wording-only, zero behavior, no action needed, recorded for the process journal.

## 5. Gates re-run on merged HEAD (this reviewer, foreground)

| Gate | Result |
|---|---|
| `cargo test --workspace` | 37 targets, all `test result: ok`, 0 failed |
| `cargo test -p muxsmith-cli --test catalog_completeness` | 4 passed, 0 failed |
| `cargo test -p muxsmith-core --test planner_non_utf8_path` | 2 passed, 0 failed |
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| E2E dry-run (real binary + mkvmerge, mixed clean/non-UTF-8 fixture) | indent + NonUtf8Path coexist, exit 2, no FSI/PDI |

(cargo deny + frontend parts not re-run here: no dependency or frontend surface in the range; the controller's post-merge nine-part runs and the T5 CI push cover them.)

## 6. House dimension + harvest (whole branch)

House, over the merged whole: bilingual-same-commit held in both locale-touching commits; core-37 upheld (NonUtf8Path carries code + params only, prose lives in the catalogs); T3's warning conforms to the degradation shape; BTreeSet-based dedup keeps diagnostic order deterministic (house determinism posture); no bare-English eprintln anywhere in the range.

**I3 (informational, spec wording):** D37 and the spec 5.2 row say "once per offending **file**"; the implementation (correctly, per the T4 review) emits once per offending **path**, deduped within a plan - a plan with two distinct non-UTF-8 paths yields two diagnostics on one FileReport. Tighten the wording to "once per offending path" at the next spec touch; code is the better behavior.

Harvest (this review's own, beyond the four per-task lists which I re-endorse after whole-branch view - esp. T4-H1 de-catalog gate gap, T4-H2 completeness anchor, T2 atomic-publish shape, T3 Fluent-whitespace idiom):

- **H-A (product, funnel candidate):** the `UnknownExtension` warning misfires semantically on attachment `add` locators. My E2E's `extensions: [jpg]` drew "mkvmerge will not be able to process them" - false for `--attach-file`, which attaches arbitrary payloads (cover.jpg is the canonical attachment). Pre-existing (extension check from plan 5.5 Task 5.9 applies the mkvmerge *source* list to attachment locators), not a range regression; candidate ROADMAP v1.x polish item: exempt or re-word the check for `attachments.rules[i].add`.
- **H-B (test-asset recipe):** the one CLI-reachable per-file NonUtf8Path vector is an attachment/chapters donor under a non-UTF-8-named directory hit by a `recursive: true` locator (`resolve_locator` skips non-UTF-8 filenames but `walk_files` descends into non-UTF-8 dirs). Recorded here so a future CLI-level E2E suite does not have to re-derive it.
- **H-C (pattern reinforcement):** the branch's only textual auto-merge landed in a file whose design made the merge self-verifying (append-ordered allowlist + enum-keyed match arms + dedup assert + bidirectional wiring test): registry files built with tripwires turn cross-stream merges from a review risk into a compile/test-time check. Same family as T4-H2; worth one conventions.yaml entry when a third instance appears.

## Evidence summary

Range diff read in full (12 files, 301+/15-); merged catalog file read end to end; T1 reviewed-vs-merged tree delta diffed (comment re-wrap only); all gates above run foreground by this reviewer on merged HEAD; E2E executed against a reviewer-built fixture in scratch (nothing written into the repo). T*-verdict claims relied upon (T4 role-inventory completeness, T2 durability reasoning) were spot-re-verified against the diff and the E2E rather than trusted.
