### Task 5: BCP-47 canonical-form matching (D19)

Make `exact` language matching semantically correct: two tags that denote the same language match, whatever spelling each carries. `lang_eq` canonicalizes both non-ISO operands (case + script suppression + deprecated-subtag replacement, via the crate's bundled IANA registry) and compares canonical forms case-insensitively; raw compare stays the fallback when a value does not canonicalize. This closes the fragility D19 named (`pt-Latn-BR` vs `pt-BR`) instead of leaving it. Also states the underlying principle - `exact` is typed value-equality, not string equality - in spec 4.3.

Accept/match asymmetry (deliberate): validation (Task 4) uses `parse()` well-formedness (accept liberally, mkvmerge rejects the pathological tag); matching uses `canonicalize()` (compare precisely). A tag that does not canonicalize simply never matches, which is harmless.

**Files:**
- Modify: `crates/muxsmith-core/src/matcher.rs:138` (`lang_eq`)
- Test: `crates/muxsmith-core/src/matcher.rs` `#[cfg(test)]` module
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` section 4.3 (the `exact` principle)

**Interfaces:**
- Consumes: `LanguageIndex::normalize`; `language_tags::LanguageTag::{parse, canonicalize, as_str}`.

- [ ] **Step 1: Confirm mkvmerge's IETF normalization default (SI-3, informational)**

Run: `mkvmerge --help 2>&1 | grep -A2 normalize-language-ietf` and, if unclear, mux a tiny file with `--language 0:pt-Latn-BR` and `mkvmerge -J` the output to see whether mkvmerge stores `pt-Latn-BR` or the suppressed `pt-BR`. Record the observed default in a one-line comment above `lang_eq` (it sets how often canonicalization actually fires: mkvmerge-authored tracks are usually already canonical; the real hits are non-mkvmerge-authored and old files). Not a gate, just an honest note.

- [ ] **Step 2: Write the failing tests**

In the matcher test module add (canonical equality, deprecated code, meaningful distinctions preserved):

```rust
#[test]
fn lang_eq_canonical_forms_match() {
    let idx = LanguageIndex::default(); // empty: neither token normalizes as ISO
    assert!(lang_eq("pt-BR", "pt-Latn-BR", &idx)); // redundant default script suppressed
    assert!(lang_eq("pt-BR", "PT-br", &idx));       // case
    assert!(lang_eq("he", "iw", &idx));             // deprecated code replaced
}

#[test]
fn lang_eq_preserves_meaningful_distinctions() {
    let idx = LanguageIndex::default();
    assert!(!lang_eq("pt-BR", "pt-PT", &idx));       // region
    assert!(!lang_eq("zh-Hans", "zh-Hant", &idx));   // script (both meaningful)
    assert!(!lang_eq("pt-BR", "pt", &idx));          // region-specific != bare
}
```

- [ ] **Step 3: Run to verify they fail**

Run: `cargo test -p muxsmith-core lang_eq_canonical -- --nocapture`
Expected: FAIL - the current raw fallback does not match `pt-BR` to `pt-Latn-BR` or `he` to `iw` (different strings).

- [ ] **Step 4: Implement canonical-form comparison**

Replace `lang_eq` (matcher.rs:138-143) with:

```rust
fn lang_eq(a: &str, b: &str, lang: &LanguageIndex) -> bool {
    if let (Some(na), Some(nb)) = (lang.normalize(a), lang.normalize(b)) {
        return na == nb;
    }
    // Non-ISO operand(s): compare BCP-47 CANONICAL forms (case + script
    // suppression + deprecated-subtag replacement) so two spellings of the
    // same language match (pt-Latn-BR == pt-BR, iw == he) while meaningful
    // distinctions survive (pt-BR != pt-PT). Raw case-insensitive compare is
    // the fallback when a value is not a canonicalizable tag.
    if let (Some(ca), Some(cb)) = (canonical_tag(a), canonical_tag(b)) {
        return ca.eq_ignore_ascii_case(&cb);
    }
    a.eq_ignore_ascii_case(b)
}

// The canonical BCP-47 form of `s`, or `None` if it is not a valid,
// canonicalizable tag. Well-formed-but-invalid tags (e.g. `xx-YY`) return
// `None` and fall back to a raw compare.
fn canonical_tag(s: &str) -> Option<String> {
    language_tags::LanguageTag::parse(s)
        .ok()?
        .canonicalize()
        .ok()
        .map(|t| t.as_str().to_string())
}
```

(If `canonicalize()`'s exact behavior on a given tag differs from the test expectation, adjust the test to the crate's documented canonical output - the assertions encode intent; the crate is authoritative on RFC 5646 canonical form.)

- [ ] **Step 5: Run the tests, then the suite**

Run: `cargo test -p muxsmith-core lang_eq`
Expected: PASS.
Run: `cargo test --workspace`
Expected: PASS (existing ISO-code matching unaffected: those hit the `normalize` arm first).

- [ ] **Step 6: State the principle in the spec**

In `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` section 4.3, add a short paragraph:

> `exact` is typed value-equality, not raw string equality: each property is compared in its own domain. Numbers compare numerically (`6` == `6.0`); languages compare as languages, with ISO 639 spellings and BCP-47 tags reduced to canonical form (`de` == `ger`, `pt-Latn-BR` == `pt-BR`) while meaningful distinctions are preserved (`pt-BR` != `pt-PT`, `zh-Hans` != `zh-Hant`). Use `regex` for byte-literal matching.

- [ ] **Step 7: Gate and commit**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`

```bash
git add -A
git -c commit.gpgsign=false commit -m "$(cat <<'EOF'
feat(matcher): BCP-47 canonical-form language equality (D19)

exact matches two tags that denote the same language whatever their
spelling (pt-Latn-BR == pt-BR, iw == he) via canonicalize(), while meaningful
distinctions survive (pt-BR != pt-PT). Documents the core principle in spec
4.3: exact is typed value-equality, not string equality; regex for literal.

Co-Authored-By: <session model> <noreply@anthropic.com>
EOF
)"
```

---

