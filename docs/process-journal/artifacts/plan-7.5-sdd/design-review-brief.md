# Plan 7.5 design review brief (round 1)

Independent reviewer, fresh eyes; you did not author the design. Artifact
under review: `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`
(commit e662994). Ground truth: the controller brief
`.superpowers/sdd/plan-7.5/design-brief.md` (requirements), the spec
(§8.2/8.3/5.2/4.5), the Tier-2 house files, and the ACTUAL TREE - the
design's claims are verified against reality, never taken from the
document. The author's claims of empirical verification are re-run, not
believed.

## Dimensions

1. **Brief compliance**: every "must RESOLVE" item of the brief is
   resolved in the document; the five owner rulings appear unmodified
   (D65-D69) - flag any drift from the ruling wording's substance.
2. **Brief-correction audit**: the document's section 0 corrects brief
   premises. Verify each correction against the tree (a refuted premise
   is a valid outcome, but the refutation itself must be true).
3. **Latitude scan, BOTH forms**: (a) explicit permissions ("either
   works", "implementer may choose"); (b) omission latitude - any
   unenumerated set in a normative position ("one per X" without the X
   list, lists ending open, mandated-but-never-listed). The test: must a
   plan author or implementer invent something they are not licensed to
   invent?
4. **Empirical claims re-run**: the empty-skeleton diagnostic claim
   (quoted `empty-match-expression` warning at `tracks[{i}].match`) and
   the zero-rule drop/keep surfacing - reproduce them yourself against
   the built CLI/core (foreground). Quote fidelity: every quoted
   diagnostic/catalog string byte-checked against the tree.
5. **Citation audit**: file/symbol anchors checked against the CURRENT
   tree (code-comment-line-citations-drift rules); SI-3 parity citations
   spot-checked at ~/Downloads/mkvtoolnix (do the cited files say what
   the design says they say?); licensing boundary respected (no literal
   text adoption).
6. **House conformance**: gui-helpid-equals-labelkey vs the D71 no-new-
   help-ids strategy - verify the claimed D62/check-i18n mechanics
   yourself (does the gate actually stay green with zero new ids and
   zero new topic files, given the buttons carry no help-id?); the
   46-id budget claim vs editor-generic-action-keys; help-mode
   pointer-suppression conformance (D70/D71 claims vs the shipped
   suppression code); help-topic-h1-scheme if any topic content changes.
7. **No-work-needed check**: every passage concluding something is
   unnecessary (D72's no-tooltips ruling and its "obviousness premise
   verified"; "zero new ids/topics"; "no IPC/wire change") - run the
   premise, do not weigh it. For D72 specifically: the brief demanded
   verification against spec 8.3's own wording; check that the design's
   reading of "every non-obvious control" honestly carries the two
   buttons.
8. **ADR quality**: every D-entry has decision, rationale, rejected
   alternatives EACH with a genuine steelman (no strawmen), triggers
   named where created; D-numbering stays within D65-D74.
9. **Spec-amendment check**: the verbatim 8.2 amendment against the
   current spec text (does it splice cleanly, contradict nothing
   adjacent, and cover the flagged 5.2 staleness row correctly?).
10. **e2e plan sanity**: the nine cases against the existing e2e
    structure (files, fixtures, helpers) - implementable as named,
    D62/D55 ripple correctly enumerated.

## Output

Write `.superpowers/sdd/plan-7.5/design-review-round-1.md`: verdict
APPROVED or NEEDS FIXES; findings by severity, each with artifact +
location + what to change; a HARVEST section (dominant patterns,
repeated rejections, over-restriction flags). Final message: verdict
word + at most three lines + the file path.

## Constraints

Read-only on the tree except your verdict file; no git writes; never
call EnterWorktree/ExitWorktree or any session-relocation tool; absolute
paths; anything you run, run foreground.
