# Plan 11 - controller brief for the plan author

You author the execution plan for Muxsmith's Plan 11. This brief is the input;
the plan is your artifact. An independent reviewer grades it against this brief
(requirement compliance, quality, the coverage dimension, the latitude scan) and
a fix loop runs until it is approved. The governing human approves after that.

House pattern for the document itself: `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`.
Match its structure, its level of specificity per step, and its acceptance-item
style. Your file: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`.

## Verify the brief, do not trust it

Every premise below is the controller's and may be wrong. Refuting one against
the tree is a valid and wanted completion, not a deviation. Where a figure
appears, re-measure it; where a file:line appears, open it. Several figures in
this project's history were controller recollections that a plan author
corrected, and one of them (a corpus count of 17 that was really 20) reached the
tracker before it was caught.

## What this plan is, and what it is not

Two independent bodies of pre-1.0 residue: the two open dependency-vulnerability
alerts, and four routed documentation defects that were each correctly left
un-fixed by earlier plans because they sat outside every task's Files list.

**It is NOT a 1.0 completeness statement, and nothing in the plan may read as
one.** The owner QA gate (Tier-2 `owner-manual-qa-gates-the-1-0-release`) is
unsatisfied: the owner's manual product pass is running in parallel with this
plan, and its output is first-class scope input in three shapes he named (real
bugs, behaviour he dislikes even where it matches the spec, v1.x items he wants
in 1.0 after all). So 1.0 scope is unknown by construction while this plan runs.
Do not write "this closes the pre-1.0 list" anywhere.

## Structure the plan must carry

**Two streams, in separate git worktrees, merged sequentially with a full gate
run on each merged state.**

- **Stream A - documentation accuracy.** Work items 2 to 5 below. Serial tasks
  inside the one worktree: they are documentation and configuration edits, they
  partly share files, and the doctrine's parallelism boundary (a stream earns its
  overhead only when its own work exceeds one gate run plus one merge) puts them
  in one tree rather than one tree each.
- **Stream B - the dependency alerts.** Work item 1 below, as ONE task with
  three parts, per the owner's ruling of 2026-07-29 recorded in the ROADMAP's
  pre-1.0 gates section. It clears the parallelism boundary on its own (it
  installs, compiles and runs suites), and it is streamed separately for a second
  reason worth stating in the plan: a transitive lockfile bump can turn frontend
  gate parts red for reasons that have nothing to do with stream A, and a shared
  tree would block stream A behind that.

The task cut WITHIN each stream is yours to decide and the reviewer grades it.
The stream split above is fixed.

Owner-ruling note you must respect while cutting: the dependency work was ruled
"its OWN one-task vehicle, not a Plan-10 rider", with the recorded reason that
Plan 10 was a contract already under execution and a package reopened for every
incoming finding stops being a contract. This plan carries it from the start as
its own stream, which satisfies that reason; it does not become a rider on the
documentation work, and no documentation task touches its files.

## Work item 1 - the two open dependency alerts

Tracker: `docs/ROADMAP.md`, section "Pre-1.0 release gates", the entry beginning
"TWO OPEN VULNERABILITY ALERTS". Read it in full; it carries the exposure
analysis and the owner's ruling on the vehicle.

Both alerts re-verified OPEN at the source on 2026-07-30 by the controller
(`gh api repos/senolfeldmann/Muxsmith/dependabot/alerts`): `postcss` HIGH in
`pnpm-lock.yaml`, `glib` MEDIUM in `Cargo.lock`.

Three parts, one task:

**(a) Bump `postcss` past 8.5.17 through the lockfile.**

Decision the plan settles explicitly, because leaving it open is latitude: the
tracker's words are "through the lockfile ... a lockfile decision rather than a
pinned-dependency one", so the prescribed mechanism is a lockfile-level update
of a transitive dependency, NOT a new `pnpm.overrides` entry in `package.json`
and NOT a direct dependency addition. Reason to state: this project's toolchain
doctrine pins direct dependencies exactly, and adding an override for a
build-time transitive package changes what the manifest asserts about the
dependency graph.

If the transitive parent constrains `postcss` so that a lockfile update cannot
move it past 8.5.17, that is a genuine fork and it returns as NEEDS_CONTEXT with
the options and their costs. The plan must say that in those terms rather than
pre-authorising the override as a fallback - a pre-authorised fallback is the
sanctioned fork the latitude ban exists to prevent.

**(b) MEASURE the `cargo deny` disagreement. Do not fix it.**

The gap is worth more than either bump: `cargo deny check` is a gate part and it
is green on this tree while GitHub reports a Rust advisory, and `deny.toml`'s
ignore list does not mention `glib`. Two mechanisms this project relies on
disagree, and until the disagreement is explained neither may be quoted as
coverage.

Two constraints the plan states:

- **The output is the measurement, not the hypothesis.** A hypothesis is on
  record (RustSec classes unsoundness as `informational`, and this
  configuration may not fail on that class). The task's deliverable is what was
  measured and how, at the source - the advisory's own class, this project's
  effective `cargo deny` configuration for that class, and a demonstration that
  reproduces the green result and the reported advisory together. Restating the
  hypothesis is not a completion.
- **The task changes neither `deny.toml` nor the `cargo deny` invocation.**
  Reason to state: making the check fail on that class would interact with the 18
  commented RUSTSEC ignores already in `deny.toml` and would change what a gate
  part covers, which is a separate decision with its own owner-visible surface.
  The measurement's disposition belongs to the controller and, where it moves
  gate coverage, to the owner.

**(c) `glib`: investigate only, no fix.** Establish whether it can move
independently of Tauri's own dependency tree. If it cannot, the finding IS the
result - "this is an upgrade project, not a bump" - and it earns its own vehicle
rather than being forced into this task. State that outcome as an acceptable and
expected completion, so nobody stretches the task into a Tauri upgrade.

## Work items 2 to 5 - four routed documentation defects

Read each tracker entry in full before planning it; each carries measurements
and a recorded reason for the disposition.

**Item 2 - `.github/workflows/ci.yml`'s surviving line-number citation.**
Tracker: `docs/ROADMAP.md` section "Docs accuracy", first entry. A comment cites
`queue.rs:73`, which the tracker records as already stale. **Correction to that
entry you should verify and then fix as part of your work: its "OPEN OWNER
QUESTION" paragraph is stale.** The owner widened the ruling on 2026-07-29 to
reach CI and configuration comments, and the widening is recorded in the Tier-2
statement of `comments-locate-by-symbol-never-by-line-number` in
`docs/conventions.yaml`. Read that statement: it is the governing text, it names
the handle (replace the number with the symbol the line sits in; where no symbol
names it, name the nearest one plus what you mean), and it carries a scope
boundary that still exempts process artifacts under `docs/`. The tracker's own
correction is the controller's to write, not yours - surface it and I will fix
it; your plan covers the `ci.yml` comment itself.

**Item 3 - "byte-exact" overstates what `raw:` does for numeric scalars.**
Tracker: `docs/ROADMAP.md` section "Docs accuracy", the entry beginning
'"byte-exact" overstates'. **The behaviour stays and only the wording changes**
- that is the recorded disposition, and a behaviour change would be an
owner-visible product decision, not a documentation repair. The controller has
put the behaviour question to the owner separately; if he rules that `raw:`
should really be byte-exact, that is its own vehicle and this plan is unaffected.

The plan states the exact replacement wording for every site, and the wording is
derived from what `scalar_eq` actually does (its cross arms comparing
`Scalar::Int` against `PropValue::Float` and the reverse after an `as f64`
conversion) rather than from the tracker's paraphrase. The spec is authoritative,
so the spec's wording is decided first and the other sites follow it.

**Item 4 - the v1 spec's section 8.1 synopsis omits `validate`'s flags.**
Tracker: `docs/ROADMAP.md` section "Docs accuracy", the entry beginning "The v1
spec's section 8.1 synopsis". The correct flag set is derived from the SHIPPED
BINARY's `validate --help`, not from the tracker and not from the source, because
the drift here runs the other way (the authoritative document underclaims a real
surface). A spec amendment sweeps the spec for self-contradictions before commit
- doctrine section 1, and the tracker notes this item is itself one such
contradiction. Items 3 and 4 both amend the v1 spec, so they cannot be
concurrent tasks.

**Item 5 - `BUILDING.md`'s three positional gate ordinals plus its one
over-80 prose line.** Tracker: `docs/ROADMAP.md` section "Gate-count derivation
has no check", the paragraph beginning "A neighbouring class". Three ordinal
sites re-measured there with a stated expression, one of which hides in a
hard-wrapped paragraph, plus one 86-character non-fenced prose line that Plan 10
Task 1 left behind and correctly did not reflow.

Two constraints the plan must state:

- The reflow and the ordinal rewrite land in the SAME edit, because the fenced
  paragraph carries a within-file qualifier naming the very ordinals a reflow
  would move.
- `scripts/ledger-lint.py` parses `BUILDING.md`'s `gate-total` and `gate-block`
  markers and is itself a gate part. Any edit to that file must leave its
  invariant intact and must not add, remove or move a marked block. Verify by
  running the check, and say in the plan that this is verified rather than
  assumed.

## The scope unit for a repeated fact is the set of assertions

House rule this project paid for three times in one session: when a change moves
a FACT that several texts assert, the scope unit is the set of assertions, not a
file list. Items 3, 4 and 5 are all of that shape.

So: **the tracker entries above are the STARTING POINT, not the enumeration.**
For each item, derive the complete set of sites by grepping the tree for the
fact, state the expression you used, and report the delta against the tracker's
list. If your derived set is larger than the tracker's - which has happened
before with exactly these entries - the delta is a finding, and it belongs in the
plan.

When you write a search expression that contains an enumerated set of its own
(file extensions, word forms, keywords, paths), derive that set from the artifact
too. The recorded defect here is a corpus count that came out at 17 instead of
20 because the expression enumerated the cited file extensions and left out
`.md`; a fire test against a known-present member does not catch a missing
member of the pattern's own set.

## Absence-shaped acceptance items need a prescribed red state

Most of stream A's acceptance items will be absences: a grep that finds no
line-number citation, no "byte-exact" claim, no positional ordinal. An absence
proves nothing until the check has been made to fire once.

Therefore every absence-shaped acceptance item in this plan names three things:
the expression, the PRE-STATE run that makes it fire with an exact expected
non-zero count, and the END-STATE run with its expected zero. An item that
carries only the end state is incomplete. This is not a general nicety: the
Plan-10 review found that three of four absence-shaped checks in that plan had
no prescribed red state at all, and the plan-review brief for Plan 11 will check
for it.

Related distinction the reviewer will apply, so build the plan to survive it: a
figure you MEASURED against the tree is reproducible by a reviewer now; a check
an implementer will PERFORM later against a deliverable that does not yet exist
is not. Keep the two visibly apart in the plan.

## Standing constraints every task in the plan inherits

- **The v1 spec is authoritative** over designs and plans on conflict.
- **SI-3, the mkvtoolnix parity duty:** any behavioural question compares against
  mkvtoolnix-gui / mkvmerge, reading the source at `~/Downloads/mkvtoolnix` and
  confirming behaviour by running the binary rather than from memory. Item 3
  touches matching semantics wording, so it is the one place here where this is
  likely to bite; do not skip it on the grounds that only wording changes.
- **Tier-2 conformance:** `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
  `docs/process-conventions.yaml` are review ground truth alongside the spec.
- **No task edits the house-knowledge YAML files** (`decision-ledger.yaml` and the
  three Tier-2 files). The controller is their single writer.
- **The gate is what `BUILDING.md` enumerates**, foreground, no subsets, before
  any push. Do not state a part count from memory; if the plan needs one, cite
  the file and agree with it.
- **A comment never locates code by line number** and **a document never cites a
  line number inside itself** - both owner-ruled Tier-2 entries, and item 2 and
  item 5 are in that family, so the plan itself must not violate what it repairs.
- **Two writers in one working tree share one git index**, so a task that commits
  while another writer is live uses pathspec-scoped commits (`git commit --
  <paths>`).
- **Tests belong to the feature package.** A package producing a user-visible
  consequence ships its test in the same package. Weigh this honestly per item:
  most of stream A produces no behavioural consequence and owes no test, and
  saying so with the reason is correct. Do not use it as a blanket exemption, and
  do not defer a scenario the existing test infrastructure can already express.
- Every dispatch in this project names its model explicitly; the plan does not
  hardcode a model name or a commit trailer string, because the trailer is
  derived from the dispatch's model parameter. A plan that writes one as a
  literal contradicts the dispatch that assigns another.

## What the plan must contain beyond its tasks

- The two streams, their worktrees, the merge order, and the gate run per merge.
- Per task: its Files list (an unmarked list reads as exhaustive; mark an
  exemplary one explicitly), its steps, its acceptance items with the red/green
  prescription above, and its constraints.
- A closing "deferred by decision" note covering every deferral the plan itself
  utters, each with a concrete vehicle. "Later" and "a cleanup pass" are not
  vehicles.
- No design-latitude clause anywhere. Not "if you find a simpler equally-safe
  alternative", not "either approach works", not "the implementer may choose".
  The omission form counts too: any set that is mandated but never enumerated,
  any list ending in an ellipsis, any "one per X" without the X list, is
  latitude. Ask of every normative sentence: must the implementer invent
  something it is not allowed to invent?

## Scoping decision, recorded so you do not re-open it

**No separate design document.** The forks in this plan are wording-level and
mechanism-level, all decidable against the tree, and no interface, wire format or
architecture is at stake. The plan carries the decisions and the plan review
grades them. If you find a fork that genuinely needs an architectural decision,
that is a finding: return it rather than resolving it inside the plan.
