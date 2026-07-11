# Muxsmith - unrealized ideas (potential future features)

Product-scope ideas deliberately deferred out of v1, kept here so a later
session (or Şenol) has full context without re-deriving it or digging through
code. Each entry states: what it is, what mkvtoolnix does, why it is deferred,
what building it would take, and when to reconsider.

Scope boundary: this file is for **deferred product/feature ideas**, mostly
surfaced by the mkvtoolnix parity audit. Engineering follow-ups and cleanups
(test dedup, CI, diagnostic minors) live in the plan docs, the HANDOFF, and
the design memos, not here.

Nothing here is committed work. An item graduates only when Şenol green-lights
it into a plan.

---

## 1. Language derivation from filename

**Idea.** Infer a track's language from its filename when the profile does not
set it - e.g. an external `Movie.eng.srt` gets `eng`, `Show.ger.forced.srt`
gets `ger`.

**mkvtoolnix behavior.** On by default for audio and subtitles
(`m_deriveAudioTrackLanguageFromFileNamePolicy` / `...Subtitle...` =
`IfAbsentOrUndetermined`; video = `Never`). It scans the filename against the
set of recognized ISO codes, using boundary characters `[](){}.+-=#`. So
`movie.eng.srt` in the GUI silently becomes `--language 0:eng`.

**Why deferred.** This is a *magic guess*. mkvtoolnix is interactive: the user
sees the guessed language pre-filled and corrects it per file before muxing.
Muxsmith is declarative batch - the same guess would fire unseen across
hundreds of files with no review step, which is exactly the failure mode the
tool exists to avoid. Şenol's ruling: "not the concern of Muxsmith, hard no."

**What building it would take (if ever).** An **opt-in** locator option (off by
default), e.g. `language_from_filename: true` on an external-subtitle rule, so
the guess is explicitly requested per rule rather than global-by-default. Would
reuse the existing language index for validation. It must never be the default.

**When to reconsider.** If the community specifically asks for it for
external-subtitle bulk workflows (the one place its value is high: naming
conventions like `Movie.eng.srt` are near-universal for sidecar subs).

---

## 2. Flag derivation from filename or track name

**Idea.** Set the commentary / hearing-impaired / forced-display flags from the
filename or track name - e.g. `Movie.Commentary.ac3` gets the commentary flag,
`subs.forced.srt` gets the forced flag.

**mkvtoolnix behavior.** All three derivations on by default
(`m_setCommentaryFlagFromFileName`, hearing-impaired, and forced-subtitle name
recognition via a configurable regex, default `forced`). Derived flags are then
emitted as `--commentary-flag` / `--hearing-impaired-flag` /
`--forced-display-flag`.

**Why deferred.** Same category as idea 1: a filename-based magic guess,
appropriate for an interactive tool with per-file review, wrong as an unseen
default in declarative batch. Şenol's ruling: same hard no as idea 1.

**What building it would take (if ever).** Opt-in, explicit per-rule config
(e.g. a `flags_from_name` option), never global-by-default. Lower value than
idea 1.

**When to reconsider.** Community request; lower priority than language
derivation.

---

## 3. `unique` collision policy value (auto-suffix output names)

**Idea.** A fourth `on_collision` value, `unique`, that renames a colliding
output to `Name (1).mkv`, `Name (2).mkv`, ... instead of erroring or skipping.

**mkvtoolnix behavior.** This is the GUI's *default* (`m_uniqueOutputFileNames`
= true): it appends ` (1)`, ` (2)`, ... until the name collides with neither an
on-disk file nor a pending job in the queue.

**Why deferred.** Muxsmith's `on_collision` values are `error` (default) |
`skip` | `overwrite`; no `unique`. Şenol's ruling: for a bulk tool the user is
expected to target a fresh output directory, and the batch itself must not
contain internal name conflicts anyway (two plans to one path is always a hard
`OutputCollision` error, independent of policy). Silent ` (1)` suffixing just
litters the output tree and hides a real naming mistake. Fail loud instead.

**What building it would take (if ever).** A new `on_collision: unique` arm in
the planner's collision resolution, rendering a disambiguating suffix and
re-checking against both disk and the in-batch planned-output set.

**When to reconsider.** Only if real usage shows a workflow where colliding
outputs are expected and suffixing is genuinely wanted rather than a mistake to
surface.

---

## 4. Batch append of multi-part sources (CD1/CD2 concatenation)

**Idea.** Rejoin a movie that was ripped into sequential parts -
`Movie.CD1.mkv` + `Movie.CD2.mkv`, or `part1`/`part2` - into one continuous
output, concatenated along the time axis.

**Two different mkvmerge operations, to be clear about the distinction:**
- **Muxing (what Muxsmith does):** combine *different tracks* from *different
  files* onto one timeline (video from A, German audio from a donor B), mkvmerge
  `( fileA ) ( fileB )` with track selection. Parallel, same time range.
- **Appending (this idea):** concatenate *whole files* end to end so part 2
  plays *after* part 1, mkvmerge `file1 + file2` with `--append-to` track
  mappings. Sequential, extends the timeline.

**mkvtoolnix behavior.** `m_mergeReconstructSequencesWhenAdding` = true by
default: adding files whose names form a numeric sequence auto-appends them into
one output instead of treating them as separate jobs.

**Why deferred.** Appending is a fundamentally different operation from
Muxsmith's model (one primary file -> one output, rule-based track selection).
Muxsmith has no append concept at all; spec section 11 lists it explicitly out
of v1 scope. Today each `CDx` file is its own primary and produces its own
output.

**What building it would take (if ever).** A whole append layer: a way to
*declare* which files form an append group (patterns grouping `CD1`/`CD2` to one
logical title), append-mapping generation (`+`, `--append-to`), and validation
that the parts have compatible track layouts. Non-trivial and orthogonal to the
selection core.

**Design note if ever built.** mkvtoolnix *auto-detects* sequences from
filenames - the same magic-guess pattern Muxsmith rejects (ideas 1-2). A
Muxsmith append feature would be **explicit** (the profile declares the append
grouping), never auto-guessed.

**When to reconsider.** Community request from users with disc-split rips
(older DVD rips, some anime releases) who expect CD1/CD2 rejoining.

## 5. Zero-track outcome options: skip or error instead of empty MKV

**Idea.** When a file's plan resolves to zero output tracks, offer
alternatives to writing the valid-but-empty MKV: skip the file entirely
(reported as skipped, not success), or treat it as a per-file error (honoring
fail-fast) - e.g. a profile-level `on_empty_plan: write|skip|error`.

**mkvtoolnix behavior.** mkvmerge itself exits 0 and writes the empty file
(verified live against the binary in the Plan-3 whole-branch review).
mkvtoolnix-gui's zero-selected-tracks UX has not been audited yet - check it
(SI-3) when building this.

**Why deferred.** Şenol 2026-07-11 (sweep walkthrough #6): v1 emits a
per-file warning and still writes the file - one sane default; more choices
would confuse. No user demand yet for skip/error.

**What building it would take (if ever).** The `on_empty_plan` enum in the
profile model, a planner/executor branch for skip and error, batch-report
representation for "skipped", tests for all three paths, docs. Small,
well-contained once the v1 warning (ROADMAP pre-1.0 gate) exists.

**When to reconsider.** First real-world report of unwanted empty outputs in
large batches, or a user asking to fail a batch on empty plans.

## 6. Un-dispositioned parity extras from the Plan-3.5 mkvtoolnix audit

**What this is.** The audit's reference-extraction pass listed mkvtoolnix-gui
behaviors "a bulk tool could get observably wrong"; the synthesis
dispositioned only part of them (ideas 1-4 plus one tier-4 match). Seven were
never classified into a tier. Recorded 2026-07-11 (sweep walkthrough #22) as
a collective entry; none has been individually audited against the mkvtoolnix
source per the SI-3 method - "presumed" below is deliberate honesty.

- Audio delay derived from filename (`m_setAudioDelayFromFileName`, default
  on): presumed magic-guess class (same rationale as ideas 1-2).
- Track-enabled-flag auto-repair: presumed magic-guess class.
- Dialog normalization gain removal: presumed magic-guess class (a silent
  audio mutation is exactly what unattended batch must not guess at).
- Bluray cover art handling: presumed magic-guess class.
- Subtitle default-track suppression: presumed magic-guess class.
- Output naming from title (reference §6): genuine naming divergence, never
  tiered.
- **Missing-audio warning**: NOT input guessing - an output plausibility
  check ("this output has no audio track - sure?"), conceptual sibling of
  the zero-track warning (ROADMAP pre-1.0 gate). Deserves its own review
  whenever output-plausibility diagnostics are extended.

**Source situation.** The audit's full raw corpus (~120 inventory items)
died with the task outputs (the "save recon corpora as artifacts" learning
postdates it); it remains reconstructable from the Plan-3.5 session
transcript if a deeper disposition round is ever wanted.

**When to reconsider.** At the next parity-audit round, or when
output-plausibility diagnostics are extended.
