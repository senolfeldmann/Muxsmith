### Task 3: Gated live test - `keep` track-order against real mkvmerge

Confirms the D20 assumption (memo open mechanic #3, per SI-3): under `keep`, tracks kept but absent from `--track-order` land in source-relative order after the ordered ones. Locks it as a standing guard. Skips when mkvmerge is absent (CI parity with the other gated tests).

**Files:**
- Test: `crates/muxsmith-core/tests/command_integration.rs` (add a gated case alongside the existing live round-trip)

**Interfaces:**
- Consumes: the gating helper already used in `command_integration.rs` to locate mkvmerge and self-skip; `command(&Plan)`.

- [ ] **Step 1: Write the gated test**

Add to `crates/muxsmith-core/tests/command_integration.rs`, following the file's existing gated-test pattern (same mkvmerge-locate/skip guard, same tiny-fixture generation via mkvmerge): build a 2+-track source, a profile-equivalent `Plan` with `keep_unmatched: true` that orders only the second track, run the generated argv through mkvmerge, then `mkvmerge -J` the output and assert (a) all source tracks are present (kept), and (b) the ordered track precedes the unlisted ones with the unlisted ones in source order. Emit `eprintln!` + early `return` when mkvmerge is not found, exactly like the sibling tests.

- [ ] **Step 2: Run it (skips if mkvmerge absent, passes if present)**

Run: `cargo test -p muxsmith-core --test command_integration keep -- --nocapture`
Expected: PASS, or a self-skip line if mkvmerge is not installed.

- [ ] **Step 3: If the observed order contradicts the assumption**

If mkvmerge does NOT append unlisted-kept tracks after ordered ones (e.g. it interleaves or errors), STOP and report the observed behavior: the fix is to have the planner emit a full `--track-order` including the kept-unmatched primary track ids (in source order) rather than only matched ids. Do not silently paper over a mismatch; the assumption is explicit in the memo and a contradiction is a design signal.

- [ ] **Step 4: Gate and commit**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`

```bash
git add -A
git -c commit.gpgsign=false commit -m "$(cat <<'EOF'
test(command): gated live mkvmerge guard for keep track-order (D20)

Confirms unmatched-kept primary tracks land after the ordered ones in
source order (SI-3: verified against mkvmerge v100, not from memory).

Co-Authored-By: <session model> <noreply@anthropic.com>
EOF
)"
```

---

