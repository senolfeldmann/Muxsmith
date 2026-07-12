# Task 21.5 reviewer verdict (model: sonnet, 2026-07-12)

Diff: 2ae62dd..4d946f4, fix b833f2a on plan55-t215

## Spec Compliance
✅ all four brief requirements with real IPC evidence (recorded-invoke
mechanism verified against mocks source; save-gating genuine - dialog only
closes after setSettings resolves; mockIPC-reassignment reload simulation
verified against @tauri-apps/api source). Failing-e2e-first.

## Review findings -> fix wave b833f2a (controller-verified: 6/6 e2e,
parity green, lint clean)
- Endonym labels: English/Deutsch identical in both catalogs (universal
  picker convention; T21's "precedent" was an accidental byproduct).
- Evergreen hint: no language enumeration (was re-staling at locale #3);
  tightened to two plain sentences after a controller wording pass.
- Restart notice: one sentence, both locales (bootstrap-once architecture;
  silent-until-restart was the exact honesty gap this task existed for).
- e2e locator consistency aligned.

## Recorded (not fixed)
- No live in-session locale switch (pre-existing bootstrap-once
  architecture); product call for later - Plan 6 candidate.
- e2e phase-2 seeds DE settings from a constant rather than threading the
  recorded write (mirrors pre-existing pattern; phase-1 asserts catch the
  silent-save failure mode).

## Assessment
Spec compliance ✅. Task quality: Approved.
