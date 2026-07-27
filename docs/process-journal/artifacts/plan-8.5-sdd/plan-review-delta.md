# Plan 8.5 plan review, delta judgement (round 1 fixes)

Same reviewer as round 1, same standards; delta only - settled
non-findings not reopened. Graded: commit
`ff05ac3c94179d2052f1d6349718b16319d99b52` ("plan: Plan 8.5 round-1
review fixes (M1 + five minors)"), 64 insertions / 14 deletions, touching
only `docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md`
(verified `git show --stat`). Working tree clean at start and end. The
interleaved `f627105` (plan-9 recon inventory; +61 ROADMAP lines, one hunk
at `@@ -249,6 +249,67 @@`) is the unrelated commit that moved the ROADMAP
a second time; it is not part of this judgement except where it tests the
new content anchors.

## STATUS: APPROVED

All six findings are faithfully complied with, none over- or
under-shoots, and nothing unrelated rode along - I walked every hunk and
each maps to one of the six findings or to the self-review paragraph that
documents the round. The one number the author corrected in MY verdict is
indeed my error (ruling below).

## The decisive question: does the new Step 6 close the failure mode?

**Yes - verified by simulation against measurements at HEAD**, not by
reading the step. The three checks, each run by me at HEAD (pre-edit
state):

- (a) the scoped grep: exactly **4** hits (`draft-body.md:1`,
  `INSTALL.md:5`, `INSTALL.md:50`, `ROADMAP.md:547` - the wording line,
  matching the step's snapshot :547). The step now enumerates this
  pre-edit fire as its OWN invocation, names its structural blindness to
  the two other sites in-step, and orders "Never report the sweep green
  on (a) alone".
- (b) `command grep -c 'per-OS unsigned-install' README.md` = **1**
  pre-edit (the fire), expected 0 post-edit; correctly scoped to
  README.md with the two legitimate survivors (plan-8 frozen fence,
  design 4.5 fence) named.
- (c) `command grep -c 'superseded in part 2026-07-27, Plan 8.5 ruling 1'
  docs/ROADMAP.md` = **0** pre-edit; the pattern is a verbatim substring
  of Step 3's new S22 block, so it reads 1 iff that edit landed; presence
  checks self-verify.

Skip-site simulation: skip the S22 ROADMAP edit -> (c) returns 0, red.
Skip the README edit -> (b) returns 1, red. Skip any of the three
(a)-visible edits -> (a) returns >1, red. Every one of the five sweep
sites now has a check that goes red on its omission, and every
absence-expecting check fires by its own invocation. The Step-4 sentence
that borrowed the fire is replaced with what the sweep actually proves.
The failure mode the Major existed for is closed.

(One observation, no finding: Step 4's new parenthetical says the
pre-edit sweep firing "confirms the CLASSIFICATION below is complete" -
strictly, hitting the five corrected sites confirms those five rows;
bucket completeness is confirmed by comparing every hit against the
buckets, which round 1 did and the two new bullets complete. The
corrective gates all live in Step 6 now, so nothing rests on the loose
phrasing.)

## The other five fixes, verified

- **Anchors (m1):** all three `:483` citations are content anchors now;
  the only remaining `483` in the plan is the historical narrative
  (":483 -> :489 -> :550"). Measured at HEAD: `Whatever lands` sits at
  ROADMAP:550, the wording line at :547 - both exactly as the fix
  records. The un-annotated `ROADMAP:221-223` citation in the coverage
  map still resolves (f627105's insertion point is :249, after it;
  `Acceptance: quarantine` measured at :221). The kept-list's :375+
  snapshots are stale by +61 by construction, and the fix explicitly
  demotes every ROADMAP number to a dated snapshot with the quoted
  content as the anchor - which is the durable form; the file moving
  twice in one day while the fix ran is the finding proving itself.
- **:168 relabel (m2):** now "Plan-8 section preamble 'Carries its own
  constraint set (code signing, notarization) ...' (still-true framing;
  snapshot :168)" - measured at HEAD, :168 is exactly that line. Correct.
- **Hit classes (m3):** two new kept-bullets enumerate the historical
  plan-preamble SI-4 class (nine plans named; plan-8's two hits covered
  by "their Global-Constraints blocks") and this plan file itself,
  excluded by role with the reason stated. Matches my round-1 sweep
  residue exactly; the out-of-surface bullet was consistently trimmed
  (ledger/process-conventions hits moved into the commit-signing-subject
  bullet). Complete.
- **Branch switch + fallback A5 (m4):** stronger than prescribed, in the
  right direction: a printed `1` now returns NEEDS_CONTEXT with the
  verbatim Step 1-2 outputs, and the fallback executes only on controller
  dispatch; the fallback header enumerates exactly three triggers, each
  routing through NEEDS_CONTEXT first; the plan close routes a failed O2
  the same way (trigger 3) - the internal references stay consistent
  ("fully specified - a dispatch, not a second planning round" preserves
  the brief's both-branches requirement). The fallback A5 is carried
  verbatim as item 6 with exactly one bracket-marked slot whose source is
  named (the Task 2 report or the O2 failure record - an observed output,
  not a judgement). Its content carries everything the old transformation
  described (route, trigger quote, D82 scope note, D86 correction, fence
  bookkeeping now naming BOTH new files), and its mechanism sentence
  matches what I verified at the bundle_dmg source in round 1. No
  latitude remains on this surface.
- **RR4 (m5):** my measurement is recorded as evidence (LPic=2 on run
  30273529210's `muxsmith-macos-arm64` dmg, with the drafts-deleted
  context and the fired `gh release list` control), the artifact
  download is named the primary corroboration while retention lasts, the
  ~2026-08-03 expiry is stated, and the post-expiry epistemic state is
  honest (recorded measurement stands; planted-copy fire remains the only
  live mechanics check). Faithful to the finding and to what I actually
  measured.

## Ruling on the arithmetic footnote: the author is right, the error was mine

Measured at HEAD by extracting the Task 1 Step 3 S22 fences: old block
**3** lines, new block **6** lines - shift **+3**, not the "+5 (3 old
lines -> 8 new)" my round-1 minor 1 parenthetical claimed. The 8 was the
new fence counted WITH its two fence markers - a measurement error of
exactly the class my own verdict policed elsewhere. The footnote is
confirmed; my round-1 file stands uncorrected on disk (per the
new-file-not-edit constraint), and this delta judgement is the correction
of record. The prescribed fix is untouched by it, as the author said:
content anchors are drift-proof regardless of the drift's magnitude, and
the actual drift that occurred between my review and the fix was +61 from
an unrelated commit - a shape no line-number fix would have survived.

## Delta discipline

Hunk-by-hunk walk: coverage-map duty line, O1, plan close (m1); Step-4
surface paragraph (M1 relabel + m2 + snapshot demotion); kept-list
(m2/m3); Step 6 (M1); Task 2 Step 4, Step 7 pointer, fallback header,
fallback item 6 (m4); RR4 (m5); self-review paragraph (documents the
round, updates the placeholder claim to match the now-verbatim fallback
A5). No product file, no workflow, no design/spec/ROADMAP edit in
`ff05ac3`. Nothing beyond the findings.

## HARVEST (delta round)

- **H6.** A content anchor proved itself mid-fix: the anchored line moved
  +61 lines from an unrelated commit between review and fix, and every
  re-anchored citation still resolves (measured :550/:547 at HEAD). The
  kept-list's "snapshot :N + quoted content" form is the right template
  for citing a hot file.
- **H7.** The reviewer's own numbers are subject to the same discipline
  as the plan's: my +5 was a fence-marker miscount that survived because
  the parenthetical was illustrative, not load-bearing - exactly where
  proc-normative-count-recomputed says unwalked numbers hide. An author
  verifying findings before complying (and footnoting rather than
  refuting) is the process working in both directions.
