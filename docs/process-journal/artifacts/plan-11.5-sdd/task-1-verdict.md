# Task 1 review verdict - Plan 11.5

**Verdict: APPROVED_WITH_MINORS.**

The shipped `deny.toml` change is correct, does exactly what its comment says,
and needs no edit. Every claim the implementer makes about the KEY holds when
re-measured with instruments I built myself from the committed file. One claim
the implementer makes about the key's LIMIT does not hold, and it matters
because it is the premise of adjudication question 1. Separately, the
controller-owned ROADMAP entry that this key is supposed to route a reader to
now contradicts itself, and it will be read by someone within days.

Nothing in the reviewed file requires a fix round. Findings 1 and 2 are
record-level and neither is the implementer's to repair alone (finding 2 sits
in a file the brief forbade it to touch).

- Graded in `/home/senol/Git/Muxsmith` on `master`, head
  `937ae42aeceb280a3e3232cfc322c429881ea65d`, parent `4b01cb6`.
- Eleven runs of my own, exit codes read from `$?` directly after the command
  (redirection only, never a pipeline).
- Tree byte-identical to `937ae42` at finish: `git diff --quiet 937ae42` exit 0,
  `git status --porcelain` zero lines, `sha256sum -c` of my own pre-review
  baseline for `deny.toml` and `Cargo.lock` exit 0. This verdict file is under
  `.gitignore`'s `.superpowers/` (`git check-ignore -v` confirms) and therefore
  does not dirty the tree.

---

## 1. Ground truth at the source (cargo-deny 0.19.9)

Read at `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/`,
binary `cargo-deny 0.19.9` at `/home/senol/.cargo/bin/cargo-deny`.

| Fact | Site | Result |
|---|---|---|
| TOML spelling | `src/advisories/cfg.rs:268-270` | `.optional("unused-ignored-advisory")` |
| Default | `src/advisories/cfg.rs:115` and `:269-270` | `LintLevel::Warn` (also `src/advisories/snapshots/...deserializes_advisories_cfg-2.snap:44`, `"unused_ignored_advisory": "warn"`) |
| What it governs | `src/advisories/cfg.rs:95-97` | "Determines the response to advisories in the `ignore`ed list which do not exist in the dependency tree." |
| Where it becomes an exit code | `src/advisories.rs:186-194` -> `diag_for_advisory_not_encountered` (`src/advisories/diags.rs:316-333`), severity from the key | message `advisory was not encountered`, label `no crate matched advisory criteria`, code `AdvisoryNotDetected` |
| Also governs unused `ignore-yanked` | `src/advisories.rs:196-205` | same key, `diag_for_ignored_yanked_not_encountered`. Muxsmith has no `ignore-yanked`, so this is headroom, not coverage. |
| When an ignore entry counts as HIT | `src/advisories/diags.rs:158-178`, called from `src/advisories.rs:142-146` after the scope filter at `:106-140` | the hit bit is set only when the advisory both matches a crate AND survives the scope filter AND its id is in `ignore`. This is exactly the mechanism the shipped comment asserts. |
| Ordering of the `ignore` list | `src/advisories/cfg.rs:303` -> `src/cfg.rs:20-28` (`dedup` sorts) | the `binary_search_by` at `diags.rs:158-161` is safe regardless of the order ids appear in the file. The committed list is NOT sorted (families grouped), and that is harmless. Ruled out as a trap, not a finding. |

The implementer's source reading of the key itself is accurate in every
particular. Its reading of the NEIGHBOURING path is where it goes wrong; see
finding 1.

## 2. The demonstration, on my own variants

Variants built by `build_variants.py` from `git show 937ae42:deny.toml`, each
mutation asserted (`assert text.count(line) == 1`) so a mutation that matched
nothing fails the builder instead of producing a silent no-op variant. `v1-ship`
is byte-identical to the committed file (sha `42a10942f9b0...`, same as the
worktree's `deny.toml`). My non-matching probe is `RUSTSEC-2021-0128`
(rusqlite, real advisory in the local db, crate absent from `Cargo.lock`) -
deliberately a different id from the implementer's `RUSTSEC-2016-0001`.

| Run | Config state | Exit | Diagnostics |
|---|---|---|---|
| v0 | repo file, gate's own invocation, no `-c` | **0** | `advisories ok` |
| v1 | shipped copy | **0** | `advisories ok` |
| v2 | `unsound` line dropped, key present | **1** | `error[advisory-not-detected]` at the glib line, `advisories FAILED` |
| v3 | control for v2: `unsound` dropped, key ALSO dropped | **0** | `warning[advisory-not-detected]`, same label, `advisories ok` |
| v4 | glib entry kept, non-matching id added, key present | **1** | `error[advisory-not-detected]` at the added line |
| v5 | control for v4: same list, key dropped | **0** | `warning[advisory-not-detected]`, `advisories ok` |
| v6 | the controller brief's run-3 design (glib id REPLACED), key present | **1** | `error[unsound]` for glib PLUS `error[advisory-not-detected]` |
| v7 | control for v6: the brief's design, key dropped | **1** | `error[unsound]` for glib PLUS `warning[advisory-not-detected]` |
| v8 | fabricated id `RUSTSEC-2099-0001` added, key present | **1** | `warning[unknown-advisory]` AND `error[advisory-not-detected]`, `advisories FAILED` |
| v9 | control for v8: same, key dropped | **0** | `warning[unknown-advisory]` AND `warning[advisory-not-detected]`, `advisories ok` |
| v10 | shipped plus a bogus config key `not-a-real-key` | **1** | `error[unexpected-keys]`, config deserialization aborts |

Each failing case has its matching no-key control, and every control flips to
exit 0 on a one-line config difference. **The key is what converts the finding**,
for the dropped-scope case (v2/v3), the obsolescence case (v4/v5) and the
unknown-id case (v8/v9) alike.

v10 is not decorative: it establishes that cargo-deny rejects unknown keys with
a hard error rather than ignoring them, which is what makes the version question
below safe in both directions.

**Version floor: unchanged by this task.** `unused-ignored-advisory` (PR#823) and
`unsound` (PR#826) both landed in cargo-deny **0.19.0** (`CHANGELOG.md`, the
0.19.0 "Added" block). The already-shipped `unsound = "all"` therefore imposes
the same floor the new key does, and by v10 a cargo-deny below that floor would
fail loudly on the OLD key first, not silently skip the new one. The repo's own
record (`docs/decision-ledger.yaml:5497`) states CI's pinned action runs 0.19.8;
I verified the consequence rather than borrowing it - `src/advisories/cfg.rs`,
`src/advisories.rs` and `src/advisories/diags.rs` are byte-identical between the
0.19.8 sources and 0.19.9 (`diff -q`, exit 0 on all three), so the key's parse,
severity mapping and exit path are the same code on both sides.

## 3. Scope

- `git show --name-only 937ae42` lists exactly `deny.toml`.
- `git diff --stat 4b01cb6 937ae42 -- BUILDING.md .github/workflows/ci.yml scripts/`
  is empty. `BUILDING.md:88` still reads `cargo deny check`; `ci.yml:169` still
  pins `EmbarkStudios/cargo-deny-action@bb137d7...` (v2.0.20).
- The gate's command count is unchanged: the Rust gate block still enumerates six
  commands and `python3 scripts/ledger-lint.py` is green (exit 0, "566 entries
  across 4 files plus BUILDING.md's gate enumeration, all invariants hold"),
  which is the check that cross-verifies that enumeration.
- No ignore id reworded, reordered or removed, checked by content rather than by
  the insertions/deletions count: the 19 `"RUSTSEC` lines of `4b01cb6:deny.toml`
  and of head `deny.toml`, line numbers stripped, `diff` exit 0.
- Comment register and width: max comment column 77 before and after; the ten
  inserted lines run 65-77. The implementer's claim is exact.
- Table membership postcondition re-run:
  `line 28: key is in table [advisories]`.

---

## Findings

### 1. MINOR (report accuracy, shipped artifact unaffected) - the stated limitation is false; the key DOES catch an id that is in no advisory database

The report says (line 65) that such an id "takes a *different* path", that it is
"outside this key's reach" (line 409), and, load-bearing for the brief's
dimension 3 and adjudication 1, that "a fabricated id would therefore exit 0 and
prove nothing" (line 69).

Measured: it exits **1**. Run v8, `RUSTSEC-2099-0001` added to the ignore list on
the shipped config, produces `warning[unknown-advisory]` AND
`error[advisory-not-detected]`, `advisories FAILED`, exit 1. Its control v9,
identical config minus the key, produces the same two diagnostics both as
warnings, `advisories ok`, exit 0.

The source explains the miss precisely. `src/advisories.rs:176-180` emits
`diag_for_unknown_advisory` for an id missing from every database, and that
diagnostic is indeed hard-coded `Severity::Warning` (`diags.rs:353-364`) - the
implementer read that correctly. What it did not notice is that the loop at
`:186-194` is **not gated on the id being known**: an unknown id can never set
its hit bit, so it falls through to the not-encountered loop as well and gets
the key's severity. The two paths are additive, not alternative.

The repo's own records already carried the co-occurrence: the previous review at
`docs/process-journal/artifacts/plan-11-sdd/task-b1-verdict.md:467` recorded a
bogus-id config emitting `1 warning[advisory-not-detected]` and
`1 warning[unknown-advisory]` together at defaults. The single missing step was
noticing that the first of those two is the one this key escalates.

Consequence, and why this is a MINOR rather than a NEEDS_FIXES: the shipped
comment says nothing about the unknown-id case, so the file does not over-claim
or under-claim. The report is `.gitignore`d and ships nowhere. The damage would
be done only if the controller lifts concern 3 into the ROADMAP, the ledger or
the house-knowledge YAML, where it would understate the guard permanently. **Do
not carry that sentence forward.** The true residual is finding 5.

### 2. MAJOR (adjacent, controller-owned, in the parent commit) - the ROADMAP trigger this key routes to still parks the decision and recommends against the key

`docs/ROADMAP.md:956-982` is one Triggers entry. Commit `4b01cb6` rewrote its
head to the mechanical form and left its tail untouched, and the tail is now
false in three places at once:

- `:974-975` calls the two attached measurements things that "change the shape of
  the eventual decision" - the decision is not eventual, it was ruled and shipped
  the same day.
- `:979-981`: "**The reviewer nonetheless recommends AGAINST setting that key
  blind**, because it also reddens the gate when an ignored advisory legitimately
  disappears upstream."
- `:982`: "**PARKED as a one-key owner decision with the measurement attached.**"

This is not a cosmetic staleness. The entry's own headline is the exact error
string a reader will paste when the gate goes red, so the entry is the designed
landing point for exactly the person the shipped comment is trying to steer. That
reader will find, four lines below the instruction to delete the ignore entry, a
bolded recommendation against the key that just failed their build and a line
saying the decision is still parked. That is the "revert this key" reaction the
comment exists to prevent, handed to them by the project's own record.

The implementer is not at fault: its brief said "Do not edit it" about this file,
and it correctly did not. The fix belongs to the controller and should land
before the push, not after. The overtaken recommendation does not need deleting;
it needs dating and marking as superseded by the owner ruling of 2026-07-30, so
the record keeps its history without reading as live advice.

### 3. MINOR (adjacent, controller-owned) - the superseded trigger formulation is still live in a second place

`docs/ROADMAP.md:966-969` states that the predecessor formulation ("a dependency
PR or a Tauri release moves the gtk-rs generation past 0.18 in `Cargo.lock`") was
replaced by the mechanical form and is "kept here only to record that the
mechanical form replaced it". But `docs/ROADMAP.md:2833-2837`, in the v1.x
`glib` entry, still presents that same formulation as its live
"**Trigger, observable rather than remembered:**", unmarked. Two records of one
trigger, one of them declared superseded by the other and not saying so where a
reader of the v1.x entry alone would see it.

Same class as finding 2, one order of magnitude smaller: the change moved a fact
and the sites that repeat the fact were not all visited.

### 4. INFO - one clause in the shipped comment describes the pre-key world in the present tense

"drop the `unsound` scope key and that class stops being evaluated, leaving its
ignore entry suppressing nothing while the check still reports success"
(`deny.toml:20-22`). With the key set, that state no longer reports success - it
is run v2, exit 1. The clause is inside an enumeration of "both ways of losing
one silently", so it is describing the defect being converted rather than the
outcome after conversion, and it reads correctly on a careful pass. I would not
change it; recorded because a hurried reader can take it as a statement about
today's behaviour, and because a future edit that shortens the sentence could
easily strand the clause.

### 5. INFO - the residual the key genuinely does not reach, stated correctly

Measured coverage of `unused-ignored-advisory = "deny"` on this tree: an ignore
entry whose advisory exists but does not fire (scope dropped, v2; crate gone or
advisory fixed, v4), an ignore entry whose id is in no database at all (v8), and
per source also an unused `ignore-yanked` entry (`src/advisories.rs:196-205`,
no such entries here today).

What it does not reach is one thing only: **an edit that removes the key itself**,
alone or together with the ignore entry. That state is gate-green and no
configuration key can catch it, which is the residual plan-11's records already
named as the part that would need new infrastructure
(`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md:815`).
Nothing new is owed here; the point is that this residual, not the unknown-id
case from finding 1, is the honest boundary.

---

## Adjudications

### Adjudication 1: is the change complete as a guard, or does it advertise more than it delivers?

**It delivers slightly MORE than the report credits it with, and the comment's
current scope is already honest. Nothing needs adding.**

The question was posed on the premise that an id absent from every advisory
database escapes the key. Finding 1 shows the premise is false: that case fails
too, exit 1, with its no-key control at exit 0. So the readable-in-the-file
coverage is every way an entry in `ignore` can stop suppressing something -
advisory does not fire, or advisory does not exist. A reader who takes the
ignore list as self-policing against obsolete entries is, on measurement, right.

The shipped comment does not claim completeness and does not need a disclaiming
sentence; adding one for the unknown-id case would state a limit that is not
real. Adding one for the real residual (finding 5: deleting the key itself) would
be worse than useless in that spot, because a comment cannot guard against its
own deletion, and the residual is already recorded where a plan reader meets it.

The one thing that does need doing is negative: do not promote the report's
concern 3 into any repo record. If the controller wants the boundary written
down at all, the sentence to write is finding 5's, not concern 3's.

### Adjudication 2: were the three workarounds correct, and did any quietly change what the task asserts?

**All three correct. None changed what the task asserts. The first one converted
an uninterpretable test into an interpretable one and is the most valuable thing
in the report.**

**(a) The confounded run 3.** Correct, and I reproduced the confound
independently rather than taking it on report. The brief's design (replace the
glib id) is my v6: exit 1. Its control, the same design with the key removed, is
my v7: **also exit 1**, because the unignored glib advisory raises
`error[unsound]` on its own. The brief's run 3 would have exited 1 with or
without the key, which is precisely the defect the brief's own run 4 exists to
exclude for run 2 - the brief applied the control discipline to one test and not
to its neighbour. The implementer's replacement (keep the glib entry, add a
non-matching id alongside) is my v4/v5: exit 1 with the key, exit 0 without,
single-line difference. It is also the more faithful simulation, because a real
upstream fix leaves the ignore list untouched and simply stops matching one
entry. The assertion under test - "an ignore entry that stops matching fails the
gate" - is unchanged; only the confound was removed.

**(b) `git diff --exit-code -- deny.toml` as the untouched-file proof.** Correct,
and the objection is stronger than the report states. Pre-commit the command
exits 1 by construction, as the report shows. I ran it post-commit: it exits
**0** - and that 0 is vacuous, because it would also appear if the file HAD been
mutated to produce a variant and then committed in that state. So the command
fails as evidence in both directions in a task whose deliverable is a change to
the file it inspects. The substituted before/after `sha256sum` pair is the proof
that actually holds, and it is the form a future config-editing brief should
specify. No change to what is asserted.

**(c) The two extra controls (4b, and 3b's construction).** Correct and required
rather than optional. The brief's run 4 controls only the dropped-scope case; run
3 had no control at all, so as written the obsolescence claim - the one the owner
asked for - rested on an uncontrolled run. Adding the matching control is what
makes the owner's case demonstrated rather than asserted. My v4/v5 pair is the
independent rebuild of it.

The one place where a workaround did NOT go far enough is finding 1: having
correctly identified the fabricated-id trap in the source, the implementer
declined to run the fabricated-id case and reported its predicted exit code
instead. The prediction was wrong. That is the exact shape the brief's dimension
6 is aimed at - a conclusion that something is impossible, not run.

### Adjudication 3: is the project ready for the first Renovate-triggered failure to be read correctly?

**The mechanism is ready. The record is not, and finding 2 is why. Fix the
ROADMAP tail before the push; nothing else is needed.**

Ready, verified:

- The failure will be unmissable and self-explaining at the point of failure:
  `error[advisory-not-detected]`, the config file, the exact line and column, the
  label `no crate matched advisory criteria`, and the ignore entry's own comment
  block directly above the named line.
- It fires in both places the gate runs, with identical code: local `cargo deny
  check` per `BUILDING.md:88`, and the CI `deny` job's pinned action, whose
  cargo-deny 0.19.8 has byte-identical `advisories` sources to the 0.19.9 I
  measured on.
- The shipped comment carries the correct reaction in the file itself, and states
  the failure is "not a regression".
- `deny.toml:29-32`, the ignore list's header, independently instructs "drop an
  ID once its crate is gone from `Cargo.lock` (upstream fix) instead of leaving
  it stale". Two in-file carriers, not one.

Not ready:

- The ROADMAP Triggers entry is the designed landing point - its headline is the
  literal error string - and its tail tells that reader the key was recommended
  against and the decision is parked (finding 2). Of everything in the project,
  this is the single artifact most likely to be read by the person meeting the
  red gate, and today it argues for the wrong reaction.

Not needed: a louder channel than the comment. The comment plus the list header
plus a repaired Triggers entry is three carriers for a failure that names its own
file and line. What was missing was never volume, it was one stale paragraph.

---

## Evidence appendix

All instruments under
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/p115rev-independent/`,
built by me from the committed file; none of the implementer's variants were
re-run.

| Path | What it is |
|---|---|
| `base-committed.toml` | `git show 937ae42:deny.toml`, sha `42a10942f9b0...`, identical to the worktree file |
| `build_variants.py` | variant builder, every mutation asserted, prints per-variant property counts and shas |
| `v1-ship.toml` ... `v9-unknownid-nokey.toml` | the nine variants of the table in section 2 |
| `v10-bogus-key.toml` | unknown-config-key probe |
| `run_variants.sh` | the runner; `$?` read directly after each command, output via redirection only, no pipelines |
| `out-v0-repo-asis.log` ... `out-v10-bogus-key.log` | full cargo-deny output per run |
| `results.txt` | the eleven exit codes as produced |
| `repo-baseline.sha256` | pre-review hashes of `deny.toml` and `Cargo.lock`, `sha256sum -c` exit 0 at finish |
| `ledger-lint.log` | `python3 scripts/ledger-lint.py`, exit 0 |
| `ids-parent-bare.txt` / `ids-head-bare.txt` | the 19 ignore-id lines before and after, `diff` exit 0 |
| `deny-parent.toml` | `git show 4b01cb6:deny.toml`, the pre-change file |

Source authority read at
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/`
(`src/advisories/cfg.rs`, `src/advisories.rs`, `src/advisories/diags.rs`,
`src/cfg.rs`, `CHANGELOG.md`), cross-diffed against the 0.19.8 sources at
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/cargo-deny-0.19.8/`.

---

## HARVEST

1. **An additive diagnostic loop defeats a "different path" reading of source.**
   The implementer found the right function, read its hard-coded severity
   correctly, and concluded exclusivity that the code does not implement, because
   the second loop it fell through to has no guard against the first loop's
   cases. Reading one branch tells you what that branch does; it does not tell
   you that control flow stops there. Handle: when the conclusion is "case X
   takes path A instead of path B", check whether B is actually skipped for X, or
   just described somewhere else.

2. **A limitation is a claim, and dimension 6 covers it.** The report's concern 3
   is a negative claim with a predicted exit code attached and no run behind it,
   in a report that ran ten commands. It cost nothing to run - one variant, one
   invocation - and it was wrong. Any sentence of the form "a test of X would
   exit 0 and prove nothing" is a test design plus its predicted result: run it,
   or do not assert the result.

3. **The control discipline applies per test, not per report.** The controller's
   brief prescribed a no-key control for run 2 and none for run 3, so its central
   demonstration (the owner's own question) was uncontrolled while a secondary one
   was rigorous. When a plan enumerates runs, sweep the enumeration and ask which
   ones have a matching control, rather than checking that the word "control"
   appears.

4. **A proof command must be able to fail for the reason it is quoted for.**
   `git diff --exit-code -- <file>` in a task that edits `<file>` exits 1 before
   the commit and 0 after it, and neither value distinguishes "never mutated for a
   variant" from "mutated and committed". The generalisable trigger: a check whose
   subject is also the task's deliverable is measuring the deliverable, not the
   property.

5. **Rewriting the head of a long record leaves its tail asserting the old
   world.** `4b01cb6` re-pointed a trigger's headline and instruction and left
   nine lines below them saying the decision is parked and recommended against.
   The entry is one paragraph; the edit was correct; the entry as a whole now
   argues both ways. When an owner ruling overtakes a recorded recommendation,
   the recommendation gets dated and marked superseded in the same change, and
   the sweep runs to the end of the entry, not to the end of the edited sentence.

6. **Two live copies of one trigger.** The same commit declared a trigger
   formulation replaced in one entry while the other entry that carries it kept
   presenting it as live (finding 3). The fact moved; the sites repeating it were
   not enumerated.

7. **A version floor added by a config key is answered by the key's release, not
   by the key's novelty.** `unused-ignored-advisory` and `unsound` shipped in the
   same cargo-deny release, so the newer-looking key imposes no floor the already
   shipped one does not - and cargo-deny rejects unknown keys with exit 1, so a
   too-old toolchain fails loudly on the old key first. Two cheap reads
   (CHANGELOG, one bogus-key run) closed a question that looks like it needs a CI
   round trip.
