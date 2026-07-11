<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-4  (round 2 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01Me3MPNTa3wWH6qWhYSbPFu
  agent_id:           a3fac7ec74620cf89
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a3fac7ec74620cf89.jsonl
  dispatch_desc:      Review Task 4 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-07T23:28:09.552Z
  continuation_trigger: The coordinator sent a message while you were working: Your Important finding was fixed in
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

All six verification points check out against the fix diff.

1. **Newtype conversion** — `TemplateBlock { template: String }` and `ExternalBlock { external: Locator }` both carry the full derive set plus `#[serde(deny_unknown_fields)]`. All four enums converted: `FilenameCfg::Template(TemplateBlock)`, `TitleCfg::Template(TemplateBlock)`, `SourceCfg::External(ExternalBlock)`, `ChaptersCfg::External(ExternalBlock)`. In every enum the struct-carrying variant remains declared before `Keyword(String)`. Sharing one block struct across two enums is correct here, not premature abstraction: the wire shapes are genuinely identical by spec. Wire format is unaffected (untagged newtype-over-struct serializes identically to the former inline variant), and the generated JSON Schema improves as a side effect (`additionalProperties: false` now actually emitted for these blocks).
2. **Helpers/Defaults intact** — `FilenameCfg::keep()`, `SourceCfg::primary()`, and the `Default` impls for `OutputCfg`/`ChaptersCfg`/`TitleCfg` are unchanged (visible in the diff's context lines).
3. **Regression tests hit the previously-silent paths** — `unknown_key_inside_filename_template_is_rejected` places `extra: 1` inside `filename: { template: ... }` (the exact `FilenameCfg::Template` bypass); `unknown_key_inside_source_external_is_rejected` places `bogus: true` inside `source: { external: ... }` (the exact `SourceCfg::External` bypass). Both inputs would have parsed successfully at `3ab1cdc`, so these are genuine regression tests: the map now fails the block struct on the unknown key and fails `Keyword(String)` on type, so the whole untagged enum errors.
4. **Defaults test and `at` param** — `defaults_apply_when_sections_absent` now additionally asserts `on_collision == Error`, `chapters == "keep"`, `title == "keep"`, `tags.track == Keep`, `attachments.unmatched == Keep`; `unknown_key_is_parse_error_with_path` now asserts `err.params.contains_key("at")`.
5. **Duplicate dep removed** — the `[dev-dependencies]` section is gone entirely; `serde_json` lives only under `[dependencies]`. No `Cargo.lock` change needed since the resolved dependency set is identical.
6. **Nothing else changed** — three files touched; the only other edit is the mechanically required destructuring update in `reference_profile_parses` (`SourceCfg::External(block)` / `block.external.*`) plus the `CollisionPolicy` import. No non-ASCII introduced, no scope creep.

### Strengths

- The fix implements exactly the remedy named in the review (extracted named structs, newtype variants) with correct rationale documented in the doc comments, including *why* the structs are standalone — the non-obvious serde limitation is now recorded at the point where a future refactor would otherwise inline them again.
- The two regression tests target precisely the inputs that demonstrated the bug, so this class of silent acceptance is now locked in, and via the shared blocks the coverage extends transitively to `TitleCfg` and `ChaptersCfg`.

### Issues

#### Critical

None.

#### Important

None.

#### Minor

- The two new regression tests assert only `is_err()`, not `code == DiagCode::ParseError` or the `at`/`detail` params. In practice `from_str` can only return a `ParseError` diagnostic, so this is not a correctness gap; and untagged-enum failures produce serde's generic "did not match any variant" detail anyway, so param assertions would be brittle. Acceptable as is.
- Residual coverage gaps from the original review remain (by design, since the fix was scoped to the Important finding): `Locator.recursive`/`case_sensitive` defaults untested (the reference fixture's external locator omits both and could assert them for free), and `reference_profile_parses` still checks nothing about the six subtitle rules beyond the count. Carry-over observations, not defects of this fix.

### Assessment

**Task quality:** Approved

**Reasoning:** The fix resolves the Important finding exactly as prescribed — effective `deny_unknown_fields` on all profile shapes including the untagged blocks, verified by targeted regression tests — cleans up both Minor items it was asked to (duplicate dev-dep, defaults/`at` assertions), and changes nothing beyond that scope.