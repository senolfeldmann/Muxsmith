# Task 3 report: D46 - keyword domains from one constant set

## Status: DONE

## What was implemented

**`crates/muxsmith-core/src/profile/model.rs`.** Four `pub const KEYWORDS:
&'static [&'static str]` associate consts, one per keyword-bearing untagged
enum, placed beside the enum they belong to:

```rust
impl FilenameCfg { pub const KEYWORDS: &'static [&'static str] = &["keep"]; }
impl SourceCfg   { pub const KEYWORDS: &'static [&'static str] = &["primary"]; }
impl ChaptersCfg { pub const KEYWORDS: &'static [&'static str] = &["keep", "drop"]; }
impl TitleCfg    { pub const KEYWORDS: &'static [&'static str] = &["keep", "clear"]; }
```

(`ChaptersCfg` and `TitleCfg` previously had no inherent `impl` block besides
`impl Default`; added one.) Each `Keyword(String)` arm keeps its `String` and
gets `#[schemars(schema_with = "<enum>_keyword_schema")]`. A shared private
helper `keyword_domain_schema(domain) -> Schema` builds the `{"type":
"string", "enum": [...]}` shape (D46's decision, not `oneOf`+`const`); the
four `*_keyword_schema` wrapper functions match the `fn(&mut SchemaGenerator)
-> Schema` signature `schema_with` requires and call the helper with the
matching `KEYWORDS` const. The doc comment on each `Keyword` variant is left
untouched; schemars merges it into the branch's `description` on top of the
overridden schema (confirmed against schemars 1.2.1 source:
`schemars_derive-1.2.1/src/schema_exprs.rs:600` calls
`variant.add_mutators(&mut schema_expr)` unconditionally after the
`schema_with` function runs, and `insert_metadata_property_if_nonempty` in
`schemars-1.2.1/src/_private/mod.rs:324` inserts unconditionally since the
override never sets `description` itself).

**`crates/muxsmith-core/src/profile/validate.rs`.** The four match guards at
(pre-edit) `:105`, `:129`, `:149`, `:166` now read
`<Enum>Cfg::Keyword(k) if <Enum>Cfg::KEYWORDS.contains(&k.as_str())`, and the
four `.with("allowed", <literal>)` calls now read `.with("allowed",
domain_hint(<Enum>Cfg::KEYWORDS))`, reusing the pre-existing `domain_hint`
helper (`:430-437`, unchanged).

**Tests.** `crates/muxsmith-cli/tests/cli_schema.rs`: extracted the shared
`schema_json()` helper (both existing invocations collapsed into one),
re-pointed `schema_prints_json_schema_and_exits_zero` at it, added
`keyword_domains_project_as_closed_enums_not_bare_strings` verbatim from the
brief. `crates/muxsmith-core/tests/validate_semantics.rs`: added
`misspelled_chapters_keyword_is_invalid_keyword_with_const_derived_allowed`
(`chapters: kepp` -> `InvalidKeyword` with `found: "kepp"`,
`allowed: "keep, drop"`).

## TDD evidence

RED (before any implementation, helper extracted, new test added, consts/schema
not yet touched):

```
$ cargo test -p muxsmith-cli --test cli_schema keyword_domains
running 1 test
test keyword_domains_project_as_closed_enums_not_bare_strings ... FAILED
thread '...' panicked at crates/muxsmith-cli/tests/cli_schema.rs:49:32:
FilenameCfg's string branch must carry an enum, not a bare string type
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out
```

GREEN (after consts + `schema_with` projections landed, guards/domain_hint not
yet touched -- schema projection and validation are independent halves):

```
$ cargo test -p muxsmith-cli --test cli_schema keyword_domains
running 1 test
test keyword_domains_project_as_closed_enums_not_bare_strings ... ok
```

Empirical schema shape (matches D46 `:1112-1120`'s measured output exactly,
including the merged `description`):

```
FilenameCfg -> [{"description": "...", "$ref": "#/$defs/TemplateBlock"},
                {"description": "Keyword form; the only accepted value is \"keep\" (...)",
                 "type": "string", "enum": ["keep"]}]
ChaptersCfg -> [{"description": "...", "$ref": "#/$defs/ExternalBlock"},
                {"description": "Keyword form: \"keep\" or \"drop\".",
                 "type": "string", "enum": ["keep", "drop"]}]
```

`InvalidKeyword` reachability (step 6, GREEN once guards were pointed at the
consts):

```
$ cargo test -p muxsmith-core --test validate_semantics misspelled_chapters
test misspelled_chapters_keyword_is_invalid_keyword_with_const_derived_allowed ... ok
```

## Snapshot-diff proof, and a finding about what it actually proves

Prescribed check, run after all edits landed:

```
$ git diff --exit-code crates/muxsmith-cli/tests/snapshots/
$ echo $?
0
```

Exits 0, no output, as required.

**Finding: this specific check is vacuous for the class of defect it is meant
to catch.** Per the standing rule that a check whose passing result is an
absence must be proven to fire before it is trusted, I deliberately broke
`ChaptersCfg::KEYWORDS` (added a bogus third element) and re-ran everything.
Result: `cargo test -p muxsmith-cli --test cli_validate --test dry_run_cli
--test run_cli --test run_live` (the four suites that own
`crates/muxsmith-cli/tests/snapshots/*.snap`) stayed **green**, and `git diff
--exit-code crates/muxsmith-cli/tests/snapshots/` still exited **0** -- clean
even though the const-derived `allowed` value was wrong. Cause: none of the
eleven pinned `.snap` fixtures (`cli_validate`, `dry_run_cli`, `run_cli`,
`run_live`) exercise an invalid-keyword profile; `bad.yaml`/`good.yaml` only
hit regex-error, type-mismatch, and overlap-warning diagnostics. `insta` also
never rewrites a tracked `.snap` in place on mismatch (it fails the test and
writes an untracked `.snap.new`), so even a real regression in a snapshot
would show as a test **failure**, not as `git diff` output -- the diff check
adds no signal beyond "did `cargo test` pass," and the specific byte-identity
claim it's cited to prove (the four hand-typed `allowed` strings vs. the
const-derived ones) is never touched by any pinned fixture.

What *does* prove the byte-identity claim: the same deliberate-break run
against `cargo test -p muxsmith-core --test validate_semantics` and `-p
muxsmith-cli --test cli_schema` both caught it immediately
(`keyword_domains_project_as_closed_enums_not_bare_strings` failed on the
schema shape, `misspelled_chapters_keyword_is_invalid_keyword_with_const_derived_allowed`
failed on `assert_eq!(d.params["allowed"], "keep, drop")`). Ran a throwaway
(not committed) fifth assertion covering all four domains at once before
reverting the break, confirming byte-for-byte: `chapters -> "keep, drop"`,
`output.filename -> "keep"`, `title -> "keep, clear"`, `tracks[0].source ->
"primary"` -- exactly the four pre-refactor hand-typed literals. The const
restored, full workspace re-verified green, snapshot diff re-confirmed clean
(now genuinely, not vacuously, since it was rerun on correct code).

Net: the refactor is correct and proven, but by the two new unit-level tests,
not by the CLI-snapshot invariant the brief and D46 name as the proof
mechanism. Flagging per NEEDS_CONTEXT-adjacent duty (not blocking, since the
outcome is correct and no design latitude was exercised to get there) --
worth a note back to the design/plan owner that D46's "the existing snapshot
tests... prove it" claim doesn't hold for this task's specific `allowed`-value
concern, in case a later task relies on the same snapshot suite to guard a
similar keyword-domain claim.

## Gate results (nine parts, foreground, run to completion, then re-run once
more in full immediately before commit)

1. `cargo fmt --all --check` -- clean (one round of `cargo fmt --all` was
   needed after pasting the brief's test code verbatim; ran once, confirmed
   clean after).
2. `cargo clippy --workspace --all-targets -- -D warnings` -- clean, 0
   warnings.
3. `cargo test --workspace` -- 476 passed, 0 failed, 0 ignored, across all
   crates (core, cli, gui lib, xtask, codegen, doc-tests).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- clean, 0
   warnings.
5. `cargo deny check` -- `advisories ok, bans ok, licenses ok, sources ok`;
   pre-existing duplicate-dependency warnings (base64, toml, etc., all from
   the Tauri/GUI dependency tree) are unrelated to this change and do not
   fail the check.
6. `pnpm lint` -- clean.
7. `pnpm build` -- clean, `vue-tsc --noEmit && vite build` succeeded.
8. `pnpm check:i18n` -- `ok (17 source files scanned, 181 catalog ids, 12
   unused warning(s), ...)`; the 12 unused-key warnings are pre-existing GUI
   catalog entries, untouched by this task (this task adds no Fluent keys).
9. `pnpm test:e2e` -- 7/7 Playwright tests passed.

`node_modules` already existed in the worktree from Task 2; no `pnpm install`
needed.

## Files changed

- `crates/muxsmith-core/src/profile/model.rs` (+4 `KEYWORDS` consts, +4
  `schema_with` fns, +1 shared schema-shape helper, +4
  `#[schemars(schema_with = ...)]` attributes)
- `crates/muxsmith-core/src/profile/validate.rs` (4 guards, 4 `allowed`
  params repointed at the consts + `domain_hint`)
- `crates/muxsmith-cli/tests/cli_schema.rs` (`schema_json()` helper
  extracted, 1 new test)
- `crates/muxsmith-core/tests/validate_semantics.rs` (1 new test)

Commit: `7134bb6` "core: keyword domains project into the schema from one
constant set (D46)", on branch `plan6-a`, not pushed. Working tree clean
after commit (`git status --porcelain` empty).

## Self-review findings

- **Consts byte-identical to today's hand-typed strings**: verified both by
  code inspection (`domain_hint` is a plain `.join(", ")` for domains <= 8,
  and all four domains are 1-2 elements, so it degenerates to the literal
  strings) and empirically via the deliberate-break/restore exercise above,
  which exercised all four `allowed` sites in one profile
  (`output.filename`, `tracks[0].source`, `chapters`, `title` all invalid at
  once) and got back exactly `"keep"`, `"primary"`, `"keep, drop"`, `"keep,
  clear"`.
- **Doc comments merged into descriptions, not replaced**: confirmed by
  reading the actual `muxsmith schema` JSON output (see TDD section above);
  also traced the mechanism in schemars_derive source rather than trusting
  the design doc's claim at face value, since it's a load-bearing premise
  for the whole decision.
- **Snapshots unmoved**: `git diff --exit-code` on the snapshots dir is 0,
  but see the finding above -- this check doesn't carry the weight the brief
  assigns it; the real proof is the two new unit tests.
- **`Keyword(String)` arm untyped**: confirmed unchanged; grepped the diff,
  no `enum Keyword` typing was introduced, `InvalidKeyword` stays reachable
  (proven by the new validate_semantics test).
- **`enum`, not `oneOf`+`const`**: confirmed in the raw schema JSON dump
  above -- each branch is `{"type": "string", "enum": [...]}`, no `oneOf` or
  `const` anywhere in the four projections.
- **Placement**: all four consts and their `schema_with` fns live in
  `profile/model.rs`, none in `capability/`, matching D46's placement
  rationale.
- **Rustdoc**: all four `pub const KEYWORDS` have doc comments (required,
  `#![deny(missing_docs)]`); the private helper and the four
  `*_keyword_schema` fns are not `pub` so not lint-gated, but each still
  carries a one-line `///` comment for readability.
- **Typography**: ASCII hyphens and straight quotes throughout the diff (grep
  confirmed no em/en-dash, no curly quotes, no Unicode ellipsis introduced).
- **Scope discipline**: staged and committed exactly the four files the
  brief named; `git status --porcelain` was empty after commit; no stray
  worktree/branch touched (`.worktrees/plan6-a` / `plan6-a` only).

## Concerns

1. The snapshot-diff-as-proof finding above: not a defect in the delivered
   code (the refactor is correct, proven by the unit tests), but a gap in
   the specified verification method that's worth surfacing to whoever owns
   D46/the plan-6 design, since a future task might lean on the same "CLI
   snapshots as proof" pattern for a claim the snapshots don't actually
   reach.
2. None of the four `*_keyword_schema` wrapper functions or the shared
   `keyword_domain_schema` helper are covered by a unit test directly (only
   transitively via the CLI schema test and manual JSON inspection during
   this task). Given the CLI-level test already asserts the exact enum
   membership and shape end-to-end, a core-level unit test would be
   redundant, but flagging the coverage shape in case Task 5's emitter
   consumer surfaces a gap.
