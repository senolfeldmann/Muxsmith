# Muxsmith process journal - entry prompt

You are working on Muxsmith (~/Git/Muxsmith) as Şenol's technical collaborator.
Append one entry to the process journal. The journal is the primary source for
a future process-focused technical blog post and retrospective documentation of
how this project was built in human-AI collaboration. It is written while
context is hot because transcripts compact, scratch dirs die with reboots, and
memory of *why* decays within a session. You are writing for a future writer
(human or AI) who was not there.

## File and hygiene

- Append to `docs/process-journal.md` (create with an H1 and a one-line purpose
  note if absent). Git-TRACKED deliberately: git-ignored artifacts are how
  process history nearly got lost once. Never rewrite or reorder old entries;
  append only. Commit the entry: `journal: <plan/phase> entry`.
- Entry header: `## <ISO date> | <phase, e.g. "Plan 1 complete"> | session <n or label>`
- Register: lab notebook, not marketing. Terse, factual, past tense. English.
  ASCII punctuation only (no em-dashes, no curly quotes). It is raw material,
  not prose for publication; incomplete sentences are fine, vagueness is not.

## Salvage pass (EVERY plan close, not only the first run)

At every plan/phase close, before the journal entry:
1. `.superpowers/sdd/` (or `.superpowers/sdd/plan-N/`) in the repo dir
   (progress ledger, task briefs/reports, review packages, fix reports,
   REVIEWER VERDICT files): copy the whole directory to
   `docs/process-journal/artifacts/<phase>-sdd/` and commit it. These files
   are the authentic per-task record (TDD evidence, reviewer verdicts, fix
   waves). Remove any `.gitignore` inside the copied directory first and
   verify the file count in the COMMIT, not the working tree (a copied
   ignore file once silently excluded all 49 artifacts).
2. Snapshot the current `HANDOFF.md` to
   `docs/process-journal/artifacts/handoffs/<date>-<phase>-close.md` with a
   one-line provenance comment (the HANDOFF itself is untracked and
   replace-in-place; without the snapshot its states are only recoverable
   by transcript mining - 2026-07-10/11 that recovery took a session).
3. If parts are missing, say so in the entry ("lost: X, reconstructed
   from Y").
4. On a first/retroactive run: write the entry from the ledger, `git log`,
   the spec and plan docs, and whatever the current session still knows
   firsthand.

## What an entry captures (in this order, omit empty sections)

1. **Scope of this entry.** One line: which plan/phase/session span it covers,
   with the commit range (e.g. `1f00aa6..cd3f239`).
2. **Decisions and their why.** Only decisions whose rationale is NOT obvious
   from the artifacts: what was decided, the alternatives seriously considered,
   why this one won, who decided (Şenol / agent / forced by a constraint).
   The spec records WHAT; the journal records the WHY and the road not taken.
3. **What the process caught.** Review findings that mattered, each one line:
   the defect, which stage caught it (task review / final review / controller
   verification / CI), and whether it originated in the plan, the implementer,
   or upstream. Explicitly separate real bugs from noise. This list is the
   evidence for any claim the blog will make about multi-stage review value.
4. **Process mechanics and metrics.** Numbers that cannot be reconstructed
   later: tasks executed, subagent dispatches (implementer/reviewer/fixer),
   models used per role, fix waves, re-review cycles, wall-clock feel, token
   counts if visible, CI minutes if relevant. Approximations are fine if
   labeled as such.
5. **Friction and failure.** What went wrong or was awkward in the
   collaboration itself: tool failures, misdispatches, wrong assumptions,
   instructions that had to be repeated, anything the human had to correct.
   Unflattering entries are the valuable ones; a journal with no failures is
   fiction.
6. **Moments.** 1-3 concrete anecdotes worth retelling, one line each, with
   enough detail to find the full record later (task number, file, commit).
7. **Deltas.** Where reality diverged from the plan/spec and what that says
   about the planning approach.
8. **Open threads.** What the next phase inherits: deferred findings, pending
   decisions, known risks. One line each.

## Anti-goals

- Do not restate what `git log`, the spec, or the plan already record; link or
  name them instead (commit SHAs, doc paths, task numbers are the pointers).
- No self-praise, no hedging boilerplate, no "successfully" - outcomes are
  stated plainly ("81 tests green", "reviewer rejected twice").
- No forward-looking promises; the journal records what happened.
- Length discipline: a plan-sized entry is 30-60 lines. If it grows beyond
  that, you are transcribing instead of distilling.

## Trigger discipline (for the session reading this)

Write an entry when: a plan completes, a session is about to close or hand
off, or something happened that section 3/5 would capture and that only the
current context still knows. When in doubt, a short entry now beats a
reconstructed one later.
