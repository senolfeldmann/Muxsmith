# Audit: core-45-yaml-fragment-serializer (PROMOTION candidate)

- **Cluster:** `core-45-yaml-fragment-serializer` (pattern · core · settled)
- **Claimed count:** 3 · **promoted:** true (promoted_at 3)
- **Statement under audit:** Render the suggestion `yaml_fragment` by serializing the real `MatchExpr` delta via `yaml_serde`, not by hand-formatting, so any value (colon/comma/brace, bool/int) round-trips. Bug D: inline code interpolated values unquoted, breaking the YAML the CLI prints verbatim; the serializer also fixed an unstated bool/int-stringification bug.
- **Verdict:** **CONFIRMED** — all 3 occurrences survive; promotion stands.
- **verified_count:** 3

This is a promotion candidate (would become a standing convention), so each cited recurrence was opened and confirmed to attest the *same (topic, approach)* — yaml_fragment rendered via `yaml_serde` serialization of the `MatchExpr` delta — and to be a distinct artifact, not a duplicate. The cluster shares its commit and its review file with the sibling `core-44` (no-clobber), so the central risk was cross-cluster misattribution; each occurrence was checked against the yaml topic specifically.

---

## Occurrence 1 — `independent review bug D` — violated-corrected

- **Artifact:** `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`
- **Supports the topic?** Yes.
  - Summary line 21: "D. `yaml_fragment` interpolates values unquoted -> a track_name with `:`/`,`/`{` emits broken YAML into the CLI-printed suggestion."
  - Confirmed finding line 70: "CONFIRMED planner.rs:837 - `yaml_fragment` interpolates string values with zero quoting/escaping -> colon/comma/brace in a track_name breaks the YAML the CLI prints verbatim (dry_run.rs:126). [D]"
- **Distinct / not misattributed?** Yes. The sibling no-clobber bug is a *different* labelled finding in the same review — bug **C** (line 20 / line 69: `with_rule_match` BTreeMap::extend). Bug D is unambiguously the YAML-fragment quoting defect, and it maps to `core-45`, not `core-44`. This is the independent *find* event (the violation identified). SURVIVES.

## Occurrence 2 — `F7 review (b)` — violated-corrected

- **Artifact:** `docs/process-journal/artifacts/plan-2-fixes-sdd/F7-review.md`, section **(b) `yaml_fragment` valid, round-trippable YAML** (lines 23-31).
- **Supports the topic?** Yes, and it substantiates the full statement including the unstated-bug clause:
  - Approach match (line 25): new `MatchFragment<'a>` wrapper; `yaml_fragment` now takes `&MatchExpr` (the actual simulated delta `cand.apply`) and serializes via `yaml_serde::to_string` — i.e. serialize the real delta, not hand-format.
  - Unstated bool/int bug (line 25): "This also fixes a second, unstated defect beyond quoting ... a boolean/int candidate (e.g. `forced_track: true`) would have rendered as the string `"true"` regardless; using `cand.apply` preserves the original `Scalar` variant." Matches the statement's "also fixed an unstated bool/int-stringification bug."
  - "Norway problem" / 40 adversarial inputs (lines 27, 29): independent round-trip probe over 40 adversarial values, all round-trip; the historical `serde_yaml` "Norway problem" class named explicitly. Matches the parenthetical.
- **Distinct / not misattributed?** Yes. The review is partitioned into (a) no-clobber → `core-44`, (b) yaml_fragment → `core-45`, (c) SuggestionsCapped → `core-34`. "(b)" is exactly the yaml section. This is the *review-of-fix* event, a separate attestation point from the find (occ 1) and the commit (occ 3). SURVIVES.

## Occurrence 3 — `commit 68ec6aa` — violated-corrected

- **Artifact:** `git show 68ec6aa` — `fix(core): suggestion engine no-clobber, valid YAML fragments, cap logging`.
- **Supports the topic?** Yes. Commit body: "`yaml_fragment` hand-formatted values into a YAML string with no quoting/escaping, producing invalid or wrong YAML for any value containing `:`/`,`/`{`/`}` ... Serialize the actual `MatchExpr` delta via `yaml_serde` instead, which quotes correctly per Scalar variant and guarantees a round-trippable fragment." Diff touches `planner.rs` (the `yaml_fragment` rewrite) and `tests/suggestions.rs`.
- **Distinct / not misattributed?** Yes. The commit bundles three fixes (no-clobber, yaml, cap). `core-45` cites it once, for the yaml fix, which the commit genuinely contains; the no-clobber portion belongs to `core-44`, the cap portion to `core-34`. This is the *fix/implementation* event. SURVIVES.

---

## Distinctness of the three (anti-inflation check)

The three occurrences are the framework's declared find → review → fix triple for one correctness bug (cluster-core.md "Find-vs-fix are two events", line 347), instantiated as three genuinely separate artifacts:

1. independent review (found bug D) — `independent-review-2026-07-09.md`
2. F7-review section (b) (reviewed the fix, incl. 40-input adversarial round-trip) — `F7-review.md`
3. commit 68ec6aa (implemented the fix) — git object

None collapses into another under the counting rule (no verdict-plus-journal double-cite; the commit is a separate touchpoint from its review by the rule's own statement). No occurrence is fabricated, none is misattributed to the sibling `core-44` no-clobber arc, and no two are the same artifact.

The shared use of commit 68ec6aa and `F7-review.md` across `core-44` and `core-45` is not inflation *within* `core-45`: the commit and the review each address two distinct `(topic, approach)` items (no-clobber vs. yaml-serializer) and are correctly partitioned by finding label (C vs D) and review subsection ((a) vs (b)).

**Result: 3 distinct occurrences survive. Verdict CONFIRMED — promotion to standing house convention stands.**
