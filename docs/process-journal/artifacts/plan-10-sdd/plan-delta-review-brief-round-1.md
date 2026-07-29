# Plan 10 delta review brief (after fix round 1)

You are the round-1 reviewer, resumed. Same judge, same standards. You are
grading a DELTA, not re-running the review: your round-1 non-findings stay
settled and are not re-litigated.

**Delta under review:** `da60634` (558 lines, what you graded) ->
`afaf9a0` (619 lines). `git diff da60634 afaf9a0 -- docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`
is the whole change; nothing else in the repo moved except house-knowledge
files, which no task touches.

All fourteen of your findings were routed FIX. None was disputed. The author
refuted nothing and reports F1 and F2 reproduced for them as well.

## What you are judging

1. **Does each fix actually close its finding?** Fourteen of them. A fix that
   restates the problem in better prose without removing it is a finding.
2. **The two controller decisions that removed choices your verdict left open**,
   for correctness rather than for authority: F3 was ruled EXTEND (widen the
   remaining self-descriptions rather than fence them off), and F12 was ruled
   name the spec SECTION rather than the line.
3. **One form deviation the author flags and invites you to reject.** On F6 the
   `latitude-carveout-presentation-tokens` licence is gone, but the id survives
   as an explicitly REJECTED reading carrying its boundary sentence, on the house
   pattern of keeping a losing argument so it is not rediscovered. Judge whether
   that is the right call or whether the id should be absent entirely.
4. **Three things the fix forced that your verdict did not anticipate.** These
   are new material, so they get full scrutiny rather than delta scrutiny:
   - F11 became a second prescribed corpus expression rather than a note, after
     the author's own first control reported a false clean tree - a one-pipeline
     form matched its own `git grep -n` `file:line:` prefix and filtered every
     line away. The corpus is now claimed at **24 lines across 16 files**.
     Re-measure it with your own instrument, and check that the prescribed
     per-file form is actually immune to the failure it was written for.
   - A fifth fire, F5, for the continuation guard, on the principle that an
     anchor with no measured red state is a defect.
   - The acceptance map grew to **18 rows**, with W2 split across two
     expressions and W2's old row renamed W2-c. Re-walk the map for coverage the
     way you did in round 1; a renamed row is where a silent drop hides.
5. **The replacement self-audit in F2's fix.** It claims a fire-verified search
   reporting **nine** sites, one of which is the audit sentence itself because it
   quotes its own expression. Re-run it. This is the finding whose whole subject
   was a negative check that could not fire, so its replacement carries a higher
   bar than the others: if the new search cannot fire either, the fix failed in
   the same way the original did.
6. **The two controller additions**, which were not in your verdict: the three
   bare line-span citations folded into Task 5 (corpus, Files list, acceptance
   map, absence check, `git add` set - check set-equality again, the count moved),
   and the `prHourlyLimit` trap written into Task 3 Step 2.

## Standards that carry over

Verify, never believe. Build your probes at a scratch path you name in this
pass; do not re-run the author's instruments, and do not re-run your own round-1
scratch files without re-deriving them - the tree has moved. Every negative
result fire-verified against a known-present case, and say so. Where a passage
concludes something is unnecessary or already covered, run the premise.

## Output

Append to `.superpowers/sdd/plan-10/plan-review-round-1.md` a section
`## Delta review after fix round 1`, or write
`.superpowers/sdd/plan-10/plan-delta-review-round-1.md` - your choice, state
which in your final message.

- Verdict: `APPROVED` or `NEEDS FIXES`.
- Per round-1 finding: closed / not closed / closed with a new problem.
- Findings on the new material, by severity.
- A short `## HARVEST` for anything new; do not repeat round 1's harvest.

Final message: the verdict word, at most three lines, and the file path.

Read-only on the tree except your verdict file. No git write commands;
read-only git is expected here, since the delta is a diff.
