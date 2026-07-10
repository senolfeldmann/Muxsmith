### Task 2: Hoist report JSON assembly into core (`report::json`)

**Files:**
- Create: `crates/muxsmith-core/src/report/json.rs` (turn `report.rs` into `report/mod.rs` if needed)
- Modify: `crates/muxsmith-cli/src/commands/dry_run.rs` (`batch_json`, `config_only_json` move out), `crates/muxsmith-cli/src/commands/run.rs` (`run_json_document` moves out)
- Test: existing CLI snapshot/JSON tests are the harness; add `crates/muxsmith-core/tests/report_json.rs` for direct shape assertions

**Interfaces:**
- Produces: `report::json::{config_only_document(..), batch_document(..), run_document(..)}` with signatures lifted 1:1 from the current CLI functions (same inputs, same `serde_json::Value` output, byte-identical documents). `run_document` keeps injecting per-job `"index"` and `"output"` into each serialized `JobOutcome`. Consumed by T6 (summary.json), T7/T8 (IPC returns).
- Rationale (spec 7): "CLI and GUI render the same diagnostic and report structures; neither owns logic."

- [ ] **Step 1:** Move the three functions to core verbatim (public, documented); CLI call sites delegate. Add a rustdoc note on `run_document`: `jobs[].index` indexes the QUEUE (specs slice), not the source-file list (HANDOFF backlog item).
- [ ] **Step 2:** `cargo test --workspace` - the existing dry-run/run JSON tests prove byte-identical output. Add one direct core test asserting `run_document` field presence (`jobs[].{index,output,state}`, `summary.{ok,warning,failed,cancelled}`).
- [ ] **Step 3: Commit** `refactor(core): hoist batch/config/run JSON documents into report::json`

---

