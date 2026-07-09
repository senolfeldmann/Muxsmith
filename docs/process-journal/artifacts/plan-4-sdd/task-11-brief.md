### Task 11: Gated end-to-end `run` test

**Files:**
- Test: `crates/muxsmith-cli/tests/run_live.rs` (new)

- [ ] **Step 1:** Gated (locate-or-skip): build 2 tiny source MKVs (SRT fixture pattern) in a temp source dir, a minimal profile, invoke the actual `muxsmith` binary (`env!("CARGO_BIN_EXE_muxsmith")`) with `run --source ... --output ...`; assert exit 0, both outputs exist and `-J`-identify as MKVs, and stdout contains the summary line. Add a second case: rerun with `--on-collision skip` exits 1 and leaves outputs untouched (mtime unchanged) - the rerun workflow guard (D14/D17).
- [ ] **Step 2:** Full gate. **Step 3: Commit** - `test(cli): gated end-to-end run over real mkvmerge (D15)`

---

## Self-review (controller, after all tasks)

- **Memo coverage:** D13 -> T1-T3 (+ parent-dir in T2); D14 -> T3 (+ rerun guard T11); D15 -> T4, T8, T9, T11; D16 -> T10; D17 -> T2 delete-partial (divergence already recorded); D18 -> T5, T6, T7. Deferred by decision: NDJSON `--json-events` (v1.x); persisted job logs (Plan 5); `--fail-fast=now` (v1.x); zero-track empty-plan warning (cleanup pass).
- **Waves:** wave 1 = T1+T4+T5+T6+T7 in parallel worktrees per SI-1; merges sequential with gate re-runs.
- **Close-out:** whole-branch review on the most capable model, SI-2 journal entry, salvage `.superpowers/sdd/` Plan-4 artifacts, HANDOFF refresh, push (verify T6's CI effect in the Actions log).
