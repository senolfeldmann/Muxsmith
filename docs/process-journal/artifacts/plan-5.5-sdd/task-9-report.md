# Task 9 report: diagnostics polish (nine items)

Branch `plan55-t9` in worktree `.worktrees/post-t9`, based on post-wave-1
master (`240fd35`). Nine commits, one per item, `408593e..697dd70`, unsigned,
trailer `Co-Authored-By: Claude Fable 5`. Working tree clean. Full nine-part
gate green (see end). All anchors re-verified on this branch before editing
(they had all shifted from the brief's line numbers).

TDD throughout: failing test first, then fix, one commit per item. Tests run
foreground. mkvmerge is present on this host, so the mkvmerge-gated CLI
integration tests actually executed (did not self-skip).

## Per item

### (ix) plan-time `changes.language` InvalidPropertyValue missing `allowed` — commit 408593e
- **Anchor** (brief `planner.rs:600-605`, actual `resolve_changes` at
  `planner.rs:709-717`): emitted `InvalidPropertyValue` with only
  `property`+`value`; the `invalid-property-value` template also needs
  `$allowed`. Sibling `walk_exact_languages` (`planner.rs:388-396`) sets it.
- **Failing test** (`crates/muxsmith-cli/tests/catalog_completeness.rs`,
  `invalid_changes_language_diagnostic_renders_without_placeholder_leak`):
  drives the REAL emitter via `plan_batch` (small in-test `OneIdent`, SERIES
  fixture, a rule with `changes: { language: 'zz!' }`), finds the
  `.changes.language` diagnostic, renders it via the CLI `Renderer`, asserts no
  `{$`. Before the fix it rendered `Allowed values include: {$allowed}.` — the
  exact leak. Note: the token must be genuinely BCP-47-invalid; `xx-not-a-code`
  parses as a well-formed tag (`is_valid_value` accepts any grammatical tag),
  so `zz!` (illegal char) is used.
- **Fix**: added `.with("allowed", "a valid ISO 639/BCP-47 language code")`.
  Updated the catalog-completeness doc-comment limitation note (the "known
  case" is now fixed and pinned by the new test).
- **Files**: `crates/muxsmith-core/src/planner.rs`,
  `crates/muxsmith-cli/tests/catalog_completeness.rs`.

### (i) OverlappingRules names every claimant — commit bce35cd
- **Anchor** (`planner.rs:610-620`): named only `rules[0]`/`rules[1]` via
  `rule_a`/`rule_b`; a third claimant went unreported.
- **Failing test** (`planner_resolution.rs`,
  `overlapping_rules_names_every_claimant_not_just_the_first_two`): three rules
  each resolving to the single audio track; asserts `params["rules"]` contains
  `tracks[0]`, `tracks[1]`, `tracks[2]` and `params["track"] == "1"`.
- **Fix**: params become one rendered list `$rules` (comma-joined
  `tracks[N]`), keeping `track`; message updated to
  `Rules { $rules } all claim track { $track }.`; the exhaustive catalog
  fixture (`fixture_args` OverlappingRules arm) moved in lockstep. The
  config_path anchor (`tracks[rules[0]]`) and the `track` param are unchanged,
  so the R1-v multiset-signature test in `planner.rs` still holds.
- **Files**: `planner.rs`, `locales/en/diagnostics.ftl`,
  `catalog_completeness.rs`, `planner_resolution.rs`.

### (ii) `any: []` double report — commit 3081fc6
- **Anchor** (`validate.rs:65` vs `:301`): `{ any: [] }` is `is_empty()` true
  (empty `any` counts as empty), so it fired both `EmptyMatchExpression`
  (generic, `:65`) and `EmptyMatchList` (specific, `:301`).
- **Failing test** (`validate_hardening.rs`,
  `empty_any_list_reports_only_empty_match_list_not_empty_match_expression`):
  asserts exactly one diagnostic (`EmptyMatchList`) for `{ any: [] }`.
- **Fix**: suppress `EmptyMatchExpression` when a top-level `any`/`not` list is
  empty (that node already gets the more specific error). The truly-empty `{}`
  case still warns (`validate_semantics` unaffected, 14 pass).
- **Files**: `validate.rs`, `validate_hardening.rs`.

### (v) lint rule refs as `tracks[N]` — commit cedea75
- **Anchor** (`lint.rs:34-36`): `ProvableOverlap` set `rule_a`/`rule_b` as bare
  indices (`0`, `1`) while the planner's `OverlappingRules` uses `tracks[N]`.
- **Failing test**: the two existing lint tests' assertions changed to expect
  `tracks[0]`/`tracks[1]` (failing against the bare-index output).
- **Fix**: format both refs as `tracks[{idx}]`. Param names unchanged, so the
  `provable-overlap` message is untouched; the catalog fixture values updated
  for faithfulness. No other consumers of these params exist.
- **Files**: `lint.rs`, `catalog_completeness.rs`.

### (vi) donor-side UnsupportedSource — commit 22b83ae
- **Anchor** (donor identify branch, actual `planner.rs:510-530`): only handled
  `identify()` erroring (`UnidentifiableSource`); a donor that identified
  cleanly but whose container is unsupported silently resolved to nothing.
- **Failing test** (`planner_resolution.rs`,
  `unmuxable_donor_yields_unsupported_source_not_unidentifiable`): donor fixture
  `recognized:true, supported:false`; asserts `UnsupportedSource` present,
  `UnidentifiableSource` absent, plan nulled. Before the fix: no diagnostic at
  all (silent), plan present → the `plan.is_none()` assertion failed on `[]`.
- **Fix**: at the donor `Ok(di)` arm, apply the primary branch's exact
  container predicate (`!container_recognized || !container_supported`) →
  push `UnsupportedSource` + placeholder assignment + `continue`.
- **Scope note**: added ONLY the container predicate ("same predicate as
  :374"), not the primary's `format_version` skew warning — that is a separate
  concern the brief did not scope in.
- **Files**: `planner.rs`, `planner_resolution.rs`.

### (iii) print each filename once (human mode) — commit 0bce722
- **Anchor** (`commands/mod.rs` `print_batch_human`): each per-file diagnostic
  rendered via `renderer.diagnostic(d)`, which uses `diagnostic-line-file`
  (includes the file) — but the `dry-run-file` header already named the file,
  so it appeared twice per diagnostic line.
- **Failing test** (`commands/mod.rs` `#[cfg(test)]`,
  `per_file_diagnostics_do_not_repeat_the_filename_the_header_prints`): a file
  with a `for_file` diagnostic; asserts the full path appears exactly once.
- **Fix**: new `Renderer::diagnostic_no_file` (shares `diagnostic()`'s body via
  a private `render_diagnostic(d, show_file)`), always renders `diagnostic-line`
  (file-less). `print_batch_human` now delegates to a string-building
  `batch_human_report` (testable without capturing stdout); per-file
  diagnostics render file-less, batch-level diagnostics keep their file (no
  header precedes them). Output is byte-identical except the intended drop.
- **Files**: `i18n.rs`, `commands/mod.rs`.

### (iv) sort dry-run/run diagnostics errors-first — commit 747c968
- **Anchor** (`dry_run.rs`/`run.rs`/`mod.rs`): human output rendered
  diagnostics in emission order; `validate` sorts `Reverse(severity)`.
- **Failing test** (`commands/mod.rs`,
  `per_file_diagnostics_print_errors_before_warnings`): warning-then-error
  emitted; asserts the error line precedes the warning line. TDD-verified by
  temporarily bypassing the sort (test failed), then restoring.
- **Fix**: `severity_sorted(&[Diagnostic]) -> Vec<&Diagnostic>` (stable,
  error-first; `Severity` is `Info<Warning<Error`, so `Reverse` = error-first).
  Applied at every HUMAN render site: per-file and batch diagnostics in the
  report builder, plus the config-time set in both commands (main + not-found
  branches). JSON documents are deliberately untouched (they carry the
  `severity` field; consumers sort themselves) — see Concerns.
- **Files**: `commands/mod.rs`, `dry_run.rs`, `run.rs`.

### (vii) query-failed human path — commit bfbc493 — DETERMINATION: NOT deliberate → FIXED
- **Pre-check (10-min protocol) evidence read**:
  - Commit `3f66a4e` ("emit json document on ... list-languages failures"):
    "a `list_languages` failure printed nothing to stdout at all. Both now fold
    into the ... json document builders ... **human-mode output and exit codes
    are unchanged.**" — a SCOPING statement (F1 fixed only `--json`), not a
    design endorsement of stderr-only human mode.
  - Commit `9009d34` (sibling `mkvmerge_found` fix): likewise `--json`-scoped.
  - The code comment "human mode is unchanged (stderr only)" described F1's
    non-change, not a rationale.
  - `dry_run.rs`'s own doc comment states the superset-of-validate guarantee
    holds "**unconditionally**"; the sibling locate()-failure branch DOES print
    config diagnostics in human mode. The query-failed branch dropping them was
    an asymmetry within the same command, not intent.
  - Conclusion: NOT deliberate → fix (the brief's second outcome).
- **Failing tests**: the two existing human-mode tests
  (`dry_run_human_mode_still_just_reports_..._on_stderr`,
  `run_human_mode_still_just_reports_..._on_stderr`) pinned the old behavior
  but used a diagnostic-FREE profile, so they never exercised the guarantee.
  Rewritten to a config-warning profile (`match: {}` → EmptyMatchExpression)
  and renamed `..._surfaces_config_diagnostics_on_a_language_query_failure`,
  asserting the config diagnostic (`tracks[0].match`) reaches stdout and the
  query-failed line still hits stderr. Both failed against the old code (empty
  stdout).
- **Fix**: both `dry_run.rs` and `run.rs` query-failed branches now print
  `severity_sorted(&config_diags)` to stdout in human mode before the stderr
  message (never touching the queue), mirroring the locate()-failure branch.
  Obsolete "stderr only" comments updated.
- **Files**: `dry_run.rs`, `run.rs`, `dry_run_cli.rs`, `run_cli.rs`.

### (viii) IdentifyError English `detail` — commit 697dd70 — DETERMINATION: KEEP (deliberate pass-through)
- **Pre-check evidence**: spec §8.4 already accepts "third-party error text
  passed through as a `detail` param (regex, serde, I/O)". `IdentifyError`'s
  `Display` is core-authored English framing ("mkvmerge failed: ...",
  "invalid identification JSON: ...", "cannot read file: ...") wrapping
  inherently unstructured third-party text (mkvmerge subprocess stderr, serde,
  std::io), reaching the user only via the `detail` param of
  `UnidentifiableSource`, never as a `DiagCode`-templated message. Task-4 report
  established the project stance: raw, non-templatable third-party/developer
  text is a deliberate documented exception, never a catalog message. The
  payload is not catalog-templatable and v1 ships English-only, so
  catalog-routing every variant would be disproportionate. → KEEP (brief's
  first outcome).
- **Action**: added exactly ONE entry to the spec §8.4 "Accepted v1 exceptions"
  list naming the `IdentifyError::Display` framing pass-through; added a code
  comment at the `Display` impl documenting it as a deliberate §8.4 exception.
  No behavior change, no new test.
- **Spec self-contradiction check on §8.4**: PASS. The new entry sits inside the
  explicit "Accepted v1 exceptions" clause (the carve-out mechanism), sibling to
  the clap and detail-param entries; it does not contradict the "no hardcoded
  strings" rule (which the exceptions clause explicitly qualifies) nor the other
  §8.4 bullets (catalog source-of-truth, locale selection, English-only).
- **Files**: `docs/.../2026-07-08-muxsmith-v1-design.md`,
  `crates/muxsmith-core/src/identify.rs`.

## Gate results (nine parts, from worktree root, all GREEN)

1. `cargo fmt --all --check` — clean
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean
3. `cargo test --workspace` — all pass (no FAILED across every binary)
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — clean
5. `cargo deny check` — advisories/bans/licenses/sources ok
6. `pnpm lint` — clean
7. `pnpm build` — built
8. `pnpm check:i18n` — ok (175 catalog ids; 12 unused warnings are pre-existing
   GUI ids, unrelated to this task)
9. `pnpm test:e2e` — 3 passed

`node_modules` was missing → `pnpm install --frozen-lockfile` run once.

## Self-review

- Every item is its own commit with a failing-test-first flow; item (iv)'s test
  was additionally verified to fail without the fix via a temporary bypass.
- The exhaustive catalog fixture guard (`catalog_completeness.rs`) was updated
  in the same commit as every message/param change it covers (items i, v), and
  it stays green — confirming no `{$` leak from the reworded messages.
- Behavior-changing output edits (iii drop-file, iv sort, vii query-failed) were
  validated against the real CLI via the mkvmerge-backed integration suites, not
  only unit tests.
- The two pre-check items were resolved by reading the named history/comments;
  outcomes recorded above with the evidence (vii: FIXED; viii: KEPT).

## Concerns

- **Item (iv) JSON scope**: I sorted HUMAN output only. `validate` also sorts
  its JSON, but its JSON is a flat diagnostics list it fully owns;
  dry-run/run JSON is a structured per-file document where within-file order is
  a different question, and the documents carry the `severity` field for
  consumers to sort. If the reviewer wants JSON error-first too, that is a
  separate, larger change (into `report::json`) — flagged, not done.
- **Item (vi) attribution**: the donor `UnsupportedSource` uses
  `for_file(&primary.path)` and config_path `tracks[N].source.external`,
  mirroring the sibling donor diagnostics' convention (the primary
  `UnsupportedSource` message has no file/donor placeholder). So the message
  reads as if about the primary, disambiguated only by the config_path pointing
  at the external source. Consistent with existing siblings; noted in case a
  donor-naming param is later wanted.
- **Item (viii)**: spec line ~405 ("core is prose-free by construction", a
  different CI bullet) is in mild pre-existing tension with `IdentifyError`'s
  `Display`; that tension already existed under the original detail-param
  exception and line 405 is outside this item's "exactly one list entry" scope,
  so it was left untouched.
