### Task 4: BCP-47 language validation (D19, the bug fix)

Swap the language predicate from ISO-index-membership to "in the ISO index OR a well-formed BCP-47 tag", so `changes: { language: pt-BR }` and `exact: { language: sr-Latn }` stop hard-failing at plan time. Uses `language-tags::LanguageTag::parse` (well-formedness only; no registry `validate()`, per D19 - mkvmerge rejects the pathological well-formed-but-nonexistent tag at mux time).

**Files:**
- Modify: `crates/muxsmith-core/Cargo.toml` (add `language-tags = "0.3.2"`), `Cargo.lock`, `deny.toml` (only if `cargo deny` flags the new license/advisory)
- Modify: `crates/muxsmith-core/src/capability/runtime.rs` (add `LanguageIndex::is_valid_value`)
- Modify: `crates/muxsmith-core/src/planner.rs:301` (`walk_exact_languages`) and `planner.rs:568` (`resolve_changes`)
- Test: `crates/muxsmith-core/tests/planner_resolution.rs` (a regional tag now validates)

**Interfaces:**
- Consumes: `LanguageIndex::normalize(&self, &str) -> Option<String>`.
- Produces: `LanguageIndex::is_valid_value(&self, token: &str) -> bool`.

- [ ] **Step 1: Write the failing test**

In `crates/muxsmith-core/tests/planner_resolution.rs`, add a test (using the file's `plan`/`plan_one` helper) that a profile with `changes: { language: pt-BR }` on a matched track produces NO `InvalidPropertyValue` diagnostic (whereas today it does). Assert the resulting plan is present and diagnostic-free for that rule. Follow the helper conventions already in the file.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p muxsmith-core --test planner_resolution pt_br -- --nocapture`
Expected: FAIL (an `InvalidPropertyValue` for `pt-BR` is currently emitted).

- [ ] **Step 3: Add the dependency**

In `crates/muxsmith-core/Cargo.toml` under `[dependencies]`, add:

```toml
language-tags = "0.3.2"
```

Run `cargo build -p muxsmith-core` to update `Cargo.lock`.

- [ ] **Step 4: Add the predicate**

In `crates/muxsmith-core/src/capability/runtime.rs`, add to `impl LanguageIndex`:

```rust
    /// Whether `token` is an acceptable language value: a recognized ISO
    /// 639-1/2/3 code (via [`normalize`](Self::normalize)) OR a well-formed
    /// IETF BCP 47 tag (region/script subtags, e.g. `pt-BR`, `sr-Latn`).
    /// Well-formedness only (RFC 5646 grammar); a grammatically valid but
    /// nonexistent tag is accepted here and left for mkvmerge to reject at
    /// mux time (D19).
    pub fn is_valid_value(&self, token: &str) -> bool {
        self.normalize(token).is_some() || language_tags::LanguageTag::parse(token).is_ok()
    }
```

- [ ] **Step 5: Use the predicate at both validation points**

`planner.rs:301` (`walk_exact_languages`): change `&& lang.normalize(v).is_none()` to `&& !lang.is_valid_value(v)`.

`planner.rs:568` (`resolve_changes`): change `let valid = matches!(value, Scalar::Str(s) if lang.normalize(s).is_some());` to `let valid = matches!(value, Scalar::Str(s) if lang.is_valid_value(s));`.

- [ ] **Step 6: Run the test, then the suite and deny**

Run: `cargo test -p muxsmith-core --test planner_resolution pt_br`
Expected: PASS.
Run: `cargo test --workspace && cargo deny check`
Expected: PASS. If `cargo deny check` flags `language-tags` (license/advisory), add the minimal allow entry to `deny.toml` with a one-line comment; MIT/Apache-2.0 is standard and usually already allowed.

- [ ] **Step 7: Gate and commit**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`

```bash
git add -A
git -c commit.gpgsign=false commit -m "$(cat <<'EOF'
fix(planner): accept well-formed BCP-47 language tags (D19)

Language values validate as ISO-code-or-well-formed-BCP-47 (language-tags
crate), so changes.language and match values like pt-BR / sr-Latn stop
hard-failing at plan time. mkvmerge supports them; the spec always said
BCP-47. Registry validation deliberately deferred.

Co-Authored-By: <session model> <noreply@anthropic.com>
EOF
)"
```

---

