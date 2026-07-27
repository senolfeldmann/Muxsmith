# joblog calendar-bomb fix - independent review verdict

**Verdict: APPROVED**

**Under review:** commit `c06b8dd3ef8bb7b710b4a4c2ce70ca15ec5ec92d`, diff package
`.superpowers/sdd/plan-8/review-89782cd..c06b8dd.diff`, implementer report
`joblog-datebomb-fix-report.md`.
**Tree:** MAIN `/home/senol/Git/Muxsmith`, branch `master`, HEAD `c06b8dd` for the whole review
(no worktree entered, no session relocation). Working tree verified clean before and after
every probe. A concurrent session advanced master to `e525813` at 12:26 while this review ran
("plan-7.5: amendment date fix"), a docs-only change to
`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md` (two `2026-07-22`
-> `2026-07-27` edits). No code, no stamps; live-code stamp total re-measured at the new HEAD
is still 28. Nothing in this verdict is affected.
**Governing rule:** ledger entry `test-fixture-dates-outside-retention-windows`
(`docs/decision-ledger.yaml:4252-4265`, `tier: 1`).

The code fix is correct, minimal, and control-proven load-bearing. The sweep's *conclusion*
(exactly one must-survive site workspace-wide) survives independent re-verification. One
Medium finding is a report/record accuracy defect in the sweep enumeration, not a code
defect: the count and file set are wrong and have already propagated into the commit
message and `progress.md`. Nothing in it changes the fix or the conclusion, so it is a
controller correction, not a re-implementation.

---

## 1. Required verifications

### (1) The fixture id derives from the clock; both dirs, both expectations; collision behavior untouched - PASS

`crates/muxsmith-core/tests/joblog.rs:141-160`. `let run_id = make_run_id(SystemTime::now());`
computed **once** and reused at all five consumption points: the two pre-created dirs
(`runs_root.join(&run_id)`, `runs_root.join(format!("{run_id}-2"))`), both `create` arguments
(`&run_id`), and both expectations (`format!("{run_id}-2")`, `format!("{run_id}-3")`). No
literal stamp remains in the test. Single derivation means no second-boundary flake: the
two `create` calls cannot disagree about the name.

Mechanism claims in the report checked at the source:
- `prune_stale_runs(runs_root, SystemTime::now())` at `src/executor/joblog.rs:205`, ahead of
  the `create_dir` collision loop at `209-218`. Report's line citations are exact.
- `run_id_timestamp` parses the fixed 16-byte prefix (`joblog.rs:107-112`), so
  `<run_id>-2` carries the same fresh timestamp and survives the second `create`'s prune.
  Unparseable names hit `let Some(started_at) = ... else { continue }` and are never deleted.
- Behavior under test unchanged: two collisions, `-2` then `-3`.

**Control (mine, not borrowed).** Restored the pre-fix blob
(`git show 89782cd:crates/muxsmith-core/tests/joblog.rs`) into the tree, ran the single test,
restored, confirmed `git diff --quiet`:

```
thread 'collision_appends_a_numeric_suffix' panicked at crates/muxsmith-core/tests/joblog.rs:147:5:
assertion `left == right` failed
  left: "/tmp/.tmppIB2hN/runs/20260710-120000Z"
 right: "/tmp/.tmppIB2hN/runs/20260710-120000Z-2"
```

Identical file:line and left/right shape to the report's quoted red. The literal stamp is
still red today; the derivation is therefore load-bearing, not decoration.

### (2) `cargo test -p muxsmith-core --test joblog` - PASS, 12/12

Run by me, foreground, in the main tree at `c06b8dd`:

```
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All twelve named tests listed, including `collision_appends_a_numeric_suffix`,
`create_prunes_run_dirs_older_than_14_days_by_name_only`,
`prune_stale_runs_leaves_a_stale_named_symlink_and_its_target_untouched`. Also ran
`cargo fmt --all --check`: exit 0, no diff. The workspace-total claim (494) was not re-run;
the change is confined to one test target in one crate, so it is not load-bearing for this
verdict, and I am not adopting the number.

### (3) Sweep re-run and classification spot-check - PASS with one finding (M1 below)

**Fire-check first.** Pattern `[0-9]{8}-[0-9]{6}Z` fired against this fix's own diff context
(6 removed lines in `review-89782cd..c06b8dd.diff`) and against the pre-fix blob at the
known-bad site (lines 144, 146, 147, 149, 150, 151). The pattern can fire; an empty result
elsewhere is evidence rather than a malformed-pattern artifact.

**Pattern adequacy.** Sufficient for the mechanism under review: `prune_stale_runs` decides
age from the directory NAME via `run_id_timestamp`, which only accepts this compact shape.
A fixture stamped in any other format (RFC3339, etc.) parses to `None`, is skipped, and can
never become a prune candidate. So no second stamp shape needs sweeping for this class.

**Single retention mechanism, verified independently.** Every deletion call in live code:
`joblog.rs:131` (`remove_dir_all`, inside `prune_stale_runs`), `executor/job.rs:205`
(`remove_file`, delete-on-failure of a mux output), `src-tauri/src/settings.rs:188`
(`remove_file`, tmp cleanup). Only the first is date-keyed. `prune_stale_runs` has exactly
one production caller, `RunLogger::create` at `joblog.rs:205`; it is never called from
`src-tauri/`. Retention-keyword grep over `.rs/.ts/.tsx/.svelte/.js/.sh` hits only
`joblog.rs` plus generated e2e bundle noise. Report's claim holds.

**Measured totals (mine).** Live code only (`crates src-tauri src e2e scripts help locales
.github index.html`, excluding `target/`, `.worktrees/`, `node_modules/`, `docs/`,
`.superpowers/`):

| | pre-fix `89782cd` | post-fix `c06b8dd` |
|---|---|---|
| `src-tauri/src/run.rs` | 17 | 17 |
| `crates/muxsmith-core/tests/joblog.rs` | 14 | 8 |
| `src-tauri/src/error.rs` | 2 | 2 |
| `e2e/smoke.spec.ts` | 1 | 1 |
| **total** | **34** | **28** |

Report says 33 across 3 files pre-fix, 27 post-fix. See **M1**.

**Classification spot-check.** The must-survive class (6 occurrences, the collision site) and
the aging class are as the report describes. Verified empirically rather than by reading:

*Probe A - `create_prunes_run_dirs_older_than_14_days_by_name_only` (now L269-270).* Replaced
its two absolute stamps with a clock-derived, one-day-old name, ran the test, restored:

```
assertion `left == right` failed: the two stale run dirs must be pruned; ...
  left: ["20260726-102630Z", "20260726-102630Z-2", "20260727-102630Z", "keep-me", "notes.txt", "this-run"]
 right: ["20260727-102630Z", "keep-me", "notes.txt", "this-run"]
```

Clock-deriving breaks it. Absolute stamps load-bearing, correctly untouched.

*Probe B - `prune_stale_runs_leaves_a_stale_named_symlink_and_its_target_untouched` (now L387).*
Report's concern-4 reasoning needs a two-factor witness, because "test still passes with a
fresh name" is uninformative on its own (the symlink exclusion at `joblog.rs:120-122` runs
*before* the name parse, so the name never matters while the guard is intact). Removed the
guard branch (`if !file_type.is_dir() { continue; }`) from the source and crossed it with the
fixture:

| source | fixture stamp | result |
|---|---|---|
| guard removed | absolute `20200101-000000Z` | **FAIL** ("the symlink entry itself must still be there: NotFound") |
| guard removed | clock-derived | **PASS** (vacuous) |

Exactly the report's claim, now with a mechanism witness: the absolute stamp is what makes
the test discriminate on symlink-ness instead of on freshness. Correctly untouched. Both
mutated files restored; `git diff --quiet` confirmed clean after each probe.

### (4) Comment cites the ledger entry by id and names the counter-case - PASS

`joblog.rs:143-149` names `test-fixture-dates-outside-retention-windows` verbatim, states the
mechanism (prune precedes the collision search), and names
`create_prunes_run_dirs_older_than_14_days_by_name_only` as the counter-case, correctly
described as "below" (L264 vs L141). It sits on the derivation line, which is the line a
future author would edit back to a literal. See **N1** for a scoping nitpick.

### (5) Nothing else in the diff - PASS

`git show --stat c06b8dd` and `git diff --name-only 89782cd c06b8dd` both report a single
file, `crates/muxsmith-core/tests/joblog.rs`, one hunk, 15 insertions / 8 deletions. All of
it inside `collision_appends_a_numeric_suffix` plus its new inline comment. No drive-by
edits, no reformatting, no doc-comment churn. `.superpowers/` is gitignored, confirmed by the
clean `git status --porcelain`.

---

## 2. Judgment on report concern 3 (four sites pass stale stamps as `create`'s `run_id`)

**The deliberately-not-touched call is correct.** Verified, not accepted:

- All four sites confirmed: `joblog.rs:70` (`full_lifecycle_writes_job_and_summary_files`),
  `src-tauri/src/run.rs:1340` (`run_batch_writes_job_log_files`), `:1499`
  (`finalize_joblog_ok_is_complete`), `:1510` (`finalize_joblog_write_failure_is_incomplete`).
  Each is a distinct `#[test]` with its own tempdir root and exactly one `RunLogger::create`
  against it (checked by enumerating every `RunLogger::create` call against the `#[test]`/`fn`
  boundaries in both files). `prune_stale_runs` is never called directly from `run.rs`.
- Under the ledger entry's own scope these are not violations. Its statement binds "fixture
  stamps that must count as **fresh**". These must not: prune runs before the leaf exists, so
  the stale-named leaf is created after the only prune in its test and is never a candidate.
  The class the entry names is "must survive a retention mechanism", and none of these do.
- Fixing them would be scope creep on a single-defect dispatch, and would cost information in
  the neighbouring read-path tests. The report's classification there is right: `run.rs:1578-1596`
  (`list_runs_in_skips_unreadable_dirs_and_sorts_newest_first`) asserts
  `metas[0].run_id == "20260710-120000Z"` for newest-first ordering, which needs two known
  distinct dates; clock-deriving them would break the ordering assertion outright.
- The residual is real but documentary, not behavioral: the protecting invariant
  (prune-before-leaf, one `create` per root) is implicit and unstated at those sites, so a
  future second `create` against the same root silently re-creates this class. Right home is
  the ledger or a note on next touch of those files, which the controller has already logged
  in `progress.md:31`. See **H4**.

Raising it rather than silently fixing it was the correct move, and the report's framing
("nothing is broken today, so nothing was changed") is accurate.

---

## 3. Findings

### Medium

**M1 - the sweep enumeration is incomplete; the measured figure is wrong and has already propagated.**

Report section 3 states "33 stamp occurrences pre-fix across 3 files - joblog.rs 14,
`src-tauri/src/run.rs` 17, `src-tauri/src/error.rs` 2" and "Post-fix 27". Measured pre-fix
live-code total is **34 across 4 files**; post-fix **28**. The omitted file is
`e2e/smoke.spec.ts:481`:

```ts
  const RUN_ID = "20260710-190000Z";
```

Two distinct problems, and the second is the one that matters:

1. *The count is wrong.* It is a stated measurement, and it is off by one in both the
   occurrence total and the file count.
2. *The stated method was not the executed method.* The report describes the sweep as
   "`grep -rnE '[0-9]{8}-[0-9]{6}Z'` over the workspace, excluding `target/` and the
   `.worktrees/` siblings", which would have found this site. The executed sweep was
   effectively Rust-only. The report's non-Rust grep is cited as covering that gap, but it
   searched *retention keywords* (`prune|retention|older.?than|14 ?day`), a different
   question: it can establish that no non-Rust retention *mechanism* exists while saying
   nothing about non-Rust *fixture stamps*. Passing the first grep does not scope the second.

**Classification of the omitted site, verified by me: no-retention-interaction, not a bomb.**
`e2e/smoke.spec.ts:477-505` (`jobs view: live run`) drives the GUI against
`installTauriMocks`, with `start_run: [resolveWith(startedRun)]` returning a fabricated
`StartedRun { run_id: RUN_ID, total_jobs: 2, run_dir: "/runs/<RUN_ID>" }`. No filesystem, no
`RunLogger`, no prune anywhere in the path. The stamp is an opaque string in a mocked IPC
payload.

So the report's headline conclusion, "exactly one must-survive site workspace-wide", is
**correct** and survives my independent complete sweep. What does not survive is the number
and the file set, and those are already in two places that quote them as a baseline:

- commit `c06b8dd` message: "33 absolute-stamp occurrences across 3 files" (permanent record,
  and the brief forbids git writes, so this is corrected by annotation, not amendment)
- `.superpowers/sdd/plan-8/progress.md:31`: "sweep 33 hits classified 6/3/24"

**Owed:** controller-side correction to 34/28 across 4 files with `e2e/smoke.spec.ts:481`
classified as no-retention-interaction, in `progress.md` and in the ledger occurrence for
`test-fixture-dates-outside-retention-windows`; plus a note that the entry's sweep is not
Rust-scoped. No code change. Not blocking, because the forward risk is a wrong baseline for
the next sweep, not a live bomb.

### Low

**L1 - every line citation in the report is a pre-fix line number.** The aging tests are cited
as "L262-263" and "L380"; in the committed file they are at **L269-270** and **L387** (the fix
added 7 lines above them). The positive-control lines "144, 146, 147, 149, 150, 151" are also
pre-fix positions; post-fix the site spans L141-160. Fine for a reader working from the diff,
wrong for anyone opening HEAD. `run.rs` and `error.rs` citations are unaffected and check out.

### Nitpicks

**N1 - the comment states a stricter universal than the ledger, and one its own file does not
observe.** "Absolute stamps belong only in fixtures that TEST the aging path" reads as a
workspace-wide prohibition. `full_lifecycle_writes_job_and_summary_files`, 70 lines above in
the same file, passes `"20260710-120000Z"` as `create`'s `run_id` argument and is neither an
aging test nor a violation. The ledger's own sentence carries the scoping clause the comment
drops ("Fixture stamps that must count as **fresh** derive from now"). A future author
following the comment literally either "fixes" L70 or reads it as an unfixed violation; one
clause ("fixtures that must SURVIVE a prune") closes it. Also worth noting the counter-case
list names only the seed-and-prune test, not the symlink test whose stamp is the subtler of
the two, though "e.g." makes that non-exhaustive by construction.

**N2 - dispatch/ledger tier mismatch: the report flagged it and is right.** `docs/decision-ledger.yaml:4254`
records `tier: 1`; the dispatch called it Tier 2. Cited by id in the comment, so the code is
correct either way. Controller-side text fix.

**N3 - commit trailer.** The report's note is accurate: the repo's recent commits carry
`Co-Authored-By` only, no `Claude-Session` trailer. Consistent with local practice; owner call,
not a review finding.

---

## 4. HARVEST

**H1 - a guard test whose FIXTURE is what makes it discriminate needs a two-factor probe.**
Removing the guard and watching the test fail proves the guard is covered; it does not prove
the *fixture* is load-bearing. Mutating the fixture and watching the test still pass proves
nothing on its own, because a healthy guard can mask the fixture's irrelevance. Only the
cross does: guard removed x absolute stamp -> FAIL, guard removed x clock-derived stamp ->
PASS is what establishes that the stamp, not the guard alone, carries the discrimination.
Generalizes to any test where a guard branch short-circuits before the fixture property is
read, which is exactly where "would this go vacuous?" cannot be answered by reading. Sits
next to the existing `redundant-layers-need-mechanism-witness` entry and to the house rule
that a check whose passing result is an absence must be made to fire once.

**H2 - a sweep report states its file-type filter verbatim, or its count is unbounded.** M1's
root cause is not arithmetic: a pattern-shaped sweep ran over `*.rs` while the method line
claimed "over the workspace". Candidate pattern: a completeness claim carries the exact
invocation, including includes/excludes, and a keyword grep for the *mechanism* never counts
as scope coverage for a *fixture-shape* grep. The trigger is readable: you are writing the
words "over the workspace" above a number.

**H3 - a measured figure in a commit message becomes uncorrectable in place.** The wrong 33/3
now lives in `c06b8dd`'s permanent message and in `progress.md`. Sweep counts belong in the
report and the ledger occurrence, where a correction lands next to the claim; putting them in
the commit message guarantees the next reader quotes a wrong baseline with no visible
correction path.

**H4 - a fixture safe only by call order is safe by accident.** The four concern-3 sites hold
only because prune precedes leaf creation and each test calls `create` once per root. Nothing
at those sites says so. Candidate ledger pattern: when an invariant of the code under test is
what keeps a fixture valid, that invariant is stated at the fixture or recorded in the ledger,
otherwise the next edit (a second `create` against the same root) re-creates the original
defect class with no warning. The controller already logged the observation; this promotes it
from a note to a rule with a trigger.

---

## 5. Verification hygiene of this review

- All test runs foreground, in the main tree, absolute paths throughout.
- Four temporary mutations (pre-fix blob restore; aging-test stamps; symlink-test stamp;
  `prune_stale_runs` guard branch) applied via `python3` heredocs with an asserted
  single-occurrence match, backed up and restored with `command cp -f` (alias-proof), each
  inside a `trap ... EXIT` so a failed run could not leave the tree dirty. `git diff --quiet`
  confirmed clean after each, and `git status --porcelain` is empty at the end apart from this
  verdict file, which lives under gitignored `.superpowers/`.
- No commits, no git writes, no session-relocation tools, no worktree entered.
- Borrowed vs verified: every load-bearing claim above is mine. The report's workspace-wide
  `cargo test` total (494) and its `clippy` result are the only claims I did not reproduce,
  and no finding rests on them.
