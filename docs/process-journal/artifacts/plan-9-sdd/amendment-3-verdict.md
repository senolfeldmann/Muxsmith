# Amendment 3 verdict - Plan-9 design (D96 rider, run_batch rustdoc restated)

Reviewer: independent, did not author the amendment. Read at the tree
(HEAD `d7fd277` at review time - one house-knowledge commit landed after
the amendment commit `08621cb`; the design file is untouched since
`08621cb`, confirmed via `git log` on the file). All instruments my own,
under
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a3rev-independent/`.

## 1. Verdict: APPROVED

No BLOCKING, MEDIUM, or LOW finding. Every load-bearing claim of the
amendment reproduced at the tree with independent instruments; the fence
is true for both callers, complete, transcription-ready, and loses no
fact that still holds. The two named drops are ruled correct below.

## 2. Findings

None. Per-dimension results, with the evidence that would have carried a
finding if it had fired:

**Dimension 1 - the fence is true for both callers.** Each claim checked
against code, not against the old text:

- *Tee order (logger first, then `on_event`)*: `queue.rs:359-364` - the
  drain loop calls `l.on_event(&event)` inside `if let Some(l) =
  logger.as_mut()` and only then `on_event(&event)`. TRUE, and "first ...
  then" is a precision gain over the old text, which stated no order.
  Load-bearing consumer confirmed: the design's D96 CLI paragraph
  (design `:521-524`) pins exactly this order.
- *Synchronous return*: `run_batch` drains `rx` until the channel closes
  (the `tx` moved into the scoped `run_queue` thread drops when it
  returns), then `handle.join()`; `run_queue` returns one terminal
  outcome per spec (Cancelled fallback for never-dequeued slots,
  `queue.rs:311-324`). "the call returns only once every job is
  terminal" TRUE.
- *GUI detached runner thread / start command returns immediately*:
  `std::thread::spawn` at `src-tauri/src/run.rs:444`, inside `start_run`'s
  own body (`#[tauri::command]` at `:399`), handle dropped (detached),
  `Ok(StartedRun{..})` returned right after the spawn (`:471-475`). TRUE.
- *GUI "emits each event to its frontend"*: the closure at `:462-464` is
  `|event| { let _ = app_bg.emit("muxsmith://job-event", event); }` -
  unconditional, every event. TRUE.
- *CLI "simply blocks its calling thread"*: `run_batch` called directly
  at `crates/muxsmith-cli/src/commands/run.rs:208`; my instrument
  `grep -cinE "thread|spawn\(" ` over that file -> 0, fire control over
  `src-tauri/src/run.rs` -> 28 (the author's narrower `grep -c "thread"`
  reproduces as 0 / 28). TRUE.
- *"renders its human milestone lines" under `--json`*: ruled TRUE as
  written. The CLI hook (`run.rs:208-217`) early-returns under `--json`
  and renders milestone lines otherwise; the code comment directly above
  (`:204-206`) states "milestone rendering is the only thing `--json`
  suppresses". The fence's qualifier "human" names exactly the channel
  `--json` suppresses, and the parenthetical is the module's established
  non-exclusive illustrative form (`queue.rs:19-20` "the CLI renders it,
  Plan 5's Tauri shell forwards it" - same shape, unchallenged by the
  Task-2 review). Flagging this would indict `:19-20` equally; neither
  misleads a core reader about the function.
- *Index-alignment and still-open-logger contract*: returns paragraph is
  byte-identical to `queue.rs:344-347` (my extraction + `diff`, empty);
  both callers build the document first and `finish` after (CLI
  `run.rs:219-248`, GUI `src-tauri/src/run.rs:466-467`). TRUE.
- *D23 reference*: D-numbers do not live in the v1 spec (grep for `D23`
  and control `D13` over the v1 spec: both 0 - the fire that exposed my
  own instrument's wrong assumption). D23 is
  `2026-07-10-plan-5-gui-design-decisions.md:46` "Thin event-forwarding
  shell; enumerated IPC surface; one run at a time", which decided the
  spawn-drain-re-emit pattern `run_batch` embodies. A design-history
  cite of the same kind as the file's D13/D14/D16/D25 cites; kept per
  the controller brief's own "change no factual claim that is currently
  true" list. Sound.
- *Teardown paragraph*: "everything a caller tracks about a run in
  flight is still in place when this function returns" - GUI: the
  active-run slot is still `Running` when `run_batch` returns; clearing
  is `finish_teardown` via `TeardownGuard::drop`. CLI: nothing is
  cleared by `run_batch` (cancel flag, outcomes all intact). TRUE for
  both; vacuous-but-true for a caller with no teardown code, which is
  the correct shape for a core doc.

**Dimension 2 - nothing true lost.** Rulings in section 3.

**Dimension 3 - latitude, both forms.** The fence contains no ellipsis,
no unenumerated set, no mandated-but-unwritten passage (grep for `...`
over my fence extraction: no hit; the instruction sentence mandates
"exactly this, transcribed character for character"). The rider's
instruction carries no explicit-permission clause (no "may reword", no
implementer choice); the one parenthetical - "`:327-347` at amendment
time; locate by the `pub fn run_batch` anchor, not the line numbers" -
is a deterministic locating instruction that pre-closes the line-drift
fork, not latitude. Tighter than the plan's Task-1 precedent (which
needed a wrapping carve-out): the fence is pre-wrapped at max 75 chars
(measured; the file's existing doc max is 77), so character-for-character
includes the wrapping. PASS.

**Dimension 4 - coverage and one voice.** My own sweep `grep -rn "moves
as-is"` over `docs/` and `.superpowers/`: in the design, exactly one
mandating hit (`:476`), now qualified in place by the inserted
parenthetical (`:478-480`); the second design hit (`:1745`) is the log
entry quoting the phrase. Reproduces the author's "exactly one hit, now
qualified". Section 5's D96 bullet (`:1483-1488`) carries the rider
pointer; the D96 body's surviving quote "`finish` needs the very
document it is about to persist" (`:504-505`) stays true against the
fence (byte-identical paragraph). A `rustdoc`-token sweep over the
design finds no other consumer of the old doc's content; the v1 spec
never mentions `run_batch` (grep 0; fire control: 22 hits in the
design). The plan still carries the executed Task-2 mandate ("rustdoc
moved with it", plan `:193`) - that is the plan amendment's vehicle, not
this one's, and is listed in section 5 below. The Round-4 log entry
matches the established shape (Round 3's "OWNER-RULED AMENDMENT n,
timing (routing: ...); n rulings, nothing else touched" pattern, with
the amendment-2-is-plan-only offset explained inline). PASS.

**Dimension 5 - completeness pass re-run.** My own instrument, broader
than the author's: case-insensitive superset pattern
(`tauri|shell|window|start_run|frontend|gui|cli|ipc|emit|render|plan
5|collector|production`) over the whole of `queue.rs` -> 53 hits
(`sweep-queue.txt`), fire-confirmed on the defective block itself, which
is still on disk until Task 3 (hits at `:332`, `:334`, `:335`, `:340` -
matching the rider's enumeration). Every other hit classified: the
`:19-20` module-form illustration (still true; "a future --json-events"
also re-checked - no `json.events|json_events` anywhere in
`crates/muxsmith-cli/src/`, grep exit 1 measured on grep itself, so
"future" still holds); the "e.g." illustrations at `:87`, `:102`,
`:166-167` (the CLI genuinely shares the ctrlc `Arc`, `run.rs:172-177`);
the D25 rationale mentions of "a GUI" (`:177-178`, test docs
`:1041-1044`) - generic and true; "window" at `:408/:481/:486/:1006/:1238`
is the race-window sense; all `#GUI#` hits are mkvmerge's output
protocol in fixtures and `JobEvent` docs. I also verified the author's
eleven top-level `///` span boundaries against the file
(`:19-20, :71, :83, :86-93, :164-180, :327-347, :371-378, :383-384,
:387-393, :402-423, :465-469` - each ends one line above its item).
**I agree: nothing else in `queue.rs` went caller-stale.** Nothing the
author's instrument missed.

**Dimension 6 - house conformance.** Fence line width max 75 vs file doc
max 77 - conforms. The " - " aside form already exists in the file's
docs (e.g. `:174`), so no style outlier. Link set extracted from both
the old block and the fence and diffed: identical five
(`[`JobSpec`] [`JobEvent`] [`run_queue`] [`Spawn`]
[`RunLogger::finish`]`), all core-resolvable today (same set currently
compiles under the gate's `RUSTDOCFLAGS="-D warnings" cargo doc`), so no
new link surface and no resurrected link - `finish_teardown` appears
nowhere in the fence, and the old block's mention was already a plain
code span (`proc-48-docsurface-delink`, `docs/decision-ledger.yaml:1700`,
re-verified at that line with the 2026-07-28 occurrence). Typography:
my scan of all amended spans (`:475-650`, `:1483-1489`, `:1721-1746`)
for em/en dash, curly quotes, Unicode ellipsis, NBSP, Unicode minus -
zero hits, pattern fire-verified against an em-dash sample (count 1).
PASS.

**Dimension 7 - consequences for the plan amendment, both verified.**
(a) Task 3's EXHAUSTIVE Files list (plan `:220-232`) names
`crates/muxsmith-core/src/executor/queue.rs` at `:222`, and Step 10's
`git add` line (`:254`) already stages it - so one added instruction,
a "Read first" addition (the current line `:218` names D98/D99/D100 but
not D96's rider), and NO new file is correct. (b) The controller brief
(`amendment-3-brief.md:127-128`, "that file is in Task 3's Files list")
is indeed wrong: `src-tauri/src/run.rs` is absent from `:220-232`; the
nearest entry is the compiler-sweep line (`:224`), which is EXEMPLARY
and conditional on `JobOutcome` literals, not a named listing. The
author's correction stands. Additionally verified: the author's claim
that the plan already binds Task 3 to the design's amendment log at
execution time is literal plan text (Global Constraints, "EVERY entry in
its `## Amendment log` bind this plan, at the log's state at EXECUTION
time").

## 3. Rulings on the two named drops

**Drop 1 - "a plain collector in tests": correctly dropped; nothing true
lost.** The clause is falsified inside core's own test module: of the
two moved tests, `queue.rs:1376` passes a collecting closure
(`collected.push(e.clone())`) and `queue.rs:1414` passes a no-op
(`|_| {}`) - both verified by direct read. As a universal it is false in
the new home; as an illustration it survives generalized in "directly
unit-testable with a scripted [`Spawn`]", which is the fact the clause
existed to carry. No still-true fact is lost.

**Drop 2 - the `finish_teardown`/D31 rationale: correctly dropped, and a
core reader is left with enough.** The rationale survives caller-side at
all three claimed sites, each verified by read: `finish_teardown`'s doc
(`src-tauri/src/run.rs:641-650` - "an empty slot always means 'teardown
fully complete'", the summary.json-loss rationale, D23/D31 cites),
`TeardownGuard`'s doc (`:656-667` - exactly-once teardown on normal end
or unwind), and the runner-thread comment at the call site (`:447-454` -
"Teardown is complete only once this drops ... never earlier, or a
confirmed quit could kill the process before summary.json is written").
The reachability impossibility is real, re-verified independently:
`fn finish_teardown` at `:651` has no `pub`, `mod run;` at
`src-tauri/src/lib.rs:23` is private, and
`crates/muxsmith-core/Cargo.toml` declares no workspace or gui
dependency at all (full `[dependencies]` section read; fire control:
the same `tauri` pattern hits 6 in `src-tauri/Cargo.toml`). What core
keeps - "performs no teardown of caller-side run state ... clearing it
is the caller's job, documented where its teardown code lives" - is the
complete contract of the FUNCTION; the slot/summary.json sequencing is a
GUI invariant core cannot name reachably. A fourth restatement would
duplicate three existing docs. Correct cut.

## 4. Evidence appendix

Scratch root:
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a3rev-independent/`

- `amendment-3.diff` - `git show 08621cb` (diff read; content graded at
  the current files, not the hash).
- `old-block.txt` / `fence.txt` - my own extractions (`sed` from
  `queue.rs:327-347` and design `:576-600`).
- `old-returns.txt` / `fence-returns.txt` - `diff` empty: returns
  paragraph byte-identical.
- `links-old.txt` / `links-new.txt` - `grep -o '\[`[^`]*`\]' | sort -u`,
  `diff` empty: link set identical.
- `sweep-queue.txt` - 53-hit superset caller-vocabulary sweep, all
  classified; fires on `:332/:334/:335/:340`.
- `jsonevents.txt` - empty; grep exit 1 measured on grep itself (first
  attempt measured `head`'s exit through a pipe and was discarded as a
  broken instrument).
- `amended-spans.txt` - typography scan target; banned-glyph pattern
  fire-verified (em-dash sample, count 1).
- Line widths: `awk length` over `fence.txt` (max 75) and over
  `queue.rs`'s `///` lines (max 77).
- D23: located via repo-wide spec grep after the v1-spec grep returned
  0 for both D23 and the D13 control (the D-registry lives in the
  plan-N decisions files, not the v1 spec).

## 5. HARVEST

- **Confirmed brief defect (already surfaced by the author, verified
  here):** the controller brief asserted `src-tauri/src/run.rs` sits on
  Task 3's exhaustive Files list; it does not (plan `:220-232`). The
  next dispatch (plan amendment) must not inherit that premise.
- **The plan amendment must carry:** (1) the one queue.rs instruction
  (replace the `///` block above `pub fn run_batch` with D96's
  amendment-3 fence, verbatim, locate by anchor); (2) Task 3 "Read
  first" gains D96's amendment-3 rider; (3) no Files-list change.
  Recommended, not required: extend the queue.rs Files-entry
  parenthetical (plan `:222`) with the rustdoc replacement so the
  entry's work description stays complete - optional under the
  file-vs-within-file ruling (`d7fd277`: an entry without an explicit
  within-file qualifier does not constrain within-file work), but cheap
  insurance against a reviewer reading the parenthetical as exhaustive.
  The plan amendment should also decide explicitly whether Task 2's
  executed Step-1 text ("rustdoc moved with it", plan `:193`) gets a
  historical qualifier or is left as record - either is defensible; a
  silent leave-as-is invites the same "moves as-is" misreading the
  design side just closed.
- **Hash-pinning drift, second instance in this amendment cycle:** my
  dispatch named `08621cb` as the amendment; `d7fd277` had already
  landed on top when I started (the author's cycle had the same shape
  with `e592f55`/`851ada1`). The read-files-not-hashes instruction in
  both briefs absorbed it; the ledger direction the author proposed
  (pin dispatches to content, or re-issue after any amend) is
  supported by a second data point.
- **Precedent worth keeping:** the "its human milestone lines" ruling
  (section 2, dimension 1) - the module's non-exclusive illustrative
  form (`queue.rs:19-20`) is the house shape for caller mentions in
  core docs, and the "human" qualifier is what keeps the CLI
  illustration true under `--json`. A future third caller (e.g. a real
  `--json-events` stream) does not re-falsify the fence.
- **Instrument hygiene, two self-caught failures worth the reminder:**
  an absence check piped through `head` measures `head`'s exit, not
  grep's; and a fire control can itself be wrong about where the
  ground truth lives (D13 was as absent from the v1 spec as D23 -
  the registry is elsewhere). Both were caught only because the
  pass-is-absence rule forced a fire; neither reached the verdict.
