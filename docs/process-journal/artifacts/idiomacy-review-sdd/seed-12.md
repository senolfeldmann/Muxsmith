# Seed 12 — [whole-branch M1] de catalog headers overclaim check-i18n.mjs enforcement

**Verdict: CONFIRMED** (at HEAD `befc74e`)

## Original finding

`whole-branch-verdict.md:36`: de catalog headers claim "keys, placeables and
selector structure mirror it and are parity-enforced by scripts/check-i18n.mjs",
but the script enforces **message-id parity only**. Placeable-name and
selector-structure drift in de is not machine-checked. Fix wording at next
touch, or accept.

## State on disk at HEAD

`scripts/check-i18n.mjs` check 3 (lines 239-289) compares catalog **file sets**
and **message-id sets** per file, via the line-based
`MESSAGE_ID_RE = /^([A-Za-z][A-Za-z0-9_-]*)\s*=/` (line 122). The script's own
header comment says so explicitly ("exactly the same message ids", line 52;
"currently id-set-only", line 118). Nothing else machine-checks de placeables
or selectors:

- `crates/muxsmith-cli/tests/catalog_completeness.rs` parses only
  `locales/en/cli.ftl` (line 384 `include_str!`), never de.
- `e2e/smoke.spec.ts` real-parses de strings it renders (catches Fluent syntax
  errors and spot-checks a couple of placeable-bearing strings), but is no
  systematic placeable-name/selector-structure parity guard.

The de headers, current wording:

| File | Line(s) | Claim | Accurate? |
|---|---|---|---|
| `locales/de/cli.ftl` | 2-3 | "keys, placeables and selector structure mirror it and are parity-enforced by scripts/check-i18n.mjs" | **No** (placeables + selectors overclaimed) |
| `locales/de/diagnostics.ftl` | 2-3 | "keys, placeables and selector structure mirror it (parity-enforced by scripts/check-i18n.mjs)" | **No** |
| `locales/de/gui-batch.ftl` | 2-3 | same as diagnostics | **No** |
| `locales/de/gui-jobs.ftl` | 2-3 | same as diagnostics | **No** |
| `locales/de/gui-common.ftl` | 2-3 | "keys and placeables mirror it (parity-enforced by scripts/check-i18n.mjs)" | **No** (placeables overclaimed) |
| `locales/de/gui-settings.ftl` | 2-3 | "keys mirror it (parity-enforced by scripts/check-i18n.mjs)" | **Yes** — already the correct wording |

So 5 of 6 headers still overclaim (the verdict said "all six"; gui-settings is
already accurate at HEAD). No commit since the verdict touched these headers
for this purpose (`b833f2a` was endonym labels / evergreen hint only).

## Fix

Reword the 5 overclaiming headers to scope the machine-checked claim to keys,
keeping the placeable/selector statement as a convention claim. Model wording
(matching `gui-settings.ftl`'s accurate pattern):

> keys mirror it (id parity enforced by scripts/check-i18n.mjs); placeables
> and selector structure mirror it by convention (reviewed manually, not
> machine-checked)

Leave `gui-settings.ftl` untouched. Manual review in the whole-branch pass
found no actual placeable drift, so this is a comment-precision fix only, no
code or catalog-content change.

- **Tag:** doc
- **lines_cut:** 0 (rewording ~5 header lines across 5 files, net zero)
- **deps_cut:** 0
