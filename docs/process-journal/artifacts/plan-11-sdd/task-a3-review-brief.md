# Task A3 review brief - Plan 11

**Role:** independent reviewer of Plan 11, Task A3 - the plan's one BEHAVIOUR
change: `raw:` compares without type conversion. One new private comparator, one
re-pointed call site, twelve repair sites across six files in two natural
languages, three tests. You did not write this change. Model tier: mid (dispatch
model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content in short form as your final message.

## What is at stake here that was not at stake in A1 and A2

Those tasks changed prose. This one changes what the matcher DOES, and it ships
into a spec, a README and two user-facing help topics in two languages. The
single most expensive mistake available is the inverse of the change: **stripping
`scalar_eq`'s int/float cross arms instead of leaving the no-conversion rule to
the new `scalar_eq_same_type`.** Those arms are correct, documented behaviour of
the TYPED `exact` path, and mkvmerge's five float-declared properties need them.
Test T-1 exists solely to catch that; the implementer reports measuring that
stripping them fails exactly one test of 124, and that reverting the call site
fails exactly two - disjoint mutations. **Reproduce both mutations yourself.**

## Preamble (binding)

- Never call session-relocation tools. Absolute paths, **foreground runs only**.
- **The work sits in a worktree:** `/home/senol/Git/muxsmith-plan11-a`, branch
  `plan-11-stream-a`, head `164e571` over `5d305a2`. The stream's base is
  `5378264`. Do not touch `/home/senol/Git/muxsmith-plan11-b`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a3rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never use a
  shared default path. **Your mutation experiments go on a COPY of the crate, not
  in the worktree** - a mutation left behind in a live stream would be merged.
- If you must mutate a tracked file in place, capture the CURRENT content as the
  baseline (`sha256sum` plus a scratch copy) and restore to it; `git checkout --`
  restores from HEAD and is only safe here because the work IS committed. A bare
  `cp` is aliased interactive; use `command cp -f`. Prove every restore.
- The tree must be byte-identical to `164e571` when you finish. Prove it.
- **The plan document on `master` has since gained Amendment 5**, which touches
  Tasks A1 and B1 only - nothing in A3's territory or D111's. Your ground truth
  is the worktree copy plus D111.

## Ground truth, in precedence order

1. **ADR D111**, `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`
   - the semantics table (3.1), the comparator pair (3.2), the call site (3.3),
   what does not change (3.4), the twelve repair sites with their exact
   replacements (4.3), the seven retained sites and two different-claim sites
   (4.4), the checks (4.6), the three tests (5), the diagnostics decisions (6-8),
   parity (10). **D111's fences are the SINGLE SOURCE of the twelve replacement
   strings** - the plan deliberately does not duplicate them, so your
   character-for-character comparison is against D111 itself.
2. The v1 spec, which outranks the plan on conflict.
3. The plan in the worktree: Global Constraints, Task A3 in full, acceptance rows
   **W3-a through W3-m**.
4. `.superpowers/sdd/plan-11/plan-brief.md` item 3; the four house-knowledge YAML
   files, cited by id.

The implementer's brief (`task-a3-brief.md`) and report (`task-a3-report.md`) are
**evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/review-5d305a2..164e571.diff`

## Dimensions

1. **The comparator pair and the call site, character for character against D111
   sections 3.2 and 3.3.** Then the property that matters more than the
   transcription: `scalar_eq` keeps BOTH cross arms, the `raw:` branch calls
   `scalar_eq_same_type`, and **the two non-`raw:` call sites still call
   `scalar_eq`**. Establish that call-site split yourself.
2. **Reconstruct, do not inspect** - the method both prior reviews used and this
   implementer also ran: rebuild each of the six end-state files from its
   `5d305a2` blob plus only the D111/plan fences, and compare byte for byte. Its
   precondition is that each fenced OLD block occurs exactly once in its file;
   **check that per site**, because `matcher.rs` and the v1 spec are in BOTH the
   repair set and the retained set, so a file-level identity check on either is
   impossible by construction.
3. **The retained set, per site rather than per file.** The seven retained lines
   of D111 section 4.4 must be byte-identical across the change while their files
   legitimately move. The v1 spec's second occurrence in section 9.2 and
   `matcher.rs`'s two scoped sentences are the ones to check hardest.
4. **The three tests: do they test what they claim?** Run the two mutations
   yourself on a crate copy - strip the cross arms from `scalar_eq`, and revert
   the call site to `scalar_eq` - and record exactly which tests fail in each. A
   test that stays green under the mutation it exists to catch is a Critical
   finding. Also check T-3's four-by-four matrix actually enumerates the cases
   the absolute quantifies over.
5. **The behaviour change end to end, through the shipped binary** (SI-3,
   `testing-si3-run-binary`). The implementer reports both cross directions
   flipping from match to `missing-track` while both same-kind runs still match.
   **Rebuild before you probe** - none of the task's exit bars rebuilds
   `target/debug/muxsmith`, so an unrebuilt binary shows the OLD behaviour while
   appearing to confirm the new one. Verify with
   `find crates src-tauri -name '*.rs' -newer target/debug/muxsmith` returning
   nothing.
6. **The four retirement checks, re-run with your own instruments:** R' (two
   invocations, summed) with its soundness control; K' with its own fire; the
   alternation-free vocabulary sweep, whose 71 hits the implementer classified
   line by line - **spot-check the classification rather than accepting the
   tally, and look hardest at the 52 it called noise**; and R'' over the edited
   product files.
7. **The German half is not a translation of the English half.** R-9 reuses the
   German topic's own established vocabulary. Read both help topics and judge
   whether they now say the same thing about the same mechanism, since
   `pnpm check:i18n` checks completeness and never agreement.
8. **Diff scope.** Exactly six files. `locales/` unchanged (no Fluent value
   moved), `profile/validate.rs` and `tests/validate_semantics.rs` unchanged.
   No `DiagCode` added, widened or re-severitied; no Fluent key added; the
   profile schema, `Scalar`, `PropValue` and `scalar_fits` untouched.
9. **Latitude, both forms**, including the inverse. **The no-work-needed check**:
   D111 and the report both conclude several times that a test, a snapshot update
   or a validate-side change is unnecessary - run each premise rather than
   weighing it.
10. **House dimension** by id, in particular `core-72-exact-typed-value-equality`,
    `tests-ship-with-the-feature-never-after`, `proc-proposed-safeguard-stays`,
    `proc-06-mkvtoolnix-parity`.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **Test PLACEMENT is the one thing D111 does not fence.** The three test bodies
   are verbatim, but where each lands in the `tests` module is written nowhere.
   The implementer placed T-2 in b7's position (mandated), T-1 as the last
   typed-path test before the B-5..B-8 section comment, and T-3 closing the
   `raw:` group after b8 - and surfaced the gap rather than deciding silently.
   **Is that unenumerated placement a latitude-by-omission defect in D111, and is
   the chosen placement right?**
2. **R'' was run over SIX files where the plan says five.** The implementer widened
   it and reports the v1 spec's two loose candidates are exactly the two D111
   already measured as permitted. **Was widening the surface correct, or does a
   check run over a different set than specified need routing before it runs?**
3. **A drifted citation inside D111.** The R' soundness control's ninth hit sits
   at a different ROADMAP line than D111 prints; D111 anticipated this and cites
   by wording, which is byte-identical. **Does the control still discriminate,
   and is citing-by-wording sufficient here?**
4. **The corpus discriminator is a grep where it should be a parse.** The
   implementer reports that a line-shaped `pattern:` test misreports the README's
   passthrough example, whose `input:` is an inline flow mapping, as a second
   defective profile. It affects the owner-scheduled example-validation vehicle
   rather than this task. **Is the corpus derivation this task actually performed
   sound despite that, and what exactly does the finding oblige the close to
   record?**
5. **`tests-ship-with-the-feature-never-after`, in the direction that is easy to
   miss.** Three tests ship, which inverts this task's pre-amendment position that
   none was owed. **Is the shipped test set sufficient for the behaviour this
   task introduces** - both cross directions, both same-kind counterparts, the
   typed path's preserved behaviour - **or is there a user-visible consequence of
   this change with no producing test?** Walk the observable's halves, not the
   observable.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a3-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings, each with `file:line`, the evidence you ran,
  and the exact required change.
- The five adjudications, one explicit verdict each.
- An evidence appendix naming your instrument paths and commands.
- A **HARVEST** section, including what Task A4 and the plan close must carry.

Your final message carries the short form only.
