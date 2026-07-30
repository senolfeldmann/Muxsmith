# Task B1 review verdict - Plan 11, stream B (W1)

**Verdict: APPROVED_WITH_MINORS**

**Reviewed:** `/home/senol/Git/muxsmith-plan11-b`, branch `plan-11-stream-b`,
head `c422999`, base `5378264`. Tree verified byte-identical to `c422999` before
and after this review (`git status --porcelain` empty, `git diff --exit-code
HEAD` -> 0, `git diff --exit-code -- deny.toml` -> 0).

**What the verdict means.** The task itself is clean. Every contract term is met
character for character on my own extraction, every prescribed measurement
reproduces on instruments I built myself, the diff is bounded rather than merely
non-empty, and the two plan defects the implementer hit were surfaced with pasted
evidence rather than absorbed at the keyboard. Three of my findings are defects
in the PLAN's fenced content and its deferral reasoning, which the implementer
was forbidden to touch and correctly did not; they need a plan amendment, not a
fix round against this implementer. Two of those ride a shipped configuration
file, so they should land BEFORE this branch merges, while the amendment still
has its natural vehicle.

---

## Findings

### 1. IMPORTANT - the shipped `deny.toml` comment misstates what the `workspace` scope excludes

`deny.toml:7-8` (`# cargo-deny's 'unsound' scope defaults to 'workspace', which
excludes every external crate`).

The default is `workspace` - that half is correct and I verified it at the
`Default` impl. "Excludes every external crate" is false. Measured at
cargo-deny 0.19.9's own `src/advisories.rs`, the `Scope::Workspace |
Scope::Transitive` arm keys on the advisory crate's DIRECT DEPENDENTS, not on
whether the crate itself is first-party:

```rust
Scope::Workspace | Scope::Transitive => {
    let nid = ctx.krates.nid_for_kid(&krate.id).unwrap();
    let dds = ctx.krates.direct_dependents(nid);
    let transitive = scope.value == Scope::Transitive;
    if dds.iter().any(|dd| ws_set.contains(&dd.krate.id) ^ transitive) {
        break 'block;   // emit
    }
    continue 'lup;      // suppress
}
```

So under `workspace` an EXTERNAL crate that a workspace member depends on
directly is in scope and fires. Demonstrated rather than argued, with a scratch
crate of my own whose only dependency is the advisory crate itself:

```
scopeprobe2/Cargo.toml: glib = "=0.18.5"      scopeprobe2/deny.toml: unsound = "workspace"
$ cargo deny check advisories
error[unsound]: Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
   ID: RUSTSEC-2024-0429
advisories FAILED
```

Negative half of the same probe, on the real tree: `unmaintained = "workspace"`
-> 0 `note[unmaintained]`, `"transitive"` -> 16, `"all"` -> 16. Both directions
fire, so neither result is a check that cannot discriminate.

The accurate account is the one the ROADMAP's own v1.x entry already uses
("`glib` arrives transitively through Tauri's GTK stack, so the default scope
excludes it") - the exclusion is about where glib sits in the graph, not about
external crates as a class.

**Required change:** amend the fenced comment; replacement wording in
adjudication 1(b). Not the implementer's defect - the fence is plan-mandated and
"Must not decide" forbids editing it.

### 2. IMPORTANT - the same comment's "reported its 18" conflates two sets

`deny.toml:9-10` (`while 'unmaintained' (default 'all') reported its 18`).

Reproduced independently on the pre-state, driving the base commit's own
`deny.toml` through `-c` rather than a retyped copy:

```
$ cargo deny -L info check advisories -c <base-deny.toml> | grep -oE '^note\[[a-z-]+\]' | sort | uniq -c
     18 note[advisory-ignored]
     16 note[unmaintained]
      2 note[vulnerability]
```

18 is the ignore-entry count; `unmaintained` reported 16. The two
`note[vulnerability]` are `RUSTSEC-2026-0194` and `RUSTSEC-2026-0195`
(quick-xml). The implementer measured this, applied the fence verbatim as the
plan requires, and reported the contradiction - correct handling.

**Required change:** amend the fenced comment; see adjudication 1(b). I
recommend dropping the number rather than correcting it to 16: the count is
illustrative, not load-bearing, and a count over the ignore list goes stale on
the next ignore addition (`proc-normative-count-recomputed`, trigger 2).

### 3. IMPORTANT - the permanent-guard deferral rests on a premise that does not hold

Plan, "Deferred by decision" row *No permanent guard that the `unsound` scope
STAYS on*: "A lint asserting a `deny.toml` key would be new gate infrastructure,
which the tests-belong-to-the-package rule explicitly still allows deferring."
Report Step 9 restates it: "the gate stays green and the coverage loss is silent
- the same shape as the defect just repaired."

Both halves are measurably wrong. Ran the premise (`proc-no-work-needed-check`).

**(a) The loss is not silent at defaults.** Config with the `unsound` key removed
and the ignore entry left in place, which is what a revert or a stale copy
produces:

```
$ cargo deny check advisories -c <rev-v5-key-dropped-ignore-kept.toml>
warning[advisory-not-detected]: advisory was not encountered
53 |     "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness
   |      ^^^^^^^^^^^^^^^^^ no crate matched advisory criteria
advisories ok
EXIT: 0
```

Gate-green, yes. Silent, no: cargo-deny names the exact line. Its
`unused_ignored_advisory` default is `LintLevel::Warn`
(`src/advisories/cfg.rs`).

**(b) The hard guard needs no new infrastructure.** It is one key in the same
`[advisories]` table this task already edits. Both states demonstrated:

```
shipped config + unused-ignored-advisory = "deny"        -> advisories ok       EXIT 0
   same, with the unsound key dropped                    -> error[advisory-not-detected]
                                                            advisories FAILED   EXIT 1
```

So the regression becomes a hard gate failure, with a reachable green state on
today's tree. No lint, no new gate part, no new file, no new dependency - which
is exactly the boundary the house rule draws: new test INFRASTRUCTURE may be
deferred, a scenario the existing infrastructure can already express may not.

**Required change:** the deferral row's reasoning and the report's "silent by
construction" sentence are both falsified and must be corrected. The knob itself
is an owner decision, not an automatic add - see adjudication 5 for the cost.

### 4. MINOR - the plan's Step 5 `git diff --exit-code -- deny.toml` prescription is unperformable as written

Plan Task B1 Step 5: "the repository's own `deny.toml` is not mutated to produce
them, and `git diff --exit-code -- deny.toml` is pasted after the variants to
prove it." At Step 5 the Step-4 edits are applied and uncommitted, so that
command exits 1 by construction and proves nothing about mutation-into-a-variant.

The implementer substituted a stronger proof (the bounded `-U0` diff showing
exactly the two fenced regions, zero deletion lines) and stated the reason in the
report, which is refutation-with-evidence rather than silent absorption. It did
not list the contradiction among its numbered findings, so the controller would
not see it as a defect to route.

**Required change:** record it as a third plan defect alongside findings 1 and 2
of the report, so the prescription is not reused in this shape.

### 5. MINOR - CI runs a different cargo-deny than the local gate, and the task did not check the new key against it

`.github/workflows/ci.yml:167` pins `EmbarkStudios/cargo-deny-action` by SHA
(v2.0.20). That action's Dockerfile at that SHA carries `ENV
deny_version="0.19.8"`, while the local gate part runs cargo-deny 0.19.9. This
task adds a config key whose TYPE changed across cargo-deny releases
(`unmaintained`/`unsound` were `LintLevel` before becoming `Scope`), so an older
CI-side tool would have rejected `unsound = "all"` with a value error and turned
the `deny` job red on the plan close's single push, with the local gate green.

I checked rather than assumed: fetched the 0.19.8 crate and diffed it.
`src/advisories/cfg.rs` is **byte-identical** between 0.19.8 and 0.19.9, and
0.19.8 carries the same `Scope` enum and the same `unsound: Spanned<Scope>` with
`Default` of `Scope::Workspace`. **No change to the diff is required and CI will
not break.**

**Required change:** none to the artifact. Record the version skew, because the
premise is load-bearing and currently unstated anywhere: the local gate and the
CI job run different cargo-deny versions, and a future config key present in only
one of them fails asymmetrically.

### 6. MINOR - the "eleven parents on normal edges" caveat is right for the wrong reason

Covered in full in adjudication 2. No figure and no decision moves; the plan's
stated ground for the qualifier does not exist.

---

## Adjudications

### 1. A wrong count inside a shipped comment

**(a) The mechanism survives the correction and is strengthened by it.** The
mechanism is per-class default scope, and the corrected breakdown makes it a
three-way contrast instead of a two-way one. **The implementer's sharpening is
ACCEPTED, verified at the tool's own source rather than taken on its word.**
`src/advisories.rs` applies the scope filter only after matching the advisory's
`informational` field:

```rust
let Some(scope) = advisory.advisory.informational.as_ref().and_then(|info| match info {
    model::Informational::Unmaintained => Some(&ctx.cfg.unmaintained),
    model::Informational::Unsound      => Some(&ctx.cfg.unsound),
    _ => None,
}) else { break 'block; };   // no informational class -> always emitted
```

A real vulnerability has `informational: None`, falls through the `else`, and is
emitted unconditionally. The config struct has scope keys for exactly those two
classes and no other. So: vulnerability - no scope, always fires (the 2 quick-xml
ids); unmaintained - `Scope::All` by default, fires (16 ids); unsound -
`Scope::Workspace` by default, did not fire. That is a better account than the
one the plan shipped, not a weaker one.

**But the correction changes the account in a second way the implementer did not
name**, and any amendment that fixes only the number ships a comment that is
still false: "which excludes every external crate" is wrong on its own terms
(finding 1). Both defects sit in the same two sentences and must be fixed
together.

**(b) Replacement wording.** Ship this in place of the current fence A. ASCII
only, straight quotes, no line over 76 characters (the file's existing maximum is
78), no count that a later ignore addition can falsify:

```
# cargo-deny scopes only the two informational classes, and their defaults
# differ: `unmaintained` defaults to `all`, `unsound` to `workspace`. A
# `workspace` scope reaches only crates a workspace member depends on
# directly, and glib sits deeper, so its unsound advisory produced no error,
# no warning and not even an ignored note, while the unmaintained advisories
# beside it all reported. (A real vulnerability has no scope key at all and
# always fires.) `all` rather than `transitive`: it keeps one scope posture
# for both scoped classes, and `transitive` would exempt first-party
# unsoundness, which is the case we would most want to hear about. Both
# values behave identically on today's tree.
unsound = "all"
```

Every claim in it is measured above. The last sentence was re-verified rather
than carried over: `unsound = "all"` and `unsound = "transitive"` both exit 1
with the same single id `RUSTSEC-2024-0429` on this tree.

The plan's authoring-section sentence that states the same thing in its own words
("18 transitive *unmaintained* advisories produce notes ... while one transitive
*unsound* advisory produces nothing") needs the same correction; it is a
consumer of the figure, not a separate claim.

### 2. The inverted twelfth consumer

**No figure and no decision moves, and there is no excluded consumer at all.**

The eleven-parent figure reproduces exactly on my run, and the same eleven appear
in cargo-deny's own inclusion graph in run 2. The plan's sentence is wrong in
three independent ways, all measured:

1. **Direction inverted.** `glib` depends on `glib-macros`, not the reverse.
   `cargo tree -p glib@0.18.5 -e normal --depth 1` lists `glib-macros v0.18.5
   (proc-macro)` among glib's own dependencies; `cargo tree -i glib@0.18.5 -e
   {normal,build,dev,all} --depth 1 | grep -c glib-macros` returns 0 in all four.
2. **`-e normal` does not exclude proc-macro edges.** Cargo has a separate
   `no-proc-macro` edge kind for that (`cargo tree --help`: `[possible values:
   all, normal, build, dev, features, public, no-normal, no-build, no-dev,
   no-proc-macro]`). Demonstrated on this very tree: `cargo tree -i
   quick-xml@0.39.4 -e normal --depth 1` prints `wayland-scanner v0.31.10
   (proc-macro)` as the parent. A proc-macro is a crate type, not an edge kind.
3. **There is no consumer on any other edge kind either.** `cargo tree -i
   glib@0.18.5 -e build` and `-e dev` both print "nothing to print"; `-e all`
   adds only feature edges. **Fire control for those empty results:** the same
   `-e build` invocation against `quick-xml@0.39.4` also prints "nothing to
   print" while `-e normal` prints its parent, so the empty result is the
   command's real semantics and not a malformed probe.

**Correct statement of the caveat.** Keep "eleven direct parents on normal edges"
- the unit is still right, because it describes the command that produced the
number - and replace the twelfth-consumer sentence with the measurement:

> `-e normal` is the filter used; it excludes build- and dev-dependency edges,
> not proc-macro edges. On this tree neither filter hides anything: `-e build`
> and `-e dev` return no consumer of `glib` at all, and cargo-deny's own
> inclusion graph shows the same eleven. Eleven is the complete direct-consumer
> set.

The implementer's stated hypothesis for the origin (a misread of `deny.toml`'s
own `glib -> glib-macros -> proc-macro-error` comment) is plausible and correctly
offered as a hypothesis rather than a measurement. Six sites in the plan state
the parent figure; only the twelfth-consumer clause is affected, and the figure
itself stands at every one.

### 3. The unrun alert feed

**The right call, and W1-e is satisfied. No measurement is missing.**

Three grounds, in order of weight:

1. **The plan itself designates the alert side as the controller's measurement.**
   Acceptance row W1-e's `evidence` column reads `authoring`, not `task`, with
   the parenthetical `GHSA-wrw7-89jp-8q8g on both sides`. Its producer clause
   asks for "the advisory file's `aliases` field beside the alert's `ghsa` field,
   both pasted" - which is what the report does, with the borrowed half
   attributed rather than claimed. Requiring a fresh `gh api` run would be
   reading a `task` obligation into a row the plan marks `authoring`.
2. **The gh-log reasoning holds independently.** Every `gh` call against the
   owner's repositories owes a `gh-log.md` entry, and that file lives only in
   `/home/senol/Git/Muxsmith/`, which the dispatch forbade the implementer to
   touch. Refusing to create an unlogged call was the correct resolution of that
   conflict, and it named the conflict instead of quietly skipping the step.
3. **The substitute is sound and I re-verified both sides myself.** RustSec's
   local record carries `aliases = ["GHSA-wrw7-89jp-8q8g"]`, `informational =
   "unsound"`, `patched = [">=0.20.0"]` over `>=0.15.0,<0.20.0` - my own read at
   `~/.cargo/advisory-dbs/advisory-db-3157b0e258782691/crates/glib/RUSTSEC-2024-0429.md`.
   I additionally fetched the postcss advisory GHSA-r28c-9q8g-f849 directly:
   affected `<= 8.5.17`, first patched `8.5.18`, High - which independently
   confirms the `>= 8.5.18` requirement the whole of part (a) rests on.

The one residual is that "Muxsmith's own alert carries that GHSA id" stays a
borrowed claim. The report attributes it as such, and the controller re-ran the
feed at this session's start. That is the correct disposition of a claim whose
only source is a call the implementer was barred from making.

### 4. What the change buys

**All four statements are present and accurate, and no sentence anywhere makes a
resolution claim it is not entitled to make.**

Per statement: (1) gate part and feed now agree about the unsound class -
accurate, and the report gives the mechanism rather than asserting agreement;
(2) the two mechanisms never disagreed - accurate, and grounded at the `Default`
impl rather than at output silence, which is the exact inversion the plan's own
self-review records; (3) ignored, not fixed, alert stays open and undismissed -
present verbatim in Step 6 item 3, in Step 11 item 3, and again in Step 11 item
1's "must not record the `glib` alert as resolved"; (4) blast radius from the
implementer's own measurement - present, and it reproduces on mine.

**Resolution-claim sweep, run over the report, the full diff and the commit
message** with the term set `resolved|resolve[sd]?|fixed|remediat|dismiss|closed
the alert|no longer (vulnerable|affected)|patched now|secure(d)? now|vulnerability
(is )?gone`:

- report: 11 hits, every one either a negation ("not fixed", "is not dismissed",
  "must not record ... as resolved", "not resolved at the keyboard") or pnpm's
  own progress output ("Progress: resolved 265").
- diff: 0 hits. commit message: 0 hits.
- **Fired control**, because two of those three results are absences: the same
  term set against `docs/ROADMAP.md` returns 39 hits, so the expression matches
  this vocabulary when it is present.

The commit subject "cargo-deny sees the unsound class and ignores the ruled glib
advisory" is accurate on both halves and claims nothing more.

### 5. The permanent-guard gap

**The three-way fire is adequate coverage for what the task CHANGES. The deferral
is not adequate, because its premise is refuted.**

The fire is genuinely good: three runs, each discriminating, and it establishes
what a single green run cannot - the scope live (run 2, exit 1), the scope rather
than the ignore doing the work (run 3, exit 0), and the shipped state green (run
1). I rebuilt all three from my own reading of `deny.toml`, at my own paths, and
all three reproduce exactly. That is the coverage assertion for the change, and
it is not deferred.

The gap is the other question: a config key whose loss is gate-green. On the
house rule's own boundary - new INFRASTRUCTURE deferrable, an expressible
scenario not - **this one falls on the non-deferrable side**, because
`unused-ignored-advisory = "deny"` is a key of the same table this task already
edits and it converts the regression into a hard gate failure (finding 3, both
states demonstrated). The deferral row's argument that a guard "would be new gate
infrastructure" is the premise that fails, and the plan's own
`proc-no-work-needed-check` is the rule that says to run it rather than weigh it.

**But it is not an automatic add, and I do not recommend one.** The knob is
untargeted: it fails the gate on ANY unused ignore entry, including the case
where an ignored advisory legitimately disappears because upstream fixed it.
`deny.toml`'s own header comment arguably wants that ("drop an ID once its crate
is gone from Cargo.lock ... instead of leaving it stale"), and the ROADMAP's
v1.x glib trigger describes exactly that event - so the knob would enforce the
convention the file already states. The cost is that a Renovate PR which happens
to obsolete an ignored advisory turns the `deny` job red for a reason the PR
author did not cause, in a repo whose stated cadence already accepts grouped PRs
being hard to take apart (`ci-04-dependabot-cadence`).

**Recommendation:** route it to the owner as a one-key decision with this
measurement attached, in the same amendment that carries findings 1 and 2. It is
his call whether an enforced drop-when-obsolete rule is worth a periodically red
dependency PR. What must NOT survive either way is the current justification: the
row may not keep claiming a guard requires new infrastructure, and the report may
not keep claiming the loss is silent.

Note also that the guard, whichever form it takes, is partial by construction: it
fires only while the ignore entry is present. A future edit removing both the key
and the entry is caught by nothing, which is my run 3 (exit 0, no diagnostic).
That residual is genuinely un-guardable without new infrastructure and IS
correctly deferrable.

---

## Evidence appendix

**Instrument root** (created for this review; nothing here was written by the
implementer and no shared default path was used):
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/b1rev-independent/`

| instrument | purpose |
|---|---|
| `extract_fences.py`, `fence_a.txt`, `fence_b.txt` | extract the plan's two fenced insertions from the plan's own markup |
| `check_placement.py` | byte-for-byte presence + placement, fired against a mutated copy and against the base file |
| `base-deny.toml`, `base-lock.yaml` | `git show 5378264:<path>`, the pre-state |
| `toml_delta.py` | structural TOML delta base vs shipped |
| `lock_delta.py` | YAML-parsed lockfile delta (packages, snapshots, importers, settings) |
| `make_variants.py`, `rev-v2-scope-on-no-ignore.toml`, `rev-v3-no-scope-no-ignore.toml` | my own three-way fire configs |
| `rev-v4-firecontrol.toml` | bogus-id config, fires the class-tally instrument |
| `rev-v5-key-dropped-ignore-kept.toml` | the dropped-key regression at defaults |
| `rev-v6-shipped-plus-uia.toml`, `rev-v7-uia-key-dropped.toml` | the `unused-ignored-advisory = "deny"` guard, green and red states |
| `rev-unmaint-{workspace,transitive,all}.toml`, `rev-unsound-*.toml`, `rev-v2-transitive.toml` | scope-semantics probes |
| `scopeprobe/`, `scopeprobe2/` | throwaway single-crate cargo projects proving an external DIRECT dependency is in scope under `workspace` |
| `cd0198.crate` -> `cargo-deny-0.19.8/` | the CI-side cargo-deny version, fetched and diffed against 0.19.9 |
| `sweep18.py` | newline-flattened repo sweep for stale "18 ignores" assertions |
| `replacement-fence-a.txt` | the adjudication 1(b) wording, width- and glyph-checked |

**Commands run** (all foreground, absolute paths, `--manifest-path` rather than
`cd`; no session-relocation tool was called and `gh` was not invoked):

```
git -C <wt> {status --porcelain, rev-parse HEAD, log -1 --format=..., diff -U0 -- deny.toml,
             diff --exit-code -- <paths>, hash-object <path>, rev-parse 5378264:<path>,
             show 5378264:{deny.toml,pnpm-lock.yaml}, diff --stat 5378264 c422999}
cargo deny --manifest-path <wt>/Cargo.toml check advisories [-c <my config>] [-L info]
cargo deny --manifest-path <wt>/Cargo.toml check [-c <my config>]
cargo tree --manifest-path <wt>/Cargo.toml -i glib@0.18.5 -e {normal,build,dev,all} [--depth 1]
cargo tree --manifest-path <wt>/Cargo.toml -p glib@0.18.5 -e {normal,all,no-proc-macro} --depth 1
cargo tree --manifest-path <wt>/Cargo.toml -i quick-xml@0.39.4 -e {normal,build} --depth 1
cargo generate-lockfile --offline --manifest-path <scopeprobe{,2}>/Cargo.toml
pnpm install --frozen-lockfile ; pnpm lint ; pnpm build ; pnpm check:i18n
python3 scripts/ledger-lint.py
grep -cE '^\s*"RUSTSEC-' deny.toml            # the plan's own expression, run as written
curl -sSL https://static.crates.io/crates/cargo-deny/cargo-deny-0.19.8.crate
WebFetch https://github.com/advisories/GHSA-r28c-9q8g-f849
WebFetch <cargo-deny-action@bb137d7 action.yml and Dockerfile>
```

**Results, by dimension.**

1. **Contract compliance.** Both fences present verbatim (1 occurrence each) and
   at the prescribed placement: fence A immediately after `yanked = "deny"` and
   immediately before `# All entries below are transitive`; fence B immediately
   after the quick-xml `RUSTSEC-2026-0195` line and immediately before the list
   close. Fired both ways: a one-character mutation and the base file both fail
   every assertion. Commit: single commit, subject byte-identical to the plan's
   fence, `%G?` -> `N` (unsigned), exactly one `Co-Authored-By` trailer, zero
   `Claude-Session` lines.
2. **Bounded diff.** `deny.toml`: 2 hunks at `-U0`, **0** deletion lines; TOML
   structural delta is exactly one new key (`advisories.unsound`) and exactly one
   new ignore id, with the base list order preserved as a prefix - no existing id
   reworded, reordered or removed, no other key touched. `pnpm-lock.yaml`: parsed
   both revisions as YAML; **exactly two packages moved**, `postcss` 8.5.16 ->
   8.5.25 and `nanoid` 3.3.15 -> 3.3.16; `importers`, `settings`, `lockfileVersion`
   and the total package count (265) identical; the only in-place snapshot changes
   are postcss's two parents. `nanoid` is justified by the lockfile itself
   (`postcss@8.5.25: dependencies: nanoid: 3.3.16`).
3. **Three-way fire, rebuilt.** run 1 shipped -> `advisories ok`, **EXIT 0**; run
   2 scope on / ignore removed -> `error[unsound]: Unsoundness in 'Iterator' and
   'DoubleEndedIterator' impls for 'glib::VariantStrIter'`, `ID:
   RUSTSEC-2024-0429`, `advisories FAILED`, **EXIT 1**; run 3 both removed ->
   `advisories ok`, **EXIT 0**.
4. **Blast radius as a set.** run 2 error/warning classes: `1 error[unsound]`,
   nothing else. Distinct fired ids: `{RUSTSEC-2024-0429}`. Set-differenced
   against the base ignore list (18 ids): fired MINUS pre-existing =
   `{RUSTSEC-2024-0429}`; pre-existing MINUS fired = 18. **Instrument fired:** the
   same class tally against a config with a bogus id returns `1 error[unsound]`,
   `1 warning[advisory-not-detected]`, `1 warning[unknown-advisory]`. `-L info`
   corroboration, base vs shipped: `advisory-ignored` 18 -> 19, distinct ids 18 ->
   19, `36 notes` -> `38 notes`, note-class difference exactly one
   `advisory-ignored` plus one `unsound`, `comm -13` on the id sets yields exactly
   `ID: RUSTSEC-2024-0429`. Full `cargo deny check` (all four checks) identical
   under base and shipped: `advisories ok, bans ok, licenses ok, sources ok`, 32
   pre-existing `warning[duplicate]`, exit 0 both.
5. **Untouched things.** Blob-vs-base per file: MATCH for `BUILDING.md`,
   `.github/workflows/ci.yml`, `Cargo.lock`, `package.json`,
   `crates/muxsmith-core/src/profile/model.rs`; MISMATCH for `pnpm-lock.yaml` and
   `deny.toml`, so the comparison discriminates on this tree. `git diff --stat
   5378264 c422999` names exactly those two files. The `cargo deny` invocation is
   `cargo deny check` in `BUILDING.md` and the pinned action in `ci.yml`; both
   blobs unchanged. (Post-commit, `git diff --exit-code -- <moved file>` returns 0
   for both - the blob comparison against the base is the instrument that still
   discriminates at this point, and it does.)
6. **The postcss requirement.** Advisory's own content, fetched independently:
   affected `<= 8.5.17`, first patched `8.5.18`, High. All four lockfile sites read
   `8.5.25`; `8.5.25 >= 8.5.18`. `pnpm install --frozen-lockfile` -> **EXIT 0**,
   `Already up to date`. `pnpm lint` 0, `pnpm build` 0 (identical asset hashes to
   the report's run: `index-DGn2eD1R.css`, `index-CO0ABiMW.js`), `pnpm check:i18n`
   0. `python3 scripts/ledger-lint.py` -> 0, `560 entries ... all invariants hold`.
7. **Latitude, both forms.** Nothing was decided at the keyboard that should have
   returned: both contradictions with the plan were surfaced with pasted evidence
   and neither fence was adjusted, which is what "Must not decide" requires.
   Nothing settled by the plan was returned or omitted: all eleven steps
   discharged, including Step 8's full reverse tree and the twelve-member tally
   with versions read from `Cargo.lock`. One unlisted third contradiction
   (finding 4). Both numbered findings hold on my own re-measurement.
8. **House dimension.** `proc-07-verify-against-source` - discharged, the
   mechanism is read at cargo-deny's `Default` impl and `Scope` enum, not at its
   output. `proc-verification-step-must-be-falsifiable` - discharged, the
   `--exit-code` and blob instruments are fired on the two files that moved, and
   the class tally is fired with a bogus id. `proc-normative-count-recomputed`
   trigger 2 - discharged and well: adding the nineteenth ignore, the implementer
   swept for consumers of "18" with a newline-flattened pass after naming why a
   line-based grep could not see them, and found two live ROADMAP sites plus one
   arguable. **I re-ran that sweep independently** and reproduce exactly those
   three ROADMAP sites and no fourth live one. `ci-10-pin-everything` - respected;
   the lockfile pins 8.5.25 exactly and `package.json` is untouched.
   `ci-04-dependabot-cadence` - no conflict; the ROADMAP records this vehicle as
   owning the window before Renovate goes live. `proc-no-work-needed-check` -
   this is where the task falls short: see finding 3.
9. **The no-work-needed check.** Every therefore-unnecessary in the report was
   run. "No NEEDS_CONTEXT is owed on part (a)" - premise verified (both parent
   ranges admit the patched version, frozen install succeeds). "nanoid is in
   scope" - verified from the lockfile's own snapshot. "No new scenario is owed"
   for part (a) - verified, no source file moved. **"Building that guard means new
   gate infrastructure" - REFUTED (finding 3).**

---

## HARVEST

**Dominant patterns worth carrying into the rest of Plan 11 and into the house
files. The controller is the single writer; these are surfaced, not written.**

1. **A verbatim fence is a custody boundary, not a truth guarantee - and the
   verbatim rule removes the last reader who could catch the error.** Two of the
   three defects in this task live inside a plan-mandated fence. The implementer
   applied both correctly, measured both to be false, and reported both; the
   mechanism worked exactly as designed, and the residue is that the fence's
   truth belongs to whoever wrote it. **Handle:** when a dispatch fences a factual
   block "apply verbatim", the fencing author owes that block a re-measurement
   against its own source document before the dispatch goes out, because after it
   goes out nobody is permitted to check it. This is the second consecutive
   session in which this shape produced a defect that reached an artifact.

2. **An explanation in a config comment is a normative claim about a tool's
   semantics, and it decays the same way a count does.** Fence A carried two
   independent factual errors, one of which (the count) the implementer caught and
   one of which (the scope semantics) nobody did - because the count is a NUMBER
   and the semantics is a SENTENCE, and this project's instruments are all aimed
   at numbers. **Handle:** where a comment explains why a tool behaves as it does,
   the explanation is verified at the tool's source with the same discipline
   `proc-07-verify-against-source` already applies to behaviour. The trigger is
   readable: the comment contains the word "because", "so", or "which means".

3. **`proc-no-work-needed-check` fired again, on a deferral rather than on a
   verification** - and the refuted premise had already survived a plan review,
   two delta reviews and an implementer. The shape: a deferral names a COST ("that
   would be new gate infrastructure") and the cost is never measured. This is
   structurally the same defect the plan's own self-review records against work
   item 1(b), where an inference from output silence survived until somebody
   opened the `Default` impl. **Handle worth promoting:** a deferral whose ground
   is "that would require X" is a claim about X, and the reviewer runs it. A
   deferral row is not exempt from the rule because it is a deferral.

4. **Tool-version skew between the local gate and CI is unstated anywhere and is
   load-bearing for exactly this kind of change.** `cargo deny check` means
   0.19.9 locally and 0.19.8 in CI. It happened to be safe here; it will not
   always be, and the first time it is not, the failure appears on the plan
   close's single push with every local gate green. **Handle:** record the skew
   where the gate is documented, and treat "a config key new in the local tool
   version" as a check the task owes.

5. **The instrument that best exposed all of this was building the regression
   state, not the success state.** Runs 1-3 prove the change works. Run 5 (key
   dropped, ignore kept) is what showed the deferral's premise was wrong, and the
   scratch single-crate projects are what showed the scope comment was wrong.
   **Handle:** where a change adds a configuration key, the fire set includes the
   state a future edit would produce by dropping it, not only the states the
   change moves between.

6. **A good pattern to keep, observed rather than criticised.** The implementer's
   stale-count sweep for "18 -> 19" named its own blind spot (a line-based grep
   cannot see a hard-wrapped assertion), built a second newline-flattened pass to
   close it, and fired that pass against a known-present control before reporting.
   I reproduced its result exactly. That is `proc-normative-count-recomputed`
   trigger 2 executed the way the entry describes, and it is worth an occurrence
   of kind `reinforced` rather than only being noted.

---
---

# FIX-ROUND DELTA (scoped re-review)

**Reviewed:** commit `5bf65dc` on `plan-11-stream-b`, parent `c422999`, one file,
8 insertions / 5 deletions. New commit, no amend - confirmed
(`git log -1 --format=%P 5bf65dc` -> `c422999`; `c422999` still reachable and
readable). Worktree clean at `5bf65dc`; `/home/senol/Git/muxsmith-plan11-a` not
entered. Instruments for this round are under
`.../b1rev-independent/fixround/`, built fresh from the `5bf65dc` blob rather
than reused from round 1.

**Delta verdict: both findings ADDRESSED. No new breakage. Round closed.**

## Per-finding

### Finding 1 (workspace scope "excludes every external crate") - ADDRESSED

The false clause is gone. The shipped replacement states what the filter actually
tests ("A `workspace` scope reaches only crates a workspace member depends on
directly, and glib sits deeper"), which is the formulation my adjudication 1(b)
supplied and which matches `src/advisories.rs`'s `Scope::Workspace |
Scope::Transitive` arm keying on `direct_dependents`.

### Finding 2 (the conflated count) - ADDRESSED

`grep -oE 'reported its [0-9]+' ` returns the string at `c422999` and returns
nothing at `5bf65dc`. The count is dropped rather than corrected, as recommended.

### The wording, checked rather than assumed

What shipped is **byte-identical** to my adjudication 1(b) block: present
verbatim, **1** occurrence, immediately after `yanked = "deny"` and immediately
before `# All entries below are transitive`. **Checker fired:** the same test
against the `c422999` blob returns False. Typography clean - ASCII only
(`grep -P '[^\x00-\x7F]'` empty), no line over 76 characters against the file's
own maximum of 78.

### The mechanism account, re-verified at the source

The new sentence covers more ground than the old one, so I re-read rather than
carried the round-1 result. `model::Informational` in cargo-deny 0.19.9 has four
variants - `Notice`, `Unmaintained`, `Unsound`, `Other(&str)` - and the scope
lookup maps only `Unmaintained` and `Unsound` to a config key; everything else,
including a non-informational advisory, falls through `_ => None` to `break
'block` and is emitted unconditionally. So both new claims hold: **"cargo-deny
scopes only the two informational classes"** is exact (two of four are scoped),
and **"a real vulnerability has no scope key at all and always fires"** is exact.
The unchanged tail was re-measured, not cited: `all` and `transitive` both exit
1 with the same single id on this tree.

## The operation

The replace-not-insert restatement is correct, and I fired the counterfactual
rather than accepting the amendment reviewer's measurement. Reconstructing what a
literal insert at `c422999` would have produced:

```
unsound assignment lines in the insert result: 2
[ERROR] failed to parse config from '<...>/deny-literal-insert.toml': duplicate key: `unsound`
EXIT: 1
```

cargo-deny rejects it at parse time, before evaluating a single advisory - so the
original instruction would have produced a red gate part, not a subtly wrong
config. The end state is right:

| blob | `^unsound = ` lines | parses | `advisories.unsound` | ignore ids |
|---|---|---|---|---|
| `5378264` | 0 | yes | absent | 18 |
| `c422999` | 1 | yes | `"all"` | 19 |
| `5bf65dc` | **1** | yes | `"all"` | 19 |

Ignore list at `5bf65dc` is **identical to `c422999` in content AND order**
(Python list equality, not a count). Structural TOML delta `c422999` ->
`5bf65dc`: zero keys added, zero removed, zero values changed. Every changed line
in the diff is a `#` comment line (`git diff | grep '^[+-][^+-]' | grep -v '^[+-]#'`
returns nothing). `git diff --name-only c422999 5bf65dc` -> `deny.toml` alone.

## The three-way fire, re-run on the changed config

Variants rebuilt from the `5bf65dc` blob at fresh paths, driven through `-c`; the
repository's `deny.toml` was never mutated.

| run | config | expected | observed |
|---|---|---|---|
| 1 | shipped `5bf65dc` | exit 0, `advisories ok` | **exit 0**, `advisories ok` |
| 2 | scope on, ignore entry removed | exit 1, `error[unsound]`, `ID: RUSTSEC-2024-0429` | **exit 1**, both strings present |
| 3 | both removed (control) | exit 0 | **exit 0**, `advisories ok` |

Blast radius unchanged by the fix: run 2 emits `1 error[unsound]` and no other
error or warning class. Gate part 5 on the shipped tree: `advisories ok, bans ok,
licenses ok, sources ok`, exit 0.

## New breakage in the fix diff

**None found.** Comment text only; the key, its value, the ignore list and every
other setting are bit-for-bit what `c422999` shipped, and the behaviour is
identical across all three fire states plus the full four-check run. Commit
hygiene holds: unsigned (`%G?` -> `N`), exactly one `Co-Authored-By` trailer,
zero `Claude-Session` lines, single file staged.

## The count-staleness question: CONFIRMED, and sharper than I stated

The implementer's claim holds, and the measurement makes it stronger than the
ground I gave in adjudication 1(b). Measured across the three blobs, each driven
through `-c` against this tree:

| blob | ignore ids | `note[unmaintained]` | `note[advisory-ignored]` | `note[vulnerability]` | comment says |
|---|---|---|---|---|---|
| `5378264` | 18 | 16 | 18 | 2 | (no such sentence) |
| `c422999` | **19** | **16** | **19** | 2 | `reported its 18` |
| `5bf65dc` | 19 | 16 | 19 | 2 | (removed) |

`git log -S` confirms the coincidence is exact: **`c422999` is the commit that
introduced both the string `reported its 18` and the id `RUSTSEC-2024-0429`**,
and `5bf65dc` is the commit that removed the string.

So at `c422999` every candidate referent for "18" is falsified **inside the blob
that shipped it**: the unmaintained class, which is what the sentence
grammatically asserts, read 16; the ignore list read 19; the `advisory-ignored`
notes read 19. The only state in which 18 is true is the PARENT commit's ignore
list - a state the same diff destroyed, four lines below the sentence asserting
it.

**What this changes in my reasoning.** I justified dropping the count on the
prediction that "a count over the ignore list goes stale on the next ignore
addition". The measurement replaces the prediction with an observation: it did
not wait for a next addition, it went stale within its own commit. That is
`proc-normative-count-recomputed`'s trigger 2 - *you are adding a member to an
enumerated set, so grep every numeral describing that set* - firing at zero
distance and being missed, which is the strongest possible case for the trigger
and the reason it exists.

**One honest qualification, so the close does not overclaim.** Staleness alone
does not decide between "drop" and "correct to 16": the unmaintained count is
stable at 16 across all three blobs and a corrected 16 would not have rotted.
What decides it is that the number carries no load in the sentence and sits
inches from a list built to accumulate, so any count there re-arms the same
trigger for the next writer. The sharper finding is about the FAILURE MODE being
demonstrated rather than predicted; the drop-versus-correct choice stays a
judgment, and it is the right one.

## Harvest (fix round)

Deferred minors and observations. None extends this loop.

1. **Trigger 2 fired at zero distance and was still missed - promote the
   evidence.** A number describing a set was falsified by an edit to that set in
   the same commit, four lines apart. Every existing occurrence of
   `proc-normative-count-recomputed` records the count and its enumeration drifting
   apart over time or across files; this one records them contradicting each other
   in a single diff, which is the case a reviewer is most likely to assume cannot
   happen. Worth an occurrence of kind `violated-corrected` with that distance
   stated, because the entry's own steelman ("a wrong count has never by itself
   produced a defect") is answered by it: here the count was one of two false
   sentences in a shipped security-gate comment.
2. **A prescribed edit whose operation is "insert between two anchors" needs its
   postcondition stated, not its position.** The plan's instruction was correct
   against the pre-B1 file and wrong against the file the branch actually had, and
   the failure mode was a duplicate TOML key that no amount of care at the
   keyboard would have caught before running the tool. The generalisable handle:
   an edit instruction that names a POSITION assumes a state; one that names a
   POSTCONDITION ("exactly one `^unsound = ` line") is checkable against whatever
   state is actually there. The implementer firing the insert reading first and
   watching it fail is exactly the discipline
   `proc-verification-step-must-be-falsifiable` asks for, applied to an edit
   rather than to a grep.
3. **Deferred minor, no action this round.** The new comment's parenthetical
   singles out real vulnerabilities as unscoped. `Notice` and `Other(&str)`
   advisories are unscoped too, by the same `_ => None` fall-through. The sentence
   is true as written and claims no exhaustiveness, so nothing is owed; recorded
   only so a future reader does not take the parenthetical as the complete
   complement of the scoped set.
4. **Round-1 findings 3, 4 and 5 are untouched by this fix and remain open** -
   the refuted permanent-guard premise, the unperformable Step-5 diff
   prescription, and the unrecorded local-vs-CI cargo-deny version skew. They were
   never in this round's scope; naming them here so the close does not read the
   fix round's approval as closing them.
