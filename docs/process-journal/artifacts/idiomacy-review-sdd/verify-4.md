# Verify-4: dead `at` diagnostic param in profile/load.rs

**Finding:** yagni - dead diagnostic param `at` set at both emitter sites (load.rs:56, :64), duplicating `config_path`; no Fluent catalog, frontend, or CLI code consumes it. Replacement: drop both `.with("at", ...)` calls, the `contains_key("at")` assert in `tests/profile_load.rs:92`, and the fixture line in `muxsmith-cli/tests/catalog_completeness.rs:57`.

**Verdict: TRACKED**

**Ref:** `docs/ROADMAP.md`, section "v1.x candidates" -> entry **"Cosmetic cleanup, one pass (sweep group K)"**: "dead `at` param (load.rs:56,64)" (recorded from sweep walkthrough #21, group K).

## Technical verification (the finding is substantively correct)

Every load-bearing claim checked against HEAD (`2f17880`):

- **(a) Cited code says what the finding claims.** `crates/muxsmith-core/src/profile/load.rs:56` has `.with("at", "")` (I/O-failure site) and `:64` has `.with("at", err.path().to_string())` (parse-failure site). `Diagnostic::error(code, config_path)`'s second argument *is* `config_path` (`report/mod.rs:218`), and at the parse site it receives the identical `err.path().to_string()` (load.rs:62), at the I/O site the identical `""` (load.rs:53). So `at` duplicates `config_path` exactly at both sites - claim (c) has no load-bearing difference between the two.
- **No consumer exists.** `$at` appears in none of the 12 `.ftl` files under `locales/`; both `parse-error` messages use only `{ $detail }` (`locales/en/diagnostics.ftl:6`, `locales/de/diagnostics.ftl:11`). Grep over all `.rs`/`.ts`/`.vue` (gui/src, muxsmith-cli/src) finds exactly four references to `"at"`: the two emitter sites and the two test sites the finding names. Fluent ignores extra args, so nothing renders it either way.
- **The replacement is complete and consistent.** `tests/profile_load.rs:92` (`assert!(err.params.contains_key("at"))`) pins the dead param and must go with it. The `catalog_completeness.rs:57` fixture's own doc contract says fixtures mirror "per param an emitter actually sets", so dropping the emitter param requires dropping the fixture line. Minor addendum the finding omits: the `DiagCode::ParseError` doc comment at `report/mod.rs:70` still mentions ``config_path`/`at``; it should be trimmed in the same pass. That is an incompleteness, not a refutation.
- **(d)** Tag is yagni with a concrete construct and a concrete replacement named - satisfied.

## Why TRACKED rather than CONFIRMED

The decision guard hits: the ROADMAP's "v1.x candidates" section carries the identical construct at the identical file and lines as part of the planned one-pass cosmetic cleanup (sweep group K). There is no recorded decision to *keep* the param (so not DECISION_CONFLICT); the removal is already on the books as deferred work. Reporting it as a fresh finding would duplicate an existing tracker entry.
