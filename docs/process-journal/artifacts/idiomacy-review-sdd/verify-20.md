# Verify-20: yagni — no_clobber_batch() single-caller delegate wrapper

**Finding:** crates/muxsmith-core/tests/suggestions.rs:194, tag `yagni`, slice F2a.

**Verdict: CONFIRMED**

## Checks

### (a) Cited code says what the finding claims — PASS

- Line 194-202: `fn no_clobber_batch() -> (Batch, tempfile::TempDir)` is a pure
  delegate: its entire body is one call,
  `plan_multi(P_NO_CLOBBER, &[("Show.S01E01.mkv", AMBIGUOUS_FOO), ("Show.S01E02.mkv", GUARDED_FOO)])`.
  No added logic, no default, no type adaptation.
- Single caller: `grep -rn no_clobber_batch` over the repo yields exactly the
  definition (line 194) and one call site (line 233, inside
  `with_rule_match_never_widens_an_existing_substring_constraint`).
- Duplication: lines 266-271 of the same test restate the byte-identical
  two-entry array `[("Show.S01E01.mkv", AMBIGUOUS_FOO), ("Show.S01E02.mkv", GUARDED_FOO)]`
  inline for the re-plan (`plan_multi(&edited, &[...])`). The wrapper therefore
  fails to abstract the very repetition it exists for — the second site cannot
  use it because the profile differs.

### (b) Replacement is current idiom — PASS

The replacement uses only core-language constructs: a local
`let files = [(..., ...), (..., ...)];` binding plus array-to-slice coercion
via `&files` against `plan_multi(profile_yaml: &str, files: &[(&str, &str)])`
(line 117). Both constants are `const ...: &str` (lines 152, 176), so the
binding types as `[(&str, &str); 2]` and coerces cleanly at both call sites,
which live in the same function scope. This is edition-independent, stable
since Rust 1.0; no library API is involved, so there is no doc-currency risk
to check. The claim that TC-C already uses this pattern is verified directly
against source: lines 875-879 (`tc_c_batch_unsafe_overlap_narrowing_is_rejected_by_the_multiset_guard`)
bind `let files = [...]` and pass `&files` to `plan_multi`. The replacement is
the in-repo idiom, not an import of a foreign one.

### (c) Load-bearing difference between the two sites — NONE

The two `plan_multi` calls differ only in the profile argument
(`P_NO_CLOBBER` baseline vs `&edited` re-plan). The finding's replacement
binds only the file array and leaves the profile arguments untouched, so that
difference is preserved. The file arrays themselves are identical in names and
content constants. Each `plan_multi` call creates its own `TempDir`; a shared
binding changes no lifetime or state semantics.

### (d) Concreteness for tag=yagni — PASS

Concrete construct named (single-caller pure-delegate wrapper fn at line 194)
and concrete replacement named (delete wrapper, bind array once, pass to both
calls). Matches the review's yagni dimension "wrapper that only delegates ...
layer with one caller" verbatim (ROADMAP idiomacy-review entry).

## Decision guard — NO HIT

- `grep -rni "no_clobber|suggestions.rs|clobber|never_widens|AMBIGUOUS_FOO|GUARDED_FOO"`
  over docs/superpowers/specs/*.md, docs/IDEAS.md, docs/ROADMAP.md: zero hits.
- ROADMAP "Cosmetic cleanup (sweep group K)": enumerated items do not include
  this wrapper.
- ROADMAP "Test-hygiene collection (docs-tree B-minors)" B1-B11: not included.
- ROADMAP idiomacy-review NAMED INPUTS (T2-m1 ... final-verification nit): not
  included. The idiomacy-review ROADMAP entry itself is the process producing
  this finding, not a tracker of it.

No recorded decision protects the wrapper; nothing tracks this cleanup already.

## Notes

`lines_cut: 5` is plausible: the 9-line wrapper (194-202) is removed, a ~4-line
binding is added inside the test, both call sites shrink to `&files`.
