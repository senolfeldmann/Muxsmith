# T2 verdict: settings.rs fsync before atomic rename (Stream B, verdict item 6)

Independent review of commit `91438667c743a5f352170c38a3ccdf0d983da54a` on
`plan57-b` (worktree `.worktrees/plan57-b`). Reviewed against the plan
(Task 2 + standing constraints), adjudication verdict item 6, the Tier-2
house files (`docs/conventions.yaml`, `docs/process-conventions.yaml`),
and the diff itself; every implementer-report claim re-verified on disk.

## VERDICT: APPROVED

One minor report-accuracy finding (test count off by one, no code impact).
No substance, correctness, scope, or house-style defect. All three
surfaced deviations adjudicated below; none is an actual deviation.

## Checks performed (all independently re-run, not taken from the report)

1. **Write path is the contracted shape - PASS.** `git show 9143866`
   matches the report's diff byte-for-byte; on-disk `settings.rs:176-186`
   is `fs::File::create` -> `write_all` -> `sync_all` -> explicit
   `drop` -> `fs::rename`, in that order. Each of the three new fallible
   steps maps via `.map_err(|e| SettingsError::Io(e.to_string()))` - the
   identical closure the file already uses at :160 (`create_dir_all`),
   :187 (`fs::rename`), and :124 (`read_to_string`). No new error
   variant, no composed prose (`detail` = OS error text only, the
   documented core-37 shell-side analogue at :89-92 holds). Call style
   (`fs::File::create`, module-prefixed) matches the file's existing
   `fs::write`/`fs::rename`/`fs::read_to_string`; the `Write` trait
   import is folded into the existing `use std::io;` line - minimal.

2. **Durability claim is now true as written - HOLDS.** Reasoned through
   the power-loss window independently:
   - `sync_all` returns only after the temp file's data and metadata are
     durable (fsync on Linux, F_FULLFSYNC via std on macOS,
     FlushFileBuffers on Windows). The rename is issued strictly after
     `sync_all` returns, so the data blocks are on disk *happened-before*
     the rename ever reaches the filesystem - the delayed-allocation
     reordering (rename journaled before data blocks land) that item 6
     identified is closed.
   - Power cut before the rename commits: the final path still holds the
     previous complete file (itself published through this same fsynced
     path), or no file on a true first run - which `load` (:123) maps to
     defaults. Within the contract.
   - Power cut after the rename commits: the new file's bytes are already
     durable, so the reader sees the new complete file. Never a torn one.
   - NO directory fsync is correct scope: losing the rename yields the
     *previous* complete file, which the rustdoc's claim ("previous
     complete or new complete, never partial") explicitly permits; the
     doc never promises rename durability. Exactly the adjudication's
     boundary.
   - The explicit `drop(tmp_file)` before the rename is sound and its
     comment accurate: it preserves `fs::write`'s open-write-close parity
     instead of relying on Windows share-mode defaults permitting
     rename-while-open. Closing after `sync_all` costs nothing
     durability-wise.
   - The added rustdoc paragraph (:145-151) states this mechanism
     correctly (ext4/btrfs delayed-allocation example is accurate); the
     original claim text was kept verbatim, not weakened - as the plan
     required ("keep the rustdoc claim (it becomes true as written)").

3. **Scope discipline - PASS.** `git show --stat`: exactly 1 file,
   `src-tauri/src/settings.rs`, 19 insertions / 2 deletions. Repo-wide
   grep: `sync_all` exists nowhere else; no joblog file touched
   (separately tracked v1.x, per item 6 and the plan); no directory
   fsync anywhere. Tests unmodified (the diff touches no `#[cfg(test)]`
   line). Branch topology: `plan57-b` = master HEAD (`cd5e917`, the plan
   commit) + exactly this one commit; merge-base = master HEAD; worktree
   clean (`git status --short` empty). Commit unsigned (`%G?` = N),
   conforming proc-05.

4. **19-insertion accounting - NOTHING BEYOND CONTRACT.** 1 import line +
   7 rustdoc lines + 11 write-path lines (create 1, write_all 3,
   sync_all 3, parity comment 3, drop 1). The two deletions are the old
   import and the replaced `fs::write` line.
   - **The new intra-doc link `[`fs::File::sync_all`]` - ACCEPTED.** The
     surrounding rustdoc already documents the publish mechanism
     step-by-step with intra-doc links (`[`fs::rename`]` at :141); a new
     load-bearing step documented in the same style is completion of the
     mechanism doc, not scope creep, and satisfies BUILDING.md's
     "comments state MEANING" bar. It is behavior-free and gated by the
     `cargo doc -D warnings` part (re-verified below).
   - The 7-line rustdoc paragraph is within the plan's contract in
     spirit: "keep the claim" was honored verbatim, and leaving the new
     fsync step undocumented would have made the step-by-step mechanism
     doc silently incomplete - the worse outcome in this file's
     exhaustively-rationaled house style.

## Deviation adjudications (three surfaced by the implementer)

**(a) Claude-Session trailer added - CONFORMANCE, not deviation.**
Verified: all 3 most recent master commits carry both `Co-Authored-By`
and `Claude-Session:` trailers; the plan's standing constraints say
"trailer per convention". The task's literal command was a floor, not a
ceiling; matching the repo's actual commit style is the house dimension
working as intended. No action.

**(b) NO temp-file cleanup on failed write/sync - ACCEPTABLE, not a
finding.** Verified pre-change behavior at `9143866^:settings.rs`:
`fs::write(&tmp_path, ...)` is internally create + write_all, so a failed
write already left the created temp file behind; only the failed *rename*
path cleans up - exact behavioral parity, as claimed. Sibling check:
settings.rs is the repo's only temp+rename atomic-write site (repo-wide
grep for `fs::rename`), so there is no divergent sibling pattern to
conform to. Mitigations already present: the temp name embeds the pid, so
the next save from the same process reuses (and renames away) the same
temp path; the directory-listing regression test asserts a successful
save leaves only `settings.json`. Residual exposure - a stale hidden
`.settings.json.tmp-<pid>` after a failed write/sync, accumulating only
under repeated write failures across distinct pids - is cosmetic, was
equally present before this change, and is outside item 6's named minimal
fix. Adding cleanup would have widened the diff and the test surface for
zero contract value. Right call; surfacing it instead of silently
expanding scope is the correct process behavior. If cleanup is ever
wanted, it belongs to the v1.x joblog atomic-write work as part of a
shared publish shape, not here.

**(c) cargo deny run despite no dependency change - REQUIRED, not
deviation.** ci-06-per-commit-gate: all gate parts "before every commit,
never skipped". Running it was conformance; skipping would have needed
the justification. (Contrast T1's sanctioned narrowing: that was a
docs/CI-only diff outside the gate's observable surface; this diff is
product Rust code, squarely inside it - full gate correct here.) No
action.

## Findings

- **F1 (minor, report accuracy only, non-blocking):** the report claims
  "all 7 settings tests"; the settings `mod tests` contains **8**
  `#[test]` functions (verified on disk and in the independent run
  below: 8 `settings::tests::` tests executed). No code impact; noted so
  the T5 roll-up funnel does not propagate the wrong count.

## Independent test run (this reviewer, FOREGROUND, worktree root)

Package name verified in `src-tauri/Cargo.toml`: `muxsmith-gui`
(lib target `muxsmith_gui_lib`).

- `cargo test -p muxsmith-gui` (foreground, this reviewer):
  **78 passed / 0 failed** on the lib target, 0 on the `main.rs` and
  doc-test targets - matches the implementer's numbers exactly.
- `cargo test -p muxsmith-gui settings` (foreground): all **8**
  `settings::tests::` functions ran and passed, including both
  atomic-publish regression guards
  (`save_leaves_no_temp_file_behind_after_a_successful_write`,
  `save_cleans_up_its_temp_file_when_the_publish_rename_fails`) - the
  latter exercises the failed-rename cleanup path over the new
  create/write/sync/drop sequence. Tests pass unchanged, as the task
  requires; the filter run also confirms F1 (8 tests, not 7).
- `RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-gui --no-deps`
  (foreground): exit 0 - the new `[`fs::File::sync_all`]` intra-doc
  link resolves; the report's doc-gate claim independently confirmed.

## Harvest (house dimension, per Tier-2 reviewer brief)

- **Ledger candidate (nature: technical-code, domain: shell/persistence):
  the house atomic-publish shape is now fixed by this commit** -
  same-directory temp (pid-suffixed, dot-prefixed) -> `write_all` ->
  `sync_all` -> close -> `fs::rename` as the only publish point ->
  cleanup only on failed rename; directory fsync deliberately out
  (contract is previous-or-new-complete, not rename durability). The
  ROADMAP v1.x joblog atomic-write item should clone THIS shape, fsync
  included - recording it prevents the v1.x work from re-introducing the
  exact gap item 6 just closed.
- **Reinforcement occurrence:** deviations disclosed, never silently
  resolved - third consecutive task (5.6 T1, 5.7 T1, now 5.7 T2 with
  three surfaced items, two of which turned out to be conformance).
  Consistent with proc-01's disclosure culture; the pattern is doing
  real work (deviation (b) is precisely the kind of judgment call that
  silent scope expansion would have hidden).
- **Observation for the brief template:** two of three "deviations" here
  were house-conformance acts (trailer, cargo deny) that the implementer
  had to defensively surface because the task text was narrower than the
  house rules. Briefs that say "commit with trailers per convention; run
  the full ci-06 gate" instead of literal commands would cut this
  disclosure noise without losing the culture.

## Summary

APPROVED. Diff is exactly the contracted fix; the durability rustdoc is
now true as written (power-loss window reasoned through independently);
scope is clean (1 file, no dir fsync, no joblog); all three surfaced
deviations adjudicated as non-deviations or correct restraint; tests and
doc gate independently re-run green (78/78, 8/8 settings, doc exit 0).
One minor report-count inaccuracy (F1) for the T5 funnel.
