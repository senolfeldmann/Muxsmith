# Amendment 3 verdict - Plan-9 plan side (Task 3 carries the run_batch rustdoc restatement)

Reviewer: independent, did not author the amendment. Read at the tree
(HEAD `36d8538` at review time, which IS the amendment commit; the plan
file is byte-identical to it - `git diff 36d8538 -- <plan>` empty). All
instruments my own, under
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a3plan-rev/`.
Line numbers in this verdict are measured at this tree state.

## 1. Verdict: APPROVED_WITH_MINORS

One MEDIUM (a wrong ledger-id attribution the amendment writes into the
plan's amendment log), one LOW (one consumer enumeration the ripple sweep
missed). Both are single-line edits in the plan file; neither creates
latitude for Task 3's implementer, and neither blocks the Task-3 dispatch
provided the fix lands. Everything else - coverage, single-homing,
latitude closure, renumbering, the three ripple edits, the two decisions'
substance, every other added factual claim - verified clean at the source.

## 2. Findings

**MEDIUM-1 - the file-vs-within-file ruling is attributed to the wrong
ledger id.** Plan `:464` (amendment-3 log): "optional under the
file-vs-within-file ruling (`proc-latitude-clause-boundary`: the entry
carries no 'only', span or region qualifier, so it never constrained
within-file work)". The ruling does not live on that id. Evidence:
`docs/process-conventions.yaml` - `proc-latitude-clause-boundary` is the
entry at `:325`; its statement (`:331`) contains no within-file-qualifier
language, and its occurrence list ends 2026-07-27 (no 2026-07-28 entry).
The owner's 2026-07-28 ruling ("an entry constrains work WITHIN its file
only where it carries an explicit within-file qualifier - the word
'only', a named line span, a named region") lives in the statement of
**`latitude-carveout-zero-content-structural-forks`** (id at `:347`,
statement at `:353`, with the 2026-07-28 `decided` occurrence), adopted
in commit `d7fd277` - whose diff edits exactly that entry. The confusion
mechanism is visible: that statement OPENS with "Ruled:
proc-latitude-clause-boundary stays UNCHANGED", so a grep for the id hits
`:353` inside the other entry's statement; the cited id is the one entry
the ruling explicitly does NOT amend. The author's report claims it read
the ruling itself rather than the brief's paraphrase and cites ":353
(proc-latitude-clause-boundary)" - right line, wrong owning id; the
review brief for this review carries the same wrong id, inherited from
the author's artifacts (report 18:33 predates the review brief 18:36).
Violates the plan's own Global Constraint "cite entries by id" in the one
place dimension 7 exists to check: a factual claim the amendment adds to
the plan. **Required change, plan `:464`:** replace
"(`proc-latitude-clause-boundary`: the entry carries no 'only', span or
region qualifier, so it never constrained within-file work)" with
"(recorded on `latitude-carveout-zero-content-structural-forks`, which
rules `proc-latitude-clause-boundary` unchanged: the entry carries no
'only', span or region qualifier, so it never constrained within-file
work)".

**LOW-1 - Task 3's "Must not decide" enumeration was not swept.** Plan
`:266` names the section-5 entries touching Task 3 (D98 field shape,
errors token, fork 9, section 2's discarded failures, no logging facade,
"the four D99 Fluent texts and the D100 render semantics character for
character") but not the amended section-5 D96 bullet, which now states
"the fence in D96's amendment-3 rider is the contract, transcribed
character for character by Task 3" (design `:1486-1488`). Under Global
Constraints `:18` ("every task below inherits that list in full and names
the entries that touch it") and under the author's own ripple ground
("enumerations the new step falsified, each a surface the reviewer's walk
consumes"), this is the fourth such consumer, and the only one the sweep
missed. No latitude results: the obligation is deterministically present
in Step 2, the header, Read-first and the coverage map - which is why
this is LOW, not MEDIUM. **Required change, plan `:266`:** after "the
four D99 Fluent texts and the D100 render semantics character for
character;" insert "the `run_batch` rustdoc exactly as D96's amendment-3
rider fence writes it, character for character including its wrapping
(amendment 3);". **Explicitly ruled NOT required:** Task 2's must-not
list (`:212`, "the verbatim move with today's signature") needs no
qualifier - Step 1's in-place amendment-3 parenthetical two paragraphs
above covers the executed record, mirroring the design's own minimal
two-point sweep (entry opening + section 5 bullet); the fix round must
not over-annotate every historical mention.

## 3. Ruling on dimension 5 - the three ripple edits: ALL THREE IN SCOPE

Per edit:

1. **Task 3's header parenthetical** (`:216`, "D98, D99, D100, D96's
   amendment-3 rider"): the header is the task's design-citation
   enumeration; a task now implementing a D96-rider obligation with a
   header naming only D98-D100 misstates its design surface. In scope.
2. **Coverage map D96 row** (`:87`): the map's own preamble makes the
   call - "This is the walk the plan reviewer repeats; a row missing here
   is a defect." Without the row, my dimension-1 walk would have fired a
   finding (design obligation with no plan home in the map). Not editing
   it would have manufactured a defect. In scope.
3. **Sequencing 2->3 edge** (`:110`): the edge enumerates Task 3's
   `queue.rs` edits after Task 2; the new step falsified that
   enumeration. In scope.

Common ground, verified: the brief's freeze enumerated its members ("not
Task 3's other steps, not another task, not the Global Constraints") and
none of the three is a member; each edit is an enumeration rendered false
by the mandated change, so updating them completes the mandate rather
than extending it; and the design amendment swept its own analogous
consumers (D96 opening `:478-480`, section 5 bullet `:1486-1488`) - the
author applied the same principle one document down. The freeze read
strictly-literally would have required committing a plan whose own
coverage map contradicts it; that reading refutes itself. Note the
symmetric consequence: the same ground convicts the author of missing the
fourth consumer (LOW-1).

## 4. Ruling on dimension 6 - the two decisions: BOTH CORRECT

**Decision 1 (historical qualifier on "rustdoc moved with it"): correct,
correctly executed.** Verified at `:193`: the original clause stands
verbatim; the parenthetical is additive, provenance-marked ("amendment
3:"), points at Task 3 Step 2, and states exactly why the clause no
longer describes the end state. The stated reason reproduces: the design
closed the identical misreading in place (opening sentence `:478-480`
"amendment 3 restates the moved rustdoc for its new home ... body and
signature stay as-is"), and the Round-4 log (`:1744-1746`) names that
purpose. The steelman (purity of the executed record) was considered and
is correctly outweighed - the record IS preserved verbatim; only a
mislead is closed.

**Decision 2 (Files-entry parenthetical extended): correct in substance;
its citation is MEDIUM-1.** The extension (`:222`, sixth clause "the
`run_batch` rustdoc replaced with D96's amendment-3 rider fence") is
consistent with the owner's file-vs-within-file ruling as actually
recorded: the queue.rs entry carries none of the three qualifier forms
('only', named span, named region), so it never constrained within-file
work and the extension is genuinely optional; the added clause introduces
no qualifier either, so the entry stays descriptive - the normative
source remains Step 2, exactly the division the ruling draws. The stated
reason (a five-item work description omitting the sixth invites the
exhaustive misreading the T2 review measured two implementers resolving
by feel, in opposite directions) matches the ruling's own evidence record
in its 2026-07-28 occurrence. The one defect is where the plan says the
ruling lives (finding MEDIUM-1).

**Placement call (its own Step 2): sound.** All three stated grounds
reproduce: the work answers the D96 rider, not D98 (folding into Step 1
would break the one-to-one design-citation discipline); amendments 1 and
2 both added dedicated steps with renumbering (plan `:448` "renumbered to
seven", `:454` "new Step 3; steps renumbered to five"); and position 2
groups the two queue.rs doc edits (Step 1's licence-block rewrite, Step
2's rustdoc replacement) ahead of Step 3's "Build". The two deliberate
non-changes (no per-task `cargo doc`; the wrapping-fork closure) are both
correctly grounded - Task 1, the largest fence-transcriber, runs no
per-task `cargo doc` either, the rider states the link set is unchanged,
and the ten-part gate's `RUSTDOCFLAGS="-D warnings" cargo doc` binds
pre-push.

## 5. Per-dimension results

**Dimension 1 - coverage: PASS (one enumeration gap = LOW-1).** The
amended design places on the plan: (a) the replacement instruction, (b)
anchor-location not line numbers, (c) character-for-character
transcription, (d) no src-tauri file on this amendment's account, (e) the
rider as Task-3 reading. All five land in Task 3 Step 2 (`:240`) and the
Read-first line (`:218`). The rider has exactly one implementing task and
step: of the ten "amendment-3 rider" mentions in the plan (`:87`, `:110`,
`:193`, `:216`, `:218`, `:222`, `:240`, `:460`, `:462`, `:463`), only
`:240` carries an instruction verb; the rest are pointers or log
narrative. The amendment log binds via the Global-Constraints pointer,
verified as literal plan text at `:17` ("EVERY entry in its `## Amendment
log` bind this plan, at the log's state at EXECUTION time"). The one
unswept consumer is Task 3's must-not list (LOW-1).

**Dimension 2 - single home for the fence: PASS.** Six distinctive fence
phrases ("run lifecycle's core body", "tee-ing every", "per-event hook
carrying", "detached runner", "index-aligned to `specs`", "no teardown of
caller-side run state") each hit the design (fire: 1-2 hits) and hit the
plan zero times; a seventh pattern was discarded as a broken instrument -
its own design-side fire returned 0 because the phrase straddles a line
wrap inside the fence, exactly the failure a fire control exists to
catch. Second probe: `^/// ` lines in the plan -> 0 (grep exit 1 measured
on grep itself; design fire: 37). The plan points, never copies.

**Dimension 3 - latitude, both forms: PASS.** Step 2 and every touched
line scanned: no explicit-permission clause anywhere; no member to
invent - source (the rider's single fence: exactly one ``` pair in the
rider span, design `:575`/`:601`, so "the fence" is deterministic),
target (the `///` block above the `pub fn run_batch` anchor), extent
("everything else in the file outside this `///` block stays
byte-identical under this step" - correctly scoped to the step, since
Steps 1/3/7 edit the same file), and the src-tauri boundary ("on this
amendment's account" - correctly worded, since Step 3's EXEMPLARY
compiler sweep may legitimately touch other files) are all pinned. **The
wrapping instruction is unambiguous as written:** Task 1's allowance is a
real, quoted carve-out ("rustdoc line-wrapping is the only permitted
difference", `:157`); Step 2 names it and negates it ("line wrapping
included ... Task 1's rustdoc-line-wrapping allowance does NOT apply
here"), and grounds the negation in a fact I reproduced (fence pre-wrapped,
25 lines, max width 75 - my own awk; fire control: a known-long plan line
measured 1084). A Task-3 implementer has no fork to resolve.

**Dimension 4 - renumbering integrity: PASS.** Task 3's step headers
extracted mechanically: exactly Steps 1-11, contiguous, no gap, no
duplicate; all other tasks unchanged (T1 7, T2 7, T4 8, T5 8, T6 5, T7 6 -
matching the amendment-1/2 records). Every non-header step reference in
the plan classified: `:87` and the `:193`/`:465` qualifiers say "Task 3
Step 2" (new numbering, and Step 2 IS the restatement step); `:258` says
"Step-3 compiler sweep" (new numbering, and Step 3 IS the sweep); `:435`
and `:454` reference Task 6 Step 3 (untouched task); `:462-464` quote the
OLD name "Step-2 compiler sweep" only as the delta record's description
of what was renamed, and "Step-11 `git add` line" is correct (`git add`
block at `:255` under Step 11 at `:252`). The design contains zero step
references (grep exit 1 measured on grep itself; fire control `D96` -> 8
hits). SDD scratch swept: `progress.md` (controller-rewritten after the
author's report flagged it empty) uses the new numbering ("Task 3 gains
Step 2", `:11`/`:27`); stale old-numbering references survive only in
dated archives - `amendment-3-report.md:176` ("before Step 9") and
`amendment-3-verdict.md:153` ("Step 10's `git add` line") - which are
stale by construction (both predate the renumbering; the review brief
itself declares their line cites stale) and are consumed as historical
evidence, not current instruction. No live dangling reference exists.

**Dimension 7 - accuracy: PASS except MEDIUM-1.** Every factual claim the
amendment adds, verified at the source: `:327-347` at amendment time is
true at this tree (`queue.rs` contiguous `///` block `:327-347` - every
line rustdoc, verified mechanically - `pub fn run_batch` at `:348`, and
the three falsified passages still on disk at `:332`, `:334-335`,
`:338-342`, so Step 2 has a real pre-state); fence pre-wrapped max width
75 (measured, 25 lines); "no src-tauri file ... the rider adds no
src-tauri sentence" true (rider text) and the three caller-side rationale
sites spot-checked in `src-tauri/src/run.rs` (`finish_teardown` doc at
`:641-650`, `TeardownGuard` at `:668` region, runner-thread comment at
`:447-454`); queue.rs on Task 3's Files list (`:222`) and Step-11 `git
add` (`:255`); `src-tauri/src/run.rs` absent from the Files region
(`:221-232`) verified as a positive enumeration - its plan hits are
`:54`, `:57`, `:148`, `:169`, `:185`, `:199`, `:206`, `:464`, none inside;
the refuted controller-brief claim exists as described
(`amendment-3-brief.md:127-128`: "that file is in Task 3's Files list");
Task-2 review MEDIUM-1 is what the log says it is (task-2-verdict `:33`:
"the moved rustdoc asserts things that are false about the function it
now documents"); the design delta review is APPROVED with no findings;
"the two logs are deliberately not numbered in lockstep" matches the
design's Round 4 (`:1721-1723`, "amendment 2 was plan-only and lives in
the plan's own log"); "steps renumbered to eleven, recounted from the
headers" reproduced by my own recount. The one false added claim is the
`proc-latitude-clause-boundary` attribution (MEDIUM-1). Typography over
the whole plan: zero banned glyphs (pattern fire-verified on an em-dash
sample, count 1). The commit is pathspec-scoped to the plan file, six
hunks, all graded; no untouched-but-changed region.

## 6. Evidence appendix

Scratch root:
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a3plan-rev/`

- `amendment.diff` - `git show 36d8538` (6 hunks, 1 file; graded at the
  current file, which is byte-identical to the commit).
- `task3-steps.txt` / `all-steps.txt` - mechanical step-header extraction
  (T3 = 1..11 contiguous; other tasks' counts unchanged).
- `plan-step-refs.txt` - every non-header step reference in the plan,
  classified above.
- `sdd-step-refs.txt` / `sdd-step-refs-broad.txt` - the scratch-wide
  sweep; design absence measured on grep's own exit (1) with `D96` fire
  control (8).
- `fence-dup-check.txt` - six firing phrase probes, plan 0 / design >0
  each; the discarded seventh recorded as the broken-instrument case.
- `queue-prestate.txt` - `queue.rs:320-352`; block span, anchor, and the
  three falsified passages.
- `fence-extract.txt` - design `:576-600`; `awk` measured lines=25
  maxwidth=75; fire control: plan line 9 = 1084 chars; single ``` pair in
  the rider span verified (`:575`/`:601`).
- `files-region-check.txt` - Task 3 region bounds, Files region, the
  src-tauri positive enumeration, queue.rs hits.
- `rider-mentions.txt` - the ten mentions, one implementing.
- `typo-scan.txt` / `typo-fire.txt` - banned-glyph scan (0, exit 1) and
  its fire (1).
- MEDIUM-1 evidence: reads of `docs/process-conventions.yaml:325-354`,
  `git show d7fd277` on both files (the ruling text and its 2026-07-28
  occurrence land in `latitude-carveout-zero-content-structural-forks`;
  the ledger addition is the unrelated hash-pinning entry).

## 7. HARVEST

- **Task 3's dispatch must carry:** Step 2 verbatim (anchor-location, the
  character-for-character-including-wrapping contract, the negated Task-1
  allowance, the no-src-tauri boundary, the byte-identical-outside-the-
  block scope) and the Read-first rider addition - and it must NOT paste
  the fence text: the fence is single-homed in the design; the
  implementer transcribes from D96's amendment-3 rider at its
  execution-time state. It should also carry `progress.md`'s "Carried
  into Task 3" block (the post-`9b2843f` anchors: `recover_panicked_worker`
  `queue.rs:424`, the `eprintln!` `:441`, the panic test `:783`; the
  discarded-failures constraint; the import-removal doc-link sweep).
- **Task 3's reviewer verifies the transcription mechanically:** extract
  the new `queue.rs` block and `diff` it against the rider fence's
  content lines (design `:576-600` at this tree; re-verify the span
  then), expecting empty - wrapping included - plus byte-identity of the
  file outside the block under Step 2.
- **Citation lesson, the second facet of the author's own surfaced
  item:** a ruling cite wants the OWNING entry's id, and an id that
  merely appears inside another entry's statement is not the home. The
  readable trigger: the grep hit containing your quoted language sits
  below a different `- id:` line - walk up to the owning id before
  citing. This cycle produced both facets (the design verdict cited a
  bare hash; the plan author, correcting that, cited the id named INSIDE
  the ruling's statement). Worth one ledger occurrence when the
  controller mines this cycle; the wrong id also sits in the author's
  report and in this review's own brief (dated scratch - no retroactive
  edit; only the plan needs MEDIUM-1's fix).
- **Fix-round scope guard:** exactly two line edits (plan `:464` per
  MEDIUM-1, plan `:266` per LOW-1); Task 2's must-not list stays
  untouched (ruled above); no other surface was found wanting.
- Stale Task-3 step numbers exist only in dated scratch archives
  (`amendment-3-report.md:176`, `amendment-3-verdict.md:153`); no action.

---

# Delta verdict (fix round, commit `63fc5b2`): APPROVED

Same reviewer, resumed; judged at the tree (HEAD `63fc5b2`, plan file
byte-identical to it - `git status` clean, `git diff 63fc5b2 -- <plan>`
empty). Instruments in the same scratch root (`fix-round.diff`).

**Both findings closed at the file.**

- **MEDIUM-1 closed.** Plan `:464` now reads "(recorded on
  `latitude-carveout-zero-content-structural-forks`, which rules
  `proc-latitude-clause-boundary` unchanged: the entry carries no 'only',
  span or region qualifier, so it never constrained within-file work)" -
  my proposed wording verbatim. The re-attribution is factually right at
  its source, verified in the original round and unchanged since: the
  ruling's text and its 2026-07-28 `decided` occurrence live on
  `latitude-carveout-zero-content-structural-forks`
  (`docs/process-conventions.yaml` id `:347`, statement `:353`; commit
  `d7fd277` edits exactly that entry), and the new clause's "rules
  `proc-latitude-clause-boundary` unchanged" is what that statement
  literally opens with.
- **LOW-1 closed.** Plan `:266` carries the insertion verbatim, at the
  specified position: "...the D100 render semantics character for
  character; the `run_batch` rustdoc exactly as D96's amendment-3 rider
  fence writes it, character for character including its wrapping
  (amendment 3); no other user-visible string changes." Task 2's
  must-not list (`:212`) is untouched, per the explicit ruling.

**Scope held.** The commit is pathspec-scoped, one file, exactly two
hunks (`@@ -263` and `@@ -461`), each replacing one line - precisely the
two lines the findings named; nothing else in the plan changed.

**No passed dimension re-opens.** The remaining
`proc-latitude-clause-boundary` cite at `:19` (Global Constraints,
fork-closure/NEEDS_CONTEXT routing) is correct as it stands - that IS
that entry's own statement content (`:331`) - and stays. Single-homing
holds: the inserted `:266` clause names the fence and copies none of its
text (instruction language already present in Step 2). No step header,
number, or reference touched, so renumbering integrity stands as passed.
Typography of both changed lines: zero banned glyphs (exit 1 measured on
grep; pattern fire-verified this session, sample count 1).

The plan-side half of amendment 3 is APPROVED with no open findings.
