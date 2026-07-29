# Task 3 verdict - Plan 10 (W4: `renovate.jsonc`)

**Verdict: APPROVED_WITH_MINORS.**

The artifact is byte-perfect against the plan's fence and expresses
`ci-04-dependabot-cadence` clause by clause, including the four deliberate
absences and their stated reasons, each of which I re-derived at the vendor's
source rather than from the report. Nothing became permanent. The activation
boundary is stated and not overclaimed.

One MEDIUM finding is against the PLAN's Step 3, not against the implementer:
the fenced validator invocation validates this file as GLOBAL self-hosted
config, and I measured two real defect classes it lets through that the
repo-config path catches. The implementer ran the fence verbatim (as
"must not decide: the validator invocation" requires), surfaced the gap, and
closed it with supplementary runs I reproduced. That is the correct handling;
the defect belongs to the plan and routes to the controller.

Two MINORs: one imprecise sentence in the report's controller-facing section,
and one gap in the ledger entry the file exists to express.

---

## 1. Findings

### F1 - MEDIUM (plan defect, routed; no change to `renovate.jsonc`)

**The fenced Step-3 invocation does not validate the file as what it will be
used as.**

- Site: `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md:470-473`
  (the Step 3 bash fence). Implementer's surfacing:
  `.superpowers/sdd/plan-10/task-3-report.md:791-816` (concern 2) and `:872-877`.
- The claim is correct and I verified it at the source at the exact tag that
  was run, not at `main`: `lib/config-validator.ts` at tag `43.287.0`, lines
  178-182 - `let isGlobalConfig = true; if (opts.global === false) { isGlobalConfig
  = false; } const configType = isGlobalConfig ? 'global' : 'repo';` - and the
  program help at `:140`, "When specifying [config-files...], Renovate will treat
  them as global self-hosted configuration files. You can disable this behaviour
  with --no-global". Identical at current `main`.
- **The report reasons that global mode "would not flag a `globalOnly` option
  wrongly present". I measured it instead**, with two probes built outside the
  repo tree from the committed file (`probe-global/probeA.jsonc`,
  `probeB.jsonc`):

  | probe | mutation | fenced form (global) | `--no-global` (repo) |
  |---|---|---|---|
  | A | `"repositoryCache": "enabled"` added (a `globalOnly` option) | EXIT=0, "validated successfully" | EXIT=1, `The "repositoryCache" option is a global option reserved only for Renovate's global configuration and cannot be configured within a repository's config file.` |
  | B | `extends` carries `global:disableInherit` | EXIT=0, "validated successfully" | EXIT=1, `extends: you cannot extend from "global:" presets in a repository config's "extends"` |

  Both under `--strict`, both at `43.287.0`. So the fenced command's green
  result is silent for two real repo-config defect classes. This is the
  "a check whose passing result is an absence" case, and the fire test I ran is
  what makes the negative interpretable.
- **The supplementary run does close it for THIS file**, and not merely by
  assertion. I confirmed at both `43.287.0` and current `main` that none of the
  six top-level keys carries `globalOnly` (`$schema`, `extends`, `timezone`,
  `schedule`, `prHourlyLimit`, `packageRules`; fire control: the same scan sees
  `globalOnly: true` on `repositoryCache`), and `extends` carries no `global:`
  preset. My own `--no-global` runs exit 0 as "repo config" at both versions.
- **Exact required change: none in `renovate.jsonc`.** For the controller: any
  future re-validation of a Renovate repo config must use
  `renovate-config-validator --strict --no-global <file>` or the no-argument
  auto-detect form; Step 3's fence should be corrected wherever it is reused,
  and the trap is ledger-worthy as the report proposes.

### F2 - MINOR (report precision, activation boundary)

- Site: `.superpowers/sdd/plan-10/task-3-report.md:866-869`, the clause "the
  config is on `master` as of `630d418`".
- Evidence run: `git rev-list --left-right --count origin/master...HEAD` returns
  `0	5`; `git branch -r --contains 630d418` returns nothing; `git log --oneline -1
  origin/master` is `754cb73`. The commit is on the LOCAL master only.
- Why it matters: the ordering rider Step 4 makes load-bearing is about the
  config being present in the repository the vendor sees, since that is what
  `onboarding/branch/check.ts` reads to suppress the onboarding PR. Read alone,
  this sentence tells the controller the rider is satisfied and the owner may
  install the app. The report does say "Not pushed" at `:924`, so the two
  statements recover each other, but the imprecise one sits in section 10, which
  is the section written to be acted on.
- Exact required change: "the config is on the local `master` as of `630d418`;
  the ordering rider is satisfied once the plan-close push lands."

### F3 - MINOR (house record, controller-owned)

- Site: `docs/process-conventions.yaml:304` (the `ci-04-dependabot-cadence`
  statement).
- Evidence run: the statement line grepped for `runner|github-runner|ubuntu`
  returns nothing. Fire control, same grep form on the same line for
  `mise|packageManager|rust-toolchain`, returns all three.
- The shipped config disables `github-runner` (packageRules entry 4) with a D85
  rationale. The entry records the `packageManager` consequence the controller
  drew, verbatim and at length, but not this one - so the ledger entry
  under-describes the config that exists to express it. Not a defect in the
  file: the plan brief authorizes the runner rule explicitly, with the same D85
  ground. I verified D85 itself
  (`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:231-238`,
  the Tauri AppImage guide's "you must build your Tauri application using the
  oldest base system you intend to support"; `docs/ROADMAP.md:687-689` names the
  raised "glibc/webkit floor"), so the file comment's "the oldest supported base
  for the AppImage glibc floor" is accurate.
- Exact required change: add the runner-image consequence to `ci-04`'s
  statement in the next controller write.

### F4 - INFO (a measurement the report owes its own concern 3)

- Site: `.superpowers/sdd/plan-10/task-3-report.md:817-823` (concern 3).
- The report frames `EBADENGINE` as a property of the pinned validator, which
  invites "bump the pin and it goes away". Evidence run: the registry metadata
  for both versions declares the same range -
  `43.287.0 engines: {'node': '^24.11.0', 'pnpm': '^11.0.0'}` and
  `44.0.1 engines: {'node': '^24.11.0', 'pnpm': '^11.0.0'}`.
- The warning is structural, not stale: Renovate tracks a Node LTS line and this
  repo pins a newer one. No pin refresh removes it. This strengthens rather than
  weakens the "acceptable as-is" ruling in adjudication 3.

---

## 2. Dimension results

| # | Dimension | Result |
|---|---|---|
| 1 | Transcription fidelity | **PASS**, byte-identical |
| 2 | Expresses the owner's ruling | **PASS**, all clauses carried |
| 3 | Premises re-verified at the source | **PASS**, every spot-check held |
| 4 | Repo facts the comments assert | **PASS**, all four verified on the tree |
| 5 | Validation evidence | **PASS with F1** - the runs are real, the fence measures the wrong config type |
| 6 | Nothing became permanent | **PASS** |
| 7 | Activation boundary | **PASS with F2** |
| 8 | House dimension | **PASS** |
| 9 | No-work-needed premises | **PASS**, each run |
| 10 | Blast radius on Task 5 | **PASS**, corpus unchanged |

### D1 - transcription fidelity

Own instrument, not the implementer's: a Python extractor that finds the first
` ```jsonc ` fence after the `## Task 3` heading and writes the body, then
`diff -u` and `sha256sum`.

```
fence lines: 84   start line(1-based): 375   end: 460
2a6911d43f826252c0bdb27d5f3d11e0902a0c7792525cae0c9d3f8a110a4b4d  fence.jsonc
2a6911d43f826252c0bdb27d5f3d11e0902a0c7792525cae0c9d3f8a110a4b4d  renovate.jsonc
DIFF_EXIT=0
```

The report's md5 reproduces independently: `c3c3b7da87843b050ede361049e05b46`
for both the worktree file and `git show 630d418:renovate.jsonc`.

**Two fire controls**, because a clean `diff` is an absence and order is
semantic here:

- one-character mutation (`prHourlyLimit` 0 -> 1): `PROBE_DIFF_EXIT=1`, hunk
  printed.
- **rule reorder** (npm group swapped with the github-actions group, content
  otherwise untouched): `REORDER_DIFF_EXIT=1`. The instrument sees order, which
  the value-mutation control alone would not have proved.

Order also checked semantically after a JSONC parse: 7 `packageRules` entries in
the fenced sequence; the npm group is index 1 and the `packageManager` disable
index 4, so later-wins holds.

### D2 - the owner's ruling, clause by clause

| `ci-04` clause | Carried by | Verified |
|---|---|---|
| monthly cadence | `"schedule": ["* * 1-3 * *"]` + `"timezone": "Europe/Berlin"` | the entry's own 2026-07-29 occurrence records this exact expression as the ruled form |
| grouped per ecosystem (Cargo, npm, GitHub Actions) | rules 1, 2, 3 (`groupName` "cargo dependencies" / "npm dependencies" / "github actions") | parsed |
| majors on their own PRs | **absence** of `separateMajorMinor` | `lib/config/options/index.ts:1832-1837`, `default: true`; `docs/usage/configuration-options.md`, "This option also has priority over package groups configured by `packageRule`." Setting it anywhere is the defect; it is set nowhere |
| security immediate and ungrouped | **absence** of `vulnerabilityAlerts` | `index.ts:2366-2385` ships `groupName: null`, `schedule: []`, `prCreation: 'immediate'`; the docs section states it ignores `prHourlyLimit`/`schedule` and "skip the line" |
| `mise` OFF | rule 6 | manager registered (`api.ts:192`) and `**/{,.}mise{,.*}.toml` matches this repo's `mise.toml`, so the disable is not a no-op |
| `rust-toolchain` on, own group | rule 7 | registered (`api.ts:218`), `managerFilePatterns: ['/(^|/)rust-toolchain(\.toml)?$/']` matches `rust-toolchain.toml` |
| `packageManager` depType disabled (controller consequence) | rule 5, **after** rule 2 | depType exists (`npm/dep-types.ts`); ordering verified |
| runner images unmanaged | rule 4 | depType exists (`github-actions/dep-types.ts`); see F3 on where the ruling for it lives |

The two other stated absences also check out at the source, reason and all:
`config:best-practices` extends `security:minimumReleaseAgeNpm` and
`:maintainLockFilesWeekly` (which is `lockFileMaintenance: { enabled: true,
extends: ['schedule:weekly'] }`, contradicting the monthly cadence) and
`docker:pinDigests` (`matchDatasources: ['docker']`, dead weight in a repo with
no Docker surface).

### D3 - premises re-verified, source named per item

I resolved `renovatebot/renovate` `main` myself and got the same SHA the report
used, `04b629b1285e45fb789d285c52bc9eaf22782cfc`, then fetched 17 files at that
SHA (all HTTP 200) plus `lib/config-validator.ts`, `lib/config/validation.ts` and
`lib/config/options/index.ts` at tag `43.287.0`.

| item | source I read | observed |
|---|---|---|
| `prHourlyLimit` default (**the trap**) | S `lib/config/options/index.ts:2225-2229` | `default: 2`. `prConcurrentLimit` `default: 10` at `:2232-2236`, immediately after - the trap reproduces exactly. Fence and comment CORRECT |
| `separateMajorMinor` default + priority | S `index.ts:1832-1837` + D `configuration-options.md` | `default: true`; the priority-over-groups paragraph present verbatim |
| what `config:best-practices` extends | S `config.preset.ts` | full list read; both named members present |
| `schedule:monthly` literal | S `schedule.preset.ts:5`, `:67-71` | `const monthly = ['* 0-3 1 * *'];` |
| backend cadence vs windows | D `key-concepts/scheduling.md:13` | "it is not possible for _repository configuration_ to reduce that to less than X" |
| `mise` / `rust-toolchain` managers + file matching | S `manager/api.ts`, both `index.ts` | registered; both patterns match this repo's files |
| `action`, `github-runner` depTypes | S `github-actions/dep-types.ts` | both present by name |
| `packageManager` depType | S `npm/dep-types.ts` | present |
| `vulnerabilityAlerts` shipped defaults | S `index.ts:2366-2385` + D | as tabled in D2 |
| `renovate.jsonc` search position | S `lib/config/app-strings.ts` | 6 patterns brace-expanding to 14 names; `renovate.jsonc` at position 2. Confirmed at runtime too: the no-argument validator run found it |
| `$schema` URL | S `index.ts` `onboardingConfig` default | current |
| `helpers:pinGitHubActionDigests` | S `helpers.preset.ts:118-126` | `{ matchDepTypes: ['action'], pinDigests: true }` |
| cargo `rangeStrategy` | S `index.ts:1904-1912` + D `cargo/readme.md` | global `default: 'auto'`; readme's `auto` -> `update-lockfile` wording matches plan correction 3 exactly |

Nothing refuted. The plan's Renovate premises block holds one day after
authoring.

### D4 - the repo facts the comments assert

All four read off the tree:

- `mise.toml`: `node = "26.5.0"`, `pnpm = "11.10.0"`.
- `package.json:6`: `"packageManager": "pnpm@11.10.0"` - the mirror the comment
  describes.
- `rust-toolchain.toml`: `channel = "1.96.1"`.
- `.github/workflows/release.yml`: `ubuntu-22.04` at `:29` (`runs-on:`), `:81`
  (matrix `os:`, consumed by `runs-on: ${{ matrix.os }}` at `:83`) and `:186`
  (`runs-on:`); the in-file rationale at `:21-25` names D85 and says the pin
  "deliberately diverges from the test matrix's ubuntu-26.04". `ci.yml:29` runs
  `ubuntu-26.04`, so the comment's contrast is real.

### D6 - nothing became permanent

Own commands, not the report's assertion:

```
git status --porcelain            -> (empty)
git show --name-status 630d418    -> A	renovate.jsonc      (one file, nothing else)
git diff --stat 9ff4173..630d418 -- package.json pnpm-lock.yaml Cargo.lock Cargo.toml  -> (empty)
grep -rn renovate BUILDING.md .github/workflows/ scripts/ package.json eslint.config.js -> no hits
```

Fire control for that last absence, same grep form and file set: `ledger-lint`
returns 24 lines; `mise` returns four files. The instrument sees a term when one
is there.

`eslint` selection measured rather than argued, with the gate's own command:
`npx eslint . --format json` reports on 63 files; `renovate.jsonc` is not among
them; fire control, `eslint.config.js` is. And the working tree remained
byte-identical across all six of my own validator runs and the eslint run.

### D7 - activation boundary

Report section 5 states both owner actions, states the ordering with the correct
causal reason, marks W4-c OPEN, and states the ROADMAP trigger has NOT fired
with the right ground ("its condition is activation, not configuration"). It
never claims Renovate is active. F2 is the one imprecision, and it is about
which `master`.

I confirmed the vendor mechanism the ordering rests on rather than taking it
from the plan: onboarding is skipped as soon as a config file exists, so the
config must precede app installation.

### D8 - house dimension

- Tier-2 conformance: `proc-latitude-clause-boundary` - no fork was resolved at
  the keyboard; `proc-57-briefs-not-ground-truth` - every load-bearing premise
  re-verified at the source, which is the whole judgment budget of this task and
  was spent correctly; `latitude-carveout-zero-content-structural-forks` - the
  over-restriction watch has **nothing to report**: the boundary stopped nothing
  in this task, because the file is fenced end to end and the only two things
  the implementer could have "helpfully" changed (the validator invocation, an
  added option) are both explicitly on the must-not-decide list, and both were
  surfaced instead.
- Typography inside the file's comments: `LC_ALL=C grep -nP '[^\x00-\x7F]'`
  exits 1 (no non-ASCII byte at all - a superset check), and an explicit
  AI-tell glyph scan for em/en dash, smart quotes, ellipsis, NBSP and Unicode
  minus also exits 1. **Both fire-controlled**: the superset expression returns 1
  on `rust-toolchain.toml`, the glyph expression returns 1 on a probe file
  containing an em dash, curly quotes and an ellipsis.
- SI-4: exactly one trailer,
  `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`; `%G?` is `N`
  (unsigned); commit subject matches the fence.

### D9 - the no-work-needed premises, each run

| report conclusion | my run |
|---|---|
| "43.287.0 still resolves, so the fallback clause does not fire" | HTTP 200 |
| "44.0.1 is current" | registry `latest` -> `44.0.1` |
| "npx left nothing in the tree" | `git status --porcelain` empty after six of my own npx runs |
| "no gate part reads the file" | grep over BUILDING.md/workflows/scripts/package.json/eslint.config.js: no hits, fire-controlled |
| "eslint does not select .jsonc" | measured with `--format json`, fire-controlled |
| "the config is fine either way (global vs repo)" | `--no-global` clean at 43.287.0 AND 44.0.1; no top-level key is `globalOnly`, fire-controlled |
| "no premise refuted" | re-derived independently, see D3 |

### D10 - blast radius on Task 5

Corpus re-measured on the tree at `630d418`:

- **Expression A**: 20 matched lines across 13 files - unchanged.
- **Expression B**: 4 matched lines across 4 files - unchanged
  (`profile_save.rs:95`, `suggestions.rs:1016`, `ts_export.rs:10`,
  `registries.ts:12`).
- **Union: 24 lines across 16 files**, `suggestions.rs` shared. The plan's
  numbers stand.

`renovate.jsonc` adds no corpus member, and the claim is fired rather than
assumed:

- the `git ls-files -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py'` selector
  returns 0 hits for `renovate`;
- **fire control 1**: that selector demonstrably reaches root-level files
  (`eslint.config.js`, `playwright.config.ts`, `vite.config.ts`);
- **fire control 2**: the same selector widened with `'*.jsonc'` returns exactly
  1 hit for `renovate.jsonc` - so the exclusion is the extension set, and the
  pipeline would have seen the file had it been in scope;
- and belt-and-braces, both expressions run directly against `renovate.jsonc`
  return 0, while the same two expressions return 1 against
  `src/editor/fieldSpec.ts` and `src/editor/registries.ts`.

Both expression sets were derived from the plan's pasted text rather than from
recall of which extensions "should" be in them.

---

## 3. Adjudications

### Q1 - the pinned validator is one major behind

**Verdict: running the fenced pin was correct fidelity. NEEDS_CONTEXT was not
owed.**

The fence's escape hatch is conditioned on one thing, that the version "no
longer resolves", and I measured that condition false (HTTP 200 for
`43.287.0`). Staleness is not the trigger the plan wrote, and "Must not decide"
names the validator invocation explicitly. Substituting `44.0.1` because it is
newer would have been the implementer deciding a fenced item at the keyboard -
the exact failure `proc-latitude-clause-boundary` exists to prevent. The pin is
an instrument, not a dependency: nothing persists, verified above.

The counter-case deserves stating, since a stale instrument can measure the
wrong thing: if `43.287.0` and `44.0.1` disagreed on this config, the pin would
be validating a Renovate that will never see the repo. That is a real risk and
it is why the implementer's extra `44.0.1` run is the right instinct. But it is
measurable, and it was measured: clean at both.

**What the report should record about `44.0.1`, beyond what it already does:**
it already gives the version and one clean strict repo-config run. Two things
to add. First, from my own runs, `44.0.1` is clean in BOTH modes (global and
`--no-global`), so the majors agree on this file in every combination that was
run. Second, F4: `44.0.1` declares the identical engine range, so refreshing the
pin buys nothing on the EBADENGINE front either. Together those turn "should we
refresh the pin?" from an open worry into a plan-hygiene question with no
technical content today - which is the form it should reach the controller in.

### Q2 - what the fenced invocation actually validates

**Verdict: the implementer's claim is correct, and the fenced invocation does
NOT validate the thing this file will be used as. That is a genuine defect in
the plan's Step 3 and is worth routing. The supplementary run closes it for this
file, but not for the method.**

Verified myself, at the tag that was run and not only at `main`: naming a file
sets `configType = 'global'` unless `--no-global` is passed, and my own fenced
runs reproduce `INFO: Validating renovate.jsonc as global config` verbatim.

I did not stop at reading the flag. Reading `lib/config/validation.ts` at the
same tag shows three places where `configType` changes the outcome - the
`isGlobalOption(key)` branch, the `global:` preset check in `extends`, and the
`env`/`hostRules` allowlist source - and then I made the difference produce
output, because "global mode is weaker" is otherwise an argument rather than a
measurement. Probes A and B in F1 are that measurement: two mutations of this
very file that the fenced command passes at exit 0 and the repo-config command
fails at exit 1. A green fenced run is therefore not evidence of repo-config
validity in general.

**For this artifact it is nevertheless sufficient**, and that is a separate
claim I checked separately: the file has no `globalOnly` key and no `global:`
preset at either version, and both `--no-global` runs pass. So no fix is owed to
`renovate.jsonc`, and the implementer's decision to run the fence verbatim,
report the divergence, and add measurement rather than edit the fence is exactly
right under "must not decide".

The routing is the deliverable here: the plan's Step 3 fence should read
`--strict --no-global` (or drop the file argument), and the class is ledger-
worthy in the form the report proposes - a command that looks right, exits 0,
and answers a slightly different question than the one asked.

### Q3 - the `EBADENGINE` warning and the two depType description drifts

**Verdict: correct restraint on both. Neither changes a fact the file's comments
assert.**

I checked the comments rather than recalling them, and enumerated them
mechanically rather than from memory of what is in the file: **9 comment blocks,
38 comment lines**, at `renovate.jsonc:4-9`, `:14-20`, `:23-24`, `:27-30`,
`:35-38`, `:51-54`, `:60-65`, `:71-72`, `:77-79`. Six carry vendor claims (what
`config:best-practices` pulls and what `security:minimumReleaseAgeNpm` would do;
the literal `schedule:monthly` value plus the backend-cadence limitation;
`prHourlyLimit`'s default of 2; `separateMajorMinor`'s default and its priority
over groups; the cargo `rangeStrategy` default chain), one carries a repo claim
(the `release.yml` ubuntu-22.04 pin with D85), one carries a repo claim plus the
later-rules-win note (the pnpm mirror across `mise.toml` and `package.json`), and
two carry owner rulings only (`mise` off, `rust-toolchain` in its own group).

**The negative claim is measured over all nine blocks, not asserted:** a scan of
the comment lines alone finds 0 hits for the `action` and `github-runner`
description strings as they read at current main
("repository-based action reference", "uses:", "GitHub-hosted runner version",
"runs-on"), and **0 hits for any `x.y.z` version token at all**, so the comments
pin no version anywhere. The single hit for the node/engine expression is
`// This rule follows the npm group above on purpose: later rules win.` - the
word "npm" as a manager name, not an engine claim. Fire controls on the same
instrument: `rangeStrategy` returns 1 hit, `ubuntu` returns 2.

- **EBADENGINE** is npm's install-time warning for the one-shot `npx` fetch, not
  validator output; both runs completed at exit 0, and I reproduced that. It
  touches no asserted fact. F4 adds the measurement that makes the ruling
  durable rather than provisional: `44.0.1` declares the same range, so this is
  structural and recurs regardless of the pin. Acceptable as-is, and worth one
  line in the house record so the next agent does not re-litigate it.
- **The depType descriptions.** `matchDepTypes` matches on the depType NAME, and
  both names are unchanged at the commit I read. I confirmed the delta is an
  appended parenthetical example on each
  (`(e.g. `actions/checkout@v4`)`, `(e.g. `ubuntu-24.04`)`). One precision note
  on the report's framing: it calls this "drift ... at current main" against the
  plan's authoring quote, but the plan was authored the same day as the commit I
  read, so what is established is that the plan's quotation lacks the trailing
  example - not that the source changed after authoring. Flagging it as
  reviewer-facing context rather than as a refutation was the right call either
  way; the correction is only to the direction of the inference.

---

## 4. Evidence appendix

**Instrument directory (independent, created for this review):**
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t3rev-independent/`

| artifact | what it is |
|---|---|
| `fence.jsonc` | fence body extracted by my own Python extractor (first ` ```jsonc ` after `## Task 3`) |
| `probe.jsonc`, `probe2.jsonc` | value-mutation and rule-reorder fire controls for the diff instrument |
| `renovate-src/` | 17 vendor files at `04b629b1285e45fb789d285c52bc9eaf22782cfc`, plus `config-validator.ts`, `validation.ts`, `options/index.ts` at tag `43.287.0` |
| `run1.out` .. `run6.out` | my six validator runs |
| `probe-global/probeA.jsonc`, `probeB.jsonc` | the two global-vs-repo probes for Q2 |
| `exprA.txt`, `exprB-raw.txt` | Task-5 corpus re-measurement |
| `latest.json` | npm registry metadata for `renovate@latest` |

**Commands, foreground, absolute paths, zsh (exit codes captured directly, never
through a pipe):**

```
diff -u fence.jsonc /home/senol/Git/Muxsmith/renovate.jsonc
sha256sum / md5sum on fence, worktree file, and git show 630d418:renovate.jsonc
curl https://api.github.com/repos/renovatebot/renovate/commits/main
curl https://raw.githubusercontent.com/renovatebot/renovate/<sha|tag>/<path>   (x20, all 200)
npx --yes --package renovate@43.287.0 -- renovate-config-validator renovate.jsonc
npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict renovate.jsonc
npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict --no-global renovate.jsonc
npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict            (auto-detect)
npx --yes --package renovate@44.0.1  -- renovate-config-validator --strict --no-global renovate.jsonc
npx --yes --package renovate@44.0.1  -- renovate-config-validator --strict renovate.jsonc
npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict probeA.jsonc / probeB.jsonc  (both modes)
npx eslint . --format json
git ls-files / grep -nE for corpus expressions A and B, plus their fire controls
git status --porcelain / show --name-status / diff --stat / rev-list --left-right --count / log -1 --format="%(trailers)" "%G?"
LC_ALL=C grep -nP '[^\x00-\x7F]' and the explicit glyph scan, each with its fire control
```

**Tree state, proved rather than asserted.** I mutated nothing in the repo; the
two probe files were built in the scratchpad from a read of the committed file.

```
git status --porcelain                 -> (empty)
git rev-parse HEAD                     -> 630d4183f5bf2403b99f19cbb31b8b803973b659
git diff --quiet 630d418 --            -> exit 0   (worktree identical)
git diff --cached --quiet 630d418 --   -> exit 0   (index identical)
sha256sum renovate.jsonc               -> 2a6911d43f826252c0bdb27d5f3d11e0902a0c7792525cae0c9d3f8a110a4b4d
```

That sha256 is the same value measured before my first validator run and after
the last of the six plus the eslint run.

---

## 5. HARVEST

**Patterns worth carrying.**

1. **A validator's default mode is part of the command's meaning.** The class F1
   belongs to is wider than Renovate: a tool that accepts a file argument may
   validate it against a schema the file will never be used under, exit 0, and
   look like proof. The generalizable trigger is readable - a validation command
   whose subject has more than one config type, where the type is chosen by a
   flag with a default. The house already has the doctrine (a check whose
   passing result is an absence must be made to fire); this is the instance
   where the check fired green and was still measuring the wrong object. The
   fix that generalizes: when a plan fences a validation command, fence the
   assertion about WHAT IT VALIDATES alongside it, so the implementer can check
   the command against its own stated purpose.
2. **"Would it catch X?" is a probe, not an argument.** The report reasoned
   correctly that global mode accepts a superset and concluded, correctly, that
   this file is fine either way. Constructing the two probes took under a minute
   and converted a sound argument into a measurement - and it is the measurement
   that makes the routing decision defensible, because it names the exact two
   defect classes that slip through instead of a category.
3. **A reorder control belongs beside the value control** wherever order is
   semantic. The implementer's fire control mutated a value, which is the
   standard move; but this fence's own "Must not decide" says rule ORDER is
   fixed because later rules win, so the property most worth firing was
   ordering. A diff does see it - I confirmed - but the control that proves it
   is the one that should be pasted.
4. **The absence-fire duty extends to the SELECTOR, not just the pattern.**
   D10's claim is that a new file adds no corpus member. Firing the grep proves
   the grep works; it says nothing about whether the file was ever in the
   selector's reach. The control that carries the claim is the widened selector
   (`+ '*.jsonc'` -> 1 hit), which shows the exclusion is the extension set and
   not an accident of path or timing.
5. **A local commit is not "on `master`" for a claim about what a remote
   service sees** (F2). Worth a house line, because the same sentence shape will
   recur at every activation, publication and tag gate in this repo, and it is
   the plan-close push that changes its truth value.

**Repeated rejections.** None in this task. The implementer resolved no fork,
added no option, and edited no fence. Both temptations the plan pre-named - the
stale pin and the "wrong" validator invocation - were surfaced instead of acted
on, which is the behaviour the fence was written to produce.

**What Tasks 4 and 5 must carry.**

- **Task 5's corpus is unchanged at `630d418`**: 20 lines / 13 files under
  expression A, 4 / 4 under B, union 24 / 16. `renovate.jsonc` is outside the
  selector's extension set and matches neither expression. Task 5 re-runs both
  expressions before editing anyway, per its own Step 1; this verdict is a
  concurring measurement, not a substitute.
- **Task 4 is the one that moves the corpus**, not Task 3: it edits `README.md`,
  and three expression-A hits in `crates/muxsmith-cli/tests/run_live.rs` cite
  `README.md` line spans. The sequencing already accounts for this; carrying it
  forward here only because Task 3 was the task that could have perturbed the
  count and did not, which leaves Task 4 as the sole remaining perturber before
  Task 5 measures.
- **W4-c stays OPEN** and must be recorded as an owner action at the plan close,
  with F2's correction: the ordering rider becomes satisfied at the plan-close
  push, not at `630d418`.
- **The ROADMAP's Renovate entry carries one line that this task falsifies** and
  that the close disposition should catch: "The cadence itself is still open and
  is settled when the config is written" (`docs/ROADMAP.md`, Pre-1.0 gates,
  Dependabot/Renovate activation entry). The cadence was settled by owner ruling
  before the plan brief, and it is now written. Flagged for the controller as a
  disposition input; no task edits the ROADMAP.

**For the controller's house-knowledge write** (the controller is the single
writer): F1's validator trap, F3's missing runner-image consequence on `ci-04`,
and F4's structural EBADENGINE finding.
