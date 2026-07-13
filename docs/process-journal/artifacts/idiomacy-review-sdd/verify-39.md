# Verify-39: run.rs:453 hand-rolled `match` + explicit `drop(reservation)` vs `?`

**Verdict: CONFIRMED**

## Finding under test

`start_run` (src-tauri/src/run.rs:453-459 at HEAD 2f17880) hand-rolls

```rust
let outcome = match outcome {
    Ok(outcome) => outcome,
    Err(e) => {
        drop(reservation);
        return Err(e);
    }
};
```

where `let outcome = outcome?;` is the idiomatic construct; the explicit `drop` is redundant.

## Checks

### (a) Code says what the finding claims — YES

Read at HEAD: lines 453-459 match the finding verbatim. Context verified:

- `plan_run` returns `Result<PlanOutcome, IpcError>` (run.rs:249-255); `start_run` returns `Result<StartedRun, IpcError>` (run.rs:436). Identical error type, so `?`'s `From` conversion is the blanket identity impl; the replacement compiles with the same semantics.
- `reservation` is used after the match (run.rs:472, 479), and `let outcome = outcome?;` does not touch it. No borrow/move issue.

### Explicit `drop` genuinely redundant — YES

- `Reservation`'s `Drop` (run.rs:162-168) clears the slot iff uncommitted; at line 456 the reservation is always uncommitted (commit happens later, line 479).
- The type's own doc comment (run.rs:110-113) names implicit Drop as the *designed* mechanism: "dropping the reservation *without* commit clears the slot again -- one mechanism covering every soft-outcome early return (and a mid-planning panic) so no path can leak the reservation."
- The `map_err(...)?` two lines above (run.rs:451, the `JoinError` path) already returns early relying on exactly that implicit Drop, with the reservation live. The hand-rolled match is internally inconsistent with the adjacent line, precisely as the finding states.
- Drop-order nuance checked: explicit `drop` clears the slot before `return Err(e)` evaluates; with `?` the drop happens at scope exit after the error value is produced. Nothing can observe the interval — the error is a plain value, the slot is only readable via the `AppState` mutex, and both variants clear it before the command's `Result` reaches any caller. No load-bearing difference.

### (b) Replacement is current idiom — YES

Verified via context7 against rust-lang/rust docs (not training memory): `?` is the standard error-propagation construct (rustdoc "what-to-include" examples, RELEASES.md, try-blocks doc all use it as the canonical form); the rust-analyzer style guide's only carve-out is *against* `Err(())?` where a plain `return Err` is meant — the opposite direction, not applicable here where the value-unwrapping `let x = x?;` form is exactly the textbook use. Clippy's `question_mark` lint targets this hand-rolled pattern. Nothing in Rust 1.96 / edition 2024 changes `?` desugaring (match + early return + normal scope-exit drops).

### (c) Duplication difference — N/A

Not a duplication finding.

### (d) yagni without construct — N/A

Tag is `idiom`, and both construct and replacement are concrete.

### Decision guard — NO HIT

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `drop(reservation)`, `reservation`, `run.rs`, `match outcome`, `start_run`, and the question-mark construct. `start_run` appears only in the GUI design memo (thread-spawn/IPC surface) and ROADMAP's test-harness note (S4/S5/S6, orchestration body untested); cosmetic-cleanup group K lists unrelated items (load.rs, model.rs, planner.rs, command_integration.rs). No decision records or tracks this construct.

## Notes for the fixer

The replacement is exactly `let outcome = outcome?;` for lines 453-459 (7 lines -> 1, lines_cut 6 as claimed). Leave the `drop(reservation)` in the `PlanOutcome::Soft` branch (run.rs:472) alone — it is not covered by this finding and is load-bearing in ordering terms: it releases the slot *before* `finish_without_queue` emits `run-finished`, so a listener reacting to that event can immediately start a new run.
