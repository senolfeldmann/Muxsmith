# Task 4 implementer brief - Plan 10

**Role:** fresh implementer for Plan 10, Task 4 (W5: the user-facing
documentation pass - `README.md` against the shipped CLI surface, the
exact-typed-matching paragraph and its four-item matching-magic list, the two
stale counts in the "How this got built" paragraph, and `docs/INSTALL.md`'s
unsigned-package warning). Model tier: mid (dispatch model: Opus 5). Effort:
xhigh. An independent reviewer grades your work afterwards; the controller
re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools. Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.** Tasks 1 through 3 have landed.
- Shell hazards, both measured in this project: a bare `cp` is aliased
  interactive here and blocks on overwrite; this shell is **zsh**, where
  `${PIPESTATUS[0]}` is empty (bash-only, zsh spells it `$pipestatus[1]`).

## What to read first

1. The plan,
   `/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`:
   the **Global Constraints**; the **Authoring-time verification** section's
   README block (the four premises measured at authoring, each of which you
   RE-measure); **Task 4** in full - Files list, Steps 1 through 8, "Must not
   decide"; acceptance rows **W5-a through W5-f**; and **Amendment 1 and
   Amendment 2** in full, which is where two of your five edit steps come from
   and where their reasoning lives.
2. `.superpowers/sdd/plan-10/plan-brief.md`, section 4's **W5** item.
3. `docs/ROADMAP.md`, three entries in the Pre-1.0 release gates section:
   - the **README entry**, including the owner's 2026-07-29 split and its
     **Content anchors** block (the four-item matching-magic list originates
     there, verbatim);
   - the **OWNER QA PASS, round 1** entry, which carries the Fedora warning
     string verbatim and the ruling that the disposition is to DOCUMENT it -
     Step 5 orders the written note checked against THAT entry, so the task
     cannot be executed without it;
   - the **"Artifact signing: firm 1.x"** entry in the v1.x candidates section,
     the source of Step 5's claim that signing itself is deferred.
4. `README.md` in full and `docs/INSTALL.md` in full - every claim you correct
   is DERIVED from their current text.
5. The v1 spec's **sections 4 and 5** (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`),
   and the code half of the same verification:
   `crates/muxsmith-core/src/matcher.rs` (`exact_matches`, `lang_eq`,
   `canonical_tag`), `crates/muxsmith-core/src/capability/runtime.rs` and
   `capability/mod.rs` (`LanguageIndex::normalize`, `matchable_type`,
   `matchable_domain`), `crates/muxsmith-core/src/profile/validate.rs`
   (`validate_expr`).
6. The three exit-code sources: `crates/muxsmith-cli/src/cli.rs` (`Cli`'s doc
   comment), `crates/muxsmith-cli/src/commands/mod.rs` (`severity_exit`),
   `crates/muxsmith-cli/src/commands/run.rs` (`job_exit_code`). Two of the three
   do NOT live in `cli.rs`; name each with its file.
7. `crates/muxsmith-cli/tests/run_live.rs`, whose inlined copy of the README
   passthrough recipe is what makes Step 7's green gate evidence that you did
   not touch that recipe.
8. Tier-2 `owner-manual-qa-gates-the-1-0-release` (why a QA finding is
   documented rather than fixed pre-1.0), `proc-wrapped-prose-quote-grep`
   (Step 5(b)'s hard-wrapped comment), and
   `latitude-carveout-zero-content-structural-forks` (the two named-region
   repairs).

## Scope

**Files (EXHAUSTIVE):** `README.md`; `docs/INSTALL.md` - and in the latter only
the two named regions of Step 5 (the Linux section's note, and the file-top HTML
comment whose enumeration that note falsifies). Nothing else in either file
beyond what the steps name.

**What stays untouched, deliberately** (Step 6): the four `placeholder(1.0)`
comments and the work-in-progress banner. The owner ruled they may remain;
filling one is out of scope, not helpful. The passthrough recipe's YAML is not
edited.

## The three things this task is really made of

1. **Re-derive the CLI surface from the BINARY, not from the source** (Step 1),
   and build a divergence table at FLAG granularity - value names, defaults,
   possible-value sets - which goes into the report as the record. Every
   correction the plan lists was measured at authoring and must be RE-measured
   before it is written; **a correction the re-measurement does not reproduce is
   a finding, not a silent drop.** Exit codes are not in the help text: derive
   them from the three named sources.
2. **Correct the exact-typed-matching paragraph and write in the four-item
   anchor** (Step 3), verifying each of the four against the spec AND the code
   before writing it, and naming the spec section and the core symbol per item
   in the report. The anchor is a paraphrase of an owner remark, not a
   specification: **a claim the code does not support is reported, not written.**
3. **Measure the two counts rather than transcribing them** (Step 4), because
   the tree moves and a freshly wrong number is worse than a stale one - it
   reads as checked. Both commands are fenced in the plan. Two traps, both
   named there and both real:
   - the decision series is NOT contiguous, so a range endpoint and a count are
     different claims, and "D1 through D105" would assert 105 decisions where
     there are 103;
   - the verdict count's units FORKED. `78` was correct when written; the
     basename rule now returns a much larger number while the
     `verdicts/`-directory rule still returns 78, frozen. **A re-measurement
     under the frozen unit reproduces the README's current figure and looks like
     confirmation.** Paste both, so the fork is visible to your reviewer rather
     than taken on trust.

## Step 5, the amendment-2 note, in the exact scope the plan allows

- Verify the warning string and its source **against the ROADMAP's own record**
  before writing it, never from a paraphrase: a near-miss quotation is worse
  than none, because a reader who meets the warning greps for the exact string.
- The tool is **`dnf`, not the `rpm` binary**. `@commandline` is dnf's
  pseudo-repository for a package given as a path. The ROADMAP's phrase "during
  the rpm install" means the rpm PACKAGE.
- Scope: the note documents the **dnf case only**, which is what was observed.
  It makes NO claim about what `apt install ./...deb` prints, because nobody
  measured that.
- Register: match the macOS Gatekeeper detour's shape, do not invent one. The
  note lands beside the Linux section's existing "No gatekeeping dialog exists
  on Linux" sentence, which stays true - a warning line is not a gatekeeping
  dialog, and the note says so rather than contradicting it.
- (b) The file-top HTML comment is **hard-wrapped across three lines, so it does
  not grep as one string**: locate it by `code signing lands` on its own line.
  It enumerates the two sections that shrink when signing lands; your note is a
  third, so the enumeration is extended.

## Standing rules

- **No design latitude**, in either form. What keeps the prose work out of the
  latitude ban is that the SET of facts is closed and fixed by the plan: you
  decide nothing, you write the sentences that carry them. A fork found on
  contact returns as **NEEDS_CONTEXT with a decision memo**.
- **Register:** the README is written in the owner's sell-tone, a case-scoped
  exception to the house writeup voice, recorded on its ROADMAP entry. That
  entry is the whole basis for the register; `latitude-carveout-presentation-tokens`
  is NOT invoked here.
- **No task edits any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md`. Ledger-worthy observations go in your report.
- **Every observed value in your report is pasted from the run that produced
  it.** Help output is pasted, not summarized.
- **Locate code by symbol, never by line number**, in anything you write.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis. Note that
  Task 5 will sweep line-number citations out of source comments next, and three
  of its sites cite README spans - so do not introduce a new `file:line`
  citation anywhere.
- A claim about the tree is a COUNT: measure before writing "the only", "every",
  "no X exists".

## Verification bar

1. The pasted help outputs and the divergence table (Step 1).
2. Per-claim verdicts with named spec sections and core symbols (Steps 2-3).
3. Both count measurements with their commands, plus the verdict count's
   boundary checks AND the frozen-unit counter-measurement (Step 4).
4. The written INSTALL.md note grepped against the ROADMAP entry's verbatim
   string (Step 5).
5. **The full gate as `BUILDING.md` enumerates it**, foreground, green, before
   the commit. No test asserts README prose, so the gate's role here is narrow
   and stated as such: `crates/muxsmith-cli/tests/run_live.rs` inlines the
   passthrough recipe verbatim and asserts it still runs, so the green run is
   the evidence you did not touch it.
6. `git diff --stat` covering exactly the two files.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply here. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...`, agent commits deliberately unsigned.
- Stage explicitly by name, **never `git add -A`**.
- The commit command and message are fenced in Step 8.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-4-report.md`:

- Status: DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT.
- The full pasted help output and the divergence table (README says / binary
  says / verdict), at flag granularity.
- Per-claim verification of the matching paragraph and the four anchor items,
  each naming its spec section and core symbol.
- Both counts: commands, outputs, boundary checks, the frozen-unit
  counter-measurement, and the sentence as you wrote it.
- The INSTALL.md note as written, plus the grep proving its string matches the
  ROADMAP's record, plus the extended file-top enumeration.
- Anything the plan listed as a correction that your re-measurement did NOT
  reproduce, named as a finding.
- Full gate result; `git diff --stat`.
- Divergences and judgment calls, each named.
- Numbered concerns a reviewer can rule on yes/no.
- What you surface for the controller.
- Commit hash and `git show --stat`.
