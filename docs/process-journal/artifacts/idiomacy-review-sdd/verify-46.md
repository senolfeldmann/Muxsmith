# Idiomacy verify-46

**Finding:** `e2e/i18n-en.ts:136` — `assertAllCatalogsParseCleanly()` runs as a module-import side effect, so a broken catalog fails the spec via an opaque module-load error rather than one named test failure, and the multi-locale parse re-runs per parallel worker. Playwright idiom for a run-gating invariant is a named test (or globalSetup).

**Replacement:** keep `buildEnBundle()` at module scope (memoized bundle for `en()`), move the all-locales guard into a dedicated `e2e/catalogs.spec.ts` with `test("all Fluent catalogs parse cleanly", () => assertAllCatalogsParseCleanly())`.

## Verdict: CONFIRMED

### (a) Code matches the claim — yes
- HEAD `2f17880`. Line 135-136 are top-level statements: `const bundle = buildEnBundle();` then `assertAllCatalogsParseCleanly();`. Both execute at module import time. The module doc itself states this explicitly (lines 48-50): "Runs for its throw side effect at import time, same as `buildEnBundle`: any missing/Junk id in any catalog of any locale fails the whole e2e run immediately."
- `en()` is the sole export, imported by `e2e/smoke.spec.ts` (line 23) — currently the only importer, and it holds 6 `test()` blocks. An import-time throw therefore takes down all 6 as a file-load error, not one named test. Characterization accurate.
- `playwright.config.ts`: `fullyParallel: true`, worker-based. Each worker process re-imports `i18n-en.ts` to get `en`, so the top-level `assertAllCatalogsParseCleanly()` re-runs the full de+en walk (2 locales, 6 ftl each) per worker. Accurate. (Cost is small today, but the mechanism is as claimed.)

### (b) Replacement is current Playwright idiom (1.61.1) — yes
Verified against Playwright docs via context7 (`/microsoft/playwright`), not training memory:
- **globalSetup** is a first-class mechanism: "required and run before all tests", exports a single function — a legitimate home for a run-gating precondition.
- Test-file module-level code runs during the **load-tests task**; a throw there is a **load error** (`failOnLoadErrors`), surfaced as a file-level error, not attributed to any named test. This is exactly the "opaque module-load error instead of one named test failure" the finding describes; the source confirms load happens as a distinct pre-run phase.
- A dedicated `test("all Fluent catalogs parse cleanly", ...)` runs in exactly one worker, once, and reports as a named failure with trace attribution. Both benefits the finding claims (correct attribution + no per-worker re-run) hold. Keeping `buildEnBundle()` at module scope is correctly scoped: `en()` hard-depends on the memoized `bundle`, and an unparseable en catalog invalidates every assertion in the spec, so its import-time throw is defensible; only the broader all-locales guard (consumed by nothing) is the misplaced side effect.

### (c) n/a — not a duplication finding.

### (d) tag=idiom, not yagni. Both a concrete construct (`assertAllCatalogsParseCleanly()` at module scope, line 136) and a concrete replacement (dedicated spec with a named `test()`) are named.

## Decision guard — no conflict, not tracked
Grepped `docs/superpowers/specs/*.md` (D-memos, 8 plan files), `docs/IDEAS.md`, `docs/ROADMAP.md` for `i18n-en`, `assertAllCatalog`, `parseCleanly`, `catalogs.spec`, `globalSetup`, `run-gating`, import-time/side-effect terms.
- Only hits are in `docs/process-journal/artifacts/plan-5.5-sdd/*` (the diffs and task-21/21.5 reports that introduced the guard) — build artifacts, not decision sources, and they only record that the guard exists and runs at import time, not a decision that it *must* be an import side effect rather than a named test.
- ROADMAP's nearest entries — "Test-hygiene collection (B-minors)" (B1-B13) and "check-i18n.mjs fixture self-test (T20-m2)" — do not cover this construct or the attribution/idiom issue.
- The code comment's rationale is about *why the run must go red* on a broken catalog. The finding does not dispute that; it preserves the red while fixing attribution and per-worker duplication. No decision is contradicted.
