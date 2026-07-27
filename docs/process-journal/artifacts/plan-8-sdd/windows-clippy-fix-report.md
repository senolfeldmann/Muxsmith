# Windows-only clippy failure: `unused import: crate::error::ParamValue`

Standalone fix, plan-8. CI run 30260189439, `windows-2025` leg, red since
900db87 (2026-07-22).

## Defect

`src-tauri/src/lib.rs:591` imported `crate::error::ParamValue` unconditionally
at the top of `#[cfg(test)] mod tests`. Every consumer of that name sits behind
`#[cfg(unix)]`, so on Windows the import resolves to nothing and
`cargo clippy --workspace --all-targets -- -D warnings` (ci.yml:86) turns the
`unused_imports` lint into a hard error. The Linux and macOS legs stayed green
because there the gate is true and the import is used.

## Which platform-gated test uses `ParamValue`

Exactly one, and it is the only use of the name anywhere in `muxsmith-gui`
outside `src-tauri/src/error.rs`:

- `tests::detect_mkvmerge_body_too_old_carries_found_and_minimum`
  (`src-tauri/src/lib.rs`), carrying `#[test]` + `#[cfg(unix)]`, at the two
  assertions on `err.params["minimum"]` and `err.params["found"]`.

The test is unix-only because it drives `detect_mkvmerge_body` through
`fake_mkvmerge`, the helper that writes a fake `mkvmerge` **shell script** and
chmods it executable via `std::os::unix::fs::PermissionsExt` — itself
`#[cfg(unix)]`.

`src-tauri/src/error.rs` also uses `ParamValue` heavily in its own test module,
but resolves it through `use super::*` from the same file, so it is unaffected
on every platform.

## Fix

```rust
 #[cfg(test)]
 mod tests {
     use super::*;
+    // Only the #[cfg(unix)] fake-script test consumes the param type; an
+    // unconditional import fails clippy -D warnings on Windows (unused).
+    #[cfg(unix)]
     use crate::error::ParamValue;
     use std::process::Command;
```

Gate the use-declaration with the same `cfg` predicate that gates its only
consumer. No `#[allow]`.

**House pattern, not invention.** `crates/muxsmith-core/tests/mkvmerge_runtime.rs`
already solves the identical situation the identical way — a `#[cfg(unix)]`
directly on the `use` of `RuntimeError`, with a two-line comment naming this
same Windows clippy failure. The new comment is worded to match it. The
alternative (a function-local `use` inside the test body) is also present in the
module for `PermissionsExt`, but that form is used there for genuinely
unix-API-specific traits; the top-of-module gated `use` is the closer precedent
for a crate-internal type consumed by a platform-gated test.

## cfg-graph argument

One predicate, `unix`, now governs both ends of the dependency:

| item | gate | Linux/macOS | Windows |
| --- | --- | --- | --- |
| `use crate::error::ParamValue` | `#[cfg(unix)]` | present | stripped |
| `fn detect_mkvmerge_body_too_old_carries_found_and_minimum` (sole consumer) | `#[cfg(unix)]` | present | stripped |
| `fn fake_mkvmerge` (the helper that forces the gate) | `#[cfg(unix)]` | present | stripped |

Since definition and use are stripped together by the same predicate, no
configuration exists in which the import survives without a consumer. The
failure mode is closed structurally, not suppressed — there is no third state,
because `cfg(unix)` is a two-valued predicate evaluated at expansion.

## Verification

Not just cfg reasoning: the `x86_64-pc-windows-msvc` target turned out to be
installed locally, and clippy needs no linker, so the real Windows
configuration was checked directly.

| # | check | result |
| --- | --- | --- |
| 1 | `cargo clippy --workspace --all-targets -- -D warnings` (host, Linux) | pass (exit 0) |
| 2 | `cargo test -p muxsmith-gui` | pass — 82 passed, 0 failed |
| 3 | `cargo test -p muxsmith-gui detect_mkvmerge_body_too_old_carries_found_and_minimum` | pass — the gated test really runs on Linux |
| 4 | `cargo clippy -p muxsmith-gui --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | pass (exit 0) |
| 5 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | pass (exit 0) — no other Windows-only clippy defect anywhere in the workspace |

### The negative checks were made to fire first

A green result here is an absence, so each check was proven capable of
producing the failure before its pass was trusted:

- **Pre-fix reproduction on Linux.** Temporarily flipping the sole consumer's
  gate to `#[cfg(windows)]` puts the host build in exactly the cfg state
  Windows CI sees. Clippy then emitted, verbatim,
  `error: unused import: crate::error::ParamValue`. Putting the import under
  the same temporary `windows` gate made it green again; both flips were then
  reverted to `unix`.
- **Pre-fix reproduction on the real Windows target.** With the `#[cfg(unix)]`
  removed from the import again, check #4 exited 101 with the same
  `error: unused import: crate::error::ParamValue`. So check #4's green is a
  real signal, not a check aimed at nothing.

The final `git diff` is the three added lines above and nothing else — every
temporary flip is reverted.

### Note on the cross-target run

Cross-compiling to `x86_64-pc-windows-msvc` from Linux emits one build-script
warning, `muxsmith-gui@0.1.0: GNU compiler is not supported for this target`
(the `cc` crate probing for an MSVC toolchain that is not present on this host).
It is a build-script diagnostic, not a clippy lint; the run still exits 0 and
the real Windows CI leg has a genuine MSVC toolchain, so it does not appear
there. Do not mistake it for a finding.

## Scope

One file, three added lines, no behavior change: the import list of a test
module. No production code path, no test logic, no CI config touched.
