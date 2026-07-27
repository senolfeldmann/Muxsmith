# Review brief: plan-7.5 close fix (citations, supersession notes, DE wording)

You are the independent reviewer for a small documentation change at the
close of Plan 7.5 in the Muxsmith project (`/home/senol/Git/Muxsmith`, a
public Rust + Tauri 2/Vue 3 MKV muxing tool). You did not write it. Grade it
against its contracts, not against what the implementer reported.

## What was supposed to happen

The implementer's contract is `.superpowers/sdd/plan-7.5/close-fix-brief.md`
plus one controller addendum delivered mid-task (quoted in full in the
implementer's report). Read the brief first. In summary, three things:

1. **Citation re-pointing.** Plan 7.5's execution record lived in the
   git-IGNORED `.superpowers/sdd/plan-7.5/`; at the close it was salvaged
   into the tracked tree at `docs/process-journal/artifacts/plan-7.5-sdd/`
   (commit `8e2c044`, 31 files). Two citations inside the plan-7.5 design
   document had to move to the salvaged path. Deliberately NOT re-pointed:
   the plan document's tracker-location mentions, the ROADMAP trigger entry,
   the plan's close-actions bullet, and the salvage directory's own internal
   self-references - all of these record what was true or what a trigger
   said, rather than pointing a reader at a live artifact. That boundary is
   the plan-7 precedent (commit `9d01862`).
2. **Supersession notes.** The owner ruled a wording change to the v1 spec's
   section 8.2 on 2026-07-27 (commit `406e91b`). The plan-7.5 design and the
   plan document both still carry the PRE-ruling wording as a verbatim
   mandated block. Those blocks had to stay byte-unchanged (they are the
   historical mandate that Task 4's transcription check graded against); each
   got a one-line supersession note after it instead.
3. **DE cross-reference alignment** (owner ruling, delivered as addendum 1):
   two sites in `help/de/` read "siehe das Thema zur Vorschlagskarte" where
   the German help surface's form everywhere else is "siehe das Thema
   <Topic-Titel>". The word "zur" was to be deleted at both sites, English
   counterparts explicitly out of scope at that point.
4. **EN cross-reference alignment** (owner ruling, delivered as addendum 2,
   after the implementer surfaced it): the same two documents' English
   counterparts read "see the suggestion card topic" in lowercase where the
   other 11 English cross-references spell the topic title as its own h1
   does, against an h1 of "# Suggestion card". Two sites, one character
   each: "suggestion" -> "Suggestion".

   Both wording rulings are the owner's, on his own shipped surface. Do not
   re-litigate whether the alignment was the right call; DO check that what
   landed is what was ruled, that it is correct German and English in
   context, and that the two locales now agree.

## What you review

The union of the implementer's commits since `7302e1b`, EXCLUDING the
salvage commit `8e2c044` (a controller action, already verified in its own
commit). Establish the exact commit set yourself with `git log 7302e1b..HEAD`
rather than taking a number from the report.

Their report is `.superpowers/sdd/plan-7.5/close-fix-report.md`.

**The working tree also carries uncommitted controller changes** to
`docs/ROADMAP.md`, `docs/conventions.yaml` and `docs/decision-ledger.yaml`.
Those are mine, out of your scope, and the implementer was told not to touch
them. Confirm the implementer did not stage or commit any of them - that IS
in your scope.

## Dimensions

1. **Fidelity.** Did each specified edit land exactly as specified, and did
   nothing else change? The quoted mandate blocks in particular must be
   byte-identical to their pre-change state. Verify that yourself against
   `7302e1b`; do not accept the report's proof as the proof.
2. **Completeness against the named surface.** Are there sites the change
   should have covered and did not? Run your own content-anchored searches.
   Note that a firing positive control proves your PATTERN, never that your
   SEARCH SURFACE was complete - name the surface you searched.
3. **The re-pointing boundary.** Are the four deliberately-untouched site
   classes really the right ones, or did the boundary leave a citation that a
   public-repo reader cannot follow and that is NOT a historical record? A
   defensible disagreement here is a finding worth stating; say which side you
   land on and why.
4. **German correctness** of the two aligned sentences: does the replacement
   read correctly in context, and does it match the target topic's actual h1?
   The house form is the criterion, not your own stylistic preference.
5. **House conformance**: the four house-knowledge files (`docs/*.yaml`) are
   review ground truth alongside the brief. Flag deviations by entry id.
   Relevant here at minimum: `proc-wrapped-prose-quote-grep`,
   `proc-sweep-surface-completeness`,
   `proc-verification-step-must-be-falsifiable`.
6. **Commit hygiene**: unsigned (`%G?` = `N`), trailer present, explicit
   staging (no unrelated paths), no push.
7. **Adjudication.** Three items, each gets an explicit verdict from you,
   phrased in both directions - "the implementer was wrong" is as available
   an answer as agreement:
   a. It refuted the brief's site enumeration as off by one (an eighth
      tracked site: the plan's own close-actions bullet, which cites the
      pre-salvage path while registering the very trigger this change
      discharges). Was that correct, and was leaving it untouched right?
   b. It refuted a verification clause that said "three files" where its own
      enumeration named two. Correct?
   c. It surfaced the English casing deviation from inside a scope boundary
      that told it not to touch those files, and reported instead of
      editing. Was the finding correct on the measurement, and was reporting
      rather than fixing the right handling of a boundary whose premise it
      believed to be wrong?

## Output

Write `.superpowers/sdd/plan-7.5/close-fix-verdict.md`:
`## VERDICT: APPROVED` or `## VERDICT: NEEDS FIXES`, findings by severity
with evidence, the two adjudications, and a `## HARVEST` section (patterns
worth recording, including anything the brief itself got wrong - the brief
was controller-authored and its defects are exactly the class you are here
to catch).

That file and scratchpad files are your only writes. No git write operations,
no edits to any product file, no session-relocation tools
(EnterWorktree/ExitWorktree or equivalent). Every command foreground,
absolute paths. Note the environment trap: `grep` is bound to a function that
respects `.gitignore`, so a plain `grep -r` silently skips the git-ignored
`.superpowers/` tree; use `git grep` or `command grep`. Quote only text you
have opened; measure every number you report.
