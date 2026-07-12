# Task 10 reviewer verdict (model: opus, 2026-07-11)

Diff: 374005a..b0d8741, fix 25c417d (review-374005a..25c417d.diff)

## Round 1: Needs fixes
Guard well-built: exhaustive no-wildcard match over all 38 DiagCodes
(compile-error-on-missing-variant real); real Fluent parser for key
enumeration (malformed entries fail loudly; multi-line selectors handled);
{$ is the correct leak marker (verified against RED output); allowlist
8->24 keys = correct reading of "and future ones", each renders with a
fixture + stale-reverse-check.
IMPORTANT (the review's central named risk paid off): fixture-fidelity
spot-check across emitters found planner.rs:600-605 emits
InvalidPropertyValue with only property+value while the template requires
$allowed - the plan-time invalid-changes.language path renders a literal
{$allowed} to users. Guard is green anyway: per-DiagCode fixtures are
structurally blind to single-site divergence when sibling sites
(validate.rs:242, planner.rs:317) set the full param set. Only multi-site
divergence in the repo (7 other multi-emitter codes verified consistent).
Report overclaimed ("all 38 cross-checked one-for-one", "exhaustive over
the real emitter surface").

## Disposition
- Production fix routed to plan T9 item (ix) (controller plan amendment,
  with site-specific regression test).
- Fix 25c417d: limitation note on fixture_args (right placement, claims
  verified accurate); report correction appended (original preserved).
  Structural diff confirmed doc-only, zero original lines removed.

## Residual Minors
- parse-error fixture carries inert `at` param (faithful to emitters, not
  in template).
- Plural-branch drift out of the guard's reach (string fixtures always hit
  *[other]); disclosed by implementer, out of scope for a leak guard.

## Final Assessment
Spec compliance ✅. Task quality: Approved.
