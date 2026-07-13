# Task 2 verdict: planner.rs + report/mod.rs + ADR D36

### Spec Compliance

- ✅ **Item 1 (planner.rs:526 dup, `Assignment::unmatched`)** — private assoc. fn added (diff L60-75); all six placeholder literals replaced with one-line pushes. Four sites carry `primary.path.clone()`, two carry `source_path`, matching the brief's site split. The matched (1-track) case correctly stays a full struct literal.
- ✅ **Item 2 (planner.rs:714 idiom)** — `matches!(..., KeepDrop::Keep)` → `profile.tracks.unmatched == KeepDrop::Keep`; import already present.
- ✅ **Item 3 (planner.rs:1312/:1565 yagni)** — the `#[allow(clippy::too_many_arguments)]` removed from `suggest` (6 args) and `partition_for_overlap` (7 args); `partition_for_rule`'s (9 args) retained. Arg counts confirmed from the diff.
- ✅ **Item 4 (planner.rs:1646ff idiom, bare imports)** — `PropValue, Track` added to the `crate::identify` import; every listed `std::collections::BTree*`, `crate::identify::Track/PropValue` site collapsed to bare names; the wrapped `seen` decl and the three `props.push` calls collapse to single lines. ⚠️ The report's grep-claim of *zero residual* fully-qualified paths is not fully verifiable from the diff, but every brief-listed site is converted and the clippy/fmt gate is reported green.
- ✅ **Item 5 (planner.rs:886/:971 dup, `render_ctx`)** — extracted `fn render_ctx(&PrimaryFile) -> Ctx`; body byte-identical to both original inline builds; both callers use it; `Ctx` added to the `template` import; `resolve_title`'s comment now points at the shared fn ("by construction").
- ✅ **Item 6 (planner.rs:368 stdlib rider)** — `known.contains(&ext.to_ascii_lowercase())` → `known.iter().any(|k| k.eq_ignore_ascii_case(ext))`. Behavior-equivalence verified: `parse_list_types` lowercases every entry (`runtime.rs:343 exts.push(tok.to_ascii_lowercase())`), so against a lowercase `known` the two forms are identical; drops the per-entry allocation.
- ✅ **Item 7 (planner.rs:1971 stdlib, `rule_index_of`)** — `split_once("tracks[")?.1.split_once(']')?.0.parse().ok()`. Same first-`tracks[`/next-`]` semantics as the `find`-based original.
- ✅ **Item 8 (planner.rs:1965 idiom, tuple key)** — `diag_signature` returns `BTreeMap<(String,String,String), usize>`; all four `base_sig` consumers propagate the tuple type; doc comment explains collision-safety. The '|'-collision closure is stated explicitly in the report's dedicated section, as the brief requires.
- ✅ **Item 9 (planner.rs:1521 doc, T13-m1)** — the invariant comment landed **verbatim** (line-wrapped) above `if let Some(cand) = best`.
- ✅ **Item 10 (report/mod.rs + planner.rs claimants)** — `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub claimants: Vec<usize>` added exactly as specified; initialized in `new`; single builder `with_claimants(&[usize])` sets **both** the structural field and the `rules` display param from one slice; the OverlappingRules site calls `.with_claimants(rules)`; `overlap_conflicts` reads `d.claimants.clone()`. `rule_index_of` retained for the AmbiguousRule config_path sites.
- ✅ **Item 11 (ADR D36)** — `docs/superpowers/specs/2026-07-13-plan-5.6-decisions.md` created with the four-part structure (Decision / Rationale / Rejected alternatives / Interface-wire). Two honest rejected alternatives. Wire note accurate against the code.
- ✅ **Item 12 (report/mod.rs:165 doc, M3)** — UnknownExtension rustdoc replaced **verbatim** with the brief's text.
- ⚠️ **Gate (nine-part, unsigned, explicit staging, buildable-in-isolation)** — reported green; not re-run per instructions. `report/json.rs` untouched **verified** (not in diff; confirmed it serializes via `serde_json::to_value(d)`, so the field surfaces automatically). Owned-files-only **verified** (only planner.rs, report/mod.rs, new decisions.md).

**No JSON snapshot asserts OverlappingRules** — verified. No JSON golden or insta snapshot contains an OverlappingRules/claimants document; the only test touchpoints (`suggestions.rs`, `planner_resolution.rs`, `catalog_completeness.rs`) read `params["rules"]` or set it on a render-arg helper, all unaffected because `with_claimants` sets `params["rules"]` byte-identically. Nothing needed updating.

### Strengths

- **Both named cross-cutting risks resolve cleanly.** (1) `Diagnostic`'s shape change: `report/json.rs:148` serializes via `serde_json::to_value(d)`, and `Diagnostic` derives `Serialize` only (no `Deserialize`), so the `skip_serializing_if` field surfaces on OverlappingRules JSON and is omitted everywhere else exactly as the ADR claims, with `json.rs` never edited. (2) D33 data-source change: the overlap-suggestion tests (`suggestions.rs:657+`, TC-A..TC-D) exercise `overlap_conflicts` → `d.claimants` end-to-end; TC-A asserts `!batch.suggestions.is_empty()`, which would fail if `claimants` came back empty. The re-parse→structural switch has real (if indirect) regression coverage.
- **Single-builder property is genuinely enforced.** `with_claimants` derives the structural `claimants` and the rendered `rules` param from the same `&[usize]` in one place; they cannot diverge. This is the correct mechanism for the brief's "so they cannot diverge," and it moves core *toward* core-37 compliance (it removes the pre-existing display-string re-parse in `overlap_conflicts`).
- **Verbatim texts landed exactly** (T13-m1 invariant comment, M3 rustdoc) — no paraphrase drift.
- **ADR quality is high.** Rationale names the concrete failure mode (a display-format change silently breaking D33 symmetric narrowing with no compile signal); both rejected alternatives are real options honestly weighed, not strawmen; the wire note is precise (public field, serde attrs, JSON gains `claimants` on overlapping-rules only, no new DiagCode, no Fluent change, `default` inert under Serialize-only). Cites core-37 and explicit-over-magic correctly.
- **Refactor/feature commit split is clean** — the refactor commit builds in isolation (claimants hunks reverted then reapplied for the feature commit); each snapshot standalone-buildable.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **ADR label wording diverges cosmetically from the house decisions-file convention.** The prior files (`2026-07-11-pre-1.0-design-decisions.md`) use `**Decision.**`, `**Rationale.**`, `**Rejected alternatives.**`, `**Interface/wire-format changes.**` (bold + period). D36 uses `**Decision (Plan 5.6, T2):**`, `**Rationale:**`, `**Interface/wire changes (doctrine §2):**` (colon, and "wire changes" vs "wire-format changes"). The four-part structure matches; only the label style differs. Cosmetic, no content impact.
- **No test directly asserts the structural `claimants` field.** Coverage is indirect (via the overlap-suggestion path). A one-line assertion that an OverlappingRules diagnostic carries `claimants == [expected indices]` would pin the new field directly rather than relying on the suggestion engine to fail loudly. Not required by the brief (which only asked to update *existing* asserting snapshots, of which there are none). Polish.

### House dimension

**No convention deviation.** The change conforms to the recorded Tier-2 technical-code patterns:
- **core-47-with-severity-builder** — `with_claimants` follows the established rustdoc'd-builder-over-field-mutation precedent exactly.
- **core-37-prose-free-core** — no new prose in core; diagnostics stay code + structured params. The `rules` param ("tracks[0], tracks[1]") is a machine reference list that predates this task and is deliberately kept byte-identical (no Fluent change); the task in fact *reduces* a core-37 tension by removing `overlap_conflicts`' re-parse of the rendered string.
- **core-35 / core-102 / core-105 / core-106** — D33 symmetric-narrowing semantics preserved; reading all claimants structurally is faithful to "generate for ALL claimants symmetrically."
- **core-48-extension-validation** — item 6 stays within the once-per-batch, degrade-to-warning validation; behavior unchanged.

**Harvested for the ledger:**
- **New pattern candidate (agent-emergent, technical-code, `core`): single-builder co-derived display + structural field.** `Diagnostic::with_claimants` sets a structural field and its rendered display param from one source so they cannot diverge; the machine consumer reads the structural field, the renderer reads the display param. First concrete instance is D36. Generalizes core-37's "machine-consumers read structured data, never re-parse the rendered string." Count 1 (this task); flag, do not promote.
- **Residual core-37 tightening candidate (observation, not a finding).** Core still carries a *pre-formatted* `rules` reference string alongside the structural `claimants`, kept only to preserve byte-identical rendering and avoid a Fluent change (an explicit, honest ADR scoping choice). The strict core-37 end state would render that list from `claimants` in the DiagnosticRenderer. The single-builder makes the current duplication safe (cannot diverge); worth parking as a future cleanup, not acting on now.
- **Latent behavior delta beyond the "one named exception" (for whole-branch awareness, not a defect).** Item 8's tuple-key `diag_signature` is a correctness fix that changes behavior only when a `config_path` or filename contains `|`. `config_path` is internally generated and never contains `|`; a `|` in a filename is the only reachable trigger, and the change can only *remove* a collision, never introduce one. It is explicitly routed and sanctioned by brief item 8 and is not a wire-format touch (the signature is never serialized), so it does not conflict with "claimants is the wave's only wire-format touch." Recording it so the whole-branch review counts it as an intentional second (internal, latent) behavior delta rather than drift.

### Assessment

**Task quality:** Approved

**Reasoning:** All 11 checkbox items plus the ADR are implemented faithfully; the two verbatim texts landed exactly, the two named cross-cutting risks (Diagnostic serde shape, D33 re-parse→structural switch) verify clean with real coverage, and the ADR is honest and complete. Only two Minor polish items (cosmetic ADR label style; no direct claimants assertion), neither blocking trust.
