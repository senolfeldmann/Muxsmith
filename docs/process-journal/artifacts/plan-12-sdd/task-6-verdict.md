# Task 6 verdict (independent review)

Reviewer instruments live outside the repo at
`/tmp/claude-1000/-home-senol-agents-peter/a1386daa-bdbc-4366-b18d-375daf90cf89/scratchpad/muxsmith-copy/`
(a full `rsync` of the repo at `a47fc19`, `node_modules` reused since `pnpm-lock.yaml` is
byte-identical to the repo's) and `.../scratchpad/muxsmith-base-ed1a635/` (a `git archive`
snapshot of the pre-task commit, used once for a baseline test count). Every in-tree
mutation below was applied with `Edit`/`python3`, rebuilt where frontend source changed
(`pnpm build` before every e2e run touching `src/`), run, then restored and verified
**by content** (`diff` against the real repo file, not `git status` or exit code) before
the next one. The real repo (`/home/senol/Git/Muxsmith`) was never written to; it served
only as the diff/content source for restore verification.

## Verdict 1: spec compliance

**MET, requirement by requirement, against `task-6-brief.md`.**

- **Step 1 (`AppState` fields, two commands).** MET. `editor_dirty: AtomicBool`
  (default `false`), `dialog_locale: Mutex<String>` (default `"en"`), each doc-commented
  with what it mirrors, sole-writer, and failed-sync cost (see Finding 2 below on one of
  these costs). `set_editor_dirty`/`set_shell_locale` registered in the one
  `invoke_handler` list beside `set_settings`. No `sys-locale` anywhere: `git diff
  ed1a635..a47fc19 --stat -- Cargo.toml Cargo.lock src-tauri/Cargo.toml crates/` is empty,
  confirmed myself.
- **Step 1b (locale table, row/chain split).** MET. `DE_GUI_COMMON`/`LOCALES` match the
  brief's exact form and `crates/muxsmith-cli/src/i18n.rs`'s own `LOCALES` shape (read
  directly: two-tuple vs the CLI's three-tuple, for the stated reason - one catalog file
  per locale here). `lookup_in`'s body is the old `ftl_message`'s body verbatim with
  `.unwrap_or(key)` removed (byte-diffed against `git show ed1a635`). New `ftl_message`
  collapses to primary subtag, walks `[requested, en]`, falls back to `key` - matches the
  brief's prose exactly.
- **Step 2 (four-variant `CloseDecision`, `close_decision`, dialog wiring).** MET. The
  fenced closure `abort_and_quit(&app.state::<AppState>(), |code| app.exit(code))`
  appears byte-identical at both run-bearing sites (`command grep`-verified). Four-row
  match in `close_decision` verified both by direct read and by two of my own inverting
  mutations (below) that only the intended new unit tests catch.
- **Step 2b (`reconfirm_decision`).** MET. Signature and doc comment byte-identical to
  the brief's fenced block (diffed directly). Wired in `on_close_requested`: re-read
  happens once, inside the confirming branch, before `abort_and_quit`/`app.exit(0)`;
  `None` proceeds as originally answered; `Some(v)` shows `v`'s own dialog, terminal.
- **Step 3 (catalog, both locales).** MET. All twelve new lines (six en, six de)
  byte-identical to the brief's fenced blocks (diffed directly, zero deltas). The de
  header note no longer claims the `close-abort-*` strings are unshown to a de user; it
  now states the true, current mechanism (locale-aware lookup via the `LOCALES` table) -
  read directly, matches reality.
- **Step 4 (frontend syncs).** MET. Both watchers match the brief's fenced shape modulo
  Prettier line-wrapping (diffed token-for-token). `{ immediate: true }` present on the
  locale watcher, commented as load-bearing with the correct reason (`main.ts` applies
  locale before mount).
- **Step 5 (Rust tests, three groups).** MET as built and as run - see Findings 2 and 3
  for one incorrect doc-comment claim and one report miscount, neither of which is a
  brief-compliance failure. All three groups exist, and I independently reproduced every
  claimed result:
  - Extended dialog-string test: ten ids in the loop (all non-literal, matching the
    dispatch's pre-adjudicated fact), pinned en wording kept, `"en"` passed explicitly.
  - Four `close_decision` cases: I mutated one arm of the match (`(false, true) =>
    CloseDecision::Close` instead of `ConfirmDiscard` - an inverting mutation, the
    dangerous direction: silent data loss) and only
    `close_decision_confirms_discard_while_idle_and_dirty` caught it (43 passed, 1
    failed). Restored, content-verified, re-ran green.
  - Twelve-cell `reconfirm_decision` matrix: hand-derived the full 3x4 truth table myself
    from the documented semantics and it matches all twelve `assert_eq!` calls exactly.
    I also ran my own inverting mutation, distinct from the report's (dropped only the
    `dirty`-strengthening clause rather than both negations): caught cleanly by
    `reconfirm_decision_fires_exactly_on_a_strengthening`, restored, verified.
  - Three-part shell parity test: I independently reproduced **all three** of the
    brief's prescribed red states, from scratch, in my own copy:
    - `de` row -> en catalog: exactly (c) failed, 43 passed/1 failed. Matches the report
      byte for byte.
    - Delete `close-discard-title` from the de catalog: exactly (b) failed, 43/1.
      Matches.
    - Delete the `de` row from `LOCALES`: (a) **and** (c) both failed, 42 passed/2
      failed. Matches the report's "two failures from one mutation" claim exactly.
    - Every mutation restored and content-diffed against the real repo file before the
      next; full suite re-ran green (44/44) after each restore.
- **Step 6 (wire tests).** MET. Read directly in the diff and independently exercised
  (see Q2): `set_editor_dirty` asserted `[true, false]` in order; `set_shell_locale`
  asserted `["en", "de"]` against concrete values, not "whatever was applied."
- **Step 6b (allowlist).** MET. I independently reverted `RUST_ONLY_IDS` to the
  pre-task four-id set and ran `node scripts/check-i18n.mjs` myself: it reported exactly
  the six new ids as unused, byte-identical to the report's paste. Restored,
  content-verified, re-ran clean.
- **Step 7 (verification).** MET. I independently ran `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace
  --all-targets --target x86_64-pc-windows-msvc -- -D warnings` (no linker needed),
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items`,
  and `cargo test -p muxsmith-gui --lib` - all green in my own copy. `git diff --stat`
  covers exactly the nine Files-list files (read directly from the diff header).
- **Step 8 (commit).** MET. Subject line byte-identical to the brief. Unsigned
  (`git cat-file -p` shows no `gpgsig` field, consistent with SI-4). Correct file set
  staged.

**Must not decide.** Checked every item in the list against the diff; all compliant -
one prompt from the four-row matrix, single strengthening-only re-read before the
action, no fifth message, decline-returns-unarmed (verified: `show_close_dialog`'s
`on_confirm` only fires `if confirmed`), combined case's own message, discard-only
exits via `app.exit(0)` (not `abort_and_quit`), six fenced strings verbatim in both
locales, shell told not resolving (no `sys-locale`), CLI-shaped locale table,
source-derived key set, `check-i18n.mjs` scoped to the allowlist only (full diff of that
file read directly - no other hunk), CLI gap surfaced not fixed, both frontend syncs
tolerant, dialog-string enumeration extended.

## Verdict 2: task quality

**0 Critical, 2 Moderate, 2 Low.**

**Finding 1 (Moderate) - the plan's acceptance-map row W4-m claims a producer that does
not exist.** `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`'s
acceptance map names W4-m ("Confirming a discard-only close quits; cancelling does
not") as machine-verified ("yes"), producer "the dialog-callback unit coverage." I
grepped every call site of `confirm_close` and `show_close_dialog` in the shipped
source: both are called **only** from `on_close_requested`, never from any
`#[cfg(test)]` function, and the crate has zero Tauri mock-app test infrastructure
(`command grep`-confirmed - no `tauri::test`, `mock_app`, or `MockRuntime` anywhere
under `src-tauri/`). So nothing machine-verifies that a confirmed `ConfirmDiscard` close
actually calls `app.exit(0)` and a declined one does not - the identical Tauri-runtime
limitation the **same acceptance map** honestly discloses for the neighbouring row
W4-w ("no, by nature... the matrix proves the rule, never that it is called"). This is
**not** a defect in the implementer's fidelity to `task-6-brief.md`: that brief's own
Step 5 never prescribes a callback-wiring test, and every test it does prescribe is
built and passes. It is a plan-level acceptance-map inaccuracy that this task's
completion should have surfaced back to the controller (matching how W4-w is already
honestly labeled) rather than letting an inaccurate "yes" stand.

**Finding 2 (Moderate) - the `editor_dirty` field's doc comment overclaims a safety
property the mechanism does not have.** `src-tauri/src/lib.rs`'s `AppState::editor_dirty`
doc comment: "A failed sync leaves it stale: the close-with-unsaved-changes warning can
be missed, never shown where nothing is at risk." The second half is false. `editor_dirty`
is a plain `AtomicBool` with a single fire-and-forget push per `dirty` transition and no
retry (`.catch(() => { /* background bookkeeping */ })` is a pure no-op). If the
**false**-transition push fails (user saves, but the IPC call to clear the shell's flag
is lost), the shell retains `true` while the editor is actually clean - `close_decision`
then returns `ConfirmDiscard` and the "Unsaved changes... quit and lose them?" dialog
fires with nothing to lose, which is exactly "shown where nothing is at risk." The
brief itself (Step 1) only asks for "what a failed sync costs (a stale flag, or a stale
dialog language - never a missing dialog)" - the "never a missing dialog" clause is
true and was written for the **locale** field (`ftl_message` always resolves to at
least the raw key, structurally, so a dialog can never be textually empty). The
implementer extended that same "never X" phrasing pattern to the dirty flag, where the
analogous absolute claim does not hold given the field's actual (symmetric,
retry-less) failure mode. Not a functional bug - the underlying stale-flag tradeoff is
the plan's own accepted design (D109 rationale: "the alternative, surfacing an error
dialog from a bookkeeping write, is worse") - but a false invariant claim in permanent,
shipped documentation that a future maintainer could rely on.

**Finding 3 (Low) - the report's own test-count claim contradicts its own
enumeration and the measured delta.** Section 4 claims "all seven net-new test
functions this task adds," then parenthetically names exactly six. I measured
independently two ways: `git diff ed1a635..a47fc19 -- src-tauri/src/run.rs | grep
'^+.*#\[test\]'` returns exactly six added lines, and I built and ran the pre-task
commit (`ed1a635`, via `git archive`) myself - `cargo test -p muxsmith-gui --lib`
reports 80 passed there against 86 on the final tree, a delta of exactly +6, matching
the diff count. `proc-normative-count-recomputed`'s trigger ("you are typing a number
that summarizes a list") applies directly; the count should have been six.

**Finding 4 (Low, procedural) - the e2e fixture repair (report section 3) is
substantively correct but falls outside the implementer's standing unilateral-edit
grant.** I independently reproduced both of the report's claims by mutation: with the
**original** `.fill(".*")` restored and Save's own `savedSnapshot.value =
JSON.stringify(profile)` line disabled (simulating a Save that never actually clears
the save state), the pre-existing test still **passed** - a genuine false green. With
the task's fix (`.fill(".+")` plus the matching `saveArgs.profile.input.pattern`
assertion) in place under the same broken-Save mutation, the test correctly **failed**.
So the repair is right and strictly increases what the test proves. But it mutates an
existing test's typed value and its assertion's expected literal, which
`latitude-carveout-zero-content-structural-forks` (`docs/process-conventions.yaml`)
names explicitly in its stop list: "weakening, deleting, skipping or rewording an
existing assertion, mutating existing fixture values... all stop." That entry's own
recorded precedent (2026-07-28, plan-9 Task 2, an argv fixture change) resolved the
identical class exactly this way: "the outcome is harmless, verified by the reviewer's
own mutation run... so the code stands and the finding is the ROUTE." Same
disposition here: the code stands, and the finding is that this should have returned
as NEEDS_CONTEXT rather than being decided at the keyboard, notwithstanding the
implementer's transparent disclosure of it in the report.

No other findings survived verification. Typography (no em-dash/en-dash/curly
quotes/ellipsis in any added line, `command grep -nP` swept the whole diff), German
orthography in all six new de strings, the `comments-locate-by-symbol-never-by-line-
number` and `a-document-never-cites-a-line-number-inside-itself` house entries, every
`a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact` cross-reference I
checked (the CLI-shape claim, the pre-existing non-literal call-site claim, the
marker-pollution self-check), `gitignored-paths-need-command-grep` (the report's one
bare `grep` targets a single tracked file, not a recursive sweep - not the trap case),
and `frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run` (I rebuilt before
every e2e run touching `src/`, and the report documents doing the same) were all
checked and held.

## Adjudication answers

**Q1.** Reasoning is correct, verified empirically (not just argued): I restored the
pre-task literal form (`ftl_message("no-such-key")`/`ftl_message("close-abort")`) and
ran the parity test - `every_row_carries_every_key_the_shell_source_literally_looks_up`
failed **in the correct, unmutated state** ("locale \"en\" has no non-empty value for
key \"no-such-key\""), because `close-abort` is a genuine strict prefix that
`lookup_in` correctly refuses to match. Rebinding to local variables is the right
repair - it is semantically neutral (same `&'static str` value reaches `ftl_message`
either way; I confirmed the pre-existing test's assertions are byte-identical before
and after) and does not weaken what the test asserts. It was a change the task was
entitled to make without a fork: it touches no user-visible behavior, and it was
necessary to satisfy this task's own prescribed, load-bearing requirement (Step 5: "the
derivation must not be replaced by a literal list") - leaving it undone would make the
brief's own required test fail on a correct tree. Reported transparently, as directed.

**Q2.** Both claims verified empirically, not just reasoned through. Coincidence:
confirmed via `EditorView.vue`'s actual mechanism (`dirty = savedSnapshot !==
history[position]`, both `JSON.stringify` of the whole model) - reverting the pattern to
its original loaded value necessarily re-serializes identically. False-green: with the
original `.fill(".*")` restored **and** Save's snapshot-clearing line disabled, the
built-and-run test still passed (`npx playwright test` on my own rebuilt bundle). Repair
ruling: changing the typed value to `.+` is the right fix - it is the minimal change that
removes the coincidence, and I confirmed with the same broken-Save mutation that the
fixed test now correctly fails. The changed assertion still asserts the same thing it
did before (Save writes the currently-edited pattern to `save_profile`'s payload), just
pinned to a different, now-non-coincidental literal. It does **not** fall within the
standing grant that lets an implementer extend an unbroken local pattern without a fork
(`latitude-carveout-zero-content-structural-forks`): that grant's stop list names
"mutating existing fixture values" and "rewording an existing assertion" explicitly, and
this edit does both. Per that entry's own recorded precedent for the identical class,
the correct disposition is: the code stands (it is verified correct and an improvement),
but this should have been returned as NEEDS_CONTEXT rather than decided unilaterally.

**Q3.** Confirmed at the tool, not just by reading visibility keywords. I turned both
plain code-spans into real intra-doc link syntax (`` [`run::close_decision`] ``,
`` [`run::ftl_message`] ``) and ran the actual gate command,
`RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-gui --no-deps
--document-private-items`: it failed with four "unresolved link" errors naming exactly
those two functions, confirming they are genuinely unreachable as intra-doc links
(private functions in a private `mod run`). Restored, re-ran, clean. The resulting plain
`` `run::close_decision` ``/`` `run::ftl_message` `` code-span text still names the exact
module and function a reader needs to grep for, so it does point at the right place,
just not as a clickable link.

**Q4.** Walked all sixteen of this task's acceptance rows (W4-h through W4-w, plus
W4-o/p/q/r/s/t) against `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-
findings.md`'s acceptance map. Fifteen have a real, verified producer: W4-h
(`set_editor_dirty` [true,false] ordering, e2e), W4-i/j/k/l (the four `close_decision`
matrix cases, one pre-existing extended, three new), W4-n (the extended ten-id
dialog-string test), W4-u/v (the twelve-cell `reconfirm_decision` matrix), W4-o/p (the
two `set_shell_locale` e2e assertions), W4-q/r/t (parity test parts b/a/c, all three
independently reproduced above), W4-s (the check-i18n before/after allowlist run), and
W4-w is honestly labeled non-machine-verifiable already. **W4-m is the one row whose
labeled producer does not exist** (Finding 1). The unchanged e2e total (101 before and
after, per the report) is fully explained and not itself suspicious: `smoke.spec.ts`'s
own `test(`/`test.` block count is identical before and after this task's diff (58 =
58, measured directly against both commits) - every one of this task's e2e observables
(W4-h, W4-o, W4-p) is a new assertion added inside two pre-existing test cases, exactly
the "fine and expected" shape the brief names, and no other e2e spec file was touched.

## Harvest

- **An acceptance-map row can claim a producer that structurally cannot exist, and the
  giveaway is a neighbouring row that already admits the same limitation.** W4-m and
  W4-w describe the same OS-dialog-callback surface; W4-w's own text names exactly why
  it cannot be unit-tested (Tauri runtime needed), and that reasoning applies unchanged
  to W4-m's "confirm quits, cancel doesn't" claim. When one row in a map explains its
  own non-verifiability, checking every row that shares its underlying mechanism is
  cheap and catches this class before it ships.
- **A "never X" absolute-safety phrase attached to one field's doc comment does not
  transfer to a sibling field with a superficially similar shape.** The locale field's
  "never a missing dialog" is structurally guaranteed (total fallback to the raw key);
  the dirty flag's "never shown where nothing is at risk" is not (a symmetric,
  retry-less boolean has no such asymmetry). Writing two sibling fields' failure-cost
  comments side by side invites exactly this kind of unearned pattern-completion.
- **Two of the implementer's own three raised concerns (Q1, Q2) both trace back to the
  same underlying tension** - a task that must derive a test set from source, and a
  pre-existing probe/fixture value that happens to collide with what the new derivation
  or the new observable needs - and both were handled the same good way (fixed
  correctly, disclosed in the report) but land on opposite sides of the fork boundary
  once measured against `latitude-carveout-zero-content-structural-forks`'s literal stop
  list (probe-literal rebinding: outside "fixture value," inside the grant; typed-value
  fixture change: squarely "mutating existing fixture values," outside it). Worth a
  standing example pair in that entry's own guidance, since the two are easy to conflate
  at the keyboard under time pressure.
- No boundary in the brief forced a stop on a fork that, in my judgment, had no real
  decision content - Step 2b's "no fifth message... that is NEEDS_CONTEXT" fence was
  never triggered (no fifth message was needed), and every other explicit fence in the
  brief was respected without producing a dead-end.

Verdict file written to `.superpowers/sdd/plan-12/task-6-verdict.md`.
