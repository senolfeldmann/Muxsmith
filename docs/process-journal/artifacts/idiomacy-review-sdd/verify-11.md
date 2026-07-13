# Verify-11: template.rs Vec<char> index-walk parser (slice F1a)

**Verdict: CONFIRMED**

Finding: `crates/muxsmith-core/src/template.rs:92` - `Template::parse` collects into `Vec<char>` and index-walks; replacement `Peekable<Chars>` keeping the char-offset `pos` contract.

## (a) Code says what the finding claims - yes

Read at HEAD (2f17880). Line 92 is exactly `let chars: Vec<char> = text.chars().collect();`, followed by `let mut i = 0; while i < chars.len()` with `chars[i]`, `chars.get(i + 1)` lookahead-1, and a `chars[i + 1..].iter().position(|&c| c == '}')` forward scan whose `inner` is re-collected into a `String`. The claimed shape is present verbatim.

The causal claim about the footgun contract also holds: `TemplateError`'s doc (lines 28-30) documents `pos` as a CHARACTER offset precisely because the walk indexes a char sequence. The journal confirms the contract was a documentation-only resolution of that hazard (`docs/process-journal/artifacts/plan-1-sdd/verdicts/task-7-review-verdict-round-2.md` item 3; carried in `progress.md`: "TemplateError::pos is a CHAR offset (never byte-slice)").

## (b) Replacement is current idiom - yes

- Stdlib APIs (`str::chars`, `Peekable::peek`/`next_if`, `char_indices`) are stable, current, and unchanged on Rust 1.96 / edition 2024 (confirmed via context7 against rust-lang/rust; Peekable and char_indices appear in current release notes as actively maintained, no deprecation).
- Ecosystem norm confirmed via live search, not training memory: iterator-based scanning (`chars().peekable()`, `char_indices().peekable()`) is the documented idiomatic shape for hand-written lexers/parsers; collecting into `Vec<char>` is explicitly called out as the less idiomatic transplant. Sources:
  - https://users.rust-lang.org/t/idiomatic-way-to-read-chars/97262
  - https://brunocalza.me/2023/09/20/writing-a-simple-lexer-in-rust.html
  - https://petermalmgren.com/token-scanning-with-rust/
  - https://www.rustfaq.org/en/how-to-build-a-custom-parser-in-rust/
- Mechanical feasibility checked against the actual parser: lookahead is strictly 1 (`{{`, `}}` - `peek()` covers it), the field scan is strictly forward consume-until-`}` (no backtracking, no random access), and `pos` needs only a manual char counter incremented per consumed char. `UnclosedBrace` falls out when the iterator exhausts mid-field. D5 semantics (lone `}` is literal) survive unchanged.
- Nuance, not a refutation: `char_indices()` with byte offsets would be marginally more idiomatic still (zero-copy `&text[a..b]` for `inner`, byte offsets in errors) but would invert the journal-recorded char-offset contract. The finding's choice of `Peekable<Chars>` + counter is the contract-compatible idiom and is the right call given the recorded contract.

## (c) Duplication claim - n/a

No duplication is claimed.

## (d) yagni completeness - n/a

Tag is `idiom`, and concrete construct + concrete replacement are named anyway.

## Decision guard - no conflict, not separately tracked

- `docs/superpowers/specs/*.md`: `template.rs` appears only as grounding context in the plan-2/plan-3 memo headers. No D-memo covers the parser's implementation shape. D5 (lone `}` literal) is behavioral and preserved by the replacement.
- `docs/IDEAS.md`, `docs/ROADMAP.md`: no entry for this construct. The whole-codebase idiomacy review entry in ROADMAP is the umbrella under which this finding was produced; its named-inputs list does not include template.rs, so the finding is new output of the tracked review, not an already-tracked item.
- The char-offset `pos` contract IS a recorded resolution (task-7 review round 2, journal). The finding explicitly keeps that contract intact, so no DECISION_CONFLICT.
- Provenance note: the `Vec<char>` shape was prescribed verbatim by the plan-1 task-7 brief's reference implementation. Per the ROADMAP entry's own rationale ("the conventions idiomacy directive only governs code written AFTER it existed - this pass covers everything written before"), plan provenance is exactly the population this review targets, not a protecting decision.

## Net

Construct, claim, and replacement all verified. `lines_cut: 1` is plausible (the `Vec<char>` collect line goes, a char counter comes in; roughly net -1). Zero dependency impact. CONFIRMED.
