# Seed verification: [T5-m2] known_extensions required-method idiom

**Verdict: CONFIRMED** (still present at HEAD)

## Finding

`Identify::known_extensions` carries a default body returning `None`
(`crates/muxsmith-core/src/identify.rs:383-385`). This inverts the trait
idiom for a capability the planner actually consumes
(`planner.rs:242-243`, batch-wide `profile.input.extensions` validation):

- The **sole production impl** (`LiveIdentifier`, identify.rs:402) must
  override the default to do anything; the default exists purely so test
  fakes compile unchanged - the doc comment says so explicitly
  ("Defaulted here so existing `Identify` fakes need no change to keep
  compiling").
- A **future production impl that forgets the override is silently
  vacuous**: the extension check degrades to a no-op with no compiler
  signal. Rust trait defaults are idiomatic for derived behavior
  expressible in terms of required methods, not for capability stubs
  whose absence changes production semantics.

## State at HEAD

Implementors of `Identify`:

| Impl | Location | known_extensions |
|---|---|---|
| `LiveIdentifier` (production) | `crates/muxsmith-core/src/identify.rs:397` | overrides (real) |
| `FakeIdent` (test) | `crates/muxsmith-core/tests/support/mod.rs:20` | relies on default |
| `OneIdent` (test) | `crates/muxsmith-cli/tests/catalog_completeness.rs:467` | relies on default |
| `FakeIdentWithExtensions` (test) | `crates/muxsmith-core/tests/planner_resolution.rs:1695` | overrides (fixture) |

## Replacement

Make `known_extensions` a required method (drop the `{ None }` body,
declaration only), then:

1. `FakeIdent` (`tests/support/mod.rs`): add explicit
   `fn known_extensions(&mut self) -> Option<Vec<String>> { None }`.
2. `OneIdent` (`muxsmith-cli/tests/catalog_completeness.rs`): same
   explicit `None` impl.
3. Trim the now-stale doc sentence "Defaulted here so existing
   `Identify` fakes need no change to keep compiling."; keep the
   `None` semantics sentence (capability unavailable / degrades to
   no-op), which stays true.

`LiveIdentifier` and `FakeIdentWithExtensions` need no change. The
compiler then forces every new impl (production or fake) to state its
extension capability explicitly.

## Cost

- lines_cut: 0 (net approx +4 lines: two explicit 3-line fake impls
  minus default body and one doc sentence; this is an idiom/safety fix,
  not a size reduction)
- deps_cut: 0
- Risk: none; behavior identical, purely moves an implicit `None` to
  explicit impls. `cargo build --workspace --all-targets` verifies
  completeness.
