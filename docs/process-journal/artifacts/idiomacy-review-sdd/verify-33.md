# Verify-33: catalog_completeness.rs string_pairs / FluentArgs round-trip (yagni, slice F3)

**Verdict: CONFIRMED**

## Finding under test

`crates/muxsmith-cli/tests/catalog_completeness.rs:453` — fixture tables build `FluentArgs` only for `string_pairs` to round-trip them back into the `&[(&str, &str)]` shape `Renderer::msg` takes; every fixture value is a string literal, so the `FluentArgs` intermediate and `string_pairs`'s structurally-unreachable non-string panic arm are unused generality.

## Checks

### (a) Cited code says what the finding claims — yes

- `string_pairs` is at lines 453-460 with the panic arm at 457 (`other => panic!("fixture value for {k:?} is not a string: ...")`), exactly as cited.
- `fixture_args` (lines 48-204) and `allowlisted_cli_key_args` (lines 267-366) build `FluentArgs<'static>` exclusively via `args.set("key", "string literal")` — every arm inspected, no non-string value anywhere.
- `render_and_find_leaks` (lines 437-448) takes `(id, FluentArgs)` pairs, converts each back through `string_pairs` into `Vec<(&str, &str)>`, and calls `renderer.msg(id, &pairs)`.
- `Renderer::msg` (crates/muxsmith-cli/src/i18n.rs:45) is `pub fn msg(&self, id: &str, args: &[(&str, &str)]) -> String` and internally rebuilds a `FluentArgs` from the pairs. The test therefore does FluentArgs -> pairs -> FluentArgs: a double round-trip with a dead panic arm in the middle.

### (b) Replacement is current idiom — yes

- Verified against the **pinned** fluent-bundle 0.16.0 source (Cargo.lock pin; `~/.cargo/registry/src/.../fluent-bundle-0.16.0/src/types/mod.rs:304`): `impl<'source> From<&'source str> for FluentValue<'source>` yields `FluentValue::String(s.into())`, and `FluentArgs::set` goes through `V: Into<FluentValue>`. With every fixture value a `&'static str` literal, the non-string arm is structurally unreachable — not defensive coverage of a live case.
- Returning `Vec<(&'static str, &'static str)>` from both fixture functions and passing the slice straight to `renderer.msg` is plain direct Rust (produce the shape the consumer takes); no framework idiom is violated and no external-API claim is involved beyond the fluent-bundle fact verified above.
- Behavior-preserving: numeric-looking fixture values ("1", "2") already reach `msg` as strings today (Fluent string selectors fall through to `*[other]`, per the `msg_with_counts` doc comment); the replacement changes plumbing shape only, not what is rendered or asserted.
- Guard semantics preserved: the exhaustive `match code` in `fixture_args` (compile-error on new `DiagCode`) and the panic-by-default arm in `allowlisted_cli_key_args` both survive a return-type change untouched.
- Production mirrors nothing FluentArgs-shaped either: `Diagnostic.params` is `BTreeMap<String, String>` (muxsmith-core/src/report/mod.rs:197, `with(key: impl Into<String>, value: impl Into<String>)` at :237). String pairs end-to-end.
- One minor extension to the stated replacement: once both fixture functions return pairs, the `FluentArgs` import goes as well, not only `FluentValue` (`FluentResource` stays — used by the .ftl enumeration). Superset of the finding's cleanup, not a defect in it.

### (c) Duplication with load-bearing difference — n/a

No duplication claim in this finding.

### (d) yagni names concrete construct + replacement — yes

Construct: `string_pairs` + the `FluentArgs` intermediate + the unreachable panic arm. Replacement: `Vec<(&'static str, &'static str)>` return type, direct pass to `renderer.msg`, delete `string_pairs` and the now-unused imports.

## Decision guard

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `string_pairs`, `FluentArgs`, `fixture_args`, `catalog_completeness`:

- ROADMAP.md:139 — "Catalog param-drift guard: DONE" records the guard's existence and its single-site blind spot; says nothing about the arg-container shape. The replacement keeps the guard's coverage contract intact.
- specs/2026-07-11-plan-5.5-design-decisions.md:50 — "C1 exhaustive-fixture obligations in catalog_completeness" obliges exhaustive fixtures for new DiagCodes; the exhaustive match survives the return-type change.
- ROADMAP v1.x candidates, cosmetic-cleanup group K, and the test-hygiene collection (B-minors) were read in full: none lists this construct.

No conflict, not tracked.
