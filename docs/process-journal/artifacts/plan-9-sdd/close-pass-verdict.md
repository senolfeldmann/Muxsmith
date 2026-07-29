# Plan-9 close pass - independent review verdict

**Verdict: NEEDS_FIXES** (three MEDIUM, two MINOR, one LOW-pre-existing).

Nothing here is broken and nothing blocks a push technically: I re-ran the
eleven-part gate as this change now defines it, foreground, and recomputed
every aggregate from its own run rather than quoting the report. All four
baselines held. The gate change itself - the load-bearing half of the package -
is correct at both sites, and I proved the flag is load-bearing by breaking the
tree deliberately and watching it fire.

The grade is NEEDS_FIXES on the criterion the brief set, which is not "does it
compile" but "is the new sentence TRUE". Three of the seven items put a
statement into the tree that does not survive measurement: item 2's fenced
comment (self-disclosed, needs a ruling), item 4's consumer attribution (not
disclosed; the report's supporting measurement is itself wrong), and item 6's
own target, where BUILDING.md is now cited as the gate's definition while its
enumeration is one command short of the number every consumer derives from it.
All three fixes are prose, no behaviour, and cost one commit.

---

## 1. Findings

### F1 (MEDIUM) - the fenced LOW-4 comment states something false about the tree

**Where:** `crates/muxsmith-cli/tests/dry_run_cli.rs:443-446`, the clause "The
profile-load-failure shape carries neither key".

**Evidence I ran** (own instrument, own build):

```
$ ./target/debug/muxsmith dry-run /nonexistent-profile-rev.yaml --json
EXIT=2
keys: ['batch_diagnostics', 'config_diagnostics', 'files', 'suggestions']
files present: True   value: []
mkvmerge_found present: False
codes: ['parse-error']
```

Root confirmed at the emitter rather than only at the wire:
`config_only_document` (`crates/muxsmith-core/src/report/json.rs:95-114`) puts
`"files": []` into every document it builds, unconditionally at `:106`, and adds
`mkvmerge_found` only for `Some(found)`. The three call shapes are
`None` (profile-load failure), `Some(false)` (mkvmerge missing), `Some(true)`
(query failed) - `crates/muxsmith-cli/src/commands/dry_run.rs:40,50,68`. So the
comment's own next sentence, "the two config-only shapes that carry it
(mkvmerge missing, query failed)", is right; "carries neither key" is wrong on
`files`.

The Task-5 verdict contradicts itself here: its own LOW-4 measurement block,
about forty lines above the fenced text, prints `files present: True is array:
True value: []` for the same document.

**Exact required change** (one line, no logic):

```rust
    // Shape guards. `mkvmerge_found` absent rules out the two config-only
    // shapes that carry it (mkvmerge missing, query failed). The
    // profile-load-failure shape carries no `mkvmerge_found` and an empty
    // `files` array, so it satisfies both guards and is ruled out by the
    // code sequence below, where it would be a singleton `parse-error`.
```

The "singleton `parse-error`" half is true and stays: my run shows
`codes: ['parse-error']` against the four codes the sequence assertion expects.

### F2 (MEDIUM) - item 4's safety clause names three settings consumers; the tree has six

**Where:** `e2e/jobsview-reset.spec.ts:105-106`, "the settings pair belongs to
`main.ts`'s locale bootstrap and `SettingsDialog.vue`".

**Evidence I ran:** `git grep -n "getSettings\|setSettings" -- src/` returns
consumers in `src/main.ts:17`, `src/components/SettingsDialog.vue:30,64`,
`src/recentProfiles.ts:34,39`, `src/views/BatchView.vue:69,81,83`,
`src/views/EditorView.vue:135`, `src/views/FirstRun.vue:5`. The report's
supporting line - "`getSettings`/`setSettings` only in
`src/components/SettingsDialog.vue:4` (plus `main.ts`'s locale bootstrap)",
presented under "Safety verified at the consumers, not assumed" - is a false
measurement, and it is the one clause of the new sentence that a future reader
would consult before moving a settings call.

The other two attributions hold: `platform` is imported only at
`src/views/FirstRun.vue:3`, `writeTextFile` only at
`src/components/RunHistory.vue:17,125`.

**The conclusion survives, and I verified it independently** rather than
inheriting it: `JobsView.vue:29` imports `cancelJob`, `cancelRun`, `startRun`
and nothing else from `../ipc`; `RunHistory.vue:125`'s `writeTextFile` sits
behind a `save()` dialog in a user-triggered handler; the spec's only mount is
`{ component: "JobsView" }` at `e2e/jobsview-reset.spec.ts:148`, called from
lines 161, 188, 206, 245. And "every test in this file asserts DOM state rather
than a recorded call log" holds: the only `__muxsmithRecordInvoke__` occurrence
in the file is the new comment itself, with the pattern fired against
`e2e/mocks.ts` and `e2e/global.d.ts` as known-present controls.

The omission list itself is correct against `e2e/mocks.ts:84-133`; I diffed the
two installers rather than trusting the table.

**Exact required change:** replace the clause with one that does not enumerate
exclusively, e.g. "the settings pair is read across the app but by no component
this spec mounts (`main.ts`'s locale bootstrap, `SettingsDialog.vue`,
`BatchView.vue`, `EditorView.vue`, `FirstRun.vue`, `recentProfiles.ts`)".
This is the same shape as the house rule item 3 exists to enforce:
`core-docs-name-callers-illustratively-never-exclusively`
(`docs/decision-ledger.yaml:4709`).

### F3 (MEDIUM) - item 6 stopped short: BUILDING.md enumerates ten commands, and the ruling it implements says eleven

**Where:** `BUILDING.md:74-118`.

**Evidence I ran:** the file's three command blocks are the Rust gate
(`:77-82`, six lines), Frontend checks (`:105-107`, three lines) and the new
House-knowledge check (`:113`, one line). Ten. `pnpm build` is documented
at `:65` under "Building and running" and named in the CI paragraph at `:122`,
but is in no checks block. Every count outside the file counts it:
`HANDOFF.md:107`, the plan-9 plan doc at `:20`, plan-8.5 at `:20`
("'The Rust gate' six parts + the four frontend checks" - the frontend block
has three), and this pass's own brief.

The ROADMAP ruling item 6 implements (`docs/ROADMAP.md:1185-1188`) reads: "the
gate block gains `python3 scripts/ledger-lint.py` as an ELEVENTH part". After
this change there is no eleventh part anywhere in BUILDING.md, and the
controller's owed rewrite of `HANDOFF.md:107` to "ELEVEN parts per BUILDING.md"
would be false against the file it cites. That is the same defect class item 6
was created to remove: a definition that its consumers disagree with.

The implementer named this (concern 4) and correctly did not act on it under
the no-latitude rule. As reviewer I rule it owed, not optional.

**Exact required change:** add `pnpm build` to the `### Frontend checks` block
with a comment matching the neighbours, so the file enumerates eleven. Nothing
else in BUILDING.md needs a number: the "(six parts)" heading at `:74` stays
correct, because `--document-private-items` modifies part 4 rather than adding
one, and I fired the absence check for a gate total in both edited files
(`grep -nEi "ten part|ten-part|10 part|eleven" BUILDING.md .github/workflows/ci.yml`
-> exit 1, with `grep -nEi "six parts" BUILDING.md` -> `:74` as the
known-present control).

### F4 (MINOR) - the module doc three hundred lines above still carries the exclusive form

**Where:** `crates/muxsmith-core/src/identify.rs:4-5`: "The cache is
constructed per planning call and dropped with it, so separate calls
re-identify."

**Evidence I ran:** `grep -n "per planning call" crates/muxsmith-core/src/identify.rs`
-> `:5` (module doc, exclusive), `:305` (type doc, corrected by this pass),
`:392` (`LiveIdentifier.cache` field doc). The `:392` site is NOT a defect and
should stay: `LiveIdentifier` has exactly one production construction site,
`crates/muxsmith-core/src/pipeline.rs:126`, so "constructed per planning call"
is true of that field. The module doc is a statement about `IdentifyCache` in
general and is the same time bomb the type doc just had defused.

Adjudicated at Q3 below. **Exact required change:** "The cache is constructed
per call and dropped with it, so separate calls re-identify."

### F5 (MINOR) - item 7's annotation attributes a two-snapshot delta to one commit, and leaves the enumeration mixed-date

**Where:** `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1561`
and `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:80`, both reading
"3 snapshots - 5 since amendment 4 added the German case".

**Evidence I ran** (`git log --diff-filter=A` per snapshot file):

| snapshot | added by | date |
|---|---|---|
| `cli_validate__valid_profile_exits_zero_with_ok_message` | `aba7f4f` | 2026-07-12 |
| `cli_validate__invalid_profile_exits_two_and_renders_messages` | `aba7f4f` | 2026-07-12 |
| `cli_validate__warnings_only_exits_one` | `aba7f4f` | 2026-07-12 |
| `cli_validate__bare_raw_property_exits_two_and_renders_the_message` | `d768657` | 2026-07-28 20:16 |
| `cli_validate__bare_raw_property_renders_german_with_locale_flag` | `3412fcc` | 2026-07-28 20:50 (amendment 4) |

`3412fcc` added exactly one snapshot file; `d768657` (the D101 bare-raw work)
added the other. The literal reading of "5 since amendment 4 added the German
case" is true - it has been 5 since that moment - but the sentence is placed as
the explanation of a 3-to-5 delta and names one of two causes, so a reader who
does the arithmetic finds it does not close. The same applies, softer, at
`:1505`: "No new snapshot files, as designed here - amendment 4 later added
exactly one" corrects a claim that two commits falsified.

Second half: the plan-doc enumeration declares one measurement date
("measured 2026-07-21") and now carries one figure refreshed to HEAD beside
neighbours that are not. `cli_validate.rs`'s "5 helper call sites" in the same
parenthetical is 7 at HEAD (6 `support::muxsmith(` + 1
`support::muxsmith_localized(`, counted per file).

**Exact required change:** name both additions and mark the refresh as
scoped - e.g. "3 snapshots at this measurement; 5 at the Plan-9 close, the
bare-raw case (`d768657`) and the German case (amendment 4); the other figures
in this enumeration are the 2026-07-21 measurement, unrefreshed".

**Everything item 7 states as a number is correct** and I recomputed all of it:
13 snapshot files (`cli_validate` 5, `dry_run_cli` 3, `run_cli` 4, `run_live`
1, `cli_schema` 0), matched against `assert_snapshot` counts per test file
(5/3/4/1/0) so no orphan file and no inline-only snapshot exists; 5+3+4+1+0 =
13. The KIND claim is true and I verified it per test rather than per file: a
brace-balanced scan of all `crates/muxsmith-cli/tests/*.rs` functions
containing `assert_snapshot` returns 13 tests, 12 invoking `support::muxsmith`,
one (`bare_raw_property_renders_german_with_locale_flag`) invoking
`support::muxsmith_localized`, and zero touching `cargo_bin` directly. D64's
invariant holds: `cargo_bin("muxsmith")` appears in one file,
`crates/muxsmith-cli/tests/support/mod.rs`, at `:105` and `:125`.

### F6 (LOW, pre-existing, routed not blamed) - BUILDING.md's CI paragraph contradicts itself about part 5

**Where:** `BUILDING.md:120-125`: "CI runs Rust-gate parts 1-5 natively on all
three OS legs ... ; `cargo deny check` and `scripts/ledger-lint.py` ... run as
independent jobs." Part 5 IS `cargo deny check`.

**Evidence I ran** (`yaml.safe_load` over the workflow, steps per job):

```
test        -> [... cargo fmt --all --check, cargo clippy ..., cargo test --workspace,
                cargo doc --workspace --no-deps --document-private-items,
                cargo test --workspace -- --nocapture ..., pnpm lint, pnpm build,
                cargo test -p muxsmith-core --features ts, pnpm check:i18n,
                playwright install, pnpm test:e2e]
deny        -> []            (action-based job)
ledger-lint -> [venv + pip PyYAML + scripts/ledger-lint.py]
```

The matrix job runs parts 1-4, not 1-5. Untouched by `9dc3a4d`, so not this
pass's defect - but it sits three lines below the block item 6 edited, and the
report's both-sites check did not look at it. Correct wording: "parts 1-4".

---

## 2. What I verified sound

**The gate change, both sites, with its own fire.** Parsing the workflow finds
exactly one step whose `run` contains `cargo doc`; it carries
`--document-private-items` and `RUSTDOCFLAGS: "-D warnings"`. `BUILDING.md:80`
carries the identical invocation. I did not accept the flag's value on the
report's word - I restored the pre-fix text and measured:

```
# mutated tree (both links back to [`run`]), WITH the flag:
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
error: `run` is both a function and a module  --> src-tauri/src/lib.rs:54:21
error: `run` is both a function and a module  --> src-tauri/src/lib.rs:87:15
error: could not document `muxsmith-gui`
EXIT=101

# same mutated tree, WITHOUT the flag:
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
EXIT=0, zero ^(error|warning) lines
```

That is the proof the change buys something: the old gate was green on the
exact defect. Restore proven non-interactively - `git checkout -- src-tauri/src/lib.rs`,
`git status --porcelain` empty, `git hash-object src-tauri/src/lib.rs` back to
the baseline `76dcbd848cf0fc06af25c66d4d283b89b8a48a6e` taken before the
mutation, HEAD still `9dc3a4d`.

**Item 5's two link repairs point at the right items, verified in the rendered
output rather than by reasoning alone.** From `target/doc/muxsmith_gui_lib/`:
`struct.ShellRenderer.html` links `run` to `run/index.html` (the module - and
`start_run` is defined at `src-tauri/src/run.rs:400`, so the sentence means the
module); `struct.AppState.html` links `run()` to `fn.run.html` (the function -
and `.manage(AppState::default())` at `src-tauri/src/lib.rs:543` is inside
`pub fn run()` at `:537`, the sole `.manage(` in `src-tauri/`). Both choices
are right, independently and per site.

**Item 1's two trigger shapes are complete and its premise is true.** The fetch
is `doc.config_diagnostics.find((d) => d.code === "parse-error")`
(`src/views/BatchView.vue:225`), so the else arm has exactly two entries: empty
array, and non-empty with no `parse-error`. "Documented to always pair with a
lead `parse-error` diagnostic" is true of the code, not just of prose:
`load_profile_body`'s `Err(d)` arm (`src-tauri/src/lib.rs:294-297`) puts the
single diagnostic from `load::from_file`, and every failure path in
`crates/muxsmith-core/src/profile/load.rs` (`from_file` at `:56`, and the
`parse_error` helper it shares with `from_str`) builds `DiagCode::ParseError`,
which renders as `"parse-error"` (`crates/muxsmith-core/src/report/mod.rs:72`).
Old string has no consumer: `git grep -n "with no diagnostics" -- src/ e2e/
crates/ src-tauri/` exit 1, control `git grep -c "with no parse-error diagnostic"
-- src/ e2e/` returns the one changed line.

**Item 3's clause is complete for what it claims.** Three production
construction sites, measured from all seventeen `IdentifyCache` occurrences:
`crates/muxsmith-core/src/pipeline.rs:127` (inside `LiveIdentifier`, the sole
production `LiveIdentifier` construction), `crates/muxsmith-cli/src/commands/identify.rs:21`,
`src-tauri/src/lib.rs:255`. Three test sites
(`crates/muxsmith-core/tests/command_integration.rs:232,494`,
`crates/muxsmith-core/tests/identify_live.rs:42`); `identify.rs:328` is
`new()`'s own `default()` body.

**Latitude, both forms.** No item exceeded its semantics. Item 6's extra
material - the rustdoc paragraph at `BUILDING.md:88-93` and the ci.yml comment
at `:94-98` - is within the item and factually true; my mutation fire is the
evidence for both ("private items are not rendered without it, so their doc
comments' links go unchecked"). Item 2 was fenced and was applied verbatim,
which is the correct behaviour under the no-latitude rule even though the fence
is wrong (F1). The one item that stopped short is item 6, at F3.

**House dimension.** `core-docs-name-callers-illustratively-never-exclusively`
(`docs/decision-ledger.yaml:4709`) is satisfied by item 3's new clause and
violated twice elsewhere in this change's own surface: F4 (left in place) and
F2 (newly written). `ledger-lint-runs-before-every-push`
(`docs/process-conventions.yaml:687`) is landed in BUILDING.md, subject to F3's
count.

**The no-work-needed premises, each run rather than read.** Item 6's "neither
file states a gate total": fired with a control (F3). Item 4's "safe today":
verified at the mount set and the component imports, conclusion holds, one
supporting clause false (F2). Item 7's decision to leave the neighbouring
numbers: correct in principle, under-executed (F5, Q6). The report's "0 skip
markers" claim: I did not accept the plain `cargo test` grep, which passes
vacuously because cargo captures passing tests' stdout. Re-run as CI does it -
`cargo test --workspace -- --nocapture --test-threads=1` - the marker count is
0 against the literal `MKVMERGE_SKIP_MARKER`
(`crates/muxsmith-core/src/lib.rs:29`), so the `have_mkvmerge()`-gated test
carrying item 2's assertions genuinely ran.

**Typography**, fired with a control: 78 added lines in `9dc3a4d`, zero matches
for `[\x{2013}\x{2014}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]`
under `grep -P`; the same pattern on a synthetic line with an em dash and an
ellipsis returns 1.

**Commit hygiene:** exactly one trailer, no `Claude-Session` line, two commits
ahead of `origin/master`, not pushed, tree clean at HEAD.

---

## 3. Verification - the eleven-part gate, my own run, foreground

| # | Command | Result (my run) |
|---|---|---|
| 1 | `cargo fmt --all --check` | exit 0, no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, 0 lines matching `^(warning\|error)` |
| 3 | `cargo test --workspace` | exit 0, **39** `test result:` lines, 0 non-`ok`, 0 with non-zero failed, 0 with non-zero ignored; `dry_run_cli.rs` **13 tests** |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | exit 0, 0 err/warn lines |
| 5 | `cargo deny check` | exit 0, `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy ... --target x86_64-pc-windows-msvc -- -D warnings` | exit 0; only 2 `^warning` lines, both the build-script notice `muxsmith-gui@0.1.0: GNU compiler is not supported for this target` |
| 7 | `pnpm lint` | exit 0, `$ eslint .` |
| 8 | `pnpm build` | exit 0, `built in 153ms` |
| 9 | `pnpm check:i18n` | exit 0, `212 catalog ids`, 41 source files, 0 unused warnings |
| 10 | `pnpm test:e2e` | exit 0, **68 passed (2.9s)** (via `pnpm test:e2e`, which rebuilds both harness bundles) |
| 11 | `python3 scripts/ledger-lint.py` | exit 0, `516 entries across 4 files, all invariants hold` |

Baselines: 39 / 68 / 212 / 516, all held, none moved. Aggregates recomputed
from my own logs, not quoted.

**Extra, beyond the eleven:** `cargo test --workspace -- --nocapture
--test-threads=1` (CI's skip-marker step) exit 0, 0 markers.

**Cross-OS residual risk of the flag, measured because nobody named it:**
`--document-private-items` widens the rustdoc surface, and rustdoc runs on all
three CI legs while the local gate covers one. The exposure is empty here:
`git grep -nE '#\[cfg\((target_os|windows|unix|not\(target_os)' -- 'crates/*/src' 'src-tauri/src'`
returns 18 hits, all inside `#[cfg(test)]` modules (`src-tauri/src/lib.rs:571+`,
past `mod tests` at `:567`; `src-tauri/src/run.rs:901+`), which `cargo doc`
does not document. No production item is cfg-gated, so the first CI run under
the flag has no platform-only doc surface to discover.

---

## 4. The seven adjudications

**Q1 - the fenced LOW-4 comment: does it stand because it was fenced?**
**No. A narrowing edit is owed, on the controller's licence.** I reproduced the
measurement myself (F1): the profile-load-failure document carries `files: []`,
so "carries neither key" is false, and the Task-5 verdict's own LOW-4 evidence
block prints that same value forty-odd lines above the text it fenced. A fence
binds the implementer, not the tree: it is an instruction about who may edit,
not a warrant that the text is true, and the implementer discharged it exactly
right by applying it verbatim and raising it. The licence is the controller's,
because the controller owns the brief that erected the fence - not the Task-5
reviewer's, who is not in this loop. Severity is LOW in consequence (a comment
in a test file, no assertion changes) and MEDIUM in kind: this package's sole
product is true sentences, and shipping a knowingly false one because of its
provenance inverts that. Required text at F1.

**Q2 - three construction sites or four?**
**Three is right, and the clause is complete.** Measured (F4 evidence, and the
full `IdentifyCache` occurrence list): pipeline seam, CLI identify, GUI
identify. The finding's "four" is its own arithmetic slip, not a fourth site it
knows about - read its sentence: "two production constructors outside the
planning seam", which plus the seam is three. Its "hold at all four sites" more
likely counts doc sites (the whole-branch fix's own table lists module doc,
type doc and the `LiveIdentifier.cache` field doc as the three it swept). The
new clause names all three production contexts and correctly omits the three
test constructions: the doc states the type's lifetime property and illustrates
it, which is what the house rule asks for, not an enumeration.

**Q3 - the module doc: this pass, or a separate vehicle?**
**This pass.** The finding scoped itself to the type's doc because that is
where the reviewer was looking, not as a fence excluding neighbours - it says
"whenever a licensed text pass next touches the file", and this is that pass,
in that file, three hundred lines from the edit. The house rule's own trigger
("you are writing a core doc sentence that names a surface") is satisfied
identically by both sentences. Leaving one means `identify.rs` now carries the
corrected and the uncorrected form simultaneously and a reader cannot tell
which is authoritative; the cost of avoiding that is one word. The implementer
was right not to take it unasked. See F4 for the text, and note that the
`LiveIdentifier.cache` field doc at `:392` is NOT part of it and must stay.

**Q4 - `pnpm build`: add it, or leave the discrepancy?**
**Add it.** This is F3, and I rate it MEDIUM rather than a matter of taste
because of what happens next: the controller must rewrite `HANDOFF.md:107`
from "TEN parts per BUILDING.md" to eleven, and there is no truthful way to
write that sentence while BUILDING.md enumerates ten. The ROADMAP ruling this
item implements says the block "gains ... an ELEVENTH part"; after the change
nothing in the file is an eleventh part. The alternative cut - declare the gate
ten and drop `pnpm build` from every external count - contradicts CI (which
runs it), the plan-9 plan, plan-8.5, HANDOFF and this pass's own brief. One
line closes it.

**Q5 - `ci.yml`'s "ninth gate part" provenance comment: re-word?**
**No, leave it.** `.github/workflows/ci.yml:88` opens "Plan 5.5 Task 12
(#18b):", which dates the attribution on its own face; the sentence records
what that task added and when, and recomputing it to "eleventh" would state
that Plan 5.5 Task 12 added an eleventh part, which is false. It is also
self-maintaining under the next gate change, whereas a live count there would
need editing at every one. The new Plan-9 comment sits directly beneath it and
is separately dated, so the two read as a provenance stack rather than a
contradiction. No change owed.

**Q6 - annotating the dated plan-7 enumerations rather than overwriting:
correct reading?**
**Correct in principle, under-executed in fact.** A record that declares its
own measurement date is a dated record and must not be silently overwritten -
that reading is right, and it is what the ROADMAP entry specified (it named the
`cli_validate` parenthetical alone). But the execution leaves an enumeration
whose header declares one date carrying one figure at HEAD and the rest at
2026-07-21, with nothing saying so; and `cli_validate.rs`'s neighbouring "5
helper call sites" in the same parenthetical is 7 at HEAD. That is a new
ambiguity introduced by the correction, not a pre-existing one left alone. F5
carries the required change: mark the refresh as scoped, and name both commits
behind the 3-to-5 delta instead of attributing it to amendment 4 alone.

**Q7 - item 4's one long sentence: split it?**
**Yes, and F2 forces the rewrite anyway.** At roughly eighty words with three
parenthetical attributions and a trailing backstop clause, the sentence buries
its own conclusion ("no mount here reaches any of them"), which is the part a
future reader needs. Two sentences: what it omits, then why that is safe today
plus the backstop. Purely presentational - the semantics do not change - so I
would not have raised it alone; since the clause is being corrected regardless,
take it in the same edit.

---

## 5. Evidence appendix

Independent instruments, all mine, none re-run from another agent's path and
none at a path the report names:

`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/closerev-independent/`

| File | What it is |
|---|---|
| `plfail.json`, `plfail.err` | my own profile-load-failure `--json` document (F1) |
| `snapkind.py` | brace-balanced scan mapping each `assert_snapshot` test to its invocation helper (F5, item 7 KIND) |
| `gatecount.py` | repo-wide sweep for gate-part-count statements, journal and sdd-scratch excluded (F3, HARVEST) |
| `g1.log` .. `g11.log` | the eleven gate parts, one log each |
| `g3nc.log` | `cargo test --workspace -- --nocapture --test-threads=1`, the skip-marker fire |
| `fire-flagged.log`, `fire-unflagged.log` | the mutation fire: same tree, flag on (exit 101, two ambiguity errors) and flag off (exit 0) |
| `baseline.txt`, `baseline-status.txt` | HEAD + `git hash-object src-tauri/src/lib.rs` taken before the mutation, used to prove the restore |
| `added-lines.txt`, `ctrl.txt` | the 78 added diff lines and the typography control line |

Commands beyond those: `git show 9dc3a4d` per file group; `git log
--diff-filter=A` per snapshot file; `git grep` for `IdentifyCache`,
`LiveIdentifier`, `config_only_document`, `cargo_bin`, `parse-error`,
`getSettings|setSettings`, `writeTextFile`, `platform`, `cargo doc`,
`document-private-items`; `python3 -c "yaml.safe_load"` over
`.github/workflows/ci.yml`; reads of `e2e/mocks.ts`, `e2e/mount-entry.ts`,
`e2e/jobsview-reset.spec.ts`, `crates/muxsmith-core/src/profile/load.rs`,
`crates/muxsmith-cli/tests/support/mod.rs`, `BUILDING.md`, `docs/ROADMAP.md:1125-1230`,
`.superpowers/sdd/plan-9/{task-5,task-6,whole-branch-review}-verdict.md`,
`docs/decision-ledger.yaml` entries `ci-15-rustdoc-gate`,
`does-the-ten-part-gate-bind-doc-only-pushes`,
`cargo-doc-is-no-evidence-for-a-doc-comment-in-a-test-target`,
`core-docs-name-callers-illustratively-never-exclusively`; regex greps over
`target/doc/muxsmith_gui_lib/struct.{ShellRenderer,AppState}.html`.

Every absence check above was fired: the gate-total grep against "six parts",
the old-console-string grep against the new string, the typography grep against
a synthetic em-dash line, the recorded-invoke grep against `e2e/mocks.ts`, the
skip-marker grep against the `MKVMERGE_SKIP_MARKER` literal, and the flag's
absence check against the deliberately mutated tree.

**Tree state:** clean at `9dc3a4d` at the end of this review, `git status
--porcelain` empty, `src-tauri/src/lib.rs` back to its committed blob. I
committed nothing and edited no product file. (This verdict file does not
appear in `git status` because `.gitignore:2` ignores `.superpowers/`;
confirmed with `git check-ignore -v`.)

---

## 6. HARVEST - for the controller

### 6a. The report's list of stale count statements: verified, all six hold

I read each site rather than trusting the list.

1. `HANDOFF.md:107-110` - "The gate is TEN parts per BUILDING.md ... `python3
   scripts/ledger-lint.py` ... is not one of the ten. Two edits to that gate
   block are an open close action". Stale on both halves; `HANDOFF.md:150-159`
   still lists the edits as open. **Confirmed, and see F3: the replacement
   sentence cannot truthfully say eleven until `pnpm build` joins the file.**
2. `docs/process-conventions.yaml:693` (`ledger-lint-runs-before-every-push`) -
   "It is not one of the ten parts ... The gate block in BUILDING.md gains it
   as an eleventh part at the Plan 9 close, together with the rustdoc
   private-items flag". Future tense, both landed. Confirmed.
3. `docs/decision-ledger.yaml:4585` (`does-the-ten-part-gate-bind-doc-only-pushes`) -
   "ledger-lint is not one of the ten parts, so the only check a docs-or-YAML
   push can turn red was not run before the push at all". Gap now closed.
   Confirmed. Its 11-second figure is explicitly a measurement of ten parts and
   reads fine as dated.
4. `docs/ROADMAP.md:1133-1192` - the whole "Gate: rustdoc does not link-check
   private items" section is now a completed close action, including its
   instruction at `:1191`. Confirmed; note `:1136` and `:1173` quote the
   unflagged command, and `:1159` cites `BUILDING.md:76` where the block is now
   at `:76-82` with the gate line at `:80`.
5. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` at
   `:11`, `:20`, `:117`, `:131`, `:431` - the live plan quoting the ten-part
   gate and the unflagged doc command. Confirmed; this is what ROADMAP `:1191`
   means.
6. ROADMAP's D64 "Docs accuracy" entry - its vehicle fired; my recount agrees
   with its 13 (5/3/4/1). Confirmed.
7. Pre-existing nine-part statements in live rule text,
   `docs/decision-ledger.yaml:4117` (`proc-task-check-subset-clause`) and
   `:4246` (`proc-push-ci-conclusion-observed`). Confirmed as flagged.

### 6b. What the report's list MISSED - three sites

1. **`docs/decision-ledger.yaml:2329` (`ci-15-rustdoc-gate`, Tier 1, settled).**
   Its statement reads "RUSTDOCFLAGS=-D warnings cargo doc --no-deps runs as
   the ninth gate part inside the matrixed test job on all three legs". Both
   halves are now wrong: the step is `cargo doc --workspace --no-deps
   --document-private-items`, and it is not the ninth part of any current
   count. This is live pattern text, not a dated occurrence line - the
   occurrence beneath it (`2026-07-11`) is what carries the date.
2. **`docs/decision-ledger.yaml:4614`
   (`cargo-doc-is-no-evidence-for-a-doc-comment-in-a-test-target`, Tier 1).**
   "... passes the rustdoc gate, every CI leg, and **the pending
   `--document-private-items` change** alike." The change is no longer pending;
   it is HEAD. The rule's substance is unaffected and still true (my flagged
   run is green while `crates/muxsmith-cli/tests/support/mod.rs` carries
   rustdoc that `cargo doc` never documents), so this is a wording fix, not a
   re-decision. Sharpest of the three, because a future agent reading a Tier-1
   rule that describes a landed change as pending will go looking for it.
3. **`BUILDING.md:120`'s "Rust-gate parts 1-5"** - F6. Pre-existing, but it is
   in one of the two files this pass edited and inside the paragraph that now
   sits directly under the new House-knowledge section.

Deliberately NOT flagged as owed, listed so the set is complete: the retired
plan documents that quote a nine- or ten-part gate
(`plans/2026-07-11-plan-5.5`, `-07-13-plan-5.6`, `-07-14-plan-5.7`,
`-07-14-plan-5.8`, `-07-16-plan-6`, `-07-21-plan-7`, `-07-23-plan-7.5`,
`-07-23-plan-8`, `-07-27-plan-8.5`), `docs/ROADMAP.md:128,965,1111,1122,1658`,
and every `docs/process-journal.md` occurrence. All are dated records of closed
work and correctly left alone.

### 6c. Process observations worth a ledger look

1. **A fenced text is a licence boundary, not a truth warrant.** Item 2's
   fenced comment was self-contradicted by its own verdict's measurement forty
   lines above it, and the fence carried the error into the tree unchanged
   while every party behaved correctly. Candidate handle: when a brief fences a
   text, the fencing party re-reads the fenced block against the measurement in
   its own document before the dispatch goes out - the trigger is readable
   ("you are pasting a block from an earlier verdict into a brief as
   binding"). This pass is the second time in Plan 9 that a first-pass
   reviewer's own wording seeded a later finding (the same is recorded in
   whole-branch finding 3's provenance note).
2. **The exclusive-enumeration rule fired three times in one package** - once
   as the item being fixed (item 3), once as the site left behind (F4), once as
   a sentence newly written in the same commit (F2). The rule currently reads
   as a rule about *core docs*; two of the three instances are outside core (an
   e2e spec comment, a module doc). Candidate: widen its stated surface, or
   record that the shape recurs wherever a comment justifies itself by naming
   who calls something.
3. **A count that lives in one file and is derived in six** is what F3 is. The
   gate now has exactly one authoritative enumeration (BUILDING.md) and at
   least six consumers deriving a number "per BUILDING.md" from it; nothing
   checks that the derivation matches. `scripts/ledger-lint.py` is the natural
   home for a cheap invariant here if the controller wants one, but that is a
   ROADMAP candidate, not this pass's business.

---

# Delta review

Scope: the fix round `c8dfc6d` only. I did not re-open the seven adjudications
or anything settled in the first pass. Fresh instruments throughout, in
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/closerev-delta/`;
none of the fix implementer's were re-run.

**Delta verdict: APPROVED.** All six findings close on their own terms. Nothing
new is introduced in the six files. Two INFO items and one recording candidate
are below; none is owed before this pass closes.

## 1. Does each finding close?

**F1 - closes.** The tree carries the verdict's replacement text verbatim at
`crates/muxsmith-cli/tests/dry_run_cli.rs:443-447`, and
`git diff 9dc3a4d c8dfc6d -- crates/muxsmith-cli/tests/dry_run_cli.rs` is
comment-only: one line out, two in, both assertions and both messages
untouched. The new claim is true of the emitter and of the wire - I re-read
`config_only_document` (`"files": []` unconditional at
`crates/muxsmith-core/src/report/json.rs:106`, `mkvmerge_found` only inside
`if let Some(found)` at `:110`), and the document itself was measured in the
first pass. "Satisfies both guards" is right: an empty array passes
`is_array()`, and no `mkvmerge_found` passes `is_none()`. "Ruled out by the
code sequence below, where it would be a singleton `parse-error`" is unchanged
and was already verified.

**F2 - closes, both halves.** The settings clause now states the property and
illustrates: "the settings pair is read across the app but by no component
this spec mounts (...)". I re-measured the consumer set with my own scanner
over `git ls-files src`: `getSettings`/`setSettings` appear in
`components/SettingsDialog.vue`, `main.ts`, `recentProfiles.ts`,
`views/BatchView.vue`, `views/EditorView.vue`, `views/FirstRun.vue`, plus the
definition site `ipc.ts:255,259`. Six consumers, exactly the six the comment
names, no seventh. The claim "by no component this spec mounts" holds down the
whole subtree, not just the top: `JobsView.vue:26-28` mounts `JobRow`,
`LiveLog`, `RunHistory`; of those only `RunHistory` touches IPC at all
(`getJobLog`, `listRuns`, and `writeTextFile` behind the `save()` dialog), and
none touches the settings pair. `platform()` is still `FirstRun.vue`'s alone -
my scanner's only other `platform` hit is `main.ts:11`, prose inside a doc
comment ("no resolvable platform config dir"), not a call. Q7 is discharged:
the paragraph is two sentences and the second opens on the conclusion.

**F3 - closes, and see section 3.**

**F4 - closes.** `crates/muxsmith-core/src/identify.rs:5` now reads
"constructed per call and dropped with it, so separate calls re-identify", two
words changed and nothing rewrapped. My own phrase sweep over `git ls-files`
(journal and sdd-scratch excluded, fired against a known-present control)
leaves exactly two `per planning call` hits in the file: `:305`, the corrected
type doc's illustrative parenthetical, and `:392`, `LiveIdentifier.cache`'s
field doc, which is true as written and which the fix correctly did not touch.
The file no longer carries the corrected and uncorrected forms side by side.

**F5 - closes.** Both documents carry the scoping wording
(`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1561`,
`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:80`). Provenance
re-measured with a fresh script: the three 2026-07-12 snapshots (`aba7f4f`),
`bare_raw_property_exits_two_and_renders_the_message` at `d768657`
2026-07-28 20:16, `bare_raw_property_renders_german_with_locale_flag` at
`3412fcc` 2026-07-28 20:50. So "3 snapshots at this measurement" is right for
the declared 2026-07-21 date, "5 at the Plan-9 close" is right, and both
commits behind the delta are now named with the correct attribution.
Recount unchanged: 13 total, 5/3/4/1/0.

**F6 - closes.** `BUILDING.md:121` now reads "parts 1-4", which matches the
workflow: my own `yaml.safe_load` walk shows the matrixed `test` job running
`cargo fmt`, `cargo clippy`, `cargo test`, `cargo doc ... --document-private-items`
with no `if:` on any of them, and `cargo deny check` in its own `deny` job. The
sentence no longer contradicts its own second half.

## 2. Anything new, in the six files or as a ripple?

No. The eleven-part gate, my own foreground run at `c8dfc6d`:

| # | Result |
|---|---|
| 1 `cargo fmt --all --check` | exit 0 |
| 2 `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, 0 `^(warning\|error)` |
| 3 `cargo test --workspace` | exit 0, **39** `test result:` lines, 0 non-`ok`, 0 non-zero failed, 0 non-zero ignored |
| 4 `RUSTDOCFLAGS="-D warnings" cargo doc ... --document-private-items` | exit 0, 0 err/warn |
| 5 `cargo deny check` | exit 0, `advisories ok, bans ok, licenses ok, sources ok` |
| 6 windows-target clippy | exit 0, 2 `^warning` lines, both the pre-existing GNU-compiler build-script notice |
| 7 `pnpm lint` | exit 0 |
| 8 `pnpm build` | exit 0, `built in 151ms` |
| 9 `pnpm check:i18n` | exit 0, **212** catalog ids |
| 10 `pnpm test:e2e` | exit 0, **68 passed (2.9s)** |
| 11 `python3 scripts/ledger-lint.py` | exit 0, **516** entries |

Baselines 39 / 68 / 212 / 516 all held. Typography: 24 added lines, zero
matches for the AI-tell glyph class under `grep -P`, control line appended and
the same pattern then hits once. Commit hygiene: one `Co-Authored-By` trailer,
no `Claude-Session`, `master` 3 ahead of `origin/master`, not pushed, tree
clean.

**The no-work-needed premises the fix report uses, each checked:**

- *"No edit here changes any check's outcome, so there was nothing to fire."*
  Sound, and not merely asserted: the only edit rustdoc even renders is the
  `identify.rs` module doc, which carries no intra-doc link and no code fence,
  so there is no construct in the diff whose breakage a check could observe.
  My part-4 run confirms it green.
- *"`BUILDING.md:74`'s '(six parts)' stays correct."* Verified: the Rust block
  is still six command lines (`:77-82`), untouched.
- *"CI already runs `pnpm build`."* Verified in my workflow parse: unconditional
  step in the matrixed `test` job.
- *"The new scoping clause already covers the stale neighbour."* Ruled on at
  section 5b.

**INFO-1 (no fix owed) - F2's second sentence subordinates one coordinate
reason under the wrong head.** `e2e/jobsview-reset.spec.ts:102-111` reads "That
is safe today because no mount in this spec reaches any of them: ... and every
test in this file asserts DOM state rather than a recorded call log". Three of
the four listed justifications are instances of "no mount reaches"; the
recorder clause is not - a mount does not "reach" the Node-side invoke log, and
that clause is the independent reason the missing forwarding is harmless.
Nothing stated is false and the conclusion is unaffected; I raise it only
because the restructure was mine to ask for. Not worth another round.

**INFO-2 (no fix owed) - F5's scoping clause is marginally over-broad in both
documents.** "the other figures in this enumeration are the 2026-07-21
measurement, unrefreshed" is exactly true of the per-file enumeration it sits
inside, and both documents also carry Plan-9-close figures within the same
bullet/paragraph (the `5+3+4+1+0 = 13` recount and the 12/1 split). Those
carry their own explicit "recounted at the Plan-9 close" label, so no reader
can mistake which measurement is which, and the clause does the job it was
added for. Recorded, not owed.

**INFO-3 (pre-existing, adjacent to F6, no fix owed) - BUILDING.md's CI
sentence does not distinguish the Linux-only frontend steps.**
`BUILDING.md:121-124` lists `pnpm lint`, `pnpm build`, `pnpm check:i18n`,
`pnpm test:e2e` after the three-OS-leg clause. My workflow parse shows
`pnpm lint` and `pnpm build` unconditional but `pnpm check:i18n` and
`pnpm test:e2e` gated `runner.os == 'Linux'` (as is the TS-bindings step, which
BUILDING.md does not mention at all). The sentence attaches "on all three OS
legs" to the Rust parts and the frontend list to the push/tag/PR trigger, so it
is ambiguous rather than false, and it predates both commits. Worth a clause
the next time that paragraph is touched, since BUILDING.md is now the gate's
sole authoritative definition and a contributor on Windows or macOS would read
it as full coverage. Not this pass's business.

## 3. F3, specifically

**BUILDING.md's own enumeration now comes to eleven.** Measured with a fresh
parser that walks the file's headings and fenced blocks rather than by eye:

| block | commands |
|---|---|
| `### The Rust gate (six parts...)` `:77-82` | 6 |
| `### Frontend checks` `:105-108` | 4 (`pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`) |
| `### House-knowledge check` `:114` | 1 |
| **gate total** | **11** |

(The parser also picks up the three non-gate blocks - system libraries,
"Building and running", the release-bundle recipe - which are correctly outside
the count.)

**No consumer of the gate's definition still disagrees with the file**, and one
edit repaired two of them as a side effect. `plan-8.5:20` and the plan-9 plan
at `:20` both derive the gate as "'The Rust gate' six parts + **the four**
frontend checks"; before this edit the block held three, so that derivation
failed against the file it cites. It now checks out. The controller's owed
rewrite of `HANDOFF.md:107` to eleven likewise derives cleanly: 6 + 4 + 1.
`.github/workflows/ci.yml` carries the identical doc invocation and, after F6,
BUILDING.md's description of what CI runs matches the workflow I parsed. Every
remaining disagreement is a count or a tense in a controller-owned file, all of
them already enumerated in HARVEST 6a/6b - none of them is a second definition
of the gate. The defect class F3 named is closed, not moved.

## 4. Is HARVEST 6b complete?

**No - one item closed, two stand, and one new site surfaced.**

- **6b item 3 is closed** by F6: `BUILDING.md:120` no longer says "parts 1-5".
- **6b items 1 and 2 stand**, unchanged and still yours:
  `docs/decision-ledger.yaml:2329` (`ci-15-rustdoc-gate`, describing the step as
  `cargo doc --no-deps` and "the ninth gate part") and `:4614`
  (`cargo-doc-is-no-evidence-for-a-doc-comment-in-a-test-target`, calling the
  flag change "pending"). I re-read both at HEAD; neither moved.
- **New, surfaced by F4 rather than by the first pass.** With both code docs now
  reading "per call", the exclusive form survives in exactly three prose sites,
  and my phrase sweep (fired control) says they are the only ones outside the
  journal:
  1. `docs/conventions.yaml:943`, owning entry `core-20-ondisk-cache` at `:937`:
     "The identification cache is in-memory, constructed per planning call and
     dropped with it". That entry's own note says its lifetime clause was
     corrected 2026-07-29 after S-8, so it is live and actively maintained.
  2. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:312` - spec 5.5
     itself, the sentence both code docs cite.
  3. `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md:1500`
     - the plan-9 design quoting the same spec sentence verbatim.

  **Weigh it before acting.** Spec 5.5's sentence is scoped to the plan/run
  flow ("run plans and executes within a single call ... In the CLI, call and
  process coincide"), so it is not false there; what changed is that
  `identify.rs:305` now cites spec 5.5 for a property stated more broadly than
  the spec states it. That is a citation reaching one step past its source, not
  a contradiction, and closing it is a spec amendment, not a text fix. My
  recommendation is to record it - the ROADMAP's "Docs accuracy" section or
  `core-20-ondisk-cache`'s own statement - rather than open a vehicle for it in
  this close.

## 5. The two the implementer surfaced and did not touch

**5a. `docs/superpowers/specs/...-design.md:1505`, "amendment 4 later added
exactly one" - NOT owed.** Every clause in that sentence is literally true:
amendment 4 did add exactly one snapshot file (`3412fcc`, measured again this
round), the count it goes on to state is the correct 13, and the 12/1 split is
correct. The reason it can stand where F5's parenthetical could not is what the
sentence is *about*: it is the de-coverage claim, and the other post-design
addition, `d768657`'s bare-raw case, rides `support::muxsmith` - the en funnel -
so it is irrelevant to de coverage. The parenthetical F5 fixed was different in
kind: it was an arithmetic claim about `cli_validate.rs`'s own snapshot count,
where omitting one of two additions left the numbers not adding up. If you want
belt and braces it costs four words ("added one of the two, the German case"),
but it is optional, and the pass can close without it.

**5b. `cli_validate.rs`'s "5 helper call sites", 7 at HEAD - covered, NOT
owed.** The new clause names it as one of "the other figures in this
enumeration ... the 2026-07-21 measurement, unrefreshed", which is precisely
the marker F5 asked for; in the plan document it is covered twice, since that
paragraph's own header already reads "measured 2026-07-21 ... re-verify by text
at dispatch". Refreshing it would reopen exactly the overwrite-a-dated-record
question Q6 settled. Leave it.

## 6. Delta evidence appendix

`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/closerev-delta/`

| File | What it is |
|---|---|
| `enum.py` | heading-and-fence parser over BUILDING.md; the eleven-command enumeration in section 3 |
| `ci.py` | `yaml.safe_load` walk of every job, matrix and per-step `if:` in `ci.yml` (F6, INFO-3) |
| `snapprov.py` | fresh snapshot recount plus per-file `--diff-filter=A` provenance (F5) |
| `consumers.py` | scanner over `git ls-files src` for `getSettings`/`setSettings`/`writeTextFile`/`platform` (F2) |
| `percall.py` | repo-wide `per planning call` sweep, journal and sdd-scratch excluded (F4, HARVEST) |
| `derive.py` | sweep for external "frontend checks" derivations (section 3) |
| `added.txt` | the 24 added diff lines plus the appended typography control |
| `d1.log` .. `d11.log` | the eleven gate parts, one log each |

Absence checks fired this round: the `per planning call` sweep against a
known-present control in `identify.rs`; the frontend-derivation sweep against
`plan-9:20`; the typography scan against a synthetic em-dash/ellipsis line
appended to the same file. Tree clean at `c8dfc6d`, `git status --porcelain`
empty; I committed nothing and edited no product file.
