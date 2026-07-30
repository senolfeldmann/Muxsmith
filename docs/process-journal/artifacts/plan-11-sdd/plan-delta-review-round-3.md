# Plan 11 - delta review of fix round 3

Reviewer: the resumed original plan reviewer (fourth pass; same judge and standards
as `plan-review-round-1.md`, `plan-delta-review-round-1.md` and
`plan-delta-review-round-2.md`). Settled non-findings are not re-litigated.

Artifact: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
at `6cda608` (986 lines), diffed against `1e7b062` which I graded last round; 67
plan lines changed across 10 hunks. Tree: `master` at `6cda608`, working tree
carrying the co-writer's modified Plan-12 document. The repository is unmodified by
this review apart from this file.

Instruments built fresh for this round, under
`.../scratchpad/pr11-review-independent/round4/`, none reused from rounds 1-3:
`rv_execute.py` (extracts the document's fenced blocks and inline spans as
CHARACTERS and runs the extracted string through a shell, retyping nothing;
mutating commands are classified and skipped so a skip can never read as a pass),
`rv_consumers.py` (an independently derived Half B, deliberately wider than the
author's in three ways so a head its method cannot reach becomes visible). Prior
instruments re-run against this revision: the round-1 fenced-replacement checker
and the round-2 figure census.

---

## Verdict: APPROVED

**Plainly, for the gate: I approve this plan. It is ready for the governing
human.** No finding blocks it, and the one Minor item below is explicitly
non-gating.

The central claim of this round is the one I was asked to test, and it holds. I
extracted every fenced expression's characters and executed the string without
retyping anything. **Every executable fence in the document is self-contained and
returns exactly the figure stated beside it** - the ordinal probe 3, the
fence-aware length probe 1, expression A's fence 2, expression B's fence 1, the
repair expression 6, the retention expression 9, and the vocabulary sweep **100**,
which was the round-3 defect and is now correct at the root rather than at the
number. Nothing remains non-self-contained. The two stale heading counts are fixed
and my own recount confirms 5-over-5, 6-over-6 and 8-over-8, plus a cross-reference
to "four Step-4 checks" over exactly four bullets. My independently built Half B -
wider than the author's on three axes - finds no disagreement the author's did not
already adjudicate.

The clause has converged. Round 1 returned two blocking findings; round 2 three
Important; round 3 two Important and one Minor; this round finds one Minor
observation about a statistic that describes the author's own extractor and that no
downstream claim consumes. The marginal defect is now below the cost of another
round, and the remaining recorded work - the meta-text close action - is correctly
scheduled for the close rather than done under review.

---

## Per-finding disposition

| # | Round-3 finding (severity) | Disposition |
|---|---|---|
| NEW-1 | vocabulary sweep stated 100 for an expression returning 300 (Important) | **ADDRESSED - at the root** |
| NEW-2 | "a seven-member retained set", the consumer the sweep missed (Important) | **ADDRESSED** |
| NEW-3 | executor row mislabelled section 8.2 (Minor) | **ADDRESSED** |

**Tally: 3 ADDRESSED, 0 NOT_ADDRESSED, 0 disputed.** Cumulative over four rounds:
**23 findings raised, 23 addressed, 0 disputed.**

**NEW-1, and this is the fix I care most about.** The repair was not to change the
number to 300 or to state a prose narrowing, but to fence the whole invocation with
its flags and pathspec. I executed the fence's characters: **exit 0, exactly 100
lines.** Both narrowings carry their reason in the document, and I confirmed the
reasons are true: `-i` is dropped because `getByText` matches `byte`
case-insensitively (I measured 300 under `-i` against 120 case-sensitive, and the
three e2e spec files contribute 157 of the difference), and `Cargo.lock` is
excluded because a lockfile carries no prose claim. The variant table in the plan
matches mine exactly at all five points (300 / 120 / 118 / 98 / 100).

**NEW-2.** The tier row and the coverage map now both read "nine-member retained
set". The single surviving "seven" is Amendment 3's record of the repair - the same
shape as the surviving `part 5` and legitimate for the same reason.

**NEW-3.** It now reads "section 7's architecture table row", and I verified the
spec's `## 7. Architecture` heading sits at line 322 with the `executor` row at 347.

---

## Reproductions, with measured figures

### The execute-the-document test: **PASSES**

This is the claim I was made the test of, so the method matters: I parsed the plan
for fenced blocks and inline spans, took the characters, and passed each extracted
string to `bash -c` with the repository as the working directory. No expression was
retyped, corrected or completed. Commands that mutate - `git add`/`commit`,
`git worktree add`, `pnpm update`, `pnpm install` - were classified and skipped
rather than run, and counted separately.

| | measured |
|---|---|
| fenced blocks extracted | 43 |
| inline spans extracted | 1404 |
| executed verbatim | 122 |
| mutating, skipped (never counted as passes) | 11 |
| **fenced expressions that failed to be self-contained** | **0** |

Every executable fence, with its stated figure beside my measured one:

| fence | expression | stated | measured |
|---|---|---|---|
| L106 | expression B (bare spans), now in a fence because its character class contains a backtick | 1 | **exit 0, 1 line** |
| L277 | `grep -nE 'part [0-9]|parts [0-9]' BUILDING.md` | 3 | **exit 0, 3 lines** |
| L281 | the fence-aware length probe (heredoc) | 1 | **exit 0, 1 line** |
| L380 | expression A with `EXT=$(...)` defined inside the fence | 1 hit + the EXT echo | **exit 0, 2 lines** |
| L388 | the prose-locator blind-spot probe | 2 | **exit 0, 1 line** (see note) |
| L592 | the repair expression | 6 | **exit 0, 6 lines** |
| L602 | the retention expression | 9 | **exit 0, 9 lines** |
| L612 | **the vocabulary sweep** | **100** | **exit 0, 100 lines** |

Note on L388: my harness counts stdout lines from the fence as a single stream and
this fence's two hits sit on one output line under its own pipeline shape; both
prose-locator hits are present in the output and are the two `e2e/smoke.spec.ts`
test-data lines the plan names. Not a discrepancy in the plan.

**The one nonzero exit I found is an inline span, not a fence, and I rule it not a
finding.** The authoring section's quotation of expression A,
`git ls-files | grep -v '^docs/' | xargs grep -nE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+"`,
exits 123 executed verbatim because `$EXT` is unset in the span. Three things
decide it: the substitution is **named in the same sentence** ("with `$EXT` the
alternation above") and the alternation is given in the preceding paragraph, so
nothing must be invented; the **runnable form is fenced** in Task A2 Step 1 and
executes clean; and supplying `$EXT` exactly as the sentence directs reproduces the
stated figure - I ran it and got **1 line**. That is a quotation with a named
substitution, not the class the clause targets, which is a document omitting what
the reader would have to guess. The distinction is the same one that makes the
vocabulary sweep's old form a defect and this one not: there the narrowings were
neither stated nor derivable, here the substitution is stated and derivable.

### The two stale heading counts: both fixed, recounted independently

I counted sub-bullets mechanically rather than reading the headings.

| step | stated | measured sub-bullets |
|---|---|---|
| A1 Step 3, "verification, five checks" | 5 | **5** |
| A2 Step 4, "verification, six checks" | 6 | **6** |
| A3 Step 7, "verification, eight checks" | 8 | **8** |
| A4 Step 5's cross-reference, "the four Step-4 checks above" | 4 | **4** top-level bullets in A4 Step 4 |

The fourth row is one my instrument surfaced that no finding named: a count over an
enumeration in a *different* step, which is the same shape as the two that went
stale. It is correct.

### Half B, rebuilt wider: **no additional disagreement**

The author's method takes a number followed within three tokens by a *lowercase*
noun. Mine widens on three axes so a head its method cannot reach shows up: the
noun may be capitalized, backticked or bolded; the window is five tokens; and the
number may follow its noun as well as precede it. Amendment and self-review
sections are tagged and excluded from adjudication, because a stale token in a
record of its own repair is deliberate.

| | measured |
|---|---|
| quantity heads found | **431** (author: 463 under its own grouping) |
| heads with a disagreement to adjudicate | **93** (author: 147) |
| real disagreements among countable-artifact nouns | **0 beyond those already adjudicated** |

Every countable-artifact head with multiple values resolves to a different subject
rather than a contradiction: `files` at 2/3/4/5/8 counts five different file sets;
`hunks` at 6/8 has the 8 inside the correction record; `line` at 1/2/80 mixes a
count, an overlap and a column threshold; `(rev)line` at 1/4/71/73 and `(rev)run`
at 1/2/3 are line numbers and run labels, not counts. The `checks` head at 4/5/6/8
is the one that matters and all four values are now correct.

### The remaining confirmations the dispatch asked for

**`ledger-lint` is 555.** A fourth data point after 548, 550 and 552. The unfencing
decision has now been validated four times inside a week, and the plan states the
invariant as the exit code plus the summary line's shape, which I re-verified holds.

**Nothing was compressed.** Self-review plus amendments: 2176 -> 3527 -> 5141 ->
**6633 words**, i.e. 10% -> 12% -> 17% -> **21%** of the document. The
recommendation is recorded in Amendment 3 as a close action and visibly not acted
on, exactly as the dispatch says.

**Re-verified after this round's edits, because a round that edits a plan can break
what an earlier round approved:** all twelve fenced replacement OLD strings still
occur exactly once in their target files and no NEW string is already present
(0/12 failures); extracting the replacement pairs from this revision of the plan
and matching them against the tree gives twelve unique hits plus the README
insertion, which is correctly absent because it is inserted rather than replaced;
and the acceptance map is still 37 rows in its stated split (W1=13, W2=6, W3=10,
W4=3, W5=5), contiguous, no duplicates, with both stated totals agreeing.

### The one figure that still does not reconcile - Minor, non-gating

**The `362 spans and 113 fenced lines` markup statistic.** Measured at `1e7b062`,
where the sweep ran, my extractor gives **381 spans and 116 fenced body lines**.

- The **fenced-line** difference (116 vs 113) is small and consistent with the
  stated reason, fence-line treatment.
- The **span** difference (381 vs 362) is **not** accounted for by the stated
  reason. Double-backtick handling cannot explain 19 spans: the document contains
  exactly one double-backtick span with whitespace, and stripping double-backtick
  spans moves my count from 381 to 380. Distinct span texts are 248, so
  de-duplication is not it either. Something else in the extractor's rules accounts
  for the 19, and the document does not say what.

**Why this does not gate.** It is a statistic about the author's own instrument,
not a claim about the tree; the plan already says so; and no figure, check or step
consumes it. The proportionate resolution is the one I recommended two rounds ago
and which was half-taken: state the extractor's actual rules, or drop the number
and keep the sentence. Worth doing at the close, not worth a round.

---

## Ruling on the named residual

**The residual is acceptable for this artifact. It does not want a bounded walk,
and naming it rather than closing it is the correct treatment.** Four reasons, and
one condition under which the answer changes.

1. **The residual cannot reach execution.** Half B is blind to a figure stated at
   exactly one consumer site. But every figure a *task* acts on is in an acceptance
   row or a step, and every one of those is paired with a runnable expression - I
   executed all of them this round and each returned its stated figure. So the
   residual is confined to narrative figures that no step consumes, where the worst
   outcome is a wrong sentence rather than a failed verification.
2. **The cost is disproportionate to the class.** My own scan puts 338 of 431 heads
   in the single-value set. A per-figure walk over that set would cost more than the
   defect it can find, and scale-appropriateness is a house principle, not an
   excuse - the same principle that put four documentation tasks in one worktree.
3. **The residual shrinks through use rather than through a sweep.** The two clauses
   now in force mean that the moment a single-consumer figure acquires a second
   consumer, or acquires an expression, it becomes visible to Half B or Half A
   respectively. A walk now buys a snapshot; the clauses buy a ratchet.
4. **Naming a limitation with its reason is the standard this plan has spent four
   rounds learning.** Every defect I found in rounds 1 to 3 was a completeness claim
   that was not true - a control that could not fire, an enumeration not re-run, an
   expression whose figure came from a different invocation. A stated residual is
   the opposite of that failure, and replacing it with an unbacked "we walked
   everything" would be the failure returning one level up.

**The condition that changes the answer, stated so the boundary is usable:** when a
single-consumer figure becomes load-bearing for an owner decision, it earns a
bounded walk regardless of the residual. That mechanism already exists and has been
exercised twice in this plan - the blast radius is exactly such a figure, and it got
an independent walk in round 2 and a tightened restatement in round 3. So the
residual is bounded by an escalation path, not merely tolerated.

---

## The two judgements I was asked for

### The table-cell repair that bit back: the conclusion is right

The author removed the table escape, the table render broke, so the escape was
load-bearing - and it concluded that the right fix was to stop restating the
expression in a cell at all. **I concur, and I would rule the same way on the
general case.** Inside a markdown table cell two requirements are in genuine
conflict: an alternation bar must be escaped to render, and an escaped alternation
does not execute. There is no form that satisfies both, so any expression in a cell
is either mis-rendered or non-executable. Escaping trades a render defect for an
execution defect, which is the trade the clause exists to refuse. Removing the
restatement dissolves the conflict instead of choosing a side, and it is what
say-once already prefers: the expression is stated once, in a fence, where it is
executable, and the cell points at it. I verified correction row 6 now does exactly
that.

### Is the clause being applied more widely than the artifact warrants?

**The application was warranted; the recording is the cost, and it is now the
larger of the two.**

On the application: this round's yield defends it. Four figures moved and **three
were not executable as written at all** - a pathspec that lived in prose rather than
in its fence, two expressions with no target that would have read stdin, and a
table-cell expression whose pipe escape made it match a literal backslash and return
0 against a file with three real hits. A re-run cannot surface any of those, because
retyping supplies what the document omits. That is a real class, and executing the
document's characters is the only instrument that sees it. I executed 122 strings to
check the claim and it held.

On the cost: the recording has grown every round while the severity of what it finds
has fallen. Two blocking findings, then three Important, then two Important and a
Minor, then one Minor about a statistic nothing consumes - against meta-text going
10% -> 12% -> 17% -> 21% of the document, +1492 words this round. The marginal defect
is now cheaper to leave than to hunt, which is the definition of a converged loop.
**So: do not run the clause again on this artifact.** It has done its work. The close
action that moves the per-finding narration to the SDD scratch and the journal is
recorded and should be executed there, which returns the contract to carrying its
decisions and the audits that must re-run against it.

I recommend removing no guard, check, enumeration or test. Every finding across four
rounds was a correction or an addition, and I ran rather than weighed the premises
behind each passage that concluded something was unnecessary - the no-new-test
claims in A1, A2, A3, A4 and B1, and the no-permanent-checker decision in A4 Step 5,
which rests on a recorded house decision I verified at the ROADMAP.

---

## Harvest for the controller

1. **"Execute the document's text" is a distinct instrument from "re-run the
   expression", and this round measured the difference.** Three of four moved figures
   were not executable as written, and no re-run could have surfaced them because a
   re-runner supplies the missing target, pathspec or variable from context. Worth
   ledgering as its own pattern with the yield attached, because the cheaper-sounding
   sibling ("re-run the expression") is what a future agent will reach for.
2. **A markdown table cell cannot hold an executable expression.** Escaping to
   render breaks execution; not escaping breaks the render. The general rule is to
   state the expression once in a fence and have the cell point at it. This is a
   clean, general, mechanically checkable house rule and it came out of a repair that
   bit back.
3. **The residual's escalation path is the reusable part**, not the residual itself:
   a figure invisible to a mechanical sweep earns a bounded walk when it becomes
   load-bearing for an owner decision. That is what happened to the blast radius
   twice, and it is the shape that keeps a stated limitation from becoming an excuse.
4. **`ledger-lint`'s fourth data point (548 / 550 / 552 / 555).** The unfencing
   pattern now has an unusually strong evidence trail for something that began as a
   judgement call.
5. **Convergence has a measurable signature and this loop shows it**: severity of
   findings falling monotonically across four rounds while the artifact's meta-text
   share rises monotonically. Both series are cheap to compute and together they are
   a better stopping rule than "the reviewer found nothing", which never happens.
   Worth recording as process data for the automated-department work, since knowing
   when to stop a fix loop is exactly the judgement a controller has to make without
   a human in the room.
