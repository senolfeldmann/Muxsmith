# joblog calendar-bomb fix - implementer report

**Dispatch:** standalone fix, surfaced by the plan-8 stream-A merge gate.
**Tree:** MAIN `/home/senol/Git/Muxsmith`, branch `master` (no worktree entered, no session relocation).
**Commit:** `c06b8dd3ef8bb7b710b4a4c2ce70ca15ec5ec92d` (unsigned, `%G? = N`), not pushed.
**Governing rule:** ledger entry `test-fixture-dates-outside-retention-windows` (docs/decision-ledger.yaml).

## 1. Reproduction (red, pre-fix)

`cargo test -p muxsmith-core --test joblog collision_appends_a_numeric_suffix`

```
thread 'collision_appends_a_numeric_suffix' (67529) panicked at crates/muxsmith-core/tests/joblog.rs:147:5:
assertion `left == right` failed
  left: "/tmp/.tmpz8nczx/runs/20260710-120000Z"
 right: "/tmp/.tmpz8nczx/runs/20260710-120000Z-2"
```

Reads exactly as the root cause predicts: the pre-created fixture dir was
pruned before `create` looked for a collision, so `create` succeeded on the
**bare** name (`left`) instead of falling back to `-2` (`right`). The
collision never happened at all; nothing about the suffix logic is broken.

Mechanism, confirmed at the source (`crates/muxsmith-core/src/executor/joblog.rs`):
`RunLogger::create` calls `prune_stale_runs(runs_root, SystemTime::now())` at
line 205, **before** the `create_dir` collision loop at 209-218.
`prune_stale_runs` decides age from the directory NAME via `run_id_timestamp`
against a fixed `RUN_LOG_RETENTION` of 14 days (line 86). `20260710-120000Z`
passed out of that window on 2026-07-24.

## 2. Fix

`crates/muxsmith-core/tests/joblog.rs`, `collision_appends_a_numeric_suffix`:
the fixture run id now derives from the clock via the already-imported
`make_run_id`, and both pre-created dirs plus both expectations use it.

```rust
let run_id = make_run_id(SystemTime::now());
std::fs::create_dir_all(runs_root.join(&run_id)).unwrap();

let logger = RunLogger::create(&runs_root, &run_id, &[]).unwrap();
assert_eq!(logger.dir(), runs_root.join(format!("{run_id}-2")));

std::fs::create_dir_all(runs_root.join(format!("{run_id}-2"))).unwrap();
let logger2 = RunLogger::create(&runs_root, &run_id, &[]).unwrap();
assert_eq!(logger2.dir(), runs_root.join(format!("{run_id}-3")));
```

Behavior under test is unchanged: still two collisions, still asserting the
`-2` then `-3` fallback. The second `create` prunes again, and both fixture
dirs are clock-fresh, so both survive it.

The ledger entry is cited in a comment at the derivation line - the exact
spot a future author would edit back to a literal stamp - naming both the
rule and the counter-case (absolute stamps belong only in the aging tests).
The test's doc comment still describes *what* is under test; the inline
comment guards the *fixture mechanic*, so neither restates the other.

**Control for the fix:** same test, same assertions, only the stamp
derivation changed - literal stamp gave the red quoted above, derived stamp
gives green. The derivation is therefore load-bearing, not decoration.

## 3. Sweep

**Retention mechanisms in the workspace: exactly one.** `prune_stale_runs`
is the only date-keyed deletion anywhere, and `RunLogger::create` is its only
caller. Verified by enumerating every `remove_dir_all`/`remove_file` in
`crates/` and `src-tauri/`: the other two are `executor/job.rs`'s
delete-on-failure of a mux output and `src-tauri/settings.rs`'s tmp-file
cleanup, neither keyed on a date. A grep over `*.ts|*.tsx|*.js|*.svelte|*.sh|*.py`
for `prune|retention|older.?than|14 ?day` returned nothing outside `docs/`,
so no non-Rust retention path exists either.

**Grep:** `grep -rnE '[0-9]{8}-[0-9]{6}Z'` over the workspace, excluding
`target/` and the `.worktrees/` siblings.

**Positive control:** the pattern fired on the known-bad site before the fix
(joblog.rs lines 144, 146, 147, 149, 150, 151), so the empty result for the
other categories is not a malformed-pattern artifact.

**Counts (measured, not estimated):** 33 stamp occurrences pre-fix across 3
files - joblog.rs 14, `src-tauri/src/run.rs` 17, `src-tauri/src/error.rs` 2.
Post-fix 27 (the 6 at the collision site are gone). 6 + 3 + 24 = 33 below.

| Class | Occ. | Sites | Action |
|---|---|---|---|
| **must-survive** | 6 | joblog.rs `collision_appends_a_numeric_suffix` (2 pre-created dirs, 2 `create` args, 2 expectations) | **fixed** |
| deliberately-tests-aging | 3 | joblog.rs L262-263 `create_prunes_run_dirs_older_than_14_days_by_name_only`; L380 `prune_stale_runs_leaves_a_stale_named_symlink_and_its_target_untouched` | untouched |
| no-retention-interaction | 24 | joblog.rs L39, L42, L70, L324, L329; run.rs 17 sites; error.rs 2 sites | untouched |

**Result: exactly one must-survive site workspace-wide, the known defect.**

### Why the untouched classes are correctly untouched

**Deliberately-tests-aging.** L262-263 seed dirs that *must* be pruned; that
is the assertion. L380 is the subtler one: its symlink is named
`20200101-000000Z` precisely so it looks like a stale prune candidate. A
clock-derived name there would make `prune_stale_runs` skip it for the wrong
reason (too fresh) instead of the right one (symlinks are excluded from
`file_type().is_dir()`), i.e. the test would pass vacuously. Its absolute
stamp is load-bearing and must stay.

**No-retention-interaction, two distinct reasons:**

1. *No filesystem at all* - `make_run_id` expected outputs (joblog L39, L42),
   `run_id_timestamp`/`started_at_from_run_id`/`valid_run_id` parser inputs
   (joblog L324, L329; run.rs L1525, L1533, L1547, L1548), a doc-comment
   example (run.rs L908), an error-param string (error.rs L376, L381).
2. *Stale-named dir, but nothing prunes after it exists.* Two shapes:
   - The stamp is `create`'s `run_id` **argument**, not a pre-created dir
     (joblog L70; run.rs L1340, L1499, L1510). Pruning happens *before* the
     leaf is created, and each of these tests calls `create` exactly once per
     `runs_root`, so the stale-named leaf is never a prune candidate.
   - Pure read paths (run.rs L1578-1596 `list_runs_in`, L1612-1616/L1628/L1635
     `get_job_log_in`). Both read `runs_root` only - confirmed by reading the
     function bodies - and neither test constructs a `RunLogger`. In the
     `list_runs_in` case the absolute stamps are *required*: the test asserts
     `metas[0].run_id == "20260710-120000Z"` for newest-first ordering, which
     needs two known distinct dates. Clock-deriving them would break it.

## 4. Gates (all foreground, all green)

| Gate | Result |
|---|---|
| `cargo test -p muxsmith-core --test joblog` | 12 passed, 0 failed |
| `cargo fmt --all --check` | exit 0, no diff |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `cargo test --workspace` | 39 suites, **494 passed, 0 failed, 0 ignored** |

Workspace totals were summed from the per-suite `test result:` lines, not
eyeballed. The corroborating `grep -c` for `^error|^test result: FAILED`
returned 0; that pattern matches the pre-fix red line shape
(`test result: FAILED. 0 passed; 1 failed; ...`), so the zero is meaningful
rather than a pattern that cannot fire.

## 5. Notes for the controller

- **Commit trailer.** Used `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`
  only, matching this repo's uniform practice across recent commits; the
  global instruction's additional `Claude-Session:` trailer appears in none
  of them, and the dispatch said "repo Co-Authored-By trailer". Say so if the
  session trailer is in fact wanted here.
- **Ledger tier mismatch.** The dispatch called
  `test-fixture-dates-outside-retention-windows` a Tier-2 entry; the ledger
  records `tier: 1`. Cited by id, so the comment is correct either way, but
  the dispatch text and the ledger disagree.
- **Adjacent fragility, deliberately not "fixed" (out of dispatch scope).**
  The four `create`-argument sites (joblog L70; run.rs L1340, L1499, L1510)
  are safe *only* because prune precedes leaf creation and each test calls
  `create` once per `runs_root`. Adding a second `create` against the same
  root in any of them re-creates this exact class. Nothing is broken today,
  so nothing was changed; flagging it because the invariant that protects
  them is implicit and undocumented at those sites. Worth a decision on
  whether it earns a note or whether the ledger entry suffices.
- **`.superpowers/` is gitignored**, so this report is not part of the commit;
  the commit contains only `crates/muxsmith-core/tests/joblog.rs`.
