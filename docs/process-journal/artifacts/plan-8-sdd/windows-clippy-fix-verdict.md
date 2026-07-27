# Verdict: Windows-only clippy fix (`d095121..f4f932e`)

**APPROVED.**

Independent review of commit `f4f932e` (gate the `ParamValue` test import in
`src-tauri/src/lib.rs` with `#[cfg(unix)]`). Every load-bearing claim in
`windows-clippy-fix-report.md` was re-derived from the artifacts rather than
taken over from the report.

## What was verified

### 1. Diff is exactly the gated import, nothing else

`git diff d095121 f4f932e` reproduces the package file byte-for-byte: one file,
three added lines (two comment lines + `#[cfg(unix)]`), zero deletions, zero
other hunks. No production code, no test logic, no CI config.

Note for the record: `HEAD` is `dbd0dc3`, one commit *past* the reviewed range.
It changes only `docs/decision-ledger.yaml` (+2/-1: one occurrence added,
`count: 1 -> 2` incremented with it, which is the correct dependency sweep).
No Rust source, so all clippy runs below still measure the reviewed fix.

Working tree was clean before and after review (`git status --porcelain` empty).

### 2. Sole-consumer claim: confirmed, two independent ways

Every `ParamValue` occurrence in the crate, found by my own grep:

| site | role | gate |
| --- | --- | --- |
| `src-tauri/src/lib.rs:594` | the import | `#[cfg(unix)]` (the fix) |
| `src-tauri/src/lib.rs:931` | assertion on `params["minimum"]` | inside `detect_mkvmerge_body_too_old_carries_found_and_minimum`, `#[cfg(unix)]` at line 925 |
| `src-tauri/src/lib.rs:932` | assertion on `params["found"]` | same function |
| `src-tauri/src/error.rs` (many) | definition + own test module | resolves via `use super::*` (`error.rs:216`), same file, unaffected on all platforms |
| `src/ipc.ts:19` | a doc comment in TypeScript | not a Rust use |

So exactly one consumer in `lib.rs`, carrying the same predicate as the import.
The empirical check below closes this independently: had a second unix-gated
consumer existed, flipping this one test would not have produced an unused
import.

### 3. House precedent: real, and the comment is a genuine match

`crates/muxsmith-core/tests/mkvmerge_runtime.rs:6-9` reads:

```rust
// Only the #[cfg(unix)] fake-script tests consume the error type; an
// unconditional import fails clippy -D warnings on Windows (unused).
#[cfg(unix)]
use muxsmith_core::capability::runtime::RuntimeError;
```

Same construct (cfg directly on the `use`, top of module, no `#[allow]`), same
stated reason. The new comment is that sentence with "error type" -> "param
type" and the plural relaxed to singular, which is if anything the more accurate
of the two (the precedent's `RuntimeError` also has a single consumer site, at
line 161). This is house pattern application, not invention.

### 4. Clippy re-run: green on host and on the real Windows target

`rustup target list --installed` shows `x86_64-pc-windows-msvc` and
`x86_64-unknown-linux-gnu`, so the Windows configuration was checkable directly.

| command | exit |
| --- | --- |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 |

The cross-target run emits `muxsmith-gui@0.1.0: GNU compiler is not supported
for this target` twice. Confirmed as a `cc` build-script diagnostic, not a
clippy lint; the run still exits 0. The report's note on it is accurate.

CI invokes exactly the host form (`ci.yml:86`) inside a per-OS matrix job (the
surrounding steps branch on `runner.os` for Windows/macOS/Linux), so the
`windows-2025` leg runs this lint natively. The reviewed fix addresses that leg.

### 5. Fire-check reproduced: the green result is a real signal

Both clippy runs above were cache replays (0.19s / 0.13s), which on its own
proves nothing. So the check was made to fire:

- Flipped the sole consumer's gate from `#[cfg(unix)]` to `#[cfg(windows)]`
  (`lib.rs:925`), putting the host in the cfg state Windows CI sees.
- Host clippy then exited **101** with `error: unused import:
  crate::error::ParamValue`, pointing at line 594 — the exact defect.
- Reverted. Restoration is alias-proof, not eyeballed: the restored blob hashes
  to `0e8884d137d9f0e87a32a3f5e09f2d82caa54b31`, identical to the pre-mutation
  hash *and* to the post-image hash recorded in the diff's own `index` line
  (`a5a8f85..0e8884d`). `git status --porcelain` and `git diff HEAD` are both
  empty.
- The post-restore clippy run was a genuine recompile (`Checking muxsmith-gui`),
  not a replay, and exited 0. So the green in check 4 is a measured absence.

Additionally, `cargo test -p muxsmith-gui
detect_mkvmerge_body_too_old_carries_found_and_minimum` passes (1 passed, 81
filtered) — the gated test really executes on Linux, so the import is genuinely
needed there and the fix is not quietly papering over a dead test.

## Findings

None blocking. The fix is minimal, correct, structurally closed (one two-valued
predicate governs definition and use, so no configuration leaves the import
without a consumer), consistent with in-tree precedent, and free of `#[allow]`
suppression. The report's claims all held under independent check, including the
ones that favoured it.

## HARVEST

**The recurrence is the finding, not the bug.** The ledger now counts this class
at 2: `mkvmerge_runtime.rs` solved it in-tree, with a comment naming this exact
failure, and plan-7 reintroduced it anyway across five red pushes. A rule of the
form "remember that a platform-gated consumer needs a platform-gated import"
requires someone to *notice*, which is precisely what did not happen the second
time.

There is a mechanical handgriff available, and this review demonstrated it
works: `x86_64-pc-windows-msvc` is installed on the Linux dev host, and clippy
needs no linker, so

```
cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
```

reproduces the Windows leg locally in seconds. Run as a pre-push gate (or as a
step in whatever local verification precedes a push), it converts the whole
cfg-divergence class from "caught by CI after the push, five times" into "caught
before the push, deterministically". Worth considering for the project's
standing verification sequence; out of scope for this one-commit fix.

## Constraints observed

Read-only apart from this verdict file and the transient fire-check mutation,
which was restored and verified by blob hash plus clean `git status`. No
commits, no git writes, no session relocation. All runs foreground.
