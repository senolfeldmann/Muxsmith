<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 1 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01FnTcFSSSb13PueBtCZzqzS
  agent_id:           a810b23dffefd05a9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a810b23dffefd05a9.jsonl
  dispatch_desc:      Final whole-branch review Plan 1
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-08T00:52:20.354Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Verdict
**Not ready — one exit criterion ("no user-facing prose in muxsmith-core") fails on three literals in validate.rs; the fix is small and local, plus one catalog-naming decision must land before the diagnostic catalog is treated as frozen. Everything else is minor.**

Independently verified: 76 tests green at c7a70f7, tree clean, catalog complete (30 codes = 30 `.ftl` entries, guard iterates `DiagCode::ALL`), every emitter's param names match the catalog, `config_path` format is uniform across parse errors (`serde_path_to_error` yields `tracks[0].optionall`), validate (`tracks[0].match.exact.foo`) and lint (`tracks[N]`), exit codes match spec 8.1, model defaults match spec 4.2/4.6, SETTABLE matches the spec 4.4 table row-for-row, `generated.rs` committed with no upstream schema in tree (mini-schema.json is synthetic).

### Cross-task findings

#### Critical
None.

#### Important

1. **Core emits English prose in template-error params — fails the plan's own exit criterion and spec 8.4.** `crates/muxsmith-core/src/profile/validate.rs:360,364,367`: `format!("unknown filter: {name}")`, `format!("unclosed brace at {pos}")`, `format!("empty field at {pos}")`. The grep gate the exit criteria mandate finds exactly these. Visible symptom today: `locales/en/diagnostics.ftl:19` renders "Unknown template filter: unknown filter: frobnicate" (double prose); under any future locale the inner fragment stays English. This is plan-specified (plan lines 2380-2386), i.e. the plan conflicts with its own Global Constraint — per that constraint the spec wins. Fix: `UnknownTemplateFilter` gets param `name` (`unknown-template-filter = Unknown template filter "{ $name }".`); `InvalidTemplate` gets params `kind` (`unclosed-brace`/`empty-field`, a code-like token, not prose) + `pos`, catalog uses a Fluent selector on `$kind`. The regex/serde/io `detail` params stay (third-party error text, covered by the authorized InvalidRegex deviation).

2. **`DiagCode::UnknownProperty` repurposes a spec-defined code name.** Spec 5.2's table defines `UnknownProperty` as the planning-time version-skew **warning** (section 9.2); the implementation uses `unknown-property` for the config-time typo **error** (`report.rs:45`) and invents `unknown-property-skew` (`report.rs:69`) for the spec's case. A `--json` consumer coded against the spec table gets the wrong meaning and severity. Both conditions deserve distinct codes — the code is right, the spec table is now wrong. Decide before catalog freeze: amend the spec 5.2 table (recommended: rename its row to `UnknownPropertySkew`, add a config-time `UnknownProperty` error row) or rename the codes. One-line spec edit; do it now, renaming a "stable" catalog key in Plan 2 is exactly what the stability decision was meant to prevent.

3. **`match_to_source: false` produces a spurious `LocatorConflict`.** `validate.rs:327` checks `is_some()`, but spec 4.6 types the field as `match_to_source?: true` — `false` means "not in use". Concrete failure: `{ match_to_source: false, match_pattern: '{match}' }` → error that the two are mutually exclusive, though only one is active; `match_to_source: false` alone silently validates and Plan 2 has no defined semantics for it. Fix: conflict only on `matches!(locator.match_to_source, Some(true))`, and reject `Some(false)` explicitly (InvalidKeyword-style, allowed: `true`). Plan-specified code, but a real defect.

#### Minor

4. **`parse-error` renders the path twice and the IO-error case renders it empty.** `diagnostics.ftl:6` interpolates `$at` while `diagnostic-line` already prints `config_path` (same string) → `[error] tracks[0].x: The profile could not be parsed at "tracks[0].x": ...`. For a missing file (`load.rs:33-38`) config_path is `""` → `[error] : The profile could not be parsed at "": ...`, and the `file` field is only visible in `--json` (text renderer ignores it). Drop `$at` from the message; optionally teach `diagnostic-line` to print `file` when present.
5. **clap help strings are hardcoded English** (`cli.rs:14,17,20`), against a strict reading of spec 8.4 ("not in the CLI"). Plan-specified. Recording `--help` output as an accepted v1 exception in spec 8.4 is the proportionate fix; routing clap help through Fluent is not worth it now.
6. **`validate-summary` uses "error(s)"** (`cli.ftl:2`) instead of Fluent plural selectors. Fine for English-only v1; convert when the first real second locale lands.
7. **`muxsmith schema` cannot express keyword domains.** Untagged `FilenameCfg`/`ChaptersCfg`/`TitleCfg` schematize as `anyOf [object, string]`; "keep"/"drop"/"clear" live only in validate.rs. A GUI (Plan 4) generating an editor from the schema won't know the allowed keywords. Consider `schemars(schema_with)` overrides later; no action now.
8. **CLI tests couple to catalog wording** (`cli_validate.rs` contains "Profile is valid.", "provably overlap"). Acceptable as behavior locks; spec 10's insta snapshots will replace this class of assertion — don't grow the pattern.

### Minor-findings triage (1-8)

1. **Fix now** — 2 lines; `DiagCode` grows in Plan 2 and the guard exists precisely for copy-paste key collisions; make it name the duplicate.
2. **Fix now** — cheap content asserts lock the recursive-parse semantics spec 10 calls the correctness core.
3. **Partial fix now** — assert the two locator defaults (`recursive`/`case_sensitive` = false are load-bearing for Plan 2 discovery); skip deep-asserting the six subtitle rules (redundant with `reference_profile_validates_clean`, pure churn).
4. **Defer** — untriggerable with schema v20; `generated.rs` changes arrive as human-reviewed diffs. Revisit only if a pinned schema ever nests those keys.
5. **Split** — extend the SETTABLE test to lock all 10 rows now (the option strings feed Plan 3 argv generation; a typo there is invisible until integration tests). Rustdoc: defer to Plan 2's capability work (runtime queries land in that module anyway).
6. **Defer, folded into Important #1** — the template-error payloads get restructured there; write the payload asserts against the new params.
7. **Fix now** — the locator `EmptyExtensions` branch has zero coverage while the test at `validate_structure.rs:27` claims "input **and locator**" in its name; add the locator case and a filename-keyword-misuse case.
8. **Defer** — the test already JSON-parses the output; pointer-based asserts are cosmetic hardening.

### Plan 2 handoff notes

- Spec 4.8 demands the path-separator check on the **rendered** filename; `validate.rs:399` only inspects raw template text. The planner must re-check post-render (field values can contain `/`).
- `Locator.path` is `String` while `output.directory` is `PathBuf` (`model.rs`); unify before file discovery consumes both, or Plan 2 inherits the inconsistency into its public API.
- `matchable_type("codec_kind")` returns `String` (`capability/mod.rs:19`), so `substring`/`regex` on the virtual alias validates clean — but prefix-set semantics for substring matching are undefined. Either define them in the matcher or restrict `codec_kind` to `exact` in validate.
- `exact: { type: vdieo }` passes validation — property **values** have no domain checks. Decide in Plan 2 whether known-domain values (type, language via `--list-languages`) get a static or plan-time lint.
- The 12 planning-time catalog entries carry params (`$count`, `$path`, `$track`...) no emitter exercises yet; `Renderer::msg` silently renders `{$x}` on drift. Add per-code param fixtures to the completeness guard when Plan 2 starts emitting them.
- `Template::parse` treats an unmatched single `}` as literal; lock that with a test before the GUI profile editor round-trips templates.