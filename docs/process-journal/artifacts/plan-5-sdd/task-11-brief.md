### Task 11: Jobs view - live queue, cancel, history, log export (spec 8.2 view 3, D30)

**Files:**
- Create: `src/views/JobsView.vue`, `src/components/{JobRow,LiveLog,RunHistory}.vue`, `locales/en/gui-jobs.ftl`
- Test: T12 smoke; `pnpm lint`/`build` per commit

**Interfaces:**
- Consumes: T8 (`start_run`, `cancel_run`, `cancel_job`, `list_runs`, `get_job_log`, the `muxsmith://job-event` + `muxsmith://run-finished` events), T9 conventions.

- [ ] **Step 1:** On startRun: call `start_run`, render one `JobRow` per job (`data-testid="job-row"` + `data-index`): output filename, state chip, `role="progressbar"` with `aria-valuenow` (or native `<progress>`), per-row cancel button; batch header: overall progress (finished/total), cancel-batch button. Event subscription drives all state; warning count badges from `warning` events.
- [ ] **Step 2:** `LiveLog`: `role="log"` region fed by `output` events (auto-scroll unless the user scrolled up; DOM-capped at 5000 lines - the full log is in the file); per-job filter select.
- [ ] **Step 3:** `run-finished`: summary line in a polite `aria-live` region (ok/warning/failed/cancelled counts); rows finalize from the document.
- [ ] **Step 4:** `RunHistory`: `list_runs` (newest first) -> select run -> jobs from its `summary.json`, per-job log via `get_job_log`; copy-log button + save-as (dialog plugin) - the D30 gap closure (mkvtoolnix-gui parity: open finished job log as text).
- [ ] **Step 5:** lint + build green. **Commit** `feat(gui): jobs view - live queue, per-job/batch cancel, history + log export`

---

