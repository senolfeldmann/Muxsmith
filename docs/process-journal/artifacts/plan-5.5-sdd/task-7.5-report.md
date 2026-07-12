# Task 7.5 report: SourceOverwrite protection for attachment donors

## What was implemented

Extended T7's existing `resolved_sources` mechanism (did not build a
parallel one). `resolve_file` (crates/muxsmith-core/src/planner.rs) already
captures `resolved_sources` at a single point after all of a file's
resolution work but before `output`/`assignments`/`attachments` move into
the optional `Plan` - independent of whether `output` (and therefore
`plan`) ends up `Some`. That capture previously only chained
`Assignment.source` (track/primary donors, from T7). It now also chains
`attachments.add_files` (attachment donors from `resolve_attachments`):

```rust
let resolved_sources: Vec<PathBuf> = assignments
    .iter()
    .map(|a| a.source.clone())
    .chain(attachments.add_files.iter().cloned())
    .collect();
```

**Capture-point answer:** `resolve_attachments` already runs (line ~625,
just before the capture point) at the same pre-render point T7 used for
track donors - unconditionally, regardless of whether `render_output`
(called just before it) succeeded. So no new capture point was needed; the
existing one already sits downstream of attachment resolution and just
wasn't reading from it. This is architecturally identical to T7's fix: the
donor is known before rendering, only the *output* needs a rendered plan.

`detect_source_overwrites`'s doc comment was updated to describe both donor
kinds now flowing through `resolved_sources`; the S11 guard note (about
`AmbiguousExternal`'s deliberately-unprotected n-candidate case) was left
untouched since it is track-donor-specific - the attachment `add` locator
has no analogous single-slot-ambiguity branch (all locator hits are
attached, by design, so there is nothing comparable to disambiguate).

## TDD evidence

New test `source_overwrite_protects_attachment_donor_of_render_failed_file`
in `crates/muxsmith-core/tests/planner_resolution.rs`, mirroring T7's
`source_overwrite_protects_donor_of_render_failed_file` structure exactly,
substituting an `attachments.rules[].add` locator for the track rule's
`source.external`:

- Primary A resolves attachment donor D (`donors/Z.mkv`, real, on disk) via
  its `add` locator; A's own filename template renders empty ->
  `EmptyRenderedName`, `A.plan == None`.
- Primary B's own `add` locator finds nothing (its sibling `donors/` dir
  does not exist for B) - a `MissingExternal` warning only, non-fatal.
- B's rendered output (`{tag}` -> `"Z.mkv"`, output dir = `donors_dir`)
  lands on D's exact path.
- `CollisionPolicy::Overwrite` (not the Error default) proves
  `SourceOverwrite`, not the ordinary on-disk-collision path, is what stops
  this.

**RED** (before the fix): B's plan survived with only an Info-severity
`OutputCollision` diagnostic (`{"path": ".../a_dir/donors/Z.mkv"}`) - the
exact silent-overwrite exposure the brief describes.

**GREEN** (after the fix): `a.plan.is_none()` with `EmptyRenderedName`;
`b.plan.is_none()` with `DiagCode::SourceOverwrite`.

## Files changed

- `crates/muxsmith-core/src/planner.rs`: `resolved_sources` now chains
  `attachments.add_files`; three doc comments updated (`resolve_file`'s
  header comment, the capture-site comment, `detect_source_overwrites`'s
  header comment) to describe both donor kinds.
- `crates/muxsmith-core/tests/planner_resolution.rs`: new test, ~100 lines,
  placed directly after T7's test.

## Gate results

All 9 parts green, run in foreground from `/home/senol/Git/Muxsmith/.worktrees/stream-b`:

1. `cargo fmt --all --check` - clean
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean
3. `cargo test --workspace` - all suites pass (core, cli, gui, xtask
   codegen, doctests); new test included and green
4. `cargo deny check` - advisories/bans/licenses/sources ok
5. `pnpm lint` - clean
6. `pnpm build` (vue-tsc + vite) - clean
7. `pnpm check:i18n` - ok (pre-existing 12 unused-key warnings, unrelated)
8. `pnpm test:e2e` - 3/3 Playwright smoke tests pass
9. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` (ninth
   check) - 9 pre-existing errors across 4 files (`capability/runtime.rs`,
   `executor/joblog.rs`, `executor/queue.rs`, `muxsmith-cli/src/i18n.rs`
   x2, `src-tauri/src/lib.rs`), none in `planner.rs`. Verified identical
   error set on the pre-change tree via `git stash` / rerun / `git stash
   pop` - this diff adds zero new doc warnings.

## Self-review

- Mechanism is a direct extension of T7's, not a parallel one: same
  `(FileReport, Vec<PathBuf>)` return shape, same capture site, same
  `detect_source_overwrites` consumer - no new function, no new plumbing
  through `plan_core`.
- Test mirrors T7's constellation structure and naming closely enough that
  a future reader immediately sees the parallel; only the donor mechanism
  (attachment `add` vs. track `source.external`) differs.
- Core stays prose-free; the added prose is all in `//` comments (matching
  the file's existing convention - `resolve_file` and
  `detect_source_overwrites` were already extensively commented by T7, this
  follows the same style) and rustdoc-adjacent explanation, not user-facing
  strings.
- Diagnostics/severities untouched; no spec-facing behavior changed other
  than closing the protection gap itself (an `OutputCollision` Info that
  used to let B's plan survive now correctly becomes a fatal
  `SourceOverwrite`, only in the specific three-way constellation the test
  constructs).

## Concerns

- **Residual donor class outside this task's scope**: `resolve_chapters`
  (`ChapterSource::External`, planner.rs ~line 866) resolves a chapter XML
  donor via the same `discovery::resolve_locator` mechanism, at the same
  pre-render point (called just before `resolve_attachments`, same
  function), but that single `PathBuf` is *not* chained into
  `resolved_sources` either. This is the identical #7 exposure class for a
  third donor kind (chapters), not mentioned in the brief (scoped
  explicitly to "attachment donors") and not tracked in
  `docs/ROADMAP.md`'s M2/#7 entry, which only names the track-donor half. I
  did not fix it - out of this task's stated scope and no failing test was
  requested for it - but it should be triaged (either folded into #7's
  closure or logged as a new ROADMAP entry) since it is the same silent
  data-loss shape with the same fix (chain the chapter external path into
  `resolved_sources` too).
- Narrow real-world exposure, as the brief already notes: requires an
  attachment `add` locator hitting a `.mkv`-named file, since outputs are
  always `.mkv`. The fix generalizes with no such restriction (it would
  equally protect a non-`.mkv` attachment donor from a non-`.mkv` output
  collision, which cannot happen today, so no dead branch was added).
