# Plan 10 whole-branch review - verdict

Reviewer: independent whole-branch reviewer, top tier, per
`.superpowers/sdd/plan-10/whole-branch-review-brief.md`. Range reviewed:
`754cb73..80e5c19` (13 commits: 5 task + 1 fix-round + 7 controller), read as
files at HEAD, with every load-bearing claim re-measured by my own instruments
under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/wbrev-independent/`.
HEAD == origin/master == `80e5c19`; push CI run `30465581798` re-verified
read-only via `gh` (conclusion success, headSha `80e5c19`, all five jobs
success including `ledger-lint`), logged in `gh-log.md`.

## Verdict: READY_WITH_MINORS

The branch delivers all five work items as ruled. All 21 machine-verifiable
acceptance halves pass under my own instruments; W4-c is correctly OPEN. The
full eleven-part gate ran green under my own run with every part's exit code
captured individually. No shipped behaviour changed. No artifact claims or
implies 1.0 completeness. The minors: two deferred findings ruled IN for the
close fix wave, one close-sequencing staleness the salvage will create, one
routed pre-existing spec-vs-code drift this review surfaced, one NIT.

## Findings

1. **MINOR - deferred NIT ruled IN (see Rulings): the exit-code pronoun.**
   `README.md:190`: "Interrupt any of them with Ctrl-C" - *them* has no
   antecedent in its paragraph. Verified by reading the shipped line: the only
   preceding "command" token is the adjective in "command-line", and the
   nearest bindable plural is "your scripts", a coherent but wrong reading
   (the referent, the five subcommands, sits several paragraphs up). Required
   change, exact: replace `Interrupt any of them with Ctrl-C` with
   `Interrupt any subcommand with Ctrl-C`. Lands in the close fix wave.

2. **MINOR - deferred ragged wraps ruled IN (see Rulings).** Two mid-sentence
   fragment lines left by Task 5's re-wraps, both verified in the tree:
   `e2e/smoke.spec.ts:1437` (`// byte-for-byte the`) and
   `src/views/EditorView.vue:87` (`// the machinery \`ListWidget.vue\``).
   Comment-only, zero behaviour. Required change: rewrap each comment's
   sentence to the surrounding block's fill width; acceptance is mechanical -
   the whitespace-normalized word sequence of each comment is byte-identical
   before and after, and no line of the comment is a mid-sentence fragment.
   Lands in the same fix wave (one gate run covers findings 1 and 2).

3. **MINOR - the README's verdict figure is falsified by this same close's
   salvage.** `README.md:202` states "219 files under `docs/` with `verdict`
   in the name". Measured true today under BOTH unit readings (basename 219,
   full-path 219; frozen `verdicts/`-directory unit still 78). But the SDD
   salvage - a named close action of this very plan - copies
   `.superpowers/sdd/plan-10/` into `docs/process-journal/artifacts/`, adding
   at least 6 verdict-named files under `docs/` (5 task verdicts plus this
   file; plan-9's salvage added 13). The figure is stale the moment the
   salvage commit lands - the defect class W5-e existed to remove, reproduced
   by close sequencing visible at plan time and caught by no task review
   because the falsifier is a close action, not a task. Required change:
   re-measure and update the figure in (or immediately after) the salvage
   commit, which is the commit that knows the new value; the unit wording
   stays as shipped. Note for the owner: every future plan close falsifies
   this figure again; if that treadmill is unwanted, the durable fix is a
   growth-marked phrasing, which is his register call, not mine.

4. **MINOR (routed, no pre-close tree change) - `raw:` "byte-exact" is a
   pre-existing spec-vs-code drift, surfaced by this review.** The README's
   new anchor item 4 says `raw:` gives "byte-exact value equality"; spec 4.4
   and 9.2 say "byte-literal value equality". The code does not support the
   absolute: `matcher.rs::exact_matches`'s `raw:` arm calls `scalar_eq`,
   whose `(Int, Float)`/`(Float, Int)` arms (matcher.rs:207, :209) compare
   numerically, so `exact: { raw:x: 6 }` matches a reported `6.0`. Everything
   the bullet lists as switched OFF (language normalization, codec aliasing,
   false-when-absent, the substring/regex type check) is verified exactly as
   written; only the byte-exact absolute is wrong, and it is the spec's own
   wording, which Task 4 transcribed correctly under the precedence order -
   so this is NOT a Task-4 defect, but the Task-4 review's "upheld against
   spec section AND core symbol" missed the numeric cross-arm (the one
   task-review disposition I grade as too generous). Required change: route
   onto the ROADMAP's existing spec-amendment vehicle (the entry carrying the
   8.1 `validate`-flags synopsis underclaim, same commit `2f1dca0` family);
   the README follows whatever wording the spec amendment lands. Deciding the
   wording here would resolve a spec fork at the keyboard.

5. **NIT (no action required) - the gate-count counter models only backslash
   continuations.** A line ending in `|` or `&&` continues in shell but is
   counted as two commands, silently. Unlikely in blocks whose norm the
   violation message already states ("enumerate one command per line"); if
   routed at all, it rides the same ROADMAP vehicle as the fourth-gate-block
   trigger. Recorded so the next gate-block author knows the boundary.

**Two owner items must reach the close report** (both already recorded, both
still pending his eyes): (a) the `ci-04-dependabot-cadence` runner-image
exclusion, flagged for owner confirmation (occurrence 5 on that entry); (b)
the open owner question whether `comments-locate-by-symbol-never-by-line-number`
reaches CI/config comments (`#`, `<!-- -->`), riding the surviving
`.github/workflows/ci.yml:90` member (ROADMAP Docs-accuracy entry).

**Close-action condition measured MET:** 28 occurrence refs across three of
the four house YAML files cite `.superpowers/sdd/plan-10/...` scratch
basenames (conventions 1, process-conventions 10, decision-ledger 17), so the
post-salvage citation re-point sweep the close actions carry conditionally
APPLIES.

## Rulings on the two deferred findings

1. **Task 4 fix-round NIT ("Interrupt any of them"): land it before the
   package closes.** Grounds: it is a user-facing accuracy defect inside the
   one task whose entire purpose was user-facing accuracy; the repair is two
   words, fenced above, risk zero; the close fix wave exists and the gate
   re-runs before the next push regardless. Deferring it past the close would
   ship a known ambiguity to save nothing.

2. **Task 5 MINOR (two ragged re-wraps): land it now, same fix wave.**
   Cosmetic and behaviour-free, but the marginal cost is zero once ruling 1
   triggers a fix commit and a gate run, and landing it restores the Task-5
   report's "no line left ragged" claim to true instead of leaving a recorded
   false report claim standing against the tree.

## Re-examination of findings disposed without a tree change

Walked all of them in `progress.md`; none disposed too easily except the one
graded at finding 4 above (Task 4's anchor item upheld against code with the
numeric cross-arm missed). Concurring explicitly on the rest: Task 1's
"states 1 commands" unpluralized (byte-fenced reproduction was right); Task
1's fourth-block blindness (plan property, correctly a ROADMAP trigger -
text verified in the tree); Task 2's module-doc "no vehicle needed"
(provenance-tagged "(Plan 5 Task 2)" describes what that task added; an
additive test does not falsify it); Task 4's "Two conventions that hold
everywhere" (an idiom over the flags that exist, not a quantifier; owner-side
call stands); Task 5's unlabelled pre-edit line numbers (process artifact,
permitted by the convention's own scope boundary).

## Acceptance map, walked end to end

| # | Verdict | My evidence |
|---|---|---|
| W1-a | PASS | My M1 (batch_document sorted -> collection) turned `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran` RED (failure pasted in appendix); new report_json tests stayed GREEN under M1, proving no duplicate producer |
| W1-b | PASS | My M2 (config_only_document same) turned `dry_run_and_validate_json_agree_on_config_diagnostics_ordering` RED |
| W1-c | PASS | New `batch_document_preserves_batch_diagnostics_collection_order` exists; my M4 turned it RED, restore GREEN |
| W1-d | PASS | New `batch_document_preserves_per_file_diagnostics_collection_order` exists; my M3 turned it RED, restore GREEN |
| W2-a | PASS | Expression A on end state: 0 hits; fired on pre-state `44f1c8e`: 20 lines / 13 files |
| W2-b | PASS | Expression B on end state: 0 hits; fired on pre-state: 4 lines / 4 files |
| W2-c | PASS | Symbols re-verified at every non-trivial rewritten site (14 spot-checks: `validate_locator` Some(false) rejection, `jobStateKey`, `attachmentRuleFields`, `jobs-history-run`, `delta_for`'s AddExact/AddNotExact arms at :1820+, `parse_attachment` doc contract, `Assignment::track_kind` doc, `run_document(batch_document(` at commands/run.rs:151, `MATCHABLE_PROPERTIES` ("id", Integer) at generated.rs:42, tauri test name, both README anchors, scalar_eq no-(Str,Bool)-arm); 21-comment count recomputed from the diff |
| W3-a | PASS | `gate-total` anchor at BUILDING.md:74, canonical sentence at :75-78; consumed by the running check |
| W3-b | PASS | My fire G1 (11 -> 12): exit 1, "gate-total states 12 parts but the three gate blocks enumerate 11" |
| W3-c | PASS | G2 (extra frontend command): exit 1, block mismatch AND total mismatch |
| W3-d | PASS | G3 (total marker deleted): exit 1, EXACTLY ONE violation, comparisons skipped |
| W3-e | PASS | G4 (frontend marker deleted): exit 1, EXACTLY ONE violation |
| W3-f | PASS | G5 (house command as backslash continuation): exit 1, continuation message PLUS house 1-vs-2 and total 11-vs-12 - the exact three-violation shape the plan prescribes |
| W4-a | PASS | `renovate.jsonc` byte-identical to the plan's fence (python equality on the extracted fence); validator evidence stands on the task + review runs, artifact unchanged since (`630d418` untouched by later commits) |
| W4-b | PASS | Read against `ci-04-dependabot-cadence`: monthly 3-day window, per-ecosystem groups, majors separate by default (separateMajorMinor absent), security immediate via untouched vulnerabilityAlerts defaults, rust-toolchain own group, mise off, packageManager off, github-runner off |
| W4-c | OPEN, as required | ROADMAP trigger "Renovate/Dependabot activated ->" still in Triggers, NOT FIRED; no artifact claims activation |
| W5-a | PASS | Binary re-derived: `schema --help` shows only `-h`; validate/dry-run/identify/run carry `--json` + `--locale` ("default: system, fallback en"); `--on-collision` domain error/skip/overwrite with error default; 130 in `cli.rs` contract doc |
| W5-b | PASS | "each known property" + codec_kind double-rejection verified in `validate.rs` (DiagCode::CodecKindExactOnly pushed at :324) and `matcher.rs` |
| W5-c | PASS | All four anchor items verified against spec AND code (dual-field `language` lookup + `lang_eq`/`normalize`; absent-boolean false arm at exact_matches' final arm; closed domains via `matchable_domain`/InvalidPropertyValue; raw contrast) - with finding 4's routed caveat on the "byte-exact" absolute, which is the spec's own claim |
| W5-d | PASS | Re-measured: 104 defining headings, 103 distinct, reach D105, D73/D74 absent; both readings (specs dir, all of docs/) converge at 103 - the shipped sentence states count and reach as two things |
| W5-e | PASS today | 219 under both unit readings (basename and full path), frozen unit still 78 - with finding 3: the close's own salvage falsifies the value |
| W5-f | PASS | Warning string byte-identical at INSTALL.md:88 and ROADMAP:904; attributed to dnf; file-top enumeration extended to three sections |

## Dimension results beyond the map

- **Cross-task:** `report/json.rs` is blob-identical to `0a80189` (Task 2's
  mutations fully unwound, Task 5's comment edit clean on top). Task 5's
  rewritten citations name the README anchors as Task 4 actually shipped them
  (both verified verbatim in the tree). The ROADMAP QA entry's
  `docs/INSTALL.md:82` citation survived Task 4's insertion (line 82 is still
  the dnf command; the note landed below it). Task 1's check ran in four later
  task gates, my gate, and CI. The one cross-boundary residue is finding 3.
- **Self-referential claims, hunted:** 21 rewritten comments recomputed from
  the diff (4+1+1+1+3+1+1+1+1+1+1+1+1+1+1+1); 24/16 corpus reproduced by my
  own expressions; `BUILDING.md:138` measured 86 chars and the file's only
  over-80 non-fenced prose line; the three positional ordinals reproduced at
  :102/:134/:135; the `queue.rs:73` survivor verified (line 73 is
  `pub struct QueueOpts {`, the meant link two lines later); the born-stale
  `lib.rs:557-563` claim verified by archaeology (at `997666a~1` the fixture
  sat at :557; `997666a` itself cites :557-563 at :748 while its own tree has
  the target at :694). No false self-claim found beyond findings 3 and 4.
- **Gate and mis-fire analysis:** eleven parts green, exit codes individual
  (appendix). Legitimate future states checked: heading rewording (markers
  immune), comments/blank lines in blocks (tolerate-green probe G6 passed),
  fourth gate block (recorded ROADMAP trigger with readable event, verified
  in tree), backslash continuation (loud refusal, G5), reflowed canonical
  sentence (loud unparseable-sentence violation by regex construction).
  Residual: finding 5's pipe/&& NIT.
- **House-knowledge:** ledger-lint green enforces count==len(occurrences),
  refs present, tier-2 promoted_at, unique ids across 541 entries. The
  promotion checked against the matrix: `a-search-whose-terms-come-from-memory-
  produces-a-false-absence` is agent-emergent + process at count 3, moved to
  process-conventions.yaml with `promoted_at: 3`, removed from the ledger, no
  duplicate id; its statement gains the selector/pattern split, which the
  third occurrence (Task 5's file-selector miss) carries strictly. Spot-read
  occurrences carry their events strictly (ci-04's occurrence names the
  runner-image exclusion's actual ground - plan fence + D85, not an owner
  ruling - and flags it; the conventions occurrence states the survivor and
  the open question rather than claiming closure).
- **What the close may claim:** zero completeness-claim hits over every added
  line (pattern fired against a known-present control), journal untouched in
  the branch, README status still "Pre-1.0", commit messages clean, ROADMAP
  records the sweep class NOT closed tree-wide with the selector named.
- **No-work-needed premises, run:** M1/M2 fired the existing guards (so "no
  new producer for the sorted halves" is measured, not argued); the exclusion
  branch tolerate-green probed; concurrences listed above.
- **Spec conformance:** no product behaviour changed - task commits touch
  comments, tests, docs and a new config only (per-commit stats verified);
  the two new tests assert spec 5.2's contract through a discriminating
  reversed-severity fixture (collection order provable as NOT sorted), not
  the implementation's shape.

## Evidence appendix

Instruments under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/wbrev-independent/`:
`exprA-end.txt`, `exprA-fire.txt`, `exprB-end.txt`, `exprB-fire.txt`
(corpus expressions, end state + pre-state fires), `dseries.txt` (decision
headings), `baselines.txt` (pre-mutation sha256 of `json.rs` and
`BUILDING.md`), `gate.log` + `gate13.sh`/`gate46.sh`/`gate711.sh` (the
eleven-part gate, one EXIT= line per part, all 0), `index-blobs.txt`.

Key runs (all foreground, this session):
- Mutations M1-M4 applied one at a time by exact-string python edits to
  `crates/muxsmith-core/src/report/json.rs`; targeted `cargo test` per
  mutation; observed failures: M1 `dry_run_json_sorts_...` (left
  errors-last), M2 `dry_run_and_validate_json_agree_...`, M3
  `batch_document_preserves_per_file_diagnostics_collection_order`, M4
  `batch_document_preserves_batch_diagnostics_collection_order`; after each
  restore `sha256sum` = `942cdce2...99e9`, final `git hash-object` =
  `0a801895f5c5279135e08cb50565165badace5ae` = `HEAD:...json.rs`.
- Gate-count fires G1-G5 + exclusion probe G6 on `BUILDING.md`, each restored
  via `git checkout --`; final `sha256sum` = `2e056a49...1798`,
  `git hash-object` = `911fdba996fcafecbd736f92c1820ec0078d8144` =
  `HEAD:BUILDING.md`; green run exit 0 with the widened summary line.
- Tree identity per FILE against blobs: `git ls-tree -r -z HEAD` vs
  `git hash-object --stdin-paths` over all 1367 tracked paths - 0 mismatches
  (python instrument, comparison control included; the mutated states earlier
  in the session are the fired control that a changed file is detectable).
- CI: `gh run view 30465581798` (read-only), logged in `gh-log.md`.

## HARVEST

1. **A snapshot count in a shipped document is falsified by the process that
   ships it.** The README's verdict figure was measured true three times in
   this package and will be false at the close's own salvage commit, because
   the salvage adds members of the counted set. The readable trigger: a close
   action ADDS members to a set some shipped prose counts. The handle: the
   commit that changes the set updates the count, or the count is phrased as
   growing. (Finding 3; candidate occurrence for
   `a-number-in-prose-carries-a-unit-claim-that-re-measuring-does-not-check` -
   a third failure shape: the unit is well-formed and the value is freshly
   verified, and the WRITER's own pipeline is what moves it.)
2. **Transcribing an authoritative document propagates its errors with full
   verification confidence.** Task 4 verified its anchor against spec AND
   code, and the spec's own "byte-literal" absolute still shipped, because
   upholding a claim against the code invites confirming the mechanism named
   (the raw arm) rather than the absolute asserted (byte equality across
   every type pair). A verification of an absolute enumerates the type/case
   pairs the absolute quantifies over; `scalar_eq`'s six arms were the
   enumeration nobody walked. (Finding 4.)
3. **The check that guards a boundary teaches where the boundary is not.**
   Three instances in one package now: the corpus selector (ci.yml survivor),
   the marker set (fourth block), the continuation model (backslash only).
   A guard's own exclusion list is the next reviewer's target list; this
   review found the third by asking what the counter cannot see, which is a
   repeatable question, not a lucky read.
4. **`grep -c` counts lines, not matches** - re-learned live: my scratch-path
   count was taken with `-c` and confirmed with `-o | wc -l` (both 28) only
   because the house had just ledgered that exact instrument slip. The
   ledgered pattern paid for itself within hours.
