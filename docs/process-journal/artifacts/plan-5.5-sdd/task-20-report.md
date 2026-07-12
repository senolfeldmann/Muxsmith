# Task 20 Report: check:i18n cross-locale key parity (#17 step 2)

## What was implemented

`scripts/check-i18n.mjs` grew a third, independent check on top of the two
Task 12 already had:

3. **HARD FAILURE (exit 1): cross-locale key parity.** `locales/en/` is
   treated as the reference/canonical locale (`src/i18n/index.ts` falls
   back to it per-message via fluent-vue's bundle chain). Every *other*
   `locales/<tag>/` directory must carry exactly the same set of `.ftl`
   catalog files as `locales/en/`, and within each shared file, exactly
   the same message ids. Four drift classes are each reported by name,
   not just "parity failed":
   - a catalog file present in `en/` but missing from the other locale,
   - an unexpected catalog file present in the other locale but not in
     `en/`,
   - a message id present in `en/`'s copy of a shared file but missing
     from the other locale's,
   - an extra id present in the other locale's copy but not in `en/`'s.

   With only `locales/en/` present (today's tree), the comparison loop
   has no other locale directory to iterate over the very first time it
   runs `readdirSync(locales/)` — it passes trivially by construction,
   not via a special-cased early return. It activates automatically the
   moment a second `locales/<tag>/` directory (Task 21's `locales/de/`)
   exists.

Refactored `parseCatalogIds(file)` (implicitly rooted at `locales/en/`)
into `parseCatalogIds(path)` (takes a full path) so the same parser
serves both the existing en-only scan (checks 1/2) and the new
per-locale scan (check 3); added `listCatalogFiles(dir)` for "all `.ftl`
files in a directory" versus the existing `gui-*`/`diagnostics.ftl`-only
filter checks 1/2 still use unchanged.

## The cli.ftl exclusion decision

**Extended parity to `cli.ftl`, kept it excluded from checks 1/2.**

The script's existing header documents why checks 1 and 2 (literal
`$t()`-call resolution and the unused-key warning) exclude `cli.ftl`:
it's CLI-only vocabulary, mirroring `src/i18n/index.ts`'s own glob,
which never loads it because the Vue frontend never calls `$t()` for a
CLI-rendered string. That reasoning is about *who calls `$t()`* and is
untouched by this task — still correct, still scoped to checks 1/2.

Cross-locale parity is a different question: *does a translated catalog
track its English source?* Task 10's `catalog_completeness.rs` (the
"catalog-completeness guard" the brief points at) only ever parses
`locales/en/cli.ftl`. It proves every EN `cli.ftl` key resolves to a
`DiagCode` or sits on an explicit allowlist, and that each renders
without a leaked `{$param}` — internal EN-side wiring, nothing about a
second locale. It has no way to know whether `locales/de/cli.ftl`
exists or matches `en/cli.ftl`'s key set, because it doesn't read that
path. Task 21 explicitly creates `locales/de/cli.ftl` as one of the six
catalogs it translates. Once that file exists, it's a real, shipped,
user-facing catalog exactly like any `gui-*.ftl` — arguably more
exposed, since CLI output is plain text with no UI chrome to visually
mask a fallen-back-to-English string. Excluding it from parity would
protect against nothing the EN-side guard doesn't already cover, while
leaving the one catalog most likely to visibly regress with no
structural check that a German translation stays complete as `cli.ftl`
evolves post-1.0. So: keep the checks-1/2 exclusion (different
question, still valid), extend check 3 to cover `cli.ftl` (the question
it actually answers is otherwise unguarded). Recorded in both the
script's header comment and here, per the brief's requirement.

## Attribute-level parity

Grepped `locales/en/*.ftl` for Fluent attribute syntax (`.label = ...`
lines) — none found in any of the six catalogs today. Key-level parity
is therefore sufficient; noted in the header (mirroring the existing
`MESSAGE_ID_RE` comment's own attribute caveat) that attribute parity
would need adding to `parseCatalogIds` and the check-3 comparison the
day a catalog gains one, so this isn't a silent gap left undocumented.

## Self-test approach

No permanent fixture-based test added. Rationale: the new logic is
set-difference over filenames and id lists, structurally the same
complexity class as the existing checks 1/2 (which also have no
dedicated fixture test — they're exercised against the real repo tree
and reviewed by inspection, same as Task 12 shipped them). Per the
brief's own "your call, justify," I judged this doesn't cross into
"real logic" that earns a persistent fixture harness. Instead, verified
activation with a throwaway `locales/de/` directory (see below), mirroring
Task 10's own RED-then-revert demonstration pattern for its guard.

## How it was verified to activate for a second locale

1. Copied all six `locales/en/*.ftl` files verbatim into a throwaway
   `locales/de/` → `pnpm check:i18n` stayed green, reporting "1 other
   locale(s) checked for parity against 6 en/ catalog(s)" (proves a
   correct mirror passes, not just that a missing locale passes
   trivially).
2. Introduced four independent drift cases into that throwaway `de/`:
   removed `validate-ok` from `de/cli.ftl` (missing id, in the catalog
   the exclusion decision is about), deleted `de/gui-settings.ftl`
   entirely (missing catalog file), added a stray key to
   `de/gui-common.ftl` (extra id), added a whole new `de/gui-orphan.ftl`
   (unexpected catalog file). `pnpm check:i18n` failed (exit 1) and
   named all four violations correctly:
   ```
   check-i18n: cross-locale key parity violations:
     locales/de/gui-settings.ftl: missing (present in locales/en/)
     locales/de/gui-orphan.ftl: unexpected catalog file (no locales/en/gui-orphan.ftl)
     locales/de/cli.ftl: missing id "validate-ok" (present in locales/en/cli.ftl)
     locales/de/gui-common.ftl: extra id "stray-extra-key" (not present in locales/en/gui-common.ftl)
   ```
3. `rm -rf locales/de` to fully revert the throwaway directory;
   confirmed via `git status --porcelain locales/` that it left no trace,
   and `pnpm check:i18n` returned to the original trivial-pass output
   ("0 other locale(s) checked for parity").
4. Confirmed via `git status --porcelain` (repo root) that only
   `scripts/check-i18n.mjs` is modified before staging/committing —
   the throwaway `locales/de/` never touched git state (it was
   filesystem-only, outside version control the whole time).

## Gate results (full nine-part gate, from worktree root, all foreground)

- `cargo fmt --all --check`: clean
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo test --workspace`: all green (78 tests in muxsmith-cli's suite
  alone, plus muxsmith-gui/xtask/codegen suites, zero failures)
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`
- `pnpm install --frozen-lockfile`: `Already up to date` (no lockfile
  change; this task touched no dependency)
- `pnpm lint`: clean
- `pnpm build`: clean (`vue-tsc --noEmit && vite build`)
- `pnpm check:i18n`: `ok` (17 source files, 177 catalog ids, 12
  pre-existing unused warnings unrelated to this task, 0 other locales
  checked — trivially green on today's en-only tree)
- `pnpm test:e2e`: 4/4 Playwright specs pass

## Self-review findings

- `parseCatalogIds` signature change (filename-relative-to-`LOCALES_EN`
  → full path) is the only breaking change to an existing function;
  grepped the file for all three call sites (the checks-1/2 `knownIds`
  build loop, and the two new check-3 sites) and confirmed each passes
  a full `join(...)` path now.
- Confirmed `catalogFiles` (checks 1/2's `gui-*`/`diagnostics.ftl`
  filter) is untouched in meaning — same filter predicate, just now
  derived from the new `listCatalogFiles` helper instead of its own
  inline `readdirSync().filter()` — so checks 1 and 2's behavior over
  today's tree is provably unchanged (confirmed by the unused-warning
  list being byte-identical, 12 entries, before and after this change).
  `catalogFiles` still legitimately excludes `cli.ftl`, unaffected by
  the check-3 decision above.
- Considered whether "unexpected catalog file" and "extra id" should be
  warnings rather than hard failures (matching check 2's warning-only
  design for unused GUI keys). Decided against: check 2's warning-only
  status exists because of a *known, accepted, structural* false-positive
  class (ids reached only via `$t(err.code, ...)` patterns, documented
  in the header). Cross-locale drift has no equivalent legitimate
  false-positive — an extra id in a translated catalog is either a typo,
  a stale leftover, or a translator's addition that never made it back
  into `en/`, all real bugs. Kept it a hard failure across all four
  drift classes for symmetry and because that's what "structurally
  safe" (this task's own framing) means for a merge gate.
- Verified the smoke-test's `rm -f locales/de/gui-settings.ftl` (a
  zsh-aliased `rm -i` swallowed the first, non-forced attempt silently)
  actually took effect before reading the parity output, rather than
  trusting the first attempt's output blind.

## Concerns

- None structural. One forward note for Task 21: `otherLocales` is
  discovered from `locales/`'s actual directory listing (not a
  hardcoded `["de"]`), so no further change to this script is needed
  when `locales/de/` lands — it'll simply start participating the next
  time `check:i18n` runs, exactly as the header predicts.
- The four failure messages are deliberately terse (`locale/file: class
  "id" (...)`) rather than grouped/pretty-printed per locale; consistent
  with the existing checks' flat-list style (`missing`, `unused`) and
  keeps the diff small. Worth revisiting only if a locale count beyond
  `en`+`de` makes flat output hard to scan — not a concern at today's
  scale (feedback_scale_appropriate_design).
