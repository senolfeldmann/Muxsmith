# Task 1 delta verdict (Plan 12): the fix round on the report's evidence

Delta re-review by the reviewer who wrote `task-1-verdict.md`. Scope confirmed
before grading: HEAD is `f083bc215d251b5bc4bb07367150ad6375b6d5bc`, `git status`
is empty, `git diff --stat HEAD -- docs/` is empty, and commit `b381675` is
unamended - so the artifacts I approved are byte-identical to what I graded.
The only modified file is `.superpowers/sdd/plan-12/task-1-report.md`, which
grew from 288 to 949 lines.

Settled non-findings from the original verdict are not re-litigated. The six
Minor findings the fix round did not touch (M-2 through M-6) stand as recorded
and were never in this round's scope.

**All three findings: ADDRESSED.**

---

## Method

The fix round's whole claim is that its evidence can now be re-run rather than
re-derived, so the review is the re-run. I executed **every command the report
pastes**, copied from the document rather than retyped, and diffed each result
against the output pasted beneath it. Where a command writes a scratch fixture I
redirected it to a path under my own scratch directory, never
`/tmp/ordinal-probe.md` or `/tmp/typo-probe.txt`, so no control was satisfied by
executing the fixer's own file.

Every pasted output reproduced exactly. Nothing below is a re-derivation.

| what | reproduced |
|---|---|
| A1, ordinal words in list position | hits on lines 20 and 145, the two pasted |
| A2, the numeric form | 39 matches; `39 decision <n>` after normalisation; lines `16 23 39 67 70 103 107 114 120 132 139` |
| A3, the attribution awk | all ten per-slot subtotals identical; total 39 |
| A4, the suffixed form | exit 1, no hit |
| A-control a/b/c | all nine nouns fire individually; `3rd` fires; the word half fires - on my own probe path |
| B1, the typography scan | exit 1 over both touched files |
| B-control 1 and 2 | thirteen members, thirteen distinct fires - on my own probe |
| C-1, the nine-cell per-passage matrix | all nine cells identical |
| C-1 control over section 8.2 | all three expressions fire, 1 hit each |
| "Also re-run", line-number grep + control | exit 1 on the new file; `16:(runtime.rs:206, Parse error) ...` on plan-5.7 |
| Scope section | both `docs/` commands empty; the `HEAD~1` control reports the ledger commit's two files |

---

## Verdicts per finding

### I-1 (the two narrated absence checks): ADDRESSED

Both checks now carry the expression, its live output, and a control fired
against a known-present case. What raises this above a bare compliance pass is
that both controls **discriminate at the level the original instruments failed
at** - membership, not pattern validity:

- **A-control** fires one line per *enumerated noun*, nine of them, not one
  representative. That is the exact discipline
  `a-search-whose-terms-come-from-memory-produces-a-false-absence` asks for
  ("derive each set from the artifact separately"), and it is what makes A2's
  widening from five nouns to nine a measurement rather than a gesture: the count
  did not move, which is the answer to "is the enumeration complete enough".
- **B-control 2** fires each of the thirteen denylist codepoints on its own,
  which is the only way to show that the range `\x{2010}-\x{2015}` covers all six
  dashes rather than one. A single class-level fire would have passed with five
  of them missing.

The strongest single item in the round is not a check but a disclosure: the A3
note recording that the fixer's **first** awk used `next` after matching a
bold-slot line, silently skipped every reference sitting on a `**Rationale:**`
line, and reported 8 where grep reported 39 - caught only because two
instruments disagreed over one file. That is the failure mode the round exists
to close, found in the round's own tooling and written down instead of quietly
fixed. My independent python implementation agrees with the corrected awk on all
ten subtotals, so the repair is sound as well as disclosed.

### I-2 (the bolded generalisation): ADDRESSED

Corrected at every site that asserted it, and each correction is factually
right - I re-measured all of it. C-1's nine-cell matrix reproduces, its control
over section 8.2 fires on all three expressions, and the restated conclusion
("only one of them is unreachable by the sweep, not both") is what the
measurement supports. Completeness of the site sweep is ruled under Q2.

### M-1 (the "one place an item ordinal survives" clause): ADDRESSED, and the correction is more accurate than my finding was

See Q1. The corrected fact - 39 references on 11 lines, three of them on two
lines inside a `Rejected alternatives` slot - is what my own third measurement
returns. C-2 also draws the right distinction, which my finding stated less
sharply: the *artifact* is not defective, because the brief's fixed property is
about identifying a rejected alternative by its position in its list, and A2
finds zero occurrences of `alternative <n>`, `bullet <n>`, `option <n>` or
`item <n>` anywhere in the file, A4 finds zero suffixed ordinals, and A1's two
hits are false positives. The narrow property holds and is now measured instead
of recalled.

---

## The three questions

### Q1 - the discrepancy with my own verdict: THE FIXER'S FIGURE IS RIGHT. Mine was not.

Third measurement, with an instrument written for this delta review in python,
tracking the enclosing decision and bold slot with a different implementation
from the fixer's awk (it never consumes the slot-marker line, so it cannot
inherit the `next` blind spot the fixer disclosed):

```
TOTAL refs: 39
distinct lines: [16, 23, 39, 67, 70, 103, 107, 114, 120, 132, 139]
  D106  Rationale                 6
  D106  Rejected alternatives     1
  D107  Rationale                 6
  D108  Decision                  1
  D108  Rationale                12
  D109  Decision                  3
  D109  Rationale                 2
  D109  Rejected alternatives     2
  D110  Decision                  1
  D110  Rationale                 5

In a Rejected-alternatives slot: 3 refs on lines [23, 120]
    (23,  'D106', 'Rejected alternatives', 'decision 3')
    (120, 'D109', 'Rejected alternatives', 'decision 9')
    (120, 'D109', 'Rejected alternatives', 'Decision 9')
```

All ten subtotals agree with A3's summary. Line 120 is D109's third rejected
alternative, "Two sequential prompts when a run and unsaved changes coincide",
whose body carries "not superseded by decision 9" and "Decision 9 addresses a
state that changed between the read and the confirm".

**The correct figure is two lines carrying three references.** My verdict's M-1
wrote "including one inside a rejected-alternatives bullet" and cited D106's. I
intended that as an existence citation - an example of the shape - and not as a
subtotal, but it is loose enough to be read as one, which is how the fixer read
it, and the fixer was right to measure rather than defer. I record my own
sentence as the imprecise one.

Worth naming, because it is the same shape as the finding it belongs to: M-1
criticises a subtotal written from recall rather than from an attribution pass,
and its own subtotal came from a spot-read rather than an attribution pass. The
fix round ran the pass I should have run. Recording the disagreement instead of
smoothing it is exactly the right handling, and it is the reason the correct
figure is now on record at all.

### Q2 - completeness of the correction sweep: COMPLETE.

I swept the report for the affected fact myself, twice, with two different
vocabularies, and the second sweep carried a fired control (the same expression
against the spec, where the terms are known present, returns 2).

Sweep 1 (`advance`, `known in advance`, `two hits`, `neither`) and sweep 2
(`section 8.3`, `section 11`, `non-goals`) agree: **exactly three sites assert
the fact**, and each carries an adjacent `[CORRECTED 2026-07-30, fix round.]`
marker:

| line | site | corrected at |
|---|---|---|
| 3 | the status paragraph's "The single concern is a surfaced observation about Step 4's two advance-named hits" | 5 |
| 212 | Step 4, "The two advance-named hits" - the bolded sentence itself | 214 |
| 294 | "Surfaced, not resolved" item 1's heading | 296 |

No fourth site. The two remaining mentions of these passages (lines 167 and 172)
are Expression 1's classification entries, "Section 8.3's help mechanics
sentence: **consistent**" and "Section 11's non-goal `Locales beyond English and
German`: **consistent**" - they are the true evidence the false claim
contradicted, they were always correct, and leaving them untouched is right.

Six markers in total across three findings, and each of the other three sits at a
site that asserted an affected fact: line 46 (fixed property 2's ordinal
narration, I-1), line 97 (the "one place an item ordinal survives" clause, M-1),
line 107 (the `**Typography.**` paragraph, I-1). Every asserting site is covered
and no marker sits on a site that did not assert something.

**On the method, which is the part worth carrying forward.** My finding
enumerated one site because that is where I found the fact. The fixer swept for
the *fact* and found three. That is the correct reading of a finding's
enumeration - the place a sweep starts, not the place it ends - and the two extra
sites are not incidental: the status paragraph is the first sentence a reader
meets, and the "Surfaced, not resolved" item is the one the controller acts on.
Had the fix worked my file list, both would still be wrong and the surfaced item
would still be handing the controller a question about two passages when only one
of them exists.

### Q3 - the thirteen denylist characters in the report: ACCEPTABLE AS EVIDENCE FORM, with one rider.

Ruled on three measurements rather than on the declaration.

**Containment.** All 27 glyph-bearing lines sit inside fenced blocks - line 653
(the probe's `printf`), 665-677 (control 1's output), 690-702 (control 2's
output). Zero occurrences in prose, measured with a fence-state tracker. The
report's declaration under check B is accurate as written.

**Exposure.** No gate part scans for typography glyphs. BUILDING.md's eleven
parts are six cargo/`RUSTDOCFLAGS` invocations, four pnpm invocations and
`ledger-lint.py`; none of them reads `.superpowers/` at all - `grep -rn
superpowers` over `scripts/`, `.github/` and `package.json` returns nothing,
fired against `docs/`, where the term is present, to confirm the empty result is
a measurement. A scanner-detection expression (codepoint escapes in three
notations, plus `em-dash`, `en-dash`, `smart quote`, `curly quote`, `typograph`)
returns nothing over `scripts/`, `.github/`, `BUILDING.md`, `package.json` and
the eslint config, and fires 35 times against this report, so that absence is a
measurement too. On top of which `.superpowers/` is gitignored, so the file is
outside every tracked-file sweep by construction.

(My 35 differs from the coordinator's 11 because the expression is mine and
carries more alternations. The conclusion is the same and the two figures do not
conflict.)

**Necessity.** The per-member control is what separates "the class covers all six
dashes" from "the class covers one of them", and that discrimination is real -
five missing members would pass a class-level fire. The evidence earns its cost.

**Rider, and it is the one thing in this round I would have done differently.**
The report's stated reason for accepting the glyphs is that the alternative,
"writing the probe in escape notation - would have turned the control back into a
narration of itself". That is true of the probe's *content* and false of the
report's *rendering* of the run. A glyph-free form of the identical control
exists: leave the probe file exactly as written, with real characters on disk and
a real run over it, and paste `grep -cP` per member instead of `grep -nP`, so
each line reads `U+2010 -> 1` rather than `U+2010 -> 1:U+2010 HYPHEN [<glyph>]`.
I ran precisely that against my own probe and got thirteen distinct fires with no
denylist character anywhere in the output - same discrimination, zero exposure.

Not a defect in this round: the tradeoff was reasoned about and declared, and a
declared tradeoff is what the rule asks for. But the cheaper form should be the
default next time, because what the current form leaves behind is a self-declared
exception whose only trigger is a paragraph someone has to read - and a trigger
that depends on being noticed is the weakest kind there is.

---

## New breakage introduced by the fix round

One item, low severity. A precision slip in a corrected sentence, not a factual
error, and the round's own detail section states it correctly.

**N-1 (Minor). The status-paragraph correction mislabels M-1's defect class.**
Location: report, line 5, the `[CORRECTED]` block under the status paragraph.

It reads: "Two further defects were found in this report's evidence, both in the
same class (a check narrated instead of pasted)." That label fits I-1 exactly -
two checks, both narrated. It fits M-1 only partly: M-1's root cause is an
incomplete enumeration *inside* the instrument, which is
`a-search-whose-terms-come-from-memory-produces-a-false-absence`, a different
ledger entry from `design-empirical-claims-reproducible`. The round's own C-2
gets this exactly right - "What actually failed was the instrument, exactly as
M-1 says: an expression enumerating ordinal WORDS cannot see the numeric form" -
so the first paragraph a reader meets collapses two classes that the fix round
itself distinguishes fifty pages later. The narration is what let M-1's claim
survive review; it is not what produced it.

Nothing downstream turns on it, and no measurement is affected.

**Nothing else.** Every corrected sentence I checked is true as rewritten. Every
control discriminates at the level its finding failed at. Every pasted output
matches the command above it on a verbatim re-run. No control was found that
passes without measuring anything, and no corrected passage was found that is now
wrong in a different way.

---

## Observations on material this round did not touch

None new. M-2 through M-6 of the original verdict stand unchanged and were
correctly outside this round's scope, as were the three adjudication verdicts and
the harvest.
