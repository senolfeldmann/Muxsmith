### Task 9: Diagnostics polish, eight items (#13, #11, R1)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs:526-533` (OverlappingRules claimants), `~:431` (donor UnsupportedSource), `crates/muxsmith-core/src/profile/validate.rs:65,301` (double report), `crates/muxsmith-cli/src/commands/mod.rs:46ff` (double filename), `dry_run.rs`/`run.rs` (severity sort), lint vs planner rule-ref formatting sites (grep `rules\[` and the lint's bare-index format), `run.rs:131`/`dry_run.rs:98` (query-failed human path), IdentifyError `detail` handling
- Test: respective test files per item

Items (each its own commit, one task because one reviewer can gate the set):
- [ ] (i) OverlappingRules names ALL claimants: params become a rendered list (`$rules`), catalog message updated. Failing test: three claimants, all three named.
- [ ] (ii) `any: []` single report: suppress `EmptyMatchExpression` when `EmptyMatchList` fires for the same node (validate.rs:65 vs :301). Failing test: exactly one diagnostic.
- [ ] (iii) Human mode prints the filename once: drop the per-line `diagnostic-line-file` prefix when the batch header already named the file. Failing test asserts single occurrence.
- [ ] (iv) dry-run/run sort diagnostics errors-first, matching validate's `Reverse(severity)`. Failing test: warning+error emitted, error printed first.
- [ ] (v) Rule references formatted identically in lint (`0`) and planner (`tracks[0]`): unify on the planner's `tracks[N]` form; adjust lint tests.
- [ ] (vi) Donor-side UnsupportedSource: same predicate as :374 at the donor identify branch (~:431). Failing test: identifiable-but-unmuxable donor yields UnsupportedSource, not UnidentifiableSource.
- [ ] (vii) query-failed human path: FIRST verify intent (10-min pre-check per R1): read the F1 fix commit + code comment ("human mode is unchanged (stderr only)"). If deliberate: document as code comment + memo line, no code change. If not: print config diags before the query-failed message, mirroring the locate-failure branch. Either outcome is a valid task completion - record which.
- [ ] (viii) IdentifyError English `detail`: same pre-check protocol. If kept (third-party text pass-through like spec 8.4's clap exception): add it to the spec 8.4 exception list explicitly. If not: route through a catalog key. Record which.
- [ ] (ix) (added 2026-07-11, T10-review finding) planner.rs:600-605 emits InvalidPropertyValue with only `property`+`value` - the template also requires `$allowed`, so the plan-time invalid-`changes.language` path renders a literal `{$allowed}` to the user. Fix: set `allowed` (mirror the sibling site planner.rs:317). Regression test: render THAT emitter site's diagnostic and assert no `{$` substring (the T10 guard is per-DiagCode and structurally blind to single-site divergence when siblings define the full set).
- [ ] Full gate; commits per item `fix(diag): ...`.

