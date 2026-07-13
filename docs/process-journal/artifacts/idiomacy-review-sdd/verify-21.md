# Verify-21: partition_diags() single-caller yagni (suggestions.rs:591)

**Verdict: CONFIRMED**

Finding: `fn partition_diags()` at `crates/muxsmith-core/tests/suggestions.rs:591` has one caller; inline the filter chain. Tag `yagni`, slice F2a. Verified at HEAD `2f17880`.

## Checks against the refutation criteria

**(a) Cited code says what the finding claims - PASS.**
- `partition_diags` defined at lines 591-597, filters `batch.batch_diagnostics` on `DiagCode::SuggestionPartition`.
- Exactly one caller: line 627 (`no_single_fix_produces_a_two_group_partition`). Repo-wide grep finds no other use.
- The sibling overlap tests do check `SuggestionPartition` inline, as claimed: lines 806-810 and 989-993 both use `batch.batch_diagnostics.iter().any(|d| d.code == DiagCode::SuggestionPartition)` directly (absence asserts). The helper is therefore not the file's established idiom for this diagnostic; two of three sites bypass it.
- Contrast holds: `overlap_diags` (line 667) has **six** call sites (693, 761, 787, 882, 938, 973), not the four the finding states. The undercount strengthens, not weakens, the contrast; it is not load-bearing.

**(b) Replacement is current idiom - PASS.**
The proposed inline chain merges the helper's code filter with the caller's existing `kind == "group"` filter (current lines 627-630). Every expression in the replacement already exists verbatim in the file at HEAD: the `d.code == DiagCode::SuggestionPartition` predicate (line 595) and `d.params.get("kind").map(String::as_str) == Some("group")` (line 629). Type inference is unchanged (`Vec<&Diagnostic>` via the existing `let groups: Vec<_>` binding); all downstream uses of `groups` (lines 631-653) are unaffected. A plain `iter().filter().collect()` chain has no version-sensitive surface for Rust 1.96.1 / edition 2024; the in-repo usage is the authoritative evidence, no external doc lookup needed.

**(c) Load-bearing difference between sites - NONE.**
The inline sibling sites assert absence via `.any()` while the helper collects for presence, but the finding does not claim they are duplicates to consolidate; it cites them as evidence that inline checking is the file's normal mode for this diagnostic. That characterization is accurate.

**(d) yagni completeness - PASS.**
Concrete construct (the helper fn, line 591) and concrete replacement (the merged filter chain at line 627) are both named.

## Decision guard

- `docs/superpowers/specs/*.md`: partition mentions concern the D6 no-single-fix partition algorithm and its diagnostics (plan-2 D6, v1-design SuggestionPartition row, plan-5.5 T13/T17/T18), never the test helper. The plan-4 memo's test-helper dedup entry is about `FakeIdent`/`lang()` fake-mkvmerge helpers, unrelated.
- `docs/ROADMAP.md`: the whole-codebase idiomacy review entry (line 161ff) explicitly defines the `yagni` dimension including "layer with one caller" - this finding executes that mandate rather than conflicting with it. Its NAMED INPUTS list does not contain `partition_diags`. Test-hygiene collection (line 307ff) and cosmetic-cleanup group K (line 260ff) do not contain it. The "fake-mkvmerge helpers beyond three copies" trigger (line 62) is unrelated.
- `docs/IDEAS.md`: no hit.

Not tracked anywhere; no recorded decision protects the helper.

## Minor corrections to the finding (non-refuting)

- `overlap_diags` has six callers, not four.
- `lines_cut: 5` is conservative: the helper body is 7 lines plus a blank line, and inlining collapses the 4-line two-step chain at the call site into one chain; realistic net cut is 7-9 lines.
