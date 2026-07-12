# Task 22 reviewer verdict (model: sonnet, 2026-07-12)

Diff: b094ff9..aba7f4f, fix 5a1bd8f on plan55-t22

## Spec Compliance
✅ all mandate items verified: redaction complete against all 11 committed
snapshots (none embeds mkvmerge-produced text - the query-failed message
is a fixed-string Fluent key fed by a fake stub; 97-vs-100 CI matrix
survives); insta =1.48.0 exact, independently registry-re-verified;
conversions preserve semantic asserts (4 spot-checked); CI strict via
default CI=true / no INSTA_UPDATE; EN pinned by construction. Review-
before-accept did real work (caught an unredacted tmp path pre-commit).

## Concern judgments
1. regex-crate wording in two snapshots: correctly scoped (Cargo.lock
   exact; breaks only on deliberate cargo update; §10 third-party
   exception).
2. mkvmerge-present narrowing: moot under T2's CI installs; flagged
   correctly - but left a CONTRADICTING stale header comment.

## Fix wave (5a1bd8f, controller-verified: 29/29 tests, fmt, deny)
- dead predicates dev-dep removed (still transitive via assert_cmd, no
  breakage);
- stale header reconciled into one accurate statement.

## Assessment
Spec compliance ✅. Task quality: Approved.
