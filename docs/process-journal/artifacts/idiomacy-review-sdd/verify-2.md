# Verify-2: InvalidRegex block triplicated in validate_expr (F1a, tag=dup)

**Verdict: CONFIRMED**

## Finding under test

`crates/muxsmith-core/src/profile/validate.rs:280` - the 8-line InvalidRegex
compile-check block is repeated three times inside `validate_expr`'s
substring/regex loop (raw: branch, codec_kind branch, fallthrough), forced by
continue-per-branch control flow. Proposed: if / else-if / else chain for the
property-level diagnostic, then one regex compile check at the end of the loop
body; diagnostic order preserved.

## Checks

### (a) Code says what the finding claims - PASS

Read at HEAD. The block

```rust
if kind == "regex"
    && let Err(e) = regex::Regex::new(value)
{
    diags.push(
        Diagnostic::error(DiagCode::InvalidRegex, p)
            .with("detail", flatten_regex_error(&e)),
    );
}
```

appears at lines 281-288 (raw: branch, before `continue`), 301-308 (codec_kind
branch, before `continue`), 321-328 (fallthrough, end of loop body). Diffed
the three extracts ignoring indentation: byte-identical. The `continue` after
the first two branches is exactly what forces the pre-continue repetition, as
claimed.

### (c) No load-bearing difference between sites - PASS

Textually identical, including the `p` move into the diagnostic. Ownership is
compatible with the merge: each branch's property-level diagnostic either
borrows `p` (`raw_opt_in_diagnostic(&p, ...)`) or clones it
(`CodecKindExactOnly`, `NotStringProperty`), leaving `p` free to move into the
single trailing InvalidRegex check.

Semantic equivalence of the replacement, per path:

- `raw:` prefix: raw_opt_in_diagnostic, then regex check - unchanged.
- `codec_kind` with known prop_type: CodecKindExactOnly, then regex check -
  unchanged. `raw:codec_kind` still takes the raw: branch first (prop is
  `raw:codec_kind`, never equal to `codec_kind`), so the D32 "raw: sidesteps
  the exact-only guard" behavior is preserved by branch order.
- Fallthrough (unknown / non-string / string-ok): match diagnostics, then
  regex check - unchanged, including the unknown-property + bad-regex
  double-diagnostic case.

The regex diagnostic is last within the property in every current path, so
"one check at the end of the loop body" preserves diagnostic order exactly.

### (b) Replacement is current idiom - PASS

Pure control-flow restructuring (branch-tail hoisting into if/else-if/else);
no library API involved, so no docs question arises. The only non-classic
construct, the let-chain `if kind == "regex" && let Err(e) = ...`, is already
in the existing code at all three sites and compiles on the pinned toolchain
(Rust 1.96.1, edition 2024 - let-chains stable since 1.88). The replacement
introduces nothing the file does not already use.

### (d) tag=yagni gate - N/A

tag=dup; concrete construct and concrete replacement are named regardless.

## Decision guard - no hit

- `docs/superpowers/specs/*.md`: D32 (plan-5.5 design decisions) pins the
  *behavior* that raw: still reports InvalidRegex ("a value-level
  regex-compile error is still reported"); the replacement keeps that.
  Plan-2 / v1-design pin CodecKindExactOnly semantics, not code structure.
  No memo mandates the continue-per-branch shape.
- `docs/ROADMAP.md`: group K (cosmetic cleanup) lists six items, none of them
  this triplication. Residue R4 "regex compile cache (matcher.rs:74)" is a
  different file and a different concern (runtime recompilation in the
  matcher, not validate-time duplication). validate.rs at ROADMAP line 23
  concerns schema keyword domains, unrelated.
- `docs/IDEAS.md`: single regex mention (forced-flag recognition), unrelated.

## Notes

`lines_cut: 14` is plausible: two 8-line blocks plus two `continue;` lines
removed (18), minus ~3-4 lines of else-if/else scaffolding.
