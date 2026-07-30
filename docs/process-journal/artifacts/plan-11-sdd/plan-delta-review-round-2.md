# Plan 11 - delta review of fix round 2

Reviewer: the resumed original plan reviewer (third pass; same judge and standards
as `plan-review-round-1.md` and `plan-delta-review-round-1.md`). Settled
non-findings are not re-litigated.

Artifact: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
at `1e7b062` (941 lines), diffed against `fac7b50` which I judged last round. The
plan's own diff is 57 lines; the bulk of the commit is ROADMAP and ledger, which
are the controller's and outside this verdict except where the plan cites them.
Tree: `master` at `1e7b062`; working tree clean apart from the co-writer's
untracked Plan-12 document. The repository is unmodified by this review apart from
this file.

Instruments built fresh for this round, under
`.../scratchpad/pr11-review-independent/round3/`, none reused from rounds 1 or 2:
`rv_gateaudit.py` (re-runs the gate-count self-audit and assigns each hit a home
derived from the document's headings rather than from the plan's own reasoning),
`rv_figures.py` (a figure-first census that hunts stated figures and then asks
whether a probe is near them - the complement of the author's method, so it cannot
inherit its blind spot), `rv_metasearch.py` (extracts every inline span and
classifies by first-token *shape* rather than by a command-name list, so a span
whose first token is not a command name is visible by construction). The
`deny.toml` reproduction from round 2 still stands and I re-verified its premise:
`deny.toml`'s blob is byte-identical to `1e7b062:deny.toml`, and the diff hunks in
this round do not touch B1 Step 4, so the fenced insertions I fired against are
unchanged.

---

## Verdict: NEEDS_FIXES

Everything structural is now settled, and I want that on the record before the
findings: both round-1 blocking findings stay fixed, all four round-2 findings are
addressed at the root, the gate-count self-audit is stable under its own presence,
the meta-search's complement check holds up against an independently built
complement, the `ledger-lint` unfencing is the right call for the right reason, and
the blast radius now matches my own finer measurement rather than approximating it.
The convergence move was correct: one mechanical sweep found two figures no finding
had named, which is the empirical case for it over a member list.

It returns NEEDS_FIXES on two Important findings and one Minor, all of them
figures rather than structure. The sweep's own new figure is the one that does not
reproduce: A3's vocabulary sweep states 100 hits for an expression that returns
**300** as written, and 100 is only reachable by dropping the `-i` the expression
specifies and excluding a file the stated surface includes - so an implementer
running it owes a 200-line reconciliation in a required step. And the sweep missed
exactly the pair the dispatch predicted it would: a figure sitting in prose away
from its command, "a seven-member retained set" in the model-tier table, where
eleven other sites in the same document say nine. That one has been wrong since
`148f19f`, which means it survived both of my earlier passes as well as the sweep -
I am reporting it against myself as much as against the author. Neither finding
changes what gets built; both would cost a round-trip at execution.

---

## Per-finding disposition

| # | Round-2 finding (severity) | Disposition |
|---|---|---|
| 4 | "twelve direct parents", sixth site (Important, was NOT_ADDRESSED) | **ADDRESSED** |
| N1 | widened A4 sweep declared three of its six hits findings (Important) | **ADDRESSED** - one Minor sub-defect, new finding NEW-3 |
| N2 | live positional gate ordinal in the Goal sentence (Important) | **ADDRESSED** - root, and the audit is now stable |
| N3 | no-permanent-guard deferral named no observable event (Minor) | **ADDRESSED** |

**Tally: 4 ADDRESSED, 0 NOT_ADDRESSED, 0 disputed.** Cumulative across three
rounds: 20 findings raised, 20 addressed.

**Finding 4.** The deferred-by-decision row now reads "**Eleven** direct parents on
normal edges", and its previously unattributed "has not migrated off GTK3" is
attributed to `deny.toml`'s own comment, matching what B1 Step 8 already did. I
re-swept the figure by search rather than by list: six sites state it, five say
eleven and one says "Twelve" - and that one is Amendment 2's bullet recording the
repair, quoting the old token in order to record its deletion. Legitimate, and the
same shape as the surviving `part 5`. The recomputed-counts list says "**11**
direct parents of `glib` on normal edges", carrying its unit.

**N1.** The sweep is re-run and all six hits are classified with a verdict each. I
ran it verbatim and got the same six lines - 318, 319, 347, 369, 379, 391 - and
checked each verdict independently: none of the three newly visible hits
contradicts the new 130 bullet (two are module/view responsibility descriptions,
one is keyboard handling), so all three "Consistent" verdicts are correct. The
closing rule now reads "a hit outside these six". One label is wrong; see NEW-3.

**N2 - and the third instance the dispatch asked me to hunt.** The Goal sentence
names `cargo deny check` instead of a number. `part 5` now appears twice, both in
records of its own deletion (the audit paragraph and Amendment 2's N2 bullet),
which is what the new fifth kind exists to cover.

I re-ran the audit's own expression against `1e7b062` and classified every hit
myself, assigning each a home from the document's heading structure rather than
from the plan's reasoning: **34 tokens on 24 lines**. Every one falls in one of the
five kinds, with a single borderline I want to name precisely rather than inflate.
Line 739 reads "the owner decided it on 2026-07-30, **in two parts** that must not
be collapsed" - the parts of a *ruling*, not of a task. Kind 1's heading is "the
unrelated task-part sense", which covers it as a non-gate use; kind 1's examples
("about work item 1 and Task B1, and `part a:` in the coverage map") do not name
it. Because kind 1 states no count, it is a category rather than a closed list, so
the "every hit falls in one" claim **survives** on the category reading. The cheap
hardening is to call the kind "the unrelated non-gate sense of `part`", which
removes the dependence on reading a heading against its examples. I am recording
this as an observation, not a finding: the audit is not falsified, and having been
repaired twice it is now stable under its own text, which was the thing worth
checking.

**N3.** The row now names the event and it is word-for-word the one the v1.x `glib`
entry already watches - "a dependency PR or a Tauri release moves the gtk-rs
generation past 0.18 in `Cargo.lock`" - with the guard question riding it as a
rider on the ignore-entry revisit. I verified both texts name the same event, so
the close transcribes rather than invents. The reasoning the author added is also
the right generalization: a failure mode that is silent by construction cannot
carry its own trigger, so it has to ride one somebody already watches.

---

## Reproductions, with measured figures

### The gate-count self-audit, re-run by a reader who is not its writer

| claim | plan | measured |
|---|---|---|
| kinds | five, "every hit falls in one" | **34 tokens on 24 lines; every hit classifiable, one borderline (L739) that the category reading absorbs** |
| `part 5` survives only as a record of its deletion | asserted | **confirmed** - L898 (Amendment 2) and L935 (the audit), both kind 5 |
| the Goal sentence no longer carries an ordinal | asserted | **confirmed** - its only token is "three parts", kind 1 |
| no total stated | deliberate | **confirmed** - kinds are reported, no tally to falsify |

### The blast radius: **now matches my measurement exactly**

The plan states: at `-L info`, `note[advisory-ignored]` 18 -> 19, distinct ids
18 -> 19, the stats line 36 -> 38 notes, the id-set difference exactly
`{RUSTSEC-2024-0429}`, and the note-class difference exactly one
`advisory-ignored` plus one `unsound`, "which accounts for both added notes and
leaves nothing unexplained". Every one of those is what I measured last round with
my own config copies, including the arithmetic that ties the +2 in the stats line
to the two named note classes. This is a match, not an approximation, and the
sentence is now the tightest statement the tool's output supports. The premise
still holds: `deny.toml`'s blob equals `1e7b062:deny.toml` and B1 Step 4's fenced
insertions are untouched by this round's diff, so the three-way fire I reproduced
in round 2 still describes this artifact.

### The meta-search's complement check: **adequate, and I found no further member**

I built the complement independently, classifying inline spans by their first
token's *shape* rather than against a command-name list, so a span whose first
token is not a command name shows up by construction.

| result | measured |
|---|---|
| inline spans containing whitespace, outside fences | **381** (the plan says 362 for its own extractor; see the observation below) |
| runnable-*shaped* spans whose first token is not a known command name | **56** |
| of those, the ENV-ASSIGNMENT class | **exactly 2 occurrences of exactly one command** - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items`, at two sites. Precisely the miss the plan reports, and the class has no other member |
| the other 54 | **not commands**: `scalar_eq` match arms, the two JSON alert payloads, version ranges (`>= 8.5.18`), isolated flag fragments (`-e normal`, `--depth 1`, `-L info`), regex fragments, a commit subject, Fluent and YAML text. None carries a figure a run would set |
| `EXT=$(git ls-files ...)`, the other assignment-prefixed command | **inside a fenced block**, therefore covered by the hand pass over the five task sections rather than by the span filter - so it is not a member of this complement |

So the author checked the right complement and found its only member. The three
isolated flag fragments are worth noting as a non-issue: each belongs to a command
stated in the same sentence whose figure *is* in the reconciled list (54 for
`-L info`, 11 for `--depth 1`).

### The sweep's coverage: one pair missed, one figure that does not reproduce

I hunted figures first rather than expressions first, which is the complement of
the author's method. **126 lines carry a measurement-shaped figure; 78 have a probe
on the same line and 48 do not.** Walking the 48 - the hard case the dispatch names
- almost all are restatements of reconciled figures (headings, acceptance rows
restating a pre-state count, the tier table). Two do not reconcile, and they are
NEW-1 and NEW-2 below.

I also re-ran the reconciled figures that bear on the tree and they hold: the A4
sweep at 6, the retained expression at 9 across 7 files, the repair expression's
member arithmetic, the replacement control's 7-and-1, `codec_kind` at 0 in
`generated.rs`, the README corpus at 6 blocks / 3 profiles / 1 defective / delta
zero. The 16-classified-lines figure in the vocabulary sweep is **correct**, and
its stated arithmetic is right: 6 repair + 9 retained + 2 different-claim = 17 line
slots, minus the v1 spec's 9.2 line which is in both the repair and retained sets,
so 16 distinct lines.

---

## New findings

### NEW-1. Important - the vocabulary sweep's stated figure does not reproduce under its stated expression

**Location:** Task A3 Step 7, the "Vocabulary sweep, alternation-free" bullet.

The bullet prescribes `git grep -niE 'byte' -- .` "over the same live surface" and
states "**re-derived by running THIS expression rather than predicted from the two
alternations: 100 hits over the live surface**".

Run as written, over the same pathspec the repair and retention expressions use, it
returns **300** lines. The 100 is reachable, but only by changing the expression
and the surface at once:

| variant | lines |
|---|---|
| as stated (`-niE`, the live-surface pathspec) | **300** |
| case-sensitive (`-nE`), same surface | 120 |
| `-niE`, minus `e2e/` | 118 |
| `-niE`, minus `e2e/` and `Cargo.lock` | 98 |
| **case-sensitive, minus `Cargo.lock`** | **100** |

So the figure was measured without the `-i` the expression specifies and with
`Cargo.lock` excluded from a surface that does not exclude it. The `-i` is what
does most of the damage: `getByText` matches `byte` case-insensitively, which is
why three e2e spec files alone contribute 157 of the 300.

**Why this is Important rather than cosmetic.** The step requires the report to
classify **every hit**. As written that is 300 lines, 200 of them a
case-folding artifact, against a stated expectation of 100 - so the implementer
either classifies 300 lines or opens a reconciliation over a 200-line delta, in a
step whose whole purpose is to be the alternation-free backstop. And it is the one
figure this round newly created, which is exactly where the sweep's own standard
should have bitten hardest.

**What resolves it.** Make the expression produce the figure rather than the other
way round: `git grep -nE 'byte'` over the live surface with `':!Cargo.lock'`
added, stated as 100. That is also the better instrument for the purpose - the
audit is looking for prose claims about byte-exactness, where `-i` buys nothing and
a lockfile buys nothing. Alternatively state 300 and name the noise classes, but
then the classification duty needs bounding or it is unworkable.

### NEW-2. Important - the sweep missed a probe-and-figure pair: "a seven-member retained set"

**Location:** the Model tiers table, the A3 row's `ground` column.

It reads "six sites across five files in two languages, an SI-3 re-run, and **a
seven-member retained set** that must be proven unchanged". The retained set is
nine lines across seven files. The row took the file count for the member count.

The same figure is stated at twelve sites in the document; **eleven say nine** (the
authoring measurement, the corrections table, the coverage map's "nine-member
retained set", acceptance row W3-g, A3's "Nine assertions in seven files", A3 Step
8, the plan close, the deferred-by-decision row, the self-review) and **this one
says seven**. The coverage map at the other end of the same pair of tables says
"nine-member retained set" in the same words.

**It has been wrong since `148f19f`** - I checked all three revisions - which means
it survived my round-1 pass, my round-2 pass, and this round's sweep. That is the
finding's real content and I am reporting it against myself as much as against the
author: it is exactly the case the dispatch predicted, a figure sitting in prose
in a table cell rather than beside a command, so a sweep that walks probes never
arrives at it. The reconciled list contains "the retention expression (9)" - the
probe was run and its figure is right at source; what went unvisited was a
*consumer* of that figure.

**What resolves it.** "nine-member retained set" in the tier row. And the
generalizable half, which is the more valuable output: a sweep over probes visits
sources, not consumers. The pass that closes this class walks each reconciled
figure to every site that restates it - which is the same duty as changing a fact
that several texts assert, one level in.

### NEW-3. Minor - a section label in the A4 sweep's classification is wrong

**Location:** Task A4 Step 4, classified hit 3.

It reads "section 8.2's architecture table row `| `executor` | process spawn,
progress parse, cancellation, job states |`". That row is spec line 347, which sits
under **`## 7. Architecture`** (heading at line 322); section 8.2 begins later. The
neighbouring labels are right - hits 5 and 6 really are in 8.2 (the job-queue view
and the help-mode sentence).

Trivial to fix and worth fixing because the implementer reconciles hit by hit
against these labels and would find no such row in 8.2. The verdict on the hit is
correct either way.

---

## The two judgements the dispatch asked for

### 1. The `ledger-lint` unfencing: right call, right boundary, no discrimination lost

**The boundary is drawn in the right place, and the plan applies it consistently
without restating it as a rule** - which is correct for a plan; the rule belongs in
the ledger, where the controller put it. Inside the document the contrast is
visible: the entry count over the *ledger* is unfenced, while every figure over
`BUILDING.md` - three ordinals, one non-fenced line at 86 characters, `11 parts` in
the fire instruction - stays fenced, because A1 owns that file. Acceptance row W5-c
carries no number either ("exits 0 and prints its summary line"), so the unfencing
did not stop at the prose.

**No discrimination was lost.** I asked what the fenced count could have caught
that the three retained invariants cannot. A silent reduction in scope - the linter
reading fewer files - is caught by the `across 4 files plus BUILDING.md's gate
enumeration` shape. The gate-count invariant, the only thing A1 can actually break,
is caught by the exit code plus the fire that mutates the canonical total and
watches it go red. A lost ledger entry is real but is not A1's to detect, and no
plan should gate on it. So the three invariants cover the failure modes the task
owns, and the count was covering none of them.

**The empirical clincher is my own run.** The figure was 548 when the plan was
written, 550 when the author swept, and **552** today. Three values inside a few
days. A fenced number would already have been wrong twice, and the second time it
would have failed a task for a co-writer's legitimate ledger append - which is
precisely the reasoning given.

### 2. Are the two clauses being over-applied?

**Not in substance.** Clause 1 was applied to 35 probe-and-figure lines and, where
re-running showed the plan does not own the figure, the answer was to stop stating
it rather than to restate it - which is the clause working, not overreaching.
Clause 2 produced a fifth kind covering the audit's own text; that reads
self-swallowing but is a fixed point rather than a regress, and the plan says so.
Neither clause caused a safeguard to be dropped, and I recommend removing nothing.

**There is a cost, and it is in the recording rather than the applying.** Measured
across the three revisions, the self-review plus amendment sections have grown
2176 -> 3527 -> **5141 words**, i.e. **10% -> 12% -> 17%** of the document, while
the executable task sections grew far less. Each round now narrates its own repair
inside the contract: the audit paragraph explains why it has five kinds, the
amendment log re-tells each finding, and the sweep documents its own method and
result. Individually each is justified - and the audit's fifth kind genuinely
cannot be moved, because it exists to classify text that is present. But the trend
is what a third round should notice, and the doctrine already says where it lands:
a plan is a contract, and history belongs in git and the journal. **My
recommendation, for the plan close rather than for this fix round:** the
per-finding narration in the amendment sections is salvage-grade history, so it can
move to the SDD scratch and the journal entry at the close, leaving the contract
carrying its decisions and the audits that must re-run against it. I would not
strip it now - it is load-bearing while the plan is still under review, and cutting
review history mid-review is how a fix round loses the reason for a decision.

---

## Harvest for the controller

1. **A sweep over probes visits sources, not consumers.** NEW-2 is the clean
   instance: the retention expression was re-run, its figure is right at source and
   in the reconciled list, and a tier-table cell 112 lines away still states the
   file count as the member count. This is the same duty as "a fact that several
   texts assert is swept at every site" applied one level in, and it is the clause
   this round earns: **after re-deriving a figure, walk it to every site that
   restates it.** Cite `1e7b062`.
2. **A figure and its expression are one unit, and changing either invalidates the
   pair.** NEW-1 is the mirror image of last round's N1: there the terms were
   widened and the figure kept; here the figure was measured with a narrower
   expression than the one written down. Both are the same defect from opposite
   ends, which argues for stating the *exact* invocation beside any figure - flags
   and pathspec included - rather than a prose description of the surface.
3. **The `ledger-lint` unfencing pattern now has a third data point.** 548 / 550 /
   552 across three measurements in days. Worth appending as a reinforcing
   occurrence to whatever entry carries the pattern, because it is the rare case
   where the boundary can be shown rather than argued.
4. **The audit is stable, which is itself the signal.** An enumeration repaired
   twice by its own author held up against a reader who is not its writer, on an
   independently derived classification. The fifth kind is a fixed point and the
   construction generalizes: any enumeration over its own document needs a kind for
   its own classifying text, or it oscillates.
5. **Meta-text growth in a plan under repeated review**: 10% -> 12% -> 17% of the
   document across three rounds (2176 -> 3527 -> 5141 words). Not a defect, and not
   this round's business, but the plan close has a decision to make about where
   review history lives, and the doctrine's answer is already written.
6. **One figure in the sweep is not independently re-derivable, and that is
   acceptable but worth naming**: "362 spans and 113 fenced lines" describes the
   author's own extractor, not the tree. My equivalent extractor gives 381 spans,
   and the difference is method (double-backtick spans, fence-line handling), not
   disagreement. A method statistic is fine; it is simply the one figure in a sweep
   whose standard is re-derivability that a reader cannot reconcile, so it should
   say what produced it or drop the number.
