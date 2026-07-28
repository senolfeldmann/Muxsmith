# Amendment 4 verdict - Plan-9 plan side (German subprocess test rides `muxsmith_localized`)

Reviewer: same independent reviewer as amendment 3; did not author the
amendment. Read at the tree (HEAD `42fa6ea` at review time - one
controller house commit after the amendment commit `ba69c36`; the plan
file is untouched since `ba69c36`, `git diff` empty). The owner's
option-B ruling is settled and not graded. All instruments my own, under
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a4plan-rev/`.
Line numbers measured at this tree state (plan now 475 lines).

## 1. Verdict: APPROVED_WITH_MINORS

One MEDIUM (the new helper's own rustdoc is an unaddressed surface - the
one omission-form fork left in an amendment whose purpose was closing an
under-specified vehicle), one LOW (the amendment log cites a Task-4
review that does not exist yet as a renumbering constraint). Both are
small plan edits; neither invalidates the vehicle, the D64 mechanism, or
any decision. Everything else - the blocker's reality, the vehicle's
completeness on its five named surfaces, the file-level invariant in all
three places, the Step-7 check, the non-renumbering, the consumer sweep,
the plan-only claim - verified clean at the source, including my own
re-measurement of the clap rejection.

## 2. Findings

**MEDIUM-1 - `muxsmith_localized`'s own rustdoc is unspecified: the fix
round would have to invent it or omit it, and either is a defect.** The
plan pins the helper's signature, body, the funnel's delegation, and the
FUNNEL's rustdoc content (Step 4 `:298`; Files `:281`), but says nothing
about the new helper's own doc comment. Ground truth: every pub helper in
`crates/muxsmith-cli/tests/support/mod.rs` carries a deliberate,
contract-grade doc (module doc `:1-7`, `insta_settings` `:14-36`,
`insta_settings_with_tmp` `:44-47`, `fake_mkvmerge_that_fails_queries`
`:55-62`, the funnel `:80-88`, `muxsmith_bare` `:96-107`), so an
undocumented new pub fn is a house-pattern outlier the Task-4 reviewer
would flag, while invented doc prose risks exactly the wrong derivations
this cycle keeps meeting (resurrecting "nowhere outside this function"
one function over, or "every test builds here", false given `bare`). The
Files entry's "nothing else in this file" sharpens the fork: a literalist
reading can take it as forbidding the doc. Under
`proc-latitude-clause-boundary` (derivability is not an exemption) this
is omission-form latitude reaching the fix-round implementer. **Required
change, two edits:** (1) plan `:298`, insert after "...and states the
delegation plus the file-level invariant, `cargo_bin("muxsmith")`
confined to this file." the sentence: "`muxsmith_localized` carries its
own rustdoc (every helper in this file is documented): it states that it
is the pinned path's construction site - the locale appended AFTER the
caller's args, so it follows the subcommand - and that `muxsmith` is its
`"en"` delegation; the D64 contract rationale and the file-level
invariant stay stated once, in the funnel's rustdoc, which the helper's
doc points at rather than restates." (2) plan `:281`, "the new
locale-parameterized pinned helper `muxsmith_localized`" -> "the new
locale-parameterized pinned helper `muxsmith_localized` with its own
rustdoc". This fills the gap consistently with the amendment's
already-made (and settled) choice that the funnel's doc keeps the D64
contract statements; say-once holds.

**LOW-1 - the amendment log cites a review that does not exist.** Plan
`:475`: "Task 4's executed report and review cite the current step
numbers, which an insertion would dangle mid-fix-round." The report
exists and does cite them (`task-4-report.md:5`, `:243`); the review does
not exist - the SDD directory has no task-4 verdict, and `progress.md`
row 4 states the order explicitly: "pending - amendment 4 first, then the
fix round, then review". A future review also cannot dangle: it will run
against the amended plan. The decision the sentence supports
(non-renumbering) is independently grounded and stands; the sentence
overclaims one consumer. **Required change, plan `:475`:** replace
"Task 4's executed report and review cite the current step numbers, which
an insertion would dangle mid-fix-round" with "Task 4's executed report
cites the current step numbers, which an insertion would dangle
mid-fix-round (the review runs after the fix round, against the amended
numbering)".

## 3. Rulings on the dispatch's questions

**(1) Vehicle completeness: complete on the five named surfaces; the
sixth is MEDIUM-1.** Helper signature pinned with types (`pub fn
muxsmith_localized(args: &[&str], locale: &str) -> Command`, `:298`);
body pinned as today's funnel body with the locale parameterized -
construction, arg order, append-LAST with the after-the-subcommand
rationale (`Command` is already imported in the file, `.unwrap()` rides
"today's funnel body"); the funnel's delegation pinned to the exact
expression `muxsmith_localized(args, "en")`; the funnel-rustdoc update is
a real obligation (the current doc `:80-88` literally says "builds its
`Command` here" and describes the appending, both falsified by the
delegation - and its "appears nowhere outside this function" sentence is
already imprecise today given `muxsmith_bare:110`, which the mandated
file-level rewording fixes in passing) with its content mandated: keep
the D64 contract statements, state the delegation and the file-level
invariant - the same content-mandate form Task 3 Step 1 uses for the
licence block, house-accepted; the test's invocation pinned verbatim
(`support::muxsmith_localized(args, "de")`, `:299`) with the
discriminating rationale attached. No dead-code attribute question
arises: the funnel's delegation makes the helper transitively used
wherever `muxsmith` is (which today compiles under `-D warnings` without
an allow). The helper's own rustdoc is the only invention left (MEDIUM-1).

**(2) The Step-7 invariant check is real, not decoration.** It has a
command, an exact expected output (`grep -rln 'cargo_bin("muxsmith")'
--include='*.rs' crates` -> exactly `crates/muxsmith-cli/tests/support/mod.rs`),
a demonstrated fire (the pattern and pathspec return that one file on the
current tree - my own run reproduces it; a broad-pattern control over
`crates src-tauri` returns the same single file), and a reachable red
state I demonstrated rather than argued: the identical grep form over a
scratch directory with the string planted in two files lists both
(`plant/` instrument) - so a `cargo_bin` call written into
`cli_validate.rs` demonstrably adds a second output line. The green state
is today's tree and survives the helper edit (the new call site is in the
same file). It guards the actual regression the fix round could
introduce (bypassing the pinned helper) and runs in the task exit bar.
The neighbouring sentence was correctly rescoped ("Otherwise no
absence-shaped check exists in this task") - the author's own sweep
caught that falsified sentence, a consumer its dispatch had not listed.

**(3) The deliberate NON-renumbering is sound.** The two amendments are
opposite cases of the same rule, and the plan says so: amendment 3's
dedicated step answered a DIFFERENT design entry (the D96 rider, not
D98), keeping the coverage walk one-to-one; amendment 4's vehicle answers
the SAME entry as Step 4's own pinned test (D101 - verified: the design's
D101/section-7-item-5 is what the German test implements, and the
coverage map's D101 row stays one task, one row). The vehicle sits inside
Step 4 as a bold-led bullet, findable without a checkbox. The second
stated ground overclaims one artifact (LOW-1) but holds for the report,
which does cite current step numbers. Task 4's steps remain exactly 1-8,
contiguous (extracted mechanically); no step reference anywhere in the
plan dangles.

**(4) Consumer sweep: no missed enumeration surface this time.** My own
walk over every surface the vehicle touches: Files list (extended, tenth
entry - recounted, 10), `git add` line (`:306`, stages the file),
must-not list (`:312`, the vehicle exactly as amendment 4 fixes it - the
surface amendment 3's first pass missed is swept here), Step 7 (check
added, "no absence-shaped check" rescoped), amendment log (entry in the
house shape). Verified unchanged-and-correctly-so: the Task-4 header and
Read-first (no new design entry to name), the coverage map D101 row, the
acceptance map row 5 ("subprocess en/de tests" is vehicle-neutral), the
sequencing 4-5 edge (support/mod.rs IS the CLI test crate, and the
delegation is behavior-identical for every existing caller), the
Interfaces block (Task 5 consumes nothing from the new helper), the
model-tier row, the self-review counts (none covers Task 4's Files), and
the amendment-1 infrastructure boundary - whose OUT set is enumerated
(Vitest, `tauri::test`/`mock_builder`, `src-tauri/tests/`, IpcError
funnel; Global Constraints `:25` matches the log's quotation) and does
not contain a support-module function; the log records that argument
explicitly. The old pinned wording survives only as two quotations of the
dead form (`:298`, `:471`); the pre-state had exactly one live instance
(fire: `git show ba69c36^` grep -> 1) and the live instruction now
invokes the helper. The one gap my walk found that the author's did not
is not an enumeration surface but a content surface: MEDIUM-1.

**(5) The design is genuinely unaffected.** The design pins the German
subprocess test only at the level "exits 2 and renders the en/de text
verbatim (subprocess test)" (section 7 item 5, `:1593-1596`) plus D101's
validate-level tests and control (`:1023-1028`); the invocation vehicle,
the support module, the funnel, and `--locale` mechanics appear nowhere
in the design (grep for `support/mod`, `support::muxsmith`, `cargo_bin`,
`locale en` -> zero, exit 1 measured on grep; fire control: "muxsmith
validate" -> 1 hit). Nothing the delegation changes falsifies any design
sentence, and the test's design-pinned substance (exit 2, de text in the
snapshot) is untouched. Plan-only is correct, and the design log
correctly gains no round; the logs' numbering offset stands as amendment
3 recorded it.

**The file-level correction, verified in all three places.** The
dispatch's "exactly one `cargo_bin` call site" is wrong, and the plan
does NOT carry it anywhere: my grep for one-call-site claims over the
plan finds none; the in-file truth is 3 occurrences (funnel `:90`, bare
`:110`, doc mention `:88` - my own count), the file-level truth is one
file (my own `grep -rln`). Step 4 (`:298`) states the file-level
invariant on the funnel's doc; Step 7 (`:302`) checks it file-level;
the log (`:474`) states it file-level and scopes the single-construction-
site claim to the PINNED path with the bare exception explicitly
untouched. Consistent in all three; the author's refusal of the
dispatch wording was correct and correctly written in.

## 4. Blocker and accuracy verification (my own instruments)

- **The clap rejection re-measured:** `./target/debug/muxsmith validate
  /tmp/nonexistent.yaml --locale de --locale en` -> EXIT=2, stdout 0
  bytes, stderr "error: the argument '--locale <LOCALE>' cannot be used
  multiple times". Both halves of the danger reproduce: `.code(2)`
  passes on the usage error, and the snapshot would be empty - the plan's
  de-fence content requirement is the unmasking check, exactly as Step 4
  and the log state.
- **Tree state matches the narrative:** Task 4's commit `d768657`
  contains `cli_validate.rs` with only the en test (`:65`) and only the
  en snapshot; no `muxsmith_localized`, no de test, no de snapshot -
  `progress.md` row 4 concurs ("10 of 11 files; the German subprocess
  test and its snapshot are absent"). Step 4's amended pre-state is real.
- **Support-module anchors:** funnel `:89-94` (cargo_bin `:90`, args
  `:91`, en append `:92`), rustdoc `:80-88` with the quoted sentences,
  `muxsmith_bare` `:109-111` - all as the report's premise checks state.
- **Routing:** the memo exists (`task-4-report.md` section 4), is the
  measured blocker, and returned rather than resolved at the keyboard.
- **Files recount:** ten entries (`:275-284`), counted from the list.
- **Typography** over every amended line: zero banned glyphs (exit 1;
  pattern fire-verified this session, sample count 1). Commit
  pathspec-scoped to the plan, five hunks at `-U1`, all graded; the
  concurrently-modified house YAMLs were left unstaged and landed later
  in the controller's `42fa6ea`, as the report surfaced.

## 5. Evidence appendix

Scratch root:
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a4plan-rev/`

- `amendment4.diff` - `git show ba69c36`.
- `clap-stdout.txt` / `clap-stderr.txt` - my double-locale run (stdout 0
  bytes; the usage error; EXIT=2 captured in the run output).
- `cargo-bin-files.txt` / `cargo-bin-files-broad.txt` - the file-level
  invariant grep and its broad-pattern control, each -> one file.
- `plant/` - the red-state demonstration: the identical grep form lists
  both planted files, proving a second `cargo_bin` file adds output.
- `plan-vehicle-mentions.txt` - every `support/mod.rs` /
  `muxsmith_localized` mention in the plan, classified.
- Step extraction (Task 4 -> Steps 1-8 contiguous), old-wording counts
  (current: 2, both quotations; pre-state via `git show ba69c36^`: 1),
  design absence greps with fire controls - commands and outputs quoted
  in sections 3-4.

## 6. HARVEST

- **The Task-4 fix round must carry:** the amended Step 4 verbatim
  (helper signature + body, funnel delegation, funnel-rustdoc content
  mandate, the `support::muxsmith_localized(args, "de")` invocation, and
  - once MEDIUM-1 lands - the helper-rustdoc sentence);
  `muxsmith_bare` and its exception doc byte-identical (reviewer checks
  by diff); the de snapshot's CONTENT checked against D101's de fence
  text - `.code(2)` alone proves nothing, clap also exits 2; the Step-7
  subset including the file-level invariant grep (expect exactly
  `crates/muxsmith-cli/tests/support/mod.rs`); every existing CLI
  subprocess test green unchanged (the delegation is behavior-identical);
  staging `support/mod.rs` + `cli_validate.rs` + the de snapshot as
  Task 4's second commit (two-commit tasks are established execution
  shape - Tasks 1 and 3 - recorded in `progress.md`, not the plan).
- **Anchors for the fix-round dispatch, at this tree:** funnel `:89-94`,
  funnel rustdoc `:80-88`, `muxsmith_bare` `:108-111`; locate by content,
  not line.
- **For the Task-4 reviewer:** the funnel's OLD doc sentence "appears
  nowhere outside this function" was already imprecise before this
  amendment (bare's `:110` is outside it); the mandated file-level
  rewording is a correction, not a regression - do not flag the loss of
  the function-level claim.
- **Ledger note when the controller mines this cycle:** the dispatch's
  "exactly one call site" -> file-level correction is a clean instance of
  the author refusing a controller-supplied wrong premise with a
  measurement (`proc-57-briefs-not-ground-truth` working as designed);
  the companion lesson is LOW-1's shape - a rationale that enumerates its
  consumers must only name consumers that exist (an anticipated artifact
  is a prediction, not a citation).

---

# Delta verdict (fix round, commit `4e5daa6`): APPROVED

Same reviewer, resumed; judged at the tree (HEAD `8b315e6` at review
time - one controller house commit after the fix, touching only
`docs/decision-ledger.yaml` and `docs/process-conventions.yaml`; the plan
file is byte-identical to `4e5daa6`, `git diff` empty). Instrument:
`fix-round4.diff` in the same scratch root.

**Both findings closed at the file, wordings verbatim in all three
places.**

- **MEDIUM-1 closed, and the omission fork is actually removed.** Plan
  `:298` carries the helper-rustdoc sentence byte-for-byte as required
  (re-extracted whole from the file), and `:281` now reads "the new
  locale-parameterized pinned helper `muxsmith_localized` with its own
  rustdoc", which also defuses the "nothing else in this file"
  literalist reading. The fix-round implementer now has a closed content
  list for the doc - construction site of the pinned path, locale
  appended AFTER the caller's args so it follows the subcommand, and
  `muxsmith` as the `"en"` delegation - with nothing left to invent; the
  form matches the funnel-rustdoc content mandate already graded as
  house-accepted. Single-homing holds exactly as required: the sentence
  itself orders the D64 contract rationale and the file-level invariant
  stated once, in the funnel's rustdoc, with the helper's doc pointing at
  it rather than restating.
- **LOW-1 closed, and the replacement clause is true.** Plan `:475` now
  reads "Task 4's executed report cites the current step numbers, which
  an insertion would dangle mid-fix-round (the review runs after the fix
  round, against the amended numbering)"; the old "report and review"
  conjunct is gone (grep 0, exit 1). Both halves verified: the report
  does cite current step numbers (`task-4-report.md:5`, `:243`), and
  `progress.md` row 4 pins the order ("amendment 4 first, then the fix
  round, then review").

**Scope held.** One file, exactly three hunks (`@@ -280`, `@@ -297`,
`@@ -474`), 3 insertions / 3 deletions - precisely the three edits the
findings required; nothing else in the plan changed, so no ruling or
passed dimension re-opens (no step or numbering touched, no fence
content copied into the plan, the D64 mechanism and the design
untouched). Typography of all three changed lines: zero banned glyphs
(exit 1 measured on grep; pattern fire-verified this session).

The plan-side amendment 4 is APPROVED with no open findings; the Task-4
fix round can dispatch on the amended plan as it now stands.
