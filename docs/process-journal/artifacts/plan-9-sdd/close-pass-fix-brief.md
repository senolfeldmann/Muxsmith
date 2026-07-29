# Plan-9 close pass - fix round brief

**Role:** fresh implementer for the close pass's review findings. You did not
write the pass. Model tier: mid (dispatch model: Opus 5). Effort: xhigh. The
close-pass reviewer, resumed, judges your delta.

Six edits across five files. All prose or documentation; **no behaviour change
anywhere**, and no assertion, test or command semantics change.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- A bare `cp` here is aliased interactive; restore with `git checkout --`, with
  the baseline taken first, and prove it.
- **Read the verdict for each finding** at
  `.superpowers/sdd/plan-9/close-pass-verdict.md`; it carries the measurement
  behind each one and, for most, the exact required text.

## The six edits

### 1. F1 - `crates/muxsmith-cli/tests/dry_run_cli.rs`, the shape-guard comment

The comment says the profile-load-failure shape "carries neither key". It
carries `files: []` - only `mkvmerge_found` is absent. Two agents measured this
independently against the real document, and the emitter
(`config_only_document`) writes `"files": []` unconditionally.

The text came from a fenced block in an earlier verdict, and the fence is why
the previous round applied it verbatim rather than correcting it. **That fence
is lifted for this edit by the controller, who erected it.** Apply the verdict's
F1 replacement comment. The assertions do not change.

### 2. F2 - `e2e/jobsview-reset.spec.ts`, the installer disclosure

Its safety clause names three settings consumers; the tree has six
(`main.ts`, `SettingsDialog.vue`, `recentProfiles.ts`, `BatchView.vue`,
`EditorView.vue`, `FirstRun.vue`). Re-measure that set yourself before writing.

Two changes in one edit:
- Rewrite the clause so it does not enumerate exclusively - the property is
  that no component this spec mounts reads them, not that only two files do.
  This is the same house rule (`core-docs-name-callers-illustratively-never-exclusively`)
  that edit 4 below exists to satisfy.
- **Split the paragraph into two sentences**, per the verdict's Q7: what the
  installer omits, then why that is safe today plus the unmocked-command
  backstop. The conclusion ("no mount here reaches any of them") must not be
  buried at the end of an eighty-word sentence.

The other two attributions in that clause (the platform global, the file write)
were verified correct and stay.

### 3. F3 - `BUILDING.md`, the frontend checks block

The block lists three commands, so the file enumerates ten while the ruling it
implements says the gate gains an eleventh part and every consumer derives
eleven "per BUILDING.md". `pnpm build` is a gate part - CI runs it - but is
documented only under "Building and running".

Add `pnpm build` to the `### Frontend checks` block with a comment in the shape
of its neighbours, so the file's own enumeration is eleven. Do not add a total
count anywhere: the "(six parts)" Rust heading stays correct, and no gate total
belongs in this file.

### 4. F4 - `crates/muxsmith-core/src/identify.rs`, the module doc

Its closing sentence still carries the exclusive form ("The cache is
constructed per planning call and dropped with it"), which is the same shape
the type doc was just corrected for, three hundred lines below in the same
file. Change "per planning call" to "per call" in that sentence; the rest of
the sentence and the paragraph stay.

**Do NOT touch the `LiveIdentifier.cache` field doc further down.** Its "per
planning call" is TRUE - that struct has exactly one production construction
site, in the pipeline seam - and the verdict says so explicitly.

### 5. F5 - the two plan-7 documents' snapshot annotation

`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md` and
`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md`, both at the
`cli_validate.rs` parenthetical reading "3 snapshots - 5 since amendment 4
added the German case".

Two problems, both in the verdict's F5 with its per-file commit measurements:
the 3-to-5 delta has TWO causes (the bare-raw case and the German case, in
different commits) and the annotation names one; and the enumeration now mixes
a HEAD-refreshed figure with neighbours left at the 2026-07-21 measurement it
declares, without saying so.

Name both additions with their commits, and mark the refresh as scoped so a
reader knows which figures in that enumeration are current and which are the
dated measurement. Verify the two commits yourself rather than transcribing
them. Do not refresh the neighbouring figures - they are a dated record and
stay.

### 6. F6 - `BUILDING.md`, the CI paragraph

It says CI runs "Rust-gate parts 1-5" natively on all three legs and then names
`cargo deny check` as an independent job - but part 5 IS `cargo deny check`.
The matrix job runs parts 1-4. Correct the range. Pre-existing, taken here
because it sits three lines from the block edit 3 touches.

## Standing rules

- **No design latitude**, in either form. A fork returns as NEEDS_CONTEXT with
  a decision memo, never resolved at the keyboard.
- **No behaviour change.** If an edit would alter what any program or command
  does, stop and return it. Edit 3 adds a documented command to a checks list;
  it changes no script.
- Measure, do not transcribe: the settings consumers (edit 2), the two commits
  (edit 5), the module-doc sentence's neighbours (edit 4).
- Counts recomputed from their enumerations; **typography** ASCII hyphens,
  straight quotes, no Unicode ellipsis.

## Verification (foreground, no subsets)

The eleven-part gate as the pass now defines it:
`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
`cargo test --workspace`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items`;
`cargo deny check`; `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`;
`pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e`;
`python3 scripts/ledger-lint.py`.

Baselines, none of which may move: **39** `test result:` lines all ok, **68**
e2e passed, **212** catalog ids, **516** ledger entries. Report what you
measure.

There is nothing to fire here: no edit changes a check's outcome. Say that
plainly rather than manufacturing one.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push. `git -c commit.gpgsign=false`,
stage each file by name (never `git add -A`), exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line. One commit; the message names the findings by tag.

## Report

Append a `## Fix round` section to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/close-pass-report.md`, same
content as your final message: each edit's before and after, the measurements
you took, the pasted eleven-part run, anything you found and did not touch, and
the commit hash with `git show --stat`.
