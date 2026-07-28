# Amendment 3 report - Plan-9 design (D96 rider, run_batch rustdoc restated)

**Status: COMPLETE.** One file edited (the design document), committed as
`08621cb`, pathspec-scoped, not pushed. All three brief premises reproduced
at the tree; one incidental brief claim did not reproduce (the src-tauri
Files-list parenthetical, below) and one tree-state deviation was found and
resolved (HEAD hash, below). One concurrent-writer observation surfaced for
the controller.

## Tree-state deviation, resolved before anything was built on it

The brief said HEAD `e592f55`, clean tree. Actual HEAD at dispatch:
`851ada1` - a sibling of `e592f55` (same message "house+roadmap: mine the
Task 2 harvest...", same parent `9b2843f`; `git merge-base --is-ancestor
e592f55 HEAD` exits 1). `git diff e592f55 851ada1` is EMPTY: the trees are
byte-identical, so `851ada1` is a metadata-only re-commit and every content
premise was checkable against an identical tree. Tree clean at my start.

## The three premise checks

1. **CLI `on_event` is neither window-emit nor test collector - REPRODUCED.**
   `crates/muxsmith-cli/src/commands/run.rs:208-217`: the closure passed to
   `run_batch` early-returns under `--json`, else
   `for line in milestones.render(event, total, renderer) { println!("{line}"); }`
   - milestone lines to stdout.
2. **The `#[tauri::command]`/detached-thread sentence is false for the CLI -
   REPRODUCED, and strengthened.** `grep -c "thread"
   crates/muxsmith-cli/src/commands/run.rs` -> 0 (control on the same
   pattern: `src-tauri/src/run.rs` -> 28), so the CLI blocks its calling
   thread. Additional own observation: the sentence was imprecise even for
   the GUI - the detached spawn sits in `start_run`'s own body
   (`std::thread::spawn` at `src-tauri/src/run.rs:444`, inside the
   `#[tauri::command] pub async fn start_run` at `:399-400`), not in the
   macro-generated wrapper. The rider records this; the replacement
   attributes the threading to the caller, which is correct for both.
3. **The `finish_teardown`/D31 paragraph names an unreachable symbol -
   REPRODUCED, all three halves.** `fn finish_teardown` at
   `src-tauri/src/run.rs:651`, no `pub`; `mod run;` at
   `src-tauri/src/lib.rs:23`, private; `crates/muxsmith-core/Cargo.toml`
   has no gui-crate dependency (grep for `muxsmith-gui|muxsmith_gui|src-tauri`
   empty; fire-verified - `name|[dependencies]|tauri` patterns over the same
   file DO hit `name = "muxsmith-core"` and `[dependencies]`). The doc's
   mention is already a plain code span (Task 2, ledger
   `proc-48-docsurface-delink`, confirmed present at
   `docs/decision-ledger.yaml:1700` with the 2026-07-28 occurrence).

**Brief premise that did NOT reproduce (incidental):** "that file
[src-tauri/src/run.rs] is in Task 3's Files list". Task 3's EXHAUSTIVE
Files list (plan lines 220-232) does not name it; the nearest clause is the
EXEMPLARY compiler-sweep entry ("any other file the compiler flags for a
`JobOutcome` literal"). Moot for this amendment - I add no src-tauri
sentence - but had I added one, the plan amendment would have had to ADD
the file, not merely note it.

## What changed in the design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`)

Four edits, all inside the one file:

1. **D96's opening sentence** gains "(amendment 3 restates the moved
   rustdoc for its new home - the rider at this entry's end carries the
   replacement; body and signature stay as-is)" - the only place in the
   design that says "moves as-is" (swept: `grep -n
   "window-emit|tauri::command|finish_teardown|moves as-is|rustdoc"` over
   the design had exactly one hit, that line).
2. **The amendment-3 rider at D96's end**: the three passages with
   re-verified evidence; the unchanged-decision list; the replacement doc
   comment as a character-for-character transcription fence; a per-passage
   account of what the restatement changes; the no-src-tauri-sentence
   decision; the completeness pass. The fence, exactly as committed:

```rust
/// The run lifecycle's core body (D23), from the moment its [`JobSpec`]s
/// are known to the moment they are all terminal: runs `specs` to
/// completion via [`run_queue`] on its own scoped worker thread while this
/// function's own call stack drains the event channel, tee-ing every
/// [`JobEvent`] first through `logger` (when persistence is available) and
/// then through `on_event`, the caller's per-event hook carrying that
/// surface's own presentation (the GUI shell emits each event to its
/// frontend, the CLI renders its human milestone lines). Synchronous by
/// design so it is directly unit-testable with a scripted [`Spawn`]: the
/// call returns only once every job is terminal, and whether that wait
/// occupies a dedicated thread is the caller's decision, not this
/// function's - the GUI shell runs the whole call on a detached runner
/// thread so its start command can return immediately, while the CLI
/// simply blocks its calling thread.
///
/// Deliberately performs no teardown of caller-side run state: everything
/// a caller tracks about a run in flight is still in place when this
/// function returns, and clearing it - in whatever order that caller's
/// own invariants require - is the caller's job, documented where its
/// teardown code lives.
///
/// Returns the outcomes (index-aligned to `specs`, exactly like
/// `run_queue`) and `logger` back, still open, so the caller can build the
/// terminal `run_document` and only then call [`RunLogger::finish`] on it
/// (`finish` needs the very document it is about to persist).
```

   Properties, each verified: paragraph 3 is byte-identical to today's
   `queue.rs:344-347` (extracted both, `diff` empty - and D96's body quotes
   that paragraph's finish sentence, which therefore stays true); the
   rustdoc link set is unchanged (`JobSpec`, `run_queue`, `JobEvent`,
   `Spawn`, `RunLogger::finish` - no new intra-doc-link surface); every
   line <= 75 chars (measured; house width); the D23 reference, the
   logger-before-`on_event` tee order (matches the body at
   `queue.rs:359-364` and D96's CLI paragraph), and the
   index-alignment/still-open contract are kept. The caller mentions use
   the module's own non-exclusive illustrative house form (cf. `JobEvent`'s
   doc `queue.rs:19-20`), so a hypothetical third caller cannot re-falsify
   them. "renders its human milestone lines" stays true under `--json`
   (which suppresses the human channel). The `finish_teardown`/D31
   rationale is dropped from core, not moved: it already exists caller-side
   three times (`finish_teardown`'s doc `src-tauri/src/run.rs:641-650`,
   `TeardownGuard`'s doc `:656-667`, the runner-thread comment `:447-454`),
   which is why NO src-tauri sentence is added and no src-tauri file enters
   Task 3 on this amendment's account. The "plain collector in tests"
   clause was absorbed into the unit-testable sentence; it was anyway only
   half true in core (of the two moved tests, `queue.rs:1376` collects,
   `:1414` passes a no-op).
3. **Section 5's D96 bullet** gains: "Its rustdoc is restated for the new
   home: the fence in D96's amendment-3 rider is the contract, transcribed
   character for character by Task 3." (Reference sweep found no other
   design text consuming the old passages.)
4. **Amendment log, new final entry**: "Round 4 (2026-07-28), OWNER-RULED
   AMENDMENT 3, mid-execution after Task 2 (routing:
   `.superpowers/sdd/plan-9/amendment-3-brief.md`; amendment 2 was
   plan-only and lives in the plan's own log); one ruling, nothing else
   touched" - recording the ruling, the rejected alternatives, what the
   rider carries, and that D96's move decision is unchanged.

No design latitude reaches Task 3's implementer: the fence is complete (no
ellipsis, no mandated-but-unwritten passage), the target is anchored by
symbol (`pub fn run_batch`) with line numbers as of amendment time, and the
rider states explicitly that nothing else about the function changes. The
correction implies no code change beyond the doc comment - no
NEEDS_CONTEXT.

## Completeness pass (brief item 3): check and result

Two instruments over `crates/muxsmith-core/src/executor/queue.rs`:

- `grep -n "tauri\|shell\|window\|start_run\|frontend\|GUI\|CLI\|IPC\|emit"`
  over the whole file (covers indented variant/method/test docs too).
  Fire-verified: it hits the three defective lines themselves (`:332`,
  `:334-335`, `:340`), so the pattern demonstrably produces output on the
  known-present class.
- A read of the module doc (`//!`, lines 1-3) and every top-level `///`
  block, spans enumerated mechanically by awk: `:19-20`, `:71`, `:83`,
  `:86-93`, `:164-180`, `:327-347`, `:371-378`, `:383-384`, `:387-393`,
  `:402-423`, `:465-469`.

Result: **nothing else went caller-stale.** Every caller mention outside
`:327-347` is illustrative and non-exclusive and stays true with the second
caller: `JobEvent` (`:19-20`) "the CLI renders it, Plan 5's Tauri shell
forwards it" (now true via `run_batch` for both); `QueueOpts` (`:71-80`)
names `--jobs` illustratively; `QueueControl` (`:86-93`, `:100-103`) "e.g.
the CLI's ctrlc handler"; `run_queue` (`:164-180`) "e.g. the CLI's SIGINT
handler", "e.g. a GUI's per-row cancel", "a GUI needs the row-level
confirmation" (motivation prose, still true); `recover_panicked_worker`
(`:402-423`) makes no caller-specific claim (its "logged here for triage"
describes the stderr line, true on both surfaces today; D98 changes it by
design, not by falsification). All `#GUI#` hits are mkvmerge's own output
protocol, not Muxsmith's GUI. `run_document` in paragraph 3 resolves to
core (`crates/muxsmith-core/src/report/json.rs:113`), so that plain span
points at a reachable item; the paragraph is true for both callers (CLI
builds the document at `run.rs:219` then `finish` at `:235`; GUI at
`src-tauri/src/run.rs:466-467`).

## What the plan amendment (next dispatch) must carry

- **Task 3, `crates/muxsmith-core/src/executor/queue.rs`** (already on its
  EXHAUSTIVE Files list, line 222, and in its `git add` line): one added
  instruction - replace the `///` block immediately above
  `pub fn run_batch` with the fence in D96's amendment-3 rider, verbatim
  transcription, no other change to the function. Natural home: an addition
  to Step 1's queue.rs work or its own step before Step 9 (verification);
  the existing gate (`cargo doc`/clippy in the workspace gate) covers the
  link set, which is unchanged.
- **Task 3's "Read first" line** should add D96's amendment-3 rider.
- **No file additions**: no src-tauri edit exists (the rider decides
  against the optional sentence), so `src-tauri/src/run.rs` does not enter
  the Files list on this account.
- Note: the plan's Global Constraints already bind Task 3 to the design's
  amendment log "at execution time" via the pointer contract, so the design
  amendment alone is binding; the plan amendment makes the queue.rs step
  explicit rather than creating the obligation.

## Surfaced for the controller

1. **Concurrent write in the shared tree.** During my session,
   `docs/process-conventions.yaml` became modified in the working tree (the
   owner ruling on the T2 escalation: the file-vs-within-file boundary
   adopted into the latitude-grant statement, count 8 -> 9, a new decided
   occurrence). Not mine, not staged, not committed; my commit is
   pathspec-scoped (`git commit -- <design file>`) per
   `concurrent-writers-need-pathspec-scoped-commits`. The yaml sits
   uncommitted in the tree awaiting its writer.
2. **HEAD-hash drift in dispatches.** The brief's `e592f55` was amended to
   the tree-identical `851ada1` before my dispatch ran. Harmless here
   because the trees are identical, but a dispatch pinned to a hash that a
   later metadata amend replaces will confuse an agent that checks ancestry
   (as I did). Ledger-worthy shape: pin dispatches to content (a tree hash
   or file states) or re-issue the brief after any amend.
3. **The GUI-attribution imprecision was pre-existing.** The
   `#[tauri::command]`-wrapper-spawns-the-thread claim was loose even in
   the doc's src-tauri home (the spawn is in `start_run`'s body, `:444`).
   The replacement fixes it as a side effect of caller-neutral phrasing;
   recorded in the rider, no further action.

## Commit

- Hash: `08621cb` (`08621cb1d298a2cb65d17157934942f13fc7fbbf`), on
  `master`, not pushed. Trailer: exactly one `Co-Authored-By: Claude Fable
  5 <noreply@anthropic.com>`, no `Claude-Session` line.
- `git show --stat HEAD` (tail):

```
 ...-07-28-plan9-core-hoists-planner-seam-design.md | 150 ++++++++++++++++++++-
 1 file changed, 148 insertions(+), 2 deletions(-)
```

- Typography of the added lines checked: zero banned glyphs (grep over the
  diff's `+` lines; pattern fire-verified against an em-dash/smart-quote
  sample, count 1).
