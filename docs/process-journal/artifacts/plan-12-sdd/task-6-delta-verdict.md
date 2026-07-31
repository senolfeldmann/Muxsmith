# Task 6 delta verdict - fix round 1

Same reviewer as `task-6-verdict.md`. Scope: `.superpowers/sdd/plan-12/
review-011fb96..3caa87f.diff` (one commit, `src-tauri/src/lib.rs` only, 8
insertions/3 deletions, comment-only) against `task-6-report.md`'s appended
"Fix round 1" section. Findings 2 and 3 graded (routed). Findings 1 and 4
were not routed and are checked only for honest disposition, per the
dispatch - not re-graded, not re-opened. Round-1-verdict-approved parts
(spec compliance, the twelve prior findings-adjacent dimensions that held)
are not re-examined.

All checks re-run independently: fresh `rsync` of the current committed
tree (excluding `target/`, `.git/`, `node_modules/`, `dist/`) to
`/tmp/claude-1000/-home-senol-agents-peter/a1386daa-bdbc-4366-b18d-375daf90cf89/scratchpad/muxsmith-copy`,
diffed byte-for-byte against the real repo's `src-tauri/src/lib.rs` before
grading. The tracked repository (`/home/senol/Git/Muxsmith`) was touched
only for read-only `git`/`diff` commands and confirmed clean throughout.

## Per-finding verdicts

**Finding 2 - ADDRESSED.** Graded by re-deriving both directions from the
mechanism myself, not by reading the corrected prose for plausibility.
Traced independently: `set_editor_dirty` is one unconditional
`state.editor_dirty.store(dirty, Ordering::SeqCst)` (no merge, no history);
the frontend watcher (`watch(dirty, (value) => { void
setEditorDirty(value).catch(() => {}); })`) fires once per `dirty` change
with no `immediate`, and `.catch()` swallows a failed push with zero retry
- confirmed by grepping `src/ipc.ts`'s `invoke` wrapper and the watcher
itself for any retry/requeue machinery: none exists anywhere in the
frontend. Given that, both halves of the corrected sentence hold: a missed
true-transition genuinely leaves the shell's flag stale-false while the
editor is actually dirty, so `close_decision` returns `Close` and the
warning is skipped over real changes (the direction the original sentence
already had right); a missed false-transition after a save genuinely
leaves the flag stale-true while the editor is actually clean, so
`close_decision` returns `ConfirmDiscard` and the warning fires with
nothing at risk (the direction the original sentence denied could happen,
and the reason it was wrong). Neither half is invented or overclaimed -
"nothing here makes either direction structurally impossible" is itself
accurate, since I found no retry, dedup, or reconciliation anywhere in the
path. The sibling comparison ("unlike `dialog_locale` below") still holds:
that field's "never a missing dialog" guarantee is untouched by this diff
and rests on `ftl_message`'s total fallback to the raw key, a genuinely
different, asymmetric mechanism - so the corrected comment does not
over-generalize the fix onto its neighbour either. Rechecked independently
(comment-only Rust change): `cargo fmt --all --check`, `cargo clippy
--workspace --all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings"
cargo doc --workspace --no-deps --document-private-items`, and `cargo test
-p muxsmith-gui --lib` all exit 0 on my own rebuild (86 tests, 0 failed -
unchanged from round 1, as expected for a doc-comment-only diff).

**Finding 3 - ADDRESSED.** Re-measured with my own instrument rather than
accepting the report's re-run as agreement: `git diff ed1a635..a47fc19 --
src-tauri/src/run.rs | grep -c '^+.*#\[test\]'` (a differently-shaped
pattern than the report's own `grep -c '^+    #\[test\]$'`) returns **6**,
and naming each hit reproduces the same six functions the report lists. This
is independent in a second way too: round 1's original verdict already
cross-checked this count via a completely different method (building and
running the pre-task commit `ed1a635` from a `git archive` snapshot - 80
tests there against 86 on the final tree, a measured delta of exactly +6).
Three independent derivations, one number. The correction is marked in
place - `~~seven~~ **[WRONG, see fix-round-1 correction below]**` in the
original sentence, left standing, with the "Fix round 1 correction" section
below it explaining the count and re-pasting the command - matching this
project's established `left standing` correction form (the same pattern
task-1, task-2 and task-4's reports use for an in-place-corrected wrong
claim).

## Findings 1 and 4: disposition check (not re-graded)

Both are recorded honestly in the report's "Fix round 1" section and
neither is quietly restated as resolved.

- **Item 3 (Finding 1)** is labeled "not touched -- coordinator's item,"
  states plainly that this is a plan defect rather than an implementation
  one, and attributes the routing (text correction at plan close, the
  observable riding the owner's existing 1.x GUI-test-harness item) to the
  dispatch rather than claiming to have executed it. No plan edit and no
  new test infrastructure appear in this diff (confirmed: the diff touches
  only `src-tauri/src/lib.rs`).
- **Item 4 (Finding 4)** is labeled "recorded, not changed," correctly
  restates both original rulings (the probe-key rebinding inside the
  latitude grant, the fixture-value retyping outside it and stop-list-named)
  matching my original verdict's Q1/Q2 answers without softening either,
  and closes with "not as a defect to fix" rather than any resolved-sounding
  language. The code for both items is unchanged in this diff, matching
  that disposition.

## New breakage

**Zero.** `git diff --stat 011fb96..3caa87f` covers exactly one file,
`src-tauri/src/lib.rs`, 8 insertions/3 deletions, all inside the
`editor_dirty` doc comment - confirmed by reading the diff directly, no
other line moved. `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc
--workspace --no-deps --document-private-items`, and `cargo test -p
muxsmith-gui --lib` all reproduce clean (0/0/0/86-passed) on my own rebuild,
matching the report's paste. No new `[...]`-bracket syntax was introduced
that could create a fresh intra-doc-link risk (the `dialog_locale`
cross-reference uses a plain code span, not a link). Typography swept
clean (no em/en-dash, curly quotes, ellipsis, or line-number
self-citations in the diff's added lines). Commit confirmed unsigned, one
trailer, matching SI-4.

## Harvest

- The `dialog_locale` field's comment was the one thing this delta could
  have silently over-corrected onto (an implementer fixing a sibling
  comment sometimes "fixes" the one next to it too, on the same
  plausible-symmetry impulse that caused the original bug) and didn't -
  it's untouched, and correctly so, since its guarantee rests on a
  genuinely different, asymmetric mechanism (total fallback vs a
  retry-less flag). Worth naming as the check itself, not just the
  outcome, the next time a failure-cost comment gets corrected next to an
  unrelated sibling: confirm the sibling's own guarantee still holds on
  its own mechanism rather than assuming "fixed one, leave the other."

Verdict file written to `.superpowers/sdd/plan-12/task-6-delta-verdict.md`.
