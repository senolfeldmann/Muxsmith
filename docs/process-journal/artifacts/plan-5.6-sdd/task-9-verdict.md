# Task 9 review verdict: skip-marker shared const (T2-m1)

**Task quality: Approved**

## Spec Compliance

- ✅ Zero behavior change: `MKVMERGE_SKIP_MARKER` is defined as `"mkvmerge not found; skipping"` byte-for-byte (`crates/muxsmith-core/src/lib.rs` diff), and every site was mechanically converted to `eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);`. Rust cannot inline-capture a path expression (`{muxsmith_core::MKVMERGE_SKIP_MARKER}` is invalid syntax), so the positional-arg form is the only valid one here, not a stylistic deviation. Output is identical to the original literal `eprintln!`.
- ✅ `#[doc(hidden)] pub const MKVMERGE_SKIP_MARKER` added at the `muxsmith-core` crate root (`crates/muxsmith-core/src/lib.rs`), consumed via fully-qualified path from `muxsmith-cli`, `muxsmith-core`'s own integration tests (a separate compilation unit under Cargo's `tests/` convention, so referencing it as an external crate is correct), and `muxsmith-gui`/`src-tauri`.
- ✅ (file:line) All current sites migrated. Authorized grep confirms it:
  ```
  grep -rn "mkvmerge not found; skipping" --include="*.rs" --include="*.yml" .
  crates/muxsmith-core/src/lib.rs:28:pub const MKVMERGE_SKIP_MARKER: &str = "mkvmerge not found; skipping";
  .github/workflows/ci.yml:106:          count="$(grep -c 'mkvmerge not found; skipping' gated-test-output.log || true)"
  ```
  Only the const definition and the ci.yml grep literal remain as raw strings, exactly as required. Independent count of `MKVMERGE_SKIP_MARKER` call sites: 21, matching the claimed 21 across the 8 files named in the report's table (dry_run_cli.rs 6, run_cli.rs 3, run_live.rs 2, command_integration.rs 3, mkvmerge_runtime.rs 2, executor_live.rs 1, identify_live.rs 1, src-tauri/src/lib.rs 3 = 21).
- ✅ `.github/workflows/ci.yml:106`: comment added directly above the grep line, tying the literal to the const by name and file path ("This literal must match muxsmith_core::MKVMERGE_SKIP_MARKER (crates/muxsmith-core/src/lib.rs) byte-for-byte"). Grep literal itself correctly left untouched (YAML can't reference a Rust const).
- ⚠️ Unsigned commit / explicit staging / ran directly on master (wave-2 serial): not independently verifiable from the diff file alone (no git commands run, per instructions); taken on the report's word.

## Strengths

- Uniform, mechanical replacement across all 21 sites with no stray leftover literal (grep-verified) and no reworded/drifted copy.
- Correctly reasoned that Cargo integration tests under `tests/` are separate compilation units consuming `muxsmith_core` as an external crate, so no new `use` lines or dependency edges were needed anywhere.
- ci.yml comment placed exactly where it is load-bearing (immediately above the grep it constrains).

## Issues

#### Important

- **Precedent misattribution in the report, feeding directly into the "harvest" step.** The report states: "This is the first case where that pattern crosses a crate boundary rather than just a file boundary within one crate." That is incorrect. `docs/decision-ledger.yaml` already carries this exact precedent:
  ```
  - id: core-90-go-public-gates
    kind: pattern
    tier: 1
    domain: core
    statement: "Pulled-forward go-public gate: ConcurrencyTracker (test instrumentation)
      is doc(hidden) from rustdoc but kept pub for cross-crate tests; static 3-OS
      matrix pulled forward too."
    occurrences:
      - {date: "2026-07-10", kind: decided, ref: "commit 7a2bc15"}
      - {date: "2026-07-10", kind: decided, ref: "journal Plan 5"}
  ```
  `ConcurrencyTracker` already used `#[doc(hidden)] pub` for cross-crate test access in Plan 5. Task 9 is a third occurrence of the same technique, not a first. This doesn't change whether the code is correct (it is), but it does change what should be harvested: the controller should log this task/commit as a further occurrence of `core-90-go-public-gates` rather than treat the report's proposed new `conventions.yaml` entry as a de-novo pattern. See "House dimension" below.

#### Minor

- The report's own crate-breakdown prose doesn't add up: "Crates: muxsmith-cli (14), muxsmith-core (7 across its own tests/ integration files), muxsmith-gui/src-tauri (3, ...)" sums to 24, not 21, and contradicts its own per-file table two paragraphs above (muxsmith-cli's three files sum to 6+3+2=11, not 14). The table and the grep-verified total (21) are both correct; only this summary sentence is wrong. No functional impact, but worth a one-line fix in the report if it's kept as a durable record.

## House dimension

- **Ground truth checked:** `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/product-boundaries.yaml` (named), plus `docs/decision-ledger.yaml` (Tier 1, consulted because it surfaced the directly on-point precedent below).
- `docs/conventions.yaml`'s existing `testing-support-helpers` entry (id, ~line 450) only covers same-crate consolidation into `tests/support/mod.rs`; it explicitly does not reach across crate boundaries, as the report itself notes.
- `docs/decision-ledger.yaml`'s `core-90-go-public-gates` (Tier 1, ~line 514) is the actual precedent for the cross-crate `#[doc(hidden)] pub` technique (ConcurrencyTracker, Plan 5, commit 7a2bc15). It is currently `promoted_at: null`, count 2, both occurrences dated 2026-07-10.
- `docs/product-boundaries.yaml`: no relevant entries (targeted grep for skip/marker/const/cross-crate returned nothing); this is a pure test-infra change with no product-scope surface, correctly out of that file's purview.
- `docs/process-conventions.yaml`: no directly relevant entry beyond the general SDD/gate process (`proc-01-sdd`), which the task followed correctly (full 9-part gate, wave-2 serial execution directly on master, unsigned commit, explicit staging, matching what the plan specified for this wave).
- **Harvest recommendation:** log this task (commit 616778d, 2026-07-13) as a third occurrence against `core-90-go-public-gates` in `decision-ledger.yaml`, and evaluate promotion to Tier 2 given the recurrence (three instances of the same underlying technique now recorded). If promoted, generalize the statement past its current "go-public gate" framing to the general rule ("a `#[doc(hidden)] pub` item in the crate every consumer already depends on is the resolution when per-crate `tests/support` can't span crate boundaries") and cross-reference it from `testing-support-helpers`, whose statement should gain a one-line pointer to the cross-crate sibling so the two don't read as contradictory in scope.

## Assessment

**Task quality:** Approved
**Reasoning:** All binding technical constraints are met and independently verified (byte-identical marker, doc(hidden) pub const location, all 21 sites migrated, ci.yml comment tying literal to const); the house-dimension finding is a precedent-attribution and report-accuracy issue for the controller's harvest step, not a code defect, so it doesn't block merge.
