# Verify-30: dead tuple element in `unknown_keywords_are_flagged`

**Finding:** `crates/muxsmith-core/tests/validate_structure.rs:123` iterates `for (snippet, _section) in [("chapters: discard\n", "chapters"), ("title: wipe\n", "title")]` where the second element is never read. Tag: yagni. Proposed replacement: `for snippet in ["chapters: discard\n", "title: wipe\n"]`.

**Verdict: CONFIRMED**

## Checks

### (a) Code matches the claim
Read at HEAD (2f17880). Lines 123-132: the loop destructures `(snippet, _section)`; the body uses only `snippet` (string concatenation at line 127, assert message at line 130). `_section` is never read - the underscore prefix already concedes it. Its content ("chapters"/"title") is redundant with the snippet text itself, and the assert message prints the full snippet, so dropping it loses no diagnostic value. Grep across `crates/` shows this is the only `_section` occurrence; no sibling loop uses a parallel `(snippet, section)` structure that would make the tuple a deliberate pattern.

### (b) Replacement is current idiom
Verified against the Rust Reference (doc.rust-lang.org/stable/reference, via context7): `for PATTERN in iter_expr` loops over any `IntoIterator`; arrays iterate by value. The existing code already relies on exactly this mechanism (it iterates an array of tuples by value), so `for snippet in ["...", "..."]` is the same construct minus the dead element. Valid and idiomatic on Rust 1.96.1 / edition 2024.

### (c) Duplication difference
Not applicable - no duplication claim.

### (d) yagni concreteness
Concrete construct (the tuple iteration at line 123) and concrete replacement (plain array of `&str`) both named.

### Decision guard
Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `validate_structure`, `unknown_keywords`, `_section`, tuple-element phrasing: no hits. ROADMAP's cosmetic-cleanup group K (load.rs `at` param, model.rs placement, command_integration.rs doc, planner.rs eager resolve, etc.) and the test-hygiene collection (B1-B13) enumerate specific items; none touch this file or construct. Not tracked, no conflicting decision.

## Assessment
Trivial but real: a dead tuple element carrying information already present in its sibling. The fix is a strict simplification with zero behavior change.
