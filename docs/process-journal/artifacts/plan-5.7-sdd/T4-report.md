# T4 report: NonUtf8Path guard at plan finalize (Stream D, D37)

- **Worktree:** `/home/senol/Git/Muxsmith/.worktrees/plan57-d` (branch `plan57-d`, base cd5e917)
- **Status:** COMPLETE, all gates green, committed (1cf10f9 impl + 6a056eb spec)

## Sources read (in order)

Plan Task 4; ADR D37; routed-items verdict item 2; conventions.yaml (core-31, core-37), product-boundaries.yaml/process-conventions.yaml (as referenced); spec 5.2 + 8.4 (+ 4.8, 4.2, 9.2, 6 for the sweep); core-31 code site (`planner.rs::render_output`); identify-side precedent (`capability/runtime.rs::identify_json`, the runtime.rs:206 rejection).

## Design decisions (inside D37's frame)

### Where the check sits

New pass `detect_non_utf8_paths(&mut files)` in `plan_core` (`crates/muxsmith-core/src/planner.rs`), inserted **after `detect_source_overwrites` and before the first `finalize_plans` call**. Rationale:

- Finalize is the single point where every argv-bound path exists (D37's own argument against profile-validate placement): output is rendered, chapters/attachment/track donors are resolved, all carried on the completed `Plan`.
- Emitting the error *before* the first `finalize_plans` reuses the standard drop mechanism (pattern-clone of how `SourceOverwrite` leads to a dropped plan) and keeps a poisoned plan out of `detect_output_collisions`: an output that can never be produced must not count toward two-planned-outputs collisions.
- After `detect_source_overwrites` rather than before: behaviorally equivalent (both only stack error diagnostics; neither drops), keeps existing diagnostic ordering for files that trip both.
- Rejected alternative: checking inside `resolve_file`/`render_output` per path source. The sources resolve in scattered places (output render, chapters locator, attachment locators, track-donor resolution); one pass over the finished `Plan` mirrors `command()`'s own enumeration and cannot miss a source added later to the `Plan` struct without the enumeration comment flagging it.

### Path-role inventory (every argv-bound path source found)

Enumeration mirrors `command.rs` exactly (all `display().to_string()` sites: `push_global` :100/:116/:121, `push_group` :136):

| Role param | Plan field | argv site (command.rs) | config_path used |
|---|---|---|---|
| `output` | `plan.output` | `--output` (push_global) | `output` |
| `chapters` | `plan.chapters` = `ChapterSource::External(p)` | `--chapters` (push_global) | `chapters` |
| `attachment` | each `plan.attachments.add_files[i]` | `--attach-file` (push_global) | `attachments` |
| `primary` | `plan.source` | group 0 `( source )` (push_group) | `input` |
| `donor` | each `plan.assignments[i].source` with `track_id.is_some()` and `source != plan.source` | donor group `( source )` (push_group) | `tracks` |

Deliberate boundary decisions:

- **`track_id: None` assignment sources are NOT guarded.** `command::input_groups` contributes no input group for a source whose assignments all have `track_id: None` (satisfied-optional / placeholder assignments), so such a path never reaches argv; rejecting it would drop valid plans. The coupling to `input_groups`' membership condition is recorded in a comment cross-referencing it.
- **Dedup: once per offending file within a plan** (BTreeSet by path; first role in argv order wins), per D37 "once per offending file". The same offending donor referenced by two different primaries yields one diagnostic per file report (each `for_file`), consistent with `SourceOverwrite`.
- **`--track-order`, track ids, attachment ids, title, tag flags**: not paths (or not path-derived strings); out of D37 scope. `TitleAction::Set` is a rendered template from UTF-8 profile text; not a path.
- **config_path** is the coarse profile-field anchor per role (table above), matching the sibling finalize passes (`SourceOverwrite` uses `output`, `EmptyPlan` uses `tracks`). Rule-index precision for donors was considered and rejected: the user-actionable datum is the on-disk path (carried in params), and no sibling finalize pass carries rule indices.

### Role param value set

`output` / `chapters` / `attachment` / `primary` / `donor`. The `primary`/`donor` split follows the established `UnsupportedSource` `kind=primary|donor` precedent. The Fluent message selects on `$role` (default branch `*[primary]`, same default choice as `unsupported-source`); `path` carries the lossy `display()` rendering, exactly what would have corrupted the argv.

### Reachability notes (why each role is guarded even where other layers narrow it)

- Real reachable today (verdict item 2): non-UTF-8 `--output-dir` override (`output`); chapters/attachment donors under a non-UTF-8-named subdirectory, since those donors are never identified (`chapters`, `attachment`).
- `primary`/`donor` (track sources) are identified, and the real runtime rejects non-UTF-8 at `identify_json` (runtime.rs:206) -> `UnidentifiableSource` first. Guarded anyway: the invariant is guarded, not the induction proof over current callers (core-31's recorded steelman rejection); injected `Identify` impls (tests, GUI simulation) bypass the runtime path.

## Deliverables

1. **Finalize-pass check**: `detect_non_utf8_paths` in `crates/muxsmith-core/src/planner.rs` (inserted into `plan_core` between `detect_source_overwrites` and the first `finalize_plans`; pass-sequence comment updated). `crates/muxsmith-core/src/report/mod.rs`: `DiagCode::NonUtf8Path => "non-utf8-path"` placed after `SourceOverwrite` in the planning-time group, rustdoc row in sibling style (condition, spec ref, params, identify-side counterpart). Wire key confirmed by the existing `all_keys_match_serde_encoding` test (serde kebab-case of `NonUtf8Path` = `non-utf8-path`).
2. **Catalog**: `locales/en/diagnostics.ftl` + `locales/de/diagnostics.ftl`, both in the impl commit, placed after `empty-rendered-name` (the invariant cluster it pattern-clones). Both select on `$role` with `*[primary]` default (the `unsupported-source` precedent); German with real umlauts ("angehängte", "gültiges", "unverfälscht"), register matched to neighbors ("Spender-Datei", "Quelldatei", "Ausgabepfad").
3. **Fixture coverage**: `crates/muxsmith-cli/tests/catalog_completeness.rs` `fixture_args` gains the `NonUtf8Path` arm (`role=donor` non-default branch exercised, comment documents the value set and the single-fixture-per-code limitation, matching the `UnsupportedSource` arm's style). The exhaustive match forced the arm as predicted.
4. **Spec**: 5.2 diagnostics-table row added after `SourceOverwrite` (own commit); sweep below.
5. **Test**: `crates/muxsmith-core/tests/planner_non_utf8_path.rs`, `#![cfg(unix)]` file-level gate per `executor_no_hang_live.rs`, paths built via `OsStrExt::from_bytes`. Two fns: (a) non-UTF-8 `--output-dir` override through `plan_batch`: asserts code, `Severity::Error`, `params[role]=="output"`, `params[path]` equals the exact lossy U+FFFD rendering, `config_path=="output"`, `file` set, exactly one diagnostic, `plan.is_none()` (no job built); (b) per-file isolation through `plan_core` with handcrafted primaries (bypassing both discovery's name-skip and the runtime's identify rejection, as an injected `Identify` would): offending primary dropped with `role=primary`, clean sibling plans untouched.

## Spec sweep (what was checked, result)

Checked every spec claim the new row could contradict:

- **D20 passthrough language** (4.5 track-order paragraph, 4.9 `attachments.unmatched` parenthesis, 5.2 `EmptyPlan` row): `EmptyPlan`'s "survived every finalize pass (no error-severity diagnostic, local or cross-file)" wording absorbs the new pass without edit; D20's "kept-but-unmatched counts as matched" makes no path claims. **No contradiction.**
- **Section 9.2 forward-compat wording** (item 2 of section 9): "the untyped path is opt-in and declared" uses "path" as code-path, about `raw:` properties; no filesystem-path claim. **No contradiction.**
- **Section 6**: "`command` is a pure function `Plan -> Vec<String>`" stays true; the guard is why it can stay infallible (ADR's rejected-alternative rationale). No text change needed.
- **Section 4.8**: "Two invariants are checked on the RENDERED name" stays accurate; `NonUtf8Path` checks whole paths (a template render is UTF-8 by construction from profile text; the offense enters via directory components/CLI override), not the rendered name. Not edited.
- **Sections 1/5.1/5.5/8.1** ("one mkvmerge invocation per source file", error-severity semantics, exit codes): already qualified by the pre-existing error-diagnostic mechanism the new code plugs into. **No contradiction.**
- Result: **zero self-contradictions created; no compensating spec edits needed** beyond the row itself.

## Gates (all foreground)

| Gate | Result |
|---|---|
| `cargo test -p muxsmith-core` | ok (incl. 2 new tests in `planner_non_utf8_path`) |
| `cargo test -p muxsmith-cli` | ok (catalog completeness + fixture leak check with new arm) |
| `cargo fmt --all --check` | ok (after one `cargo fmt` pass over the new test file) |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo test --workspace` | 37x "test result: ok", 0 failures |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | exit 0 |
| `cargo deny check` | advisories/bans/licenses/sources ok |
| `node scripts/check-i18n.mjs` (en/de parity) | ok (12 pre-existing gui-* unused warnings, unrelated) |

Frontend: no change needed; TS side keys diagnostics dynamically, `NUMERIC_DIAGNOSTIC_PARAMS` (`diagnosticFluentParams.ts`) lists neither `path` nor `role` (both are strings, correct).

## Commits (worktree, branch plan57-d)

- `1cf10f9` core: NonUtf8Path guard at plan finalize (D37) - reject argv-bound non-UTF-8 paths (6 files: planner, report, en+de ftl, fixture arm, test)
- `6a056eb` spec: 5.2 diagnostics-table row for NonUtf8Path (D37)

## Deviations surfaced

- **None against Tier-2 files.** core-37 (prose-free core): satisfied, code + params only. core-31: pattern-cloned including its steelman rationale (guard the invariant even where upstream layers currently narrow reachability).
- **Plan-text nuance, resolved in D37's favor:** the plan's Task 4 wording "every value that reaches command.rs argv" was applied literally: assignment sources with `track_id: None` do NOT reach argv (`command::input_groups` builds no group for them) and are deliberately not guarded; rejecting them would drop valid plans. Boundary recorded in the pass comment with a cross-reference to `input_groups`' membership condition. Reviewer hunting for missed sources: the five guarded roles cover all four `display().to_string()` sites in `command.rs` (:100 output, :116 chapters, :121 attach-file, :136 group source); `--track-order`/track ids/attachment ids are numeric, `--title` is rendered UTF-8 profile text, no other argv element is path-derived.
- The prescribed commit trailer (Co-Authored-By only, no session URL) was used as given, consistent with cross-project privacy for the public repo.

## Review-minor fixes (T4-verdict.md, post-approval)

- **M1**: `detect_output_collisions` header comment updated ("post the finalize pass that drops SourceOverwrite and NonUtf8Path errors"); the old text named only SourceOverwrite.
- **M2**: de catalog `[attachment]` branch "Die angehängte Datei" -> "Die anzuhängende Datei" (the file is yet to be attached).
- Separate fixup commit `8a8aabb` (reviewed commits left intact for readable history). Re-checked green: `cargo fmt --all --check`, `cargo test -p muxsmith-cli --test catalog_completeness` (4/4), `node scripts/check-i18n.mjs` ok, `cargo test -p muxsmith-core --test planner_non_utf8_path` (2/2).
