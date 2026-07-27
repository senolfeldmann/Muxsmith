# Task 5 verdict - RIDER: ledger-lint duplicate-key extension + ci.yml wiring

**Spec compliance: APPROVED** (every brief step satisfied; two minors, none blocking)
**Task quality: APPROVED** (mechanism idiomatic, evidence reproduced byte-identical, boundary honestly recorded; two minors: one narrow behavioral regression, one incomplete sweep)

Reviewed: branch `plan8-d`, commit `92c62f1`, worktree
`/home/senol/Git/Muxsmith/.worktrees/plan8-d`. Graded against
`task-5-brief.md` + `implementer-preamble.md`; ground truth read: ROADMAP
"Ledger hygiene" (docs/ROADMAP.md:553-561 duplicate-key gap, :592-617
CI-wiring deferral incl. the S22 ruling), D83
(2026-07-22-plan8-packaging-release-design.md:715-770), ledger entries
`ci-additive-only-check-numstat`, `design-empirical-claims-reproducible`,
`deps-first-party-pinned-over-convenience`, `ci-10-pin-everything`,
`proc-noninteractive-file-ops-in-agents`.

Diff package note: the review package's per-file `--stat` figure (`51`) is the
changed-line total for `scripts/ledger-lint.py` (48 insertions + 3 deletions);
the authoritative branch numstat is `48 3` for the script and `17 0` for ci.yml,
65/3 combined, matching the report. `master` has advanced since the report was
written (three house-knowledge commits touching `docs/conventions.yaml` and
`docs/decision-ledger.yaml`, none touching ci.yml), so `git diff master` in the
worktree now lists four files; the ci.yml column is unaffected.

---

## Re-run evidence (all foreground, in the worktree, tree restored and verified after each)

| Claim | My result | Verdict |
| --- | --- | --- |
| RED, S21 fixture shape (doubled `steelman: null` at line 48, entry `proc-02-whole-branch-review`) | `FAIL docs/process-conventions.yaml: duplicate key 'steelman' (lines 48 and 49)` / `1 violation(s) across 452 entries` / exit 1 | byte-identical to the report |
| CONTROL, check 1 still live (`docs/conventions.yaml:32` `count: 3` -> `4`) | `FAIL docs/conventions.yaml: core-03-suggestion-verified-edit: count is 4 but has 3 occurrences` / exit 1 | byte-identical |
| GREEN baseline, four house files | `ledger-lint: 452 entries across 4 files, all invariants hold` / exit 0 | byte-identical (re-run 3x, incl. after every fixture restore) |
| Step 4, the job's exact step commands (`RUNNER_TEMP` venv + `pip install PyYAML==6.0.3`) | `Successfully installed PyYAML-6.0.3`, `452 entries ... all invariants hold`, exit 0 | reproduced; the pin also resolves from the live index today |
| Step 5, additive-only `git diff master --numstat -- .github/workflows/ci.yml` | `17	0	.github/workflows/ci.yml` | confirmed |
| Step 5 fire-verification (in-place edit of an existing line, `  deny:` line 156) | `18	1` -> restore -> `17	0`, `grep -c` residue `0`, `git status` empty | confirmed falsifiable |
| Non-regression of checks 2-5 (one combined fixture: `status: blocked` with `blocked_on: null`, `promoted_at: null` on a tier-2 entry, an occurrence stripped of `ref`, a second entry renamed to a colliding id) | all four fire with unchanged messages, plus check 1 and check 6 in the same run: 6 violations / `451 entries` / exit 1 | confirmed |
| Docstring sweep `grep -n "CI wiring" scripts/ledger-lint.py` | exactly one hit, line 30, the new sentence | confirmed |
| ci.yml parses; job set and shape | `['test', 'deny', 'ledger-lint']`; `ledger-lint` keys `['runs-on','steps']`, `runs-on: ubuntu-26.04`, no `needs`, checkout SHA identical to the `deny` job's house pin | confirmed |
| Report boundary claim: no anchors/aliases/merge keys in the four files | parser-level probe (`yaml.parse` events): `anchors=0 aliases=0 merge-keys=0` in all four; positive control file reports `anchors=3 aliases=2 merge-keys=1` | confirmed with a firing control (a plain grep for `&`/`*` is unusable here - it hits prose like `&&Track`) |
| Typography in both changed files | no em/en dash, smart quote, ellipsis, U+00A0, U+2212; positive control fires on a planted line | confirmed |
| Commit hygiene | single commit, exactly the two briefed files, 65/3, `%G?` = `N`, `Co-Authored-By: Claude Fable 5` trailer | confirmed |

Two claims the brief did not ask for, checked because the report asserts them:

- **Scope is per-mapping, not file-global** (the judgment asked of me): YES, and
  verified non-vacuously. The same fixture that fires
  `duplicate key 'kind' (lines 37 and 37)` inside ONE occurrence flow mapping
  leaves four sibling occurrence mappings that all carry `date`/`kind`/`ref`,
  and 452 entry mappings that all carry `id`/`kind`/`tier`/..., silent. The
  positive control and the negative live in the same run, so the absence cannot
  pass vacuously. Depth coverage is real: root-level duplicates fire too
  (three `meta:` keys at the document root -> `(lines 14 and 15)` and
  `(lines 14 and 16)`), and a key tripled yields one FAIL per extra occurrence,
  both anchored on the first - the class docstring's "collected, not raised" is
  accurate.
- **The extra fire-test's `451 entries`**, which reads like a slip, is honest:
  `total = len(seen_ids)` (scripts/ledger-lint.py:145) counts DISTINCT ids, so a
  duplicated id lowers the reported figure by one. My own duplicate-id fixture
  reproduced `451` independently.
- **The report's four `:line` citations all resolve** in the worktree state:
  `docs/decision-ledger.yaml:4068` -> `deps-first-party-pinned-over-convenience`,
  `docs/process-conventions.yaml:232` -> `ci-10-pin-everything`,
  `docs/process-conventions.yaml:408` -> `proc-verification-step-must-be-falsifiable`,
  `docs/decision-ledger.yaml:4140` -> `ci-additive-only-check-numstat`.

Restoration discipline (per `proc-noninteractive-file-ops-in-agents`): every
fixture was applied by a self-asserting `python3` heredoc, restored with pure
`git checkout --` (git is not aliased on this machine; `cp`/`rm` are, so backups
used `/usr/bin/cp -f` and cleanup `command rm -f`), and each restore was verified
with `git status --short` plus `diff -q` against a scratchpad backup. Final state:
`git status --short` empty, `python3 scripts/ledger-lint.py` green.

---

## Findings by severity

### Major: none.

### m1 - the loader constructor sits outside the parse `try`, so `ReaderError` escapes the handler

`scripts/ledger-lint.py:97-104`. `DuplicateKeyLoader(text)` is built one line
above the `try`. `yaml.reader.Reader.__init__` calls `check_printable(stream)`
for `str` input and raises `yaml.reader.ReaderError` - a `yaml.YAMLError`
subclass - from inside the constructor, i.e. outside the reach of
`except yaml.YAMLError`. `yaml.safe_load` wrapped that construction inside its
own call, so the old code caught it.

Reproduced (0x08 injected into `docs/product-boundaries.yaml` line 1, then
restored):

- master's script: `FAIL docs/product-boundaries.yaml: does not parse (unacceptable character #x0008: ...)` + `1 violation(s) across 420 entries`
- this commit's script: uncaught `yaml.reader.ReaderError` traceback, no `FAIL` line, no summary line

Scope of the regression, measured, not assumed: **ordinary** syntax errors still
report cleanly (`[unclosed: bracket` injected -> `does not parse (...)` +
summary + exit 1), and the control-char case still exits 1, so **CI stays red
and the gate does not fail open**. The cost is diagnostic quality plus a false
sentence in the report ("keeping the two error violations byte-identical to
before" - true for `FileNotFoundError` and for parser/scanner errors, false for
the reader class). It also nicks the brief's "checks 1-5 unchanged in behavior
and output" on the error path.

Fix (keeps `loader` reachable after the block for `duplicate_keys`):

```python
        loader = None
        try:
            loader = DuplicateKeyLoader(text)
            doc = loader.get_single_data()
        except yaml.YAMLError as exc:
            violations.append(f"{rel}: does not parse ({exc})")
            continue
        finally:
            if loader is not None:
                loader.dispose()
```

### m2 - the consuming-line sweep covered the script's check list but not ci.yml's job set

`BUILDING.md:92-96` reads: "CI (`.github/workflows/ci.yml`) runs the same
five-part Rust gate plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`, and
`pnpm test:e2e` on every push/PR (nine parts total); `cargo deny check` runs as
an independent job." After this commit there are two independent jobs and a red
ledger fails CI, which the file does not say; the documented nine-part local
gate also does not include `scripts/ledger-lint.py`, so a developer can pass the
full local gate and still get a red CI. Nothing in BUILDING.md becomes *false*;
the enumeration becomes incomplete, which is exactly the dependency the
sweep discipline exists to visit.

The report's self-review sweep looked only for places enumerating the script's
*checks* (correctly clearing the dated `docs/ROADMAP.md:626` line) and did not
look for places enumerating CI's *jobs*. Not fixable in-task: BUILDING.md is
stream A's file under the plan's file-disjointness cut
(`2026-07-23-plan-8-packaging-release.md:66`), so the owed action was an
escalation in Concerns next to the ROADMAP one it did raise. Verified that no
other wave-1 stream picks it up either: `grep -n ledger-lint */BUILDING.md`
across all six worktrees returns nothing. Controller/plan-close item.

### m3 - the new docstring sentence overstates the trigger scope

`scripts/ledger-lint.py:30-31`: "runs this script on every push and pull
request". ci.yml's trigger block is `push: branches: [master], tags: ['v*']` +
`pull_request` + `workflow_dispatch` (verified by parsing the file), so pushes
to non-master branches are not gated - the brief itself records the narrower
set ("ci.yml triggers only on master pushes, PRs and dispatch"). The sentence's
wording was the implementer's latitude, so this is on the task. Fix: "on every
master push, `v*` tag and pull request".

### n1 - the job comment's brew analogy is imperfect (brief-authored, no implementer latitude)

`.github/workflows/ci.yml:170-171`: "the interpreter floats with the runner
image (same recorded shape as brew above)". The brew case floats because
Homebrew offers no install-time version selector at all (ci.yml:49-53:
"the one manager where pinning is not idiomatically possible"); the interpreter
floats because a possible pin (`actions/setup-python`) was weighed and rejected
on `deps-first-party-pinned-over-convenience` grounds. Different reason, same
outcome. Recorded because the comment is house-knowledge-shaped text that a
future reader will cite; the block was prescribed verbatim by the brief, so it
is not a defect of this task's execution.

### n2 - docstring header not swept alongside item 6

`scripts/ledger-lint.py:11`: "Checks, per the ROADMAP spec plus duplicate-id
(silent shadowing)". Check 6 comes from the separate S21 duplicate-key ROADMAP
bullet, not the original four-check spec that clause refers to. Defensible as
written (both are ROADMAP-sourced); flagged only because the brief made the
docstring sweep an explicit step.

### n3 - the job's in-CI green remains unproven (informational, ruled by the brief)

`python3 -m venv` availability and a matching manylinux PyYAML wheel on the
preview `ubuntu-26.04` image are assumptions; the local Step-4 run used the
mise python 3.14.4 (cp314 wheel). The brief consciously routes the in-CI GREEN
to the controller's post-merge push observation, and the failure mode is loud
(first step fails, job red), not silent. No action in-task.

## Judgments the dispatch asked for, stated plainly

- **Per-entry / nested scoping, not file-global**: correct, and verified with a
  same-run positive control (see above). The implementation's scope (every
  mapping at any depth) is a genuine superset of the ROADMAP's "per-entry"
  requirement, and the ROADMAP's requirement is fully covered by it.
- **PyYAML pin vs house pin discipline**: consistent. `ci-10-pin-everything`
  ("standing preference is pin, not float"; "a floating version is a defect")
  is satisfied in the shape the house already uses for every package manager -
  exact install-time version, not a hash: apt `mkvtoolnix=97.0-1build1`, choco
  `--version=100.0.0`, npm save-exact, and now pip `PyYAML==6.0.3`. The runner
  image is pinned (`ubuntu-26.04`) as the policy block requires, and the action
  reuses the house `actions/checkout` SHA pin with its version comment. The
  unpinned interpreter is the one float, recorded in the job comment as the
  policy demands for a sanctioned float (n1 concerns only the wording of that
  record). The venv-over-`setup-python` choice is the first-party mechanism per
  `deps-first-party-pinned-over-convenience`, and it adds no new action pin.
- **D83 compatibility**: holds. The job is purely additive (`17 0`,
  fire-verified), adds no `permissions` key and so inherits the workflow-level
  `contents: read` that D83's separation rationale rests on, touches nothing on
  the release path, and reproduces the `deny`-job shape the brief names as
  house precedent. The intended `5 -> 6` coupling exists by construction: the
  job lives in ci.yml, so a red ledger drives the run conclusion the release
  guard polls.

---

## HARVEST

1. **Inlining a convenience loader inherits its exception surface.** Replacing
   `yaml.safe_load(text)` with an explicit `Loader(text)` + `get_single_data()`
   (to keep loader state alive) silently moves `Reader.__init__`'s
   `check_printable` outside the `try` the convenience call had around it, so a
   `YAMLError` subclass escapes a handler that used to catch it. Trigger is
   readable: you are unwrapping a one-call convenience API to reach its
   internals. Handle: everything the convenience call did inside its own
   try-block moves inside yours, construction included. Generalizes past PyYAML
   to every "inline the wrapper" refactor (a `requests.get` split into
   `Session()` + `send`, a `json.load` split into a decoder instance).
   Candidate: new `technical-code` entry, or an occurrence on an existing
   error-path entry.

2. **A change that adds a member to a set sweeps the enumerations of every set
   it joins, not just the one it obviously belongs to.** This task swept "who
   enumerates the script's checks" (found and correctly cleared ROADMAP:626)
   and missed "who enumerates CI's jobs" (BUILDING.md:92-96). Handle: list the
   sets the new thing became a member of (a check, a CI job, a gate part), one
   grep per set. Candidate: `reinforced` occurrence on
   `proc-normative-count-recomputed`'s sweep facet, or its own entry -
   controller's call.

3. **A file-disjoint stream cut converts a needed neighbour edit into a
   mandatory escalation, so the brief should name the neighbour.** Task 5's
   change made a BUILDING.md line incomplete, but BUILDING.md is stream A's
   file; the implementer had no rule prompting it to look for consuming lines in
   another stream's files and no permission to edit them. Handle for plan
   authoring: a task whose change alters an observable documented in another
   stream's file carries an explicit "escalate, do not edit" note naming the
   file. Neighbour of `plan-interfaces-absent-by-construction`.

4. **`ci-additive-only-check-numstat` reinforced, second independent run.** The
   numstat idiom stayed falsifiable in a reviewer's hands: `17 0` clean, `18 1`
   on an in-place edit of an existing line, back to `17 0` restored - no
   ambiguity, no exit-status games. Candidate `reinforced` occurrence.

5. **`proc-noninteractive-file-ops-in-agents` worked as written.** Five
   mutate-and-restore fire-tests in this review, alias-proof forms throughout
   (`/usr/bin/cp -f`, `command rm -f`, pure `git checkout --`), restoration
   verified per step against a scratchpad backup, zero blocked prompts and zero
   tool timeouts - against two `violated-corrected` occurrences from the same
   day. Candidate: first `reinforced` occurrence, which is what turns the entry
   from an incident record into a working handle.

6. **Reviewer form for a "scoped, not global" claim: put the positive control
   and the negative in the same fixture.** To test that duplicate-key detection
   is per-mapping, duplicate a key *inside one of the very mappings* whose
   sibling repetitions must stay silent. One run then proves both directions and
   the negative cannot pass vacuously (a broken check would report nothing at
   all). Instance of `design-empirical-claims-reproducible`'s
   verified-negative-with-positive-control applied to review work rather than a
   design doc.
