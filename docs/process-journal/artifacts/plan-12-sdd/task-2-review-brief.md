# Task 2 review brief (Plan 12): the three-state language control

**Your role.** Independent reviewer. You did not write this change. You grade
it, you do not fix it: a fix you would have made is a finding instead. You
commit nothing and edit no product artifact.

**Ground truth, in this order:** the v1 spec section 8.2 as Task 1 amended it;
**the controller ruling `task-2-ruling-1.md`**, which sits directly below the
plan and above the task brief for the two questions it settles and is silent on
everything else; the task brief; decision **D106** in the plan's
`## Decision register`; the plan's `## Global Constraints`; and the four
house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`,
`docs/process-conventions.yaml`, `docs/decision-ledger.yaml`), cited by entry id.

**Read the ruling before the brief.** The task returned NEEDS_CONTEXT partway
through, and two of its steps were executed under the ruling rather than as the
brief's text reads. Grading those against the brief alone would produce two
false findings.

## What to read

| What | Path |
|---|---|
| The controller ruling (authority for two deviations) | `.superpowers/sdd/plan-12/task-2-ruling-1.md` |
| The task's requirements | `.superpowers/sdd/plan-12/task-2-brief.md` |
| The implementer's report, including its NEEDS_CONTEXT memo and its post-ruling section | `.superpowers/sdd/plan-12/task-2-report.md` |
| The diff, with commit list and stat | `.superpowers/sdd/plan-12/review-0c001ee..ea39d88.diff` |
| The decision this task implements | the plan document, `## Decision register`, D106 only |

Do not read the whole plan file.

## The consolidated requirement set for THIS task

Assembled by the controller rather than taken from the plan's own table.

| # | Requirement |
|---|---|
| T2-a | The settings language control offers a third option representing "no override", preselected when nothing is stored, so the effective locale and the displayed value agree and the choice stays reversible |
| T2-b | The sentinel is the empty string in a named module constant, mapped in BOTH directions: load is `baseline.locale ?? SYSTEM_LOCALE`, save is `form.locale === SYSTEM_LOCALE ? null : form.locale` |
| T2-c | The "absent means system locale" rule lives in ONE place - the exported `effectiveLocale` seam in `src/i18n/index.ts` - with `src/main.ts` and the dialog as its two callers. This is the defect under repair: one nullable field read with two unreconciled fallbacks |
| T2-d | The control displays a plain "System language" and does NOT name the resolved language |
| T2-e | The live switch compares the raw nullable saved values, so a `null` -> `"en"` and a `"de"` -> `null` transition both fire and an unrelated settings save does not |
| T2-f | The four catalog values land byte-exact in both locales, and the German ones carry correct German orthography |
| T2-g | Both pre-existing `smoke.spec.ts` locale-control assertions stay unchanged, and the report names each with the measurement showing why it stays valid |
| T2-h | `playwright.config.ts` and `e2e/mocks.ts` are NOT edited; `e2e/smoke.spec.ts` is not edited by this task |
| T2-i | An out-of-band stored locale gets no handling, no migration, no test |
| T2-j | Absence check L1 with its RED pre-state, its GREEN end state and its soundness control, all pasted, all measured on the FINAL committed tree |
| T2-k | The commit is unsigned, carries exactly one trailer, and is pathspec-scoped to the six files of the Files list |
| T2-l | **Under the ruling:** interaction locators resolve through the catalog of the locale actually rendering; every ASSERTED German string is one whose German value DIFFERS from its English value; no locator is counted as an assertion |
| T2-m | **Under the ruling:** case 3 is split into two cases, one per direction, direction 2 starting from its own stored `locale: "en"` scenario, with no reload anywhere and every assertion the plan prescribed preserved |

## Review dimensions

1. **Spec compliance** against the table above.
2. **Quality**: is the seam's contract right, are the comments true, does the
   code read like the file around it?
3. **`house`**: deviations from a recorded Tier-2 convention, flagged with the
   entry id.
4. **Latitude, both forms** - an explicit permission, and the commoner omission
   form (a set mandated but never enumerated, a placeholder, a step requiring an
   invented name). The test is "must a later reader invent something they are not
   allowed to invent?"
5. **The no-work-needed check.** Where a passage concludes that a guard, a test
   or a case is unnecessary, **run the claim that makes it unnecessary; do not
   weigh it.**
6. **Harvest**: dominant patterns and repeated rejections worth recording. You
   surface; you never write to the ledger.

## The mutation evidence needs a reviewer, not an acknowledgement

The implementer fired three mutations against the shipped mechanisms and reports
that each killed exactly the cases depending on it. **This is the strongest
evidence in the round and therefore the thing most worth checking**, because a
mutation that fails to discriminate looks identical to one that does.

For each of the three, judge two separate questions:

1. **Does the red state actually defeat what it claims to defeat?** Where the
   path from the mutated code to the assertion crosses a FALLBACK - and this
   feature is built on `buildBundles`'s `[requested, en]` chain, so it does -
   a mutation upstream of the fallback never reaches the assertion, and the test
   passes in exactly the state it exists to forbid. Ask it per mutation.
2. **Are the SPARED cases correctly spared?** The report names, per mutation,
   which cases must die and which must survive. A mutation that kills everything
   proves nothing about which case covers what.

Re-run any mutation you doubt. **Build your own instrument**: write scratch
files to paths the implementer could not have used, and never execute a fixture
or script it left behind. Agents in one session converge on the same names, and
a re-run that silently executes the implementer's own instrument agrees by
construction.

## Adjudication questions - one required verdict each

**Q1. The split's coverage.** The ruling replaced one prescribed case with two.
Rule whether every assertion the plan's case 3 prescribed survives in the pair,
naming any that does not. Both answers are available: that the split preserves
the prescribed coverage and adds the live path the single case could not reach,
or that something the single case asserted is now unasserted.

**Q2. The locator ruling's boundary.** The ruling permits a German locator but
explicitly does NOT relax the assertion rule. Rule whether the implementation
respects that boundary: is any German-valued locator doing assertion work, and
is every asserted German string one whose value differs across the two catalogs?
Check the values yourself rather than trusting the report's list.

**Q3. A gate observation the implementer surfaced.** It reports that `pnpm lint`
runs plain `eslint .` with no `--max-warnings`, so eslint *warnings* pass the
gate silently, found by shipping a one-line `<option>` and watching it warn
twice without failing. Controller measurement, for your information rather than
as a conclusion: `eslint.config.js` sets the house's own
`@intlify/vue-i18n/no-raw-text` rule to **`"error"`**, so the D27 no-raw-text
claim in the plan's acceptance map holds; the warnings observed come from
preset-level rules. Rule on whether this affects any coverage claim THIS task
makes, and say so if it does not - a "no" recorded is worth more than a silence.

## Your verdict

Write it to `.superpowers/sdd/plan-12/task-2-verdict.md` and return a short
summary.

It carries: an overall **spec compliance** verdict and an overall **task
quality** verdict (both required); findings graded Critical / Important / Minor,
each located by symbol or test name, never by line number; your three
adjudication verdicts; your per-mutation judgment; your harvest; and anything
you could not verify, named as such with the reason.

Every quotation is copied from the artifact; every number is one you measured. A
paraphrase is fine and is marked as one.
