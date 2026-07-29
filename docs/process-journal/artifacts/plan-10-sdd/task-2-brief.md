# Task 2 implementer brief - Plan 10

**Role:** fresh implementer for Plan 10, Task 2 (W1: the D102 preserved-order
producers, selected by measurement). Model tier: mid (dispatch model: Opus 5).
Effort: xhigh. An independent reviewer grades your work afterwards; the
controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only** (no background-run-plus-monitor).
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.** Task 1 has landed; the tree has moved
  since the plan was authored.
- Shell hazards this project has already hit, both surfaced rather than
  theoretical: a bare `cp` is aliased interactive here and blocks on overwrite,
  leaving a mutated tree behind a hung command; and this shell is **zsh**, where
  `${PIPESTATUS[0]}` is empty (it is bash-only, zsh spells it
  `$pipestatus[1]`). Your task mutates a production file four times and restores
  it four times: take the baseline BEFORE mutating (`sha256sum`), restore with
  `git checkout --` or `command cp -f`, and PROVE each restoration.

## What to read first

1. The plan,
   `/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`:
   the **Global Constraints**, the **Authoring-time verification** section's
   D102 block (the contract's four halves, the three existing guards, and what
   `crates/muxsmith-core/tests/report_json.rs` covers today), **Task 2** in full
   - the Files list, Steps 1 through 6, "Must not decide" - and acceptance rows
   **W1-a through W1-d**, which are the four halves your measurement dispositions.
2. `.superpowers/sdd/plan-10/plan-brief.md`, section 4's **W1** item.
3. `docs/ROADMAP.md`, the **D102 paragraph inside the Plan-9 anchor**, through
   its "RULED 2026-07-29 ... BUILD IT" close. That ruling is why this task
   exists and what it is allowed to conclude.
4. The v1 spec,
   `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, **section 5.2
   "Diagnostics"** - the ordering sentence is the contract itself, and it is
   authoritative above plan and design on conflict.
5. `crates/muxsmith-core/src/report/json.rs`: both builder rustdocs, and the
   four emission sites your four mutations touch.
6. **`crates/muxsmith-cli/tests/dry_run_cli.rs`'s two existing D102 guards.**
   The plan forbids duplicating them and may require you to NAME one as the
   guarding test; a guard cannot be reused rather than duplicated without being
   read.
7. `crates/muxsmith-core/tests/report_json.rs` in full - your only target file.
8. The Tier-2 entry `tests-ship-with-the-feature-never-after` (grep the id in
   `docs/process-conventions.yaml` / `docs/conventions.yaml`).

## Scope

Exactly Task 2's **Files list (EXHAUSTIVE)**: `crates/muxsmith-core/tests/report_json.rs`
and nothing else. `crates/muxsmith-core/src/report/json.rs` is mutated four
times INSIDE Step 2 and must be byte-identical at commit time.
`crates/muxsmith-cli/tests/dry_run_cli.rs` is NOT edited.

The scaffolding (the `KeyRenderer` stub, `mixed_severity()`, `codes()`), the
import list, and the four candidate producers P1 through P4 are fenced in the
plan character for character. Transcribe, do not compose. **Which producers you
write is decided by the measurement, not by you**: a mutation that turns the
suite RED means that half is already guarded - name the failing test and write
NO producer for it; a mutation that leaves the suite GREEN means that half is
unguarded and gets its fenced producer.

**Two forks are pre-routed and you resolve neither:**

- **If `mkvmerge` is absent**, return NEEDS_CONTEXT rather than measuring
  (Step 1). The existing `batch_document` sort guard is gated on
  `have_mkvmerge()` and silently becomes a no-op without it, which would make
  the measurement report a half unguarded for the wrong reason. The authoring
  machine had `mkvmerge v100.0`.
- **If a mutation reveals a real ordering DEFECT rather than a coverage gap**,
  that is a finding, not a test to relax: NEEDS_CONTEXT with the evidence.
- If Step 2 selects NO producer at all, that outcome contradicts the ROADMAP's
  recorded measurement: NEEDS_CONTEXT rather than an empty commit.

## Standing rules

- **No design latitude**, in either form - an explicit permission, or an
  omission (an unenumerated set in a normative position, a name, string or
  fixture value you would have to invent). A fork found on code contact returns
  as **NEEDS_CONTEXT with a decision memo** (options, costs against the named
  invariants, a recommendation), routed by the controller, never resolved at the
  keyboard.
- **Structural-conformance grant** (`latitude-carveout-zero-content-structural-forks`,
  read the entry): following the target file's existing structural patterns is
  in scope where the extension has zero outward effect - additive,
  pattern-conforming extensions of existing tests and fixtures are covered.
  Weakening, deleting, skipping or rewording an existing assertion, mutating an
  existing fixture value, a new test file, and new test infrastructure all stop
  and return.
- **No production code changes.** This task adds tests. Any pre-existing test
  whose behaviour changes at all is a defect signal -> NEEDS_CONTEXT.
- **No task edits any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md`. Ledger-worthy observations go in your report; the
  controller is the single writer.
- **Every observed value in your report is pasted from the run that produced
  it**, never recalled, and never attributed to a command that was not the one
  run. Four mutations means four pasted result lines and every failing test name.
- **Locate code by symbol, never by line number**, in every comment and every
  assertion message you write (`comments-locate-by-symbol-never-by-line-number`,
  owner ruling; the plan states this explicitly for P1's assertion message,
  which names spec section 5.2 by SECTION and never by line).
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.

## Verification bar

1. Step 1's precondition pasted (`mkvmerge --version`).
2. Step 2's four mutations, each applied alone, each followed by a foreground
   `cargo test --workspace`, each restored and the restoration proven. Paste
   every result line and every failing test name. After M4's restore,
   `git status --porcelain` and `git diff --stat` print nothing for
   `crates/muxsmith-core/src/report/json.rs`; the fire for that check is one
   pasted MUTATED state alongside the restored one.
3. Step 4: for each producer that landed, re-apply the mutation whose green
   result selected it, run `cargo test -p muxsmith-core --test report_json`,
   paste the new test's FAILURE, restore, re-run, paste the pass. That is the
   producer's own measured red state.
4. Step 5: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets
   -- -D warnings`; `cargo test --workspace`, foreground, green. Then **the full
   gate as `BUILDING.md` enumerates it** before the commit - `BUILDING.md` is
   the single authoritative enumeration and Task 1 has just added a check to
   one of its parts, so read the file rather than a remembered list.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply here. You commit; you do NOT push (the
single push is a controller close action).

- `git -c commit.gpgsign=false commit ...` - agent commits are deliberately
  unsigned, as policy.
- Stage explicitly by name, **never `git add -A`**.
- The commit command and message are fenced in Step 6.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
  No `Claude-Session` line, no context-window suffix.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-2-report.md`:

- Status: DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT.
- The precondition, pasted.
- **The measurement table**: per mutation M1-M4, the exact edit, the command,
  the pasted result line, the failing test names, and the disposition (half
  already guarded, naming the guarding test; or unguarded, naming the producer
  that follows).
- The producers written, against the plan's fenced text, and why each one and no
  other.
- Step 4's red-then-green evidence per landed producer, pasted.
- The restoration proofs for `report/json.rs`, including the fired mutated
  state.
- Full gate result.
- Divergences and judgment calls, each named.
- Numbered concerns a reviewer can rule on yes/no.
- What you surface for the controller.
- Commit hash and `git show --stat`.
