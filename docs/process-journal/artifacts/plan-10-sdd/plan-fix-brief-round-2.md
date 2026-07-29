# Plan 10 plan fix brief, round 2

The delta review of `afaf9a0` returned **NEEDS FIXES**: one Important, two
Minor. Verdict section `## Delta review after fix round 1` in
`.superpowers/sdd/plan-10/plan-review-round-1.md`.

**All fourteen round-1 findings are confirmed CLOSED.** F2's replacement audit
fires and reproduces at nine sites, the 24/16 corpus reproduces under the
reviewer's own instrument, and the false-clean-tree failure reproduces too. Your
form deviation on F6 - keeping the id as an explicitly rejected reading - was
judged and accepted. Both controller decisions were judged correct. Nothing from
round 1 is re-opened.

## Routing: all three FIX, each written out to the clause

- **N1 (Important)** - fire F5's acceptance forbids an outcome that Step 2's
  fixed counter necessarily produces. The mutation trips the continuation guard
  AND produces a house-block mismatch (2 counted against 1 stated) AND a total
  mismatch (12 against 11); your sentence says the exit must be "NOT with a count
  mismatch", so an implementer running the fire literally gets an outcome the
  plan says must not happen, and the correct response under your own rules is
  NEEDS_CONTEXT. Take the reviewer's replacement clause: the violations must
  INCLUDE the continuation message, the accompanying mismatches are expected
  because the guard does not suppress the comparison, and a run reporting only a
  count mismatch with no continuation message is the failure the fire exists to
  catch.
- **N2 (Minor)** - Step 2 is undefined for a block whose marker is absent, and
  fire F4 walks into exactly that state. State in Step 2 that a missing block
  marker is reported and that block's comparison is skipped, so exactly one
  violation names the cause.
- **N3 (Minor)** - **CONTROLLER DECISION: take the second option, extend the
  expression.** Add the spelled-number alternation so `ten-part` and its kin
  actually match, and re-run the count, rather than dropping `ten-part` from the
  control's description. The cheaper option would leave a control whose
  description and behaviour disagree, and a control that claims coverage it does
  not have is the exact defect this package has now hit three times. Report the
  re-run count; if it changes the nine, say so.

## Nothing else changes

No other edit rides this round. If applying one of the three forces a change
elsewhere, name it in your report rather than widening silently.

Same constraints: plan document only, no git commands, nothing else in the repo.
If a prescribed change is wrong on the merits, refute it with evidence.
