# Recon brief: re-validate Plan 12 against the current tree

**Role:** recon / verification only. You author NO product artifact. You do not
edit the plan, the spec, any source file, catalog, test or document. Your entire
deliverable is a report.

**Why this exists.** Plan 12 was authored and every figure in it measured against
the source tree at `148f19f`. Plans 11 and 11.5 have landed since. Both Plan 11
and Plan 12 amend the same v1 spec document (Plan 11 in sections 4.3, 4.4, 7, 8.1
and 9.2; Plan 12 in 8.2). Different sections means no textual conflict is
EXPECTED. Expecting is not measuring, and this package's own history supplies the
sharper reason: Plan 11 carried a fenced placement instruction that was correct
when written and became unperformable against the state a later step actually
held, and two reviewers had to fire both readings to establish it.

## Ground truth

- The plan: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`
- Authoring baseline: `148f19f`. Current `HEAD`: derive it yourself, do not
  assume the value another document states.
- The delta to examine: `git diff --name-only 148f19f..HEAD`.

## What you verify

Two classes. **Derive the members of each class FROM THE PLAN DOCUMENT, never
from a list someone hands you and never from your reading memory of it.** State
the expression you used to derive each set, and state that expression's blind
spot. Where one expression's shape cannot see a member class, run a second
expression aimed exactly there - the plan's own string-surface derivation (E1/E2
in its authoring section) is the house pattern for this and is worth reading
before you build yours.

### Class A: fenced OLD strings

Every block in the plan that prescribes replacing existing text - a fenced OLD
region to be located and replaced, a "replace this exact text" instruction, a
quoted current sentence a task is told to amend - in ANY target file: the v1
spec, the README, a help topic, a catalog, a source comment, a test.

Per member, verify at current HEAD:

1. The string still occurs in its named target file.
2. It occurs **exactly once**.
3. Where the plan states a surrounding anchor or a neighbouring line, that
   context still holds.

A member that now occurs zero times or more than once is a BLOCKER finding: the
task carrying it cannot execute as written.

**Method note that decides whether your result means anything:** search with the
characters copied out of the plan document, not with a retyped equivalent. A
retyped pattern silently repairs an escape, a wrapped line or a doubled space
that the document actually contains, and then reports a match the implementer
will not get. Where a fenced block spans lines, say how you handled the line
breaks.

### Class B: tree-measured figures

Every figure in the plan that was measured against the tree rather than reasoned
about. These live densely in the `Authoring-time verification` section but are
NOT confined to it - task steps, the decision register and the acceptance map
carry measured counts, greps, symbol enumerations and pasted outputs too. Your
derivation expression must reach those.

Per member: re-run the stated expression at current HEAD, paste the actual
output, and compare to the authoring value.

Notes on specific known-heavy members, so you neither skip nor over-build them:

- **The validator seeds (S1-S5).** Re-run through the CLI path
  (`./target/debug/muxsmith validate <seed> --json`, exit code captured per
  seed); build the debug binary first if it is absent or stale. `matcher.rs` and
  `report/mod.rs` both changed in the delta, so this is one of the few figures
  with a plausible mechanism for moving. Re-build the plan's out-of-repo
  model-path probe ONLY if the CLI figures moved or if a CLI figure is
  ambiguous; if they reproduce, say so and state that you did not re-run the
  second instrument and why.
- **The mkvtoolnix parity findings.** The reference source at
  `~/Downloads/mkvtoolnix` is outside this repo and did not change in the delta.
  Confirm that (a version/mtime check is enough) rather than re-running all three
  source greps; if it DID move, re-run them.
- **Figures over `docs/ROADMAP.md`, `docs/decision-ledger.yaml` and
  `docs/process-conventions.yaml`.** These three DID change in the delta. Every
  figure resting on them - the free-D-number sweep for D106-D110, any cited
  Tier-2 entry statement, any count over the ledger - is re-run, not assumed.
- **Counts of catalog ids and registry entries.** `locales/` and `src/` did not
  change in the delta, so these should reproduce exactly. Re-run them anyway and
  say they reproduced; a figure claimed unchanged without a run is not a
  measurement.

## Absence-shaped checks

Several plan figures are absences (a grep returning nothing, an exit 1). For each
of those you re-run, the empty result alone is not evidence: fire the same
expression against a known-present case, or against the plan's own stated
synthetic fire, and report both runs. An empty result and a broken pattern look
identical.

Where an expression contains an **enumerated set** - file extensions, path
globs, keywords, symbol alternations - that enumeration is itself a claim the
fire test does not cover: firing against one present member passes while a
missing member stays invisible. Note any such enumeration you re-run and whether
its membership is still right against the current tree.

## What you do NOT do

- No fix, no edit, no "while I was in there".
- No judgement about whether a task should change. You report the measurement;
  routing is the controller's.
- No relocation tools (EnterWorktree/ExitWorktree or equivalent). Work in the
  main tree at `/home/senol/Git/Muxsmith`, absolute paths, foreground runs only.
- Do not commit anything.

## Report format

Write your report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-12/pre-execution-revalidation-report.md`
and return a short summary in your final message.

The report carries:

1. **HEAD you measured at**, and the delta file list you worked from.
2. **Your derivation method** for each class, with the expression, its blind
   spot, and the second expression that covers the blind spot.
3. **Class A table:** member, target file, occurrences found (a number), verdict
   (`unchanged` / `BLOCKER: n occurrences` / `BLOCKER: absent`).
4. **Class B table:** figure, expression as run, authoring value, current value,
   verdict (`reproduces` / `MOVED: <new value>`).
5. **Blockers**, listed separately and first if any exist.
6. **Anything you could not verify**, named as such with the reason. An honest
   gap is worth more than a green row that stands for a run nobody made.

Paste real output for every load-bearing figure. Never recall a value, and never
attribute a value to a command other than the one you ran.
