# Amendment 1 review brief

You are the INDEPENDENT REVIEWER of a mid-run amendment to an owner-approved
software plan that is currently in execution. You did not author it and you have
no stake in it standing. Your verdict gates whether the amended plan is
dispatched to implementers.

## The three artifacts

- **Requirements:** `.superpowers/sdd/plan-12/amendment-1-brief.md` - the
  controller brief the author worked against. This is what you grade compliance
  against, requirement by requirement.
- **The author's report:** `.superpowers/sdd/plan-12/amendment-1-report.md`.
- **The diff:** `.superpowers/sdd/plan-12/amendment-1-review-411122f..bf857ed.diff`
  - commit list, stat and full diff with context, in one file.

Supporting context you will need to open: the amended plan
`docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`, the amended
decisions file `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`, the
ruling `.superpowers/sdd/plan-12/owner-ruling-1-failed-load-empty-state.md` and
the memo behind it, and `src/views/EditorView.vue`.

## Scope

**The amendment only.** The approved plan is not re-reviewed and settled
decisions are not re-litigated. A pre-existing defect the amendment did not
create is reported as an observation, clearly marked as pre-existing, and does
not block approval - but say so if you find one, because the amendment may make
it more likely to mislead.

## Two verdicts, both required

1. **Requirement compliance:** brief requirements A-1 to A-10, one line each,
   MET or NOT MET, with the evidence you used.
2. **Quality**, as findings graded Critical / Important / Minor.

## Dimensions to run

**Coverage.** Walk the ruling and the brief's normative content section by
section and name the plan text that carries each. A requirement with no
carrier is a finding. Then walk the amendment's acceptance rows in their
HALVES: a consequence with two observable sides - rendered and hidden, before
and after, persisted and displayed - needs a named producing test per side. One
producer named for the whole observable satisfies the map while covering one
side, and that is the defect shape this dimension exists to catch. The row
exists, so nothing looks missing.

**Latitude, in both forms.** An explicit permission ("the implementer may
choose", "either approach works") is the rare form. The common form is
omission: a mandated set that is never enumerated, a list that trails off, a
"one per X" with no X list, a step that requires inventing a name, a string, a
key, a selector or a file that is not written down somewhere the implementer can
read. The test is not "does a permission appear?" but **"must the implementer
invent something it is not allowed to invent?"** Ask it of every normative
sentence the amendment adds, not of its vocabulary.

**House conformance.** The four house-knowledge files
(`docs/product-boundaries.yaml`, `docs/conventions.yaml`,
`docs/process-conventions.yaml`, `docs/decision-ledger.yaml`) are review ground
truth alongside the plan; cite entries by id. The ones nearest this diff:
`a-document-never-cites-a-line-number-inside-itself`,
`comments-locate-by-symbol-never-by-line-number`,
`proc-normative-count-recomputed`, `proc-proposed-safeguard-stays`,
`proc-verification-step-must-be-falsifiable`,
`proc-check-green-state-reachable`,
`a-search-whose-terms-come-from-memory-produces-a-false-absence`,
`proc-sweep-surface-completeness`, `tests-ship-with-the-feature-never-after`,
`proc-04-spec-wins`, `editor-generic-action-keys` (the editor catalog budget is
a hard boundary). Also check the ADR shape against the file's own existing
entries: required slots, immutability, dated event-log semantics, how
supersession is expressed.

**The no-work-needed check, standing.** Wherever a passage concludes that a
guard, an enumeration, a check or an edit is unnecessary - "so no spec change is
needed", "that state is unreachable", "the existing gate already covers it",
"this cannot happen" - **verify the claim that makes it unnecessary. Run it; do
not weigh it.** The trigger is readable in the text. This amendment contains at
least one such conclusion by construction, because requirement A-8 asks for one.

**Typography and prose.** ASCII hyphens, straight quotes, no Unicode ellipsis,
no em-dash or en-dash, in the plan text, in the ADR, and in every string and
comment the amendment prescribes. German orthography inside German strings is
orthography, not a violation.

## What is reproducible now, and what is not

The amendment carries two different kinds of empirical claim and they are graded
differently.

- **CLAIMED measurements** - figures and readings the author took against the
  tree as it stands: the current gating of the four surfaces, the highest
  decision number, the spec-8.2 measurement, the self-contradiction sweep, every
  recomputed count. **Reproduce these.** They are the ones that can be wrong
  right now.
- **PRESCRIBED evidence** - the fires, red states and expected results the
  amendment tells a future implementer to produce against code that does not
  exist yet. You cannot run these; do not report agreement you could not have
  obtained. **Grade each as a DESIGN against its specification:** does the
  prescribed red state actually defeat the mechanism the check exists to
  protect, or does it merely disturb that mechanism's input? Where the path from
  the mutation to the assertion crosses a fallback - a default branch, a
  chain, an `unwrap_or`, a `??` - a red state mutated upstream of the fallback
  never reaches the assertion, and the check passes in exactly the state it
  exists to forbid.

**Build your own instruments.** When you reproduce a measurement, write your
harness - the script, the extracted region, the comparison - at a path the
author could not have written and did not use, never at the obvious shared
scratch path both of you would default to. Re-running the author's own
instrument produces agreement by construction. Where a measurement's expression
contains an enumerated set (extensions, paths, keywords, word forms), that
enumeration is itself a claim: derive it from the artifact, not from the
author's list, because firing the expression against a known-present member
passes while a missing member stays invisible.

## Adjudication questions - one required verdict each, in your own judgement

The author raised three concerns. Each needs a ruling from you; a concern merely
carried dies as noted-without-ruling. They are phrased open deliberately and are
not pre-rated.

**Q1.** The author reports that several of Task 4's prescribed test cases build
undo history and then click Open, and that once Task 5 lands the `pickAndOpen`
guard those clicks will raise a confirm dialog, so the open never happens. Is
that reading correct against the plan's text for Tasks 4 and 5? If it is, does
it follow that those cases break when Task 5 lands, or is there something in
either task that already answers it? And is it, as the author claims, entirely
pre-existing - or did the amendment add or worsen an instance of it?

**Q2.** The author reports that the ROADMAP's round-3 finding-2 entry still
states that `currentPath` has one write site (`openPath`), which Task 3
falsified. Is the claim about the tree correct? Does the amendment's own
reasoning depend on that ROADMAP sentence anywhere, and if so is the dependency
sound?

**Q3.** D112 gates on `currentPath` again, after D107 moved duties off it. Is
that consistent with D107 as written, or does it re-load a term D107
deliberately unloaded? Answer against D107's actual text and the state walk,
not against the author's summary of either.

## Harvest

Separate section at the end of your verdict. Report the dominant patterns you
observed and any repeated rejection or repeated defect shape - what a future
author of this kind of artifact should know. The controller writes these into
the house ledger; you never write to it yourself. Include, explicitly, any place
where a boundary in the brief forced the author to stop on a fork that in your
judgement had no real decision content - an over-restriction finding is a wanted
harvest item here, not second-guessing.

## Output

Write your verdict to `.superpowers/sdd/plan-12/amendment-1-verdict.md`. Return
to the controller only: the two verdicts, the finding counts by severity, your
three adjudication answers in one line each, and nothing else.
