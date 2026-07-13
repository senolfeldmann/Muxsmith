### Task 11: config_diagnostics helper (Wave 2)

**Files:**
- Modify: `crates/muxsmith-core/src/profile/validate.rs` (new pub helpers), `crates/muxsmith-cli/src/commands/validate.rs:47-56`, `src-tauri/src/lib.rs:159-167`
- Modify: `docs/ROADMAP.md` (one rider line in the Plan-6 anchor)

**Interfaces:** produces `pub fn config_diagnostics(profile: &Profile) -> Vec<Diagnostic>` (validate + lint::provable_overlaps) and `pub fn config_diagnostics_from_file(path: &Path) -> Vec<Diagnostic>` (Err(d) => vec![d] | Ok => config_diagnostics) in core.

- [ ] Create the two helpers; both duplicated bodies (CLI validate.rs collect, src-tauri validate_profile_body) become one call each.
- [ ] SCOPE BOUNDARY (deliberate): the four X1-1 pipeline sites also carrying the validate+lint two-liner are NOT touched - Plan 6's plan_pipeline() replaces that whole region. Add one rider line to the ROADMAP Plan-6 anchor: "plan_pipeline consumes profile::validate::config_diagnostics (landed Plan 5.6 T11)".
- [ ] Full gate; commit `refactor(core): shared config-diagnostics helper (validate+lint funnel)`.

