# Seed [T4-m1] - lock_active doc precision

**Verdict: CONFIRMED**

- **File:** `src-tauri/src/run.rs`
- **Line:** 91 (doc comment block lines 88-99 on `lock_active`, line 100)
- **Tag:** doc
- **Lines cut:** 0 (precision rewrite, roughly net-zero)
- **Deps cut:** 0

## Current state at HEAD

The doc comment justifies poison recovery with:

> Recovery is sound because every write this lock guards is a single,
> non-panicking assignment (`*slot = Some(...)`/`*slot = None`)

This over-claims. The lock guards more than single assignments. Three
critical sections hold the guard while reading the slot and calling
methods on its contents:

- `abort_and_quit` (line 665): `cancel.store(...)` / `run.ctl.cancel_all()`
  under the guard
- `do_cancel_run` (line 873): same two calls under the guard
- `do_cancel_job` (line 894): `run.ctl.cancel_job(index)` under the guard

A panic inside one of those callees would poison the mutex while the
guard is held, i.e. "every write ... is a single, non-panicking
assignment" is not what makes recovery sound. What makes it sound: the
slot *value* is only ever replaced wholesale by a single assignment;
the other critical sections read the slot and call into its contents
without mutating it in place, so no panic point exists where the
`Option<RunSlot>` is half-applied.

## Replacement

Replace the "Recovery is sound because ..." sentence (lines 91-95) with:

> Recovery is sound because the slot value is only ever replaced
> wholesale by a single, non-panicking assignment (`*slot =
> Some(...)`/`*slot = None`); the remaining critical sections (the
> cancel/abort/close arms) only read the slot and call into its
> contents, never mutating it in place -- so even a panic while the
> guard is held (e.g. inside `cancel_all`) cannot leave a half-applied
> `Option<RunSlot>`.

Doc-precision only; no behavior change. The recovery logic itself
(`unwrap_or_else(|poisoned| poisoned.into_inner())`) is correct as is.
