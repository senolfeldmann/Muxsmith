# Idiomacy review — slice F2b (muxsmith-core integration tests, minus planner_resolution / suggestions / prop_planner)

Scope read completely: `command.rs`, `command_integration.rs`, `joblog.rs`, `prop_matcher.rs`, `validate_semantics.rs`, `prop_language.rs`, `mkvmerge_runtime.rs`, `validate_structure.rs`, `profile_load.rs`, `executor_events.rs`, `executor_no_hang_live.rs`, `report_json.rs`, `validate_hardening.rs`, `executor_live.rs`, `identify_live.rs`, `support/mod.rs` under `/home/senol/Git/Muxsmith/crates/muxsmith-core/tests/`.

Toolchain ground truth used: Rust 1.96.1 / edition 2024, proptest =1.11.0 (signature checked in the vendored registry source), insta =1.48.0 (not used by any file in this slice). Version-dependent claims verified empirically on the pinned toolchain (compile tests) or against the pinned crate source, not from memory. Notably, `Duration::from_days` was a candidate stdlib finding and was **rejected** after `rustc 1.96.1` confirmed it is still unstable (`duration_constructors`, E0658).

## Findings

### 1. idiom — `vec![…].into_iter().map(String::from).collect::<Vec<_>>()` golden-argv boilerplate (command.rs, 10 sites)

- File: `crates/muxsmith-core/tests/command.rs`, first site line 48 (also 398, 467, 565, 605, 635, 664, 702, 749, 798).
- Every golden test compares `command(&plan)` (a `Vec<String>`) against a `vec![&str…]` run through `.into_iter().map(String::from).collect::<Vec<_>>()`. std already provides `impl PartialEq<[B; N]> for Vec<A> where A: PartialEq<B>`, and `String: PartialEq<&str>`, so the idiomatic form is a direct array comparison:
  ```rust
  assert_eq!(
      muxsmith_core::command::command(&plan),
      ["--output", "/out/e.mkv", "--title", "", /* … */]
  );
  ```
  Verified to compile and pass on 1.96.1. Removes 3 lines of conversion tail per site (~30 lines net) and the `vec![` allocation of the expectation.
- lines_cut: 30, deps_cut: 0.

### 2. idiom — same conversion dance once in command_integration.rs

- File: `crates/muxsmith-core/tests/command_integration.rs`, line 121 (`let expected: Vec<String> = […].into_iter().map(String::from).collect();` closing at 177-180, consumed by `assert_eq!(command(plan), expected)` at 182).
- Same replacement as finding 1. The array mixes `&'static str` literals with `&String` bindings (`&output_disp`, `&primary_disp`, `&donor_disp`); LUB/deref coercion unifies them to `[&str; N]` — verified to compile on 1.96.1. The intermediate `expected` binding and the map/collect tail go away.
- lines_cut: 4, deps_cut: 0.

### 3. idiom — needless `.to_vec()` on `&'static` slices fed to proptest `select` (prop_matcher.rs, 14 sites)

- File: `crates/muxsmith-core/tests/prop_matcher.rs`, first site line 130 (`select(STRING_POOL.to_vec())`), also 140, 142, 143, 145, 147, 148, 149 (×2), 162, 164, 166, 172, 173.
- `proptest::sample::select` in the pinned 1.11.0 is `pub fn select<T: Clone + Debug + 'static>(values: impl Into<Cow<'static, [T]>>)` (checked in the vendored source, `src/sample.rs:156`). A `&'static [T]` converts into `Cow::Borrowed` directly, so `select(STRING_PROPS)` is the ecosystem-normal call; `.to_vec()` allocates a fresh `Vec` per strategy construction for nothing and reads as if the API demanded ownership. Pure replacement, identical downstream types (`T = &'static str`).
- lines_cut: 0, deps_cut: 0.

### 4. stdlib — hand-rolled one-entry maps instead of `BTreeMap::from` (prop_matcher.rs)

- File: `crates/muxsmith-core/tests/prop_matcher.rs`, lines 83-107 (`exact_one`, `substring_one`, `regex_one`).
- Each constructor does `let mut map = BTreeMap::new(); map.insert(prop.to_string(), val);` for a single entry. The stdlib literal-map constructor `BTreeMap::from([(prop.to_string(), val)])` (stable since 1.56) is the human-normal form and inlines into the struct literal:
  ```rust
  MatchExpr { exact: Some(BTreeMap::from([(prop.to_string(), val)])), ..Default::default() }
  ```
- lines_cut: 6, deps_cut: 0.

### 5. stdlib — manual insert loop instead of `collect()` into `BTreeMap` (prop_matcher.rs `arb_track`)

- File: `crates/muxsmith-core/tests/prop_matcher.rs`, lines 179-182.
- `let mut properties = BTreeMap::new(); for (k, v) in entries { properties.insert(k, v); }` is `FromIterator` by hand. Replacement inside the struct literal: `properties: entries.into_iter().collect(),`.
- lines_cut: 4, deps_cut: 0.

### 6. stdlib — same one-entry-map hand-roll in prop_language.rs (2 sites)

- File: `crates/muxsmith-core/tests/prop_language.rs`, lines 128-130 (`track_with_language`) and 139-141 (`language_expr`).
- Same replacement as finding 4: `BTreeMap::from([("language".to_string(), PropValue::Str(value.to_string()))])` resp. `Scalar::Str(...)`.
- lines_cut: 4, deps_cut: 0.

### 7. idiom — fully-qualified paths fighting the file's own imports (profile_load.rs)

- File: `crates/muxsmith-core/tests/profile_load.rs`, lines 131-137 and 151-158.
- The last two tests spell out `muxsmith_core::profile::load::from_str(yaml, muxsmith_core::profile::load::Format::Yaml)` and `muxsmith_core::profile::model::KeepDrop::Drop/Keep` although line 1 imports `{Format, from_str}` and line 2-4 import `KeepDrop`. Every other test in the file uses the imports. Replacement: `from_str(yaml, Format::Yaml)` / `KeepDrop::Drop` / `KeepDrop::Keep`; the three multi-line expressions each collapse to one line.
- lines_cut: 8, deps_cut: 0.

### 8. yagni — dead `_section` tuple element in test loop (validate_structure.rs)

- File: `crates/muxsmith-core/tests/validate_structure.rs`, line 123.
- `for (snippet, _section) in [("chapters: discard\n", "chapters"), ("title: wipe\n", "title")]` — the second element is never read (the assertion message already interpolates `snippet`). Replacement: iterate the snippets directly, `for snippet in ["chapters: discard\n", "title: wipe\n"]`.
- lines_cut: 0, deps_cut: 0.

### 9. idiom — `#[cfg(unix)]` duplicated on a test inside a `#![cfg(unix)]` file (executor_no_hang_live.rs)

- File: `crates/muxsmith-core/tests/executor_no_hang_live.rs`, line 31.
- The file opens with the inner attribute `#![cfg(unix)]` (line 20), which already gates the entire test binary; the per-test `#[cfg(unix)]` on its only test is redundant. Drop the per-test attribute.
- lines_cut: 1, deps_cut: 0.

## Routed (not findings; for the correctness/robustness lane)

- `crates/muxsmith-core/tests/identify_live.rs:20` — `make_sample` spawns `Command::new("mkvmerge")` by bare PATH name instead of the located `m.path()` that gates the test (and that the sibling live suites use). Behaviorally identical today because `Mkvmerge::locate()` itself resolves via bare `"mkvmerge"` on PATH (`runtime.rs:94-104`), but the fixture build would silently diverge from the handle under test if `locate()` ever gains a non-PATH source. Low priority, consistency hardening.

## Explicitly considered and rejected

- `Duration::from_secs(14 * 24 * 60 * 60)` → `Duration::from_days(14)` (joblog.rs): rejected — `from_days` is still unstable on the pinned 1.96.1 (verified by compile, E0658).
- `#[allow(dead_code)]` on `support/mod.rs` items: the standard, correct pattern for a shared `tests/` subdirectory module compiled per test binary; not a finding.
- The self-skip `let Some(m) = mkvmerge() else { … return }` pattern: deliberate, documented convention across the live suites; libtest has no runtime skip; not a finding.
- Hand-written golden argv vectors instead of insta snapshots: explicit expected values are the right form for a locked argv contract; not unidiomatic.
- `mkvmerge()` helper / fake-mkvmerge script copies across test files: cross-file duplication, out of scope for this sweep, and the fake-script copies are a recorded non-finding (tracked trigger at >3 copies).
- Verbose full-field `Plan` struct literals in command.rs: `Plan` and friends deliberately do not derive `Default` (checked `planner.rs`); exhaustive literals also make golden tests break loudly when the struct gains a field. Not a finding.
- `regex recompilation`, version-pin style, `MUXSMITH_RUNS_ROOT`: recorded non-findings per the review brief.

## Slice verdict

Not clean: 9 findings (5 idiom, 3 stdlib, 1 yagni), all small, mechanical, behavior-preserving; ~57 net lines removable, no dependency changes. Test structure, proptest usage (strategies, `prop_assert_*`, `prop_oneof!`, recursive strategies), fixture patterns, and the live-test gating conventions are otherwise sound and idiomatic for the pinned toolchain.
