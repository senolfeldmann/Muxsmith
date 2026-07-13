# House-knowledge clusters - domain `cli`

Reconstructed from 39 occurrence records spanning eras E0-E8. Records grouped by identical `(topic, approach)`; occurrences merged, identical `date+ref` deduped, distinct `date+ref` kept as genuine recurrence. `promoted = count >= 3`, `promoted_at = 3`.

Era-to-time anchoring (from the sibling `cross` reconstruction and refs): E0/E1 = 2026-07-08 (Plan 1, T12); E2 = 2026-07-09/10 (independent review + fix-pass); E5 = Plan 4 (memos D15/D16, task-N-review-verdict, whole-branch, ~2026-07-10); E7 = later plan cycle (task-N-verdict.md, distinct from E5's task-N-review-verdict.md); E8 = CONVENTIONS.md codification. Where a record carries no calendar date the era label is used as the temporal marker rather than inventing a date.

Two clusters clear the promotion threshold: `cli-02` (config diagnostics reachable in dry-run, count 4) and `cli-09` (`--on-collision` parity, count 3). No count was padded: the two E0/E1 ordering records collapse to one T12 occurrence (same commit `ad841b0`, same 2026-07-08 journal, same task-12 verdict), and every decided->reinforced pair (memo + review-verdict) is two genuinely distinct artifacts.

---

## cli-01-diag-ordering - Diagnostics render error-first (severity-sorted) on every surface via one shared fold
- **kind:** pattern | **status:** contested | **count:** 2 | **promoted:** no
- **Statement:** Diagnostics are sorted error-first (Reverse(severity)) exactly once, before the text/json branch, so every surface derives from the same sorted vector. Violated on the `--json` path (emitted in raw insertion order while text sorted, so `diagnostics[0]` could be a warning under exit 2), corrected at T12. Later codified as a CONVENTIONS pattern, but the same idiomacy sweep flags `validate.rs:L19` still re-implementing the shared sort/exit-fold as a deviation to correct - the ordering-consistency issue kept resurfacing across surfaces and is not uniformly closed (see also cli-08, the deferred `config_diags` JSON-ordering parity gap).
- **Steelman:** null
- **Occurrences:** (E0 and E1 records are the same T12 fix event -> one occurrence)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | violated-corrected | commit ad841b0 + task-12-review-verdict.md (Important #1) + journal 2026-07-08 (T12) | "--json output is not sorted error-first, unlike the text renderer ... a real, demonstrable inconsistency." Fixed by sorting once before the text/json branch. |
| E8 | reinforced | CONVENTIONS.md Patterns (b38a46f) + idiomacy finding validate.rs:L19 | Codified worst-first via shared `severity_sorted`/one fold on every surface (origin Plan 1); finding flags validate.rs:L19 re-implementing the shared sort as a deviation to correct. |

---

## cli-02-dryrun-config-diags - dry-run runs config-time validate+lint and surfaces those diagnostics on every path and renderer (spec 5.5)
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Dry-run must run `profile::validate` + `lint::provable_overlaps` and fold the resulting config-time diagnostics in on **every** code path (happy path, mkvmerge-not-found, mkvmerge-query-failed) and **both** renderers (human + JSON). Caught and enforced repeatedly across eras as the same defect class: bug A (dry-run never ran validate/lint at all -> broken regex returned empty + exit 0), then the mkvmerge-not-found branch dropping them (waved off as a judgment call, FAILED on spec 5.5, branch then found testable via PATH override), then the query-failed path (deferred out of F1 scope), then the query-failed human path (found not-deliberate and fixed). Hard-won but resolved by E7: JSON error-path handled via cli-17 (3f66a4e), human path via task-9-verdict (vii).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | independent-review-2026-07-09 bug A + journal fix-pass + commit b507f6e | "dry-run never runs validate()/lint -> config-time diagnostics unreachable; broken regex returns empty + exit 0." Spec 5.5 made explicit. |
| E2 (fix-pass) | violated-corrected | journal fix-pass F1 + progress.md ledger F1 + commit 09d7244 | "reviewer FAILED spec on the mkvmerge-not-found path silently dropping config diagnostics - the implementer had explicitly waved it off ... the fixer then found the branch WAS testable (PATH override)." |
| E2 (fix-pass) | deferred | plan-2-fixes-sdd/progress.md ledger F1 residual + journal fix-pass Open threads | "the mkvmerge-query-failed path (list_languages fails) has the same defect - config diags dropped - left out of F1 scope." |
| E7 | violated-corrected | task-9-verdict.md (vii) | Query-failed human path found NOT deliberate (F1 commits were --json-scoped, spec 5.5 unconditional); fixed to print config diagnostics first inside the human else-branch, JSON untouched. |

---

## cli-03-human-file-attribution - Human renderer includes the diagnostic's file field
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `Renderer::diagnostic` must include `d.file` when present so batch-level diagnostics (`IgnoredFile`, `DuplicateIdentifier`) are attributed to their own file rather than misread as belonging to the file shown above. The inline human renderer dropped the field (bug J); the JSON path was unaffected.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | independent-review-2026-07-09 bug J + F9-report.md + commit 2e0dc00 | "Human renderer drops the file field -> batch-level diagnostics (IgnoredFile, DuplicateIdentifier) print with no file attribution." |

---

## cli-04-json-always-parseable - --json always emits a parseable document, even on error paths
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** A machine-readable mode that emits unparseable stdout defeats `--json`; the profile-load and list-languages error paths violated this by writing non-JSON to stdout. Flagged and deferred to a fast-follow at T9 (the correct diagnostic-folding `collect()` pattern already existed in `validate.rs`), then corrected: run and dry-run now emit a JSON document on profile-load and list-languages failures.
- **Steelman:** null
- **Occurrences:** (deferred -> corrected across two artifacts)

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | deferred | task-9-review-verdict.md | Pre-existing (byte-identical at base 2478520), outside the brief's 2 TODO markers; "a --json caller on a malformed profile gets unparseable stdout." |
| 2026-07-10 | violated-corrected | commit 3f66a4e + journal 2026-07-10 Plan-4-complete | "4 RED->GREEN tests incl. a fake-mkvmerge stub; controller ruled it a bugfix not scope change ('a machine-readable mode emitting unparseable stdout defeats --json'); pattern taken from validate.rs." |

---

## cli-05-json-final-document - --json emits one final document (diagnostics + per-job results + summary)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** `--json` emits a single final document carrying planning diagnostics, per-job results and summary counts; human progress lines are suppressed in JSON mode. The `run_json_document` builder is shape-exact and unit-tested via `serde_json::Value` equality.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | "--json emits one final document ... Human progress lines are suppressed in JSON mode." |
| E5 | reinforced | task-9-review-verdict.md | "run_json_document shape matches the brief; builder unit-tested via serde_json::Value equality, transcribing the brief example literal down to duration_ms:12400." |

---

## cli-06-mkvmerge-found-honesty - mkvmerge_found reports honestly (omit / tri-state)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** `mkvmerge_found` must reflect what actually ran, as a three-way signal: `Some(true)` = binary found but query failed, `Some(false)` = lookup ran and failed, absent = lookup never ran. Two corrections: omit the key on the profile-load path (lookup never ran, so it must not assert `false`), and emit `true` on the list-languages-failure arm (locate() succeeded, only the query failed).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | violated-corrected | whole-branch-review-verdict.md + commit db9f559 | `config_only_json` parameterized to `Option<bool>`: key absent on the profile-load path (lookup never ran); accurate locate-failure `false` assertions left untouched. |
| E5 | violated-corrected | commit 9009d34 (whole-branch ledger #10) | List-languages-failure arm sets `mkvmerge_found=true`; doc spells out the three-way semantics; flagged separately by Fix-2, routed as its own accepted commit. |

---

## cli-07-ndjson-deferred - Streaming NDJSON event mode deferred to v1.x
- **kind:** non-decision | **status:** blocked | **count:** 2 | **promoted:** no
- **Statement:** A streaming NDJSON event mode (`--json-events`) riding the D13 enum is a v1.x candidate, explicitly not built in Plan 4. Reviewer confirmed no NDJSON in the diff.
- **Blocked on:** external - v1.x demand.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | deferred | memo D15 | "A streaming NDJSON event mode ... is recorded as a v1.x candidate riding the D13 enum - explicitly not built in Plan 4." |
| E5 | deferred | task-9-review-verdict.md | "git diff 2478520..77317a0 | grep -in ndjson" empty; no NDJSON anywhere in the diff. |

---

## cli-08-config-diags-json-ordering - Sort flat config_diags JSON for parity with validate (deferred)
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The flat `config_diags` JSON is unsorted, inconsistent with validate's sorted JSON; sorting for parity was deferred because consumers key on the severity field and ordering is cosmetic. Same theme as cli-01 but a distinct, deliberately-deferred decision on a specific surface.
- **Blocked on:** internal - Plan 6 / idiomacy sweep.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E7 | deferred | task-9-verdict.md (iv) + whole-branch funnel | "flat config_diags JSON unsorted vs validate parity; consumers sort by severity field." |

---

## cli-09-collision-parity - --on-collision exposed on both run and dry-run
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `--on-collision <error|skip|overwrite>` is exposed on both `run` and `dry-run` for parity, via a CLI-local `CollisionArg` (core stays clap-free) mapping 1:1 to `CollisionPolicy` and threading into the pre-existing `RunInputs.on_collision`. `run` reuses `CollisionArg` verbatim; the rerun workflow (D14/D17) is where the flag is needed.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | Spec 4.2 names the override as a run input; `RunInputs.on_collision` existed since Plan 2 but no flag exposed it, and run is where the rerun workflow needs it. |
| E5 | reinforced | task-4-review-verdict.md | `CollisionArg` CLI-local, 1:1 to `CollisionPolicy`, threads into `RunInputs.on_collision` (planner.rs:249 fallback). |
| E5 | reinforced | task-8-review-verdict.md | Run's clap variant reuses `CollisionArg` verbatim; the rerun workflow (D14/D17) needs the flag. |

---

## cli-10-run-exit-fold - run exit code is the worst-of fold over diagnostics and job states
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** `muxsmith run`'s exit code is `max(diag_exit_code, job_exit_code)` (error/Failed->2, warning/Warning->1, else 0), mirroring dry-run, with the 130 override structurally ahead. `fail_fast`'s Cancelled jobs still fold to 2 since the caller cancel flag is untouched.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | "Exit code of muxsmith run is the worst-of fold, mirroring dry-run." |
| E5 | reinforced | task-8-review-verdict.md | `max(diag_exit_code, job_exit_code)` with 130 override ahead; fail_fast's Cancelled jobs still fold to 2. |

---

## cli-11-sigint-exit-130 - SIGINT-cancelled batch exits 130
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A SIGINT-cancelled batch exits 130 (128 + SIGINT, shell convention) so scripts can distinguish user cancellation from mux failure.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | "A SIGINT-cancelled batch exits 130 (128 + SIGINT, shell convention), so scripts can distinguish user cancellation from mux failure." |

---

## cli-12-exit-doc-drift - CLI exit-code doc kept in sync (includes 130)
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `Cli::command`'s exit-code doc was stale (missing 130 from T10) and corrected: "0 clean / 1 warnings / 2 errors" -> "... / 130 cancelled (spec 8.1, D16)". A cross-task doc-drift catch.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | violated-corrected | whole-branch-review-verdict.md + commit 841db0b | "'0 clean / 1 warnings / 2 errors' -> '... / 130 cancelled (spec 8.1, D16)'; cross-task doc drift from T10." |

---

## cli-13-ctrlc-crate - SIGINT handled via the ctrlc crate setting the shared cancel flag
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** SIGINT is handled with the `ctrlc` crate (cross-platform, including Windows console events); its handler sets the queue's shared cancellation flag. One `Arc` is shared across `handler_cancel`/`queue_cancel`/`cancel`; the vendored `ctrlc-3.5.2` `os_handler` only calls `sem_post` (async-signal-safe).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D16 | "The ctrlc crate (cross-platform, including Windows console events) installs a handler that sets the queue's shared cancellation flag." |
| E5 | reinforced | task-10-review-verdict.md | One Arc across handler_cancel/queue_cancel/cancel; reviewer read vendored ctrlc-3.5.2 source (os_handler only sem_post -> async-signal-safe). |

---

## cli-14-reject-handrolled-signals - Hand-rolled cross-OS signal code rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Hand-rolling `libc`/WinAPI signal handling across three OSes was rejected in favour of one vetted crate.
- **Steelman:** Zero new dependency, but std has no stable signal API and hand-rolled signal code across three OSes is exactly the fragile platform branching the project avoids.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D16 | "std has no stable signal API, and hand-rolled libc/WinAPI signal code across three OSes is exactly the fragile platform branching the project avoids." |

---

## cli-15-sigint-single-level - Single-level SIGINT (second Ctrl-C force-exits)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** SIGINT handling is single-level: `AtomicBool::swap` arms graceful cancel on the first Ctrl-C, and a second Ctrl-C during cleanup forces immediate `std::process::exit(130)`. No two-stage graceful/hard scheme.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D16 | "A second Ctrl-C during cleanup forces immediate exit; no two-stage graceful/hard scheme." |
| E5 | reinforced | task-10-review-verdict.md | `AtomicBool::swap`: first SIGINT arms graceful cancel, second `std::process::exit(130)`. |

---

## cli-16-ctrlc-pin-deferred - Pin ctrlc to a full patch version (deferred)
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** `ctrlc` is pinned to major only ("3") unlike siblings' full-patch pins (`clap=4.6.1`, `serde_json=1.0.150`); folding it to a full pin is deferred to the next `Cargo.toml` touch. `Cargo.lock` pins `3.5.2`; a brief-literal style inconsistency.
- **Blocked on:** internal - next Cargo.toml edit.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | deferred | task-10-review-verdict.md + whole-branch ledger #12 | Cargo.lock pins 3.5.2; siblings pin full patch (clap=4.6.1, serde_json=1.0.150); style inconsistency, brief-literal. |

---

## cli-17-render-ownership - Queue on a scoped side thread; live rendering drains events on the calling thread
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The queue runs on a scoped side thread; live rendering drains events on the calling thread. The sole `Sender` is moved into the scope closure so the drain loop terminates deterministically when the batch finishes; std `mpsc` is unbounded so sends never block.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | reinforced | task-8-review-verdict.md | Sole Sender moved into the scope closure for deterministic drain-loop termination; std mpsc unbounded so sends never block. |

---

## cli-18-milestone-lines - Human run progress = dependency-free per-job milestone lines
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Human mode renders dependency-free per-job milestone lines (`[i/n] name ... start/25/50/75/terminal`) plus a one-line batch summary; pipe-safe, no TTY branching, interleaves correctly under `--jobs N`. Milestone thresholding is per-index (`last_milestone: Vec<u8>`), the renderer returns `Vec<String>` (I/O-free), unit-tested at the seam.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | Per-job `[i/n] name` start/25/50/75/terminal + one batch summary; "pipe-safe, no TTY branching, interleaves correctly under --jobs N." |
| E5 | reinforced | task-8-review-verdict.md | Per-index `last_milestone: Vec<u8>`; 13 unit tests over multi-threshold jump/repeat/regression/per-index isolation; renderer returns `Vec<String>`, I/O-free. |

---

## cli-19-reject-indicatif - Live ANSI progress bars via indicatif rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Live ANSI progress bars via `indicatif` were rejected for run progress rendering.
- **Steelman:** Nicer interactive UX, but adds a dependency, needs TTY/non-TTY special-casing, and duplicates what the Plan 5 GUI job view does properly.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | "Rejected: live ANSI bars via indicatif (new dependency, TTY/non-TTY special-casing, duplicates what the Plan 5 GUI job view does properly)." |

---

## cli-20-reject-quiet-output - Quiet start/end-only output rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Quiet start/end-only output was rejected for run progress rendering.
- **Steelman:** Minimal output, but gives no liveness signal on long muxes.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | decided | memo D15 | "Rejected: ... quiet start/end-only lines (no liveness signal on long muxes)." |

---

## cli-21-empty-batch-output - Empty-batch human output is never silent (always print the batch summary)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** A clean empty run must not be a silent success. At E5 human mode on a clean empty run (exit 0) printed nothing, flagged as a spec-level gap (D15 unspecified) and deferred to Şenol. Resolved at E7: `print_batch_human` always prints the batch summary line even at zero matches, threading the real search root and extensions; the JSON path and exit code are unchanged.
- **Steelman:** null
- **Occurrences:** (deferred gap at E5 -> decided at E7)

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | deferred | whole-branch-review-verdict.md (Minor) + progress backlog | "run --json prints a zeroed summary; human mode on a clean empty run (exit 0) prints nothing at all - a silent success. Plan/spec-level gap (D15 does not specify)." |
| E7 | decided | task-8-verdict.md + plan T8 | "unconditional summary print at the tail of print_batch_human ... exit code untouched; JSON path untouched." |

---

## cli-22-jobs-index-doc-deferred - Document that jobs[].index indexes the queue, not files (deferred)
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** `jobs[].index` indexes the queue, not files; correlating a job to its file requires joining on the output path. One doc sentence in `run_json_document` (or a source field later) prevents consumer misreads in Plan 5; deferred until Plan 5 consumes it.
- **Blocked on:** internal - before Plan 5 consumption.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E5 | deferred | whole-branch-review-verdict.md (Minor) + progress backlog | "correlating a job to its file requires joining on the output path. One sentence in run_json_document's doc (or a source field later) prevents consumer misreads in Plan 5." |

---

## cli-23-identifyerror-passthrough - IdentifyError English detail kept as third-party pass-through
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `IdentifyError`'s English detail text is kept (not routed through a catalog key) and added explicitly to the spec §8.4 exception list; the exception is noted as slightly broader than pure pass-through.
- **Steelman:** Third-party text pass-through like spec 8.4's clap exception - routing it through the catalog would re-author third-party wording.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| E7 | decided | task-9-verdict.md (viii) | "KEEP defensible, exactly one §8.4 entry." |

---

## Clustering notes (defensibility)

- **cli-01 count = 2, not 3:** the E0 record ("CLI/JSON surface parity") and the E1 record ("diagnostic output ordering") cite the *same* fix - commit `ad841b0`, journal 2026-07-08, task-12-review-verdict - so they collapse to one T12 occurrence per the dedup rule. The E8 CONVENTIONS codification is the only genuinely distinct second artifact. The `origin Plan 1` mention in the E8 evidence is not a separately cited dated artifact, so it was **not** counted as a third occurrence.
- **cli-01 status = contested (not settled):** the ordering-consistency issue kept resurfacing across surfaces (--json at T12, validate.rs:L19 flagged as a still-uncorrected deviation at E8, plus the deferred config_diags parity in cli-08). The rule is codified but not uniformly enforced -> "kept coming back, not fully resolved."
- **cli-02 count = 4, the largest merge - and defensible, not padded:** the records cross-reference each other as one defect class ("the same defect - config diags dropped"; "the same defect class as the F1 not-found fix"). Bug A is the root (dry-run never validated), the not-found and query-failed paths are the same spec-5.5 requirement violated on progressively more code paths, and the E7 fix closes the last (human query-failed) path. This is exactly the cross-era recurrence the clustering is meant to surface. Status is **settled** (resolved by E7), not blocked/contested, because no open item remains: the JSON error-path was closed by cli-04 (3f66a4e) and the human path by task-9-verdict (vii).
- **cli-04 vs cli-02:** kept separate. cli-04 is about the JSON document being *emitted at all* on error paths (parseability); cli-02 is about config *diagnostics content* being surfaced. They touch the same list-languages/query-failed path but enforce different requirements - merging would conflate two distinct considerations.
- **cli-08 vs cli-01:** kept separate. Both concern diagnostic ordering, but cli-08 is a deliberately-deferred non-decision on the flat `config_diags` JSON surface, whereas cli-01 is an adopted-and-codified pattern. Folding a deferral into a pattern cluster would misreport its status.
- **cli-11 / cli-12 / cli-15 all touch the value 130 but are three distinct (topic, approach) clusters:** the SIGINT exit-code *decision* (cli-11), the *doc* being kept in sync (cli-12), and the single-level SIGINT *mechanism* whose second-Ctrl-C path exits 130 (cli-15). Same constant, different decisions - not merged.
- **cli-21 (deferred E5 -> decided E7):** the E5 "nothing-to-do line" gap and the E7 "always print the batch summary line" decision are the same thread (empty-batch human output must not be silent); the E7 decision resolves the E5 deferral, so the cluster is a settled pattern, not a standing non-decision.
- **decided -> reinforced pairs (cli-05, cli-06, cli-09, cli-10, cli-13, cli-15, cli-18):** memo D15/D16 (the decision) and the task-N-review-verdict (independent re-attestation, usually with source-verified specifics) are two distinct artifacts -> count 2 (or 3 for cli-09, which additionally re-attests on both the run and dry-run surfaces). No same-document double-count.
