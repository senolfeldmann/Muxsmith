# Plan 2 fix pass - FINAL whole-branch review

Independent, read-only, no stake. Reviewed holistically for cross-cutting
correctness the per-task reviews could not see (F5+F6 post-pass interaction,
the controller-only tasks F2/F3/F9, spec-amendment fidelity, regressions).

## Gate commands (all green)

| Command | Result |
|---|---|
| `cargo test --workspace` | 162 passed; 0 failed; 0 ignored |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean (exit 0) |
| `cargo fmt --all --check` | clean (exit 0) |
| `cargo deny check` | advisories/bans/licenses/sources ok (exit 0) |

## Verdict

**changes-needed** - one Important regression (F6 render_output), everything
else is Minor or acceptable-as-designed. If Şenol chooses to ship and file the
F6 case as an immediate follow-up, that is defensible; it is not a
data-loss/overwrite bug. But it is a genuine regression the pass introduced in
the exact invariant it set out to harden, so the honest gate result is
changes-needed.

---

## IMPORTANT

### I1. F6: template rendering to `.mkv` produces a hidden empty-stem output instead of `EmptyRenderedName` (regression)

`crates/muxsmith-core/src/planner.rs` `render_output` (lines 470-496).

The rewritten empty-name check tests the **raw pre-append** `rendered` value:

```rust
if rendered.is_empty() || rendered == "." || rendered == ".." { EmptyRenderedName; return None; }
```

The pre-fix code (visible in the diff) stripped a trailing `.mkv`/`.MKV`
**before** the emptiness test:

```rust
let stem_only = name.strip_suffix(".mkv").or_else(|| name.strip_suffix(".MKV")).unwrap_or(&name);
if stem_only.is_empty() || name == "." || name == ".." { EmptyRenderedName }
```

So for a template that renders to exactly `.mkv` (empty stem, extension already
present): pre-fix `stem_only == ""` -> caught; post-fix `rendered == ".mkv"` is
not empty/`.`/`..` -> passes, then the template arm sees it already ends in
`.mkv` and keeps it verbatim -> output path is the hidden file `.mkv`.

Verified: probed the exact string logic (`.mkv` fails all three checks,
`Path::new(".mkv").file_stem() == Some(".mkv")`, extension `None`); Template
parse of a brace-free `.mkv` yields `.mkv`; `validate_template(".mkv", ...)`
passes (no braces, no unknown field, no separator). Nothing downstream catches
it: single-file batch, output does not pre-exist, not planned twice, not an
input path -> no OutputCollision, no SourceOverwrite.

**Failure scenario:** `output.filename: { template: '.mkv' }` (or any template
rendering to an empty stem plus the literal extension, e.g. `{g2}.mkv` where an
optional capture group `g2` did not participate). Single primary. dry-run
reports a valid plan with output `.mkv` and exit 0; a real run writes a hidden
file named `.mkv` in the output directory. This is exactly the degenerate
output that spec 4.8 ("an empty stem or `.`/`..` is EmptyRenderedName") and the
D4 rationale (which literally names the hidden `.mkv` file as the thing to
prevent) exist to reject. The pre-fix code rejected it; the fix pass lost that.

Also covers case variants (`.MKV`, `.mKv`) via the case-insensitive
`ends_with`. The keyword arm is unaffected (a normal source stem is non-empty,
and a source literally named `.mkv` has no `.mkv` extension so it is never
discovered as a primary).

**Fix direction:** perform the emptiness/dot check on the stem after stripping a
trailing case-insensitive `.mkv` (i.e. reinstate `stem_only`), or check the
final `name`'s stem. The plan text for F6 in fact says "test the pre-`.mkv`
stem", which the implementation did not do - it tested the raw rendered value.

**Coverage gap:** the new tests cover template `.` (-> `..mkv`, the original bug
B) and template `""` (empty), but not template `.mkv`. Add that case.

---

## MINOR

### M1. F3: `match:` consisting solely of an empty `any: []`/`not: []` double-reports

`validate.rs` line 65 pushes `EmptyMatchExpression` (warning) when
`rule.match_expr.is_empty()`, and `MatchExpr::is_empty()` (match_expr.rs
79-85) returns `true` when `any`/`not` is `Some` but empty. `validate_expr`
then additionally pushes `EmptyMatchList` (error) for the same list. So
`match: { any: [] }` (with nothing else) emits BOTH a warning and an error.

Not incorrect: both statements are true, and the error dominates the exit code
(2). Only noisy. It does NOT double-report when the match has other content
(`exact: {...}, not: []` -> `is_empty()` false -> only `EmptyMatchList`), which
is the common case. Acceptable; if you want it clean, suppress
`EmptyMatchExpression` when an `EmptyMatchList` already fired on the same rule.
The F3 test only asserts `contains(EmptyMatchList)`, so it does not notice the
extra warning.

### M2. F5: a donor referenced only by a render-failed file escapes the batch-wide SourceOverwrite set

`detect_source_overwrites` (planner.rs 507-525) gathers the input set from
`plan.assignments` of files whose `plan` is `Some`. A file whose own
`render_output` returned `None` (its output name was invalid:
`EmptyRenderedName`/`PathSeparatorInRenderedName`) has `plan == None` from the
start, and its assignments (including any donor paths it resolved) are
discarded in `resolve_file` (`output.map(|output| Plan { ... assignments })`).
So if primary A renders onto donor X, and X is referenced *only* by such a
render-failed file B, A does not get `SourceOverwrite` and a real run would
clobber X.

Extremely narrow (requires B to have simultaneously failed to render its own
name, and X to be referenced by no non-failed file and to not be a primary).
Files dropped for *resolution* errors (MissingTrack, AmbiguousRule, ...) still
have `plan == Some` at the point `detect_source_overwrites` runs (it precedes
the first `finalize_plans`), so their donors ARE protected - the gap is
specifically render-failure, where the assignments never survive into a Plan.

### M3. `IdentifyError` Display puts core-authored English into the `detail` param

`identify.rs` Display impl emits prefixes like `"mkvmerge failed: "`,
`"cannot read file: "`, `"invalid identification JSON: "`, which flow into the
`UnidentifiableSource` diagnostic's `detail` param and are rendered verbatim.
Only the trailing `{e}` is genuine third-party text; the framing is
core-authored, unlocalizable prose. Bug K was classified as a prose-free-core
violation; F5 improved it (the real error now reaches the user instead of a
dropped hardcoded string) but did not fully eliminate core-authored English.
The F5 plan explicitly sanctioned "third-party pass-through", so this is a
noted tradeoff, not a defect: representing an arbitrary underlying error as
structured params is impractical. Accept; be aware `detail` will always be
English.

### M4. Test hygiene: several new tests leak their TempDir

`std::mem::forget(dir)` in the new planner/suggestion helpers
(`unidentifiable_*`, `source_overwrite_*`, `plan_two_same_output`,
`plan_one_with_existing_output`, `plan_multi`) leaks the temp directory so it is
never cleaned up. Each run accumulates orphaned dirs under the temp root.
Test-only, bounded per run, but unnecessary - the TempDir stays alive for the
duration of the synchronous `plan_batch` call regardless; dropping it after the
assertions would be fine. Not blocking.

### M5. F9: per-file diagnostics now print the file twice in dry-run human mode

`diagnostic-line-file` prepends `$file`; in `print_batch_human` per-file
diagnostics are already printed under the `dry-run-file` header, so the file
now appears both in the header and inline on each diagnostic line. Harmless
redundancy; the fix correctly restores attribution for *batch-level*
diagnostics (IgnoredFile, DuplicateIdentifier) printed in the separate loop,
which was the actual bug J. `SuggestionsCapped` has no file -> uses the
fileless `diagnostic-line`, correct.

### M6. mkvmerge-not-found JSON has a `mkvmerge_found: false` field the normal report omits

`config_only_json` adds `mkvmerge_found: false`; `batch_json` has no such key.
A JSON consumer reading `mkvmerge_found` gets `null`/undefined on the normal
path. Documented in the function's doc comment; schema asymmetry only. Accept.

---

## Cross-cutting interactions verified CORRECT

- **F5+F6 ordering in `plan_core` (the headline concern).** Sequence is
  `detect_source_overwrites` -> `finalize_plans` -> `detect_output_collisions`
  -> `finalize_plans`. A file that is a SourceOverwrite gets its error in pass
  1 and its plan nulled by the first `finalize_plans`, so
  `detect_output_collisions` (which reads `f.plan.as_ref()`) skips it entirely.
  **An on-disk `overwrite` collision can never resurrect / wrongly keep a plan
  that SourceOverwrite should have killed** - the plan is already `None` before
  collision detection runs. A file that is both a SourceOverwrite and a
  two-planned collision is reported as SourceOverwrite only (more specific);
  both are errors, both dropped - same outcome. The final `finalize_plans`
  drops all error plans (planned-twice always Error; on-disk Error). The
  input-set gathering runs before any finalize, so donors of resolution-failed
  (but render-succeeded) files are still protected. Correct.
- **Decision #1 (absent boolean = false):** matcher.rs 93-99 falls back to
  `PropValue::Bool(false)` only when `matchable_type(prop) == Boolean`;
  capability confirms the four vanity flags + `default_track`/`enabled_track`/
  `forced_track` are `Boolean`, and `type` is `String`. Present booleans still
  compare by real value; absent non-booleans still no-match. Matches amended
  spec 4.4. (Note: the code generalizes to all boolean matchables, incl.
  `enabled_track` whose Matroska default is true - harmless because mkvmerge
  always emits it, and the amended spec states the general rule.)
- **Decision #2 (empty list rejected):** validate.rs 298-319 emits
  `EmptyMatchList` (error) for empty `any`/`not`. Matches spec 4.3. Matcher's
  `!any.is_empty()` guard is now dead for validated input but harmless; the
  suggestion engine never synthesizes empty lists.
- **Decision #3 (two-planned always error; on_collision on-disk only):**
  detect_output_collisions 551-559 hard-codes `Severity::Error` for
  `planned_twice` regardless of policy; on-disk follows policy (error/warn-skip/
  info-overwrite) and skip explicitly nulls the plan (bug E). Matches amended
  spec 4.8. Tested across all four policy values.
- **Suggestion engine simulation vs the new post-passes:** narrowing a
  `tracks[ri].match_expr` cannot change output paths or assignment sources
  (donors are resolved by locator, independent of match; primary rules use the
  primary path), so SourceOverwrite/OutputCollision signatures are invariant
  under every candidate edit -> no spurious regression rejection and no false
  acceptance. `plan_core` (used for simulation) does not call `suggest`, so no
  recursion. `SuggestionsCapped` is added post-`suggest` in `plan_batch`, so it
  never pollutes the baseline/sim signatures. F7 no-clobber (`or_insert`)
  correctly makes a key-colliding candidate a no-op that the acceptance check
  then rejects; `not` stays additive. yaml_fragment via `yaml_serde` is valid
  round-trippable YAML for colon/brace values.
- **F1 dry-run superset-of-validate:** config-time `validate + provable_overlaps`
  run before the mkvmerge lookup and are surfaced on every path incl.
  mkvmerge-missing (JSON and human), exit code folds all severities. The
  mkvmerge-missing path returns 2 outright rather than folding - documented and
  reasonable (mkvmerge absence is a hard failure); diagnostics are still a
  superset even though the exit code is not.
- **F2 catalog:** `EmptyMatchList`, `UnidentifiableSource`, `SuggestionsCapped`
  all in `diag_codes!` with rustdoc, unique keys, ALL membership, and ftl
  messages; every ftl param (`$detail`, `$dropped`, `$version`, `$file`) matches
  its emitter. CLI catalog-completeness guard green. F9 `unknown-property-skew`
  now references only `$version` (the `$property` literal-leak is gone).
- **F8 walk_files:** uses `symlink_metadata` (symlink branch is live), resolves
  file targets via `metadata` and includes them under the link path, never
  recurses dir symlinks, skips broken symlinks silently. Correct.

## Other checks

- No dead code / unused functions (clippy `-D warnings` green would fail
  otherwise). The one `#[allow(clippy::too_many_arguments)]` on `suggest` is
  pre-existing and justified.
- No new warning-suppression, no swallowed errors, no bumped timeouts.
- No test asserts nothing - spot-checked the new i18n, report, planner,
  suggestions, and validate tests; all carry real assertions.
- Prose-free-core otherwise intact (no `println!` in core; diagnostics remain
  code+params); the only authored English is the `detail` framing in M3.
