# Whole-branch verdict - Plan 11 (5378264..245a51a)

**Verdict: READY_WITH_MINORS.**

Graded in `/home/senol/Git/Muxsmith` on `master` at `245a51a`, read-only; no
worktree entered; independent instruments under
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/wbrev-independent/`.
Tree proven byte-identical before/after (fingerprint diff empty: tracked state,
diff-to-HEAD hash, untracked-unignored list). The one file appended outside the
repo's tracked/unignored surface is `gh-log.md` (git-ignored), carrying the
mandatory log line for one read-only `gh api` call.

Every shipped product byte this branch changes was verified correct: the
comparator pair and its call site, the three tests, the spec's 4.3/4.4/7/8.1/9.2
text, README, both help topics, the `deny.toml` comment and its two keys, the
`ci.yml` and fixture comments, `BUILDING.md`, and the lockfile move. No finding
requires a change to any product file. The minors are: one house-convention miss
only a whole-branch view could see (finding 1), and the audit result that a
defined set of close actions is still outstanding between here and the push
(findings 2-4) - all of them already ordered by the plan's own close section,
none new work, enumerated below so nothing falls out at the file boundary.

## Findings

1. **MINOR (house: `agent-commit-trailer-set`, `docs/process-conventions.yaml`) -
   the two merge commits carry no agent trailer.** `82bd016` and `73d3de2` have
   no `Co-Authored-By` trailer; every non-merge commit in the branch has exactly
   the mandated one, and every Plan-8 merge commit carries it
   (`2afa39d`, `cd9c56a`, `fcb0795`: `Claude Fable 5`). The entry is owner-ruled
   MANDATORY for every agent-authored commit, with agent provenance as its
   stated reason - these two merges now read as Şenol's own work. Evidence:
   `git log --format='%(trailers:key=Co-Authored-By,valueonly)'` over all 18
   branch commits and over the Plan-8 merges.
   **Triage: deferrable with a named vehicle, and the vehicle is the close's
   ledger pass.** A history rewrite would re-SHA five commits (`82bd016` and
   everything above it) that the tracker, verdicts and ROADMAP already cite;
   the house precedent (six session-16 commits lack the trailer and were
   recorded, not rewritten) is to record. Required change: a
   `violated-corrected`-style occurrence on `agent-commit-trailer-set` naming
   both SHAs, plus the journal stating it, before the push. Present the rewrite
   option to the owner only if he wants provenance over SHA stability.

2. **MINOR (process: verdict-arrival mining) - Task A4's verdict harvest is the
   one not yet mined.** `83af0d5` mines A1/A2/B1's harvests; A3's routed items
   landed in `2fb528f`; no ledger or ROADMAP commit carries A4's items 6-10
   (measured: ROADMAP's Reach-claim section has no 8.1-vs-`--help` member;
   `column 28` 0 hits; no deferral row or ROADMAP line for `README.md:194`'s
   unscoped 130-shell claim; no sweep-value ledger line). The close's audit duty
   exists for exactly this; this is that audit firing.
   **Triage: must land before the push** - A4 harvest items 6, 7, 8, 9, and 10
   (the last at the controller's discretion, as the harvest itself says).

3. **MINOR (promised carriers) - two ROADMAP lines the plan's deferral table
   promises "at the plan close" do not exist yet.** The parked-guard Triggers
   line (`unused-ignored-advisory` 0 hits, "unsound is still set" 0 hits,
   newline-flattened) and the 17-defaulted-fields line (`17 defaulted` 0 hits
   flattened). Both are the named interim carriers of parked/deferred items;
   until they exist, deferral rows 6 and 8 have no vehicle on disk.
   **Triage: must land before the push**, with the other ROADMAP dispositions.

4. **MINOR (routed-but-unrecorded) - four A3-review close records are still
   outstanding**: the D111-M4 dead-pathspec expression record (`M4` and
   `match nothing under git` 0 hits in the ledger), the D111-T6 note that its
   two ROADMAP citations are stale by nine lines, the corpus-discriminator
   false-positive routing to the example-validation vehicle, and the T7
   strengthening note. **Triage: must land before the push** (ledger/notes;
   ROADMAP:1799's "gate part 4" from A1 minor 2 and the A1-adjudication-5
   vehicle reconciliation from the amendment verdict ride the same pass).

5. **INFO - the `cli.rs` deferral's vehicle is an unwatched event.** "Whichever
   package next edits `crates/muxsmith-cli/src/cli.rs`" has no trigger anyone
   meets - the same argument the parked-guard row already accepted for itself.
   A4's harvest item 4 asked this review to decide: **give it a ROADMAP line at
   the close** (one sentence, the A4 Step-6 measurement attached). Deferrable;
   the close's disposition pass is the vehicle.

No finding of severity above MINOR. Specifically checked and clean: no
statement one task made true and another made false survives outside the
named pending dispositions; no document contradicts a sibling on the `raw:`
semantics (checks R', K' and the vocabulary sweep re-derived, plus a read of
the spec's retained second `:421`-family occurrence against its amended first);
no test this plan added passes for the wrong reason (mutation evidence below);
no over-claim of 1.0 completeness, of the `glib` alert being fixed, or of the
line-citation class beyond "tracked files outside `docs/`" exists in any commit
message, spec text, comment or ROADMAP line (the `5d305a2` subject omits
"tracked"; measured zero untracked non-`docs/` files, ruled coinciding by the
A2 review; the close disposition must carry the word, per A2 INFO 3).

## The five adjudications

1. **Was the amendment the right instrument? YES.** One-pair scale was correct:
   no task added, removed or re-cut (the doctrine's own discriminator, and
   Amendment 4 proves the scaling discriminates - it took the four-role form
   when A3 WAS re-cut). Batching six defects beat per-finding fixes: one
   author, the original reviewer, one delta review - which then caught the two
   things that mattered (the insert-vs-replace operation that would have died
   at `duplicate key: unsound`, fired in both readings; the 77/78 state
   confusion). **Nothing was lost between finding and applied text:** the
   shipped `deny.toml` comment is byte-identical to the B1 verdict's fenced
   wording (diffed), the postcondition holds (`grep -c '^unsound = '` -> 1),
   every sentence of the fence re-verified true at cargo-deny 0.19.9's own
   `cfg.rs`/`advisories.rs`, and all six defect repairs are present at their
   full site sets. The amendment's own two new defects were the same class it
   repaired, caught by its fix round - the pair held.

2. **Do the errata work? YES.** Read cold: A4 §1d (the "full enumeration"
   exit-site block) and B1 Step 9 plus its Findings list. Each block leaves the
   original legible, states what was claimed, what was measured (with the
   re-run pasted), and which is true now; each sits directly beneath the text
   it refutes, blockquoted, so a section reader cannot take the original at
   face value; A4's report additionally front-loads a header instructing any
   quoting reader to grep for `ERRATUM` first. Spot-verified: A4's corrected
   exit-site paste reproduces exactly (10 lines; sorted-set
   `md5 62f6101fab141a0ea79bbeca95e8e56e`, identical under the wrapper grep and
   `/usr/bin/grep`); every `silent` assertion in B1's report is either
   erratum-covered or accurate. The dated-record ruling produced records that
   inform rather than mislead.

3. **The parked owner decision: PARKING IS RIGHT.** The branch ships no gap:
   the shipped state is green and correct, and the guard question concerns a
   future edit, not this tree. The knob is genuinely double-edged -
   `unused-ignored-advisory = "deny"` also reddens the gate when an ignored
   advisory legitimately disappears upstream, the exact event the v1.x `glib`
   entry watches - and at defaults the dropped-key state is warn-visible
   (`warning[advisory-not-detected]` naming the line), not silent. Gate
   coverage is owner-visible surface by this project's own rules, and the
   session's parking ruling exists for precisely this shape: decision memo plus
   measurement, batched for his return. **Condition: the interim carrier must
   actually exist** - the Triggers line is finding 3 and lands before the push.

4. **Gate coverage: STATED WHERE A READER LOOKS, AND ACCURATELY.** The first
   place a future reader looks is `deny.toml` itself, whose comment states the
   per-class scope semantics, the ignore's reason, and the ignored-not-fixed
   status; every claim in it re-verified at the tool's source and by run
   (three-way fire on the merged state: shipped exit 0; scope-on/ignore-out
   exit 1 with exactly `RUSTSEC-2024-0429`; both-out exit 0 - and
   `transitive` behaves identically, as the comment claims). The v1.x `glib`
   ROADMAP entry carries the same distinction in full context ("ignore this
   one advisory with its reason... both may be quoted as coverage again").
   At `-L info` the merged gate now shows 19 ignored / 16 unmaintained /
   2 vulnerability notes with `RUSTSEC-2024-0429` visible as an ignored note -
   covered-and-ignored, not silent. The pending two-alerts disposition is
   already specified to keep the distinction; both alerts verified still open
   on GitHub (the `postcss` one closes only when the push lands).

5. **Anything unfinished only this review could see? ONE artifact-level item
   and one audit result.** The artifact item is finding 1 (merge-commit
   trailers) - controller-performed merges sit outside every task checklist,
   which is exactly why no per-task review saw it. The audit result is
   findings 2-4: a complete, named, pending close-action set - ordered work,
   not lost work, listed so the close can be checked against it. Beyond that:
   the safeguard genuinely guards (stripping `scalar_eq`'s cross arms on a
   scratch copy fails exactly one test across every muxsmith-core and
   muxsmith-cli suite, `--no-fail-fast`: `typed_exact_still_cross_compares_int_and_float`;
   `b7_raw_does_not_cross_compare_int_and_float` and
   `raw_compares_only_within_one_kind` still pass under that mutation, as
   designed; the gui/xtask crates reference none of the comparator internals,
   fired control on core); the known wrong-reason test
   (`numeric_exact_compares_across_int_and_float`, which stayed green under my
   mutation) is pre-existing, named, and has its vehicle written into Plan 13's
   floor in the ROADMAP; and every deferral's vehicle exists on disk except the
   two that are finding 3 and the unwatched event that is finding 5.

## Dimension record (compressed)

- **Coverage:** all 5 work items, all 40 acceptance halves walked; every named
  producer exists and produced; producer-less rows: zero. Independently
  re-verified on the merged tree (not from reports): W1-a/b/c/e/f/g/h/i/j/k/l/m,
  W2-a/b/c/d/e, W3-a/b/c/f/g/h/i/j/k, W4-a/b, W5-a/b/c/d. Accepted on
  reviewer-verified evidence (prescribed fires already graded per task):
  W1-d, W2-f, W3-d/e/l/m, W4-c, W5-e.
- **Spot-re-runs of load-bearing pastes (all reproduce, sets compared, never
  text):** A3's split - R' 8 lines/6 files pre (member-for-member) -> 0 merged;
  K' 7 lines/6 files on BOTH states, member-for-member D111 §4.4's table
  (line shifts only); vocabulary sweep 71 -> 66. A1's ordinals 3 -> 0 and
  over-80 line 1 (`:138`, 86) -> 0. A2's expressions A and B 1+1 -> 0+0, with
  fires. B1's lockfile sites (4x `8.5.25`), note tally, three-way fire. A4's
  five-subcommand flag surface re-derived from the shipped binary - matches the
  amended 8.1 block exactly; both `130` producers in `run.rs` only.
  **The A4 pattern did not extend:** no second fabricated paste found in any
  other report.
- **Behaviour change end-to-end (shipped binary, rebuilt, `find -newer`
  fired):** `raw:` Float-vs-reported-Int no longer matches (exit 2,
  missing-track), same-kind still matches, value negative control fires; typed
  path still cross-compares (assignment produced for Float 1.0 vs reported
  Int 1; `scalar_fits` confirms the realistic Int-vs-Float direction is
  config-clean while the reverse errors).
- **No-work-needed premises, run:** README CLI sentence already correct
  (`:125` area); `language`/`language_ietf` string-typed and `codec_kind`
  absent in schema v20 (59 track props, the five `number` ones as claimed);
  CI's 3-OS matrix runs clippy natively and `deny`/`ledger-lint` are
  independent jobs; README first example validates (exit 0); `all` vs
  `transitive` identical on this tree (run).
- **House by id:** `comments-locate-by-symbol-never-by-line-number` and
  `a-document-never-cites-a-line-number-inside-itself` hold across the diff;
  `code-comment-line-citations-drift` class closed on the stated surface;
  `proc-verification-step-must-be-falsifiable` - every absence check I reran
  was fired against its pre-state; `testing-si3-run-binary`/`proc-06` held
  (probe file muxed, schema read); `tests-ship-with-the-feature-never-after`
  held (three tests in A3's commit; B1's fire is its test);
  `ledger-lint-runs-before-every-push` green (566 entries, invariants hold);
  `concurrent-writers-need-pathspec-scoped-commits` held; `proc-latitude` no
  open fork found; `agent-commit-trailer-set` - finding 1;
  `owner-manual-qa-gates-the-1-0-release` - no completeness claim anywhere in
  the branch.

## Tracker triage (progress.md deferred minors and parked items)

**Must be done before the push** (all already ordered; this is the checklist):

1. ROADMAP dispositions per the plan close, with the two binding qualifiers:
   "tracked" in the citation-class disposition (A2 INFO 3) and the
   8.1-surface-only scope in the spec-drift closure (A4 harvest 5). Plus
   ROADMAP:1799 "gate part 4" (A1 minor 2) and the A1-adjudication-5 vehicle
   reconciliation (amendment verdict minor 2).
2. The two promised ROADMAP lines (finding 3): parked-guard Triggers line,
   17-defaulted-fields line.
3. The seven surfacing-list items, including the Tier-2
   `gate-includes-cross-target-lint-for-the-unrun-os` rewrite (its "gate
   part 6" clause is live-false on this tree) and the
   `a-document-never-cites-a-line-number-inside-itself` scope note.
4. A4 verdict-harvest items 6-10 (finding 2) and the four A3 close records
   (finding 4).
5. Finding 1's ledger occurrence and journal statement.
6. Then the mechanical close: plan-11 worktree teardown (both still exist,
   verified; the six legacy ones stay an owner item), the single push with its
   `gh-log.md` entry, CI green on the head SHA including `deny` (0.19.8-side
   key acceptance already proven byte-identical in `cfg.rs`) and
   `ledger-lint`, SDD salvage AFTER all close actions, journal + HANDOFF.

**Correctly deferred / parked (no pre-push action beyond the recording above):**
the parked one-key guard decision (owner batch, measurement attached); the
seven retained-vocabulary sites (owner question); `raw:codec_kind`'s diagnostic
content (owner); the `glib` upgrade project (v1.x entry, written and accurate);
the `cli.rs` sentence (finding 5's ROADMAP line recommended); `README.md:194`'s
shell claim (A4 item 6's row, then deferred); the 12 line-citing design/spec
documents under `docs/` (owner question); A2 INFO 2's short comment line (next
block-touching change); the six legacy worktrees (owner disposition at next
report).

## Evidence appendix

Instruments at
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/wbrev-independent/`:
`tree-state-before.txt`/`tree-state-after.txt` (fingerprints: porcelain status +
`git diff HEAD | sha256sum` + sorted untracked list hash; diff empty),
`check-Rprime.sh` (D111 §4.6's R' verbatim), `deny-v2/v3/v4.toml` (scratch
variants; repo `deny.toml` sha `3ea3702a...` proven unchanged after all runs),
`probe/` (mkvmerge-muxed probe MKV + five dry-run profiles),
`mut/` (rsync copy of the workspace, cross arms stripped,
`cargo test -p muxsmith-core -p muxsmith-cli --no-fail-fast`),
`readme-ex1.yaml`, `fence-verdict.txt`/`fence-shipped.txt`. Key commands: the
R'/K'/sweep invocations copied from D111's fences (two invocations each,
summed; pre-state via `git grep <expr> 5378264 --`, sets compared sorted);
`cargo deny check advisories [-c variant]` (exits 0/1/0/1);
`cargo deny -L info check advisories` (19/16/2); `cargo tree -i glib@0.18.5 -e
normal --depth 1` (eleven parents); cargo-deny source reads at
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/src/{advisories.rs,advisories/cfg.rs}`;
`gh api repos/senolfeldmann/Muxsmith/dependabot/alerts` (both alerts open;
logged in `gh-log.md`); schema read at
`~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json`.

## HARVEST

1. **A merge commit is an agent commit.** `agent-commit-trailer-set` quantifies
   over "every agent-authored commit", but the merge is controller-performed
   and sits outside every task checklist, so nothing at task level can catch
   it. Trigger, readable: an agent is about to run `git merge` (or any commit
   outside a task dispatch). Handle: the same `-c commit.gpgsign=false` +
   one-trailer form the dispatches mandate. Evidence: Plan 8's merges carry it,
   Plan 11's do not - the convention held exactly as long as the merge step's
   author remembered it.
2. **The last verdict before a phase transition is the one whose
   arrival-triggered step gets skipped.** A1/A2/B1's harvests were mined at
   arrival; A4's - the final stream-A verdict, followed immediately by merge
   work - was not. The close audit exists for this and caught it, but the
   pattern predicts where the miss lands: mining should be checked off per
   verdict in the tracker row at arrival, not remembered across the merge.
3. **`grep-output-order-is-not-stable-compare-the-set` held in practice at this
   review:** the sorted-set md5 of A4's corrected paste reproduced identically
   under the harness's wrapper grep and `/usr/bin/grep`, nine-run instability
   never surfaced because every comparison here was over sorted sets. The
   entry's handle is sufficient; no extension needed.
4. **A fire against a historical state is free with `git grep <expr> <commit> --
   <pathspec>`.** Every absence check in this review got its red state from the
   pre-state commit rather than from mutating anything - cheaper and safer than
   the mutate-and-restore form wherever the red state is "how the tree used to
   be" rather than "a state nobody built". Candidate refinement on
   `proc-verification-step-must-be-falsifiable`.
