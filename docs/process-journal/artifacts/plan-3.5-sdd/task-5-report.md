# Task 5 report: BCP-47 canonical-form language matching (D19)

(Plan 3.5, Task 5. Overwrites a stale report previously at this path from
Plan 3's differently-scoped Task 5, unrelated work, already committed.)

## Implemented

- `crates/muxsmith-core/src/matcher.rs`: `lang_eq` now falls back to comparing
  BCP-47 **canonical** forms (case-insensitively) when either operand does not
  normalize as an ISO code, via a new private `canonical_tag(s: &str) ->
  Option<String>` (`LanguageTag::parse(s).ok()?.canonicalize().ok().map(|t|
  t.as_str().to_string())`). Raw case-insensitive compare stays the last-resort
  fallback for values that do not even parse as a well-formed tag. The
  `normalize` (ISO) arm runs first and is untouched.
- Two new tests in the matcher's `#[cfg(test)]` module, exactly as specified in
  the brief: `lang_eq_canonical_forms_match`, `lang_eq_preserves_meaningful_distinctions`.
- Spec `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` section 4.3:
  added the `exact` principle paragraph verbatim (typed value-equality, not
  string equality).
- One-line comment above `lang_eq` recording mkvmerge's own
  `--normalize-language-ietf` default (informational, Step 1).

## Step 1: mkvmerge normalize default (informational, not a gate)

`mkvmerge --help` (v100.0, installed locally):

```
--normalize-language-ietf <canonical|extlang|off>
                         Normalize all IETF BCP 47 language tags to either
                         their canonical or their extended language subtags
                         form or not at all (default: canonical form).
```

Default is **canonical**. Recorded as a comment above `lang_eq`: mkvmerge-authored
tracks already arrive canonical, so the canonicalization path in `lang_eq`
mainly earns its keep on non-mkvmerge-authored or older files (hand-edited
tags, files muxed by other tools, or muxed with `--normalize-language-ietf
off`/`extlang`). Did not need to mux a probe file since the `--help` text was
unambiguous.

## SI-3 crate probe: real `canonicalize()` output (language-tags 0.3.2)

Built a throwaway scratch binary (`/tmp/.../langtag_probe`) against the exact
pinned version (`language-tags = "0.3.2"`, verified in `Cargo.toml`/`Cargo.lock`)
and printed `parse()` + `canonicalize()` for every tag in the brief plus the
two tags already used by the existing "falls back to raw compare" test:

| input | parse | canonicalize() output |
|---|---|---|
| `pt-Latn-BR` | OK | `pt-BR` |
| `pt-BR` | OK | `pt-BR` |
| `pt-PT` | OK | `pt-PT` |
| `iw` | OK | `he` |
| `he` | OK | `he` |
| `zh-Hans` | OK | `zh-Hans` |
| `zh-Hant` | OK | `zh-Hant` |
| `PT-br` | OK | `pt-BR` |
| `pt` | OK | `pt` |
| `xx-YY` | OK | `xx-YY` (unchanged; no registry validation, see below) |
| `de` | OK | `de` |
| `ger` | OK | `ger` |
| `zxx` | OK | `zxx` |
| `qqq` | OK | `qqq` |

**All required assertions hold on real crate output:**
- Script suppression merge: `pt-Latn-BR` == `pt-BR` (both canonicalize to `pt-BR`). PASS.
- Case: `PT-br` canonicalizes to `pt-BR`, matches `pt-BR`. PASS.
- Region distinction preserved: `pt-BR` (-> `pt-BR`) != `pt-PT` (-> `pt-PT`). PASS.
- Script distinction preserved: `zh-Hans` != `zh-Hant` (both left as-is, no suppress-script rule for `zh`). PASS.
- Bare vs. region-specific: `pt-BR` != `pt` (-> `pt`, unchanged). PASS.

**Nice-to-have, also holds:** `iw` -> `he`, `he` -> `he`, so `iw == he`. The
crate's bundled registry does carry the deprecated-code preferred-value
mapping. Kept the assertion in the brief's test verbatim; no downgrade needed.

**One correction to the brief's own inline comment, made from reading the
crate source** (`~/.cargo/registry/src/.../language-tags-0.3.2/src/lib.rs`,
`canonicalize()` at line 485): the brief's draft doc-comment for
`canonical_tag` said "well-formed-but-invalid tags (e.g. `xx-YY`) return
`None`". That is **not what the crate does** -- `canonicalize()` only returns
`Err` for one pathological case, `MultipleExtendedLanguageSubtags` (no unique
canonicalization when there's more than one extended-language subtag); it does
not validate that a region/script code exists in the registry, so `xx-YY`
canonicalizes to itself unchanged (confirmed above), not to `None`. Corrected
the doc-comment in the final code to describe the real behavior instead of
copying the brief's inaccurate example, per SI-3 (encode observed behavior,
never a rigged/assumed one). This does not change any test outcome (`xx-YY`
never appears in a required assertion) or the function's logic, only the
comment's accuracy.

Also confirmed the pre-existing `language_falls_back_to_raw_compare_when_unknown`
test (`zxx`/`qqq`) stays green: both tokens happen to parse as well-formed
BCP-47 primary-language subtags too (any 2-3 letter alpha string is
syntactically valid), so they now actually go through the `canonical_tag`
path rather than the raw fallback; canonicalize() leaves both unchanged, so
the assertions (`zxx == zxx`, `qqq != zxx`) hold either way. No test edit
needed; noted here since the existing doc-comment's phrasing ("fall back to
raw compare") is now imprecise for this specific pair, though still true in
spirit (raw-compare is the actual fallback path for genuinely unparseable
tokens) and out of this task's declared scope to rewrite.

## TDD RED/GREEN

- RED: temporarily reverted `lang_eq` to the pre-task raw-fallback-only body,
  ran `cargo test -p muxsmith-core --lib lang_eq -- --nocapture`:
  `lang_eq_canonical_forms_match` FAILED (`assertion failed: lang_eq("pt-BR",
  "pt-Latn-BR", &idx)`), `lang_eq_preserves_meaningful_distinctions` passed
  (expected: the meaningful-distinction test never required canonicalization,
  raw compare already rejects those pairs).
- Restored the canonical-form implementation.
- GREEN: `cargo test -p muxsmith-core --lib lang_eq -- --nocapture` -> both
  tests pass.

## Files changed

- `crates/muxsmith-core/src/matcher.rs` (`lang_eq` + `canonical_tag` + 2 new tests)
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (4.3 principle paragraph)

## Gate (all four green)

- `cargo test --workspace`: PASS (muxsmith-core lib: 75 tests, all crates 0 failures)
- `cargo fmt --all --check`: PASS
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS, 0 warnings
- `cargo deny check`: PASS (`advisories ok, bans ok, licenses ok, sources ok`)

## Self-review

- `normalize` (ISO) arm untouched and runs first; existing `de`/`ger` tests
  (`language_normalizes_iso_and_bcp47_against_both_fields`) still green,
  confirmed in the full-suite run.
- Both new functions are private (`fn`, not `pub fn`), so no `missing_docs`
  lint obligation; still gave both a doc/explanatory comment for maintainability.
- Comment above `lang_eq` states the mkvmerge default as informational per
  Step 1's "not a gate" framing.
- Spec paragraph copied verbatim from the brief (Step 6), ASCII punctuation
  only, ISO/BCP examples all ASCII already.
- Did not touch the pre-existing untracked `HANDOFF.md` /
  `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md` (unrelated,
  predate this task); commit stages only the two files this task modifies.

## Concerns

None blocking. One observation carried above for the record: the
`language_falls_back_to_raw_compare_when_unknown` test's name/doc is now
slightly imprecise (its `zxx`/`qqq` inputs exercise the `canonical_tag` path,
not the raw-fallback path) but the assertions remain correct and the brief
did not ask for that test to be touched, so left as-is.
