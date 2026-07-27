# Task 4 verdict - release.yml (D83/D84/D85/D77/D79/D88/D89/D90)

**Spec compliance: APPROVED** (byte-faithful to the amended section-2 fence, three-way
sha256 identity; every consumed cross-stream filename exact; pins 7/7 with version
comments; no finding at any severity)
**Task quality: APPROVED** (exemplary fork handling, borrowed A1 claim verified before
acting on it, every file-conditioned step re-run after the amendment; two minors, both
documentation-only, neither blocking)

Reviewed: branch `plan8-c`, commit `fe46424d4cf17faf24fdfd0c5bcb902b347d5221`, worktree
`/home/senol/Git/Muxsmith/.worktrees/plan8-c`. Graded against
`task-4-brief.md` + `implementer-preamble.md`; ground truth read from the **main-tree
(amended) design copy** `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`:
section 1.3 (runner labels), 1.4 (pin table), 1.5 (release-ops mechanics), D77, D79, D83,
D84, D85, D88, D89, D90, section 2 in full, section 6, section 7, section 8 (G4/G5 + R1-R10),
section 11, and the Amendment log (A1). Ledger entries re-read at their ids:
`design-frozen-fences-parser-loaded` (Tier 1, `docs/decision-ledger.yaml:4210`),
`proc-noninteractive-file-ops-in-agents`, `proc-latitude-clause-boundary`,
`proc-normative-count-recomputed`, `proc-verification-step-must-be-falsifiable`,
`proc-check-green-state-reachable` (all `docs/process-conventions.yaml`).

**Timing note.** `master` has advanced since the report was written: the report's "branch is
6 commits behind master" measures 9 today (`git rev-list --count plan8-c..master`), and three
of those are house-knowledge commits that landed the A1 and cp-alias harvests. Merge-base is
still `aec4cef`, ahead-count still 1, so the review package is the right diff. Not a defect,
a moving denominator.

**Fork trail, closed.** The NEEDS_CONTEXT episode is genuine, correctly routed, and already
harvested at Tier 1 as `design-frozen-fences-parser-loaded`. I reproduced both ends of it
myself rather than accepting the record (see the evidence table), so the red-green pair for
the parse check is now on the record a second time, from a different keyboard.

---

## Re-run evidence (all foreground; every mutation on a scratch copy, repo never written)

| Claim | My result | Verdict |
| --- | --- | --- |
| Fence boundaries in the amended design | section-2 header 1236, open fence 1241, close fence 1464, content 1242..1463 = **222 lines** | confirmed, re-derived from the delimiters |
| Byte fidelity, extraction 1 (sed line range) vs extraction 2 (independent awk state machine keyed on the `## 2.` header + fence markers) | identical, `sha256 9e78641c0f4b...` | both extractions agree |
| Committed blob (`git show fe46424:...`) vs fence | `diff` empty; **worktree file == committed blob == design fence, all three `9e78641c0f4b...`** | byte-faithful |
| Fire-verification of the diff method | perturbed copy (`name: release` -> `name: releasX`) printed `1c1 / < name: release / > name: releasX`, exit 1 | check demonstrably fires |
| YAML parse of the **committed blob straight from git** | `yaml-ok`, PyYAML **6.0.3**, top keys `['name', True, 'permissions', 'jobs']` | green |
| Second independent parser | `psych-ok` (Ruby Psych/libyaml) | green |
| Fire-verification of the parse check (un-quote line 94 on a scratch copy) | `yaml.scanner.ScannerError: mapping values are not allowed here ... line 94, column 59` | reproduces the pre-A1 red exactly, same line and column |
| A1 delta, from git (`git diff d21a19f^ d21a19f`) | **exactly one line inside the fence** (line 1332/1333, the quoting); the other changes are the status-header rewrap above the fence (which is why the fence shifted 1241->1242) and the appended Amendment log | the report's borrowed claim is accurate |
| G4, body extracted from the committed blob (step name line 119, `run: \|` line 121, **13 lines**, dedent 10, zero `${{ }}`) | case 1 `updater-artifact check: 0 hits across 1 bundle output files` exit 0; case 2 `::error::1 updater artifact(s) found - D76 bans updater output` exit 1; case 3a `::error::positive control failed: bundle output dir is empty` exit 1; case 3b `::error::positive control failed: no bundle output dir` exit 1; case 1 re-run green | matches design section 8; green state reachable after the reds |
| G5, `pick()` extracted from the rename body (step name 135, `run: \|` 137; `pick()` = rename-body lines 5..15, 11 lines), invoked through the real call-site shape `msi="$(pick ...)"` | one match: stderr `pick: target/release/bundle/msi/app.msi`, stdout captured the bare path, exit 0; two matches and zero matches: `::error::expected exactly one artifact, got: ...` exit 1; re-run green | matches design section 8; `exit 1` inside the command substitution does kill the calling step under `set -e` |
| Negative-space grep (`tauri-action\|softprops\|rust-cache\|mise-action\|Swatinem\|concurrency`) | exit 1 on release.yml; **positive control** on master's ci.yml hits exactly `40: Swatinem/rust-cache@c193711...` and `82: jdx/mise-action@e6a8b39...`, exit 0 | absent, with a firing control |
| Second fire-verification of the same grep | planted `# concurrency: fabricated-for-fire-test` on a scratch copy -> `224:` hit, exit 0 | fires on the token itself, not only on ci.yml's two |
| Pin recount | `grep -c 'uses:'` = **7**; enumerated: guard checkout (34), leg checkout (85), setup-node (98), pnpm/action-setup (101), upload-artifact (177), assemble checkout (190), download-artifact (191) | 7 recomputed from its enumeration |
| Every `uses:` SHA-pinned + version-commented | `grep -n 'uses:' \| grep -v '@[0-9a-f]\{40\} # v'` exit 1; SHA length audit: all five distinct SHAs measured at **40** hex chars | 7/7 |
| Fire-verification of the pin grep | line 98 -> `@v7` on a scratch copy -> `98:      - uses: actions/setup-node@v7`, exit 0 | fires |
| Every `@` in the file classified | 7 action refs + 3 shell expansions (`"$@"` 145, `[@]` 146, `[@]` 221) | the report's claim holds |
| Pins vs their sources | each of the four new SHAs appears **2x** in the design (1.4 table + section 2); checkout SHA appears **4x** in the design and **2x** in ci.yml | house pin reused, not a new one |
| Structure vs the brief's leg table | 4 include entries, `fail-fast: false`: `windows-x86_64`/`windows-2025`/`msi`, `windows-arm64`/`windows-11-arm`/`msi`, `macos-arm64`/`macos-15`/`dmg`, `linux-x86_64`/`ubuntu-22.04`/`deb,rpm,appimage` | character-for-character |
| Jobs, permissions, needs | `guard` (ubuntu-22.04, `{contents: read, actions: read}`, no needs) -> `bundle` (`needs: guard`, no block) -> `assemble` (`needs: [guard, bundle]`, `{contents: write}`); workflow-level `{contents: read}`; `contents: write` on exactly one job | as D77/D83 write it |
| Leg step count and order | **12**: checkout, rustup, apt deps (`if: runner.os == 'Linux'`), node parse, setup-node, pnpm/action-setup, `pnpm install --frozen-lockfile`, CLI build + sidecar, `tauri build`, updater assert, rename + tar.gz, upload-artifact | brief's order exactly |
| Poll cadence | `for i in $(seq 1 90)`, `sleep 30`, comment "up to 45 min"; recomputed 30 x 90 = 2700 s = **45.0 min** | consistent |
| upload/download-artifact inputs | upload: `name: muxsmith-${{ matrix.leg }}`, `path: release-assets/*`, `retention-days: 7`, `if-no-files-found: error`; download: `pattern: muxsmith-*`, `path: assets`, `merge-multiple: true` | D89 exactly |
| Triggers | `push.tags: ['v*']` + `workflow_dispatch` with the single input `rehearse-draft-release`, `type: boolean`, `default: false` | D79 exactly |
| Draft mechanics | `--draft` always (221); `extra_args=(--verify-tag)` on the tag arm only (210); `extra_args=(--target "${GITHUB_SHA}")` on the rehearsal arm (214); `relname="rehearsal-${GITHUB_RUN_ID}"` (212); body order banner -> `sed __VERSION__` template -> `generate-notes` API (215-220); `--notes-file body.md` | section 11 exactly |
| Section 11 absence: nothing publishes/edits/un-drafts, no tag is pushed | `grep 'release edit\|--draft=false\|release publish\|--generate-notes\|git tag\|git push'` exit 1; **control**: a fabricated `gh release edit "$relname" --draft=false` line is caught, exit 0 | absent, with a firing control |
| Both policy comment blocks | least-privilege block (14-15) and pinning-policy block with the two recorded D85 deviations (19-25) | present verbatim |
| Typography / hygiene of the blob | banned-glyph total **0**, non-ASCII **0**, CRLF **0**, tabs **0**, trailing-whitespace lines **0**, ends with newline, the single `!` is shell negation at line 146 | clean |
| Commit hygiene | one commit on the branch, one file, `222 insertions`, `%G?` = `N`, `Co-Authored-By: Claude Fable 5` trailer, message identical to the brief's step 7; `git diff --name-only master...plan8-c` = `.github/workflows/release.yml` alone | ci.yml untouched (D83) |
| Report claim "never pushed" (checked locally, no network, no gh call) | `branch.plan8-c.remote` unset, `refs/remotes/origin/plan8-c` absent, no `refs/remotes/origin/plan8*` at all | corroborated; amend-over-extend was legitimate |

### Four checks the brief did not ask for, run because the workflow is a *consumer*

The brief treats the cross-stream files as absent-by-construction and grades the
**names**. Names matching is necessary, not sufficient: a name can be right while the
contract behind it is wrong. Each of the five is now checked against the artifact the
sibling stream actually committed.

1. **All five consumed paths exist under exactly those paths on the sibling branches**:
   `scripts/check-version-sync.sh` and `src-tauri/tauri.bundle.conf.json` on `plan8-a`;
   `.github/release/draft-body.md`, `.github/release/rehearsal-banner.md`,
   `packaging/linux-tarball-README.txt` on `plan8-b`. No name drift in either direction.
2. **`check-version-sync.sh` is committed `100755`** with `#!/usr/bin/env bash`, and its
   argv contract (`if [ "$#" -ge 1 ]` -> tag equality) is exactly the 0-arg / 1-arg shape
   the guard's two arms pass. Had it landed `100644` the guard would die
   "permission denied" on the runner, and the workflow calls it directly rather than via
   `bash`.
3. **`draft-body.md` carries 8 `__VERSION__` occurrences**, so the assemble step's `sed`
   is not a silent no-op; its artifact table names D89's asset names. `rehearsal-banner.md`
   is 6 non-empty lines.
4. **Binary names verified against `cargo metadata`, not against the crate names**:
   package `muxsmith-cli` produces bin target **`muxsmith`** and package `muxsmith-gui`
   produces **`muxsmith-gui`**, so both `cp target/release/...` paths in the sidecar and
   tar.gz steps resolve. `tauri.bundle.conf.json`'s `externalBin: ["binaries/muxsmith"]`
   is the matching half of the workflow's `src-tauri/binaries/muxsmith-<triple>` staging.

### Three design-level behaviors made empirical instead of read

These are section-2 text, so no implementer latitude was involved; I ran them because
they are load-bearing for the section-8 acceptance test and are cheap to settle locally.

- **The D89 8-asset name set comes out character-for-character.** The rename body,
  extracted from the committed blob with only the two `${{ }}` expressions substituted
  per leg, run against fabricated *bundler-native* names
  (`Muxsmith_0.1.0_x64_en-US.msi`, `muxsmith-gui_0.1.0_amd64.deb`,
  `muxsmith-gui-0.1.0-1.x86_64.rpm`, `muxsmith-gui_0.1.0_amd64.AppImage`), produced:
  `muxsmith-0.1.0-windows-x86_64.msi`, `muxsmith-0.1.0-windows-arm64.msi`,
  `muxsmith-0.1.0-macos-arm64.dmg`, `muxsmith-0.1.0-linux-x86_64.{deb,rpm,AppImage,tar.gz}`
  - D89 items 1-7, `.AppImage` case preserved.
- **D88's tar.gz layout**: `tar tzf` lists `muxsmith-0.1.0-linux-x86_64/` with
  `muxsmith`, `muxsmith-gui`, `LICENSE`, `README.txt` under it. Four files, version-named
  directory prefix, nothing splats into cwd.
- **`sha256sum * > SHA256SUMS` does not list SHA256SUMS itself** (glob expanded before
  the redirect creates the file), and the round-trip `sha256sum -c SHA256SUMS` exits 0.
  D90's ordering claim survives contact with the shell; R4's `-c` pass is not resting on
  an assumption.

---

## Findings by severity

### Blocker: none. Major: none.

### m1 - "ten observed outputs" contradicts the report's own nine-row enumeration, and the miscount propagated one hop

Report `task-4-report.md:535` ("all ten observed outputs") and `:599`
("ten observed outputs matching section 8") summarize the evidence block at `:539-548`.
That block has **9 rows** (measured: 9 lines containing `->` between
`G4 case 1 (1 bundle file)` and `G5 one match re-run`): G4 cases 1, 2, 3a, 3b, 1-re-run
plus G5 one, two, zero, one-re-run. No counting convention yields ten - case runs are 9,
raw output lines are 11 (G5's green cases emit both a stderr and a stdout line).

The original-run count has the same defect one step smaller: `:268`
"Six observed outputs recorded, plus the two green re-runs" totals 8 against an
enumeration of 9 (7 substantive + 2 re-runs).

Root cause is identical in both and is `proc-normative-count-recomputed`'s **trigger 2**,
the one that catches what trigger 1 cannot: the design names three G4 cases, the
implementer legitimately split the positive control into 3a and 3b ("the step has two
distinct guards for them, and both fire with their own message" - a good call, it found a
real second guard), and *adding that member* is the moment the counts describing the set
went stale. Nobody was typing a number then; the numbers were elsewhere.

Evidence of harm, not hypothetical: the count reached my dispatch brief as fact
("the report claims ten observed outputs"). A reviewer who had graded by matching the
stated count against section 8's three-case-per-guard text would have gone looking for a
missing output that never existed.

Documentation-only. Every one of the nine outputs is substantively correct and I
reproduced all nine. **No fix needed in the workflow**; the report's two count words want
correcting to nine if the artifact is retained as a record.

### m2 - step 2's diff is a tautology as a *transcription* proof; its real (and earned) value is as a *restoration* proof

The brief's step 1 says "by transcribing design section 2's YAML fence exactly" and step 2
proves fidelity by extracting the fence and diffing. The implementer instead **produced**
the file by extracting the fence - correctly, and it says so plainly: extraction "is the
strictly more faithful method for a verbatim contract, and it removes transcription risk
entirely." Agreed, and I would have made the same call.

But the consequence is that step 2 then diffs an extraction against a file that *is* an
extraction of the same source. The report anticipates the objection - "Three-part, so the
empty diff is evidence rather than a tautology" - and answers it with a second, independent
extraction method. That answer does not fully hold: both methods read the same bytes of the
same file, so their agreement can only falsify extraction *mechanics* (a wrong line range),
never a transcription error, because no transcription occurred. The check cannot fail in the
direction step 2 was written to catch.

Two things keep this a minor rather than a real problem:

1. **It is load-bearing for something else, and it fired.** The file was mutated twice
   after creation (the pin fire-test at line 98, restored by re-copying the fence). The
   report's own residual note 3 records that the aliased `cp` "silently left the file
   un-overwritten, which the very next check caught because it re-read the file rather than
   assuming the copy." As a restoration check, the diff earned its keep on the day.
2. **The property is nonetheless established** - by this review, which re-extracted from
   the main-tree design with two methods of its own and matched the committed blob's
   sha256 three ways. That is the structurally sound place for the check to live: a
   third party, reading the contract independently of the tool that produced the artifact.

No change requested. Recorded because the *general* shape is worth a house entry
(HARVEST 1): when a brief mandates verbatim transcription plus a diff proof and the
implementer legitimately switches to extraction, the fidelity guarantee silently migrates
from the implementer's step to the reviewer's, and the brief does not say so.

### n1 - "matches design section 8 exactly" is marginally stronger than the artifact

G4 case 2's real output is `::error::1 updater artifact(s) found - D76 bans updater output`;
section 8 quotes `::error::1 updater artifact(s) found`. The design's string is an
abbreviation of the workflow's, so the observed output is a superstring, not an identity.
The report prints the full line in its evidence (`:220`, `:540`), so nothing is hidden and
a reader can see the difference - which is why this is a nit and not m3. Precise phrasing
would be "matches, with the design quoting the message prefix."

### n2 - informational: the nine-part gate is correctly not run

Report residual note 1. The preamble makes the gate a pre-push / post-merge controller
action, and this branch adds one CI workflow file touching no Rust or TypeScript path.
Flagged as visible-not-assumed, exactly as the implementer flagged it. Controller's item.

---

## The judgment the dispatch asked for: the `bundle` job's implicit permissions

**Not flagged. The implicit form does not diverge from D83's stated behavior, and I checked
rather than deferring to the ruling.**

D83's interface line reads "`permissions:` blocks per job (guard: `contents: read,
actions: read`; legs: `contents: read`; assemble: `contents: write`)". Section 2 gives the
legs no per-job block. Documented Actions semantics: `permissions` as a top-level key
applies to all jobs in the workflow, and specifying any scope sets every unlisted scope to
`none`. So the `bundle` job's effective token is `contents: read` and nothing else - the
same value an explicit `permissions: {contents: read}` block would produce, scope for scope.
There is no third state where the two forms differ.

Three notes on the boundary, so the adjudication is honest about what it rests on:

- The claim is **documented semantics, not a local execution**. I cannot execute the
  token-scoping locally, and I did not treat the design's or the report's assertion as the
  ground. The observable belongs to R1 (the legs' logs on a real run).
- Section 11 pins per-job permissions "as written", and section 2 is the verbatim source.
  Adding a behavior-neutral block would have been a design edit made at the keyboard -
  precisely the `proc-latitude-clause-boundary` line. **Not adding it was the correct call
  even if D83's phrasing invited the other reading**, because the implementer's route for
  the other reading is exactly what it used: flag it, let the controller rule.
- The controller ruling ("leave as written") is therefore upheld on the merits and not
  merely accepted. If anything wants tidying it is D83's interface line, which enumerates
  three blocks where the design ships two plus a default - a documentation-vs-artifact
  mismatch of zero behavioral consequence. Controller's call whether that is worth a
  line in the Amendment log; I do not recommend reopening the fence for it.

---

## Cannot verify here - these belong to Task 6 (the rehearsal), not to this gate

Named as deferred rather than treated as risk. Each with the section-8 R-step that owns
its observable, per `design-acceptance-observables-have-producers`:

| Deferred behavior | Owning observable |
| --- | --- |
| Guard's gate-green poll against real `gh run list --json status,conclusion` output | **R1** (the step's log names the found ci run) |
| `inputs.rehearse-draft-release` evaluating falsy on the push-tag path, and the `if:` gating the draft step | **R3** (step shows as skipped on run A) / **R4** (fires on run B) |
| `bundle` job's effective token scope on a live runner | **R1** (leg logs) |
| `windows-11-arm` image carrying the MSVC arm64 + WiX toolchain; `rustup toolchain install` reading `rust-toolchain.toml` (1.96.1) on each of the four images | **R1** (all four legs green) |
| `tauri build --ci -c ... --bundles` yielding exactly one file per format dir, i.e. `pick()`'s assertion in anger | **R1** (one `pick: <path>` line per selection: 1/1/3, closing `ls -l`) |
| `download-artifact merge-multiple: true` producing a flat `assets/`, so `sha256sum *` meets no subdirectory | **R4** (8 assets, `sha256sum -c` passes) |
| `gh release create --draft --verify-tag` / `--target`, the `generate-notes` endpoint, the composed body | **R4** / **R5** |
| A draft creating no tag ref | **R7** (with its `refs/heads/master` positive control) |
| Artifact names as displayed and `retention-days: 7` as applied | **R2** |
| Publisher rendering, install flows, `--version` self-report | **R8** / **R9** (owner, real hardware) |

One informational item nobody owns inside this plan: on the **tag** path, `body.md` is
first written by `sed ... >> body.md` (append), not truncate - safe, because a fresh
checkout has no `body.md` and runners are per-job, but the tag path is exercised only at
the first real tag, outside this plan (D81). Section-2 text, no latitude, no action.

---

## HARVEST

1. **Extraction-over-retyping for a frozen fence is right, and it silently moves the
   fidelity proof from the implementer to the reviewer.** A brief that says "transcribe
   verbatim" and then "prove it with a diff against the source" assumes retyping. When the
   implementer instead generates the file *from* the source (the better engineering call -
   it removes transcription risk rather than testing for it), the diff step degrades to
   comparing a method against its own input and can no longer fail in the direction it was
   written for; two independent extractions do not repair this, because both read the same
   bytes. Trigger is readable: **the artifact was produced by the same mechanism the
   verification step uses to check it.** Handle: keep the step (it still guards
   mutate-and-restore integrity, which is where it actually fired here), and state
   explicitly that byte-fidelity is established by an independent re-extraction in review.
   Candidate: new `process` entry, or a facet on
   `proc-verification-step-must-be-falsifiable` (a check that cannot fail in its intended
   direction is that entry's failure mode arriving by a different road: not an absence
   trusted without a red run, but a red run that is unreachable by construction). Neighbour
   of `proc-check-green-state-reachable`, which is the mirror image (green unreachable).

2. **`proc-normative-count-recomputed` trigger 2, third recorded shape: splitting one
   enumerated case into two.** The plan-8 T5 evidence added "a member that joins several
   sets at once." This is smaller and sneakier: the member joins **one** set, but the split
   is a *refinement of an existing member* rather than a visible addition, so it does not
   feel like adding at all - and it invalidated two counts in two places (`:268` and
   `:535`/`:599`). Handle unchanged, trigger wording could name it: "you split, merged, or
   refined a member of an enumerated set" belongs beside "you are adding a member."
   Candidate: `violated-corrected` occurrence with the refinement facet.

3. **A count that is wrong in a report reaches the next dispatch as a fact.** The "ten"
   travelled from `task-4-report.md` into the review brief without anyone recomputing it,
   which is what a downstream consumer of a report does by design. Concrete argument for
   why the count rule is not cosmetics: the enumeration binds, but the *count* is what gets
   quoted, and a quoted count is unfalsifiable at the point of use. Candidate: supporting
   evidence on entry 2's occurrence, or on the report-hygiene side of
   `proc-02-whole-branch-review`. Controller's call.

4. **`design-frozen-fences-parser-loaded` reinforced from a second keyboard.** I
   reproduced both ends independently: the amended blob loads under PyYAML 6.0.3 **and**
   Psych, and un-quoting line 94 on a scratch copy reproduces
   `mapping values are not allowed here ... line 94, column 59` - the same line and column
   the pre-A1 run reported. The red-green pair is now doubly recorded. Worth noting for the
   entry's handle: the check that caught this is trivially cheap (one `python3 -c` per
   fence) and the cost of missing it was a routed fork plus a design amendment plus two
   agent re-runs. Candidate: `reinforced` occurrence.

5. **A workflow is a consumer, so a brief that defers its inputs to sibling streams should
   name the *contract*, not only the filename.** This brief's interface list gives five
   filenames and (correctly) declares their absence expected. Names matching is necessary
   and not sufficient: `check-version-sync.sh` had to be `100755` (a `100644` fails the
   guard on the runner with "permission denied", and no name check sees it),
   `draft-body.md` had to contain `__VERSION__` (else the `sed` is a silent no-op that
   ships a release body reading "Muxsmith __VERSION__"), and the two `cp target/release/...`
   paths depend on `[[bin]] name` values, not on crate names. All four hold here - verified
   in this review, by nobody's step. Handle for plan authoring: a cross-stream interface
   line carries the consumed **property** (exec bit, placeholder token, bin name) beside
   the path, and the integration check lands in the consumer's review. Neighbour of
   `plan-interfaces-absent-by-construction`.

6. **`proc-noninteractive-file-ops-in-agents`, fourth datapoint - and the shape that makes
   the entry's second sentence the load-bearing one.** The entry's occurrence 3 already
   records T4's instance. What this review adds is the *evasion*: I avoided the class
   entirely by never mutating a repo file - every fire-test ran on a scratch copy under
   `/tmp/.../scratchpad/t4rev`, with `command cp -f ... < /dev/null` and
   `command rm -f ... < /dev/null` for the scratch churn, and the repo verified pristine
   afterwards (`git status --short` empty in both trees, and the worktree file's sha256
   still `9e78641c0f4b...`). That is strictly stronger than restore-and-verify, and it is
   available whenever the check only needs to *read* the artifact - which is every absence
   grep, every parse, every extraction fire-test in this task. Handle worth adding to the
   entry: **prefer not mutating the tracked file at all; copy it out and break the copy.**
   Mutate-in-place is for checks that must see the real path (a numstat, a build). This also
   dissolves T4's own failure mode - the report's silent un-overwritten `cp` cannot happen
   to a file nobody needed to restore. Candidate: `reinforced` occurrence carrying the
   copy-out-first handle.

7. **Fire-verifying a grep by planting the token beats fire-verifying it against a
   neighbouring file.** The brief's step 5 control greps ci.yml, which proves the *pattern*
   matches `mise-action` and `Swatinem` - two of its six alternatives. It says nothing about
   the other four (`tauri-action`, `softprops`, `rust-cache`, `concurrency`) or about the
   pattern reaching release.yml at all. I ran both: the ci.yml control, and a planted
   `# concurrency: ...` line appended to a scratch copy of release.yml. Per
   `proc-verification-step-must-be-falsifiable`'s "PER ASSERTION, not per script" facet, an
   alternation of six is six assertions and a control hitting two leaves four unexercised.
   Handle: when the absence check is an alternation, the control plants a token *in the file
   under test*, and covers the alternatives that no neighbour file happens to contain.
   Candidate: `reinforced` occurrence on the per-assertion facet, with the alternation shape
   named.

---

*Review constraints honored: read-only throughout - the only file written is this verdict.
No commit, no git write, no session-relocation tool, no `gh` call (the "never pushed" claim
was corroborated from local refs and config instead). All runs foreground, all paths
absolute. Every mutation for a fire-test happened on a copy under
`/tmp/claude-1000/-home-senol-agents-peter/956e4bb0-0f7b-4fb0-b5bb-883292a70ae4/scratchpad/t4rev`;
both worktrees verified clean and the artifact's sha256 unchanged at the end.*
