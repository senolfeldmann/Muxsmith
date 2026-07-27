### Task 5: RIDER - ledger-lint duplicate-key extension + CI wiring

**Stream D** (`.worktrees/plan8-d`). Read the two ROADMAP "Ledger hygiene" rider rulings (2026-07-22 S22), `scripts/ledger-lint.py` in full (including its docstring's house-pattern note and its now-consumed CI-wiring deferral sentence), and D83's decision + rationale. Model tier: mid.

**Controller ruling (binding, recorded in the ROADMAP, not the design):** wire `scripts/ledger-lint.py` into CI AND extend it with the per-entry duplicate-key check - both deferral triggers fired 2026-07-22 (S22); one task.

**Scope adjudication (recorded here so no reviewer re-derives it):** D83 decides the RELEASE pipeline's placement - a separate `release.yml`, ci.yml untouched by the release path - and section 11 restates that for the design's own implementers ("ci.yml is not modified, at all"). The rider is not release-pipeline work and not design scope: the design itself records it as outside ("not a design amendment; nothing in the plan-8 design depends on it" - section 7 last bullet, section 10, and the ROADMAP ruling's own wording). Resolution: no collision - the rider adds one self-contained, additive job to ci.yml and changes no existing line (Step 5 verifies exactly that), while every design-scoped task remains bound by section 11's ban. A separate lint-workflow file was weighed and rejected: it would duplicate trigger/checkout boilerplate, fragment the gate surface, and forfeit the free coupling that makes a red ledger block a release - D83's gate-green check consumes ci.yml run CONCLUSIONS, so a `ledger-lint` job inside ci.yml gates the release path at zero extra wiring (this consequence is intended and is the `5 -> 6` dependency edge). The deny job is the house precedent for exactly this shape: a cheap, independent hygiene job inside ci.yml.

**Files:**
- Modify: `scripts/ledger-lint.py`
- Modify: `.github/workflows/ci.yml` (additive only: one appended job)

**Interfaces:**
- Consumes: the four house YAML files (`docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`, `docs/decision-ledger.yaml`), PyYAML (local install measured 6.0.3; CI venv pins the same).
- Produces: ledger-lint check 6 (duplicate keys) and the ci.yml `ledger-lint` job every future push/PR runs.

- [ ] **Step 1: Extend `scripts/ledger-lint.py` with check 6 - duplicate keys.** The contract (closed; the code shape within these bounds is the implementer's):

- **Detection mechanism**: a `yaml.SafeLoader` subclass that detects duplicate keys during mapping construction (PyYAML's documented `construct_mapping` extension point), used for the whole-file load. NOT a regex or line heuristic - the script's own docstring records the real-parser principle, and a linter that must be trusted keeps it.
- **Scope**: any mapping anywhere in any of the four files (top-level entry mappings and every nested mapping). This is a superset of the ROADMAP's "per-entry" requirement and is strictly simpler than entry-scoping the loader.
- **Violation format**: one `FAIL` line per duplicate, naming the file, the key, and both 1-based line numbers from the YAML node marks (e.g. `FAIL docs/process-conventions.yaml: duplicate key 'steelman' (lines 61 and 63)`); aggregated with the existing violations and the existing summary/exit semantics (exit 0 clean, 1 on any violation).
- **The observed S21 defect shape must be caught**: a doubled `steelman:` line inside one entry (Step 2's fixture reproduces it).
- **Docstring maintenance in the same edit** (the edit is the trigger; sweep the file): the numbered check list gains item 6, and the stale deferral sentence at the docstring's end ("CI wiring is a separate step ... rides the next CI-touching plan, per the ROADMAP") is replaced by one line stating the wiring exists (ci.yml `ledger-lint` job, Plan 8 rider). Verify the sweep: `grep -n "CI wiring" scripts/ledger-lint.py` afterwards shows only the new sentence.
- **Checks 1-5 unchanged** in behavior and output.

- [ ] **Step 2: Fire-test the extension** (foreground; record outputs in the task report):

```bash
# RED (new check): duplicate the steelman: line inside one entry of
# docs/process-conventions.yaml (the observed S21 shape), then:
python3 scripts/ledger-lint.py
# Expected: FAIL line naming the file, key 'steelman' and both line numbers; exit 1.
git checkout -- docs/process-conventions.yaml

# CONTROL (old checks still live after the loader swap): bump one entry's
# count field by 1 in docs/conventions.yaml, then:
python3 scripts/ledger-lint.py
# Expected: the existing "count is N but has M occurrences" FAIL; exit 1.
git checkout -- docs/conventions.yaml

# GREEN (reachable):
python3 scripts/ledger-lint.py
# Expected: "ledger-lint: <N> entries across 4 files, all invariants hold"; exit 0.
```

- [ ] **Step 3: Append the `ledger-lint` job to `.github/workflows/ci.yml`** - exactly this block, added after the `deny` job; no existing line is touched:

```yaml
  ledger-lint:
    # House-knowledge structural integrity (scripts/ledger-lint.py):
    # count==occurrences, refs present, blocked/tier fields, duplicate
    # ids, per-entry duplicate keys. Rider on Plan 8 by controller
    # ruling (ROADMAP "Ledger hygiene", 2026-07-22 S22) - additive job
    # only; D83's "ci.yml is not modified" is scoped to the release
    # pipeline and holds. PyYAML pinned; the interpreter floats with
    # the runner image (same recorded shape as brew above).
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
      - name: ledger-lint (house YAML invariants)
        run: |
          python3 -m venv "$RUNNER_TEMP/ledger-lint-venv"
          "$RUNNER_TEMP/ledger-lint-venv/bin/pip" install PyYAML==6.0.3
          "$RUNNER_TEMP/ledger-lint-venv/bin/python" scripts/ledger-lint.py
```

Pin rationale, recorded: PyYAML==6.0.3 is the registry-latest (pypi.org, verified 2026-07-23) and matches the local install the script already runs under. The venv is the first-party mechanism (`deps-first-party-pinned-over-convenience`); a pinned `actions/setup-python` was weighed and rejected - it adds an action pin whose only value here is an interpreter the image already ships, and the stdlib venv sidesteps PEP-668 system-python concerns without it. The runner label matches the file's existing jobs (ubuntu-26.04); no new runner family.

- [ ] **Step 4: Run the job's exact step commands locally** (green proof of the step, foreground):

```bash
RUNNER_TEMP="$(mktemp -d)" bash -c '
  python3 -m venv "$RUNNER_TEMP/ledger-lint-venv"
  "$RUNNER_TEMP/ledger-lint-venv/bin/pip" install PyYAML==6.0.3
  "$RUNNER_TEMP/ledger-lint-venv/bin/python" scripts/ledger-lint.py
'
# Expected: pinned install succeeds; "all invariants hold"; exit 0.
```

The in-CI red state is deliberately NOT exercised: ci.yml triggers only on master pushes, PRs and dispatch, so a red run would require pushing a broken ledger - the red is proven locally (Step 2) plus the platform's core semantic that a nonzero step exit fails the job; the in-CI GREEN is observed after the merge-order push (merge-order section above) as the wiring's green-reachable evidence.

- [ ] **Step 5: Verify the additive-only property** (the D83-compat observable; idiom per `ci-additive-only-check-numstat`, Tier-1 ledger):

```bash
git diff master --numstat -- .github/workflows/ci.yml
# Expected: exactly one line, "<added>  0  .github/workflows/ci.yml" - the
# deletions column is 0, so the diff is pure addition. (numstat counts a
# deleted blank line too, which a grep over unified '-' lines misses, and
# its exit status stays clean either way - the ledger entry's ground.)
# Fire-verify: temporarily edit one existing ci.yml line in the working
# copy, see the deletions column go nonzero (an in-place edit counts as
# 1 deletion + 1 addition), restore.
```

- [ ] **Step 6: Commit**

```bash
git add scripts/ledger-lint.py .github/workflows/ci.yml
git -c commit.gpgsign=false commit -m "ci: ledger-lint job (Plan 8 rider, S22 ruling) + per-entry duplicate-key check via SafeLoader subclass; fixture fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Wave 2

One task, on master, after every wave-1 stream is merged, gated, and pushed. It writes no repo file.

---

