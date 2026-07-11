<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      rustdoc  (round 1 of 1)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01T7hTuQLE5jogC7ukhqurzX
  agent_id:           af046b2c95388f24c
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-af046b2c95388f24c.jsonl
  dispatch_desc:      Review rustdoc backfill pass
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-08T09:15:15.086Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance

Verified against the task's binding standard, using the diff plus direct reads of every touched source file (not diff-hunk context alone), and independently re-ran the gate: `cargo check --workspace` (deny active), `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (81 passed, matches `grep -rn '#\[test\]' | wc -l` = 81 independently). All green, corroborating the report's gate output verbatim.

- `#![deny(missing_docs)]` present on exactly the three lib.rs roots (muxsmith-core, muxsmith-cli, xtask); binaries (`main.rs` x2) got `//!` overviews without the deny attribute, as instructed.
- Crate-root `//!` overviews all state the layering rule explicitly (core: "emits no user-facing prose... for the CLI and GUI renderers to turn into localized text"; cli: "thin clap shell over muxsmith-core... no validation or planning logic lives here"; xtask: "never a runtime dependency of muxsmith-core or muxsmith-cli").
- All 6 required contracts present and independently verified correct against source, not just present:
  - `TemplateError` `pos` char-offset contract: correct (`chars: Vec<char>` indexing in `template.rs`, offsets are char not byte).
  - `Renderer::msg` raw-id fallback: correct, matches `i18n.rs` implementation and its own test (`unknown_message_id_falls_back_to_raw_id`).
  - `DiagCode::key() == serde wire string`: correct, and enforced by the pre-existing `all_keys_match_serde_encoding` test.
  - `MatchExpr` conjunction/any/not semantics: matches spec 4.3 exactly.
  - `Scalar` untagged variant order: doc correctly explains the real serde gotcha (integer `Content` will satisfy an `f64` request via widening, so `Int` must precede `Float`; `Bool`/`Str` ordering is stated too, though for structured YAML/JSON input that particular half is not actually load-bearing — see Minor note below).
  - `optional` zero-candidate semantics: correct, matches spec 4.5 verbatim ("two candidates remain an error").
- DiagCode per-variant docs (spot-checked 14 of 30, more than the required 8) all match spec 5.2 rows where a row exists, and match actual `validate.rs`/`lint.rs` behavior for config-time codes not in the 5.2 table (that table only covers 13 of 30 codes; the other 17 config-time codes are correctly grounded in code per the report's own disclosure, not fabricated).
- `UnknownProperty` vs `UnknownSettableProperty` split verified directly against `validate_expr`/`validate_changes` in `validate.rs`: match-condition unknown props emit `UnknownProperty`, `changes` unknown props emit `UnknownSettableProperty`. The controller's one-line spec edit (f7afa8d) correctly aligns the spec with this actual behavior.
- ASCII/typography: zero violations found scanning the entire diff for em/en-dash, curly quotes, ellipsis, or any non-ASCII byte.

## Strengths

- Contract-grade docs throughout: field/variant docs consistently state the *why* and edge cases (e.g. `Filter::Int`'s "a missing field still renders empty, never `"0"` for an absent value"; `Locator::recursive`'s "deliberately asymmetric with `input.recursive`").
- The `Scalar` ordering doc is the standout: it correctly identifies and explains a genuinely non-obvious serde untagged-enum coercion rule, exactly the kind of doc that earns its existence rather than restating a name.
- DiagCode per-variant rustdoc via extended macro grammar (`$($(#[$meta:meta])* $variant:ident => $key:literal),+`) is a clean, minimal extension: `ALL`/`key()` correctly keep bare `$variant`, so no behavior changed, confirmed by re-running the full gate.
- Report's own disclosure of judgment calls (binaries needing `//!` despite no deny, generic-doc superseded by per-variant docs, private items left undocumented) is accurate and matches what's in the diff.

## Issues

#### Critical

None. No behavior changes found: every hunk in the diff is a doc comment, one of the two `#![deny(missing_docs)]` insertions, the `report.rs` macro's meta-forwarding extension, or the controller's one-line spec-table edit. Independently confirmed no functional drift via passing test suite (81/81) and clean clippy/fmt.

#### Important

None. No doc found asserting semantics that contradict the spec or the actual code.

#### Minor

Name-restatement instances (5 found in a sample of 60+ examined across all three crates; not pervasive):

- `crates/muxsmith-cli/src/cli.rs:12` — `/// The invoked subcommand.` on `Cli::command: Cmd`. Pure synonym of the field name, no added semantic content.
- `crates/muxsmith-core/src/profile/model.rs:57` — `/// Free-form description of the profile's intent.` on `Meta::description`. Defines "description" using "description."
- `crates/muxsmith-core/src/profile/model.rs:169` — `/// Overwrite the existing file.` on `CollisionPolicy::Overwrite`. Adds only "the existing file" beyond the variant name.
- `crates/muxsmith-core/src/profile/model.rs:178` — `/// Retain the item in the output.` on `KeepDrop::Keep`. Synonym restatement (Keep = Retain).
- `crates/muxsmith-core/src/profile/model.rs:180` — `/// Omit the item from the output.` on `KeepDrop::Drop`. Synonym restatement (Drop = Omit).

Secondary, non-blocking observation: the `Scalar` doc's claim that "`Bool` is tried before `Str` so `true`/`false` bind as booleans" is technically not load-bearing for structured YAML/JSON input (an already-typed boolean scalar node fails to deserialize as a string regardless of declared order; only the `Int`-before-`Float` numeric-widening half is actually order-dependent). Not wrong enough to call Important since the ordering *is* genuinely load-bearing overall and the doc's practical guidance is correct, but the stated mechanism for the bool/string half overstates its necessity.

## Assessment

**Task quality:** Approved

**Reasoning:** Zero behavior drift (independently re-verified: full gate green, 81/81 tests), all six mandated contracts present and factually correct against source, and DiagCode semantics check out against both spec 5.2 and actual validation code. The five restatement instances found are genuinely minor and isolated, not a pattern.