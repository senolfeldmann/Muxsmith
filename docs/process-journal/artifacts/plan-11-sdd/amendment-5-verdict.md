# Amendment 5 delta verdict - Plan 11

**Round 1 (`44bc6f7`): NEEDS_FIXES.**
**Fix round 1 (`ba67cbc`): APPROVED** - all three findings ADDRESSED, no new
breakage. See "Fix round 1 delta re-review" at the end of this file, which also
retracts one challenge this verdict made.

---

**Verdict: NEEDS_FIXES**

**Graded:** `git diff 83af0d5..44bc6f7` over
`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`,
read in the main worktree `/home/senol/Git/Muxsmith` on `master` at
`44bc6f7c8aba002a5dc47b4f3462c1c5929a6597`. One file, 70 insertions, 20
deletions. `/home/senol/Git/muxsmith-plan11-a` was never entered; the `c422999`
state of `deny.toml` was read as a blob, not through the stream-B worktree.
Nothing was written anywhere in the repository. Tree integrity proven in the
appendix.

**What the verdict means.** The substance of this amendment is sound and, on the
dimension that mattered most, better than its predecessors: every one of the six
defects is repaired at every site, the site sets are complete on my own searches
rather than on the author's lists, the replacement fence is TRUE sentence by
sentence at cargo-deny's own source and its last sentence fires in both
directions, the guard deferral now records a routed decision rather than a
resolved one, scope is clean to the byte in A2, A3, A4 and every amendment log,
and nothing was compressed. **Two findings block.** One is a live hazard for the
fix round that follows: the fence's placement instruction, applied literally to
the file that fix round actually holds, produces a `deny.toml` that cargo-deny
refuses to parse - fired, not argued. The other is the class this plan has been
bitten by three times already: a figure measured on one state of a file and
written up as a correction to a source verdict that measured the other state.
The three minors are accounting rather than substance.

---

## Findings

### 1. IMPORTANT - Task B1 Step 4(a)'s placement instruction does not resolve against the file the fix round applies it to, and a literal application yields a config cargo-deny will not parse

**Site:** Task B1, Step 4(a): "Insert, immediately after the line
`yanked = "deny"` and immediately before the comment beginning
`# All entries below are transitive`, exactly:" followed by the replacement
fence.

Pre-amendment that instruction addressed a `deny.toml` in which those two
anchors were adjacent. The amendment changed what the fence says and added a
paragraph stating that "a B1 fix round applies it", but left the operation and
its anchors untouched. The fix round holds the file at `c422999`, where the two
anchors are **eight lines apart** and the shipped OLD fence sits between them -
including its own `unsound = "all"` key line.

Fired on reviewer-built copies, both readings:

```
anchor A (yanked = "deny")  -> 0-based line 5      [exactly 1 occurrence in c422999]
anchor B (# All entries...) -> 0-based line 14     [exactly 1 occurrence in c422999]
lines between the anchors: 8   (the shipped fence, 7 comment lines + unsound = "all")

literal INSERT between the anchors -> 2 `unsound` keys
  $ cargo deny check advisories -c literal-insert.toml
  [ERROR] failed to parse config from '...': duplicate key: `unsound`

region REPLACE between the anchors -> 1 `unsound` key
  $ cargo deny check advisories -c replace-region.toml
  advisories ok
```

Both directions discriminate, so neither result is a check that cannot fail.

The prose note added below the fence ("Amendment 5's replacement for the one B1
shipped in `c422999`") makes the REPLACE reading strongly implied, and the
Amendment 5 section says "replaced whole, not patched". That is precisely the
problem: the operative sentence and the explanatory sentence disagree, and this
plan's own doctrine is that a fenced instruction removes the last reader
permitted to reconcile them. The amendment repaired one instance of exactly this
class in the same round - Step 5's `git diff --exit-code` "clean", unperformable
at the point it appears - while leaving a second one it created.

**Required change:** restate Step 4(a) as an operation against the post-`c422999`
file. Concretely: *"Replace the eight lines between `yanked = "deny"` and
`# All entries below are transitive` - the fence B1 shipped, including its
`unsound = "all"` line - with exactly:"*, keeping the fence byte-identical.
State that the result must contain exactly one `unsound` key. The pre-state
insert wording may stay beside it as the record of what B1 was originally told,
but it must not be the operative sentence for the fix round.

### 2. IMPORTANT - a new wrong figure: `deny.toml`'s "own longest line" is measured on the wrong state of the file, and a correct verdict figure is written up as an error

**Sites:** (a) Amendment 5's figures paragraph - "longest line 76 characters
against `deny.toml`'s own longest of **77** - the B1 verdict's parenthetical says
78, and the measured maximum is 77, which changes nothing about the fence and is
recorded because an unchecked figure is how this document has been bitten
before"; (b) the note under Step 4(a)'s fence - "longest line 76 characters
against `deny.toml`'s own longest at 77"; (c) `amendment-5-report.md` concern 5,
which escalates it to the controller as an off-by-one in the verdict.

Measured, both states of the same file:

```
master (pre-B1)  deny.toml  max = 77   line 48  "# MIT project (spec 12); allow exactly the permissive licenses our dependency"
c422999 (post-B1) deny.toml max = 78   lines 54, 57, 58
```

All three 78-character lines at `c422999` are lines **B1's own fence B added** -
confirmed by `diff -u` of the two blobs; they appear only in the `+` side.

So the B1 verdict's "the file's existing maximum is 78" is a correct measurement
of the state that verdict was reviewing, and it is the state the fence lands in.
The amendment measured the pre-B1 file, called the verdict wrong, and wrote the
result into the plan without naming either state - the same defect A1's own
verdict recorded against that task's report ("the numbering changes state without
saying so"), one level up and this time with a public accusation attached.

Nothing about the fence changes: 76 clears both 77 and 78. What must change is
the sentence.

**Required change:** at (a) and (b), name the state - "76 against `deny.toml`'s
77 before B1's own fence B and 78 after it, so the fence clears the file in
either state". Withdraw "the B1 verdict's parenthetical says 78 ... the measured
maximum is 77" and the framing that an unchecked figure was caught here; retract
concern 5 in the report, since the verdict figure it corrects is right.

### 3. MINOR - the amendment's restatement accounting on the guard row is wrong in both of its terms, in the paragraph it presents as its method showpiece

**Site:** Amendment 5, "The guard premise, which is where the search beat the
list": "returns the refuted premise on **three lines**, not one: B1's Step 9
test-duty bullet, the deferred-by-decision row, and the plan-close ROADMAP line
that consumes the row's reasoning. **The restatements are four, though, because
the row's line carries two**".

Measured on the pre-amendment document, column by column:

```
:763  B1 Step 9      1 restatement   "a lint asserting one of its keys would be new gate infrastructure"
:804  deferral row   3 restatements  Why:     "the failure would be silent in exactly the way the original defect was"
                                     Why:     "A lint asserting a `deny.toml` key would be new gate infrastructure,
                                               which the tests-belong-to-the-package rule explicitly still allows deferring"
                                     Vehicle: "the failure mode here is silent by construction - drop the key and nothing happens"
:787  plan close     0 restatements  grep -c 'silent|infrastructure' on that line -> 0
```

The premise is asserted on **two** lines, not three, and the row carries
**three** of the four restatements, not two. The total of four is right by
compensating errors. `:787` is a consumer of the row's routing - it says the
residue exists and how it will be phrased - and asserts neither half of the
refuted ground.

**The repair itself is complete**, which is why this is minor: all four
restatements are gone as assertions (post `:772` and post `:813` carry them only
inside "REFUTED and must not be restated" framing), and `:787` was updated too.
It is the account of the set that is off, in the one passage the amendment offers
as its demonstration that the unit is the restatement rather than the line.

**Required change:** "two lines, four restatements, three of them inside the
deferral row (two in Why, one in Vehicle); the plan-close line is a consumer of
the routing rather than a restatement of the premise."

### 4. MINOR - two verdict-required corrections outside the plan document are routed nowhere

The B1 verdict names `task-b1-report.md` explicitly twice:

- Finding 3, Required change: "the deferral row's reasoning **and the report's
  'silent by construction' sentence** are both falsified and must be corrected";
  adjudication 5 repeats it: "the report may not keep claiming the loss is
  silent." Still present at `task-b1-report.md:967` ("the loss of coverage is
  silent by") and `:1045` ("loss is silent - the same shape as the defect just
  repaired").
- Finding 4, Required change: "record it as a third plan defect alongside
  findings 1 and 2 of the report." That report carries exactly two numbered
  findings (`:1078`, `:1148`); no Step-5 contradiction record exists in it
  (`grep -niE 'exit-code -- deny.toml|unperformable|exits 1 by construction'` ->
  no hits).

The amendment was right not to edit that file - its brief scopes it to one file.
The gap is that neither the Amendment 5 section nor the report's five
"Concerns for the controller" mentions either item, so both fall out of the
process silently.

**Required change:** add them to the controller concerns, with the disposition
question stated rather than assumed - a task report is a dated record of what an
implementer claimed, so "correct it", "annotate it as superseded" and "leave it
under the MEASURED-block principle" are three defensible answers and the verdict
asked for the first.

### 5. MINOR - A1's adjudication 5 vehicle reconciliation is carried nowhere

**Site:** plan close, ROADMAP dispositions, the "A neighbouring class" entry.

The A1 verdict's adjudication 5 records that this ROADMAP entry carries a
**Vehicle** line - `docs/ROADMAP.md:2218`, "whichever package next edits
`BUILDING.md`'s gate blocks after Plan 10's Task 1 lands" - which A1 does not
satisfy, because A1 edited prose and no gate block, and that "the controller
closing that entry has to reconcile the vehicle sentence with what actually
landed. Minor re-derivation, worth naming so it is not discovered at the close."

The amendment edited that very disposition list. Its clause for this entry still
reads only "all three ordinals and the 86-character line closed; its own
enumeration of them is now history". The controller's tracker does not carry it
either (`grep -niE 'vehicle|neighbouring class|gate blocks' progress.md` -> no
hits).

**Required change:** one clause in that disposition, or a tracker line - the
close has to reconcile the entry's vehicle sentence against a prose-only edit.

---

## The five adjudications

### 1. Is the amendment's scale call right?

**Verdict: YES, ONE-PAIR is correct.** Re-grounding a deferral and replacing a
shipped fence do not re-cut the task.

*For "this re-cuts B1."* Defects 3 and 4 replace text B1 already committed, so
the amendment's output is not an input to an unstarted task but a change to a
delivered artifact, and defect 5 rewrites the justification of something B1
weighed and decided. A re-cut is usually recognised by exactly this - work
crossing a boundary the task already closed.

*Against, and where I land.* The test for a re-cut is whether the task's WORK
changes, and it is measurable rather than arguable here. B1's step list is
unchanged in count and in what each step does; its deliverable is unchanged in
substance (same two insertions, same file, same three-way fire, same
investigation); the acceptance map has 40 rows before and after with only
**W1-g** and **W1-l** touched, both of them B1 rows and both correcting a
statement rather than adding an observable; and the deferred-by-decision table
has 8 rows before and after. What moved inside B1 is a fenced string's CONTENT,
a premise, and one unperformable check - each of which the amendment brief
enumerates as one-pair material. Defect 5 in particular changes only the ground
of a deferral and its routing; the thing deferred is the same thing, still
deferred, and B1 ships no key it did not ship before. Note the asymmetry that
makes this cheap to get right: a re-cut would have to show up as a changed
producer clause on some acceptance row, and the two that changed carry no new
producer.

The fenced text's arrival AFTER the commit is a vehicle question, not a scale
question, and the amendment handles it by naming the fix round.

### 2. Should the mise rider have been an eighth item in the surfacing list?

**Verdict: NO, the author's call is right - and it is right for the reason it
gives, which I verified rather than accepted.**

The list is introduced as "Surfaced for the controller **at plan-authoring**,
each already carrying its measurement in the authoring section". That is a
predicate about WHEN a thing was found, not a bucket for everything the
controller must act on. The rider was found at execution, by A1's reviewer, and
was disposed of before A2 was dispatched. Adding it would put an
execution-time discovery under an authoring-time heading, which is the
misdating the author names.

Verified at the artifacts rather than taken from the amendment:
`docs/ROADMAP.md` carries the rider recorded **FIRED 2026-07-30 at Plan 11 Task
A2** and "deliberately RE-DEFERRED rather than consumed", with the fenced text
left unrepaired on the stated ground that a replacement string for a source
comment is product content, and with a new observable named. `task-a2-brief.md`
carries the prohibition ("You do NOT consume that rider, and you do not apply its
replacement text"). So the item is disposed of, not merely noticed.

The count consumer is real and stays consistent: the list still enumerates
`(1)` through `(7)` in both states, and the self-review's "7 controller-surfaced"
still matches. The plan close walks the list; it does not walk it as an inventory
of everything outstanding, and the rider has a stronger home than a plan-close
bullet - a ROADMAP entry with a fired trigger and a written re-deferral.

Residue worth one line, not a finding: the rider IS named in the plan, in the
live-consumer bullet, with a pointer to its disposition. A reader who reaches the
surfacing list without reading that bullet will not meet it. That is acceptable
given the ROADMAP record.

### 3. Was correcting the source verdicts' two wrong figures silently the right handling?

**Verdict: SPLIT. Correcting the "2 matches" unit inside the amendment was
right and needs no note back. Correcting the width figure was NOT right, and it
is finding 2 - because the figure was not wrong.**

*The control figure.* The A1 verdict's replacement text says the plan-10 document
"returns 2 matches". Re-run: `grep -cE` returns **2**, `grep -oE | wc -l` returns
**5**. The verdict's figure is right on lines and loose on units. Absence check O
states its red and green states in lines ("RED: exactly 3 lines. GREEN: 0"), so
lines is the unit the check is measured in, and setting the amendment's figure to
2 LINES while saying so explicitly is the correct handling. A note back to the
verdict would record a unit slip that changes no conclusion in a document nobody
re-executes. The amendment states the departure in the plan itself, which is the
right place: the plan is what a fix round reads.

*The width figure.* This one is not a verdict error at all - see finding 2. The
verdict measured the file it was reviewing. Here the general principle does bite,
and in the opposite direction from the one the amendment applied: **a figure that
disagrees with a source is a state or unit question until proven otherwise, and
the cheap move is to reconcile the two measurements before declaring one wrong.**
The amendment skipped that step, and its report escalated the result to the
controller.

*The general rule I would draw.* A verdict figure that turns out wrong owes a
note back only when a later reader could act on the wrong figure. Neither of
these qualifies (a unit slip; a state mismatch), so the silent correction was the
right SHAPE. What was owed was the reconciliation, not the notification.

### 4. Does the fence's placement instruction still resolve unambiguously against the file as it stands after `c422999`?

**Verdict: NO. This is finding 1, and the brief's instinct that it would bite the
fix round is correct - the literal reading does not merely misplace the comment,
it produces a config cargo-deny refuses to load.**

The two anchors are individually unambiguous at `c422999` (each occurs exactly
once). What fails is the conjunction: "immediately after A" and "immediately
before B" are eight lines apart, and no INSERT satisfies both. The strict reading
leaves the false fence in the file next to the true one and duplicates the
`unsound` key -> `duplicate key: unsound`, parse failure, before any advisory
check runs. The charitable reading is a region replace -> `advisories ok`. Fired
both ways on my own copies.

The amendment is not silent about intent - it says "replacement" and "replaced
whole, not patched" - but intent sits in prose while the operation sits in the
fenced-instruction sentence, and this project's own rule is that the sentence a
verbatim-applying implementer executes is the one that has to be true. The
comment's length change is not what breaks it; the change of the target file's
state between authoring and application is.

### 5. Completeness against both verdicts

**Verdict: SUBSTANTIALLY COMPLETE inside the plan document; two items live
outside it and are routed nowhere (findings 4 and 5).**

Walked end to end.

**A1 verdict.** Finding 1 (the mise rider) - disposed of by the controller, and
the plan's claim about the live-consumer SET is corrected; the disposition is
verified at the ROADMAP and at A2's brief. Findings 2 and 3 - report-only, no
plan consequence, correctly untouched. Finding 4 (Step 3's unfireable control) -
repaired, control re-pointed, both files' real roles stated. Finding 5 (the "one
live consumer" understatement) - repaired, and the amendment goes past what was
required by classifying every non-consumer hit. Adjudication 1 - text applied
with the unit change, declared. Adjudication 2 - brief defect, nothing owed.
Adjudication 3 - the W5-e narrowness is knowingly left; **I concur**: the row was
ruled satisfied, its producer step never named the second member, and widening an
acceptance row for a task that is committed and reviewed would name a producer
that never produced. Adjudication 4 - the ledger specimen is classified as a
non-consumer, matching the ruling. Adjudication 5 - **the vehicle reconciliation
is carried nowhere; finding 5.**

**B1 verdict.** Findings 1 and 2 - both repaired inside one replacement fence,
taken verbatim (sha256 of the fence in the plan equals the sha256 of
adjudication 1(b)'s block: `eed7ff92...`). Finding 3 - repaired at all four
restatements. Finding 4 - repaired in the plan with a performable substitute
(`sha256sum` after Step 4, re-taken after each variant); **the report-side half
is unrouted, finding 4.** Finding 5 - recorded, with more precision than the
verdict had (the changelog history rather than the verdict's "were `LintLevel`
before becoming `Scope`" framing, which the author checked rather than carried).
Finding 6 - repaired at all three live sites, with a fourth wrongness the verdict
did not have. Adjudication 1's tail ("the plan's authoring-section sentence that
states the same thing in its own words needs the same correction") - repaired.
Adjudication 2 - the caveat replacement is the amendment's own composition rather
than the verdict's quoted block; the verdict offered it as a blockquote rather
than a fence, the amendment's version carries every measurement the verdict's
does plus the inclusion-graph refutation, so this is within latitude.
Adjudication 5 - the row records a PARKED owner decision with the measurement
attached and states "Not an automatic add, on the reviewer's explicit
recommendation", which is exactly a routed decision rather than a resolved one;
the un-guardable residual (key and entry dropped together) is named as the part
that genuinely needs new infrastructure.

**Beyond both verdicts:** the amendment found and recorded that the guard premise
was multi-site where the controller's brief listed only defects 3, 4 and 6 as
multi-site. That find is real and is the amendment's best work; only its
arithmetic is off (finding 3).

---

## Dimension-by-dimension record

**1. Per-defect coverage, at every site - site sets established by my own
searches over the pre-amendment document, not from the author's lists.**

| defect | my expression | my set | author's claim | all corrected? |
|---|---|---|---|---|
| 4, "external crate" shorthand | bare `external`, widest possible, then classified | **6** carrying sites (`:91`, `:168`, `:220`, `:706`, fence `:711-712`, `:925`) + 2 unrelated A3 "external-URL" hits | 6 | yes, all 6 |
| 4, blind-spot pass | `workspace.{0,40}(scope|excludes|reach)|first-party|does not reach|Scope::Workspace` | same 6 sites plus fence line `:715`, which the whole-fence replacement covers | not claimed | yes |
| 3, the misattributed 18 | every `\b18\b` line (24 lines), classified by hand | **2** class-attributing sites: `:91` and the fence's `:713-714` | 2 | yes |
| 6, proc-macro caveat | `twelfth consumer|glib-macros|proc-macro edge|excluded twelfth|inclusion graph` | **3** live (`:97`, `:225`, `:758`) + **1** dated record (`:823`, Amendment 1) | 3 + 1 | yes, 3 fixed, 1 kept as record |
| 6, amendment-log range | same expression over `## Amendment 1` .. `## Self-review` | exactly **1** | 1 | n/a, kept |
| 5, guard premise | `gate infrastructure`, `silent`, `permanent guard` run separately | **2** lines / **4** restatements (see finding 3) | 3 lines / 4 restatements | yes, all 4 |
| 2, live-consumer set | `git grep -n 'gate part 6'` over the tree | 2 live consumers, matching the amendment's classification | 2 | yes |

The author's own three quoted expressions, executed verbatim from the document
rather than retyped, reproduce its stated counts: `glib. (is )?(an )?external|external crate`
-> **6**; `proc-macro|glib-macros|twelfth|welve|normal edge` -> **24**;
`permanent guard|permanently guard|new gate infrastructure|silent|unused-ignored`
-> 17 raw lines, of which the premise sits on 2 (finding 3).

**Blind-spot note on my own instruments, disclosed because it fired.** My first
`18` sweep used 70 characters of context on each side, which structurally cannot
match a token on a 77-character line - it missed the fence's own `# reported its
18.`. Re-run unpadded, with a known-present control and a known-absent control,
before any conclusion was drawn.

**2. No new wrong figure.** Every figure the amendment states or moves, re-run:

| figure | amendment | my run |
|---|---|---|
| control expression over plan-10 | 2 lines, 5 occurrences | 2, 5 |
| plan-5.5 under the digit expression | 0 | 0 |
| note-class tally | 18 / 16 / 2 | 18 `advisory-ignored`, 16 `unmaintained`, 2 `vulnerability` |
| glib direct parents | 11 | 11, same names |
| workspace members | 4, none a glib parent | `muxsmith-core`, `muxsmith-cli`, `xtask`, `muxsmith-gui` - none among the 11 |
| gate-count audit | 26 -> 30 | 26 -> 30, by executing the document's OWN expression string |
| audit controls | `BUILDING.md` 4, `renovate.jsonc` 0 | 4, 0 |
| audit delta attribution | 3 from the bullet, 1 from the search record | set-differenced: 6 lines added, 2 removed, net +4; 4 of the 6 are the bullet (net +3), 1 is the search record, 1 is the reworded ROADMAP line |
| fence width | 76 | 76 |
| `deny.toml` longest | 77, verdict "wrong" | **77 pre-B1, 78 post-B1 - finding 2** |
| acceptance rows / deferred rows | 40 / 8 unchanged | 40 / 8, only W1-g and W1-l changed |
| surfacing list | 7 | (1)..(7) in both states, self-review "7" consistent |

**3. The new `deny.toml` fence is TRUE, sentence by sentence, and lands.**
Verified at `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/`,
at the source and not at the output.

| sentence | verified at | result |
|---|---|---|
| "scopes only the two informational classes" | `src/advisories/cfg.rs:78,80` - exactly two `Spanned<Scope>` fields, `unmaintained` and `unsound`; `src/advisories.rs:113-116` maps only those two variants | TRUE |
| "`unmaintained` defaults to `all`, `unsound` to `workspace`" | `cfg.rs:107-108` `Spanned::new(Scope::All)` / `Spanned::new(Scope::Workspace)` | TRUE |
| "a `workspace` scope reaches only crates a workspace member depends on directly" | `advisories.rs:125-137` - `direct_dependents(nid)` then `any(|dd| ws_set.contains(&dd.krate.id) ^ transitive)`; `ws_set` from `workspace_members()` at `:86-94` | TRUE |
| "glib sits deeper" | 11 direct parents measured, none in the 4-member workspace set | TRUE |
| "produced no error, no warning and not even an ignored note" | pre-state tally: `RUSTSEC-2024-0429` in no note class | TRUE |
| "while the unmaintained advisories beside it all reported" | 16 `note[unmaintained]` | TRUE |
| "a real vulnerability has no scope key at all and always fires" | `advisories.rs:116` `_ => None` -> `else { break 'block; }` = emit branch; 2 `note[vulnerability]` observed | TRUE |
| "`transitive` would exempt first-party unsoundness" | the `^ transitive` inversion suppresses a crate whose direct dependents are all workspace members | TRUE |
| "Both values behave identically on today's tree" | fired, four values, my own scratch configs | `all` EXIT 1 `{RUSTSEC-2024-0429}`; `transitive` EXIT 1 same single id; `workspace` EXIT 0; `none` EXIT 0 - identical, and the probe discriminates |

Landing check against `deny.toml`: 11 lines; **zero** codepoints above 127; zero
glyphs from the house denylist (no em/en dash, curly quote, ellipsis, nbsp);
every comment line starts `#`; the only non-comment line is `unsound = "all"`;
max width **76**. Byte-identical to the B1 verdict's adjudication 1(b) block -
sha256 `eed7ff9276e85a80b42134b3d7f532023653ba15ee92c1a991c30a251655b47a` on
both, so "taken rather than composed" is exact. Placement: finding 1.

**4. The count was DROPPED rather than corrected - is that right?** **Yes.** The
sentence does not under-explain the asymmetry; it explains it better. The old
fence stated a two-way contrast that was false in its arithmetic (18 was the
ignore-entry count) and false in its mechanism. The replacement states a
three-way contrast - `unmaintained` at `all` fires, `unsound` at `workspace` does
not, a real vulnerability has no scope key and always fires - and that is the
complete rule as the source implements it; there is no residual asymmetry a
number would carry. The dropped number was doing rhetorical work ("look how many
reported"), not explanatory work, and it sits in a file whose header comment
instructs the reader to keep adding and pruning ignore entries, so it would have
gone stale by design. Correcting it to 16 would also have been stale-prone
(16 moves when an unmaintained advisory is fixed upstream) while adding nothing
the three-way contrast does not already carry. `proc-normative-count-recomputed`
trigger 2 is the right entry and it points the same way.

**5. The guard deferral's replacement reasoning.** **Correct, and it does not
assert the opposite.** The row's Vehicle column reads "**A one-key owner
decision, PARKED with its measurement attached** ... **Not an automatic add, on
the reviewer's explicit recommendation**", names the cost the reviewer named (the
knob also reddens the gate when an ignored advisory legitimately disappears
upstream, which is the very event the v1.x `glib` entry watches for), keeps the
ROADMAP trigger line as the INTERIM carrier rather than as the answer, and
records the residual the knob cannot reach as the genuinely deferrable part. The
Why column states the refutation as a refutation rather than restating the old
ground. B1's shipped work is unchanged. This is a routed decision, not a
resolved one. The `unused_ignored_advisory` default of `LintLevel::Warn` that the
"not silent at defaults" half rests on is confirmed at `cfg.rs:115`.

**6. Scope.** Section-level sha256 over both states:

```
IDENTICAL: Global Constraints; Execution method; Model tiers; Work-item coverage map;
           Sequencing; Stream A worktree; Stream B worktree;
           Task A2; Task A3; Task A4; Amendment 1; Amendment 2; Amendment 3; Amendment 4
CHANGED:   Authoring-time verification; Corrections; Acceptance map; Task A1; Task B1;
           Plan close; Deferred by decision; Self-review
NEW:       Amendment 5
```

**Task A3's section is byte-identical**, so the worktree copy A3 is executing
against carries the same contract; so are every W3 acceptance row (only W1-g and
W1-l moved, of 40 rows unchanged in count) and Task A2's and A4's sections.
Nothing in ADR D111's territory moved: the commit touches one file, and the D111
design document is not it. Task A1's section did change, but A1 is committed and
reviewed and the change is to a control its implementer had already substituted,
so no live implementer holds a superseded contract.

**7. The amendment does not compress.** Byte counts per changed section, all
directions:

```
Authoring-time verification 30020 -> 36842    Corrections    5291 ->  5562
Acceptance map              16470 -> 16745    Task A1       10084 -> 10531
Task B1                     21147 -> 24102    Plan close     6186 ->  6402
Deferred by decision         4878 ->  6320    Self-review   26400 -> 27239
```

No section shrank in bytes or in lines. Fence B is byte-identical
(sha `9cef5045...` in both states). Typography over the whole post-state file:
zero non-ASCII codepoints, zero trailing-whitespace lines.

**8. The no-work-needed check.** Every "therefore unnecessary" run.

- **"`cargo-deny's default scope for the unsound class excludes transitive
  dependencies` stays, at both sites."** Premise run: exactly 2 sites pre
  (authoring, B1 Step 6), both retained, plus the amendment's own record. The
  sentence is true of this tree and uses the tool's own vocabulary
  (`Scope::Transitive` is the inverted branch of the same filter). Kept
  correctly. Boundary worth one line: `Workspace` and `Transitive` are not exact
  set complements - a crate with both workspace and non-workspace direct
  dependents fires under BOTH - so "literally the complement" in the report
  overstates slightly. Immaterial here; glib has no workspace dependent at all.
- **"Exactly one of Amendments 1-4 restates the void caveat."** Premise run over
  the `## Amendment 1` .. `## Self-review` range: exactly 1 hit. Correct, and
  the author's own note that its first draft said two is the right disclosure.
- **"Fence B is untouched."** Verified by hash, both states.
- **"The authoring section's `git diff --exit-code -- deny.toml` clean claim
  stays."** Premise holds: that claim describes runs made while the repo file was
  unedited, which is the state on `master` today; only B1 Step 5's copy, which
  runs after Step 4 edits the file, was unperformable.
- **"The mise rider is disposed of."** Verified at `docs/ROADMAP.md` (FIRED
  2026-07-30, deliberately RE-DEFERRED, fenced text left unrepaired with the
  stated ground, new observable named) and at `task-a2-brief.md` (prohibition
  present).
- **Report concern 4, "`docs/ROADMAP.md` asserts 18 commented RUSTSEC ignores at
  two sites."** Run: `docs/ROADMAP.md:644` and `:1440` both say "commented
  RUSTSEC ignores" - but neither carries the figure 18 in that clause
  (`grep -c '18 commented'` -> 0). The concern's substance (a stale-count risk
  from the nineteenth entry) is arguable; its stated form is not measured. It is
  a report concern, not plan text, so it is not a finding - but the controller
  should not act on "asserts 18 at two sites" without re-reading those lines.

---

## Evidence appendix

**Instrument root** (created for this review; no instrument the amendment's
author wrote was executed, and no shared default path was used):
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a5rev-independent/`

| instrument | purpose |
|---|---|
| `pre.md`, `post.md` | `git show 83af0d5:<plan>` and `git show 44bc6f7:<plan>`, the two states |
| `deny-master.toml`, `deny-c422999.toml` | the two states of the file the fence lands in, read as blobs |
| `literal-insert.toml`, `replace-region.toml` | the two readings of Step 4(a)'s placement instruction, built from `c422999` plus the fence sliced out of `post.md`'s own markup |
| `rev-noignore-{all,transitive,workspace,none}.toml` | my own four-value scope probe for the fence's last sentence |
| `fire_ext.txt`, `fire_ext_neg.txt`, `fire18.txt`, `fire18neg.txt` | known-present / known-absent controls for the two site-set expressions |
| `audit_cmd.txt`, `audit_pre.txt`, `audit_post.txt` | the gate-count audit expression extracted verbatim from the document, and its hit sets in both states for set-differencing |
| inline `python3` heredocs | fence extraction by fence-marker walk; section sha256 map; acceptance-row sha256 map; diff-hunk-to-section mapping; guard-row column split |

**Commands** (all foreground, absolute paths, read-only against the tree):

```
git -C /home/senol/Git/Muxsmith {rev-parse, log, status --porcelain, worktree list,
      show <rev>:<path>, diff [--stat|-U0|--name-only] 83af0d5..44bc6f7, diff --exit-code}
cargo deny -L info check advisories                      # class tally, repo config
cargo deny check advisories -c <my scratch config>       # 6 variant runs
cargo tree -i glib@0.18.5 -e normal --depth 1
cargo metadata --no-deps --format-version 1
/usr/bin/grep -n/-c/-o/-E/-i over pre.md, post.md, deny-*.toml, docs/ROADMAP.md,
      .superpowers/sdd/plan-11/{task-a1,task-b1}-{report,verdict}.md, task-a2-brief.md,
      progress.md, BUILDING.md, renovate.jsonc
sed -n, awk 'length($0)', diff -u, python3 (hashlib, re, json)
read-only source reads under ~/.cargo/registry/.../cargo-deny-0.19.9/src/
```

`gh` was not invoked. No session-relocation tool was called.
`/home/senol/Git/muxsmith-plan11-a` was never entered, read or listed;
`c422999`'s `deny.toml` was obtained with `git show` from the main worktree.

**Tree integrity, proven at review end:**

```
$ git -C /home/senol/Git/Muxsmith status --porcelain      (empty)
$ git rev-parse HEAD                44bc6f7c8aba002a5dc47b4f3462c1c5929a6597
$ git rev-parse HEAD^{tree}         04610b051183c5daed9fa6579828f75f5a33adf5   (== baseline at review start)
$ git diff --exit-code              exit 0
```

Every mutation this review performed happened on reviewer-owned copies under the
instrument root. This verdict file is the only thing written into the repository,
at the path the brief names.

---

## HARVEST

1. **A fenced instruction has a target STATE, not just a target file, and the
   state moves while the amendment is being written.** This is the sharpest thing
   this round produced. The plan's Step 4(a) was correct on the day it was
   written and became unperformable the moment B1 committed - not because anyone
   edited it, but because the file it addresses changed underneath it. The
   amendment then replaced the fence's CONTENT and left the operation, so the
   defect survived a round whose entire subject was that fence. **Handle:** when
   an amendment rewrites text that an already-committed task shipped, the
   placement/operation clause is re-derived against the POST-commit blob, not
   against the pre-state the plan was authored on. The trigger is readable: the
   amendment is writing a replacement for something a commit already contains.

2. **The same defect class produced both a repair and a new instance in one
   round.** The amendment fixed Step 5's `git diff --exit-code` "clean"
   (unperformable at the point it appears) while leaving Step 4(a)'s insert
   (unperformable against the file it now targets). Both are "an instruction
   whose stated result is unreachable in the state where it runs". Diagnosing a
   class in a document is not the same as sweeping the document for that class -
   which is the same shape this project already recorded when an agent diagnosed
   a failure pattern and produced its next instance in the same file.
   **Handle worth promoting:** when a round repairs an unperformable instruction,
   the repair's scope is the CLASS across the document, established by search,
   not the instance the finding named.

3. **A figure that disagrees with a source figure is a STATE or UNIT question
   until proven otherwise.** The amendment met two verdict figures it could not
   reproduce and treated both as errors. One was a unit difference (2 lines vs 5
   occurrences) and one was a state difference (77 pre-B1 vs 78 post-B1); neither
   was an error. The reflex of re-measuring is right and is what this project has
   built; what is missing is the second step. **Handle:** before writing "the
   source says X and the measured value is Y", reconcile - same file? same
   commit? same unit? - and if the two measurements are of different things, the
   sentence names both rather than declaring a winner. This is the write-side
   counterpart to the already-recorded rule that a borrowed measurement answers
   its own question, not yours.

4. **The amendment's best find and its worst arithmetic are the same paragraph.**
   Searching rather than working from the brief's list is what turned up the
   fourth multi-site defect - a genuine, load-bearing catch the controller's
   brief did not have. Then the paragraph reporting it miscounted its own units
   in both directions, with the errors cancelling to a correct total. **Handle:**
   a passage whose thesis is "the unit is the restatement, not the line" is
   exactly the passage where the unit accounting gets checked cell by cell,
   because a compensating pair of errors reads as confirmation. Extract the table
   row's columns and count per column; do not count per line while arguing that
   lines are the wrong unit.

5. **Report-side verdict requirements evaporate at the file boundary.** Two
   things the B1 verdict explicitly required - correcting the report's "silent by
   construction" and recording the Step-5 contradiction as its third finding -
   fell out of the process because the amendment was scoped to one file and
   nothing carried them onward. The mechanism is working exactly as designed and
   still loses items. **Handle:** an amendment brief that scopes an author to one
   file inherits the duty to route every requirement addressed to another file,
   and the author's concerns section is where that routing lands. The trigger is
   readable: a verdict's "Required change" names a path the author may not edit.

6. **Pattern to keep, observed rather than criticised: the author disclosed two
   of its own near misses.** Its first draft said two amendment-log sites where
   the measured answer was one, and two of its drafts added a thirteenth and
   fourteenth `...` that would have broken the self-review's three-kinds claim.
   Both are recorded in the report as caught rather than silently fixed. That is
   the disclosure the fenced-truth problem needs, and it is the reason this
   review could spend its effort on the two things the author did not catch.

---
---

# Fix round 1 delta re-review (`44bc6f7..ba67cbc`)

**Verdict: APPROVED. All three findings ADDRESSED. No new breakage in the fix
diff.**

**Scoped**, as dispatched: I judged the three findings and the fix diff. New
observations about untouched text are in this section's harvest as deferred
minors and do not extend the loop.

**Graded:** `ba67cbc` on `master`, one file, pathspec-scoped, 17 insertions / 7
deletions. The supplied
`.superpowers/sdd/plan-11/review-44bc6f7..ba67cbc.diff` differs from
`git diff 44bc6f7..ba67cbc` only in a header preamble and a wider context
setting; its 24 `+`/`-` content lines are **byte-identical** to my own
derivation of the range (`cmp` on the extracted change lines), so it is the real
delta and I graded from it. Neither stream worktree was entered; `c422999`'s
`deny.toml` was read as a blob.

---

## Per-finding verdicts

### Finding 1 (IMPORTANT, the placement instruction) - **ADDRESSED**

Step 4(a) now reads: "**Replace the eight lines between the line
`yanked = "deny"` and the comment beginning `# All entries below are
transitive`** - the fence B1 shipped at `c422999`, its seven comment lines plus
its own `unsound = "all"` line - with exactly:". That is the operation, the
count, and the identification of what is being replaced, in the operative
sentence rather than in prose beside it.

**Re-fired from scratch on the `ba67cbc` fence**, configs rebuilt from the
`c422999` blob at my own paths, exit codes captured without a pipe:

```
lines between the anchors                                    8
literal INSERT  -> 2 `unsound` keys, EXIT 1
                   "failed to parse config ...: duplicate key: `unsound`"
                   note/error/warning class lines emitted: 0   (no advisory evaluated)
region REPLACE  -> 1 `unsound` key, EXIT 0, "advisories ok"
postcondition   -> grep -c '^unsound = ' : 1 on the replace form, 2 on the insert form
```

Every claim the new OPERATION paragraph makes reproduces, including the two I
had not measured in round 1: the exit codes, and "without evaluating a single
advisory" (zero diagnostic-class lines - the parse fails before the check runs).
The postcondition it adds is falsifiable and discriminates, which is why the
last line above runs it against both forms rather than only the good one.

**The class sweep, checked independently rather than accepted.** My own
expression over the pre-fix state (`insert|insertion|append|added?ition`, fired
against known-present and known-absent controls) returns 14 lines. Classified,
the sites that describe the Task B1 Step 4 **(a)** operation are exactly four,
and all four moved:

| site | pre-fix | post-fix |
|---|---|---|
| Step 4 lead-in | "Two verbatim **insertions**, nothing else in the file." | "Two verbatim **fences** ... **The operations differ, and Amendment 5's fix round 1 states each one against the file it is applied to:** (a) is a REPLACE ... (b) is an append, unchanged" |
| Step 4(a) | "**Insert**, immediately after ... and immediately before ..." | "**Replace the eight lines between** ..." |
| post-application count | "After both **insertions** the `ignore` list holds **19** ids" | "After both **fences are applied** ... - **one** `unsound` key, not two, which is the postcondition (a)'s replace form exists to guarantee" |
| "Must not decide" | "the two fenced `deny.toml` **insertions**" | "the two fenced `deny.toml` **blocks and the operation each one is applied by**" |

The three beyond the instance my finding named are real, and the author is right
that leaving them would have left normative sentences describing the wrong
operation - the "Must not decide" line in particular, which is what bounds an
implementer's latitude. Correctly **not** changed: Step 4(b) ("Append to the
`ignore` list"), which is genuinely an append, and A3's unrelated "Insert into
the first example's `input:` block".

**Blind-spot pass on my own finding, because a replace deletes seven lines where
an insert deleted none.** I swept for any requirement the replace form would
violate: no "zero deletions", "purely additive" or equivalent constraint exists
anywhere in the document. W1-k's bound is "must show only the two fenced regions
- no existing ignore id reworded, reordered or removed, no other key touched",
all of which the replace satisfies. The fence itself is untouched by the fix
round - sha256 still
`eed7ff9276e85a80b42134b3d7f532023653ba15ee92c1a991c30a251655b47a`, 11 lines,
max width 76 - so nothing this round did reopened the truth question.

### Finding 2 (IMPORTANT, the width figure) - **ADDRESSED**

Both plan sites carry both figures with their states attached; neither was
dropped:

- Amendment 5 figures paragraph: "longest line **76** characters against
  `deny.toml`'s own longest of **77 on `master` and 78 at `c422999`**, so it
  clears the file in either state. **The two figures are the same file in two
  states, not a disagreement**", followed by an explicit account of what the
  first round got wrong.
- Step 4(a)'s artifact-check note: "checked against the artifact it lands in,
  **in the state it lands in** ... **77 before B1's fence B and 78 after it** -
  the three 78-character lines are ones that fence added - so it clears the file
  in either state."

The accusation against the B1 verdict is gone from both, and the causal
attribution matches my measurement exactly (the three 78-character lines at
`c422999` are `+` lines of B1's own fence B). Report concern 5 is **withdrawn in
place rather than deleted** - it now reads "WITHDRAWN at fix round 1 ... **The
verdict is right and this report was wrong**" - which is the right handling for
an escalation that already went to the controller.

The generalisation my verdict proposed is recorded where the figure sits, not
only in the log: "a figure that disagrees with a source figure is a STATE or a
UNIT question until reconciled."

### Finding 3 (MINOR, the guard-premise accounting) - **ADDRESSED**

Re-derived independently rather than checked against the author's text. Over the
pre-amendment document, splitting the deferral row on its column separator and
counting per cell:

```
B1 Step 9 test-duty bullet   1 restatement    "a lint asserting one of its keys would be new gate infrastructure"
deferred-by-decision row     3 restatements   Why:     "the failure would be silent in exactly the way the original defect was"
                                              Why:     "A lint asserting a `deny.toml` key would be new gate infrastructure, ..."
                                              Vehicle: "the failure mode here is silent by construction - drop the key and nothing happens"
plan-close ROADMAP line      0                grep -c 'silent'  -> 0 ; grep -c 'infrastructure' -> 0
                                              TOTAL: 2 lines, 4 restatements
```

The corrected paragraph states exactly that, per cell, with each restatement
quoted, and names the plan-close line as "a consumer of the row's ROUTING rather
than a restatement of its premise ... updated for the routing, not for the
premise". It also records the first round's error in both terms rather than
quietly restating the right numbers.

One sub-claim the fix added that I had not measured, now verified: Step 9's
*other* use of the word `infrastructure` ("using the existing infrastructure
rather than a new scenario") is about the existing test infrastructure and is
correctly excluded from the count. That line carries exactly two occurrences and
only one is a restatement.

---

## New breakage in the fix diff: none found

| check | `44bc6f7` | `ba67cbc` |
|---|---|---|
| gate-count audit, document's own expression | 30 | **30** - the document's "26 -> 30" claim survives the fix round |
| audit controls (`BUILDING.md` / `renovate.jsonc`) | 4 / 0 | 4 / 0 |
| non-ASCII codepoints | 0 | 0 |
| trailing-whitespace lines | 0 | 0 |
| fence markers | 58 | 58 |
| acceptance rows | 40 | 40 |
| lines containing `...` | 12 | 12 |
| replacement fence sha256 | `eed7ff92...` | `eed7ff92...` |

**Scope:** 21 of 23 `##` sections byte-identical. Only **Task B1** (115 -> 117
lines) and **Amendment 5** (41 -> 49) moved. Tasks A2, A3 and A4, all four
earlier amendments, Global Constraints, the Acceptance map, Plan close, Deferred
by decision and the Self-review are untouched to the byte - so A3, still
executing against a pre-amendment copy, again holds no superseded contract, and
the repaired deferral row and plan-close line from round 1 were correctly not
disturbed by an accounting fix.

---

## Retraction: my challenge to report concern 4 falls, and the concern stands

**This is a defect in my round-1 verdict, not in the amendment.** I wrote that
the author's concern 4 ("`docs/ROADMAP.md` asserts 18 commented RUSTSEC ignores
at two sites") was "not measured", on the strength of
`grep -c '18 commented' docs/ROADMAP.md` returning **0**.

The author is right: the figure and its noun phrase straddle a hard wrap at both
sites. Confirmed on an instrument I built myself, and the raw lines show the
wrap:

```
flattened (\s+ -> ' ') count of "18 commented RUSTSEC ignores" : 2

docs/ROADMAP.md:643-644   "... Then walk the 18 / commented RUSTSEC ignores in `deny.toml` and drop the ones ..."
docs/ROADMAP.md:1439-1440 "... prune the 18 / commented RUSTSEC ignores in deny.toml as Renovate PRs obsolete them (S8) ..."
```

**So concern 4 stands as stated: both sites assert 18, and B1's nineteenth entry
makes both stale. The controller owns those two lines and should treat them as
live.**

Worth naming plainly, because it is the same rule this verdict spent two findings
on: I treated a zero from a line-based grep as evidence without firing it against
a known-present case, over a fact whose blind spot this project had **already
documented** - B1's own implementer built a newline-flattened pass for this exact
assertion, and the B1 verdict reproduced it and praised it in its harvest. I
applied the discipline correctly to my own `18` sweep over the plan (I caught a
context-padding blind spot there and disclosed it) and then failed to apply it
one artifact over.

---

## Harvest, and deferred minors (scoped out of this loop)

1. **Deferred minor, probably not a defect at all: one further site carries
   insertion vocabulary about part (a).** Task B1's `Modify:` files list reads
   "**two named regions only** - the `unsound` key **added** to `[advisories]`,
   and one ignore entry with its comment appended to the existing `ignore`
   list", and the fix round did not touch it. Two readings, and the benign one
   is probably right: it bounds *which regions may move* rather than prescribing
   an operation, and it describes B1's own original mandate, which was an
   addition and is now history. The sharper reading is that against `c422999`
   the key is not "added" - it is retained while its comment is replaced.
   Nothing instructs anyone here and every bounding clause ("No other key, no
   other entry, and no existing entry reworded or reordered") survives the
   replace, so I did not raise it as a finding. Controller's call whether one
   word is worth a touch.

2. **A wording class has a normative core and a descriptive skirt, and only the
   core is load-bearing.** The fix round's sweep found four sites and repaired
   four; a fifth (harvest 1) carries the same vocabulary and no obligation. The
   useful discriminator, which this round demonstrates in both directions: does
   the sentence tell someone what to DO, or does it bound what may move? The
   first is the class; the second is prose about a state and is dated by
   construction. **Handle:** when sweeping for an operation word, classify hits
   by whether they are imperative or descriptive before counting a repair
   complete - and say which ones were excluded and why, so the next reviewer
   grades the classification rather than re-deriving the sweep.

3. **The negative check I failed is the one this project has the most evidence
   about.** Three separate artifacts in this plan already carry the rule (the
   conventions' fire-test rule, B1's implementer's flattened pass, the B1
   verdict's harvest item 6), and a reviewer who quoted the rule twice in the
   same document still ran an unfired negative against a hard-wrapped assertion
   in a hard-wrapped file. **Handle worth promoting:** in a repository whose
   prose files are hard-wrapped at ~76 columns, a line-based grep for a
   multi-word phrase is a *known-broken instrument*, not a merely fallible one.
   The flattened pass should be the default for any phrase claim over
   `docs/*.md`, with the line-based form reserved for single tokens. That is a
   sharper and more mechanical rule than "fire your negatives", and it is
   greppable: the trigger is a search pattern containing a space, run against a
   wrapped document.

4. **The fix round modelled the disclosure this process wants.** It recorded its
   own first-round error in both terms rather than silently shipping the right
   numbers, kept a withdrawn concern in place rather than deleting it, and
   stated the operation-vs-state rule beside the figure instead of only in the
   log. All three make the next reader's check cheaper, and all three are the
   reason this re-review could be short.
