<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-4  (round 1 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01Me3MPNTa3wWH6qWhYSbPFu
  agent_id:           a3fac7ec74620cf89
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a3fac7ec74620cf89.jsonl
  dispatch_desc:      Review Task 4 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-07T23:23:03.072Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance

**Missing**
- `#[serde(deny_unknown_fields)]` is not effective on the four untagged-enum struct variants — `FilenameCfg::Template`, `TitleCfg::Template`, `SourceCfg::External`, `ChaptersCfg::External`. Serde's `deny_unknown_fields` is a container attribute; it is not a recognized variant attribute and cannot be attached to an inline `Variant { field: T }` shape (confirmed against serde's own attribute docs and the standard workaround, which requires extracting the variant body into a separately-declared named struct carrying the attribute and wrapping it as a newtype variant). Net effect: `output: { filename: { template: "x", oops: 1 } }` or `tracks: [{ source: { external: {...}, bogus: true }, match: {...} }]` parses successfully today, silently dropping the unrecognized key. This contradicts spec 4.1 ("Unknown keys are errors, not warnings") and the controller's explicit binding constraint ("deny_unknown_fields on every profile struct"). **Plan-mandated**: the brief's own Step 5 code sample has this identical omission; the implementer copied it verbatim rather than introducing the gap.

**Extra:** none — no scope creep, no unrequested types or fields.

**Misunderstood:** none — model, loader, and fixture all track the brief's literal interface faithfully.

⚠️ **Unverifiable without compiling:** whether the reported 16 tests actually execute as claimed. Arithmetic is self-consistent (`match_expr.rs`: 5 tests + `report.rs`: 7 tests + `profile_load.rs`: 4 tests = 16, `muxsmith-cli` has no tests), so the count is plausible, but I did not run `cargo test`.

Everything else checks out against the binding list:
- All 9 named profile structs (`Profile`, `Meta`, `Input`, `OutputCfg`, `TrackRule`, `Locator`, `AttachmentsCfg`, `AttachmentRule`, `TagsCfg`) correctly carry `deny_unknown_fields`.
- All four untagged enums declare the struct variant before `Keyword(String)`.
- Every default in the controller's list is implemented correctly and consistently (field-level `#[serde(default = "...")]` matches the corresponding `impl Default` where a whole-struct default is also needed): `Input.recursive=true`, `TrackRule.optional=false`, `CollisionPolicy::default()=Error`, `OutputCfg.filename` default `keep`, `ChaptersCfg::default()="keep"`, `TitleCfg::default()="keep"`, `TagsCfg` both `Keep`, `AttachmentsCfg.unmatched=Keep`, `Locator.recursive=false`, `Locator.case_sensitive=false`, `TrackRule.source=primary`.
- `crates/muxsmith-core/tests/fixtures/reference.yaml` is byte-for-byte identical to spec section 4.1's example (verified against `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` lines 53-115), with the German block correctly mirroring the English one (`language: de`, `track_name` values `German forced` / `German` / `German SDH`, same `not`/`any`/`forced_track` structure) and `directory: null` present (it was already `null` in the spec's own example). The 10 tracks are in the exact required order: video, audio en, audio de, sub en forced, sub en plain, sub en SDH, sub de forced, sub de plain, sub de SDH, external srt tr.
- YAML crate correctly moved to `[dependencies]` as `yaml_serde`.
- Loader maps `serde_path_to_error::Error` to `Diagnostic { code: ParseError, params: {detail, at} }` correctly, for both YAML and JSON.
- ASCII hygiene: the only non-ASCII byte in the entire diff is the legitimate fixture value `"Türkçe"`; no em-dashes/curly quotes anywhere.

## Strengths

- Model types are a faithful, minimal mirror of the file format; semantic validation is correctly deferred to a future `validate.rs` (explicit doc comment), keeping this task's scope honest.
- Fixture is provably correct against the spec text, not just "looks plausible" — direct comparison confirms an exact match plus correct German mirroring.
- Defaults are implemented consistently at both the field level (`#[serde(default = "...")]`) and the whole-struct level (`impl Default`), so the same default value can't drift between "section present but field omitted" and "whole section omitted" — a detail easy to get inconsistent and here it isn't.
- `json_profile_parses_identically_to_yaml` is a real cross-format equality check (`PartialEq` on the whole parsed struct), not a smoke test.

## Issues

#### Important

- **Untagged-enum struct variants bypass `deny_unknown_fields`** (detailed above under Missing). Affects `crates/muxsmith-core/src/profile/model.rs`: `FilenameCfg`, `TitleCfg`, `SourceCfg`, `ChaptersCfg`. Fix requires extracting `Template`/`External` bodies into named structs annotated with `#[serde(deny_unknown_fields)]` and wrapping them as newtype variants (wire format is unaffected — a newtype variant wrapping a struct serializes identically to an inline struct variant). Labeled plan-mandated: present verbatim in the brief's Step 5 code, not introduced by the implementer, and not caught by any of the 4 tests (none probes unknown keys nested inside these four blocks).

#### Minor

- `crates/muxsmith-core/Cargo.toml`: `serde_json` is now declared in both `[dependencies]` and `[dev-dependencies]` with the identical version, left over from running `cargo add ... serde_json` without removing the pre-existing dev-only line. Harmless, but dead duplication that should collapse to the one `[dependencies]` entry.
- Test coverage gaps (all plan-mandated, inherited verbatim from the brief's Step 3 code):
  - `defaults_apply_when_sections_absent` checks only 4 of the ~11 defaults in the binding list (skips `on_collision`, `chapters`, `title`, `attachments.unmatched`, `tags.track`, `Locator.recursive`/`case_sensitive`).
  - `unknown_key_is_parse_error_with_path` asserts only `params["detail"]`, never `params["at"]`, despite the interface explicitly requiring both params (the implementation does set `at` correctly — just untested).
  - `reference_profile_parses` asserts nothing about `tracks[3..9]` (the six subtitle rules) beyond the total count of 10; a swapped `forced_track` or mismatched `en`/`de` block in the fixture would not fail any test today. The fixture was verified correct here by direct text comparison, but the suite doesn't lock that in for future edits.

## Assessment

**Task quality:** Needs fixes

**Reasoning:** The model, loader, and fixture are faithful, correctly-ordered, and default-complete implementations of the brief, but the deliverable does not actually satisfy the binding "deny_unknown_fields on every profile struct" constraint for the four untagged-enum struct variants — a real, demonstrable silent-acceptance gap that traces back to the brief's own suggested code rather than an implementer error.