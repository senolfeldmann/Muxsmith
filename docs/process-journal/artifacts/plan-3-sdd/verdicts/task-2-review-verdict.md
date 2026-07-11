<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-2  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01HbmfrHqWbTGBrgf2bWnMjF
  agent_id:           a1f925c57683add4e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a1f925c57683add4e.jsonl
  dispatch_desc:      Review Task 2 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T10:54:03.860Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

Verified against `crates/muxsmith-core/src/matcher.rs` (diff) and cross-checked the two out-of-diff call sites in `planner.rs`:

- Trait shape matches the brief exactly: `pub trait Matchable { fn get(&self, prop: &str) -> Option<PropValue>; }`, doc comment on trait and method (matcher.rs:18-21). `impl Matchable for Track` delegates to `Track::get(self, prop)` verbatim (matcher.rs:23-27).
- `matches`, `exact_matches`, `item_str` (renamed from `track_str`) are all generic over `M: Matchable`; the `language`, `codec_kind`, and boolean-absent-false branches are unchanged aside from `track`->`item` renames and the widened type — confirmed by the diff hunks, which show no logic edits, only identifier renames and signature widening.
- No `impl Matchable for Attachment` added; `git` stat for this diff touches only `matcher.rs`. Out-of-scope boundary respected.
- Named-risk check on `impl<M: Matchable> Matchable for &M` (matcher.rs:35-41): confirmed sound.
  - Verified the double-reference claim is real, not invented: read `planner.rs:355-358` and `:704-707` — `ident.tracks: Vec<Track>` (`identify.rs:131`), `.iter()` yields `Item = &Track`, `Iterator::filter`'s `FnMut(&Self::Item)` hands the closure `t: &&Track`. Without the blanket impl, `matches<M>(item: &M, ...)` unifies `M = &Track` at those call sites (no deref coercion in generic unification), requiring `&Track: Matchable`, which only the blanket impl supplies.
  - Ran `cargo check --workspace --all-targets` on the checked-out `8a2defc` tree: clean compile, confirming no coherence conflict between `impl Matchable for Track` and `impl<M: Matchable> Matchable for &M` (different `Self` shapes, no overlap) and no unbounded/infinite trait-resolution recursion (bound resolves `&Track: Matchable` -> `Track: Matchable` in one step).
  - Traced the runtime call: `(**self).get(prop)` with `self: &&M` dereferences twice to a place of type `M`, auto-refs to `&M` for the method call, and terminates at `Track`'s own `get` — no infinite runtime recursion.
  - It is the correct minimal generalization: implementing it only for the concrete `&Track` case would just have to be duplicated (or forgotten) when Task 3 adds `Attachment`, since the exact same `.iter().filter()` double-reference shape will recur there. Genericizing over `M: Matchable` avoids that duplication without pre-implementing anything for `Attachment` itself.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` both re-run clean on the current tree, matching the report. `grep -nP '[^\x00-\x7F]' matcher.rs` finds no non-ASCII bytes.
- `#![deny(missing_docs)]` confirmed present at `crates/muxsmith-core/src/lib.rs:1`; trait and method are documented, consistent with a clean clippy/build.

### Strengths
- The deviation from the brief's literal Step 3 snippet is flagged explicitly and explained with the exact mechanism (double reference from `Iterator::filter`), rather than silently patched — correct process discipline given the brief's regression-guard claim ("resolves via type inference") was factually wrong.
- The fix stays inside the brief's file scope (`matcher.rs` only) and does not touch `planner.rs`, preserving both the scope boundary and the "no caller changes" implication of the brief.
- Doc-comment updates (module doc, trait doc) are accurate and forward-looking without overstating what's implemented ("Track is the only implementor today").
- `Attachment` scope boundary respected cleanly — the blanket impl generalizes over any future `M: Matchable` without pre-implementing or referencing `Attachment`.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **matcher.rs:344-350 (`matches_is_generic_over_matchable`)** — the new test only exercises the single-reference case (`check<M>(m: &M)` called with `&t` where `t: Track`, unifying `M = Track` directly). It never goes through the blanket `impl<M: Matchable> Matchable for &M`, so the actual scenario that motivated the deviation (`&&Track` from `.iter().filter()`) has no dedicated unit-test regression guard in `matcher.rs` itself — it's only covered indirectly by `planner.rs`'s existing tests failing to compile if the blanket impl were removed. A small addition such as filtering a `Vec<Track>` through `matches` inside `matcher.rs`'s test module would pin the exact case directly instead of relying on a different file's tests as an implicit proxy.

### Assessment
**Task quality:** Approved
**Reasoning:** The trait, `Track` impl, and generic widening match the brief precisely with no behavior change to any existing matcher logic; the one deviation (blanket `&M` impl) is technically sound, verified independently by reading the actual planner call sites and by a clean `cargo check`, and is the right-sized fix rather than scope creep. Only a minor test-coverage gap remains.