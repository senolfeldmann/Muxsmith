# Task 6 review brief

You are the independent reviewer of Task 6 of Plan 12, the largest task in the plan:
nine files, Rust shell work, a locale table, a three-way parity test, six catalog
strings in two locales, and a new IPC command. Two verdicts are required and neither
is optional: **spec compliance** against the task's requirements, and **task
quality** as graded findings.

## The three artifacts

- **Requirements:** `.superpowers/sdd/plan-12/task-6-brief.md` - the task's full
  text, with the exact values the implementer was told to use verbatim.
- **The implementer's report:** `.superpowers/sdd/plan-12/task-6-report.md`.
- **The diff:** `.superpowers/sdd/plan-12/review-ed1a635..a47fc19.diff`.

Your ground truth is the brief and the spec, never the code. Where the code and the
brief disagree, the brief wins unless you can refute the brief with evidence, which
is a valid and wanted finding.

## Where to aim, in order of what it costs if wrong

**1. The parity test's three parts, and specifically that each assertion sits BELOW
the locale fallback chain.** This is the whole reason the test is split three ways.
A red state applied upstream of a fallback never reaches an assertion made downstream
of it: the chain supplies a plausible, non-empty, non-key value for every mutation,
so the check passes in exactly the state it exists to forbid. Verify per part that
the prescribed red state defeats the mechanism rather than disturbing its input, and
run each one.

**2. The derived key set.** Part (b) derives the shell's consumed keys by matching
literals in the shell source rather than from a hand-written list, which is the right
shape - and it makes the DERIVATION the thing to attack. Two known facts about it:
a non-literal call site exists in that file's own test module (adjudicated already,
consequence nil, the implementer was told so), and the implementer reports rebinding
two probe keys in a pre-existing test from literals to locals so they would not enter
the set. Derive the set yourself, independently, and compare.

**3. The four-state decision matrix and the twelve-cell re-confirmation matrix.**
These are pure functions with exhaustive tables, which makes them cheap to attack and
expensive to get wrong: one wrong cell is a prompt that does not appear over unsaved
changes. Mutate cells and check the tables catch it.

**4. German.** Six new values in the German catalog, and a header note in that file
which this task consumes rather than extends. Check the German for orthography and
for saying what the English says, and check that the consumed note now describes the
truth.

## Dimensions to run

**Spec compliance**, requirement by requirement, each MET or NOT MET with evidence.
Byte-check every fenced value: the Rust blocks, the twelve catalog lines across both
locales, the allowlist edit, the commit block.

**A step that names several sites is checked AT EACH SITE.** Not general advice: Task
4 of this plan failed exactly here, and both its task review and its delta re-review
graded compliance MET because they asked whether the step had been performed rather
than checking each site it names. This task's steps name catalogs in two locales, an
allowlist, and several call sites.
(House entry: `a-normative-sentence-naming-a-set-is-discharged-member-by-member`.)

**Test-power, and DIRECTION rather than presence.** A mutation can discriminate
perfectly and still prove only that a mechanism exists. Where a behaviour has a
dangerous direction and a safe one - prompt versus no prompt, cancel versus confirm,
abort versus quit - the mutation that earns the claim leaves the mechanism working
and inverts it. Prefer inverting mutations to removing ones, and say which kind each
of your runs is.
(House entry: `a-fire-test-on-a-two-direction-surface-attacks-the-direction-not-the-presence`.)

**Latitude, in both forms**, over anything the diff introduces that the brief did not
write out: an invented name, string, key, constant, selector or file is a finding,
and so is a mandated set the brief never enumerated that the implementer closed by
choosing.

**House conformance.** The four house-knowledge files are review ground truth
alongside the brief; cite entries by id. Nearest this diff:
`comments-locate-by-symbol-never-by-line-number`,
`a-document-never-cites-a-line-number-inside-itself`,
`a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact`,
`proc-normative-count-recomputed`, `proc-proposed-safeguard-stays`,
`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`,
`a-normative-claim-is-scoped-down-to-its-producers-reach`,
`gitignored-paths-need-command-grep`, `bash-isms-run-under-bash`,
`frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`,
`tests-ship-with-the-feature-never-after`, `design-empirical-claims-reproducible`,
plus the CLI locale-table pattern the brief names as this code's house precedent.

**The no-work-needed check, standing.** Wherever the diff, a comment or the report
concludes that a guard, an enumeration or a check is unnecessary - "so no X is
needed", "that cannot happen", "the existing test covers it" - **verify the claim
that makes it unnecessary. Run it; do not weigh it.**

**Typography.** ASCII hyphens, straight quotes, no Unicode ellipsis, no em-dash or
en-dash in comments, strings and test names. German orthography inside German catalog
values is orthography and is copied exactly - do not "fix" umlauts or eszett.

## Adjudication questions - one required verdict each, phrased open

The implementer raised three concerns, and two of them changed pre-existing test
content. None is pre-rated.

**Q1.** Two probe keys in a pre-existing shell test were rebound from literal
arguments to local variables, on the ground that as literals they would enter the
parity test's derived key set and make it fail even in the correct state, since no
catalog can carry a key named for a missing key. Is that reasoning correct at the
code? Is rebinding the right repair, or does it weaken what that pre-existing test
asserts? And is it a change the task was entitled to make, or one that should have
come back as a fork?

**Q2.** A pre-existing end-to-end test's second edit was changed from one pattern to
another, with its payload assertion updated to match, on the ground that the original
value coincidentally equalled the fixture's own pattern - so typing it back cleared
the unsaved state one edit before the save, and the new assertion would have passed
even with the save's own mechanism broken. Verify the claimed coincidence and the
claimed false-green. Then rule on the repair: is changing the typed value the right
fix, is the changed assertion still asserting what it did before, and does the
standing grant that lets an implementer extend an unbroken local pattern reach a
change to an existing fixture value, or stop short of it?

**Q3.** Two intra-doc links in new documentation were switched to plain code-spans
because the referenced functions are private and the links would not resolve. Confirm
that at the tool rather than at the sentence, and say whether the resulting text still
points a reader at the right place.

**Q4, mine rather than the implementer's.** The end-to-end suite reports the same
total as before this task, while the plan's acceptance map names several observables
this task is supposed to produce - the shell learning the save state across a save,
the startup locale push, the live locale push, and the allowlist's effect. **Walk each
of this task's acceptance rows and name the producer you actually find.** A row whose
producer turns out to be an assertion added inside an existing case is fine and
expected; a row with no producer at all is a finding, and the unchanged total is the
reason to check rather than the finding itself.

## What is reproducible now

Everything. Re-run rather than re-derive, but **build your own instruments**, outside
the repository, at a path the implementer did not use.

Four environment facts, each of which has cost this project a wrong result: `grep`
here is a shell function honouring `.gitignore`, so use `command grep` when a sweep
must reach ignored paths, and paste the command you actually ran; never read an exit
status through a pipeline; **the end-to-end suite serves the BUILT bundle**, so a
frontend mutation needs a build before the run and so does the restore afterwards;
and `cp` and `rm` are aliased to their interactive forms, so use `command cp` and
`command rm` and verify every restore BY CONTENT rather than by exit code. This task's
prescribed red states delete and edit tracked files - restore discipline is not
optional here.

You are read-only on the repository except your own verdict file. The implementer ran
the full eleven-part gate and its output is in the report; do not re-run the whole
gate, run the checks your findings need.

## Harvest

Separate section at the end. Report the dominant patterns and any repeated defect
shape a future implementer should know; the controller writes these into the house
ledger and you never write to it yourself. Include explicitly any place where a
boundary in the brief forced a stop on a fork that in your judgement had no real
decision content.

## Output

Write your verdict to `.superpowers/sdd/plan-12/task-6-verdict.md`. Return to the
controller only: the two verdicts, finding counts by severity, your four adjudication
answers in one line each, and nothing else.
