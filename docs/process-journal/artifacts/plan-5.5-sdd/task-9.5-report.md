# Task 9.5 report: donor reference in UnsupportedSource

Branch `plan55-t95`, worktree `/home/senol/Git/Muxsmith/.worktrees/t95`, commit `d5b8bef`.

## Implementation

**Bug.** T9 (commit `22b83ae`) added the donor-side `UnsupportedSource` predicate but reused
the primary's diagnostic wholesale: `.for_file(&primary.path)` with no `donor` param and an
FTL message with no placeholder. Result: for an unmuxable donor, the rendered line named the
healthy primary and the offending donor filename appeared nowhere except `config_path`
(`tracks[N].source.external`, which names the rule, not the file).

**Fix — `crates/muxsmith-core/src/planner.rs` (`resolve_file`):**
- Primary-side emission (line ~453): added `.with("kind", "primary")`.
- Donor-side emission (line ~516): added `.with("kind", "donor").with("donor", donor.display().to_string())`.
- `.for_file(&primary.path)` stays on both — diagnostics attach to the primary's `FileReport`
  (donors have none of their own), exactly like the `DonorIsPrimary` sibling precedent.

**FTL — `locales/en/diagnostics.ftl`:**
```
unsupported-source = { $kind ->
    [donor] mkvmerge identified external donor { $donor } but its container is not a supported muxing source.
   *[primary] mkvmerge identified this file but its container is not a supported muxing source.
}
```

**Doc comment — `crates/muxsmith-core/src/report/mod.rs`:** updated the `UnsupportedSource`
rustdoc (the macro forwards it as the wire-contract doc per its own header comment) to state
it now covers donors too, and documents `kind`/`donor`.

**Fixture lockstep — `crates/muxsmith-cli/tests/catalog_completeness.rs`:** same commit,
`DiagCode::UnsupportedSource` fixture now sets `kind=donor` + `donor=...` (the branch with an
actual placeholder to leak-check; the `*[primary]` branch has none).

## Shape justification (one line, as requested)

Single key + Fluent `$kind` selector (`primary`/`donor`), matching `invalid-template` and
`suggestion-partition` exactly — the only alternative ("dedicated donor key") is not reachable
without a new dynamic-key-selection mechanism, since `DiagCode::key()` is a fixed 1:1 mapping
from variant to FTL id nothing in the renderer's `render_diagnostic` currently branches on
param presence to pick.

I verified empirically (throwaway example, discarded, not committed) that fluent-bundle 0.16
falls back to the default variant when a selector variable is entirely unset — but the catalog's
own precedent (`invalid-template`, `suggestion-partition`) never relies on that fallback; both
always pass an explicit `kind` at every emission site. I followed that idiom rather than the
implicit fallback, so the primary call site now explicitly sets `kind = "primary"` instead of
omitting it.

## TDD evidence

RED first, in `crates/muxsmith-cli/src/i18n.rs` (mirrors the file's existing
`diagnostic_with_file_includes_the_file_path`-style unit tests):
```
test i18n::tests::unsupported_source_donor_variant_names_the_donor_file ... FAILED
panicked: expected the donor filename in: [error] Show.S01E01.mkv tracks[0].source.external:
mkvmerge identified this file but its container is not a supported muxing source.
```
(The paired `unsupported_source_primary_variant_renders_exactly_as_before` baseline-lock test
already passed pre-change, as expected — it pins today's exact text, not a new behavior.)

After implementing the FTL selector + planner params: both tests green.

Added/extended tests:
- `crates/muxsmith-cli/src/i18n.rs`: `unsupported_source_donor_variant_names_the_donor_file`
  (new, was RED) and `unsupported_source_primary_variant_renders_exactly_as_before` (new,
  regression-guards the exact primary string).
- `crates/muxsmith-core/tests/planner_resolution.rs`:
  `unmuxable_donor_yields_unsupported_source_not_unidentifiable` extended to assert
  `params["kind"] == "donor"` and `params["donor"]` ends with the donor's filename.
  `unrecognized_container_yields_unsupported_source_not_missing_track` and
  `unsupported_container_yields_unsupported_source_not_missing_track` (the two primary-side
  tests) extended to assert `params["kind"] == "primary"`, planner-level regression guards
  paralleling the CLI-level text guard.
- `crates/muxsmith-cli/tests/catalog_completeness.rs`: fixture updated (see above); its two
  completeness tests (`every_diag_code_has_a_catalog_message`,
  `every_diag_code_renders_without_leftover_placeholders`) pass unchanged in structure.

## Files touched

- `crates/muxsmith-core/src/planner.rs`
- `crates/muxsmith-core/src/report/mod.rs`
- `crates/muxsmith-core/tests/planner_resolution.rs`
- `crates/muxsmith-cli/src/i18n.rs`
- `crates/muxsmith-cli/tests/catalog_completeness.rs`
- `locales/en/diagnostics.ftl`

## Gate results (all foreground, all green)

1. `cargo fmt --all --check` — clean.
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean.
3. `cargo test --workspace` — 36 test binaries, all `0 failed` (includes the new/extended
   tests above).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — clean.
5. `cargo deny check` — `advisories ok, bans ok, licenses ok, sources ok` (exit 0; pre-existing
   duplicate-crate-version warnings from the Tauri dependency tree, unrelated to this change).
6. `pnpm lint` — clean.
7. `pnpm build` — clean (vue-tsc + vite build).
8. `pnpm check:i18n` — `ok (16 source files scanned, 175 catalog ids, 12 unused warning(s))`;
   the 12 warnings are pre-existing unreferenced `gui-*` keys, unrelated to `diagnostics.ftl`.
9. `pnpm test:e2e` — 3/3 Playwright specs passed.

`pnpm install --frozen-lockfile` was run once (worktree had no `node_modules`).

## Self-review / concerns

- **C2 (EN-only, core prose-free) held**: only `locales/en/diagnostics.ftl` gained prose;
  `planner.rs` only sets structured `kind`/`donor` params, never text.
- **`donor.display().to_string()` is not path-normalized** — for the T9 test fixture (locator
  `path: '.'`) it renders as `<dir>/./Donor.S01E01.srt`, not `<dir>/Donor.S01E01.srt`. This is
  pre-existing `discovery::resolve_locator` join behavior, not something this task's scope
  covers (`DonorIsPrimary`'s existing `.with("donor", donor.display().to_string())` has the
  identical property). My new planner-level test asserts `ends_with("Donor.S01E01.srt")`
  rather than an exact path match, to stay agnostic to that pre-existing quirk. Flagging in
  case a future task wants `resolve_locator` to return normalized paths — out of scope here.
- I did **not** touch `crates/muxsmith-cli/src/commands/mod.rs`'s two test-only
  `Diagnostic::error(DiagCode::UnsupportedSource, "input")` fixtures (hand-rolled, used only to
  test filename-dedup and error/warning-ordering, not message text) — they render via the
  `*[primary]` default variant fine with no `kind` set, and adding one would be scope creep
  against unrelated tests.
- No CI wiring changes; this task didn't ask for any.

## Post-review addendum: fallback-variant regression guard

T9.5 review identified a gap: while the primary-variant output was regression-tested, the implicit Fluent fallback (when `kind` is omitted) was not. Added `unsupported_source_kind_omitted_falls_back_to_primary_variant` in `crates/muxsmith-cli/src/i18n.rs` to pin Fluent's default-variant selection, asserting both that rendering with no `kind` param equals the primary-variant text exactly and contains no unresolved `{$` placeholders. This guards against a Fluent version bump or future emitter inadvertently omitting the `kind` param from silently leaking placeholders into user-facing output. Test is green; `cargo fmt --check` and i18n test suite remain clean.
