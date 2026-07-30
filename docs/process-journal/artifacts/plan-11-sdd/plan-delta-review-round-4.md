# Plan 11 - delta review of the D111 four-role fold-in

Reviewer: the resumed original plan reviewer (fifth pass; same judge and standards
as `plan-review-round-1.md` and delta rounds 1-3). Settled non-findings are not
re-litigated. **D111's decision and its safety measurement are out of scope**; this
grades only whether the plan now carries it correctly and completely.

Artifact: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
at `6a564bc` (923 lines), against `6cda608` (986 lines), the version I approved.
Consumed as the requirement set: D111 section 13 (`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`,
committed `e0c9d2b`) - 13.1's four expressions and their union of 103, 13.2's 5
in-body VOID members, 13.3's 23 plan-level VOID members, 13.4's UNAFFECTED list and
13.5's replacement clauses. Tree: `master` at `6a564bc`. The repository is
unmodified by this review apart from this file.

Instruments built fresh for this round, under `.../round5/`: `rv_overrides.py`,
which takes each member's line from the plan version D111 was written against,
extracts the exact assertion from D111's own "assertion" column, and asks whether
that text still exists in the folded plan - so a VOID member that survives and an
UNAFFECTED member that was swept are both visible without reading the diff. D111's
own expressions were extracted from its fences and run rather than retyped.

---

## Verdict: NEEDS_FIXES

**One Important finding, and it is a single-clause deletion.** Everything else in a
large and well-executed fold-in checks out: 27 of the 28 VOID members are gone, all
21 UNAFFECTED members I probed survive including the one the dispatch flagged, every
re-measured figure reproduces exactly, all the recounts hold, the cross arms are
stated as kept at ten sites with none readable as a removal, and the four other
tasks are byte-identical.

The finding is that **one of the two VOID members D111 itself singled out as "the
sharp ones" survived**: plan line 119 was rewritten with a VOID marker and the
owner's inversion, and its trailing clause "**and it is why Task A3 owes no new
test**" was left in place. The same line, forty words earlier, says the behaviour
changes and three tests ship. So the authoring block that A3's Read-first designates
as ground truth now asserts both. Nothing downstream depends on the stale clause and
an implementer cannot act on it without seeing the contradiction, which is why this
is Important rather than blocking-by-content - but it is a superseded assertion
standing inside an owner-approved contract, which is the one thing D111's preamble
says a fold-in must not do, and it costs one clause to remove.

I would approve on that alone being fixed. I am not asking for another round of
anything else.

---

## The completeness answer, stated plainly

**VOID: 27 of 28 gone, 1 survived.** **UNAFFECTED: 21 of 21 probed survive, 0 moved.**

I probed 32 fragments over the 28 members (several members carry two distinct
assertions on one line). Six fragments still matched somewhere in the folded plan;
I ran each to ground and five are legitimate:

| fragment | where it survives | judgement |
|---|---|---|
| "the behaviour stays and only the wording changes" | L463, as the disposition "the ruling **INVERTS**" | correctly voided; survives only as the named inversion |
| "seven-member retained set" | L61, L180 | **not a survival - my fragment was wrong.** Seven is the amendment's own correct figure per 13.3; I had it in the VOID set because it was the round-3 defect |
| "assertions is 15 lines, split 6 to repair and 9 to leave" | L166, with "correct for the question the plan was asked, and superseded by D111's 12 and 7" | acceptable: the corrections table is a dated record of a refutation of the brief, and deleting it would falsify that record. The supersession is stated, so no reader acts on 6/9 |
| "The behaviour is unchanged" | L873, in the amendment log describing what was repaired | correctly voided; survives as the record of its own removal |
| "assertion set is fifteen lines split six and nine" | L923, self-review's brief-refutations, same supersession note | same as L166 |
| **"it is why Task A3 owes no new test"** | **L119, present tense, as the bullet's closing clause** | **SURVIVED - finding 1** |

The layering treatment at L166 and L923 deserves one sentence, because D111's
preamble says "remove the superseded clauses rather than layer them" and this is a
layer. I rule it correct anyway: both are records of what was measured against the
controller brief at authoring, in tables whose purpose is to record refutations.
Deleting a true dated finding to satisfy a later ruling is the failure mode the
plan's own Global Constraint about dated measurements exists to prevent, and each
carries the new figure beside the old. That is a different act from leaving a
present-tense assertion standing, which is what L119 does.

### Finding 1 - Important - a VOID member survives in A3's designated ground truth

**Location:** plan line 119, final clause.

The bullet now opens correctly: "**PRE-AMENDMENT ground, VOID: this measurement was
read as showing the behaviour settled and only the wording wrong, and it is why the
task was said to owe no new test.** The owner ruled the opposite on 2026-07-30: the
behaviour changes and three tests ship (D111 section 5). What survives is the
measurement itself..." - and then closes: "The measurement is stronger than the
tracker's 'deliberate-looking' **and it is why Task A3 owes no new test**."

That final clause is D111 13.3's line-119 assertion verbatim, in the present tense,
whose measured truth after the amendment is "the behaviour changes; three tests
ship". It matters more than its size suggests for three reasons D111 states itself:
it is the AUTHORING ground on which "A3 owes no new test" rested; A3's Read-first
designates the authoring item-3 block as ground truth, so a task reads it; and D111
named line 119 as one of the two members "a fold-in that leaves them standing ships
a plan whose acceptance map contradicts its own task".

**What resolves it.** Delete the clause, or close the sentence at "the tracker's
'deliberate-looking'". The rest of the bullet is right and I would not touch it: the
void marker, the inversion, the owner's date, the D111 §5 pointer and the surviving
measurement are all correct, and the preserved measurement genuinely is what M1
re-measured.

**A note on how it got there, because it is the same class as the round-3 finding.**
The bullet was edited at its front and its middle; the clause at the end was not
visited. That is the consumer-versus-source shape one level in: the repair walked
the assertion it was rewriting and not the sentence that closes the same bullet.
Cheapest guard against a recurrence: when a bullet is void-marked, read it to its
final period.

### The UNAFFECTED side: nothing was over-applied

All 21 probes survive. The two the dispatch named specifically:

- **13.4's dated round-1 record, "It says nine now"** (old L934, now L853): present
  and **not renumbered**. I also weighed whether leaving it is right, since the tier
  table now says seven again and the sentence reads present-tense. D111's call holds:
  it is a record of a round-1 fix whose surrounding sentence dates and scopes it
  ("The model-tier table said 'a seven-member retained set' where eleven other sites
  say nine"), and the amendment moving the count back to seven for an unrelated
  reason does not falsify the fix it records. Renumbering it would have destroyed a
  true record to satisfy a coincidence.
- **The four other tasks' test-duty paragraphs** (A1 L347, A2 L442, A4 L641, B1
  L761): all four present and each still grounded on its own task producing no
  user-visible consequence, which stays true. Only A3's was void, and A3's is
  correctly replaced at L547 with "**Three tests ship in this package** ... which is
  the inverse of this task's pre-amendment position."

Running 13.1's four expressions against the folded plan returns E1 44, E2 56, E3 36,
E4 14. I walked E3's sharp subset (the terms that assert a changed fact) and E4 in
full: `b7_raw_int_float_cross_compare` now appears only at L119, as a historical
statement of what the test was, and at L521, as the name R-12 replaces. **It is no
longer prescribed anywhere** - the old Step-7 gate-parts bullet that named it at the
exit bar is gone, replaced by L543's exit bars per D111 §5. That was the member E4
was built to catch, and the catch landed.

---

## Reproductions: every figure reproduces exactly

D111's expressions extracted from its own fences and run at HEAD, both invocations
summed as the design specifies:

| figure | claimed | measured |
|---|---|---|
| R' pre-state | 8 lines across 6 files | **8 lines across 6 files** - `README.md:60`, `matcher.rs:96`, `report/mod.rs:87`, spec `:176`/`:280`/`:421`, both help topics |
| K' pre-state and end state | 7 lines across 6 files | **7 lines across 6 files** - `matcher.rs:452`/`:466`, `validate.rs:408`, `validate_semantics.rs:249`, spec `:421`, both `diagnostics.ftl` |
| vocabulary sweep | 71 | **71** |
| R' control, ROADMAP exclusion dropped | 9, ninth at `docs/ROADMAP.md:1913` | **9**, and the ninth is that line |
| **the plan's OLD check R at HEAD** | **16 across 6 files, ten inside D111** | **16 across 6 files, exactly 10 inside `2026-07-30-plan11-raw-bytewise-design.md`** |

The last one is the one the dispatch asked for and it is worth stating why it
matters: I extracted the old check-R fence verbatim from the plan I approved and ran
it unchanged. Committing the design put its own quoted phrases into the tracked
corpus, and the old pathspec excluded only three named spec files, so the check now
sweeps up ten lines of the document that defines its replacement. **A count could
not have fixed that** - 16 is a true measurement of a corpus that no longer means
what the check meant - which is exactly why 13.5 replaces the pathspec (`':!docs/superpowers/specs'`
plus an explicit second invocation over the v1 spec) rather than the number. The
repair is correct at the root.

### Recounts

| | claimed | measured |
|---|---|---|
| acceptance rows | 40, split 13/6/13/3/5 | **40**, `W1=13 W2=6 W3=13 W4=3 W5=5`, letters contiguous per item, no duplicates, and both stated totals agree |
| W3's growth | +3 rows | 10 -> **13**; the three new observables are the behaviour changing in both directions, the typed path provably unmoved, and no serialized surface moving |
| A3 repair sites / files | 12 over 6 | stated consistently at L61, L122, L180, L459; **Files (EXHAUSTIVE) carries exactly 6 Modify entries**, the five prior plus `report/mod.rs`, per 13.5 |
| A3 retained set | 7 over 6 | consistent, and K' measures 7/6 |
| tests | three | **T-1, T-2, T-3** |
| A3 steps | ten | **10** (Step 1 to Step 10) |
| verification checks | nine | heading says nine; **9 sub-bullets** |
| fenced blocks | "86 to 58" | 86 -> 58 is the **delimiter** count; **43 -> 29 blocks**. Same fact, worth stating in blocks since that is what the sentence claims to count |

### The cross arms: stated as KEPT everywhere, at more sites than reported

My own expression over cross-arm mentions returns **ten** sites, not five, and every
one frames them as preserved or scopes the removal to the `raw:` path: L118 ("which
stay"), L234 ("no cross-comparison happens **on this path**"), L241 (the safeguard
row), L482 (pre/post expectations per direction), L491 ("**PRESERVED, not removed**"),
L520 (T-1 pinning the typed path), L557 (the commit message), L562 (Must-not-decide),
L869 (the amendment log), L891 (the recomputed counts). **No plan text is readable as
removing them from `scalar_eq`.**

**T-1 stays, and I checked the claim that makes it mandatory rather than weighing
it.** The plan says a change that strips the cross arms from `scalar_eq` "passes
every other check in this task". I walked the other eight checks in Step 8: R', K',
the vocabulary sweep and R'' are fixed-phrase or vocabulary greps over prose; the
exit bars are `fmt`, `clippy`, `test -p muxsmith-core`, `doc` and `check:i18n`; the
README checks and the corpus derivation concern a different file; the diff-scope
check counts files. T-2 and T-3 pin the `raw:` path, which a cross-arm strip does not
touch. So the claim holds: T-1 is the only check that would fail. I recommend
removing no safeguard, and per `proc-proposed-safeguard-stays` T-1 is not up for
discussion at planning time anyway.

### Scope: four tasks byte-identical, one deviation to report

**A1, A2, A4 and B1 are byte-identical** between the approved plan and the fold-in -
**zero changed lines in each**, measured by extracting each task section from both
revisions and diffing. No step renumbering leaked outside A3.

**Finding 2 - Minor - the scope claim is not true of A3's Step 6.** The dispatch
reports the README-example step "byte-identical otherwise", and D111 13.5 lists
"**Untouched:** Step 6". It was condensed: 22 non-blank lines to 18, with 16 changed
lines. I checked what moved and **nothing was lost**: the fenced insertion string
`  pattern: '.*'              # every candidate file; the whole basename is the identifier`
is byte-identical; the exit-2 defect measurement, the owner's rejection of the serde
default, the column-30 form, the discriminator and the three blind-spot probes all
survive in place; and three claims moved rather than vanished - the `Profile is
valid.` green expectation into Step 8's check 6 (which states both directions), the
delta-zero claim (three occurrences remain), and the "no example-validation CHECK"
restraint into the amendment's not-folded-in list, with the deferral row still
holding the checker question.

So this is a reporting defect, not a content defect. I raise it because a scope
report is the thing a reviewer is invited to trust instead of re-deriving, and
because it is **compression, which I recommended last round be deferred to the
close** - it started here instead. Resolution: correct the claim, or restore the
step, and prefer the former since the condensation is harmless and the substance is
intact.

---

## Ruling on the fencing question

**Do not duplicate the fences. Keep D111 as the single source of the verbatim
replacement strings.** I side with the author, and the reason is an asymmetry in the
failure modes rather than a preference for brevity.

If the plan carried its own copies and the two drifted, **every check in Task A3
would still pass.** A3's acceptance is absence and invariance greps over the retired
vocabulary plus the three tests - nothing compares the applied text against D111. So
a drifted duplicate yields a green task that has written the wrong sentence into the
v1 spec, the README and two shipped help topics. That is the worst shape a defect can
take here: silent, green, and in user-visible product text. Against that, the cost of
the single source is friction and a chance to paraphrase - real, but loud rather than
silent, because a paraphrase shows up in the diff-scope check and in review as text
that matches no fence.

The drift class is also the one this project has actually paid for, repeatedly, and I
have been the one measuring it: "twelve parents" wrong at one of six sites, the
"seven-member retained set" wrong at one of twelve, "eight hunks" at three sites, a
gate ordinal reintroduced in a Goal sentence. Every one was a fact restated at
multiple sites where one site went stale. A verbatim replacement string duplicated
across two documents is precisely that shape.

Two things make the single source safe here, and I verified both rather than assuming
them:

1. **The pointer is per-site, not per-document.** Each of the twelve sites names its
   own `R-n`, and I confirmed all twelve of `R-1` to `R-12` are referenced in the plan
   (three to nine references each) and defined under their own headings in D111, whose
   section 4 carries 25 fenced blocks. A3's Read-first names section 4.3 as "the twelve
   repair sites with their exact replacements". So the lookup is deterministic - open
   4.3, find R-5 - rather than a search, which is what separates this from a
   see-the-other-document pointer.
2. **The strings are named as not the implementer's to compose.** Must-not-decide
   includes "every fenced replacement R-1 to R-12, in both natural languages", so a
   paraphrase is a rule violation rather than a judgement call.

**One hardening I would add**, forward-looking rather than a defect: state in one
clause that D111's fence is the only source and that transcribing the replacement
strings into the plan is not wanted. The plan comes close - Step 2 says "each verbatim
from D111 section 4.3" - but nothing forbids the copy. Without that clause the next
fix round can helpfully paste the fences in "for the implementer's convenience" and
manufacture exactly the drift pair this ruling avoids. That is the same reasoning as
naming a restraint so it is not re-litigated.

---

## Harvest for the controller

1. **A void-marked passage is read to its final period.** Finding 1 is the
   consumer-versus-source pattern one level in: the repair rewrote the front and
   middle of a bullet and left its closing clause asserting the voided fact. The
   handle is mechanical and cheap - when a sentence or bullet is marked VOID, the unit
   of repair is the whole bullet, not the clause that stated the fact.
2. **Committing a design changes the corpus its own checks run over.** The plan's old
   check R went from 6 hits to 16 the moment D111 was committed, ten of them inside
   D111, because the pathspec named three spec files instead of the specs tree. Any
   check whose pathspec enumerates paths rather than excluding a tree will do this the
   next time a design is committed mid-plan. Worth a ledger entry: an expression's
   pathspec is part of its claim, and a count cannot repair a corpus that changed
   meaning.
3. **The single-source ruling and its two preconditions** (per-site pointer,
   composition forbidden) generalize to any four-eyes design whose replacement strings
   a plan consumes. Worth recording with the asymmetry argument attached, because the
   convenience argument for duplicating will be made again.
4. **E4's existence is the transferable part of D111 §13.** A prose vocabulary is
   blind to an identifier by construction, and here that blindness would have cost the
   gate-parts bullet naming a renamed test - the one member no prose expression could
   reach. Any amendment that renames a symbol needs an identifier-form arm in its
   override expression.
5. **Compression started before the close** (Step 6, 22 lines to 18), against the
   recommendation recorded as a close action. Not harmful in this instance - I verified
   nothing was lost - but it is what made a scope report wrong, and the close action
   exists so that condensation happens once, deliberately, with the reviewer looking.
