# Task A3 verdict - Plan 11 (independent review)

**Verdict: APPROVED_WITH_MINORS.**

No fix round is required and no product file changes. The three findings below are
recorded items for the plan close and the controller's tracker; none touches the
shipped artifact, and all three were established by measurement rather than by
reading.

Reviewed: `164e571` over `5d305a2` in `/home/senol/Git/muxsmith-plan11-a`, branch
`plan-11-stream-a`. Ground truth: D111
(`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`, sha256
`cb112f60...`, byte-identical in the worktree and at its authoring commit `e0c9d2b`),
then the v1 spec, then the plan in the worktree. `/home/senol/Git/muxsmith-plan11-b`
untouched (verified at `c422999`, clean).

**Tree identity at the end of this review:** `git status --porcelain` = 0 lines,
`git diff --exit-code 164e571` exit 0, `git diff --exit-code HEAD` exit 0. Every
mutation ran on a copy of the crate under
`.../a3rev-independent/cratecopy` with its own `CARGO_TARGET_DIR`; no worktree file
was written at any point.

---

## 1. What was established, dimension by dimension

### 1.1 The comparator pair, the call site, and the twelve replacements

**Reconstruction, not inspection.** All six end-state files were rebuilt from their
`5d305a2` blobs plus only the D111 fences (plus the plan's Step-6 `pattern` line for
`README.md`, which is fenced in the plan, not in D111), and byte-compared against the
`164e571` blobs. **All six reconstruct exactly.**

```
OK   README.md          OK   report/mod.rs     OK   help/en
OK   v1 spec            OK   matcher.rs        OK   help/de
ALL RECONSTRUCTED: True
```

That is the character-for-character comparison against D111 itself, and it is
stronger than a site-by-site read: it also proves **no line outside the fences moved**
in any of the six files. Combined with `git diff --name-only 5d305a2..164e571`
returning exactly those six paths, the whole diff is accounted for by D111.

**Per-site uniqueness precondition, checked per site** (dimension 2's precondition,
which a file-level identity check on `matcher.rs` or the v1 spec cannot supply):

| site | OLD in pre-state | NEW in end state |
|---|---|---|
| R-1 ... R-9, R-11 | 1 each | 1 each |
| R-10 (§3.2 fence) | - | 1 |
| §3.3 call site | - | 1 |
| T-1 / T-2 (R-12) / T-3 (§5 fences) | - | 1 each |

R-4's "FIRST occurrence only" is satisfied structurally: its fenced OLD occurs
**once** in the pre-state spec, because the retained second occurrence on the same
line is a different string.

**One anchoring hazard, measured and correctly handled by the implementer:** the
§3.3 call-site line `            Some(have) => scalar_eq(want, &have),` occurs
**twice** in the pre-state `matcher.rs` (`:103`, the `raw:` arm, and `:138`, the typed
default arm), byte-identical including indent. A naive apply of §3.3 alone is
ambiguous. My reconstruction therefore anchored it to R-2's replacement block
immediately above, and the result reconstructs, i.e. the re-pointed line is the
`raw:` one.

**The call-site split, established directly** (`matcher.rs`, end state):

```
109:            Some(have) => scalar_eq_same_type(want, &have),      <- raw: arm
144:            Some(have) => scalar_eq(want, &have),                <- typed default arm
146:                Some(PropType::Boolean) => scalar_eq(want, &PropValue::Bool(false)),  <- Boolean shortcut
```

`scalar_eq` retains **both** cross arms, expressed as
`scalar_eq_same_type(want, have) || match { two cross arms }`, verbatim per §3.2.

### 1.2 The retained set, per site

K' returns **7 lines across 6 files on the pre-state AND the end state**, member for
member as D111 §4.4 lists them. The two that needed the hardest look are both proven
by the reconstruction rather than by eye:

- **v1 spec `:421`, second occurrence** - R-4 edits the first occurrence in the same
  long line; the retained clause "that `raw:` degrades to byte-literal equality"
  survives verbatim in the end state, and the whole file reconstructs from R-3/R-4/
  R-5/R-6 alone.
- **`matcher.rs`'s two scoped sentences** (`:452`/`:466` pre, `:533`/`:547` post) -
  neither is inside any fence, and `matcher.rs` reconstructs, so they are provably
  byte-identical while the file legitimately moved.

The remaining five retained sites live in files the diff does not touch at all
(`profile/validate.rs`, `tests/validate_semantics.rs`, both `locales/*.ftl`).
`b8_raw_language_is_byte_literal_no_normalization` retained by name.

**K's fire, run on the copy:** deleting the ` and matches byte-literally instead`
clause from `locales/en/diagnostics.ftl` moves K' from **7 to 6**; restored, back to
**7**, restore proven by `sha256sum` identity with the live worktree file
(`afea4505...`).

### 1.3 The three tests, and the two mutations

Both mutations reproduced independently, on the crate copy, `cargo test --workspace
--no-fail-fast` (so the roll-up is workspace-wide, not just the lib target whose
failure would otherwise stop the run):

| mutation | failing tests | count |
|---|---|---|
| strip the two cross arms from `scalar_eq` (the A-2 defect) | `matcher::tests::typed_exact_still_cross_compares_int_and_float` | **1** of 124 lib tests, 1 workspace-wide |
| revert the `raw:` call site to `scalar_eq` | `b7_raw_does_not_cross_compare_int_and_float`, `raw_compares_only_within_one_kind` | **2** |
| *(third mutation, mine)* re-point the **typed default arm** at `scalar_eq_same_type` | `typed_exact_still_cross_compares_int_and_float` | 1 |

Cleanly disjoint, exactly as the implementer reports. **T-1 is a live safeguard, and
it is the SOLE guard** for the typed path's cross arms - which is the single most
expensive mistake this task had available. Baseline before and after every mutation:
507 workspace tests, 0 failures (124 of them the `matcher` lib target's); restore
proven by sha256 against the worktree file each time.

**T-3's matrix against the absolute it quantifies over.** T-3's stated absolute is
"a profile value equals a reported value only within one kind", i.e. over pairs of
*present* values: the 4x4 loop walks all 16 and asserts `i == j`, so the enumeration
is complete for that absolute. D111 §3.1's table has a fifth column (**absent**),
which T-3 does not walk; that column is explicitly marked "Unchanged (case B-6)" in
§3.1 and its code (`None => false`) is byte-identical across the diff, so it is
outside T-3's absolute and outside this package's test duty. Partial coverage there
(B-6 covers `Bool` only) is a HARVEST line, not a defect.

### 1.4 The behaviour change end to end, through the shipped binary (SI-3)

Binary rebuilt before probing (`cargo build -p muxsmith-cli` in the copy;
`find crates src-tauri -name '*.rs' -newer <bin>` returns nothing). Probes muxed
outside the repository from the repo's own `tone.wav` seed plus an ffmpeg-generated
video; `mkvmerge v100.0`, `identification_format_version: 20`.

**Reported tokens re-measured, not assumed:** `--max-luminance 0:400` comes back as
`"max_luminance": 400.0`; `0:6.00` comes back as `6.0`; `0:400.500` as `400.5`;
`audio_channels` as `1`. D111's F3/P3 hold: mkvmerge canonicalizes the decimal text
and still writes a double with a fractional part, which is what makes the
`(Scalar::Int, PropValue::Float)` direction reachable from an ordinary file.

| profile rule | reported | **end state** | **counterfactual: pre-change binary** |
|---|---|---|---|
| `raw:audio_channels: 1.0` | `1` (Int) | `missing-track`, 0 assignments, exit 2 | **matched**, 1 assignment, exit 1 |
| `raw:max_luminance: 400` | `400.0` (Float) | `missing-track`, 0 assignments, exit 2 | **matched**, 1 assignment, exit 1 |
| `raw:audio_channels: 1` | `1` | matched, 1 assignment, exit 1 | matched, 1 assignment, exit 1 |
| `raw:max_luminance: 400.0` | `400.0` | matched, 1 assignment, exit 1 | matched, 1 assignment, exit 1 |
| `raw:audio_channels: 2` (control) | `1` | `missing-track`, exit 2 | `missing-track`, exit 2 |
| `raw:max_luminance: 401.0` (control) | `400.0` | `missing-track`, exit 2 | `missing-track`, exit 2 |

The counterfactual column is the external verifier: I rebuilt the CLI from the
**`5d305a2` `matcher.rs`** in the same copy and re-ran the identical six profiles.
Both cross rows flip and nothing else moves, so the probes measure this diff and not
an artifact of the setup.

### 1.5 The four retirement checks, re-run with my own instruments

- **R'** (two invocations, summed): pre-state **8 lines across 6 files** (5 + 3),
  member for member as D111 lists them; end state **0 + 0**.
- **R' soundness control** (drop `':!docs/ROADMAP.md'`): pre-state **9** (6 + 3), end
  state **1**, the survivor being `docs/ROADMAP.md:1922`. It fires - see adjudication 3.
- **K'**: 7/6 on both states, with its fire run (1.2 above).
- **Vocabulary sweep, alternation-free** (`-nE 'byte'`): **71** on the pre-state, 66
  on the end state. All **19** classified sites are present in the 71 (verified
  member for member by set difference, not by tally). The remaining **52** were read:
  byte arrays, byte sizes, encodings, multi-byte UTF-8 slicing, and
  `byte-identical`/`byte-for-byte` about documents, snapshots and rendered output.
  **No hit is a false unscoped claim about `raw:`**, so R' has no hole.
  One line ENTERS the sweep with this change: `matcher.rs:210`, R-10's
  "Strings compare byte-wise, `language` is not normalized here". It is inside
  `scalar_eq_same_type`'s doc, scoped to strings, and true per §3.1 ("the one place
  where byte-for-byte is precise"). Not a finding.
- **R''** (the loosened, newline-flattening form): **0 strict / 0 loose** on
  `README.md`, `matcher.rs`, `report/mod.rs`, both help topics; **0 strict / 2 loose**
  on the v1 spec, the two being R-3's and R-4's replacement texts, which D111 §4.6
  already measured and ruled PERMITTED (a bracket and a colon respectively separate
  the path sense of "matched untyped" from the equality defined after it). My run
  reproduces the implementer's exactly.

### 1.6 The German half is not a translation of the English half

Both `## The raw: bypass` / `## Der raw:-Bypass` paragraphs read in full. They now say
the same thing about the same mechanism: same negation list, same rule ("a value
matches only a reported value of the same kind" / "ein Wert matcht nur einen
gemeldeten Wert derselben Art"), same worked example (`6` vs `6.0`), and the same
back-reference - and each back-reference resolves to a real heading **in its own
file** (`## Typed equality` / `## Typisierte Gleichheit`). R-9 reuses the German
topic's established vocabulary (`Typumwandlung`, `typisierte Gleichheit`) rather than
coining a translation, as D111 requires. Both files keep their line-11
regex sentence unchanged, which is one of the two true-about-`regex` sweep members.
`pnpm check:i18n` green (`22 help id(s) x 2 help locale(s)`), but that gate checks
completeness, not agreement - the agreement above is a read, not a gate result.

### 1.7 Diff scope and interface invariance

Six files, exactly the Files list. Verified `UNCHANGED` between `5d305a2` and
`164e571`: `locales/`, `profile/validate.rs`, `tests/validate_semantics.rs`,
`profile/match_expr.rs` (`Scalar`), `identify.rs` (`PropValue`, `from_json`),
`capability/` (`scalar_fits`, `matchable_type`, `generated.rs`), `src/bindings/`,
`src/`, `e2e/`. The `diag_codes!` entry set is **identical** pre and post (47 codes,
diff empty), and the `report/` diff contains **zero** lines mentioning severity. No
`DiagCode` added, widened or re-severitied; no Fluent key added; no message value
moved.

### 1.8 Exit bars, re-run by me on the copy

`cargo fmt --all --check` exit 0; `cargo clippy --workspace --all-targets -- -D
warnings` exit 0; `cargo test --workspace --no-fail-fast` 0 failures (which is also
the snapshot-invariance evidence D111's M4 predicts); `RUSTDOCFLAGS="-D warnings"
cargo doc --workspace --no-deps --document-private-items` exit 0, **fired** by
breaking `[`scalar_eq_same_type`]` to `[`scalar_eq_same_type_nope`]` (exit 101,
`unresolved link`), restored (sha256 match, exit 0). `pnpm check:i18n` exit 0.

### 1.9 The README example and its corpus

`muxsmith validate` on extracted scratch copies, my binary:
`PRE_readme_1` **exit 2** with ``missing field `pattern` at line 4 column 3``;
`POST_readme_1` **exit 0**, `Profile is valid.`; the passthrough example exit 0 on
both states; the v1 spec example exit 0 on both. Corpus re-derived independently -
see adjudication 4.

### 1.10 House dimension

- **`core-72-exact-typed-value-equality`** (`docs/conventions.yaml:325`) - its core
  semantics ("numbers numerically") is now guarded by a named test, and I measured
  that T-1 is its **only** guard. Preserved, not weakened.
- **`tests-ship-with-the-feature-never-after`** - see adjudication 5.
- **`proc-proposed-safeguard-stays`** - T-1 was proposed by D111 and shipped, not
  argued out. It was then **measured** (mutation 1), which is the only route this
  rule permits for a later removal decision. Correct handling.
- **`proc-06-mkvtoolnix-parity` / `testing-si3-run-binary`** - discharged with my own
  runs, not cited: seven `mkvmerge -a <sel>` invocations against a probe muxed
  `--language 0:ger` reproduce D111 §10 exactly (`0`->1, `1`->0, `ger`->1, `eng`->0,
  `de`->1, `deu`->1, `de-DE`->0). Numeric literal never tried against the language;
  language normalized; meaningful distinctions preserved. The reference tool compares
  strictly within a type, which corroborates the ruling.

---

## 2. Findings

### Finding 1 - Minor. D111's M4 instrument has two dead pathspec members; the conclusion survives, the evidence does not

**Site:** `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md:192` (M4's
fenced command) and `:206` (its stated fire).

**Evidence I ran.** M4's pathspec is
`-- 'crates/**/tests' 'crates/**/*.snap' 'crates/**/fixtures' 'e2e' 'src' 'src-tauri'`.
Under git's pathspec rules (git 2.55 here), a pathspec containing a wildcard is
fnmatch'd against the **whole path** and loses the leading-directory behaviour a
literal pathspec has - so `crates/**/tests` matches only a path that literally *ends*
in `/tests`, and never a file under such a directory. Fired against a known-present
case:

```
$ grep -c "raw:x" crates/muxsmith-cli/tests/dry_run_cli.rs          -> 2
$ git grep -c "raw:x" -- crates/muxsmith-cli/tests/dry_run_cli.rs   -> 2
$ git grep -c "raw:x" -- 'crates/**/tests'                          -> (nothing)
$ git grep -ln "raw:" -- 'crates/**/tests'                          -> (nothing)
```

So `crates/**/tests` and `crates/**/fixtures` contribute **zero** files; only
`crates/**/*.snap`, `e2e`, `src` and `src-tauri` are live. D111's stated fire
("**The instrument demonstrably reaches those files:** it returned snapshot content,
and the control `grep -rln "exact" crates/muxsmith-core/tests/` returns 18 files")
does not close this: the first half fired on the `*.snap` member, and the second half
used a **different tool** (`grep -r`), so neither touched the two dead members. This
is the enumerated-set-inside-the-instrument class - the fire test passes against a
present member while a dead member stays invisible.

**Impact on the artifact: none.** I re-derived M4's conclusion tree-wide with a plain
recursive grep and it holds. The only numeric `raw:` literals outside `matcher.rs`
are `crates/muxsmith-cli/tests/dry_run_cli.rs:339` and `:418`
(`- match: { exact: { raw:x: 1 } }`) and `crates/muxsmith-core/tests/
validate_semantics.rs:211` (`raw:dolby_complexity_index: 3`). Read: the two CLI ones
are config-diagnostic **ordering** tests run against a source directory holding no
media, so the matcher never performs the comparison; the core one is a validate-time
test that never runs the matcher. And `cargo test --workspace` is green, which is the
decisive measurement. So "no snapshot, fixture or integration test asserts a numeric
`raw:` comparison" is TRUE - it was just verified with an instrument that could not
have seen two of its six selectors.

**Required change:** none in any product file, and none in D111 (an approved ADR is
not edited for this). The plan close records the pathspec defect so no later sweep
reuses the expression, alongside T12's rule-not-list duty.

### Finding 2 - Minor. A pre-existing test's NAME claims the coverage T-1 exists to provide

**Site:** `crates/muxsmith-core/src/matcher.rs:393-397`,
`fn numeric_exact_compares_across_int_and_float()`.

**Evidence I ran.** The body is:

```rust
let t = track("audio", &[("audio_channels", PropValue::Int(6))]);
assert!(matches(&expr("exact: { audio_channels: 6 }"), &t, &lang()));
assert!(!matches(&expr("exact: { audio_channels: 2 }"), &t, &lang()));
```

Both assertions are `Int` against `PropValue::Int` - **it never crosses int and
float**. Measured: mutation 1 (strip both cross arms from `scalar_eq`) leaves it
**green**; the only failure workspace-wide is T-1. So a test named
`numeric_exact_compares_across_int_and_float` would pass with the mechanism it names
deleted, and it sits in the same typed-path group as the safeguard written to catch
exactly that (`:393` and `:440` in the end state).

This is a proof that would pass if the mechanism were broken: documentation, not
coverage. It is pre-existing (present since Plan 2) and outside A3's fenced-
replacement discipline, so the implementer was right not to touch it - but it is
absent from D111, from the task brief, and from the report's Step-9 surfacing list.

**Required change:** none in A3. The close carries it to the controller as a
harmonization item: either rename it to what it measures
(`numeric_exact_compares_int_against_int`) or widen its body to the cross case, with
the note that T-1 already covers the cross case so the rename is the cheaper repair.

### Finding 3 - Info. A no-work-needed premise is factually wrong; its conclusion stands on the other ground stated beside it

**Site:** `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md:1132` -
"An automated e2e would add an mkvmerge dependency for coverage the unit tests
already give."

**Evidence I ran.** The workspace already carries mkvmerge-gated integration tests
behind `have_mkvmerge()` / `MKVMERGE_SKIP_MARKER` in **eight** files across both
crates (`crates/muxsmith-core/tests/{identify_live,executor_live,mkvmerge_runtime,
command_integration}.rs`, `crates/muxsmith-cli/tests/{run_live,run_cli,dry_run_cli}.rs`,
`crates/muxsmith-core/src/lib.rs`). The dependency exists and is gated, so adding one
more gated case would add none. The conclusion nevertheless holds on the *other*
ground D111 states in the same bullet and which I ran: the wiring from profile
through identify to the matcher is unchanged by this diff, and the end-to-end
evidence is the four binary probes - which I reproduced **with a counterfactual**
(1.4), the strongest form available.

**Required change:** none. Recorded so the premise is not reused as-is next time a
package argues an e2e away on the same words.

---

## 3. The five adjudications

### Adjudication 1 - test placement: is the unenumerated placement a latitude-by-omission defect in D111, and is the chosen placement right?

**Yes to the first, in the weak form; yes to the second.**

It IS a latitude-by-omission gap. D111 fences the three test bodies verbatim, and the
plan's "Must not decide" list names "the three tests, their names and their bodies" -
placement appears in neither. Every other dimension of this task was fixed to the
character, so an implementer meets exactly one open choice, in a document whose whole
design philosophy is that open choices get named. That is the shape D111 itself
polices elsewhere (§4.1's rule-not-list, §13.1's assertion set), applied one level
down and missed.

It is a **benign** instance, and that matters for what the close should do with it.
Moving a `#[test]` function inside a `tests` module is behaviour-neutral by
construction: the failure mode is readability, never correctness, and the exit bars
would not have hidden a wrong choice from a reader. So the correct disposition is a
note about the design's omission class, not a fix and not a fold-in.

**The chosen placement is right**, on each of the three:

- **T-1** at `:433-460`: last test of the **typed-path** group, immediately after
  `present_boolean_property_still_matches_its_real_value` and immediately before the
  `// D32 / Task 16: raw: opt-in matcher cases B-5..B-8 (untyped comparison)` section
  comment. T-1 guards the **typed** path, so putting it inside the `raw:` group would
  have been the wrong signal; putting it in the typed group directly above the
  boundary is the strongest available placement, because the boundary comment is what
  a future reader crosses on the way to the `raw:` tests.
- **T-2** at `:499-530`: b7's position, **mandated** by R-12 ("replacing
  `b7_raw_int_float_cross_compare` at `matcher.rs:444-449` in name, comment and
  body"). No latitude here at all.
- **T-3** at `:553-587`: closes the `raw:` group after b8. This is the choice most
  worth checking, because the section comment scopes the group to "B-5..B-8" and T-3
  is not a B-case. It reads correctly: T-3 sits **after** the last B-case, so the
  comment's enumeration still describes exactly the run it heads, and T-3 reads as the
  group's closing generalization - which is what it is. The alternative (before b5)
  would have broken the B-5..B-8 sequence the comment announces.

The implementer surfacing the gap rather than deciding silently is the behaviour the
process wants, and it is why this is an adjudication and not a finding.

### Adjudication 2 - R'' was run over SIX files where the plan says five. Correct, or does a differently-scoped check need routing before it runs?

**Widening was correct, and it did not need routing before it ran.** Three grounds,
in order of weight:

1. **The widened surface is a strict SUPERSET of the specified one.** The routing duty
   exists for a scope decision that could go either way and whose resolution the
   agent's own convenience colours. A run that can only add evidence and cannot
   suppress any is not that decision: the specified five could not have produced a
   result the six hide.
2. **The specified run is still readable on its own.** The report gives the five
   first, with their own TOTAL, and then labels the sixth explicitly as an addition
   with its own result. Nothing is merged into a single figure, so a reader can
   reconstruct exactly what the plan asked for.
3. **No new judgment was exercised.** The two loose hits in the sixth file are
   verbatim R-3's and R-4's replacement texts, which D111 §4.6 already measured and
   ruled PERMITTED. The implementer applied an existing ruling; it did not make one.

**The inverse, stated because it is the boundary and it should be in the close:** had
the sixth file produced an **unadjudicated** hit, the correct move would have been to
report it and route it, not to rule on it at the keyboard - R'' is explicitly "a
candidate finder, not a verdict", so a new candidate is a fork, and the discriminator
that resolves it (§4.2) is on the "must not decide" list.

**Also worth recording:** "five" is itself stale rather than a live constraint. D111
§4.6 was written while A3's Files list was five; §13.5 then made it six by adding
`report/mod.rs`, and §4.6's sentence was not swept. The plan's Step 8 copied the stale
figure. That is the same enumeration-goes-stale class T12 already names, one document
inward - a count over a set that a later section changed.

### Adjudication 3 - the drifted ROADMAP citation inside D111: does the control still discriminate, and is citing-by-wording sufficient?

**Yes and yes.** Measured myself:

```
pre-state, ROADMAP exclusion dropped, both invocations summed : 9   (6 + 3)
end state,  same                                              : 1
the survivor: docs/ROADMAP.md:1922
  "`raw:` arm call the comparison an untyped byte-literal value equality, and the"
```

D111 prints `:1913` in two places (its header re-verification note and §4.6). At this
tree the hit is at `:1922` - drifted by nine, exactly as D111 anticipated when it
wrote that a co-writer edits that file.

**The control still discriminates**, and the line number carries none of its work. The
control's job is to prove that R''s instrument reaches a file R' excludes and reports
a match there - i.e. that R''s empty end-state result is a *measurement* rather than a
pathspec that points at nothing. That is established by 9-versus-8 on the pre-state
and 1-versus-0 on the end state; the delta is precisely one ROADMAP line, present in
both states. A wrong line number cannot make that delta appear or disappear.

**Citing by wording is sufficient here, and is the better citation**, because the
wording is what identifies the hit and it is byte-identical (verified: the quoted
fragment matches `:1922` exactly). D111's own Tier-2 rule forbids a document citing
line numbers inside itself, on the reasoning that an update duty is a rule requiring
someone to *notice*; the identical reasoning applies to a line in a file a co-writer
edits, and this drift is that rule proving itself again in the neighbouring case.

**What the close should record:** the two `:1913` citations in an owner-approved ADR
are now stale, and a reviewer re-running the control meets `:1922`. The ADR is not
edited (append-only); the controller's T6 duty on the ROADMAP entry is where the note
belongs, so the next reader is not misled into thinking the control moved.

### Adjudication 4 - the corpus discriminator is a grep where it should be a parse. Is this task's derivation sound anyway, and what does the finding oblige the close to record?

**The derivation this task performed is sound.** I re-derived it independently
(`corpus.py`, YAML-parsing, run over both states) and reproduce it exactly:

```
PRE : 6 fenced yaml/yml blocks | 3 standalone profiles | 3 fragments
      README.md:28-50   parse:pattern=False   <- the one defective profile
      README.md:79-84   parse:pattern=True
      v1 spec:54-116    parse:pattern=True
      fragments: plan-6 x2 (rule-list snippets), plan-8 x1 ("name: release")
POST: 6 | 3 | 3, all three profiles carry input.pattern
PROBE 1 (a block declaring `input:` at column 0 without `profile_version`): 0
```

Exactly one profile lacked `pattern`; **delta against the ruling's named site: zero**;
discriminator (`profile_version` at column 0) applied and its blind-spot probe run.

**I also reproduced the divergence the implementer reports**, by running both methods
side by side on every block:

```
README.md:79-84 (pre) / :80-85 (post)   parse:pattern=True   linegrep:pattern=False   <-- METHODS DISAGREE
```

The passthrough example writes its `input:` as an inline flow mapping -
`input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }` - so a
line-shaped `^\s+pattern:` test cannot see it and reports a correct profile as
defective.

**What the finding obliges the close to record**, precisely:

1. **The failure direction is a FALSE POSITIVE**, not a miss - a correct profile
   reported defective. In the owner-scheduled example-validation vehicle that is a red
   build on correct content, which is worse than a silent pass because it trains the
   reader to ignore the gate.
2. **The discriminator for "does this profile carry `input.pattern`" is a YAML parse
   of the block**, never a line grep. That belongs on the vehicle's design question
   next to the fragment problem already recorded (the corpus contains fragments, so
   run-everything fails on correct content) - both are the same shape: a naive
   instrument encoding an assumption the corpus falsifies.
3. **The class**, so it generalizes past this one vehicle: a line-shaped test over
   YAML/JSON is an assumption about surface syntax, and block-vs-flow mapping is the
   assumption that breaks first. Same family as Finding 1's pathspec and as T12.

It affects **nothing in A3**: the derivation actually performed parses, and both its
pre-state and end-state figures were reproduced here independently.

### Adjudication 5 - `tests-ship-with-the-feature-never-after`: is the shipped test set sufficient for the behaviour this task introduces?

**Sufficient.** Walked as halves, not as the observable:

| half of the user-visible consequence | producing test | measured? |
|---|---|---|
| `raw:`, profile `Int` vs reported `Float` -> stops matching | T-2 assertion 1; T-3 cell (Int, Float) | mutation 2 fails T-2 and T-3 |
| `raw:`, profile `Float` vs reported `Int` -> stops matching | T-2 assertion 3; T-3 cell (Float, Int) | same |
| `raw:`, `Int` vs `Int` -> still matches | T-2 assertion 4; T-3 diagonal; B-5 | green throughout |
| `raw:`, `Float` vs `Float` -> still matches | T-2 assertion 2; T-3 diagonal | green throughout |
| `raw:`, `Str`/`Bool` diagonals and all 12 off-diagonals | T-3's 4x4 loop | 16 assertions, `i == j` |
| typed path, `Int` vs `Float` -> still matches | T-1 assertion 1 | mutation 1 fails T-1, **and nothing else** |
| typed path, `Float` vs `Int` -> still matches | T-1 assertion 2 | same |
| typed path's cross arms compare VALUES, not "any number matches any" | T-1's negative control | present |
| the typed **default arm** still routes to `scalar_eq` | T-1 (transitively) | mutation 3 (mine): T-1 fails |
| end to end through the shipped binary, both directions | not automated - by design | SI-3 probes + counterfactual (1.4) |

The rule's own trigger is readable and **does not fire here**: nothing in the report
or D111 writes "no producer exists", "coverage rides a later item" or "tests follow
in <plan>" about behaviour this package introduces. The three "tests deliberately NOT
added" are each about behaviour this package does **not** introduce, and each premise
was run rather than weighed (Findings 1 and 3 record where a premise's *evidence* was
thin, in both cases without disturbing the conclusion).

**The one half with only partial coverage, named so it is not mistaken for complete:**
§3.1's **absent** column asserts false for all four kinds; B-6 covers `Bool` only, and
`Int`/`Float`/`Str` against an absent `raw:` property have no test. This is **not** a
violation of the rule, because the absent branch (`None => false` in the `raw:` arm)
is byte-identical across the diff - the package introduces no consequence there. It is
a HARVEST line: T-3 could close it at zero cost by extending its loop with an
absent-property row, and that is the cheapest place for it if anyone wants §3.1's
table fully walked.

**The inverse direction the question points at** - three tests shipping where the
pre-amendment position was that none was owed - is handled correctly and for the right
reason. The old position rested on `b7_raw_int_float_cross_compare` covering the
behaviour; that test is the assertion being **inverted**, so it was never coverage for
the new behaviour, and T-1 additionally guards behaviour this task deliberately does
NOT change, which is precisely the case `proc-proposed-safeguard-stays` exists for.
Mutation 1 turns that from an argument into a measurement.

---

## 4. Evidence appendix

**Instrument directory** (all mine; nothing here was written by the implementer, and
no shared default path was used):
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a3rev-independent/`

| path | what it is |
|---|---|
| `extract_fences.py` | pulls the R-n OLD/NEW pairs out of D111 §4.3 by CONTENT anchor, plus the §3.2/§3.3/§5 Rust fences; prints the pairing so a mis-pair is visible |
| `reconstruct.py` | rebuilds all six end-state files from the `5d305a2` blobs + fences only; asserts `count(OLD) == 1` at every substitution |
| `rpp.py` | check R'' (strict + the loosened newline-flattening form) |
| `corpus.py` | independent corpus derivation, parse and line-grep side by side, plus PROBE 1 |
| `pre/`, `post/` | `git show`-extracted blobs of the six files at `5d305a2` / `164e571` |
| `D111.md` | `git show e0c9d2b:` copy, sha256-identical to the worktree file |
| `cratecopy/` | full source copy (no `target`, no `node_modules`, no `.git`) - **every mutation ran here** |
| `mut-target/` | private `CARGO_TARGET_DIR` for the copy |
| `matcher.rs.pristine`, `diagnostics.en.ftl.baseline` | restore baselines, captured from the CURRENT content, restored with `command cp -f`, each restore proven by `sha256sum` against the live worktree file |
| `probe/` | mkvmerge probes, six profiles, `runprobe.sh` |
| `sweep_pre.txt`, `sweep_post.txt`, `classified.txt` | the 71/66 sweeps and the 19-member classification used for the set difference |

**Commands whose result is load-bearing** (all foreground, all absolute paths):

```
python3 reconstruct.py                                   -> ALL RECONSTRUCTED: True
git grep -nE '<R alternation>' 5d305a2 -- . <9 excludes> -> 5 ;  ... v1 spec -> 3   (R' pre = 8/6)
git grep -nE '<R alternation>' -- . <9 excludes>         -> 0 ;  ... v1 spec -> 0   (R' end = 0)
   same, ':!docs/ROADMAP.md' dropped                     -> 9 pre / 1 end  (control fires)
git grep -nE '<K alternation>' both states               -> 7/6 on both;  fire 7 -> 6 -> 7
git grep -nE 'byte' both states                          -> 71 pre / 66 end; 19 classified verified present
python3 rpp.py <six files>                               -> 0 strict; 2 loose, both permitted
cargo test --workspace --no-fail-fast   (copy, baseline) -> 507 tests, 0 failed
   + mutation 1 (strip cross arms)                       -> 1 failed: typed_exact_still_cross_compares_int_and_float
   + mutation 2 (revert call site)                       -> 2 failed: b7_raw_does_not_cross_compare_int_and_float,
                                                                      raw_compares_only_within_one_kind
   + mutation 3 (typed default arm, mine)                -> 1 failed: typed_exact_still_cross_compares_int_and_float
cargo fmt --all --check / clippy -D warnings / cargo doc -> 0 / 0 / 0 (doc fired: 101 on a broken intra-doc link)
pnpm check:i18n                                          -> 0
<copy>/mut-target/debug/muxsmith dry-run --json  x6      -> end state and pre-change counterfactual, table in 1.4
mkvmerge -a {0,1,ger,eng,de,deu,de-DE}                   -> 1,0,1,0,1,1,0  (D111 section 10 reproduced)
git status --porcelain ; git diff --exit-code 164e571    -> 0 lines ; exit 0
```

**Negative results that were fired before being trusted**, per the house rule that a
check whose passing result is an absence is only evidence once it has been made to
produce output: R' end-state 0 (fired via the ROADMAP control, 9/1), K' invariance
(fired by deletion, 7->6), R'' zero-strict (fired against R-2's pre-fix text -> 1),
`cargo doc` clean (fired by breaking an intra-doc link -> 101), the corpus PROBE 1
zero (the same scanner reports 3 fragments and 3 profiles on the same pass, so it
demonstrably reaches blocks), and the M4 tree-wide absence of a numeric `raw:`
assertion (fired against `validate_semantics.rs:211`, a known-present member - which
is how Finding 1 surfaced).

---

## 5. HARVEST

**For Task A4** (edits the same v1 spec, section 8.1):

1. A3 changed the v1 spec at `:146`, `:176`, `:280` and `:421`. A4's Files list says
   "nothing else in the file, and in particular not the sections Task A3 amended" -
   those are the four lines to stay off. `:421` is section 9.2 and now carries **both**
   a repaired first occurrence and a retained second one in the same line; touching
   that line at all is a defect.
2. A4 inherits a spec whose R'/K'/R'' state is 0 / 7 / (0 strict, 2 permitted loose).
   If A4's own checks run over the spec, those are the baselines.
3. The reconstruction method used here (rebuild the end state from the base blob plus
   only the fences, assert `count(OLD) == 1` per site) transfers directly and cost
   about ten minutes. It is the only check that catches a **drifted duplicate** of a
   fenced replacement, which is the failure mode the plan's no-transcription ruling
   exists to prevent and which nothing inside the task itself tests.

**For the plan close:**

4. **Finding 2** - `numeric_exact_compares_across_int_and_float`
   (`crates/muxsmith-core/src/matcher.rs:393`) is named for coverage it does not have;
   measured green under the mutation its name describes. Rename, or widen the body.
   New trigger for the controller, not in D111's twelve.
5. **Finding 1** - D111 M4's pathspec members `'crates/**/tests'` and
   `'crates/**/fixtures'` match nothing under git's wildcard-pathspec rules. Record so
   no later sweep reuses the expression; this is T12's rule-not-list duty in its
   instrument form. The correct shape is a literal directory pathspec
   (`crates/muxsmith-core/tests`) or `':(glob)crates/*/tests/**'`.
6. **Adjudication 4** - the example-validation vehicle's `input.pattern` discriminator
   must be a YAML **parse**; a line-shaped test false-positives on the README
   passthrough example's inline flow mapping. Goes on that vehicle's open design
   question next to the fragment problem.
7. **Adjudication 2** - D111 §4.6's and plan Step 8's "the five edited product files"
   is stale against §13.5's six-file list. Both are approved documents and neither is
   edited; the close records the reading that was applied (run over all six, report
   separately, route an unadjudicated hit).
8. **Adjudication 3** - D111's two `:1913` ROADMAP citations are stale at this tree
   (`:1922`). The ADR is append-only; the note belongs with T6's ROADMAP correction so
   the next reviewer of the control is not misled.
9. **Adjudication 1** - the design's placement omission. Worth one line in the
   doctrine's direction: a design that fences a test body should also say which group
   it joins, because "verbatim" implies a completeness the fence does not deliver.
10. **Adjudication 5** - optional, cheap: extend T-3's loop with an absent-property
    row so §3.1's fifth column is walked too. Not owed by this package.
11. **Finding 3** - the "an automated e2e would add an mkvmerge dependency" premise is
    false at this repo (eight gated files already). The conclusion stands on the other
    ground; the sentence should not be reused.
12. All **twelve** of D111's triggers (T1-T12) are correctly enumerated in the
    implementer's Step-9 report and are the controller's to mirror. **T7 is the one
    carrying an open owner decision** (a config-time diagnostic for a `raw:` key whose
    scalar kind can never equal a known property's declared kind), and this review
    strengthens it independently: the two probes that flipped to `missing-track` in
    1.4 are exactly the two cases the proposed guard would catch, and for an
    **optional** rule neither produces any error severity at all.
