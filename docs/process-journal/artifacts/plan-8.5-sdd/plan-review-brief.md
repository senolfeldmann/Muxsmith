# Review brief: the Plan 8.5 execution plan (macOS packaging fixes)

You are the independent reviewer of a plan document. You did not write it,
you have no stake in it, and your job is to find what would cost an
implementer a round-trip or ship a defect - not to admire it.

Repo: `/home/senol/Git/Muxsmith`. Read-only except your verdict file. All
commands foreground, absolute paths, no session-relocation tools.

## What you are grading

`docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md`
(establish the commit yourself with `git log`).

Ground truth, in priority order:

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - the v1 spec,
   authoritative on conflict.
2. `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` -
   the packaging design (D75-D90 plus amendments A1-A3), which governs the
   surface this package modifies.
3. `docs/ROADMAP.md`: the **"Plan 8.5: macOS packaging fixes"** anchor with
   the owner's kickoff rulings, and the three finding entries under
   **"Pre-1.0 release gates"** that carry the measurements.
4. The plan author's brief: `.superpowers/sdd/plan-8.5/plan-brief.md`
   (controller-authored; its defects are in scope for you, and five briefs
   in a row on this project have carried one).
5. The four house-knowledge files (`docs/*.yaml`) as review ground truth.

## Background, so you need nothing else

Plan 8 shipped the packaging and release pipeline; its rehearsal passed
every machine-checkable acceptance item. Two items were reserved for the
owner on real hardware, and that walk-through - the first human execution of
the documented install path - found three defects, two of them 1.0 blockers:
the macOS app does not launch at all (Gatekeeper calls it damaged; the
bundle carries no `_CodeSignature` seal while its binaries carry the arm64
linker's ad-hoc signature; removing the quarantine attribute makes it launch,
confirmed by the owner); the dmg shows a pre-mount license whose text
garbles at the publisher's non-ASCII character; and the release body's three
OS links render as three paragraphs.

## Dimensions

1. **Coverage.** Walk the requirement sources section by section - the
   ROADMAP anchor's three rulings and the three finding entries - and name
   the task implementing each. A requirement with no task is a finding. This
   is the dimension that exists because an implementer sees only its own
   task and cannot notice a task that does not exist.
2. **Surviving latitude, in both forms.** An explicit permission ("the
   implementer may choose", "either approach works") and the commoner form,
   an omission: a mandated set that is never enumerated, a list ending in
   "...", a "one per X" with no X list. The test is not "does a permission
   appear" but "must the implementer invent something it is not allowed to
   invent". Ask it of every normative sentence.
3. **The settled branch, and its evidence.** One item's route depended on an
   experiment - can a Tauri platform-config overlay CLEAR an inherited key,
   or only set one? The author reports it settled on the CLEAR branch, on
   two legs: at the pinned CLI's source, platform configs and `-c` overlays
   merge by RFC 7396, where null deletes a key; and empirically, a
   null-carrying platform config made a rebuilt deb lose its `Recommends`
   line while `Depends` stayed as the control. **Re-verify both legs
   yourself** - this is the claim the whole item rests on, and a merge
   semantic read from the wrong version would send an implementer down a
   route that silently does nothing. Then check that the plan still carries
   the fallback branch as executable (the owner's tiebreaker stands even
   though it did not fire), that the branch condition is observable rather
   than a judgement call, and that the tiebreaker is quoted as the owner's
   ruling rather than re-derived.
   Second claim on the same item, also worth your own check: that the dmg
   bundler attaches its license only when `licenseFile` is set, so clearing
   it removes the defect rather than leaving an empty dialog.
4. **Owner steps are honestly marked.** Some acceptance can only happen on
   macOS hardware. Check that the plan says so, that no task claims a fix is
   verified without that observation, and that any machine-side check is
   named as the machine half rather than as the acceptance. A plan that
   quietly substitutes a config diff for a dialog observation is defective
   even if everything it does is correct.
5. **Claims and anchors.** Every `file:line` the plan cites is re-verified by
   content; every count is recomputed from the enumeration it summarizes.
   The plan carries a corrections table for three premises it refuted in the
   controller's brief (a wrong line range for the release-body defect, an
   imprecise equation of two different overlay mechanisms, and a false claim
   in the plan-8 design about deb/rpm consuming the license file). Rule on
   each: was the refutation correct, and is what replaced it right? A
   refutation that overshoots is as expensive as the premise it corrected.
   Line numbers drift, and this project has been bitten by a recorded site
   list that was wrong in three ways.
6. **House conformance** against the four `docs/*.yaml` files, by entry id -
   including the rule that a frozen transcription target is never rewritten
   but gets a supersession note, which applies to the S22 unsigned-at-1.0
   wording this package must correct.
7. **Scale.** The owner asked for KISS in the ruling itself. Three defects
   should not become a programme. Flag inflation as a finding, and equally
   flag a cut so thin that a task is not independently reviewable.

## The no-work-needed check, standing

Wherever the plan concludes that a guard, an enumeration, a check or a task
is unnecessary ("so we need no X", "X cannot happen here", "the work already
exists"), run the premise that makes it unnecessary. Do not weigh it. That
shape has produced false claims in this project repeatedly, including in
documents that had just diagnosed the shape.

And its counterpart: a safeguard the plan proposes is not argued out again in
review. It is removed only after it is built and measured redundant.

## Output

Write `.superpowers/sdd/plan-8.5/plan-review-round-1.md`:
`## STATUS: APPROVED` or `## STATUS: NEEDS FIXES`, findings by severity with
evidence and a concrete fix per finding, an explicit coverage table, and a
`## HARVEST` section (patterns worth recording, plus any brief or convention
boundary that forced a stop you judge it should have covered - that is a
wanted finding, not second-guessing).

That file plus scratchpad files are your only writes. No git write
operations, no edits to the plan or any product file. `command grep` or
`git grep` when searching the git-ignored `.superpowers/` tree. Quote only
what you have opened; measure every number you report.
