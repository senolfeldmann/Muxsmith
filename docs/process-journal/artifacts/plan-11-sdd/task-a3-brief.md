# Task A3 implementer brief - Plan 11, stream A

**Role:** fresh implementer for Plan 11, Task A3 (W3: `raw:` compares without type
conversion - one comparator pair, one re-pointed call site, twelve repair sites
across six files in two natural languages, three tests). Model tier: mid
(dispatch model: Opus 5). Effort: xhigh. An independent reviewer grades your work
afterwards; the controller re-runs your claims.

**This is the one BEHAVIOUR change in Plan 11.** It was a documentation task when
the plan was approved; the owner ruled on 2026-07-30 that no type casting happens
under `raw:`, and ADR **D111** settles the semantics, every replacement string and
the test set.

You are the THIRD task of stream A. A1 (`a0d5d3e`) and A2 (`5d305a2`) are
committed and reviewed; A4 follows you. A separate stream B is committed in a
DIFFERENT worktree and never touches your files.

## The one thing that must not get backwards

**`scalar_eq`'s two int/float cross arms STAY.** They are correct and documented
behaviour of the TYPED `exact` path - the spec says so, the README says so, and
mkvmerge's five float-declared properties need them, since it reports an integral
`max_luminance` as `400.0`. What changes is the `raw:` CALL SITE, which re-points
to the new same-kind comparator. A change that strips the cross arms from
`scalar_eq` instead of leaving the no-conversion rule to `scalar_eq_same_type`
would pass almost every other check in this task; test **T-1** exists precisely to
catch it, and the A3 reviewer measured that none of the other eight checks would.
Read D111 section 3.2 and 3.4 before you touch the comparator.

## Preamble (binding)

- **Work in `/home/senol/Git/muxsmith-plan11-a`** (branch `plan-11-stream-a`,
  head `5d305a2`). Never on `master`, never in the main worktree, never in
  `/home/senol/Git/muxsmith-plan11-b`. Absolute paths throughout.
- **Never call session-relocation tools.** Do not run `git worktree`.
- **Foreground runs only.** Your exit bars include a full `cargo test --workspace`
  and a `cargo doc`; run them in the foreground however long they take.
- You are the only writer in your worktree while you run.
- **The plan document is being amended concurrently on `master` (Amendment 5)**
  for defects in Tasks A1 and B1. **Nothing in Task A3 or in D111 is in that
  amendment's scope.** Your contract is the plan copy in YOUR worktree plus
  D111; if you find something that looks inconsistent, report it rather than
  chasing the amendment.
- If a step of yours mutates a tracked file to fire a check, capture the file's
  CURRENT content as the baseline (`sha256sum` plus a copy at a scratch path) and
  restore to that - **`git checkout -- <file>` restores from HEAD and would
  discard your uncommitted work**, which is the trap that hit Task A1. A bare
  `cp` is aliased interactive here; use `command cp -f`. Check K's fire is
  exactly such a mutation.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash - **except** where a fenced replacement or the German help topic
  legitimately carries German orthography, which is copied exactly as fenced.

## What to read first

1. **ADR D111 in full**,
   `/home/senol/Git/muxsmith-plan11-a/docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`.
   It is ground truth for this task and is not re-decided. In particular
   sections 3.1 (the sixteen-pair semantics table), 3.2 (the comparator pair,
   fenced and rustfmt-verified), 3.3 (the call site), 3.4 (what does not change),
   4.3 (the twelve repair sites with their exact replacements), 4.4 (the seven
   retained sites and the two different-claim sites), 4.6 (checks R', K', the
   vocabulary sweep and R''), 5 (the three tests and the task exit bars), 6 to 8
   (the diagnostics decisions), 10 (parity), 13.5 (this task's clause list).
2. The plan, in your worktree: the **Global Constraints** section, **Task A3** in
   full (Steps 1 through 10, the D111-is-the-single-source paragraph at its end,
   and "Must not decide"), and acceptance rows **W3-a through W3-m**.
3. `.superpowers/sdd/plan-11/plan-brief.md`, item 3.
4. The code and documents Task A3's own Read-first list names - `matcher.rs`,
   `profile/validate.rs`'s `raw_opt_in_diagnostic`, `report/mod.rs`,
   `capability/mod.rs`'s `matchable_type` and `capability/generated.rs`, the v1
   spec's sections 4.3, 4.4, 7 and 9.2, `README.md`'s matching-magic section,
   both `editor-match-expr-exact.md` help topics in full.
5. Tier-2 `core-72-exact-typed-value-equality`, `testing-si3-run-binary`,
   `proc-06-mkvtoolnix-parity`, `tests-ship-with-the-feature-never-after`,
   `proc-proposed-safeguard-stays`.

**The twelve replacement strings live in D111 section 4.3 and NOWHERE ELSE.**
This brief deliberately does not reproduce them, and neither does the plan. Each
site names its own `R-n` and each `R-n` has its own heading in that section, so
the lookup is deterministic. The reason is an asymmetry in failure modes: nothing
in this task compares your applied text against D111, so a drifted duplicate
would leave every check green while a wrong sentence ships into the v1 spec, the
README and two user-facing help topics. Transcribe from D111 itself, per site.

## Carried from the A1 and A2 reviews, because it binds you hardest

- **Reconstruct, do not inspect.** Both reviewers verified scope by rebuilding the
  end state from the base blobs plus the fenced substitutions and comparing byte
  for byte. That is the strongest instrument available to you too - but the A2
  reviewer named its precondition: **it only works where the fenced OLD block
  occurs exactly once in its file.** With twelve sites across six files, check
  that per site before relying on it, and note that `matcher.rs` and the v1 spec
  are in BOTH the repair set and the retained set, so a file-level byte-identity
  check on either is impossible by construction.
- **The retained set needs the INVERSE check per site**, not a file-level one:
  the seven retained lines must be byte-identical across the change while their
  files legitimately move.
- **An enumeration is a claim.** Where you report a classified set - the
  vocabulary sweep's 71 lines with 19 classified sites is exactly this - paste
  the command and its FULL output, then classify line by line. Do not describe
  the remainder in prose.
- **A green `ledger-lint` says nothing about your deliverable.** Do not cite it
  as coverage.

## Exit bar before you commit

Step 8 in full, nine checks, every output pasted - check R' (two invocations,
summed, with its soundness control), check K' with its fire and restore proof,
the alternation-free vocabulary sweep with every hit classified, check R'' over
the five edited product files, the six task exit bars including
`cargo test --workspace` and the rustdoc run that validates R-10's intra-doc
links, the README example in both directions, the corpus derivation with its
three blind-spot probes, the diff-scope checks, and the weighed test duty.

Step 1's four binary probes need `cargo build -p muxsmith-cli` FIRST on the post
state - none of the exit bars rebuilds `target/debug/muxsmith`, so without the
rebuild the probes would run the pre-change binary and appear to confirm the new
behaviour while showing the old one.

You do **not** run the full eleven-part gate - the stream runs it once before
merge, and that is the controller's dispatch.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner; you do not ask. Agent commits are
deliberately UNSIGNED. Use exactly the fenced commands in Step 10, and the
trailer

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

exactly one trailer, no `Claude-Session` line, no context-window suffix. Stage
explicitly; never `git add -A`. Do not push.

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a3-report.md`
(the MAIN repo path, not your worktree). Every command with its pasted output,
the Step-9 surfacing list including D111's twelve triggers, your commit SHA, and
anything you noticed but did not touch.

Return to the controller only: status
(`DONE` / `DONE_WITH_CONCERNS` / `NEEDS_CONTEXT` / `BLOCKED`), the commit SHA, a
one-line verification summary, and concerns. Not the report body.
