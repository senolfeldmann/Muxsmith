# Whole-branch fix round - Plan 9

**Role:** fresh implementer for the one blocking finding of Plan 9's
whole-branch review. You wrote none of this branch. Model tier: mid (dispatch
model: Opus 5). Effort: xhigh. The whole-branch reviewer, resumed, judges your
delta.

Scope: **two rustdoc edits in one file.** No code, no test, no other file.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- A bare `cp` here is aliased interactive; if you mutate anything to fire a
  check, restore with `git checkout --` and prove it.

## The finding

Plan 9 amended spec 5.5 (amendment S-8, owner ruling: **no GUI identification
session cache**). The amended spec now reads, at
`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`:

> Identification cache: in-memory, keyed on path + mtime + size, **constructed
> per planning call and dropped with it**. One call identifies each unchanged
> file once (run plans and executes within a single call, so its planning pass
> and the suggestion engine's re-simulations share one cache); **separate calls
> re-identify**, so a GUI dry-run followed by a run spawns `mkvmerge -J` per
> file in each (a per-session shared cache was ruled out 2026-07-28 as
> unnecessary). In the CLI, call and process coincide.

Two rustdoc passages in `crates/muxsmith-core/src/identify.rs` still describe
the cache the owner ruled out, and both cite the very section that now says the
opposite. Task 1's fix round swept for the literal string `per-session` and
therefore caught only a third site; these two survive because they state the
same thing in different words. The controller verified both at the source
before dispatching you.

**Site 1 - the module doc, `crates/muxsmith-core/src/identify.rs:3-4`:**

```rust
//! and caches results in memory keyed on path + mtime + size so dry-run and
//! run never re-identify an unchanged file (spec 5.5).
```

False on every surface of the shipped product: the planner seam constructs its
cache per planning call (`crates/muxsmith-core/src/pipeline.rs`, D93), so a GUI
dry-run followed by a run re-identifies every file, and in the CLI dry-run and
run are separate processes anyway.

**Site 2 - the `IdentifyCache` doc, `crates/muxsmith-core/src/identify.rs:302`:**

```rust
/// In-memory identification cache for one session (spec 5.5). Keyed on path
```

"for one session" is exactly the per-session description the owner ruled out.

## What to write

Re-point both docs at the amended spec. **The semantics are fixed; the exact
prose is yours** within them - this is one of the few places in this plan where
that is true, and it is true because the falsified claims are what bind, not a
fenced wording.

- Site 1 must scope the cache's effect to ONE planning call, and must not
  promise that a later call reuses it. The reviewer's suggested shape: "so one
  planning call never re-identifies an unchanged file (spec 5.5; separate calls
  re-identify)".
- Site 2 must drop "for one session" and say what the cache's lifetime actually
  is. Suggested shape: "In-memory identification cache, constructed per planning
  call and dropped with it (spec 5.5)."
- Both keep their remaining true content: the key (path plus mtime and size),
  the changed-file-re-identifies consequence, and the on-disk-cache-is-a-future-
  candidate note.
- Read the amended spec passage yourself before writing, and read
  `pipeline.rs`'s cache construction, so your wording describes the code rather
  than paraphrasing this brief.

**Then sweep the file for the same class before you stop.** The whole finding
exists because a literal-phrase sweep missed two paraphrases. Do not repeat it:
read every doc comment in `identify.rs` that touches caching or lifetime, and
report what you found - including "nothing else" if that is the answer, with the
method you used to establish it. Anything you find in ANOTHER file goes in your
report, not in your diff.

## Verification

- `cargo fmt --all --check`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean, because
  you are editing rustdoc.
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` - unchanged: **39** `test result:` lines, all ok.
  Report what you measure if it differs.
- No frontend leg: you touch no `src/` or `e2e/` file.
- One thing you cannot fire and should not pretend to: no test asserts these
  strings, which is precisely why the defect reached a whole-branch review.
  Say so in your report rather than manufacturing a check.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push. `git -c commit.gpgsign=false`,
stage the one file by name (never `git add -A`), exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line. The message states that this is the whole-branch review's MEDIUM finding
and what was false.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/whole-branch-fix-report.md`,
same content as your final message: both edits with their before and after, the
spec passage you checked them against, the same-class sweep and its method,
anything found outside this file, the pasted verification, and the commit hash
with `git show --stat`.
