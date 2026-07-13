# Task 1 review verdict: Core src, non-planner (Stream A)

Base 0b3149a..1a70936, worktree plan-5.6-a.

### Spec Compliance
✅ Spec compliant. All 8 brief items implemented, each verified against the diff by hand-tracing the before/after logic:

1. `profile/model.rs:183` KeepDrop Default+`#[default] Keep`; `keep()` and manual `AttachmentsCfg`/`TagsCfg` Default impls deleted, plain `#[serde(default)]` substituted; `TracksCfg.unmatched`'s explicit `default = "drop_policy"` left untouched (correct: its default is `Drop`, not the enum's own `Keep`). Field-by-field trace confirms identical resulting defaults.
2. `profile/validate.rs:280` if/else-if/else consolidation, `validate.rs:544-621`. Traced all three original branches (raw:, codec_kind, fallthrough): each pushed the same property-level diagnostic then the same InvalidRegex check as its last action, so hoisting the regex check to run once after the if-chain is behavior-preserving. No accidental `p`-ownership double-move (each branch only clones or borrows `p`; the final `Diagnostic::error(DiagCode::InvalidRegex, p)` is the only move, and it is per-iteration since `p` is re-bound each loop pass).
3. `executor/joblog.rs:77` + `Cargo.toml:39` - `time`'s `parsing` feature added, version unchanged (0.3.53); `run_id_timestamp` now calls `PrimitiveDateTime::parse(prefix, RUN_ID_FORMAT)`. Matches the brief's prescribed body near-verbatim. Doc comment on `RUN_ID_FORMAT` updated for accuracy (parsing is now pinned; descriptor now backs both format and parse) - correct and necessary, not scope creep.
4. `discovery.rs:187` `extension_matches` now `eq_ignore_ascii_case`; both call sites (`scan_primaries`, `resolve_locator`) pass `&input.extensions`/`&locator.extensions` directly, pre-lowering collects deleted.
5. `discovery.rs:76` `scan_primaries` collapsed to one `captures_iter` pass (`it.next()` / `it.next().is_some()` / `caps[0]`), removing the `expect()`-guarded second `captures()` call and its cross-call invariant.
6. `capability/mod.rs:126` `CODEC_KIND_NAMES` is now `LazyLock<Vec<&'static str>>` derived from `CODEC_KINDS`; `matchable_domain` relies on `&LazyLock<Vec<T>> -> &[T]` deref coercion (standard, compiles per the reported full green build/test/clippy run); the now-tautological `codec_kind_domain_matches_kinds` test deleted per the brief. Full `CODEC_KINDS` array order wasn't visible in the diff context (hunk starts at line 114, entries above are unmodified context outside the shown window) - immaterial here since the new definition derives order from `CODEC_KINDS` by construction, so drift is structurally impossible regardless of what that order is.
7. `template.rs:92` `Template::parse` moved from a `Vec<char>` index-walk to `Peekable<Chars>`. Hand-traced every branch (double-brace escape, unclosed brace, empty field, normal field, EOF-mid-field) against the old `i`-based version: the new `pos` counter (incremented once per `.next()`/consumed char, matching old `i`) reports byte-for-byte-identical `TemplateError::{UnclosedBrace,EmptyField}.pos` values in every traced case. The `while_let_on_iterator` clippy fix (`for next in chars.by_ref()`) is the correct idiom for continuing to drain the same iterator after an inner loop.
8. `executor/queue.rs:335` `jobs.max(1).min(spec_count.max(1))` -> `jobs.clamp(1, spec_count.max(1))`; `clamp`'s `min<=max` precondition holds trivially since `spec_count.max(1) >= 1`. Equivalent semantics.

No extra/undisclosed scope: the two doc-comment edits (item 3, item 6) are directly adjacent to the changed code and correct facts the mechanical change would otherwise have made stale; flagged transparently in the report rather than hidden. `planner.rs` untouched (confirmed absent from the diff's file list). `Cargo.lock` absent from the changed-files stat, confirming no dependency-graph change beyond the named feature flag.

No re-run of the test suite was performed (per instructions); confidence rests on manual trace of every changed branch plus the reported full-green nine-part gate, which is a legitimate proxy for the "existing tests pass unchanged" mechanicality criterion.

### Strengths
- Every item matches the brief's prescribed code near-verbatim; no invented alternative solutions.
- `time`'s `parsing` feature behavior was verified against the vendored crate source (proc-07) rather than trusted from the brief, with the specific range-validation mechanism (`TryFrom<Parsed>` reusing the same constructors as the old manual code) identified.
- Doc comments invalidated by the mechanical change (RUN_ID_FORMAT's feature-pinning note, CODEC_KIND_NAMES' now-deleted-test pointer) were corrected in the same commit rather than left stale.
- `std::sync::LazyLock` used over pulling in `once_cell` - native stdlib feature over a dependency, in line with the project's "dependencies are earned" convention, and requires no toolchain bump (rust 1.96.1 already exceeds LazyLock's 1.80 stabilization).
- Commit granularity is one logical group per brief bullet (Cargo.toml+joblog.rs correctly bundled since the feature flag and its only caller are one change); commit messages are precise.
- The gate-cadence deviation (below) was disclosed unprompted and reasoned through rather than silently glossed over as "gate ran before every commit."

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
- **Gate not run before every commit (plan-mandated).** The task's Global Constraint states "Nine-part gate per BUILDING.md before every commit," and process-conventions.yaml's `ci-06-per-commit-gate` ("must all pass before every commit, never skipped") is settled, reinforced 3x, with its own origin being exactly this failure mode (fmt-dirty intermediate commits slipping through). The implementer ran the gate twice total - once mid-implementation, once on final HEAD - not before each of the 7 commits (task-1-report.md "Gate cadence deviation" note). Mitigating factors: each commit touches a self-contained file with no cross-file coupling (Cargo.toml+joblog.rs is the one intentionally-bundled exception), so each commit is very likely independently buildable even though this was never actually verified in isolation; and the final HEAD state is fully gate-clean. Still, no intermediate commit in this branch has been proven bisectable, which is the exact risk `ci-06` exists to close. Recommend either a quick isolated-build spot-check per commit before merge, or squashing to fewer commits if per-commit gating isn't going to be done for real.

#### Minor (Nice to Have)
None beyond the above.

### House dimension
**Deviation:** `ci-06-per-commit-gate` (process-conventions.yaml) - see Important finding above. Everything else checked against `conventions.yaml`/`product-boundaries.yaml`/`process-conventions.yaml` is compliant: `ci-10-pin-everything` (exact `time` version held, only the feature set widened, `Cargo.lock` unchanged), `proc-05-commit-signing` (report claims `-c commit.gpgsign=false`, explicit `git add <files>`, not independently verifiable from a diff but no contrary evidence), `proc-07-verify-against-source` (positively reinforced - see Strengths), `core-07-runtime-fetching-rejected`/`core-06-schema-build-time-extraction` (untouched, `capability/mod.rs` change is purely internal derivation, no schema-fetch behavior change).

**Harvested for the ledger:**
1. **Pattern candidate:** deriving a hand-maintained "mirror" data structure from its canonical source via `LazyLock` to make drift structurally impossible (here: `CODEC_KIND_NAMES` from `CODEC_KINDS`) - a specific instance of the project's general anti-duplication instinct (echoes `core-06-schema-build-time-extraction`'s "only derived facts ship" spirit, but at the Rust-static-data level rather than build-time codegen). One occurrence so far; not yet promotion-eligible (agent-emergent needs count 3), worth watching if the same pattern recurs elsewhere.
2. **Reinforcement:** `proc-07-verify-against-source` - the `time`-crate parsing-feature equivalence was verified against the vendored crate source, not the brief's claim or training memory, with the specific validation mechanism cited.
3. **Repeated-rejection note:** none in this diff - a pure mechanical-refactor task carries no design alternatives to reject.
4. **Deviation for the ledger:** the gate-cadence gap above should be logged against `ci-06-per-commit-gate` as a further occurrence (kind: violated-corrected, since the implementer's final HEAD run does close it, but not per-commit as written).

### Assessment
**Task quality:** Approved (with a follow-up)
**Reasoning:** All 8 spec items are implemented correctly and traced by hand to be behavior-preserving; the only defect is a disclosed, low-blast-radius process-convention deviation (gate not run per-commit) rather than anything in the shipped code.
