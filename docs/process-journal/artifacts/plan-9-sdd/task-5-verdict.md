# Task 5 verdict - Plan 9 (D102, D103; spec S-7)

**Verdict: APPROVED_WITH_MINORS.**

All five character-for-character fences transcribe byte-clean on my own
comparison instrument. The hoist is semantically identical - the moved body is
byte-for-byte the CLI's old one, all nine pre-existing call sites resolve
through the re-export, the CLI human-output insta snapshots pass untouched, and
the sort reaches exactly `config_diagnostics` in the two builders and nothing
else. Both of the task's colour claims reproduce on my own harness, in both
directions. The centre of this review - the unenumerated third Rust test - is
**correct**: I re-measured the gap with my own mutation and it is total, and
the new test is its sole detector. The full Step-7 bar is green on my own runs.

Four findings: one MEDIUM (a composed doc claim this diff introduced that is
false against the tree and against D102's own consumers sweep) and three LOW
(two report-count defects, one comment/code mismatch). None is behavioral, none
is user-visible, none fails a gate.

**One aggregate in the report is wrong and one is right.** "34 test binaries" is
unreproducible (measured: 39 `test result:` lines = 35 test binaries + 4
doc-test targets, the unit Task 4's verdict settled). "64 e2e passing against a
pre-existing 63" is exact.

---

## Findings

### MEDIUM-1: the re-export's composed doc comment states a false fact about the call-site set

**Site:** `crates/muxsmith-cli/src/commands/mod.rs:16-18`

```rust
/// The one error-first ordering definition, hoisted to core (D102) and
/// re-exported here so every `crate::commands::severity_sorted` call site
/// -- the human printing paths this crate owns -- is unchanged.
pub(crate) use muxsmith_core::report::severity_sorted;
```

The appositive is false. One of the nine call sites,
`crates/muxsmith-cli/src/commands/validate.rs:21`, is not a human printing path:
the vector it sorts feeds **both** output modes -

```rust
// validate.rs:19-29
    // Error-first, stable within a severity; both output modes share it.
    let diagnostics: Vec<Diagnostic> =
        severity_sorted(&validate::config_diagnostics_from_file(profile_path))
            .into_iter()
            .cloned()
            .collect();
    let exit = severity_exit(worst_severity(&diagnostics));

    if json {
        let entries = rendered_diags(&diagnostics, renderer);
        println!("{}", serde_json::json!({ "diagnostics": entries }));
```

`validate.rs:19`'s own pre-existing comment says so ("both output modes share
it"), and D102's consumers sweep describes that exact call site as the JSON
side: "CLI `validate` is unaffected (it sorts its own flat `{diagnostics:
[...]}` envelope already, now through the re-exported core `severity_sorted`)"
(design `:1081-1084`). So the new doc contradicts the governing design entry on
the one point the entry took care to state.

Why it is not cosmetic: the sentence tells the next maintainer that
`validate --json`'s ordering does **not** come through this symbol. That is
precisely the belief D102's sweep exists to prevent, and precisely the belief
under which someone re-introduces a second sort on validate's envelope.

**Evidence I ran:** call-site enumeration
`grep -rnP 'severity_sorted' --include='*.rs' crates/ src-tauri/` with a fired
control (`worst_severity`, hits); then `validate.rs:1-60` read at the tree.

**Exact required change:** delete the appositive, or replace it with one that is
true. Minimal correct form:

```rust
/// The one error-first ordering definition, hoisted to core (D102) and
/// re-exported here so every `crate::commands::severity_sorted` call site
/// -- this crate's human printing paths and `validate`'s own `--json`
/// envelope -- is unchanged.
```

Doc-only; no code, no test, no fence touched. ASCII `--` retained per the
typography rule.

### LOW-1: the report's "34 test binaries" is unreproducible

**Site:** `.superpowers/sdd/plan-9/task-5-report.md:150`

Measured by me on the committed tree, `cargo test --workspace`, exit 0:

```
grep -c '^\s*Running '   -> 35
grep -c '^\s*Doc-tests ' -> 4
grep -c '^test result:'  -> 39      (all "ok", 0 FAILED)
```

34 matches neither reading. The house unit is settled: Task 4's verdict
(`task-4-verdict.md:402`) records "39 `test result:` lines (**35 test binaries +
4 doc-test targets**)", and Task 3's verdict and Task 4's report both state 39.
The task's own delta is real and correctly stated elsewhere in the report
(`dry_run_cli.rs` 11 -> 13; I confirmed `#[test]` count 11 at `44a2010`, 13 now,
and the pasted result line `13 passed`).

**Exact required change:** in the report, replace "34 test binaries" with the
measured "39 `test result:` lines (35 test binaries + 4 doc-test targets), every
one ok, 0 failed". No code change.

### LOW-2: the report undercounts the gate idiom's neighbours by one

**Site:** `.superpowers/sdd/plan-9/task-5-report.md:284` (divergence 7, "like
five of its neighbours in that file")

Measured at the parent commit:

```
git show 44a2010:crates/muxsmith-cli/tests/dry_run_cli.rs | grep -nP 'if !have_mkvmerge\(\)'
19:  82:  125:  212:  334:  676:      -> SIX pre-existing guards
grep -cP 'if !have_mkvmerge\(\)' crates/muxsmith-cli/tests/dry_run_cli.rs -> 7 now
```

The substance of the divergence is unaffected (the gate is the file's
pre-existing idiom, and CI installs mkvtoolnix on every leg - verified,
`.github/workflows/ci.yml:57-72` plus the explicit no-silent-skip guard at
`:108-116`, so the new producer genuinely runs there). Only the number is wrong.

**Exact required change:** "like six of its neighbours". No code change.

### LOW-3: the third test's comment describes two properties where the code asserts one

**Site:** `crates/muxsmith-cli/tests/dry_run_cli.rs:443-448`

```rust
    // Planning ran, so this document came from `batch_document`, not the
    // config-only shape: `files` is present and `mkvmerge_found` absent.
    assert!(
        report.get("mkvmerge_found").is_none(),
        "expected a planned batch document, got: {report}"
    );
```

`mkvmerge_found`-absent alone does not identify `batch_document`: CLI
`dry_run.rs:40` builds `config_only_document(&[diagnostic], None, renderer)` on
the profile-load-failure path, which also omits the key. The test is still sound
- that path would produce a singleton `parse-error` and blow up the code-sequence
assertion below - and I fire-verified the test red under the exact
`batch_document` mutation, so it is not vacuous. The defect is that the comment
claims a `files` assertion the code does not make.

**Exact required change:** either add the missing half -
`assert!(report["files"].is_array(), "...");` - or reword the comment to
describe only the assertion that exists. Prefer the first: it closes the
load-failure path at the same assertion instead of relying on the next one.

---

## The six adjudications

### 1. The unenumerated third Rust test: CORRECT, built rather than routed. All four conditions hold individually.

I did not weigh the implementer's measurement; I ran my own, and it is stronger
than the one reported because it isolates each call site.

**Mutation B** (`batch_document`'s sort removed,
`config_only_document` untouched; my own script, applied to a byte-baselined
tree), `cargo test --workspace`, exit 101:

```
test dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran ... FAILED
test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
  left: ["raw-property", "raw-on-known-property", "unknown-property", "invalid-regex"]
 right: ["unknown-property", "invalid-regex", "raw-on-known-property", "raw-property"]
```

Exactly ONE failing test in the entire workspace, and it is the unenumerated
one. Both halves of the question are answered by that single run: the gap
existed (every other test, including the plan's two enumerated ones, stayed
green with `batch_document` unsorted), and the new test closes it (it is the
sole detector).

**Mutation C** (the mirror: `config_only_document`'s sort removed,
`batch_document` untouched), `cargo test --workspace`, exit 101:

```
test dry_run_and_validate_json_agree_on_config_diagnostics_ordering ... FAILED
test result: FAILED. 12 passed; 1 failed; ...
```

Also exactly one failure, and it is the plan's enumerated parity test. The pair
is decisive: the two call sites have exactly one guard each, the guards are
disjoint, and neither test is redundant. The plan enumerated a producer for one
half of a two-site change. No mkvmerge-gated test self-skipped on this machine
(`grep -c 'MKVMERGE_SKIP\|skipping'` over the run -> 0; `mkvmerge v100.0`
installed), so the measurement is real rather than a green-by-absence.

Restore proven both times: `git checkout -- crates/muxsmith-core/src/report/json.rs`,
then `sha256sum -c` against a baseline taken before any mutation -> OK for all
six files, `git status --porcelain` empty, HEAD still `e134fdc`.

**The four conditions, ruled individually:**

| # | Condition (ledger text) | My verdict | Evidence |
|---|---|---|---|
| 1 | ADDITIVE - no existing assertion, fixture value or helper touched | **HOLDS** | `git show --numstat`: `dry_run_cli.rs` is `145 0` - 145 insertions, **zero** deletions. `#[test]` count 11 -> 13 |
| 2 | Runs on EXISTING infrastructure | **HOLDS** | `have_mkvmerge()` (`:9` at the parent), `MKVMERGE_SKIP_MARKER`, `empty_path_dir()` (`:256`), the `support::muxsmith` funnel and `tempfile` all pre-date the diff. No new file, harness, mock or helper |
| 3 | The consequence is one THIS package's own diff creates | **HOLDS** | `batch_document`'s sorted `config_diagnostics` is introduced by Step 1; before the diff that builder emitted collection order |
| 4 | Named in the report for the reviewer | **HOLDS** | Report concern 1, plus the status set to DONE_WITH_CONCERNS for that reason |

**On the precedence question.** The plan's Global Constraints (`:25`) do forbid
"new test scenarios beyond the ruled D23 tests (D104), the two amendment-1
scenarios (D101/D103), and the tests the design's D-entries pin", and this test
is outside that enumeration. That collision is exactly what the owner ruled on
2026-07-28, in the entry the brief names: *"at EXECUTION time this rule wins over
a plan's test enumeration, narrowly - the implementer BUILDS the missing producer
instead of routing it when all four hold ... Outside those four the enumeration
still binds and the fork returns"*
(`docs/process-conventions.yaml:672`). All four hold, so BUILD is the ruled
action, not a discretionary one, and routing it as NEEDS_CONTEXT would have been
the deviation. The ruling's own stated asymmetry applies verbatim here: an
unnecessary added test is visible and cheap to reject; the missing one was
invisible by construction, which mutation B demonstrates.

**Ruling: building it was correct. It should NOT have returned as NEEDS_CONTEXT.**

### 2. The re-export's position and its doc comment: position CORRECT, doc comment PERMITTED but its content must be fixed (MEDIUM-1).

**Position.** The plan says the re-export goes "in its place"; the design says
"in `commands/mod.rs`" and requires "zero wrapper code". Neither fences a
position. A `use` statement's location within a file has zero outward effect on
all four of the grant's conditions (no API/symbol surface -
`crate::commands::severity_sorted` resolves identically either way, proven by
nine unchanged call sites compiling; no data format; no verification weakened;
nothing user-visible), and the file's unbroken structural pattern is imports at
the top. That is squarely
`latitude-carveout-zero-content-structural-forks`'s "extend an unbroken local
pattern with zero outward effect, in-scope without routing". **Correct; not a
deviation.**

**Doc comment.** "Zero wrapper code" constrains code; a doc comment is not code,
and it is not on the brief's enumerated transcribe-don't-compose list (D102's
doc comment, D103's `find` line, the Step-3 YAML, the Step-5 document fields and
assertions). Every other item in `commands/mod.rs` carries a doc comment
(`severity_exit:21`, `diag_exit_code:32`), including the function this one
replaces. Under "match the house pattern" the comment conforms, and I would not
remove it.

**But its content is wrong**, which is MEDIUM-1. Keep the comment, fix the
appositive. The failure is not that prose was composed; it is that a composed
claim was not checked against the call-site set it describes - the one thing a
doc comment on a re-export exists to state.

### 3. The composed builder doc paragraphs in `json.rs`: IN SCOPE, keep, content verified accurate.

**Ruling: in scope.** Not, however, for the reason the report gives. D102's
sentence is *"Recorded **here** so the non-uniformity is a decision, not an
accident"* (design `:1072-1073`) - "here" is the design, and the recording
already happened there. D102 creates no obligation on the code, so the report's
"D102 explicitly asks to be recorded ... the code is where the next reader meets
it" over-reads its licence.

The correct ground is house pattern plus zero outward effect. `json.rs`'s
unbroken local pattern is that a builder's non-obvious semantics live in its own
rustdoc - `config_only_document`'s pre-existing `mkvmerge_found` paragraph
(`:79-87`) is the exact precedent, same file, same function, same shape. All four
zero-outward-effect conditions hold. Doc-only additions of that shape are the
grant's interior, not its edge.

**Accuracy, which I checked rather than assumed** (this is composed prose, so it
gets the MEDIUM-1 treatment):

- `batch_document:32-39` - matches D102's site, scope boundary and rationale
  clause for clause. Accurate.
- `config_only_document:89-90` - accurate.
- `rendered_diag:192-197` - "The single per-diagnostic mapping in this crate"
  verified: `grep -rnP '"rendered"' crates/muxsmith-core/src/` shows exactly one
  assignment site, `json.rs:203`. Accurate as scoped ("in this crate"); the
  cross-crate `validate` consumer is covered by the neighbouring `rendered_diags`
  doc it points at.
- `rendered_diags:206-210` - the rewrite correctly adds validate's
  already-sorted use. Accurate.

`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` clean on my run,
including the deliberately un-backticked `Info < Warning < Error` in the
transcribed fence.

### 4. The parity test's assertion set: SUFFICIENT. The missing exit-code pin leaves no real failure mode silent.

The concern this question encodes is amendment 4's measured class: a subprocess
test that passes on clap's own exit 2 while snapshotting empty stdout. That mode
is closed here, structurally rather than by luck - the JSON parse runs before any
assertion and carries the exit code into its panic:

```rust
    let parse = |out: &std::process::Output, what: &str| -> serde_json::Value {
        serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
            panic!("{what} json ({e}), exit {:?}, stderr: {}", out.status.code(), ...)
```

Fired independently: `./target/debug/muxsmith validate --json --bogus-flag` ->
**exit 2, stdout 0 bytes**, stderr `error: unexpected argument '--bogus-flag'
found`. `serde_json::from_slice(b"")` cannot succeed, so any usage-error
invocation panics loudly with the exit code attached. Both invocations pass
through that closure.

The vacuity guard the report claims is also real: two empty arrays satisfy
assertion 1, but assertion 2's `&dry_run_codes[..2]` panics on a slice shorter
than 2, so the test cannot pass empty. And the run under mutation C shows the
whole thing red on the intended defect.

The one thing the pair does not pin is the *value* of the exit code on the happy
path (both commands exit 2 here, by design, because the fixture carries
error-severity diagnostics). Pinning it would assert spec 8.1's exit mapping,
which is a different contract with its own producers (`cli_validate.rs`'s
`invalid_profile_exits_two...` snapshot, `dry_run_cli`'s existing `.code()`
assertions). Adding it here would be duplication, not coverage.

**Ruling: sufficient. No change required.**

### 5. The e2e fixture omitting `mkvmerge_found`: FAITHFUL MIRROR, and more faithful than its neighbours.

Verified at the source, not from the report. `load_profile_body`
(`src-tauri/src/lib.rs:286-300`) calls `config_only_document(..., None, ...)` on
**both** arms:

```rust
        Ok(profile) => { ... config_only_document(&diags, None, &ShellRenderer); ... }
        Err(d)      => { ... config_only_document(&[d],   None, &ShellRenderer); ... }
```

`config_only_document` only inserts the key on `Some` (`json.rs:110-112`), so
**no `load_profile` envelope ever carries `mkvmerge_found`**, on either arm. The
new fixture is exactly what core emits. `src/ipc.ts:88` types it optional, so the
TS side is satisfied. There is even a standing core-side pin of the same
property: `src-tauri/src/lib.rs:748`,
`dry_run_body_load_failure_mirrors_config_only_document_with_no_mkvmerge_key`.

The sibling comparison inverts the question. `loadedForApply`
(`e2e/smoke.spec.ts:428-435`), the very fixture this scenario's scaffold is
replayed from, sets `mkvmerge_found: true` on a `load_profile` document - which
core never emits on either arm. **The neighbour is the unfaithful one**; it is
pre-existing and not this task's to fix. Divergence 6 was surfaced with a
code-comment rationale at the site (`smoke.spec.ts:535-537`), which is the right
handling.

**Ruling: correct as written. No change required.** The neighbour's drift goes to
HARVEST.

### 6. BatchView's else-branch text: LEAVE AS IS in this task; ROUTE the correction. The implementer's route was right.

The falsification is real, and I confirm it independently. After the code-keyed
`find`, the else-branch at `src/views/BatchView.vue:229-240` fires on two
distinct triggers - an empty `config_diagnostics`, and a non-empty one that
contains no `parse-error`. Both texts describe only the first:

- comment `:232-233`: "An empty `config_diagnostics` here means core broke that contract"
- `console.error` `:237`: "load_profile returned profile: null with no diagnostics"

D103 anticipated the widening explicitly ("strictly more detection than today, no
lost case", design `:1120-1123`) and ordered no text change; the plan's Step 4
positively fences the string ("the existing else-branch `console.error` stays").

Two rules could reach the comment and they resolve cleanly:

- The structural grant's "repairing a reference which the task's OWN enumerated
  edit invalidated (a doc link, a **comment referent**, an import)" does **not**
  reach it. Nothing here is a dangling *referent*; the comment makes a factual
  claim about the branch's trigger set that became incomplete. That is a
  truthfulness correction, a different class.
- The grant also states "an explicit enumeration in brief, design or spec always
  wins over it", and the plan enumerates that the `console.error` stays. The
  string is fenced outright.

Amendment 3 is the governing precedent and it is exactly on point: Task 2 moved
`run_batch`'s rustdoc verbatim as ordered while the same commit falsified three
of its passages, and the owner ruled the correction **enters the design** and
rides a later task, rejecting both an ordinary keyboard fix and a 1.x deferral.
Same shape, one task later.

**Ruling: leave as is - the implementer was right not to touch either. Route the
correction.** It must not evaporate: no test asserts the string (verified,
`grep -rnP 'profile: null with no diagnostics|broke that contract' e2e/ src/`
hits only the source line itself, with a fired `console\.error` control), so
nothing will catch it later. Recommended route: a controller-routed design note
under D103 (amendment-3's form) with the code edit riding the plan close or Task
6, whichever the owner prefers. Suggested replacement content, for whoever
authors it: the comment should name both triggers (empty vector, or a vector
whose diagnostics are all non-`parse-error`), and the `console.error` string
should read as "no parse-error diagnostic" rather than "no diagnostics".

---

## Verification quality: the Step-7 bar, re-run

Every part run by me, foreground, no subsets, on the committed tree at
`e134fdc`, working tree clean.

| # | Command | My result |
|---|---|---|
| 1 | `cargo fmt --all --check` | exit 0, no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `Finished dev profile`, no warnings |
| 3 | `cargo test --workspace` | exit 0; **39 `test result:` lines, all ok, 0 FAILED**; 35 test binaries + 4 doc-test targets; 0 skipped gated tests; 0 pending `*.snap.new` |
| 4 | `pnpm lint` | `eslint .`, no findings |
| 5 | `pnpm build` | `built in 150ms` |
| 6 | `pnpm test:e2e` | **64 passed** |

Extra, because this task writes rustdoc and deletes a `use` line:

- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean.
- Same with `--document-private-items` - **`muxsmith-gui` fails**, two ambiguous
  links at `src-tauri/src/lib.rs:54:21` and `:87:15` (`` `run` is both a function
  and a module``). **Pre-existing, not this task's**: the commit touches no
  `src-tauri` file (`git show --name-only` lists seven files, none under
  `src-tauri/`). Observation 1 in the report is reproduced and confirmed. It goes
  to HARVEST because the plan close is scheduled to add that exact flag to the
  gate.

**Aggregate recomputation:**

| Report claim | My measurement | Verdict |
|---|---|---|
| 7 files, 286 insertions / 24 deletions | `--numstat` sums to 286 / 24 over 7 files | exact |
| 34 test binaries | 39 result lines = 35 binaries + 4 doc-tests | **wrong (LOW-1)** |
| `dry_run_cli.rs` 13 passed, was 11 | `#[test]` 11 at `44a2010`, 13 now; result line `13 passed` | exact |
| 64 e2e passing, pre-existing 63 | 64 passed; diff adds exactly one `test(` to `smoke.spec.ts` (31 -> 32) | exact |
| `ls e2e/*.spec.ts \| wc -l` -> 9 | 9 | exact |
| "like five of its neighbours" (gate idiom) | six at the parent | **wrong (LOW-2)** |

---

## Contract compliance: the five fences, byte-compared on my own instrument

Built as an independent extractor (`fences.py`, appendix) that pulls each fence
from the design/plan by line range and the implemented text from the tree, and
diffs them. I did not read the report's comparison.

| Fence | Source | Target | Result |
|---|---|---|---|
| D102 doc comment + signature | design `:1040-1044` | `report/mod.rs:308-312` | **IDENTICAL** |
| D103 `find` line | design `:1106` | `BatchView.vue:225` | **IDENTICAL** |
| Step-3 profile YAML | plan `:338-344` | parity test, `dry_run_cli.rs:335-341` | **IDENTICAL** |
| same YAML, third test | plan `:338-344` | `dry_run_cli.rs:414-420` | **IDENTICAL** |
| S-7 sentence | design `:1412` | spec `:255`, appended | **IDENTICAL**, joined by exactly one space after "humans read text." |

The Step-5 scenario's design-fixed contents, checked item by item against D103's
amendment-1 producer paragraph (`:1139-1155`), all present and exact:
`profile: null`; a singleton diagnostic with `code: "parse-error"`,
`severity: "error"`, `config_path: ""`,
`params: { detail: "unknown field", at: "" }`, `rendered: "parse-error"`; empty
`files`/`batch_diagnostics`/`suggestions`; assertion (a) the `role="alert"`
element contains "The profile could not be parsed"; assertion (b) the recorded
invoke log contains zero `apply_suggestion` and zero `save_profile`. The scaffold
is the `:460` apply-flow test's, with exactly one substitution
(`load_profile`), and `apply_suggestion`/`save_profile` stay mocked so absence is
an invocation fact - matching D103's "one substitution" and making assertion (b)
meaningful.

---

## The hoist's semantics

- **Body identity.** The moved implementation is byte-for-byte the deleted one
  (`let mut sorted ... sort_by_key(|d| Reverse(d.severity)); sorted`); only the
  visibility keyword and the doc changed.
- **Call sites, complete set** (`grep -rnP 'severity_sorted' --include='*.rs'
  crates/ src-tauri/`, with a fired `worst_severity` control): nine, all
  pre-existing, none touched - `commands/mod.rs:111,115` (human batch printing),
  `dry_run.rs:53,71,85`, `run.rs:95,117,135`, `validate.rs:21`. `pub(crate) use`
  keeps `crate::commands::severity_sorted` resolvable identically; the workspace
  compiles with `-D warnings` and the CLI human-output insta snapshots
  (`cli_validate__*`, `dry_run_cli__*`, `run_cli__*`; 0 pending `.snap.new`) pass
  unchanged, which is the behavioral proof.
- **Sort scope.** Exactly two application sites, `json.rs:61`
  (`batch_document`) and `:100` (`config_only_document`), both on
  `config_diagnostics` only. `files[].diagnostics` (`:57`) and
  `batch_diagnostics` (`:68`) still route through `rendered_diags` in collection
  order; `run_document` extends an already-built base and adds no sort. D102's
  scope boundary is honoured exactly.
- **No second rendering implementation.** `rendered_diag` is the only
  `v["rendered"] = ...` site in `muxsmith-core` (verified with a cross-crate
  control), and `rendered_diags` delegates to it.

---

## Latitude, both forms

**Resolved at the keyboard that should have returned:** none found. I walked all
nine named divergences and re-derived each licence:

| # | Divergence | Ruling |
|---|---|---|
| 1 | Third Rust test | Ruled action under the live four-condition precedence (adjudication 1) |
| 2 | Re-export position | Structural grant, zero outward effect, house pattern (adjudication 2) |
| 3 | Re-export doc comment | Permitted, but its content is MEDIUM-1 |
| 4 | Builder doc paragraphs | In scope on house pattern; content verified accurate (adjudication 3) |
| 5 | Parity test assertion set | Sufficient (adjudication 4) |
| 6 | `mkvmerge_found` omitted | Faithful to core; more so than its neighbour (adjudication 5) |
| 7 | `have_mkvmerge()` gate | File's pre-existing idiom, verified; CI installs mkvtoolnix on every leg with a no-silent-skip guard. Count off by one (LOW-2) |
| 8 | Both commands stay mocked | Required by D103's assertion (b) to be an invocation fact |
| 9 | Raw-string YAML fixture | Correct, and necessary: `dry_run_cli.rs`'s own escaped one-line form cannot carry a character-for-character transcription. `r#"..."#` is the same crate's house pattern (`cli_validate.rs:39,66,95`); the only two such blocks in `dry_run_cli.rs` are this task's two additions |

**The inverse form - returned or surfaced when it should have built:** none
found. Two "no work needed" conclusions were checked by running their premises
(below), and both routes are correct.

**House dimension, the four named entries:**

- `latitude-carveout-zero-content-structural-forks` **as amended today**: the new
  import case fired exactly as designed. `use std::cmp::Reverse;` was ADDED in
  `report/mod.rs` (a LISTED file) because the task's own enumerated addition does
  not compile without it - binary at the compiler, not a keyboard judgement - and
  DELETED in `commands/mod.rs` (also LISTED) because the enumerated deletion made
  it unused and `-D warnings` rejects it. Both correct; neither needed routing.
  The file-vs-within-file boundary is respected: seven files touched, all seven on
  the Files list, nothing outside it.
- `tests-ship-with-the-feature-never-after` and its execution-time four
  conditions: fired once, correctly, all four hold (adjudication 1). The
  "where it did NOT fire" section is present and both entries are correctly
  routed, with one reasoning correction below.
- `an-import-removal-sweeps-the-doc-links-that-named-the-symbol`: swept
  independently, `grep -rnP '\[`[^`]*(Reverse|severity_sorted)[^`]*`\]'` over
  `crates/ src-tauri/ xtask/` -> the single hit is the link this task ADDED
  (`json.rs:33`), and it resolves under `-D warnings`. Fired control on
  `rendered_diags` returns a real hit, so the empty result is evidence.
- `proc-normative-count-recomputed`, callers'-docs facet: swept. `json.rs`'s
  module doc "the three functions here" (`:5`) counts document *builders*
  (`batch_document`, `config_only_document`, `run_document`); the added
  `rendered_diag` is a private per-diagnostic mapping helper, so the count is
  untouched - the implementer's read is correct and I confirm it. No plan or
  design line states a count of Task 5's tests: the nearest candidate,
  Global Constraints `:25` "the two amendment-1 scenarios (D101/D103)", counts
  e2e scenarios one per task and this task added exactly one. The report's
  claim (e) holds.

---

## The no-work-needed check: each premise RUN, not weighed

**1. "The GUI display-order consequence is vacuous with respect to this diff."**
Premise verified - `e2e/mocks.ts` reassigns `window.__TAURI_INTERNALS__.invoke`
with hand-written JS objects, so no e2e fixture passes through core, and an
e2e assertion on list order would only assert that the DOM mirrors a
hand-written array, true before this diff as well. The conclusion (no additional
producer owed) stands.

**Correction to the route, not the outcome:** the report says condition 3 FAILS.
It does not. D102 itself calls the GUI display order a "user-visible GUI
consequence" and the diff creates it, so condition 3 HOLDS. What actually
disposes of it is that the producers which *can* observe the consequence are the
core unit test and the two subprocess tests, all three of which the package
ships - a producer exists, so the rule has nothing to fire on. Same answer,
sounder reason; worth correcting because the stated reason would license
skipping a genuinely uncovered core-produced consequence next time.

**2. "D102's scope boundary is preserved behaviour, not a consequence this diff
creates."** Premise correct - per-file `diagnostics` and `batch_diagnostics` are
byte-identical in behaviour before and after, so condition 3 genuinely fails and
surfacing rather than building is right. **But I measured the gap the report
claims, and it is total** (mutation D, my own): widening `batch_document` to sort
`&f.diagnostics` and `&batch.batch_diagnostics` as well left
`cargo test --workspace` at **exit 0, zero failures**. A contract now stated in
three places - D102, spec S-7 (which this task added), and both builder doc
paragraphs - has zero producers. Correctly out of Task 5's scope; goes to
HARVEST as a real, measured gap.

**3. "The parity test's assertion pair is self-guarding against vacuity."**
Verified, and its stronger guard (the JSON-parse panic) fired independently -
adjudication 4.

**4. "The omitted `mkvmerge_found` key is the faithful mirror of core."**
Verified at both `load_profile_body` arms - adjudication 5. Stronger than
claimed.

**5. "No plan or design line states a count of this task's tests."** Swept
independently with a fired control; confirmed - see the
`proc-normative-count-recomputed` row above.

---

## Evidence appendix: my instruments

All under
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t5rev-independent/`.
None is a path the report names, none is a shared default, and none re-runs an
instrument the implementer wrote.

| File | What it is |
|---|---|
| `fences.py` | independent character-for-character fence extractor/differ (five fences) |
| `mutate_batch_sort.py` | mutation B - removes `batch_document`'s sort only |
| `baseline.sha256` | pre-mutation byte baseline of all six touched source files |
| `baseline-head.txt`, `baseline-status.txt` | HEAD + clean-tree proof taken before any mutation |
| `baseline-cargo-test.txt` | full `cargo test --workspace` on the committed tree |
| `mutated-cargo-test.txt` | full run under mutation B |
| `mutC-cargo-test.txt` | full run under mutation C (`config_only_document` sort removed) |
| `mutD-cargo-test.txt` | full run under mutation D (scope boundary widened) |
| `baseline-e2e.txt` | full `pnpm test:e2e` on the committed tree |
| `usage-stdout.txt`, `usage-stderr.txt` | the CLI usage-error fire for adjudication 4 |
| `touched.txt` | the commit's file list, for the Files-list check |

Mutations C and D were applied inline via heredoc'd Python against the same
baseline. Mutations A and B on `BatchView.vue` (pre-edit positional fetch;
defective `"parse_error"` predicate) likewise, each followed by `pnpm build`
because the e2e suite drives `dist/`, not `src/` - a step the report's quoted
invocations do not show, and without which either mutation would have measured
nothing.

**Restore proof, after every mutation:** `git checkout -- <path>` (never a bare
`cp`), then `sha256sum -c baseline.sha256` -> OK for all six files, plus
`git status --porcelain` empty and `git rev-parse HEAD` still `e134fdc`. Final
state re-confirmed green: 39 ok test-result lines, 0 FAILED, 64 e2e passed, tree
clean.

**Colour claims, both directions, my own harness:**

| Direction | Mutation | Result |
|---|---|---|
| Green on the pre-edit positional fetch (the plan's no-red-today claim) | `config_diagnostics[0]` restored + rebuild | **1 passed (607ms)** - claim confirmed |
| Red under a defective code-keyed rewrite (its stated discriminating power) | `d.code === "parse_error"` + rebuild | **1 failed**, on assertion (a) at `smoke.spec.ts:587` |
| Restored end state | `git checkout --` + rebuild | **1 passed (610ms)** |

**Commit hygiene:** message byte-identical to the plan's Step-8 text; exactly one
trailer (`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`); no
`Claude-Session` line; unsigned (`%G?` -> `N`); seven files, all on the Files
list; zero house-knowledge YAML touched.

---

## HARVEST

### What Task 6 must carry

1. **Its `smoke.spec.ts` anchors are UNSHIFTED.** Task 5's addition landed
   entirely at `:530-591`, below every span amendment 2 cites. Re-measured now:
   `function name(` at `smoke.spec.ts:60` (doc comment `:53-59`),
   `editor-markers.spec.ts:29`, `editor-rule-add-remove.spec.ts:41` - exactly the
   amendment-2 numbers. `grep -c FluentVariable` is 2 in each of the three files,
   so the unused-import cleanup premise still holds. Task 6 should still
   re-derive by content (`proc-57`), but no correction is owed.
2. **`ls e2e/*.spec.ts | wc -l` -> 9.** Task 6's `jobsview-reset.spec.ts` is
   still the TENTH spec file; amendment 2's statement stands unchanged.
3. **Baselines to measure its own delta against:** `pnpm test:e2e` **64 passed**
   (Task 6's four new tests should take it to 68; the `name()` hoist is a pure
   move and must change no count). `cargo test --workspace` **39 `test result:`
   lines (35 binaries + 4 doc-test targets), 0 failed** - Task 6 touches no Rust,
   so this must be unchanged. Do not carry the report's "34".
4. **The `--document-private-items` rustdoc failure blocks a close action, not
   Task 6 itself.** `muxsmith-gui` fails with two ambiguous `[`run`]` links
   (`src-tauri/src/lib.rs:54:21`, `:87:15`), fixable as `mod@run` / `run()`. It is
   pre-existing and outside the current ten-part gate - but
   `ledger-lint-runs-before-every-push` records that the gate gains *both*
   ledger-lint and the rustdoc private-items flag at the Plan 9 close. If that
   close action lands before the two links are fixed, the first gate run under
   the new leg goes red. **Fix before, not after.**

### What Task 7 must carry

5. Task 7's green control (`cargo test -p muxsmith-core --test suggestions`) is
   unaffected by Task 5 and green at `e134fdc`. Its byte-clean end-state duty is
   verifiable against the same baseline discipline used here; the tree is clean
   and at `e134fdc` as I leave it.

### Ledger-worthy

6. **D102's scope boundary has zero producers - measured, not argued.** Sorting
   `files[].diagnostics` and `batch_diagnostics` in `batch_document` leaves
   `cargo test --workspace` at exit 0 with zero failures (my mutation D). The
   boundary is now asserted in three normative places, one of which
   (spec S-7) this task added, and guarded nowhere. Correctly out of Task 5's
   four-condition scope. The cheap producer the report proposes is right: a
   `batch_document` case with a mixed-severity `batch_diagnostics` vector
   asserting it is NOT reordered. Route at the plan close or as a registered
   trigger; do not let the measurement evaporate.
7. **Second consecutive task where the four-condition rule fired on a TOTAL
   gap** (Task 3's persisted field, now Task 5's second call site). The
   companion plan-review handle recorded under
   `tests-ship-with-the-feature-never-after` - "every acceptance observable is
   walked in its HALVES to a named producing test" - would have caught this one
   at planning time: acceptance observable 6 names one subprocess emitter for a
   sort with two call sites. **My symmetric mutation pair is the sharper
   calibration datum**: each call site has exactly one guard, they are disjoint,
   and the plan enumerated only one of them. Worth an occurrence on that entry,
   as evidence the handle is aimed correctly and is still not being applied at
   authoring.
8. **A pre-existing e2e fixture diverges from core in the direction opposite to
   the one the review question assumed.** `loadedForApply`
   (`e2e/smoke.spec.ts:433`) sets `mkvmerge_found: true` on a `load_profile`
   document, but `load_profile_body` passes `None` on **both** arms
   (`src-tauri/src/lib.rs:290`, `:295`), so no `load_profile` envelope ever
   carries the key. Task 5's new fixture is the faithful one; its neighbour is
   not. Nothing in the suite catches fixture-vs-core drift of this shape - the
   mock is semantics-blind by construction. Candidate for the same class of
   finding as plan-6 T14's "27/27 e2e green because the echo mock is
   semantics-blind".
9. **Over-restriction watch,
   `latitude-carveout-zero-content-structural-forks`: no stop to report, one
   wording asymmetry.** Today's amendment spells out ADDING a required import in
   detail; DELETING an import the task's own enumerated deletion made unused
   (this task's CLI case, compiler-forced under `-D warnings`) rests on the older,
   more general "repairing a reference ... (a doc link, a comment referent, an
   import)" clause. Both directions arose in this one task and both were handled
   correctly without routing, so nothing was over-restricted - but the asymmetry
   in how explicitly the two are stated is worth a note if the entry is edited
   again.
10. **A composed doc comment on a re-export is a claim about a call-site set, and
    this one was not checked against it** (MEDIUM-1). The generalizable trigger is
    readable: *you are writing prose that characterizes "every call site" / "all
    consumers" / "the only user" of a symbol you just moved*. The handle is the
    grep you are already required to run for the doc-link sweep - it returns the
    call-site set, so the characterization can be checked against it in the same
    breath. Worth folding into
    `an-import-removal-sweeps-the-doc-links-that-named-the-symbol` (same trigger,
    one extra use of the same output) rather than as a new entry.
11. **The e2e suite drives `dist/`, so any `src/` mutation used as evidence needs
    an intervening `pnpm build`.** The report's quoted mutation runs
    (`pnpm exec playwright test ...` immediately after editing `BatchView.vue`)
    do not show one; the reported colours are nonetheless correct - I reproduced
    both independently *with* explicit rebuilds - but the pasted invocation as
    written would have measured the previously-built bundle. This is a live trap
    for every future frontend mutation-evidence run in this repo
    (`playwright.config.ts` webServer is `vite preview` over `dist/`). Cheap
    process handle worth recording.

---

# Delta review

**Delta verdict: APPROVED.** Fix round `17505d8`, two files, 6 insertions / 1
deletion, tree clean.

Both routed findings close on their own terms. MEDIUM-1 closes cleanly and
completely. LOW-3 closes **as I scoped it** - the comment no longer promises an
assertion the code does not make - but my own first-pass recommendation carried
a false rationale alongside it, the fix implemented that rationale faithfully,
and the result is one new LOW: two assertion messages now claim an
identification neither performs. That is my defect propagating, not the fix
implementer's; it is recorded below with the measurement that settles it.

Nothing else was introduced, in the two files or as a ripple. Full Rust bar and
both frontend legs green on my own runs.

---

## 1. Does MEDIUM-1 close? YES.

The new appositive:

```rust
/// -- this crate's human printing paths and `validate`'s own `--json`
/// envelope -- is unchanged.
```

Checked against a freshly-classified call-site set, not against my first pass's
prose. New instrument (`callsite_class.py`, appendix): it walks outward from each
call to the nearest enclosing `if json` / `else` and prints the surrounding
context, so the classification is auditable rather than asserted.

**Nine call sites, all nine classified:**

| Site | Enclosing branch | Consumed by | Surface |
|---|---|---|---|
| `validate.rs:21` | none - before the `if json` at `:27` | `rendered_diags` -> `{"diagnostics": ...}` at `:28-29` **and** the human loop at `:33-36` | **both** |
| `dry_run.rs:53` | `else` of `if json` (`:47`) | `renderer.diagnostic` | human |
| `dry_run.rs:71` | `else` of `if json` (`:65`) | `renderer.diagnostic` | human |
| `dry_run.rs:85` | `else` of `if json` (`:79`) | `renderer.diagnostic` + `print_batch_human` | human |
| `run.rs:95` | `else` of `if json` (`:85`) | `renderer.diagnostic` | human |
| `run.rs:117` | `else` of `if json` (`:107`) | `renderer.diagnostic` | human |
| `run.rs:135` | `if !json` (`:134`) | `renderer.diagnostic` + `print_batch_human` | human |
| `mod.rs:112` | inside `print_batch_human` | `line(...)` | human |
| `mod.rs:116` | inside `print_batch_human` | `line(...)` | human |

Eight human-only sites plus one that is both. The appositive names exactly that
union: "this crate's human printing paths" covers the eight and validate's human
half; "`validate`'s own `--json` envelope" covers the remaining half of the
ninth. **True and complete - no call site falls outside it, and it claims no
surface that does not exist.**

The dry-run/run `--json` branches do **not** belong in the appositive and are
correctly absent: they call `config_only_document`/`batch_document`/
`run_document`, which sort through **core's** `severity_sorted`, not through
`crate::commands::severity_sorted`. The doc is scoped to the latter's call
sites, and that scoping is now exact.

The text is the brief's verbatim, ASCII `--`, no other line in the file touched.
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` clean; the change
adds no intra-doc link.

**Line-shift ripple, checked:** the added doc line pushed `print_batch_human`'s
two call sites from `:111`/`:115` to `:112`/`:116`. Swept for orphaned citations
(`grep -rnP 'commands/mod\.rs:(111|115)\b' docs/ .superpowers/sdd/plan-9/*.md`,
with a fired control on `commands/mod.rs:\d+` that returns the design's and
plan's real `:21` anchors): the only hit is my own first-pass verdict, a dated
review artifact correct as of `e134fdc`. No normative document cites them.

## 2. Does LOW-3 close? YES as scoped - and the fire is real, the restore complete.

**The fire, on my own mutation, deliberately different from the fix
implementer's.** Their rename of `batch_document`'s `"files"` key to
`"files_MUTANT_FIRE"` removes the key, so it exercises only the `is_some()` half
of `report.get("files").is_some_and(|f| f.is_array())`. The `is_array()` half
went unexercised, and `proc-verification-step-must-be-falsifiable` is explicit
that the duty is **per assertion, not per script**. So I fired the other half:

Mutation - `batch_document`'s envelope emits `"files": serde_json::Value::Null`,
key **present** but not an array:

```
thread 'dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran' panicked at
crates/muxsmith-cli/tests/dry_run_cli.rs:445:5:
expected a planned batch document with a files array, got: {"batch_diagnostics":[],
"config_diagnostics":[... "unknown-property" ... "invalid-regex" ... "raw-on-known-property"
... "raw-property" ...],"files":null,"suggestions":[]}
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out
```

Red at `:445`, the new assertion, on the half their fire did not reach. Both
halves of the predicate are therefore independently fire-verified, one by each
of us. The pasted document also confirms the mutation was surgical: the
four-code sorted sequence is intact.

**Restore complete on the tree I see now**, checked independently of their
proof:

- `git checkout -- crates/muxsmith-core/src/report/json.rs`;
  `sha256sum -c` -> OK on all three files; `git status --porcelain` empty; HEAD
  `17505d8`.
- **`json.rs` hashes `5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a`
  - byte-identical to the baseline I took at `e134fdc` in the first pass.** Their
  fire-and-restore left the emitter exactly as Task 5 committed it. That is a
  stronger proof than a same-session `sha256sum -c` because the reference
  predates their round.
- Residue sweep for **both** markers,
  `grep -rn "DELTAREV\|MUTANT_FIRE" crates/ src-tauri/ src/ e2e/` -> no hits,
  exit 1, with a fired control (`severity_sorted` in `report/mod.rs` -> 4).

**What closes and what does not.** LOW-3 named a comment/code mismatch: the
comment at `:443-444` claimed two properties, the code asserted one. The code now
asserts both. **That mismatch is closed.** What does *not* follow - and what my
own finding wrongly said would - is that the added assertion closes the
identification gap. See LOW-4.

## 3. Anything new introduced? One finding, and its root cause is mine.

### LOW-4 (new): both assertion messages claim an identification neither assertion performs, and the fix report states the gap is closed when it is not

**Sites:** `crates/muxsmith-cli/tests/dry_run_cli.rs:443-444` (comment), `:447`
(new message), `:451` (pre-existing message); and
`.superpowers/sdd/plan-9/task-5-report.md`, fix-2 section.

**The measurement.** I ran the real profile-load-failure path - the exact path
LOW-3 named, `dry_run.rs:40` -> `config_only_document(&[diagnostic], None,
renderer)` - against both assertions:

```
$ ./target/debug/muxsmith dry-run /nonexistent-profile.yaml --json     # exit 2
keys: ['batch_diagnostics', 'config_diagnostics', 'files', 'suggestions']

  ASSERTION 1 (new):  report.get('files').is_some_and(|f| f.is_array())
     files present: True   is array: True   value: []
     -> PASSES on this config_only load-failure document

  ASSERTION 2 (pre-existing): report.get('mkvmerge_found').is_none()
     mkvmerge_found present: False
     -> PASSES on this config_only load-failure document

  config_diagnostics codes: ['parse-error']
```

**A real `config_only_document` from the load-failure path satisfies both
assertions.** `config_only_document` emits `"files": []`, which is an array
(`json.rs:106`). So the new assertion rules out no real emitter shape at all -
every document `batch_document`, `config_only_document` and `run_document`
produce carries `files` as an array. It is a wire-shape regression guard (it
fires if the key is renamed or retyped, as both of our mutations showed), not a
builder discriminator.

Consequently three statements overclaim:

1. the new message, `"expected a planned batch document with a files array"`;
2. the pre-existing message, `"expected a planned batch document"` - which rules
   out only the two config-only shapes that DO carry `mkvmerge_found`
   (mkvmerge-missing, query-failed), not the load-failure one;
3. the fix report's `"That is exactly the identification gap LOW-3 named."` The
   mutation it fired produced a `files_MUTANT_FIRE` document that no code path
   emits, which demonstrates the assertion is bound to the key - a real and
   worthwhile fire - but not that the gap is closed.

The test itself remains sound and I do not ask for it to change: the
load-failure shape is ruled out by the code-sequence assertion below, which
would meet `["parse-error"]` against the expected four codes. That is what my
first pass also said, in the same finding.

**The root cause is my own first-pass text, and I record it as mine.** LOW-3's
required change read: *"Prefer the first: it closes the load-failure path at the
same assertion instead of relying on the next one."* That clause is false, and it
contradicted a correct sentence two lines above it in the same finding. The fix
implementer implemented the recommendation faithfully and cannot be faulted for
it. I also passed over the pre-existing `"expected a planned batch document"`
message in the first pass without flagging its identical overclaim, having quoted
it verbatim.

**Exact required change** (LOW, one comment and two strings; no logic change -
keep the assertion, it earns its place as a shape guard):

```rust
    // Shape guards. `mkvmerge_found` absent rules out the two config-only
    // shapes that carry it (mkvmerge missing, query failed). The
    // profile-load-failure shape carries neither key and is ruled out by the
    // code sequence below, where it would be a singleton `parse-error`.
    assert!(
        report.get("files").is_some_and(|f| f.is_array()),
        "expected a report document carrying a files array, got: {report}"
    );
    assert!(
        report.get("mkvmerge_found").is_none(),
        "expected a planned batch document, not the mkvmerge-missing config-only shape, got: {report}"
    );
```

**Routing note:** the new message and the comment are this round's own output and
sit inside its scope. The pre-existing message was explicitly off-limits to the
fix brief, so editing it needs a fresh licence from the controller. If minimum
churn is wanted, correcting only the new message and the comment closes the
overclaim this round introduced; the pre-existing message's identical overclaim
is a first-pass miss of mine and can ride the plan close. Not worth a fix round
of its own.

### Nothing else new

- **Scope.** `git diff --stat e134fdc..17505d8` over code: exactly the two
  files. No drive-by edits, no doc changes outside `commands/mod.rs`, no test
  addition beyond the single named assertion.
- **Test-count invariant held.** `dry_run_cli.rs` is still **13 passed** - the
  fix added an assertion to an existing test, not a test. Had that number moved,
  it would have meant a new test slipped in.
- **The diff-paste collision** the fix report flags itself (a removed `///` line
  colliding with the `-` marker) is self-disclosed and the authoritative
  `git show 17505d8` matches. No finding.

### The no-work-needed check on the fix report's premises

- **"No `pnpm` leg: the frontend is untouched by this round."** Premise RUN, not
  weighed: `git diff --name-only e134fdc..17505d8 -- src/ e2e/ package.json`
  returns nothing, with a fired control (the same command over `crates/` lists
  the two files). I ran both frontend legs anyway - `pnpm lint` clean,
  `pnpm test:e2e` **64 passed**, unchanged from my first pass. Premise correct.
- **"`--document-private-items` deliberately NOT added, per brief."** Following
  an instruction, not a no-work conclusion - but I re-ran it: still two errors,
  still `src-tauri/src/lib.rs:54:21` and `:87:15`, still pre-existing and
  untouched. **HARVEST 4 of the first pass stands unchanged and still blocks the
  scheduled close action.**
- **"the cheaper edit is to the older message ... which is a one-line follow-up,
  not a defect."** This is a no-work conclusion and it is the one premise that
  does not survive - adjudication 4.

## 4. Adjudication: the near-duplicate assertion messages

**Ruling: it is a real defect, but not the one the fix implementer described,
and the edit does not belong where it proposes.**

Taking both directions honestly:

**For "acceptable as it stands."** The strings differ by a trailing clause, they
print only on failure, and each prints the whole document alongside, so a reader
who hits either one sees everything needed regardless of the wording. The fix
brief put the older message off limits, and shaping the new one like its
neighbour was the instruction - so on the letter of its own scope the round did
the right thing, and touching the old string would have been the deviation.
Under-differentiated diagnostic text is, in isolation, cosmetic.

**For "a real defect."** The measurement above changes what the question is. The
problem is not that two strings look alike; it is that **both make the same claim
and neither assertion supports it**. A near-duplicate of a true message is a
readability nit. A near-duplicate of a *false* message is two instances of the
defect MEDIUM-1 was raised for one round earlier - prose asserting more than what
was verified - now in a test that a future reader will consult precisely to learn
what identifies a planned batch document. And this test is the sole guard on
`batch_document`'s sort, so a reader trusting its stated discrimination is
trusting it at exactly the point where it matters.

The second reading wins, and the near-duplication turns out to be a symptom worth
having: the two messages coincide *because* they assert the same unsupported
thing.

**Which message.** The fix implementer proposes editing the **older** one
(appending "without mkvmerge_found"). That is backwards on both counts.

- The **new** message is the one whose claim has no support at all - `files` as
  an array is true of every document any builder in this codebase emits, so
  "expected a planned batch document with a **files array**" derives an
  identification from a property with zero discriminating power. It is also this
  round's own output, so correcting it needs no new licence.
- The **older** message's claim is merely too broad rather than empty - absent
  `mkvmerge_found` genuinely rules out the two config-only shapes that carry it.
  Sharpening it ("not the mkvmerge-missing config-only shape") is worthwhile and
  disambiguates the pair as a side effect, but it is the second edit, not the
  first, and it needs the licence the fix brief withheld.

So: **disambiguate, on the NEW message first**, and take the older one only with
a controller licence. Exact text in LOW-4. Severity LOW; route as a one-line
follow-up or plan-close residue, not a fix round.

---

## Delta verification: the bar, re-run on my own instruments

Tree at `17505d8`, working tree clean, all foreground.

| Check | My result |
|---|---|
| `cargo fmt --all --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | 0 errors, 0 warnings |
| `cargo test --workspace` | exit 0; **39 `test result:` lines, all ok**, 0 non-ok, **0 skip markers**; `dry_run_cli.rs` **13 passed** |
| `pnpm lint` | clean |
| `pnpm test:e2e` | **64 passed** |
| `python3 scripts/ledger-lint.py` | 508 entries across 4 files, all invariants hold |

Zero skip markers matters here as it did in the first pass: the assertion the fix
adds lives inside a `have_mkvmerge()`-gated test, so a green run with a skipped
test would prove nothing about it.

**Intervening house commit `d0e160a` checked for ground-truth movement**, since it
landed between `e134fdc` and the fix. It touches `docs/ROADMAP.md`,
`docs/decision-ledger.yaml`, `docs/process-conventions.yaml`, and in the two
Tier-2 entries this review leans on
(`latitude-carveout-zero-content-structural-forks`,
`tests-ship-with-the-feature-never-after`) it adds one occurrence each and bumps
`count` 11->12 and 2->3. **No statement text changed**, so every ruling in my
first pass and in this delta rests on the same text it rested on before. Counts
recomputed against their enumerations: both correct.

## Delta instruments

New, under the same directory
(`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t5rev-independent/`).
None re-runs the fix implementer's.

| File | What it is |
|---|---|
| `callsite_class.py` | fresh call-site classifier for MEDIUM-1: walks each `severity_sorted(` call to its enclosing json/else branch and prints the context |
| `delta-baseline.sha256`, `delta-head.txt` | byte baseline of the three relevant files taken before my mutation |
| `delta-cargo-test.txt` | full `cargo test --workspace` at `17505d8` |
| `loadfail.json`, `loadfail.err` | the real profile-load-failure `--json` document, the measurement behind LOW-4 |

My mutation (`"files": serde_json::Value::Null`) was applied inline via heredoc'd
Python against that baseline, fired, then reverted with `git checkout --` (never
a bare `cp`) and proven by `sha256sum -c` plus the cross-round hash identity
noted above.

## Delta HARVEST

1. **A reviewer's "exact required change" is itself a claim, and this one shipped
   a false rationale into a fix round.** LOW-3 offered two remedies and preferred
   one on a reason ("it closes the load-failure path at the same assertion") that
   contradicted a correct sentence two lines earlier in the same finding, and
   that I never measured. The fix implementer implemented it faithfully; the
   defect travelled one hop and became two false assertion messages. The trigger
   is readable and belongs with the existing borrowed-claim family: **you are
   writing a preference between two remedies, and the preference rests on a
   behavioural claim about code you have not run.** The handle is the one already
   standing for evidence in reports - run it, or state the remedy without the
   comparative. Worth an occurrence on the call-site/measurement entry the
   controller created from this review, since it is the same class one level up:
   prose about code, checked against the code.
2. **An assertion can be fire-verified and still not do the job its message
   claims.** Both of us fired the new `files` assertion successfully - one half
   each - and it is genuinely red-able. Neither fire touched the question of
   whether any *real* emitter output could fail it, and the answer is no. The
   distinguishing question, cheap and rarely asked: *which real code path
   produces a document that fails this assertion?* If the answer is "none, only a
   mutation", it is a shape guard, not a discriminator, and its message must not
   claim discrimination. This generalizes past `proc-verification-step-must-be-
   falsifiable`, which asks whether a check can fire at all, not whether anything
   reachable makes it fire.
3. **Per-assertion fire duty proved its worth on a two-predicate `assert!`.** The
   fix implementer's rename exercised `is_some()`; only my null-value mutation
   exercised `is_array()`. A single `assert!` whose condition is a compound
   predicate is two checks wearing one macro, and the existing per-assertion rule
   should be read to reach inside it. Cheap sharpening if that entry is edited.
4. **Unchanged and still owed:** first-pass HARVEST 4 (the
   `--document-private-items` rustdoc failure at `src-tauri/src/lib.rs:54`,
   `:87`) is live and must be fixed before the plan close adds that flag to the
   gate; first-pass HARVEST 6 (D102's unguarded scope boundary) now has its
   ROADMAP close action per `d0e160a`. Task 6's anchors and baselines from the
   first pass are unaffected by this round - it touched no frontend file and no
   `e2e/` file, so `pnpm test:e2e` at **64 passed** and `cargo test --workspace`
   at **39 `test result:` lines** remain the numbers Task 6 measures its delta
   against.
