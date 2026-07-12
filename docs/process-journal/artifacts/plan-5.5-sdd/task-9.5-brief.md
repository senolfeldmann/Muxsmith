### Task 9.5: Donor reference in UnsupportedSource (added 2026-07-12, T9-review routing)

T9 item (vi) gave donors the primary's UnsupportedSource predicate, but the
message has no placeholder and renders against the PRIMARY file - for an
unmuxable donor the misdirecting line names the healthy primary while the
offending donor filename appears nowhere (config_path is the only
disambiguator). Sibling precedent: DonorIsPrimary names its donor via
$donor. Must land BEFORE T21 translates the catalogs.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (donor UnsupportedSource
  emission site: add a `donor` param), `locales/en/diagnostics.ftl`
  (message gains an optional-donor rendering or a dedicated donor variant
  - pick the shape consistent with the catalog's existing selector usage),
  `crates/muxsmith-cli/tests/catalog_completeness.rs` (fixture in the SAME
  commit)
- Test: planner test asserting the rendered donor line names the donor file

- [ ] Failing test first (rendered output must contain the donor
  filename), implement, fixture lockstep, full nine-part gate; commit
  `fix(diag): name the donor in donor-side UnsupportedSource (T9 follow-up)`.

