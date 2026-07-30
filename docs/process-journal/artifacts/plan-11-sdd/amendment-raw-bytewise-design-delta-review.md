# D111 delta review - the fix round, graded

**Companion to** `.superpowers/sdd/plan-11/amendment-raw-bytewise-design-review.md`
(round 1). Same reviewer, resumed. **Artifact:**
`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`, 1693 lines,
untracked. **Tree:** `6585f69`, with `docs/ROADMAP.md` and `docs/decision-ledger.yaml`
dirty from the controller's concurrent writes; the plan document is byte-identical to
round 1 (`git diff --stat` over it returns nothing), so every plan line number below
is comparable with round 1's.

**Instruments** at a path neither the author nor round 1 used:
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/d111-delta-independent/`
(`e1.txt`/`e2.txt`/`e3.txt`/`union.txt` the re-derived section-13 sets, `rpp.py` my own
reimplementation of R'', `fmt/` the rustfmt isolation control, `probe/` fresh probe
files and profiles for the exit-code matrix and the parser precondition). Round 1's
`d111-review-independent/` was not reused for anything re-measured here.

**Not re-run, per the dispatch and because it was settled:** the typed-path
differential harness. 2209 pairs over sixteen cells, zero divergences, control 17.

---

## Verdict: NEEDS_FIXES

Everything I found in round 1 is genuinely fixed, and four of the seven are fixed past
what I asked for: the exit-code correction went to a four-case matrix that voids an
inference I had left standing, R-2's repair came with a purpose-built check (R'') whose
control fires on the pre-fix text, section 13 was rebuilt from a rule rather than
patched with my table, and the rebuild step arrived with the mechanism spelled out. The
union reproduces at exactly 102/27 with my own instrument, the four VOID items the
author added beyond my 21 are all real, and E3 earns its place. Both tool-level findings
the author's own re-run produced are real and both matter more than their size. What
holds the gate is one line: plan line **626** is VOID, is named nowhere in section 13,
and sits outside the 102-line union because all three expressions are blind to a Rust
identifier by construction - the exact blindness the design documents two sections
earlier for `matcher.rs:457` and did not carry into 13.1. Per the doctrine's convergence
rule I am returning the missing **clause**, not another member, so this round cannot
widen again: add identifier forms to the expression set and the union re-derives 626 by
itself. That plus one stale self-citation is the whole outstanding list, and neither
touches the decision, the semantics, the tests or the safety property.

---

## Round-1 findings, disposition

| # | round-1 finding | disposition |
|---|---|---|
| 1 | MAJOR: section 13's override list incomplete | **ADDRESSED, and exceeded.** Rebuilt as 13.1 (rule + three expressions + measured union), 13.2 (4 in-body VOID), 13.3 (23 plan-level VOID), 13.4 (UNAFFECTED, so the rule is not over-applied), 13.5 (the executable clauses as before). 13.4 is the half I did not ask for and it is the right instinct: a plan author applying 13.3 mechanically would have swept the dated records. One residual, finding **D1**. |
| 2 | MEDIUM: optional-rule "exit 0" | **ADDRESSED, and exceeded.** Both occurrences corrected, `severity_exit` cited, the four-case matrix added, probe (A) explicitly disqualified for the no-signal claim, probe (B) added, and the word "silently" retracted with the surviving signal named. Reproduced in full below, including the success case. |
| 3 | MEDIUM: R-2 wrote "untyped value equality" | **ADDRESSED.** R-2 rewritten to keep "matched untyped" in the path sense with an explicit gloss ("the capability model is not consulted"), a dash boundary, and the type rule stated separately; the "why this wording" paragraph records the ground. The `matcher.rs:407` and R-11 uses I flagged as related are now named as deliberately retained with the reason. And a new check (R'') was built for the class rather than the instance. |
| 4 | MINOR: section 3.3's absolute | **ADDRESSED.** Bounded to non-test code outside the comparator pair and the one re-pointed call site, with the four test-module changes enumerated (R-11, R-12, T-1, T-3). The count is right and not an off-by-one: R-12 *is* T-2, so naming only T-1 and T-3 from section 5 avoids double-counting. |
| 5 | MINOR: T5's `:78` | **ADDRESSED.** T5 now reads `:74`, carries the measuring `grep -n "B-7"` and quotes the row, so the citation is checkable rather than assertable. |
| 6 | MINOR: `<surface>` placeholders | **ADDRESSED.** R', K' and the vocabulary sweep are each fenced as two complete invocations with the pathspec inline, with the plan's own line 972 and 948 cited as the precedent. The zsh note landed too. |
| 7 | MINOR: no rebuild before the post-change probes | **ADDRESSED, and exceeded.** 13.5's Step-1 bullet now names `cargo build -p muxsmith-cli` plus M1's own freshness check, and states which five exit bars leave the binary untouched and what the failure would look like ("would show the OLD behaviour while appearing to confirm the new one"). |

**No safeguard was removed.** T-1 is intact, still labelled `SAFEGUARD`, still carrying
the not-argued-out clause. R'' is a net addition. I recommend no removals.

---

## The section-13 union: reproduces at 102/27

Re-derived with my own runs of the three fenced expressions, over the unchanged plan
document:

| set | design | measured |
|---|---|---|
| E1 (names the task or work item) | 39 | **39** |
| E2 (asserts the sets or carries the vocabulary) | 69 | **69** |
| E3 (asserts a changed fact in words) | 41 | **41** |
| union | 102 | **102** |
| E1 union E2 alone | 87 | **87** |
| E3-only | 15 | **15**: 116 147 150 344 439 517 627 628 644 705 722 842 916 972 986 |
| VOID | 27 (4 in-body, 23 plan-level) | 27 confirmed as stated, plus one uncovered line (D1) |

**E3 earns its place, and the blind-spot argument is sound.** Of the 15 lines E1 union
E2 cannot see, two are VOID exactly as claimed: line **116** ("six sites to repair, nine
to leave" - it says "sites to repair", not "repair set", and names neither A3 nor the
retired vocabulary) and line **986** ("item 3's assertion set is fifteen lines split six
and nine"; note it survives on `nine` alone, since E3's `six (sites|lines|sentences|repair)`
cannot match "six and nine" either). The other 13 are other tasks' expressions, other
tasks' test-duty paragraphs and A3's own executable clauses already covered by 13.5. So
the claim being made - that a token union over the task's name and the retired
vocabulary cannot see a sentence stating the fact in its own words - is demonstrated
rather than asserted, and it is the same class as section 4.5's four wording sites.

**The four added VOID items are all real, and I had missed all four.** Verified by
reading each line:

- **Line 118** - "`scalar_eq` has SIX arms and two of them coerce", with all six arms
  pasted verbatim, concluding "the comparison is byte-exact for strings and NUMERIC for
  numbers". Three things go false: the arm count and shape, the sentence "The `raw:` arm
  of `exact_matches` calls it directly", and the NUMERIC conclusion for the `raw:` path.
  **This is the most consequential of the four**, and not because of the count: A3's
  Read-first (line 458) designates "the authoring section's item-3 block in full" as
  required reading, so an implementer reads this paste as ground truth for the very
  function it is about to change.
- **Line 476** - Step 1: "run `cargo test -p muxsmith-core matcher` naming
  `b7_raw_int_float_cross_compare` from the output". R-12 renames it, so the clause
  cannot be satisfied.
- **Line 639** - Step 9's commit message asserts "numbers still compare numerically",
  which the ruling inverts, and uses "untyped value equality", which R-2 now retires.
  Round 1 explicitly considered the commit message and dismissed it; I was reasoning
  about whether the *absent replacement* was latitude and missed that the *existing
  message* states an inverted fact. The author is right and I was wrong.
- **Line 950** - the absence-check enumeration carries three figures (red 6, 9 on both
  states, fire moves to 8), all three of which move, and A3 gains a third check.

My round-1 plan-level 21 are all inside the author's 23; the author's 4 in-body are my 2
plus 476 and 639. 23 + 4 = 27.

---

## New findings

### D1. MEDIUM - Plan line 626 is VOID, is named nowhere, and the rule's expressions cannot see it

**Location:** section 13.1's three expressions; section 13.2's table (missing a row);
plan line 626.

**Defect.** Task A3's Step 7 gate-parts bullet reads:

> - **The gate parts that read these files.** `cargo fmt --all --check`; `cargo clippy
>   --workspace --all-targets -- -D warnings`; `cargo test -p muxsmith-core`, with
>   `b7_raw_int_float_cross_compare` and `b8_raw_language_is_byte_literal_no_normalization`
>   named from the pasted output; `pnpm check:i18n`, which is what machine-checks the two
>   help edits. All green, foreground.

R-12 renames `b7_raw_int_float_cross_compare`, so this normative acceptance clause
cannot be satisfied. Section 5's "Task exit bars" is its replacement and already names
the three new tests plus `b8`, but no bullet in section 13 says so, and the plan would
carry two contradictory gate-parts instructions - the "layer them rather than remove
them" failure section 13 opens by warning against.

**Evidence.** The plan names the renamed test at lines 119, 240, 458, 476, 626, 628,
644. Six are covered: 119 and 240 by 13.3 rows, 458 and 476 by 13.2 rows, 628 by 13.5's
"Step 7's test-duty paragraph", 644 by 13.5's "Must not decide". **626 is covered
nowhere**, and it is outside the 102-line union:

- **E1** misses it: the line carries no `A3` and no `W3`.
- **E2** misses it: its terms are `raw:` (needs the colon) and `byte-literal` (needs the
  hyphen). The line carries `b7_raw_int_float_cross_compare` and
  `b8_raw_language_is_byte_literal_no_normalization` - `raw_` and `byte_literal`, in
  identifier form.
- **E3** misses it: no spelled count, no `scalar_eq`, no behaviour phrasing.

I probed for the whole class rather than for this member. Every identifier-form
reference the amendment touches, checked against the union:

| pattern | plan lines | outside the union |
|---|---|---|
| `b7_raw` | 119 240 458 476 626 628 644 | **626** |
| `b8_raw` | 240 458 626 | **626** |
| `byte_literal` | 240 458 626 | **626** |
| `B-7` | 119 234 519 628 | 519, which is inside Step 3's fenced replacement text and is covered wholesale by 13.5's Step-3 bullet - not a miss |
| `exact_matches`, `raw_opt_in_diagnostic` | 118 124 458 462 476 / 124 458 | none |
| `B-5`, `scalar_fits`, `scalar_eq_same_type` | absent from the plan | none |

So the class has exactly one uncovered member, and all three identifier patterns
converge on it.

**What resolves it, stated as the missing clause rather than as the member**, because
this is the second consecutive round returning the same finding one level wider and the
doctrine's convergence rule says to fix the rule, not to route more members:

> **13.1's expression set must include the IDENTIFIER forms of every name the amendment
> renames.** A Rust test name carries `raw` and `byte_literal` with underscores and no
> colon, so a vocabulary alternation written for prose (`raw:`, `byte-literal`) is blind
> to it by construction - the same blindness section 4.6 already records for
> `matcher.rs:457`. Concretely: add `b7_raw` to E1 or E2, re-run the union, and add the
> resulting 13.2 row (626, replaced by section 5's Task exit bars).

With that clause the union re-derives 626 by itself, the 102/27 figures move and must be
restated, and any future identifier rename is covered without another round.

### D2. MINOR - The design cites ten of its own line numbers, and all ten are already stale

**Location:** section 4.6, the R'' extractor-pitfall paragraph.

**Defect.** It instructs: "Anchor on the ten fenced blocks whose preceding line ENDS with
`with exactly` (lines 534, 552, 592, 605, 621, 634, 647, 661, 676, 697 at the time of
writing)". Measured at this tree: the ten lines ending with `with exactly` are **539,
557, 597, 610, 626, 639, 652, 666, 681, 702** - every one off by 5, because the document
grew above them after that paragraph was written.

Tier-2 `a-document-never-cites-a-line-number-inside-itself` bans exactly this: "A
document never cites a line NUMBER inside ITSELF. Name what the line IS ... because the
name survives every insertion above it and the number does not," taken by the owner
"over the alternative of an update duty on his own reasoning that updating can simply be
forgotten: a rule that requires someone to NOTICE is decoration." The plan being amended
lists that convention among the ones that bind hardest. "At the time of writing" is
precisely the update duty the ruling rejected.

**What resolves it.** Drop the line list. The repair is already written into the same
sentence: "or extract by the R-n headings". My own reimplementation of the extractor
keyed on the *content* rule (preceding line ends with `with exactly`) and found all ten
blocks with no line numbers involved, so the anchor is sufficient without them.

*(Non-finding, recorded because a sweep will surface it: the `with exactly` pitfall
itself reproduces. The literal string occurs **11** times in the document while exactly
**10** lines end with it, the eleventh being the paragraph that names the phrase.)*

---

## Reproductions

### Check R breaks at execution time: 6 tracked, 16 with `--untracked`

Run with the plan's own Step-7 pathspec, verbatim:

| form | design | measured |
|---|---|---|
| as the plan fences it (tracked only) | 6 | **6** |
| with `--untracked` | 16 lines across 6 files, ten in the design document | **16 across 6**: design doc **10**, v1 spec 2, `README.md` 1, `help/en` 1, `help/de` 1, `matcher.rs` 1 |

The mechanism is exactly as stated and it is a genuine execution-time break rather than
fragility: `git grep` skips untracked files, so the check passes today only because the
design document is untracked; committing it - which the fold-in makes permanent - turns
6 into 16, because the plan's pathspec excludes the `07-11`, `07-21` and `07-28` dated
specs and not `07-30`, while the design quotes every retired phrase in its "replace
exactly" blocks. An implementer would meet 16 where the plan promises 6. Section 4.1's
clause 2 excludes it by construction, and 13.5 replacing the checks replaces the pathspec
with it. **This is the strongest item in the fix round**: a prescribed red state that
silently depends on a file's git status is a defect neither round-1 review nor the plan's
own three fix rounds caught, and only running it after the artifact existed could have
found it.

### Pathspec exclusion beats inclusion: 1 versus 2

| form | design | measured |
|---|---|---|
| one invocation, v1 spec added as a positive pathspec beside `':!docs/superpowers/specs'` | 1 | **1** (`report/mod.rs:87` only) |
| two invocations, summed | 2 | **2** (`report/mod.rs:87` + v1 spec `:280`) |

Mechanism confirmed: git applies the exclusion after the inclusion, so the positively
named file is silently dropped. The missing line is the v1 spec's `:280` - one of the two
sites R-6 repairs, and one of the two that ground the "untyped equality" boundary. Had
the one-invocation form been used, the design's "it isolates exactly two lines tree-wide"
would have come back as one and R-6 would have looked like churn. **The expected figure
is what caught it**, which is the argument for stating an expected count beside a check
rather than only a green/red condition.

### The rustfmt note is right, not convenient

Tested with a control at my own path:

| input | result |
|---|---|
| section 3.3's fenced block alone (`            Some(have) => scalar_eq_same_type(want, &have),`) | `error: missing 'fn' or 'struct' for function or struct definition` |
| the identical arm wrapped in a `fn` | formats without error |

So the error is a property of isolation, not of the code: rustfmt parses a file as a
sequence of items and a bare match arm is not an item. The note is correct and the
control is what distinguishes it from a convenient excuse. Two further checks, because
"cannot be formatted in isolation" must not become cover for a formatting defect: the
existing tree line is `            Some(have) => scalar_eq(want, &have),` with **12**
leading spaces (`cat -A`), and the design's replacement carries the same 12 spaces at the
same position with only the function name changed, so `cargo fmt --all --check` has
nothing to say either way. Round 1's separate rustfmt runs on the comparator pair and the
three tests were both zero-diff, and those blocks are unchanged by this round.

### R'', reimplemented independently

I wrote my own extractor and my own copy of the LOOSE pattern rather than running the
author's:

| measure | design | measured |
|---|---|---|
| replacement blocks (preceding line ends `with exactly`) | 10 | **10** |
| Rust blocks | 5 | **5** |
| replacements: strict hits | 0 | **0** |
| replacements: loose candidates | 2, both permitted | **2**, both permitted: R-3 ("is matched untyped (value equality...", separated by a bracket) and R-4 ("and is matched untyped: value equality...", separated by a colon) |
| Rust blocks: strict / loose | 0 / 0 | **0 / 0** |
| occurrences of "untyped" in the Rust blocks | 1 | **1** (section 3.2's "the declared untyped path", path sense) |
| **fired control** on R-2's pre-fix text | 1 | **1**, matching `untyped value equalit` |

The control is the part that matters and it fires on the exact text I quoted in round 1's
finding 3, so the check demonstrably catches the defect it was built for and a zero is a
measurement. Grading it as a design rather than as a result: its red state exercises its
anchor, its boundary is the section-4.2 discriminator rather than a fresh judgement, it is
correctly labelled a candidate finder rather than a verdict, and it is prescribed to run
over the five edited product files at execution time - where its expected result is zero
strict and zero loose, which is a falsifiable end state and not a tautology.

### The exit-code matrix, with a control in both directions

Fresh probes and profiles at my own path, `$?` captured immediately after each run (my
first attempt printed a column of zeros because the command substitution in the same
`printf` consumed `$?` - an all-clean column that was my harness, not the tool, which is
why the control below is not optional):

| case | design | measured | `--json` diagnostics |
|---|---|---|---|
| typed rule, matches, no `raw:` | - | **0** | none |
| **`raw:` rule MATCHES** | 1 | **1** | `raw-property` info, `unknown-property-skew` warning |
| required rule, `raw:` non-match | 2 | **2** | + `missing-track` error |
| optional, probe (A) single-rule | 1 | **1** | + `empty-plan` warning |
| optional, probe (B) multi-rule | 1 | **1** | info + skew warning only |

Control: the clean typed run returns 0, the required non-match returns 2, a missing
profile returns 2 - so the harness distinguishes, and the 1s are real.

**The success case is the one that voids my own round-1 inference.** I had written that
exit 1 "IS a machine-visible signal", qualified only weakly. It is not: a working `raw:`
rule exits 1 with the same two diagnostics as probe (B)'s failing optional rule, because
`unknown-property-skew` fires on any consumed `raw:` property. So the exit code carries
"this profile used `raw:`", not "something failed". The design's correction is strictly
stronger than the one my finding asked for, and its refusal of the word "silently" is
correctly grounded on the one signal that does survive - the human rendering's
`rule N -> track -`. Probe (A) being explicitly disqualified for the no-signal claim is
the right call: its `empty-plan` warning is causally tied to the non-match and only the
degenerate one-rule shape produces it.

### The parser precondition, and the choice not to borrow my harness

Five `validate` probes reproduced at my own path, plus a control:

| profile literal | design | measured |
|---|---|---|
| `audio_channels: 1.0` | `has type float, expected integer` | **same** |
| `audio_channels: 6.0` | `has type float, expected integer` | **same** |
| `audio_channels: 400.0` | `has type float, expected integer` | **same** |
| `track_name: 6` | `has type integer, expected string` | **same** |
| `track_name: 400` | `has type integer, expected string` | **same** |
| control: `audio_channels: 1` | - | **no type mismatch reported** |

`Scalar::type_name()` is at `match_expr.rs:38-43` and returns `"float"` only for
`Scalar::Float` and `"integer"` only for `Scalar::Int`, so the diagnostic is a sound
read-out of the parsed variant. The control matters: without it, five identical
"mismatch" lines could equally mean the probe always reports one.

**Carrying this with the author's own instrument rather than citing round 1's
`yaml_serde` harness is the right call and not a formality.** The two measure different
things - mine reads the variant directly out of the deserializer, the author's reads it
through the shipped binary's diagnostic - and the second is the one an implementer can
re-run at Step 1 without building a scratch crate. Prescribing it as a Step-1 re-run
converts a reviewer's one-off check into a standing precondition, which is where it
belongs, since it is what makes T-1 a safeguard rather than a formality.

---

## Harvest

- **A prescribed check can depend on a file's git status, and nothing in the check says
  so.** `git grep` skipping untracked files made the plan's check R return the promised 6
  today and 16 once the document under review is committed. The trigger is readable: a
  check counts occurrences over a repository, and the artifact that will change the count
  is the one being written. Handle: run absence checks with `--untracked` while the
  artifact is untracked, or state the expected figure for the committed state.
- **The expected figure is the instrument that audits the invocation.** Two of this
  round's findings were caught by a count coming back wrong, not by anyone inspecting the
  command: the pathspec exclusion silently dropping the v1 spec (2 became 1) and my own
  `$?` column of zeros. A check specified as "must return nothing" cannot catch either.
- **An alternation written for prose is blind to the same word as an identifier.** `raw:`
  does not match `b7_raw_`, `byte-literal` does not match `byte_literal`. The design
  recorded this for its wording surface and then built section 13's instrument with the
  same blind spot. Whenever a rename is in scope, the expression set needs the identifier
  form, and the check is one grep for the old symbol across the consuming document.
- **A second round of "one more member" is the signal to fix the rule instead.** Round 1
  returned 23, the author returned 27, this round returns 1. Routing that member alone
  would leave the same instrument in place for the next rename. Returning the clause
  (identifier forms belong in the expression set) closes the class and makes the round
  terminal by construction.
- **"Cannot be verified in isolation" needs a control before it is accepted.** The
  bare-match-arm rustfmt error is genuine, and the way to know is to format the same arm
  inside a function and watch it succeed. Without that, the sentence is
  indistinguishable from an excuse for an unformatted block, which is exactly what it
  would have been covering if the indentation had been wrong.
- **A reviewer's own inference can be the weakest link in the artifact.** My round-1
  finding corrected "exit 0" to "exit 1" and then built a claim on the 1 being a signal.
  Running one more case - a *successful* `raw:` match - removed the claim. The general
  form: when a corrected figure is used to support a new inference, measure the
  neighbouring case that would falsify it, not only the case that produced the
  correction.

---

## Summary

**Verdict: NEEDS_FIXES**, narrowly: 1 medium, 1 minor, both mechanical.

**All seven round-1 findings are addressed**, four of them beyond what I asked for.

**The section-13 union reproduces at exactly 102/27** with my own instrument (E1 39, E2
69, E3 41, union 102, E1 union E2 87, E3-only 15), the four added VOID items are all
real, and E3's blind-spot argument holds.

**Outstanding:** plan line 626 is VOID and uncovered, and the fix is the clause -
identifier forms in 13.1's expression set - not the member; and the ten self-cited line
numbers in section 4.6, all stale by 5, against a Tier-2 convention the plan names as
binding.

Apply those two and the design is fit to fold in. Nothing else is outstanding, the
decision needs no change, no safeguard should be removed, and the safety property stands
as settled in round 1.
