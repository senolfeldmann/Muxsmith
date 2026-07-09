# Task 6 report: resolve title and tags

## What was done

Added two private helpers to `crates/muxsmith-core/src/planner.rs` and wired
them into the `Plan { .. }` construction in `resolve_file`, replacing the
Task-4 hardcoded defaults (`TitleAction::Keep`, `TagFlags { global_keep:
true, track_keep: true }`).

- `resolve_title(profile: &Profile, primary: &PrimaryFile) -> TitleAction`
  - `TitleCfg::Keyword("clear")` -> `Clear`.
  - `TitleCfg::Keyword(_)` (i.e. `"keep"`, and defensively any other value)
    -> `Keep`.
  - `TitleCfg::Template(block)` -> builds a `Ctx` from
    `primary.identifier.to_ctx()` plus `source_stem` (see "Design decision"
    below), parses the template, and returns `Set(rendered)` on success or
    `Keep` on a parse failure (never panics, no `unwrap` on the `Result`).
- `resolve_tags(profile: &Profile) -> TagFlags`: direct
  `KeepDrop::Keep`-equality mapping for `global` and `track`.

Both are called once per file in `resolve_file`, before the `Plan { .. }`
construction, and their results (`title`, `tags`) are passed in by value.

## Design decision: `source_stem` in the title Ctx

The brief's Step 3 text says only "render with `primary.identifier.to_ctx()`
via `render_literal`", without mentioning `source_stem`. But
`crates/muxsmith-core/src/profile/validate.rs` (lines ~112-160) builds the
allowed-field list for `title.template` identically to
`output.filename.template`: both start from `template_fields` (the
`input.pattern` capture groups) and then push `"source_stem"`. If
`resolve_title`'s `Ctx` omitted `source_stem`, a profile using `{source_stem}`
in `title.template` would validate successfully (validate.rs allows the
field) but silently render as an empty string at plan time (a mismatch
between what's declared valid and what's actually bound) -- exactly the kind
of latent bug the "mirror that usage" hint in the task context was pointing
at. I resolved this by copying `render_output`'s exact Ctx-construction idiom
(`to_ctx()` + conditional `source_stem` set from the file stem) into
`resolve_title`, and added a regression test
(`title_template_supports_source_stem_field`) and a comment cross-referencing
validate.rs so the two field lists don't drift apart again. I did not
refactor `render_output` to share a helper with `resolve_title`; scope
discipline said "only title + tags", so I duplicated the ~4-line idiom
in the new function instead of touching the existing, already-tested
`render_output`.

## TDD: RED

Added 7 tests to `crates/muxsmith-core/tests/planner_resolution.rs`
(4 mandated by the brief, 3 additional regression tests for the invariants
the brief called out or implied):

- `title_clear_resolves_to_clear`
- `title_keep_resolves_to_keep`
- `title_template_renders_raw_capture_into_set`
- `title_template_rendering_empty_is_a_legitimate_set` (extra: locks in
  "no empty-name invariant for titles")
- `title_template_supports_source_stem_field` (extra: locks in the design
  decision above)
- `tags_global_drop_track_keep_resolves_to_flags`

`cargo test -p muxsmith-core --test planner_resolution title` before
implementation:

```
running 5 tests
test title_keep_resolves_to_keep ... ok          <- trivially green (matches the old hardcoded default)
test title_clear_resolves_to_clear ... FAILED
test title_template_supports_source_stem_field ... FAILED
test title_template_rendering_empty_is_a_legitimate_set ... FAILED
test title_template_renders_raw_capture_into_set ... FAILED

test result: FAILED. 1 passed; 4 failed
```

`cargo test -p muxsmith-core --test planner_resolution tags`:

```
running 1 test
test tags_global_drop_track_keep_resolves_to_flags ... FAILED
  left: TagFlags { global_keep: true, track_keep: true }
 right: TagFlags { global_keep: false, track_keep: true }

test result: FAILED. 0 passed; 1 failed
```

`title_keep_resolves_to_keep` was expected to pass trivially pre-implementation
since it asserts the same value the old hardcoded default already produced;
it still exercises the real `resolve_title` code path once implemented, so it
stays as a genuine (if not RED) regression test.

One iteration note during RED: my first version of
`title_template_renders_raw_capture_into_set` used the on-disk filename
`Show.S01E01.mkv` but expected `season` = `"03"`, copying the brief's example
value without matching it to the fixture's actual filename. Caught by the
first test run (`left: Set("Show S01") right: Set("Show S03")`); fixed by
using `Show.S03E01.mkv` as the on-disk name so the capture group actually
produces `"03"`. Not a RED-phase failure of the implementation, a test-authoring
mistake caught by running the test.

## GREEN

After implementation, `cargo test -p muxsmith-core --test planner_resolution`:
32 passed, 0 failed (all pre-existing tests plus the 7 new ones).

## Full gate (run once before commit)

- `cargo test --workspace`: all green (every crate/test-binary reported
  `test result: ok`, no `FAILED`).
- `cargo fmt --all --check`: exit 0, no diff.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/src/planner.rs`: import `KeepDrop`, `TitleCfg`;
  added `resolve_title`, `resolve_tags`; wired both into `resolve_file`'s
  `Plan { .. }` construction, replacing the Task-4 hardcoded defaults.
- `crates/muxsmith-core/tests/planner_resolution.rs`: 7 new tests (see
  above).

No changes to chapters, attachments, or `changes` resolution (Tasks 5, 7, 8
untouched). No changes to `Plan`/`TagFlags`/`TitleAction` struct shapes
(already added in Task 4).

## Self-review

- Both new functions are private (`fn`, not `pub fn`); `#![deny(missing_docs)]`
  does not require doc comments on them, though I gave both a `//` comment
  explaining intent (matching the file's existing convention for private
  helpers like `render_output`, `resolve_changes`).
- No `unwrap`/`expect`/panic added on any path through `resolve_title`;
  `Template::parse`'s `Result` is matched explicitly, falling back to `Keep`
  on `Err` rather than propagating a panic, exactly as the brief requires
  for the "cannot occur post-validate" case.
- `resolve_tags` has no fallible path at all (`KeepDrop` is a closed
  two-variant enum already validated by serde deserialization), so no
  defensive fallback is needed there.
- Verified `TagFlags` and `TitleAction` already derived `PartialEq` (added in
  Task 4), so the new tests' `assert_eq!` calls compile without further
  trait work.
- Confirmed scope: `git diff --stat` touches exactly the two files the brief
  named (`planner.rs`, `tests/planner_resolution.rs`); `git status --short`
  shows no other modified/staged files (an unrelated untracked `HANDOFF.md`
  from a prior session is left alone).
- Considered extracting a shared `primary_ctx()` helper between
  `render_output` and `resolve_title` to eliminate the ~4-line duplication,
  but decided against it: the task's scope discipline said "only title +
  tags", and refactoring `render_output` (Task 4/prior work, already tested)
  is out of scope for a task that only needs to add new behavior. Flagging
  this as a possible follow-up cleanup, not a defect.

## Concerns

None blocking. One minor judgment call to flag explicitly (see "Design
decision" above): I extended the Ctx to include `source_stem` for title
templates, going beyond the brief's literal Step-3 wording but matching
validate.rs's actual allowed-field list and the task context's "mirror that
usage" instruction. I'm confident in this call given the direct cross-file
evidence, but it is a place where I filled in a gap between the brief's
prose and the codebase's actual cross-file contract rather than asking
before proceeding, per the "state a marked assumption and act" guidance for
this kind of well-evidenced gap.

Note: this file previously held a report for an unrelated, already-completed
"Task 6: Capability module" from an earlier plan iteration/numbering; that
content has been replaced with this task's report.
