# Plan 7 design review, round 6 (T13 hover-clear amendment)

Artifact: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
(+25/-8). Same reviewer; scope is the D52 clear-enumeration amendment
and its two ripples (amendment 6(b), section 9). Probes foreground; the
execution worktree `.worktrees/plan7-f` was consulted read-only (greps
only, nothing modified).

## Verdict: APPROVED

No findings.

## Dispositions

1. **Amended D52 mechanics - internally consistent, gap correctly
   closed.** The old text cleared only `pinnedId` on view switch while
   asserting the sidebar "falls to the new view's topic" - provably
   false under its own resolution chain (pin > hover > view): with every
   view root annotated, `hoverId` is non-null after the first hover, and
   the nav sits outside `<main>` so no mouseover fires en route to the
   tab, leaving the stale hover topic selected. With both refs cleared,
   the chain lands on the new view's topic instantly - the amended
   paragraph now proves the assertion it always made. The recorded
   reproduction is structurally faithful to the tree: verified in the
   worktree that `<nav>` (App.vue:161) sits outside `<main>` (:215-237),
   `SuggestionCard.vue:74` carries `data-help-id="batch-suggestion-card"`
   and `BatchView.vue:334` carries `view-batch` - hover card, click
   `nav-jobs`, and under the pin-only clear the resolution chain yields
   exactly the stale `batch-suggestion-card` topic described. The
   pin-clear rationale is retained unchanged.
2. **The narrowness clause - watertight.** "The hover clear triggers
   ONLY on a view switch" plus the explicit contrast with the
   delegation's normal null-setting ("never an eager clear") confines
   the new behavior to one trigger and expressly preserves amended
   D54's pinned-branch fallback (hover off-table -> null -> pinned else
   view). No eager clearing is implied anywhere; the one other clear
   site (help-mode exit tears down both refs and the listeners) remains
   explicitly enumerated in the adjacent sentence, so "nothing else
   clears hoverId eagerly" cannot be misread as forbidding mode
   teardown - D52 is the named interaction authority and states both.
3. **Amendment 6(b) - consistent, lands together.** The third pin-release
   condition now reads "or the active view is switched, in which case
   the hover state resets too and the sidebar shows the new view's
   topic" - the same semantic as amended D52, attributed to the T13
   amendment, and explicitly still landing via Task 21 with the rest of
   amendment 6. No contradiction with 6(a)'s hover-fallback wording
   (which the narrowness clause exists to preserve) or with (b)'s Esc
   qualifier and (c).
4. **Section 9's line - same content, no third variant.** "A view switch
   clears both `pinnedId` and `hoverId` - and nothing else clears
   `hoverId` eagerly; Esc yields to an open settings dialog; hover and
   focusin are equivalent (D52)" restates D52's clear set, negative
   guard, Esc qualifier and hover/focus equivalence exactly; the old
   pin-only line is gone.
5. **Sweep - clean.** The only surviving pin-only-clear wording is the
   corrective record inside D52 itself ("with only the pin cleared, the
   asserted fall-to-new-view-topic did not happen"), correctly
   attributed to T13. D54's fallback sentence (:649) is untouched, as
   the narrowness clause requires. The header's fork-closure claim
   stands: this was a self-consistency defect inside an already-ruled
   mechanic, controller-ruled as a narrow fix - no new fork, no
   escalation language introduced.

## HARVEST

- **Third live traversal of the execution-layer gates**, and the first
  of the *self-consistency* class: rounds 4-5 caught premise defects
  (document vs tree), this one caught the document contradicting itself
  (clear enumeration vs asserted outcome) - a class the design rounds,
  mine included, verified only statically. The reproduction recipe
  (concrete element, concrete click path, observed stale state) is the
  UI sibling of round 3's probe-profile pattern; "a behavioral assertion
  in a design is demonstrated by a reproduction recipe, not only by its
  enumeration" extends the fire-verification convention candidate from
  round 5 to interaction mechanics.
- **The deliberately-narrow clause is the right immunization**: T13's
  fix could have been generalized at the keyboard ("clear hover whenever
  the topic would go stale") - the clause pre-empts exactly that
  over-generalization, names the one trigger, and anchors the preserved
  semantics to the section (amended D54) that depends on them. Pairs
  with round 4's aligned-paraphrase discipline as the two halves of
  keeping a multi-site behavior description coherent under amendment.
- **Over-restriction watch**: nothing wrongly stopped. The negative
  guard ("nothing else clears hoverId eagerly") is a new tight
  constraint, but it is semantic-carrying interaction behavior -
  squarely enumeration-bound under the presentation-token carve-out's
  own boundary - and it exists to protect an owner-approved fallback
  semantic, not to gate implementer style.

## Whole-document justification

A one-line-class defect got the full treatment proportionally: the
self-contradiction is precisely identified (the clear enumeration never
supported the asserted outcome once the view roots became annotated -
an interaction between two amendments that no single round's static
read caught), the fix is the minimal set change that makes the ADR's
own assertion true, the reproduction is recorded concretely and its
structural premises verify against the implemented tree, and the
deliberately-narrow clause plus the section-9 negative guard close off
the over-generalized fix an implementer might otherwise reach for. Both
ripples say the same thing as the authority section, and the sweep
confirms no stale variant survives. Approved without reservation.
