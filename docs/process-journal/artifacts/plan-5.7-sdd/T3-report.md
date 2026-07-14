# T3 report (Stream C): dry-run indent + ctrlc registration warning

- Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan57-c` (branch plan57-c, base cd5e917)
- Scope: verdict items 8 (dry-run indent, owner ruling YES) + 5 (ctrlc warning), per plan 5.7 Task 3.
- Status: COMPLETE, all gates green, committed (hash at bottom).

## Empirical idiom verification (before any edit)

Temporary probe test (`tests/zz_render_probe.rs`, deleted before commit) through
the real `Renderer` path (en, `set_use_isolating(false)` as in `i18n.rs:31`),
plus a standalone bundle configured identically for the two candidate idioms:

- BEFORE, current catalog: `"rule 0 -> track 1"` / `"output: /out/movie.mkv"` --
  flush-left; the two post-`=` spaces are parser-stripped, confirming the defect.
- Idiom `{"  "}` (single placeable, both spaces): renders `"  rule x -> track y"`.
- Idiom `{" "}{" "}` (two placeables): renders identically.

Chosen: single `{"  "}` placeable -- one placeable carrying both spaces, cleaner
source, empirically identical rendering. Applied to all four lines (en+de
identical mechanics). Isolation marks are off (i18n.rs:31), so no FSI/PDI
pollution around the literal.

## Snapshot/test re-verification (upstream claim: no snapshot pins these lines)

Re-verified by grep over the whole worktree:

- `dry-run-assignment` / `dry-run-output` appear only in `commands/mod.rs`
  (render site) and `catalog_completeness.rs` (allowlist + fixtures; its leak
  check only tests for `{$`, not leading whitespace).
- All 11 `.snap` files inspected; the three dry-run snaps cover empty-dir,
  language-query-failure, and bad-regex config-diag cases only -- none contains
  a rendered assignment/output line. No other test asserts the rendered lines.

Claim confirmed; no test updates were needed for the indent change.

## Rendered before/after (dry-run block, unit-level through the real Renderer)

Before (flush-left, the source's visual intent silently not shipping):

```
/in/movie.mkv (identifier: movie)
rule 0 -> track 1
output: /out/movie.mkv
```

After (two-space indent under the `dry-run-file` header, as intended):

```
/in/movie.mkv (identifier: movie)
  rule 0 -> track 1
  output: /out/movie.mkv
```

(Probe output, debug-formatted: `"  rule 0 -> track 1"`,
`"  output: /out/movie.mkv"`, `dry-run-file` unchanged flush-left. Rendered via
the unit-test path; no cheap media fixture exists without driving mkvmerge, and
the render site `commands/mod.rs:101-116` feeds `renderer.msg` directly, so the
unit rendering is the exact production string.)

## How the new key mirrors the joblog pattern

`run-joblog-unavailable` (the established degradation-warning shape,
run.rs create_logger doc + `eprintln!("{}", renderer.msg("run-joblog-unavailable", &[]))`):

- Failure never aborts the run; warn on stderr via the Fluent catalog, continue.
- Same render call shape: `eprintln!("{}", renderer.msg("run-signal-handler-unavailable", &[]))`
  on `ctrlc::set_handler(...).is_err()` (clippy-clean `.is_err()` over
  `if let Err(_)`).
- Doc comment on the registration site extends the existing D16 comment and
  explicitly cross-references `create_logger`'s degradation contract.
- Message register mirrors the sibling: en ends "...; continuing." like
  "continuing without persisted logs"; de uses "für diesen Lauf ... es wird
  fortgefahren" exactly like the de joblog message. Real umlauts in de
  (kontrollierter Abbruch, Löschen unvollständiger Ausgaben, Exit-Code 130 --
  "Exit-Code" matches `run-job-failed`'s existing terminology).
- catalog_completeness registration mirrors the sibling exactly: appended to
  `ALLOWLISTED_CLI_KEYS` (after the joblog keys, matching catalog order) and
  added to the same zero-arg fixture arm as `run-joblog-unavailable` in
  `allowlisted_cli_key_args`.

## Exact diff (committed)

```diff
diff --git a/crates/muxsmith-cli/src/commands/run.rs b/crates/muxsmith-cli/src/commands/run.rs
--- a/crates/muxsmith-cli/src/commands/run.rs
+++ b/crates/muxsmith-cli/src/commands/run.rs
@@ -215,13 +215,23 @@ pub fn run(
 
     // Single-level SIGINT (D16): first Ctrl-C requests graceful cancel
     // (queue kills in-flight, partials deleted, summary printed, exit 130);
-    // a second Ctrl-C during cleanup force-exits immediately.
+    // a second Ctrl-C during cleanup force-exits immediately. Registration
+    // fails only on an OS-level signal-registration error (this is the one
+    // registration in the process, so ctrlc's double-registration error is
+    // unreachable here); like `create_logger` below, that degrades to a
+    // stderr warning instead of aborting: the batch still muxes, it only
+    // loses the graceful-abort semantics above (a terminal Ctrl-C then
+    // SIGINTs the whole process group without cleanup).
     let handler_cancel = Arc::clone(&cancel);
-    let _ = ctrlc::set_handler(move || {
+    if ctrlc::set_handler(move || {
         if handler_cancel.swap(true, Ordering::SeqCst) {
             std::process::exit(130);
         }
-    });
+    })
+    .is_err()
+    {
+        eprintln!("{}", renderer.msg("run-signal-handler-unavailable", &[]));
+    }
 
     // D26: persisted job logs. Created before the queue runs so `on_event`
     // can tee every event as it arrives (see the drain loop below); a
diff --git a/crates/muxsmith-cli/tests/catalog_completeness.rs b/crates/muxsmith-cli/tests/catalog_completeness.rs
--- a/crates/muxsmith-cli/tests/catalog_completeness.rs
+++ b/crates/muxsmith-cli/tests/catalog_completeness.rs
@@ -195,6 +195,7 @@ const ALLOWLISTED_CLI_KEYS: &[&str] = &[
     "run-joblog-unavailable",
     "run-joblog-written",
     "run-joblog-incomplete",
+    "run-signal-handler-unavailable",
 ];
 
@@ -209,7 +210,8 @@ fn allowlisted_cli_key_args(key: &str) -> Vec<(&'static str, &'static str)> {
         "validate-ok"
         | "mkvmerge-not-found"
         | "mkvmerge-query-failed"
-        | "run-joblog-unavailable" => vec![],
+        | "run-joblog-unavailable"
+        | "run-signal-handler-unavailable" => vec![],
diff --git a/locales/de/cli.ftl b/locales/de/cli.ftl
--- a/locales/de/cli.ftl
+++ b/locales/de/cli.ftl
@@ -24,8 +24,8 @@
-dry-run-assignment =   Regel { $rule } -> Spur { $track }
-dry-run-output =   Ausgabe: { $path }
+dry-run-assignment = {"  "}Regel { $rule } -> Spur { $track }
+dry-run-output = {"  "}Ausgabe: { $path }
@@ -45,3 +45,4 @@
+run-signal-handler-unavailable = Der Strg-C-Handler konnte nicht registriert werden; ein kontrollierter Abbruch (Beenden laufender Jobs, Löschen unvollständiger Ausgaben, Exit-Code 130) ist für diesen Lauf nicht verfügbar; es wird fortgefahren.
diff --git a/locales/en/cli.ftl b/locales/en/cli.ftl
--- a/locales/en/cli.ftl
+++ b/locales/en/cli.ftl
@@ -17,8 +17,8 @@
-dry-run-assignment =   rule { $rule } -> track { $track }
-dry-run-output =   output: { $path }
+dry-run-assignment = {"  "}rule { $rule } -> track { $track }
+dry-run-output = {"  "}output: { $path }
@@ -38,3 +38,4 @@
+run-signal-handler-unavailable = The Ctrl-C cleanup handler could not be registered; graceful abort (cancelling in-flight jobs, deleting partial outputs, exit code 130) is unavailable for this run; continuing.
```

## Gate output (all FOREGROUND, run on the final state after probe deletion)

| Gate part | Result |
|---|---|
| `cargo test --workspace` | 461 passed, 0 failed, 0 compile errors (superset of the prescribed `-p muxsmith-cli`) |
| `cargo test -p muxsmith-cli --test catalog_completeness` (explicit) | 4 passed, 0 failed |
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | finished clean |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | finished clean |
| `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| `pnpm check:i18n` (the BUILDING.md i18n part covering the CLI catalogs: check 3 parity de vs en incl. cli.ftl) | `ok (17 source files scanned, 179 catalog ids, 12 unused warning(s), 1 other locale(s) checked for parity against 6 en/ catalog(s))`; the 12 unused warnings are pre-existing gui-*.ftl keys, none names the new key or cli.ftl |

Not run: `pnpm lint`, `pnpm build`, `pnpm test:e2e` -- no frontend file touched
(cli.ftl is CLI-only vocabulary, excluded from frontend usage checks per
check-i18n.mjs header); controller re-runs the full nine-part gate at merge per
plan constraint.

## Deviations surfaced (none resolved silently)

1. **None against Tier-2 conventions.** The change follows core-37's
   surface-side counterpart (warning through `renderer.msg`, no bare English
   string), i18n bilingual-same-commit (plan constraint), ci-06 gate
   discipline.
2. Line-number drift vs the brief: de dry-run keys sit at de/cli.ftl:27-28 (the
   brief said ~25-26; the file's 8-line header comment shifts them). Content
   sites matched exactly; noted only for traceability.
3. The prescribed `cargo test -p muxsmith-cli` was run as part of the wider
   `cargo test --workspace` (ci-06 requires the workspace run before every
   commit anyway); the explicit catalog-completeness run was done separately as
   prescribed.
4. `run_live` snapshot (`run_live__live_run_muxes_two_sources...`) passed
   unchanged -- it pins run output, not dry-run output; recorded here because it
   is the nearest rendered-output snapshot to the touched keys.

## Commit

- Hash: `17ae87c0754da5d9274f25006a29fc828e328560` (branch plan57-c)
- Staged explicitly: `locales/en/cli.ftl locales/de/cli.ftl
  crates/muxsmith-cli/src/commands/run.rs
  crates/muxsmith-cli/tests/catalog_completeness.rs` (no `git add -A`)
- Message: `cli: render intended dry-run indent via placeables; warn on ctrlc
  registration failure (bilingual)` + co-author trailer, unsigned
  (`-c commit.gpgsign=false`) per SI-4.
