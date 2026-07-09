### Task 9: `run --json` final document

**Files:**
- Modify: `commands/run.rs`
- Test: `run_cli.rs` + a unit test on the document builder

**Interfaces:**
- Produces (D15): dry-run's document (`config_diagnostics`, `files`, `batch_diagnostics`, `suggestions` - reuse `batch_json`) EXTENDED with:

```json
"jobs": [ { "index": 0, "output": "...", "state": "ok", "exit_code": 0, "warnings": [], "errors": [], "duration_ms": 12400 } ],
"summary": { "ok": 2, "warning": 1, "failed": 0, "cancelled": 0 }
```

- [ ] **Step 1:** Failing unit test on the builder (given outcomes -> expected JSON, serde_json::Value equality). **Step 2:** implement: in json mode suppress ALL human lines (drain events silently; JobOutcome is already `Serialize`). NDJSON stays deferred (v1.x, D15). **Step 3:** GREEN + gate. **Step 4: Commit** - `feat(cli): run --json final document with per-job results (D15)`

---

