# Plan 12 - controller brief for the plan author

You author the execution plan for Muxsmith's Plan 12. This brief is the input;
the plan is your artifact. An independent reviewer grades it against this brief
(requirement compliance, quality, the coverage dimension, the latitude scan) and
a fix loop runs until it is approved. The governing human approves after that.

House pattern for the document itself:
`docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`. Match its
structure, its per-step specificity and its acceptance-item style. Your file:
`docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`.

## Where the requirements come from

Both work items are findings from the owner's manual QA pass, round 3, on
Windows, and both are already OWNER-RULED. The authoritative record is
`docs/ROADMAP.md`, section "Pre-1.0 release gates", the entry beginning
"OWNER QA PASS, round 3". **Read that entry in full before anything else.** It
carries the measured cause of each finding, the spec classification, the parity
comparison, the rulings, the boundary the second ruling had to be fenced
against, and the cost measurements. This brief does not restate it.

The reconnaissance that produced those measurements was read-only and is not on
disk as an artifact; the ROADMAP entry is its distillate and is what you work
from. Where the entry states a measurement, RE-MEASURE it. Several figures in
this project's history were controller recollections that a plan author
corrected against the tree, and correcting one is a wanted result, not a
deviation.

## Scope

**In scope, two work items:**

1. **The locale settings defect.** Owner-ruled shape A: the settings language
   control gains a third option representing "no override" (system language),
   preselected on first run, so the effective locale and the displayed value
   agree and the choice stays reversible.
2. **Blank profile creation.** Owner-ruled: a New action must create an empty
   profile in the editor, because without it the product cannot be used at all
   without hand-authoring a YAML file first.

**Explicitly OUT of scope, and the plan says so in its scope section rather than
leaving it to be inferred:**

- **Deriving a profile from a file the user selects whose structure is read
  out.** The owner ruled this a pre-1.0 item as well, but it gets its own
  package with its own design round ahead of it, because it needs new core
  mapping machinery and a product-boundary ADR. It must not be started, prepared
  for with speculative seams, or partially built here. If you find that item 2
  cannot be built without a decision that belongs to it, that is a finding:
  return it rather than resolving it.
- **A bundled template or example profile.** Not ruled in, and recorded in the
  ROADMAP as staying unbuilt so it is not revived by implication.

**This plan does not close 1.0 scope and no sentence in it may read as though it
did.** The owner QA gate is a standing precondition on the tag; his pass is
currently STOPPED on work item 2 and resumes once a build carrying it exists.
Further findings are expected.

## Decisions the plan must settle, explicitly

The latitude ban is absolute: no unresolved design question reaches an
implementer. Each item below is a fork today. The plan decides it and states the
decision; it does not hand it over, and it does not hand it over by omission
either (an unenumerated set in a normative position is latitude just as much as
an explicit permission).

### Work item 1, the locale control

1. The sentinel a `<select>` uses for "system", since an option value cannot be
   `null`, and its mapping in BOTH directions (load and save).
2. Where the resolution rule lives. "Absent means system locale" currently
   exists in exactly one place, `resolveLocale` in `src/main.ts`, and the dialog
   now needs it too. A shared seam or a deliberate duplication - decide, and say
   why.
3. What the control DISPLAYS while "system" is selected, including whether it
   names the resolved language.
4. The live-switch path when the user switches TO system. The existing
   switch calls `applyLocale` with a concrete locale, which the system option
   does not directly supply.
5. The exact catalog strings, in `locales/en/` AND `locales/de/`. Cross-locale
   parity is a hard failure in `scripts/check-i18n.mjs`, not a warning, and
   `eslint.config.js`'s no-raw-text rule forbids a literal in the template.
6. The existing e2e assertion that expects the locale control to hold `en`
   asserts the defect as correct behaviour. Decide whether it is corrected or
   replaced, and state which.
7. The new test's mechanism for presenting a non-English system locale. No test
   in the suite can observe one today: `playwright.config.ts` pins `en-US` and
   plan-5 D29 pins the test locale to English deliberately so role names match
   the `en` catalog. A describe-level `test.use({ locale: ... })` does not
   disturb that decision; a config change would. Decide and justify.
8. Whether `e2e/mocks.ts`'s default `locale` value changes. Every scenario
   inherits that default, so this is a suite-wide input change to be weighed
   explicitly, never assumed.
9. The spec amendment. Spec 8.2's app-settings enumeration does not list locale
   at all and 8.4 says nothing about the surface, so the control's behaviour has
   never been specified. State what the spec gains.
10. An ADR is owed, because D56 declared the settings write out of scope by name
    and that scope statement is why the defect survived.

### Work item 2, blank profile creation

11. **The seed, MEASURED against the validator rather than reasoned about.** A
    schema-minimum profile satisfies every JSON Schema `required` and still
    fails validation, and Save is disabled while any error-severity diagnostic
    exists, so a bare seed produces a dead Save button on first use. Run the
    validator on your candidate seeds, paste the output, and choose on the
    measurement. The recommendation on record is a seed following the existing
    "Add appends an empty rule, incomplete until filled, announced by a
    validation warning" idiom from spec 8.2; if your measurement contradicts
    that recommendation, the measurement wins and you say so.
12. **The `currentPath` decoupling.** It doubles as "where to save" and as "may
    I edit and validate at all", and the validation watcher is gated on it, so a
    created-but-unsaved profile would not revalidate. State what replaces each
    duty and what the watcher gates on instead. This is the actual work of the
    item; treat it as such rather than as a precondition.
13. Save with no path yet: whether Save opens a save dialog, or a distinct
    Save-as action appears. The save-dialog capability is already granted and
    already used once for the job-log export, which carries a documented
    capture-state-before-the-dialog-gap pattern worth conforming to.
14. Losing an unsaved new profile: switching tabs, opening another profile,
    closing the app. Decide what happens. Whether the editor may hold an unsaved
    profile at all has never been decided, so silence here is a real fork rather
    than a detail.
15. Where New is offered. The editor's empty state currently renders a
    "Diagnostics" heading over a panel with no empty-state branch, so it displays
    nothing; whether that is repaired here, and whether Batch's empty state also
    offers New, are decisions.
16. The exact catalog strings, both locales, under the same hard parity gate as
    item 5.
17. The in-app help currently names open-or-reopen as the editor's entry set.
    Decide whether the help topic changes; the help-topic-tree gate (D62) and
    the editor-tooltip completeness gate (D55) apply to whatever lands.
18. The spec amendment. Spec 8.2's editor list has no create clause and a Plan-6
    design reviewer struck the create premise explicitly. The owner has now ruled
    creation in, so the spec gains it.
19. An ADR is owed for the same reason as item 10: a recorded design position is
    being reversed by an owner ruling, and the reversal needs its record.

**Both work items amend the v1 spec, so they cannot be concurrent tasks.** A
spec amendment also sweeps the spec for self-contradictions before commit
(doctrine section 1). Cut the tasks accordingly.

## Structure

Serial tasks in ONE worktree. The doctrine's parallelism boundary is a
comparison, not a count: a stream earns a worktree, a merge and a full gate run
only when its own work exceeds that overhead, and these two items share the spec
file and both touch frontend catalog and test surfaces. Two streams would run
the eleven-part gate twice and still have to serialise the spec edit.

## Tests are part of the package, not a follow-up

Both items produce user-visible consequences, so both ship their tests in the
same package. The recorded exemption is narrow and does not apply here: new test
INFRASTRUCTURE may be deferred, a scenario the existing infrastructure can
already express may not. A describe-level locale override is existing
infrastructure. Do not write "coverage follows in a later plan" about behaviour
this plan introduces, and do not use a scope argument to remove a test - that
argument proves too much, since it would remove the test of every feature.

Where an acceptance observable has two sides - rendered and persisted, the
control's displayed value and the file's written content, the editor's state and
the saved YAML - each side needs its own named producing check. One producer
named for the whole observable satisfies the coverage map while covering one
side, which is how a gap survived two review rounds on Plan 9.

## Absence-shaped acceptance items need a prescribed red state

An absence proves nothing until the check has been made to fire once. Every
absence-shaped acceptance item names three things: the expression, the PRE-STATE
run that makes it fire with an exact expected non-zero count, and the END-STATE
run with its expected zero. An item carrying only the green state is incomplete.
The Plan-10 review found three of four such checks had no prescribed red state,
and the reviewer brief for this plan will check for it.

Keep visibly apart: a figure you measured against the tree, which a reviewer can
reproduce now, and a check an implementer will perform later against a
deliverable that does not yet exist. Only the first is re-runnable.

## Standing constraints every task inherits

- **The v1 spec is authoritative** over designs and plans on conflict - except
  where this plan amends it under an owner ruling, which is the point of items 9
  and 18.
- **SI-3, the mkvtoolnix parity duty:** behavioural questions compare against
  mkvtoolnix-gui / mkvmerge, reading the source at `~/Downloads/mkvtoolnix` and
  confirming behaviour by running the binary, never from memory. The ROADMAP
  entry already carries the parity finding for the New action and its empty
  state; verify it rather than inheriting it, and note that the load-bearing
  distinction on record is interactive-versus-declarative-batch.
- **Tier-2 conformance:** `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
  `docs/process-conventions.yaml` are review ground truth alongside the spec.
- **No task edits the house-knowledge YAML files** - the controller is their
  single writer.
- **The gate is what `BUILDING.md` enumerates**, foreground, no subsets, before
  any push. Do not state a part count from memory; cite the file.
- **A comment never locates code by line number** (name the symbol; naming the
  file is fine) and **a document never cites a line number inside itself**. Both
  are owner-ruled Tier-2 entries.
- **Two writers in one working tree share one git index**, so a task committing
  while another writer is live uses a pathspec-scoped commit.
- The plan hardcodes no model name and no commit-trailer string: the trailer is
  derived from each dispatch's model parameter, and a plan that writes one as a
  literal contradicts the dispatch that assigns another.

## Scoping decision, recorded so you do not reopen it

**No separate design document.** The forks above are surface and seam decisions,
all decidable against the tree, and no architecture, wire format or public
interface is at stake. The plan carries the decisions and the plan review grades
them; the two ADRs are authored inside their tasks' diffs, which is the light
form the doctrine provides. If you find a fork that genuinely needs an
architectural decision - and item 12 is where that would show up - return it as
a finding rather than resolving it inside the plan.

## Do not commit

Write the file and stop. Another agent is committing in this working tree in
parallel, and two writers sharing one git index is a recorded defect class here.
The controller commits your plan.
