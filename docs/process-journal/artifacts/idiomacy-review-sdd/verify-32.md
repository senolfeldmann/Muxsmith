# Verify-32: Renderer::msg duplicates msg_with_counts body

**Finding:** `crates/muxsmith-cli/src/i18n.rs:45` (tag `dup`, slice F3) - `Renderer::msg` re-implements `msg_with_counts`'s FluentArgs-building body minus the counts loop; replace with delegation `self.msg_with_counts(id, args, &[])`.

**Verdict: CONFIRMED**

## (a) Cited code matches the claim

Read at HEAD (`2f17880`). Line 45 is `pub fn msg`. Its body (lines 46-50) is `FluentArgs::new()` + the `args` loop + `self.render(id, fargs)`. `msg_with_counts` (lines 63-77) is byte-for-byte the same construction plus one extra loop over `counts` (lines 73-75). With `counts = &[]` that loop iterates zero times, so `msg(id, args)` and `msg_with_counts(id, args, &[])` build identical `FluentArgs` and hit the same `render`. The claim is accurate.

## (b) Replacement is valid and idiomatic for the pinned toolchain

The replacement introduces no new library API - it is plain method delegation with an empty slice literal; the only external API (`FluentArgs::set`) is reached through the existing `msg_with_counts` body unchanged. Empirically verified on the pinned toolchain: a minimal reproduction of the two signatures with the proposed delegation compiles and runs correctly under `rustc 1.96.1 --edition 2024` (`&[]` coerces to `&[(&str, usize)]`; output as expected). No version-sensitive idiom question remains beyond that compile check.

## (c) No load-bearing difference between the two sites

The only textual difference is the counts loop, a no-op for an empty slice. Additional evidence the delegated path is behavior-identical and already production-exercised: `render_diagnostic_message` (line 168) already calls `msg_with_counts` with an empty `counts` vec for every `DiagCode` whose `numeric_diagnostic_params` arm is `&[]` (the `_` default, i.e. most codes). `msg`'s doc-comment contract (raw-id fallback) lives in `render` and is untouched. Public signature unchanged; the `msg_with_counts` doc's "plural selectors must go through here instead of msg" guidance is unaffected by msg delegating downward.

## (d) Tag check

Tag is `dup`, not `yagni` - criterion (d) does not apply. Concrete construct and concrete replacement are named anyway.

## Decision guard

Grepped `docs/superpowers/specs/*.md` (D1-D35 memos), `docs/IDEAS.md`, `docs/ROADMAP.md` for `msg_with_counts`, `Renderer::msg`, `i18n.rs`, Fluent/renderer decisions:

- No design memo pins `msg` as an independent implementation. The T19 report (`docs/process-journal/artifacts/plan-5.5-sdd/task-19-report.md`) records the `msg_with_count` -> `msg_with_counts` generalization but no decision to keep `msg` non-delegating.
- ROADMAP cosmetic-cleanup group K (walkthrough #21), test-hygiene collection (B-minors), and the deferred/deliberate-restraint entries do not track this duplication.
- IDEAS.md: no hits.

No DECISION_CONFLICT, not TRACKED.
