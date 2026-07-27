# Plan-7.5 close fix: independent review verdict

## VERDICT: NEEDS FIXES

One Medium finding, narrowly scoped: a measured count that is wrong by one and
has propagated into a commit message and into the report's central verification
table. The change's *content* is correct and complete; nothing needs re-editing
in a product file. Everything else in the seven dimensions passes.

---

## Scope and method

**Graded set**, established with `git log 7302e1b..HEAD` and not taken from the
report: `d5a6470`, `ddb707a`, `eb4608b`. Salvage `8e2c044` excluded per brief;
verified it touches nothing outside `docs/process-journal/artifacts/plan-7.5-sdd/`
(31 files, all additions).

**Union diff** `7302e1b..eb4608b` excluding the salvage directory, measured:
6 files, 15 insertions, 6 deletions. Per commit: `d5a6470` 2 files 11+/2-,
`ddb707a` 2 files 1/1 each, `eb4608b` 2 files 1/1 each. This reproduces the
report's three stat claims exactly.

**HEAD kept moving during this review.** At review start HEAD was `eb4608b`. At
18:37:20 `ecab53a` landed from a concurrent writer ("ledger-lint: keep the loader
constructor inside the parse try", 1 file: `scripts/ledger-lint.py`), and by
18:43 HEAD was `bcb67f3` ("BUILDING: add the cross-target clippy gate part") with
two plan-8 documents newly modified in the working tree. None of it intersects
the graded surface, and I treat all of it as controller-side and out of scope.
**Every verification below is pinned to `eb4608b`, not to `HEAD`.** See open
question Q1.

**Environment.** All checks run with `git grep` or `command grep`; the shell's
`grep` function respects `.gitignore` and would silently skip `.superpowers/`.
Every absence-result below is paired with a control that I watched fire.

---

## Findings

### MEDIUM 1: the English cross-reference census is off by one, and the wrong number is in commit `eb4608b`

**Claim under review.** Report addendum 2: "total cross-references 13 | 13",
"conformant (capitalized title) 11 | 13", and "Controller's census reproduced
independently, and it matches exactly." Commit `eb4608b` message: "Eleven of the
thirteen cross-references did."

**Measured.** The `help/en/` surface carries **14** `see the ... topic`
cross-references, not 13. Pre-edit: **12** conformant, 2 lowercase. Post-edit:
14 conformant, 0 lowercase.

The uncounted site is `help/en/editor-tracks-rules.md:7`:

```
... only donor tracks follow in rule order - see the Unmatched (tracks) topic.
```

Its target `help/en/editor-tracks-unmatched.md:1` is `# Unmatched (tracks)`, so
the site is fully conformant, and conformant for the ruled reason: entry
`help-topic-h1-scheme` (`docs/conventions.yaml:1035`) says "Prose cross-references
retain the parenthetical only where the bare label collides across sections",
and `Unmatched` collides with `# Unmatched (attachments)`.

**Why both censuses missed it.** A pattern that stops at `)` cannot match a
title containing a parenthesis. I hit the identical defect on my own first pass
and caught it only because the German census returned a parenthetical title
(`siehe das Thema Nicht zugeordnet (Spuren)`) that the English list had no
counterpart for. Demonstrated, not asserted:

| pattern over `help/en/` | hits |
|---|---|
| `see the [^)]*topic` (parenthesis-blind) | 13 |
| `see the [^;]*? topic` | 14 |

**Consequence.** The edit itself is unaffected: the two sites needing change
were the right two, and the 14th needs nothing. What is affected is a normative
count in a durable record. This is exactly `proc-sweep-surface-completeness`
(`docs/process-conventions.yaml:597`): "A firing positive control proves a sweep
PATTERN is valid, never that its SEARCH SURFACE is complete". The entry's own
newest occurrence is the same shape, recorded `violated-corrected`:
"joblog-datebomb-fix verdict Medium ... the sweep claimed 33 hits / 3 files
workspace-wide, actual 34 / 4 ... benign hit, but the figure propagated into the
commit message and progress ledger". Same day, same class, and the house
precedent corrects rather than waives.

Second defect in the same claim: **"reproduced independently" is not true of a
reproduction that inherits the original's blind spot.** The controller's census
and the implementer's agree at 13 because both patterns exclude parentheticals.
Agreement between two instances of the same method is not corroboration.

**Fix (one number, no product edit).** `eb4608b` is unpushed. Either amend its
message to "Twelve of the fourteen cross-references did", or leave the commit
and record the correction at the close; and correct the report's addendum-2
table to 14 / 12 / 2 and drop the independence claim. The controller's own
census needs the same correction wherever it is recorded.

### LOW 2: the plan document carries a fifth superseded-wording site the brief's exception clause does not name

The brief exempts "the plan's coverage-map rows that name the clause by its old
wording". Measured, the plan document has five sites matching
`invalid until filled|legal down to zero rules per 4\.5`:

| line | what it is | covered by the brief's exception? |
|---|---|---|
| 71 | coverage-map row (D65) | yes |
| 75 | coverage-map row (D69) | yes |
| 367 | the quoted mandate block | yes (explicitly protected) |
| 372 | the new supersession note | n/a |
| **400** | **Step 5, the unabbreviated-transcription check** | **no** |

Line 400 reads `in particular amendment 1's parenthetical "- invalid until
filled, announced by validation -"`. Leaving it untouched is the **correct**
disposition, and for the brief's own stated reason: it is the wording Task 4 was
graded against, and rewriting it would falsify the fidelity record the whole edit
exists to preserve. But it is a third old-wording carrier the enumeration did not
name, 28 lines below the note that covers the drift, and the implementer's sweep
did not surface it even though it surfaced the structurally identical off-by-one
on the citation side (adjudication (a)). Same defect class, caught once, missed
once. No edit owed; recorded so the close does not read line 400 as an
unfinished replacement.

### LOW 3: the untouched evidentiary sites get "left bare" where the house rules "qualifier plus live twin"

Dimension 3 invites disagreement, so here is mine, and the side I land on.

The **don't-rewrite half of the boundary is right**, and it is the ruled
behaviour: `code-comment-line-citations-drift` (`docs/conventions.yaml:1013`)
discriminates two classes and says an evidentiary record "is never re-pointed
(that would falsify the record)". The design's two citations were the only live
descriptive pointers on the surface, and they moved. Correct cut.

The **leave-bare half under-delivers what the same entry rules.** Its evidentiary
handling is not silence: the record "gets a scoping qualifier naming the staling
event, the original span verbatim, and a pointer to the live twin where one
exists". A live twin exists for every untouched site:
`docs/process-journal/artifacts/plan-7.5-sdd/progress.md` and
`.../design-review-round-1.md` are both in the 31-file salvage. Post-edit state,
measured (6 tracked non-salvage hits for `.superpowers/sdd/plan-7.5`):

| site | qualifier + live-twin pointer? |
|---|---|
| `docs/ROADMAP.md:309` | yes, in the controller's uncommitted ROADMAP text ("FIRED AND CONSUMED 2026-07-27 ... keep the pre-salvage path deliberately") |
| plan doc `:415` close-actions bullet | no, but the ROADMAP entry above names it |
| plan doc `:5` | **no** |
| plan doc `:31` | **no** |
| two handoff snapshots | no; frozen snapshots, correctly out of reach |

Lines 5 and 31 are the sharp case, because they are **present tense**: "The
tracker is `.superpowers/sdd/plan-7.5/progress.md`" and "Progress lives in
`.superpowers/sdd/plan-7.5/progress.md`". The brief's justification is "they
state where the tracker lived while the plan executed, which remains true", which
is a past-tense reading of two present-tense sentences. A public-repo reader gets
a present-tense assertion pointing at a path that is not in the repo, with a
salvaged twin sitting untold three directories away.

**Where I land:** the boundary is substantively right and I would not re-point
anything. But "untouched" is not the house's evidentiary handling, and the two
present-tense sentences are the sites where the gap is visible to an outside
reader. Controller-side call, not an implementer defect: the brief specified this
boundary and the implementer complied exactly. See Q2.

### INFO 4: the plan's supersession note separates a sentence from its antecedent

At `docs/superpowers/plans/...:372` the note now sits between the quoted block
and the sentence that refers to it, which reads "The wording above is the
design's verbatim text, line-wrapped for this plan" (line 374). "The wording
above" now most proximately names the note. The design has no equivalent problem
(its follow-on is "The rest of the item ... is unchanged"). Brief-induced: the
brief mandated "immediately after each of the two quoted blocks". Cosmetic.

### INFO 5: byte-identity was proven twice, once by a method the house deprecates

The report's proof 2 diffs fixed line ranges (design 794-805, plan 359-370).
`proc-wrapped-prose-quote-grep` (`docs/process-conventions.yaml:564`) names
exactly this as the anti-pattern: "block re-extractions key on structural anchors
like code fences (fixed line ranges produce phantom deltas once surrounding edits
shift them)", with a recorded occurrence of a phantom one-line delta from a
fixed-range fence extraction. Not load-bearing here: the report's proof 1 (the
whole diff contains exactly two removal lines, both outside either block) is the
content-anchored form and carries the claim on its own, and the implementer did
fire-control the range method as the entry's second sentence demands. Recorded as
method drift, not as a defect in the result.

---

## Dimension results

**1. Fidelity: PASS.** Verified against `7302e1b`, not against the report.

- Mandate blocks byte-unchanged. `diff` of design 794-805 and plan 359-370
  between `7302e1b` and `eb4608b`: empty for both. Control fired twice, so the
  empty result means identity and not a broken comparison: the same invocation
  on the re-pointed citation line (`7302e1b`:836 vs `eb4608b`:843) printed the
  change, and a deliberately one-line-shifted range on the plan block also
  reported a difference.
- Citation re-pointing is prefix-only. The union diff's only two removal lines in
  the design are the two citation lines; file names, section names and
  surrounding text are unchanged, and neither site carried a `:line` suffix.
- Supersession note text is the brief's text verbatim at both sites. Normalized
  (blockquote markers and the design's 3-space indent stripped, hard wraps
  joined) and diffed against the brief's lines 91-96: design identical, plan
  identical modulo one trailing space my own join introduced. Control fired
  against a single-character mutation of the reference.
- Nothing else changed. The union diff over the whole tree minus the salvage
  directory is the 6 files listed above, nothing more.
- Typography: 0 hits for em/en dash, curly quotes, ellipsis, NBSP and Unicode
  minus across all 15 added lines; the detector fired against a control string
  carrying all four. No trailing whitespace on any added line (my first
  trailing-whitespace pattern was itself broken, `[ \t]` inside a POSIX bracket
  expression matches the letter `t`, so it flagged a line ending in "commit";
  re-run with `-P` and fire-controlled, result NONE). German letters intact in
  both edited DE lines, verified with `cat -A`.

**2. Completeness against the named surface: PASS with Medium 1 and Low 2.**
Surfaces I searched, named explicitly: (a) all tracked files for
`.superpowers/sdd/plan-7.5` and for `sdd/plan-7.5`, excluding the salvage
directory; (b) all tracked files for `invalid until filled` and
`announced by validation`; (c) `help/de/` and `help/en/` for every cross-reference
form; (d) the whole tracked tree for both old help strings; (e) `src`,
`src-tauri`, `e2e`, `scripts`, `crates` for any code-side copy of the
cross-reference sentence.

- (a) 8 sites pre-edit, 6 post. `sdd/plan-7.5` without the leading dot: 0
  additional.
- (b) exactly two non-journal documents carry the pre-ruling block, as the brief
  claims, and the current v1 spec line 375 carries the post-ruling wording the
  brief quotes. Both verified by opening the lines.
- (d) both old strings return 0 outside frozen process-journal records
  (`plan-7-sdd/review-0fea107..cc0e6d7.diff`, `plan-7-sdd/task-08-verdict.md`,
  `plan-7.5-sdd/wording-fix-verdict.md`), which are correctly out of reach per
  the provenance-exclusion practice recorded under `proc-sweep-surface-completeness`.
  Control: the two new strings return 2 each.
- (e) 0 hits; control fired (`suggestion` is present in 8 files across those
  dirs). No companion code or fixture change is owed.

**3. The re-pointing boundary: PASS with Low 3.** Both re-pointed targets are
tracked (`git ls-files --error-unmatch`), so the new citations resolve for a
public-repo reader. The precedent claim holds as stated: `9d01862` is one file,
3 insertions / 3 deletions, preserves the `:60,64` suffix, and the plan-7 design
carries 0 remaining `.superpowers/sdd` citations today.

**4. German correctness: PASS.** `siehe das Thema Vorschlagskarte` matches the
target h1 `help/de/batch-suggestion-card.md:1` = `# Vorschlagskarte` byte for
byte, and matches the ruled form in `help-topic-h1-scheme`: the suggestion card
is on that entry's closed exemption list of bare h1s, its label is unique, so it
leads bare with no parenthetical. It matches the corpus: 11 other German
cross-references, all `siehe das Thema <Titel>`, none with a preposition. Both
edited sentences read correctly in context (a bare apposition after "das Thema"),
no double space introduced, one token deleted per line and nothing added.

**5. House conformance:**

| entry | result |
|---|---|
| `proc-verification-step-must-be-falsifiable` | **held, well.** Every absence claim in the report has a pre-edit non-zero control I re-ran and reproduced. Its stated reason for not controlling the new patterns ("a non-zero hit self-validates the pattern") is correct. |
| `proc-sweep-surface-completeness` | **violated** (Medium 1): a named-surface count claim whose pattern did not cover the named surface. |
| `proc-wrapped-prose-quote-grep` | **method drift** (Info 5), result unaffected. |
| `code-comment-line-citations-drift` | two-class cut applied correctly; evidentiary handling incomplete (Low 3). |
| `help-topic-h1-scheme` | the landed text conforms in both locales. Not cited by the implementer, which is how the parenthetical case went unmodelled. |
| `proc-noninteractive-file-ops-in-agents` | not engaged; no mutate-and-restore was needed, pre/post counts served as the control. |

**6. Commit hygiene: PASS.**

- Unsigned: `%G?` = `N` on all three.
- Trailer `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>` present on all
  three.
- Explicit staging: the three commits touch exactly 6 paths, all intended. The
  index is empty (`git diff --cached --name-only` returns nothing) and
  `git diff --cached HEAD` for the three controller paths is empty.
- **The controller's files were neither staged nor committed.**
  `git log 8e2c044..eb4608b -- docs/ROADMAP.md docs/conventions.yaml
  docs/decision-ledger.yaml` returns nothing; all three are still ` M`
  (unstaged) in the working tree. `BUILDING.md` joined them from a concurrent
  writer after `eb4608b`.
- Not pushed: `origin/master` is still `7302e1b`. No tags, no stashes, no new
  branches (the six `plan75-*`/`plan8-*` heads predate this range).
- The three-commit split instead of the brief's "one commit" is not a deviation:
  `d5a6470` is exactly the brief's one commit, and the other two answer addenda
  that arrived afterwards. The surface argument in the report is sound.

**7. Adjudication:** below.

---

## Adjudications

**(a) The eighth tracked site. Implementer correct; leaving it untouched was
right.** Reproduced at `8e2c044`: `git grep -F '.superpowers/sdd/plan-7.5'` over
tracked files excluding the salvage directory returns **8** hits, against the
brief's predicted 7, and the extra one is
`docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md:413` (now
:415), the close-actions "Salvage re-pointing" bullet. Its class matches the
ROADMAP trigger entry the brief itself exempts, the controller's own ROADMAP text
has since ruled the same disposition for both, and the brief's standing
instruction was report-not-edit. Caveat carried in Low 3: "untouched" is not the
same as the ruled evidentiary handling.

**(b) "Three files" against a two-file enumeration. Implementer correct.** The
brief's verification item 1 says "exactly four changed regions across three
files" and then enumerates two files and four regions (design x3, plan x1).
Measured, `d5a6470` is 2 files, 11 insertions, 2 deletions, four regions. The
brief's count is internally inconsistent and the observed diff matches its
enumeration, not its number.

**(c) The English casing finding. Correct in substance, wrong on its numbers;
reporting rather than editing was the right handling.** The deviation is real:
`help/en/batch-suggestion-card.md:1` is `# Suggestion card` and both sites read
lowercase, against a house form that `help-topic-h1-scheme` rules explicitly.
Surfacing it was right and its premise-refutation was right: addendum 1's "the
English counterparts are ALREADY house-conformant" is false, and the false
premise is traceable, the earlier `wording-fix-verdict.md:185` called the English
pair "the bare-label form" without checking its casing. Reporting rather than
editing was correct on three independent grounds: the boundary was explicit, the
surface is shipped user-facing text where the owner rules wording, and the
implementer's own contract says to report a broken premise rather than resolve it
at the keyboard. Its dismissal of the fourth `see the [a-z]` hit also reproduces
exactly (4 lines, two of them not the ruled construction, and the German
counterpart is the only `siehe die ...` form in `help/de/`). **But** the finding's
measurement is off by one in both columns: 12 conformant of 14, not 11 of 13.
Being right about the defect and wrong about its census is the split ruling here.

---

## Open questions for the controller

**Q1. Is `ecab53a` in the graded set?** The brief defines the set as "the
implementer's commits since `7302e1b`, excluding `8e2c044`". `ecab53a` landed
during this review, nine minutes after `eb4608b` and ten after the review brief
was written, touches only `scripts/ledger-lint.py`, and matches no part of the
implementer's contract. I treated it as a concurrent controller action and
excluded it, pinning every check to `eb4608b`. If it is in fact the implementer's,
it is ungraded and needs its own pass.

**Q2. Does the evidentiary class get a live-twin pointer here or not?** Low 3.
`code-comment-line-citations-drift` rules "qualifier plus original verbatim plus
pointer to the live twin where one exists" for evidentiary records; this change
delivered "leave bare". The ROADMAP entry supplies the qualifier for two of the
four sites. Plan-document lines 5 and 31 supply nothing and are phrased in the
present tense. Either the entry's evidentiary handling has an unstated exemption
for whole-path staling, or those two lines are owed a clause. Controller's call.

**Q3. Two adjudications or three?** The brief's Dimensions section names three
lettered items; its Output section asks for "the two adjudications". I answered
three.

---

## HARVEST

**H1. A reproduction that inherits the original's pattern is not an independent
check.** The controller's census and the implementer's "independently reproduced"
census agreed at 13 because both used a parenthesis-blind pattern; the agreement
read as corroboration and was shared blindness. `proc-sweep-surface-completeness`
already covers pattern-versus-surface; what this adds is the **agreement trap**:
when a second measurement confirms a first, ask whether the second could have
disagreed. A reproduction that reuses the method can only confirm. The
independent form varies the method (I found it by cross-checking the German
census against the English one and asking why the counts differed by one), not
just the operator. Trigger is readable: you are writing "reproduced
independently" or "matches exactly" about a count someone else already measured.

**H2. Titles with parentheses are the standing blind spot of help-surface
sweeps.** Any pattern of the form `see the [^)]*topic` or `siehe das Thema
[^).]*` silently drops every cross-reference to a disambiguated topic, and
`help-topic-h1-scheme` guarantees such topics exist (it mandates the
`<label> (<section>)` form with a closed bare-h1 exemption list). Handle: sweep
the help cross-reference surface with a pattern whose terminator is the word
`topic` / the sentence end, never one that excludes `)`; and cross-check the two
locale counts against each other, since a divergence is either a real parity gap
or a broken pattern and both are worth knowing. Candidate ledger occurrence
under `proc-sweep-surface-completeness`.

**H3. Cite the governing house entry, not the corpus, when one exists.** The
implementer derived the help cross-reference house form empirically from the
other sites and got the right answer. Had it opened `help-topic-h1-scheme`, the
entry's own words ("retain the parenthetical only where the bare label collides")
would have told it that parenthetical-bearing cross-references exist, and the
census pattern would have covered them. Corpus-derived rules reproduce the
corpus's shape; ruled entries state the shape including the cases the sample
happens to miss.

**H4. Brief defects found (controller-authored, the class this review exists to
catch):**

- The review brief's "one controller addendum delivered mid-task (quoted in full
  in the implementer's report)" is wrong twice. There were **two** addenda, which
  the brief itself then enumerates as items 3 and 4, and **neither is quoted at
  all**: the report contains zero blockquote lines, and no addendum file exists
  in `.superpowers/sdd/plan-7.5/` (the only owner-input file there,
  `owner-surface-pass-inputs.md`, has mtime 12:53, before the 18:14 brief). The
  consequence is not
  cosmetic: the instruction "check that what landed is what was ruled" cannot be
  fully discharged, because the ruling text is nowhere on disk. I graded against
  the review brief's own restatement of the two rulings, which is a controller
  paraphrase of a controller ruling, so that half of dimension 4 rests on an
  unopenable source. **A mid-task addendum that changes scope should be persisted
  verbatim in the task folder before the report is written**, the same way the
  brief itself is.
- The review brief repeats the census error: "the other 11 English
  cross-references" (line 38). The wrong number originated controller-side, was
  confirmed by a same-method reproduction, and reached a commit message. Three
  layers, one measurement, zero independent checks.
- Dimensions section names three adjudications, Output section asks for two.
- The implementer's brief says "exactly four changed regions across three files"
  where its own enumeration names two, and predicts seven citation sites where
  the tree holds eight. Both caught by the implementer, both confirmed here. Two
  count defects in one short brief, alongside the census error in the review
  brief, is a pattern: **the counts in these briefs are being written from memory
  of the enumeration rather than measured off it.** The cheap mechanical fix is
  to never write a cardinal next to a list in a brief; write the list and let the
  reader count.

**H5. What the implementer did that is worth copying.** Its refusal to resolve
two broken premises at the keyboard, the pre-edit firing control on every
absence-expectation including its own typography and double-space detectors, and
the pre-emptive dismissal of the fourth `see the [a-z]` hit so the reviewer did
not have to re-derive it. The last one is a genuine reviewer-time saver and
reproduced exactly; more reports should carry a "checked and dismissed" section.

**H6. The prior wording-fix verdict predicted both halves of this change.** Its
H1 asked for "one line of disposition at plan close" on the plan doc's superseded
quote, and its H2 flagged the German `zur` outlier as "a residual owner-pass
candidate". Both were discharged here. That is the deferral mechanism working
end to end, and it is also where the English half was lost: H2 described the
English pair as "the bare-label form" without checking its casing, and that
uninspected characterization became addendum 1's scope boundary. **A harvest item
that describes a second surface in passing is asserting something about that
surface**, and it will be quoted later as if it had been measured.
