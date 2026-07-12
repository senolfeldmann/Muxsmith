# Task 7 report: SourceOverwrite completeness + S11 guard comment (#7)

Commit: `0456f72` on `plan55-stream-b`
(`/home/senol/Git/Muxsmith/.worktrees/stream-b`).

## What was implemented

`detect_source_overwrites` (planner.rs) gathered protected input paths only
from files whose plan rendered (`f.plan.is_some()` -> `plan.assignments`). A
donor resolved by a file whose own output-filename template failed to
render never survived into a `Plan` (assignments live only inside `Plan`),
so it silently dropped out of the protection set: a colliding output
elsewhere in the batch could overwrite that donor with no diagnostic
(Plan-2 FINAL review finding M2; the only audit finding with data-loss
potential, per `docs/ROADMAP.md`).

Fix, per brief Step 2 (interfaces unchanged: `Batch`/`FileReport`/`Plan`/
`plan_core`/`plan_batch` public shapes are identical to before):

- `resolve_file` (private) now returns `(FileReport, Vec<PathBuf>)`. The
  second element is every `Assignment.source` path this file resolved
  (primary or donor), captured right after the assignments loop and before
  `assignments` moves into `Plan` - so it exists regardless of whether
  `render_output` later succeeds. The two early-return paths
  (`UnidentifiableSource`, `UnsupportedSource`) return an empty `Vec` since
  no assignments were built yet at that point.
- `plan_core` accumulates these into a flat `resolved_sources: Vec<PathBuf>`
  across all primaries and passes it to `detect_source_overwrites` alongside
  `primary_paths` (chose a flat `Vec<PathBuf>` over a per-file
  `Vec<Vec<PathBuf>>`, since the function only ever unions everything into
  one `BTreeSet` - no need for per-file granularity there).
- `detect_source_overwrites` now takes `resolved_sources: &[PathBuf]` as a
  third parameter and unions it straight into `inputs`, replacing the old
  `for f in files.iter() { if let Some(plan) = &f.plan { ... } }` gather.

Step 3 (S11 guard): added a comment block on `detect_source_overwrites`
documenting that `resolve_file`'s `AmbiguousExternal` branch (2+ candidate
donors for one locator) deliberately pushes a placeholder assignment sourced
at the primary path, not at any of the n candidates, since which one is
"the" donor is genuinely unknown. Safe only because `AmbiguousExternal`
stays unconditionally Error-severity (that file's own plan never survives
regardless); flagged to revisit if it is ever downgraded to non-fatal (F5
report), per `docs/ROADMAP.md`'s "docs-tree S11" note.

## TDD evidence

**RED** (Step 1): `source_overwrite_protects_donor_of_render_failed_file`
in `crates/muxsmith-core/tests/planner_resolution.rs`. Three-way
constellation: primary A ("Prime.mkv") has an external rule (relative path
`donors`, resolved against A's own directory) that finds donor D
("Z.mkv", real file on disk) via an unrestricted locator; A's own output
template (`{tag}`, `tag` an optional capture absent from A's filename)
renders empty -> `EmptyRenderedName` -> `A.plan == None`. Primary B
("PrimeZ.mkv", `tag` = "Z") lives in a sibling directory with no `donors/`
subdirectory, so its own (optional) external rule finds nothing - B never
references D itself. B's template renders `{tag}` = "Z" -> "Z.mkv", and
with the batch's shared output directory pointed at A's donors dir, B's
absolute output path lands exactly on D's path.

Used `CollisionPolicy::Overwrite` deliberately (not the `Error` default) to
prove `SourceOverwrite` - not the ordinary on-disk-collision path - is what
stops this: under `Overwrite`, an on-disk collision that the code does not
recognize as a batch input is only Info-severity and does not null the plan
(only Error does). Against the pre-fix code this is exactly the silent
data-loss shape M2 described: the test failed at
`assert!(b.plan.is_none())` because `b.plan` was `Some` (only an
Info-severity `OutputCollision` fired) - i.e. a real run would have
overwritten donor D with no error surfaced.

**GREEN**: after the fix, `b.plan.is_none()` and
`b.diagnostics` contains `SourceOverwrite` (Error). Full command output
captured during the session; rerun via:
`cargo test -p muxsmith-core --test planner_resolution source_overwrite_protects_donor_of_render_failed_file`.

## Files changed

- `crates/muxsmith-core/src/planner.rs` - `resolve_file` return type,
  `plan_core`'s call site, `detect_source_overwrites` signature/body/
  comments.
- `crates/muxsmith-core/tests/planner_resolution.rs` - new test
  `source_overwrite_protects_donor_of_render_failed_file`.

## Gate results

All run foreground from the worktree root.

1. `cargo fmt --all --check` - PASS (one block needed `cargo fmt --all`
   once, applied, then check passed clean).
2. `cargo clippy --workspace --all-targets -- -D warnings` - PASS, no
   warnings.
3. `cargo test --workspace` - PASS, all suites green (planner_resolution:
   55 passed, including the new test; no failures anywhere in the
   workspace).
4. `cargo deny check` - PASS (`advisories ok, bans ok, licenses ok, sources
   ok`).
5. `pnpm lint` - PASS.
6. `pnpm build` - PASS (`vue-tsc --noEmit && vite build`).
7. `pnpm check:i18n` - PASS (12 pre-existing "unused" warnings, unrelated,
   informational only; exit 0).
8. `pnpm test:e2e` - PASS (3/3 Playwright specs green).
9. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` (the new 9th check) -
   **FAILS**, but not from this task's diff. Verified via `git stash` that
   the exact same 4 errors (3 in `muxsmith-core`, 2 more surfacing in
   `muxsmith-cli` on a full-workspace `--doc --workspace` run) already exist
   on this worktree's HEAD (`a60e9a0`) before any Task 7 change:
   - `capability/runtime.rs:110` - public doc links to private
     `platform_candidates`.
   - `executor/joblog.rs:124` - public doc links to private
     `JobAccumulator`.
   - `executor/queue.rs:73` - public doc links to private `worker_count`.
   - `muxsmith-cli`: two "unresolved link to `msg`" errors.

   None of these files are in Task 7's scope (`planner.rs` only) or touched
   by this diff. Confirmed no new rustdoc errors are introduced by this
   change (identical error set before/after, byte-for-byte). Flagged as a
   concern below rather than fixed, to avoid scope creep into other
   tasks'/streams' files.

## Self-review

- Public interfaces unchanged: `Batch`, `FileReport`, `Plan`, `plan_core`,
  `plan_batch` all identical to before (per brief: "Interfaces: none new").
  Only `resolve_file` and `detect_source_overwrites` (both private)
  changed shape.
- The fix protects sources from ALL files' resolved assignments
  independent of render success, exactly per Step 2's instruction ("sources
  are known before rendering; only outputs need a rendered plan") - not
  scoped to render-failure only, so it also naturally covers any other
  reason a plan might end up `None` (e.g. a resolution error that fires
  after assignments are built) as a side effect, which is strictly more
  correct, not narrower.
- Verified the `AmbiguousExternal` fallback assignment's `source` field is
  always `primary.path.clone()` (never a real donor candidate), so pulling
  ALL assignment sources into the protection set (not just non-primary
  ones) adds no incorrect protection there - the S11 guard comment
  documents this precisely rather than changing behavior.
- Confirmed the flat `Vec<PathBuf>` (vs. a per-file `Vec<Vec<PathBuf>>`) is
  sufficient: `detect_source_overwrites` only ever unions everything into
  one `BTreeSet<PathBuf>`, so no caller ever needed per-file separation -
  chose the simpler shape per the project's scale-appropriate-design
  convention.
- Confirmed via `git stash` twice (scoped `-p muxsmith-core` and
  `--workspace`) that the rustdoc gate-9 failures are 100% pre-existing on
  this worktree's HEAD, not introduced by this diff.

## Concerns

- **Gate-9 (rustdoc) is red on this worktree independent of Task 7**, in
  files outside this task's scope (`capability/runtime.rs`,
  `executor/joblog.rs`, `executor/queue.rs`, and a `msg`-link issue in
  `muxsmith-cli`). This needs a fix - either by whichever task/stream owns
  those files, or as a dedicated cleanup task - before the branch as a
  whole can pass the full 9-part gate at merge. Task 7's own contribution
  to `planner.rs` is rustdoc-clean.
- No other concerns; the fix is narrowly scoped to the described gap and
  the new test exercises exactly the three-way constellation the brief
  specifies.
