# Plan 10 plan review, round 1

**Verdict: NEEDS FIXES**

Artifact: `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md` at
`da60634` (558 lines, five tasks). Reviewer scratch path for every probe built
in this pass, none of it inside the repo:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-r1-indep/`
(`corpus_probe.py`, `idcheck.py`, `dumpentry.py`, `dashctl.txt`). No instrument
the plan author left behind was re-run; every count below was recomputed with a
Python re-implementation rather than the author's shell pipeline, and the two
vendor refutations were re-fetched from the vendor's own source, not from the
plan's quotation of it.

**Coverage is complete. No brief obligation lacks a task.** The four
refutations all hold under independent measurement, the four-halves claim about
D102 is true against the code, the pairwise-disjointness claim is true, all 21
house-knowledge ids resolve, and no scope prohibition is breached. What fails is
the evidence layer: two load-bearing claims in the plan do not reproduce when
run, one anchor that the task exists to protect has no fire, one edit is
unfenced where its three siblings are fenced, and one settled entry is cited for
something it does not cover.

---

## 1. Coverage walk (from the BRIEF, then checked against the plan's own map)

### Brief section 2: where the package sits

| Brief obligation | Implemented by | Status |
|---|---|---|
| No sentence claims this package completes 1.0 scope | Plan header par. 3; Global Constraints; close action "The completion statement this close is allowed to make" | OK |
| No task prepares, versions or tags the release | Global Constraints bullet 6 | OK |
| `owner-manual-qa-gates-the-1-0-release` named as the binding precondition | Header par. 3; close actions | OK |

### Brief section 3: ground truth, exhaustively enumerated

| Brief obligation | Implemented by | Status |
|---|---|---|
| Spec authoritative on conflict, then brief, then the named ROADMAP entries, then the four house YAMLs, then `BUILDING.md` | Global Constraints bullet 1, all five layers named in order | OK |
| Cite house entries by id | 21 distinct ids cited; all 21 resolve against the four YAML files (probe `idcheck.py`, 519 entries loaded) | OK |

### Brief section 4, W1: the D102 producer

| Brief obligation | Implemented by | Status |
|---|---|---|
| Build the producer (owner RULED 2026-07-29) | Task 2, Step 3 | OK |
| Shape the review named: `batch_document` case, mixed-severity `batch_diagnostics`, asserting NOT reordered | Task 2, P1 | OK |
| Measure per array, not once (mutate, run, record, restore) | Task 2, Step 2, M1-M4 | OK |
| A guarded half gets no second producer; an unguarded half gets one | Task 2, Step 2 "Reading the result, fixed" | OK |
| Do not duplicate the `dry_run_cli.rs` guard | Task 2 Files note + "Must not decide" | OK |
| The measurement belongs in the report for independent re-measurement | Task 2, Steps 2 and 4 | OK |
| A producer named per surviving half | Acceptance map W1-a..W1-d, all four with a named producer under both measurement branches | OK |

**Halves walk, verified independently.** I read
`crates/muxsmith-core/src/report/json.rs` directly. `batch_document` sorts
`config_diagnostics` (lines 61-64), and emits `files[].diagnostics` (line 57)
and `batch_diagnostics` (line 68) through `rendered_diags`, which preserves.
`config_only_document` sorts `config_diagnostics` (lines 100-103) and emits
`"files": []` and `"batch_diagnostics": []` as literals (lines 106-107). The
author's claim is exact: **four halves, not six**, because
`config_only_document` has no preserved-order half to guard. `run_document`
adds no diagnostic array of its own. Producer named per half: yes, all four.

The four mutations compile-check as written: `severity_sorted` is already
imported in `json.rs` (line 15), `rendered_diags` returns
`Vec<serde_json::Value>` so M1/M2 are type-compatible, and M3/M4's
`collect::<Vec<_>>()` sits in `serde_json::json!` value position where the
existing calls do. The fenced scaffolding resolves against real API: `pub code:
DiagCode` (mod.rs:198), `DiagCode::key()` (mod.rs:58), `Diagnostic::info` /
`::warning` / `::error(code, config_path)` (mod.rs:245-257), and the three keys
the assertions expect are literally `"invalid-regex"` (mod.rs:82),
`"raw-property"` (:86), `"raw-on-known-property"` (:88). `Batch` has exactly
`files` / `batch_diagnostics` / `suggestions` (planner.rs:256-264); `FileReport`
has exactly `source` / `identifier` / `plan` / `diagnostics`
(planner.rs:190-199). `report::json` is `pub` (report/mod.rs:10). The existing
guards exist at the names given, and `dry_run_json_sorts_..._when_planning_ran`
is `have_mkvmerge()`-gated at dry_run_cli.rs:409, exactly as claimed; its doc
comment reads "The `batch_document` half of the same D102 change" verbatim
(dry_run_cli.rs:397).

### Brief section 4, W2: the comment line-citation sweep

| Brief obligation | Implemented by | Status |
|---|---|---|
| Sweep the corpus under the ruling | Task 5, Step 2 | OK |
| RE-MEASURE before editing anything | Task 5, Step 1 | OK |
| State the search in the report so a reviewer can reproduce it with a different instrument | Task 5, Step 1 (command + full output pasted) | OK |
| Historical citations lose their numbers too | Task 5, Step 2 bullet 1 | OK |
| Ambiguous file references disambiguated (`run.rs`) | Task 5, Step 2 bullet 2 | OK |
| Scope boundary: source comments only, not process artifacts | Task 5, Step 3 | OK |
| Absence check on the end state | Task 5, Step 4 | Partial, see F10, F11 |

### Brief section 4, W3: the gate-count invariant

| Brief obligation | Implemented by | Status |
|---|---|---|
| `BUILDING.md` states the total once, canonically | Task 1, Step 1(a) | OK |
| A check verifies the stated total against the enumerated commands | Task 1, Step 2 | OK |
| The check lives in `scripts/ledger-lint.py`, not a new script | Task 1 Files + "Must not decide" | OK |
| Its header comment and **any self-description** widen accordingly | Task 1 Steps 1(d), 2, 3 | **GAP, see F3** |
| No rename | Global Constraints; "Must not decide" | OK |
| The check is FIRE-VERIFIED, verification part of the deliverable | Task 1, Step 4, F1-F3 | Partial, see F8 |
| The canonical sentence machine-findable by a stable anchor, with the stability argued | Task 1, Step 1(a) marker + Step 1(c) rationale | OK |
| No cross-file "N parts per BUILDING.md" lint | "Must not decide" | OK |

### Brief section 4, W4: the Renovate configuration

Every settled item walked against the fence in Task 3, Step 1:

| Brief item | In the fence | Status |
|---|---|---|
| `$schema` | line 1 of the fence | OK |
| `extends: ["config:recommended", "helpers:pinGitHubActionDigests"]` | present | OK |
| `config:best-practices` NOT used, with the reasons | comment above `extends` | OK |
| `security:minimumReleaseAgeNpm` NOT used | same comment | OK |
| `timezone: "Europe/Berlin"` | present | OK |
| `schedule: ["* * 1-3 * *"]`, NOT the preset, reasoning in a comment | present, with the "do not simplify this back" line the brief asked for | OK |
| `prHourlyLimit: 0` | present | OK |
| Cargo group with `rangeStrategy: "bump"` | rule 1 | OK |
| npm group | rule 2 | OK |
| GitHub Actions group on `action` depType | rule 3 | OK |
| Runner images disabled (`github-runner`) | rule 4 | OK |
| `packageManager` depType disabled | rule 5, ordered after the npm group with "later rules win" stated | OK |
| `mise` manager disabled | rule 6 | OK |
| `rust-toolchain` in its own group | rule 7 | OK |
| Majors not folded: do NOT set `separateMajorMinor` | absent, and the absence is stated as fixed in "Must not decide" | OK |
| `vulnerabilityAlerts` not configured, claim verified before relying on it | Step 2, with the fallback if defaults moved | OK |
| Validate with the official validator at a pinned version, paste command and output | Step 3, both plain and `--strict` | OK |
| Add no gate part, CI job, runtime dependency | Files list, Step 3, Step 5, Global Constraints | OK |
| Must NOT claim activation; two OWNER actions; config on `master` before app install | Step 4; acceptance map W4-c marked OWNER, stays OPEN | OK |

packageRules recount from the fence's own enumeration: cargo, npm,
github-actions/action, github-actions/github-runner, npm/packageManager, mise,
rust-toolchain = **7**, matching the self-review's figure.

### Brief section 4, W5: the README accuracy pass

| Brief obligation | Implemented by | Status |
|---|---|---|
| Re-check the CLI reference against the shipped surface, from the binary's help | Task 4, Step 1 | OK |
| Re-check the exact-typed-matching paragraph | Task 4, Step 3 | OK |
| Write in the content anchor, four items, verified against spec AND code | Task 4, Step 3 | OK |
| The four `placeholder(1.0)` comments stay | Task 4, Step 4 | OK |
| The WIP banner stays | Task 4, Step 4 | OK |
| The plan says so where a reader would look | Task 4, Step 4 is titled exactly that | OK |

Verified: README has exactly four `placeholder(1.0)` comments (`:7`, `:61`,
`:99`, `:187`), matching the plan's parenthetical.

### Brief section 5: what the plan must contain

| Brief obligation | Implemented by | Status |
|---|---|---|
| Sequencing argued as a comparison, not a count | Sequencing section, with the 17.186 s warm measurement | OK |
| **"note the file overlap that exists"** | Not noted; replaced by a disjointness claim | **GAP, see F7** |
| Model tiers as a table, top tier reserved for the plan-close whole-branch review | Model tiers section | OK |
| Every dispatch names its model explicitly | Model tiers section, second sentence | OK |
| Global constraints, all seven named sub-items | Global Constraints, all present | OK |
| Acceptance map, a producer per half | 16 rows, one owner-observation row marked | OK |
| Close actions as an explicit controller list | Plan close, all six required items plus the two required notes | OK |

### Brief sections 6 and 7

Every prohibition in section 6 appears in Global Constraints. Section 7's four
house rules are all carried (progress deviation in the header; every acceptance
row machine-checkable or marked owner; `proc-proposed-safeguard-stays`;
`tests-ship-with-the-feature-never-after`).

### The plan's own coverage map, checked against this walk

The five-row map is accurate: W1 to Task 2, W2 to Task 5, W3 to Task 1, W4 to
Task 3 with activation marked OWNER, W5 to Task 4. It claims no coverage the
tasks do not deliver. The gaps above are inside work items, not missing rows.

---

## 2. The author's four refutations, re-measured

All four hold. Two were re-measured against the tree with an independent
instrument, two against the vendor's own source.

**Refutation 1: the Frontend checks section states no count. CONFIRMED.**

```
$ grep -niE "\b(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b" BUILDING.md
74:### The Rust gate (six parts, run from the repo root, workspace-wide)
98:CI-red twice in Plan 5 and sat unobserved for five runs in Plan 8 (an
122:three OS legs (its Windows leg covers natively what part 6 cross-checks
```

`### Frontend checks` at `:103` carries no count. The brief's "six in one
section and four in another" is refuted.

**Refutation 2: the corpus is 20 lines across 13 files, not 17. CONFIRMED, and
the cause reproduces.** Re-measured with a Python `re` scan over `git ls-files`
rather than the author's `xargs grep`:

```
crates/muxsmith-cli/tests/run_live.rs [273, 274, 315, 326, 327, 361]
crates/muxsmith-core/src/identify.rs [557]
crates/muxsmith-core/src/planner.rs [2226]
crates/muxsmith-core/src/report/json.rs [161]
crates/muxsmith-core/tests/suggestions.rs [1015, 1035, 1071]
e2e/smoke.spec.ts [1434]
src-tauri/src/lib.rs [730]
src/components/ResolutionTable.vue [32]
src/editor/fieldSpec.ts [65]
src/editor/widgets/FieldWidgetDispatcher.vue [6]
src/editor/widgets/OptionalFlagWidget.vue [4]
src/editor/widgets/PropertyMapWidget.vue [48]
src/views/EditorView.vue [83]
TOTAL LINES: 20 FILES: 13
```

Identical set, identical line numbers. I also reproduced the *mechanism* the
correction claims, by running the same scan with the old cited-extension set:

```
old-extension-set matches: 17  new-set matches: 20
in new set only: ['crates/muxsmith-cli/tests/run_live.rs:274',
                  'crates/muxsmith-cli/tests/run_live.rs:315',
                  'crates/muxsmith-cli/tests/run_live.rs:327']
```

Exactly 17 under the old set, exactly the three `README.md`-citing lines added
by including `md`. The refutation and its explanation are both correct.

I additionally verified every one of the 20 sites is a comment line, so Task 5's
"COMMENT TEXT ONLY" constraint is satisfiable today. See F10 for why that is a
property of the corpus rather than of the instrument.

**Refutation 3: the cargo default is `auto`, resolving to `update-lockfile`.
CONFIRMED at the vendor.** Fetched `https://docs.renovatebot.com/modules/manager/cargo/`
independently:

> "If a 'less than' instruction is found (e.g. `<2`) then `rangeStrategy=widen`
> will be selected, Otherwise, `rangeStrategy=update-lockfile` will be selected."
> "The `update-lockfile` default means that most upgrades will update
> `Cargo.lock` files without the need to change the value in `Cargo.toml`."

The plan's quotation is verbatim and its conclusion (`bump` is still right)
stands.

**Refutation 4: the documented validator invocation is neither form the recon
started from. CONFIRMED.** Fetched `https://docs.renovatebot.com/config-validation/`:

```
npx --yes --package renovate -- renovate-config-validator
npx --yes --package renovate -- renovate-config-validator --strict
npx --yes --package renovate -- renovate-config-validator first_config.json
```

All three documented forms present, so `--strict` and a file argument are both
documented. `https://registry.npmjs.org/renovate/latest` returns version
**43.287.0**, matching the plan's pin exactly.

**Renovate premises beyond the two refutations, spot-checked at source** (these
are Task 3 Step 2's duty at execution, checked here because they carry the
fenced comments):

- `prHourlyLimit` default: the docs page rendering is ambiguous and a
  small-model extraction of it returned `10`, which is `prConcurrentLimit`'s
  default. Reading `lib/config/options/index.ts` directly settles it:
  `{ name: 'prHourlyLimit', ..., default: 2 }`. **The plan is right.** I record
  the false lead because it is the trap a Step-2 re-verification will hit.
- `schedule:monthly`: `const monthly = ['* 0-3 1 * *'];` in
  `lib/config/presets/internal/schedule.preset.ts`. Verbatim match.
- `config:best-practices` extends `["config:recommended", "docker:pinDigests",
  "helpers:pinGitHubActionDigests", ":configMigration", ":pinDevDependencies",
  "abandonments:recommended", "security:minimumReleaseAgeNpm",
  ":maintainLockFilesWeekly"]`. Both reasons the plan gives hold.

---

## 3. Sequencing, latitude, tiers, house, scope

**Disjointness: CONFIRMED for the Files lists as written.** Task 1 {BUILDING.md,
scripts/ledger-lint.py, .github/workflows/ci.yml}, Task 2 {crates/muxsmith-core/tests/report_json.rs},
Task 3 {renovate.jsonc}, Task 4 {README.md}, Task 5 {13 source files}. No
intersection. The 13-item Files list and the 13 pathspecs in Task 5's `git add`
line are set-equal and every path is tracked (verified programmatically). See F7
for the write-set that the Files lists do not capture.

**The 4 -> 5 edge is real.** `run_live.rs:274`, `:315` and `:327` cite
`README.md:71-78` and `README.md:91`; Task 4 edits `README.md`. No other
unstated edge found: no Task-5 site cites `BUILDING.md`, `ci.yml`,
`ledger-lint.py` or `renovate.jsonc`, and the only cross-task coupling beyond
4 -> 5 is the one the plan already states (`run_live.rs` inlines the README
passthrough recipe, which Task 4 does not touch).

**The worktree comparison resolves, and the "strictly larger" half is argued,
not asserted.** The plan gives the mechanism (a fresh worktree carries neither
`target/` nor `node_modules/`, so the first gate run is a cold cargo build plus
`pnpm install`). I ran the premise that could falsify it: `.cargo/config.toml`
sets only `TS_RS_EXPORT_DIR` and `TS_RS_LARGE_INT`, no `CARGO_TARGET_DIR`, and
`git grep CARGO_TARGET_DIR` finds it only inside process-journal artifacts. So a
worktree genuinely does not inherit the warm target dir and the argument holds
by construction. `/target` and `node_modules/` are both gitignored, and both are
present in the main tree, so the 17.186 s figure is a warm number as labelled.

**Latitude, three named suspects.**

1. *Task 1's fence carries `11 / 6 / 4 / 1`.* **I apply the SCOPED reading and
   the author is right.** The brief's clause sits inside the sub-item
   describing the gate CONSTRAINT ("the gate per BUILDING.md, foreground, no
   subsets, before any push, with no count written into the plan - name the
   file, because ... a plan that hardcodes a number would fork the contract it
   executes against"). The stated harm is a forked contract, and Global
   Constraints names the file and states no count, so no contract is forked.
   Removing the number from Task 1's deliverable would leave the implementer
   inventing it, which the same brief bans harder. The plan's NEEDS_CONTEXT
   guard on a disagreeing recount is the correct handling. Not a finding. But
   the plan's *self-audit* about where else the count appears is false; see F2.
2. *W1's producer set, measurement-gated over four written-out candidates.*
   **Closed enumeration, acceptable.** The four mutations map one-to-one onto
   the four candidates; each producer carries its name, home, fixture, call and
   assertion; both branches of each measurement are pre-decided ("name the
   failing test and write NO producer" / "write the enumerated producer"); the
   all-green outcome is pre-routed to NEEDS_CONTEXT. Nothing is handed to the
   keyboard.
3. *W5's sentence wording.* **Acceptable in substance, wrong in its citation.**
   The fact set is genuinely closed (the corrections are enumerated, the anchor
   is a fixed four-item list, each item bound to a named spec section and core
   symbol). Prose that carries a closed fact set is implementation. But see F6:
   the entry cited to license it does not cover it.

**Omission latitude, swept.** Every Files list is marked EXHAUSTIVE. Interfaces
are stated per task. Every string a task writes is fenced except one: Task 1
Step 1(d)'s new paragraph sentence (F5). No list ends open, no "one per X"
without an X list, no unnamed test, no "the appropriate module".

**Model tiers.** A tier per task, each with a stated ground. Top tier reserved
for the plan-close whole-branch review, matching `proc-03-model-assignment`'s
OWNER BOUND ("the top model serves ONE role, the plan-close whole-branch review
including its delta re-reviews"). The cheap-tier claim is tested against the
right task: Task 3 is the one whose content the plan carries most literally, and
the plan says explicitly why it does not qualify (Step 2's fifteen-item vendor
re-verification and the `--strict` adjudication are judgment). That grounding is
sound.

**House.** No ticked checkboxes (0 ticked, 29 unticked, so the check is not
vacuous). No task edits a house YAML. SI-4 restated with the trailer DERIVED
from the dispatch's model parameter and no literal model name anywhere in the
document (grep for `opus|sonnet|fable|haiku|claude-` returns only the SI-4 line
carrying the `<model>` placeholder). Explicit staging in all five commit blocks,
`git add -A` appearing only inside its own prohibition. Typography clean: a
grep for em-dash, en-dash, figure dash, horizontal bar, Unicode minus, curly
quotes, Unicode ellipsis and NBSP over the whole plan returns nothing, and I
fire-verified that same pattern against a scratch file containing a single
em-dash, where it matched. `comments-locate-by-symbol-never-by-line-number`
applied to comments the plan instructs an implementer to write: the fenced
rustdoc in Task 2 Step 3 carries no line-number citation. See F12 for the one
adjacent instruction.

**Scope.** No version bump, no tag, no release-body edit, no placeholder
resolution, no banner removal, no new gate part or CI job or runtime dependency,
no house-YAML edit, no `ledger-lint.py` rename. The negative half holds too: the
plan states three times that this package is not 1.0 completeness and does not
propose the tag.

**No-work-needed passages, premises run rather than weighed.**

- "renovate.jsonc is not consumed by any gate part" (Task 3 Step 5). Ran it:
  `"lint": "eslint ."`, and `eslint.config.js` registers only
  `typescript-eslint`, `eslint-plugin-vue` and `@intlify/...no-raw-text`. No
  JSON/JSONC plugin, so a root `.jsonc` matches no config object. `pnpm build`
  is `vue-tsc --noEmit && vite build`. Premise holds.
- "No live reference to the Rust-gate heading's wording exists outside
  `BUILDING.md`". Ran it. **Does not hold as stated.** See F1.
- "The gate's own part count appears nowhere in this plan except inside Task 1's
  fenced deliverable". Ran it. **Does not hold.** See F2.
- "the existing success line reads `ledger-lint: {total} entries across
  {len(FILES)} files, all invariants hold`". Ran `python3 scripts/ledger-lint.py`:
  `ledger-lint: 519 entries across 4 files, all invariants hold`, exit 0. Holds,
  and the script's structure supports the prescribed integration (a local
  `violations` list in `main()`, `Path` already imported, `FAIL {rel}: ...`
  print shape, docstring check list ending at 6 so "check 7" is the right
  number).

---

## 4. Findings

### Critical

None.

### Important

**F1. The Rust-gate reference evidence line does not reproduce, and Step 1(b)
rests on it.** Location: plan `:62`, consumed at Task 1 Step 1(b) ("No live
reference to the old wording exists (authoring section)").

The plan states: "`git grep -n "The Rust gate\|the-rust-gate\|Rust gate"`
excluding `docs/ROADMAP.md` and `docs/process-journal.md` returns exactly one
live hit, `BUILDING.md:74`; every other hit is a historical process artifact
under `docs/process-journal/artifacts/`".

Run with the stated exclusions:

```
$ git grep -n "The Rust gate\|the-rust-gate\|Rust gate" \
    | grep -v "^docs/ROADMAP.md:" | grep -v "^docs/process-journal.md:" | wc -l
57
```

and of those, six sit outside `docs/process-journal/artifacts/` and outside
plan-10 itself:

```
docs/superpowers/plans/2026-07-14-plan-5.7-routed-items-pre-1.0-fixes.md:102
docs/superpowers/plans/2026-07-14-plan-5.7-routed-items-pre-1.0-fixes.md:124
docs/superpowers/plans/2026-07-14-plan-5.7-routed-items-pre-1.0-fixes.md:148
docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:1926
docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md:20
docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md:20
```

Both halves of the sentence are false: it is not one hit, and the other hits are
not under `docs/process-journal/artifacts/`. The two plan-8.5 and plan-9 hits are
particularly relevant, because they quote the heading's wording inside their own
normative gate constraint ("Ten-part gate per BUILDING.md ('The Rust gate' six
parts + the four frontend checks)").

The conclusion probably survives, because retired plan documents are history by
the same principle the ROADMAP's MEASURED block establishes for every other
stale count. But the plan does not make that argument here; it asserts a
location that is wrong, and a Task-1 step is licensed by it.

**Change:** restate `:62` with the correct enumeration and the correct ground
("six further hits sit in retired plan documents under `docs/superpowers/plans/`,
which are history by the ROADMAP MEASURED block's own principle and are not
edited"), and re-point Step 1(b)'s parenthetical at the corrected line.

**F2. The self-review's gate-count audit is false, and the check it claims to
have run is aimed at forms that cannot occur.** Location: plan `:557`
(Self-review), consumed implicitly wherever the plan asserts it never hardcodes
the count.

Claim: "The gate's own part count appears nowhere in this plan except inside
Task 1's fenced deliverable ... checked by grepping this document for the stale
forms ('ten-part', 'eleven-part', 'N parts per')."

Two failures. First, the count does appear elsewhere, at plan `:59`:

> Rust block (lines 77-82) `6`; frontend block (105-108) `4`; house-knowledge
> block (114) `1`. Sum `11`.

Second, the grep as described, run verbatim, is not empty:

```
$ grep -niE "ten-part|eleven-part|N parts per" docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md
216:... No cross-file "N parts per BUILDING.md" lint is built in any form ...
557:... checked by grepping this document for the stale forms ("ten-part", ...)
```

The searched forms are hyphenated compounds and a literal placeholder; the form
actually present is `Sum \`11\`` and bare `` `6` ``/`` `4` ``/`` `1` ``. This is a
negative-result check that could never have fired against what it was auditing,
reported as if it had.

Substantively `:59` is authoring evidence rather than a constraint, so no
contract is forked and F2 does not overturn my scoped reading of the fence
question. But a self-audit sentence that is measurably false is worse than no
sentence, because a downstream reader treats it as the check having been done.

**Change:** narrow the claim to what is true ("the count appears in the
authoring measurement at `:59`, in three quotations of `BUILDING.md`'s current
heading, and in Task 1's fenced deliverable; the Global Constraints' gate clause
names the file and states no count"), and either drop the grep sentence or
replace it with a search that can actually match (`parts:`, `Sum`, and the
number words).

**F3. Task 1 widens two of three narrow self-descriptions of
`scripts/ledger-lint.py` and fences off the third.** Location: Task 1 Files list
(the BUILDING.md within-file qualifier and the ci.yml "no step" clause).

Task 1's stated purpose for Step 1(d) is "so the script's name does not
mislead", and the brief requires "its header comment and any self-description
widen accordingly". Three self-descriptions exist. Two are widened (the
BUILDING.md House-knowledge section, the ci.yml job comment). Two are left
narrow and are explicitly out of reach:

- `BUILDING.md:125`: "`cargo deny check` and `scripts/ledger-lint.py`
  (house-knowledge invariants, Plan-8 rider) run as independent jobs." This sits
  in the CI paragraph, not the House-knowledge section, and Task 1's Files entry
  says "nothing else in the file".
- `.github/workflows/ci.yml:180`: `- name: ledger-lint (house YAML invariants)`.
  Task 1 Step 3 says "no step".

Under `latitude-carveout-zero-content-structural-forks`, both would otherwise be
in scope (repairing a reference the task's own edit invalidated, inside a listed
file), but the entry is explicit that an in-file qualifier fences the grant:
"an entry constrains work WITHIN its file only where it carries an explicit
within-file qualifier - the word 'only', a named line span, a named region". The
plan supplies exactly such qualifiers, so the implementer must stop and file a
finding rather than fix a one-line parenthetical the same task falsified.

The entry's own over-restriction watch invites exactly this flag.

**Change:** extend Task 1's BUILDING.md scope by one named region (the CI
paragraph's ledger-lint parenthetical), fence the replacement text, and allow
the ci.yml step `name:` to widen alongside the comment it sits above, or state
explicitly that both stay narrow and why.

**F4. `BUILDING.md`'s positional gate ordinals are neither swept, scoped out,
nor surfaced.** Location: `BUILDING.md:95` ("The cross-target clippy run (part
6)") and `:121` ("runs Rust-gate parts 1-4").

Step 1(b)'s stated rationale for stripping the count from the heading is that
"leaving it here would leave the file with one checked number and one unchecked
one, which is the defect class this task closes". That reasoning applies
verbatim to `:95` and `:121`, which are gate part numbers the new check does not
cover. Once the file states "The pre-push gate is 11 parts", "part 6" acquires
a second possible referent (the sixth of eleven is a frontend part), resolved
only by section context.

The plan mentions neither line. The number-word grep at `:60` cannot see them
because both use digits. The review brief records these as findings the author
surfaced; they are not in the plan document, and no author report exists in
`.superpowers/sdd/plan-10/` to carry them.

Surfacing for controller routing is the correct treatment here, exactly as the
plan does for the three bare-span citations and the two stale README counts.
Silently dropping them is not.

**Change:** add a bullet to Task 1 surfacing both ordinals as OUT of this task
with the reason (they are Rust-block-local positions, not gate totals, and
checking them would need a second parser), so the controller can route them.

**F5. Step 1(d) is the only edit in Task 1 Step 1 that is not fenced, and it
leaves an existing sentence's fate to the implementer.** Location: Task 1, Step
1(d).

Steps 1(a), 1(b) and 1(c) each fence their text and say "exactly". Step 1(d)
fences the inline comment ("becomes exactly ...") but then says: "the paragraph
below it **keeps its first sentence and gains one**: the script also checks
that ...".

The paragraph at `BUILDING.md:117-119` has two sentences:

```
A gate part like the Rust and frontend ones above, binding before every push
(house rule `ledger-lint-runs-before-every-push`). Needs PyYAML; CI runs it
from a throwaway venv as its own job.
```

"Keeps its first sentence" does not say whether the second survives. Dropping it
loses the PyYAML prerequisite and the CI-job fact, which nothing else in that
section carries. And the added sentence's own wording is not written down
anywhere the implementer can copy, in a plan whose Global Constraints ban "a
step that requires inventing a name, a string, or a file that is not written
down somewhere the implementer can read".

**Change:** fence the full replacement paragraph verbatim, the way 1(a) and 1(c)
are fenced.

**F6. Task 4 cites a settled entry that does not cover what it licenses.**
Location: Task 4, Step 3, "Register" bullet.

The plan writes: "the sentences that carry them are the implementer's, in the
surrounding register (`latitude-carveout-presentation-tokens`)."

That entry reads: "visual presentation tokens - exact colors, widths, spacing,
minor styling - within the app's existing design language are implementer-owned
... The boundary: anything semantic-carrying stays enumeration-bound - which
severity maps to which color, what an icon means, layout structure, anything
whose variation changes meaning rather than looks."

README prose is semantic-carrying by construction; it is the meaning. The entry
does not reach it and its boundary sentence excludes it. This is a
misrepresentation of a settled decision, which is in scope even though the
decision itself is not.

The underlying latitude is fine on its merits (the fact set is closed, the
sentences are implementation), and the register itself is correctly grounded on
the ROADMAP README entry, which does say "sell-tone per Şenol's register
override - a case-scoped exception to the writeup-stimme rule". Only the id
citation is wrong.

**Change:** drop the `latitude-carveout-presentation-tokens` citation and rest
the register on the ROADMAP entry alone, or route the question of whether a
prose-wording carve-out exists to the controller as a decision memo.

**F7. The sequencing section's disjointness claim understates Task 2's real
write-set, and the brief's explicit "note the file overlap" ask is unanswered.**
Location: Sequencing section, first paragraph.

The claim "the five tasks' Files lists are pairwise disjoint" is true of the
lists. It is not true of the write-sets: Task 2 Step 2 mutates
`crates/muxsmith-core/src/report/json.rs` four times and restores it, and that
file is item 4 of Task 5's Files list. The plan states the mutation and its
restore proof inside Task 2, but the sequencing argument does not carry it, so
the sentence a later reader uses to reason about concurrency is missing the one
production file two tasks touch.

The brief asked for this directly: "note the file overlap that exists - the
comment sweep touches files in the same crates and frontend directories the
other tasks touch." The plan answers a different, narrower question.

This is benign under the serial ruling. It matters because the serial ruling's
own supporting argument is weaker than it could be: the plan gives "a concurrent
writer's gate run would exercise a half-edited lint script" but not the stronger
"a concurrent Task 5 would be editing a file Task 2 has deliberately broken".

**Change:** state the transient write-set in the sequencing section, and add the
report/json.rs collision to the list of reasons parallelism is not available
even in principle.

**F8. The three gate-block marker anchors have no fire, and Step 1(c) cites a
fire that tests a different marker.** Location: Task 1, Step 1(c) rationale and
Step 4; acceptance map row W3-d.

Step 1(c) argues for HTML-comment markers over headings and closes with: "fire
F3 below proves that deleting one turns the gate red instead of silently
disabling the check."

F3 deletes `<!-- gate-total; checked by scripts/ledger-lint.py -->`, the total
anchor. The three block anchors (`<!-- gate-block: rust; ... -->` and siblings)
are never deleted in any fire. Their existence check is specified in Step 2
("exactly one occurrence must exist"), so it would fire, but that is argued, not
measured, in the one task whose entire subject is that an unfired absence check
proves nothing.

The plan's own Global Constraints bind here: "Each absence check below carries
both halves." Four absence-shaped checks are specified (three block markers, one
total marker); three of them have no red state.

**Change:** add a fourth fire, F4: delete one gate-block marker, run, confirm
exit 1 naming the missing marker, restore. Then re-point Step 1(c)'s sentence at
F4 rather than F3.

### Minor

**F9. A cited line number is attributed to the wrong key.** Location: plan `:86`.

The plan states `.github/workflows/release.yml` "uses `runs-on: ubuntu-22.04` at
`:29`, `:81` and `:186`".

```
$ grep -n "ubuntu-22.04" .github/workflows/release.yml
22:# (the only GA windows-arm64 image); ubuntu-22.04 here deliberately
29:    runs-on: ubuntu-22.04
81:            os: ubuntu-22.04
186:    runs-on: ubuntu-22.04
```

At `:81` the key is `os:` inside a matrix `include:` block; the `runs-on:` that
consumes it is `runs-on: ${{ matrix.os }}` at `:83`. The substantive fact holds
and the in-file divergence comment exists at `:22-23` as claimed, but the plan's
own `design-empirical-claims-reproducible` constraint says a value is "never
attributed to a command that was not the one run".

**Change:** write "`runs-on:` at `:29` and `:186`, and the matrix `os:` at
`:81`".

**F10. Task 5's instrument is broader than the observable it certifies.**
Location: Task 5, Step 1 command; acceptance map W2-a.

W2-a's observable is "No tracked source COMMENT cites `<filename>:<line>`". The
Step-1 command matches any tracked source LINE, comment or not. Today the two
coincide: I checked all 20 hits and every one is inside a `//`, `///`, `//!` or
`/* */` comment, so the absence check is reachable under the comment-only
constraint. If a future hit is a string literal or a code line, the green state
becomes unreachable without editing code, which Task 5 forbids, and the plan's
NEEDS_CONTEXT routing only covers "a site in a file not listed above".

**Change:** widen the NEEDS_CONTEXT trigger from "a file not listed above" to
"a site outside a comment, or a file not listed above".

**F11. A continuation line carrying a bare span is invisible to both
instruments.** Location: `crates/muxsmith-core/tests/suggestions.rs:1015-1016`.

```
// `not` entry's `exact` (delta_for's two exact-bearing arms, planner.rs:1812,
// :1817). Returns None if the key is absent, which is itself a guard failure.
```

`:1015` matches the corpus grep; `:1016`'s `:1817` has no filename token and no
backtick, so it matches neither the corpus expression nor the bare-span control
the plan uses to enumerate the three OUT sites. The site is swept in practice
because the whole comment is rewritten, but the absence check does not see this
member, and the plan's "reachable green state, argued member-by-member" argument
enumerates 20 members where a 21st fragment exists.

I confirmed the plan's underlying stale-citation claim is right, incidentally:
`delta_for` is at `planner.rs:1820`, so `:1812` and `:1817` do not point at its
arms at HEAD.

**Change:** note in Step 2 that a citation's continuation lines are rewritten
with it, so the absence check's member enumeration is comments rather than
matched lines.

**F12. The plan instructs writing a new document line-number citation inside the
package that sweeps them out.** Location: Task 2, Step 3, P1 ("with the
assertion message naming D102's scope boundary and spec line 255").

`comments-locate-by-symbol-never-by-line-number` names comment forms, and an
`assert!` message is a string, not a comment, so the ruling's letter does not
reach it. But the plan itself surfaces the neighbouring class (bare line spans
into design documents) as an unrouted question for the controller, and then
prescribes creating one more instance of it. Naming the spec SECTION rather than
the line would be durable and equally precise.

**Change:** either name the spec section instead of the line, or note in Step 3
that this citation is deliberately in the neighbouring class and rides the same
controller routing.

**F13. Typo in a rationale sentence.** Location: Task 1, Step 1(c): "Headings in
this file get reworded - this very step reworts one". Read "rewords".

**F14. An overstated "and nothing else".** Location: plan `:71`. The plan says
`./target/debug/muxsmith schema --help` "prints `Usage: muxsmith schema` with
`Options: -h, --help  Print help` and nothing else". Run:

```
$ ./target/debug/muxsmith schema --help
Print the profile JSON Schema

Usage: muxsmith schema

Options:
  -h, --help  Print help
```

The about line is also printed. The load-bearing point (no `--json`, no
`--locale`) is exactly right, and both README claims at `:91` and `:115` are
false for `schema` as stated.

---

## HARVEST

**Dominant pattern: the plan's own evidence discipline is excellent where it
measures, and fails where it audits itself.** Every figure this plan *measured*
reproduced under an independent instrument, including the two that mattered most
(the corpus at 20/13, with the 17-versus-20 mechanism reproducing exactly, and
D102's four halves against the source). Every figure this plan *asserted about
its own text* was wrong: F1 and F2 are both sentences of the form "I checked, and
nothing turned up". That is the same defect class the package exists to close,
one level up, and it is the second time in this package's short history that a
count in a normative position came from a mis-shaped search rather than from
counting (the first was the controller's 17). The generalizable handle: a
negative result about a document you are currently writing is the hardest kind
to fire-verify, because the writer is also the only reader, and the search terms
get chosen from memory of what was written rather than from what is there.

**Repeated rejection shape: the exhaustive Files list as a within-file fence.**
F3 and F4 are the same underlying tension seen twice in Task 1. The plan uses
"nothing else in the file" and "no step" to prevent scope creep, and those
qualifiers work, but they also fence off two repairs the task's own edit
requires and one class the task's own rationale condemns.
`latitude-carveout-zero-content-structural-forks` anticipates exactly this and
asks reviewers to flag it, so this is a wanted harvest item rather than a
criticism of the boundary: **when a task widens a tool's remit, the set of
sites describing that tool is the natural scope unit, and enumerating it by
region inside each listed file is cheaper than discovering the leftovers at the
task review.**

**Where this brief's own boundary forced a stop I judge it should have covered.**

1. *Task 1's three fires are prescriptions, not claims.* Dimension 6 told me to
   "re-run" the three fires in Task 1 and the fired control in Task 5. Only the
   controls are re-runnable: F1-F3 describe fires an implementer will perform
   against a check that does not exist yet, and re-running them would require me
   to build Task 1's deliverable, which is out of a reviewer's role and would
   also violate the read-only constraint. I re-ran the three *controls* the
   plan does claim as fired (the backslash-continuation control, the
   corpus-pattern control, the renovate/dependabot ls-files control) and all
   three reproduce, and I evaluated F1-F3 as designs against the Step-2
   specification. F8 is what that evaluation found. The brief's phrasing
   flattened a real distinction between a claimed measurement and a prescribed
   one.
2. *No author report exists to check F4 against.* The brief states the author
   surfaced the `part 6` / `parts 1-4` ordinals. They appear nowhere in the plan
   and `.superpowers/sdd/plan-10/` holds only the two briefs, so I could not
   distinguish "the author surfaced it outside the artifact" from "the plan
   dropped it". I graded the artifact, which is what a plan review can grade,
   but the routing decision needs the controller's own record.
3. *The Renovate re-verification is genuinely deferred, and one of its steps has
   a live trap.* Dimension 2 named only two Renovate refutations, so a full
   re-verification of Task 3's fifteen Step-2 items was out of my scope. I did
   five of them anyway because they carry fenced comments that ship into the
   repo, and one produced a false negative on the first pass: the rendered
   configuration-options page yields `prHourlyLimit` default `10` under naive
   extraction (that is `prConcurrentLimit`'s value), and only reading
   `lib/config/options/index.ts` settles it at `2`. Task 3's implementer will
   hit that same page. Worth putting in the dispatch.

**One thing worth keeping regardless of this plan.** The plan's Task 5 control
("running the corpus expression with the filename requirement stripped produces
output, so an empty-looking result would have been a real absence rather than a
malformed pattern") is the cleanest instance of a fired negative control I have
seen in this repo's plan documents. It is exactly the construct F1 and F2 lack.
The asymmetry inside one document is the useful signal: the author applied the
discipline rigorously to the tree and not at all to the plan.

---

# Delta review after fix round 1

**Verdict: NEEDS FIXES**

Delta: `da60634` (558 lines) -> `afaf9a0` (619 lines). Confirmed the plan file is
byte-identical at `afaf9a0` and at HEAD `528be92`; the only movement since is
`docs/decision-ledger.yaml`, which no task touches, so the brief's scope
statement holds.

Reviewer scratch path for this pass, distinct from round 1's:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-delta1-indep/`
(`plan.diff`, `corpusB.py`, `audit_fire.txt`, `tenpart.txt`, `elevenparts.txt`,
`dash2.txt`). Round-1 probes were re-derived, not re-run: the corpus scan was
rewritten to measure both expressions, and every count below was recomputed
against the moved tree.

**All fourteen round-1 findings are closed.** The verdict is NEEDS FIXES on one
new defect the fix itself introduced: fire F5's acceptance criterion cannot be
met by the check behaviour Step 2 fixes.

## Round-1 findings, one line each

| # | Status | Evidence |
|---|---|---|
| F1 | **Closed** | The evidence line now states the distribution instead of a location. Recomputed with my own bucketing over `git grep`: 57 lines after the stated exclusions, split `BUILDING.md: 1`, `artifacts: 45`, `retired plans: 6`, `plan-10: 5`. The plan's `1 + 45 + 6 + 5` is exact. The ground moved to the ROADMAP MEASURED block's principle, which is the argument round 1 said was missing. |
| F2 | **Closed** | See the dedicated section below. |
| F3 | **Closed** | Ruled EXTEND. Both new target strings exist verbatim: `BUILDING.md:125` `and \`scripts/ledger-lint.py\` (house-knowledge invariants, Plan-8 rider)`, and `.github/workflows/ci.yml:180` `      - name: ledger-lint (house YAML invariants)`. Judged correct below. |
| F4 | **Closed** | Step 1(f) surfaces `BUILDING.md:95` and `:121` with reasoning and edits neither; Step 1(e) names them as explicitly preserved; the close actions carry them as a named harvest input. Surfaced, not absorbed, not dropped. |
| F5 | **Closed** | Step 1(d) now fences the whole replacement paragraph. Both facts round 1 said were at risk survive verbatim in the fence: the PyYAML prerequisite and the throwaway-venv CI job. |
| F6 | **Closed** | Judged below. |
| F7 | **Closed** | The sequencing section now leads with the write-set distinction, names `report/json.rs` as the one production file two tasks touch, states Task 5's crate and directory span, and adds the second independent reason parallelism is unavailable. |
| F8 | **Closed, with a new problem** | F4 (block anchor) and W3-e added; Step 1(c) re-pointed from F3 to "fire F3 deletes the total anchor and fire F4 deletes a block anchor". The block-marker choice is well argued (frontend, because F2 exercises the same block). The new problem is F5, not F4 - see N1. |
| F9 | **Closed** | Now reads `runs-on:` at `:29` and `:186` and the matrix `os:` key at `:81` which `runs-on: ${{ matrix.os }}` at `:83` consumes. Matches the file. |
| F10 | **Closed** | The NEEDS_CONTEXT trigger is now "outside a comment, or in a file not listed above", with the reason for each half, plus the measured statement that all 24 hits sit in comments today. |
| F11 | **Closed** | Superseded by expression B. Verified independently below. |
| F12 | **Closed** | Ruled section-not-line. I checked the new claim rather than accepting it: `### 5.2 Diagnostics` is at spec `:247` and the D102 sentence is on `:255`, inside that section, and the spec has not moved since `de4ea38`. My first probe truncated the line at 200 characters and appeared to refute this; the full line carries the sentence. Both halves of the citation are correct. |
| F13 | **Closed** | `reworts` gone, `rewords one` present. |
| F14 | **Closed** | Now states the about line, then `Usage:`, then `Options:` with `-h, --help` as its only entry. Matches the output I measured in round 1. |

## F2's replacement, which carried the higher bar

The old audit claimed a clean sweep from a grep for hyphenated compounds that
could not match anything it audited. The replacement claims nine sites from an
expression aimed at the forms actually used. Run verbatim:

```
$ grep -nE 'Sum \`|[0-9]+ parts|[0-9]+-part|six parts|\`6\`|\`4\`|\`1\`\.' \
    docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md | wc -l
9
```

Nine, at `:59`, `:60`, `:62`, `:96`, `:182`, `:190`, `:224`, `:240`, `:619`.
The plan's categorization maps onto those hits exactly: `:59` the authoring
measurement; `:60`, `:190`, `:96`, `:62` the four quotations (heading twice,
brief once, retired plans once); `:182` the fenced deliverable; `:224` and
`:240` the two internal references; `:619` the audit sentence itself. **The
Global Constraints gate clause is not among them**, which is the whole of what
the ban protects.

It fires. Against a scratch file:

```
$ grep -nE '...' audit_fire.txt
2:b 11 parts here
3:c Sum `11` done
```

And I tested it for the failure mode that killed its predecessor - being too
narrow to see what it audits. An independent sweep for word-spelled counts
(`six|eleven|ten|nine|eight|seven`) over the plan returns six lines, five of
which are already among the nine; the sixth is `:61` "Fenced code blocks in
BUILDING.md ... seven", which is a count of fenced blocks, not of gate parts,
and the same line says "Only three of the seven are gate blocks". A separate
sweep for `[0-9]+ (Rust|frontend|house-knowledge)` returns only `:182`, the
fence. **I found no gate part count the audit misses.** F2's fix does not repeat
F2's failure. See N3 for the one blemish on its control.

## The two controller decisions, judged for correctness

**F3 ruled EXTEND, and EXTEND is right.** The alternative (fence the two regions
off and let the implementer file a finding) would have left the task's stated
purpose half-served and spent a round on a parenthetical. The extension is done
the way `latitude-carveout-zero-content-structural-forks` requires: both regions
are NAMED in the Files list rather than opened by a general licence, the
within-file qualifier still bites everywhere else, and Step 1(e)/Step 3 fence
the exact replacement strings. The CI step `name:` is the sharpest of the four,
being the most-read self-description, and the plan says so. One accuracy nit,
not a finding: the plan says "four self-descriptions exist"; the script's own
docstring is a fifth, but Step 2 widens it separately, so nothing stale survives
in a live file. Retired design documents under `docs/superpowers/specs/` also
carry the narrow description and are correctly left alone as history.

**F12 ruled section-not-line, and that is right too.** The reasoning the plan
gives is the right one and is stated rather than assumed: an `assert!` message is
a string, so the convention does not reach it by its letter, but writing one more
line-number locator inside the package whose W2 sweeps that class out would be
incoherent. The section name is verified correct and is strictly more durable.

## The form deviation on F6, judged

**Keep it.** The licence is gone; what survives is `latitude-carveout-presentation-tokens`
named as an explicitly REJECTED reading, carrying its own boundary sentence and
why it does not reach README prose. That is the house pattern for a losing
argument, it is applied here to a reading a future reader would otherwise
rediscover from scratch, and the characterization of the entry is accurate
against its text. Removing the id entirely would delete the only record of why
the obvious carve-out does not apply. The replacement also fixes the deeper
issue round 1 flagged: the register now rests on the ROADMAP entry with its
wording quoted, and the latitude question is answered by the shape of the work
("the SET of facts is closed, so the implementer decides nothing") rather than
by a carve-out at all.

## The three things the fix forced, under full scrutiny

**The corpus at 24 lines across 16 files: CONFIRMED with my own instrument.**
I re-implemented both expressions in Python from their stated semantics rather
than from their shell forms:

```
A: 20 lines / 13 files
B: 4 lines / 4 files
   B -> crates/muxsmith-core/tests/profile_save.rs [95]
   B -> crates/muxsmith-core/tests/suggestions.rs [1016]
   B -> crates/muxsmith-core/tests/ts_export.rs [10]
   B -> src/editor/registries.ts [12]
lines matched by BOTH: []
UNION lines: 24 UNION files: 16
```

Exactly the plan's enumeration, and the union arithmetic is right because no
line is matched by both expressions while `suggestions.rs` is hit by each on a
different line. The prescribed shell form returns the same four lines with the
same content.

**The prescribed per-file form is immune to the failure it was written for, and
that failure is real.** I fired it:

```
$ git grep -nE '(^|[[:space:]`,(])[:][0-9]+' -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py' | wc -l
4
$ git grep -nE '(^|[[:space:]`,(])[:][0-9]+' -- ... \
    | grep -vE '[A-Za-z0-9_./-]+\.(rs|ts|vue|mjs|js|py|toml|ftl|json|yaml|md):[0-9]+' | wc -l
0
```

Zero against a tree with four hits: the false clean tree, reproduced. The
prescribed form escapes it structurally rather than by care - `grep` runs on the
raw file, and `sed` prepends the `file:` prefix afterwards, so the prefix can
never enter the match. That is the right shape of fix.

Green-state reachability for absence check B is real today: all four hits are
comment lines (`///`, `//`, `//!`, and a `/* */` continuation), so the
comment-only constraint can reach empty. And the folded-in sites collide with no
other task: Files list 16, `git add` pathspecs 16, **set-equal and every member
tracked**, with no member appearing in tasks 1-4.

The controller ruling that folds the bare spans IN is correct. The ruling's
stated ground ("a bare span is the WORSE form of what the owner's ruling bans")
holds: the owner's entry bans locating by line number, and a span with no
filename carries the same volatility with less information. The rewritten Step 3
also fixes a real misreading in the old text - the convention's scope boundary
separates by the artifact DOING the citing, not by the artifact cited, and the
entry's wording ("does not reach dated evidence citations in PROCESS artifacts")
supports the plan's new reading. The repairs invent nothing: each of the three
design citations already names its D-entry, which I confirmed in round 1.

**The fifth fire, F5.** Right principle, defective acceptance criterion. See N1.

**The acceptance map at 18 rows: no silent drop.** Rows are W1-a..d, W2-a..c,
W3-a..e, W4-a..c, W5-a..c. Round 1 had 16; the delta is exactly +W2-b and +W3-e.
The renamed W2-c carries the old W2-b's content ("Each rewritten comment still
points at the code it meant") with the reviewer instruction widened to both
expressions. Nothing was lost in the rename.

## The two controller additions

**The bare-span fold-in:** checked above - corpus, Files list, acceptance map,
absence check and `git add` set all moved together, and the set-equality holds at
the new count.

**The `prHourlyLimit` trap:** accurate and well aimed. I hit this exact trap in
round 1, from the rendered page, and only `lib/config/options/index.ts`
(`{ name: 'prHourlyLimit', ..., default: 2 }`) settles it against
`prConcurrentLimit`'s `default: 10`. The generalization the plan draws from it -
"where a rendered docs page and the source disagree, `lib/config/options/index.ts`
and `lib/config/presets/internal/` are the ground truth, and the report names
which of the two it read for each item" - is the right rule and binds the whole
Step-2 list rather than just this entry.

## New findings

### Important

**N1. Fire F5's acceptance criterion cannot be met by the check Step 2 fixes.**
Location: Task 1, Step 4, F5.

F5 prescribes rewriting the house-knowledge block's single command as two
backslash-continued lines and states the check "must exit 1 with the message
that the counter does not model shell continuations, **NOT with a count
mismatch**."

Step 2's fixed behaviour, which the implementer may not alter ("Must not decide:
... the counting rule and its continuation guard"), makes both happen:

- `python3 \` and `  scripts/ledger-lint.py   # ...` are each "non-empty and does
  not start with `#`", so the house block counts **2** commands against a stated
  **1** - a per-block mismatch.
- The total then counts 6 + 4 + 2 = **12** against a stated **11** - a total
  mismatch.
- `python3 \` ends with a backslash, so the continuation violation also fires.

Three violations, two of them count mismatches. Nothing in Step 2 says the
continuation guard suppresses the comparison, and adding that suppression is
precisely the counting-rule decision the task is forbidden to make. An
implementer running F5 literally gets an outcome its own plan says must not
happen, and the correct response under this plan's rules is NEEDS_CONTEXT - a
round spent on a sentence.

A charitable reading ("not MERELY a count mismatch") is available and is probably
what was meant, but a fresh implementer cannot ask, and the substantive fact -
that this mutation necessarily produces count mismatches alongside the
continuation message - is stated nowhere.

**Change:** one clause. "It must exit 1 and its violations must INCLUDE the
message that the counter does not model shell continuations; the accompanying
house-block and total mismatches are expected, because the guard does not
suppress the comparison. A run that reports only a count mismatch, with no
continuation message, is the failure this fire exists to catch."

### Minor

**N2. Step 2 is undefined for a block whose marker is absent, and F4 walks
straight into it.** Location: Task 2's neighbour - Task 1, Step 2 "The
comparison", against Step 4 F4.

"Each of the three stated per-block numbers is compared against its block's
counted commands" assumes three blocks were found. F4 deletes the frontend
marker, so the frontend block is never parsed, and the implementer must decide
whether the comparison skips that block or treats it as zero commands (which
would emit a frontend mismatch and a total mismatch on top of the
missing-marker violation). F4's stated acceptance is satisfied either way, so
this does not block the fire the way N1 does - but it is an unfenced decision in
a plan that bans them, and it is one sentence to close.

**Change:** state in Step 2 that a missing block marker is reported and that
block's comparison is skipped, so exactly one violation names the cause.

**N3. The F2 audit's fire control names three tokens, one of which the
expression does not match.** Location: plan `:619`.

The audit says it was "fire-verified against a scratch line containing
'ten-part', '11 parts' and 'Sum `11`', where it matches". Isolated:

```
$ printf 'a ten-part gate\n' > tenpart.txt && grep -cE '...' tenpart.txt
0        (exit 1)
$ printf '11 parts\n' > elevenparts.txt && grep -cE '...' elevenparts.txt
1        (exit 0)
```

`[0-9]+-part` requires digits, so `ten-part` does not match. On a single line
carrying all three tokens the line matches anyway, which masks the dead member.
This is not a live defect - my independent word-spelled sweep found no gate count
the audit misses - but `ten-part` is exactly the form the OLD failing grep
searched for, so naming it in the control implies a coverage the expression does
not have, and a future plan writing "ten-part gate" would slip past.

**Change:** either drop `ten-part` from the control's description, or add
`(one|two|...|twelve)-part` to the expression and re-run the count.

## HARVEST

**A fix round's own controls need the same scrutiny as the thing they fix, and
this round proved it twice in opposite directions.** The corpus measurement's
first pass came back clean from a pipeline that matched its own `git grep`
prefix - a false absence produced by the instrument's plumbing rather than by its
pattern - and it was caught only because someone fired it. In the same round, the
replacement for F2 shipped a control naming a token the expression cannot match
(N3). The generalizable handle: **a control that bundles several tokens onto one
line reports a match for the line, not for each token**, so a compound control
proves only that at least one member fires. Fire each alternative separately, or
describe the control by what actually matched.

**A guard and a counter that observe the same mutation will both speak, and a
fire that forbids one of them is over-specified.** N1 is not a mistake about the
check; it is a mistake about what a single mutation produces. When a fire is
written for guard X, the other checks that X's mutation also trips have to be
listed as expected output, or the fire's acceptance is unmeetable. The trigger is
readable: the fire's sentence contains the word "NOT" about a neighbouring
check's output.

**One measurement discipline that paid off here and is worth keeping.** My first
probe against the F12 fix appeared to refute it, because I truncated a 200-plus
character spec line before the quoted sentence began. Reporting that would have
cost a round on a correct fix. The handle: **when a probe seems to refute a
claim about a long line, re-run it without the truncation before writing it
down** - `cut` and `head` are display conveniences that silently become part of
the measurement.

---

# Delta review after fix round 2

**Verdict: NEEDS FIXES**

Range: `afaf9a0` -> `58d0c88`, plan document only, still 619 lines. Confirmed
`58d0c88` is HEAD and the plan file has not moved since; the only other change
in the range is `docs/decision-ledger.yaml`, which no task touches.

Reviewer scratch path for this pass, distinct from both earlier ones:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-delta2-indep/`
(`one.txt`, `neg.txt`, `dash3.txt`). Every count below was recomputed against the
moved tree; no earlier scratch file was re-run.

**All three delta-round-1 findings are closed, and the two highest-bar items -
N3's per-alternative fire and the new self-reference - reproduce exactly.** The
verdict is NEEDS FIXES on one number, found in this pass, that neither of us
caught earlier: the self-review's count list says Task 1 has four fires, and it
has five.

## The three findings

| # | Status | Evidence |
|---|---|---|
| N1 | **Closed** | F5 now reads "its violations must INCLUDE the message that the counter does not model shell continuations; the accompanying house-block and total mismatches are expected, because the guard does not suppress the comparison", and names the failure the fire catches ("a run that reports only a count mismatch, with no continuation message"). It goes further than the routing asked and spells out the arithmetic: `python3 \` and its continuation are both non-empty and neither starts with `#`, so house counts 2 against a stated 1 and the total counts 6+4+2=12 against a stated 11. I checked that arithmetic against Step 2 and it is right. The continuation case is deliberately left OUT of the new skip rule, which is what makes F5's three-violation expectation correct rather than accidental. |
| N2 | **Closed, and extended correctly** | Step 2's comparison bullet now states that a block whose marker was not found is reported once and that BOTH that block's comparison AND the total comparison are skipped, with the reason (with a block missing, the counted total is not derivable), plus the explicit "the block is not treated as zero commands" and the same rule for an unterminated fence. |
| N3 | **Closed** | See below. |

## N3's fix, which carried the higher bar

The expression gained a spelled-number alternation and the control was rebuilt
one token per line. Re-measured:

```
$ grep -cE 'Sum \`|[0-9]+ parts|[0-9]+-part|(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)-part|six parts|\`6\`|\`4\`|\`1\`\.' <plan>
9
```

at `:59`, `:60`, `:62`, `:96`, `:182`, `:190`, `:224`, `:240`, `:619` - the same
nine lines the extension was supposed to leave untouched, so the widening added
coverage without adding hits.

Each alternative fired separately, each token alone on its own line, which is the
thing the old compound control could not prove:

```
a ten-part gate                    -> 1
the eleven-part block              -> 1
11 parts                           -> 1
Sum `11`                           -> 1
six parts                          -> 1
the `6` count                      -> 1
the `4` count                      -> 1
ends in `1`.                       -> 1
the frontend checks run in order   -> 0
```

`ten-part`, the token that could not match in round 1 and that named the exact
form the original failing audit searched for, now matches on its own. The
negative control returns zero, so the expression is not matching everything. The
plan's reported figures are reproduced without exception.

## The two tightened acceptance sentences, judged

F3 and F4 now read "its violations must be EXACTLY ONE". Both are **correct**
against the fixed Step 2, and I checked each by walking the state rather than
accepting the claim:

- **F3** deletes the total marker. Step 2's new clause skips every comparison
  when no stated numbers exist, and the three block markers, their fences and
  the continuation guard all still pass on an otherwise-clean tree. One
  violation.
- **F4** deletes the frontend block marker. The frontend and total comparisons
  are skipped; the rust comparison (stated 6, counted 6) and the house
  comparison (stated 1, counted 1) still run and are clean; the total marker is
  present and matches. One violation.

Tightening these beyond the routing was the right call and it is reported rather
than slipped in. "Name the missing marker" was satisfiable by a run emitting
three violations, one of which happened to name the marker - which is N1's defect
shape pointed the other way, and the author is right that the skip rule alone did
not close it. F1 and F2 were correctly left untightened: F1 produces exactly one
violation and names it, F2 produces exactly two and names both.

## The half of N2 the author found itself

Correct catch, and mine was the incomplete half. My finding named the missing
BLOCK marker. The identical hole existed for a missing TOTAL marker: the old
"the stated total" bullet said a missing marker is a violation, and the old
comparison bullet then said "the stated total [is compared] against the sum of
the three counted blocks" with no stated numbers in existence. Fire F3 walks into
that state by construction, exactly as F4 walks into the block case. The new
clause covers both and says which fire measures which.

## The new self-reference, verified

Adding `x 6 Rust y` as a quoted scratch line made the audit sentence match its
own companion expression. Measured:

```
$ grep -nE '[0-9]+ (Rust|frontend|house-knowledge)' <plan>
182:The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge. ...
619:Coverage: all five brief work items appear ...
```

Two lines, exactly as the plan now states, against the one line the same sweep
returned in my delta round 1. Negative control on a line with no counts returns
zero. The plan's handling is right on both halves: the count was corrected by
re-running after the edit rather than carried over, and the sentence says which
of the two hits is itself.

## New finding

### Minor

**M1. The self-review's count list says Task 1 has four fires; it has five.**
Location: plan `:619`, the counts enumeration.

```
$ grep -cE '^\s+- \*\*F[0-9]+,' <plan>
5          (F1 :240, F2 :241, F3 :242, F4 :243, F5 :244)
$ grep -n "Step 4: fire-verification" <plan>
239:- [ ] **Step 4: fire-verification, five fires, ...**
$ grep -oE "[0-9]+ fires in Task 1" <plan>
4 fires in Task 1
```

Step 4 says five and enumerates five. The count list says four. The same
sentence opens "Counts recomputed from their enumerations at fix round 1, each
by counting its own list in this file rather than from memory" - and F5 was
*added* at fix round 1, in response to F8, so the count did not follow the
enumeration it claims to have been recomputed from.

The neighbouring phrase "W3 into the statement plus its four fires" is a
different statement and is **accurate**: the acceptance map really does carry
five W3 rows, one statement plus four fire rows (W3-b/F1, W3-c/F2, W3-d/F3,
W3-e/F4). What that exposes is the real asymmetry underneath the wrong number:
**F5 is the one fire with no acceptance-map row.** Verified - no W3 row mentions
F5 or the continuation guard.

Nothing an implementer executes depends on this: Step 4 is unambiguous at five.
The defect is confined to the sentence whose function is to certify that the
counts were checked, which is the same class as F2 and N3 and the reason it does
not get waved through after three rounds.

I record plainly that this is a miss of my own from delta round 1 rather than
something this round introduced: I verified the acceptance map at 18 rows and
audited the gate-count expression, and did not walk the rest of the count list.

**Change, either form:** correct the number to "5 fires in Task 1" and add a
clause saying F5 is a robustness fire on the counting rule rather than an
acceptance half, so the four-row map and the five-fire step stop looking like a
contradiction; or add a W3-f row for F5 and make both numbers five. The first is
the smaller change and matches what the map already means.

**Every other count in that list recomputed clean**, since I was there: 5 tasks,
5 work items, 18 acceptance-map rows, 4 brief corrections, 4 mutations (M1-M4),
4 candidate producers (P1-P4), 24 corpus lines across 16 files, 4 `ledger-lint`
self-descriptions, 7 packageRules entries, 4 anchor items, 16 files in Task 5.
Hygiene also holds: zero ticked checkboxes against 29 unticked, no typographic
tells (count 0, exit 1, with the same pattern fired at count 1 on a scratch
em-dash), no literal model name, and `git add -A` appearing only inside its own
prohibition.

## HARVEST

**A count that names a set has to be re-run when the set grows, and the fix round
that grows the set is the moment it goes stale.** F5 was added in response to a
review finding; the sentence enumerating "4 fires in Task 1" was in the same
document, three hundred lines away, and stayed. The trigger is readable and
belongs with the edit rather than with a later audit: **you just added a member
to a set this document counts somewhere else.** The existing house rule about
sweeping consuming references covers this once the writer thinks of the count as
a consumer of the enumeration, which is precisely the framing that goes missing
when the enumeration is a list of fires rather than a list of files.

**My own probe error this pass, recorded because it is the mirror of the one I
warned about last round.** My first count of acceptance rows returned 23 against
the plan's stated 18, and for about a minute that looked like a finding. The
expression was right and the SCOPE was wrong: `^\| W[0-9]` catches the work-item
coverage map's five rows as well, and 18 + 5 = 23. Last round I truncated a line
and nearly reported a correct fix as broken; this round I under-scoped a search
and nearly reported a correct count as wrong. Same lesson from the other side:
**when a probe disagrees with a stated count, suspect the probe's boundaries
before the count**, and re-derive with the boundary made explicit. Both near
misses were caught by the same habit - re-running the probe a second way before
writing the finding down - which is worth more than either individual catch.

---

# Delta review after fix round 3

**Verdict: NEEDS FIXES**

Range: `58d0c88` -> `60edd07`, plan document only, 619 -> 620 lines. `60edd07`
is HEAD and the plan has not moved since; the only other change in the range is
`docs/decision-ledger.yaml`, which no task touches.

Reviewer scratch path for this pass:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-delta3-indep/`
(`neg.txt`, `pos.txt`, `ctl.txt`, `dash4.txt`).

**M1 is closed on both halves the ruling required, and the framing repair is
right.** One new Minor, created by this round's own edit and confined to the same
sentence: the self-review's companion-sweep evidence cites plan line `:182`, and
the row inserted at `:152` moved that line to `:183`.

## M1, both halves

**The count.** Task 1 enumerates five fires (`F1`-`F5` at `:240`-`:244`); the
count list now reads `5 fires in Task 1`; Step 4's header reads "five fires";
zero instances of the old `4 fires in Task 1` survive. Consistent in all three
places.

**The row.** `W3-f` is present, and it states a genuine observable rather than a
robustness note. The distinction holds against its neighbour: `W3-c` is "a gate
block that gains or loses a command turns the gate red" (a count that changed
and was caught); `W3-f` is "a shell continuation inside a gate block makes the
check REFUSE rather than silently miscount" (an input the counting rule cannot
model, where the alternative to refusing is a plausible wrong number). The second
is not a subset of the first, and the framing the author gives it is correct
against W3's own origin: the ROADMAP records that this work item exists because
`BUILDING.md` enumerated ten while every consumer said eleven, so a wrong number
that looks right is precisely the failure class. Producer named (`F5`), MV column
`yes`.

**The row count.** Re-measured with the boundary made explicit rather than read:

```
acceptance-map rows (section-scoped, ^| W[0-9]-): 19
['W1-a','W1-b','W1-c','W1-d','W2-a','W2-b','W2-c','W3-a','W3-b','W3-c','W3-d',
 'W3-e','W3-f','W4-a','W4-b','W4-c','W5-a','W5-b','W5-c']
coverage-map lines matching ^| W[0-9]- : 0
coverage-map lines matching ^| W[0-9]  : 5
whole file ^| W[0-9]- : 19      whole file ^| W[0-9]  : 24
```

Nineteen, and W3 carries six rows (statement plus one per fire). The `-`
requirement is what makes the whole-file sweep safe: without it the coverage
map's five rows join in and produce the 24 that misled me last round. The harvest
item was adopted and it works.

**Both places carrying the number agree**: "nineteen acceptance halves" and "19
acceptance halves", with zero surviving "eighteen" or "18 acceptance".

**Nothing else the new row could have disturbed did.** The gate-count audit still
returns nine, at `:59 :60 :62 :96 :183 :191 :225 :241 :620` - the same nine
sites, shifted by one below the insertion - and the `W3-f` row itself matches the
audit expression zero times, so the widening added a row without adding a hit.
Positive control 1, negative control 0. The companion sweep still returns two.

## The framing repair, judged

**Keep it, including the superseded quotation.** The clause now reads "a row per
fire, five of them - one per FAILURE MODE the check must catch, which is a wider
set than the two anchor kinds", and records that the old wording "four fires, one
per anchor KIND" was true of F1-F4 and silently excluded F5. That is the better
half of the fix: the number was the symptom, the anchor-kind framing was the
cause, and a corrected number alone would have left the next reader free to
re-derive the same four.

Keeping the superseded wording rests on the same basis I approved twice already -
the `latitude-carveout-presentation-tokens` reading kept as explicitly rejected
(F6), and the F2 rewrite recording that its predecessor could not fire. In all
three the quotation is marked as superseded at the point of use, so it reads as
history rather than as a live claim, which is the same treatment the plan gives
the retired plan documents in its Rust-gate evidence line. The disclosed cost is
real and correctly disclosed: a grep for "four fires" hits that one instance, and
I measured it at exactly one. No sweep the plan states searches for fire counts,
so nothing currently consumes it.

**Staleness corrected**: "re-walked at every fix round through round 3",
"recomputed from their enumerations at fix round 3", "Absence checks, swept at
fix round 3". No "fix round 1" survives in a describing position.

## New finding

### Minor

**M2. The self-review cites its own line `:182`, and this round's insertion moved
that line to `:183`.** Location: plan `:620`.

The sentence reads: "Companion sweep ... `grep -nE '[0-9]+ (Rust|frontend|house-knowledge)'`,
returns two lines: `:182`, the fence itself, and this sentence". Measured:

```
$ grep -nE '[0-9]+ (Rust|frontend|house-knowledge)' <plan> | cut -d: -f1
183 620
$ sed -n '182p' <plan>
<!-- gate-total; checked by scripts/ledger-lint.py -->
$ sed -n '183p' <plan>
The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three
$ git show 58d0c88:<plan> | sed -n '182p'
The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three
```

`:182` was correct at `58d0c88` and is wrong at `60edd07`, because `W3-f` was
inserted at `:152`. The sweep the sentence describes does not return `:182`; it
returns `:183`. This is the plan's **only** self-pointing line citation - I swept
every backticked line citation in the document and classified each, and all the
others point at `BUILDING.md`, the spec, `release.yml`, `README.md`,
`package.json` or the corpus source files, none of which moved. The control for
that sweep fires against a scratch line carrying the same form.

To be precise about what this is and is not: the convention's scope boundary does
NOT ban it. A plan is a process artifact, and process artifacts may cite a line.
The defect is narrower and purely factual - the evidence line states a number
that the command it quotes does not produce, inside the sentence whose entire
function is to certify that the sweeps were re-run.

The author's own catch this round was the count self-reference: adding a scratch
line changed what a sweep counts, so the count was re-run rather than carried
over. The line self-reference is the same mechanism one step over - adding a ROW
changed what the line numbers are - and the re-derivation covered counts but not
citations.

**Change, and prefer the durable form over a corrected number:** replace `:182`
with the symbolic reference the same sentence already uses elsewhere, e.g. "the
canonical fence in Task 1 Step 1(a), and this sentence". Bumping 182 to 183
restores correctness until the next inserted line; naming the fence is immune to
it, and the sentence already refers to `Step 1(f)` and `fire F1` symbolically two
clauses earlier.

**Everything else re-derived clean:** 5 fires, 19 acceptance rows in two agreeing
places, 9 audit sites with both controls behaving, 2 companion-sweep hits, one
superseded quotation, zero ticked checkboxes against 29 unticked, and no
typographic tells (count 0, exit 1, with the same pattern fired at count 1 on a
scratch em-dash).

## HARVEST

**A document that measures itself has two kinds of self-reference, and fixing one
does not fix the other.** The author caught the COUNT self-reference this round -
adding a quoted scratch line changed what a sweep counts, and the count was
re-run rather than carried forward - and wrote it into the ledger. The LINE
self-reference is the same mechanism displaced by one: adding a row changed what
every line number below it is, including one the document had written down about
itself. The generalizable handle, and it is readable at the moment of editing:
**an insertion invalidates every self-citation below it, a quotation invalidates
every self-count that searches for what it quotes.** A re-derivation pass that
covers counts has done half the sweep.

**The durable answer is the one this package is already shipping to the
codebase.** W2's whole content is that a locator by line number goes stale
because lines move, and the repair is to name the thing instead. The plan's own
self-review is a small instance of exactly that, in the one place a plan is
allowed to use a line number and does not need to. When a document must cite
itself, the symbol form costs nothing extra and removes the failure class - which
is the argument the plan makes to the codebase and had not yet applied to its own
last paragraph.

---

# Delta review after fix round 4

**Verdict: APPROVED**

Range: `60edd07` -> `09c37b3`, plan document only, one clause on `:620`, still
620 lines. `09c37b3` is HEAD and the plan has not moved since. The working tree
carries one modified file, `docs/process-journal.md`, which is the controller's
own uncommitted session-close writing and is out of this delta's scope; no task
in this plan may edit it in any case (Global Constraints).

Reviewer scratch path for this pass:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-delta4-indep/`
(`spanfire.txt`, `spanneg.txt`).

M2 is closed. I found nothing new. Every finding across the plan review and four
delta rounds is closed, and the plan is fit to go to the owner.

## 1. Does the anchor resolve, and is it drift-proof?

**It resolves, exactly.** Measured:

```
162:## Task 1: `BUILDING.md` states the gate total once, and `ledger-lint` checks it (W3)
177:- [ ] **Step 1: `BUILDING.md`.** Five edits, each fenced here, plus one deliberate non-edit at (f).
179:  (a) Insert, immediately after the paragraph that ends ...
183:The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three

$ grep -nE '[0-9]+ (Rust|frontend|house-knowledge)' <plan> | cut -d: -f1
183 620
```

"The canonical gate-total sentence fenced in Task 1 Step 1(a)" points at `:183`,
which is precisely the first of the two lines the sweep returns. The author's
reported `:179` / `:183` are both exact.

**Drift-proof in the way a number is not, and the asymmetry is the point.** The
anchor is built from a task number, a step number, a sub-label and a description
of the artifact; a line inserted anywhere in the document moves none of them.
The number form failed on a single row inserted four hundred lines away, which
is the characteristic failure: silent, remote, and invisible to the person who
caused it. The symbolic form can only break if Step 1(a) is relabelled or the
sentence deleted, and both are content edits made by someone who is looking
straight at them. Loud and local beats silent and remote, which is the whole
content of W2 restated for a document instead of a comment.

Worth stating honestly rather than glossing: the symbolic anchor is not
machine-checkable the way a line number trivially is. It does not need to be.
The clause is prose in a self-review, and the evidence it carries is the quoted
command, which any reader can re-run - as I just did.

## 2. Does the kept sentence of record earn its place?

**It earns it.** The test is `conventions.md`'s: historical commentary stays only
where it carries ongoing value, never as self-purpose, and "a line that just
records what changed is token waste." This clause does not just record what
changed. It does three things with forward value:

- It states the standing reason the reference is symbolic, which pre-empts the
  obvious regression - a later editor restoring a "more precise" line number.
- It names the rejected alternative and why it loses: "a bumped number would have
  been correct today and wrong at the next insertion." That is the house pattern
  of keeping a losing argument so it is not rediscovered, and it is the third
  time this plan has used it in a form I have already approved - F6's explicitly
  rejected `latitude-carveout-presentation-tokens` reading, F2's record that its
  predecessor could not fire, and F3's superseded "four fires, one per anchor
  KIND". Rejecting it here would be inconsistent with those three.
- Its factual claim is true: I verified that `W3-f`'s insertion at `:152` moved
  the target from `:182` to `:183`.

The "W2 in miniature" framing is accurate rather than inflated. W2's content is
that a line-number locator goes stale because lines move; the incident is a
line-number locator going stale because a line moved. Same structure, one clause,
no claim beyond what happened. Self-indulgence would be a paragraph about the
process; this is a sentence about the form choice, which is what a later editor
needs.

## 3. The sweep, re-measured

**The substantive claim holds: no self-pointing citation survives.** I classified
every colon-digits citation in the document by resolving each to its target
rather than counting them:

- 46 bare-span occurrences on 13 lines, plus 27 filename-attached citations.
- Every one resolves to a **different** repo file: `BUILDING.md`, four retired
  plan documents, the twenty corpus source files, the spec at the named commit
  `de4ea38`, `release.yml`, `README.md`, `package.json`, `ci.yml`,
  `.github/release/draft-body.md`.
- **Zero point at the plan itself.**

The sweep fires: against a scratch line carrying `` `:182` `` and
`` `:1517-1535` `` it returns both; against a line with no span it returns none.

One residual `:182` match exists and is not a citation. Extracted in context:

```
$ grep -n ':182' <plan> | grep -oE '.{40}:182.{20}'
t HEAD `delta_for` begins at `planner.rs:1820`, so neither cited
```

It is the substring of `planner.rs:1820`, an external citation I verified in
round 1. I checked this rather than reporting the raw `grep -c` of 1 as a
survival.

**On the figure 36, with its unit named**, because the author's report will be
read downstream and my first count disagreed with it. `36` is the number of
**distinct** bare-span values; the number of **occurrences** is 46. Both are
right at their own unit:

```
occurrences: 46
distinct span values: 36
```

The number does not appear in the plan document at all, so nothing in the
artifact depends on it. I record both figures with their units rather than
calling either wrong - the disagreement was my probe's unit, not the author's
count, which is the same lesson my delta-2 harvest recorded from the other side.

## 4. Nothing else drifted

Re-derived against the moved document: gate-count audit still **nine** sites;
acceptance rows still **nineteen** under the scoped `^| W[0-9]-` probe; fires
still **five** with `5 fires in Task 1` stated; companion sweep still **two**
lines; zero ticked checkboxes against 29 unticked; no typographic tells (count 0,
with the pattern fired at count 1 on a scratch em-dash).

## Closing statement

Across the plan review and four delta rounds this artifact took fourteen findings
plus three plus one plus one, and every one is closed with evidence I reproduced
rather than read. The two that mattered most - the coverage walk and the
self-audit that could not fire - were both settled by measurement, and the
corpus, the four brief refutations, the D102 halves, the disjointness claim and
every count in the self-review now reproduce under an independent instrument.

**APPROVED.** It goes to the owner.

## HARVEST

**A number without its unit is not a measurement, and this closed on that note
twice from opposite directions.** The author's 36 and my 46 were both correct
about the same sweep; one counted distinct values and one counted occurrences,
and neither said so. Earlier in this chain the same shape cost more: a corpus
counted 17 because its search enumerated cited file extensions rather than
citations, and a fire count sat at four because it counted acceptance rows rather
than fires. The handle is cheap and belongs at the moment the number is written,
not at the audit: **write the unit into the sentence** - "36 distinct spans", "46
occurrences", "20 matched lines across 13 files" - because a bare integer forces
every later reader to re-derive the boundary, and most will re-derive a different
one and call it a defect.

**The last finding of a long review chain is worth more than its size suggests.**
M2 was one wrong line number in prose no implementer reads, and fixing it
produced the one change in this whole package that makes the document itself
obey the rule it is about to enforce on the codebase. Reviews that stop when the
executable content is clean leave exactly this class behind, and it is the class
that quietly teaches the next author that the rule has exceptions where it is
inconvenient.

---

# Delta review after amendments 1 and 2

**Verdict: NEEDS FIXES**

Range: `09c37b3` -> `6bbead0`, plan document 620 -> 695 lines. `6bbead0` is HEAD
and the plan has not moved since. My APPROVED verdict stands for everything
outside these two changes.

Reviewer scratch path for this pass:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-amend12-indep/`
(`plan.diff`, `dnums.txt`, `plan_str.txt`, `road_str.txt`, `nearmiss.txt`,
`dash5.txt`).

Both amendments are sound and every empirical claim in them reproduces, including
the refutation of the controller's brief. One Minor: Task 4's `Read first` line
is the one header element neither amendment swept, and it is now narrower than
the task's Files list and its governing ROADMAP entries.

## 1. The unit question, re-measured

**Every figure verifies exactly.** Decision series, the plan's own command run
verbatim:

```
defining headings: 104
distinct numbers:  103
max:               D105
duplicated:        D32
missing in 1..105: D73, D74
```

104 headings, 103 distinct, reaching D105, the duplicate is `D32` (its addendum
heading) and the gaps are exactly `D73`/`D74`. The non-contiguity claim holds in
every element, so "D1 through D105" would indeed assert 105 decisions where there
are 103.

Verdict count, both units and the historical fork:

```
basename rule today:            219
verdicts/ directory today:       78   (frozen)
basename rule at 62aaf61:        78
verdicts/ directory at 62aaf61:  78
README at 62aaf61:              "D1 through D35" / "all 78 of them"
```

**The brief's premise is correctly refuted.** `78` was not a number that never
counted the right unit; it was true when written, when both units returned 78
because every verdict then lived in a `verdicts/` directory. The three boundary
checks all return 0 with controls that fire: non-markdown matches 0 against 219;
`brief` matches 0 against 185 briefs in `docs/`; files under `verdicts/` whose
basename lacks `verdict` 0 against 78 such files. I also cross-checked the unit
itself - the 219 basenames are `task-N-verdict.md`, `task-N-review-verdict.md`,
`whole-branch-verdict.md`, `amendment-N-verdict.md` and kin, and the independent
path-contains-`verdict` rule returns 219 too, so the two natural definitions
agree.

**Does the step defend, or merely name the hazard? It defends, by three
mechanisms rather than by warning.**

- **The unit is not the implementer's to choose.** The command is fenced and
  returns 219. Arriving at 78 requires abandoning a fenced command, which is a
  visible deviation, not a silent confirmation.
- **Boundary check three is the structural fact, not a caution.** "No file inside
  a `verdicts/` directory is missed by the basename rule" establishes that the
  basename set is a strict SUPERSET of the frozen set. Once that is measured, the
  fork is a property of the two numbers rather than something a reader has to
  notice.
- **Both numbers must appear in the report**, and the frozen-unit command is given
  inline, so the reviewer sees 219 and 78 side by side before forming a view.

The first figure is defended differently and correctly so: its hazard is a
range-versus-count conflation rather than a forked unit, and the step names the
exact wrong output ("D1 through D105 would be a fresh wrong number") instead of
leaving it to be discovered. Naming the specific wrong answer is the strongest
available form for both.

One consistency note, not a finding: the step fences the fact set and the
requirement that each figure name its unit, and leaves the sentence's wording to
the implementer. That is the same closed-fact-set/open-wording pattern already
approved for W5-a..c and grounded the same way.

## 2. The renaming sweep

**Complete.** Four sites carry the new name: the Goal (`:11`), the model-tier row
(`:50`, "4 (W5: user-facing docs)"), the coverage map's right column (`:111`) and
the task heading (`:488`). Exactly two "README accuracy" occurrences survive and
both are correct:

- `:111`, the coverage map's LEFT column, which is the work item's name from the
  plan brief - correctly not rewritten, since the brief is not this plan's to
  edit - with the rename stated in the same row's right column.
- `:685`, the amendment-2 log recording the rename.

**The two names cannot read as two things** because the single place they meet is
that coverage-map row, and it states the relation explicitly. (My first sweep for
this used a fixed-width context pattern and returned nothing, which would have
been a false clean; the `:111` hit has only nine characters before the phrase.
Re-run without the context requirement, with a fired control on a phrase known
present.)

## 3. The three renumberings

**Every reference resolves.** Task 4 now carries Steps 1-8, enumerated from the
headers, contiguous, no duplicates. Each cross-reference checked individually:

| citation | site | resolves to |
|---|---|---|
| `(Step 3)` | acceptance row W5-c | Step 3, the anchor step, unmoved by either amendment |
| `Task 4 Step 4` | W5-d, W5-e | the amendment-1 counts step |
| `Task 4 Step 5` | W5-f | the amendment-2 INSTALL.md step |
| `Task 4 Step 5` | close actions, QA disposition | same |
| `in Step 5` | Task 4 Files list, INSTALL.md entry | same |
| `RE-measured in Step 1` | Step 2 | Step 1, unmoved |
| `Step 1's table` | Step 2's last bullet | same |
| `corrected in Step 4 above` | Step 6, "what stays untouched" | the amendment-1 step |

No dangling reference, and both amendment logs' renumbering claims (4 -> 5,6,7
then 5 -> 6,7,8) match the headers. The acceptance map recounts to **22** from its
own list (W1 4, W2 3, W3 6, W4 3, W5 6), stated in both places that carry it
("twenty-two", "22 acceptance halves"), with no stale "21 acceptance" or
"nineteen" surviving. The single "twenty-one" is inside the amendment-1 log as a
dated record of that amendment's own state, which is correct history rather than
a live claim.

## 4. The warning string

**Byte-identical, verified by comparison rather than by eye**, one occurrence in
each document:

```
$ cmp plan_str.txt road_str.txt && echo BYTE-IDENTICAL
BYTE-IDENTICAL
$ od -c plan_str.txt | tail -2
0000100   c   o   m   m   a   n   d   l   i   n   e  \n
```

Near-miss control, one character changed (`OpenPGP` -> `OpenPGp`): `cmp` rejects
it at byte 24, it is absent from the ROADMAP, and the exact string is present.
So the check distinguishes the string from a plausible neighbour, which is the
property that matters when a user will grep it.

## 5. The two decisions the author made rather than absorbed

**`dnf`, not `rpm`: correct, and the correction landed at the source.**
`docs/INSTALL.md:82` documents `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`,
and `@commandline` is dnf's pseudo-repository for a package given by path, which
is what makes the tool identifiable from the warning text at all. The ROADMAP
entry now says so explicitly and records the flag. Reading it as the `rpm` binary
would have put the wrong tool in front of a reader who is already unsure whether
they have a real problem.

The paired restraint is the better half and is not trivial: `docs/INSTALL.md`
documents `sudo apt install ./...deb` two lines above the Fedora command, so
"deb and rpm alike" is a live temptation. Nobody measured what apt prints. A
sentence covering both would have carried the authority of a QA pass over a
guess, and declining it is right.

**The file-top enumeration: correct, and it is a genuine third member.** The
comment at `docs/INSTALL.md:22-24` enumerates exactly two sections that shrink
when signing lands - SmartScreen and Gatekeeper. A signed rpm would remove the
dnf OpenPGP warning by the same mechanism, so the new Linux note belongs to that
class and would otherwise be stranded by the 1.x signing work. Handled the same
way the fix-round-1 F3 ruling settled `ledger-lint`'s self-descriptions: a named
region in the Files list, not a general licence. Consistent with a decision I
already approved.

## 6. The wrapped-locator defect the author found in its own text

**Real, and correctly repaired.** The comment is hard-wrapped across three lines,
so the unwrapped sentence returns 0 from a line-based grep. The joined form is
byte-identical to the plan's quotation, which I verified programmatically rather
than by reading (`joined == plan quote: True`). The replacement locator `code
signing lands` sits on line 22 and occurs exactly once in the file, so it
resolves uniquely. The cited entry `proc-wrapped-prose-quote-grep` exists in
`docs/process-conventions.yaml` and its statement covers this case in terms
("line-based greps false-negative on soft-wrapped sentences"), so the citation is
accurate and not decorative.

## 7. Nothing else drifted

Gate-count audit still **nine** sites; companion sweep still **two**; Task 5's
Files list and `git add` set still set-equal at 16; **Task 4's Files list and
`git add` set are set-equal at two** (`README.md`, `docs/INSTALL.md`), so
amendment 2 propagated into the commit block; zero ticked checkboxes; no
typographic tells with the pattern fired on a scratch em-dash; no literal model
name; `git add -A` only inside its prohibition.

## New finding

### Minor

**A1. Task 4's `Read first` line is the one header element neither amendment
swept.** Location: plan `:490`.

It reads: "the plan brief section 4 W5; `docs/ROADMAP.md`'s README entry in the
Pre-1.0 release gates section, including its owner split of 2026-07-29 and its
Content anchors block; `README.md` in full; the authoring section's README
verification block."

Two things the task now depends on are absent from it:

- **The ROADMAP's OWNER QA PASS, round 1 entry** (`docs/ROADMAP.md:872`), which
  carries the verbatim warning string and the ruling, and which Step 5 explicitly
  orders the implementer to verify against. This is the sharper half: every other
  task's `Read first` names the ROADMAP section that governs it - Task 1 the
  "Gate-count derivation" section, Task 3 the Renovate entry and its two riders,
  Task 5 the "Docs accuracy" entry - and Task 4 is now governed by two ROADMAP
  entries while its header names one.
- **`docs/INSTALL.md`**, which the task now edits. `README.md` is named "in full"
  beside it, and Task 1 names both of its edited documents the same way.

This is not a fork: the Files list names `docs/INSTALL.md` and Step 5 names the
QA entry, so a compliant implementer reaches both. The defect is that the header
whose function is to front-load the required reading now describes half the job,
so an implementer forms its plan from a README-only picture and meets the second
document at Step 5.

It is also the same class this plan has spent four rounds closing: a standing
statement whose consuming references were not swept when the set it names grew.
The amendment-2 log enumerates what moved - the task name, the model-tier row's
ground, the Files list, the new step, `W5-f`, the close actions - and `Read
first` is not in that enumeration.

**Change:** extend the line with the ROADMAP's OWNER QA PASS round-1 entry and
`docs/INSTALL.md` (its Linux section and its file-top comment, matching the
scoping the Files list already states).

## HARVEST

**A rename sweep and a scope sweep are different sweeps, and the second is the
one that gets missed.** Amendment 2 swept the task's NAME across four sites
correctly and completely, including the awkward one where the old name has to
survive because it belongs to the brief. What it did not sweep was the set of
things the task must READ, which grew at the same moment and for the same reason.
The two feel like one job while editing and are not: the name is a string with
occurrences you can grep, the reading list is a set with members you have to
re-derive from the task's new content. **The readable trigger is the Files list:
if a task gained a file, its `Read first` and its governing-document list are
both consumers of that change**, and neither is findable by grepping for the old
text, because nothing is wrong with the old text - it is merely no longer
complete.

**Refuting the brief a second time, on a premise about history rather than about
the tree.** Every earlier refutation in this package corrected a claim about the
current state - a count, a default, a documented invocation. This one corrected a
claim about how a number came to be wrong, and it needed a measurement at a
commit from eighteen days earlier to settle it. The distinction earned its keep:
"never counted the right unit" and "was right and its unit forked" prescribe
different steps, and only the second one produces the both-numbers-in-the-report
requirement that makes the fork visible to a reviewer. **When a brief explains
WHY a fact is wrong, that explanation is itself a claim with a measurable
history**, and the archaeology is usually one `git ls-tree` away.

---

# Delta review after the inputs-list sweep

**Verdict: NEEDS FIXES**

Range: `6bbead0` -> `3a5680c`, plan document only, ten lines across five hunks.
`3a5680c` is HEAD and the plan has not moved since.

Reviewer scratch path for this pass:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-inputs-indep/`
(`inputs_probe.py`, `d6.txt`).

A1 is closed, the three further members are closed, both finds beyond the routing
are correct, and both exemptions hold. One Minor: re-running the sweep with my
own probe finds three more members of the same class in Task 4, in the very step
this commit edited.

## The probe, re-derived my own way

I did not re-run the author's instrument. I wrote one that splits the plan on
task headings, strips each task's `Read first` line, extracts every backticked
token from the remaining body that resolves against `git ls-files` (resolving
bare basenames where they are unambiguous and flagging them where they are not),
extracts every backticked token that resolves against the four house YAML files'
id set, and diffs both against that task's `Read first` line and Files list.
Result:

| task | body-named repo files in neither list | body-named house ids not in `Read first` |
|---|---|---|
| Task 1 | none | none |
| Task 2 | `BUILDING.md`, `crates/muxsmith-cli/tests/dry_run_cli.rs` | `comments-locate-by-symbol-never-by-line-number` |
| Task 3 | `BUILDING.md`, `deny.toml` | none |
| Task 4 | `BUILDING.md`, `crates/muxsmith-cli/src/cli.rs`, `crates/muxsmith-cli/src/commands/mod.rs`, `crates/muxsmith-cli/src/commands/run.rs`, `crates/muxsmith-cli/tests/run_live.rs` | `latitude-carveout-presentation-tokens` |
| Task 5 | `BUILDING.md`, `run.rs` (ambiguous) | none |

**The derivation is sound and the routed fixes land.** Task 1 comes back clean -
`ci.yml` and `latitude-carveout-zero-content-structural-forks` are both in its
`Read first` now. Task 5's `README.md` addition is there, so its only residuals
are the universal `BUILDING.md` and the `run.rs` token, which is not an input at
all: it is the ambiguous citation quoted from the comment Step 2 rewrites, and my
probe flags it as ambiguous for exactly the reason Step 2 exists.

Most of the rest is correctly out of scope on inspection: `deny.toml` appears in
Task 3 only in a negative scope statement ("no edit to `deny.toml`");
`latitude-carveout-presentation-tokens` appears in Task 4 only as the explicitly
REJECTED reading; `comments-locate-by-symbol-never-by-line-number` appears in
Task 2 only as the ground for a decision about a string it writes.
`crates/muxsmith-cli/tests/dry_run_cli.rs` is reachable through the pointer
`Read first` already carries ("the existing guards named in the authoring
section", and that section names the file with both test names), which is
indirect but resolves.

What does not survive inspection is Task 4's three exit-code source files. See
the finding.

## The two finds beyond the routing

**The `severity_exit` / `job_exit_code` attribution: correct, and measured.**

```
severity_exit    crates/muxsmith-cli/src/commands/mod.rs
job_exit_code    crates/muxsmith-cli/src/commands/run.rs
$ git grep -n "fn severity_exit\|fn job_exit_code" -- crates/muxsmith-cli/src/cli.rs
exit=1        (neither is in cli.rs; fired control: grepping cli.rs for "Cli" returns 1)
```

The old clause read "`cli.rs`'s `Cli` doc comment and the `severity_exit` /
`job_exit_code` functions", which lets all three read as living in one file. Two
of the three do not. The new form names each with its own file and says why.

**Task 2's spec citation: it sits INSIDE the convention's boundary, not near
it.** The new form is "spec section 5.2 "Diagnostics" (the ordering sentence,
`:255` at `de4ea38`)". Three properties, and the third is the one that matters:

- A plan is a process artifact, which is the boundary's own cut ("this governs
  comments in SOURCE files").
- It satisfies the boundary's stated qualifier rather than merely its category:
  the permission is for "a tracker recording a measurement at a named commit",
  and this names `de4ea38`.
- **The line is no longer the pointer.** "Section 5.2 Diagnostics" plus "the
  ordering sentence" locate the target on their own; `:255` is evidence beside
  them. That is what separates this from the `:182` case I flagged two rounds
  ago, where the number WAS the pointer and nothing else in the sentence found
  the target.

Verified factually as well: `### 5.2 Diagnostics` is at spec `:247`, the ordering
sentence is on `:255`, and the spec has not moved since `de4ea38`. And the
routing gap the controller names is real - F12's ruling was scoped to the assert
string, so this instance survived it.

## The two exemptions

**Both hold, and both are principled rather than convenient.**

`BUILDING.md` is a *constraint* for four of the five tasks, not an input: Global
Constraints bind "the gate as `BUILDING.md` enumerates it" across all of them,
and each task's verification step invokes it. Putting it in five `Read first`
lines would restate a global in five local places. The test that settles it is
that the plan already treats it correctly in the one task where it IS an input:
Task 1 edits it, and Task 1's `Read first` names it "in full". Exempt where it
binds, listed where it is read - self-consistent.

`renovate.jsonc` cannot be an input because it does not exist. Task 3's Files
list says `Create:`, `git ls-files` finds no `renovate*` file, and the whole
content is fenced in Step 1. An inputs list names what to read before starting.

**One correction to the dispatch, because it decides whether this recurs.** The
dispatch says both residuals are "left deliberately, with reasons written into
the plan so you do not re-flag them on re-running the same probe". **They are
not in the plan.** I read all five `Read first` lines in full and searched the
document for any exemption reasoning about inputs lists; there is none, and the
searches are not vacuous - the same extraction returns all five `Read first`
lines and the full counts list. The exemptions are still correct, because their
grounds are visible in the artifact (Global Constraints for one, the `Create:`
verb for the other) - but the note whose stated purpose was to stop the next
probe run from re-raising them was not written, so the next run will raise them.

## No count moved, checked

The self-review's counts list enumerates tasks, work items, acceptance halves,
brief corrections, D102 halves, mutations, candidate producers, corpus lines and
files, `ledger-lint` self-descriptions, fires, packageRules entries, anchor items
and Task-5 files. **No entry counts `Read first` members**, so the author's
claim holds. The extraction that establishes it is fire-verified by returning the
whole list rather than nothing.

Nothing else drifted: gate-count audit still **nine**, companion sweep still
**two**, acceptance rows still **22**, fires still **five**, Task 4 still eight
steps, zero ticked checkboxes, no typographic tells with the pattern fired at 1
on a scratch em-dash.

Every anchor the new `Read first` lines add resolves, with a bogus-anchor control
at zero:

```
OWNER QA PASS, round 1          -> 1     OWNER QA PASS, round 9        -> 0
Artifact signing: firm 1.x      -> 1     Artifact signing: firm 2.x    -> 0
  (ROADMAP:1537, under '## v1.x candidates' at :1520 - as the line says)
spec '## 4.'/'## 5.'            -> 2 (Profile format, Planning semantics)
plan '## Amendment 1/2'         -> 2     plan '## Amendment 7'         -> 0
```

## New finding

### Minor

**B1. The sweep's own criterion is not applied to Task 4's Step 1, in the commit
that edited that step.** Location: plan `:490` (`Read first`) against `:500`
(Step 1) and `:511` (Step 3).

Step 1, as rewritten in this commit, instructs: "derive them from
`crates/muxsmith-cli/src/cli.rs`'s `Cli` doc comment, `severity_exit` in
`crates/muxsmith-cli/src/commands/mod.rs`, and `job_exit_code` in
`crates/muxsmith-cli/src/commands/run.rs`". All three are files the implementer
must open to derive the exit-code contract that Step 2 then writes into the
README. None is in Task 4's Files list, and none is in its `Read first`.

Step 3 has the same shape one level down: the new `Read first` adds "the v1
spec's sections 4 and 5, which Step 3 verifies the matching claims against", but
Step 3 says to verify each anchor item "against the spec AND the code", and names
`matcher.rs`, `capability::*` and `profile/validate.rs` as the code half. The
spec half got an inputs entry; the code half did not.

The criterion is the plan's own, not one I am importing. Read-only source files
belong in a `Read first` line when a step depends on their content: Task 2's line
already names `crates/muxsmith-core/src/report/json.rs`'s two builder rustdocs,
and this very commit added `README.md` to Task 5 for exactly that reason - a file
it reads but does not edit, whose anchors a step cannot name without it.

**Where the probe's boundary sits, since that is what this turns on.** Everything
the sweep added is a *document* or a config file - `ci.yml`, `README.md`,
`docs/INSTALL.md`, ROADMAP entries, spec sections, house ids, plan sections. No
source file was added to any task. That is a coherent universe and defensible on
its own; it is simply narrower than the plan's existing precedent, and Task 4 is
where the difference bites, because its steps depend on four source files it
neither edits nor lists.

Nothing is operationally missing: each file is named at its point of use with its
symbol, which is why this is Minor and not more. The defect is that a
class-sweep - the whole subject of this round - left members of its class in the
one task it edited most.

**Change:** extend Task 4's `Read first` with the three exit-code sites
(`crates/muxsmith-cli/src/cli.rs`'s `Cli` doc comment, `severity_exit` in
`crates/muxsmith-cli/src/commands/mod.rs`, `job_exit_code` in
`crates/muxsmith-cli/src/commands/run.rs`) and the core matching symbols Step 3
verifies against, or state the document-versus-source boundary explicitly as a
third recorded exemption alongside the two above.

## HARVEST

**A class sweep needs its universe written down, or the next reviewer re-derives
a different one.** The sweep here was real and it worked - it found three members
the routing had not named and two more beyond it. What it did not do is say what
counts as an input, and because it did not, the boundary it actually used
(documents and config, not source) is only visible by diffing what it added
against what it skipped. My probe used a wider universe, found four more members,
and cannot tell from the artifact whether they were considered and excluded or
never seen. **The cheap fix is one clause naming the universe** - the same fix
the two exemptions needed and did not get. A sweep that records its scope can be
re-run by anyone; one that does not gets re-litigated every time someone points a
slightly different instrument at it.

**An exemption is only durable if it is written where the probe will hit it.**
Both exemptions here are correct, and both were reported to the controller rather
than into the artifact. The report is consumed once; the plan is read by every
later reviewer and by the implementer. The trigger is readable at the moment of
deciding: **you just decided NOT to add something a systematic check flagged** -
that decision belongs next to the thing, because the check will flag it again and
the reasoning will not be there the second time.

---

# Delta review after the widened inputs derivation

**Verdict: NEEDS FIXES**

Range: `3a5680c` -> `6b05f15`, plan document only, seven lines across four hunks.
`6b05f15` is HEAD and the plan has not moved since.

Reviewer scratch path for this pass:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-widened-indep/`
(`widened_probe.py`, `d7.txt`).

B1 is closed on its members: the additions land, the widened derivation returns
nothing outstanding for all five tasks under my own instrument, the `deny.toml`
rejection is right, and the Global Constraints placement contradicts nothing.
**The class is not yet closed**, and the answer to the question you asked is
below with its measurement.

## The central question: is the written rule sufficient to re-derive deterministically?

**No, and one missing distinction accounts for it.** Not a wording quibble - the
gap is measurable in the current lists.

The rule's decision procedure, decomposed:

- **INCLUDE**: a file the task must OPEN to execute a step, document or source
  alike, in order to derive "an anchor, a symbol, a current string or a
  contract".
- **EXCLUDE (1)**: a universal input bound by Global Constraints (`BUILDING.md`).
- **EXCLUDE (2)**: a file the task CREATES; and one named only to say it is NOT
  edited.
- **EXCLUDE (3)**: "A file the task merely EDITS is enumerated in its Files list
  and needs no second entry."

**EXCLUDE (3) and INCLUDE both fire on every edited file in this plan, and the
five lists split on which wins:**

| task | files it edits | of those, also in `Read first` |
|---|---|---|
| Task 1 | 3 | **3** |
| Task 2 | 1 | 0 |
| Task 3 | 1 (created) | 0 |
| Task 4 | 2 | **2** |
| Task 5 | 16 | 0 |

Task 4's line even says it out loud: "`README.md` in full and `docs/INSTALL.md`
in full, **the two documents this task edits**". So the plan lists five edited
files under a rule that says an edited file needs no entry. The word that would
rescue it is "merely" - and deciding whether an edit is "mere" is exactly the
judgment the rule was written to remove. Every edit in this plan requires reading
first: Task 1 must find the current heading to replace it, Task 4 must read the
claims to correct them, and Task 5's Step 2 says in its own words "**Read the
cited code before naming its symbol**", where "a symbol" is verbatim one of the
four things the rule says earns a line.

**The same gap has a second face, and it is the one that proves a distinction is
missing rather than a member.** Task 5 must open the targets its comments cite.
Measured by extracting the cited token from each of the twenty corpus lines and
resolving it against `git ls-files`:

```
cited target        in Files   in ReadFirst
  README.md           False      True
  run.rs              False      True
  identify.rs         True       False
  json.rs             True       False
  lib.rs              True       False
  planner.rs          True       False
  registries.ts       True       False
  RunHistory.vue      False      False   <-- NEITHER
  generated.rs        False      False   <-- NEITHER
  jobRowState.ts      False      False   <-- NEITHER
  matcher.rs          False      False   <-- NEITHER
  validate.rs         False      False   <-- NEITHER
```

Under the written rule all twelve are read-inputs, because Step 2 orders each one
opened. Two are listed and five are in neither list. The author is not being
careless here - it is following a real distinction that the rule does not
contain.

### What the rule is still missing, as one clause

**A test for whether a read-input is nameable in advance or discovered by
executing a prescribed measurement.** That is the line the current lists actually
follow, and it explains all five of them at once:

- Task 1's three edited files and Task 4's two are **pre-nameable**: the plan
  fences their target strings and names their regions, so the implementer knows
  before starting which documents it must open. Listed, correctly, even though
  edited.
- `README.md` and both `run.rs` files are **named in Step 2's own prose**.
  Listed, correctly.
- Task 5's sixteen edit sites and its five unnamed cited targets are **produced
  at execution** by Step 1's two corpus expressions. Not listable in advance -
  the corpus is re-measured precisely because it may have moved. Correctly
  absent.

Add that clause and every one of the five lists follows from the rule
mechanically. Without it, a third derivation has to pick a reading of "merely",
and whichever it picks, three lists change: either Tasks 1 and 4 shed five
entries, or Task 5 gains twenty-one.

**Suggested wording, to be taken as a sketch rather than a fence:** "A read-input
is listed when the plan can name it in advance. Where a step's own prescribed
measurement produces the set of files to open (Task 5's corpus expressions), the
measurement is the entry and the files it returns are not enumerated here; a file
named in the plan's own prose is always listed, edited or not."

## Secondary checks

**The additions, re-derived, with the unit named.** I re-ran a probe of my own
whose token pattern deliberately does not anchor at the closing backtick - the
same widening the author reports - and diffed each task's `Read first` line
against its form at `3a5680c`:

```
Task 2: +1   dry_run_cli.rs
Task 4: +8   cli.rs, commands/mod.rs, commands/run.rs, run_live.rs,
             matcher.rs, capability/runtime.rs, capability/mod.rs, profile/validate.rs
Task 5: +3   commands/run.rs, src-tauri/src/run.rs, (bare "run.rs")
TOTAL path tokens: 12   task-file pairs: 11   distinct files: 10   prose entries: 5
```

**"Eight" reproduces under none of those units.** It is not stated in the plan -
the three `eight` occurrences there are two number-word alternations inside grep
expressions and one step renumbering - so this is a report-level figure and no
artifact depends on it. I record the four units rather than calling the number
wrong, which is the discipline the corpus and span counts earlier in this chain
taught. It is worth naming because "eight" is closest to Task 4's own count, and
a per-task figure quoted as a total is how the fire count sat at four.

**The `deny.toml` rejection is correct.** Its only mention inside Task 3 is
"no edit to `deny.toml` (its RUSTSEC-ignore pruning is a rider that fires on
Renovate PRs, not on this file)", which is exactly EXCLUDE (2)'s second half. My
probe also surfaced `mise.toml`, `package.json`, `Cargo.toml` and `release.yml`
as body-named in Task 3 and absent from both lists - correctly, because Step 1
transcribes a fenced file and Step 2 verifies against vendor docs, so the task
never opens them; the INCLUDE clause simply does not fire. That case is worth
noting as evidence the rule's gate works where it is unambiguous.

**The Global Constraints placement contradicts nothing.** The new bullet is
fifteenth of sixteen and is the only statement about `Read first` in the section,
so there is no sibling to collide with. Its deference to the gate clause is
accurate: that clause does name `BUILDING.md` and state no count, verbatim. And
"a merely edited file is enumerated in its Files list" is consistent with
`latitude-carveout-zero-content-structural-forks`'s "a Files/Interfaces list
reads as an enumeration boundary", which the plan already relies on.

**The mechanism the author found in its own probe is real and correctly
diagnosed.** A pattern that requires the path to end at its extension cannot see
`matcher.rs::exact_matches`, which is why Step 3's code half was invisible to the
first sweep and visible to mine. My probe reproduces both behaviours.

**No drift:** audit sites still **nine**, companion sweep still **two**,
acceptance rows still **22**, fires still **five**, zero ticked checkboxes, no
typographic tells with the pattern fired at 1 on a scratch em-dash.

## HARVEST

**A derivation rule is only closed when it decides the cases its own artifact
already contains, and the way to test that is to run it backwards.** The rule
here reads as sound prose and passes forward - point it at a step and it tells
you what to list. It fails backwards: point it at the five lists that exist and
ask which rule produced them, and two answers survive. That asymmetry is worth a
handle, because forward-reading is what an author does and backward-reading is
what every later reviewer does. **The check is cheap and mechanical: tabulate
what the artifact already does against each clause of the new rule, and look for
a column where the artifact disagrees with itself.** Here it was one column -
edited-files-also-listed - reading 3, 0, 0, 2, 0 across five tasks.

**The distinction that kept going missing has a name worth keeping: an input the
plan can name versus a set a step computes.** Three rounds of this class - the
`Read first` omission, the document-versus-source universe, and now the
edited-file question - all turned on it without anyone stating it. A plan that
prescribes a measurement is delegating enumeration to execution on purpose, and
that is a legitimate third category beside "listed" and "excluded". Where it is
not named, every reviewer re-derives it from the lists and some of them derive it
differently.

---

# Delta review after the inputs test

**Verdict: APPROVED**

Range: `6b05f15` -> `0fee52a`, plan document only, nine lines across two hunks.
`0fee52a` is HEAD and the plan has not moved since.

Reviewer scratch path for this pass:
`/tmp/claude-1000/-home-senol-agents-peter/8c72ab74-5a32-498f-b78c-c2249fb4bb75/scratchpad/pr10-test-indep/`
(`d8.txt`).

**The class is closed.** I applied the three questions cold to every file each of
the five tasks names, without consulting the lists first, and landed on the same
five lists in every case - including the five Task-5 cited targets whose absence
I could not justify from the previous wording. That is the property asked for two
rounds ago, and it now holds.

## The cold application, task by task

I worked from the test's three questions and the tasks' step text, then compared
against the `Read first` lines. Every row below is a decision the test made, not
a list I read off.

**Task 1** - all three of its Files entries land in `Read first`.
`BUILDING.md`: Q1 yes, because Step 1's own NEEDS_CONTEXT clause makes the
implementer recount the three gate blocks and Step 1(d) needs the surrounding
section, neither of which the plan states as executable fact; Q2 nameable ->
listed. `scripts/ledger-lint.py`: Q1 yes - Step 2 integrates into `main()`, the
existing `violations` list and the exit-code logic, none of which is fenced;
listed. `.github/workflows/ci.yml`: Q1 yes - Step 3 says "extend that
enumeration", and the comment block around the quoted enumeration is not stated;
listed. Nothing else is body-named. **Matches.**

**Task 2** - `report/json.rs`'s builder rustdocs and `dry_run_cli.rs` in, its own
Files entry out. `report_json.rs`: Q1 **no**, and this is the case where the test
earns its keep. Everything written into it is fenced down to the assertions, and
the one anchor the implementer needs - "the file's existing `use` block" - is
named by the plan, so nothing is derived. Not listed. `report/json.rs`: Q1 yes,
the four mutations are applied to code the plan quotes but does not reproduce,
and the rustdocs are paraphrased rather than stated; listed. `dry_run_cli.rs`:
Q1 yes - the must-not-decide list forbids duplicating guards that can only be
reused by being read; listed. `BUILDING.md`: Q3. **Matches.**

**Task 3** - nothing listed, everything decided at Q1. `renovate.jsonc` is
created; `deny.toml` is named only as not-edited; `mise.toml`, `package.json`,
`Cargo.toml` and `release.yml` are asserted facts already verified in the
authoring section, and Step 1 transcribes a fenced file while Step 2 verifies
against vendor docs, so no step opens them. `BUILDING.md`: Q3. **Matches.**

**Task 4** - both edited documents in, plus seven source files.
`README.md`/`docs/INSTALL.md`: Q1 yes - the divergence table, the two counts and
the wrapped comment are all derived from current text; listed. The three
exit-code sources and the four matching sites: Q1 yes, Q2 nameable; listed. The
one I had to think about is `crates/muxsmith-cli/tests/run_live.rs`, where Q1 is
carried by the task's obligation not to disturb the inlined passthrough recipe -
knowing what not to touch requires seeing it. Defensible, and the weakest Q1 fit
in the plan; it errs toward listing, which is the safe direction. `BUILDING.md`:
Q3. **Matches.**

**Task 5** - the case the previous wording could not decide, and the test decides
it cleanly. The sixteen corpus files: Q1 yes, Q2 **measurement-computed** by Step
1's two expressions, whose Files list is explicitly "the authoring measurement,
not a ceiling"; not listed. The two `run.rs` candidates: Q1 yes, Q2 nameable
because Step 2's disambiguation bullet names them; listed. `README.md` as Task 4
committed it: Q1 yes, Q2 nameable; listed. **And the five cited targets I flagged
last round** - `matcher.rs`, `profile/validate.rs`, `src/jobRowState.ts`,
`src/components/RunHistory.vue`, `generated.rs` - now resolve at Q2 rather than
being unexplained: which files a comment cites is readable only from the comments
Step 1 returns, so they are downstream of the same measurement. Not listed,
correctly. `BUILDING.md`: Q3. **Matches.**

Five tasks, no case where the three questions failed to decide, and no list I
would write differently.

## The biconditional is entailed, not asserted

"An edited file appears in `Read first` if and only if it is also an
advance-nameable read-input."

- Forward: edited + advance-nameable read-input -> Q1 yes, Q2 nameable ->
  listed. Edited-ness contributes nothing; the two questions carry it.
- Backward: edited + listed -> it got there by passing Q1 and Q2, since no other
  path to `Read first` exists. Q3 only exempts `BUILDING.md` from repetition in
  its GATE role and explicitly leaves a task free to list it for its own reason,
  which is Q1+Q2 again.

So it is a corollary of the first two questions restricted to edited files, with
"edited" doing no work in either direction - which is exactly why the earlier
form went wrong by carrying it as a separate clause. Stating it as a corollary is
useful: it is the case a reader will ask about, and answering it in advance stops
the re-derivation the previous three rounds kept triggering.

## Task 5's asymmetry is right, and it errs safe in both directions

Listing the sixteen would freeze an authoring snapshot into `Read first` and
contradict Step 1's own "if a re-measurement returns a different set, that is the
ground truth". Listing the two `run.rs` files costs nothing and is stable: that
two tracked files share a basename is a fact about the repo the plan states
independently of the corpus. The residual risk on that side is a dead entry if a
re-measurement drops the citing site - an implementer reads a file it did not
need. The residual risk on the other side would be an implementer told to read a
file the measurement says is not in scope. Over-inclusion of a stable fact
against under-inclusion of a computed set: the asymmetry is in the right
direction on both counts.

## Secondary

**The residue the change created is closed, and the sweep for others is clean.**
Task 4's line now reads "listed because every claim this task corrects is DERIVED
from their current text ..., not because the task edits them". My own sweep for
edited-ness framing across all five `Read first` lines returns exactly one hit,
and reading its context shows it is inside that explicit negation rather than a
survival. The sweep is fire-verified: the same pattern against Task 4's pre-fix
line at `6b05f15` returns "documents this task", so it detects the real framing.

**No drift:** audit sites still **nine**, companion sweep still **two**,
acceptance rows still **22**, fires still **five**, Task 4 still eight steps, zero
ticked checkboxes, no typographic tells with the pattern fired at 1 on a scratch
em-dash.

## HARVEST

**A rule that describes its artifact and a rule that decides new cases are
different objects, and only the second one closes a class.** Every earlier form
of this bullet was a true description of the five lists - and each one still
admitted a second reading, because a description tells you what was chosen and
not how to choose. The turn came from converting it into ordered questions with
first-answer-settles semantics: that removes the reader's freedom to weigh
clauses against each other, which is where "merely EDITS" versus "must read to
derive a symbol" had been pulling in opposite directions for two rounds. **The
readable trigger: if a rule's clauses can both fire on one case and nothing says
which wins, it is a description wearing a rule's grammar.**

**The test that a derivation rule is finished is running it backwards on the
artifact it came from, cold.** I applied the three questions without looking at
the lists and then compared - and the value was not the four cases that matched
trivially but the two that took work (`report_json.rs`, `run_live.rs`), because
those are where a second reader would have diverged if the rule were still loose.
A rule that decides its own hard cases the same way twice, by two people, is
finished. That is a cheaper acceptance criterion than any amount of re-wording,
and it is the one I would ask for first next time rather than after three rounds.

**Closing note on this review chain.** Fourteen findings at plan review, three
across the first delta, one each in the three that followed, one over the
amendments, one on the inputs class and one on its rule. Every one closed against
evidence I reproduced. The last four rounds were all one class - a list, a rule
for the list, the rule's universe, and the rule's decision procedure - which is
the shape of a defect that is genuinely structural rather than a slip: each fix
was correct and each exposed the next layer down. Worth remembering that the
first of them looked like a one-line omission in a header.
