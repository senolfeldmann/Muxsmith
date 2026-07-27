# Task 1 verdict: ad-hoc signing + the S22 wording sweep (design amendment A4)

Reviewer: independent task reviewer (did not author the task). Commit
`9460daf14089d6d61af8fed859bd5f1aaedf46f7` on `master`, range
`ff05ac3..9460daf` (parent established via `git rev-parse 9460daf^`, not
taken from the report). Ground truth read in the plan's stated precedence:
plan 8.5 Task 1 + Global Constraints + Owner steps; the plan-8 packaging
design incl. A1-A3; `docs/ROADMAP.md`'s Plan-8.5 anchor and the three
finding entries; the four house YAML files by entry id.

- **Fidelity: PASS**
- **Sweep completeness: FAIL (one uncovered live site)**
- **No claim beyond the machine half: PASS**
- **House conformance: PASS**
- **Commit hygiene: PASS**

**VERDICT: NEEDS FIXES** - one major finding, no blocker. The commit itself
is byte-perfect against the plan; the defect is a site the plan's own
classification table promised would be amended and which no Step-5 edit
touches, leaving the newly inserted D75 note and A4 asserting a completeness
that is not true as written.

---

## 1. Fidelity: every specified edit, byte-compared, and nothing else

Method: the ten verbatim old->new blocks were extracted from the plan file by
line range, fences stripped, and compared as exact substrings against the
post-edit tree. A negative control (the config block with `"-"` mutated to
`"x"`) returned 0 occurrences, so the comparison can fail.

| block | target | occurrences in tree |
|---|---|---|
| Step-1 config block | `src-tauri/tauri.conf.json` | 1 |
| INSTALL intro (new) | `docs/INSTALL.md` | 1 |
| INSTALL Gatekeeper (new) | `docs/INSTALL.md` | 1 |
| draft-body line 1 (new) | `.github/release/draft-body.md` | 1 |
| ROADMAP S22 record (new) | `docs/ROADMAP.md` | 1 |
| README rider (new) | `README.md` | 1 |
| plan-8 supersession note | `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md` | 1 |
| A4 (a) D75 note | design | 1 |
| A4 (b) D86 bullet | design | 1 |
| A4 (c) amendment-log entry | design | 1 |

Every one of the five superseded OLD blocks measures **0 in HEAD and 1 in the
parent blob** (the parent measurement is the fire: the same comparison
returns a hit against a known-present case).

**Nothing else changed.** `git show --numstat` gives exactly seven paths, and
the set is byte-identical to Task 1's `Files:` list (diffed against a
literal expectation file - no out-of-scope path):

```
.github/release/draft-body.md                                         1/1
README.md                                                             1/1
docs/INSTALL.md                                                       7/3
docs/ROADMAP.md                                                       4/1
docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md         2/0
docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md  32/0
src-tauri/tauri.conf.json                                             2/1
```

Ten diff hunks total, one per specified edit. Insertion points verified
against the plan's anchors: the D75 note follows the `... and not design
scope.` paragraph; the D86 bullet follows the `macOS.minimumSystemVersion`
bullet; A4 is appended after A3 at end of file (design 2133 -> 2165 lines).

Collateral count preserved: `grep -c 'placeholder(1.0)' README.md` = **4**,
identical to the parent. The rider is a comment-text edit, not a resolution
(D81 constraint holds).

### The frozen fences are byte-unchanged (verified, not taken)

Two independent measurements, both against the parent:

1. **Zero deletions** in both fence-carrying files (design 32/0, plan-8 plan
   2/0), so no pre-existing byte was rewritten.
2. **Fence corpus identity.** Every fenced block was extracted from both
   revisions and compared as a list: design **14 fenced blocks, parent ==
   head**; plan-8 plan **30 fenced blocks, parent == head**. This catches the
   case zero-deletions does not - an insertion landing *inside* a fence.
3. **Fence-parity of the inserted lines.** Design insertions at post-edit
   lines 286-295, 979-985, 2151-2165; plan-8 plan at 362-363. Membership test
   against the fence toggle: **none inside a fence**.

The plan-8 plan's Task-3 README fence (`:358`) is intact and now carried by
the supersession note at `:362`. Correct treatment per
`proc-supersede-never-overwrite`: the frozen transcription target keeps its
statement, the supersession rides beside it.

### Step 2's red state

The report's account is internally consistent with the tree and with the
plan's authoring-time observation, and it honestly discloses that
`git diff src-tauri/tauri.conf.json` showed the one-line addition **plus the
trailing-comma change on the preceding line** - which the plan's Step-2
wording ("exactly the one-line addition") did not anticipate. Disclosing that
rather than glossing it is the right call; the diff confirms 2 added / 1
removed, i.e. exactly the comma plus the new key. Not a finding.

---

## 2. Sweep completeness - the task's hard part

### 2.1 The three closing checks, as I measured them

Each run against the post-edit working tree, then the **identical invocation**
fired against the parent blob `ff05ac3` (a negative result is worth nothing
until the same call is watched returning a hit).

**(a)** `git grep -nE 'unsigned on all three|All 1.0-era builds are \*\*unsigned\*\*|unsigned builds|the app is unsigned' -- README.md docs/INSTALL.md docs/ROADMAP.md .github/release/`

- **Post-edit: 1 hit** - `docs/ROADMAP.md:550`,
  `  to the "unsigned on all three OS at 1.0" wording and wants an explicit`
  (the BLOCKER finding entry quoting the old wording as its subject; matched
  by content, exactly as the plan directs).
- **Fire (same invocation, `ff05ac3`): 4 hits** -
  `.github/release/draft-body.md:1`, `docs/INSTALL.md:5`,
  `docs/INSTALL.md:50`, `docs/ROADMAP.md:547`. Exactly the four the plan
  names.

**(b)** `command grep -c 'per-OS unsigned-install' README.md`

- **Post-edit: 0.** **Fire (parent README blob, same pattern): 1.**
- The two legitimately surviving instances of that string outside README are
  confirmed present and untouched: plan-8 plan `:358` (frozen fence, covered
  by the Step-4 note) and design `:1797` (section 4.5 fence, covered by A4).

**(c)** `command grep -c 'superseded in part 2026-07-27, Plan 8.5 ruling 1' docs/ROADMAP.md`

- **Post-edit: 1.** Parent: 0. Because a presence check cannot self-fire on
  the parent, I added a pattern-sanity control on the same parent blob
  (`grep -c 'UNSIGNED artifacts on all three OS'` = 1), proving the
  invocation shape reaches that file's content.

All three reproduce the plan's expected values. The (a)-grep's structural
blindness is real and confirmed: it matches neither the S22 record's
`UNSIGNED artifacts on all three OS` (case + the word `artifacts`) nor
README's `per-OS unsigned-install`. Reporting green on (a) alone would have
been a false pass on two of the five sites; (b) and (c) close exactly those.

### 2.2 The surface I swept, and the full accounting

Surface: `git grep -niE "unsigned|not signed|notariz"` over the **entire
tracked tree** (the plan's named surface), re-run by me post-edit, plus a
supplementary pass for phrasings the regex cannot see
(`all three OS|three OS at 1\.0|no code signature|not code-?signed|ships? unsigned`)
over `README.md BUILDING.md docs/INSTALL.md docs/ROADMAP.md .github/
docs/superpowers/specs/ src-tauri/`.

**308 hits post-edit, 302 pre-edit** (the report's 302 is correct). Every hit
is accounted for; the buckets sum to 308 exactly:

| bucket | hits | disposition |
|---|---|---|
| `docs/process-journal.md` + `docs/process-journal/artifacts/**` | 228 | out of surface (dated historical records) |
| plan 8.5 itself | 28 | excluded by role (the instrument; its hits are edit instructions) |
| other `docs/superpowers/plans/**` | 11 | 10 SI-4 commit-signing class + 1 frozen fence |
| `docs/superpowers/specs/**` | 23 | see below |
| `docs/ROADMAP.md` | 11 | 2 corrected/new, 9 kept |
| `docs/INSTALL.md` | 5 | 3 new, 2 kept (Windows) |
| `*.yaml` house knowledge | 2 | SI-4 commit-signing class |
| `README.md`, `BUILDING.md`, `.github/`, `src-tauri/` | 0 | swept clean |

**Live surface = 52 hits** (308 minus journal and the plan itself), fully
classified: 11 corrected/new, 21 kept and still true as written, 7 inside
frozen fences covered by A4 or the Step-4 note, 12 commit-signing class,
**1 uncovered** (finding M1).

Kept sites I checked individually rather than accepting the plan's table:

- `docs/INSTALL.md:33,:36` - SmartScreen, "the installer is unsigned" /
  "for an unsigned build". Windows genuinely stays unsigned. Correct.
- `docs/ROADMAP.md:168` (Plan-8 preamble constraint set), `:221-222`
  (Plan-8.5 ruling 1 describing this very transition), `:439`, `:502-503`
  (the registered signing-revisit trigger; its subject is real identity
  signing, untouched by ad-hoc), `:546`, `:550` (the BLOCKER entry's own
  analysis), `:990` (Homebrew-Cask v1.x). All still true post-change.
- design `:51` (historical correction record), `:279` (D75 heading),
  `:281-283` (the S22 ruling text, now immediately followed by the
  supersession note), `:368` (D76 secret-custody rationale), `:1229`,
  `:1238` (D90), `:1815` (section 5 table), `:1842-1843` (section 7). The
  "no keys, no certificates, no identity" posture these lean on is unchanged
  by ad-hoc signing. Correct.
- The SI-4 lines in plans 5, 5.5, 5.6, 5.7, 5.8, 6, 7, 7.5 and 8 plus
  `decision-ledger.yaml:4418` and `process-conventions.yaml:104` are commit
  signing, a different subject class. Correctly excluded.

### 2.3 M1 (MAJOR): one live site of the superseded wording was not amended

`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:312`,
inside D75's own decision body:

```
- **macOS**: unsigned and not notarized; macOS 15+: first open is blocked,
```

Measured: **not inside any fence** (fence-membership test returned
`in_fence=False`), i.e. live design prose, not frozen graded material.

This is not a site the plan overlooked. The plan's Step-4 classification
table names it explicitly:

> - Design doc, amended via Step 5: D75 ruling text + its INSTALL outline
>   bullet ("unsigned and not notarized"); section 3.1/4.x fence divergences
>   recorded in A4.

Step 5 has exactly three edits. (a) covers "D75 ruling text" - the note is
inserted directly after it. (c)'s A4 fence bookkeeping enumerates sections
3.1, 4.1, 4.2, 4.5 and 11 - none of which is this bullet. **Nothing in the
task amends the INSTALL outline bullet**, and A4's own enumeration does not
reach it.

Why it matters beyond bookkeeping:

1. The bullet is D75's description of what `docs/INSTALL.md` says about
   macOS. `docs/INSTALL.md:52` now says the opposite ("the app is ad-hoc
   signed"). The design and the tree diverge, and Global Constraints forbid
   exactly this: "Where this package contradicts a D-number, the task says so
   explicitly and carries the design amendment - **never a silent
   divergence**." A4 says it for the fences; nothing says it for this bullet.
2. The text this commit inserted asserts the opposite. D75's new note ends
   "Every site restating the old wording was corrected in the same change
   (Plan 8.5 Task 1 sweep table)." As written that is false by one site, in
   the same section, 17 lines below the sentence making the claim. This is
   the same defect class the task exists to remove: a recorded statement that
   is untrue as written.
3. `proc-sweep-surface-completeness` is directly on point - a firing positive
   control proves a sweep PATTERN, never that its SEARCH SURFACE is complete;
   the two are separate obligations. All three Step-6 checks are scoped to
   `README.md docs/INSTALL.md docs/ROADMAP.md .github/release/` or narrower.
   **The design doc is in none of them.** The design row of the
   classification table was the only thing covering it, and it was not
   executed.

**Accountability.** The implementer executed Step 5 verbatim; editing `:312`
without instruction would itself have been a fidelity violation, and
`proc-latitude-clause-boundary` forbids resolving a discovered fork at the
keyboard. The correct implementer move was **NEEDS_CONTEXT with a decision
memo** at Step 4, when the classification table is read. Instead the report
records "Premises refuted: None" and asserts the table complete. The residue
is a plan defect; the missed catch is a task defect.

**Routing (controller decision, not an implementer's).** Two options:

- **(i) Correct the bullet** to match the tree, e.g. `- **macOS**: ad-hoc
  signed, no Apple developer identity, not notarized; macOS 15+: ...`, and
  extend A4 by one clause naming the D75 outline bullet among the corrected
  sites. Consistent with what this very commit already did to D75 prose (a
  note) and D86 prose (a bullet), so it sets no new precedent, and it makes
  the note's completeness sentence true.
- **(ii) Leave the bullet, extend A4's bookkeeping** to declare it superseded
  by the note above it. Cheaper, but preserves a live design sentence that
  contradicts the tree, which is the thing Global Constraints call a silent
  divergence.

**Recommendation: (i).** The bullet is live prose, not frozen graded
material, so the byte-unchanged rule does not protect it; and (i) is the only
option that leaves no false statement in the tree. Note for whoever executes
it: adding a member to A4's corrected-sites set fires
`proc-normative-count-recomputed` trigger 2 - sweep the plan's Step-4
classification row and any count over that set in the same change.

---

## 3. No claim beyond the machine half: PASS

The plan forbids any task from claiming the owner's acceptance (O1). Checked
three surfaces:

- **Commit message.** `packaging: ad-hoc sign the macOS bundle + S22 wording
  sweep (Plan 8.5 ruling 1, A4)`. Describes the config change and the sweep.
  No "fixes", "verified", "resolves", no dialog claim. Clean.
- **Report.** Grepped for `verified|confirm|works|fixed|resolved|proven|passes|success|accepted|launch`.
  Every hit is about the grep fire-verification or the config-schema
  validation. **No claim anywhere that the signing fix was observed on macOS
  hardware**, and no claim about RR3. Clean.
- **Edited docs.** The three sites stating a macOS outcome - `INSTALL.md:52`
  ("so macOS treats it as coming from an unidentified developer"), the D75
  note ("produces the documented ... flow instead of Gatekeeper's 'damaged'
  refusal"), the D86 bullet ("seals the bundle with
  `_CodeSignature/CodeResources` so a quarantined install fails Gatekeeper
  as ...") - are all plan-verbatim, are mechanism/documentation statements in
  the indicative, and none asserts an observation was made. `A4`'s
  "re-verified at v2.tauri.app/distribute/sign/macos 2026-07-27" refers to
  the vendor-documentation check that did happen at plan authoring, not to
  hardware. O1 is untouched and still pending.

Standing note for the plan close, not a Task-1 finding: `INSTALL.md`'s new
Gatekeeper sentence states the expected macOS behaviour before O1 has run.
That is a plan-authoring choice for user-facing documentation, and the
ROADMAP BLOCKER entry's duty ("Whatever lands, `docs/INSTALL.md`'s macOS
section is re-verified against the real flow afterwards", folded into O1) is
precisely the corrective. It must actually be executed.

---

## 4. House conformance, by entry id

| entry | disposition |
|---|---|
| `proc-supersede-never-overwrite` | **PASS.** The reversed S22 wording is never rewritten into its opposite or deleted: the ROADMAP record keeps `UNSIGNED artifacts on all three OS at 1.0` and gains a bracketed supersession pointer; D75 keeps its ruling paragraph and gains a note; the plan-8 frozen fence keeps its bytes and gains a note beside it; A4 carries the live rule with the pointer back. The reversal is reconstructible from the files alone. |
| frozen transcription target never rewritten | **PASS.** Verified by measurement, not assertion: 30/30 plan-8 fenced blocks and 14/14 design fenced blocks byte-identical to the parent; no insertion inside a fence. A4 carries the fence bookkeeping (3.1, 4.1, 4.2, 4.5, 11) in the A3 form. |
| `proc-05-commit-signing` | **PASS.** `%G?` = `N`. |
| `proc-07-verify-against-source` / `proc-57-briefs-not-ground-truth` | **PASS at task level.** Every load-bearing mechanism claim (the `-` pseudo-identity, the schema's `["string","null"]` typing) was verified at source at plan authoring and re-confirmed by the implementer's own green/red validator runs rather than taken from the brief. |
| `proc-latitude-clause-boundary` | **PARTIAL - see M1.** No latitude was taken (nothing unspecified was invented), which is the primary duty. The secondary duty - a fork found on code contact returns as NEEDS_CONTEXT - was not discharged: the classification table's unexecuted design row is a fork and went unreported. |
| `proc-normative-count-recomputed` | **FAIL at report level - see m1.** The count contradicts its own list (failure shape 1). |
| `proc-sweep-surface-completeness` | **FAIL - see M1.** Pattern validity was demonstrated three times over; surface completeness was asserted, not established. |
| `proc-03-model-assignment` | **PASS (informational).** Task 1 is the cheap tier and the plan's ground holds - every edit was carried verbatim, and the implementer needed no judgment call. Worth recording that the one thing the task missed is exactly the thing a cheap-tier transcriber is least likely to catch: a classification-table row with no matching step. Consider whether the classification-table confirmation belongs to the reviewer rather than the implementer. |

---

## 5. Commit hygiene: PASS

| requirement | measured |
|---|---|
| unsigned (`%G?` = N) | `N` |
| trailer present | `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, line 3 of a 4-line message, exact bytes confirmed with `cat -A`; `git log --format='%(trailers)'` parses it as a trailer |
| explicit staging, never `git add -A` | 7 paths committed, byte-identical to Task 1's `Files:` list; no stray path, no untracked artifact swept in |
| nothing pushed | `origin/master` = `b1eb231`; `git rev-list --left-right --count origin/master...HEAD` = `0 6`; `git branch -r --contains 9460daf` empty |
| no tag created | `git tag --contains 9460daf` = 0 |
| no path outside scope | confirmed by set diff against a literal expectation file |
| tree clean after commit | `git status --porcelain` = 0 lines |

The Task-4 precondition ordering is respected: the gate and the push are
deliberately not this task's business, and the implementer did not push.

---

## 6. DONE_WITH_CONCERNS adjudication

**The report flags none.** Section "Premises refuted" reads "None. Every
content anchor ... matched the tree exactly as the plan carried it. No fork
triggered, no NEEDS_CONTEXT."

Adjudicated: nothing to sustain or overturn, and that absence is itself the
substance of M1. A concern was owed. The plan's Step-4 classification table
row for the design doc names a site as "amended via Step 5" that Step 5 does
not amend; the fork was on the page the implementer read, in the same step
whose completion it reported.

---

## 7. Findings by severity

### MAJOR

- **M1 - the sweep leaves one live site of the superseded wording.**
  `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:312`
  still reads `- **macOS**: unsigned and not notarized; ...` in live,
  non-fenced D75 prose. Not covered by the D75 note, not covered by A4's
  fence enumeration, not reachable by any of the three Step-6 checks. It
  contradicts `docs/INSTALL.md:52` in the same commit and falsifies, by one
  site, the completeness sentence this commit inserted into D75. Routing and
  recommendation in section 2.3.

### MINOR (report accuracy; no artifact impact, no re-edit required)

Four measured numbers in `task-1-report.md` do not survive re-measurement.
None changes a file, all are the `proc-normative-count-recomputed` /
quote-and-number-checked class, and they are listed so the report is not
carried forward as a source of figures:

- **m1 (the substantive one).** Report line 44: "the five Step-3 sites
  (`docs/ROADMAP.md:181`, `README.md:101`, `docs/INSTALL.md:5`,
  `docs/INSTALL.md:31`, `docs/INSTALL.md:34`, `docs/INSTALL.md:50`,
  `.github/release/draft-body.md:1`)". That is **seven** paths under the
  label "five", and two of them (`docs/INSTALL.md:31`, `:34`) are the plan's
  explicitly KEPT Windows sites, not Step-3 sites. The count contradicts its
  own list, and the list misclassifies. The sentence then concludes
  "confirming the plan's classification table was complete" - checking that
  the corrected sites appear in the surface cannot establish that, and the
  row it did not check is the one that fails (M1).
- **m2.** Report line 29: "this run's edit added 2 net lines to the S22
  record above it". Measured **+3**: `docs/ROADMAP.md` numstat is 4 added / 1
  removed, and the (a)-check hit moves `:547` -> `:550`.
- **m3.** Report line 14: "A3 was the last entry, file ends at line 2150
  pre-edit". The pre-edit design file ends at line **2133**; 2150 was its
  position after the D75 and D86 insertions had already landed. The
  placement itself is correct (A4 appended after A3 at end of file).
- **m4.** Report line 9: the Gatekeeper line "shifted from :50 by the intro
  edit's **+3** lines". The intro hunk is 2 removed / 4 added = **+2**; the
  stated result (`:52`) is right, the delta is not.

### Recorded as correct, worth not losing

- The report's disclosure that the config diff carries a trailing-comma
  change beyond the "one-line addition" the plan predicted is exactly the
  right instinct: a small honest deviation reported rather than smoothed
  over.
- Reporting check (a) as `:550` while the plan cited `:547`, and naming why,
  is correct content-anchoring on a file that moved three times on
  2026-07-27. The plan's insistence on content anchors over line numbers
  earned its keep here.

---

## 8. Re-review scope after the fix

M1's fix touches `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
only. A scoped re-review needs: the corrected bullet byte-compared against
whatever the controller rules; A4 extended and its enumeration re-swept per
`proc-normative-count-recomputed` trigger 2; the fence corpus re-measured for
identity (14 blocks, parent vs new HEAD); and the three Step-6 checks re-run
unchanged, since the design doc lies outside all three and their values must
therefore stay 1 / 0 / 1. The minors need no re-review; correct the report in
place if it is salvaged into `docs/process-journal/artifacts/`.
