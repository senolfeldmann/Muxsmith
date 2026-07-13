# Audit: `testing-support-helpers` (PROMOTION candidate)

**Cluster:** `testing-support-helpers` (kind: pattern, domain: testing)
**Statement:** Cross-file test helpers (`FakeIdent`, `lang()`) are consolidated into a shared `tests/support/mod.rs` subdirectory module - deliberately a submodule, not its own `tests/*.rs` binary, to avoid Cargo autodiscovery - and duplicating a helper within a crate is a defect.
**Claimed count:** 4 | **Promoted:** yes (`promoted_at: 3`)

**Verdict: CONFIRMED** - verified_count = 4 (all four occurrences survive). >= 3 distinct, non-fabricated, correctly-attributed occurrences; the standing rule's count is real.

Ground truth confirmed independently of the ledger: both crates carry a real `tests/support/mod.rs` subdirectory module (`crates/muxsmith-core/tests/support/mod.rs`, `crates/muxsmith-cli/tests/support/mod.rs`); the core one defines `FakeIdent` (L16) and `lang()` (L33) with a doc comment (L4) explaining the autodiscovery avoidance; `CONVENTIONS.md` L30-33 codifies the rule.

---

## Per-occurrence verification

### Occ 1 - 2026-07-09, `deferred` - CONFIRMED
**Ref:** `plan-3-sdd/verdicts/whole-branch-review-verdict.md` (Minor 7) + `plan-3-sdd/verdicts/task-12-review-verdict.md`
- whole-branch verdict L55 (Minor 7): "the `FakeIdent`+`lang()` helper is now duplicated across three test files (T12/T7) ... both are cheap DRY cleanups, not blockers."
- task-12 verdict L43: "`FakeIdent` ... and `lang()` are duplicated verbatim ... this is now the third copy (also in `tests/suggestions.rs`) ... past the 'three similar lines' threshold where a shared `tests/support.rs` (via `#[path]` include) would pay for itself. Worth a small cleanup pass, not blocking."
- Both artifacts flag the 3x duplication and explicitly defer it (non-blocking). Kind `deferred` supported. Correctly attributed to Plan 3.

### Occ 2 - 2026-07-09, `decided` - CONFIRMED
**Ref:** memo D18
- Located at `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`, section "## D18: Plan 3 follow-up triage", L147-148: "Dedup the `FakeIdent`/`lang()` test helpers (3 copies) into `tests/support.rs`; new executor/run tests want the same helpers." (byte-identical to the occurrence's evidence quote).
- A decision memo scheduling the dedup into Plan 4. Kind `decided` supported. Distinct from Occ 1 despite the shared date: Occ 1 is the review flag (deferral), Occ 2 is the scheduling decision - different artifact, different act.

### Occ 3 - 2026-07-10, `reinforced` - CONFIRMED
**Ref:** `plan-4-sdd/verdicts/task-5-review-verdict.md`
- L22: "`tests/support/mod.rs` is a subdirectory module (not `tests/support.rs`), correctly avoiding Cargo's `tests/*.rs` autodiscovery, per its own doc comment."
- L23-24: `FakeIdent` and `lang()` bodies "byte-identical" to the canonical source.
- L25: all three consumers switched to `mod support; use support::{FakeIdent, lang};` "with local struct/fn definitions fully deleted."
- Matches the occurrence evidence verbatim in substance. Kind `reinforced` (implementation + review verification) supported. The submodule-not-binary detail in the cluster statement traces to this ref.

### Occ 4 - 2026-07-12, `reinforced` - CONFIRMED
**Ref:** CONVENTIONS.md Patterns (commit `b38a46f`) + idiomacy finding `run_cli.rs:L498`
- `git show b38a46f`: "docs: house-knowledge instance - CONVENTIONS.md + decision-ledger (Tier 1)", Sun Jul 12 22:22:42 2026; adds `docs/CONVENTIONS.md` L30-33: "Shared test helpers via `tests/support/mod.rs` ... same-crate duplication of a helper is a defect." The codification.
- Idiomacy finding X1-9 in `.superpowers/sdd/idiomacy-review/find-X1.md` L125-137: flags `fake_mkvmerge_that_fails_queries` verbatim-duplicated in `run_cli.rs:498-512` and `dry_run_cli.rs:576-590` (same-crate, crate already has `tests/support/mod.rs`), noting the in-code soft counter-preference ("kept local per this file's existing per-file-helper convention", visible at `run_cli.rs` ~L495-497) and distinguishing it from the tracked cross-crate 3-copy fake-mkvmerge decision. Matches the occurrence evidence exactly.
- A genuine fresh recurrence: a *new* helper (`fake_mkvmerge`, not `FakeIdent`/`lang`) caught same-crate against the now-codified rule. Kind `reinforced` supported.

---

## Skeptical checks

- **No fabrication:** every ref exists and its cited line(s) say what the occurrence claims. No invented artifact.
- **No misattribution:** Occ 1 -> Plan 3 verdicts, Occ 3 -> Plan 4 task-5 verdict, memo D18 -> plan-4 design-decisions, codification -> commit `b38a46f`. All checked against the actual files, not the ledger's own summary.
- **No hidden duplicates:** the four occurrences are four distinct acts (flag / decide / implement / codify+re-flag), not one event double-counted. Occ 1 and Occ 2 share a date but are separate artifacts and separate kinds.
- **Narrative in the statement holds end to end:** "flagged at 3x in Plan 3" (Occ 1), "scheduled (D18)" (Occ 2), "implemented byte-identical in Plan 4" (Occ 3), "codified in CONVENTIONS" (Occ 4).

**Result: promotion stands. 4/4 occurrences survive.**
