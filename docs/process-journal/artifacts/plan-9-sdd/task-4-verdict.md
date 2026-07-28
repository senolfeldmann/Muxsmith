# Task 4 review verdict - Plan 9 (`EmptyRawProperty`, D101; spec S-1 new row, S-3, S-5, S-6; amendment 4)

**Verdict: APPROVED_WITH_MINORS.**

Graded against the current tree (`cfe0515`, clean, master, main worktree) across
both task commits, `d768657` and `3412fcc`. Every character-for-character fence
holds, the German test's load-bearing assertion is real (fired twice, in both
failure directions), the delegation changed no existing suite, and the Step-7
invariant check fires and passes. No code change is required.

The minors are three report/evidence defects, one incomplete sweep whose action
falls to the controller, and two low doc/bookkeeping items. None of them affects
the shipped behavior.

---

## 1. Findings

### MINOR-1 - the count-word sweep found one stale consumer; there are three

**Where:** report §3.4 / §6 item 1 / F.6 note 2 claim one stale consumer
(`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`). Two more exist.

| Site | Text | Now |
|---|---|---|
| `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1505` | "the 11 insta snapshots stay en-pinned" | 13 |
| `.../2026-07-21-plan7-help-i18n-design.md:1556` | "`cli_validate.rs` (1 constructor, 3 snapshots)" | 5 snapshots |
| `.../2026-07-21-plan7-help-i18n-design.md:1563` | "covers all **11 insta snapshots**" | 13 (and see MINOR-2) |
| **`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:80`** | "`cli_validate.rs` (1 local `fn muxsmith()` helper, **5 helper call sites, 3 snapshots**) ... covers all 11 insta snapshots (**3+3+4+1+0 = 11**)" | 6 funnel call sites + 1 localized; 5 snapshots; 5+3+4+1+0 = 13 | 
| **`docs/ROADMAP.md:1085`** | "`ls crates/*/tests/snapshots/*.snap \| wc -l` -> **12**, `cli_validate__*` -> **4**" | 13 / 5 - the controller's own recount went stale when the fix round added the 13th |

**Evidence I ran** (own instruments, `t4rev-independent/sweep-snapshots-2.txt`):

```
$ ls crates/*/tests/snapshots/*.snap | wc -l      -> 13
$ ls crates/muxsmith-cli/tests/snapshots/cli_validate__*.snap | wc -l -> 5
per-file: cli_validate 5, dry_run_cli 3, run_cli 4, run_live 1, cli_schema 0
$ grep -c "support::muxsmith(" crates/muxsmith-cli/tests/cli_validate.rs -> 6 (pre-state: 5)
```

The sweep was fire-verified against the known-present D64 line before being
trusted; see the instrument note in §4 - my first pattern silently returned zero
on that very line.

**Required change:** none by any Plan-9 task (all three files are outside every
task's Files list, and the plan's "nothing else under `docs/` is touched by any
task" binds). The `docs/ROADMAP.md` "Docs accuracy" item is the registered
vehicle; it must be **extended** to name the plan-7 PLAN file at `:80` and to
carry 13/5 rather than 12/4.

**Class:** `proc-normative-count-recomputed`, trigger 2 (adding a member to an
enumerated set), one hop further than the implementer swept. The implementer's
action - surface, do not repair - was correct; only the enumeration was short.

### MINOR-2 - D64's sentence is falsified in KIND, not only in count

`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1563` says "The
funnel covers all **11 insta snapshots**". After amendment 4 the German snapshot
(`cli_validate__bare_raw_property_renders_german_with_locale_flag.snap`) does
**not** ride the `muxsmith` funnel - it rides `muxsmith_localized`. The
ROADMAP's stated plan-close vehicle ("either requalify the sentence as a dated
measurement or recount it") repairs neither reading: recounting to 13 still
asserts a false coverage claim, and dating it leaves the claim inside a block
that D64 uses normatively.

**Required change (plan close, one line, not a task):** the sentence must be
restated in coverage terms, e.g. *"every CLI-invoking snapshot test rides a
pinned helper - the en funnel or its locale-parameterized construction site -
and every locale-sensitive stdout/stderr assertion is pinned."* Recounting alone
closes the wrong half.

### MINOR-3 - the rustdoc-link evidence in the report does not hold

Report F.5: "`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` ->
`Finished`, zero warnings (beyond the bar; **the two new rustdocs carry
intra-doc links**)". `cargo doc` does not document integration-test targets, so
that run validated none of the new links in
`crates/muxsmith-cli/tests/support/mod.rs` (`:83`, `:90` carrying two, `:99`).
The file already carried unchecked intra-doc links before this task (`:33`,
`:44`, `:45`), so the gap is older than amendment 4 - amendment 4 only widened
it.

**Fire I ran** (the check's passing result is an absence, so I made it produce
output):

```
# injected into support/mod.rs: /// ... See [`t4rev_no_such_item_at_all`].
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
   Documenting muxsmith-cli v0.1.0 (...)
    Finished `dev` profile ...          # zero warnings, exit 0
# restored with git checkout --; sha256sum -c baseline -> 4x OK; git status empty
```

**The links themselves are sound** - `muxsmith`, `muxsmith_localized` and
`muxsmith_bare` are all `pub fn` in that same module, so they resolve if ever
rendered. No code change. The defect is the evidence line, and it matters
because the same sentence will be reused: the repo has **no** check that can go
red on a broken intra-doc link inside a `tests/` module, and the pending
"rustdoc private-items flag" ROADMAP item will not add one (private items are
orthogonal to test targets).

### MINOR-4 - the placement rationale does not survive measurement

Report §1: the scenario's position "maximizes distance from where Task 5's
parse-failure apply scenario will most plausibly land."

Measured - the four tests inside the `batch view: dry run` describe
(`e2e/smoke.spec.ts:140`):

```
:199  dry-run document renders the resolution table ...
:307  diagnostics summary and suggestions-capped pluralize ...
:369  an error-severity config diagnostic disables Run ...   <- Task 4
:460  a suggestion card's apply button drives load_profile ... <- Task 5 lands beside this
```

Task 4's scenario sits **immediately before** Task 5's anchor, i.e. adjacent,
not maximally distant; the top of the describe (after `:199`) would have been
~260 lines further away. The **placement is sound** (adjudication 5); the stated
reason is a spatial claim that was never measured.

### LOW-5 - `crates/muxsmith-core/src/profile/validate.rs:409-411`: a residual general sentence

The doc's closing sentence still reads "`path` keeps the literal `raw:`-prefixed
key; the `property` param carries the stripped bare name." That is now false for
one of three branches. The inserted sentence states the exception explicitly
three sentences earlier ("the diagnostic carries no `property` param"), and the
final sentence reads naturally as attaching to the two branches just described,
so no reader is misled.

**Optional one-liner** if the controller wants it airtight: `... the `property`
param carries the stripped bare name **on the two non-empty branches**.` Not
required; recorded because this is a weaker second instance of exactly the class
amendment 3 exists for.

### LOW-6 - `crates/muxsmith-cli/tests/support/mod.rs:102-103`: one sentence beyond amendment 4's content list, undisclosed

Amendment 4 pins the helper's rustdoc to: single construction site + the
append/subcommand mechanics + "`muxsmith` is its `"en"` delegation", and
explicitly **not** a restatement of D64's rationale or the invariant. The
committed doc satisfies all four, and adds: *"Call this directly only to pin a
non-en locale, which is a test asserting that locale's rendered output."*

That sentence restates neither the rationale nor the invariant, so it does not
cross the amendment's negative constraint; it is correct, useful, and conforms
to the file's pattern (every helper carries usage guidance). **In scope.**
Flagged only because it is the one keyboard resolution in the fix round that was
not named in F.1's divergence list, and the brief flags the list as closed.

### LOW-7 (controller) - `.superpowers/sdd/plan-9/progress.md:12` is stale

Row 4 still reads `NEEDS_CONTEXT, partially committed | d768657 (10 of 11 files;
the German subprocess test and its snapshot are absent)`. The fix round landed.
It should read DONE with `d768657` + `3412fcc`, and the Files-list arithmetic is
now 10 bullets / 12 distinct paths.

### Nits (no action)

- Report §3.4 "**`use`-line deletions:** none in this diff". The diff does carry
  `-import { en } from "./i18n-en";`, replaced by the widened form. No symbol was
  dropped, so the doc-link sweep duty
  (`an-import-removal-sweeps-the-doc-links-that-named-the-symbol`) is genuinely
  not triggered - the conclusion is right, the wording loose.
- "39 test binaries" (§2, F.5) is 39 `test result:` lines = **35 test binaries +
  4 doc-test targets**. Recomputed from my own run; substance (all green, 0
  failed) holds. The same figure has propagated through earlier plan-9 artifacts.
- F.2's "de snapshot non-empty body chars: 308" recomputes to **309** including
  the trailing newline (308 stripped). Immaterial.
- F.1's `dead_code` control pastes three "could not compile" lines; my run of the
  same control produced four (`cli_validate`, `run_live`, `dry_run_cli`,
  `run_cli`). The conclusion is unaffected.

---

## 2. What I verified green (dimension by dimension)

### D1 - contract compliance, both commits

Every fence machine-compared against the design document itself, never eyeballed
or hand-copied:

| Contract | Result |
|---|---|
| `raw_opt_in_diagnostic` three-branch form (D101 fence, design `:944-952`) vs `validate.rs:412-420` | **byte-identical** (`IDENTICAL: True`) |
| `EmptyRawProperty` variant doc (design `:911-917`) vs `report/mod.rs:88` | **identical after resolving the design's own escapes** (see adjudication 1) |
| Variant position: next to `RawProperty`/`RawOnKnownProperty` | yes, directly after `RawOnKnownProperty`, `report/mod.rs:89` |
| `locales/en/diagnostics.ftl` line (design `:928`) | **byte-identical**, positioned after `raw-on-known-property` |
| `locales/de/diagnostics.ftl` line (design `:934`) | **byte-identical**, same position, real orthography (`ä`, `ö`, `ß`) confirmed at codepoint level |
| `catalog_completeness.rs` fixture row `DiagCode::EmptyRawProperty => vec![]` | present, after the `RawOnKnownProperty` row |
| Two pinned core tests, per arm, exactly-one/error/path | present, `validate_semantics.rs:290`, `:306` |
| B-2/B-3 as the control, no duplicate written | confirmed - both pass in the same run; no new control function |
| Two pinned subprocess tests | present, `cli_validate.rs:65`, `:94` |
| Both snapshots, en and de fence text | **both contain their fence verbatim; both cross-controls False** |
| Run-gate scenario vs D101's enumeration | every element matches (see D-scenario table below) |
| Spec S-1 new row, S-3, S-5, S-6 | **all four byte-identical to their design fences, each present exactly once**; S-1's `WorkerPanicked` row untouched (Task 3's) |

Run-gate scenario against D101's producer paragraph, item by item: batch-view
describe (`smoke.spec.ts:140`) yes; `detect_mkvmerge -> MKVMERGE_INFO` yes;
`plugin:dialog|open -> PROFILE_PATH` yes; `validate_profile` resolving a
document with `mkvmerge_found: true`, empty `files`/`batch_diagnostics`/
`suggestions`, and `config_diagnostics` = exactly one diagnostic with
`code: "empty-raw-property"`, `severity: "error"`, `config_path:
"tracks[0].match.exact.raw:"`, `params: {}`, `rendered: "empty-raw-property"`
yes (`:353-367`); flow = pick the profile, no dry-run click yes; assertions =
`batch-run` disabled **and** `title` equal to the localized
`batch-run.tooltip-errors` yes (`:390-391`).

I re-verified the discrimination claim rather than borrowing it:
`BatchView.vue:293-307` orders `runDisabledReason` as run-active -> no-profile ->
mkvmerge-missing -> errors, so `mkvmerge_found: true` plus a completed pick
closes the earlier three by construction, and the `title` assertion is what
proves the errors branch fired. `enAttr` (`e2e/i18n-en.ts:170-176`) throws if the
id or attribute is missing, so the assertion cannot pass vacuously; it is the
house pattern (`help-mode.spec.ts:147`, `editor-tooltips.spec.ts:33`).

**Two design boundary claims I verified empirically at the binary**, because the
spec row makes them user-facing and only two of the three maps are test-pinned:

```
$ ./target/debug/muxsmith validate <regex: { 'raw:': 'en' }> --json --locale en
{"diagnostics":[{"code":"empty-raw-property","config_path":"tracks[0].match.regex.raw:",...,"severity":"error"}]}   EXIT=2
$ ./target/debug/muxsmith validate <exact: { 'raw: ': eng }> --json --locale en
{"diagnostics":[{"code":"raw-property","config_path":"tracks[0].match.exact.raw: ","params":{"property":" "},...,"severity":"info"}]}   EXIT=0
```

So S-1's "in an `exact`, `substring` or `regex` map" is true (the `substring` and
`regex` maps share one loop, `validate.rs:306-316`, so the two pinned tests do
cover all three maps), and D101's "the boundary is exact emptiness" holds - a
whitespace bare name stays `RawProperty` info.

### D2 - the German test's load-bearing assertion (the one that nearly shipped a lie)

Content check with **my own** extraction from the design file:

```
de fence text in de snapshot: True
en fence text in en snapshot: True
CONTROL de fence in en snapshot (must be False): False
CONTROL en fence in de snapshot (must be False): False
de snapshot body: 309 chars, 2 lines
```

Then two independent fires, because an exit code proves nothing here:

**Fire A - break the German rendering.** Mutated the `empty-raw-property` line in
`locales/de/diagnostics.ftl` ("Das raw:-Präfix" -> "MUTIERT raw:-Praefix"),
`cargo test -p muxsmith-cli --test cli_validate`:

```
test bare_raw_property_renders_german_with_locale_flag ... FAILED
    1       │-[Fehler] ... Das raw:-Präfix erfordert einen Eigenschaftsnamen: ...
          1 │+[Fehler] ... MUTIERT raw:-Praefix erfordert einen Eigenschaftsnamen: ...
test result: FAILED. 6 passed; 1 failed
```

**Fire B - reproduce the exact trap that caused amendment 4.** Replaced the
invocation with the originally pinned impossible form,
`support::muxsmith(&["validate", path, "--locale", "de"])`:

```
test bare_raw_property_renders_german_with_locale_flag ... FAILED
-old snapshot
+new results
    1       │-[Fehler] tracks[0].match.exact.raw:: Das raw:-Präfix ...
    2       │-1 Fehler, 0 Warnungen, 0 Infos.
                                       (nothing on the + side: stdout empty)
```

Fire B is the decisive one: the `.code(2)` assertion **passes** (execution
reaches the snapshot comparison), and the snapshot is what goes red, on an empty
stdout. The committed test therefore catches precisely the failure mode the
amendment was written for. Both mutations were baseline-hashed first, restored
with `git checkout --`, and the restoration proven (`sha256sum -c` 4/4 OK,
`git status --porcelain` empty). Every stray `.snap.new` was deleted; zero
pending snapshots at the end.

### D3 - the delegation's blast radius

- `crates/muxsmith-cli/tests/support/mod.rs` diff vs the pre-state
  (`8b315e6:...`) is **exactly one hunk**: the funnel's doc + body and the new
  helper. Nothing else in the file.
- **`muxsmith_bare` plus its closed two-caller exception doc: byte-identical.**
  Block extracted from both revisions (doc + `#[allow(dead_code)]` + fn):
  `sha256 e5d119306aed45f3c3f79e8929e3fa58022f8b5336be788e04276879eb0d8425` on
  both sides.
- **argv is structurally unchanged**: the pre-state body was `cargo_bin`,
  `args(args)`, `args(["--locale", "en"])`; the new path is the same three calls
  with `locale` bound to `"en"` through one delegation.
- **No existing suite changed.** `#[test]` counts, pre-state `760c00a` vs now:
  `cli_schema` 3/3, `dry_run_cli` 11/11, `run_cli` 11/11, `run_live` 4/4,
  `catalog_completeness` 4/4; only `cli_validate` 5->7 and `validate_semantics`
  22->24 moved, which is exactly this task's four new tests. All green in my own
  `cargo test --workspace` (exit 0, 39 result lines, zero with a non-zero
  `failed`).
- The en snapshot from commit 1 still matches its fence after the funnel change.
- The funnel doc's **new** claim is also true: "every integration test that
  asserts English muxsmith output invokes the binary through here" -
  `cli_schema.rs`'s two bare callers assert `.failure()` and parsed JSON
  respectively, neither an English text assertion.

### D4 - the two rustdocs against what amendment 4 pins

Sentence-level comparison against the pre-state:

```
'builds its `Command` here'                pre=True  now=False   (falsified statement, removed)
'The funnel appends `--locale en`'         pre=True  now=False   (moved to the helper)
'appears nowhere outside this function'    pre=True  now=False   (function-level wording, removed)
'Post-sweep invariant, at FILE level'      pre=False now=True    (the pinned replacement)
D64 contract sentence ("Pinning rides the CLI's own contractual surface,
  never environment variables: `sys_locale` reads OS APIs, not env vars,
  on Windows and macOS (D64's rejected alternative).")   pre=True  now=True
```

Helper doc against amendment 4's closed list: single construction site yes;
locale appended after the caller's args so it follows the subcommand yes;
`muxsmith` named as its `"en"` delegation yes; D64 rationale and invariant **not**
restated, pointed at instead yes. One extra sentence - LOW-6.

The `validate.rs` rustdoc edit is **exactly one inserted sentence**, verified
mechanically: normalizing whitespace and deleting the added sentence reproduces
the pre-state doc byte-for-byte (`removing it reproduces PRE exactly: True`).

### D5 - Step 7's invariant check

```
GREEN  $ grep -rln 'cargo_bin("muxsmith")' --include='*.rs' crates
       crates/muxsmith-cli/tests/support/mod.rs
FIRE   (appended a cargo_bin call to cli_validate.rs)
       crates/muxsmith-cli/tests/cli_validate.rs
       crates/muxsmith-cli/tests/support/mod.rs
RESTORE  git checkout -- ...; grep -> the single file again; sha256sum -c 4/4 OK; git status empty
```

Real fire, reachable red, green on the end state. Confirmed by my own run, not
borrowed from F.3.

### D6 - latitude, both forms

Keyboard resolutions, all in scope: D-1 (adjudication 1), D-2 (adjudication 2),
D-3 (the `enAttr` import, see the harvest note), the fix round's helper
placement, the removed `#[allow(dead_code)]` (adjudication 3), the additive
`!cs.contains(RawProperty)` / `!cs.contains(UnknownProperty)` lines in the two
core tests (additive verification along the file's own pattern, never
weakening), and LOW-6's extra doc sentence.

**The inverse direction - was anything routed that should have been built?**
No. Under `tests-ship-with-the-feature-never-after`'s execution-time precedence
the implementer must BUILD only when all four conditions hold, and the German
test fails the first: every route to it touches an **existing helper** (option B
changes `muxsmith`'s body; option A reopens D64's closed exception set, which the
plan's "no design decision is re-opened" forbids outright). No additive route
existed, so the fork correctly returned. The memo is the strongest artifact in
this task: it ran the impossible form, pasted the empty-snapshot failure, and
named the trap that a green `.code(2)` would have hidden.

### D7 - house dimension

- Tier-2 conformance: no house YAML touched by either commit (verified by
  `--name-only` on both); commits unsigned (`%G?` -> `N`), exactly one
  `Co-Authored-By` trailer each, no `Claude-Session` line
  (`agent-commit-trailer-set`, SI-4).
- `latitude-carveout-zero-content-structural-forks` (file-vs-within-file): see
  adjudication 2 and harvest (b).
- `a-null-assertion-over-a-dynamic-map-proves-nothing-without-a-presence-check`:
  **satisfied.** Both core tests assert presence positively first
  (`empties.len() == 1` over a filtered vector, then `severity` and `config_path`
  on the found element); the two `!cs.contains(...)` lines are absence assertions
  over a vector proven non-empty in the same test. No dynamic-map null assertion
  anywhere in this diff.
- `proc-normative-count-recomputed` incl. the callers'-docs facet: MINOR-1/2.
  The facet itself was honoured inside the diff - `muxsmith`'s rustdoc is the
  caller doc restating `muxsmith_localized`'s contract, and it was updated in the
  same edit.
- `tests-ship-with-the-feature-never-after` execution-time precedence: applied
  correctly (see D6). Nothing this package's diff creates is uncovered.
- `a-mutate-restore-step-restores-non-interactively-and-proves-it`: the report
  discloses one `cp` in the fix round (fresh scratchpad destination, restoration
  done by exact-tail removal, copy deleted). Correct disclosure; no hazard hit.

### D8 - the no-work-needed check (every premise re-run)

| Report passage concluding "unnecessary / already covered" | My check |
|---|---|
| §5: `dry-run` exit 2 covered by `dry_run_cli.rs::dry_run_surfaces_config_time_invalid_regex` at `:104` | **verified** - test at `:81`, `out.status.code()` assertion at `:104-105` |
| §5: `run` exit 2 covered by `run_cli.rs::bad_regex_profile_exits_two_without_executing_a_job` at `:69` + `asserts_no_job_ran` | **verified** - `:51` / `:69-70` / `:80` |
| §5: Save gate already asserted, `smoke.spec.ts:1251` | **verified** - test at `:1251`, green in my own e2e run |
| §5: only the Run gate lacked an assertion | **verified** - `batch-run` + `toBeDisabled` appears nowhere else in the suite |
| §3.4: catalog guard covers the new key/row | **verified** - `every_diag_code_renders_without_leftover_placeholders` iterates `DiagCode::ALL` with `fixture_args` and rejects any `{$` (`catalog_completeness.rs:163-175`); `all_keys_match_serde_encoding` (`report/mod.rs:386`) iterates `DiagCode::ALL` |
| F.1: the `dead_code` lint does not fire | **verified with my own control** - adjudication 3 |
| §3.4: no `use`-line deletion, no doc-link sweep owed | conclusion right, wording loose - see nits |
| §3.4: no live count exists elsewhere | **partly wrong** - MINOR-1 |
| §2/F.5: no pending snapshots | **verified** - `crates/*/tests/snapshots/*.new` has no matches |
| D-3: no collision with Task 6's `FluentVariable` removal | **verified** - `FluentVariable` is imported on `smoke.spec.ts:25`, a different line from the `./i18n-en` import at `:24` |

### D9 - verification quality: every aggregate recomputed, the Step-7 bar re-run

| Check | My own result |
|---|---|
| `cargo fmt --all --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished`, zero warnings |
| `cargo test --workspace` | **exit 0**; 39 `test result:` lines (**35 test binaries + 4 doc-test targets**); zero lines without ` 0 failed` |
| CLI suite counts | cli_validate **7**, cli_schema **3**, dry_run_cli **11**, run_cli **11**, run_live **4**, catalog_completeness **4**, validate_semantics **24** - all seven match the report exactly |
| `pnpm check:i18n` | `ok (41 source files scanned, **212 catalog ids**, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s))` - matches |
| `pnpm lint` | exit 0 |
| `pnpm test:e2e` | **63 passed**; the D101 scenario green at `smoke.spec.ts:369` |
| Step-7 invariant grep | one file; fired; restored (D5) |
| Typography, exact codepoints over all 16 files in `760c00a..3412fcc` | **0 AI-tell glyphs**, control fired on a synthetic string carrying em-dash / smart quotes / ellipsis / NBSP |
| Step-8 `git add` coverage | the two commits' path union == the plan's 12-path line, **exactly**; no extras, no misses; `cli_validate.rs` is the only overlap |

---

## 3. The five adjudications

### 1. `get(\"\")` -> `get("")` in the variant doc: **correct transcription, not a deviation**

Mechanically settled. Unwrapping the design's `:911-917` block, taking the text
inside `doc comment: "..."`, and resolving `\"` reproduces the committed doc
comment **exactly**; keeping the escapes literal does not:

```
IDENTICAL (escapes resolved): True
IDENTICAL (escapes literal):  False
```

The backslashes are an artifact of the design's own delimiting quotes, not
content. A literal `\"` in a Rust doc comment would render `get(\"\")` with
visible backslashes in rustdoc - which is not what the design describes, and
would itself be the deviation. The implementer transcribed the content and
flagged it; both halves right.

### 2. The added sentence in `raw_opt_in_diagnostic`'s rustdoc: **in scope, correctly applied and correctly surfaced**

Four independent conditions, all met:

- **Listed file.** `crates/muxsmith-core/src/profile/validate.rs` is Files-list
  entry 2.
- **No within-file qualifier.** Its parenthetical reads "(`raw_opt_in_diagnostic`
  three-branch form verbatim)" - a work description, carrying no "only", no line
  span, no named region. Under the owner's 2026-07-28 file-vs-within-file ruling
  the grant fills that silence, and amendment 3 recorded exactly this reading for
  an entry of the same shape ("the entry carries no 'only', span or region
  qualifier, so it never constrained within-file work").
- **The task's own edit invalidated the referent.** The doc enumerated two
  outcomes; the enumerated body change made that false. "Repairing a reference
  which the task's OWN enumerated edit invalidated ... inside a LISTED file" is
  named in scope by the grant.
- **All four zero-outward-effect conditions hold.** No API/symbol surface, no
  data format, verification untouched, nothing user-visible.

And the edit is minimal in fact, not just in claim: removing the one added
sentence reproduces the pre-state doc byte-for-byte. This is the correct
resolution of exactly the class amendment 3 exists because it was *not* done.
Surfacing it as D-2 was also right - the amendment-3 precedent makes the
distinction (design-pinned doc text vs implementer-owned doc prose) worth a
reviewer's explicit ruling rather than silence.

### 3. Removing the drafted `#[allow(dead_code)]`: **right call**

Verified in both directions with my own instruments, not from the report:

```
$ cargo clippy --workspace --all-targets -- -D warnings        # attribute absent
    Finished `dev` profile ...                                  # zero warnings

# control - is dead_code live in this module at all?
(appended `pub fn t4rev_probe_unused() -> u8 { 0 }` to support/mod.rs)
$ cargo clippy -p muxsmith-cli --all-targets -- -D warnings
error: function `t4rev_probe_unused` is never used
error: could not compile `muxsmith-cli` (test "cli_validate") due to 1 previous error
error: could not compile `muxsmith-cli` (test "run_live") ...
error: could not compile `muxsmith-cli` (test "dry_run_cli") ...
error: could not compile `muxsmith-cli` (test "run_cli") ...
(restored; sha256sum -c 4/4 OK; git status empty)
```

The lint is live in that module and does not fire on `muxsmith_localized`,
because `muxsmith` calls it and every consuming binary uses `muxsmith` (which is
itself the one helper in the file carrying no `#[allow(dead_code)]` - the
file's own evidence for that). The attribute would have been dead weight **and**
a false signal, since it asserts "some binary does not reach this", which is
untrue. Measuring rather than reasoning from the file's four sibling instances is
the right instinct and the right outcome.

### 4. Committing the verified subset while returning NEEDS_CONTEXT: **acceptable here; worth a standing rule in a narrow form**

Acceptable, for reasons that are checkable rather than stylistic:

- The committed subset is **self-consistent and green on the full Step-7 bar** -
  it is not a half-applied change. I re-ran the bar on it transitively.
- It is what **every** option A-D would have kept; no ruling could have made the
  committed content wrong.
- It is local, unpushed, and amendable, and the plan is serial on master with no
  branches - the routing interval was a real wall-clock gap (34 minutes between
  the two commits, with **four** house/plan commits landing inside it:
  `ba69c36`, `42fa6ea`, `4e5daa6`, `8b315e6`).
- The alternative - ten modified files sitting dirty across an owner-routing
  interval on a single-index tree - is the strictly worse risk: the plan itself
  names it (`concurrent-writers-need-pathspec-scoped-commits`, "one tree means
  one index"), and a house or amendment commit landing meanwhile would have had
  to work around them.

**Yes, worth a standing rule**, and I would state it narrowly, because the
default must not become "commit whatever is done":

> A task returning NEEDS_CONTEXT MAY commit the subset that is (a) complete
> against its own Files-list entries, (b) green on the task's full verification
> bar, and (c) kept under every option its decision memo enumerates. It MUST
> stage explicitly by path, name the omitted Files-list entries and the reason in
> the report, and record the partial state in `progress.md`. Anything failing (a),
> (b) or (c) stays uncommitted.

Task 4 satisfies (a), (b), (c) and the disclosure duty; the only gap is the
`progress.md` half, which the controller did write but has not since updated
(LOW-7). Tasks 5-7 can hit this shape - Task 5 in particular, whose D102 hoist
and D103 fetch are separable.

### 5. The Run-gate scenario's placement: **sound; the rationale is false; the collision risk is nil, but Task 5's anchors are now stale**

**Sound.** It sits inside the one describe the plan and design name (`batch view:
dry run`, `smoke.spec.ts:140`), touches neither Task 3's fixture region nor
Task 5's apply block, and adds exactly one test plus its fixture const, matching
the describe's existing shape (`pluralReport` at `:270` is the same pattern).

**No collision with Task 5's region ownership.** Execution is strictly serial on
one tree with no branches and no merges, so there is no concurrent-writer
hazard; Task 5's anchor is the apply-flow test, a distinct and content-locatable
site; and ownership stays legible because the new scenario carries a D101 comment
naming its ruling.

**The rationale does not hold** - MINOR-4. Adjacent, not maximally distant.

**The real consequence, and it is actionable:** the insertion shifted every
anchor below it by **+54 lines**, so Task 5's plan citations are stale by
construction (`proc-57-briefs-not-ground-truth`, the same class the Task-2 report
handled correctly). Measured:

| Anchor | Plan says | Now |
|---|---|---|
| apply-flow test (Task 5's landing site) | `:406` | **`:460`** |
| `await expect(runButton).toBeEnabled()` | `:511` (locator `:510`) | **`:565`** (locator `:564`) |
| `batch view: dry run` describe opens | `:140` | `:140` (unchanged) |
| `BatchView.vue:225` `config_diagnostics[0]` | `:225` | `:225` - Task 4 did not touch the file |

---

## 4. Evidence appendix - instruments

All instruments are mine, written for this review, under
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t4rev-independent/`.
No instrument the implementer wrote was re-run, and no shared default path was
used. All runs foreground, absolute paths, no session-relocation tools.

| File | What it holds |
|---|---|
| `baseline.sha256` | pre-mutation hashes of the four files I mutated; `sha256sum -c` run after every restore |
| `fence-extract.txt` | Fluent en/de fence vs locale files, byte equality |
| `spec-amendment-check.txt` | S-1/S-3/S-5/S-6 presence + occurrence count in the spec, plus the fence-delimiter check |
| `snapshot-fence-check.txt` | de/en snapshot content vs the design fences, with both cross-controls |
| `cli_validate-green-baseline.txt` | the suite green before any mutation |
| `fire-1-de-text-mutation.txt` | fire A - broken German rendering turns the test red |
| `fire-2-impossible-invocation.txt` | fire B - the amendment-4 trap reproduced; `.code(2)` passes, snapshot goes red on empty stdout |
| `deadcode-control.txt` | the `dead_code` control firing in four test binaries |
| `invariant-green.txt` / `invariant-red.txt` | Step-7 grep, green state and fired state |
| `rustdoc-fire.txt` | the broken-intra-doc-link injection that `cargo doc` did **not** catch (MINOR-3) |
| `support-prestate.rs` / `support-now.rs` | the two revisions diffed and block-hashed for `muxsmith_bare` |
| `smoke-pre.ts` | pre-state `smoke.spec.ts` for the +54 anchor measurement |
| `workspace-test.txt`, `suite-counts.txt` | full `cargo test --workspace` output and the recomputed per-suite counts |
| `typography.txt` | exact-codepoint sweep over all 16 changed files, with control |
| `sweep-snapshots.txt` / `sweep-snapshots-2.txt` | the malformed first sweep and the fire-verified replacement |
| `e2e.txt`, `i18n.txt`, `lint.txt`, `clippy.txt` | the frontend and lint legs |

**Instrument-hygiene warning, reusable and worth the harvest slot.** `grep` on
this machine is **ugrep 7.5.0**, not GNU grep. Under `-E` it uses a
non-backtracking engine: a bounded repetition combined with word boundaries
silently matches nothing. Measured on a file that demonstrably contains the
target:

```
$ grep -cE 'snapshots'                              -> 5
$ grep -cE '[0-9]{1,3}[^.]{0,30}snapshots'          -> 4
$ grep -cE '\b[0-9]{1,3}\b[^.]{0,30}\bsnapshots\b'  -> 0     <- silently empty
```

My first count-word sweep returned zero on `11 insta snapshots` for exactly this
reason and would have shipped as "no stale counts found" had I not fired it
against a known-present case first. Use `grep -P` (pcre2 is available) or an
exact-codepoint script for any absence check in this repo. The same class hit my
shell typography sweep, which produced nonsense counts until I redid it in
Python.

**Second instrument trap, same family.** A typography checker whose pattern
carries the target glyphs *as literals* is unreliable: in one of my runs the
literal U+00A0 in the script degraded to a plain space, and the checker
cheerfully reported 5407 "nbsp" hits in its own verdict file. Both the
false-clean and the false-alarm directions are reachable. The sound form is
`chr(0x00A0)` and friends - codepoints, never literals - with a control string
built the same way. The final sweep in this review uses that form; its control
returns exactly one hit per glyph.

---

## 5. HARVEST

### What Tasks 5-7 must carry

1. **`e2e/smoke.spec.ts` anchors are +54.** Task 5's dispatch must not transcribe
   the plan's `:406` and `:511`/`:510`. Correct values: apply-flow test at
   `:460`, enabled assertion at `:565` (locator `:564`); the describe still opens
   at `:140`. Locate by content, per `proc-57-briefs-not-ground-truth` and the
   Task-2 precedent. `BatchView.vue:225` is unmoved.
2. **Task 5's smoke.spec.ts Files entry carries the same "nothing else in the
   file" qualifier.** Its apply scenario replays the `:460` scaffold and should
   need no new import; if it does, pre-decide it in the dispatch rather than
   leaving the implementer the boundary question Task 4 had to resolve (see 5
   below).
3. **`tests-ship-with-the-feature-never-after` precedence is now live and was
   exercised.** Tasks 5-7 should run its four conditions explicitly and say so in
   the report, the way Task 4's memo did. Task 4's return was correct precisely
   because condition 1 (additive; no existing helper touched) failed.
4. **The plan-close D64 one-liner needs two edits, not one** (MINOR-1, MINOR-2):
   the numbers are 13 total / 5 `cli_validate__*`, and the *coverage* sentence
   must be restated, since one snapshot no longer rides the en funnel. The
   ROADMAP item at `docs/ROADMAP.md:1080-1093` must also gain the plan-7 PLAN
   file (`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:80`, whose
   `3+3+4+1+0 = 11` arithmetic is now `5+3+4+1+0 = 13`) and have its own 12/4
   corrected to 13/5.
5. **Over-restriction-watch calibration datum** for
   `latitude-carveout-zero-content-structural-forks` - the entry explicitly asks
   reviewers to flag stops the boundary forced that it arguably should cover.
   Task 4's `smoke.spec.ts` entry reads "ONE addition ... **nothing else in the
   file**" - an explicit within-file qualifier, so the grant does not fill silence
   there. Yet the enumerated addition **cannot compile** without widening the
   existing `./i18n-en` import to `enAttr`, which the design's own assertion
   (title equals the localized text, derived not hand-duplicated) requires. The
   implementer applied it and disclosed it (D-3); I rule that correct - a
   qualifier written to fence *other tasks' regions* should not be read as
   fencing a symbol import the same addition needs. **Proposal:** extend the
   entry's named-in-scope list from "repairing a reference the task's own edit
   invalidated" to also cover "adding a symbol import that the task's own
   enumerated addition requires, in a listed file, where the addition is
   otherwise uncompilable" - explicitly surviving a "nothing else in the file"
   qualifier, since that qualifier's purpose is region ownership.
6. **No check in this repo can go red on a broken intra-doc link inside a
   `tests/` module** (MINOR-3, fire-verified). `cargo doc --workspace --no-deps`
   does not document integration-test targets, and the pending "rustdoc
   private-items flag" ROADMAP item will not change that. Either stop writing
   intra-doc links in `tests/` modules, or accept them as unchecked prose and
   stop citing `cargo doc` as their evidence. Ledger-worthy as a readable
   trigger: *you are about to cite a `cargo doc` run as evidence for a doc
   comment that lives in a target `cargo doc` does not build.*
7. **Instrument hygiene, two measured traps.** (a) `grep` here is **ugrep
   7.5.0**, and `\b` plus bounded repetition under `-E` silently returns zero.
   (b) A checker carrying its target glyphs as literals can degrade in
   transit - my own NBSP literal became a plain space in one run. Both directions
   of failure are reachable, both are silent, and both hit me in this single
   review. Handles: `grep -P` or a script for any absence check; codepoints
   (`chr(0x00A0)`) never literals in a glyph checker; and a control that fires
   before any empty result is believed. Candidate for the house files - it is
   partly machine-specific, but it invalidated three of my own instruments in one
   session and would silently invalidate any implementer's.
8. **`.superpowers/sdd/plan-9/progress.md:12`** must move to DONE with both
   commits before the plan close (LOW-7).

### Standing rule proposed

The NEEDS_CONTEXT partial-commit rule in adjudication 4, in the three-condition
form written there. Tasks 5-7 can hit the shape; Task 5 most plausibly.

### Recorded positives worth keeping

- The decision memo (report §4) is the model for this class: it **ran** the
  impossible invocation, pasted the empty-snapshot failure, and named why a
  green `.code(2)` would have proved nothing. The resulting ledger entry
  `a-plan-that-pins-a-test-invocation-runs-it-once-first`
  (`docs/decision-ledger.yaml:4686`) is the durable residue.
- The fix round measured the `dead_code` question in both directions instead of
  reasoning from the file's four sibling attributes, and removed its own draft.
- Both snapshot content checks carry cross-controls, and the `muxsmith_bare`
  byte-identity check carries a mutated-copy control - all three are the shape a
  check needs to mean anything.
