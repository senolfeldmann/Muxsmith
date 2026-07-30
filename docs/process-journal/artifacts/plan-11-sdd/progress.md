# Plan 11 - progress tracker

SDD ledger, plan: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`

The plan document carries no progress (house deviation stated in its header);
this file is the tracker. Controller-written. A task's state is its row here,
never a ticked box in the plan.

Execution session: session 30, 2026-07-30. Base at kickoff: `5378264`
(master == origin/master, clean tree). Worktrees created by the controller:
`../muxsmith-plan11-a` on `plan-11-stream-a`, `../muxsmith-plan11-b` on
`plan-11-stream-b`, both off `5378264`.

## Owner rulings at this session's kickoff (2026-07-30, source: human)

Recorded here at creation because all three shape how this run proceeds while
the owner is away from the machine.

1. **Plans 11 and 12 run back to back without an intermediate check-in.** The
   ruled six-step order is unchanged; only the waiting point between the two
   plans is removed.
2. **The draft build for his QA round is controller-triggered** after Plan 12
   lands: `workflow_dispatch` on `release.yml` with the draft flag, no tag,
   nothing published.
3. **A fork that would normally route to him is PARKED with its decision memo
   and the rest of the run continues**; the parked questions are presented in
   one batch on his return. The run halts only if everything depends on the
   answer. Product-scope decisions are still not the controller's to take -
   parking is deferral, not delegation.

## Controller obligations carried into this run

- **Task A3's dispatch carries ADR D111 as required reading** and states the
  cross-arm invariant as its own point: `scalar_eq`'s int/float cross arms
  STAY (typed `exact` path); only the `raw:` call site re-points to
  `scalar_eq_same_type`. T-1 is the safeguard that catches a strip.
- **Before Plan 12's pre-execution gate**, after Plan 11 merges: re-verify that
  every fenced OLD string in Plan 12 still occurs exactly once, and re-run
  every tree-measured figure in it. Different spec sections means no textual
  conflict is EXPECTED; expecting is not measuring.

| # | Task | State | Commits | Verdict |
|---|---|---|---|---|
| A1 | W5: `BUILDING.md` ordinals + reflow | DONE | `a0d5d3e` (base `5378264`) | `task-a1-verdict.md` APPROVED_WITH_MINORS, no fix round; 1 Important routed as a controller action, 4 minors below |
| A2 | W2: the two surviving line citations | DONE | `5d305a2` (base `a0d5d3e`) | `task-a2-verdict.md` APPROVED, no fix round; 5 INFO findings, none a product byte |
| A3 | W3: the `raw:` comparator, twelve sites, three tests | DONE | `164e571` (base `5d305a2`) | `task-a3-verdict.md` APPROVED_WITH_MINORS, no fix round; reviewer reproduced both mutations and ran a counterfactual binary |
| A4 | W4: the v1 spec's 8.1 CLI synopsis | DONE | `06e896e` (base `164e571`) | `task-a4-verdict.md` APPROVED_WITH_MINORS, no fix round on the artifact; 1 Important is report-evidence only, correction round dispatched |
| B1 | W1: the two dependency alerts | DONE | `c422999` + `5bf65dc` | `task-b1-verdict.md` APPROVED_WITH_MINORS; 3 Important, all PLAN defects -> Amendment 5 (APPROVED after its own fix round) -> `5bf65dc` applies the corrected fence; controller verified one `unsound` key, 19 ids, both falsehoods gone |

## Deferred minors and parked findings

**Task A1** (`task-a1-verdict.md`, APPROVED_WITH_MINORS, no fix round; all five
disposed here at verdict arrival, none carried silently):

1. **Important, and it was a controller action rather than an A1 fix.** The
   ROADMAP's mise rider is gated on "the next `ci.yml`-touching change - the edit
   is the trigger", and its fenced replacement text writes `Rust gate part 6`
   into the exact comment block Task A2 is authorized to edit. Applied as
   written it would have undone A1 in another file, invisibly to every check A2
   runs. **DISCHARGED before A2 was dispatched:** the ROADMAP entry now records
   the trigger as FIRED and deliberately RE-DEFERRED with a new observable, the
   fenced text is deliberately NOT repaired (a replacement string for a source
   comment is product content, which the controller does not author), and A2's
   brief carries the prohibition verbatim as a binding cross-task constraint.
2. Minor, report quality: A1's residual-sweep section enumerated 9 of the 13
   lines its own sweep returned and pasted no command. All 13 are benign, so no
   disposition moves - but one of the four unnamed is `docs/ROADMAP.md:1793`
   ("gate part 4"), a genuine instance of the positional-ordinal class living in
   the live tracker. **Controller close action** together with the other ROADMAP
   dispositions; A1's own surface was `BUILDING.md` and closing it there was
   correct.
3. Minor, report quality: a passage mixes end-state and pre-state line numbers
   in adjacent sentences, each true alone. No tree consequence.
4. Minor, plan defect: Step 3's unfireable soundness control (item 4 above).
   The reviewer ruled acceptance row W5-a SATISFIED by the substituted control
   and the amendment NOT a precondition for done, and supplied exact replacement
   text naming a document measured to contain two digit-form matches rather than
   the plan-11 document, whose count moves on every amendment.
5. Minor, plan defect: the "one live consumer" claim, understated by one
   (item 2 above). Rides the same amendment.

**Task A2** (`task-a2-verdict.md`, APPROVED, no fix round; five INFO findings,
none of which changes a product byte, all disposed here):

1. The report renders A1's "run it, and the green means less than it looks" as a
   constraint NOT to run it - attribution compression. No tree consequence; the
   reviewer measured the run anyway (0.39 s, exit 0) and confirmed the cost was
   never the reason. **A4's exit bar deliberately DOES include `ledger-lint`; the
   A2 adjudication is not licence to drop it there.**
2. `ci.yml`'s new comment has a short line inside a wider paragraph - the plan's
   fenced text verbatim, so not the implementer's defect. For whichever change
   next touches that block.
3. The commit subject says "outside docs/" where the plan says "every TRACKED
   file outside docs/". Coincides today (zero untracked non-`docs/` files), but
   **the plan close's ROADMAP disposition must carry the word "tracked"** or it
   repeats the over-claim the entry exists to prevent.
4. `cargo test --workspace` was the one implementer claim the reviewer did not
   fully reproduce; it ran the D48 guard and proved structurally that the result
   cannot differ.
5. **The class is closed on a WIDER surface than the plan asked for.** The
   reviewer probed three further citation forms the plan's expressions cannot see
   (`#L<n>`, prose `at line`/`L`/`@`, `L<n>-L<m>`) plus the extension-excluded
   file types: all zero on the tree, each against a control it built.

**Surfaced by that review for the close:** measured residue UNDER `docs/` is 566
journal artefact files, 16 plan documents and **12 design/spec documents**. The
last group is the interesting one - governed by `code-comment-line-citations-drift`'s
two-class rule and closed by nothing. Owner-visible question, not this plan's work.

**Task A3** (`task-a3-verdict.md`, APPROVED_WITH_MINORS, no fix round; three
findings, none a product-file change, all disposed here):

1. **A false-coverage test NAME, pre-existing and correctly untouched.**
   `matcher.rs`'s `numeric_exact_compares_across_int_and_float` compares only
   `Int` against a reported `Int`, so it stays green under the mutation its own
   name describes - sitting in the same group as the safeguard Plan 11 just added
   to catch that mutation. Outside every D111 fence and outside A3's Files list,
   so A3 was right to leave it; absent from D111, the brief and the report's
   surfacing list, which is why it needed routing. **VEHICLE WRITTEN: a fourth
   member of Plan 13's floor**, in the ROADMAP, with the choice between rename
   and widen left to its implementer.
2. Minor: two of the six pathspec members in D111's own M4 instrument match
   nothing under git's wildcard rules, and D111's stated fire used a different
   tool so it never touched the dead members. **The conclusion survives** - the
   reviewer re-derived it tree-wide and the suite is green. Evidence defect, not
   artifact defect. **Close action: record it so no later sweep reuses the
   expression.**
3. Info: D111's ground for shipping no e2e test ("would add an mkvmerge
   dependency") is false - eight `have_mkvmerge()`-gated files already exist. The
   conclusion stands on the other ground stated beside it, which the reviewer ran.

**Also from that review, for the close:** D111's two citations of one ROADMAP line
are stale by nine lines (it cites by wording, which is byte-identical, so the
control still discriminates - note belongs with D111's trigger T6); the corpus
discriminator's grep-vs-parse divergence is a FALSE POSITIVE on the README
passthrough example's inline flow mapping, and belongs on the example-validation
vehicle's open design question; and D111's trigger T7 is independently
strengthened - the two probes that flipped are exactly the cases the proposed
config-time guard would catch.

**Amendment queue (one amendment, authored by the plan's author and judged by
its original reviewer, batched rather than one per finding):** A1 minors 4 and 5,
plus B1's fenced wrong count and its inverted `glib-macros` claim (items 6 and 7
above). Ordered after B1's verdict so the reviewer's replacement wording for the
`deny.toml` comment is in hand before the author is resumed.

**Amendment 5 LANDED** `44bc6f7` on `master`, one file, pathspec-scoped, all six
defects plus the two minors across 17 sites. **In delta review.** Verified by the
controller before dispatching the review: nothing in Task A2, A3 or A4 territory
moved, which matters because A3 is executing against a worktree copy that
predates the amendment.

**Amendment 5 delta review: NEEDS_FIXES, fix round 1 dispatched to the resumed
author.** Substance sound - all six defects repaired at every site, the reviewer
reproducing each site set independently, the fence byte-identical to the B1
verdict's wording and verified true sentence by sentence at cargo-deny's own
source, and Tasks A2/A3/A4 plus Amendments 1-4 byte-identical, so A3's worktree
copy holds no superseded contract. Two Important findings, both of which would
have hit the B1 fix round:

- **The placement instruction does not resolve against the file the fix round
  holds.** At `c422999` the two anchors are eight lines apart with the SHIPPED
  fence between them, its own `unsound` key included. The reviewer fired both
  readings: a literal insert makes `cargo deny` fail with `duplicate key:
  unsound`; a region replace gives `advisories ok`. Restated as a replacement of
  the eight lines between the anchors.
- **A new wrong figure, of exactly the class the amendment exists to repair.**
  `deny.toml`'s longest line is 77 on `master` and **78 at `c422999`**, at three
  lines B1's own second fence added - and `c422999` is the state the replacement
  lands in. The B1 verdict's 78 was right for the state it reviewed; the
  amendment measured the other state, named neither, and escalated a verdict
  error that does not exist. **Rule the reviewer drew and worth keeping: a
  disagreement with a source figure is a STATE-or-UNIT question before it is an
  error.**

**Two minors the amendment verdict routes OUTSIDE the plan, both controller
work:**

1. **B1's report still asserts the refuted silence** (two lines) and still
   carries two numbered findings where its verdict required a third. Routed into
   the B1 fix round's scope, which has to touch that report anyway.
2. **A1 adjudication 5's vehicle reconciliation.** The ROADMAP's over-80-line
   item names as its vehicle "whichever package next edits `BUILDING.md`'s GATE
   BLOCKS"; Task A1 discharged it by editing that file's PROSE and touched no
   gate block. The work is done and the condition was never met, so the entry
   needs the reconciliation recorded rather than silently ticked. **Controller
   close action.**

**Amendment 5 APPROVED** after one fix round (`ba67cbc`), all three findings
ADDRESSED, no new breakage; the reviewer re-fired both readings of the restated
operation from scratch and confirmed the postcondition discriminates (1 key on
the good form, 2 on the broken one). Two things from that round worth keeping:

- **The insert-wording defect had FOUR sites, not the one the finding named.**
  The author closed it with a vocabulary sweep rather than by repairing the named
  instance; the reviewer's own independent sweep returned 14 insertion-vocabulary
  lines, of which exactly four describe that operation, and confirmed all four
  moved while a genuine append elsewhere was correctly left alone. Third time
  today that the complete site set came from searching rather than from working a
  list.
- **The reviewer retracted one of its own challenges**, and the retraction is the
  instructive part: its `grep -c '18 commented'` returned 0 because both ROADMAP
  sites hard-wrap between the figure and its noun. So **concern 4 stands: those
  two sites assert 18 and are stale once stream B merges.** Ledger occurrence
  written against `proc-wrapped-prose-quote-grep`.

**Controller ruling on the round's one deferred minor:** B1's `Modify:` files list
still describes the `unsound` key as "added to `[advisories]`". That is
descriptive scope bounding rather than an operation, it was true of B1's original
mandate, and every bounding clause survives the replace. **No change.**

**A controller defect the amendment author caught, recorded because it is the
same class this plan keeps producing:** the amendment brief asserted that "three
of the six defects appear at more than one site" - a claim the controller made
without measuring. Searching found a fourth (the guard premise, at three lines
plus four restatements, one invisible to a line-based grep because a table row
carries it in two columns). The author fixed it and said so. Same shape as the
A1 brief's wrong restore instruction: a controller instruction stating a set size
it never derived.

## Surfaced by the controller during this run

1. **Six worktrees from earlier plans were never torn down** and still sit at
   `.worktrees/plan75-a`, `plan75-b`, `plan8-a`, `plan8-b`, `plan8-c`,
   `plan8-d`. Measured 2026-07-30: every one has a clean working tree
   (`git status --porcelain` empty), and every one of their branches is an
   ancestor of `master`, so nothing is unmerged and nothing is uncommitted.
   They are the residue of two plan closes whose teardown step did not run.
   Not removed by this session: the removal is not this package's work and
   nothing depends on it. Owner disposition at the next report; the safe form
   is `git worktree remove` for the six while KEEPING their branches.

2. **A THIRD live consumer of the "part 6" wording exists, against the plan's
   "one live consumer" claim.** Task A1's implementer found it and the
   controller reproduced it: `docs/ROADMAP.md` around line 2725 carries a fenced
   forward-looking prescription - an exact replacement text for the next
   `ci.yml`-touching change - citing "BUILDING.md, Rust gate part 6". This is
   NOT the historical-record class the ROADMAP's MEASURED block protects:
   applying it as written would newly write a citation that A1 has just made
   false. **Controller close action** (the ROADMAP is controller-owned), to be
   disposed of with the other ROADMAP dispositions at the plan close.

3. **The ledger's positional-reference entry is NOT stale, ruled by the
   controller.** `docs/decision-ledger.yaml`'s entry against positional
   references uses "gate part 6" inside its own statement as an ILLUSTRATIVE
   EXAMPLE of the class it describes, not as a citation of `BUILDING.md`'s
   current text. An example of a pattern is not falsified by the pattern being
   removed at one site; the entry's own closing sentence keeps its scope
   unchanged. No edit. Routed to the A1 reviewer as an adjudication question
   anyway, so the ruling is not the controller's alone.

4. **The plan's Step-3 soundness control for absence check O cannot fire.**
   Reproduced by the controller: the plan names
   `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` as the
   known-present control for the DIGIT-form expression `part [0-9]|parts [0-9]`,
   and that file returns 0 for the digit form and 3 for the spelled form - the
   plan's own authoring section names the same file as the control for the
   SPELLED sweep, which is where it belongs. Measured to be contained to Task
   A1's step: the controls named in A2's and A3's steps have different targets.
   The implementer did not adjust the plan and substituted the authoring
   section's own control. Whether an amendment is owed is the A1 reviewer's
   adjudication, not the controller's to settle.

5. **The A1 brief carried a wrong restore instruction, and the defect is the
   controller's.** `git checkout -- BUILDING.md` restores from HEAD, and at the
   point in the task where the fire runs, the deliverable is still uncommitted -
   the instruction would have discarded it. The implementer refuted it and
   substituted a sha256 baseline of the edited file. Recorded as a dated ERRATUM
   block inside `task-a1-brief.md` rather than by silently rewriting the
   instruction, because the brief is the artifact the reviewer grades against.
   Ledger-worthy at the plan close: a controller brief that prescribes a
   restore mechanism must state WHICH state it restores to.

6. **A SHIPPED `deny.toml` comment carries a wrong count, and the plan fenced
   it.** The fence says `unmaintained` "(default `all`) reported its 18".
   Reproduced by the controller on `master`, directly:
   `note[advisory-ignored]` 18, `note[unmaintained]` **16**,
   `note[vulnerability]` **2**. So 18 is the ignore-ENTRY count and the sentence
   attributes it to the unmaintained CLASS - two different sets. The implementer
   applied the fence verbatim, as the plan requires, and raised it rather than
   correcting at the keyboard. **Route: the B1 reviewer supplies the replacement
   wording, then a one-pair plan amendment carries it, then a B1 fix round
   applies it.** The controller writes neither the fence nor the file.

7. **The plan's "twelfth consumer" claim is inverted.** It states that
   `glib-macros 0.18.5` consumes `glib` over a proc-macro edge `-e normal`
   excludes. Reproduced by the controller: `cargo tree -i glib-macros@0.18.5
   -e normal --depth 1` shows `glib` as the DEPENDENT, and `glib`'s own
   `--depth 1` dependency list contains `glib-macros`. The direction is the
   other way round. The eleven-parent figure the task acts on is unaffected and
   no decision moves; the caveat sentence is wrong. Rides the same amendment as
   item 6.

8. **B1's change makes a repeated fact stale in two ROADMAP sites.** The ignore
   list now holds 19; `docs/ROADMAP.md` asserts "18 commented RUSTSEC ignores"
   twice - in the Renovate trigger entry and in the activation entry's riders.
   Both straddle a hard wrap, so a line-based grep cannot see either; found with
   a newline-flattened pass and reproduced by the controller the same way.
   **Controller close action.** The handle for whoever repeats it: search the
   flattened text for the fact, not the line.

## Plan close (2026-07-30, session 30)

**Merges and gates.** Stream A merged first (`82bd016`), then stream B
(`73d3de2`), in the order the plan fixed and for its stated reason: stream B is
the one whose gate result can move for reasons outside anybody's diff, so
merging it second makes the bisect free if the merged state goes red. The full
eleven-part gate as `BUILDING.md` enumerates it ran three times - stream A's own
worktree, the state after merging A, the state after merging B - each part's
exit code captured separately rather than trusting an aggregate. All three green:
**507 Rust tests over 39 suites, 68 e2e cases, zero failures.** The test count is
an independent check on Task A3: 505 at the session-29 close, +2 for the two
added tests, the third having replaced an existing one in place.

**Whole-branch review: READY_WITH_MINORS**, top tier, no product byte to change.
It re-derived A3's twelve-repair / seven-retain split itself from D111's own
fenced expressions rather than trusting the pasted result, and confirmed the
mis-pasted-evidence pattern found in A4's report did NOT extend to any other
report. It also reproduced the safeguard mutation: stripping the cross arms
fails exactly one test across every core and CLI suite.

**Push:** one, at the close, `5378264..71cce6a`, behind the second merged-state
gate. `gh-log.md` entry written.

**The five must-fix-before-push minors it raised, all controller work, all done:**
the two merge commits' missing trailer (recorded, not rewritten - the SHAs are
cited in three places and re-SHAing them to add a provenance line would falsify
five citations); A4's verdict harvest mined; the two promised ROADMAP lines
written; the four A3-review close records written; the `cli.rs` deferral given a
ROADMAP trigger of its own rather than an unwatched event.

**Blocked-pool sweep** over all 18 `status: blocked` entries in the four
house-knowledge files. One stale, and it was the one that had predicted this very
close: `a-count-a-close-action-moves-needs-growth-proof-phrasing-or-a-standing-duty`
expected plan 11's salvage to falsify the README's artifact count exactly as plan
10's had. It cannot - commit `d9a4fa2` retired that figure, taking one of the
entry's own two named options without anyone recording it against the entry.
Settled with a resolving occurrence; verified at the artifact that no remaining
README numeral is a figure a salvage could move.

**Worktree teardown:** both plan-11 worktrees removed. The six legacy worktrees
from Plans 7.5 and 8 stay, as the whole-branch triage ruled - clean trees, all
branches ancestors of master, and not this package's work.
