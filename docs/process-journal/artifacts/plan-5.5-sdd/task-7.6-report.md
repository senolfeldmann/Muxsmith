# Task 7.6 report: SourceOverwrite protection for chapters donors - #7 class closure

Worktree: `/home/senol/Git/Muxsmith/.worktrees/stream-b` (branch `plan55-stream-b`)
Commit: `99b2e34` `fix(planner): protect chapters donors of render-failed files (#7 class closure)`

## What was implemented

`ChapterSource::External` (`resolve_chapters`, `planner.rs:~866`) resolves a chapters
donor via the same `resolve_locator` machinery and pre-render timing as a track
rule's external source, but was never fed into `resolved_sources` -
`detect_source_overwrites`'s protection set. A chapters donor referenced solely
by a file whose own output failed to render (`plan == None`) could be silently
overwritten by a different file's colliding output, identical to the #7 class
T7 (track donors) and T7.5 (attachment donors) already closed.

Fix in `resolve_file` (`planner.rs`): `resolved_sources` now chains a third
source - `chapters` (`ChapterSource::External(path) => Some(path)`, `Keep`/`Drop`
-> `None`) - alongside `assignments` (track rules/primaries) and
`attachments.add_files` (Task 7.5). Chained before `chapters` moves into `Plan`,
same pattern as the other two.

Class closure is now structural: `model.rs` has exactly two `Locator` field
sites - `ExternalBlock.external` (shared by `SourceCfg::External` and
`ChaptersCfg::External`) and `AttachmentRule.add` - and both now flow into
`resolved_sources` through all three donor kinds their resolution sites
produce (track rules, attachments, chapters).

## Completeness comment (brief mandate)

At the `resolved_sources` gathering site in `resolve_file`:

```rust
// Captured before `assignments`, `attachments` and `chapters` move into
// `Plan` below: every source this file resolved is known at this point
// already, regardless of whether `output` renders successfully.
// Completeness (Task 7.6, #7 class closure): every donor kind reaches
// this chain - track rules (`Assignment.source`), attachment `add`
// donors (`AttachmentPlan.add_files`, Task 7.5), and now chapters
// (`ChapterSource::External`, Task 7.6). `model.rs` has exactly two
// `Locator` field sites feeding these three kinds:
// `ExternalBlock.external` (shared by `SourceCfg::External` and
// `ChaptersCfg::External`) and `AttachmentRule.add`. A future third
// `Locator` field site must be chained in here too, or it silently
// re-opens this class.
```

Also updated the parallel doc comment on `detect_source_overwrites` (mirroring
how T7.5 updated it for attachments) to enumerate all three donor kinds and
note the class is now closed by construction, pointing back at the completeness
comment above.

## TDD evidence

New test `source_overwrite_protects_chapters_donor_of_render_failed_file`
(`crates/muxsmith-core/tests/planner_resolution.rs`), mirroring T7/T7.5's
three-way constellation, adapted for one real difference: chapters has no
`optional` escape (`resolve_chapters`'s docstring: "there is no `optional`
escape: zero matches is always `MissingExternal`"), unlike a track rule's
external source. T7/T7.5 made primary B's own donor lookup find nothing (its
sibling `donors` directory doesn't exist) with no penalty, because the track
rule/attachment rule tolerates a miss. Chapters can't tolerate a miss at all,
so B needed its *own* distinct, successfully-resolved chapters donor (a
`b_dir/donors/Z.mkv`, separate physical file from A's `a_dir/donors/Z.mkv`) so
its plan survives long enough to reach `detect_source_overwrites` rather than
dying earlier to an unrelated `MissingExternal`.

Constellation: primary A (`a_dir/Prime.mkv`) resolves chapters donor D
(`a_dir/donors/Z.mkv`, real, on disk) via `chapters.external`, but A's own
`{tag}`-template renders empty -> `EmptyRenderedName`, so `A.plan == None`.
Primary B (`b_dir/PrimeZ.mkv`) resolves its own distinct chapters donor
(`b_dir/donors/Z.mkv`) successfully, no diagnostic, but B's rendered output
(`output: Some(a_donors)`, template renders `"Z.mkv"`) lands exactly on D's
path. `CollisionPolicy::Overwrite` (not `Error`) to prove `SourceOverwrite` -
not the ordinary on-disk-collision path - is what stops this.

- **RED** (before the fix): `b.plan.is_some()`, only diagnostic was
  Info-severity `OutputCollision` (`diags: [Diagnostic { code:
  OutputCollision, severity: Info, ... }]`) - B's plan survived, would have
  silently overwritten D on a real run. Assertions on `b.plan.is_none()` and
  `SourceOverwrite` both failed.
- **GREEN** (after the fix): test passes; `b.plan.is_none()` with a
  `SourceOverwrite` diagnostic.

## Files changed

- `crates/muxsmith-core/src/planner.rs` - chained `chapters` into
  `resolved_sources`; completeness comment; updated `detect_source_overwrites`
  doc comment.
- `crates/muxsmith-core/tests/planner_resolution.rs` - new test (+109 lines).

## Gate results

All run in foreground, from the worktree root.

1. `cargo fmt --all --check` - clean.
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean, no warnings.
3. `cargo test --workspace` - all green (planner_resolution: 61 passed, 0
   failed, includes the new test; full workspace: no `FAILED`/`error[` lines).
4. `cargo deny check` - `advisories ok, bans ok, licenses ok, sources ok`,
   exit 0.
5. `pnpm lint` - clean.
6. `pnpm build` - vue-tsc + vite build succeeded.
7. `pnpm check:i18n` - `ok (16 source files scanned, 173 catalog ids, 12
   unused warning(s))` - the 12 unused-key warnings are pre-existing backend
   error-message keys, unrelated to this diff.
8. `pnpm test:e2e` - 3 passed.
9. **Rustdoc ninth check**: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace
   --no-deps` fails with 9 pre-existing errors outside this diff
   (`capability/runtime.rs`, `executor/joblog.rs`, `executor/queue.rs`,
   `muxsmith-cli/i18n.rs`, `src-tauri/src/lib.rs`). Verified byte-identical
   before/after via `git stash` + `diff` on the sorted `error:` line sets
   (`IDENTICAL - no new doc warnings from this diff`) - this diff adds none.

## Self-review

- `resolved_sources`'s new `.chain(...)` borrows `&chapters` and clones only
  the `PathBuf` out; `chapters` itself is moved into `Plan` a few lines later
  (`output.map(|output| Plan { ..., chapters, ... })`), identical borrow
  pattern to the pre-existing `attachments.add_files.iter().cloned()` chain
  link - compiles clean, no borrow conflict.
- Exhaustive match (`External` / `Keep` / `Drop`) rather than `if let`: any
  future `ChapterSource` variant forces a compile-time decision here, in
  keeping with the "closed by construction" intent of the task.
- Verified the `AmbiguousExternal` (S11 guard) and `MissingExternal` branches
  of `resolve_chapters` both return the `Keep` placeholder, which the new
  chain maps to `None` - consistent with the existing guard's reasoning for
  track-rule donors: an ambiguous or missing chapters candidate contributes no
  path, and is safe only because both branches are unconditionally
  Error-severity (that file's own plan never survives regardless).
  `resolve_chapters`'s existing docstring already documents this ("Both error
  branches return `Keep` as a placeholder").
  Confirmed both `resolve_chapters` error branches are unconditionally
  `Diagnostic::error(...)` (no severity downgrade path), so the guard holds
  today; if that ever changes the S11 guard's caveat (already in the code
  comment) applies equally to chapters.
- Confirmed via `grep` that `resolved_sources` / `ChapterSource::External` are
  only referenced within `planner.rs` (plus `command.rs`'s unrelated
  downstream mkvmerge-argv construction) - no parallel resolved-sources
  tracking elsewhere in the codebase needed the same fix.
- Confirmed the two `Locator` field sites named in the completeness comment
  against `model.rs`: `ExternalBlock.external` (used by both
  `SourceCfg::External(ExternalBlock)` and `ChaptersCfg::External(ExternalBlock)`)
  and `AttachmentRule.add: Option<Locator>` (a direct field, not wrapped) -
  matches the brief's phrasing exactly.

## Concerns

None. The #7 data-loss class (silent overwrite of a donor belonging to a
render-failed file) is closed by construction: `resolved_sources` now chains
all three donor kinds reachable through the two `Locator` field sites in
`model.rs`, and the completeness comment makes a future fourth `Locator`
position visibly demand a corresponding chain entry. No open items carried
forward from this task.
