### Task 12: rustdoc as ninth gate part (#18b; after T2 - shares ci.yml)

**Files:**
- Modify: `crates/muxsmith-core/src/executor/queue.rs:73` (dead `[worker_count]` intra-doc link)
- Modify: `.github/workflows/ci.yml` (new step), `BUILDING.md` (gate description gains part nine)

- [ ] Step 1: Fix the link; run `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` locally - expect clean after the fix; if further warnings surface, fix them in this task (they are exactly what the gate is for).
- [ ] Step 2: Add the CI step (all legs) + BUILDING.md gate list update.
- [ ] Step 3: Commit `ci: cargo doc -D warnings as ninth gate part (#18b)`.

