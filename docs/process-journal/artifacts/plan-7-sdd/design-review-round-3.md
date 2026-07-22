# Plan 7 design review, round 3 (E1 re-fold delta)

Artifact: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
(1791 lines, D50-D64). Same reviewer; scope per the coordinator: the E1
re-fold delta (D63, D64, trigger 11, the reworded sections) plus
regression watch. Everything else stands settled by round 2 and the
owner's approval. Every claim below was verified on the artifact, the
tree, the installed crates' source, the built binary, and
`~/Downloads/mkvtoolnix` - foreground, with controls on negatives.

## Verdict: APPROVED

(One new Minor - an off-by-one locale count in the SI-3 evidence row -
rides the controller's consuming edit; it sits in an evidence table, not
in anything an implementer transcribes, and warrants no fix round.)

## Dispositions on the seven verification items

1. **Boundary conformance - CONFORMANT.** Both Tier-2 entries read:
   `cli-english-only` (:433) carries the SUPERSEDED marker naming its
   successor; `cli-multilang-rendering` (:448) is the live rule. D63
   matches its statement clause by clause: both locales' cli.ftl +
   diagnostics.ftl embedded at build time (the four-constant embed
   table); resolution `--locale` > system locale > en (the existing
   `i18n.rs:21-27` chain, verified at those lines); per-message fallback
   de -> en -> raw id (chain walk + the raw-id return at `i18n.rs:79-83`,
   verified); the companion pinning constraint is D64, explicitly bound
   via the entry's statement and held as a proposed safeguard under
   `proc-proposed-safeguard-stays` (:425, verified present).
2. **D63 technical claims - VERIFIED, all three.**
   - *Single-bundle plural claim*: verified at the installed
     fluent-bundle 0.16.0 source. `FluentBundle::new` binds its intl
     memoizer to the FIRST locale only (`bundle.rs:634-640`,
     `intls: IntlLangMemoizer::new(first_locale)`); plural-variant
     matching resolves `PluralRules` through `scope.bundle.intls`
     (`types/mod.rs:203-211`); the memoizer is language-bound
     (intl-memoizer 0.5.3 `:182-190`). So one merged de-langid bundle
     would pluralize en fallback content under de CLDR rules - the
     rejected alternative's rejection is factually correct, and the
     chain-of-per-locale-bundles design is the right consequence.
   - *`include_str!` has no glob form*: measured, not weighed - a scratch
     `include_str!("*.txt")` fails with "couldn't read `*.txt`" (the
     string is a literal path), the sibling literal-path control compiles
     and runs. The embed-table asymmetry justification holds; trigger 11
     records the per-locale row duty.
   - *The four renderer unit tests*: (1) de renders de, (2) en-only
     message falls back under a de chain, (3) "de-DE" collapses to the de
     row, (4) unknown tag renders the en chain - plus the two existing
     tests verified in the module (`unknown_message_id_falls_back_to_raw_id`,
     `invalid_locale_falls_back_to_en_and_renders`, `:200-212`), these
     cover every deterministically testable behavior class of the chain;
     the one untested path (system-locale fallback) is exactly the one
     `sys_locale`'s OS-API reads make unpinnable in-process, and D64
     states that exclusion with its reason.
3. **D64 empirical claims - VERIFIED, each measured.**
   - *sys_locale*: installed sys-locale 0.3.2 source read - Windows uses
     `GetUserPreferredUILanguages` (windows.rs:5), macOS
     `CFLocaleCopyPreferredLanguages` (apple.rs:49,143), Unix reads
     LANGUAGE/LC_ALL/LC_MESSAGES/LANG env vars (unix.rs:3-32). The
     env-pinning rejection ("would hold on exactly the CI legs that never
     break") is precisely right.
   - *`--locale` per subcommand*: `cli.rs:31,55,66,93` verified - four
     per-subcommand `locale: Option<String>` args, exactly the cited
     lines. Flag-after-positional measured on the built binary:
     `muxsmith validate <file> --locale en` parses and runs (the error
     produced is the renderer's profile-load error, not a clap usage
     rejection).
   - *Pinning surface*: recounted independently. 11 `.snap` files
     splitting cli_validate 3 / dry_run_cli 3 / run_cli 4 / run_live 1 -
     exact. `cargo_bin` sites: cli_validate 1, dry_run_cli 13, run_cli 1,
     run_live 1, cli_schema 2 - exact, five files, `cli_schema.rs`
     included. `tests/support/mod.rs` exists as the established shared
     helper home; `cli_validate.rs:3-5` is the per-file helper the funnel
     replaces, verbatim as cited.
   - *"No test pins a locale today"*: verified - `grep locale` over
     `crates/muxsmith-cli/tests/` returns only catalog_completeness.rs's
     catalog-file references, and the identical grep form fires on
     `cli.rs`'s four locale args (positive control).
   - *Greppable invariant*: `cargo_bin` currently appears in five test
     files (measured), i.e. the invariant is false today - and the design
     states it strictly as "**Post-sweep** invariant", not current fact.
     Correctly framed.
   - *e2e invokes no CLI binary*: verified - the muxsmith/cargo_bin grep
     over `e2e/*.ts` returns only mock/type identifiers
     (`__muxsmithE2E__` etc., which prove the pattern fires), no process
     invocation.
4. **Amendment-5 half-withdrawal - VERIFIED SOUND** (the no-work-needed
   check executed). Spec 8.4's actual sentence - "Locale selection:
   system locale with manual override in app settings and `--locale` on
   the CLI; falls back to English per message" - becomes true clause by
   clause under D63 (explicit flag, sys_locale, per-message en
   fallback). The GUI live-switch half of amendment 5 is retained; the
   CLI half is recorded as checked-no-change rather than silently
   dropped. No residual stale claim: the rustdoc's two "English only"
   comments (`i18n.rs:12`, `:19-20`, verified still in the tree) remain
   covered by amendment 3 in its rewritten multilang form, and the
   sweep's new sequencing constraint (amendments 1/2 land with D63's
   code, 3 rides the code change) closes the
   spec-asserts-what-code-cannot-do window.
5. **SI-3 new row - substance VERIFIED, one count wrong.**
   `translation.cpp:435-437` is exactly
   `bindtextdomain("mkvtoolnix")` / `textdomain("mkvtoolnix")` /
   `bind_textdomain_codeset` - one textdomain serving CLI and GUI alike;
   `po/de.po` exists. **But `po/*.po` counts 28, not 29** - see finding
   N1.
6. **Sweep completeness - CLEAN.** Every "English only"/"en only"/"GUI
   feature" hit in the document is historical record (section 7's
   two-ruling history, superseded-entry citations), an accurate
   description of today's tree (ground truth §1, explicitly closed by
   the re-ruling paragraph), or the unrelated close-abort shell residual.
   Header carries both E1 events and the fork-closure claim holds. D56's
   not-re-rendered CLI entry correctly reworded (locale resolved per
   invocation; a GUI live switch neither reaches nor needs a separate
   process). Section 9 carries exactly three new CLI lines (embed
   set/chain, en-pinned funnel + grep invariant + no env pinning, four
   unit tests / no de snapshots) and the old "CLI grows no de embedding"
   line is gone. Trigger 1 extended ("CONSUMED twice"), 2-10 byte-stable,
   11 new and correct.
7. **Consistency - HOLDS.** Section 2's catalog table is rightly
   untouched (D63 embeds existing catalogs, changes no ids). Section 5
   replaces the stale out-of-scope entry with the sharper "CLI live
   locale switching" boundary. The E1 history keeps both analyses with
   the English-only steelman preserved in the superseded entry's own
   field - nothing was quietly deleted. No contradiction with
   round-2-settled content found. Bonus: round 2's leftover Minor (the
   ":106 full audit in section 5" cross-reference) was fixed in this
   round ("section 3", line 117).

## New findings in the delta

1. **Minor - SI-3 count: "29 `po/*.po` locales" is 28.** Measured:
   `ls po/*.po | wc -l` = 28 at `~/Downloads/mkvtoolnix`; the likely
   slip is counting `ls po/` entries, where the 29th entry is the `qt/`
   subdirectory, not a `.po` file. The row's load-bearing substance (one
   gettext textdomain shared by CLI and GUI, de.po present, MATCH-after-
   D63 classification) is verified. Count-recompute defect class
   (`process-conventions.yaml` entry); evidence-row only, no implementer
   transcribes it. Routed to the controller's consuming edit alongside
   triggers 9/10.

No regressions found outside the delta.

## HARVEST

- **The reversal itself is process gold**: a controller steelman review
  measuring the real effort delta at the code overturned a same-day owner
  decree, and the paper trail is exemplary - the superseded entry keeps
  the original statement and reversal rationale, the superseding entry
  carries the losing steelman, and the design keeps both rulings'
  analyses. "Supersede, never overwrite" for Tier-2 reversals is a
  convention candidate if not already implicit.
- **D64's funnel is the round-1 harvest pattern applied**: round 1
  flagged eight scattered IpcError render sites as a missed one-funnel;
  the CLI test surface now gets exactly that shape (one constructor,
  greppable single-file invariant, pinning by construction). The
  IpcError render funnel remains open as the Plan-9 candidate.
- **Post-sweep invariant framing**: stating a greppable invariant as a
  target with its current falseness acknowledged ("pre-sweep") is the
  honest form round 1's finding 8 wanted - worth keeping as the standard
  way to specify sweep-style work.
- **Locale-pinning as a general class**: D64's insight (embedding a
  second locale converts every output-asserting test into a
  host-locale-dependent test) generalizes to any tool that grows its
  first alternative rendering; the boundary entry's companion-constraint
  mechanism carried it well.
- **Over-restriction watch**: nothing in the delta was stopped that the
  structural-conformance or presentation-token grants should have
  covered. The rejected clap-global `--locale` was correctly routed as
  an interface change rather than absorbed - and correctly recorded as
  an idea, not a defect. D64 itself is a new proposed safeguard held
  under `proc-proposed-safeguard-stays`; first calibration data comes
  when the sweep is built and the grep invariant is measured.

## Whole-document justification

The re-fold is the design's original recommendation landing under a
properly recorded owner reversal, and it arrives at round-1 standards:
D63's mechanism choice rests on a library-behavior claim I verified in
the installed crate's source down to the memoizer construction (the
single-bundle alternative really would mis-pluralize fallback content),
D64's audit rests on platform claims I verified in sys-locale's per-OS
implementations and on a test-surface enumeration that recounts exactly
(11 snapshots 3/3/4/1, five files, 18 invocation sites), the withdrawn
amendment half survives the no-work-needed check against the spec's
actual sentence, and the sweep left no stale English-only claim outside
legitimate historical record. The one defect in the delta is an
off-by-one locale count in an SI-3 evidence row - real, of the recorded
count-defect class, and too small to hold the document for: nothing an
implementer builds consumes it. Approved; the 28-vs-29 correction and
the trigger 9/10 duties ride the controller's consuming edit.
