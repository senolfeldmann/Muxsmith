# Plan 10 plan fix brief, round 1

You are the plan's author, resumed. The independent review of your plan at
`da60634` returned **NEEDS FIXES**: 0 Critical, 8 Important, 6 Minor. The verdict
is at `.superpowers/sdd/plan-10/plan-review-round-1.md` - read it in full; this
brief routes it, it does not replace it.

**What the review confirmed, so you do not re-open it.** Coverage is complete and
no brief obligation lacks a task. All four of your refutations hold under an
independent instrument, including the corpus mechanism (17 under the old
extension set, 20 under the new, the delta being exactly the three
`README.md`-citing lines). The four-halves claim about D102 is true against the
source. The pairwise-disjointness claim is true of the Files lists. All 21 house
ids resolve. No scope prohibition is breached. Three latitude suspects were
adjudicated in your favour on the merits: the scoped reading of the gate-count
ban, the measurement-gated producer set, and W5's closed fact set.

The controller has independently re-run F1 and F2 and both reproduce. Do not
dispute them.

## Routing

**All fourteen findings are FIX.** None is disputed, none is accepted-as-known.
Two carry a controller decision that removes the choice the verdict left open;
they are marked below.

### Important

- **F1** - restate the Rust-gate reference evidence line with the correct
  enumeration and the correct ground, and re-point Step 1(b)'s parenthetical at
  it. The conclusion survives; the located claim does not. Controller
  re-measurement: 57 hits under your stated exclusions, and besides `BUILDING.md`
  the non-artifact hits sit in four retired plan documents under
  `docs/superpowers/plans/` (plans 5.7, 7, 8.5, 9). Ground the conclusion on the
  same principle the ROADMAP's MEASURED block already establishes - retired plan
  documents are history and are not edited - rather than on a location claim.
- **F2** - narrow the self-review claim to what is true and either drop the grep
  sentence or replace it with a search that can actually match what it audits.
  A self-audit sentence that is measurably false is worse than no sentence,
  because a downstream reader treats it as the check having been done.
- **F3** - **CONTROLLER DECISION: extend, do not leave narrow.** Widen Task 1's
  `BUILDING.md` scope by the one named region (the CI paragraph's `ledger-lint`
  parenthetical) with fenced replacement text, and let the `ci.yml` step `name:`
  widen alongside the comment it sits above. Your task's own stated purpose is
  that the script's name must not mislead; leaving two of three
  self-descriptions narrow defeats that purpose, and the within-file qualifiers
  you wrote would force an implementer to file a finding rather than repair a
  parenthetical its own edit falsified.
- **F4** - surface `BUILDING.md`'s positional ordinals (`part 6`, `parts 1-4`) as
  explicitly OUT of Task 1, with the reason, so the controller can route them.
  Surfacing is the correct treatment, the same one you already give the bare-span
  citations and the stale README counts; silently dropping them is not.
- **F5** - fence the full replacement paragraph for Step 1(d) verbatim, the way
  1(a) and 1(c) are fenced. "Keeps its first sentence" leaves the second
  sentence's fate - and with it the PyYAML prerequisite and the CI-job fact - to
  the implementer, and the added sentence's own wording is written down nowhere.
- **F6** - drop the `latitude-carveout-presentation-tokens` citation and rest the
  register on the ROADMAP README entry alone. The underlying latitude is sound;
  the entry cited does not reach prose and its own boundary sentence excludes
  semantic-carrying content.
- **F7** - state Task 2's transient write-set in the sequencing section and add
  the `report/json.rs` collision to why parallelism is unavailable even in
  principle. Answer the brief's actual ask ("note the file overlap that exists")
  rather than the narrower disjointness question. This strengthens your serial
  ruling rather than weakening it.
- **F8** - add a fourth fire that deletes one gate-BLOCK marker, runs, confirms
  exit 1 naming the missing marker, and restores; then re-point Step 1(c)'s
  closing sentence at that fire rather than at the total-marker fire. Three of
  your four absence-shaped checks currently have no prescribed red state, in the
  one task whose entire subject is that an unfired absence check proves nothing.

### Minor

- **F9** - attribute the `:81` hit to the matrix `os:` key rather than to
  `runs-on:`.
- **F10** - widen Task 5's NEEDS_CONTEXT trigger to "a site outside a comment, or
  a file not listed above".
- **F11** - note in Step 2 that a citation's continuation lines are rewritten with
  it, so the absence check's member enumeration is comments rather than matched
  lines.
- **F12** - **CONTROLLER DECISION: name the spec SECTION, not the line.** The
  neighbouring class is not staying unrouted (see the addition below), so writing
  one more instance of it inside the package that sweeps it out is incoherent.
- **F13** - "reworts" -> "rewords".
- **F14** - correct the overstated "and nothing else"; the about line is printed
  too. The load-bearing point is untouched.

## Two controller additions to the plan, beyond the findings

1. **The three bare line-span citations join Task 5's corpus.** They cite line
   spans into the design document with no filename token
   (`crates/muxsmith-core/tests/profile_save.rs`,
   `crates/muxsmith-core/tests/ts_export.rs`, `src/editor/registries.ts`), which
   is why your derivation rule did not reach them and why you correctly left them
   out. Controller ruling: a bare span without a filename is the worse form of
   the thing the owner's ruling bans, not the exempted one, and excluding them on
   a property of the search pattern would repeat exactly the mis-enumeration that
   cost the corpus count its first measurement. Each already names its D-entry, so
   the repair is a token deletion. Add them to Task 5's Files list, its corpus
   enumeration, its acceptance observable and its absence check, and state the
   ruling's ground in the task so the implementer does not re-derive it.
2. **Task 3 Step 2 carries a named trap.** The reviewer hit it: the rendered
   configuration-options page yields `prHourlyLimit` default `10` under naive
   reading, which is `prConcurrentLimit`'s value; only
   `lib/config/options/index.ts` settles it at `2`. Your fence is right. Write the
   trap into Step 2 so the implementer's re-verification does not "correct" a
   correct value.

## One item still open with the owner

The two measurably false counts in README's "How this got built" paragraph -
"D1 through D35" against a decision series reaching D105, and "all 78 of them"
against 219 verdict files - are with the owner as a scope question. They are NOT
in this fix round. If he rules them in, the controller will send that as a
separate addendum through this same channel; do not anticipate it.

## Constraints, unchanged

Write only the plan document. No git commands. Change nothing else in the repo.
If a finding's prescribed change is wrong on the merits, say so with evidence
rather than implementing it - but note that F1 and F2 have been independently
re-measured by the controller and reproduce.

Your report: what you changed per finding, anything you refuted with its
evidence, and any place where a fix forced a change the verdict did not
anticipate.
