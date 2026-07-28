# Task 4 implementer report - Plan 9 (`EmptyRawProperty`, D101; spec S-1 new row, S-3, S-5, S-6)

**Status: NEEDS_CONTEXT.** Nine of the ten Files-list entries are complete,
verified and committed (`d768657`). **One pinned test is not implemented and
is not in the commit:** Step 4's second subprocess test,
`bare_raw_property_renders_german_with_locale_flag`, cannot be written as the
plan pins it without reopening D64 (the closed en-pinning funnel) or editing
`crates/muxsmith-cli/tests/support/mod.rs`, which is not on Task 4's exhaustive
Files list. That is a fork on code contact; the decision memo is section 4. Its
snapshot file (the tenth Files-list entry) consequently does not exist.

Everything else in Steps 1-8 landed, the full Step-7 bar is green, and the
red-then-green exit-code demonstration is in section 3.

The pre-routed fork (uncovered user-visible consequence) **did not fire**;
section 5 gives the evidence for why, since an absence claim of that shape is
what the routing exists to catch.

---

## 1. Per-file changes against the Files list

| # | Files-list entry | State |
|---|---|---|
| 1 | `crates/muxsmith-core/src/report/mod.rs` | DONE. `EmptyRawProperty => "empty-raw-property"` added between `RawOnKnownProperty` and `CodecKindExactOnly`, with D101's doc comment (see divergence D-1 on the `get("")` escaping). |
| 2 | `crates/muxsmith-core/src/profile/validate.rs` | DONE. `raw_opt_in_diagnostic` replaced by D101's three-branch fence, character for character. Its rustdoc gained one sentence for the new branch (divergence D-2). |
| 3 | `locales/en/diagnostics.ftl` | DONE. `empty-raw-property` line inserted after `raw-on-known-property` (`:15`), byte-identical to D101's fence (proof in section 3). |
| 4 | `locales/de/diagnostics.ftl` | DONE. Same, `:22`, byte-identical, real orthography (`ä`, `ö`, `ß`) confirmed at byte level. |
| 5 | `crates/muxsmith-cli/tests/catalog_completeness.rs` | DONE. `DiagCode::EmptyRawProperty => vec![]` after the `RawOnKnownProperty` row. |
| 6 | `crates/muxsmith-core/tests/validate_semantics.rs` | DONE. Both pinned per-arm tests, placed after the B-1..B-4 block. No duplicate control written; B-2/B-3 are the control and pass unchanged in the same run. |
| 7 | `crates/muxsmith-cli/tests/cli_validate.rs` | **PARTIAL.** `bare_raw_property_exits_two_and_renders_the_message` implemented. `bare_raw_property_renders_german_with_locale_flag` **not implemented** - blocked, section 4. |
| 8 | `crates/.../snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap` | DONE (created, insta-accepted, contains the en text of D101's fence). |
| 9 | `crates/.../snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap` | **NOT CREATED** - its test does not exist. |
| 10 | `e2e/smoke.spec.ts` | DONE. One addition: the amendment-1 Run-gate scenario in the `batch view: dry run` describe, plus `enAttr` on the existing `./i18n-en` import line, which the scenario's title assertion requires (divergence D-3). Nothing else in the file touched. |
| 11 | `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` | DONE. S-1 (EmptyRawProperty row only), S-3, S-5, S-6, each byte-identical to its design fence (proof in section 3). |

Scenario placement in `smoke.spec.ts`: after the plural-counts test, before the
Task-14 apply block. Deliberate - it maximizes distance from where Task 5's
parse-failure apply scenario will most plausibly land, under the serial
smoke-region ownership scheme.

---

## 2. Verification bar (plan Step 7, foreground, no subsets)

| Check | Result |
|---|---|
| `cargo fmt --all --check` | `FMT_OK` (exit 0, no output) |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 1.16s` - zero warnings |
| `cargo test --workspace` | 39 test binaries, **every one** `test result: ok`, `0 failed`; no pending snapshots (`ls .../snapshots/*.new` -> `no matches found`) |
| `pnpm check:i18n` | `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| `pnpm lint` | `$ eslint .` - exit 0, no findings |
| `pnpm test:e2e` | `63 passed (2.8s)`, including `✓ 37 [chromium] › e2e/smoke.spec.ts:369:3 › batch view: dry run › an error-severity config diagnostic disables Run with the errors tooltip (D101's Run gate) (189ms)` |

Beyond the bar (cheap, and both are gate parts my change could plausibly break):

- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -> `Finished`,
  zero warnings (the new doc comment contains `get("")` and backticks).
- Typography sweep over every changed file for the AI-tell glyph set
  (em/en dash, figure dash, horizontal bar, U+2212, smart quotes, U+2026,
  U+00A0): **zero hits**. Fire-verified rather than trusted: the same pattern
  against `printf 'em\xe2\x80\x94dash'` returns `1`.

Discriminating controls green in the same `cargo test --workspace` run
(`validate_semantics`, 24 passed):

```
test b3_raw_unknown_substring_is_raw_property_info_no_type_error ... ok
test b2_raw_unknown_exact_is_raw_property_info_untyped ... ok
test empty_bare_raw_substring_is_empty_raw_property_error ... ok
test empty_bare_raw_exact_is_empty_raw_property_error ... ok
```

No snapshot churn beyond the one new file (`git show --stat`, section 7).

---

## 3. Evidence

### 3.1 The red-then-green exit-code flip (this task's red-today half)

**RED - the new CLI test on the pre-change tree** (test written first, core
untouched):

```
$ cargo test -p muxsmith-cli --test cli_validate bare_raw_property_exits_two_and_renders_the_message
test bare_raw_property_exits_two_and_renders_the_message ... FAILED

thread '...' panicked at crates/muxsmith-cli/tests/cli_validate.rs:78:10:
Unexpected return code, failed var == 2
└── var: 0

command=`"/home/senol/Git/Muxsmith/target/debug/muxsmith" "validate" "/tmp/.tmpQlwPOY/bare-raw.yaml" "--locale" "en"`
code=0
stdout=```
[info] tracks[0].match.exact.raw:: Property \"\" is used with a raw: prefix; it bypasses the capability model and is matched untyped. This is the opt-in for forward compatibility with a newer mkvmerge identification schema.
0 errors, 0 warnings, 1 info.
```
```

Same state at the binary, independently (the authoring probe reproduced):

```
$ ./target/debug/muxsmith validate <bare-raw.yaml> --json --locale en; echo "EXIT=$?"
{"diagnostics":[{"code":"raw-property","config_path":"tracks[0].match.exact.raw:","params":{"property":""},"rendered":"[info] tracks[0].match.exact.raw:: Property \"\" is used with a raw: prefix; ...","severity":"info"}]}
EXIT=0
```

**GREEN - the same test after Steps 1-2.** The `.code(2)` assertion passes
(the run proceeds past it to the snapshot comparison, which is what produced
the new snapshot):

```
$ cargo test -p muxsmith-cli --test cli_validate bare_raw_property_exits_two_and_renders_the_message
Snapshot: bare_raw_property_exits_two_and_renders_the_message
+new results
  1 │+[error] tracks[0].match.exact.raw:: The raw: prefix requires a property name: a bare "raw:" names no property, and the rule could never match any track. Add the property name after the colon (for example raw:dolby_complexity_index).
  2 │+1 error, 0 warnings, 0 infos.
```

Snapshot accepted via `cargo insta accept --workspace`; the test is green in
the workspace run above.

**GREEN at the binary, both locales, plus the emptiness discriminator:**

```
$ ./target/debug/muxsmith validate <bare-raw.yaml> --locale en; echo "EXIT=$?"
[error] tracks[0].match.exact.raw:: The raw: prefix requires a property name: a bare "raw:" names no property, and the rule could never match any track. Add the property name after the colon (for example raw:dolby_complexity_index).
1 error, 0 warnings, 0 infos.
EXIT=2

$ ./target/debug/muxsmith validate <bare-raw.yaml> --locale de; echo "EXIT=$?"
[Fehler] tracks[0].match.exact.raw:: Das raw:-Präfix erfordert einen Eigenschaftsnamen: ein bloßes "raw:" benennt keine Eigenschaft, und die Regel könnte nie auf eine Spur zutreffen. Ergänze den Eigenschaftsnamen nach dem Doppelpunkt (zum Beispiel raw:dolby_complexity_index).
1 Fehler, 0 Warnungen, 0 Infos.
EXIT=2

$ ./target/debug/muxsmith validate <nonempty-raw.yaml> --locale en; echo "EXIT=$?"
[info] tracks[0].match.exact.raw:dolby_complexity_index: Property "dolby_complexity_index" is used with a raw: prefix; ...
0 errors, 0 warnings, 1 info.
EXIT=0
```

The last block matters twice: it is the branch discriminator at binary level
(the new branch fires on emptiness, not on `raw:` generally), and it shows the
German rendering is correct and reachable - only the pinned TEST vehicle is
blocked, not the feature (section 4).

### 3.2 The Run-gate e2e scenario carries no red-today claim

Per the plan and my brief, it asserts existing gating behaviour that nothing
asserted before. It is green in the run above. **No red state was manufactured
and none is reported.**

Two facts I re-measured rather than borrowed, because the scenario's value
rests on them:

- `hasErrors` gating exists today (`BatchView.vue:282`) and
  `runDisabledReason` returns `"tooltip-errors"` for it (`:304`), after the
  run-active / no-profile / mkvmerge-missing branches - so
  `mkvmerge_found: true` plus a completed pick closes those three by
  construction, which is why the `title` assertion (not just `disabled`)
  is what discriminates.
- The `title` expectation is derived, never hand-duplicated:
  `enAttr("batch-run", "tooltip-errors")` reads the real en catalog, the same
  idiom `help-mode.spec.ts:147` and `editor-tooltips.spec.ts:33` use.

The assertion is not vacuous: `getByTestId("batch-run")` resolving to nothing
would fail the `toBeDisabled` call rather than pass it, and a wrong reason key
fails the `toHaveAttribute` comparison.

### 3.3 Character-for-character contracts

Machine-compared, not eyeballed (Python, exact string equality against the
design's own fences):

```
en identical: True
de identical: True
S-1 row: in design fence=True  in spec=True
S-3:     in design fence=True  in spec=True
S-5:     in design fence=True  in spec=True
S-6:     in design fence=True  in spec=True
```

`cat -A` on the de line confirms the letters are real UTF-8 orthography
(`PrM-CM-$fix`, `bloM-CM-^_es`, `kM-CM-6nnte` = `Präfix`, `bloßes`, `könnte`),
not transliterations.

### 3.4 Sweeps the standing rules require

- **`use`-line deletions:** none in this diff (the only import change is an
  addition, `enAttr`). No doc-link sweep owed.
- **Count-word sweep on the sets I extended.** Grep for numeric counts over
  the DiagCode / catalog-id sets across `crates src src-tauri docs locales
  scripts e2e`: every hit sits in `docs/process-journal/` - dated records of
  past runs, not live normative counts. No live count exists in code, the v1
  spec, the plan, the Plan-9 design, or the four house YAMLs. The grep is
  fire-verified: it returns dozens of hits (so it is not malformed), and the
  control search for a known-present phrasing found D64's snapshot counts.
  **One stale consumer found, in a file I may not touch - surfaced in
  section 6, item 1.**
- **Null-assertion rule:** the two core tests assert presence positively
  (`empties.len() == 1` over a filtered vector, then `severity`/`config_path`
  on the found element). The `!cs.contains(...)` lines are absence assertions
  over a vector that is provably non-empty in the same test.
- **No house-knowledge YAML edited** (`git show --stat`, section 7).

### 3.5 Probe mutation and its restoration

To produce the evidence in section 4 I temporarily wrote the German test into
`cli_validate.rs`, ran it, then reverted. Baseline taken before mutating,
restored non-interactively, restoration proven:

```
$ sha256sum crates/muxsmith-cli/tests/cli_validate.rs   # baseline, before
599bb699b5529c04ed63a0972f59b59e79980cb943738453a2c8ae2cd66c043d  crates/muxsmith-cli/tests/cli_validate.rs

$ sha256sum -c <baseline>                               # after reverting
crates/muxsmith-cli/tests/cli_validate.rs: OK
```

The probe also left an insta `.snap.new`; it was deleted, and the count of
pending snapshots afterwards is `0`. `git status --porcelain` at that point
listed exactly the nine intended modifications plus the one new snapshot, no
residue. No `cp` was used anywhere in this task.

---

## 4. Decision memo: the pinned German subprocess test (BLOCKING FORK)

### The fact

`crates/muxsmith-cli/tests/support/mod.rs`'s funnel appends `--locale en`
**after** the caller's args unconditionally (`cmd.args(["--locale", "en"])`,
`:92`). clap rejects a repeated `--locale`, measured:

```
$ ./target/debug/muxsmith validate <p> --locale de --locale en; echo "EXIT=$?"
error: the argument '--locale <LOCALE>' cannot be used multiple times
EXIT=2
```

So Step 4's `args plus --locale de` through `support::muxsmith` produces
`validate <p> --locale de --locale en`, which never reaches the renderer.

**This is worse than a plain failure, and that is the reason I am returning it
rather than writing something adjacent.** I wrote the test exactly as the plan
pins it and ran it:

```
$ cargo test -p muxsmith-cli --test cli_validate bare_raw_property_renders_german_with_locale_flag
stored new snapshot .../cli_validate__bare_raw_property_renders_german_with_locale_flag.snap.new
test bare_raw_property_renders_german_with_locale_flag ... FAILED

Snapshot: bare_raw_property_renders_german_with_locale_flag
+new results
────────────┬────────────────────────────────────────────────────────────────
────────────┴────────────────────────────────────────────────────────────────
```

The `.code(2)` assertion **passes** - clap's own usage error is also exit 2 -
and the new snapshot is **empty**, because clap writes to stderr. An
implementer who accepts that snapshot ships a green test that proves nothing
about German rendering and nothing about D101. The design's own acceptance
condition ("the snapshot must contain the de text of D101's fence") is the only
thing that catches it.

### Why I did not resolve it at the keyboard

Every available route crosses a line the plan or the design draws:

- `support::muxsmith_bare()` exists and would work, but D64 closes its
  exception set at exactly two named callers, both "locale-moot by
  construction: no `Renderer` exists on either path"
  (`2026-07-21-plan7-help-i18n-design.md:1599-1601`, mirrored in the helper's
  own rustdoc). A German-rendering test is the opposite of locale-moot, and
  the design says a third caller "reopens D64 rather than riding the helper".
  Using it would also falsify the helper's enumeration in
  `support/mod.rs`, a file I may not edit.
- A locale-parameterized funnel helper, or making the funnel append
  `--locale en` only when the caller passed none, both edit
  `crates/muxsmith-cli/tests/support/mod.rs` - not on Task 4's EXHAUSTIVE
  Files list, and the second also weakens D64's "pinned by construction"
  property for every existing caller.
- Env-var pinning (`LC_ALL`/`LANG` on the child) is D64's explicitly
  **rejected** alternative (portability: `sys_locale` reads OS APIs, not env
  vars, on Windows and macOS).

### Options, with costs against the named invariants

| # | Option | Cost |
|---|---|---|
| A | Add a third `muxsmith_bare()` caller in `cli_validate.rs`, and amend the helper's rustdoc + D64's exception block to a three-caller set with the new one's justification. | Reopens D64, which the plan's "no design decision is re-opened" forbids to a task; touches an unlisted file and a closed plan's design. Cheapest in code, most expensive in doctrine. |
| B | Add `pub fn muxsmith_localized(args: &[&str], locale: &str)` to `support/mod.rs` (one `cargo_bin` call inside the same file, so D64's greppable invariant - `cargo_bin("muxsmith")` in exactly one file - holds verbatim) and use it for this one test. | Extends the Files list by one file. D64's *mechanism* (pin explicitly, never by environment, `cargo_bin` in one file) is preserved rather than reopened; only its "one funnel" wording widens to "one funnel per pinned locale". |
| C | Drop the German test; keep only the en subprocess test. | Loses the only end-to-end proof that the de fence renders. Weak, but note the de line is already covered indirectly: `pnpm check:i18n` enforces en/de key parity, `catalog_completeness` renders every code, and section 3.1 shows the de text rendering correctly at the binary. |
| D | Defer the German test to a later plan item. | Contradicts amendment 1's ruling A (a feature's tests ship with the feature) at exactly the granularity that ruling was made about. |

### Recommendation

**Option B.** It is the only route that satisfies D101's pinned test without
contradicting D64: the invariant D64 actually enforces is the greppable one
(`cargo_bin("muxsmith")` in exactly one file, no env-var pinning, every
invocation explicitly locale-pinned), and B keeps all three. The two-caller
closure D64 states is about the *bare, unpinned* helper - a genuinely different
thing from a second *pinned* helper. Cost is honest and small: one new function
in one unlisted file, plus the one-line update to D64's mechanism bullet if the
owner wants the design to record it.

If B is ruled, the concrete follow-up work is: `muxsmith_localized` in
`support/mod.rs`, the pinned test in `cli_validate.rs`, and its snapshot -
about fifteen lines, all verifiable by the same bar this task already ran.

**I did not implement any option.** The commit contains the en test only.

---

## 5. The pre-routed fork: checked, did not fire

My brief pre-routes one fork: a user-visible consequence this task introduces
whose coverage the plan's enumerated tests do not carry. I enumerated the
consequences D101 names and traced each to an assertion. All are covered, so
there is nothing to route - but the negative is only worth stating with the
evidence attached, so:

| Consequence D101 names | Covering chain |
|---|---|
| `validate` exits 2 instead of 0 | Directly: the new `bare_raw_property_exits_two_and_renders_the_message`. |
| `dry-run` exits 2 | Link 1 (new): the two core tests prove bare `raw:` yields an error-severity diagnostic. Link 2 (existing): `dry_run_cli.rs::dry_run_surfaces_config_time_invalid_regex` asserts `out.status.code() == Some(2)` (`:104`) for an error-severity config diagnostic. |
| `run` exits 2, muxing nothing that fails to plan | Link 1 as above. Link 2 (existing): `run_cli.rs::bad_regex_profile_exits_two_without_executing_a_job` asserts `Some(2)` (`:69`) plus `asserts_no_job_ran`. |
| GUI Batch view: Run gate disabled | This task's new e2e scenario - the one the owner ruled must ship here, precisely because no existing assertion covered link 2. |
| GUI editor: Save gate "behaves like any other error" | Link 1 as above. Link 2 (existing, and I verified it fires in this task's own run): `✓ 57 [chromium] › e2e/smoke.spec.ts:1251:3 › editor view: open/save (Task 13, D45/D41) › ... Save is disabled while an error diagnostic exists and enabled when clean; saving calls save_profile`. |

The Run gate and the Save gate are the same shape, and the difference is
exactly why one needed a new test and the other did not: `batch-run`-disabled
was asserted **nowhere** before this task, `settings-save`-disabled-on-error
was already asserted. That asymmetry is what the amendment-1 ruling turned on,
and it holds on measurement.

I claim no more than this: the consequences **D101 enumerates** are covered. A
consequence nobody has named is not something a coverage walk can find.

---

## 6. Surfaced for the controller (no task edits house YAMLs; I edited none)

1. **A stale count in a closed plan's design, invalidated by this task, in a
   file I may not touch.** `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
   D64 says "The funnel covers all **11 insta snapshots** (`tests/snapshots/`,
   counted)" (`:1563`) and enumerates "`cli_validate.rs` (1 constructor, 3
   snapshots)" (`:1556`). This task's new snapshot makes those 12 and 4;
   recounted: `ls crates/*/tests/snapshots/*.snap | wc -l` -> `12`,
   `cli_validate__*` -> `4`. Both numbers are inside a dated measurement block
   ("measured: `cargo_bin` grep, 2026-07-21"), so one defensible reading is
   that they are a historical record and go nowhere stale; the "covers **all**
   11" phrasing is the part that now reads as a live claim. Not repaired: the
   file is not on Task 4's Files list, and the plan's "nothing else under
   `docs/` is touched by any task" binds. Controller/owner call. (Task 5 adds
   no snapshot per its Files list, so this does not compound during Plan 9.)
   D64's greppable invariant itself still holds, re-measured:
   `grep -rl 'cargo_bin("muxsmith")' --include='*.rs' crates` -> exactly
   `crates/muxsmith-cli/tests/support/mod.rs`.
2. **Ledger-worthy, if the owner agrees it generalizes** (I am not writing
   it): a plan step that pins a test's *invocation form* against a repo whose
   test-invocation surface is a closed funnel can pin an invocation that is
   impossible - and, as measured here, one whose exit-code assertion passes
   for the wrong reason while its content assertion is empty. The readable
   trigger: a task step names CLI arguments for a test, and the repo has a
   single mandated invocation helper. The handle: run the invocation once
   before writing the step. This one was catchable at plan-authoring by a
   probe of the exact form (the authoring section has probes for the profile
   syntax and the diagnostic path, but none for the German invocation).
3. Not a finding, a note for whoever runs the plan close: the two
   `--locale`-repetition measurements in section 4 are the reason the
   ten-part gate would not have caught this either - the test as pinned
   *fails*, it does not silently pass, but only because of the snapshot half.

---

## 7. Divergences and judgment calls, each named

**D-1: `get(\"\")` -> `get("")` in the doc comment.** D101 gives the doc
comment as prose inside double quotes, with the inner quotes backslash-escaped
by that quoting. I transcribed the *content*, resolving the escapes:
`` `get("")` ``. Writing `get(\"\")` literally would put backslashes in the
rendered rustdoc, which is not what the design describes. Judgment call, not a
design choice; flagged so a reviewer can rule it either way in one line.

**D-2: `raw_opt_in_diagnostic`'s rustdoc gained one sentence.** The design
fences the function body, not its doc. The existing doc enumerated exactly two
outcomes ("`RawOnKnownProperty` (warning) when ...; otherwise `RawProperty`
(info)"), which my own enumerated edit made false. I added one sentence naming
the empty-name branch and its missing `property` param, preserving every
existing word (the following lines are reflowed, not rewritten). Rationale:
this is a reference the task's own edit invalidated, inside a LISTED file, with
zero outward effect - the case
`latitude-carveout-zero-content-structural-forks` names as in scope after the
2026-07-28 owner amendment, and the case amendment 3 exists because it was
*not* done. I judged it distinguishable from amendment 3's situation, where the
design had pinned the doc text itself. If the reviewer disagrees, the sentence
is a one-line revert.

**D-3: `enAttr` added to `smoke.spec.ts`'s existing `./i18n-en` import.**
Required by the scenario's `title` assertion; the file's binding convention
forbids hand-duplicated literals. Addition only, no deletion - it does not
collide with Task 6's planned removal of `FluentVariable` from a different
import line.

**D-4: the commit stages ten paths, not the plan's eleven.** The German
snapshot is absent because its test is. Everything else in the plan's `git add`
line is staged, by name, no `git add -A`.

**D-5: I committed a task that is returning NEEDS_CONTEXT.** The committed
subset is fully verified against the whole Step-7 bar and is what a fix round
would keep in any of options A-D. Leaving ten files uncommitted across an
owner-routing interval seemed the worse of the two risks. Reversible: the
commit is local, unpushed, and a fix round can amend or add to it.

---

## 8. Numbered concerns a reviewer can rule on yes/no

1. **Is D-1 (`get(\"\")` -> `get("")`) the correct transcription?**
   My read: yes, the escapes are an artifact of the design's own quoting.
2. **Is D-2 (the one added rustdoc sentence in `validate.rs`) inside the
   structural-conformance grant, or should it have been surfaced instead of
   applied?** My read: inside it - listed file, own edit invalidated it, zero
   outward effect. This is the exact class the T2/T3 reviews split on, so it
   deserves an explicit ruling rather than silence.
3. **Is the memo's option B the right resolution of the German-test fork, and
   does the plan reader agree that B preserves D64 rather than reopening it?**
4. **Is committing the verified subset (D-5) acceptable, or should a
   NEEDS_CONTEXT task leave the tree dirty?** Either answer is cheap to act on
   now; it is worth a standing rule, because Tasks 5 and 6 can hit the same
   shape.
5. **Should the stale D64 snapshot counts (section 6 item 1) be repaired, and
   by whom?** They are in a closed plan's design and outside every Plan-9
   task's Files list.
6. **Is the scenario's placement inside the batch describe (after the
   plural-counts test) acceptable**, given Task 5 writes the same file next?

---

## 9. Commit

`d768657` on `master`, main worktree, unsigned (`git log -1 --format='%G?'` ->
`N`), exactly one trailer, no `Claude-Session` line, not pushed.

```
$ git show --stat HEAD
commit d7686571e815eefa04f299f6f8d2e8ca2232ea16
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 20:16:58 2026 +0200

    validate: bare raw: with an empty property name is an error, own DiagCode + Run-gate e2e (D101, S-1/S-3/S-5/S-6)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/tests/catalog_completeness.rs  |  1 +
 crates/muxsmith-cli/tests/cli_validate.rs          | 26 ++++++++++
 ...property_exits_two_and_renders_the_message.snap |  6 +++
 crates/muxsmith-core/src/profile/validate.rs       | 22 +++++----
 crates/muxsmith-core/src/report/mod.rs             |  2 +
 crates/muxsmith-core/tests/validate_semantics.rs   | 41 ++++++++++++++++
 .../specs/2026-07-08-muxsmith-v1-design.md         |  7 +--
 e2e/smoke.spec.ts                                  | 56 +++++++++++++++++++++-
 locales/de/diagnostics.ftl                         |  1 +
 locales/en/diagnostics.ftl                         |  1 +
 10 files changed, 151 insertions(+), 12 deletions(-)
```

Working tree clean after the commit (`git status --porcelain` empty). No
house-knowledge YAML, no ROADMAP, no plan or design file appears in the diff.

---
---

# Task 4 fix round (amendment 4) - delta report

**Status: DONE.** The blocking fork from section 4 was ruled (owner: option B
with the single-construction-site sharpening), the plan was amended twice
(`ba69c36`, `4e5daa6`), and this round implements Step 4's amended invocation
vehicle. Task 4's Files list is now complete: all ten entries done, both pinned
subprocess tests present, both snapshots created and content-checked.

Second commit: `3412fcc`. Nothing I dispute.

## F.1 What I wrote, per file

### `crates/muxsmith-cli/tests/support/mod.rs`

Three changes, exactly the amendment-4 scope, nothing else in the file:

1. **`muxsmith`'s body is now the delegation**, verbatim as pinned:
   `muxsmith_localized(args, "en")`. Nothing else in the body.
2. **`muxsmith_localized(args: &[&str], locale: &str) -> Command` added**, the
   old funnel body with the locale parameterized: `cargo_bin("muxsmith")`,
   `cmd.args(args)`, then `cmd.args(["--locale", locale])` LAST. Placed
   immediately after the funnel, so its rustdoc's "points at the funnel's doc"
   reads in source order.
3. **Both rustdocs.** The funnel's dropped the two statements the delegation
   falsified ("builds its `Command` here"; the appending description, which now
   lives in the helper) and the function-level invariant wording; it keeps the
   D64 contract statement byte-identical ("Pinning rides the CLI's own
   contractual surface, never environment variables: `sys_locale` reads OS
   APIs, not env vars, on Windows and macOS (D64's rejected alternative)"), and
   states the delegation plus the invariant at FILE level, naming both call
   sites. The helper's own rustdoc states it is the pinned path's single
   construction site and carries the append/subcommand mechanics, names
   `muxsmith` as its `"en"` delegation, and explicitly does NOT restate the D64
   rationale or the invariant - it points at the funnel's doc for both, as the
   amendment requires.

**`muxsmith_bare` and its closed two-caller exception doc: byte-identical.**
Machine-checked rather than eyeballed, block extracted from
`git show HEAD:...` and from the working tree:

```
muxsmith_bare block (doc + attribute + fn) byte-identical: True
block length: 921 bytes, 16 lines
CONTROL (compare against a mutated copy, must be False): False
```

The control is there because an equality check that can only return True
proves nothing.

**One judgment call, measured rather than assumed.** My first draft gave
`muxsmith_localized` an `#[allow(dead_code)]`, reasoning from the file's four
existing instances (the helper is called by only one test file). **That was
wrong, and I removed it.** The lint does not fire, because `muxsmith` delegates
to it, so it is reachable in every binary that uses the funnel. Verified in
both directions rather than by argument:

```
$ cargo clippy -p muxsmith-cli --all-targets -- -D warnings   # attribute removed
    Finished `dev` profile ...                                 # no warning

# control - is dead_code live in this module at all?
$ printf '\npub fn probe_unused_helper() -> u8 { 0 }\n' >> support/mod.rs
$ cargo clippy -p muxsmith-cli --all-targets -- -D warnings
error: function `probe_unused_helper` is never used
    = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`
error: could not compile `muxsmith-cli` (test "cli_validate") due to 1 previous error
error: could not compile `muxsmith-cli` (test "run_live") due to 1 previous error
error: could not compile `muxsmith-cli` (test "run_cli") due to 1 previous error
```

So the attribute would have been dead weight AND a false signal (it would say
"not every binary uses this", which is untrue). Probe removed afterwards;
proof in F.4.

### `crates/muxsmith-cli/tests/cli_validate.rs`

`bare_raw_property_renders_german_with_locale_flag` added after the en test:
the identical profile (`profile_version: 1`,
`input: { pattern: 'E(\d+)', extensions: [mkv] }`, one rule
`- match: { exact: { 'raw:': eng } }`), invoked as
`support::muxsmith_localized(&["validate", path], "de")`, `.code(2)`, stdout
snapshotted - the file's documented insta + tempfile idiom, same shape as
`warnings_only_exits_one` and the en test beside it.

Its doc comment records why it does not ride the en funnel and, explicitly,
that **the snapshot is the load-bearing assertion, not the exit code** - so the
next reader cannot re-derive the trap that produced this round.

### `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap`

Created, insta-accepted, header matching the directory's convention (`source` +
`expression`, no `assertion_line`):

```
---
source: crates/muxsmith-cli/tests/cli_validate.rs
expression: "String::from_utf8(out).unwrap()"
---
[Fehler] tracks[0].match.exact.raw:: Das raw:-Präfix erfordert einen Eigenschaftsnamen: ein bloßes "raw:" benennt keine Eigenschaft, und die Regel könnte nie auf eine Spur zutreffen. Ergänze den Eigenschaftsnamen nach dem Doppelpunkt (zum Beispiel raw:dolby_complexity_index).
1 Fehler, 0 Warnungen, 0 Infos.
```

## F.2 The snapshot content check against D101's fence

Run **before** accepting the snapshot, on the pending `.snap.new`, then
re-run on the accepted file. Exact substring equality against the fence text
parsed out of the design document itself - not a hand-copied literal:

```
de snapshot contains D101's de fence text : True
en snapshot contains D101's en fence text : True
CONTROL de-fence-in-EN-snapshot (must be False): False
CONTROL en-fence-in-DE-snapshot (must be False): False
de snapshot non-empty body chars: 308
```

```
ACCEPTED de snapshot contains the fence verbatim: True
```

Four things this establishes, and the reason each line is there:

- The de fence renders (the assertion the whole round exists for).
- The cross controls fail, so the checks discriminate; a check that passes
  against both snapshots would be testing nothing.
- `308` body characters, so the empty-stdout failure mode that the mis-invoked
  form produced (section 4, `+new results` with an empty diff body) is
  positively excluded rather than assumed absent.
- The en snapshot from commit 1 still matches its fence after the funnel
  change - the delegation did not alter any existing caller's output.

## F.3 The Step-7 invariant check, with its fire

```
$ grep -rln 'cargo_bin("muxsmith")' --include='*.rs' crates
crates/muxsmith-cli/tests/support/mod.rs
```

Exactly one file: D64's file-level invariant holds through the helper edit
(two call sites now, both inside it - the pinned path and `muxsmith_bare`'s
exception).

**Fire**, per the plan's own reachable-red-state note (a `cargo_bin` call
written into `cli_validate.rs` instead of riding the helper adds a second
file). Done rather than argued:

```
$ printf '\n// probe\nfn _probe() { let _ = assert_cmd::Command::cargo_bin("muxsmith"); }\n' >> crates/muxsmith-cli/tests/cli_validate.rs
$ grep -rln 'cargo_bin("muxsmith")' --include='*.rs' crates
crates/muxsmith-cli/tests/cli_validate.rs
crates/muxsmith-cli/tests/support/mod.rs
```

Probe removed; the grep returns the single file again (F.4).

## F.4 Probe mutations and their restoration

Two probes in this round (the `dead_code` control and the invariant fire), both
appended as exact tails and removed by exact-tail assertion in Python, which
fails loudly if the text is not what I wrote:

```
probe removed          # support/mod.rs  (dead_code control)
probe removed          # cli_validate.rs (invariant fire)

$ grep -rln 'cargo_bin("muxsmith")' --include='*.rs' crates
crates/muxsmith-cli/tests/support/mod.rs

$ git status --porcelain          # before staging
 M crates/muxsmith-cli/tests/cli_validate.rs
 M crates/muxsmith-cli/tests/support/mod.rs
?? crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap

$ git diff --stat
 crates/muxsmith-cli/tests/cli_validate.rs | 29 +++++++++++++++++++++++++
 crates/muxsmith-cli/tests/support/mod.rs  | 35 ++++++++++++++++++++++---------
 2 files changed, 54 insertions(+), 10 deletions(-)
```

Exactly the three intended paths, no residue. **Disclosure:** one `cp` was used
in this round, to back up `support/mod.rs` before the dead_code probe. It did
not hit the interactive-overwrite hazard the brief warns about (the destination
was a fresh scratchpad path, so no overwrite prompt) and it completed; the
restoration itself was done by exact-tail removal, not from that copy, and the
copy was deleted. Recording it because the brief names `cp` specifically.

## F.5 Verification bar

| Check | Result |
|---|---|
| `cargo fmt --all --check` | `FMT_OK` (exit 0) |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished` - zero warnings |
| `cargo test --workspace` | exit 0; 39 test binaries, **every one** `test result: ok`; grep for any result line without `0 failed` returns nothing; 0 pending snapshots |
| `pnpm check:i18n` | `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| `pnpm lint` | `$ eslint .` - exit 0 |
| `pnpm test:e2e` | `63 passed (2.8s)` |
| Step-7 invariant grep | one file, fired (F.3) |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | `Finished`, zero warnings (beyond the bar; the two new rustdocs carry intra-doc links) |
| Typography sweep on both changed source files | zero hits; control on a real em-dash returns `1` |

**Every existing CLI subprocess suite green and unchanged** - the delegation's
whole risk surface, so measured per suite rather than inferred from the
aggregate:

```
cli_validate:          test result: ok. 7 passed; 0 failed; ...
cli_schema:            test result: ok. 3 passed; 0 failed; ...
dry_run_cli:           test result: ok. 11 passed; 0 failed; ...
run_cli:               test result: ok. 11 passed; 0 failed; ...
run_live:              test result: ok. 4 passed; 0 failed; ...
catalog_completeness:  test result: ok. 4 passed; 0 failed; ...
validate_semantics:    test result: ok. 24 passed; 0 failed; ...
```

`cli_validate` is 7 (was 5, plus this task's two). Every other count is
identical to the pre-task tree, and no snapshot outside this task's two
changed - `cargo insta` reported nothing pending at any point after acceptance.

## F.6 Disputes and findings

**Nothing disputed.** The ruling and the amended Step 4 are implementable
exactly as written; nothing in Step 4, the Files list, Step 7 or the must-not
list required me to invent a name, a string, or a placement decision with
outward effect. The one thing the amendment leaves open - where in the file the
new helper sits - has zero outward effect in Rust and is covered by the
structural grant; I recorded my reason above rather than treating it as a gap.

Two notes, neither a finding against the amendment:

1. **The coordinator's correction to the invariant's level was right and is
   load-bearing.** The pre-amendment rustdoc said `cargo_bin("muxsmith")`
   "appears nowhere outside this **function**", which the delegation would have
   falsified on its own (the funnel no longer contains the call). The FILE-level
   statement the plan pins is both true and the one D64's own text
   ("`cargo_bin("muxsmith")` appears in exactly one file", plan-7 design
   `:1602-1603`) always carried. Had the amendment kept the function-level
   wording, this round would have shipped a false doc.
2. **Section 6 item 1 of the original report still stands and is now larger.**
   D64's "The funnel covers all **11 insta snapshots**" and "`cli_validate.rs`
   (1 constructor, 3 snapshots)" are now 13 and 5. Recounted:
   `ls crates/*/tests/snapshots/*.snap | wc -l` -> `13`; `cli_validate__*` -> `5`.
   Still not repaired: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
   is on no Plan-9 task's Files list. A reader of that section should also note
   that its "1 constructor" figure for `cli_validate.rs` was already about the
   pre-D64 per-file helpers and is unaffected by this round.

## F.7 Commit

`3412fcc` on `master`, main worktree, unsigned (`%G?` -> `N`), exactly one
trailer, no `Claude-Session` line, not pushed. Task 4's second commit; Tasks 1
and 3 set that precedent.

```
$ git show --stat HEAD
commit 3412fcc3b1b13abcc05b7c30bec4d273f06b1f2a
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 20:50:53 2026 +0200

    cli: the German validate subprocess test rides a locale-parameterized pinned helper (Task 4 fix round, amendment 4)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/tests/cli_validate.rs          | 29 ++++++++++++++++++
 ...w_property_renders_german_with_locale_flag.snap |  6 ++++
 crates/muxsmith-cli/tests/support/mod.rs           | 35 +++++++++++++++-------
 3 files changed, 60 insertions(+), 10 deletions(-)
```

Working tree clean afterwards. Task 4's two commits together are `d768657` +
`3412fcc`; between them they cover the plan's full Step-8 `git add` line,
including `support/mod.rs` and both snapshot files.
