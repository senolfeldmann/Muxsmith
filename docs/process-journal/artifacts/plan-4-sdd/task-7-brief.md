### Task 7: Richer gated live test (attachment + changes round trip) [WAVE 1 - independent]

**Files:**
- Test: `crates/muxsmith-core/tests/command_integration.rs` (append one gated test)

- [ ] **Step 1:** Following the file's existing gated pattern: build a primary MKV from an SRT plus `--attach-file` (a small .txt as attachment; confirm mkvmerge accepts a text attachment with an explicit `--attachment-mime-type text/plain` - probe the real binary first per SI-3); build a `Plan` (or drive `plan_batch` with a profile) that selects the subtitle track with `changes: { track_name: Renamed, default_track: true }` and keeps the attachment; run `command(&plan)` through real mkvmerge; `-J` the output and assert: track_name == "Renamed", default_track true, attachment present with the original file_name. This converts Plan 3's one-off manual v100 validation into a standing guard (D18).
- [ ] **Step 2:** Full gate green (test runs live locally; self-skips without mkvmerge).
- [ ] **Step 3: Commit** - `test(command): gated live guard for attachment + changes round trip (D18)`

---

