# Task 5 fix round - Plan 9

**Role:** fresh implementer for the Task-5 fix round. You did not write Task 5.
Model tier: mid (dispatch model: Opus 5). Effort: xhigh. The original reviewer,
resumed, judges your delta.

Task 5 (`e134fdc`) is APPROVED_WITH_MINORS. Two of the four findings are
report-count defects the controller already routed as tracker residue; you fix
the other two. Both are written out to the line below, and the controller
verified both at the source before dispatching, so neither is a claim you have
to take on trust - but do read the sites before editing.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- Scope is EXACTLY the two changes below, in exactly two files. Nothing else:
  no drive-by cleanups, no doc improvements elsewhere, no test additions beyond
  the one assertion named. Anything you find outside this scope goes in your
  report, not in the diff.
- **No design latitude.** A fork returns as NEEDS_CONTEXT with a decision memo
  before it is resolved, never after.

## Fix 1 (MEDIUM-1): the re-export's doc comment states a false fact

**Site:** `crates/muxsmith-cli/src/commands/mod.rs:16-18`, the doc comment on
`pub(crate) use muxsmith_core::report::severity_sorted;`.

Its appositive claims the call sites are "the human printing paths this crate
owns". That is false for one of the nine: `crates/muxsmith-cli/src/commands/validate.rs:21`
sorts a vector that feeds BOTH output modes - the `--json` envelope built at
`validate.rs:29` and the human loop below it. The call site's own pre-existing
comment says so ("both output modes share it"), and D102's consumers sweep in
the Plan-9 design describes that call site as the JSON side. The false half is
exactly the belief that sweep exists to prevent: it invites someone to re-add a
second sort on validate's envelope.

Apply the reviewer's minimal correct form:

```rust
/// The one error-first ordering definition, hoisted to core (D102) and
/// re-exported here so every `crate::commands::severity_sorted` call site
/// -- this crate's human printing paths and `validate`'s own `--json`
/// envelope -- is unchanged.
```

Doc-only. ASCII `--` as written; no other line in the file changes.

## Fix 2 (LOW-3): a test comment describes an assertion the code does not make

**Site:** `crates/muxsmith-cli/tests/dry_run_cli.rs:443-448`, in
`dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`.

The comment says "`files` is present and `mkvmerge_found` absent"; only the
`mkvmerge_found` assertion exists. `mkvmerge_found`-absent alone does not
identify `batch_document`: the CLI's profile-load-failure path builds
`config_only_document(&[diagnostic], None, renderer)`, which also omits the
key.

The reviewer's preferred repair, which you apply: **add the missing half**, an
assertion that `files` is an array, beside the existing one, so the comment
becomes true and the load-failure path is closed at this assertion instead of
relying on the code-sequence assertion below. Shape it like its neighbour
(same `assert!` form, a message that names what was expected). Do not touch the
existing `mkvmerge_found` assertion, the fixture, or anything below.

## Verification (foreground, no subsets within each command)

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` - paste the aggregate. The house unit is
  `test result:` lines: **39** on the current tree (35 test binaries + 4
  doc-test targets), all ok, 0 failed. Report what you measure, not what this
  brief says, if they differ.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean, because
  fix 1 edits rustdoc. (Do NOT add `--document-private-items`: that run fails
  on two pre-existing ambiguous links in `src-tauri`, tracked in the ROADMAP,
  and is not yours to fix.)
- **Fix 2 needs its own fire**: show that the new assertion can go red. Mutate
  the document under test or the assertion's target so `files` is absent, watch
  it fail, restore non-interactively (`git checkout --`, never a bare `cp` -
  it is aliased interactive here), and prove the restore (`sha256sum -c`
  against a baseline you take BEFORE mutating, plus `git status --porcelain`).
- The frontend is untouched by this round, so no `pnpm` leg is required. The
  controller runs the full gate before any push.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push. `git -c commit.gpgsign=false`,
stage the two files by name (never `git add -A`), exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line. Message: state that this is the Task-5 fix round, and name each finding
by its tag (MEDIUM-1, LOW-3) with one clause of what was wrong.

## Report

Append to `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-5-report.md`
under a `## Fix round` heading, same content as your final message: what each
fix changed, the pasted evidence including the fire for fix 2 and its restore
proof, anything you found and did not touch, and the commit hash with
`git show --stat`.
