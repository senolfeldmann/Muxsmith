# Task 12 report: rustdoc as ninth gate part (#18b)

Worktree: `.worktrees/stream-d`, branch `plan55-stream-d`. Commit `004e1e8`
("ci: cargo doc -D warnings as ninth gate part (#18b)").

## The known link fix

`crates/muxsmith-core/src/executor/queue.rs:73` — `QueueOpts::jobs` (public)
linked `[`worker_count`]`, a private `fn` at line 299. Delinked with meaning
preserved: `see the private \`worker_count\` helper` (plain code span, no
link brackets), since the item genuinely has no public doc page under a
plain `cargo doc --no-deps` and making it linkable would mean a visibility
change out of scope for a doc-correctness fix.

## Further warnings surfaced and fixed

`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` did not go
clean after the one known fix; three more dead/broken intra-doc links
surfaced incrementally (fixed one layer at a time, each re-run revealing
the next):

1. `crates/muxsmith-core/src/capability/runtime.rs:110` — `Mkvmerge::detect`
   (public) linked `[`platform_candidates`]`, a private `fn`. Delinked the
   same way (`the private \`platform_candidates\``).
2. `crates/muxsmith-core/src/executor/joblog.rs:124` — `RunLogger::create`
   (public) linked `[`JobAccumulator`]`, a private `struct`. Delinked
   (`one private \`JobAccumulator\` per index`).
3. `src-tauri/src/lib.rs:397` — `run()` (public) linked
   `[`run::on_close_requested`]`. `on_close_requested` is itself `pub fn`,
   but it lives in `mod run;`, which is private — the module boundary is
   what rustdoc rejects, not the function's own visibility. Delinked
   (`the private \`run::on_close_requested\``); rewrapped the surrounding
   two lines since removing the link brackets shifted line balance.

One case was a genuine misresolution rather than a truly private target and
got a real link fix instead of a delink:

4. `crates/muxsmith-cli/src/i18n.rs:53,59` — `Renderer::msg_with_count`'s
   doc comment linked bare `[`msg`]` twice, intending to point at the
   sibling public method `Renderer::msg`. A bare method name inside a doc
   comment on another method of the same `impl` block isn't in scope for
   rustdoc's path resolution; it needs `Self::` (or the type name). Fixed
   both occurrences to `[`Self::msg`]`, which resolves and links correctly
   (verified: the final clean `cargo doc` run generates the linked page).

All four defects are the same root-cause class the task exists to catch:
`#![deny(missing_docs)]` only gates *presence*, never checked whether a
link inside a present comment actually resolves.

## CI step placement

Added to `.github/workflows/ci.yml`'s `test:` job (all three matrix legs:
ubuntu-26.04, windows-2025, macos-15), immediately after `cargo test
--workspace` and before the pre-existing Task-2 (#14) mkvmerge-skip
assertion step — keeps the four core `cargo` gate commands (fmt, clippy,
test, doc) contiguous, ahead of the CI-only instrumentation and the
frontend (`pnpm`) steps:

```yaml
- name: cargo doc (rustdoc warnings as errors)
  run: cargo doc --workspace --no-deps
  env:
    RUSTDOCFLAGS: "-D warnings"
```

Did not touch the mkvtoolnix/Tauri install steps or any other existing
line — pure insertion, so the merge against master (which has since grown
a GITHUB_PATH block in the Windows install step) should stay conflict-free.
Did not add anything to the `deny:` job (Linux-only, supply-chain scope,
unrelated).

`cargo doc --workspace --no-deps` (no explicit `--workspace` in the brief's
local-check wording) was verified to be equivalent here: the repo's root
`Cargo.toml` is a virtual workspace manifest with no root package, so plain
`cargo doc --no-deps` already documents every member by default. Used the
explicit `--workspace` form in both BUILDING.md and CI to match the
existing style of the sibling `clippy`/`test` gate commands.

## BUILDING.md

"The Rust gate" section: four parts -> five (fmt, clippy, test, **doc**,
deny), same fenced bash-block style as before, with `RUSTDOCFLAGS="-D
warnings" cargo doc --workspace --no-deps` inserted between `cargo test
--workspace` and `cargo deny check`. Added one short prose line
distinguishing presence (`#![deny(missing_docs)]`) from correctness (the
new `cargo doc` run) — the exact distinction the task exists to close. The
closing CI-summary paragraph updated from "four-part Rust gate" to
"five-part Rust gate... (nine parts total)", matching the "eight-part ->
nine-part" phrasing already used elsewhere in the repo (ROADMAP.md,
recent handoffs) for this exact transition.

Did not touch `docs/ROADMAP.md`'s existing "rustdoc gate step + dead
intra-doc link" entry (#18b) — checked, and other already-merged Plan 5.5
task entries (#14, group T/#21) are still present in ROADMAP.md post-merge,
confirming entries are retired at plan close, not per-task, by convention.

## Full nine-part gate (foreground, all green)

1. `cargo fmt --all --check` — clean, no output.
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean.
3. `cargo test --workspace` — 0 failed across every crate (core suite 78
   passed, cli/gui/xtask suites all green, doc-tests 0/0 as expected).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — clean
   after all four link fixes.
5. `cargo deny check` — exit 0, "advisories ok, bans ok, licenses ok,
   sources ok".
6. `pnpm lint` — clean.
7. `pnpm build` — vue-tsc + vite build clean.
8. `pnpm check:i18n` — exit 0 ("ok", 12 pre-existing unused-key warnings
   unrelated to this task, unchanged by it).
9. `pnpm test:e2e` — 3/3 Playwright smoke tests passed.

## Self-review

- Diff touches exactly the 7 files this task owns: `queue.rs` (the known
  defect), the three further doc files the gate surfaced, `ci.yml`,
  `BUILDING.md`. No changes to `catalog_completeness.rs` or T11's test
  additions — confirmed via `git diff HEAD~1 --stat` before writing this
  report.
- No behavior change anywhere: every edit is doc-comment prose or CI/build
  config; `cargo test --workspace` (0 failed) and the unchanged clippy/fmt
  results confirm no code-path drift.
- Delink vs. relink was decided per-case on whether the target is
  genuinely private (module- or item-scoped, no public doc page under
  plain `cargo doc`) vs. a resolution mistake on an actually-public target
  (`Self::msg`) — not a blanket policy in either direction.
- Working tree is clean after the commit; nothing left uncommitted.

## Concerns

- None blocking. One judgment call worth flagging to the reviewer: the
  `run::on_close_requested` fix delinks rather than making `mod run` (or
  just that function's containing path) public. Making the module public
  would have kept a working link, but that's a visibility/API-surface
  decision beyond "fix the doc gate," so I treated it the same as the other
  two private-item cases. Flag if the reviewer wants the module made
  public instead.
- The CI step's actual behavior (green run on all three OS legs) is
  unverified beyond local execution — this task's scope was the local gate
  plus wiring the CI step, not a pushed/observed CI run.
