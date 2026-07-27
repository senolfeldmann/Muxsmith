# Task 2 verdict: the license experiment, then the picked branch (A5)

**NEEDS FIXES** - one major, two minors. The machine half is correct in every
respect I could measure, and it is correct for stronger reasons than the report
gives: I re-ran the experiment on my own harness, added two controls the plan
does not carry, and proved the macOS-only selection in both directions. The
major is not in what landed but in what was not swept: this commit added a
member to an enumerated set and did not visit the two live lines that enumerate
that set, which is the second trigger of `proc-normative-count-recomputed` and
the plan's own Global-Constraints sentence on recomputed counts - in a commit
whose entire routed instruction was about exactly that defect class.

**Graded:** commit `50e08cd` (`50e08cdd5763b8e5baa5716e1afd2726f564461c`),
range `5060ef5..50e08cd`, 3 files, +37/-2, on `master`, unpushed
(`origin/master` still `b1eb231`), tree clean at grading start and end.

**Ground truth used, in the stated order:** the plan's Global Constraints,
owner-steps section and all of Task 2; the plan-8 packaging design; the ROADMAP
Plan-8.5 anchor (`docs/ROADMAP.md:209`); the four house YAML files by entry id.
The report was read once, then verified rather than graded.

**Method note (`proc-readonly-reviewer-falsification`).** Where a side-effect-free
substitute existed I used it (comparisons piped from `git show 5060ef5:<file>`,
a synthetic em-dash piped through the glyph grep, known-present controls for
every absence check). The platform-config mechanism cannot be exercised that way
- the CLI resolves it by filename on disk - so those probes mutated the tree and
were restored alias-proof (`command cp -f` / `command rm -f`) and verified
against a pre-experiment `sha256sum` backup, with `git status --porcelain` and
`git diff --stat` empty afterwards and the empty-status instrument fired once
against a planted canary (`?? src-tauri/zzz-reviewer-canary.json`). `git tag`
count 0; HEAD unchanged.

**My instrument:** `/tmp/claude-1000/-home-senol-agents-peter/d4bb860f-d33c-4173-b6ff-d56c277d5084/scratchpad/rv-t2-peter-8f3c1d/`
(`probe-01-baseline.sh`, `probe-02-null.sh`, `probe-03-setvariant.sh`,
`probe-04-platform-selection.sh`, `probe-05-c-overlay-interaction.sh`,
`verbatim.py`, `fence44.py`). Written before any of the implementer's outputs
were re-read, at a path no other agent in this session would land on, with its
own capture files and its own control set.

---

## 1. The experiment, re-run on my own harness

Repo-pinned CLI `tauri-cli 2.11.4`; `target/debug/muxsmith-gui` present
(281228536 bytes), so no rebuild.

| phase | `src-tauri/tauri.linux.conf.json` | `grep -cE '^ Recommends'` | control |
|---|---|---|---|
| A baseline | absent | **1** (` Recommends: mkvtoolnix`) | ` Depends: libwebkit2gtk-4.1-0, libgtk-3-0`, count 1 |
| B null-clear | `licenseFile: null` + `linux.deb.recommends: null` | **0** | Depends line byte-identical, count 1 |
| C set-variant (mine) | `recommends: ["mkvtoolnix","zzz-reviewer-control"]` | **1** (` Recommends: mkvtoolnix, zzz-reviewer-control`) | injected token count 1 |

`dpkg-deb -I` control member went `429 bytes, 12 lines control` ->
`406 bytes, 11 lines control`; deb size 72965246 -> 72965226; the A-vs-B diff is
the `Recommends` line plus the two size lines, nothing else. Both figures the
report states are reproduced exactly.

**Fire-verification of the absence:** the identical invocation returned 1 on
phase A minutes earlier, in the same shell, against the same glob.

**Phase C is a control the plan and the report do not carry**, and it closes the
gap the 1 -> 0 result alone leaves: the same file, at the same path, with a
non-null value, *injects* a new recommends entry. So the file is genuinely read
and the merge is directional; the 0 in phase B is the null doing work, not the
file's presence breaking the control archive.

**The CLI accepts the null rather than rejecting it:** phase B's
`tauri bundle -d --bundles deb` exited **0**, produced the deb, and its stderr
carries zero matches for `error|not of type|invalid`. Note this is the evidence,
not the report's `tauri inspect` run - see minor M2.

**Extra: the merge DELETES, it does not preserve a null.** `bundle.active` is
schema-typed bare `boolean` (non-nullable) and the base config sets it `true`.
Red control `{"bundle": {"active": 123}}` in the platform file, target
`x86_64-unknown-linux-gnu` -> exit 1,
``Error `"tauri.conf.json"` error on `bundle > active`: 123 is not of type "boolean"``.
Same file, same target, `{"bundle": {"active": null}}` -> passes schema
validation and proceeds to the binary-open stage. A preserved null at a
non-nullable key would have failed; it did not. RFC 7396 delete semantics
confirmed empirically on the pinned CLI, independent of A5's source citation.

**Extra: the clear survives D82's `-c` overlay.** `release.yml:118` runs
`pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json --bundles dmg`,
so the macOS leg applies *both* the auto-merged platform file and a `-c` file.
Neither the plan nor the report tests the composition; if `-c` were applied to
the base config rather than the platform-merged one, the licence would come
back and the whole fix would be silently void at release time. Measured:
platform null-clear plus `-c '{"bundle": {"category": "Video"}}'` -> Recommends
**0**, Section `video`, Depends intact; control with the same `-c` and no
platform file -> Recommends **1**. The clear survives, and the difference is
attributable to the platform file alone.

**Verdict on dimension 1: CONFIRMED.** Fourth independent reproduction (plan
author, plan reviewer, implementer, me), first one with a set-variant control, a
delete-vs-preserve discriminator and the `-c` composition test.

## 2. The clear branch is what landed; no fallback artifact; no trigger passed over

- `src-tauri/tauri.macos.conf.json` carries `{"bundle": {"licenseFile": null}}`,
  5 lines, 46 bytes, LF, trailing newline, no `$schema` - the house shape of
  `src-tauri/tauri.bundle.conf.json`. Not the fallback's `"../packaging/macos-license.rtf"`.
- `packaging/` holds only `linux-tarball-README.txt`. `git ls-files` piped
  through `command grep -i 'macos-license\|\.rtf'` returns nothing; the same
  invocation returns 1 for `tauri.macos.conf.json`, so the empty result is
  evidence.
- The committed A5 is the clear variant: the fallback A5's distinguishing phrase
  `via the pre-decided tiebreaker` occurs **0** times in the design and **1**
  time in the plan (instrument fired); `macos-license.rtf` occurs **0** times in
  the design and **9** in the plan.
- Step 4's branch rule: my phase B printed 0, the authoring-time result, so the
  CLEAR branch is the mechanically correct branch and no NEEDS_CONTEXT trigger
  of Step 4 fired. Triggers 2 and 3 of the fallback section are post-hoc (release
  leg, owner O2) and cannot fire at this task. No bundle error, validation error
  or missing tool occurred in my re-run either.
- BUILDING.md carries the clear-branch paragraph, not the fallback variant.

## 3. The macOS overlay does what it must and nothing more

Proven in both directions with a fired positive and two fired negatives. Probe:
a deliberately schema-invalid `licenseFile: 123` planted in the committed
overlay, then read back under three targets.

| probe | target | result |
|---|---|---|
| D5 | `aarch64-apple-darwin` | exit 1, ``error on `bundle > licenseFile`: 123 is not of types "null", "string"`` - **the overlay IS loaded** |
| F3 | `x86_64-unknown-linux-gnu` | passes config validation, fails at `can't open main binary .../muxsmith-gui` - **not loaded** |
| D6 | `x86_64-pc-windows-msvc` | passes config validation, fails at `can't open main binary .../muxsmith-gui.exe` - **not loaded** |
| F2 | `x86_64-unknown-linux-gnu`, invalid value in `tauri.linux.conf.json` instead | exit 1, same schema error - **positive control: this instrument does fire on a Linux-loaded overlay** |

F2 is what makes F3 and D6 mean something: the same target and the same command
do reject an invalid *linux* platform file, so passing with an invalid *macos*
one is a real negative, not a validation stage that never runs.

Corollary the plan's Step 8 cannot give: **the committed overlay is schema-valid
for macOS.** With the real committed file and `--target aarch64-apple-darwin`,
the run reaches the binary-open stage (D2), i.e. past config validation - and D5
proves that stage would have caught a malformed overlay.

Windows and Linux surfaces untouched:

- `src-tauri/tauri.conf.json` is not in the commit; `licenseFile: "../LICENSE"`
  still at `:38`, `license: "MIT"` at `:37`.
- Pinned schema `node_modules/@tauri-apps/cli/config.schema.json`: `license` and
  `licenseFile` occur exactly once each, both at
  `definitions.BundleConfig.properties`, both typed `["string","null"]`. No
  Dmg/Wix/Nsis/Deb/Rpm/AppImage section carries a licence property, so the
  global key is the msi's only source and the platform file is the only lever.
  The design's claim on this surface holds at the pinned artifact.
- The msi keeping its licence text is therefore load-bearing and safe **at the
  config level**: the Windows target provably never loads the overlay (D6) and
  the global key is byte-identical. The artifact-level confirmation is Task 4
  RR5 and I make no claim on it.
- The pinned CLI's own `bundle --help` names the mechanism first-hand: "a
  platform-specific file is looked up and merged with the default file by
  default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json,
  tauri.android.conf.json and tauri.ios.conf.json)". The filename is right.

## 4. The routed status-line cut

**The enumeration is gone and replaced by a pointer, and nothing else in the
header moved.** Line-by-line comparison of the whole file against
`5060ef5`: exactly lines **3 and 4** differ, and the tail grew by 24 lines
(2174 -> 2198) for A5 plus its blank separator. Nothing else in 2174 shared
lines changed.

```
PRE : Status: DRAFT 2026-07-22, fix round 1 applied 2026-07-23; amendment log at
      the end (A1, 2026-07-23; A2 and A3, 2026-07-27). Numbering starts
POST: Status: DRAFT 2026-07-22, fix round 1 applied 2026-07-23; every amendment
      is recorded in the `## Amendment log` section at the end. Numbering starts
```

Character-level: the prefix `Status: DRAFT 2026-07-22, fix round 1 applied
2026-07-23; ` and the suffix ` Numbering starts` are both byte-preserved, so the
DRAFT date, the fix-round note and the D-numbering paragraph (lines 5-8,
unchanged) are untouched. The changed span is exactly the enumeration clause.
Pointer target `^## Amendment log$` occurs **1** time (design `:2063`);
instrument fired (a deliberately misspelled heading returns 0). Report's own
greps re-run by me: `A2 and A3, 2026-07-27` -> **0** post, **1** on the
`5060ef5` blob.

**Ruling on the correction: the implementer is right, and the disposition is
right for a reason stronger than the one given.** I reproduced all three greps.
`amendment log at` is now 0 tree-wide. `A1, 2026-07-23` returns three hits, all
under `docs/process-journal/artifacts/plan-8-sdd/`; `A2 and A3` returns two, one
of them the session-23 handoff. I opened every one. They are past-tense records
of the 2026-07-23 fix round and the session-23 close - e.g.
`owner-wording-verdict-delta.md:182` reads "enumerated the amendment set as
"(A1, 2026-07-23)" and had been stale since A2 landed". The controller's
"`git grep` finds no consumer of the enumeration anywhere in docs/" was, as
phrased, false: hits exist. The *substance* survives, because none of them is a
consumer.

The house entry that settles it is `code-comment-line-citations-drift`
(`docs/conventions.yaml:1013`), which discriminates exactly this: a LIVE
DESCRIPTIVE claim gets re-pointed on drift, an EVIDENTIARY record - what the
author saw - "is never re-pointed (that would falsify the record)". The one hit
that phrases itself in the present tense
(`wording-fix-round-report.md:188`, "so it now reads ...") is an evidentiary
record of a completed fix round, not a live claim, so leaving it is not
sloppiness but the prescribed handling. Correcting the controller rather than
letting the claim pass is `proc-57-briefs-not-ground-truth` behaving as designed.

**Ruling on the cut: sound, and empirically vindicated inside its own commit.**
The enumeration was a dependency over a growing set, in a header, with no
consumer; it had gone stale twice in one day; the pointer target exists exactly
once; the surrounding header is byte-preserved. And the very same commit
appended A5, which would have staled it a third time - the strongest available
evidence that killing the class beat scheduling its next maintenance.

**But the cut removed one instance of the class while two more sat in a live
file, unswept and unnamed** - see F1.

## 5. Amendment A5

**Byte-identical to the plan's Step-7 clear-branch fence.** 23 lines, unified
diff empty. Appended after A4 at design `:2175` of 2198, one blank line between,
inside `## Amendment log` (`:2063`). Exactly one non-ASCII character in the block
(`Ş`, U+015E, at `:2179`), which is the D86 orthography exception; no smart
quotes, no long dashes, no Unicode ellipsis anywhere in the commit diff
(instrument fired on a planted em-dash).

Every claim A5 makes about the design resolves at the tree:

- `### 4.4 \`BUILDING.md\` rider (D82/D86, addition, complete)` at `:1767`.
- `## 11. What the implementer must not decide` at `:2021`.
- D82's "The name deliberately avoids the auto-merged `tauri.<platform>.conf.json`
  filenames, so it never applies implicitly" at `:635-636` - A5's scope note
  ("sidecar-specific; `externalBin` is processed on every compile") is a correct
  reading of what that sentence is protecting, and it is the reconciliation the
  plan's correction table 2 promised.
- D86's parenthetical "deb/rpm embed it as-is" at `:950` - the site the
  correction rides on exists.
- 4.4's fence vs the tree: I diffed them. The **only** divergence is the new
  paragraph. A5's "superseded by the tree at the new platform-config paragraph"
  is precise, not an understatement.

**The design's own claims still hold after A5.** The plan does not commission
`proc-04-spec-wins`' self-contradiction sweep, so I ran one over the surface the
new file touches: every `tauri.*conf.json` mention in the design, D82's naming
rationale, section 11's frozen-literal list, section 4.4's fence, D86. All four
affected sites are named in A5. Section 1's ground-truth table row 4
("bundle block today: ... no per-OS config") is an evidentiary pre-plan-8
snapshot of the tree as the design author found it, not a live claim, so the new
file does not contradict it. Nothing uncovered.

## 6. No claim beyond the machine half

Clean. The report's "Not claimed" section is accurate and I could not falsify
it: nothing in the commit, the commit message, A5, the BUILDING.md paragraph or
the report asserts O2, a dmg observation, or ruling 2 as verified. A5's "the dmg
ships without the pre-mount SLA" and BUILDING.md's "the dmg ships without a
pre-mount license dialog" are plan-authored design prose describing the
mechanism, carried verbatim, not verification claims - and both are the plan's
words, not the implementer's. The report's "CONFIRMED, not refuted" is scoped to
the authoring-time experiment result, which is the correct object.

## 7. Commit hygiene and house conformance

| check | result |
|---|---|
| unsigned per SI-4 | `git log -1 --format='%G?'` -> `N` |
| trailer | `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, exact |
| explicit staging | 3 files, exactly the plan's Step-9 `git add` list; no stray file |
| message | first line byte-matches the plan's Step-9 clear-branch message |
| no tag / publish / delete | `git tag` count 0; `origin/master` still `b1eb231` |
| typography | no AI-tell glyph in the diff (instrument fired); `Ş` intact |
| `proc-latitude-clause-boundary` | no latitude clause carried anywhere in the change |
| `proc-07-verify-against-source` | experiment re-run against the repo-pinned CLI, not asserted from the plan |
| `proc-normative-count-recomputed` | **violated - F1** |
| `proc-04-spec-wins` (sweep corollary) | not commissioned by the plan; run by me, nothing uncovered |
| `code-comment-line-citations-drift` | journal records correctly left as evidentiary |

No house YAML entry covers the licence/dmg/EULA surface; it lives in the design
(D86 + A5), which is where the plan puts it. Ledger bookkeeping is a plan-close
controller duty, not this task's.

---

## Findings by severity

### MAJOR

**F1 - the trigger-2 sweep was not run: two live lines still enumerate the
amendment set as `A1-A3`.**

Plan Global Constraints: "a task that changes a set re-recounts and updates the
consuming line in the same change." `proc-normative-count-recomputed`
(`docs/process-conventions.yaml:497`) states the readable trigger explicitly:
"(2) **you are ADDING A MEMBER to an enumerated set - then grep every numeral
and count-word describing that set**, because at that moment nobody is typing
the number, the number is somewhere else and stale, and trigger 1 never fires."

Task 2 added A5 to the amendment set. The sweep the implementer ran was scoped
to the three phrasings of the one line the controller routed
(`amendment log at`, `A1, 2026-07-23`, `A2 and A3`) - it was a consumer check
for the *removed line*, not a trigger-2 sweep over the *set*. My sweep
(`git grep -nE '\bA1[ ]*(-|to|,|and)[ ]*A[2-9]\b|amendments? A[1-9]|amendment log A'`,
instrument fired against a known-present range) finds exactly two live sites,
both in the currently-executing plan:

- `docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md:17`
  (**Global Constraints, normative precedence**): "the plan-8 design
  (`...plan8-packaging-release-design.md`, D75-D90 + A1-A3) governs the
  packaging surface this plan touches". The governing set is now A1-A5, and A5
  is precisely the amendment governing the file this commit created. This is an
  unenumerated set in a normative position - the shape the same Global
  Constraints paragraph calls a defect.
- `...:113` (Task 1 "Read first: ... amendment log A1-A3"). Dated dispatch
  instruction, weaker, same class.

Every other `A1`/`A2`/`A3` hit outside the journal is a single cross-reference,
not a set enumeration; journal hits are frozen evidentiary records. So the
finding is exactly two sites, one of them normative.

**Attribution, stated honestly:** A4 (Task 1, `9460daf`) staled these lines
first; Task 1, its reviewer, its delta reviewer and the controller all missed
them. A5 re-staled them. Task 2 is not the origin - it is the task that was
standing directly in front of the class, executing a controller ruling whose
stated purpose was "an enumeration in a header over a growing set is a
dependency that must be visited on every append", and it did not look one file
over.

**Remedy - route, do not fix at the keyboard.** The plan file is not in Task 2's
Files list, so editing it unbidden would repeat the error Task 1's review named
("the implementer was right not to touch it unbidden; Step 4 should have
returned NEEDS_CONTEXT"). Required: run the trigger-2 sweep, report its full
result, and return the two sites to the controller as NEEDS_CONTEXT for routing.
The obvious routing is the same medicine the controller already prescribed -
replace `A1-A3` at `:17` with a pointer to the design's amendment log rather
than a corrected range that will stale again at A6.

### MINOR

**M1 - report count wrong on a five-line file.** The report states
`src-tauri/tauri.macos.conf.json` is "four lines plus trailing newline". It is
five: `wc -l` -> 5, and the diff hunk header is `@@ -0,0 +1,5 @@`. A count is a
measurement (`proc-normative-count-recomputed`, trigger 1); in a report whose
subject is stale enumerations it should not be off by one. Fix the report line.

**M2 - Step 8's `tauri inspect` is a weaker instrument than the report reads.**
The report presents Step 8 (`tauri inspect wix-upgrade-code` exit 0, GUID
printed) under the new file's edit narrative. I measured that `tauri inspect`
does **not** validate platform overlays at all: with a schema-invalid
`tauri.linux.conf.json` on disk it still exits 0 and prints the GUID, while the
identical file makes `tauri bundle --target x86_64-unknown-linux-gnu` exit 1
with ``error on `bundle > licenseFile``. So Step 8 could not have caught a
malformed `tauri.macos.conf.json`; it evidences only that the base config still
loads. This is the plan's step and the plan scopes its claim correctly, so it is
not an implementer defect - but the report should say what the check does and
does not cover. The overlay's validity is established, independently, by my D2/D5
probe, and that is what should be cited. Worth a line in the plan-close harvest:
`tauri inspect` is not a config-validation instrument.

### OBSERVATIONS (no fix required)

- **The `-c` composition was untested by both plan and report.** The macOS
  release leg applies the platform file *and* `-c src-tauri/tauri.bundle.conf.json`
  (`release.yml:118`); had `-c` been applied against the base config, the clear
  would have been silently void at release time. I tested it: it survives
  (Recommends 0 with both, 1 with `-c` alone). Recommend carrying this as
  recorded evidence into Task 4 rather than re-deriving it there.
- **The commit message does not mention the routed status-line cut** it carries.
  The message is plan-verbatim (Step 9) and the routing is recorded at
  `progress.md:13-14`, so this is a plan artifact, not an implementer deviation.
  Flagging it only because the diff and the message diverge in scope.
- **Positive signal worth keeping:** the implementer corrected a controller claim
  rather than letting it pass, disclosed the fourth edit with its exact old/new
  text, fire-verified its own empty `git status` against a later non-empty run of
  the same invocation, and refused to claim ruling 2. That is the behaviour the
  house rules are trying to produce.

---

## Fix list

1. Run the `proc-normative-count-recomputed` trigger-2 sweep over the amendment
   set; report the full result; return the two sites
   (plan `:17`, plan `:113`) as NEEDS_CONTEXT for controller routing. Do not
   edit the plan file at the keyboard.
2. Correct the report: the overlay is five lines, not four.
3. Correct the report's framing of Step 8 per M2, citing what actually
   establishes the overlay's validity.

None of the three touches the committed tree. Re-review scope: the report plus
the routed-sites return; the commit itself needs no change.
