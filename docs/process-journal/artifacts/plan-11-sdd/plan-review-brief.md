# Plan 11 - controller brief for the plan reviewer

You grade an execution plan authored by a different agent. You did not write it,
you owe it no deference, and your verdict gates it: the governing human sees it
only after you approve. A fix loop runs until you do.

## What you are grading

The plan: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`,
committed as `148f19f` (818 lines).

Against the requirement set: `.superpowers/sdd/plan-11/plan-brief.md`, the
controller brief the author worked from. It is the complete requirement set for
this plan; no addenda were sent. Read it before the plan.

Ground truth, in this order on conflict: the v1 spec
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) outranks designs and
plans; then the Tier-2 house-knowledge files
(`docs/product-boundaries.yaml`, `docs/conventions.yaml`,
`docs/process-conventions.yaml`); then `docs/ROADMAP.md`'s entries for the five
work items. `~/agents/peter/prompts/software-dev-process.md` is the controller
doctrine and binds the plan's form.

## Your dimensions

**1. Coverage.** Walk the brief's requirement set section by section and name the
task implementing each. A requirement with no task is a finding. Run the walk
over each acceptance observable's HALVES, not over the observable: a consequence
with two observable sides needs a named producing check per side. One producer
named for the whole observable satisfies a coverage map while covering one side,
which is how a real gap survived two review rounds on Plan 9.

**2. Latitude.** No sanctioned fork, in either form. The explicit form is a
permission. The commoner form is omission: a set mandated but never enumerated, a
list ending in an ellipsis, a "one per X" with no X list, a placeholder. The test
is not whether a permissive word appears - a keyword scan cannot see the omission
form. Ask of every normative sentence: **must the implementer invent something it
is not allowed to invent?**

**3. House.** Conformance to the recorded Tier-2 conventions. Flag any deviation
from a recorded convention, and HARVEST: report dominant patterns you observe and
repeated rejections you notice, so the controller can ledger them. You surface;
you never write to the house-knowledge files.

## The distinction that governs how you check evidence

The plan carries two kinds of evidence and only one is re-runnable by you.

- **CLAIMED measurements**: figures the author measured against the tree. These
  you reproduce NOW, and this plan rests on an unusual number of them (see
  below). Reproduce them with **your own instruments**: build your script, your
  fixture, your fetched copy at a path the author could not have written, and
  never at the obvious shared scratch path both of you would default to. Agents
  in one session share a filesystem and converge on the same names; a re-run that
  silently executes the author's own instrument produces agreement by
  construction. Put your instruments under
  `/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr11-review-independent/`.
- **PRESCRIBED evidence**: fires and checks an implementer will perform later
  against a deliverable that does not exist yet. You cannot re-run these without
  building the deliverable, and you must not report agreement you could not have
  obtained. Grade each one as a DESIGN against its specification instead: does
  the prescribed red state actually exercise the anchor the check exists to
  protect? That is a different and answerable question, and on Plan 10 it found
  that three of four absence-shaped checks had no prescribed red state at all.

## Six refutations the plan rests on - reproduce each one independently

The author refuted six premises of the controller brief against the tree. The
plan's content now depends on those measurements, so they are the highest-value
targets in this review. Reproduce each; a refutation that does not reproduce is a
blocking finding, and so is a refutation that reproduces but with a different
figure.

1. **Work item 2 is two sites, not one.** Claimed: besides
   `.github/workflows/ci.yml`, a `#` comment in
   `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` carries a bare line
   span. Derive the corpus yourself. **Derive the pattern's own enumerated set
   from the tree** (file extensions, comment forms, citation shapes) rather than
   from the plan's expression or from recall of what should be in it - the
   recorded defect in this exact class produced a count wrong by three because
   the measuring expression's own extension list was incomplete, and a fire test
   against a known-present member does not catch a missing member of the set.
2. **Work item 3 is 15 assertion lines, split 6 repair / 9 retain**, not the
   tracker's "three places", and the retain set is justified by
   `scalar_eq`'s behaviour for `language` and `codec_kind` being `PropType::String`
   so that no matchable value is coerced. Check both halves: that the repair set
   is complete, and that every retained line is genuinely TRUE rather than
   conveniently exempted. The second half is the one where a wrong call ships a
   false statement.
3. **Work item 4 is four of five synopsis lines plus an exit-code bullet**, not
   `validate`'s flags alone. Derive the CLI surface from the shipped binary's own
   `--help` output, not from source and not from the plan.
4. **Work item 1(b)'s hypothesis is refined**, and the refinement changes the
   disposition question: the advisory exists in RustSec as an `informational`
   unsoundness whose alias matches the GitHub advisory, so the two mechanisms see
   one advisory and this is not a database gap - but the class appears to not be
   EVALUATED at all rather than evaluated and tolerated. This is the measurement
   the whole work item exists for. Verify the claim and, in particular, verify
   whether the plan's task is written so that its output is a MEASUREMENT rather
   than a restatement of a hypothesis.
5. **The plan's own first pass violated the pattern-set rule** and the author
   says it self-corrected. Verify the correction landed, and check whether any
   other expression in the plan has the same shape.
6. **Work item 5 reproduces exactly, plus one spelled-out ordinal the tracker's
   expression cannot see**, judged a non-defect because it is a dated provenance
   record. Verify the reproduction and independently judge the non-defect call.

## Specific things to check, beyond the dimensions

- **The absence-shaped acceptance items.** Every one names its expression, a
  PRE-STATE fire with an exact expected non-zero count, and an END-STATE run with
  its expected zero. The author reports finding and repairing four rows of the
  `git diff --exit-code` kind that had a green state only. Verify all of them
  now carry a fire, and that each fire actually exercises the anchor rather than
  merely producing some non-zero output.
- **The controller's own arithmetic.** The author reports repairing a stated
  acceptance total of 34 against a real 32, present at two sites. Recount.
- **Two Tier-2 statements the author says this plan falsifies or dates**:
  `gate-includes-cross-target-lint-for-the-unrun-os` (which locates a check by a
  positional gate ordinal that task A1 removes) and
  `a-document-never-cites-a-line-number-inside-itself` (whose scope sentence
  predates the owner's widening to CI and configuration comments). The plan may
  not edit those files - the controller is their single writer - so confirm or
  correct the two claims and report them. Do not edit them.
- **The postcss decision.** The author reports the brief's fork does not exist
  today (both parent ranges admit the patched version) and deliberately does NOT
  fence the landing version, requiring only `>= 8.5.18`. Judge whether an
  unfenced landing version is right here or whether it is latitude wearing a
  reasonable justification. Both readings are defensible; say which and why.
- **The glib finding.** Claimed: twelve direct parents, all one gtk-rs
  generation, nothing newer in the lock, therefore an upgrade project rather than
  a bump. Verify the shape of the claim; the owner's ruling explicitly accepts
  "it cannot move independently" as a valid completion, so the risk here is an
  under-measured conclusion rather than a wrong one.
- **A safeguard the plan proposes stays in it.** If the plan proposes a guard,
  test, enumeration or check, do not recommend removing it on the grounds that it
  is unnecessary - it comes out only after it is built and measured redundant.
  Where any passage of the plan concludes that something is unnecessary, run the
  premise that makes it unnecessary rather than weighing it. That "so we need no
  X" shape has produced three false claims in one design round in this project's
  history.
- **Scope discipline.** Work items outside this plan (the owner QA round-3
  findings, which are Plan 12; the derivation package) must not appear as work
  here. Equally, the plan must not silently drop a briefed item.

## Verdict format

Write your verdict to `.superpowers/sdd/plan-11/plan-review-round-1.md` as a
file, in this session, before you answer. A verdict that exists only as your
final message is unsalvageable by construction, and 78 verdicts once had to be
re-mined from transcripts in this project.

Structure it as: overall verdict (APPROVED / NEEDS_FIXES / BLOCKED) with one
paragraph of reasoning; then numbered findings, each with severity, the exact
location, what is wrong, the evidence, and what would resolve it; then the six
refutation reproductions with your measured figures beside the author's; then
your harvest section for the controller.

Number every finding. If you carry a concern without a ruling, it dies as a note
- so give each one a verdict.

Do not edit the plan, any product file, or the house-knowledge YAML files. Do not
commit, stage or push anything. You grade; the author fixes.

Your final response is the return value, not a message to a human: the verdict,
the finding count by severity, and the refutations that did not reproduce.
