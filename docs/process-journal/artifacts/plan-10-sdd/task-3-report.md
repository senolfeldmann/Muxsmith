# Task 3 implementer report - Plan 10 (W4: `renovate.jsonc`)

**Status: DONE_WITH_CONCERNS.**

Every fenced premise held at the vendor's source. The file is committed at
`630d418`, byte-identical to the plan's fence. Both fenced validator runs exit 0.
The full gate is green, foreground, on the tree carrying the new file. The
concerns are numbered in section 8 and none of them changes a value in the file;
two are about the *validator invocation* the fence pins, not about the config.

---

## 1. Source of truth used for the re-verification

Renovate's own repository, pinned to a single commit so every value below is
reproducible:

```
$ curl -s https://api.github.com/repos/renovatebot/renovate/commits/main | head -c 400
{
  "sha": "04b629b1285e45fb789d285c52bc9eaf22782cfc",
  "node_id": "C_kwDOBJLedNoAKDA0YjYyOWIxMjg1ZTQ1ZmI3ODlkMjg1YzUyYmM5ZWFmMjI3ODJjZmM",
  "commit": {
    "author": {
      "name": "renovate[bot]",
      "email": "29139614+renovate[bot]@users.noreply.github.com",
      "date": "2026-07-29T12:23:42Z"
```

All files were fetched from
`https://raw.githubusercontent.com/renovatebot/renovate/04b629b1285e45fb789d285c52bc9eaf22782cfc/<path>`,
each returning HTTP 200. Reading `main` at a pinned SHA rather than a release tag
follows the plan's own authoring method ("against Renovate's source on `main`,
read 2026-07-29"); the one exception is item 15 below, where the *validator's own
version* is the subject and the file was read at tag `43.287.0`.

Where a rendered docs page was the right source (a documented statement rather
than a schema value), the file read is `docs/usage/...` **from that same pinned
commit**, which is the source of the rendered page. The per-item table names
which kind was read.

**The named trap is confirmed, not corrected.** See item 8.

---

## 2. Per-option re-verification (Step 2)

`S` = schema/preset source (`lib/config/options/index.ts`,
`lib/config/presets/internal/`, `lib/modules/manager/...`).
`D` = documentation source (`docs/usage/...` at the same commit).

| # | Item | Source read | Value observed (pasted) | Fence |
|---|---|---|---|---|
| 1 | `renovate.jsonc` filename search order | S `lib/config/app-strings.ts` | see below | **held** |
| 2 | jsonc-over-json5 recommendation | D `docs/usage/configuration-options.md:46` | see below | **held** |
| 3 | `$schema` URL | S `lib/config/options/index.ts:738` | see below | **held** |
| 4 | `config:recommended` | S `lib/config/presets/internal/config.preset.ts` | see below | **held** |
| 5 | `helpers:pinGitHubActionDigests` | S `lib/config/presets/internal/helpers.preset.ts:118-126` | see below | **held** |
| 6 | what `config:best-practices` extends | S `config.preset.ts` | see below | **held** |
| 6a | `:maintainLockFilesWeekly` is weekly | S `default.preset.ts:359-365` | see below | **held** |
| 6b | `docker:pinDigests` is Docker-only | S `docker.preset.ts:36-48` | see below | **held** |
| 6c | `security:minimumReleaseAgeNpm` | S `security.preset.ts:26-41` | see below | **held** |
| 7 | literal value of `schedule:monthly` | S `schedule.preset.ts:5,67-71` | see below | **held** |
| 8 | `prHourlyLimit` default (**the trap**) | S `lib/config/options/index.ts:2225-2237` | see below | **held** |
| 9 | backend cadence vs schedule windows | D `docs/usage/key-concepts/scheduling.md:12-13` | see below | **held** |
| 10 | cargo default `rangeStrategy`; what `bump` does | S `lib/config/options/index.ts:1904-1916` + D `lib/modules/manager/cargo/readme.md`, `configuration-options.md:4514+` | see below | **held** |
| 11 | `action` and `github-runner` depTypes | S `lib/modules/manager/github-actions/dep-types.ts` | see below | **held** (wording drift, concern 4) |
| 12 | npm `packageManager` depType | S `lib/modules/manager/npm/dep-types.ts` | see below | **held** |
| 13 | `mise` and `rust-toolchain` managers | S `lib/modules/manager/api.ts`, `rust-toolchain/index.ts`, `mise/index.ts` | see below | **held** |
| 14 | `separateMajorMinor` default + priority over groups | S `lib/config/options/index.ts:1832-1837` + D `configuration-options.md:4854-4856` | see below | **held** |
| 15 | `vulnerabilityAlerts` shipped defaults, bypass of schedule/limits | S `lib/config/options/index.ts:2366-2385` + D `configuration-options.md` | see below | **held** |

### Item 1 - filename search order

`lib/config/app-strings.ts`:

```ts
const configFilePatterns = [
  'renovate.json{,c,5}',
  '.github/renovate.json{,c,5}',
  '.gitlab/renovate.json{,c,5}',
  '.renovaterc',
  '.renovaterc.json{,c,5}',
  'package.json',
];

const configFileNames = configFilePatterns.flatMap((p) => braceExpand(p));
```

Brace-expanded in order: `renovate.json`, **`renovate.jsonc`**, `renovate.json5`,
`.github/renovate.json{,c,5}` (3), `.gitlab/renovate.json{,c,5}` (3),
`.renovaterc`, `.renovaterc.json{,c,5}` (3), `package.json` = **14 entries,
`renovate.jsonc` at position 2**. Matches the plan's authoring measurement
exactly. The rendered docs section carries the injection marker
`<!-- config-filenames-begin -->` (`configuration-options.md:27`), i.e. the
rendered list is generated from this same source.

### Item 2 - jsonc preference

`docs/usage/configuration-options.md:45-47`:

```
  Renovate supports `JSONC` for `.json` files and any config files without file extension (e.g. `.renovaterc`).
  We also recommend you prefer using a `.jsonc` file if you want to add comments to your configuration, instead of a `.json5` file.
  Using an explicit `.jsonc` file is preferred over using a `.json` file with comments, as it can cause issues with editors and syntax highlighting.
```

### Item 3 - `$schema`

`lib/config/options/index.ts:734-740`:

```ts
    name: 'onboardingConfig',
    description: 'Configuration to use for onboarding PRs.',
    stage: 'repository',
    type: 'object',
    default: { $schema: 'https://docs.renovatebot.com/renovate-schema.json' },
```

The fence's `"$schema": "https://docs.renovatebot.com/renovate-schema.json"` is
the current value.

### Items 4 and 6 - `config:recommended` and `config:best-practices`

`lib/config/presets/internal/config.preset.ts`:

```ts
  'best-practices': {
    description:
      'Preset with best practices from the Renovate maintainers. Recommended for advanced users, who want to follow our best practices.',
    extends: [
      'config:recommended',
      'docker:pinDigests',
      'helpers:pinGitHubActionDigests',
      ':configMigration',
      ':pinDevDependencies',
      'abandonments:recommended',
      'security:minimumReleaseAgeNpm',
      ':maintainLockFilesWeekly',
    ],
  },
...
  recommended: {
    description:
      'Recommended configuration for most users. It does not matter what programming language you use.',
    extends: [
      ':dependencyDashboard',
      ':semanticPrefixFixDepsChoreOthers',
      ':ignoreModulesAndTests',
      'group:monorepos',
      'group:recommended',
      'mergeConfidence:age-confidence-badges',
      'replacements:all',
      'workarounds:all',
      'helpers:forgejoDigestChangelogs',
      'helpers:giteaDigestChangelogs',
      'helpers:githubDigestChangelogs',
      'helpers:gitlabDigestChangelogs',
      'helpers:goXPackagesChangelogLink',
      'helpers:goXPackagesNameLink',
      'helpers:renovateChangelog',
    ],
  },
```

Both reasons the fence's first comment gives for NOT extending
`config:best-practices` hold: it pulls `:maintainLockFilesWeekly` and
`docker:pinDigests`.

**6a** - `lib/config/presets/internal/default.preset.ts:359-365`:

```ts
  maintainLockFilesWeekly: {
    description: 'Run lock file maintenance (updates) early Monday mornings.',
    lockFileMaintenance: {
      enabled: true,
      extends: ['schedule:weekly'],
    },
  },
```

**6b** - `lib/config/presets/internal/docker.preset.ts:36-48`: `pinDigests` matches
`matchDatasources: ['docker']`. This repo has no Docker surface, so it is dead
weight rather than harmful; the fence's wording ("this repo has no Docker
surface") is accurate.

**6c** - `lib/config/presets/internal/security.preset.ts:26-34`:

```ts
  minimumReleaseAgeNpm: {
    description:
      'Wait until the npm package is three days old before raising the update. ...',
    packageRules: [
      {
        internalChecksFilter: 'strict',
        matchDatasources: ['npm'],
        minimumReleaseAge: '3 days',
      },
```

### Item 7 - `schedule:monthly`

`lib/config/presets/internal/schedule.preset.ts:5` and `:67-71`:

```ts
const monthly = ['* 0-3 1 * *'];
...
  monthly: {
    description:
      'Schedule once a month on the first day of the month before 4 AM.',
    schedule: monthly,
  },
```

The fence's comment ("That preset is ['* 0-3 1 * *'] - a four-hour window on the
1st") is literally correct.

### Item 8 - `prHourlyLimit` default: THE NAMED TRAP, CONFIRMED

`lib/config/options/index.ts:2224-2237`, the two entries pasted together because
their adjacency is the whole trap:

```ts
  {
    name: 'prHourlyLimit',
    description:
      'Rate limit PRs to maximum x created per hour. 0 means no limit.',
    type: 'integer',
    default: 2,
  },
  {
    name: 'prConcurrentLimit',
    description:
      'Limit to a maximum of x concurrent branches/PRs. 0 means no limit.',
    type: 'integer',
    default: 10,
  },
```

`prHourlyLimit` is `default: 2`; the `10` a naive read of the rendered page picks
up is `prConcurrentLimit`'s, from the immediately following entry. **The fence's
`"prHourlyLimit": 0` and its comment "The schema default is 2" are CORRECT and
were not touched.**

### Item 9 - backend cadence vs schedule windows

`docs/usage/key-concepts/scheduling.md:12-13`:

```
How often Renovate runs per-repository subsequently depends on how many repositories there are to check, and how many updates are pending for each repository at the time.
If the backend configuration for Renovate means it runs scheduled jobs per-repo approximately every X hours, it is not possible for _repository configuration_ to reduce that to less than X, or to force Renovate to run at exact times on specific repos.
```

This is the documented statement the fence's schedule comment leans on. It
supports the fence's wording ("the hosted service runs a repo's job no more often
than its own backend cadence allows, which the vendor documents as something repo
config cannot tighten") precisely - the doc states the *inability to tighten*, and
the starvation risk follows from that plus the preset's four-hour window.

### Item 10 - cargo `rangeStrategy`

Global default, `lib/config/options/index.ts:1904-1916`:

```ts
    name: 'rangeStrategy',
    description: 'Determines how to modify or update existing ranges.',
    type: 'string',
    default: 'auto',
    allowedValues: [
      'auto',
      'pin',
      'bump',
      'replace',
      'widen',
      'update-lockfile',
      'in-range-only',
    ],
```

`lib/modules/manager/cargo/readme.md`:

```
When using the default rangeStrategy=auto:

- If a "less than" instruction is found (e.g. `<2`) then `rangeStrategy=widen` will be selected,
- Otherwise, `rangeStrategy=update-lockfile` will be selected.

The `update-lockfile` default means that most upgrades will update `Cargo.lock` files without the need to change the value in `Cargo.toml`.
```

What `bump` does, `docs/usage/configuration-options.md` (`## rangeStrategy`, line 4514ff):

```
- `bump` = e.g. bump the range even if the new version satisfies the existing range, e.g. `^1.0.0` → `^1.1.0`
```

The fence's corrected wording (plan correction 3) - "the cargo manager's default
is the global rangeStrategy 'auto', which resolves to update-lockfile for cargo in
the normal case" - matches the readme exactly.

### Item 11 - GitHub Actions depTypes

`lib/modules/manager/github-actions/dep-types.ts`, both entries the fence matches on:

```ts
  {
    depType: 'action',
    description:
      'A repository-based action reference in a `uses:` field (e.g. `actions/checkout@v4`)',
  },
...
  {
    depType: 'github-runner',
    description:
      'A GitHub-hosted runner version in a `runs-on:` field (e.g. `ubuntu-24.04`)',
  },
```

Both depType *names* exist and are unchanged. The descriptions now carry an
appended parenthetical example that the plan's authoring quote does not show -
recorded as concern 4, not as a refutation: the fence matches on the name.

### Item 12 - npm `packageManager` depType

`lib/modules/manager/npm/dep-types.ts`:

```ts
  {
    depType: 'packageManager',
    prettyDepType: 'packageManager',
    description: 'Listed under `packageManager`',
  },
```

### Item 13 - managers

`lib/modules/manager/api.ts`, all five managers the fence names:

```
142:api.set('cargo', cargo);
164:api.set('github-actions', githubActions);
192:api.set('mise', mise);
196:api.set('npm', npm);
218:api.set('rust-toolchain', rustToolchain);
```

`lib/modules/manager/rust-toolchain/index.ts` - the pattern that must match this
repo's `rust-toolchain.toml`:

```ts
export const defaultConfig = {
  commitMessageTopic: 'Rust',
  managerFilePatterns: ['/(^|/)rust-toolchain(\\.toml)?$/'],
};
```

`lib/modules/manager/mise/index.ts:28-38` - the pattern that must match this
repo's `mise.toml`, so that disabling the manager is not a no-op:

```ts
export const defaultConfig = {
  managerFilePatterns: [
    '**/{,.}mise{,.*}.toml',
    '**/{,.}mise/config{,.*}.toml',
    '**/.config/mise{,.*}.toml',
    '**/.config/mise/{mise,config}{,.*}.toml',
    '**/.config/mise/conf.d/*.toml',
    '**/.rtx{,.*}.toml',
  ],
  pinDigests: false,
};
```

`**/{,.}mise{,.*}.toml` matches `mise.toml`, so the fence's `mise` disable rule
targets a manager that would otherwise be live on this repo.

### Item 14 - `separateMajorMinor`

Default, `lib/config/options/index.ts:1832-1837`:

```ts
    name: 'separateMajorMinor',
    description:
      'If set to `false`, Renovate will upgrade dependencies to their latest release only. Renovate will not separate major or minor branches.',
    type: 'boolean',
    default: true,
```

Priority over package groups, `docs/usage/configuration-options.md` (`## separateMajorMinor`):

```
This option also has priority over package groups configured by `packageRule`.
So Renovate will propose separate PRs for major and minor updates of packages even if they are grouped.
If you want to enforce grouped package updates, you need to set this option to `false` within the `packageRule`.
```

The absent-option decision is correct: default `true` + priority over groups =
majors already get their own PRs inside the groups, which is
`ci-04-dependabot-cadence`'s ruled behaviour. Setting it anywhere would be the
defect.

### Item 15 - `vulnerabilityAlerts`

Shipped default object, `lib/config/options/index.ts:2366-2385`:

```ts
    name: 'vulnerabilityAlerts',
    description:
      'Config to apply when a PR is needed due to a vulnerability in the existing package version.',
    type: 'object',
    default: {
      groupName: null,
      schedule: [],
      dependencyDashboardApproval: false,
      minimumReleaseAge: null,
      rangeStrategy: 'update-lockfile',
      commitMessageSuffix: '[SECURITY]',
      branchTopic: `{{{datasource}}}-{{{depNameSanitized}}}-vulnerability`,
      prCreation: 'immediate',
      vulnerabilityFixStrategy: 'lowest',
    },
    mergeable: true,
    cli: false,
    env: false,
    supportedPlatforms: ['github'],
```

`groupName: null` = ungrouped. `schedule: []` = no schedule window.
`prCreation: 'immediate'`.

Bypass of limits, `docs/usage/configuration-options.md` (`## vulnerabilityAlerts`):

```
  When Renovate creates a `vulnerabilityAlerts` PR, it ignores settings like `branchConcurrentLimit`, `commitHourlyLimit`, `prConcurrentLimit`, `prHourlyLimit`, or `schedule`.
  This means that Renovate _always_ tries to create a `vulnerabilityAlerts` PR.
  In short: vulnerability alerts "skip the line".
```

**The ruled behaviour (immediate, ungrouped, bypassing schedule and rate limits)
IS the shipped default, with no explicit configuration.** The defaults have NOT
moved, so the plan's third pre-routed fork does not fire and the fence's
deliberate omission of `vulnerabilityAlerts` stands.

The same section also carries the GitHub-side precondition that Step 4's
activation statement rests on:

```
For this to work, you must enable the [Dependency graph](...), and [Dependabot alerts](...).
```

### Verdict on Step 2

**Every fenced premise held. Nothing was refuted, so no value in the file was
changed and the "premise refuted at the source" fork did not fire.**

---

## 3. Both validator runs (Step 3)

### Version resolution

The fenced pin is `renovate@43.287.0`. It still resolves, so the fence's fallback
clause ("if that version no longer resolves") does **not** fire and the pinned
version was used as fenced:

```
$ curl -s -o /dev/null -w "%{http_code}\n" https://registry.npmjs.org/renovate/43.287.0
200
$ curl -s https://registry.npmjs.org/renovate/43.287.0 | head -c 120
{"name":"renovate","version":"43.287.0","keywords":["automated","azure","bazel","bitbucket","buildkite","dependencies","
```

Read anyway, because it is decision-relevant and is concern 1:

```
$ curl -s https://registry.npmjs.org/renovate/latest | python3 -c "import json,sys; print(json.load(sys.stdin)['version'])"
44.0.1
```

Local toolchain, for reproducibility: `node --version` -> `v26.5.0`,
`npx --version` -> `11.17.0`.

### Run 1 (fenced, plain)

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator renovate.jsonc
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'renovate@43.287.0',
npm warn EBADENGINE   required: { node: '^24.11.0', pnpm: '^11.0.0' },
npm warn EBADENGINE   current: { node: 'v26.5.0', npm: '11.17.0' }
npm warn EBADENGINE }
npm warn deprecated inflight@1.0.6: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.
npm warn deprecated glob@10.5.0: Old versions of glob are not supported, and contain widely publicized security vulnerabilities, which have been fixed in the current version. Please update. Support for old versions may be purchased (at exorbitant rates) by contacting i@izs.me
npm warn deprecated node-domexception@1.0.0: Use your platform's native DOMException instead
npm warn deprecated boolean@3.2.0: Package no longer supported. Contact Support at https://www.npmjs.com/support for more info.
npm warn deprecated rimraf@2.4.5: Rimraf versions prior to v4 are no longer supported
npm warn deprecated glob@6.0.4: Old versions of glob are not supported, and contain widely publicized security vulnerabilities, which have been fixed in the current version. Please update. Support for old versions may be purchased (at exorbitant rates) by contacting i@izs.me
 INFO: Validating renovate.jsonc as global config
 INFO: Config validated successfully against 1 file(s)
EXIT=0
```

The `npm warn` block is npm's install-time noise for the one-shot `npx` fetch, not
validator output; it appeared only on the first invocation (cache cold). It is
pasted in full rather than filtered, and its EBADENGINE line is concern 3.

### Run 2 (fenced, `--strict`)

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict renovate.jsonc
 INFO: Validating renovate.jsonc as global config
 INFO: Config validated successfully against 1 file(s)
EXIT=0
```

**No migration finding, no deprecation warning, no error.** The `--strict` fork
does not fire.

### Supplementary runs (evidence, not part of the fence)

Both fenced runs report `Validating renovate.jsonc as global config`. That is the
validator's documented default for an explicitly named file, not a defect in the
file - see concern 2 and the source excerpt there. Because the file is a *repo*
config, three additional runs were made so the repo-config code path is on the
record too. None of these is a substitute for the fenced runs; they are extra
measurement.

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict
 INFO: Validating renovate.jsonc
 INFO: Config validated successfully against 1 file(s)
EXIT=0
```

(no file argument -> auto-detection found `renovate.jsonc` through the documented
search order, which independently confirms item 1 at runtime)

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict --no-global renovate.jsonc
 INFO: Validating renovate.jsonc as repo config
 INFO: Config validated successfully against 1 file(s)
EXIT=0
```

```
$ npx --yes --package renovate@44.0.1 -- renovate-config-validator --strict --no-global renovate.jsonc
 INFO: Validating renovate.jsonc as repo config
 INFO: Config validated successfully against 1 file(s)
EXIT=0
```

(the currently published major, i.e. the version the hosted app will actually run
against this repo; strict, repo config, clean)

### The validation left nothing behind

The claim was verified, not assumed:

```
$ git status --porcelain
?? renovate.jsonc
$ git diff --stat -- package.json pnpm-lock.yaml
(no output)
```

No gate part, no CI job, no dependency, no lockfile entry, no npx artifact inside
the repo. `npx --yes --package` resolves into npm's own `_npx` cache outside the
working tree.

---

## 4. The transcription check (method named)

Not an eyeball. The fence body was extracted from the plan **mechanically** and
compared byte for byte:

```
$ PLAN=docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md
$ START=$(awk 'NR>360 && /^```jsonc$/{print NR; exit}' "$PLAN")
$ END=$(awk -v s="$START" 'NR>s && /^```$/{print NR; exit}' "$PLAN")
$ echo "fence body = lines $((START+1))..$((END-1))"
fence body = lines 376..459
$ sed -n "$((START+1)),$((END-1))p" "$PLAN" > .../fence.jsonc
$ diff -u .../fence.jsonc renovate.jsonc && echo "DIFF: byte-identical (exit $?)"
DIFF: byte-identical (exit 0)
$ md5sum .../fence.jsonc renovate.jsonc
c3c3b7da87843b050ede361049e05b46  .../fence.jsonc
c3c3b7da87843b050ede361049e05b46  renovate.jsonc
```

**Fire control for that check** (a passing `diff` is an absence, so it was made to
produce output once):

```
$ cp renovate.jsonc .../mutated.jsonc
$ sed -i 's/"prHourlyLimit": 0/"prHourlyLimit": 1/' .../mutated.jsonc
$ diff -u .../fence.jsonc .../mutated.jsonc
@@ -22,7 +22,7 @@
 
   // The schema default is 2, which would dribble the ecosystem group PRs
   // across successive jobs instead of opening them in one pass.
-  "prHourlyLimit": 0,
+  "prHourlyLimit": 1,
 
   // separateMajorMinor is deliberately absent. Its default is true and it
   // has priority over package groups, so major updates already get their
diff exit on a one-character mutation: 1
```

**Re-run AFTER the full gate**, because the plan's Task 2 hit a fence-vs-formatter
collision and this task's fence is JSONC:

```
$ diff -u .../fence.jsonc renovate.jsonc ; echo "diff exit: $?"
diff exit: 0
$ md5sum renovate.jsonc
c3c3b7da87843b050ede361049e05b46  renovate.jsonc
```

**No gate part rewrote or rejected the file.** `pnpm lint` is `eslint .`, and
`eslint.config.js` registers only the default JS/TS/Vue file sets (no JSON/JSONC
processor), so `.jsonc` is outside its file selection; it neither linted nor
touched the file. No other gate part reads it. The Task-2 collision class did not
recur, so there was nothing to report under it.

**And against the committed object**, not just the working tree:

```
$ git show HEAD:renovate.jsonc | md5sum
c3c3b7da87843b050ede361049e05b46  -
$ git show HEAD:renovate.jsonc | diff -u .../fence.jsonc - ; echo "diff exit: $?"
diff exit: 0
```

### Typography

Checked as a superset rather than by recalling which glyphs to look for - any
non-ASCII byte at all, with its own fire control:

```
$ LC_ALL=C grep -nP '[^\x00-\x7F]' renovate.jsonc ; echo "grep exit: $?"
grep exit: 1
$ LC_ALL=C grep -cP '[^\x00-\x7F]' rust-toolchain.toml
1
```

Exit 1 = no non-ASCII byte in `renovate.jsonc`; the same expression returns a
match against a file that does contain one, so the instrument is sound. ASCII
hyphens, straight quotes, no Unicode ellipsis - by construction, since the file is
byte-identical to the fence.

---

## 5. Activation statement (Step 4), in the exact scope the plan allows

**Writing this file does not activate Renovate.** Activation needs two actions
only the OWNER can take:

1. **Install the Renovate app against this repository.**
2. **Enable the repository's dependency graph and Dependabot alerts**, so the
   immediate-security half exists at all. This is the vendor's own stated
   precondition, quoted from `docs/usage/configuration-options.md` under
   `## vulnerabilityAlerts` in item 15 above.

**The ordering matters:** the config must be on `master` **before** the app is
installed. That is what suppresses the vendor's onboarding pull request - the
onboarding worker treats a repository as already onboarded as soon as a config
file exists.

Both remain **OPEN**. Acceptance row W4-c is not satisfied by this task and is not
claimed to be.

**The ROADMAP trigger "Renovate/Dependabot activated -> prune deny.toml RUSTSEC
ignores; TS-7 bump arrives via its PRs" has NOT fired.** Its condition is
activation, not configuration. `deny.toml` was not read for content, not edited,
and is untouched by this commit.

---

## 6. Full gate result (Step 5)

Run foreground, in `BUILDING.md`'s own order, on the tree carrying the new
(then-untracked) `renovate.jsonc`, before the commit. `BUILDING.md`'s canonical
statement is "The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1
house-knowledge", and all eleven commands were run.

**Harness soundness first.** Several parts were piped to `tail` to keep the
transcript readable, which makes `$?` the pipe's status unless `pipefail` is on.
That was verified rather than assumed, with a fire control, because a green gate
whose exit-code capture is broken is not evidence:

```
$ set -o | grep -i pipefail
pipefail              on
$ cargo fmt --all --check --manifest-path /nonexistent/Cargo.toml 2>&1 | tail -3; e=$?; echo "EXIT=$e"
  -h, --help
          Print help

EXIT=1 (must be non-zero for the harness to be sound)
```

| # | Block | Command | Exit |
|---|---|---|---|
| 1 | Rust | `cargo fmt --all --check` | 0 |
| 2 | Rust | `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| 3 | Rust | `cargo test --workspace` | 0 |
| 4 | Rust | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 |
| 5 | Rust | `cargo deny check` | 0 |
| 6 | Rust | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 |
| 7 | frontend | `pnpm lint` | 0 |
| 8 | frontend | `pnpm build` | 0 |
| 9 | frontend | `pnpm check:i18n` | 0 |
| 10 | frontend | `pnpm test:e2e` | 0 |
| 11 | house | `python3 scripts/ledger-lint.py` | 0 |

Terminal lines, pasted:

```
##### 5/11 cargo deny check
advisories ok, bans ok, licenses ok, sources ok
EXIT=0

##### 7/11 pnpm lint
$ eslint .
EXIT=0

##### 9/11 pnpm check:i18n
check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
EXIT=0

##### 10/11 pnpm test:e2e
  68 passed (2.9s)
EXIT=0

##### 11/11 python3 scripts/ledger-lint.py
ledger-lint: 535 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
EXIT=0
```

**No `FAIL BUILDING.md: ...` line.** Task 1's gate-count invariant ran green (it
is inside part 11, whose summary line names it), so nothing wrote to `BUILDING.md`
outside its owner's Files list. The defect-signal fork did not fire.

**What the gate does and does not prove here.** `renovate.jsonc` is consumed by no
gate part - not by `eslint`, not by `vue-tsc`, not by `ledger-lint.py`, not by any
cargo command. **The gate therefore proves only that nothing else broke.** This
file's own evidence is the validator runs in section 3.

---

## 7. Working-tree evidence

Before the commit:

```
$ git status --porcelain
?? renovate.jsonc
$ git add renovate.jsonc
$ git diff --cached --stat
 renovate.jsonc | 84 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 1 file changed, 84 insertions(+)
$ git status --porcelain
A  renovate.jsonc
```

Exactly one new file, nothing else touched.

After the commit (tree clean, so both are empty - the pre-commit staged diffstat
above is the "exactly one new file" evidence):

```
$ git diff --stat
(empty)
$ git status --porcelain
(empty)
```

---

## 8. Concerns a reviewer can rule on yes/no

1. **The pinned validator version is one MAJOR behind current.** The fence pins
   `renovate@43.287.0`; `https://registry.npmjs.org/renovate/latest` returned
   `44.0.1` today. The fence's fallback clause is conditioned on the pin *no
   longer resolving*, and it does resolve (HTTP 200), so I used the pin exactly as
   written and did not substitute. I additionally validated against `44.0.1`
   (strict, repo config): clean. **Question: should the fence's pin be refreshed
   to the then-current version whenever this config is re-validated, or does the
   authoring pin stay as the reproducible reference?** Not a defect in the file;
   a question about the fence.
2. **The fenced invocation validates the file as GLOBAL config, not as repo
   config.** Both fenced runs print `Validating renovate.jsonc as global config`.
   That is documented default behaviour, from `lib/config-validator.ts` at tag
   `43.287.0`:

   ```
   When specifying [config-files...], Renovate will treat them as global self-hosted configuration files. You can disable this behaviour with --no-global
   ```

   ```ts
       if (files.length) {
         let isGlobalConfig = true;
         if (opts.global === false) {
           isGlobalConfig = false;
         }
         const configType = isGlobalConfig ? 'global' : 'repo';
   ```

   Global validation accepts a superset of options, so it would not flag a
   `globalOnly` option wrongly present in a repo config. This file contains no
   `globalOnly` option (`$schema`, `extends`, `timezone`, `schedule`,
   `prHourlyLimit`, `packageRules` are all repo options), and the two
   supplementary repo-config runs in section 3 pass, so **the config is fine
   either way**. **Question: should the fenced invocation carry `--no-global`, or
   the no-argument auto-detect form, in any future re-validation?** I did not
   change the fence, per "must not decide: the validator invocation".
3. **`EBADENGINE` on the pinned validator.** `renovate@43.287.0` declares
   `required: { node: '^24.11.0', pnpm: '^11.0.0' }`; this machine runs
   `node v26.5.0` (the version `mise.toml` pins). npm warns, does not fail, and
   the validator ran to completion with exit 0. Recording it because it is an
   observed divergence between the repo's pinned Node and the pinned validator's
   declared engine range, and because it will recur on every re-run. **Question:
   acceptable as-is?** No action taken.
4. **Wording drift in two depType descriptions, not a refutation.** At current
   main the `action` and `github-runner` descriptions carry an appended example
   (`(e.g. `actions/checkout@v4`)`, `(e.g. `ubuntu-24.04`)`) that the plan's
   authoring quote does not show. The depType *names* - the only thing
   `matchDepTypes` matches on - are unchanged. Flagged only so a reviewer
   re-deriving item 11 against the plan's quoted text is not surprised by the
   delta.

---

## 9. Divergences and judgment calls, each named

1. **Steps 1 and 2 were executed in the order 2-then-1** (re-verification before
   writing the file), not 1-then-2 as the plan numbers them. Reason: the plan
   pre-routes a refuted premise to NEEDS_CONTEXT, and writing first would have
   left an untracked artifact behind in that branch. No step's output depends on
   the order, the deliverable is identical, and every fenced premise held anyway,
   so the branch never arose. Named because it is a divergence from the written
   sequence.
2. **Three supplementary validator runs beyond the two fenced ones** (section 3).
   Evidence-gathering, not a change to the fence: the fenced commands were run
   first, verbatim, and their output is what section 3 leads with. Motivated by
   the `as global config` line the fenced runs printed.
3. **The source was read at a pinned commit SHA** (`04b629b...`) rather than at
   `main` unqualified, so every quoted value is reproducible. Method choice, not a
   scope change.
4. **The gate transcript is tailed for the long parts.** Full output of
   `cargo test --workspace` and `pnpm test:e2e` is thousands of lines; each part's
   exit code was captured under a fire-verified `pipefail` harness (section 6) and
   the terminal lines are pasted. Named so the reviewer knows the exit codes, not
   the excerpts, are the evidence.

Nothing else. **No fork was resolved at the keyboard**; none arose. No option was
added, removed, or altered; the file is byte-identical to the fence.

---

## 10. What I surface for the controller

Report-only, per the standing rule that no task edits house-knowledge YAML,
`docs/ROADMAP.md` or `docs/process-journal.md`.

1. **W4-c stays OPEN.** The two owner activation actions (section 5) are not done
   and are not claimable by this task. The ordering rider - config on `master`
   BEFORE app installation - is now satisfiable: the config is on `master` as of
   `630d418`.
2. **The ROADMAP trigger has not fired** (section 5). No `deny.toml` pruning, no
   TS-7 movement.
3. **Ledger-worthy, the validator's global-vs-repo default** (concern 2). Any
   future dispatch that re-validates a Renovate config by naming the file will
   silently validate it as a *global self-hosted* config unless it passes
   `--no-global` or omits the argument. This is a trap of exactly the class the
   house records: the command looks right, exits 0, and answers a slightly
   different question than the one asked.
4. **The `prHourlyLimit` trap reproduced exactly as the plan predicted** (item 8),
   and it is a permanent property of the rendered page's layout, not a transient
   docs bug: the `prConcurrentLimit` entry with `default: 10` sits immediately
   after `prHourlyLimit` with `default: 2` in the same source array. The plan's
   general rule - source over rendered page - earned its place here.
5. **`renovate@44.0.1` is current** (concern 1). If the controller wants the
   plan's authoring pin refreshed anywhere, this is the number, and the config
   validates clean against it.
6. **No premise refuted.** The plan's Renovate premises block and its corrections
   3 and 4 all hold at the vendor's source one day after authoring, including the
   `.gitlab/` addition to the filename patterns, which leaves the "item 2 of 14"
   measurement intact.

---

## 11. Commit

```
$ git -c commit.gpgsign=false commit -m "renovate: the ruled monthly per-ecosystem config, majors and security on their own paths" -m "Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>"
[master 630d418] renovate: the ruled monthly per-ecosystem config, majors and security on their own paths
 1 file changed, 84 insertions(+)
 create mode 100644 renovate.jsonc
```

```
$ git show --stat HEAD
commit 630d4183f5bf2403b99f19cbb31b8b803973b659
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Wed Jul 29 15:22:15 2026 +0200

    renovate: the ruled monthly per-ecosystem config, majors and security on their own paths
    
    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 renovate.jsonc | 84 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 1 file changed, 84 insertions(+)
```

Exactly one trailer, verified mechanically:

```
$ git log -1 --format="%(trailers)"
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

No `Claude-Session` line, no context-window suffix, unsigned per SI-4. Committed
on `master` in the main worktree. **Not pushed** - the single push is a plan-close
action.
