### Task 4: `--on-collision` flag on dry-run (and the CLI mapping type) [WAVE 1 - independent]

**Files:**
- Modify: `crates/muxsmith-cli/src/cli.rs` (add `CollisionArg` + flag on `DryRun`), `crates/muxsmith-cli/src/main.rs` (pass-through), `crates/muxsmith-cli/src/commands/dry_run.rs` (signature + `RunInputs.on_collision`)
- Test: `crates/muxsmith-cli/tests/dry_run_cli.rs` (flag reaches the planner)

**Interfaces:**
- Produces: `cli::CollisionArg` (clap ValueEnum: `error|skip|overwrite`) with `pub fn policy(self) -> CollisionPolicy`; `dry_run::run` gains `on_collision: Option<CollisionPolicy>` before `json`. Task 8 reuses `CollisionArg` for `run`.

- [ ] **Step 1: Failing test** - in `dry_run_cli.rs`: a profile whose output collides with a pre-existing file; default invocation exits 2 (`error`); with `--on-collision skip` exits 1 (warning) and the JSON carries the `output-collision` diagnostic at warning severity. Follow the file's existing invocation helper.
- [ ] **Step 2: RED** (unknown flag). **Step 3:** implement:

```rust
/// CLI value for the collision-policy override (spec 4.2 run input). Maps
/// to core's CollisionPolicy; a CLI-local type so core stays clap-free.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CollisionArg {
    /// Refuse the colliding output (default policy).
    Error,
    /// Skip the colliding output with a warning.
    Skip,
    /// Replace the pre-existing file.
    Overwrite,
}
impl CollisionArg {
    /// The core policy this argument selects.
    pub fn policy(self) -> muxsmith_core::profile::model::CollisionPolicy { /* 1:1 match */ }
}
```

DryRun variant gains `#[arg(long, value_enum)] on_collision: Option<CollisionArg>`; main.rs passes `on_collision.map(CollisionArg::policy)`; dry_run.rs threads it into `RunInputs { on_collision, .. }` (replacing the hardcoded `None` at dry_run.rs:78).
- [ ] **Step 4: GREEN + full gate. Step 5: Commit** - `feat(cli): --on-collision override for dry-run (D15)`

---

