# Task 2 verdict (independent review): D86 bundle config + D82 CLI-sidecar overlay + collateral

Reviewed commit `c884bd6233355deedf1e89f808619b711f01af1b` on branch `plan8-a`,
worktree `/home/senol/Git/Muxsmith/.worktrees/plan8-a`, base `7e36f96` (Task 1).
Inputs: `task-2-brief.md`, `task-2-report.md`, `review-7e36f96..c884bd6.diff`,
`implementer-preamble.md`.

**Citation pin.** All design line numbers are anchored to the post-A1 main-tree copy
`/home/senol/Git/Muxsmith/docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`,
sha256 `734490f8d7186391949cd22ef875e69bc3e41f502645287fc6b8d03c27638af3` (same pin the
Task 1 verdict used). Sections read in full: D82 (:593-715), D86 (:922-1015), 3.1
(:1478-1527), 3.2 (:1529-1537), 3.4 (:1569-1571), 4.4 (:1742-1765), 11 (:1989-2028),
8 (:1849-1936). The implementer's source was the pre-A1 worktree copy, sha256
`576f4dc3997fea71ee5e3aa466d6254b39e21589a6ec5e92c026e80c40188113`; I graded against the
amended main-tree copy directly, so the pre/post-A1 question is settled by measurement,
not by trusting the report's A1 analysis (which I also re-ran and confirmed).

Everything below was re-run by me in the foreground. Mutating probes used
`/usr/bin/cp -f` / `/usr/bin/rm -f` inside `bash` trap-restore scripts
(`proc-noninteractive-file-ops-in-agents`, `shell-dialect-assumptions-in-agent-scripts`);
every restore was confirmed by `sha256sum -c` against a pre-mutation baseline plus an
empty `git status --porcelain`. Final tree state: clean, `0` untracked
(`--untracked-files=all`), no strays.

## Verdict 1 - spec compliance: APPROVED

**The three design blocks are byte-identical, proven by hash equality (an affirmative
result, not an absence).**

| Block | design lines | sha256 (design == landed) |
|---|---|---|
| 3.1 bundle block (32 lines) | :1490-1521 | `3850cc560df2ed64c41d29ae5f6c3af605513817eb384d298d25ac8673abec7c` |
| 3.2 overlay (whole file, 5 lines) | :1532-1536 | `780279a2b6acf4feb5aaefed102ac5475c6f89d43aa97779faa3a8a1797e4f9f` |
| 4.4 rider (18 lines) | :1747-1764 | `9dad09509c4f1988e72d467cb3a84d51af81b30e8d5ce74ff4c7f7f52644c907` |

Extraction was anchor-based, not fixed-range, for the bundle block (`awk` from
`^  "bundle": \{$` to EOF minus the root brace) per `proc-wrapped-prose-quote-grep`;
`diff` reported zero differences on all three and the hashes match, so this is not a
silent empty-vs-empty pass. The 3.2 hash also equals the committed blob hash the report
states (`780279a2...`), and the committed `tauri.conf.json` blob is
`3371d13dbd232e0366f173c24611a425535abe8a6426eead5ee429bdc2687aec`, matching the report.

**Overlay filename.** Exactly one spelling at every design site:
`grep -o 'tauri\.[a-z.]*conf\.json'` over the design yields `9 tauri.bundle.conf.json`
and `17 tauri.conf.json`, no third form. The shipped path is
`src-tauri/tauri.bundle.conf.json`, mode `100644`, trailing newline present. (The count
is 9, not the 8 the report and the review dispatch both state - finding m1; the
substantive claim "one name at all sites" is true.)

**`Ş` is a real U+015E in the committed text, not a transliteration and not a `\u`
escape.** Hexdumped from `git show HEAD:src-tauri/tauri.conf.json` line 34, i.e. the raw
committed bytes with no jq decoding in the path:
`20 20 20 20 22 70 75 62 6c 69 73 68 65 72 22 3a 20 22 c5 9e 65 6e 6f 6c ...`
(`c5 9e` = U+015E). Same two bytes in `copyright`. R8's rehearsal observable is in place.

**Targets list exact.** `jq -c '.bundle.targets'` -> `["msi","dmg","deb","rpm","appimage"]`
(compared as the JSON array, not through a `join`, so order and element count are both
covered). `icon` length 5, and all five paths resolve to files that are exactly the
contents of `src-tauri/icons/` (no sixth icon exists to have been omitted).
`licenseFile: "../LICENSE"` resolves to the repo-root `LICENSE` (present, 1072 bytes).

**`.gitignore` rule in the right block.** Line 16 `src-tauri/binaries/`, directly below
`src-tauri/gen/` (15), inside the `# JS/Tauri frontend` block (12-16), one hit total.
Attribution proven affirmatively, not by absence:
`git check-ignore -v src-tauri/binaries/muxsmith-x86_64-unknown-linux-gnu` ->
`.gitignore:16:src-tauri/binaries/`, with a negative control
(`src-tauri/PROBE-NOT-IGNORED` -> not ignored) showing the tool discriminates.
`git ls-files src-tauri/binaries` -> 0.

**Scope discipline.** `git diff --name-status 7e36f96..HEAD` = exactly the four briefed
files (`M .gitignore`, `M BUILDING.md`, `A src-tauri/tauri.bundle.conf.json`,
`M src-tauri/tauri.conf.json`); numstat 48/2 as reported. Non-bundle content of
`tauri.conf.json` is unchanged, proven canonically:
`jq -S 'del(.bundle)'` of base and HEAD blobs share sha256
`6a9dca8e6f963eb7ef9bc0c85935780966c6020291655c925a71101a1261885b`, and the control
(the same comparison over `.bundle`) fires. Top-level keys
`$schema,productName,identifier,build,app,bundle`; `productName` and `identifier`
unchanged per D86 :925.

**4.4 placement.** `BUILDING.md` :97-114, one blank line either side, last subsection of
`## Building and running` (`##` heading map: 57 -> children 70, 84, 97 -> next `##` at
116 `Tooling quirks`). Exactly as Step 4 specifies.

**D76/D86 absences, each with an affirmative control.** I made `tauri-build` enumerate
the accepted field sets, so the `has() == false` results below are meaningful rather than
name-typo passes: bundle level accepts `createUpdaterArtifacts` and `fileAssociations`
(FT-5 output), `.bundle.macOS` accepts `dmg`, `.bundle.linux` accepts exactly
`appimage, deb, rpm`. Against that: `createUpdaterArtifacts` false, `fileAssociations`
false, `macOS.dmg` false, `linux.appimage` false, `resources` false, `externalBin` false
(base config). `version` absent (D87).

**Design cross-checks the design itself asserts.** `longDescription` is
character-identical to `README.md:3` (1 hit, near-miss control 0 hits), so D86 :949
"the README's own tagline, reused not re-invented" holds. `windows.wix.upgradeCode`
matches the value D86 :951 pins (inside the byte-identical block). The rider's
`target/release/muxsmith` is correct: `crates/muxsmith-cli/Cargo.toml` declares
`[[bin]] name = "muxsmith"`, and `cargo metadata` confirms bin names `muxsmith` (cli) vs
`muxsmith-gui` (gui), i.e. D82 :641-642's no-collision premise holds in the tree.

**Preamble compliance.** Commit unsigned (`%G?` = `N`), repo trailer present, files
staged explicitly, no `ci.yml` contact, no new dependency, no tag/release/README
placeholder touched, ASCII typography throughout the added text with the D86-sanctioned
`Ş` exception in exactly the two config values and nowhere else.

## Verdict 2 - task quality: APPROVED

**`cargo check -p muxsmith-gui` is clean under a genuinely forced rebuild**
(`build-recheck-forces-rebuild`). I ran the warm control first to make the difference
visible: warm re-run = `Finished dev profile ... in 0.18s`, no `Compiling` line, no
diagnostics possible. Then `touch src-tauri/build.rs src-tauri/tauri.conf.json` (both are
`rerun-if-changed` inputs - the build script's own output names
`cargo:rerun-if-changed=.../src-tauri/tauri.conf.json`) -> `Compiling muxsmith-gui`,
exit 0, zero `^(warning|error)` lines. The report's numbers (0.49s, `Compiling` present)
reproduce within noise, so its forced-rebuild claim was real, not a warm-cache artifact.
`build.rs` mtime restored afterwards; content hashes unchanged.

**D87 state preserved.** `jq 'has("version")'` -> `false`; top-level key set unchanged
from Task 1's end state; no `version` key anywhere in the overlay;
`scripts/check-version-sync.sh` -> `version-sync: OK (0.1.0)`, exit 0, against
`Cargo.toml` :6 `version = "0.1.0"`. The rewrite did not silently reintroduce the key.

**Fire-tests reproduced independently (three of five, plus one probe the report did not
run).**

- **FT-3, the load-bearing one (overlay filename).** Same bytes, two names:
  `src-tauri/tauri.linux.conf.json` (sha256 identical to the overlay, verified) makes a
  forced `cargo check` exit `101` with
  `resource path 'binaries/muxsmith-x86_64-unknown-linux-gnu' doesn't exist`; with only
  `tauri.bundle.conf.json` present the same command exits `0`. So D82 :623-625's
  "the name deliberately avoids the auto-merged patterns, so it never applies implicitly"
  is verified at runtime with the installed tooling, and D82 :657-658's
  "byte-identical dev/test surface" rests on a check that demonstrably can fail. Probe
  file removed (`/usr/bin/ls src-tauri/tauri.*.conf.json` -> only the overlay), tree clean.
- **FT-5, unknown-field denial.** Injecting `.bundle.reviewerBogusProbe` -> exit `101`,
  `unknown field 'reviewerBogusProbe', expected one of ...`. The report's quoted
  enumeration is character-identical to my live output (412 bytes each, `diff` clean), so
  that quote holds up. Consequence, and it is the strongest quality point in this task:
  the clean `cargo check` is not merely "the JSON parses" - `tauri-build` denies unknown
  fields, so it independently validates every field name and type transcribed.
- **Extension the report asserted but did not test: nested denial.** The report claims
  the clean check also covers the `windows.wix` / `linux` / `macOS` sub-objects. That was
  a borrowed inference from top-level `deny_unknown_fields`, not a measurement. I tested
  it: `.bundle.windows.wix.reviewerNestedProbe` -> exit `101` with a wix-specific
  accepted set (`upgradeCode`, `language`, `template`, ...), and the same for `.macOS`
  and `.linux`. The claim is true; it is now measured.
- **FT-4 (ignore rule)** re-verified as described under Verdict 1, with the control.

**Report accuracy overall: high, one exception.** Verified true: the A1 analysis
(`d21a19f` is not an ancestor of the base; exactly 3 hunks; and independently, D82, D86,
section 3 and section 4.4 are byte-identical pre- and post-A1, with a firing control on
the amendment log); commit metadata and numstat; both committed blob hashes; the
`BUILDING.md` :66-68 quotation (whitespace-tolerant match, near-miss control 0 hits);
`LICENSE` 1072 bytes; the coverage grep behind concern 1 (no other brief mentions
`BUILDING.md` except the preamble's gate line). The exception is m1 below.

**Self-disclosure quality.** The FT-2 `PIPESTATUS`-under-zsh abort that left the config
mutated was reported unprompted, with the recovery and its verification, and is already
ledgered as `shell-dialect-assumptions-in-agent-scripts` (tier 1, `b8a1274`). Disclosing
a mid-fire-test abort that nothing else would have surfaced is the behaviour the process
wants; it costs nothing here because the committed text was re-verified from `git show`.

**The brief is a faithful extraction** of the plan's Task 2 section (`diff` = one blank
line), so grading against the design and grading against the brief coincide
(`proc-57-briefs-not-ground-truth` satisfied by construction rather than by assumption).

## Findings by severity

No Major findings. No fix round required; nothing below changes the landed artifacts.

**m1 (Minor, report-side, no artifact impact): the overlay-filename evidence block is
not the command's actual output, and the count derived from it is wrong.** The report
pastes a `grep -n 'tauri.bundle.conf.json'` listing with 8 lines and concludes "One name
at all eight sites". The file has 9 hits (`grep -c` on both the pre-A1 worktree copy and
the post-A1 main-tree copy). The missing site is section 11 "What the implementer must
not decide", post-A1 :2003 / pre-A1 :2002:
`  \`src-tauri/tauri.bundle.conf.json\` and its exact content, the sidecar`.
The conclusion survives (all 9 sites carry one spelling, verified), so this is evidence
fidelity, not a wrong decision. It matters because the number travelled: the review
dispatch for this task repeats "the design's eight sites" as fact. Note the contrast
that makes the handle in H1 concrete: in the very next section the report computed its
`.gitignore` hit count with `grep -c` and got it right. The hand-counted number is the
one that broke (`proc-normative-count-recomputed`,
`design-empirical-claims-reproducible`).

**m2 (Minor, controller-facing, concern-1 wording): "wrong on both halves" overstates;
only the first half is falsified.** `BUILDING.md` :66-68 reads "Building the desktop
bundle itself (`pnpm exec tauri build`) is out of scope for local development and not
part of the CI gate yet." Half 1 is falsified by this commit's own subsection. Half 2 is
still true after plan 8: `BUILDING.md` :92-95 defines "CI" as `ci.yml`'s nine parts,
`grep -c 'tauri build' .github/workflows/ci.yml` -> 0, Task 4 adds a separate
`release.yml` (not the gate), and Task 5's rider adds a lint job, not a bundle build. The
consolidated whole-branch item currently inherits the "wrong once 4.4 + release.yml land"
framing; the doc hunk should rewrite half 1 and decide half 2's wording deliberately
(it is defensible as-is, and arguably wants "not part of the CI gate" without "yet").

**i1 (Informational, design errata, no artifact change): D86 contradicts itself about the
upgradeCode comment.** D86's Interface-changes line (:1011-1012) says the upgradeCode is
"recorded as a comment beside the value"; section 3.1 (:1525-1527) states the opposite
and resolves it ("A JSON file cannot carry the ... warning as a comment; the warning
lives in this D86 and in the BUILDING.md subsection"). JSON has no comments, so the
Interface line is simply unexecutable. The brief pre-resolved it (:59) and the
implementer followed the brief, which is correct. Flagged so that a later reader does not
"repair" the missing comment, and as a plan-close errata candidate.

**i2 (Informational, ledger hygiene): the new tier-1 entry's occurrence ref points at the
wrong section of the report.** `shell-dialect-assumptions-in-agent-scripts` cites
"task-2-report.md concern 2" for the `PIPESTATUS` abort; the incident is documented in
the FT-2 "Process note", while the report's Concerns item 2 is the separate audit-trail
note. One-word ref fix at plan close.

**i3 (Informational, observation, routed to whole-branch triage): the shipped local-repro
command is executed by nothing in this plan, and the D86 behaviour it relies on is the
one uncited claim in that decision.** The 4.4 rider documents
`pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json` with no `--bundles`,
against a `targets` list of five formats of which two (msi, dmg) cannot build on a Linux
or macOS host. D86 :931-933 asserts "anyone running a bare `tauri build` locally gets
exactly the shipped formats for their OS" without a section-1 verification citation,
unlike its neighbouring facts, which all carry one. Section 8's R1-R10 does not run the
documented sequence (grep: the string "Reproducing a release bundle" appears only in the
design, plan and this task's brief), and `tauri-bundler`'s Rust source is not in the
local cargo registry, so I could not settle whether it silently skips non-host targets or
errors out. Honest state: unknown, not a defect claim. Exposure is bounded - all four CI
legs pass explicit `--bundles`, so R6's artifact checks are unaffected; the risk is a
developer following BUILDING.md verbatim and hitting an unsupported-target error. Cheap
resolutions, either is fine: one local run at plan close, or add the host's `--bundles`
value to the documented command. Design-verbatim text, so explicitly NOT a Task 2 defect.

## Q1 - was leaving `BUILDING.md` :65-68 untouched the correct scope call, or did the brief license the edit?

**Verdict: leaving it untouched was correct. The brief did not license the edit, and
`DONE_WITH_CONCERNS` with an explicit routing recommendation was the right channel.**

Steelman for the other side first, because it is genuinely strong:

1. `conventions.md`'s sweep duty is explicit that changing a standing statement obliges
   visiting the references that named the old one, and a contradicting sentence 30 lines
   above, in the same file the brief opens for modification, is the paradigm case. The
   commit knowingly ships a self-contradicting document.
2. House precedent exists for exactly this move: the plan-7 T3 occurrence under
   `latitude-carveout-zero-content-structural-forks` (2026-07-21) ruled an implementer's
   comment edit correctly permitted because "the en edit repaired a sentence the change
   itself falsified (sweep duty)".
3. The brief's Files entry names a file, not a byte range.

It loses on three independent grounds, any one of which is sufficient:

1. **The grant that carried that precedent is not wired into this plan.** The exemption
   lives in the brief wiring by design ("controller-authored and pre-decided, never
   implementer-judged at the keyboard"). `grep` over `implementer-preamble.md`, all five
   task briefs and the plan document finds no house-pattern grant clause (the single
   "house-pattern" hit is Task 5's unrelated pointer to `ledger-lint.py`'s docstring;
   control grep on a phrase that IS in the preamble returns 1, so the search works). No
   grant, no route.
2. **Even with the grant, it "fills SILENCE only - an explicit enumeration in brief,
   design or spec always wins over it", and here there are three closures.** The Files
   entry reads "Modify: `BUILDING.md` (one appended subsection)"; Step 4's verb is
   "Append ... insert it at the end of that section"; design 4.4's header is "(D82/D86,
   addition, complete)". The same ledger entry's brief-authoring rule settles the rest:
   an unmarked Files list is exhaustive, and this one carries no exemplary marking. A
   paragraph rewrite would have made the delta "one appended subsection plus one
   paragraph edit", contradicting the brief's own normative count.
3. **The correct fix is not task-2-sized, and the plan-7 T3 analogy breaks on exactly
   that point.** There the falsified sentence was about the edit's own subject, in the
   artifact being authored. Here the sentence's correct rewording depends on the
   post-merge end state of two streams this implementer cannot see (Task 4's
   `release.yml`, Task 5's `ci.yml` rider), and m2 shows the wording question is subtle
   enough that the implementer's own read of it is half wrong. Under
   `proc-latitude-clause-boundary`'s test - "must the implementer invent something it is
   not allowed to invent?" - a rewrite is invented prose about other streams' end states.
   Yes, therefore route. The controller's consolidation of T2 concern 1 with the T5
   review minor into one whole-branch doc hunk is the proof: the fix's natural unit spans
   streams.

**On the status choice.** `NEEDS_CONTEXT` would have been wrong. The preamble's
`NEEDS_CONTEXT` channel is for a fork discovered on code contact, i.e. a decision the
implementer must have made to proceed. Nothing here blocked: the task was fully specified
and completable as written, and the residue is a documentation-accuracy item outside the
permitted delta. Surfacing it in the report, with the coverage grep proving no other
brief owns the file, is precisely the right weight of response - and the coverage grep is
the part that made the controller's consolidation possible.

**One residue the finding shifts upstream.** The defect is in brief authoring, not in
execution: a brief that appends prose to an existing doc should enumerate the neighbouring
statements the addition falsifies, or name the deferral. Plan 8 produced two instances in
one file (:65-68 via T2, :92-96 via T5's review), both caught downstream rather than at
authoring. See H2.

## HARVEST

**H1 - Mechanize the count next to a pasted listing (`proc-normative-count-recomputed`
occurrence candidate).** A quoted command output with one line dropped reads exactly like
a faithful one, and the count someone eyeballs off it then travels as fact - here into
the review dispatch. Handle, trigger readable at the keyboard: when you write "N sites",
"N hits", "N occurrences" beside a pasted listing, the N comes from `grep -c` / `wc -l`
in the same block, never from counting the pasted lines. Evidence that the handle is the
right one rather than an exhortation: this same report computed its `.gitignore` count
with `grep -c` and got it right, and hand-counted exactly the number that broke. Cheap
reviewer-side counterpart, which is how m1 was found: re-run the report's own grep with
`-c` before reading its conclusion.

**H2 - A doc-append brief owes the sweep enumeration (candidate rider on
`proc-latitude-clause-boundary`'s brief-authoring side).** Trigger is readable at
brief-authoring time: the Files list contains a documentation file with an append-only
verb. Handle: grep the target file for statements naming the thing being added (here
`tauri build`, `CI gate`) and either fold the repair into the brief's permitted delta or
state the deferral in the brief. Plan 8 is two-for-two on this in a single file, both
caught downstream (T2 concern 1, T5 review minor), which is the argument for moving the
grep one level up rather than relying on implementers to surface it. This is
`conventions.md`'s sweep duty applied to brief authoring instead of to prose editing.

**H3 - A decision's summary/interface line is a dependency of its body, and drifts from
it.** D86's Interface-changes line survived an owner-approved four-eyes design review
while asserting something the same decision's own body explicitly rules out (i1: a
comment in JSON). Handle: when a decision block has both a body and a summary or
Interface-changes line, the summary is re-read against the body in the same pass, exactly
as `conventions.md` treats a restatement as a dependency rather than a duplicate. Note
what saved this task: the brief pre-resolved the contradiction in its Step 1 prose. A
brief that had said only "design 3.1 verbatim" would have handed the implementer a fork.

**H4 - Over-restriction-watch calibration data for
`latitude-carveout-zero-content-structural-forks`, do-not-loosen direction.** The stop
this boundary forced here was correct on the merits (Q1, ground 3: the fix's natural unit
spans two other streams). Separate observation the watch should record alongside it:
plan 8's standing wiring carries no house-pattern grant clause at all, although the
ledger entry states the exemption "lives in the BRIEF". For a config-and-CI plan with no
structural code work that omission is plausibly deliberate, but it is worth a controller
yes/no rather than being rediscovered per plan - a doc-sweep repair had no route except
surfacing, and next time the missing route may cost more than one report paragraph.

**H5 - Reviewer method note: turn a borrowed inference into a measurement when the probe
is one line.** The report's "the clean check validates the sub-objects too" was sound
reasoning from top-level `deny_unknown_fields`, but unmeasured. Injecting a bogus field
into `.bundle.windows.wix`, `.macOS` and `.linux` cost three `cargo check` runs and
returned the accepted-field enumerations, which simultaneously (a) confirmed the report's
claim and (b) supplied the affirmative controls that make the four D76/D86 absence
assertions meaningful. Reusable shape for any serde-backed config: make the parser
enumerate its own accepted set, then assert absence against that list.
