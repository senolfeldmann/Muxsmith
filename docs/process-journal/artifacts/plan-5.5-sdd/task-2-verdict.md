# Task 2 reviewer verdict (model: opus, 2026-07-11)

Diff: 209218c..374005a (review-209218c..374005a.diff)

## Spec Compliance
✅ Steps 1-3 spec compliant. Implementer corrected two brief errors, reviewer
confirmed both: matrix runs ubuntu-26.04 (not 24.04); skip marker is inlined
at 19 call sites, not centralized in tests/support/mod.rs.
⚠️ resolved by controller: commit message documents pin decision + registry
sources (verified via git show 374005a).

## Strengths
- Grep pattern `mkvmerge not found; skipping` verified against all 19 Rust
  call sites (muxsmith-core tests, muxsmith-cli tests, src-tauri/src/lib.rs).
- Design cannot false-pass: PATH-lookup detection means installed-but-not-
  on-PATH self-skips, fires the marker, fails the step loudly.
- Coverage complete and future-proof: assertion reruns the identical
  `cargo test --workspace`; new gated tests are covered without a filter list.
- Shell semantics correct: pipefail + -e abort on real regressions; `|| true`
  scoped to grep's no-match exit only; 2>&1 routes eprintln stderr into the log.

## Failure-mode analysis
(a) present/no skips -> pass; (b) missing/skips -> fails loudly;
(c) other-reason skips -> no false marker (marker only in the mkvmerge-absent
branch); (d) future marker reword -> latent false-negative (Minor 1);
(e) grep zero matches -> healthy pass. No constructible false-negative in the
delivered state. Windows CRLF non-issue (unanchored substring match).

## Issues
Critical: none. Important: none.
Minor:
1. Marker string is an unenforced cross-file contract (19 call sites + 1 CI
   grep, no shared constant); a future reword silently reintroduces the
   false-negative. Hardening: shared const referenced by tests + asserted in CI.
2. Doubled Rust test wall-clock from the assertion rerun (self-flagged).
   Two cheaper designs exist, neither clearly better (single-step loses
   multi-threaded coverage; dropping --test-threads=1 is safe since eprintln
   holds the stderr lock, but serialization is merely stricter than needed).
3. apt exact-build pin 97.0-1build1 fragile (archive rebuild supersedes it,
   install fails "version not found"); correct under the pin preference,
   maintenance cost disclosed.
4. Cross-leg version divergence (apt 97 vs choco/brew 100): live coverage is
   not version-uniform across legs; sanctioned by per-manager pin policy,
   for the record.

## Live-CI checks for the controller (post-push)
- Three legs green WITH "Skip-marker occurrences: 0" and tests actually
  executed (rule out count-0-from-non-run by eye).
- Pins still resolvable at push time (apt 97.0-1build1, choco 100.0.0).
- choco/brew put mkvmerge on PATH in the cargo-test step env.
- Expect possible first-run platform failures: gated tests run live on
  win/mac for the FIRST time in the primary cargo test step - a genuine
  platform bug now fails where it previously passed-by-skipping (intended).

## Assessment
Task quality: Approved. No constructible false-negative; all open items are
documented tradeoffs (Minor) or live-CI-only checks.
