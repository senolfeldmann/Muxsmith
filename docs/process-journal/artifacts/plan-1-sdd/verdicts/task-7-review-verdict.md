<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 1 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01FiKAXfK4UqVuM4mU1ZVHQV
  agent_id:           a379436f841447550
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a379436f841447550.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-07T23:49:53.346Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance

**Missing:** None. All mandated interfaces are present and match exactly: `template::{Filter, Template, Ctx, TemplateError}`, `Template::parse/field_names/render_literal/render_regex_pattern`, `Ctx::new/set` with private `get` defaulting missing fields to `""`, all three `TemplateError` variants, `{{`/`}}` literal-brace escapes, all four filters (raw/int/pad2/pad3). The mandated staffel example (`staffel0*{season:int}episode0*{episode:int}`, season=03/episode=01) is present verbatim and, traced by hand, correctly matches `staffel03episode01`, `staffel3episode01`, `Staffel3Episode1` and rejects `staffel4episode01` given `(?i)` prefixing and the `0*` literal-regex-source passthrough.

**Extra:** None. `crates/muxsmith-core/src/template.rs` is a verbatim transcription of the code the brief itself supplies in Step 4/Step 2 — the only divergence from the brief's literal text is `c.set(*k, *v)` vs. the brief's `c.set(k, v)` in the test helper (a necessary fix for a real compile error the report describes; brief's `&&str` wouldn't satisfy `Into<String>`).

**Misunderstood:** Nothing outright wrong, but two spec-adjacent ambiguities are resolved in ways that are internally inconsistent or unverified (detailed under Issues). Both come from the plan's own provided code, not from implementer judgment, since the implementer did not deviate from the given implementation.

⚠️ Unverifiable without the full spec 4.7 text or Task 9's consumer code: whether `render_regex_pattern`'s unanchored `is_match` semantics (no implicit `^…$`) is intentional; whether `field_names()` is expected to deduplicate repeated field references; whether `TemplateError::pos` is meant to be a byte or char offset for downstream error display.

## Strengths

- Interface parity with the brief is exact, including method signatures and error-variant shapes.
- Parser uses `Vec<char>` indexing throughout (never slices the original `&str` by computed offsets), so it can't panic on UTF-8 char-boundary violations the way a naive byte-index parser would.
- Regex-mode correctly separates "literal template text is regex source" (unescaped) from "interpolated field values are literal data" (escaped via `regex::escape`), which is the crux of the dual-mode design and is exercised by a real test (`a.b(c)` vs. `aXb(c)`).
- No unwarranted abstraction: a single hand-rolled loop for a small, fixed grammar (four filters, two escapes, one field syntax) — appropriate for this scale, not over-engineered.

## Issues

#### Important

1. **Error-type precedence is order-dependent, not deliberate** (plan-mandated). In `Template::parse`'s filter match, an empty field *name* only reaches the `if name.is_empty()` check when the filter suffix is a recognized one (`int`/`pad2`/`pad3`). If the filter suffix is anything else — including simply empty — the wildcard arm returns early with `UnknownFilter` before the name is ever checked:
   ```rust
   Some((_, f)) => return Err(TemplateError::UnknownFilter { name: f.to_string() })
   ```
   So `{:int}` (empty name, known filter) correctly yields `EmptyField`, but `{:}` or `{:foo}` (empty name, unknown/empty filter) yield `UnknownFilter` instead — even though the name is equally empty in both cases. None of the brief's tests exercise this shape (only `{}`, `{season:frobnicate}`, `S{season` are tested), so it went uncaught.
   File: `crates/muxsmith-core/src/template.rs:199-210` (`Template::parse`).

2. **`pos` is a character index into an intermediate `Vec<char>`, not a byte offset into the original template string** (plan-mandated / design gap). Parsing itself is multibyte-safe (no slicing of the raw `&str`), but the position carried in `UnclosedBrace`/`EmptyField` is only meaningful against `text.chars().collect()`, not against `text` itself. If Task 9's error display later does the conventional thing (index/slice the original string by `pos`), any template with non-ASCII characters before the error position (accented titles, German umlauts — realistic here) will point at the wrong spot. No test uses non-ASCII template text, so this is unverified either way.
   File: `crates/muxsmith-core/src/template.rs:175-230` (`Template::parse`).

#### Minor

1. **Missing-field + `:int` filter breaks the "renders as empty string" contract** (plan-mandated). `Ctx::get` correctly returns `""` for a missing field, but `apply_filter(Filter::Int, "")` then falls into the all-zero-collapse branch (`trim_start_matches('0').is_empty() => "0"`) and returns `"0"` instead of `""`. The brief's own note ("validation prevents that from being reachable") suggests this path is expected to be dead in practice, so this is low-stakes, but it's an untested, silent deviation from the stated interface contract if that upstream validation is ever incomplete.
2. **Test-coverage gaps, all inherited verbatim from the brief's Step 2 code**: no test for `{name:}` (empty-filter-string) behavior specifically flagged for review; no test exercises an actually-missing context field (every `ctx()` call in every test sets every field the template references); no template in any test contains non-ASCII/multibyte characters despite that being an explicit review concern; error-variant tests use `matches!(_, Err(Variant{..}))` and never assert the payload (`pos`/`name` values), so a scrambled position or captured filter name wouldn't be caught.
3. **Report reliability**: task-7-report.md calls `regex::escape` "the standard library function" — it's from the third-party `regex` crate, not `std`. Combined with the controller-flagged wrong test-count (claimed 38, actual 32), this is a second data point that the report's self-review claims need independent verification, not restatement.
4. **`field_names()` doesn't deduplicate** repeated field references (`{x}...{x}` yields `["x","x"]`). Harmless given `Ctx` lookups are idempotent by name, but worth confirming Task 9's validation doesn't assume uniqueness. ⚠️ unverifiable without that code.

## Assessment

**Task quality:** Approved

**Reasoning:** Every interface, filter, escape rule, and the mandated staffel regex example is implemented correctly and matches the brief exactly (the code is essentially a verbatim transcription of the plan's own given implementation). The flagged Important items are real but narrow edge-case/ambiguity gaps inherited from the plan's code rather than implementer errors, worth a follow-up ticket before Task 9 builds error-display logic on top of `pos`, but not blocking for this task-scoped gate.