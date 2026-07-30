# Task A1 verdict - Plan 11, stream A (W5)

**Verdict: APPROVED_WITH_MINORS**

**Graded:** worktree `/home/senol/Git/muxsmith-plan11-a`, branch
`plan-11-stream-a`, head `a0d5d3e28d44ee464dd9175ea11e5446e9b0dc0c` over base
`5378264`. Files read, not a commit description. Tree byte-identical to
`a0d5d3e` at review start and at review end (proof in the appendix); every
mutation this review performed ran on reviewer-owned copies under
`/tmp/.../scratchpad/a1rev-independent/`, and the tracked tree was never
written to.

**The deliverable is exact, and the proof is stronger than a line-by-line
comparison.** I extracted the four fenced blocks of the plan's Step 2
programmatically, applied both substitutions to `git show 5378264:BUILDING.md`
in my own process, and compared the result byte for byte against the committed
file:

```
DERIVED (pre-state with both fenced substitutions applied) == end state : True
```

That single equality discharges contract compliance (both replacements went in
verbatim, character for character) and scope (nothing else in the file moved by
one byte) simultaneously. `git diff --name-status 5378264..a0d5d3e` names one
file. No marker line, no fence line, no line inside a fence, and not the
canonical gate-total sentence appears among the 15 changed lines, verified by a
reviewer-written `-U0` hunk parser whose four assertions were each fired
separately and each restored.

The five findings below are two report-completeness defects, two plan defects,
and one sequencing hazard for the next task. **None of them is a defect in the
edit**, and none requires a fix round on A1.

---

## Findings

### 1. Important (controller action BEFORE A2 is dispatched, not a fix for A1) - the ROADMAP rider that A2 is about to trigger will write back exactly what A1 removed

`docs/ROADMAP.md:2718-2726`, in the post-1.0 "Remove mise from CI" item:

> **Rider, gated on the next ci.yml-touching change whichever it is** (this
> item is the expected carrier; an earlier ci.yml edit inherits the duty -
> the edit is the trigger)

and its prescribed text:

> Exact replacement, so nobody re-derives it: `# legs, matching the
> cross-target clippy gate part (BUILDING.md, Rust gate part 6; cfg-gated
> items can differ per platform).`

Task A2's EXHAUSTIVE Files list authorizes editing
`.github/workflows/ci.yml` - "the `cargo doc` step's leading comment block,
comment text only". That block is `.github/workflows/ci.yml:88-98`, and lines
`:92-93` are precisely the two lines the rider replaces:

```
      # legs, matching the cross-target lint rule (cfg-gated items can
      # differ per platform).
```

So **A2 is the next ci.yml-touching change, and its authorized write region is
the rider's target region.** If the rider is applied as written, it writes
`BUILDING.md, Rust gate part 6` into a CI comment: a positional gate ordinal
citing a position `BUILDING.md` no longer states, inside the CI-and-configuration
comment class the owner's session-28 widening put in scope, in the same stream
that just removed that class from `BUILDING.md`. The failure would be invisible
to every check A2 runs, because no gate part reads `ci.yml`'s comments.

The implementer surfaced `:2725` (report 5.4) and correctly did not edit it. It
did not surface the trigger collision, so the controller would have to re-derive
it. **Required change:** dispose of `docs/ROADMAP.md:2725` before A2's dispatch,
not at the plan close, and state in A2's brief whether it inherits the rider
and, if so, with what replacement text. The natural repair of the rider is to
drop the position: `# legs, matching the cross-target clippy gate part
(BUILDING.md's Rust gate block; cfg-gated items can differ per platform).`
Wording is the controller's, not mine; the point is that the rider must not
enter A2 in its current form.

Not a defect in A1, and A1 could not have repaired it: the plan forbids a task
editing the ROADMAP.

### 2. Minor - report 5.6's residual enumeration lists 9 of the 13 lines its own sweep returns

`.superpowers/sdd/plan-11/task-a1-report.md`, section 5.6:

> The digit-form sweep over live files (excluding plans and the process journal)
> returned, besides the three above: two `docs/IDEAS.md` hits about *file* parts
> in an appending feature, one `decision-ledger.yaml` occurrence `ref` reading
> `part 3`, and one `decision-ledger.yaml` statement using `gate part 1` as an
> example. None is a claim about `BUILDING.md`'s ordinals.

Re-run in the worktree, the sweep the report describes returns **13** lines:

```
$ git grep -nE 'parts? [0-9]' -- ':!docs/superpowers/plans' ':!docs/process-journal*' | wc -l
13
```

Sections 5.1, 5.2, 5.4, 5.5 and 5.6 together account for nine of them. Four are
unaccounted for:

| line | text | class |
|---|---|---|
| `docs/ROADMAP.md:182` | `1.0 [superseded in part 2026-07-27, ...` | idiom "in part", pattern false positive |
| `docs/ROADMAP.md:1793` | `Measured 2026-07-28 (Plan 9 Task 1 review, HARVEST): gate part 4` | **genuine positional gate ordinal**, dated historical record under `:1791`'s "The record below is the history", and it names the command in the same breath |
| `docs/ROADMAP.md:2722` | `cross-target lint rule became gate part 6 (commit bcb67f3). Nothing there` | dated historical record; quoted inside 5.4's context block but never classified |
| `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:1598` | `**Superseded in part 2026-07-27:** the tree is authoritative ...` | idiom "in part" |

**Every one of them is benign under the report's own reasoning**, so no
disposition changes and the sentence "None is a claim about `BUILDING.md`'s
ordinals" survives. What does not survive is the enumeration that carries it:
`:1793` is a live tracker line carrying `gate part 4`, exactly the shape the
sweep exists to classify, and it is not in the list. Section 5.6 also pastes no
command for its run, which the plan's `design-empirical-claims-reproducible`
constraint requires of an observed value.

**Required change:** none to the deliverable. Report 5.6 should paste the sweep
command and its full 13-line output, or state the residual set as measured
rather than enumerated. Controller may record it and move on.

### 3. Minor - report 4.2's must-ignore list silently switches state mid-passage

`.superpowers/sdd/plan-11/task-a1-report.md` section 4.2 lists the eight fenced
over-80 lines as `29, 36, 87, 89, 116, 123, 159, 160`. Those are the **end
state's** numbers, measured:

```
PRE-STATE  @5378264: ALL lines over 80 at [29, 36, 87, 89, 116, 123, 138, 158, 159]
END-STATE  @a0d5d3e: ALL lines over 80 at [29, 36, 87, 89, 116, 123, 159, 160]
```

The next sentence attributes the arithmetic to the pre-state ("9 over-80 lines
total in the pre-state, 8 of them fenced, 1 not"), where the same two lines are
`:158` and `:159`. Both statements are individually true; read together the
numbering changes state without saying so. Changes no conclusion.

**Required change:** label which state each list belongs to. Report-only.

### 4. Minor (plan defect, amendment routed to the plan's author and original reviewer) - Step 3's soundness control for absence check O cannot fire

Reproduced independently, on `/usr/bin/grep` rather than the shell's ugrep
function:

```
$ /usr/bin/grep -nE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
exit=1                      # file present, 40122 bytes
$ /usr/bin/grep -nEio '(first|...|twelfth)[ -](gate|part)|(gate|part) (one|...|twelve)\b' <same file>
7:ninth gate
18:part nine
271:third-part
315:ninth gate
319:part nine
323:ninth gate
```

The file carries only spelled ordinals. See adjudication 1 for the amendment
text and for why this does not block W5-a.

### 5. Minor (plan defect, same routing) - the "one live consumer" claim at the authoring section is understated by one

The plan states "**The one live consumer of the 'part 6' wording is a Tier-2
statement.**" `docs/ROADMAP.md:2725` is a second and materially different one.
See adjudications 3 and finding 1.

---

## The five adjudications

### Adjudication 1 - the unfireable soundness control

**Verdict: acceptance row W5-a IS satisfied by the substituted control. The plan
needs an amendment, but the amendment is not a precondition for calling A1
done.**

Argued in both directions.

*For "the plan must be amended first."* Step 3 names a specific file as the
known-present control. The implementer used a different one. The plan's Global
Constraints say every fork is closed and that a fork discovered on code contact
returns as NEEDS_CONTEXT, never resolved at the keyboard. On that reading, the
control choice was a step the implementer executed differently from how it was
written, and a reviewer who accepts a substituted control accepts a precedent
that any prescribed check may be swapped when it is inconvenient.

*Against, and this is where I land.* Three things break that reading.

1. **The row does not depend on the file.** W5-a's own text is "RED: the
   pre-state run returns exactly 3 lines (`:102`, `:134`, `:135`). GREEN: 0",
   with evidence "authoring: 3, with a fired control". The fired control the row
   points at is the authoring section's, which is `README.md` -> 0 and
   `BUILDING.md` -> 3. The plan-5.5 file appears in Step 3 only, and only as the
   mechanism for one of the row's premises.
2. **The implementer did not invent a substitute; it read another sentence of
   the same plan.** Choosing between two control statements the plan itself
   makes, one false and one true, is reading the plan, not exercising latitude.
   No design dimension was left open and nothing about the deliverable turned on
   it.
3. **The substance the control exists to establish is established, and I
   re-established it independently.** The concern the control serves is that an
   empty grep and a broken grep look identical. I built six patterns of my own -
   the plan's digit expression plus five wider forms it cannot see (case-
   insensitive, any separator, spelled cardinal after `part`, ordinal before
   `gate`/`part`, prose "as the Nth gate part", numeral-suffix `6th`) - fired
   **every one of them** against a fixture I wrote carrying a known-present
   instance of each form, and only then ran them on both states:

   ```
   PRE-STATE  BUILDING.md @5378264 : 3 hits (plan expression), 3 (wide digit), 0 0 0 0
   END-STATE  BUILDING.md @a0d5d3e : 0 0 0 0 0 0
   all six patterns fire against the known-present fixture: True
   ```

   The end-state zero is a measurement. Additionally the ordinals are gone in
   forms the plan's own expression could never have seen.

**The amendment, stated precisely.** Task A1's Step 3 should read, in place of
its current soundness-control sentence:

> **Soundness control, because an empty grep and a broken grep look identical:**
> the same expression over `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`
> returns 2 matches, so the pattern demonstrably hits a positional gate ordinal
> when one is present. (`2026-07-11-plan-5.5-pre-1.0-hardening.md` carries only
> SPELLED ordinals and is the authoring section's control for the SPELLED sweep,
> not for this one.)

Measured for the amendment, so its own figure is not another recalled number:

```
2  docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md
2  docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md
15 docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md
```

The plan-10 file is the better control than the plan-11 file the implementer
used as its third data point, because plan-11 is the document under execution
and its count moves whenever the plan is amended.

### Adjudication 2 - the restore mechanism

**Verdict: the substitution was the correct handling. This fork should NOT have
returned as NEEDS_CONTEXT.**

The reason is precedence, and the plan settles it without any judgment call
left over.

- The restore mechanism came from the **brief**, which the review brief's own
  ground-truth ordering places below the plan, and which
  `proc-57-briefs-not-ground-truth` places below the plan as a standing rule.
- The **plan** prescribes no mechanism. Step 3 says "and **restore**", and "The
  restore is proven in the diff check below."
- The plan's Step 3 then makes the brief's mechanism **impossible**: the
  post-restore `git diff -U0 -- BUILDING.md` "must SHOW" Step 2's two regions
  and `git diff --stat` must name exactly one file. After `git checkout --
  BUILDING.md` at a point where the Step-2 edits are uncommitted, both are empty.

So this is not an open fork with two defensible answers. It is a brief
instruction that the ground-truth document already contradicts, and the plan's
Global Constraints tell the implementer what to do with exactly that: "A
contradiction discovered on code contact is refuted with evidence or returned,
never silently absorbed." It refuted, with evidence, in writing, as a named
concern, and returned `DONE_WITH_CONCERNS`. NEEDS_CONTEXT is the channel for a
fork the ground truth leaves open; it is not the channel for an instruction the
ground truth already decides.

**And the substitute mechanism was sound, which I verified rather than took on
the hash.** The implementer's claim is a sha256 baseline of the edited file
taken before any mutation, `3ce5b604...`, restored to after each of five fires.
Independent corroboration:

```
worktree BUILDING.md sha256    3ce5b604d353e9f235860a6696e42b3be2ef3e36615a640bd087c829fa026c1f
a0d5d3e:BUILDING.md sha256     3ce5b604d353e9f235860a6696e42b3be2ef3e36615a640bd087c829fa026c1f
```

and, stronger than the hash, the DERIVED == end-state equality at the top of
this verdict: the committed file is the pre-state plus exactly the two fenced
substitutions and nothing else, so no residue of any of the five fires reached
the commit. A `git checkout --` restore would have produced a `BUILDING.md`
byte-identical to the base, which the same instrument would have caught.

One caveat for the record: the implementer's refutation is correct on the
mechanism, and its account of the danger ("a less careful sequencing could have
produced a clean `git diff` read as success while the deliverable no longer
existed") is a fair statement of the failure mode, not an exaggeration.

### Adjudication 3 - the third live consumer

**Verdict: surfacing it was the right handling and was owed, not optional. The
plan's "one live consumer" claim IS a defect, and the acceptance map inherits it
as an incomplete set - but W5-e is still satisfied as written.**

*Was surfacing right?* Yes, and the plan mandates it. Task A1's Global
Constraints name `proc-sweep-surface-completeness` among the entries that bind
hardest, and that entry reads: "A firing positive control proves a sweep PATTERN
is valid, never that its SEARCH SURFACE is complete - the two are separate
obligations". The implementer swept for the fact rather than trusting the plan's
three-item list, found a fourth, and did not edit it, which the plan also
requires. That is the correct discharge of both halves.

*Is the plan's claim a defect?* Yes. Reproduced:

```
$ git grep -n 'gate part 6'
docs/ROADMAP.md:2722:  cross-target lint rule became gate part 6 (commit bcb67f3). Nothing there
docs/ROADMAP.md:2725:  (BUILDING.md, Rust gate part 6; cfg-gated items can differ per platform).`
docs/decision-ledger.yaml:5430: ... gate part 6 ... [illustrative example]
docs/process-conventions.yaml:661: ... documented as gate part 6 in BUILDING.md ...
```

`:2722` is a dated record of what happened at commit `bcb67f3` and is protected
by the same MEASURED-block principle that protects `ci.yml:88`. `:2725` is not:
it is a **forward-looking prescription**, fenced as an "exact replacement, so
nobody re-derives it", waiting on a trigger. The implementer's classification of
the difference is exactly right and is the substantive part of the finding.

*Does the acceptance map inherit it?* Yes, in the shape the plan itself named
elsewhere. W5-e claims "The Tier-2 statement that cites 'gate part 6' is
surfaced" - one item, one row. The Tier-2 statement WAS surfaced with its clause
quoted, so the row passes. But the row's set is short by one live consumer that
has no row of its own, which is precisely the defect the plan called out for
W2-f: "this one lacked one, which mattered because the acceptance map is the
artifact a reviewer walks". Same shape here, one level down.

The disposition urgency is finding 1 and is where this matters operationally.

### Adjudication 4 - the ledger neighbour

**Verdict: NOT stale. An example of a pattern is immune to the pattern's removal
at one site. The implementer's handling - flag as a lower-confidence judgment
call, take no action - is correct.**

The entry is `a-positional-ordinal-into-a-list-is-a-reference-that-drifts` in
`docs/decision-ledger.yaml` (Tier **1**, not Tier 2 as the review brief's House
dimension groups it; count 2, status settled). Two clauses of its statement
could be argued stale, and neither is.

**(a) "the third rejected alternative, gate part 6, the second bullet"** is a
three-member list of illustrations of the anti-pattern's SHAPE. It is not a
citation of any file's current content. Treating it as stale would mean every
successful repair falsifies the rule that mandated it - the rule would be
unrepairable by construction. The entry's own discriminator settles it from the
other side: the defect it names is "an INDEX A READER RESOLVES BY COUNTING",
and nobody resolves "gate part 6" here by counting anything; it is a specimen,
not a pointer.

**(b) "This is a SIBLING of the owner's line-number ruling and its BUILDING.md
instance"** is the clause the implementer flagged. Two readings, both benign.

- Read as "the line-number ruling's instance in `BUILDING.md`": no such instance
  ever existed. Measured on both states, `BUILDING.md` carries zero
  self-referential or cross-file line citations at `5378264` and at `a0d5d3e`.
  A clause whose referent never existed is not made stale by this edit.
- Read as the entry's own occurrence record spells it out - "the other two being
  the owner-ruled comment line citations and BUILDING.md's positional gate
  ordinals" - the clause is a **scope disclaimer about the relationship between
  rules** ("not a widening of either: their scope is unchanged and this entry
  claims no authority over it"). A1 repaired one instance of one sibling class.
  It changed neither rule's scope and therefore not the relationship the
  sentence asserts.

The only residue is discoverability: a reader chasing "BUILDING.md's positional
gate ordinals" will now find them only in the ROADMAP's historical record. That
is a wrinkle, not a false statement, and the controller may leave it.

### Adjudication 5 - what the task did NOT do

**Verdict: all three plan-named items were surfaced with their quoted clause,
none was edited, and the surfacing of two of the three is complete enough to act
on without re-derivation. The third (the ROADMAP paragraph) is complete as a
quote but incomplete as a disposition input - see finding 1.**

*None was edited*, proven at the tree rather than at the report:

```
$ git diff --name-status 5378264..a0d5d3e
M	BUILDING.md
$ git status --porcelain          # (empty)
```

House-knowledge YAML, `docs/ROADMAP.md` and `.github/workflows/ci.yml` are
untouched at every level - not in the commit, not in the index, not in the
working tree.

*Each quote verified at its artifact*, not at the report:

| plan item | report | quote checked against | verdict |
|---|---|---|---|
| `gate-includes-cross-target-lint-for-the-unrun-os` | 5.1 | `docs/process-conventions.yaml:661`, statement read via PyYAML | verbatim, and the plan's note that the clause carries no backticks in the source is correct |
| ROADMAP "A neighbouring class" paragraph | 5.2 | `docs/ROADMAP.md:2196-2218` | verbatim, elisions marked with `[...]`; both halves quoted (the three-site enumeration and the 86-character line) |
| `.github/workflows/ci.yml`'s spelled ordinal | 5.3 | `.github/workflows/ci.yml:88` | verbatim; classified as a measured non-defect with the corroborating retired-plan heading, as the plan's "Must not decide" line requires |

*Actionability.* Items 1 and 3 are complete: the controller can rewrite the
Tier-2 clause and leave `ci.yml` alone with no further measurement. Item 2 is
complete as a quote but the report says only "Disposition is a controller close
action", which understates it: the ROADMAP entry also carries a **Vehicle** line
("whichever package next edits `BUILDING.md`'s gate blocks after Plan 10's Task
1 lands") that A1 does not satisfy, since A1 edited prose and no gate block. The
controller closing that entry has to reconcile the vehicle sentence with what
actually landed. Minor re-derivation, worth naming so it is not discovered at
the close.

The fourth and fifth items the implementer added (report 5.4, 5.5) were not
required by the plan's Step 4 but were owed by `proc-sweep-surface-completeness`,
and both were correctly surfaced without edit.

---

## Dimension-by-dimension record

**1. Contract compliance.** DERIVED == end state, byte for byte (see top). Both
fenced sources occur exactly once in the pre-state and zero times in the end
state; both targets zero and once. A is a pure single-line deletion; B is six
lines replaced by seven. The four named commands read out of `BUILDING.md`'s own
Rust gate block, by walking the `<!-- gate-block: rust; ... -->` marker to its
fence:

```
1. cargo fmt --all --check
2. cargo clippy --workspace --all-targets -- -D warnings
3. cargo test --workspace
4. RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
5. cargo deny check
6. cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
```

First four are `cargo fmt`, `cargo clippy`, `cargo test`, `cargo doc`, in that
order. Matches. No NEEDS_CONTEXT was owed on that ground.

**Additionally: the fenced text is TRUE, not merely correctly transcribed.** A
fenced block that an implementer applies verbatim removes the only reader who
would have caught an error in it, so its truth is the reviewer's to check.
Verified against `.github/workflows/ci.yml`: the `test` job's matrix is
`["ubuntu-26.04", "windows-2025", "macos-15"]` on `runs-on: ${{ matrix.os }}`;
steps `:85`, `:86`, `:87` and `:99-100` run `cargo fmt`, `cargo clippy`,
`cargo test` and `cargo doc` with **no `if:` condition on any of them**, so all
four do run natively on all three legs; `cargo doc` carries
`RUSTDOCFLAGS: "-D warnings"` as step env, matching the gate block; `deny`
(`:161`) and `ledger-lint` (`:169`) are separate jobs. The paragraph's carried-
over trigger clause ("every master push, `v*` tag and PR") matches `on:` at
`:3-7`, which additionally carries `workflow_dispatch` - unchanged from the
pre-state text and not this task's claim.

**2. Scope.** `git diff --stat 5378264..a0d5d3e` names one file. `-U0` shows 15
changed lines at old `[102, 134-139]` and new `[102, 134-140]`, both inside Step
2's two regions. All four marker lines present once and byte-identical on both
sides; the canonical gate-total sentence identical on both sides.

**3. Both absence checks re-measured with my own expressions.** Check O: 3 -> 0,
plus five wider forms at 0 on both states. Check L: 1 -> 0. My counts agree with
the plan's and the implementer's in every figure.

**4. Everything I relied on was fired, by me, on instruments I built.** Six
check-O patterns against my own fixture; check L in both directions against the
END state (a 90-char line injected into prose is caught at `:106`; the byte-
identical line injected inside the Rust fence is correctly invisible); four
`ledger-lint` fires on my own copy of the repo layout plus a control; four
diff-scope assertions plus a control. No instrument the implementer wrote was
re-run. Details in the appendix.

**5. The 80-column measurement.** Longest non-fenced line in the end state:
**80 characters at `BUILDING.md:77`**, `this sentence against them, so the
stated number cannot drift from what the file` - at the norm, not over it. The
replacement paragraph's own longest line is 77, as the plan states. Eight
over-80 lines remain in the file, all inside fences, unchanged.

**6. Latitude, both forms.** No keyboard decision that should have returned as
NEEDS_CONTEXT: the two triggers the plan enumerates (a re-measurement returning
a different set; a different first four commands) both failed to fire because
both readings matched. The three numbered concerns are, respectively, a plan
defect correctly diagnosed and correctly not acted on, a brief defect the ground
truth already resolves, and a measured plan understatement - none of them a
design fork. Nothing the plan settled was returned or omitted: all five Step-3
checks, all three Step-4 surfacings, and Step 5's commit are discharged.
`DONE_WITH_CONCERNS` rather than `DONE` is the right status for two genuine
instruction defects.

**7. House dimension.**

| id | verdict |
|---|---|
| `ledger-lint-runs-before-every-push` | conformant. Run green on the end state (`560 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold`, exit 0), reproduced by me. Not pushed, correctly - the controller pushes at the plan close. |
| `proc-wrapped-prose-quote-grep` | conformant. The replacement was applied as a multi-line exact substring keyed on content, with a uniqueness assertion first, never on line positions. The hard-wrapped paragraph is the reason the task is one edit, and the report states it. |
| `proc-verification-step-must-be-falsifiable` | conformant, including its PER ASSERTION clause. Four diff-scope assertions fired separately with hash-proven restores between each. I reproduced all four independently. |
| `proc-check-green-state-reachable` | conformant. Both checks pass on the intended end state, and I re-ran both there. |
| `proc-normative-count-recomputed` | conformant. A1 removes members from an enumerated set; the count-carrying statements about that set (`docs/ROADMAP.md`'s "Three sites", the Tier-2 clause) are surfaced rather than edited, which is what the plan's no-edit constraint requires. No count inside `BUILDING.md` moved - the canonical sentence is byte-identical. |
| `proc-sweep-surface-completeness` | conformant in method, imperfect in its report. The implementer swept rather than trusting the plan's list and found a fourth consumer; but the residual set in 5.6 lists 9 of 13 (finding 2), which is the same entry one level down. |
| `a-positional-ordinal-into-a-list-is-a-reference-that-drifts` | not violated; see adjudication 4. Note it is Tier **1**, not Tier 2. |
| SI-4 commit shape | conformant. Unsigned (`%G?` = `N`), exactly one `Co-Authored-By` trailer naming the dispatch model, zero `Claude-Session` lines, one file, explicit pathspec. |
| typography | conformant, verified on the added lines: zero em-dash, en-dash, figure dash, horizontal bar, Unicode minus, curly quote, ellipsis or nbsp; the added region contains no codepoint above 127 at all. Probe control fired. |

**8. No-work-needed check.** Two such conclusions, both premises run.

- "This task produces no user-visible consequence ... so it ships no new test."
  The premise is that nothing reads `BUILDING.md` except `scripts/ledger-lint.py`.
  Run: `git grep -ln 'BUILDING' -- ':!docs' ':!*.md'` returns
  `.github/workflows/ci.yml` and `scripts/ledger-lint.py`, and both `ci.yml`
  hits are prose comments (`:97`, `:172`), not a read. No build step, test, lint
  or bundler consumes the file. Premise holds. The weighing is correct on the
  rule's own boundary: `tests-ship-with-the-feature-never-after` forbids
  deferring a scenario the existing infrastructure can express, and there is no
  observable behaviour to express here.
- "The 80-column norm has no checker anywhere in the repo." Run: no
  `.editorconfig`, no markdownlint config, no prettier config in the tree, and
  no gate part reads the file for width. Premise holds - which is exactly why
  W5-b rests on a measurement and why I re-measured it with my own script rather
  than accepting the plan's.

I also ran the premise the report does NOT state but relies on, that
`ledger-lint` is green *because the file is intact* rather than because it stops
looking. My control fire mutates the very prose this task edited
(`CI (...) runs the Rust block's ...` -> `CI (...) runs ABSOLUTELY NOTHING AT
ALL,`) and `ledger-lint` stays green, exit 0. That is the honest boundary of
W5-c: the green run proves the gate structure is intact, and says nothing at all
about the edit. The four red fires prove it still reads the file.

**9. Blast radius for A2, A3, A4.** Nothing in this diff can turn a gate part
red on any tree those tasks legitimately produce.

- `ledger-lint`'s check 7 reads only the four `<!-- gate-* -->` markers, the
  fenced command lines under them, and the canonical sentence. All are byte-
  identical across this diff (proven), and the prose-mutation control above
  proves the edited regions are outside its input entirely.
- No Rust, frontend or e2e gate part reads `BUILDING.md`.
- A2 writes `.github/workflows/ci.yml` and one test fixture; A3 writes
  `matcher.rs`, the v1 spec, `README.md`, two help topics and tests; A4 writes
  the spec. None of them reads `BUILDING.md` for anything a gate parses.

The one real hazard is not a red gate part but a silent regression, and it is
finding 1.

---

## Evidence appendix

All instruments written by me, under
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a1rev-independent/`.
No instrument the implementer wrote was executed at any point.

| path | what it does |
|---|---|
| `extract_fences.py` | walks the plan's Task A1 section, collects its 7 fenced blocks in order with sha256 per block |
| `contract_check.py` | applies blocks 3/4/5/6 to `git show 5378264:BUILDING.md` and compares byte for byte against the end state; occurrence counts per fence on both states |
| `rustblock.py` | reads the Rust gate block's commands out of `BUILDING.md` via its marker, not out of the plan |
| `check_O.py` | six independently derived ordinal patterns over both states |
| `fixture_O.md` + `fire_O.py` | reviewer-built known-present fixture; fires all six patterns |
| `check_L.py` | fence-aware length pass; recognises indented and tilde fences, reports codepoint and byte length, longest non-fenced line |
| `fire_L.py` | fires check L in both directions against the END state (90-char line injected into prose, then the same line inside a fence) |
| `ll-copy/` + `fire_ledgerlint.py` | full isolated copy of `scripts/`, `BUILDING.md` and the four `docs/*.yaml`; four red fires plus one green control, each restored and hash-proven |
| `diffscope_rev.py` | `-U0` hunk parser mapping each changed line to its number on its own side, testing all four W5-d properties against both file states |
| `fire_diffscope.py` | fires all four assertions via `git diff --no-index` on scratch copies, plus a must-stay-silent control |
| `entry.py` / `entry2.py` | PyYAML readers pulling named house-knowledge entries by id |

Read-only commands against the tracked tree: `git status --porcelain`,
`git log`, `git rev-parse`, `git show <rev>:<path>`, `git diff` (all forms),
`git grep`, `sed -n`, `/usr/bin/grep`, `sha256sum`, `wc`, and one read-only
`python3 scripts/ledger-lint.py` in the worktree (the script only reads).

**Tree integrity, proven at review end:**

```
$ git -C /home/senol/Git/muxsmith-plan11-a status --porcelain
                                          (empty)
$ git rev-parse HEAD                      a0d5d3e28d44ee464dd9175ea11e5446e9b0dc0c
$ sha256sum BUILDING.md                   3ce5b604d353e9f235860a6696e42b3be2ef3e36615a640bd087c829fa026c1f
$ git show a0d5d3e:BUILDING.md | sha256sum 3ce5b604d353e9f235860a6696e42b3be2ef3e36615a640bd087c829fa026c1f
$ git rev-parse HEAD^{tree}               5acd23d467cb78ec6dd810c7fdf22e62ea4f084a
$ git rev-parse a0d5d3e^{tree}            5acd23d467cb78ec6dd810c7fdf22e62ea4f084a
$ git diff a0d5d3e --stat                 (empty)
```

`/home/senol/Git/muxsmith-plan11-b` was neither read nor touched.
`/home/senol/Git/Muxsmith` master is at `5378264` and still carries the two
`part 6` occurrences, as it must.

---

## HARVEST

**Dominant pattern this task exhibits, worth carrying: the strongest scope check
is a reconstruction, not an inspection.** Grading "did anything else change?" by
reading a diff is an eye check. Deriving the end state from the base plus the
plan's own fenced substitutions and comparing byte for byte answers contract
compliance and scope in one instrument, and it cannot be fooled by a change that
happens to look like context. Where a plan fences its replacements character for
character, this is the check the reviewer owes, and it costs one script. Candidate
for a process convention.

**A fenced block is a custody boundary, not a truth guarantee - and this is the
one thing a reviewer must do that the implementer structurally cannot.** The plan
fenced "the Rust block's `cargo fmt`, `cargo clippy`, `cargo test` and `cargo
doc` commands natively on all three OS legs" and told the implementer to apply it
verbatim. The implementer did, correctly, and in doing so the fence removed the
only reader who would have opened `ci.yml` to check whether those four steps
really run unconditionally on all three legs. They do (verified above), so
nothing is broken - but the review is where that check lives, and it is not on
any acceptance row. **Every fenced replacement carrying a factual assertion needs
its truth verified at the review, separately from its transcription.**

**Repeated rejection, twice in one task, of instructions that could not be
executed as written.** Step 3's control cannot fire; the brief's restore would
have destroyed the deliverable. Both were caught by the implementer, both
correctly refuted with evidence rather than absorbed, and both are the same
class: an instruction whose passing state is indistinguishable from its broken
state until someone runs it. The plan's own Tier-2 entry
`proc-verification-step-must-be-falsifiable` describes the first exactly, one
level up ("a control that cannot fire is the vacuous-negative failure applied to
the control"). Worth a ledger occurrence.

**What the remaining three stream-A tasks must carry.**

1. **A2 must not inherit the ROADMAP rider in its current form.** Finding 1.
   This is the single item on this list that can silently undo A1. The rider's
   trigger is "the edit is the trigger" and A2 is the edit; its target region is
   inside A2's authorized write region.
2. **A2, A3 and A4 each run `python3 scripts/ledger-lint.py` for free, and their
   green run means less than it looks.** The check reads four markers, the
   fenced command lines, and one sentence. It is blind to every prose region any
   of these tasks touches. A green `ledger-lint` after A2/A3/A4 is evidence about
   YAML integrity and gate arithmetic, not about their deliverables.
3. **The house's ordinal expression `part [0-9]|parts [0-9]` is case-sensitive,
   digit-only and separator-rigid.** It is used as if it were the definition of
   the class in the plan, the ROADMAP and the tracker. It cannot see `Part 6`,
   `part-6`, `part six`, `the sixth part` or `6th part`. A1's end state is clean
   under all of those (measured), but the instrument gap is real and the plan
   already records it as surfaced item 6 for the controller.
4. **The residual-set discipline.** A sweep report's classification section is an
   enumeration, and an enumeration is a claim (finding 2). Whichever of A2/A3/A4
   reports a classified residual set should paste the command and the full
   output, then classify line by line - not describe the remainder in prose.
   A3's alternation-free vocabulary sweep is where this will bite hardest.
5. **Reviewer instruments belong in the scratch, never in the tree.** The full
   `ledger-lint` fire ran on a copied `scripts/`+`docs/`+`BUILDING.md` layout,
   because the script resolves its repo root from `Path(__file__).parent.parent`.
   That property makes an isolated fire trivial and should be the default for
   every later task that needs to make this gate part go red.
