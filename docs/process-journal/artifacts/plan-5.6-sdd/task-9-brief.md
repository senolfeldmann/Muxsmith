### Task 9: Skip-marker shared const (Wave 2, seed T2-m1)

**Files:**
- Modify: `crates/muxsmith-core/src/lib.rs` (or the crate root's most fitting module), the ~21 eprintln sites across 8 files in 3 crates (re-grep the exact set: `grep -rn "mkvmerge not found; skipping"`), `.github/workflows/ci.yml:105` (comment)

- [ ] Add `#[doc(hidden)] pub const MKVMERGE_SKIP_MARKER: &str = "mkvmerge not found; skipping";` to muxsmith-core (cli and src-tauri already depend on it; per-crate tests/support cannot span 3 crates). Replace every eprintln site with `eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);`. Site count grew 19 -> 21 between defer and review; re-grep and cover ALL current sites, report the count.
- [ ] ci.yml:105: comment stating the grep literal must match that const (the CI no-silent-skip gate depends on it).
- [ ] Full gate; commit `refactor: single source for the mkvmerge skip marker (T2-m1)`.

