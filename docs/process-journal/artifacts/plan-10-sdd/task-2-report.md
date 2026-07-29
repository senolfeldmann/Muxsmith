# Task 2 report - Plan 10 (W1: the D102 preserved-order producers, selected by measurement)

**Status: DONE_WITH_CONCERNS** (three numbered concerns below, none blocking; the
deliverable is complete and the full gate is green).

Worked on `master` in the main worktree, `/home/senol/Git/Muxsmith`. No branch, no
worktree, no session-relocation tool. All runs foreground. Head at start:
`39a9055`.

---

## Step 1: the precondition, pasted

```
$ mkvmerge --version
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
exit=0
```

The authoring machine's `mkvmerge v100.0` is the same binary, so the
`have_mkvmerge()`-gated `batch_document` sort guard is live rather than a no-op,
and M1's result is a real measurement of that half. The mkvmerge-absent fork does
not fire.

**Baseline of the mutated production file, taken BEFORE the first mutation:**

```
$ sha256sum crates/muxsmith-core/src/report/json.rs
5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a  crates/muxsmith-core/src/report/json.rs
```

---

## Step 2: the measurement

Four mutations, each applied alone to `crates/muxsmith-core/src/report/json.rs`,
each followed by a foreground `cargo test --workspace`, each restored with
`git checkout --` before the next. Nothing else in the tree was touched during the
measurement.

| # | Half (acceptance row) | The exact edit | Command | Pasted result | Failing test(s) | Disposition |
|---|---|---|---|---|---|---|
| **M1** | W1-a: `batch_document` sorts `config_diagnostics` errors-first | in `batch_document`, `severity_sorted(config_diags).into_iter().map(...).collect()` -> `rendered_diags(config_diags, renderer)` | `cargo test --workspace` | `EXIT=101`; `test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s` | `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran` | **RED -> already guarded.** Guarding test: `crates/muxsmith-cli/tests/dry_run_cli.rs::dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`. **No producer written** (P3 not written) |
| **M2** | W1-b: `config_only_document` sorts `config_diagnostics` errors-first | the same replacement in `config_only_document` | `cargo test --workspace` | `EXIT=101`; `test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s` | `dry_run_and_validate_json_agree_on_config_diagnostics_ordering` | **RED -> already guarded.** Guarding test: `crates/muxsmith-cli/tests/dry_run_cli.rs::dry_run_and_validate_json_agree_on_config_diagnostics_ordering`. **No producer written** (P4 not written) |
| **M3** | W1-d: `batch_document` leaves per-file `files[].diagnostics` in collection order | the per-file `"diagnostics"` value -> `severity_sorted(&f.diagnostics).into_iter().map(\|d\| rendered_diag(d, renderer)).collect::<Vec<_>>()` | `cargo test --workspace` | `EXIT=0`; 39 `test result: ok.` lines, zero `FAILED` / `failures:` markers, 503 tests passed in total | none | **GREEN -> unguarded.** Producer written: `report_json.rs::batch_document_preserves_per_file_diagnostics_collection_order` (**P2**) |
| **M4** | W1-c: `batch_document` leaves `batch_diagnostics` in collection order | the `"batch_diagnostics"` value -> `severity_sorted(&batch.batch_diagnostics).into_iter().map(\|d\| rendered_diag(d, renderer)).collect::<Vec<_>>()` | `cargo test --workspace` | `EXIT=0`; 39 `test result: ok.` lines, zero `FAILED` / `failures:` markers, 503 tests passed in total | none | **GREEN -> unguarded.** Producer written: `report_json.rs::batch_document_preserves_batch_diagnostics_collection_order` (**P1**) |

### The red runs, pasted in full detail

**M1**, `cargo test --workspace`, exit 101:

```
test dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran ... FAILED

---- dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran stdout ----

thread 'dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran' (2605474) panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:462:5:
assertion `left == right` failed: errors first, then the warning, then the info
  left: ["raw-property", "raw-on-known-property", "unknown-property", "invalid-regex"]
 right: ["unknown-property", "invalid-regex", "raw-on-known-property", "raw-property"]

failures:
    dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran

test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

error: test failed, to rerun pass `-p muxsmith-cli --test dry_run_cli`
```

**M2**, `cargo test --workspace`, exit 101:

```
test dry_run_and_validate_json_agree_on_config_diagnostics_ordering ... FAILED

---- dry_run_and_validate_json_agree_on_config_diagnostics_ordering stdout ----

thread 'dry_run_and_validate_json_agree_on_config_diagnostics_ordering' (2610156) panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:386:5:
assertion `left == right` failed: dry-run and validate must order the same diagnostics identically
  left: ["raw-property", "raw-on-known-property", "unknown-property", "invalid-regex"]
 right: ["unknown-property", "invalid-regex", "raw-on-known-property", "raw-property"]

failures:
    dry_run_and_validate_json_agree_on_config_diagnostics_ordering

test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s
```

Both red states are the expected coverage signal, **not an ordering defect**: in
each case the mutation deletes the sort and the guard observes exactly the
unsorted collection order that the deletion produces. The pre-mutation tree is
correct; nothing in the production code contradicts D102 or spec section 5.2.

### The green runs, how the "no failure" claim was derived

For M3 and M4 no single result line carries the verdict (39 test binaries run), so
the claim is derived by three independent measurements over each run's captured
log, all pasted:

```
$ cargo test --workspace > m3.log 2>&1; echo "EXIT=$?"
EXIT=0
$ grep -c "^test result: ok\." m3.log
39
$ grep -cE "FAILED|^failures:" m3.log
0
```

```
$ cargo test --workspace > m4.log 2>&1; echo "EXIT=$?"
EXIT=0
$ grep -c '^test result: ok\.' m4.log
39
$ grep -cE 'FAILED|^failures:' m4.log
0
```

The `FAILED` / `failures:` grep is not an unfired negative: the identical
expression run over `m1.log` and `m2.log` returns non-zero, and the two panic
blocks above are what it matches there. The counting instrument is therefore
demonstrably able to see a failure when one exists.

Total passed per run, same instrument on all three logs:

```
$ grep -oE "^test result: ok\. [0-9]+ passed" <log> | awk '{s+=$4} END {print s}'
m3.log (mutated, pre-producer tree): 503
m4.log (mutated, pre-producer tree): 503
g3.log (final gate run, producers landed): 505
```

**505 - 503 = 2**, exactly the two producers this task adds. No pre-existing test
changed its behaviour, which is the Step-5 defect-signal check stated as a number
rather than as an impression.

### Restoration proofs for `crates/muxsmith-core/src/report/json.rs`

The absence check ("the file is byte-identical at commit time") is fire-verified:
the mutated state below is a real observation from the M1 run, taken with the same
three commands that print nothing on the restored state.

**FIRED, mutated state (during M1):**

```
$ git status --porcelain
 M crates/muxsmith-core/src/report/json.rs
$ git diff --stat -- crates/muxsmith-core/src/report/json.rs
 crates/muxsmith-core/src/report/json.rs | 5 +----
 1 file changed, 1 insertion(+), 4 deletions(-)
$ sha256sum crates/muxsmith-core/src/report/json.rs
04f636297a44b982bc55eb0e5e4738d43da4421390825c4c45287912db24d3de  crates/muxsmith-core/src/report/json.rs
```

**Restored state after each of the four mutations** (`git checkout -- <path>`
each time), the hash printed after every restore and identical to the baseline:

```
after M1: 5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a
after M2: 5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a
after M3: 5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a
after M4: 5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a
```

**After M4's restore, the two commands the plan names:**

```
$ git status --porcelain
[no output]
$ git diff --stat -- crates/muxsmith-core/src/report/json.rs
[no output]
```

Two further mutation/restore cycles followed in Step 4 (the producer fires); the
final tree state after all six restores is at the end of this report, where
`git status --porcelain` shows the one modified test file and nothing else.

---

## Step 3: the producers written, and why these and no other

Landed, in `crates/muxsmith-core/tests/report_json.rs`:

- **P1** `batch_document_preserves_batch_diagnostics_collection_order` - written
  **because M4 stayed green**, which is the plan's stated selection rule for W1-c.
- **P2** `batch_document_preserves_per_file_diagnostics_collection_order` - written
  **because M3 stayed green**, the same rule for W1-d.

Not written:

- **P3** `batch_document_sorts_config_diagnostics_errors_first` - **M1 went red**,
  so W1-a is already guarded by
  `dry_run_cli.rs::dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`.
  Writing it would duplicate that guard, which the plan's must-not-decide list
  forbids (`reuse before writing`).
- **P4** `config_only_document_sorts_config_diagnostics_errors_first` - **M2 went
  red**, so W1-b is already guarded by
  `dry_run_cli.rs::dry_run_and_validate_json_agree_on_config_diagnostics_ordering`.
  Consequently `config_only_document` is **not** imported, exactly as the plan's
  import note conditions it.

Both guards were read before being named. The first one's doc comment names itself
"The `batch_document` half of the same D102 change"; the second forces the
no-mkvmerge PATH and therefore exercises `config_only_document`. Neither is edited.

The shared scaffolding (`KeyRenderer`, `mixed_severity()`, `codes()`) is
transcribed from the plan's fence, plus the fenced import list minus
`config_only_document`. The two assertions compare the `code` sequence against
`["raw-property", "raw-on-known-property", "invalid-regex"]`, the collection order
of the fixture, which the fixture's reversed-severity construction makes
discriminating against the sorted order.

Both assertion messages name D102's scope boundary and cite **spec section 5.2
"Diagnostics" by SECTION, never by line**, per the owner ruling
`comments-locate-by-symbol-never-by-line-number` and the plan's explicit extension
of the section-not-line form to every string this task writes. The producers' doc
comments locate the reused guard by symbol as well.

---

## Step 4: red-then-green evidence per landed producer

Command in every case: `cargo test -p muxsmith-core --test report_json`.

### P1, with M4 re-applied (`batch_diagnostics` sorted)

```
test batch_document_preserves_batch_diagnostics_collection_order ... FAILED

---- batch_document_preserves_batch_diagnostics_collection_order stdout ----

thread 'batch_document_preserves_batch_diagnostics_collection_order' (2623827) panicked at crates/muxsmith-core/tests/report_json.rs:159:5:
assertion `left == right` failed: D102's scope boundary: only `config_diagnostics` is ordered errors-first, while `batch_diagnostics` keeps collection order (spec section 5.2, Diagnostics)
  left: ["invalid-regex", "raw-on-known-property", "raw-property"]
 right: ["raw-property", "raw-on-known-property", "invalid-regex"]

failures:
    batch_document_preserves_batch_diagnostics_collection_order

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p muxsmith-core --test report_json`
```

Exit code of that run, captured directly (not through a pipeline): `EXIT=101`.
Note that P2 stayed green here, so the two producers are disjoint rather than
redundant.

Restored (`sha256sum` back to
`5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a`), re-run:

```
running 5 tests
test run_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty ... ok
test run_document_jobs_carry_index_output_state_and_summary_carries_all_four_counts ... ok
test batch_document_preserves_batch_diagnostics_collection_order ... ok
test batch_document_preserves_per_file_diagnostics_collection_order ... ok
test run_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`EXIT=0`.

### P2, with M3 re-applied (per-file `diagnostics` sorted)

```
test batch_document_preserves_per_file_diagnostics_collection_order ... FAILED

---- batch_document_preserves_per_file_diagnostics_collection_order stdout ----

thread 'batch_document_preserves_per_file_diagnostics_collection_order' (2624849) panicked at crates/muxsmith-core/tests/report_json.rs:189:5:
assertion `left == right` failed: D102's scope boundary: only `config_diagnostics` is ordered errors-first, while per-file `diagnostics` keep collection order (spec section 5.2, Diagnostics)
  left: ["invalid-regex", "raw-on-known-property", "raw-property"]
 right: ["raw-property", "raw-on-known-property", "invalid-regex"]

failures:
    batch_document_preserves_per_file_diagnostics_collection_order

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p muxsmith-core --test report_json`
```

`EXIT=101`. P1 stayed green here, the mirror image of the run above.

Restored, re-run:

```
running 5 tests
test batch_document_preserves_batch_diagnostics_collection_order ... ok
test run_document_jobs_carry_index_output_state_and_summary_carries_all_four_counts ... ok
test batch_document_preserves_per_file_diagnostics_collection_order ... ok
test run_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty ... ok
test run_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`EXIT=0`.

The two fires are a symmetric pair: each mutation reddens exactly its own producer
and leaves the other green, so neither producer is covering the other's half by
accident.

---

## Step 5: verification

The three named commands, foreground:

| command | exit |
|---|---|
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 0 |

Then **the full gate as `BUILDING.md` enumerates it** - read from the file rather
than from memory: 11 parts (6 Rust, 4 frontend, 1 house-knowledge), run in the
file's own order, all foreground, no subsets.

| # | block | command | exit |
|---|---|---|---|
| 1 | rust | `cargo fmt --all --check` | 0 |
| 2 | rust | `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| 3 | rust | `cargo test --workspace` | 0 |
| 4 | rust | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 |
| 5 | rust | `cargo deny check` | 0 |
| 6 | rust | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 |
| 7 | frontend | `pnpm lint` | 0 |
| 8 | frontend | `pnpm build` | 0 |
| 9 | frontend | `pnpm check:i18n` | 0 |
| 10 | frontend | `pnpm test:e2e` | 0 |
| 11 | house | `python3 scripts/ledger-lint.py` | 0 |

Part 11's output, which carries Task 1's new check:

```
ledger-lint: 533 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

**No `FAIL BUILDING.md: ...` line appeared**, so nothing wrote to that file outside
its owner's Files list.

Part 10's tail:

```
  68 passed (3.0s)
```

Working tree at the end of the gate run, before the commit:

```
$ git status --porcelain
 M crates/muxsmith-core/tests/report_json.rs
$ git diff --stat
 crates/muxsmith-core/tests/report_json.rs | 98 ++++++++++++++++++++++++++++++-
 1 file changed, 97 insertions(+), 1 deletion(-)
```

Exactly the Files list: one file, the target test file. `report/json.rs` is
byte-identical to `HEAD`, `dry_run_cli.rs` untouched, no production code changed.

**Ordering disclosure.** The `cargo fmt` experiment in divergence 1 was run AFTER
this gate run, and it wrote to the target file twice (the one-line form in, then
`cargo fmt --all`). `diff -u` against a pre-experiment copy returns nothing, so the
committed content is byte-identical to what the gate ran against. Confirmed
afterwards anyway, foreground:

```
$ cargo fmt --all --check                        -> EXIT=0
$ cargo test -p muxsmith-core --test report_json -> EXIT=0
$ python3 scripts/ledger-lint.py                 -> EXIT=0
ledger-lint: 533 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
$ sha256sum crates/muxsmith-core/src/report/json.rs
5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a  crates/muxsmith-core/src/report/json.rs
```

---

## Divergences and judgment calls, each named

1. **`mixed_severity()`'s middle entry is wrapped across four lines** rather than
   the plan's single fenced line. Token content is identical; only the line breaks
   differ. **Measured rather than assumed**, because the obvious explanation is
   wrong: the fenced line is **96 columns** with its indentation
   (`awk '{print length($0)}'`), so `max_width = 100` is not what rejects it -
   rustfmt's `fn_call_width` (default 60) is, and the argument list is 66
   characters. The check was fired: the fenced one-line form was written into the
   file, and

   ```
   $ cargo fmt --all --check
   EXIT=1
   Diff in /home/senol/Git/Muxsmith/crates/muxsmith-core/tests/report_json.rs:42:
    fn mixed_severity() -> Vec<Diagnostic> {
        vec![
            Diagnostic::info(DiagCode::RawProperty, "tracks[0].match.exact.raw:x"),
   -        Diagnostic::warning(DiagCode::RawOnKnownProperty, "tracks[1].match.exact.raw:language"),
   +        Diagnostic::warning(
   +            DiagCode::RawOnKnownProperty,
   +            "tracks[1].match.exact.raw:language",
   +        ),
            Diagnostic::error(DiagCode::InvalidRegex, "tracks[2].match.regex.title"),
        ]
    }
   ```

   `cargo fmt --all` then reproduced **exactly** the committed wrapped form (`diff
   -u` against a pre-experiment copy of the file returns nothing), so the written
   form is rustfmt's own output for the fenced content and gate part 1 could not
   have accepted the fence verbatim. Concern 2 puts this to the reviewer.
2. **The import addition merged into the existing `use` line.** The plan lists
   `muxsmith_core::report::json::{DiagnosticRenderer, batch_document, config_only_document}`
   as an addition, and the file already carried
   `use muxsmith_core::report::json::run_document;`. Written as one statement,
   `use muxsmith_core::report::json::{DiagnosticRenderer, batch_document, run_document};`
   - `config_only_document` dropped because P4 did not land (the plan conditions
   that import on P4), `run_document` folded in because a second `use` of the same
   module path is not this tree's pattern. Covered by the structural-conformance
   grant: additive, zero outward effect, no assertion touched.
3. **P2's assertion message is composed in P1's prescribed shape.** The plan
   fences P1's message requirement explicitly (D102's scope boundary + spec section
   5.2 by section) and then states the section-not-line form binds every string
   this task writes; P2's message follows the same template with "per-file
   `diagnostics`" in place of "`batch_diagnostics`".
4. **Both producers carry doc comments.** Additive and pattern-conforming (the
   file's existing tests carry them), and they are where the reused CLI guard is
   named by symbol so the next reader does not re-add P3.
5. **The target file's module doc comment was NOT updated.** See concern 1.
6. No production code changed; no pre-existing test's behaviour changed (measured:
   503 -> 505 passing, delta exactly the two new tests).

---

## Numbered concerns a reviewer can rule on yes/no

1. **Module doc comment left narrow.** `crates/muxsmith-core/tests/report_json.rs`
   opens with a doc comment describing the file as "the one direct-in-core
   assertion the brief calls for, plus the `run_document` unit tests relocated
   from `muxsmith-cli/src/commands/run.rs`". Two `batch_document` document-shape
   producers now sit below it. The statement is **incomplete, not false**, and the
   Files list names three regions (stub renderer, shared fixture, the selected
   producers) rather than the module doc, so I left it. Should it be widened by one
   sentence, or does it stay?
2. **The rustfmt-forced wrap of the fenced fixture** (divergence 1, with its fired
   `cargo fmt --all --check` evidence): acceptable as transcription of a fence that
   the gate's own formatter cannot accept verbatim, or a divergence that needed
   routing before it was written? If the latter, note that the fence and gate part
   1 are in direct conflict, which is a plan defect rather than an implementer
   choice.
3. **The mkvmerge-gated sorted-half guard is still gate-dependent, and this task
   measured the gap from the inside.** M1's red state exists only because this
   machine has mkvmerge; on a machine without it,
   `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran` returns
   early and W1-a would have measured **unguarded**, which would have selected P3
   for the wrong reason. The ROADMAP already records this as a neighbouring
   coverage fact with the vehicle "whichever package next touches the diagnostics
   ordering contract, or the owner QA pass". Plan 10's Task 2 is a package touching
   that contract, but it forbids duplicating the two CLI guards and enumerates the
   four candidate producers exhaustively, so I built nothing for it. Is that the
   correct stop, and does the vehicle stay open?

---

## What I surface for the controller

- **The plan's conditional acceptance rows all resolved, and the split is the one
  the ROADMAP predicted:** the two SORTED halves (W1-a, W1-b) were already
  guarded, both by existing `dry_run_cli.rs` tests; the two PRESERVED-ORDER halves
  (W1-c, W1-d) were both unguarded and both got producers. The Plan-9 Task-5
  review's original finding covered `batch_diagnostics` only; **the per-file
  `diagnostics` half (W1-d) was a second, independently unguarded half that the
  four-mutation measurement found and a one-mutation eyeball would not have.** That
  is a datum for the doctrine handle that an observable's HALVES each need a named
  producer, and for `tests-ship-with-the-feature-never-after`'s companion.
- **Neither red mutation exposed an ordering defect.** The production code agrees
  with spec section 5.2 and D102 on all four halves; the mutations measured
  coverage only.
- **`have_mkvmerge()` gating of the sorted-half guard** (concern 3) is still open
  and its ROADMAP vehicle sentence arguably points at this package. Controller
  call.
- Task 1's `BUILDING.md` gate-count check ran green in this task's gate run and
  emitted the widened summary line; four more independent executions of it are what
  the sequencing section budgeted, and this was the first.

---

## Step 6: the commit

```bash
git add crates/muxsmith-core/tests/report_json.rs
git -c commit.gpgsign=false commit -m "report: guard D102's preserved-order half in batch_document, selected by mutation measurement" ...
```

Staged explicitly by name, unsigned, exactly one trailer
(`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`), no `Claude-Session`
line, no context-window suffix. No push (that is a controller close action).

**Commit:** `35bc363` (`35bc3630061e9d318a8ae0de440a07856c319a20`)

```
$ git show --stat HEAD
commit 35bc3630061e9d318a8ae0de440a07856c319a20
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Wed Jul 29 14:50:06 2026 +0200

    report: guard D102's preserved-order half in batch_document, selected by mutation measurement

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-core/tests/report_json.rs | 98 ++++++++++++++++++++++++++++++-
 1 file changed, 97 insertions(+), 1 deletion(-)
```

Signature status and trailer set, checked rather than assumed:

```
$ git log -1 --format="%G? | %(trailers)" HEAD
N | Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

`N` = unsigned, as policy; exactly one trailer. Working tree after the commit:
`git status --porcelain` prints nothing.
