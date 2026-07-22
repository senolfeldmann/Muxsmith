# Plan 7 design review, round 4 (plan-authoring amendments)

Artifact: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`.
Same reviewer; scope is the three plan-authoring amendments plus their
two ripples (spec amendment 6(a), section 9's funnel line). Everything
else stands settled by rounds 1-3 and owner approval. All claims below
verified on the artifact, the tree, and the built binary; foreground.

## Verdict: APPROVED

No findings. The delta is clean.

## Dispositions

1. **D64's `muxsmith_bare()` exception - VERIFIED, all four aspects.**
   - *The behavioral claim, measured on the built binary*: bare
     `muxsmith` prints the missing-subcommand usage (command list) and
     exits 2; `muxsmith --locale en` fails with clap's
     `error: unexpected argument '--locale' found` and exits 2. Same
     assertion result (`.failure()` passes either way), different
     verified behavior - the silent test-meaning-change rationale is
     exactly right, and the test body (`cli_schema.rs:26-28`,
     `Command::cargo_bin("muxsmith").unwrap().assert().failure()`)
     matches the cited lines.
   - *Pinning-moot claim, verified in source*: `main.rs:9` runs
     `cli::Cli::parse()` first; `Renderer::new` is constructed only
     inside the subcommand arms (`:21,:32,:43,:56`). A top-level clap
     usage error exits before any Renderer exists, so both failure
     outputs are clap library-generated text (spec 8.4's accepted
     exception) - no locale-dependent rendering occurs, on any host
     locale. Locale pinning is genuinely meaningless for this test, so
     the exception does not erode `cli-multilang-rendering`'s purpose
     (host-locale-independent suite).
   - *Grep invariant survives verbatim*: the bare helper lives in
     `tests/support/mod.rs` too, so "cargo_bin("muxsmith") appears in
     exactly one file" is untouched - confirmed the invariant bullet is
     byte-identical, and the helper correctly does not pre-exist in the
     tree (design, not implementation).
   - *Closed-ness and consistency*: the exception names the helper, its
     exactly-one caller, and the reopening rule ("a second caller
     reopens D64 rather than riding the helper"); the "Where it applies"
     enumeration now splits cli_schema.rs's 2 measured sites
     (schema_json via funnel, no_args via bare helper); section 9's
     must-not-decide line carries the same exception with the same
     reopening rule. Provenance recorded as a controller ruling at
     plan-authoring - correctly routed (internal technical fork per
     proc-latitude's routing rule) and correctly recorded in the ADR.
2. **D62 empty-state note - VERIFIED CORRECT against the check
   definitions.** Check 3 as defined ("the set of locale directories
   under `help/` must equal the set under `locales/`") fails
   unconditionally on an absent `help/` tree (no set vs `{en, de}`);
   check 1 fails from the first annotated control. The rewritten note
   states both, and its conclusion - the gate cannot land green before
   `help/en/` and `help/de/` exist - now follows from the definitions
   instead of contradicting them. The earlier single-check phrasing
   (implying the gate could precede the tree) is gone.
3. **D54 / amendment 6(a) / D52 alignment - VERIFIED, one semantic,
   no third variant.** D54's totals sentence now states the D52
   mechanics exactly (hover off-table -> `hoverId` null -> pinned topic
   else view topic), names D52 as the interaction authority, and
   honestly records the visible consequence (fallback when an annotated
   topic was showing unpinned). Amendment 6(a) carries the identical
   semantic into spec 8.3. D52's deliberately unchanged click sentence
   ("on an unannotated target, no topic change") is consistent rather
   than a third variant: at click time the preceding hover has already
   set `hoverId` null, so the displayed topic is already
   pinned-else-view and the click's effect is genuinely nil - the
   sentence describes the click event, not the hover fallback. A
   full-document sweep for the old paraphrase finds only D52's
   consistent click sentence and D54's own alignment marker; no stale
   variant survives. No behavior changed - the paraphrases were aligned
   to the owner-approved D52 mechanics, so no fork was reopened.

## HARVEST

- **The exception pattern is the right shape**: a funnel exception that
  (a) is forced by a measured behavior difference, not convenience,
  (b) lives in the same support module so the greppable invariant is
  untouched, (c) names its exactly-one caller, and (d) defines what
  reopens the ADR. Worth reusing verbatim next time a sweep-style
  invariant meets a legitimate outlier: the invariant survives because
  the exception is itself enumerated and greppable, not waived.
- **Plan-authoring as a review layer keeps earning**: all three defects
  were caught when someone tried to *build* from the document (the
  house's recorded pattern - the D41/D44 class from plan 6, now caught
  before dispatch instead of after). The forcing function is real and
  cheap; nothing here suggests the design rounds should have been
  longer instead.
- **Aligned-paraphrase discipline**: D54's fix names D52 as the single
  interaction authority and demotes its own sentence to a follower.
  Declaring one section the authority and marking restatements as
  followers is the cheap immunization against the
  divergent-paraphrase class - candidate for the house design-doc
  pattern.
- **Over-restriction watch**: nothing stopped that the standing grants
  should have covered. The `muxsmith_bare` exception is the
  watch's positive mirror - the funnel rule was deliberately tight, hit
  a legitimate outlier at first contact, and the resolution was an
  enumerated carve-out plus a reopening trigger rather than either
  silent bypass or wholesale loosening. That is the calibration
  mechanism working as designed.

## Whole-document justification

The round-4 delta is three surgical corrections of exactly the kinds the
earlier rounds established as this document's standards: a test-surface
invariant meeting its first legitimate exception and recording it closed
(with the load-bearing behavior difference verified here on the real
binary - bare usage-failure vs clap unexpected-argument, Renderer
provably never constructed for either); an empty-state description
brought into agreement with the gate's own check definitions; and a
hover-semantics paraphrase aligned across its three sites to the one
authoritative mechanics section, with the deliberately unchanged click
sentence confirmed consistent rather than divergent. No behavior
changed, no fork reopened, no enumeration loosened, and the greppable
invariant survives verbatim. The document remains what rounds 2 and 3
approved, now with its plan-facing text agreeing with itself; approved
without reservation.
