### Task 13: Catalog completeness guard and CI

**Files:**
- Test: `crates/muxsmith-cli/tests/catalog_completeness.rs`
- Create: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: `DiagCode` (Task 2), catalogs (Task 12).
- Produces: the spec 10 guard "CI fails on diagnostic codes without message templates", plus the CI matrix. (The help-id guard arrives with the GUI plan.)

- [ ] **Step 1: Write the failing-or-passing completeness test**

`crates/muxsmith-cli/tests/catalog_completeness.rs`:

```rust
//! Spec 10: every DiagCode must have a message template in the English
//! catalog. This test IS the CI guard.

const CATALOG: &str = include_str!("../../../locales/en/diagnostics.ftl");

// Keep in sync with report::DiagCode::key() - the test fails loudly when a
// new code is added without a catalog entry, which is exactly its job.
const ALL_KEYS: &[&str] = &[
    "unsupported-profile-version",
    "parse-error",
    "no-track-rules",
    "empty-match-expression",
    "empty-extensions",
    "invalid-regex",
    "unknown-property",
    "not-string-property",
    "value-type-mismatch",
    "unknown-settable-property",
    "invalid-keyword",
    "locator-conflict",
    "invalid-template",
    "unknown-template-field",
    "unknown-template-filter",
    "path-separator-in-template",
    "attachment-rule-shape",
    "provable-overlap",
    "ambiguous-rule",
    "overlapping-rules",
    "missing-track",
    "missing-external",
    "ambiguous-external",
    "output-collision",
    "source-overwrite",
    "duplicate-identifier",
    "donor-is-primary",
    "ignored-file",
    "multiple-identifier-matches",
    "unknown-property-skew",
];

#[test]
fn every_diag_code_has_a_catalog_message() {
    let missing: Vec<&str> = ALL_KEYS
        .iter()
        .filter(|key| {
            !CATALOG
                .lines()
                .any(|l| l.starts_with(&format!("{key} =")))
        })
        .copied()
        .collect();
    assert_eq!(missing, Vec::<&str>::new(), "missing catalog entries");
}
```

- [ ] **Step 2: Run the test**

Run: `cargo test -p muxsmith-cli --test catalog_completeness`
Expected: PASS (Task 12 wrote all entries; if it fails, add the missing lines to the catalog now)

- [ ] **Step 3: Create the CI workflow**

`.github/workflows/ci.yml`:

```yaml
name: ci
on:
  push:
    branches: [master]
  pull_request:

jobs:
  test:
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all --check
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo test --workspace
```

- [ ] **Step 4: Verify locally what CI will run**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
Expected: all three PASS (fix any fmt/clippy fallout now)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "ci: test matrix, lint gates, and diagnostic catalog completeness guard"
```

---

## Plan 1 exit criteria

- `cargo test --workspace` green on Linux (CI proves Windows/macOS).
- `muxsmith validate docs/examples/reference.yaml` style invocation works against the test fixture: exit 0.
- `muxsmith validate` on a profile with an ambiguity-prone ruleset renders actionable, localized messages with exact config paths; exit codes 0/1/2 correct.
- `muxsmith schema` emits the generated JSON Schema.
- No user-facing prose anywhere in `muxsmith-core` (review gate: grep string literals in core for sentence-like text).
- `generated.rs` committed; no upstream schema file in the tree.

## Deferred to later plans

- Plan 2: `identify` (mkvmerge -J + cache), `matcher` evaluation against real tracks, language normalization via `--list-languages`, `planner` + batch report + suggestion engine, `dry-run` and `identify` subcommands, runtime capability queries (`--version`, `--list-types`), file discovery (pattern scan, external locator resolution).
- Plan 3: `command` argv generation, `executor` with `--gui-mode` progress, job queue, `run` subcommand, integration tests against real mkvmerge fixtures.
- Plan 4: Tauri shell, React GUI (profile editor, batch view, job queue), help mode with help-id completeness guard, frontend Fluent wiring, packaging.
