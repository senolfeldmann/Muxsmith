# Task 5 report - Plan 9 (D102, D103; spec S-7)

**Status: DONE_WITH_CONCERNS.** Every step landed, the whole verification bar
is green, and the two colour predictions were confirmed as stated. The status
is not plain DONE for one reason: the four-condition test-coverage rule fired
and I BUILT a third Rust test the plan does not enumerate (concern 1). It is
additive, in a listed file, on existing infrastructure, and it covers a gap I
measured rather than argued.

Commit `e134fdc`, 7 files, 286 insertions / 24 deletions. Tree clean.

---

## Per-file changes against the Files list (EXHAUSTIVE list, all 7 touched, nothing else)

| File | Change |
|---|---|
| `crates/muxsmith-core/src/report/mod.rs` | `pub fn severity_sorted` added after `worst_severity`, with D102's doc comment transcribed character-for-character; `use std::cmp::Reverse;` added (required by that addition); the Step-2 order test `severity_sorted_orders_errors_first_stable_within_severity` in the existing `#[cfg(test)]` module |
| `crates/muxsmith-core/src/report/json.rs` | private `rendered_diag(d, renderer)` factored out; `rendered_diags` now delegates to it; `batch_document` and `config_only_document` each build `config_diagnostics` from `severity_sorted(config_diags).into_iter().map(rendered_diag)`; `use crate::report::{Diagnostic, severity_sorted};`; doc paragraphs on both builders stating the sort and the scope boundary |
| `crates/muxsmith-cli/src/commands/mod.rs` | the `pub(crate) fn severity_sorted` definition and its doc deleted; `pub(crate) use muxsmith_core::report::severity_sorted;` in its place; `use std::cmp::Reverse;` deleted (now unused); no call site touched |
| `crates/muxsmith-cli/tests/dry_run_cli.rs` | Step 3's `dry_run_and_validate_json_agree_on_config_diagnostics_ordering`; **plus** `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran` (the four-condition addition, concern 1) |
| `src/views/BatchView.vue` | line 225, D103's line verbatim; nothing else |
| `e2e/smoke.spec.ts` | ONE addition inside `batch view: dry run`, after the apply-flow test: the `parseFailureOnLoad` fixture + the parse-failure apply scenario. Task 3's and Task 4's regions untouched |
| `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` | S-7 only: the design's sentence appended to spec 5.2's paragraph above the table (line 255) |

No house-knowledge YAML touched. No file outside the list touched (`git diff --stat` in the evidence below is the proof).

## Anchors re-derived at dispatch (proc-57)

All three of the brief's pointers reproduced exactly; no divergence.

- `batch view: dry run` describe opens at `e2e/smoke.spec.ts:140`.
- the apply-flow test is at `:460` (the brief's re-derivation, not the plan's stale `:406`).
- `src/views/BatchView.vue:225` carried `const parseDiagnostic = doc.config_diagnostics[0];` and was the file's only hit.

## Evidence

### The absence check, fired before it was trusted (Step 7)

Instrument is ugrep 7.5.0, so the pre-state run is what licenses the zero.

```
$ grep --version | head -1
ugrep 7.5.0 x86_64-pc-linux-gnu +sse2; -P:pcre2jit; -z:zlib,bzip2,zstd,brotli,7z,tar/pax/cpio/zip

PRE-EDIT (must hit):
$ grep -n 'config_diagnostics\[0\]' src/views/BatchView.vue
225:      const parseDiagnostic = doc.config_diagnostics[0];
exit=0

POST-EDIT:
$ grep -n 'config_diagnostics\[0\]' src/views/BatchView.vue
exit=1 (1 = no match)
$ grep -c 'config_diagnostics\[0\]' src/views/BatchView.vue
0
$ grep -rn 'config_diagnostics\.find' src/views/BatchView.vue
225:      const parseDiagnostic = doc.config_diagnostics.find((d) => d.code === "parse-error");
```

### Step 3's red-then-green (its fire verification)

The parity test was written and run FIRST, on the unmodified tree.

RED on today's tree, and it reproduces the plan's authoring probe on both sides:

```
$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_and_validate_json_agree_on_config_diagnostics_ordering
running 1 test
test dry_run_and_validate_json_agree_on_config_diagnostics_ordering ... FAILED

thread '...' panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:386:5:
assertion `left == right` failed: dry-run and validate must order the same diagnostics identically
  left: ["raw-property", "raw-on-known-property", "unknown-property", "invalid-regex"]
 right: ["unknown-property", "invalid-regex", "raw-on-known-property", "raw-property"]

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out
```

GREEN after Step 1:

```
$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_and_validate_json_agree_on_config_diagnostics_ordering
running 1 test
test dry_run_and_validate_json_agree_on_config_diagnostics_ordering ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s
```

The plan's predicted end-state sequence is confirmed verbatim by the RED run's
`right` side (the validate envelope, already sorted today):
`unknown-property`, `invalid-regex`, `raw-on-known-property`, `raw-property`.
The plan's fixture YAML parses as written (unquoted `raw:x` / `raw:language`
keys in flow mappings), so no fixture deviation was needed.

### Step 2's order test

```
$ cargo test -p muxsmith-core --lib severity_sorted_orders_errors_first_stable_within_severity
running 1 test
test report::tests::severity_sorted_orders_errors_first_stable_within_severity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 121 filtered out; finished in 0.00s
```

### Step 5's e2e scenario: no red-today claim, and its actual discriminator demonstrated

Written BEFORE the BatchView edit and run against the unmodified positional
fetch. It passes, exactly as the brief states:

```
$ pnpm exec playwright test smoke.spec.ts -g "parse failure on apply"     # BEFORE the BatchView edit
  ✓  1 [chromium] › e2e/smoke.spec.ts:554:3 › batch view: dry run › a parse failure on apply's load_profile
     surfaces the parse-error alert and invokes neither apply_suggestion nor save_profile (D103) (218ms)
  1 passed (686ms)
```

Its discriminating power is against a DEFECTIVE code-keyed rewrite, so that is
what I fired it against: with the predicate's string mutated to `"parse_error"`
(one character class off, the exact defect the test exists for), it goes red on
assertion (a):

```
    > 587 |     await expect(batch.getByRole("alert")).toContainText("The profile could not be parsed");
  1 failed
    [chromium] › e2e/smoke.spec.ts:554:3 › ... (D103)
```

Restored from a byte snapshot taken before the mutation (`command cp -f`, never
a bare `cp`), proven, and re-run green:

```
$ sha256sum -c BatchView.edited.sha256
src/views/BatchView.vue: OK
$ grep -n 'config_diagnostics\.find' src/views/BatchView.vue
225:      const parseDiagnostic = doc.config_diagnostics.find((d) => d.code === "parse-error");
  ✓  1 [chromium] › ... (D103) (205ms)
  1 passed (608ms)
```

### The verification bar (Step 7, foreground, no subsets, final state)

```
### 1 cargo fmt --all --check
fmt: clean (no output above)

### 2 cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s

### 3 cargo test --workspace
   34 test binaries, every line "test result: ok", 0 failed anywhere
   (dry_run_cli.rs: 13 passed, was 11 before this task)

### 4 pnpm lint
$ eslint .          (no findings)

### 5 pnpm build
✓ built in 150ms

### 6 pnpm test:e2e
  ✓  38 [chromium] › e2e/smoke.spec.ts:554:3 › batch view: dry run › a parse failure on apply's load_profile
     surfaces the parse-error alert and invokes neither apply_suggestion nor save_profile (D103) (240ms)
  64 passed (2.8s)
```

### The two predictions the brief asked me to confirm or refute

1. **The design's consumers sweep (existing CLI JSON tests pass unchanged
   because they assert membership, not position): CONFIRMED.** No positional
   failure anywhere. `cargo test --workspace` needed no test relaxed, and
   nothing was returned as NEEDS_CONTEXT on this axis.
2. **Every pre-existing e2e suite passes unchanged: CONFIRMED.** 64 passed;
   the count is the pre-existing 63 plus my one added scenario, and no
   pre-existing test changed behaviour. (Task 6 consumes the e2e SPEC-FILE
   count: `ls e2e/*.spec.ts | wc -l` -> 9, unchanged by this task, so its
   "TENTH spec file" statement still holds.)

### Extra checks I ran beyond the bar (read-only, none required)

- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean. Worth
  running because this task writes rustdoc, including a transcribed doc comment
  whose `Info < Warning < Error` prose is deliberately un-backticked.
- The same with `--document-private-items`, because the doc-link ledger rule
  notes the plain gate is silent on private items: `muxsmith-core` and
  `muxsmith-cli` both clean; `muxsmith-gui` fails - see observation 1, it is
  pre-existing and not mine.

## The four-condition test-coverage check (tests-ship-with-the-feature-never-after)

Run explicitly, as instructed, with the result written down either way.

**It FIRED, once.** My diff applies the sort at TWO call sites,
`config_only_document` and `batch_document`. The plan's enumerated tests reach
the first one end-to-end (the parity test forces the no-mkvmerge path, which is
`config_only_document`) and pin the sort function in isolation. Nothing reached
`batch_document` - the builder every successful dry-run and every GUI report
document goes through.

I measured the gap instead of arguing it. With `batch_document`'s sort removed
and everything else intact, on this machine (`mkvmerge v100.0` installed, so
the mkvmerge-gated tests actually ran rather than self-skipping):

```
$ cargo test --workspace      # batch_document UNSORTED
   every binary "test result: ok", 0 failed
```

Total gap, the same shape as Task 3's. The four conditions:

| # | Condition | Result |
|---|---|---|
| 1 | Additive | YES - a new test function; no existing assertion, fixture value or helper touched |
| 2 | Rides existing infrastructure | YES - `dry_run_cli.rs`'s existing subprocess harness, its `have_mkvmerge()` gate idiom and the `support::muxsmith` funnel; no new file, harness or mock |
| 3 | Consequence created by this package's own diff | YES - `batch_document`'s sorted `config_diagnostics` is introduced by Step 1 |
| 4 | Named in the report for the reviewer | YES - concern 1 |

All four hold, so I BUILT it: `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`
in `crates/muxsmith-cli/tests/dry_run_cli.rs` (a LISTED file). Fire-verified
against the exact mutation that had left the workspace green:

```
$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran
thread '...' panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:455:5:
assertion `left == right` failed: errors first, then the warning, then the info
  left: ["raw-property", "raw-on-known-property", "unknown-property", "invalid-regex"]
 right: ["unknown-property", "invalid-regex", "raw-on-known-property", "raw-property"]
test result: FAILED. 0 passed; 1 failed
```

and green on the intended end state (`test ... ok`). `json.rs` was restored
from a byte snapshot and verified (`sha256sum -c` -> OK).

**Where the rule did NOT fire, checked and recorded:**

- **The GUI display-order consequence** (D102's swept consequence: EditorView's
  diagnostics list, DiagnosticsPanel, BatchView's general-diagnostics list now
  show errors first). Condition 3 FAILS. The ordering is produced entirely in
  core; the frontend renders whatever order it receives and no frontend code
  path changed. e2e fixtures are mock documents that never pass through core,
  so a mock-IPC scenario asserting list order would assert only that the DOM
  mirrors a hand-written array - true before this diff as well, i.e. vacuous
  with respect to it. The producers that can actually observe the consequence
  are the core unit test and the two subprocess tests, all built.
- **D102's scope boundary** (per-file `diagnostics` and `batch_diagnostics`
  stay unsorted). Condition 3 FAILS: this is preserved behaviour, not a
  consequence the diff creates. No guard exists for it today - see observation 2,
  surfaced rather than built.

## Divergences and judgment calls, each named

1. **A third Rust test, not enumerated by the plan.** The four-condition
   addition above. This is the one place where the plan's test enumeration was
   not followed literally; the Tier-2 precedence rule the brief cites is what
   licensed it, and concern 1 puts it in front of the reviewer.
2. **Position of the re-export.** The plan says the re-export goes "in its
   place"; I read that as "as its replacement" and put it in the import region
   after `use crate::i18n::Renderer;` rather than mid-file between two
   functions, which is where the deleted function's body was. The statement
   itself is verbatim from the design. Nothing else in the crate re-exports, so
   there was no house precedent to conform to; the ecosystem norm is imports at
   the top.
3. **A doc comment on the re-export.** Three composed lines saying why the CLI
   re-exports a core function. Not in the design (which specifies "zero wrapper
   code" - a doc comment is not code). Kept because every other item in that
   file carries a doc comment, and "one definition, call sites unchanged" is the
   non-obvious part a reader needs. Zero outward effect; delete on request.
4. **Doc paragraphs added to both builders in `json.rs`.** Composed, not
   design-fenced: each states that `config_diagnostics` is sorted and (on
   `batch_document`) why the per-file arrays are not. The scope boundary is a
   deliberate non-uniformity that D102 explicitly asks to be recorded "so the
   non-uniformity is a decision, not an accident"; the code is where the next
   reader meets it.
5. **The parity test asserts exactly the plan's two enumerated assertions**
   (sequence equality, then the two leading error codes) and does NOT assert
   exit codes. Exit status travels in the JSON-parse panic context instead, so a
   crashed subprocess still fails loudly. The pair is self-guarding against
   vacuity: two empty arrays would satisfy the equality but fail the
   begins-with assertion.
6. **`mkvmerge_found` is omitted from the e2e parse-failure fixture.** The
   design enumerates `profile: null`, the singleton diagnostic and the three
   empty arrays, and does not name the key; the TS type has it optional; and
   `config_only_document`'s own doc says the key "stays absent on a profile-load
   failure". Omitting it is the faithful mirror of what core actually emits on
   that path. Stated because the sibling fixtures in that describe all set it.
7. **The `batch_document` producer is `have_mkvmerge()`-gated**, like five of
   its neighbours in that file: reaching `PipelineOutcome::Planned` requires a
   resolvable mkvmerge, by construction. The gate is the file's pre-existing
   idiom, not something I introduced, and CI installs mkvtoolnix on every leg,
   so it does not self-skip there. It needs no fixture MKV: an empty source
   directory keeps the batch empty while the config-time diagnostics still flow
   through `batch_document`.
8. **The e2e scenario keeps `apply_suggestion` and `save_profile` mocked.**
   D103 says "one substitution", and the negative assertion is only meaningful
   if the mocks exist and are recorded: absence then means the branch returned,
   not that the command was unmockable.
9. **Raw-string YAML fixture** (`r#"..."#`) rather than `dry_run_cli.rs`'s own
   escaped one-line form, so the plan's fenced profile is transcribed
   character-for-character. The idiom is house pattern in the same crate's test
   suite (`cli_validate.rs`).

## Numbered concerns a reviewer can rule on yes/no

1. **The unenumerated third Rust test.** Was building
   `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`
   correct under the four-condition precedence, or should the measured
   `batch_document` gap have returned as NEEDS_CONTEXT? The evidence for
   building it is above (all four conditions, plus a measured-green workspace
   with the sort removed and a fired red with the test present). Yes/no.
2. **The re-export's position and its doc comment** (divergences 2 and 3):
   in the import region with three composed lines of doc, versus bare at the
   deleted function's old position. Yes/no.
3. **The composed builder doc paragraphs in `json.rs`** (divergence 4). Yes/no.
4. **The parity test's assertion set** (divergence 5): exactly the two
   enumerated assertions, no exit-code pin. Yes/no.
5. **The e2e fixture omitting `mkvmerge_found`** (divergence 6). Yes/no.
6. **BatchView's else-branch text was left untouched.** After the code-keyed
   `find`, that branch fires for strictly more envelopes than before (D103:
   "strictly more detection than today"), so its comment ("An empty
   `config_diagnostics` here means core broke that contract") and its
   `console.error` string ("returned profile: null with no diagnostics") now
   describe only one of the branch's triggers. I did not touch either: the plan
   says "the existing else-branch `console.error` stays", the design considered
   the branch explicitly, and amendment 3 is the precedent that a doc/text
   falsified by a move is a design matter, not an implementer's keyboard fix.
   Ruling wanted: leave as is, or route a correction. Yes/no.

## What I surface for the controller

1. **Pre-existing rustdoc failure in `muxsmith-gui`, under
   `--document-private-items` only.** Not caused by this task (`git show --stat`
   lists no `src-tauri` file) and NOT part of the ten-part gate, which runs
   `cargo doc --workspace --no-deps` without that flag - so nothing is broken
   for the plan close. Recorded because the ledger rule
   `an-import-removal-sweeps-the-doc-links-that-named-the-symbol` explicitly
   names private-item invisibility as the blind spot this class hides in:

   ```
   error: `run` is both a function and a module
     --> src-tauri/src/lib.rs:54:21
   54 | /// `pub(crate)`: [`run`]'s `start_run` builds the same document shapes for
   error: `run` is both a function and a module
     --> src-tauri/src/lib.rs:87:15
   87 | /// once in [`run`] -- Tauri resolves `State<AppState>` for every command
   ```

   Two ambiguous links, fixable by `mod@run` / `run()`. Ledger-worthy in the
   direction of the gate itself: a private-items doc leg would have caught both.

2. **D102's scope boundary has no guard.** "Per-file `diagnostics` and
   `batch_diagnostics` stay in collection order" is now a documented contract in
   three places (D102, spec S-7, both builder docs) and in zero tests: widening
   the sort to all arrays would leave the whole suite green. It is out of this
   task's four-condition scope (preserved behaviour, not a consequence this diff
   creates) and I did not build it. A cheap producer exists if the controller
   wants one - a `batch_document` case with a mixed-severity `batch_diagnostics`
   vector asserting it is NOT reordered.

3. **The measured coverage shape is the Task-3 class again, one task later.**
   Task 3's gap was a persisted field with no producer; this one was the second
   call site of a two-site change, where the plan's acceptance walk named one
   producer for an observable with two halves. The plan-review handle recorded
   under `tests-ship-with-the-feature-never-after` ("every acceptance observable
   is walked in its HALVES to a named producing test") would have caught it at
   planning time: acceptance observable 6's "CLI `validate --json` and `dry-run
   --json` now agree on ordering for one mixed-severity fixture" reads as one
   emitter but the sort has two call sites. Worth a funnel entry as calibration
   data that the handle is aimed correctly.

4. **The doc-link sweep the removed `use` line triggers came back clean**, with
   a fired control so the empty result is evidence rather than an artifact:

   ```
   $ grep -rnP '\[`[^`]*severity_sorted[^`]*`\]|\[`Reverse`\]' --include='*.rs' crates/ src-tauri/
   crates/muxsmith-core/src/report/json.rs:33:/// [`crate::report::severity_sorted`] (D102), so every surface that builds a
   (the only hit is the link this task ADDED, and it resolves)
   $ grep -rnP '\[`[^`]*rendered_diags[^`]*`\]' --include='*.rs' crates/     # CONTROL: pattern does find links
   crates/muxsmith-core/src/report/json.rs:197:/// per-diagnostic mapping in this crate: [`rendered_diags`] applies it to
   ```

   The count-word sweep over the sets I extended also came back clean: the only
   count-shaped statements naming `dry_run_cli.rs` / `smoke.spec.ts` live in
   dated process-journal artifacts of past plans (historical records, not
   normative), `report/json.rs`'s module doc still describes three document
   builders (`batch_document`, `config_only_document`, `run_document` -
   `rendered_diag` is a private mapping helper, not a document builder), and no
   plan or design line states a count of Task 5's tests.

## Commit

`e134fdc`, on `master`, main worktree, unsigned, one trailer, no
`Claude-Session` line, staged file by file (never `git add -A`).

```
$ git show --stat HEAD
commit e134fdcd0624cb52276ff5724a53f7fe191a42c9
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 22:03:32 2026 +0200

    report: config_diagnostics sorts errors-first centrally; BatchView fetches parse-error by code + apply e2e (D102, D103, S-7)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/src/commands/mod.rs            |  14 +-
 crates/muxsmith-cli/tests/dry_run_cli.rs           | 145 +++++++++++++++++++++
 crates/muxsmith-core/src/report/json.rs            |  52 ++++++--
 crates/muxsmith-core/src/report/mod.rs             |  32 +++++
 .../specs/2026-07-08-muxsmith-v1-design.md         |   2 +-
 e2e/smoke.spec.ts                                  |  63 +++++++++
 src/views/BatchView.vue                            |   2 +-
 7 files changed, 286 insertions(+), 24 deletions(-)

$ git status --porcelain
(clean)
```

## Fix round

Fresh implementer, did not author Task 5. Scope was the two findings in
`task-5-fix-brief.md`, in exactly two files. Both sites read before editing;
both findings confirmed at the source. No design fork arose, so nothing came
back as NEEDS_CONTEXT.

**Commit:** `17505d8` - `report: Task-5 fix round - MEDIUM-1 and LOW-3`

### Fix 1 (MEDIUM-1) - `crates/muxsmith-cli/src/commands/mod.rs:16-19`

Doc-only. The appositive on the `severity_sorted` re-export claimed every call
site is "the human printing paths this crate owns". Confirmed false at
`crates/muxsmith-cli/src/commands/validate.rs:20-29`: the sort at line 21
produces the `diagnostics` vector that line 29 serializes into the `--json`
envelope AND the human loop at 33-36 iterates; the call site's own comment
("both output modes share it") already said so. Applied the brief's text
verbatim, ASCII `--`, no other line in the file touched.

### Fix 2 (LOW-3) - `crates/muxsmith-cli/tests/dry_run_cli.rs:445-448`

Added the missing half of what the comment at 443-444 promised. Confirmed the
gap: `mkvmerge_found`-absent alone does not identify `batch_document`, because
`config_only_document` (`crates/muxsmith-core/src/report/json.rs:95-114`) only
sets `mkvmerge_found` when its `Option<bool>` is `Some`, and the CLI's
profile-load-failure path passes `None`. New assertion sits above the existing
one, matching the comment's own order; the existing `mkvmerge_found` assertion,
the fixture, and everything below are untouched.

```rust
    assert!(
        report.get("files").is_some_and(|f| f.is_array()),
        "expected a planned batch document with a files array, got: {report}"
    );
```

### Committed diff

```
$ git show HEAD -- crates/muxsmith-cli/src/commands/mod.rs crates/muxsmith-cli/tests/dry_run_cli.rs
@@ -15,7 +15,8 @@ use crate::i18n::Renderer;

 /// The one error-first ordering definition, hoisted to core (D102) and
 /// re-exported here so every `crate::commands::severity_sorted` call site
--- the human printing paths this crate owns -- is unchanged.
+/// -- this crate's human printing paths and `validate`'s own `--json`
+/// envelope -- is unchanged.
 pub(crate) use muxsmith_core::report::severity_sorted;

@@ -442,6 +442,10 @@ tracks:
     });
     // Planning ran, so this document came from `batch_document`, not the
     // config-only shape: `files` is present and `mkvmerge_found` absent.
+    assert!(
+        report.get("files").is_some_and(|f| f.is_array()),
+        "expected a planned batch document with a files array, got: {report}"
+    );
     assert!(
         report.get("mkvmerge_found").is_none(),
         "expected a planned batch document, got: {report}"
```

(The first hunk's `-` line is the removed doc line `/// -- the human printing
paths this crate owns -- is unchanged.`; its leading `///` and the diff marker
collide in this paste. The authoritative form is `git show 17505d8`.)

### Verification (all foreground, full commands, no subsets)

```
$ cargo fmt --all --check
FMT_CLEAN                      (no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
    Checking muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Checking muxsmith-gui v0.1.0 (/home/senol/Git/Muxsmith/src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s

$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
doc exit=0
grep -ci warning -> 0
(--document-private-items deliberately NOT added, per brief)
```

`cargo test --workspace`, measured on the final restored tree (post-fire),
matches the brief's stated house unit exactly:

```
$ grep -c '^test result:' <run>            -> 39
$ grep '^test result:' <run> | grep -v '^test result: ok\.' | wc -l   -> 0
$ grep -o '[0-9]* failed' <run> | sort | uniq -c
     39 0 failed
$ grep -c "MKVMERGE" <run>                 -> 0   (no mkvmerge skips; mkvmerge
                                                   is on PATH at
                                                   /home/linuxbrew/.linuxbrew/bin/mkvmerge,
                                                   so the fix-2 test really ran)
```

39 `test result:` lines, all ok, 0 failed. Same count before and after the
fire. The zero skip markers matter for fix 2: the target test early-returns
when mkvmerge is missing, so without that check "green" would not prove the
new assertion executed at all.

### Fire for the new assertion (fix 2)

Mutated the document under test rather than the assertion, so the fire proves
the assertion is bound to the real `files` key of `batch_document`, not merely
that an `assert!` can panic. Baseline taken BEFORE mutating.

```
$ sha256sum <the two edited files> crates/muxsmith-core/src/report/json.rs
df358061576708a02fe1dca55d64390be10a8a26952a2a7f87de9513479f2111  crates/muxsmith-cli/src/commands/mod.rs
c5a5656b7124f393019d41039ddddbf042fb735fe404a9bed28c99013bd500b7  crates/muxsmith-cli/tests/dry_run_cli.rs
5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a  crates/muxsmith-core/src/report/json.rs

$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran -- --exact --nocapture
test dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out
```

Mutation: in `crates/muxsmith-core/src/report/json.rs:67`, `batch_document`'s
envelope key `"files"` renamed to `"files_MUTANT_FIRE"`. Re-run of the same
single test:

```
thread 'dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran' panicked at
crates/muxsmith-cli/tests/dry_run_cli.rs:445:5:
expected a planned batch document with a files array, got: {"batch_diagnostics":[], ...
 "files_MUTANT_FIRE":[],"suggestions":[]}
test dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out
```

Red at line 445 (the new assertion), with the new message, and the pasted
document confirms `files` was genuinely absent while `mkvmerge_found` was too,
so the neighbouring assertion would still have passed. That is exactly the
identification gap LOW-3 named.

Restore, non-interactive, and its proof:

```
$ git checkout -- crates/muxsmith-core/src/report/json.rs
$ sha256sum -c <pre-fire baseline>
crates/muxsmith-cli/src/commands/mod.rs: OK
crates/muxsmith-cli/tests/dry_run_cli.rs: OK
crates/muxsmith-core/src/report/json.rs: OK

$ git status --porcelain
 M crates/muxsmith-cli/src/commands/mod.rs
 M crates/muxsmith-cli/tests/dry_run_cli.rs

$ grep -rn "MUTANT_FIRE" crates/ src-tauri/
grep exit=1 (no hits)
```

No `pnpm` leg: the frontend is untouched by this round, per brief.

### Commit and final state

```
$ git show --stat 17505d8
commit 17505d8e360e20ae55d909d0db28ae41927d8fce
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 22:38:00 2026 +0200

    report: Task-5 fix round - MEDIUM-1 and LOW-3
    [body: one paragraph per finding, naming what was wrong]
    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/src/commands/mod.rs  | 3 ++-
 crates/muxsmith-cli/tests/dry_run_cli.rs | 4 ++++
 2 files changed, 6 insertions(+), 1 deletion(-)

$ git show -s --format=%B HEAD | grep -c "Co-Authored-By"   -> 1
$ git show -s --format=%B HEAD | grep -c "Claude-Session"   -> 0
$ git status --porcelain
(clean)
```

Committed with `git -c commit.gpgsign=false`, the two files staged by name, no
`git add -A`, not pushed.

### Found and not touched (outside scope, for the controller)

1. **The two assert messages now nearly coincide.** The new assertion says
   "expected a planned batch document with a files array", the pre-existing one
   "expected a planned batch document". Shaping the new one like its neighbour
   was the instruction and the pre-existing message was explicitly off-limits,
   so both stand. If the reviewer wants them disambiguated, the cheaper edit is
   to the older message (". . . without mkvmerge_found"), which is a one-line
   follow-up, not a defect.
2. **Nothing else was changed anywhere.** No drive-by cleanups, no doc edits
   outside `commands/mod.rs`, no test additions beyond the single named
   assertion. `git status` is clean and the commit touches exactly two files.
