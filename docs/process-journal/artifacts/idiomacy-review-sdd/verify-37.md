# Verify-37: ActiveRun single-field wrapper (run.rs:84, yagni, slice F4)

**Verdict: CONFIRMED**

## Finding under test

`ActiveRun` is a single-field wrapper struct (`ctl: Arc<QueueControl>`) with one
production construction site, no methods, and a doc explaining why it carries
nothing else; `RunSlot::Running` already names the state. Replace with
`RunSlot::Running(Arc<QueueControl>)`; match arms become
`Some(RunSlot::Running(ctl)) => ctl.cancel_all()` etc. at commit,
abort_and_quit, do_cancel_run, do_cancel_job, and the `running()` test helper.

Verified at HEAD `2f17880a956e05f833a3afdec2c650c176e391e5`.

## (a) Cited code matches — yes

- `run.rs:84-86`: `pub(crate) struct ActiveRun { ctl: Arc<QueueControl> }`.
  Single field, no `#[derive]`, and `grep "impl ActiveRun"` over `src-tauri/`
  returns nothing — no methods, no Drop, no construction-time invariant.
- Doc comment `run.rs:76-83` explains only why the struct carries *no more* than
  `ctl` (the run_id "is not duplicated here"). That rationale argues against
  adding fields, i.e. supports inlining; it does not justify the struct's
  existence, and attaches equally to the `Running` variant's own doc.
- `run.rs:73`: `Running(ActiveRun)` — the variant already names the running
  state.
- Construction sites: `run.rs:157` (`Reservation::commit`, the sole production
  site) and `run.rs:1007` (the `running()` test helper). "One construction site"
  is accurate for production; the test helper is explicitly named in the
  replacement.
- Field-access sites, all mechanical delegations inside run.rs:
  `run.ctl.cancel_all()` at 671 (`abort_and_quit`) and 879 (`do_cancel_run`),
  `run.ctl.cancel_job(index)` at 897 (`do_cancel_job`). Each becomes
  `Some(RunSlot::Running(ctl)) => ctl.cancel_*()`, behavior unchanged.
- No file outside run.rs references `ActiveRun`; `lib.rs` names `RunSlot` only as
  the type of `AppState::active` and never pattern-matches it. Tail 1307-1714
  clean.

## (b) Replacement is current idiom — yes

Enum tuple/newtype variants with match binding (`Some(RunSlot::Running(ctl)) =>
ctl.cancel_all()`) are core, stable Rust, unchanged in edition 2024 / Rust
1.96.1 (`Cargo.toml`: `edition = "2024"`) — nothing version-sensitive. The Rust
book (ch. 6, "Defining an Enum") states enums can store data directly in their
variants, "eliminating the need for an additional struct," and that this is
"often more concise than wrapping an enum inside a struct." The newtype pattern
earns its keep via invariants, methods, or trait impls; `ActiveRun` has none.

Strongest evidence is in-repo: the sibling variant `RunSlot::Reserved(Arc<
AtomicBool>)` (run.rs:71) already carries its payload bare, so the codebase's own
idiom for *this very enum* is exactly the proposed form. Inlining makes `Running`
consistent with `Reserved`.

Residual difference considered and dismissed: field privacy. `ActiveRun.ctl` is
private to the `run` module; a bare variant payload is as visible as the
`pub(crate)` enum. Not load-bearing — nothing outside run.rs matches `RunSlot`,
and `Reserved` already exposes its `Arc` bare at the same visibility, so the
wrapper preserves no uniform encapsulation.

## (c) Duplication difference — n/a

Not a duplication finding; a single-wrapper yagni. `RunSlot::Running(Arc<
QueueControl>)` is functionally identical to `RunSlot::Running(ActiveRun { ctl })`.

## (d) yagni completeness — yes

Concrete construct (`ActiveRun`, run.rs:84) and concrete replacement
(`RunSlot::Running(Arc<QueueControl>)` with all five named sites) are stated.

## Decision guard — no conflict, not tracked

- `docs/superpowers/specs/*.md` (D1-D35): no decision on `ActiveRun` as struct
  vs. tuple variant. The `single-field` hits are all B-8 (`raw:` property-literal
  rule), unrelated.
- `docs/IDEAS.md`: no hit.
- `docs/ROADMAP.md`: cosmetic-cleanup sweep group K (lines 260-267) enumerates
  dead `at` param, template mislabel, TracksCfg placement, stale module doc,
  Plan-1 remnants, eager chapters/attachments resolve — `ActiveRun` inlining is
  NOT among them; deferred/restraint entries don't name it either.
- The struct's doc and the plan-5 task-8 report are code/process-journal
  artifacts describing the implementation, not a recorded decision to preserve
  the struct.

Not TRACKED, not conflicting.

## Conclusion

Every claim is accurate against the source; the replacement is idiomatic Rust for
the pinned toolchain and matches the enum's own sibling variant; no recorded
decision keeps the struct or tracks its removal. The wrapper adds a name the
`Running` variant already provides, at the cost of ~13 lines and an inconsistency
with `Reserved`. **CONFIRMED.**
