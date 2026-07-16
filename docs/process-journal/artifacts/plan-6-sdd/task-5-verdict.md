# Verdict (extracted from the reviewer transcript at write-time)

Exactly 21 exported types, matching the enumeration one-for-one, no residue and no gap. All spec-compliance risks are now verified. Writing the final review.

---

### Spec Compliance

- ✅ `ts-rs = { version = "12.0.1", optional = true }` and `ts = ["dep:ts-rs"]` feature — `crates/muxsmith-core/Cargo.toml` diff, exact match, registry-verified (implementer's report shows the corrected `curl` retry with a User-Agent header after the bare `curl -s` returned no `crate` key; both `max_stable_version` and `newest_version` = 12.0.1).
- ✅ `.cargo/config.toml` — byte-for-byte the brief's step-3 block (`TS_RS_EXPORT_DIR` relative to `src/bindings`, `TS_RS_LARGE_INT = "number"`). No prior `.cargo/config.toml` existed; no conflict.
- ✅ 21/21 types derived, no residue. Counted `cfg_attr(feature = "ts"` occurrences directly against the diff: 18 in `model.rs` (12 structs + 6 enums), 2 in `match_expr.rs` (`Scalar`, `MatchExpr`), 1 in `planner.rs` (`StructuredEdit`) = 21. Cross-checked against the committed `src/bindings/profile.ts`: `grep -c '^export type'` = 21, and the 21 names match the enumeration exactly (`AttachmentRule, AttachmentsCfg, ChaptersCfg, CollisionPolicy, ExternalBlock, FilenameCfg, Input, KeepDrop, Locator, MatchExpr, Meta, OutputCfg, Profile, Scalar, SourceCfg, StructuredEdit, TagsCfg, TemplateBlock, TitleCfg, TrackRule, TracksCfg`).
- ✅ `keywords.ts` byte-matches Task 3's `KEYWORDS` consts. Grepped `model.rs` directly: `FilenameCfg::KEYWORDS = &["keep"]`, `SourceCfg::KEYWORDS = &["primary"]`, `ChaptersCfg::KEYWORDS = &["keep", "drop"]`, `TitleCfg::KEYWORDS = &["keep", "clear"]`. Committed `keywords.ts` reproduces all four as `as const` arrays, exact values, exact order.
- ✅ `Scalar` emits `boolean | number | number | string` (diff line 1017); `grep -c bigint src/bindings/profile.ts` = 0 (confirmed directly, and the pattern was verified to fire on a synthetic hit before trusting the empty result). `StructuredEdit`'s four variants (diff lines 1036-1060) carry `value: Scalar` on `add_exact`/`add_not_exact` and `value: string` on the two substring variants, matching D49 `:1214-1218` exactly.
- ✅ Emitter matches D44's verbatim block in substance — same variable names, same tuple list, same format strings, same destination write. Differences are pure rustfmt reformatting (multi-line `.map()` chain, dropped inline alignment whitespace, one inline comment replaced by an equivalent module-level `//!` sentence) — cosmetic, not a divergence from the specified logic, and consistent with the house's `testing-si3-run-binary` precedent of adapting brief code to local idiom.
- ✅ CI drift step: Linux-leg only (`if: runner.os == 'Linux'`), identical shape to the pre-existing `check:i18n` step immediately below it in `ci.yml`. Hole comment present, accurate, and matches the brief's own wording closely (states the untracked-file blind spot, names Task 9/wave 3 as the closer, explicitly notes the two-wave gap, and states why `git status --porcelain` was rejected).
- ✅ No bare `use ts_rs::TS;` — exactly three occurrences (`model.rs`, `match_expr.rs`, `planner.rs`), all `#[cfg(feature = "ts")]`-gated.
- ✅ `Cargo.lock` staged with exactly three new packages (`ts-rs`, `ts-rs-macros`, `termcolor`), matching the report's claim; `deny.toml`'s allow list (`MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-3-Clause, BSL-1.0, MPL-2.0, Unicode-3.0, Zlib`) needed no change — checked directly, `Unlicense` is absent from the allow list, but `termcolor`'s license is an OR-expression (`Unlicense OR MIT`) and cargo-deny satisfies an OR-expression on any allowed member, so the existing `MIT` entry covers it without modification. Claim holds.
- ✅ No scope creep: `git diff --stat 5349ed0 1f02004` lists exactly the ten files the brief's step 10 names, one commit, no stray files.
- ✅ Unsigned commit confirmed directly (`git log --format='%G?'` = `N`).
- ✅ ASCII typography: grepped the full commit diff for em/en-dash, curly quotes, ellipsis, NBSP — zero hits; pattern verified to fire against a synthetic em-dash first, so the empty result is trustworthy.
- ⚠️ Rustdoc on new public items: no new public API surface beyond derive attributes on existing (already-documented) types; the one new function (`emit_keywords_ts`) is a private `#[test]` fn, conventionally undocumented, covered by the file's `//!` module doc. Nothing to flag — the brief's rustdoc requirement has no undocumented target to bind to here.

### Adjudications

**Q1(a):** Confirmed directly against the committed worktree, not the report. `src/bindings/keywords.ts` is exactly 5 lines (header + 4 `export const` lines), `src/bindings/profile.ts` is exactly 405 lines ending cleanly on `rules: Array<TrackRule>, };`. `grep -n "drift"` against both files returns nothing (exit 1), and that grep pattern was verified live against a synthetic `// drift` string before trusting the negative result. `git status --porcelain` in the worktree is empty, meaning the working tree matches HEAD exactly — no leftover, uncommitted stray text anywhere. The claim holds: no residue from the failed first attempt reached the committed artifacts.

**Q1(b):** Same root cause, distinct instance — record separately, do not fold into the step-8 comment. Both trace to the identical git mechanic D44 itself measured (`git diff --exit-code` is blind to a new untracked path, exit 0 regardless of content), but they manifest at different sites with different consequences and different closure status:

- Step 8's hole is an *ongoing CI-gate coverage gap*: after this commit, `src/bindings/` is tracked, so the hole only matters for a hypothetical future re-generation that briefly loses tracking; it is explicitly accepted and stated to close in wave 3 via Task 9's TypeScript build failing on a missing import. It is documented permanently in a code comment for future readers of `ci.yml`.
- Step 9's hole is a *one-time task-execution hazard in the brief's own step ordering*: generation happens in step 6, staging happens only in step 10, and step 9 (whose entire purpose is to satisfy `proc-verification-step-must-be-falsifiable` by proving the CI gate actually fires) sits between them. Anyone following the brief's steps in written order hits this — the drift-append run silently returned exit 0 against an untracked path where exit 1 was expected, not because the check was vacuously green but because it produced the *wrong* result relative to the brief's own stated expectation. Nothing downstream mitigates this within Task 5's own execution; Task 9/wave 3 has no bearing on whether step 9 of Task 5 proves anything during this task's run. The implementer caught it (the expected-vs-actual exit code mismatch was visible, not silent), staged before re-running, and disclosed the whole episode in the report's Concerns section — correct handling per `testing-si3-run-binary`'s standing precedent (brief-vs-tree divergence adapted and surfaced, not silently resolved or transcribed).

Ruling: two occurrences of one underlying git behavior, at two different sites with two different remedies; the ledger should carry step 9's manifestation as its own entry (a brief-authoring pattern: a git-diff-based falsifiability self-proof needs its target already staged, stated as an explicit precondition or reordered ahead of the proof step) rather than treat it as already covered by the step-8 record.

### Strengths

- Isolation claim proven both directions: `cargo tree` = 0 without `ts`, 0 again after the derives exist, plus a positive control (`--features ts` → 2) showing the absence-check can actually fire — textbook `proc-verification-step-must-be-falsifiable` compliance, unprompted.
- D45's citation range (`:768-782`) was checked against the tree rather than trusted, found to state counts rather than enumerate names, and the implementer independently derived and listed all 13 struct / 7 enum names from source — caught a citation gap the brief itself didn't flag.
- Step 9's failure was caught, diagnosed correctly (untracked-path git-diff blind spot, plus a silently-failing `git checkout` on a nonexistent pathspec), fixed by reordering rather than patched around, and the regenerated file was confirmed byte-identical to the first generation before re-proving.
- The `ts` feature's Cargo.toml doc comment and the CI hole comment both carry the actual rationale (xtask feature-unification leak, two-wave gap) rather than a restated summary — comment quality matches the design doc's own precision.
- registry verification handled correctly, including recovering from crates.io's User-Agent requirement rather than accepting a malformed empty response.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. No plan-mandated defect reached the deliverable — the one brief defect found (step 9's ordering) was caught and routed around within the task's own execution rather than implemented literally.

#### Minor (Nice to Have)
None beyond what's already captured in the HARVEST candidate below.

### HARVEST

- **Falsifiability self-proof needs its staging precondition stated.** Task 5's step 9 (`printf ... >> src/bindings/keywords.ts; git diff --exit-code ...`) is only meaningful once `src/bindings/` is tracked, but the brief's own ordering (generate → step 6, prove-it-fires → step 9, stage+commit → step 10) puts the proof before the staging it depends on. Generalizes beyond this task: any brief step whose falsifiability proof is a `git diff --exit-code` against a path that is freshly generated in the same task needs either (a) an explicit "ensure the target is staged first" precondition in the step text, or (b) reordering so staging precedes the proof. Candidate addition to `proc-verification-step-must-be-falsifiable`'s guidance, or a new narrower pattern scoped to git-diff-based proofs specifically.
- **D45-citation-states-a-count-not-a-list pattern recurs.** This is now at least two instances in this plan (D45's own self-correction on the "three sets enumerated" ADR section, and now Task 5 catching `:768-782` stating "13 structs ... 7 enums" derivationally rather than listing names) — worth watching for whether a design doc that cites a line range as authority for a *set* should be required to actually enumerate at that range, not just state its cardinality.

### Assessment
**Task quality:** Approved
**Reasoning:** Every spec-compliance item verified directly against the committed worktree and diff (not the report) checks out exactly: 21/21 types, byte-exact keyword arrays, correct `Scalar`/`StructuredEdit` wire shapes, correct CI gating shape, no scope creep, unsigned commit, clean typography, no stray drift residue. The one process wrinkle (step 9's ordering hazard) was self-caught, correctly diagnosed, fixed, and fully disclosed rather than papered over — exactly the standing house discipline this codebase enforces elsewhere.
