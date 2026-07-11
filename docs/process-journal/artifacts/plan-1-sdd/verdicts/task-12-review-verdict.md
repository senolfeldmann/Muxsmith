<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-12  (round 1 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_016rdFwF7cM98Rj2T7BYYSuw
  agent_id:           aebfc18cc8ad04358
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-aebfc18cc8ad04358.jsonl
  dispatch_desc:      Review Task 12 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-08T00:32:21.292Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

# Task 12 Review: Fluent catalogs, renderer, and `validate` subcommand

### Spec Compliance

- **Catalog completeness**: verified against the actual `DiagCode` enum in `crates/muxsmith-core/src/report.rs` (30 variants, `diag_codes!` macro). All 30 keys match `locales/en/diagnostics.ftl` exactly, plus `severity-error/warning/info` (33 total), plus `validate-ok`/`validate-summary`/`diagnostic-line` in `cli.ftl` (3 more) = 36 required entries, all present, no extras, no mismatches.
- **No panic on missing catalog entry**: `Renderer::msg` (`crates/muxsmith-cli/src/i18n.rs:37-44`) uses two `let-else` guards returning the raw id; no `.unwrap()`/`.expect()` in that path. Confirmed.
- **Locale resolution** (explicit -> sys-locale -> en, parse-fallback to en): `i18n.rs:12-19` implements exactly that chain, including graceful fallback for a syntactically invalid explicit `--locale` value (falls to `"en".parse().unwrap()`, which cannot fail).
- **Exit codes**: `worst_severity` (Task 2, unmodified) drives `2`/`1`/`0` correctly; `load::from_file`'s I/O and parse errors are `Diagnostic::error(...)`, so missing/unreadable files correctly land on exit 2.
- **ASCII quotes / no em-dashes or curly quotes**: grepped both new `.ftl` files and all new Rust files for U+2018/2019/201C/201D/2013/2014 — zero hits.
- **No hardcoded prose outside catalogs**: every `println!` in `main.rs`/`commands/validate.rs` prints either a `renderer.msg(...)`/`renderer.diagnostic(...)` result or a `serde_json::Value`. Confirmed.
- **Sort order (text mode)**: `sort_by_key(|d| Reverse(d.severity))` on a derived `Ord` enum (`Info < Warning < Error`) correctly yields error-first, and `sort_by_key` is a stable sort, so the "stable order otherwise" requirement holds.
- **JSON shape**: `serde_json::to_value(d)` plus `v["rendered"] = ...` correctly adds the field; test asserts `code`/`severity`/`rendered` are all strings.
- **Two compliance gaps found** — see Issues below (JSON ordering; catalog-completeness regression coverage).

### Strengths

- Renderer is a small, direct implementation with no unearned abstraction: one struct, two public methods, a private helper. Matches the framework's own "match structural complexity to scale" bar.
- The fallback chain (`get_message` -> `message.value()` -> raw id) is exactly the "visible failure over silent/panic" behavior the spec demands, and it's the only place any hardcoded-adjacent value (the id itself) can leak into output, which is the intended emergency valve, not a violation.
- Test suite (5 tests) genuinely exercises all three exit codes, the `--json` shape, and a dynamically-written tempfile fixture for the warnings-only path, matching the brief's TDD plan exactly.
- Dependency additions (`fluent-bundle`, `unic-langid`, `sys-locale`, dev-dep `tempfile`) are minimal and match `Cargo.toml`/`Cargo.lock` consistently; no stray transitive surprises.

### Issues

#### Important

1. **`--json` output is not sorted error-first, unlike the text renderer** (`crates/muxsmith-cli/src/commands/validate.rs:16-25` vs. `:29-30`). The JSON branch builds `entries` straight from `diagnostics.iter()` (raw insertion order: `validate::validate` diagnostics, then `lint::provable_overlaps` appended), while the text branch explicitly clones and sorts by `Reverse(severity)` before printing. This is a real, demonstrable inconsistency, not just a hypothetical: `crates/muxsmith-core/src/profile/validate.rs:59-64` pushes a `Diagnostic::warning(DiagCode::EmptyMatchExpression, ...)` *inside* the per-track loop, before several error-severity checks that run later in the same function (e.g. `InvalidKeyword` on `output.filename`/`chapters`/`title` at lines 102/122/139, or error checks on later tracks). A profile with an empty match expression on `tracks[0]` and an error-triggering issue anywhere after it produces `diagnostics[0]` == a Warning in `--json` even though the overall exit code is 2 and errors exist further into the array. `json_output_is_machine_readable` only asserts the array is non-empty and checks field presence on `diags[0]`, so this wouldn't be caught. If a GUI or script naively takes `diagnostics[0]` as "the primary issue" (plausible given the "sorted error-first" contract is stated as a property of `validate` in general, not scoped to text output), it gets the wrong entry.
   - Fix: sort `entries`/`diagnostics` once before branching, and derive both the JSON array and the printed lines from the same sorted vector.

2. **Misleading comment, no regression test for catalog completeness** (`crates/muxsmith-cli/src/i18n.rs:39`: `// ... CI guards this case.`). No test anywhere in the workspace (checked `muxsmith-core`, `muxsmith-cli`, `xtask`) asserts that every `DiagCode::ALL` key resolves to a real catalog message. `report.rs`'s `all_keys_are_unique`/`all_keys_match_serde_encoding` tests check internal consistency of `DiagCode`/`key()`, not catalog coverage. The brief's own hard requirement ("Catalogs must contain a message for every DiagCode key") is currently satisfied only by the implementer's manual grep in the report's self-review section — nothing enforces it going forward. Add a workspace test (in `muxsmith-cli`, since only it can embed the `.ftl` files) that iterates `DiagCode::ALL` and asserts `renderer.msg(code.key(), &[]) != code.key()`; otherwise the comment's claim is false and the very safety net the spec asks for (visible failure) will pass silently in CI the day someone adds a 31st `DiagCode` and forgets the catalog line.

#### Minor

3. `i18n.rs` repeats fully-qualified `muxsmith_core::report::{Diagnostic, Severity}` paths inline (function signature and `severity_key`'s match arms) instead of a single `use` import — purely a readability nit, no functional effect.
4. No unit-level test directly exercises `Renderer`'s two safety-net paths (unknown message id -> raw id; malformed/unknown `--locale` -> en) in isolation; today this is only covered indirectly (a couple of real diagnostic codes render correctly) or manually per the report's self-review. Cheap to add as a 1-2 line unit test alongside fixing #2.

### Assessment

**Task quality:** Needs fixes

**Reasoning:** Core i18n architecture, catalog completeness, panic-safety, and exit-code logic are all correctly implemented and verified against the actual `DiagCode` source, not just the brief's copy. The `--json` sort-order gap is a concrete, reproducible violation of the "sorted error-first" contract for one of the two supported output modes, and the missing catalog-completeness test leaves an explicit "must" requirement enforced by hand rather than by CI, contradicted by the code's own comment.