# Task B1 implementer report - Plan 11, stream B (W1)

**Status:** DONE_WITH_CONCERNS
**Commit:** `c42299936a23d2818d3b7926f2f60f8d9c3901ca` on `plan-11-stream-b`
**Worktree:** `/home/senol/Git/muxsmith-plan11-b`
**Worktree base:** `5378264f35616adcb2356e297301b5fa3b8b5719`
**Files changed:** `pnpm-lock.yaml`, `deny.toml` - exactly the two the Files list names.

Two findings contradict the plan's authoring section. Neither changes any decision
and neither was resolved at the keyboard; both are in "Findings" below with their
pasted output.

---

## Step 1: starting state

```
$ pnpm --version
11.10.0

$ node --version
v26.5.0

$ git rev-parse HEAD
5378264f35616adcb2356e297301b5fa3b8b5719

$ grep -nE '^ *postcss@|postcss: ' pnpm-lock.yaml
1082:  postcss@8.5.16:
1734:      postcss: 8.5.16
2242:  postcss@8.5.16:
2354:      postcss: 8.5.16

$ cargo deny --version
cargo-deny 0.19.9

$ grep -n -A2 '^name = "glib"' Cargo.lock
1346:name = "glib"
1347-version = "0.18.5"
1348-source = "registry+https://github.com/rust-lang/crates.io-index"
```

```
$ pnpm why postcss
postcss@8.5.16
├─┬ @vue/compiler-sfc@3.5.39
│ └─┬ vue@3.5.39
│   ├─┬ @vitejs/plugin-vue@6.0.7
│   │ └── muxsmith-gui@0.1.0 (devDependencies)
│   ├─┬ @vue/server-renderer@3.5.39
│   │ └── vue@3.5.39 [circular]
│   ├─┬ fluent-vue@3.8.2
│   │ └── muxsmith-gui@0.1.0 (dependencies)
│   ├── muxsmith-gui@0.1.0 (dependencies)
│   └─┬ vue-demi@0.14.10
│     └── fluent-vue@3.8.2 [deduped]
└─┬ vite@8.1.4
  ├── @vitejs/plugin-vue@6.0.7 [deduped]
  └── muxsmith-gui@0.1.0 (devDependencies)

Found 1 version of postcss
```

Pre-state lockfile proven self-consistent:

```
$ pnpm install --frozen-lockfile
[... dependency listing ...]
Done in 402ms using pnpm v11.10.0
(exit 0)
```

---

## Step 2: part (a), the `postcss` lockfile bump

Mechanism as prescribed - a lockfile-level update of a transitive dependency. Not
a `pnpm.overrides` entry, not a direct dependency addition. The reason, stated as
the step requires: this project pins direct dependencies exactly
(`ci-10-pin-everything`), and an override for a build-time transitive package
changes what the manifest asserts about the dependency graph. `--depth` was left
at its default of `Infinity`, which is what reaches a transitive dependency; no
`--latest`, because both parents' declared ranges already admit the patched
version.

```
$ pnpm update postcss
✓ Lockfile passes supply-chain policies (verified 8d ago)
Progress: resolved 1, reused 0, downloaded 0, added 0
[WARN] 1 deprecated subdependencies found: glob@10.5.0
Packages: +2 -2
++--
Progress: resolved 265, reused 227, downloaded 2, added 2, done

Done in 1s using pnpm v11.10.0
(exit 0)
```

The `glob@10.5.0` deprecation warning is pre-existing (it is a transitive
dependency unrelated to postcss and did not move in the diff below).

### W1-a: the landing version, measured

**Requirement `>= 8.5.18`. Observed: `8.5.25`.** The authoring probe landed on
`8.5.24` while the registry's `latest` read `8.5.25`; this run landed on `8.5.25`.
Not fenced, and pasted rather than recalled:

```
$ pnpm why postcss
postcss@8.5.25
├─┬ @vue/compiler-sfc@3.5.39
│ └─┬ vue@3.5.39
│   ├─┬ @vitejs/plugin-vue@6.0.7
│   │ └── muxsmith-gui@0.1.0 (devDependencies)
│   ├─┬ @vue/server-renderer@3.5.39
│   │ └── vue@3.5.39 [circular]
│   ├─┬ fluent-vue@3.8.2
│   │ └── muxsmith-gui@0.1.0 (dependencies)
│   ├── muxsmith-gui@0.1.0 (dependencies)
│   └─┬ vue-demi@0.14.10
│     └── fluent-vue@3.8.2 [deduped]
└─┬ vite@8.1.4
  ├── @vitejs/plugin-vue@6.0.7 [deduped]
  └── muxsmith-gui@0.1.0 (devDependencies)

Found 1 version of postcss

$ grep -nE '^ *postcss@|postcss: ' pnpm-lock.yaml
1082:  postcss@8.5.25:
1734:      postcss: 8.5.25
2242:  postcss@8.5.25:
2354:      postcss: 8.5.25
```

All four sites moved. `8.5.25 >= 8.5.18` holds, and `Found 1 version of postcss`
means no second resolution was left behind.

The genuine fork did not materialise: no transitive parent constrains `postcss`
below the patched version, so no NEEDS_CONTEXT is owed on part (a).

### W1-b: the manifest did not move, with the instrument fired

```
$ git diff --exit-code -- package.json
(no output)
EXIT: 0

$ git diff --exit-code --stat -- pnpm-lock.yaml
 pnpm-lock.yaml | 18 +++++++++---------
 1 file changed, 9 insertions(+), 9 deletions(-)
EXIT: 1

$ grep -c '"overrides"' package.json
0
EXIT: 1
```

The pair is the point: the same `--exit-code` instrument on the same tree exits
**1** for the file that moved and **0** for the file that did not, so the zero is
a measurement rather than a misspelled path.

### W1-c: the diff covers exactly two packages

```
$ git diff -- pnpm-lock.yaml
diff --git a/pnpm-lock.yaml b/pnpm-lock.yaml
index 69b3185..22f6370 100644
--- a/pnpm-lock.yaml
+++ b/pnpm-lock.yaml
@@ -1007,8 +1007,8 @@ packages:
   muggle-string@0.4.1:
     resolution: {integrity: sha512-VNTrAak/KhO2i8dqqnqnAHOa3cYBwXEZe9h+D5h/1ZqFSTEFHdM65lR7RoIqq3tBBYavsOXV84NoHXZ0AkPyqQ==}
 
-  nanoid@3.3.15:
-    resolution: {integrity: sha512-y7Wygv/7mEOvxTuEQDB8StXdMRBWf1kR/tlhAzBRUFkB2jfcLOAxO/SHmOO2zgz1pVgK29/kyupn059/bCHdjA==}
+  nanoid@3.3.16:
+    resolution: {integrity: sha512-bzlKTyNJ7+LdGIIwy8ijFpIqEQIvafahV7eYykJ8Cvh42EdJeODoJ6gUJXpQJvej1BddH8OqTXZNE/KfbWAu8Q==}
     engines: {node: ^10 || ^12 || ^13.7 || ^14 || >=15.0.1}
     hasBin: true
 
@@ -1079,8 +1079,8 @@ packages:
     resolution: {integrity: sha512-HeP7D2wyhkR+XaK6v4W8oRF62Dsz4flyuczALJp61GckGm42u1saSSJ/0auvcBqxs3jMRFEcPK34At/0JBKdOg==}
     engines: {node: '>=4'}
 
-  postcss@8.5.16:
-    resolution: {integrity: sha512-vuwillviilfKZsg0VGj5R/YwwcHx4SLsIOI/7K6mQkWx+l5cUHTjj5g0AasTBcyXsbfTgrwsUNmVUb5xVwyPwg==}
+  postcss@8.5.25:
+    resolution: {integrity: sha512-DTPx3RWSSnWyzLxQnlH0rJP+EW5ekl16ZU4/psbIhA0e53kJfdgaN5vKM+xP7yJtXVu+nfdVFmlgFDEKAe4Pyw==}
     engines: {node: ^10 || ^12 || >=14}
 
   prelude-ls@1.2.1:
@@ -1731,7 +1731,7 @@ snapshots:
       '@vue/shared': 3.5.39
       estree-walker: 2.0.2
       magic-string: 0.30.21
-      postcss: 8.5.16
+      postcss: 8.5.25
       source-map-js: 1.2.1
 
   '@vue/compiler-ssr@3.5.39':
@@ -2174,7 +2174,7 @@ snapshots:
 
   muggle-string@0.4.1: {}
 
-  nanoid@3.3.15: {}
+  nanoid@3.3.16: {}
 
   natural-compare@1.4.0: {}
 
@@ -2239,9 +2239,9 @@ snapshots:
       cssesc: 3.0.0
       util-deprecate: 1.0.2
 
-  postcss@8.5.16:
+  postcss@8.5.25:
     dependencies:
-      nanoid: 3.3.15
+      nanoid: 3.3.16
       picocolors: 1.1.1
       source-map-js: 1.2.1
 
@@ -2351,7 +2351,7 @@ snapshots:
     dependencies:
       lightningcss: 1.32.0
       picomatch: 4.0.5
-      postcss: 8.5.16
+      postcss: 8.5.25
       rolldown: 1.1.5
       tinyglobby: 0.2.17
     optionalDependencies:
```

**Every changed package, named:** `postcss` (8.5.16 -> 8.5.25) and `nanoid`
(3.3.15 -> 3.3.16). Two, no third. `nanoid` is in scope because the diff itself
shows it as postcss's own dependency (`postcss@8.5.25: dependencies: nanoid:
3.3.16`), which is the reachability the step requires - so no NEEDS_CONTEXT is
owed on that either.

Hunk figures, for comparison with the authoring probe's context-carrying number:

```
$ git diff -U3 -- pnpm-lock.yaml | grep -c '^@@'
6
$ git diff -U0 -- pnpm-lock.yaml | grep -c '^@@'
7
```

6 hunks at git's default `-U3`, 7 at `-U0`, 9 insertions and 9 deletions -
identical to the authoring probe in every respect except the landing version.

### The new lockfile is internally consistent

```
$ pnpm install --frozen-lockfile
Already up to date
Done in 20ms using pnpm v11.10.0
(exit 0)
```

---

## Step 3: the four frontend gate parts on the bumped lockfile (W1-d)

Each run foreground, individually, because a transitive lockfile bump can move
exactly one of them.

```
$ pnpm lint
$ eslint .
EXIT: 0

$ pnpm build
$ vue-tsc --noEmit && vite build
vite v8.1.4 building client environment for production...
transforming...✓ 165 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                   0.39 kB │ gzip:   0.26 kB
dist/assets/index-DGn2eD1R.css    1.31 kB │ gzip:   0.49 kB
dist/assets/index-CO0ABiMW.js   325.85 kB │ gzip: 105.87 kB

✓ built in 174ms
EXIT: 0

$ pnpm check:i18n
$ node scripts/check-i18n.mjs
check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
EXIT: 0

$ pnpm test:e2e
$ tsc --noEmit -p e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && vite build --config e2e/vite.mount.config.ts && playwright test
[...]
  68 passed (3.0s)
EXIT: 0
```

All four green. No red result, so nothing to report as a finding here.

Note on evidence hygiene: the first `pnpm test:e2e` run was piped to `tail` and
its exit code was swallowed by a zsh/bash `PIPESTATUS` difference (the login shell
here is `/usr/bin/zsh`). It was re-run redirected to a file so the exit code above
is the real one, not an inference from the last line of output. Every exit code in
this report was captured that way.

---

## Step 4: part (b), the two named `deny.toml` regions

Both insertions applied verbatim from the plan's fences, nothing else in the file.
The bounded diff is under Step 7.

### Verification of the file after the edit

```
$ grep -cE '^\s*"RUSTSEC-' deny.toml
19

$ python3 -c "import tomllib; d=tomllib.load(open('deny.toml','rb')); ..."
advisories.unsound == 'all'
ignore len: 19
RUSTSEC-2024-0429 in ignore: True
```

19 ids as the step states, the file parses as TOML, and `advisories.unsound` reads
`all`.

### W1-e / W1-f: the advisory's class and its GHSA alias, at RustSec's own record

Path: `/home/senol/.cargo/advisory-dbs/advisory-db-3157b0e258782691/crates/glib/RUSTSEC-2024-0429.md`

```toml
[advisory]
id = "RUSTSEC-2024-0429"
package = "glib"
date = "2024-03-30"
url = "https://github.com/gtk-rs/gtk-rs-core/pull/1343"
informational = "unsound"
aliases = ["GHSA-wrw7-89jp-8q8g"]

[affected.functions]
"glib::VariantStrIter::next" = [">=0.15.0,<0.20.0"]
"glib::VariantStrIter::nth" = [">=0.15.0,<0.20.0"]
"glib::VariantStrIter::last" = [">=0.15.0,<0.20.0"]
"glib::VariantStrIter::next_back" = [">=0.15.0,<0.20.0"]
"glib::VariantStrIter::nth_back" = [">=0.15.0,<0.20.0"]

[versions]
patched = [">=0.20.0"]
unaffected = ["<0.15.0"]
```

Title line, quoted from the same file:
`# Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter``

**The GitHub side, and a deliberate non-measurement with its reason.** The plan's
authoring section pastes the alert record
`{"ghsa":"GHSA-wrw7-89jp-8q8g","manifest":"Cargo.lock","patched":"0.20.0","pkg":"glib","sev":"medium","vuln_range":">= 0.15.0, < 0.20.0"}`
from `gh api repos/senolfeldmann/Muxsmith/dependabot/alerts`. **I did not re-run
that call**, because every `gh` interaction against one of the owner's own repos
owes an entry in `gh-log.md`, which lives only in the main worktree at
`/home/senol/Git/Muxsmith/gh-log.md` - a path this dispatch forbids me to touch.
Rather than either violate that boundary or leave the claim borrowed, I verified
the alias against the public GitHub advisory, which is a third-party read and
carries no log duty:

`https://github.com/advisories/GHSA-wrw7-89jp-8q8g` returns package `glib`
(Rust/Cargo), title `Unsoundness in `Iterator` and `DoubleEndedIterator` impls for
`glib::VariantStrIter``, severity `Moderate` (CVSS v4 6.9), affected
`>= 0.15.0, < 0.20.0`, first patched `0.20.0`.

That independently corroborates all four substantive fields of the pasted alert
record (package, severity class, range, patched version) and confirms the alias
resolves to one advisory rather than two. The one half that remains borrowed from
the controller's authoring run is that **Muxsmith's own open alert** carries that
GHSA id; it is attributed rather than claimed as mine. The controller owns
`gh-log.md` and can re-run the call in one command.

### W1-g: the mechanism, settled at cargo-deny's own `Default` impl

Read at `/home/senol/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/src/advisories/cfg.rs`:

```rust
    /// Whether to error on unmaintained advisories, and for what scope
    pub unmaintained: Spanned<Scope>,
    /// Whether to error on unsound advisories, and for what scope
    pub unsound: Spanned<Scope>,
```

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: None,
            db_urls: Vec::new(),
            ignore: Vec::new(),
            unmaintained: Spanned::new(crate::cfg::Scope::All),
            unsound: Spanned::new(crate::cfg::Scope::Workspace),
```

and the deserializer's own fallback in the same file:

```rust
            unmaintained: unmaintained.unwrap_or(Spanned::new(Scope::All)),
            unsound: unsound.unwrap_or(Spanned::new(Scope::Workspace)),
```

`Scope` at `.../cargo-deny-0.19.9/src/cfg.rs`:

```rust
pub enum Scope {
    /// Matches any crate
    All,
    /// Matches crates in the workspace
    Workspace,
    /// Matches external crates
    Transitive,
    /// Matches no crates
    None,
}
```

So the default `unsound` scope is `Workspace`, `Workspace` matches only workspace
crates, and `glib` is external. That is the mechanism, read at the `Default` impl
rather than inferred from the tool's output.

**Additional measurement not in the plan, relevant to finding 2 below:** the
config struct carries a scope key for exactly two classes. Its full field list:

```
70:    pub db_path: Option<Spanned<PathBuf>>,
72:    pub db_urls: Vec<Spanned<Url>>,
74:    pub yanked: Spanned<LintLevel>,
78:    pub unmaintained: Spanned<Scope>,
80:    pub unsound: Spanned<Scope>,
82:    pub ignore_yanked: Vec<Spanned<PackageSpecOrExtended<Reason>>>,
84:    pub git_fetch_with_cli: Option<bool>,
86:    pub disable_yank_checking: bool,
94:    pub maximum_db_staleness: Spanned<Duration>,
97:    pub unused_ignored_advisory: LintLevel,
```

There is **no scope key for the vulnerability class** - a real vulnerability is
always reported regardless of workspace/transitive. That matters below.

---

## Step 5: the three-way fire

The two variant configs are **copies at a scratch path outside the repository**,
driven through `cargo deny check advisories -c <path>`. The repository's own
`deny.toml` was never mutated to produce them; the bounded diff in Step 7 shows
its content is exactly the two Step-4 insertions and nothing else (17 insertions,
**zero deletions**, two hunks), which is the stronger form of that proof than a
bare `--exit-code` run, since `deny.toml` legitimately differs from HEAD after
Step 4.

Variant construction, pasted:

```
variant2: removed lines 53 to 61 ( 9 lines )      # the ignore entry + its 8 comment lines; scope kept
variant3: additionally removed the unsound line at 14

deny-variant2-scope-on-no-ignore.toml -> unsound: all       | ignore ids: 18 | 0429 present: False
deny-variant3-no-scope-no-ignore.toml -> unsound: <ABSENT>  | ignore ids: 18 | 0429 present: False
```

### Run 1 - shipped state (W1-k, first half)

```
$ cargo deny check advisories
advisories ok
EXIT: 0
```

### Run 2 - the scope is live (W1-h)

```
$ cargo deny check advisories -c <scratch>/deny-variant2-scope-on-no-ignore.toml
error[unsound]: Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
    ┌─ /home/senol/Git/muxsmith-plan11-b/Cargo.lock:138:1
    │
138 │ glib 0.18.5 registry+https://github.com/rust-lang/crates.io-index
    │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ unsound advisory detected
    │
    ├ ID: RUSTSEC-2024-0429
    ├ Advisory: https://rustsec.org/advisories/RUSTSEC-2024-0429
    ├ The `VariantStrIter::impl_get` function (called internally by implementations of the `Iterator` and `DoubleEndedIterator` traits for this type) was unsound, resulting in undefined behaviour.
      [... advisory prose ...]
    ├ Announcement: https://github.com/gtk-rs/gtk-rs-core/pull/1343
    ├ Solution: Upgrade to >=0.20.0 (try `cargo update -p glib`)
    ├ glib v0.18.5
      ├── atk v0.18.2
      │   └── gtk v0.18.2
      │       ├── muda v0.19.3
      │       │   └── tauri v2.11.5
      [... the full inclusion graph, reproduced in Step 8 ...]

advisories FAILED
EXIT: 1
```

Exit 1, `advisories FAILED`, the exact `error[unsound]` string the plan
prescribes, and `ID: RUSTSEC-2024-0429`. All three as specified.

### Run 3 - the control: the scope is what does the work (W1-i)

```
$ cargo deny check advisories -c <scratch>/deny-variant3-no-scope-no-ignore.toml
advisories ok
EXIT: 0
```

Exit 0 with the ignore entry **also** absent. So run 2's failure is attributable
to the scope key and not to a config that would have failed anyway.

No deviation from the three prescribed outcomes.

---

## Step 6: what the change buys, and what it does not

Stated in the step's own four terms.

**1. Gate part `cargo deny check` and the GitHub alert feed now agree about the
unsound class,** so either may be quoted as coverage again - which neither could
be while the disagreement stood. That, not a green run, is what part (b) was for.
Before the change the class was not evaluated for external crates at all, so a
green `cargo deny check` said nothing about it; after the change the class is
evaluated over every crate and the one advisory it finds is the one GitHub
reports, recorded with its reason.

**2. The two mechanisms never actually disagreed.** cargo-deny's default scope for
the unsound class is `Workspace`, which by the `Scope` enum's own doc comment
matches only workspace crates; `glib` is external; GitHub's feed has no such
scope. This is the resolution of the ROADMAP's "the gap worth more than either
alert" item: **it was a configuration default, not a hole in either mechanism, and
not a silenced advisory.** Measured, not inferred - the `Default` impl is quoted
above, and the pre-state run below shows the id appearing on zero lines with a
fired control proving the search works.

```
$ grep -c 'RUSTSEC-2024-0429' <pre-state -L info run>
0
$ grep -c 'RUSTSEC-2024-0415' <the same run>     # fired control: an id that IS present
3
```

**3. The advisory is ignored, not fixed.** `glib` remains at 0.18.5, the
vulnerable version. The ignore entry records a known, accepted, argued exposure
with a drop condition (the ROADMAP's trigger: a dependency PR or Tauri release
moving the gtk-rs generation past 0.18 in `Cargo.lock`). **This report does not
describe the alert as resolved, and the `glib` alert on GitHub is not dismissed by
this task** - dismissing an alert is an owner action.

**4. Blast radius, restated from my own measurement rather than from the plan's.**
Tallied as a set over run 2's output:

```
$ grep -oE '^(error|warning)\[[a-z-]+\]' run2.log | sort | uniq -c
      1 error[unsound]

$ grep -oE 'RUSTSEC-[0-9]{4}-[0-9]{4}' run2.log | sort -u
RUSTSEC-2024-0429

pre-existing ignore ids (the Step-4 entry excluded): 18
fired ids: ['RUSTSEC-2024-0429']
fired MINUS pre-existing ignores: ['RUSTSEC-2024-0429']
```

One `error[unsound]`, zero other error or warning classes, distinct fired ids the
single-member set `{RUSTSEC-2024-0429}`, and set-differencing the fired ids
against the 18 pre-existing ignores leaves exactly that one. **Nothing else
fired**, so the scope decision does not return to the controller on this ground.

**The class-tally instrument was fired, because "zero other classes" is an absence
claim.** A scratch config carrying a bogus id was run through the same grep:

```
$ grep -oE '^(error|warning)\[[a-z-]+\]' firecontrol.log | sort | uniq -c
      1 error[unsound]
      1 warning[advisory-not-detected]
      1 warning[unknown-advisory]
```

The instrument sees warning classes when they exist, so its silence on run 2 is a
measurement rather than a pattern that cannot match.

### Corroborating no-collateral measurement at `-L info`

Pre-state config (variant 3, semantically the pre-Step-4 configuration) against
the shipped state:

| | pre | post |
|---|---|---|
| `note[advisory-ignored]` | 18 | 19 |
| distinct RUSTSEC ids on `ID:` lines | 18 | 19 |
| stats line | `advisories ok: 0 errors, 0 warnings, 36 notes` | `advisories ok: 0 errors, 0 warnings, 38 notes` |
| other `error`/`warning` classes | 0 | 0 |

```
$ comm -13 ids-pre.txt ids-post.txt
ID: RUSTSEC-2024-0429

$ diff cls-pre.txt cls-post.txt
1c1
<      18 note[advisory-ignored]
---
>      19 note[advisory-ignored]
2a3
>       1 note[unsound]
```

The id-set difference is exactly `{RUSTSEC-2024-0429}` and the note-class
difference is exactly one `advisory-ignored` plus one `unsound`, which accounts
for both added notes and leaves nothing unexplained. Reproduces the authoring
measurement in every figure.

The full `cargo deny check` (all four checks, not only advisories) was also run
under the base config and the shipped config for comparison:

```
base config:    advisories ok, bans ok, licenses ok, sources ok   |  32 warning[duplicate]   | EXIT 0
shipped config: advisories ok, bans ok, licenses ok, sources ok   |  32 warning[duplicate]   | EXIT 0
```

Identical, so the change adds no collateral to the licence, bans or sources checks
either. The 32 `warning[duplicate]` are pre-existing, produced by
`bans.multiple-versions = "warn"`.

---

## Step 7: the untouched things, proven untouched, instruments fired (W1-m)

```
$ git diff --exit-code -- BUILDING.md .github/workflows/ci.yml Cargo.lock package.json crates/muxsmith-core/src/profile/model.rs
(no output)
EXIT: 0
```

Per file against the base commit's blobs, `5378264f35616adcb2356e297301b5fa3b8b5719`:

```
MATCH     BUILDING.md                                worktree=911fdba996fcafecbd736f92c1820ec0078d8144  base=911fdba996fcafecbd736f92c1820ec0078d8144
MATCH     .github/workflows/ci.yml                   worktree=278bc545d5d813973b35cf5cc34b22f3e3dabbc6  base=278bc545d5d813973b35cf5cc34b22f3e3dabbc6
MATCH     Cargo.lock                                 worktree=02a80150ff252a95c0d56542934052b885ce43e0  base=02a80150ff252a95c0d56542934052b885ce43e0
MATCH     package.json                               worktree=d7e98e95caa83e2386ef5c455eb7f7c4f2d586e9  base=d7e98e95caa83e2386ef5c455eb7f7c4f2d586e9
MATCH     crates/muxsmith-core/src/profile/model.rs  worktree=1fee01aa98170af4cac2cb731517e4e1194143f4  base=1fee01aa98170af4cac2cb731517e4e1194143f4
MISMATCH  pnpm-lock.yaml                             worktree=22f63701cd396ee9aee0f7db1eff36b1810be4b1  base=69b318512b1a0fb257603b6a49128216b7cf1efa
MISMATCH  deny.toml                                  worktree=119012e43b1b998a9572af1b07bdf11664c7f601  base=ab8bc5df2b475916901552a58c284eb05f76072f
```

Both instruments fired on the two files that DID move:

```
$ git diff --exit-code --stat -- pnpm-lock.yaml   ->  EXIT 1
$ git diff --exit-code --stat -- deny.toml        ->  EXIT 1
```

So the five MATCH lines and the exit-0 are discriminating results, not what a
misspelled path or a wrong base revision would also produce.

`Cargo.lock` matching its base blob is also the evidence for Step 8's "no upgrade
attempted": no `[patch]` section, no Tauri version change, no `cargo update`.

### W1-k: `deny.toml`'s diff is bounded

```
$ git diff -U0 -- deny.toml
diff --git a/deny.toml b/deny.toml
index ab8bc5d..119012e 100644
--- a/deny.toml
+++ b/deny.toml
@@ -6,0 +7,8 @@ yanked = "deny"
+# cargo-deny's `unsound` scope defaults to `workspace`, which excludes every
+# external crate - so a transitive unsound advisory produced no error, no
+# warning and not even an ignored note, while `unmaintained` (default `all`)
+# reported its 18. `all` rather than `transitive`: it keeps one scope posture
+# for both informational classes, and `transitive` would exempt first-party
+# unsoundness, which is the case we would most want to hear about. Both
+# values behave identically on today's tree.
+unsound = "all"
@@ -44,0 +53,9 @@ ignore = [
+    # glib: unsoundness in the Iterator/DoubleEndedIterator impls for
+    # VariantStrIter (RustSec informational = "unsound"; its GHSA alias is the
+    # GitHub advisory, so both feeds see one advisory). Vulnerable
+    # >= 0.15.0 < 0.20.0, patched at 0.20.0; reached transitively through the
+    # gtk-rs 0.18 generation under Tauri's GTK stack, whose whole family would
+    # have to move together - an upgrade project in someone else's tree rather
+    # than a bump. Ignored as the ruled interim disposition; addressing it
+    # properly is a v1.x item (ROADMAP).
+    "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness

$ git diff -U0 -- deny.toml | grep -cE '^-[^-]'
0
```

Two hunks, both at the two named regions, **zero deletion lines**. No existing
ignore id reworded, reordered or removed; no other key touched.

---

## Step 8: part (c), the `glib` investigation

### Direct parents

```
$ cargo tree -i glib@0.18.5 -e normal --depth 1
glib v0.18.5
├── atk v0.18.2
├── cairo-rs v0.18.5
├── gdk v0.18.2
├── gdk-pixbuf v0.18.5
├── gdkx11 v0.18.2
├── gio v0.18.4
├── gtk v0.18.2
├── javascriptcore-rs v1.1.2
├── pango v0.18.3
├── soup3 v0.5.0
└── webkit2gtk v2.0.2

$ grep -c '^name = "glib"' Cargo.lock
1
```

**Eleven direct parents on normal edges**, counted from that list. One `glib` in
the lock, so there is no second resolution hiding a different version.

### Full reverse tree

```
$ cargo tree -i glib@0.18.5 -e normal
glib v0.18.5
├── atk v0.18.2
│   └── gtk v0.18.2
│       ├── muda v0.19.3
│       │   └── tauri v2.11.5
│       │       ├── muxsmith-gui v0.1.0 (/home/senol/Git/muxsmith-plan11-b/src-tauri)
│       │       ├── tauri-plugin-clipboard-manager v2.3.2
│       │       │   └── muxsmith-gui v0.1.0 (/home/senol/Git/muxsmith-plan11-b/src-tauri)
│       │       ├── tauri-plugin-dialog v2.7.1
│       │       │   └── muxsmith-gui v0.1.0 (/home/senol/Git/muxsmith-plan11-b/src-tauri)
│       │       ├── tauri-plugin-fs v2.5.1
│       │       │   ├── muxsmith-gui v0.1.0 (/home/senol/Git/muxsmith-plan11-b/src-tauri)
│       │       │   └── tauri-plugin-dialog v2.7.1 (*)
│       │       └── tauri-plugin-os v2.3.2
│       │           └── muxsmith-gui v0.1.0 (/home/senol/Git/muxsmith-plan11-b/src-tauri)
│       ├── tao v0.35.3
│       │   └── tauri-runtime-wry v2.11.4
│       │       └── tauri v2.11.5 (*)
│       ├── tauri v2.11.5 (*)
│       ├── tauri-runtime v2.11.3
│       │   ├── tauri v2.11.5 (*)
│       │   └── tauri-runtime-wry v2.11.4 (*)
│       ├── tauri-runtime-wry v2.11.4 (*)
│       ├── webkit2gtk v2.0.2
│       │   ├── tauri v2.11.5 (*)
│       │   ├── tauri-runtime v2.11.3 (*)
│       │   ├── tauri-runtime-wry v2.11.4 (*)
│       │   └── wry v0.55.1
│       │       └── tauri-runtime-wry v2.11.4 (*)
│       └── wry v0.55.1 (*)
├── cairo-rs v0.18.5
│   ├── gdk v0.18.2
│   │   ├── gdkx11 v0.18.2
│   │   │   └── wry v0.55.1 (*)
│   │   ├── gtk v0.18.2 (*)
│   │   └── webkit2gtk v2.0.2 (*)
│   ├── gtk v0.18.2 (*)
│   └── webkit2gtk v2.0.2 (*)
├── gdk v0.18.2 (*)
├── gdk-pixbuf v0.18.5
│   ├── gdk v0.18.2 (*)
│   └── gtk v0.18.2 (*)
├── gdkx11 v0.18.2 (*)
├── gio v0.18.4
│   ├── gdk v0.18.2 (*)
│   ├── gdk-pixbuf v0.18.5 (*)
│   ├── gdkx11 v0.18.2 (*)
│   ├── gtk v0.18.2 (*)
│   ├── pango v0.18.3
│   │   ├── gdk v0.18.2 (*)
│   │   └── gtk v0.18.2 (*)
│   ├── soup3 v0.5.0
│   │   ├── webkit2gtk v2.0.2 (*)
│   │   └── wry v0.55.1 (*)
│   └── webkit2gtk v2.0.2 (*)
├── gtk v0.18.2 (*)
├── javascriptcore-rs v1.1.2
│   ├── webkit2gtk v2.0.2 (*)
│   └── wry v0.55.1 (*)
├── pango v0.18.3 (*)
├── soup3 v0.5.0 (*)
└── webkit2gtk v2.0.2 (*)
```

The tree bottoms out at `tauri 2.11.5` through `muda`, `tao`, `tauri-runtime`,
`tauri-runtime-wry`, `webkit2gtk` and `wry`, and from there at `muxsmith-gui`
directly and through its four Tauri plugins. Reproduces the authoring measurement.

### The tally, defined as the step defines it

Every crate in the `--depth 1` parent set plus `glib` itself, versions read from
`Cargo.lock` rather than from the tree output:

```
members counted: 12
  glib                   0.18.5
  atk                    0.18.2
  cairo-rs               0.18.5
  gdk                    0.18.2
  gdk-pixbuf             0.18.5
  gdkx11                 0.18.2
  gio                    0.18.4
  gtk                    0.18.2
  javascriptcore-rs      1.1.2
  pango                  0.18.3
  soup3                  0.5.0
  webkit2gtk             2.0.2
```

Nine of the twelve are gtk-rs `0.18.x`. The three that are not
(`javascriptcore-rs 1.1.2`, `soup3 0.5.0`, `webkit2gtk 2.0.2`) never used gtk-rs
versioning; they are the same generation's bindings. Every version of every
gtk-rs-versioned crate and `-sys` sibling in the whole lock:

```
$ grep -A1 '^name = "\(glib\|gio\|gtk\|gdk\|gdk-pixbuf\|gdkx11\|atk\|pango\|cairo-rs\|glib-macros\|glib-sys\|gio-sys\|gtk-sys\|gdk-sys\|atk-sys\|pango-sys\|cairo-sys-rs\|gdk-pixbuf-sys\|gdkx11-sys\|gdkwayland-sys\|gtk3-macros\)"' Cargo.lock | grep version | sort -u
0.18.0  0.18.1  0.18.2  0.18.3  0.18.4  0.18.5
```

**Nothing 0.20-or-newer exists anywhere in the lock.**

### The finding, which IS the result

Moving `glib` to the patched 0.20.0 means moving the whole gtk-rs 0.18 generation
that surrounds it, and that generation is pulled in by `tao`, `wry`, `muda` and
`webkit2gtk`, which is Tauri 2's own Linux backend. **It is an upgrade project in
someone else's tree, not a bump, and not Muxsmith's to drive.** That is the
acceptable and expected completion.

**No upgrade was attempted.** `Cargo.lock` matches its base blob (Step 7), no
`[patch]` section was added, and no Tauri version changed.

**One borrowed claim, attributed as such:** that Tauri 2's tao/wry have not
migrated off GTK3 comes from `deny.toml`'s own existing comment
(`gtk-rs GTK3 bindings (gtk-rs/gtk3-rs): archived upstream in favor of gtk4-rs;
Tauri 2's Linux backend (tao/wry) has not migrated off GTK3`), not from a fresh
measurement of Tauri's upstream repository. What I did measure is consistent with
it: every gtk-rs crate in this lock is 0.18.x and the reverse tree runs through
tao/wry/webkit2gtk.

---

## Step 9: verification

### The full gate as `BUILDING.md` enumerates it - 11 parts, foreground, in this worktree

`BUILDING.md` states 11 parts: 6 Rust, 4 frontend, 1 house-knowledge, enumerated
by its three marked gate blocks. All eleven, in that order:

| # | command | exit |
|---|---|---|
| 1 | `cargo fmt --all --check` | 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| 3 | `cargo test --workspace` | 0 |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 |
| 5 | `cargo deny check` | 0 |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 |
| 7 | `pnpm lint` | 0 |
| 8 | `pnpm build` | 0 |
| 9 | `pnpm check:i18n` | 0 |
| 10 | `pnpm test:e2e` | 0 |
| 11 | `python3 scripts/ledger-lint.py` | 0 |

Prerequisites confirmed present rather than assumed: `rustc 1.96.1` (the pin),
`x86_64-pc-windows-msvc` in `rustup target list --installed`, PyYAML 6.0.3.

Pasted tails:

```
$ cargo fmt --all --check
(no output)                                                        EXIT: 0

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 21.70s   EXIT: 0

$ cargo test --workspace                                           EXIT: 0
$ cargo test --workspace 2>&1 | grep -E '^test result' | sed -E 's/;.*finished in.*//' | sort | uniq -c | sort -rn
      9 test result: ok. 0 passed
      4 test result: ok. 7 passed
      3 test result: ok. 4 passed
      3 test result: ok. 2 passed
      3 test result: ok. 1 passed
      2 test result: ok. 8 passed
      2 test result: ok. 3 passed
      2 test result: ok. 15 passed
      2 test result: ok. 13 passed
      1 test result: ok. 80 passed
      1 test result: ok. 70 passed
      1 test result: ok. 5 passed
      1 test result: ok. 37 passed
      1 test result: ok. 24 passed
      1 test result: ok. 19 passed
      1 test result: ok. 122 passed
      1 test result: ok. 11 passed
      1 test result: ok. 10 passed
   -> result lines: 39 | sum passed: 505 | every line "ok", zero "FAILED"

   Interrogated rather than trusted: a grep for `FAILED|panicked` returns 2 hits,
   both of them TEST NAMES that are passing
   (`commands::run::tests::finished_panicked_renders_two_lines_without_na ... ok`
   and `panicked_outcome_persists_its_payload_on_the_job_record ... ok`). The
   load-bearing counts are `grep -cE '^test result: FAILED'` -> 0 and the count of
   `[0-9]+ failed` lines that are not `0 failed` -> 0.

$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
   Generated /home/senol/Git/muxsmith-plan11-b/target/doc/muxsmith_cli/index.html and 5 other files   EXIT: 0

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok                    EXIT: 0
   (32 warning[duplicate], pre-existing, from bans.multiple-versions = "warn")

$ cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.32s   EXIT: 0
   (two "GNU compiler is not supported for this target" lines are build-script
    cargo:warning notes, not clippy diagnostics; the run exits 0 under -D warnings)

$ pnpm lint                 -> EXIT 0
$ pnpm build                -> EXIT 0
$ pnpm check:i18n           -> check-i18n: ok (41 source files scanned, 212 catalog ids, ...)   EXIT 0
$ pnpm test:e2e             -> 68 passed (2.9s)                     EXIT 0
$ python3 scripts/ledger-lint.py
ledger-lint: 560 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold   EXIT: 0
```

Parts 7-10 were run twice: once in Step 3 immediately after the lockfile bump, and
once here in the final state after the `deny.toml` edit. Both green.

### The diff names exactly two files

```
$ git diff --stat
 deny.toml      | 17 +++++++++++++++++
 pnpm-lock.yaml | 18 +++++++++---------
 2 files changed, 26 insertions(+), 9 deletions(-)

$ git diff --name-only | wc -l
2
```

### Test duty, weighed per part

**Part (a)** changes a dependency resolution and no behaviour of Muxsmith's own.
Its user-visible consequence is that the build still works, which the four named
frontend commands in Step 3 and the full gate here assert using the existing
infrastructure. No new scenario is owed and none is deferred.

**Part (b)** does change what a gate part covers, which normally owes a test, and
**its test is the three-way fire in Step 5**, built from cargo-deny's existing
`-c` facility rather than deferred to a later package. That fire is the coverage
assertion: run 2 shows the scope live, run 3 shows the scope rather than the
ignore entry doing the work, run 1 shows the shipped state green. I additionally
fired the class-tally instrument (Step 6) so the "no other class" half is a
measurement too.

**The real gap, named rather than hidden:** nothing permanently guards that
`unsound = "all"` stays in `deny.toml`. A future edit could drop the key and every
gate part would stay green, because the loss of coverage is silent by
construction - which is exactly the failure mode this task just repaired. Building
that guard means new gate infrastructure (a lint over a config file with no test
harness), which is outside this task's Files list. It is routed in the plan's
deferred-by-decision note and in the plan close's ROADMAP dispositions, and I
re-surface it in Step 11 below.

> **ERRATUM, B1 fix-round implementer, 2026-07-30, on the controller's ruling
> that this report is a dated record to annotate rather than rewrite.** Both
> halves of the paragraph above are measurably false, refuted at review (B1
> verdict finding 3, adjudication 5) and carried into the plan at Amendment 5.
> **The loss is NOT silent by construction:** with the `unsound` key dropped and
> the ignore entry left in place, cargo-deny emits
> `warning[advisory-not-detected]` naming the exact ignore line and exits 0 - so
> it is gate-green but not silent, and its `unused_ignored_advisory` default is
> `LintLevel::Warn` in `src/advisories/cfg.rs`. **And the guard is NOT new gate
> infrastructure:** `unused-ignored-advisory = "deny"` is one key in the same
> `[advisories]` table this task already edits, which turns that same state into
> `error[advisory-not-detected]`, `advisories FAILED`, exit 1, with the shipped
> state still green - no lint, no new gate part, no new file, no new dependency.
> The gap is therefore an owner DECISION rather than a deferrable infrastructure
> cost, and it is parked as one with its measurement attached; the knob is
> untargeted and also reddens the gate when an ignored advisory legitimately
> disappears upstream. The sentences above stay legible as what was claimed at
> the time.

**Part (c)** produces a measurement, and a measurement's artifact is its pasted
transcript, above.

---

## Step 10: commit

```bash
git add pnpm-lock.yaml deny.toml
git -c commit.gpgsign=false commit -m "deps: postcss past the patched version through the lockfile; cargo-deny sees the unsound class and ignores the ruled glib advisory" -m "Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>" -- pnpm-lock.yaml deny.toml
```

```
[plan-11-stream-b c422999] deps: postcss past the patched version through the lockfile; cargo-deny sees the unsound class and ignores the ruled glib advisory
 2 files changed, 26 insertions(+), 9 deletions(-)
EXIT: 0
```

**SHA: `c42299936a23d2818d3b7926f2f60f8d9c3901ca`**

SI-4 conformance, verified rather than asserted:

```
$ git log -1 --format='%G? | %GS'
N |                                    # unsigned, as required

$ git log -1 --format=%B | grep -c 'Co-Authored-By\|Claude-Session'
1                                      # exactly one trailer, no Claude-Session line

$ git log -1 --format=%B
deps: postcss past the patched version through the lockfile; cargo-deny sees the unsound class and ignores the ruled glib advisory

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

Trailer model name derived from this dispatch's model parameter (Opus 5), no
context-window suffix. Staged explicitly, pathspec-scoped commit, never
`git add -A`. Not pushed - the controller pushes once, at the plan close.

Working tree clean after the commit (`git status --porcelain` printed nothing).

---

## Step 11: surfaced, not edited

Nothing below was edited by this task. Every item is the controller's.

1. **`docs/ROADMAP.md`'s "TWO OPEN VULNERABILITY ALERTS" entry** in the Pre-1.0
   release gates section. Its three ruled parts discharge against this commit and
   this report: part 1 produced a fix (the lockfile bump), part 2 produced a
   configuration repair plus its measurement, part 3 produced a finding. Its
   disposition is a controller close action. **The entry must not record the
   `glib` alert as resolved** - the advisory is ignored, not fixed.

2. **`docs/ROADMAP.md`'s v1.x entry for `glib` unsoundness RUSTSEC-2024-0429.**
   Its INTERIM half is discharged here (scope on, advisory ignored with its
   reason); its deferred half stays open and its recorded trigger (a dependency PR
   or Tauri release moving the gtk-rs generation past 0.18 in `Cargo.lock`) stays
   unfired - nothing in this lock is 0.20+. The `deny.toml` comment I inserted
   agrees with that entry on every fact I could check against the tree.

3. **The `glib` alert stays OPEN on GitHub and is not dismissed by this task.**
   Dismissing an alert is an owner action, and the advisory is ignored rather than
   fixed, so a dismissal would misstate the position.

4. **Gate part `cargo deny check` now covers a class it did not cover before.**
   A change in what a gate part covers is owner-visible even when the owner
   ordered it, and it belongs in the controller's record.

5. **The unguarded residue.** Nothing checks that `unsound = "all"` stays in
   `deny.toml`. If a later edit drops it, the gate stays green and the coverage
   loss is silent - the same shape as the defect just repaired. Observable trigger
   for a ROADMAP line: *`grep -c '^unsound' deny.toml` returns 0 while
   `RUSTSEC-2024-0429` is still in the ignore list.*

   > **ERRATUM, B1 fix-round implementer, 2026-07-30, on the controller's ruling
   > that this report is a dated record to annotate rather than rewrite.** "The
   > coverage loss is silent" is the same refuted claim as in Step 9's test-duty
   > paragraph, and the full correction is in the erratum there. In short: the
   > loss is gate-GREEN but NOT silent - cargo-deny emits
   > `warning[advisory-not-detected]` naming the exact ignore line - and a hard
   > guard costs one key (`unused-ignored-advisory = "deny"`) in the same
   > `[advisories]` table rather than new infrastructure. The observable trigger
   > this item proposes stands on its own and is unaffected; what falls is the
   > reason given for why nothing catches the drop.

6. **A stale count my change creates, derived rather than recalled.** The
   `deny.toml` ignore list now holds **19** RUSTSEC ids, not 18. Two live
   (non-archived) sites still assert 18 about that list, both in
   `docs/ROADMAP.md`'s "Dependabot/Renovate activation" area:
   - `... Then walk the 18 commented RUSTSEC ignores in `deny.toml` and drop the one[s] ...`
   - `... Two riders (2026-07-11, docs-tree sweep): prune the 18 commented RUSTSEC ignores in deny.toml as Renovate PRs ob[solete] ...`

   A third, `... exactly as the 18 existing ignores work ...` in the v1.x `glib`
   entry, reads as a statement about the pre-change state and is arguably still
   true; the controller decides.

   **Method note, because a line-based grep could not have found these.** Both
   sites straddle a hard wrap, so `git grep` for `18` near `RUSTSEC` returned
   neither. I named that blind spot and ran a second, newline-flattened pass over
   every live tracked doc and config file, excluding
   `docs/process-journal/artifacts/` (archived process artifacts, which the plan's
   Global Constraints say are history and are not rewritten). The flattened
   expression was fired against the plan document as a known-present control and
   returned 9 hits. Historical assertions of 18 exist in
   `docs/process-journal/artifacts/handoffs/2026-07-29-session-28-close.md` and in
   the plan-5 SDD artifacts; those are correctly out of scope.

7. **Findings 1 and 2 below**, both of which contradict the plan's authoring
   section and neither of which I resolved at the keyboard.

   > **ERRATUM, B1 fix-round implementer, 2026-07-30, on the controller's ruling
   > that this report is a dated record to annotate rather than rewrite.** The
   > count in this item is short by one. The review required a THIRD finding (B1
   > verdict finding 4: the plan's Step 5 `git diff --exit-code -- deny.toml`
   > prescription was unperformable at the point it appears), and it is recorded
   > in the erratum under the `## Findings` heading below. Read this item as
   > "Findings 1 and 2, and the third added by erratum".

---

## Findings

> **ERRATUM, B1 fix-round implementer, 2026-07-30, on the controller's ruling
> that this report is a dated record to annotate rather than rewrite.** This list
> is short by one. The review required a THIRD numbered finding alongside the two
> below (B1 verdict finding 4, "Required change: record it as a third plan defect
> alongside findings 1 and 2 of the report, so the prescription is not reused in
> this shape"), and it is stated here rather than inserted into the list, so the
> original two-item list stays legible as what was submitted.
>
> **Finding 3 (added by erratum): the plan's Task B1 Step 5 prescription
> `git diff --exit-code -- deny.toml` was unperformable at the point it
> appears.** Step 5, as B1 was given it, required that command "pasted after the
> variants to prove" the repository's own `deny.toml` was not mutated into a
> variant. By Step 5 the Step-4 edits are applied and uncommitted, so the command
> exits 1 by construction and proves nothing about mutation. This report DID
> refute the
> prescription with evidence and substituted a stronger proof - the bounded `-U0`
> diff of Step 7 showing exactly the two fenced regions with zero deletion lines
> (Step 5 preamble, Step 7 "W1-k: `deny.toml`'s diff is bounded") - but it did
> not list the contradiction among its numbered findings, so the controller had
> no defect to route. The plan's own Amendment 5 has since replaced that
> prescription with a `sha256sum deny.toml` taken after Step 4 and re-taken after
> every variant run, which is performable at that point; this fix round ran it in
> that form.
>
> **Consumer of the count, flagged rather than edited:** Step 11 item 7 above
> reads "Findings 1 and 2 below" and is short by the same one.

### Finding 1: the "twelfth consumer" claim is inverted - `glib-macros` is glib's DEPENDENCY, not its parent

**The plan's authoring section states** (in "`glib`: eleven parents, all one
generation"): "A twelfth consumer exists and is deliberately not in this figure:
`glib-macros 0.18.5` reaches glib over a proc-macro edge that `-e normal` excludes
by design, so it appears in cargo-deny's inclusion graph and not in this command's
output."

**Measured: false in both halves.** `glib-macros` does not consume `glib` at all,
under any edge filter, and it does not appear in cargo-deny's inclusion graph
either. The dependency runs the other way round - `glib` depends on `glib-macros`.

From `Cargo.lock`, `glib`'s own dependency list:

```
[[package]]
name = "glib"
version = "0.18.5"
dependencies = [
 "bitflags 2.13.0", "futures-channel", "futures-core", "futures-executor",
 "futures-task", "futures-util", "gio-sys", "glib-macros", "glib-sys",
 "gobject-sys", "libc", "memchr", "once_cell", "smallvec", "thiserror 1.0.69",
]
```

and `glib-macros`' own, which contains no `glib`:

```
[[package]]
name = "glib-macros"
version = "0.18.5"
dependencies = [
 "heck 0.4.1", "proc-macro-crate 2.0.2", "proc-macro-error", "proc-macro2",
 "quote", "syn 2.0.118",
]
```

Both directions probed rather than argued:

```
$ for e in normal build dev all; do cargo tree -i glib@0.18.5 -e $e --depth 1 | grep -c 'glib-macros'; done
normal   -> 0
build    -> 0
dev      -> 0
all      -> 0

$ cargo tree -i glib@0.18.5 -e normal | grep -c 'glib-macros'
0

$ cargo deny check advisories -c <variant2> | grep -c 'glib-macros'     # cargo-deny's inclusion graph
0
```

**Consequence: none for any decision, and none for the `deny.toml` comment**, which
does not mention `glib-macros`. The figure the plan actually acts on - eleven
direct parents on normal edges - is correct and reproduced. If anything the
finding strengthens part (c)'s conclusion: there is no twelfth consumer, so the
parent set is eleven full stop, and "eleven parents **on normal edges**" needs its
qualifier for a different reason than the plan gives (the qualifier is still
right, because `-e normal` is the command that was run, but the excluded
proc-macro edge it names does not exist here).

Likely origin, offered as a hypothesis and not as a measurement: `deny.toml`'s own
existing comment says `proc-macro-error` is "reached as glib-macros' proc-macro
dependency", i.e. `glib -> glib-macros -> proc-macro-error`. The direction appears
to have been read backwards somewhere during plan authoring.

**Not resolved at the keyboard.** The plan's authoring section is the controller's
document; this task edits neither it nor the ROADMAP.

### Finding 2: "`unmaintained` (default `all`) reported its 18" is 16 unmaintained plus 2 vulnerability

**The plan's authoring section states**: "18 transitive *unmaintained* advisories
produce notes because `unmaintained` defaults to `All`, while one transitive
*unsound* advisory produces nothing because `unsound` defaults to `Workspace`."
The **fenced `deny.toml` comment** carries the same claim in its own words:
"... while `unmaintained` (default `all`) reported its 18."

**Measured on the pre-Step-4 configuration at `-L info`:**

```
$ grep -oE '^note\[[a-z-]+\]' info-pre.log | sort | uniq -c
     18 note[advisory-ignored]
     16 note[unmaintained]
      2 note[vulnerability]
```

18 notes total is the stats line's arithmetic (18 + 16 + 2 = 36, and the run
prints `36 notes`): each ignored advisory emits one class note plus one
`advisory-ignored` note. So of the 18 pre-existing ignores, **16 arrive through
the `unmaintained` class and 2 through the `vulnerability` class**. The two:

```
note[vulnerability]: Quadratic run time when checking a start tag for duplicate attribute names
    ├ ID: RUSTSEC-2026-0194
note[vulnerability]: Unbounded namespace-declaration allocation in `NsReader` enables memory-exhaustion denial of service
    ├ ID: RUSTSEC-2026-0195
```

Both quick-xml, both real vulnerabilities rather than informational advisories -
which the plan-5 task-4 review verdict already recorded when the ignores were
added ("`RUSTSEC-2026-0194/0195` = quick-xml DoS, confirmed real advisories").

**Why this does not change the mechanism, and in fact sharpens it.** The
vulnerability class has **no scope key at all** in cargo-deny 0.19.9 (its config
struct's full field list is in Step 4 above: scope keys exist only for
`unmaintained` and `unsound`). A vulnerability therefore reaches transitive crates
unconditionally. So the pre-state evidence is a three-way contrast rather than a
two-way one:

| class | scope | reached transitive `glib`? |
|---|---|---|
| vulnerability | no scope key exists | yes, always - the 2 quick-xml ids |
| unmaintained | `Scope::All` by default | yes - the 16 gtk-rs/unic/proc-macro-error ids |
| unsound | `Scope::Workspace` by default | **no** - which is why RUSTSEC-2024-0429 was silent |

The decision, the scope value, the blast radius and the disposition are all
unaffected.

**What is affected:** the fenced comment now shipped in `deny.toml` attributes all
18 to `unmaintained`, which is off by two. **I applied the fence verbatim as the
plan requires** ("Must not decide: the two fenced `deny.toml` insertions") and did
not adjust it at the keyboard. The controller decides whether the sentence is
worth a follow-up edit; the accurate form would be "while `unmaintained` (default
`all`) reported its 16, and the scope-less vulnerability class its 2".

> **ERRATUM, B1 fix-round implementer, 2026-07-30, on the controller's ruling
> that this report is a dated record to annotate rather than rewrite.** This
> paragraph is wrong in two independent ways, and only the first is a matter of
> time passing.
>
> **1. The present tense is false as of commit `5bf65dc`.** "The fenced comment
> now shipped in `deny.toml`" described the state at `c422999`. Amendment 5
> replaced that fence, and this fix round applied the replacement in `5bf65dc`.
> The sentence it describes no longer exists in the file: `excludes` and
> `reported its` both occur zero times in `deny.toml` at `5bf65dc`. As a dated
> statement about `c422999` the paragraph is accurate and stays legible as one.
>
> **2. The accurate form it proposes was superseded by a different DECISION, not
> by a different number.** This paragraph offers "while `unmaintained` (default
> `all`) reported its 16, and the scope-less vulnerability class its 2". That is
> not what shipped. Amendment 5 DROPPED the count rather than correcting it to
> 16, on the reviewer's explicit ground (`proc-normative-count-recomputed`,
> trigger 2) that a count over the ignore list goes stale on the next ignore
> addition, in a file whose whole purpose is to accumulate such entries. Anyone
> reading this paragraph as a to-do would ship a corrected number into exactly
> the position the review ruled a number must not occupy.
>
> **The reviewer's ground is not hypothetical here; this task itself is the
> counter-example.** The number 18 was already stale in the commit that shipped
> it. It was measured on the PRE-B1 file, while `c422999` - the same commit
> carrying the comment - added the `glib` entry and left the ignore list at 19
> ids. Measured on the shipped state of this fix round, whose ignore list is
> identical to `c422999`'s in content and order:
>
> ```
> $ cargo deny -L info check advisories | grep -oE '^note\[[a-z-]+\]' | sort | uniq -c
>      19 note[advisory-ignored]
>      16 note[unmaintained]
>       1 note[unsound]
>       2 note[vulnerability]
> ```
>
> So the gap this paragraph calls "off by two" (18 against 16) was already three
> (19 against 16) the moment the comment landed, and the class breakdown the
> paragraph identifies - 16 unmaintained plus 2 vulnerability - is otherwise
> confirmed. The `1 note[unsound]` is new and is the point of the whole task: it
> appears only because the scope is now `all`.

---

## Noticed, not touched

- **`pnpm update postcss` printed `[WARN] 1 deprecated subdependencies found:
  glob@10.5.0`.** Pre-existing and unrelated to postcss - `glob` did not move in
  the lockfile diff. Not a Renovate-worthy item yet; recorded so a later reader
  does not mistake it for fallout of this bump.

- **`cargo deny check` emits 32 `warning[duplicate]`** from
  `bans.multiple-versions = "warn"`, identical under the base config and the
  shipped one. Pre-existing and deliberate policy, not fallout.

- **This worktree's cargo target directory is worktree-local**
  (`cargo metadata` reports `/home/senol/Git/muxsmith-plan11-b/target`; the repo's
  `.cargo/config.toml` sets only `TS_RS_EXPORT_DIR`). So the concurrent stream A
  cannot contend with stream B on a shared `target/`, which is worth knowing
  before the merges but needed no action here.

- **`cargo test --workspace` left the tree clean** despite
  `.cargo/config.toml`'s `TS_RS_EXPORT_DIR = src/bindings`: `git status
  --porcelain` after the test run printed only the two intended files. No
  generated binding drifted into the diff.

---

## FIX ROUND (2026-07-30): Amendment 5's replacement fence applied

Fresh implementer, not the author of `c422999`. Brief:
`.superpowers/sdd/plan-11/task-b1-fix-brief.md`. Worktree
`/home/senol/Git/muxsmith-plan11-b`, branch `plan-11-stream-b`, base `c422999`.
Everything below was run in this worktree, foreground, with variant configs at a
scratch path outside the repository.

**Status: COMPLETE. New commit `5bf65dc13199aca190046f2fcb7a863bb72bef39` on top
of `c422999`, one file (`deny.toml`), comment text only.**

### 1. The fence, proven byte-identical across its two sources before it was applied

The replacement wording exists in two places: the amended plan's Step 4(a) on
`master` (Amendment 5) and B1 verdict adjudication 1(b), whose wording it is.
Rather than retype either, both were extracted programmatically and compared. The
fence that was applied is the extracted file, so no transcription step exists.

```
$ sha256sum fence-from-plan.txt fence-from-verdict.txt
eed7ff9276e85a80b42134b3d7f532023653ba15ee92c1a991c30a251655b47a  fence-from-plan.txt
eed7ff9276e85a80b42134b3d7f532023653ba15ee92c1a991c30a251655b47a  fence-from-verdict.txt
$ diff fence-from-plan.txt fence-from-verdict.txt
(no output - byte-identical)

lines: 11   longest line: 76 characters
$ LC_ALL=C grep -nP "[^\x00-\x7F]" fence-from-plan.txt
(no output - pure ASCII, straight quotes, no em-dash)
```

### 2. The operation is a REPLACE, and the anchors were measured before the edit

Amendment 5's fix round 1 (delta-review finding 1) restated Step 4(a) as a
replace, because the file moved under the instruction: at `c422999` the two
anchors are eight lines apart with B1's own fence between them, including its own
`unsound = "all"` key line. Measured on the base blob rather than assumed:

```
$ grep -n '^yanked = "deny"$' deny.toml            # anchor A, occurrences: 1
6:yanked = "deny"
$ grep -n '^# All entries below are transitive' deny.toml   # anchor B, occurrences: 1
15:# All entries below are transitive, from Tauri 2's Linux windowing stack

lines strictly between the anchors: 8   (7 comment lines + unsound = "all")
```

### 3. Postcondition, and the fire test that proves the check can fail

The brief's postcondition is *exactly one `^unsound = ` line in `deny.toml`*. An
assertion whose passing result is a count of 1 looks identical whether the edit
was right or the pattern was wrong, so the check was fired against the reading it
exists to exclude - the literal INSERT built on a scratch copy of the `c422999`
blob, the repository's file untouched.

```
$ grep -c '^unsound = ' literal-insert.toml      # the insert reading
2
$ cargo deny check advisories -c literal-insert.toml
2026-07-30 15:02:47 [ERROR] failed to parse config from 'literal-insert.toml': duplicate key: `unsound`
EXIT: 1

$ grep -c '^unsound = ' deny.toml                 # the end state, the replace reading
1
$ grep -n '^unsound = ' deny.toml
17:unsound = "all"
```

Both directions discriminate (2 against 1, exit 1 against exit 0), so the
postcondition is a check that can fail rather than one that cannot. The insert
reading dies at config parse, before a single advisory is evaluated, which is
what Amendment 5 measured and what this run reproduces.

### 4. The three-way fire, re-run against the NEW fence

The fence changed, so the fire is re-run rather than cited from the first round.
Variant configs at a scratch path outside the repository, driven with
`cargo deny check advisories -c <path>`; the repository's own `deny.toml` was
never mutated to produce one. `cargo-deny 0.19.9`.

The mutation-proof instrument is Amendment 5's replacement for the unperformable
`git diff --exit-code`: a `sha256sum deny.toml` taken immediately after Step 4
and re-taken after every variant run.

```
sha256 deny.toml, immediately after the edit and before any run:
3ea3702a3d2b19ec2accd39f54edad58066dcfaedc28f27c4e9805c7c44718b2  deny.toml

--- RUN 1: shipped state (the repository deny.toml) ---
$ cargo deny check advisories
advisories ok
EXIT: 0
sha256 after run 1: 3ea3702a3d2b19ec2accd39f54edad58066dcfaedc28f27c4e9805c7c44718b2

--- RUN 2: the scope is live (ignore entry + its comment removed, scope on) ---
$ cargo deny check advisories -c v2-scope-live.toml
error[unsound]: Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
    ├ ID: RUSTSEC-2024-0429
advisories FAILED
EXIT: 1
sha256 after run 2: 3ea3702a3d2b19ec2accd39f54edad58066dcfaedc28f27c4e9805c7c44718b2

--- RUN 3: the control (BOTH the scope key and the ignore entry removed) ---
$ cargo deny check advisories -c v3-control.toml
advisories ok
EXIT: 0
sha256 after run 3: 3ea3702a3d2b19ec2accd39f54edad58066dcfaedc28f27c4e9805c7c44718b2
```

All three match the plan's prescription exactly: green shipped, red with the
ignore entry gone and the scope on, green again with the scope gone too. The
control is what separates "the ignore entry is load-bearing" from "the scope was
never on". The sha256 is identical at all four points, so no variant was produced
by mutating the repository's file.

**Blast radius, and the class tally that actually proves it.** "Nothing else
fired" cannot be read off a grep that filters for the class one expects, so run 2
was re-run unfiltered and every emitted class counted:

```
$ cargo deny check advisories -c v2-scope-live.toml | grep -oE '^(error|warning|note)\[[a-z-]+\]' | sort | uniq -c
      1 error[unsound]
$ cargo deny check advisories -c v2-scope-live.toml | grep -oE 'ID: RUSTSEC-[0-9-]+' | sort -u
ID: RUSTSEC-2024-0429
```

One class, one id. The fired set is the single-member set `{RUSTSEC-2024-0429}`.

**The same instrument corroborates the new comment's account of the three
classes**, on the shipped state at `-L info`:

```
$ cargo deny -L info check advisories | grep -oE '^note\[[a-z-]+\]' | sort | uniq -c
     19 note[advisory-ignored]
     16 note[unmaintained]
      1 note[unsound]
      2 note[vulnerability]
```

16 + 1 + 2 = 19, and 19 is the ignore-ENTRY count - which is precisely the
conflation the old comment made when it attributed all of them to `unmaintained`
(as 18, the pre-B1 entry count). The unsound note is now present at all because
the scope is `all`; under the default `workspace` it was absent, which is the
asymmetry the new comment describes. Note that the entry count has already moved
from 18 to 19 since the old comment was written, which is the reviewer's stated
reason for dropping the number rather than correcting it to 16.

### 5. The bounded diff over the new commit alone, and the whole-file accounting

```
$ git diff -U0 c422999 5bf65dc -- deny.toml
diff --git a/deny.toml b/deny.toml
index 119012e..27f3679 100644
--- a/deny.toml
+++ b/deny.toml
@@ -7,5 +7,8 @@ yanked = "deny"
-# cargo-deny's `unsound` scope defaults to `workspace`, which excludes every
-# external crate - so a transitive unsound advisory produced no error, no
-# warning and not even an ignored note, while `unmaintained` (default `all`)
-# reported its 18. `all` rather than `transitive`: it keeps one scope posture
-# for both informational classes, and `transitive` would exempt first-party
+# cargo-deny scopes only the two informational classes, and their defaults
+# differ: `unmaintained` defaults to `all`, `unsound` to `workspace`. A
+# `workspace` scope reaches only crates a workspace member depends on
+# directly, and glib sits deeper, so its unsound advisory produced no error,
+# no warning and not even an ignored note, while the unmaintained advisories
+# beside it all reported. (A real vulnerability has no scope key at all and
+# always fires.) `all` rather than `transitive`: it keeps one scope posture
+# for both scoped classes, and `transitive` would exempt first-party

$ git diff --stat c422999 5bf65dc
 deny.toml | 13 ++++++++-----
 1 file changed, 8 insertions(+), 5 deletions(-)

$ git diff --name-only c422999 5bf65dc
deny.toml
```

**Every changed line accounted for.** The whole-file comparison of `deny.toml` at
`c422999` against the end state shows one contiguous region: five old comment
lines out, eight new comment lines in. The old fence's last two comment lines
(`# unsoundness, which is the case we would most want to hear about. Both` /
`# values behave identically on today's tree.`) and its `unsound = "all"` line are
byte-identical in the new fence, which is why the diff does not touch them. That
is the whole change: 8 insertions, 5 deletions, one file, comment text only.

Invariants held across the commit, measured rather than eyeballed:

```
ignore ids at c422999: 19   end state: 19
ignore id list, content AND order, c422999 vs end state:
  IDENTICAL - no id reworded, reordered or removed
unsound line, c422999: unsound = "all"
unsound line, end state: unsound = "all"   (unchanged in value)
non-ASCII bytes in the end state: none
longest line in the end state: 78 characters (pre-existing, from fence B; the new fence's own longest is 76)
```

**The two falsehoods are gone, checked with terms derived from the `c422999` text
rather than from memory of it.** `excludes` occurred once at `c422999` (line 7)
and occurs zero times in the end state. `reported its` likewise. The token `18`
survives only as `# gtk-rs 0.18 generation ...` inside fence B's glib comment, a
version reference and not the count claim.

### 6. Commit, SI-4 conformance verified rather than asserted

```
$ git add deny.toml && git -c commit.gpgsign=false commit -F <msg> -- deny.toml
[plan-11-stream-b 5bf65dc] deny: say what the unsound scope actually tests, and drop the conflated count
 1 file changed, 8 insertions(+), 5 deletions(-)

SHA: 5bf65dc13199aca190046f2fcb7a863bb72bef39
parent: c42299936a23d2818d3b7926f2f60f8d9c3901ca   # c422999 - a NEW commit, no amend, no history rewrite

$ git log -1 --format=%G?
N
  # unsigned, as SI-4 requires
$ git log -1 --format=%B | grep -c "Co-Authored-By"
1
$ git log -1 --format=%B | grep -c "Claude-Session"
0   # no Claude-Session line, no context-window suffix
$ git status --porcelain
(no output - working tree clean)
```

Staged explicitly, pathspec-scoped commit, never `git add -A`. Not pushed. SI-4
was verified in the repository's own doctrine (`docs/process-journal.md`, the
session-close handoffs' "SI-4. Git commits and pushes are STANDING-authorized for
this repo") rather than taken from the brief's assertion of it.

The full eleven-part gate was NOT run: the brief assigns it to the controller on
the merged state.

### 7. The guard-premise measurements, re-run for the errata rather than borrowed

The erratum blocks added earlier in this file assert four things the B1 verdict
measured. Because an erratum that repeats a borrowed claim is still a borrowed
claim, all four were re-run here rather than cited: the not-silent warning at
defaults, the one-key hard guard, the reachable green state under that guard, and
the source-level default the first of them rests on.

```
--- unsound key dropped, ignore entry KEPT, defaults (what a revert or a stale copy produces) ---
warning[advisory-not-detected]: advisory was not encountered
63 │     "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness
   │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria
advisories ok
EXIT: 0
   -> gate-GREEN, but NOT silent: it names the exact ignore line.

--- same config + unused-ignored-advisory = "deny" ---
error[advisory-not-detected]: advisory was not encountered
64 │     "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness
advisories FAILED
EXIT: 1
   -> one key in the same [advisories] table turns it into a hard failure.

--- shipped state + unused-ignored-advisory = "deny" (is the green state reachable today?) ---
advisories ok
EXIT: 0
   -> yes, green today.

--- the default the first run rests on, read at the source ---
$ grep -n unused_ignored_advisory .../cargo-deny-0.19.9/src/advisories/cfg.rs
97:    pub unused_ignored_advisory: LintLevel,
115:            unused_ignored_advisory: LintLevel::Warn,
417:    pub unused_ignored_advisory: LintLevel,

sha256 deny.toml after all four: 3ea3702a3d2b19ec2accd39f54edad58066dcfaedc28f27c4e9805c7c44718b2   (unchanged)
```

### 8. Errata added to this report, on the controller's annotate-do-not-rewrite ruling

Three dated erratum blocks were added above, in the form the controller used on
Task A1's brief. No original sentence was altered, deleted or reflowed; each block
sits directly under the text it corrects and the claim it corrects stays legible.

1. **Step 9, "Test duty, weighed per part"** - the "loss of coverage is silent by
   construction" sentence and the "building that guard means new gate
   infrastructure" sentence. Both refuted, with the three runs of section 7 as the
   evidence.
2. **Step 11, item 5 ("The unguarded residue")** - the same refuted claim in its
   second occurrence, cross-referenced to the block above rather than repeated in
   full. Its proposed observable trigger is unaffected and says so.
3. **The `## Findings` heading** - the list is short by one. The third finding
   (B1 verdict finding 4: the plan's Step 5 `git diff --exit-code -- deny.toml`
   prescription was unperformable at the point it appears) is stated in the
   erratum rather than inserted into the list, so the submitted two-item list
   stays legible.

### 9. Concerns

1. **RAISED AND RULED - now annotated.** Step 11 item 7 reads "Findings 1 and 2
   below", and the findings list has grown to three by erratum, so that phrase is
   short by one. I raised it rather than annotating it, because the ruling had
   named three sites and a fourth looked like a scope decision. **The controller
   ruled to annotate**, stating the principle behind the three sites: a claim in
   this report that time or a review has falsified gets a dated erratum and the
   original text stays legible. Item 7 now carries one, pointing at the findings
   erratum.
2. **RAISED AND RULED - now annotated, and it was wrong in two ways at once.**
   Finding 2's closing paragraph ("What is affected") says "the fenced comment now
   shipped in `deny.toml` attributes all 18 to `unmaintained`" and proposes an
   accurate form ("reported its 16, and the scope-less vulnerability class its
   2"). Present tense: true of `c422999`, false since `5bf65dc`. Proposed wording:
   superseded by a different DECISION rather than a different number - Amendment 5
   DROPPED the count rather than correcting it to 16, so a reader treating the
   paragraph as a to-do would ship a number into the position the review ruled a
   number must not occupy. Both halves are in the erratum, with this round's own
   `-L info` tally (19 ignored against 16 unmaintained) as the concrete
   demonstration of the reviewer's staleness ground: the figure 18 was already
   stale in the commit that shipped it.
3. **Transcripts of the old fence are deliberately untouched.** Step 7's W1-k
   pasted `git diff -U0` still shows the `c422999` fence text. That is a
   transcript of a run, not a claim, and falsifying it would be exactly what the
   annotate-do-not-rewrite ruling forbids. Same for the table row inside Finding 2
   whose "silent" refers to the advisory not reporting, a different sense from the
   refuted guard claim, left alone on purpose.
4. **CI/local cargo-deny version skew is unchanged by this commit and carries no
   new exposure.** The local gate runs 0.19.9, the pinned CI action 0.19.8, and
   verdict finding 5 established their `src/advisories/cfg.rs` byte-identical.
   This commit changes comment text only - no key, no value - so nothing here can
   fail asymmetrically between the two.
5. **Plan Step 4(b) needed no work and was verified rather than skipped.**
   `c422999` already applied it; the ignore list holds 19 RUSTSEC ids, which is
   the count the plan states for the post-application state.

### 10. Shell hazards hit in this round, for the house record

Two, both in the same class as the interactive-`cp` hazard the A1 brief already
carries. The exact invocations, because a paraphrase does not let the next agent
recognise the shape.

**(a) A bare `cp` blocked on its interactive alias.** The invocation was
`cp "$SP/deny.toml.new" deny.toml`, which printed `cp: overwrite 'deny.toml'?`
and hung until the tool timed out and killed it. No mutation occurred - the
prompt comes before the write - and `sha256sum deny.toml` confirmed the file was
still at its pre-edit hash afterwards. **Used instead:** a redirect into a
sibling temp file plus `mv -f`:

```
awk '{ print }' "$SP/deny.toml.new" > deny.toml.tmp && mv -f deny.toml.tmp deny.toml
```

**(b) `echo` expanded a backslash escape inside a single-quoted string and wrote
a NUL byte into this report.** The invocation was, generating section 1 of this
fix report:

```
echo '$ LC_ALL=C grep -nP "[^\x00-\x7F]" fence-from-plan.txt' >> "$R"
```

Single quotes stop the SHELL from touching the string, but they do not stop
`echo` itself: this shell's `echo` interprets backslash escapes in its argument,
so the intended literal text `\x00` was written as byte `0x00` and `\x7F` as byte
`0x7F`. **Used instead:** `printf '%s\n' '...'`, which never interprets escapes
in its argument, or a quoted heredoc; the damaged line was repaired in place with
`perl -i -pe` using a single-quoted `q{}` replacement so the backslashes survive.

**The consequence is the reason this is worth recording, and the mechanism is not
the one I first assumed - it was read at the function definition rather than
inferred.** In this harness `grep` is not GNU grep. It is a shell function
(defined in `~/.claude/shell-snapshots/snapshot-zsh-*.sh`, inspected with
`type -f grep`) that execs the Claude Code binary as `ugrep` with a fixed flag
set including **`-I`, "ignore binary files"**. A file containing one NUL is
therefore SKIPPED WHOLE: no output, exit 1. Verified against a two-line scratch
file containing a NUL and a known-present match:

```
grep -n 'ERRATUM' bin-test.txt        ->  (no output)                  exit 1
/usr/bin/grep -n 'ERRATUM' bin-test.txt -> binary file matches         exit 0
grep -an 'ERRATUM' bin-test.txt       ->  1:hello ERRATUM here         exit 0
file bin-test.txt                     ->  data
```

**Plain GNU grep signals the condition; the wrapper converts it into a clean
negative.** Three verification greps run against this report immediately after
the NUL landed - for `ERRATUM`, for `^## FIX ROUND`, and for a string copied out
of the file's own last lines - all returned empty and exited 1. That is
indistinguishable from "the annotations were never written", which is exactly the
shape this project's rule about absence-as-evidence warns about, and here the
false negative was produced by the instrument rather than by the artifact.

What caught it was not the greps. An empty result contradicted a `wc -l` taken
one command earlier, which forced a second instrument: `rg` reported
`binary file matches (found "\0" byte around offset 53218)` and `file` reported
`data` rather than text. **Two practical rules for the next agent:** after
generating any report text through the shell, run `file <report>` once - if it
does not say "text", every grep-based check performed on that file up to that
point is void. And more generally, because `-I` is always in force, a `grep`
sweep in this harness silently omits every binary file it encounters; when a
sweep must be exhaustive over a tree that may hold binary artifacts, use `grep -a`
or `/usr/bin/grep` and say which was used.
