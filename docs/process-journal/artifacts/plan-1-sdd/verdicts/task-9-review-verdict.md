<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-9  (round 1 of 1)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01RYQqxTUToQT9Q3CLzg1PZG
  agent_id:           a0667516f45d839c9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a0667516f45d839c9.jsonl
  dispatch_desc:      Review Task 9 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-08T00:08:57.549Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

All four global-constraint areas were traced against the diff and cross-checked against `model.rs` / `template.rs` / `report.rs`, not just the report's prose:

- **Numbered group derivation** (`crates/muxsmith-core/src/profile/validate.rs:34-42`): `capture_names().enumerate()`, skips index 0 (regex crate's implicit whole-match group), pushes `g{i}` for every group and additionally the name for named ones. Matches regex crate semantics (`capture_names()` yields `None` for group 0 first, then one item per group in index order) and the spec's "{match} + g1..gN + named groups."
- **`source_stem` scoping**: base `template_fields` never contains it; appended only for `output.filename.template` (line 108) and `title.template` (line 145); the `match_pattern` path in `validate_locator` (line 333-343) passes the un-augmented `template_fields`, so `{source_stem}` in a `match_pattern` correctly falls through to `UnknownTemplateField`. Verified by the `match_pattern_may_not_use_source_stem` test, which is one of the 10 green tests.
- **Path-separator ban**: `forbid_path_separators=true` only at the `output.filename.template` call site (line 113); `false` at `title.template` (line 150) and hardcoded `false` inside `validate_locator`'s `match_pattern` call (line 341). `chapters` has no template variant at all (`ChaptersCfg` is `External | Keyword` only per `model.rs:199`), so there's nothing to (mis)apply the ban to there. Correct.
- **`g1`/`g2` test actually exercises validation**: traced by hand — pattern `S(\d{2})E(\d{2})` (unnamed groups) yields `template_fields = ["match","g1","g2"]`, template `S{g1}E{g2}.mkv` field_names → `["g1","g2"]`, both allowed. Confirmed by running `cargo test -p muxsmith-core --test validate_structure` myself: 10/10 pass, and `cargo test --workspace`: 58/58 pass (matches the report's claim, independently reproduced).
- **InvalidRegex flattening**: `flatten_regex_error` (line 159-164) is defined exactly as the authorized adaptation states (`split_whitespace().join(" ")`) and is used at both the new `input.pattern` site (line 31) and the pre-existing Task-8 condition-regex site inside `validate_expr` (line 250). No third regex-error site exists in the diff.
- **Newtype-block adaptation**: `SourceCfg::External(block)`/`.external`, `FilenameCfg::Template(block)`/`.template`, `ChaptersCfg::External(block)`/`.external`, `TitleCfg::Template(block)`/`.template` — all four match the actual enum shapes in `model.rs:94-239` exactly.
- **Keyword rules**: source→`primary`, filename→`keep`, chapters→`keep`/`drop`, title→`keep`/`clear`, all with `found`/`allowed` params on the `InvalidKeyword` branch — all four present and correct.
- **Locator XOR / EmptyExtensions**: `validate_locator` checks `match_to_source.is_some() && match_pattern.is_some()` → `LocatorConflict` (this is what the brief's own reference code does too — there's no "at least one required" rule per spec 4.6, so "both set" is the entire conflict condition, not a bug); `EmptyExtensions` fires for both `input.extensions` (line 46-51) and every `Locator.extensions` via `validate_locator` (line 321-326), reached from all four locator call sites (source/chapters/attachment-add).
- **Template error mapping**: `UnknownFilter→UnknownTemplateFilter`, `UnclosedBrace`/`EmptyField→InvalidTemplate`, unknown fields→`UnknownTemplateField` with `field`+`allowed` params — all as specified.
- **ASCII**: `grep -nP '[^\x00-\x7F]'` over both changed files returns nothing.
- **`validate_expr` left unchanged** except swapping `e.to_string()` for `flatten_regex_error(&e)` — confirmed, matches the authorized adaptation #3.
- **Dropped `_reserved: bool`**: confirmed absent from `validate_locator`'s signature (4 params, not 5) and from every call site.

No missing checks, no extra/unrequested checks, nothing misunderstood against the four global constraint blocks or the three controller adaptations.

### Strengths

- Every one of the four controller-authorized adaptations was applied precisely, not approximately — verified line-by-line against `model.rs`, not just taken on the report's word.
- `flatten_regex_error` is a genuine one-line-by-construction guarantee (`split_whitespace().join(" ")` cannot produce embedded newlines regardless of `regex::Error`'s multi-line caret-art Display), applied uniformly at both sites rather than special-cased.
- The clippy self-review is accurate: I independently ran `cargo clippy -p muxsmith-core --all-targets` and got exactly the one `collapsible_if` warning, at the same line (246), inside the untouched `if kind == "regex" { if let Err(e) = ... }` nesting from Task 8; the diff only replaced the `.with("detail", ...)` argument there. Correctly identified as pre-existing, not a new construct from this diff.
- `source_stem` inclusion/exclusion and the path-separator ban are each applied at exactly one call site per field, with an inline comment at the one place (`match_pattern`) where the omission could look like an oversight instead of a deliberate spec-driven exclusion.

### Issues

#### Minor

- **Misleading test name / coverage gap, plan-mandated**: `empty_extensions_flagged_for_input_and_locator` (`crates/muxsmith-core/tests/validate_structure.rs:41-45`) only exercises `input.extensions` being empty (`BASE` has no `source.external`/attachment-add locator in this test) — it never constructs a `Locator` with empty `extensions` to prove the *locator* half of the `EmptyExtensions` check fires. This is copied verbatim from the brief's Step 1, so it's inherited, not an implementer defect; the implementation itself is correct (`validate_locator` line 321-326 does check `locator.extensions.is_empty()`, and this is reachable from three call sites), just untested by this suite. Low risk since the code path is straightforward and structurally identical to the already-tested input case.
- Similarly plan-mandated: `unknown_keywords_are_flagged` exercises `chapters`/`title`/`source` keyword misuse but not `output.filename`'s keyword misuse (e.g. `filename: wipe`); again inherited from the brief, and the four keyword-check blocks are structurally parallel enough that this is low-risk, not a real gap in confidence.

No Critical or Important issues found.

### Assessment

**Task quality:** Approved
**Reasoning:** All spec-mandated checks and all three controller adaptations are correctly and precisely implemented, verified independently (re-running the test suite and clippy, and tracing the capture-group/field-scoping logic by hand rather than trusting the report). The only gaps are two brief-authored test-coverage omissions, explicitly plan-mandated and low-risk, not implementer defects.