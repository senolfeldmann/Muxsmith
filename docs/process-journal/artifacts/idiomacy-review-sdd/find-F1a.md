# Idiomacy review — slice F1a

Scope: `crates/muxsmith-core/src/{planner,matcher,template,discovery}.rs` + `profile/{validate,model,lint,match_expr,load,mod}.rs`. All 10 files read completely. Toolchain ground truth: Rust 1.96.1 / edition 2024 (pinned via `rust-toolchain.toml`); no `clippy.toml`, no `[workspace.lints]`, CI runs `cargo clippy --workspace --all-targets -- -D warnings`.

Verification performed for load-bearing claims:
- clippy `too_many_arguments` threshold empirically confirmed on the 1.96 toolchain (scratch crate): fires at 8+ args ("8/7"), silent at 6 and 7. `get_or_insert_with(BTreeMap::new)` is NOT linted by clippy 1.96 (so not flagged here).
- `$at` grep across all `locales/**/*.ftl`, `src/`, `src-tauri/`, `crates/muxsmith-cli/src`: zero consumers of the `at` diagnostic param (only setters in `load.rs`, a presence assert in `tests/profile_load.rs`, a fixture value in `catalog_completeness.rs`). The `parse-error` message uses only `$detail`. Plan-1 progress log ("parse-error $at dedup") removed `$at` from the catalog but left the emitter param behind.
- Locator with neither `match_to_source` nor `match_pattern` set: recorded decision (task-9 report: "no 'at least one required' rule"), not flagged.

## Findings (most severe first)

### 1. dup — planner.rs:526 — six copies of the placeholder `Assignment` literal in `resolve_file`
Lines 526-533, 563-569, 583-589, 603-609, 646-652, 675-681: the identical `Assignment { rule_index: ri, source: ..., track_id: None, track_kind: None, changes: vec![] }` push repeated six times (four with `primary.path.clone()`, two with `source_path`).
**Replacement:** associated constructor `Assignment::unmatched(rule_index: usize, source: PathBuf) -> Assignment`; each site becomes `assignments.push(Assignment::unmatched(ri, source_path));`.
**lines_cut:** 26, deps_cut: 0.

### 2. idiom — model.rs:183 — `KeepDrop` misses the `#[derive(Default)]` + `#[default]` idiom its sibling `CollisionPolicy` uses in the same file
`CollisionPolicy` (line 166) derives `Default` with `#[default] Error`. `KeepDrop` instead drags a free `fn keep()` (line 294) referenced by three `#[serde(default = "keep")]` attrs (lines 284, 373, 376) plus two manual `impl Default` blocks (`AttachmentsCfg` 321-328, `TagsCfg` 380-387) that a derive would produce verbatim.
**Replacement:** `#[derive(..., Default, ...)] pub enum KeepDrop { #[default] Keep, Drop }`; the three attrs become plain `#[serde(default)]`; delete `fn keep()` and both manual `Default` impls, adding `Default` to the `AttachmentsCfg`/`TagsCfg` derives. `TracksCfg.unmatched` keeps its explicit `default = "drop_policy"` (its default is `Drop`, deliberately asymmetric).
**Tradeoff:** `KeepDrop::default() == Keep` becomes a type-level statement while tracks default to `Drop`; the field-level `drop_policy` fn stays explicit, so no behavior ambiguity.
**lines_cut:** 17, deps_cut: 0.

### 3. dup — validate.rs:280 — `InvalidRegex` compile-check block repeated three times in `validate_expr`
The identical 8-line `if kind == "regex" && let Err(e) = regex::Regex::new(value) { ... }` block appears in the `raw:` branch (282-289), the `codec_kind` branch (301-308), and the fallthrough path (321-328) — forced by the `continue`-per-branch control flow.
**Replacement:** turn the three branches into an `if / else if / else` chain (each pushing only its property-level diagnostic), then run the regex compile check once at the end of the loop body. Diagnostic order is preserved exactly (in every current path the regex diag is pushed last before the next iteration), so insta snapshots are unaffected.
**lines_cut:** 14, deps_cut: 0.

### 4. stdlib — discovery.rs:187 — hand-rolled lowercase-then-compare where `str::eq_ignore_ascii_case` is the stdlib API
`extension_matches` allocates `e.to_ascii_lowercase()` per candidate and compares against pre-lowercased extension lists, which forces both callers to build lowered `Vec<String>` copies (`scan_primaries` 61-65, `resolve_locator` 147-151).
**Replacement:** `exts.iter().any(|x| x.eq_ignore_ascii_case(e))` inside `extension_matches`, passing `&input.extensions` / `&locator.extensions` directly; both pre-lowering collects are deleted. (Same pattern exists once in planner.rs:368 `validate_extension_list` — single line there, lower value, noted for the merge stage.)
**lines_cut:** 10, deps_cut: 0.

### 5. yagni — load.rs:56 — dead `at` diagnostic param, set but consumed nowhere
Both emitter sites (`from_file` line 56 `.with("at", "")`, `parse_error` line 64 `.with("at", err.path().to_string())`) duplicate `config_path` into a param that no Fluent catalog (`$at` absent from every `.ftl`), no frontend, and no CLI code reads. Leftover from the plan-1 "parse-error $at dedup" fix, which removed the placeholder from the message only.
**Replacement:** drop both `.with("at", ...)` calls; drop the `contains_key("at")` assert in `tests/profile_load.rs:92` and the `args.set("at", ...)` fixture line in `muxsmith-cli/tests/catalog_completeness.rs:57`.
**lines_cut:** 4 (2 in slice, 2 in tests), deps_cut: 0.

### 6. yagni — planner.rs:1312 — two of three `#[allow(clippy::too_many_arguments)]` are dead suppressions
Threshold is 7 (fires at 8+, empirically confirmed on the pinned 1.96 toolchain; no clippy.toml overrides). `suggest` has 6 params, `partition_for_overlap` (line 1565) has 7 — neither triggers the lint; only `partition_for_rule` (9 params, line 1481) needs its allow.
**Replacement:** delete the attributes at 1312 and 1565.
**lines_cut:** 2, deps_cut: 0.

### 7. idiom — discovery.rs:76 — three regex passes plus an `expect` invariant where one `captures_iter` pass is the norm
`scan_primaries` runs `re.find_iter(name)` (first match + multiplicity), then `re.captures(name).expect("first match implies captures")` — a third scan whose correctness rests on a cross-call invariant documented in the expect string.
**Replacement:** `let mut it = re.captures_iter(name); let Some(caps) = it.next() else { ... }; if it.next().is_some() { ... }`; whole match is `&caps[0]`, groups come from the same `caps`. One pass, no invariant to document.
**lines_cut:** 2, deps_cut: 0.

### 8. idiom — planner.rs:714 — `matches!` with fully-qualified variant path where siblings use `==` with the imported name
`matches!(profile.tracks.unmatched, crate::profile::model::KeepDrop::Keep)` — `KeepDrop` is already imported (line 19) and derives `PartialEq`; the sibling resolvers do `profile.tags.global == KeepDrop::Keep` (line 987) and `profile.attachments.unmatched == KeepDrop::Keep` (line 1084).
**Replacement:** `let keep_unmatched = profile.tracks.unmatched == KeepDrop::Keep;`
**lines_cut:** 3, deps_cut: 0.

### 9. idiom — planner.rs:1646 — fully-qualified paths for types the file already imports
`std::collections::BTreeSet`/`BTreeMap` written out at 1646-1647, 1825, 1846, 1903, 1952-1953 despite `use std::collections::{BTreeMap, BTreeSet};` (line 7); `crate::identify::Track`/`PropValue` at 1653, 1665-1680, 1717, 1998-2003 despite the existing `use crate::identify::{...}` (line 15).
**Replacement:** add `PropValue, Track` to the identify import and use the bare names throughout; several wrapped signatures collapse to one line.
**lines_cut:** 2, deps_cut: 0.

### 10. stdlib — planner.rs:1971 — `rule_index_of` hand-rolls `find` + index arithmetic where `str::split_once` is the normal parse
**Replacement:**
```rust
fn rule_index_of(config_path: &str) -> Option<usize> {
    config_path.split_once("tracks[")?.1.split_once(']')?.0.parse().ok()
}
```
**lines_cut:** 1, deps_cut: 0.

### 11. dup — planner.rs:886 — `render_output` and `resolve_title` duplicate the Ctx build the comment demands stay in lockstep
Both build `primary.identifier.to_ctx()` + conditional `source_stem` (886-890 and 971-974); `resolve_title`'s doc comment explicitly warns the two "must stay in lockstep".
**Replacement:** `fn render_ctx(primary: &PrimaryFile) -> Ctx` — the lockstep invariant enforced by construction instead of by comment.
**lines_cut:** 2, deps_cut: 0.

### 12. idiom — template.rs:92 — `Vec<char>` collect + index arithmetic parser; `char_indices`/`Peekable` is the ecosystem norm
`let chars: Vec<char> = text.chars().collect();` plus manual `i` bookkeeping is the classic plausible-but-unidiomatic Rust parser shape; hand parsers normally iterate `text.char_indices()` (byte offsets, free slicing) or a `Peekable<Chars>`. The current shape is also what forces the "pos is a CHAR offset, do not byte-slice" contract on `TemplateError` (a deliberate, journal-recorded contract — a `Peekable<Chars>` rewrite with a char counter preserves it; a `char_indices` rewrite would switch `pos` to byte offsets and change diagnostic values).
**Replacement:** `Peekable<Chars>` with lookahead-1 via `peek()` and a consume-until-`}` scan, keeping char-offset `pos`.
**lines_cut:** 1, deps_cut: 0.

### 13. idiom — planner.rs:1965 — composite map key via `format!("{}|{}|{}")` where a tuple key is the Rust norm
`diag_signature` joins `(code, config_path, file)` into a `|`-separated `String` for the `BTreeMap` key; the idiomatic composite key is a tuple `(String, String, String)` (or `(&str, ...)`) — no separator convention, no formatting pass.
**Replacement:** `BTreeMap<(String, String, String), usize>`; `base_sig.get(&sig)` unchanged in shape.
**lines_cut:** 0, deps_cut: 0.

## Routed (not findings)

- **planner.rs:1965 (correctness, theoretical):** the `|`-joined signature key can collide for user-controlled file paths containing `|` (path is the last segment, config_path the middle one, so `("output", "x|y")` vs `("output|x", "y")` collide only if a config_path contained `|` — none currently do). A collision could mask or fake a regression in suggestion acceptance. Closed for free by finding 13's tuple key.

## Considered and rejected (with reason)

- `matcher.rs` per-call regex compilation: recorded non-finding (tracked v1.x entry).
- `discovery::walk_files` vs `walkdir`: walkdir is not an already-present dependency; the walker's exact symlink semantics (include symlink-to-file, never recurse symlink-to-dir) are custom, tested, and 34 lines. Under the repo's dependencies-are-earned doctrine, hand-rolling is the justified choice.
- Locator with neither `match_to_source` nor `match_pattern`: recorded spec decision (task-9 report), and match-everything locators are a legitimate `add` use (attach a font directory).
- `Option<bool>` for `match_to_source`: documented deliberate (reject explicit `false`, distinguish absent).
- `get_or_insert_with(BTreeMap::new)` → `get_or_insert_default()`: available since 1.83 but clippy 1.96 does not lint it (verified); too weak to flag.
- `strip_mkv_suffix` manual `is_char_boundary`: `str::get` would do the boundary check natively, but the current code is correct, commented, and equivalent; below the finding bar.
- `scalar_display` as free fn vs `impl Display for Scalar`: single call site, local helper is defensibly better-scoped; a Display impl could mislead in YAML-quoting contexts.
- `SeedMode` enum, `Matchable` trait, `plan_core`/`plan_batch` split: all have ≥2 real users; earned abstractions.
- `overlap_conflicts` parsing claimants back from the rendered `$rules` param string: a design smell, but restructuring `Diagnostic` to carry structured params is a behavior/architecture change outside this sweep's dimensions.
- MUXSMITH_RUNS_ROOT, fake-mkvmerge copies, RECENT_PROFILES_CAP, version pins: recorded non-findings per brief.
