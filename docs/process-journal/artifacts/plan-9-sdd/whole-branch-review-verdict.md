# Plan 9 whole-branch review verdict

**Verdict: NEEDS_FIXES** - one MEDIUM finding (a doc-only fix, two rustdoc
sites in one file) plus its LOW house-record rider. Everything else on the
branch checks out: all eight acceptance observables walk to producing tests
that ran green in my own gate re-run, every D-entry D91-D105 is implemented
as amended, all eight spec amendments landed verbatim with no spec
self-contradiction, the cross-task seams hold, and the full ten-part gate
plus `ledger-lint` reproduced foreground with every claimed aggregate
recomputed from its enumeration (39 / 68 / 212 / 515). The finding is
gate-invisible by construction (no test asserts the stale strings), which is
exactly why it lands here and not earlier.

Reviewed at HEAD `23136b6` (branch `9bbe53d~1..HEAD`, 41 commits, 47 files -
file list pulled with `git diff --name-only 629dc64..HEAD` and walked
against the tasks' Files lists; the three files outside any list are Task
3's compiler-sweep EXEMPLARY set plus its fix-round producer, each verified
below). Tree clean at HEAD before and after my runs (`git status
--porcelain` empty both times). I committed nothing and edited no product
file.

---

## Findings

### Finding 1 (MEDIUM): two session-cache claims in `identify.rs` contradict spec 5.5 as this branch amended it (S-8)

**Sites, both citing the spec section S-8 rewrote:**

1. `crates/muxsmith-core/src/identify.rs:3-4` (module doc): "and caches
   results in memory keyed on path + mtime + size so dry-run and run never
   re-identify an unchanged file (spec 5.5)". That is the cross-command
   sharing claim S-8 deleted from the spec. It is false on every surface of
   the shipped product: the seam constructs its cache per planning call
   (`pipeline.rs:126-129`, D93), so a GUI dry-run followed by a run
   re-identifies every file, and CLI dry-run and run are separate processes.
   Spec line 312 now says the opposite in so many words: "separate calls
   re-identify, so a GUI dry-run followed by a run spawns `mkvmerge -J` per
   file in each".
2. `crates/muxsmith-core/src/identify.rs:302` (`IdentifyCache` doc):
   "In-memory identification cache for one session (spec 5.5)". "For one
   session" is the per-session description the owner ruled out (amendment 1
   ruling B); the spec sentence it cites now reads "constructed per planning
   call and dropped with it".

**Evidence, run by me:** `git grep -n "spec 5.5" crates src-tauri src` -
the exact sweep the branch's own new Tier-1 rule
`a-spec-amendment-sweeps-the-code-that-cites-that-section` prescribes for
S-8's author - returns both sites (plus the already-corrected `:388` and
the harmless orchestration citations). Literal-phrase control: `git grep -n
"per-session\|one session\|per session" -- crates src src-tauri` returns
exactly `identify.rs:302` in product code, which is why Task 1's fix round
(commit `fed55be`) caught only `:388`: the Task-1 verdict's sweep pattern
was the hyphenated literal `per-session`, and its result ("the only
surviving per-session claim") answered that pattern, not the class.

**Why MEDIUM and not a close-routed LOW:** this is the identical defect
class, in the identical file, falsified by the identical amendment, that
the Task-1 review graded MEDIUM-1 and blocked on (`task-1-verdict.md:121`,
`:210`); Task-5 review MEDIUM-1 (a false composed doc claim on the
re-export) sets the same bar. A core module's headline sentence
misdocumenting caching behavior with a spec citation is the "belief under
which someone re-adds the session cache" case the sweep rule exists for.

**Required change (doc-only, two `///`/`//!` edits):** re-point both docs
at the amended spec. Module doc `:3-4`: replace "so dry-run and run never
re-identify an unchanged file (spec 5.5)" with wording scoped to one call,
e.g. "so one planning call never re-identifies an unchanged file (spec
5.5; separate calls re-identify)". Struct doc `:302`: drop "for one
session", e.g. "In-memory identification cache, constructed per planning
call and dropped with it (spec 5.5)." Exact prose is the fixer's within
these semantics; the falsified claims are not. Vehicle is the controller's
call - a licensed doc fix round, or folded into the close's existing
text-corrections pass with the owner's licence - but the branch should not
close with core docs contradicting the spec the same branch amended.

### Finding 2 (LOW, house): the ledger occurrence restates the Task-1 sweep as a class claim the tree contradicts

`docs/decision-ledger.yaml`, entry
`a-spec-amendment-sweeps-the-code-that-cites-that-section`, its single
occurrence: "...identify.rs:388 kept citing it as 'The per-session
identification cache (spec 5.5)' - **the last per-session claim in the
tree**...". The underlying verdict measured the literal phrase and said so
(`task-1-verdict.md:121`: "the only surviving `per-session` claim in
code"); the occurrence's unqualified restatement is false for the class its
own rule targets - the rule's own prescribed grep finds Finding 1's two
sites. A borrowed measurement answered its own question (literal phrase),
and the restatement widened it.

**Required change (controller ledger edit, same change as Finding 1's
fix):** qualify the occurrence to the literal measurement (e.g. "the last
literal `per-session` claim; two same-class session-cache claims in the
same file were caught by the whole-branch review") so the entry's statement
family is true of the tree it describes.

---

## What I verified, dimension by dimension

**1. The eight acceptance observables, walked to producers that ran in my
own gate re-run** (test names confirmed present in my logged output, paths
in the appendix):

1. *Pipeline hoist + funnel migration*: `git grep -F
   "config_diags.extend(lint::provable_overlaps"` = 0 at HEAD, fired = 4 at
   `629dc64` (the pre-branch tree); all CLI subprocess suites and src-tauri
   inline tests green in my `cargo test --workspace` run.
2. *run_batch hoist*: `git grep -F "fn run_batch" -- src-tauri` = 0 at
   HEAD, fired = 3 at `629dc64`; both surfaces call core `run_batch` (CLI
   `run.rs:208`, src-tauri `run.rs:462`); the two moved tests ran and
   passed under core (`executor::queue::tests::run_batch_emits_started_output_finished_in_order`,
   `...::run_batch_writes_job_log_files`).
3. *Runs-root deletion*: `MUXSMITH_RUNS_ROOT` in src-tauri 0/2
   (HEAD/fire), `resolve_runs_root` 0/4; the CLI gate is byte-identical
   (the branch diff over CLI `run.rs` contains no hunk touching it;
   `create_logger`'s cfg split read at `:275-279`); muxsmith-gui tests
   green in the workspace run.
4. *Worker-panic path, all halves*: core
   (`worker_panic_is_reported_as_failed_not_cancelled` asserts `panic:
   Some("scripted worker panic for job 0")` AND the unchanged prefix
   token, `queue.rs:819-830`); CLI human
   (`finished_panicked_renders_two_lines_without_na`); GUI row (the
   `jobsview-reset.spec.ts` panic test, 4 spec hits in my e2e log);
   persisted (`panicked_outcome_persists_its_payload_on_the_job_record`,
   `joblog.rs:148`, plus the always-on-wire shape assertion `joblog.rs:118`);
   `--json` wire shape (`report_json.rs` asserts `"panic": null` in
   `run_document` jobs entries); catalog guard (`WorkerPanicked =>
   vec![("detail", ...)]`, `catalog_completeness.rs:153`); core stdio grep
   1 hit = the `lib.rs:24` comment, control 6 CLI files. The halves defect
   class that cost Task 3 its blocking round is closed at branch level.
5. *Empty bare `raw:`*: per-arm core tests + the B-2/B-3 controls
   (`validate_semantics.rs:290/:306`), en/de subprocess tests with
   snapshots containing D101's fenced texts verbatim (read both `.snap`
   files), catalog row, and the Run-gate e2e (`smoke.spec.ts:357`,
   disabled + `tooltip-errors` title equality via `enAttr`).
6. *Central sort + BatchView*: the `[B, D, C, A]` order test
   (`report/mod.rs:366`), the parity subprocess test plus the fix-round
   `batch_document`-half test (`dry_run_cli.rs:333/:408`),
   `config_diagnostics[0]` 0/1 (HEAD/fire), the parse-failure apply e2e
   (`smoke.spec.ts:542`) asserting the alert text and zero
   `apply_suggestion`/`save_profile` invocations.
7. *D23 item*: all four `jobsview-reset.spec.ts` tests ran green;
   `gui-d23-reset-gating-form` present in the ledger; `ledger-lint` green.
8. *D49 experiment*: Task 7 report carries the pasted
   control/mutated/restored runs; anomaly branch (G2 red via anti-vacuity,
   G1/G3 green) - settled adjudication, not re-opened. Tree byte-unchanged
   (task-7 verdict's own sha256 check; my `git status` clean).
   `core-d49-g1g2-experiment` correctly absent (grep 0; fired control:
   `core-121-planner-seam-and-hoist` grep 1) - a close action.

**2. Design coverage D91-D105 against the tree:** `pipeline.rs` read in
full - types, fields, docs, S1-S7 order, per-call cache at S6, the D95
default, `job_specs` all match the fences; the four call-site mappings read
in full (CLI `dry_run.rs`, CLI `run.rs:56-259`, `lib.rs:199-221`, src-tauri
`run.rs:241-320`) - presentation, exit codes, settings-read-first and
empty-specs gates all as mapped; D92's doc on `config_only_document`
(`json.rs:79-87`) carries the unified contract; D96's replacement rustdoc
mechanically diffed against the amendment-3 rider fence -
**byte-identical** (25 lines, `diff` clean); the D96 boundary held
(`run_document` + logger-finish caller-side, `TeardownGuard` at src-tauri
`run.rs:455`, `fail_fast` caller-built both sides, tee order
logger-before-`on_event` at `queue.rs:364-368`); D97's three call sites
(`:301`, `:505`, `:511`); D98 (field + doc, plain serialization, the
rewritten licence block `queue.rs:423-432`, `recover_panicked_worker` the
only `Some`-setter); D99 (typed branch at CLI `run.rs:434-461`, all four
Fluent texts grepped and matched character for character); D100
(`JobRow.vue:44-47/:61-64`); D101 (three-branch funnel verbatim at
`validate.rs:412-420`, both arms via the shared funnel, matcher/planner
untouched - `get("")` premise re-read at `matcher.rs`); D102 (hoist +
re-export + `rendered_diag` factoring + builders sort `config_diagnostics`
only); D103 (the `find` line verbatim at `BatchView.vue:225`); D104
(harness fence verbatim in `mount-entry.ts`, hook typed in `global.d.ts`,
the four tests with item 2 on the amendment-5 vehicle -
`toHaveCount(0)` paired with `jobs-empty` visible, and the gating anchors
re-verified at `JobsView.vue:258/:263/:327-328`); D105 (no residue,
nothing committed).

**3. Spec amendments S-1..S-8:** all eight located and text-matched at spec
lines 176, 255, 281, 290, 302, 312, 344, 411. Contradiction sweep re-run:
greps for `raw`, `worker-panicked`, `config_diagnostics`, `mkvmerge_found`,
`cached`, `session` over the spec; `mkvmerge_found` has zero spec mentions
(pattern live: the same grep matched its `cached`/`session` alternates);
4.3's opt-out sentence, 5.5's run bullet (`:310`) and the suggestion
contract (`:294`) all read consistent with S-8's per-call cache. No spec
self-contradiction found. The one code-level S-8 ripple missed is Finding 1.

**4. Cross-task seams:** the Task-1-retains / Task-2-replaces staging of
the CLI inline queue block reached the designed end state; the Task-2-moves
/ Task-3-restates rustdoc seam ended byte-equal to the rider fence; the
Task-3 `panic` wire vs Task-6 e2e fixture agree (required `panic: string |
null` in `ipc.ts:167/:227` forces every fixture, and the smoke live-run
literals carry `panic: null`); the four-writer `smoke.spec.ts` regions
landed disjoint (fixture sweep / Run-gate / apply scenario / `name()`
deletion + import). The caller-vocabulary grep over `queue.rs` re-run:
every hit outside the replaced block is illustrative/non-exclusive, nothing
went caller-stale with the second caller.

**5. Windows/macOS read:** the branch's platform-touching surface is std
`mpsc`/`thread::scope` (portable), `PathBuf::from(".")` (CWD-relative,
portable), the DELETION of a debug-only env read (release builds never read
it on any OS; the debug loss is ruled in D97), the untouched CLI
`cfg(debug_assertions)` gate, serde serialization of an `Option<String>`,
and Node-side `resolve(import.meta.dirname, ...)` (portable). No new
platform branch, no new path parsing, no exit-code change beyond the
designed panic rendering. The cross-target Windows clippy compiled the
whole workspace clean in my run.

**6. The gate, re-run by me, foreground, this session:** all ten parts plus
`ledger-lint` at HEAD `23136b6`, exit 0 each. Aggregates recomputed from
their enumerations, not quoted: `cargo test --workspace` produced exactly
**39** `test result:` lines, all 39 `ok` (grep -c over my saved log);
`pnpm test:e2e` **68 passed** (runner summary in my saved log, with the 4
`jobsview-reset` rows present); `check:i18n` **212 catalog ids**;
`ledger-lint` **515 entries across 4 files, all invariants hold**.

**7. House dimension:** the branch's ledger delta extracted (`git diff
629dc64..HEAD` over the four YAMLs; 25 new entry ids, 5 added occurrence
lines carrying file:line citations - the citation-bearing subset found with
a fired pattern that hits 6 known-present sites in the full file). Each of
the five citation-bearing occurrences spot-verified against the tree:
`identify.rs:388` corrected, src-tauri `run.rs:360` link resolvable,
`commands/mod.rs:14-20` re-export doc names both consumer families,
`smoke.spec.ts` `loadedForApply` drift is a dated record (now `:416`; dated
occurrences are historical, no action). The statements of the five
owner-ruled entries, `cli-08`, `gui-d23-reset-gating-form` and
`gui-identification-cache-per-call-not-per-session` read against the tree:
true of it now, with the expected pre-landing tense ("Today it emits an
info diagnostic, exits 0") that the close's promotion sweep exists to
rewrite. The one house statement the tree contradicts is Finding 2.

**8. No-work-needed checks, premises run:** D103's singleton-envelope
premise (`load_profile_body`'s `Err` arm; `load::from_file`'s two
`ParseError` constructors - cited in the fixture comment, consistent with
the fixture); D92's no-consumed-reason premise (structural at
`pipeline.rs:111-113`); D101's never-match premise (`matcher.rs` `get`
impls; `exact_matches`' raw arm returns no-match on absent property);
D104's not-duplicated orderings premise (the smoke `jobs view: live run`
describe exists at `smoke.spec.ts:582`); D96's completeness-pass premise
(caller-vocab grep re-run, above); D97's no-consumer premise (tree grep,
0/2 with fire); amendment-5's no-bearer premise (the `v-if` disjuncts at
`JobsView.vue:258`, `jobs-empty` as its `v-else`). All hold.

**9. Latitude:** the branch diff over product code grepped for
TODO/FIXME/placeholder/XXX - zero hits (the placeholder-leak guard's own
name excluded). Every invented name I encountered (test fn names, fixture
consts, `muxsmith_localized`) is either plan-fenced or inside the
presentation carve-out. No unenumerated set in a normative position found.

---

## Evidence appendix

Instrument directory (mine, created this session):
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/wbr-independent/`
- `a1-head.txt` / `a1-fire.txt` - funnel-inline grep at HEAD / at `629dc64`
- `a4-head.txt` - core stdio grep at HEAD (the single comment hit)
- `fence-d96.txt` / `tree-d96.txt` - the rider fence vs `queue.rs:328-352`
  (diff: only my sed range's trailing ``` delimiter; 25 doc lines identical)
- `gate-cargo-test.txt` - full `cargo test --workspace` output (39/39)
- `gate-i18n.txt`, `gate-e2e.txt`, `gate-ledger-lint.txt` - gate part logs

Commands run (all foreground, cwd `/home/senol/Git/Muxsmith`, HEAD
`23136b6`): `git status --short`; `git log --oneline 9bbe53d~1..HEAD`;
`git diff --stat/--name-only 629dc64..HEAD` (whole and per-file); `git
show fed55be -- crates/muxsmith-core/src/identify.rs`; `git show
629dc64:crates/muxsmith-core/src/identify.rs` (pre-branch state); the
fire-verified absence greps of section 1 above (each as `git grep` at HEAD
and at `629dc64`); `git grep -n "spec 5.5" crates src-tauri src`; `git
grep -n "per-session\|one session\|per session" -- crates src src-tauri`;
targeted reads of every file named in section 2; `diff` of the D96 fence;
`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D
warnings`; `cargo test --workspace`; `RUSTDOCFLAGS="-D warnings" cargo doc
--workspace --no-deps`; `cargo deny check`; `cargo clippy --workspace
--all-targets --target x86_64-pc-windows-msvc -- -D warnings`; `pnpm
lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e` (which rebuilds
the bundles itself; I fired no frontend mutation, so no separate rebuild
evidence is owed); `python3 scripts/ledger-lint.py`; the ledger-delta
extraction via `git diff` + python over `docs/decision-ledger.yaml`;
`git status --porcelain` (empty) at close.

---

## HARVEST (for the controller)

1. **The two findings and their cheapest vehicle.** Finding 1 is two
   rustdoc edits in `crates/muxsmith-core/src/identify.rs` (`:3-4`,
   `:302`); Finding 2 is one occurrence-text qualification in
   `docs/decision-ledger.yaml` (controller-written file, so it is yours
   either way). They can ride one licensed doc fix round or the close's
   existing text-corrections pass - owner's licence for the latter, since
   the close list currently carries only the three routed items.
2. **Close actions confirmed still open and correctly recorded:**
   `core-d49-g1g2-experiment` absent (needs the controller-composed
   anomaly statement per the progress tracker - D105's two fixed texts
   cover neither measured branch); `core-121-planner-seam-and-hoist` still
   carries its `blocked_on` (clearing it is the recorded close action);
   the five promotion-sweep entries still Tier 1 with pre-landing tense in
   their statements ("Today it emits an info diagnostic, exits 0" is now
   false as present-tense prose and true as the ruling's record - the
   promotion rewrite should fix the tense); the consumed D49 ROADMAP
   trigger line still names only the two clean branches.
3. **The citation re-point sweep at SDD salvage will have real work:** 47
   lines across `docs/decision-ledger.yaml` and
   `docs/process-conventions.yaml` cite `.superpowers/sdd/plan-9/...`
   paths (my count, `grep -c`), all of which move at the salvage.
4. **The three items already routed to the close** (D102's unguarded scope
   boundary with its named cheap producer, BatchView's else-branch text,
   the three `dry_run_cli.rs` assertion strings) verified still open and
   accurately recorded in the ROADMAP, together with the Task-6 LOW-1
   disclosure sentence for the spec-local IPC installer. No sharpening
   measurement from me beyond what the Task-5 reviewer already recorded.
5. **Positive observation worth the close's notice:** acceptance observable
   4's three halves (rendered CLI, rendered GUI, persisted record) plus
   the `--json` wire shape all have named producers after Task 3's fix
   round - the halves-coverage defect class this plan was blocked on once
   is closed at branch level, and the joblog/report_json producers are the
   pattern to point at next time a wire field lands.

---

## Delta review

**Delta verdict: READY.** Both findings close, the third-site correction is
right and the entry is true of the tree, and the three commits introduce
nothing false. One new LOW, explicitly non-blocking, recorded below with its
one-clause remedy. Reviewed at HEAD `e255d40`, tree clean before and after my
runs; delta instruments under
`.../scratchpad/wbr-independent/delta/` (fresh files, none of the fix
implementer's paths re-run).

### 1. Finding 1 closes

Both replacement docs read against the code and against spec line 312 (S-8):

- Module doc (`crates/muxsmith-core/src/identify.rs:1-6`): "so one planning
  call never re-identifies an unchanged file (spec 5.5). The cache is
  constructed per planning call and dropped with it, so separate calls
  re-identify." - matches the spec sentence clause for clause ("One call
  identifies each unchanged file once ... separate calls re-identify") and
  the construction site (`pipeline.rs:127`, stack-local, dropped on return).
- Struct doc (`identify.rs:304-307`): "constructed per planning call and
  dropped with it (spec 5.5)" plus the kept key/staleness/on-disk content -
  same verification.

Propagation into the built rustdoc verified with my own greps after my own
`cargo doc` run: the new phrase present (module index 3, struct page 1), the
old phrases zero on both pages, positive controls fired ("keyed on path" 2
hits). The falsified cross-call claim is gone from source and artifact.

### 2. Finding 2 closes

`docs/decision-ledger.yaml:4650` now reads "the last LITERAL `per-session`
claim in the tree ... and only the literal one: the plan-9 whole-branch
review later found two same-class session-cache claims in the same file ...
which the literal-phrase sweep could not see. A borrowed measurement
answered its own question and the restatement widened it." That is exactly
the qualification my finding required - the measurement scoped to its
pattern, the class residue named, the rule itself correctly left standing
(it was vindicated, not violated, by the finding). Closes.

### 3. The third site (`core-20-ondisk-cache`) is correctly fixed and true of the tree

Graded as a controller artifact: the `statement` now carries the per-call
lifetime (clause-consistent with spec :312 and the code), the correction
note with its date, the pointer to
`gui-identification-cache-per-call-not-per-session` - which I re-verified
exists at `docs/decision-ledger.yaml:4581` with the ruling as its statement,
confirming the coordinator's correction of the fix report's "does not
exist" claim (that claim's stated grep ran over `conventions.yaml` alone; a
scope-of-search error, now on record) - and the untouched deferral (no
on-disk cache in v1). `blocked_on` re-worded to the per-call premise without
weakening the deferral's ground. Occurrences recounted: 3 listed, `count:
3`, the new occurrence factually accurate (I verified its "third artifact
class" narrative against the actual history: code fixed in `fed55be`, docs
in `96dbcf6`, this entry in `e255d40`). `ledger-lint` green in my own run
(515 entries, all invariants hold). The inline correction narrative inside
`statement` matches the house pattern (`core-121` carries its
measured-correction the same way). Correct.

### 4. Nothing new introduced

All three diffs read in full: each touches exactly its declared scope (one
occurrence line; two rustdoc blocks; one entry's statement/blocked_on/count/
occurrences). No product behavior, no test, no other file. My fresh
verification subset on the delta: `cargo fmt --all --check` OK,
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` exit 0,
`cargo clippy --workspace --all-targets -- -D warnings` exit 0,
`python3 scripts/ledger-lint.py` 515/all-hold, `git status --porcelain`
empty. Typography of all added text clean (ASCII hyphens, straight quotes).
The one wording-level residue is the LOW below.

### 5. The fix report's "nothing else" sweep: method sound, conclusion true

I ran the premise with my own, deliberately different method rather than
weighing the report's. Two passes, both fired before being believed:

- A line-based pattern-family sweep (`session|shared cache|between dry|
  dry.?run and run|across (call|command|invocation)s?|warm.*cache`) over
  crates/src/src-tauri/e2e/locales and separately over the four house YAMLs
  (`.../delta/class-sweep-code.txt`, `class-sweep-house.txt`): 4 code hits,
  9 house hits, every one judged - all either a different sense of the word
  (Tauri window session, MRU session, instrument-scratchpad session, test
  fixture pool, a component-sharing note) or the correction/ruling records
  themselves. No stale claim.
- A wrap-insensitive phrase sweep (python, doc-comment continuations
  collapsed before matching - my line-based pass would have MISSED the old
  module-doc phrase, which wrapped across lines 3-4; the report's
  single-token Pass B did not have this blind spot, but mine did, so I
  closed it), fired on the pre-fix tree where it finds exactly the two
  finding-1 sites (3 phrase hits), then run at HEAD over all product code:
  4 hits, all judged not-the-class
  (`.../delta/wrap-insensitive-sweep.txt`).

Conclusion: **no fourth site of the class exists in product code or the
house files.** The report's three-pass method is sound (its Pass A count
match, its fired Pass B/C) and its conclusion is true by independent
reproduction.

### New finding (delta)

**Finding 3 (LOW, non-blocking): "constructed per planning call" reads as
an exclusive constructor claim; the identify surfaces also construct the
cache.** `IdentifyCache` has two production constructors outside the
planning seam: CLI `muxsmith identify`
(`crates/muxsmith-cli/src/commands/identify.rs:21`) and the GUI
`identify_body` (`src-tauri/src/lib.rs:255`), each per identify invocation,
neither a planning call. The corrected docs' load-bearing semantics
(per-call lifetime, dropped with the call, no cross-call reuse) hold at all
four sites, and spec 5.5 - which both docs cite - governs the planning
cache, so nothing is false in the sense finding 1 was. But under the
branch's own Tier-1 rule
`core-docs-name-callers-illustratively-never-exclusively` (state the
property, then illustrate), "constructed per planning call" on the TYPE's
doc names one construction context as if it were the definition, and the
falsifying second caller already exists today. Honest provenance note: my
finding-1 required-change proposed this exact example wording, so the
imprecision is seeded by my verdict, not invented by the implementer.
Remedy, one clause, whenever a licensed text pass next touches the file
(the close's text-corrections pass qualifies): struct doc "constructed per
call and dropped with it (per planning call in the pipeline seam, per
invocation on the identify surfaces; spec 5.5)" or equivalent. Not worth a
dedicated round; recorded so it does not resurface as someone else's
discovery.

### Delta evidence appendix

Fresh instruments:
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/wbr-independent/delta/`
- `class-sweep-code.txt` / `class-sweep-house.txt` - line-based class sweeps
- `wrap-insensitive-sweep.txt` - the normalized phrase sweep with its
  pre-fix fire
- `gate-subset.txt` - fmt/doc/clippy/ledger-lint outputs on the delta

Commands: `git log --oneline -5`; `git show <sha> --stat` and full `git
show <sha> -- <file>` for `b40db26`, `96dbcf6`, `e255d40`; read of
`.superpowers/sdd/plan-9/whole-branch-fix-report.md`; `git grep` for
`IdentifyCache` construction sites; the two class sweeps above (line-based
via `git grep`, wrap-insensitive via python over `git ls-files`, fired at
`23136b6`); reads of `commands/identify.rs`, `lib.rs:245-265`, the
`core-docs-name-callers-illustratively-never-exclusively` statement, the
`gui-identification-cache-per-call-not-per-session` entry; `cargo fmt
--all --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace
--no-deps`; `cargo clippy --workspace --all-targets -- -D warnings`;
`python3 scripts/ledger-lint.py`; built-doc greps over
`target/doc/muxsmith_core/identify/{index.html,struct.IdentifyCache.html}`
(negatives paired with fired controls); `git status --porcelain` (empty).

### Delta HARVEST

1. Finding 3's one-clause sharpening is a candidate for the close's
   existing text-corrections pass; it needs no vehicle of its own.
2. The fix report's mis-scoped existence check ("the id does not exist",
   grepped over one file) is a live instance of the search-scope half of
   the absence-check discipline: the grep was fired against a control in
   the same file, which proved the pattern but not the scope. If the
   controller mines the fix round, that is the occurrence-worthy lesson;
   the coordinator already caught it operationally.
3. The unpushed state: three commits sit on local master past the pushed
   gate run. The standing rule (`ledger-lint-runs-before-every-push`, gate
   binds every push) means the close's push needs the full ten-part gate
   re-run; my delta subset (fmt/doc/clippy/ledger-lint) covers the
   doc-only delta's risk surface but is not the gate.
