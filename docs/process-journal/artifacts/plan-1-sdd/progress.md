# Plan 1 progress ledger

Task 1: complete (commits 1f00aa6..a7d550e, review clean, Minor: Cargo.lock committed - accepted)
Task 2: complete (commits a7d550e..a7c0d89 incl. fix a7c0d89, review clean after fix)
  Minor open: all_keys_are_unique failure message could name colliding key (final-review triage)
  Controller note for Task 13: completeness guard should iterate DiagCode::ALL instead of the brief's hand-copied key list
Task 3: complete (commits a7c0d89..c6a2df2, review approved; evidentiary finding resolved by controller-run cargo test: 12/12 real)
  Note: YAML crate resolved as yaml_serde 0.10.4 (underscore) - use in later briefs
  Minor open: parses_nested_any_and_not asserts lengths only (final-review triage)
Task 4: fix in flight (base c6a2df2, first commit 3ab1cdc). INTERFACE CHANGE for later tasks:
  untagged enums now use newtype block variants: FilenameCfg::Template(TemplateBlock{template}),
  TitleCfg::Template(TemplateBlock), SourceCfg::External(ExternalBlock{external: Locator}),
  ChaptersCfg::External(ExternalBlock). Later briefs (esp. Task 9 validate code) matching
  SourceCfg::External { external } / FilenameCfg::Template { template } must adapt patterns.
Task 4: complete (commits c6a2df2..b5eaa3d incl. fix b5eaa3d, review clean after fix)
  Minor open: reference_profile_parses does not deep-assert the six subtitle rules; locator recursive/case_sensitive defaults untested (final-review triage)
Task 5: complete (commits b5eaa3d..e78847d incl. codegen rename fix e78847d, review clean)
  Note: module is xtask::codegen (gen reserved in edition 2024); CLI subcommand string stays gen-capability
  Minor open: dedup tie-break if schema ever nests keys named type/codec/id - theoretical (final-review triage)
Task 6: complete (commits e78847d..4750abb, review clean; schema v20)
  Minor open: rustdoc missing on capability pub fns; SETTABLE test covers 3/10 entries (final-review triage)
Task 7: complete (commits 4750abb..2254eca incl. fix 2254eca, review clean after fix)
  Notes for later tasks: TemplateError::pos is a CHAR offset (never byte-slice); field_names() does not dedupe; UnknownFilter{name:""} for {x:}
  Minor open: error tests assert variant only, not payload values (final-review triage)
Task 8: complete (commits 2254eca..6827894, review clean; 48 workspace tests green, controller-verified)
  Deferred into Task 9 dispatch: flatten InvalidRegex detail param to single line (regex::Error Display is multi-line)
Task 9: complete (commits 6827894..9fe1e43, review clean; 58 tests green)
  Minor open: EmptyExtensions locator half untested; filename keyword misuse untested (final-review triage)
  For Task 13: pre-existing clippy collapsible_if in validate.rs:246 must be fixed for the -D warnings gate (plan step 4 covers it)
Task 10: complete (commits 9fe1e43..90d04bc incl. test additions 90d04bc, review clean; 66 tests, controller-verified)
Task 11: complete (commits 90d04bc..bcf078b, review clean; 68 tests; schema output smoke-tested)
  Minor open: schema test is substring-based (plan-mandated); predicates dep unused until Task 12
Task 12: complete (commits bcf078b..ad841b0 incl. fix ad841b0, review clean after fix; 75 tests; live-verified text+json output)
Task 13: complete (commits ad841b0..c7a70f7, review clean; full CI gate green locally: fmt, clippy -D warnings, 76 tests)
ALL 13 TASKS COMPLETE. Final whole-branch review pending.
Final review: NOT READY - fix wave dispatched. Important: (1) template-error prose params in core, (2) UnknownProperty spec/catalog naming, (3) match_to_source:false spurious LocatorConflict. Fix-now minors: unique-key msg, nested-content asserts, locator defaults, SETTABLE 10/10, locator EmptyExtensions + filename keyword tests, parse-error $at dedup.
Plan 2 handoff notes (from final review): rendered-filename separator re-check in planner; unify Locator.path vs output.directory types; define codec_kind substring/regex semantics or restrict to exact; value-domain lint decision (type/language values); param fixtures for planning-time catalog entries when emitters land; lock unmatched '}' template behavior with a test.
Fix wave 3c24845 + spec amendments cd3f239: final reviewer CONFIRMED READY. PLAN 1 COMPLETE at cd3f239 (81 tests, fmt+clippy clean, 3-OS CI committed).
Non-blocking nits for next touch: load.rs still sets unused `at` param; invalid-template selector default variant would mislabel an unknown kind.
Docs backfill: c402914 + 9d61813 (per-variant DiagCode) + f7afa8d (spec precision) + 9a7f49f (review polish). deny(missing_docs) on all three lib roots. Review approved. Pushed to github.com:senolfeldmann/Muxsmith master; first CI run pending.
First CI run 28931578050: SUCCESS on all 3 OSes (2m44s). Repo currently PRIVATE - Actions burns free-tier minutes with macOS 10x multiplier; matrix-trim or go-public decision pending with Şenol.
