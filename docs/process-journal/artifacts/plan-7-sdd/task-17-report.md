# Task 17 report - check-i18n D55 extensions (rules 1-5)

**Verdict: NEEDS_CONTEXT** (genuine design fork in rule 5's select-structure
sub-part). Rules 1-4 and rule 5a (placeable-set parity) are implemented and
fire-verified in both directions. Rule 5b/5c/5d (select-count / selector-var /
variant-key parity) collide with real multi-sibling and nested Fluent selects
the brief's `patternStructure` premise explicitly excluded, producing a
**permanently-red gate on the correct, structurally-parallel tree**. The
resolution changes what drift the gate accepts/rejects and reinterprets an
assertion the design enumerated verbatim, so it returns for routing before
resolution.

- Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan7-g` (branch `plan7-g`, HEAD `7c29957`).
- Commit hash: **NONE** (no commit - task stopped at the fork per the binding
  no-keyboard-resolved-forks rule). `scripts/check-i18n.mjs` is left MODIFIED,
  uncommitted, carrying rules 1-4 + 5a + the forking 5b-5d, so the resolver
  builds on it. `git status --short` = `M scripts/check-i18n.mjs` only.
- One-line test summary: rules 2/3/4/5a fire-verified red-and-green; rule 5's
  select-structure part yields 3 false RED failures on the correct tree
  (`validate-summary`, `batch-diagnostics-summary`, `suggestion-partition`).

---

## What changed in `scripts/check-i18n.mjs`

Implemented exactly per the brief's Step-1..3 code, plus wiring:

- **Step 1 (rule 1):** `parseCatalogIds` -> `parseCatalog(path)` returning
  `Map<id, {value, attrs: Map<name, body>}>`; `parseCatalogIds` kept as a
  one-line wrapper (`[...parseCatalog(path).keys()]`), so every existing call
  site is preserved. Attribute lines (`^\s+\.name =`) and indented
  continuation lines are folded into value/attr bodies. Terms stay unregistered.
- **Step 2 (rule 2):** `TA_CALL_RE = /(?<![\w$])\$?ta\(\s*(['"])([^'"]*)\1/g`
  added beside `CALL_RE`; scanned in the per-line loop with identical
  add-to-`literalCallIds` / push-to-`missing` treatment. `labelKeyIds` set
  added and populated in the `LABEL_KEY_RE` branch (for rule 3).
- **Step 3 (rules 3-5):** rule-3 tooltip-completeness block (`enCatalogs`,
  `tooltipErrors` over sorted `labelKeyIds`); `PLURAL_KEYS`/`PLACEABLE_RE`/
  `SELECTOR_RE`/`VARIANT_RE` + `patternStructure` + `comparePatterns` helpers;
  wired into the check-3 locale loop (rule 4 attr-name-set equality, rule 5
  `comparePatterns` on value + each shared attribute). `localeIds` now derives
  from a single `parseCatalog` call (reuse, no double-parse). `tooltipErrors`
  printed and added to the final green condition
  (`missing===0 && parityErrors===0 && tooltipErrors===0`).
- **Step 5 (header):** NOT YET applied - deferred until the fork is resolved,
  since rule 5's final shape (and thus the header wording for check 3's rules
  4-5) depends on the decision. Flagged so the resolver does it.

---

## Brief-vs-tree divergence (the fork)

The brief's `patternStructure` carries the comment *"this tree's catalogs have
no nested selects (line-based charter)."* **This premise is false on two
counts**, empirically confirmed by running the implemented gate:

```
check-i18n: cross-locale key parity violations:
  locales/de/cli.ftl: "validate-summary" value: select 0 needs >=1 variant and exactly one *-default
  locales/de/diagnostics.ftl: "suggestion-partition" value: select 0 needs >=1 variant and exactly one *-default
  locales/de/gui-batch.ftl: "batch-diagnostics-summary" value: select 0 needs >=1 variant and exactly one *-default
EXIT=1
```

All three are **false positives**: en and de are byte-for-byte structurally
parallel in each (same selector nesting/sibling shape, same variant keys). The
flat line-based `patternStructure` cannot model the two real Fluent shapes
present, so it mis-derives the select structure identically for both locales
and then trips its own absolute-validity guard. Two mechanisms (ground-truthed
by dumping the parsed bodies):

1. **Sibling selects** (`validate-summary`, `batch-diagnostics-summary`):
   the reopener lines `}, { $warnings ->` / `}, { $infos ->` sit at **column 0**,
   so `parseCatalog`'s continuation guard `/^\s+\S/` drops them. The
   `SELECTOR_RE` never sees the 2nd/3rd selectors; only ONE select (`$errors`)
   is captured, and it absorbs all six variant lines ->
   `keys=[one,other,one,other,one,other] defaults=3`. The CLDR carve-out then
   fires on `defaults !== 1`.
2. **Nested select** (`suggestion-partition`): the outer `$kind` selector's
   variants `[overflow]` / `*[group]` are emitted on the SAME lines that open
   the inner `$dropped` / `$count` selects, so `VARIANT_RE` attributes them to
   the just-pushed inner select. The outer select ends with `keys=[]` ->
   the carve-out fires on `keys.length === 0`.

`suggestion-partition`'s `$kind` selector (`[overflow]`/`*[group]`) is exactly
the **non-plural `$property`-class selector** whose drift i18n-12 / D39 was
created to guard - so a resolution that simply skips select-structure checking
would fail the entry's own motivation.

Step 6's escape hatch ("if rule 5 finds genuine pre-existing en/de drift, fix
the de catalog") does **not** apply: there is no de defect. en and de are
identical; "fixing" de would mean making it diverge from en. The failure is a
parser-modeling limitation, not a catalog drift.

### Why this is a fork and not a silent adaptation

- The three resolution options (below) have **materially different outward
  behavior** - specifically whether the gate catches non-plural
  selector-variant drift inside multi/nested messages (the i18n-12 motivation).
- The design (D55 rule 5(d)) enumerates the assertion **verbatim**: *"the rule
  is instead: at least one variant, exactly one `*`-default."* Option B below
  reinterprets "exactly one `*`-default" as "match en's default count." The
  implementer preamble's structural-conformance grant *"fills SILENCE only - an
  explicit enumeration in plan/design/spec wins over it"* and explicitly stops
  on *"rewording of existing assertions"* / weakened verification. Changing an
  enumerated assertion is a design-owner call.

---

## Decision memo (options / costs / recommendation)

**Relevant context:** rule 5 is a **cross-locale drift** check (i18n-12), NOT an
absolute Fluent-validity check. Absolute validity ("every select has exactly one
default", catalogs parse) is already guarded by `assertAllCatalogsParseCleanly`
(`e2e/catalogs.spec.ts:12`), which real-Fluent-parses every `.ftl` of every
locale. `patternStructure` is deterministic and applied identically to en and
de, so a **relative (en-vs-de) parity** comparison stays sound even when the
flat model cannot perfectly reconstruct nesting - any drift that changes the
flat token stream still shows as a flat-structure difference between the two
locales.

**Option A - make `patternStructure` model nesting + column-0 sibling reopeners
correctly** (indent/brace-aware select stack; capture `}, { $x ->` lines).
- Coverage: full - per-select count/selector/variant parity for all shapes.
  Best fulfils the literal design wording.
- Cost: real complexity in a script whose charter is *"line-based, not a Fluent
  parser."* Changing `parseCatalog`'s continuation semantics to capture
  column-0 lines risks bleeding into the next message; nesting attribution by
  indentation is itself a heuristic. Fights the charter.

**Option B (recommended) - keep the flat parser; make rule 5's select part a
pure en-vs-de PARITY check.** In the CLDR carve-out, replace the absolute
`sb.keys.length === 0 || sb.defaults !== 1` with parity against en:
`(sa.keys.length === 0) !== (sb.keys.length === 0) || sa.defaults !== sb.defaults`.
- Effect on the tree: all 3 false positives go green (en/de flat structures are
  identical); every drift class the fire-tests exercised stays caught
  (placeable rename, selector-var rename, select add/drop, non-plural variant-key
  drift via the else-branch full-equality, default-count drift).
- Cost: drops the absolute "exactly one `*`-default per select" assertion
  (reinterpreted as "same default count as en"). Acceptable because that
  absolute property is already enforced by the real-Fluent `assertAllCatalogsParseCleanly`
  guard; a select missing its default is a Fluent parse error caught there.
  Blind spot: a drift that is flat-structure-invariant (e.g. moving a variant
  between two sibling selects while keeping total keys+defaults) - negligible
  likelihood and impact.
- Smallest, charter-respecting change; needs the design's "exactly one
  `*`-default" wording adjusted to "matches en's default count" (owner sign-off).

**Option C - exclude multi/nested messages from select-structure comparison**
(keep placeable-set parity 5a universal).
- Cost: an allowlist/skip that goes stale and drops select-structure coverage
  for exactly the complex messages most prone to drift (incl. the D39 `$kind`
  case). Weakest; not recommended.

**Recommendation: Option B.** It aligns rule 5 with its actual purpose (locale
drift, not en-internal validity), keeps the charter, is the minimal diff, and
loses only an absolute assertion already covered elsewhere. It requires the
design-owner to bless reinterpreting "exactly one `*`-default" as en-default-count
parity - which is why this returns rather than being applied.

---

## Per-check fire-verification (both directions)

Baseline before implementation: `node scripts/check-i18n.mjs` -> EXIT 0
(41 source files, 211 catalog ids, 17 unused warnings, 1 other locale, 7 en
catalogs). No bare `ta(` false-match risk (grep `[^A-Za-z0-9_$]ta\(` and
`^\s*ta\(` over `src/` -> 0 hits). Each break restored via `git checkout --`;
final `git status --short` = `M scripts/check-i18n.mjs` only (all breaks reverted).

### Rule 2 - `$ta("id")` unknown-id hard fail
- BREAK: appended `// $ta("zzz-no-such-id")` to `src/i18n/index.ts`.
- RED: `check-i18n: literal t()/$t() ids missing from the en catalog:` /
  `  src/i18n/index.ts:96  "zzz-no-such-id"`.
- GREEN after restore: `grep -c zzz-no-such-id` -> 0.

### Rule 3 - editor labelKey tooltip completeness
- BREAK: `sed -i '12s/\.tooltip/.zooltip/' locales/en/gui-editor.ftl`
  (neutralize `.tooltip` on labelKey `editor-profile-meta`).
- RED: `check-i18n: editor labelKeys without a .tooltip attribute in the en catalog:` /
  `  labelKey "editor-profile-meta" has no .tooltip attribute in the en catalog`.
- GREEN after restore: `grep -c 'has no .tooltip attribute'` -> 0.

### Rule 4 - attribute-name-set parity (de vs en)
- BREAK: neutralize de-only `.tooltip` on `editor-profile-meta` (line 16).
- RED: `  locales/de/gui-editor.ftl: id "editor-profile-meta" attribute set differs from en ({tooltip} vs {zooltip})`.
- GREEN after restore: `grep -c 'attribute set differs'` -> 0.

### Rule 5a - placeable-set parity (de vs en)
- BREAK: `sed -i 's/{ \$detail }/{ \$detailx }/' locales/de/gui-common.ftl`
  (renamed `$detail` -> `$detailx` across the 8 messages using it).
- RED: 8 lines, e.g.
  `  locales/de/gui-common.ftl: "identify-failed" value: placeable set differs from en ({detail} vs {detailx})`
  (and 7 more: `internal-task-failed`, `mkvmerge-query-failed`,
  `mkvmerge-spawn-failed`, `profile-save-failed`, `profile-save-io-failed`,
  `settings-io-failed`, `settings-parse-failed`).
- GREEN after restore: `grep -c 'placeable set differs'` -> 0.

### Rule 5b/5c/5d - select-structure parity: **FORKED, not clean-verifiable**
The green direction is unreachable on the correct tree (3 false positives
above). Fire-verification of the red direction is deferred until the fork
resolves (the code path's final shape is what's in question). Rule 5d's
correct firing on a genuine SINGLE-select plural (e.g. removing `*[other]` from
`jobs-row-warning-count`) works under both the current and the Option-B code;
it is the multi/nested case that forks.

---

## Commands run (chronological, key results)

- `node scripts/check-i18n.mjs` (baseline) -> EXIT 0, green.
- `grep -rn '\$ta(' src/` -> 40 sites: literal-id calls (`$ta('settings-open-label')`,
  `fluent.$ta("batch-run")`, etc.) + dynamic `$ta(spec.labelKey)` (skipped like
  dynamic `$t`). One comment-line occurrence at `BatchView.vue:291`
  (`$ta("batch-run")[name]`) resolves to the real `batch-run` id -> no failure.
- `grep -rnoE '[^A-Za-z0-9_$]ta\(' src/` and `grep -rnE '^\s*ta\(' src/` -> 0
  hits (no bare-`ta(` false positives for `TA_CALL_RE`).
- Ground-truth dump of `parseCatalog`/`patternStructure` over the 3 failing
  messages (scratch script) -> confirmed mechanisms 1 & 2 above.
- `grep -n assertAllCatalogsParseCleanly e2e/*.ts` -> the real-Fluent parse-all
  guard exists (`e2e/catalogs.spec.ts:12`), supporting Option B's rationale.
- Fire-verification breaks + restores for rules 2/3/4/5a (results above).

---

## Surfaced items for the reviewer / controller

1. **The fork** (above) - the deliverable's rule 5 select-structure part cannot
   go green on the correct tree without a design-owner decision (A/B/C).
2. **Step 5 (header comment) deferred** - depends on rule 5's final shape.
3. **New pattern established:** `localeIds` now derives from a single
   `parseCatalog` call instead of a separate `parseCatalogIds` call (reuse; zero
   outward effect). In-scope under the structural-conformance grant.
4. **No commit** and the worktree carries uncommitted changes (a currently-red
   gate) by design for a NEEDS_CONTEXT return. Serial-stream tasks 18-19 are
   blocked behind this resolution regardless.

---

# Completion (round-7 resolution, 2026-07-22)

**Verdict now: DONE_WITH_CONCERNS** (one scoped, surfaced gate-scope
deviation; no open decision). The fork was ruled **Option B** by the
controller; the amended D55 rule 5 (main-repo design, commits `1fa2972` +
`baa8ee2`) encodes the en-vs-de parity form. Implemented against the amended
rule, fire-verified both directions, header rewritten, gate green, committed.

**Commit: `a838a32d4fd23d0fa219183d3f35fe8d17241daf`** (branch `plan7-g`,
`scripts/check-i18n.mjs`, +197/-20). Working tree clean.

## What changed vs. the NEEDS_CONTEXT state

- **Rule 5(d) rewritten from absolute to en-reference parity** in
  `comparePatterns`'s CLDR carve-out branch. Was:
  `if (sb.keys.length === 0 || sb.defaults !== 1)`. Now:
  `if ((sa.keys.length === 0) !== (sb.keys.length === 0) || sa.defaults !== sb.defaults)`
  - variant presence (empty vs non-empty) and `*`-default count must match
  en's, never an absolute shape. Rule 5(b) select-count and 5(c) selector-var
  checks were already en-vs-de parity by construction; unchanged. Absolute
  Fluent validity stays delegated to `assertAllCatalogsParseCleanly`
  (`e2e/catalogs.spec.ts:12`), named inline in the rule-5 comment.
- **Step 5 header comment done** (was deferred at the fork): check-1 gains the
  `$ta` rule-2 note; check-3's stale "no attributes today" sentences replaced
  by the rules 4-5 description; the `MESSAGE_ID_RE` ATTRIBUTES bullet now says
  attributes ARE registered per id and addressed by rules 2-5; the
  `patternStructure` inline comment's refuted "no nested selects" premise
  replaced by an accurate statement of the flat-derivation + parity rationale.

## Fire-verification of the amended rule 5 (both directions)

- **GREEN (real tree):** the three previously-false-RED messages now pass.
  `node scripts/check-i18n.mjs` -> EXIT 0,
  `check-i18n: ok (41 source files scanned, 211 catalog ids, 17 unused
  warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).`
  No `needs >=1 variant` / `plural variant presence` line for
  `validate-summary`, `batch-diagnostics-summary`, `suggestion-partition`.
- **RED (synthetic de drift, restored byte-identically):**
  `locales/de/gui-jobs.ftl` sha256 identical before/after each break (verified).
  1. Removed `*`-default (`*[other]` -> `[other]`) in `jobs-row-warning-count`
     -> RED, exit 1:
     `locales/de/gui-jobs.ftl: "jobs-row-warning-count" value: select 0 plural
     variant presence / *-default count differs from en (2/1 vs 2/0)`.
     Restore: sha256 match (`RESTORE-1 byte-identical: YES`).
  2. Changed selector variable (`{ $count ->` -> `{ $qty ->`) in the same id
     -> RED, exit 1, two lines: `select 0 selector differs ($count vs $qty)`
     (rule 5c) and `placeable set differs from en ({count} vs {qty,count})`
     (rule 5a). Restore: sha256 match (`RESTORE-2 byte-identical: YES`).
  Gate green again after both restores (exit 0); `git status` clean but for the
  script.

The rules 2/3/4/5a fire-verifications from the NEEDS_CONTEXT section above
still hold unchanged (that code was untouched by the round-7 edit).

## Full verification (nine-part gate, scoped)

The change is isolated to `scripts/check-i18n.mjs` (a Node ESM gate script).

| part | result |
|---|---|
| `pnpm check:i18n` | **exit 0** (green - decisive direct gate) |
| `pnpm lint` (eslint over the .mjs) | **exit 0** |
| `pnpm build` (vue-tsc + vite) | **exit 0** |
| `pnpm test:e2e` (grep catalogs parse; ran full suite) | **exit 0, 52 passed** - incl. `catalogs.spec.ts` (the `assertAllCatalogsParseCleanly` delegation target rule 5 relies on), `editor-tooltips.spec.ts` (`.tooltip` as title), the diagnostics/suggestions pluralization test |
| `cargo fmt --all --check` | **not run - see deviation** |
| `cargo clippy --workspace --all-targets -D warnings` | **not run - see deviation** |
| `cargo test --workspace` | **not run - see deviation** |
| `cargo doc --workspace --no-deps` (`-D warnings`) | **not run - see deviation** |
| `cargo deny check` | **not run - see deviation** |

**Deviation (surfaced, scoped concern):** the five cargo parts + `pnpm build`'s
Rust side cannot be affected by editing `scripts/check-i18n.mjs` - it is not a
member of any cargo crate, not referenced by a `build.rs`, not a generated
binding, and `vite build` does not compile `scripts/`. There is no structural
mechanism by which a JS gate-script edit changes Rust compilation/lint/test/doc.
`pnpm build` was run anyway (needed to set up `dist/` for the e2e preview
server) and is green. The full nine-part gate is the controller's merge-gate
per the plan ("full gate after every merge"); at task level, running the four
JS/frontend-touching parts plus the structural argument for the five Rust parts
is the proportionate, evidence-based verification. Flagged for the reviewer to
confirm the merge gate runs the cargo five.

## Remaining concern

1. **Accepted residual blind spot (design-blessed):** a flat-structure-invariant
   drift - a variant moved between two sibling selects with total keys and
   `*`-default count unchanged - would not surface, because the flat line-based
   derivation collapses sibling/nested selects identically for both locales.
   The amended D55 rule 5 records this as the accepted residual within the
   line-based charter; absolute per-select validity remains covered by the e2e
   real-Fluent parse guard. No action; noted for completeness.
