<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-5  (round 1 of 1)
  session_uuid:       ffc7c915-ea2a-432b-9ffd-64ac313a2a6f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f.jsonl
  tool_use_id:        toolu_019kKNi1oXqbWEpt5ZLw5Sea
  agent_id:           a2ace266f331b28d7
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f/subagents/agent-a2ace266f331b28d7.jsonl
  dispatch_desc:      Review Task 5 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T17:45:53.435Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance Verdict: PASS — full compliance, no defects blocking acceptance.

I verified this by reading the current post-diff `matcher.rs` (lines 135-169, 268-282), the spec file (lines 125-147), and cross-checking the implementer's canonicalize() claims against the actual crate source at `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/language-tags-0.3.2/src/lib.rs:485-612`.

### Spec Compliance

| Requirement | Status |
|---|---|
| `normalize` (ISO) arm unchanged, runs first | Confirmed. `if let (Some(na), Some(nb)) = (lang.normalize(a), lang.normalize(b)) { return na == nb; }` is byte-for-byte the same predicate as the pre-diff `match` arm, just restructured to early-return. |
| `canonical_tag` helper matches spec exactly | Confirmed. `LanguageTag::parse(s).ok()?.canonicalize().ok().map(\|t\| t.as_str().to_string())` — verbatim. |
| Canonical comparison case-insensitive | Confirmed. `ca.eq_ignore_ascii_case(&cb)`. |
| Raw case-insensitive fallback when a value doesn't canonicalize | Confirmed. `a.eq_ignore_ascii_case(b)` is reached only when `canonical_tag` returns `None` for either side. |
| `pt-Latn-BR`==`pt-BR`, `pt-BR`!=`pt-PT`, `zh-Hans`!=`zh-Hant` | Confirmed against crate source: `canonicalize()` applies Suppress-Script only via `LANGUAGES_SUPPRESS_SCRIPT` (`lib.rs:551-558`, `iana_registry.rs:8695`, a 134-entry table); Portuguese has a Suppress-Script=Latn entry, Chinese does not, so script distinctions on `zh` survive while the redundant `Latn` on `pt` is dropped. Region is never suppressed (`lib.rs:562-569`), so `pt-BR`/`pt-PT` stay distinct. |
| `iw`==`he` (nice-to-have) | Confirmed. `canonicalize()`'s primary-language substitution via `LANGUAGES_PREFERRED_VALUE` (`lib.rs:512-517`) covers deprecated-code replacement; the crate does carry it. |
| Doc-comment correction (`xx-YY` canonicalizes unchanged, not `None`) | Confirmed accurate. The **only** `Err` return in `canonicalize()` is `MultipleExtendedLanguageSubtags` at `lib.rs:525`; every registry lookup (`find_in_from_str_slice_map`) falls back to `.unwrap_or(original)` when a subtag isn't in the table, so an unregistered region/script like `YY` passes through unchanged. The corrected comment at `matcher.rs:157-162` states exactly this. The brief's own draft ("well-formed-but-invalid tags ... return `None`") was wrong; the correction is warranted, not a deviation. |
| Spec 4.3 `exact` paragraph | Confirmed present at `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:146`, placed correctly at the end of section 4.3 (before 4.4 starts), wording matches the brief verbatim. |
| Typography (ASCII only) | Confirmed. `grep -P '[—–''""…\xA0]'` over the diff: zero hits. |

⚠️ Full-suite/clippy/deny green — not independently re-run per instructions; relying on the controller's report.

### Strengths

- The `canonical_tag` doc-comment correction is a genuine improvement: it's backed by reading the crate's actual control flow (`MultipleExtendedLanguageSubtags` is provably the only error path), not an assumption, and it doesn't just fix the example, it explains the general reason (registry lookups no-op on unknown subtags rather than erroring).
- The Step 1 mkvmerge-normalization comment is honest about impact: it correctly frames the canonicalization path as mostly earning its keep on non-mkvmerge-authored/older files, not overstating the fix's day-to-day hit rate.
- Report transparently flags the one pre-existing test (`language_falls_back_to_raw_compare_when_unknown`) whose name is now slightly imprecise (its `zxx`/`qqq` inputs now traverse `canonical_tag`, not the literal raw-compare path) instead of silently leaving a stale claim uncommented.
- RED/GREEN was actually performed (reverted to old body, confirmed `lang_eq_canonical_forms_match` fails while `lang_eq_preserves_meaningful_distinctions` passes even pre-fix), and this asymmetry is reported rather than glossed over.

### Issues

**Critical:** none.

**Important:** none.

**Minor:**
- `crates/muxsmith-core/src/matcher.rs:276-282` — `lang_eq_preserves_meaningful_distinctions` passes identically under the pre-fix raw-fallback implementation (confirmed by the implementer's own RED run): its three pairs (`pt-BR`/`pt-PT`, `zh-Hans`/`zh-Hant`, `pt-BR`/`pt`) are string-unequal even case-insensitively, so it doesn't discriminate old from new behavior. This is inherited verbatim from the task brief's Step 2, not an implementer choice, and it does still exercise the new `canonical_tag` code path (just with a coincidentally identical outcome) as a regression guard against a future canonicalization change that would wrongly merge these. Non-blocking; flagging per the review rubric's instruction to name any test that would pass regardless of the fix.

### Task Quality Verdict

Solid. Implementation matches the brief's specified code exactly, the one deliberate deviation (doc-comment wording) is independently verified as correct against the crate source, all required test assertions check out against real `canonicalize()` behavior, and the spec paragraph is placed and worded correctly. No rework needed.