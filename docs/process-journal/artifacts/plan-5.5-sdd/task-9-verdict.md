# Task 9 reviewer verdict (model: opus, 2026-07-11/12)

Diff: 240fd35..697dd70, 9 commits on plan55-t9 (review-240fd35..697dd70.diff)

## Spec Compliance
✅ all nine items, source-verified: (i) $rules deterministic (BTreeMap +
ascending ri), message/fixture/test in lockstep (controller verified the
per-commit boundary via git show: bce35cd carries both); (ii) suppression
tightly scoped to the current rule's top-level lists, is_empty() makes the
top-level check sufficient, {} still warns; (iii) filename once with
batch-level diags keeping theirs; (iv) errors-first at ALL human render
sites incl. not-found/query-failed branches; (v) tracks[N] in lint,
template untouched; (vi) donor gets the primary's exact predicate;
(vii) NOT-deliberate determination well-evidenced (F1 commits were
--json-scoped; spec 5.5 unconditional; sibling prints already), fix
strictly inside the human else-branch (JSON untouched); (viii) KEEP
defensible, exactly one §8.4 entry; (ix) both emitter sites identical,
site-specific no-{$ regression drives the real emitter, guard note
updated.

## Scope-boundary judgments
(iv) JSON-unsorted defensible, BUT flat config_diags JSON is now
inconsistent with validate's sorted JSON (validate sorts one vec for both
modes). Minor; consumers sort by severity field.
(vi) Attribution genuinely lossy: unsupported-source has no placeholder,
renders against the PRIMARY file; the offending donor filename appears
nowhere by name (config_path is the only disambiguator); sibling
DonorIsPrimary names its donor via $donor. Spec-compliant per the brief's
explicit scope, but the weakest deliverable. IMPORTANT follow-up.

## Issues
Critical: none.
Important:
- (vi) donor misdirection -> ROUTED to new plan Task 9.5 (donor reference
  in the message, param change; MUST land before T21 translates).
Minor:
- (iv) flat config_diags JSON unsorted vs validate parity; cheap close if
  wanted.
- (viii) exception is slightly broader than "pass-through" (core-authored
  framing text blessed too); named explicitly in the entry, pre-existing
  tension flagged.
- per-commit fixture lockstep: resolved by controller (git show).

## Assessment
Spec compliance ✅. Task quality: Approved.
