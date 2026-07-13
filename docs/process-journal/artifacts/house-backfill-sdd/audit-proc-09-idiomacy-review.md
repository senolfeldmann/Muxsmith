# Audit: proc-09-idiomacy-review (PROMOTION candidate)

- **Cluster:** `proc-09-idiomacy-review` (pattern / process / settled)
- **Claimed count:** 3 occurrences, `promoted: true`
- **Verdict:** **CONFIRMED** - 3 distinct occurrences survive; promotion stands.
- **verified_count:** 3
- **Audited:** 2026-07-13, against `/home/senol/Git/Muxsmith`

## Statement under audit

A whole-codebase idiomacy review is a distinct pre-1.0 gate scoped to complexity
only (correctness/security/perf routed to a separate list). Six-dimension rubric
(idiom, dup, stdlib, dep, yagni/over-abstraction, native/platform-reinvention)
with a ranked one-line-per-finding output contract; executed with many finders +
seed-verification + a code-level dedup barrier + one adversarial verifier per
finding.

## Per-occurrence verification

### Occ 1 - 2026-07-11, `decided`, ref "journal session-8 close (E8[86])" - SUPPORTED

Evidence claim: "New pre-1.0 gate: whole-codebase idiomacy review after Plans 5.5/6."

`docs/process-journal.md`, session-8-close entry (line 509), Decisions block, line 538, verbatim:

> Idiomacy directive added to shared conventions (Şenol): ... New pre-1.0 gate:
> whole-codebase idiomacy review after Plans 5.5/6.

Reinforced in the same entry's Open-threads block (line 587): "Whole-codebase
idiomacy review scheduled after Plans 5.5/6." This is the **gate-creation** event,
recorded as a Şenol decision. Ref supports the occurrence. **SURVIVES.**

### Occ 2 - 2026-07-12, `decided`, ref "commit b535038 + ponytail-mining.md candidate 2 (E8[91])" - SUPPORTED

Evidence claim: rubric gains yagni + native axes and the one-line-per-finding
contract (four -> six dimensions); correctness/security/perf stay out of scope.

Both halves of the compound ref are real and corroborating:

- **Commit `b535038`** (in git, author Şenol, 2026-07-12 10:56, `docs/ROADMAP.md` +14/-1):
  adds "SIX dimensions (four original + two adopted 2026-07-12 ...): ... `yagni`
  (over-abstraction ...); `native` (platform reinvention ...)"; adds the OUTPUT
  CONTRACT "one line per finding `<file>:L<n>: <tag> <what to cut>. <replacement>.`,
  ranked biggest-cut-first, ending `net: -<N> lines, -<M> deps possible`"; and
  "Correctness/security/perf explicitly OUT of scope for this pass ... route such
  finds to a normal review."
- **`.superpowers/ponytail-mining.md` Candidate 2** (lines 99-109): "give the
  idiomacy-review dispatch (C) the two axes it's missing + the output contract ...
  C's four dimensions become six", enumerating the `yagni` and `native` axes and
  the identical output contract, with correctness/security/perf "explicitly out of
  scope." Overlap-map row (line 82) confirms C originally had exactly four
  dimensions (unidiomatic + near-duplicate + hand-rolled-vs-stdlib/lib + inverse
  dependency sweep).

This is the **rubric-refinement** event (4 -> 6 dims + contract). Ref supports the
occurrence. **SURVIVES.**

### Occ 3 - 2026-07-12, `decided`, ref "journal session 10 + idiomacy-review-findings.md header (E8[96])" - SUPPORTED (with a provenance caveat)

Evidence claim: executed six-dimension review; 11 finders + 13 seed verifications +
code-level dedup barrier + one adversarial verifier per finding.

Primary in-repo half of the ref - `docs/process-journal.md` session-10 entry
(line 740) - supports every element verbatim:

- "Six dimensions (idiom, dup, stdlib, dep, yagni, native), correctness/security/
  perf explicitly out of scope and routed to a separate list" (line 750).
- "11 finders (9 subsystem slices, the two largest crates split ..., plus whole-tree
  dup and dep sweeps)" (line 752).
- "13 seed verifications (Plan 5.5 funnel items ...)" (line 754).
- "a code-level dedup barrier with n-in/n-out accounting" (line 755).
- "one adversarial verifier per deduped finding" (line 756).

Independently corroborated by the on-disk artifact directory
`.superpowers/sdd/idiomacy-review/` (gitignored scratch, `.gitignore` = `*`):
**11** `find-*` files (F1a, F1b, F2a, F2b, F3-F7, X1, X2), **13** `seed-*` files
(seed-1..seed-13), plus 52 `verify-*` files. The finder/seed counts match exactly.

**Provenance caveat (does not drop the occurrence):** the second half of the ref,
`idiomacy-review-findings.md`, does **not** exist anywhere in the repo working tree
or git history. The journal explains why (lines 743-745): the "merged ranked
findings report" was "persisted outside the repo (project non-repo material),
pending triage." So it is a real artifact living outside the repo, not a
fabrication - and the evidence claim is fully substantiated by the in-repo journal
entry plus the on-disk finder/seed artifacts regardless. The occurrence is the
**execution** event and is genuine. **SURVIVES.**

## Distinctness check (no duplicates)

The three occurrences are distinct phases of the same topic, each with its own
artifact, not restatements of one event:

1. Occ 1 (07-11, session-8 close): gate **created / scheduled**.
2. Occ 2 (07-12, commit b535038 + mining): rubric **refined** (4 -> 6 dims + output contract).
3. Occ 3 (07-12, session 10): review **executed**.

No two collapse into each other. 3 genuine recurrences.

## Minor discrepancies noted (non-material)

- Journal line 747 states "67 finder/seed/verifier artifacts"; the directory now
  holds 76 (11+13+52). Consistent with the journal's account of later inline
  re-verification adding verify files after the entry was written. Does not affect
  the load-bearing 11-finder / 13-seed counts.

## Conclusion

All three cited refs resolve to real, authoritative artifacts (in-repo journal
entries, a git commit, an on-disk mining memo, and the gitignored execution-artifact
directory) that each support "the whole-codebase idiomacy-review gate arose here as
a decision," across three genuinely distinct events. The promoted count of 3 is
real. **Verdict: CONFIRMED. verified_count = 3. Promotion stands.**
