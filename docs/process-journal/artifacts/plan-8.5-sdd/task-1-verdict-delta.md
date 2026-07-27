# Task 1 delta verdict: review round 1 fix (commit `5060ef5`)

Same judge as `task-1-verdict.md` (round 1). Scope: the delta only. Settled
non-findings from round 1 are not reopened; round-1 PASS dimensions were
re-measured only where the delta could have disturbed them.

Commit `5060ef5b8dae58d1a8e649abad00f377f871c953`, parent `9460daf`, one
file, `14/6` lines, two hunks.

- **M1 fix (bullet substance): PASS**
- **A4 extension: PASS**
- **Count recompute: PARTIAL - the absence holds for the set that was swept, and one consuming enumeration over a second set is stale**
- **Report-figure corrections: PASS (4/4)**
- **Fenced-occurrence ruling: outcome correct and mandatory; coverage genuine; reasoning half-holds**
- **Commit hygiene: PASS**

**VERDICT: NEEDS FIXES** - one minor, tightly scoped, and it is a round-1
residue rather than a delta regression. Everything the controller routed
landed correctly and verifiably. The finding is the second instance the
routing message anticipated: a count over an affected set does exist, and
the recompute did not reach it because it was scoped to one of the two sets
A4 joined.

---

## 1. The `:312` bullet: substance matches INSTALL, remainder untouched

Independently re-extracted both revisions of the bullet by content anchor
(the D75 outline's `- **Windows**: SmartScreen interception` bullet, asserted
unique, then the following `- **macOS**` bullet) rather than by line number.
My first extraction attempt grabbed a different `**macOS**` bullet in the
bundler-mechanics section and I discarded it; the anchored form is the one
below.

**Substance, whitespace-normalized:**

```
bullet head  : 'ad-hoc signed - no Apple developer identity, not notarized - so macOS treats it as coming from an unidentified developer'
INSTALL.md:52: 'ad-hoc signed - no Apple developer identity, not notarized - so macOS treats it as coming from an unidentified developer'
IDENTICAL: True
```

Exact match after stripping only `**Gatekeeper:** the app is`, which the
bullet's own `- **macOS**:` label subsumes. Negative control (`ad-hoc` ->
`adhoc` in the INSTALL string) returns False, so the comparison can fail.
**No third wording was invented** - this is INSTALL's sentence, not a
paraphrase of it.

**Remainder untouched.** Everything from the shared anchor `macOS 15+:`
onward, whitespace-normalized:

```
tail old: 'macOS 15+: first open is blocked, then System Settings > Privacy & Security > "Open Anyway" (Apple citation in section 0, note 2); macOS 11-14: Control-click -> Open; alternative for terminal users: `xattr -d com.apple.quarantine`; CLI location inside the app bundle and the manual PATH step.'
tail new: (byte-identical to the above)
IDENTICAL: True   |   negative control (mutated "Open Anyway"): False
```

The 5-line bullet became 7 lines purely by re-wrapping. Incidental
improvement, worth recording because it is a diff line nobody commissioned:
the old bullet carried an 81-character line (`citation in section 0, note 2);
macOS 11-14: Control-click -> Open; alternative`), the new one's widest is
73, back inside the file's prose wrap. No content rides on it.

Neither the Windows nor the Linux bullet of the same outline changed
(context-only in the diff; numstat 14/6 across two hunks accounts for every
line).

## 2. A4 extension

The inserted clause names the site, records that it was missed, gives the
rationale, and closes the classification row:

> D75's own macOS outline bullet also restated the superseded wording
> ("unsigned and not notarized") and was missed at this amendment's first
> pass (Plan 8.5 Task 1 review round 1); it is corrected in place to match
> `docs/INSTALL.md`'s Gatekeeper substance - live decision prose, not a
> frozen fence, so the tree is directly authoritative. This completes the
> Step-4 classification's D75 sub-entry (ruling text + outline bullet).

Correct on all counts I can check. It draws the live-prose / frozen-fence
distinction explicitly, which is the distinction that made M1 a defect and
the section-4.1 occurrence not one. A4's pre-existing text is preserved - the
single removed line in that hunk is the re-broken join at `the note in D75.
Fence bookkeeping, same rule as A3 (frozen graded`, and the fence-bookkeeping
sentence survives verbatim. It conforms to `proc-supersede-never-overwrite`:
the miss stays on the record rather than being quietly absorbed.

## 3. Count recompute: the absence holds for the set swept, one enumeration over a second set does not

I did not accept the absence. I first made the instrument fire against
known-present counts in the same document (`the seven`, `**zero**
owner-pending forks`, `its three red states`, `The 8-asset name set`, `the
four verbatim documentation artifacts`, `exactly one new file`), so an empty
result would mean something.

**Where the implementer's claim holds - verified:**

- **A4 body.** Every numeral in it is a date (`2026-07-27`), a plan/ruling id
  (`8.5`, `1`), a version (`1.0`), or a section reference (`3.1`, `4.1`,
  `4.2`, `4.5`, `11`). The one quantifier over the sites, `all three OS`, is
  a quotation of D75's superseded wording, not a count of sites. **No count
  over the affected-sites set.** Correct.
- **Section 11's frozen-literal list.** The relevant bullet reads `The full
  bundle block of section 3.1 including **every literal** (...)` - a
  universal plus an unnumbered parenthetical. Adding the `signingIdentity`
  literal stales no numeral. Correct.
- **Neighbouring counts in section 11, checked and correctly not touched:**
  `the four verbatim documentation artifacts of section 4 (INSTALL.md,
  draft-body.md, rehearsal-banner.md, linux-tarball-README.txt)` - Task 1
  changed two of those files' *content* and added no artifact, so the set is
  still 4; `The 8-asset name set of D89` and `its three red states` are over
  sets this plan does not touch.
- **D75's note.** `Every site restating the old wording was corrected` is a
  universal, not a numeral. No recompute owed.

**Where it does not hold - finding D1.**

`proc-normative-count-recomputed` trigger 2, last sentence: *"A member that
joins SEVERAL sets at once ... gets the trigger-2 sweep per set it joins, not
only the obvious one."* **A4 joined two sets:** the corrected-sites set (swept
- no count, claim correct) and **the amendment log's entry set** (not swept).

The design's own status line enumerates the amendment set:

```
docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:3-4
Status: DRAFT 2026-07-22, fix round 1 applied 2026-07-23; amendment log at
the end (A1, 2026-07-23; A2 and A3, 2026-07-27). Numbering starts
```

Measured per revision, entries counted from `^\*\*A<n> \(` headings:

| revision | log entries | status line names | agrees |
|---|---|---|---|
| `ff05ac3` (pre-A4) | 3 - A1 A2 A3 | A1, A2, A3 | **yes** |
| `9460daf` (A4 added) | 4 - A1 A2 A3 A4 | A1, A2, A3 | no |
| `5060ef5` (delta) | 4 - A1 A2 A3 A4 | A1, A2, A3 | no |

The `ff05ac3` row is the fire: this is a **maintained** consuming reference,
not decoration - it tracked the set exactly through A1, A2 and A3, and stopped
tracking at A4. Per `conventions.md`, an enumeration that lists a thing is a
dependency, not a duplicate, and the edit that adds a member is the trigger to
visit it.

**Attribution, stated plainly.** The staleness originated in `9460daf`, Task 1
Step 5c, when A4 was appended. The delta neither caused nor worsened it; the
delta extended A4's body without adding a log entry. **My round-1 verdict
missed it too** - I swept the wording sites and never asked which sets A4
itself joined, which is the same scoping error I am now recording against the
recompute. The implementer's claim is precisely true as scoped and the scope
was one set too narrow.

**Second consuming reference, checked and NOT stale:** the plan file's two
`A1-A3` mentions (Global Constraints and Task 1's "Read first"). Both describe
the design as of plan authoring, and the same Global Constraints sentence
already announces `(A4 in Task 1, A5 in Task 2)`. The plan is the instrument
and carries an explicit house deviation that progress never enters it. Nothing
to do there. That is the complete set of live references naming the amendment
range; the pattern was fired over the whole tree to confirm it finds them.

**Fix (minor, one line).** Update the status line to enumerate A4, e.g.
`(A1, 2026-07-23; A2 and A3, 2026-07-27; A4, 2026-07-27)`.

**Forward note, worth more than the fix.** Task 2 appends **A5** to this same
log. Unless the status line is named as a consuming line in Task 2's Step 7,
this defect recurs immediately and identically. Recommend the controller add
one clause to Task 2 Step 7: *"and update the status line's amendment
enumeration."* Fixing A4 without that buys one commit of correctness.

**Scope disposition is the controller's,** not mine: the status line sits in
Task 1's file but the staleness predates the delta. Either fold it into a
second small fix commit here, or route it to Task 2's A5 step, which must
touch that line anyway. I have no preference between them; leaving it in
neither is the only wrong option.

## 4. The four report-figure corrections: 4/4 confirmed, none pushed back

Re-measured each against the commits rather than reading the report's
restatement:

| # | corrected to | my measurement | agrees |
|---|---|---|---|
| m1 | five Step-3 sites enumerated (`INSTALL.md:5`, `:50`, `draft-body.md:1`, `ROADMAP.md:181`, `README.md:101`), plus two separately-classified KEPT sites (`INSTALL.md:31`, `:34`) = seven paths, two classes | five and two, seven total | yes |
| m2 | ROADMAP S22 edit `+3` net (1 removed / 4 added) | numstat 4/1; `:547` -> `:550` | yes |
| m3 | design file 2133 lines pre-edit, "not 2150 as originally reported here" | 2133 | yes |
| m4 | Gatekeeper line `+2` net (2 removed / 4 added), `:50` -> `:52` | +2, `:52` | yes |

m1's rewrite goes further than the correction required and does so in the
right direction: it withdraws the completeness conclusion outright and names
the row it never checked (*"it says nothing about the Design-doc row ... which
I never separately verified. That is exactly the row that turned out
incomplete ... Found by the reviewer, not by this check."*). That is the
candid form. No figure was contested.

One methodological note, not a finding. The report supports the not-in-a-fence
claim with *"no fence markers between lines 250-340"*. That window argument is
locally insufficient - a fence opened before 250 and closed after 340 would
defeat it - and it happens to be sound here only because no such fence exists.
The load-bearing check is the full-file toggle parse, which I ran in both
rounds and which returns `in_fence(312) = False` independently. Conclusion
right, method weaker than the conclusion it carries.

## 5. Ruling: the surviving fenced occurrence

Located and classified independently. Post-delta, `unsigned and not
notarized` occurs in the design at exactly two lines: `:2161` (inside A4's new
clause, quoting it as the thing that was corrected - not a site) and
**`:1647`**, measured `in_fence=True`, fence opening at line **1604**, whose
nearest preceding heading is:

```
1602: ### 4.1 `docs/INSTALL.md` (D75/D82, new, complete)
```

So it is genuinely inside section 4.1's verbatim transcription fence.

**(a) Was leaving it correct? Yes - and it was mandatory, not optional.**
A3's rule, restated inside A4, is that frozen graded material stays
byte-unchanged and the tree is authoritative. Editing that fence would have
violated it, and in round 1 I graded fence byte-identity as a PASS criterion.
Re-measured across the delta: **14 fenced blocks pre, 14 post, corpus
identical**. The delta disturbed no fence.

**(b) Is the coverage genuine? Yes, and at line level rather than section
level.** A4's fence bookkeeping reads:

> 4.1 and 4.2, already tree-authoritative per A3, changed wording again
> (INSTALL.md intro and Gatekeeper lines, draft-body title line)

`:1647` **is** the Gatekeeper line under 4.1, and A4 names it by name, not
merely by enclosing section. Coverage is real and specific. This is the
distinction that separates it from M1: `:312` was live decision prose with no
covering record; `:1647` is frozen transcription with one.

**(c) Does the "measurement rather than re-assertion" reasoning hold? Half.**

The sound half: had the fence been edited too, `every site was corrected`
would be true by construction and unfalsifiable - you cannot check a claim
whose evidence you manufactured. Leaving a classified residue gives a later
reader something to run and something to compare against. That is a real
epistemic property and the instinct behind it is right.

The unsound half, two parts:

1. **It was not a choice.** A3 forbade the edit. Presenting a move compelled
   by the frozen-material rule as one selected for its epistemic virtue is
   post-hoc framing. The correct account is "the rule forbade touching it, and
   the coverage record is what makes that safe" - which is a stronger argument,
   because it does not depend on anyone agreeing about epistemics.
2. **A claim becomes a measurement only when the surface and the result are
   recorded where the claim lives.** The implementer did take the measurement,
   and took it correctly. But it lives in `.superpowers/`, which is
   git-ignored scratch. A reader of D75 in six months has the totality sentence
   and A4; they do not have the report. Today the sentence still rests on the
   reader trusting it, not on a recorded measurement they can re-run.

**Consequence for D75's totality sentence, since it now rests on this.**
`Every site restating the old wording was corrected in the same change (Plan
8.5 Task 1 sweep table)` is **defensible with its parenthetical and false
standalone**. The parenthetical routes the reader to the sweep table, whose
classes distinguish *corrected* from *frozen, superseded by note* - and read in
that vocabulary the sentence is exactly true, because `:1647` is not in the
corrected class. Read on its own it is false by one occurrence. I am not
raising this as a finding: the parenthetical is present, it is the sweep
table's own vocabulary, and the fence classification was settled in round 1.

**Optional polish, explicitly not blocking and explicitly the controller's
call:** one clause would make the sentence true standalone and cost nothing -
`... was corrected in the same change; the frozen section-4.x transcription
fences restate it by design and are superseded by the tree per A3/A4 (Plan 8.5
Task 1 sweep table).` That also lands the measurement next to the claim, which
is what item (c)'s second half is missing. If the controller prefers to leave
D75 alone, nothing breaks.

**Ruling in one sentence:** the unprompted search was worth doing, its result
is correct, the occurrence is genuinely covered by name, leaving it untouched
was required rather than clever, and the completeness claim is sound in the
sweep table's vocabulary while not yet self-evidencing in the document itself.

## 6. Non-regression on round-1 PASS dimensions

Re-measured only what the delta could have disturbed:

- **Three closing checks, post-delta:** (a) **1** hit (`docs/ROADMAP.md:550`,
  unchanged), (b) **0**, (c) **1**. Identical to round 1, as expected - the
  design doc is outside all three scopes, which is exactly why M1 escaped them.
- **Full surface:** 308 -> **309** tracked hits; design 23 -> **24**. Accounted
  for: the corrected `:312` line still matches the pattern (`not notarized`),
  and A4's new clause contributes one line quoting the old wording. Net +1,
  measured, no unexplained movement.
- **Fence corpus:** 14/14 identical across the delta (section 3.1, 4.1, 4.2,
  4.5 and section 11's literals all byte-unchanged).
- **No claim beyond the machine half:** the delta adds no outcome claim. The
  corrected bullet states the same mechanism INSTALL.md states; O1 remains the
  acceptance and is untouched.

## 7. Commit hygiene (delta)

| requirement | measured |
|---|---|
| unsigned | `%G?` = `N` |
| trailer | `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present |
| explicit staging | one file, the one named in the fix; no stray path |
| nothing pushed | `origin/master` = `b1eb231`; `rev-list --left-right --count` = `0 7` |
| no tag | `git tag --contains 5060ef5` = 0 |
| tree clean | `git status --porcelain` = 0 lines |
| message | scopes itself to the review round: `packaging: fix D75 outline bullet missed by the S22 sweep (Plan 8.5 Task 1 review round 1)` |

## 8. Findings

### MINOR

- **D1 - the design's status-line amendment enumeration is stale.**
  `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:3-4`
  names `A1, A2 and A3`; the log has carried **four** entries since `9460daf`.
  Fired against `ff05ac3`, where the enumeration matched the set exactly, so
  it is a maintained reference. A round-1 residue, not a delta regression,
  missed by my own round-1 verdict as well. `proc-normative-count-recomputed`
  trigger 2, multi-set clause. Fix and forward note in section 3.

### OBSERVATIONS (no action required)

- The report's not-in-a-fence evidence uses a line-window argument that does
  not generalize; the conclusion is independently correct (section 4).
- D75's totality sentence is true in the sweep table's vocabulary and false
  read standalone; optional one-clause polish in section 5.

### RE-REVIEW SCOPE AFTER D1

One line. Confirm the status line enumerates A4, confirm the log still has
four `^\*\*A<n>` headings, and confirm no fence moved. If it is instead routed
to Task 2, verify there that A5's step names the status line, and expect five
entries then.
