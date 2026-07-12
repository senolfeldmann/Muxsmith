# Muxsmith conventions

The always-current house rulebook: how we write code in this repo, right
now. This is the materialized current-state view over our decisions; the
D-memos (`docs/superpowers/specs/*-design.md`) are the dated, append-only
event log with the full reasoning, and each rule here points to its memo
for the why. When a rule changes you edit THIS file (and record why in a
memo); you never edit a memo, you supersede it.

How entries get here (recording, recurrence counter, threshold, the
deliberation trigger): software-dev-process doctrine, section 7. Tier-1 raw
considerations and their recurrence counts live in `docs/decision-ledger.md`;
an item is promoted here at count 3.

## Patterns (do it this way here)

- **One GUI report document shape.** Every GUI report command returns the
  same document (`config_only` / `batch` / `run_document` via
  `report::json`), so the frontend has ONE rendering path, not one per
  command. New report commands conform. (src-tauri/src/lib.rs.)
- **Diagnostics through the catalog.** User-facing diagnostics are
  constructed as `Diagnostic`/`DiagCode` and rendered via the Fluent
  catalog; never format a diagnostic string inline.
- **Error-first ordering everywhere.** Diagnostics print worst-first
  (`Reverse(severity)`); every surface uses the shared `severity_sorted` /
  the same fold. (commands/mod.rs.)
- **Vue props: `defineProps` + `props.x`.** Prop-taking SFCs declare
  `const props = defineProps<...>()` and read `props.x`. All SFCs conform;
  a lone deviation is the outlier, not a new style.
- **Shared test helpers via `tests/support/mod.rs`.** Cross-file test
  helpers live in the crate's `tests/support` subdirectory module (not a
  `tests/*.rs` file, which Cargo compiles as its own binary); same-crate
  duplication of a helper is a defect.
- **Bilingual Fluent.** New or changed user-facing messages land in both
  locales (en + de) in the same change; the parity gate enforces it.
- **Pin everything.** Exact version pins (toolchain, JS deps), SHA-pinned
  GitHub Actions; a floating version is a defect, not a convenience.

## Restraints (deliberately NOT done, with why)

- **No reactive-props destructure in Vue SFCs.** `defineProps` + `props.x`
  (with `withDefaults` for defaults) is the house form; reactive-props
  destructure would make a component the lone outlier vs its siblings. The
  steelman for destructure (terser, 3.5-supported) lost to internal
  consistency. (Idiomacy review 2026-07-12, refuted finding.)
- **No mkvtoolnix input-convenience guesses.** Filename-derived
  language/flags, auto-title, unique-name suffixing, sequence auto-append
  are deliberately NOT emulated: Muxsmith is declarative-batch, the profile
  is the spec, not an interactive tool that guesses. (docs/IDEAS.md 1-4.)
- **mise is a dev tool, not a CI tool.** CI must not fetch a floating mise
  binary at run time. (Post-1.0 removal tracked, ROADMAP v1.x.)

## Non-decisions (can't decide yet; blocked on B; reactivate when B clears)

- **Injectable-planner-seam interface** (S4/S5/S6). Blocked on internal
  progress: the profile-editor design in Plan 6, since a shared
  `plan_pipeline()` IS the seam and the four-copy planning pipeline hoist
  is folded into Plan 6. Reactivate when Plan 6 brainstorming starts.
  (ROADMAP Plan-6 anchor.)
