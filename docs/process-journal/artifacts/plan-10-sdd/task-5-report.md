# Task 5 report - Plan 10 (W2: the comment line-citation sweep)

**Status: DONE_WITH_CONCERNS.** Every step executed, the full gate green
foreground, both absence checks empty on the end state with their fires, commit
`1a23283` on `master`. The concerns are three named judgment calls plus one
controller surface (a citation that was already stale in the commit that
introduced it), none of which blocks the task.

- Commit: `1a23283ab0bed61e80c81a7229d98fe15fa4c16e`
- Corpus: **20 lines / 13 files (A)**, **4 lines / 4 files (B)**, **union 24
  lines across 16 files**, in **21 comments**. Identical to the plan's authoring
  measurement; set-equal to the Files list, checked by `diff`, not by eye.
- Sites rewritten: **21 comments** covering all 24 matched lines.

---

## Step 1: the corpus, re-measured before any edit

### Expression A, filename citations

```bash
git ls-files -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py' \
  | xargs grep -nE '[A-Za-z0-9_./-]+\.(rs|ts|vue|mjs|js|py|toml|ftl|json|yaml|md):[0-9]+'
```

Full output, pasted:

```
crates/muxsmith-cli/tests/run_live.rs:273:/// D40 regression (report/json.rs:44, whole-branch-verdict.md Finding 1):
crates/muxsmith-cli/tests/run_live.rs:274:/// the README's passthrough recipe (README.md:71-78) inlined verbatim --
crates/muxsmith-cli/tests/run_live.rs:315:    // README.md:71-78, verbatim.
crates/muxsmith-cli/tests/run_live.rs:326:    // (pre-fix: panics at report/json.rs:44 building the `Set` plan value,
crates/muxsmith-cli/tests/run_live.rs:327:    // per README.md:91 "every command takes --json").
crates/muxsmith-cli/tests/run_live.rs:361:    // same `Set` value, run.rs:274-275).
crates/muxsmith-core/src/identify.rs:557:    // parse_attachment's contract (identify.rs:224-225): required fields
crates/muxsmith-core/src/planner.rs:2226:    // D40 (report/json.rs:44 panic fix, whole-branch-verdict.md Finding 1):
crates/muxsmith-core/src/report/json.rs:161:/// without ever panicking (D40, report/json.rs:44's fix): every `Plan`
crates/muxsmith-core/tests/suggestions.rs:1015:// `not` entry's `exact` (delta_for's two exact-bearing arms, planner.rs:1812,
crates/muxsmith-core/tests/suggestions.rs:1035:// (matcher.rs:202-212 has no (Str, Bool) arm) and the rule would match nothing.
crates/muxsmith-core/tests/suggestions.rs:1071:// (generated.rs:42); a `Scalar::Str("1")` would fall through `scalar_eq` exactly
e2e/smoke.spec.ts:1434:// (the `RunHistory.vue:168-173` house precedent, not a hand-rolled
src-tauri/src/lib.rs:730:        // A load failure (missing file, the same fixture shape as lib.rs:557-563):
src/components/ResolutionTable.vue:32:  // `Assignment::track_kind` doc comment (`planner.rs:53`) states
src/editor/fieldSpec.ts:65:  // checked -> Some(true), unchecked -> absent (validate.rs:466-472 rejects
src/editor/widgets/FieldWidgetDispatcher.vue:6:// shape at `src/jobRowState.ts:44-55`, which relies on plain control-flow
src/editor/widgets/OptionalFlagWidget.vue:4:// (validate.rs:466-472 rejects `Some(false)`; model.rs's own doc says "the
src/editor/widgets/PropertyMapWidget.vue:48:// for (`registries.ts:185-189`).
src/views/EditorView.vue:83:// select">` with `:aria-current`, the `RunHistory.vue:168-173` house
```

Line count `| wc -l` -> `20`. File count, same expression with `grep -l | wc -l`
-> `13`, and the 13 names pasted in full were the 13 distinct files above. **A =
20 lines / 13 files.**

### Expression B, bare line spans (per file, prefix stripped before the second filter)

```bash
git ls-files -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py' \
  | while read -r f; do grep -nE '(^|[[:space:]`,(])[:][0-9]+' "$f" | sed "s|^|$f:|"; done
```

Full output, pasted:

```
crates/muxsmith-core/tests/profile_save.rs:95:/// defaulted fields (design `:1517-1535`), that the published JSON
crates/muxsmith-core/tests/suggestions.rs:1016:// :1817). Returns None if the key is absent, which is itself a guard failure.
crates/muxsmith-core/tests/ts_export.rs:10://! `:592-611`). Both artifacts share one destination, `TS_RS_EXPORT_DIR`
src/editor/registries.ts:12: * straight from the 43-row table in D45 (design `:889-936`); do not
```

**B = 4 lines / 4 files.**

### Union and set-equality with the Files list

Union = 16 files (`suggestions.rs` is the one file both expressions hit): 13 + 4
- 1. Checked mechanically rather than by eye - the two file lists sorted into
one set and `diff`ed against the plan's sixteen-entry Files list:

```
16
--- set-diff vs plan Files list ---
SET-EQUAL
```

**The re-measurement equals the plan's authoring numbers exactly** (20/13, 4/4,
union 24/16). Fourth consecutive unchanged measurement, counting the three
during Tasks 1-3. No divergence to report, so the Files list stands unaltered
and no NEEDS_CONTEXT arises from Step 1.

**Comment containment, checked per hit before editing:** all 24 matched lines sit
inside `//`, `///`, `//!` or `/** */` comments (verified by opening each site,
not inferred). No hit sits on a code line; no hit sits in a file off the list.
Neither NEEDS_CONTEXT trigger fired.

---

## Step 2 + 3: the per-site table

24 matched lines, 21 comments, 16 files. "Rewrite unit is the comment" is why
line and comment counts differ: `run_live.rs:273/274` and `:326/327` are one
comment each, and `suggestions.rs:1015/1016` is one comment split across an
A-hit and a B-hit.

| # | File | Before (citation only) | After | Symbol named, and how it was verified |
|---|---|---|---|---|
| 1 | `crates/muxsmith-cli/tests/run_live.rs` (:273-274, doc) | ``D40 regression (report/json.rs:44, whole-branch-verdict.md Finding 1): the README's passthrough recipe (README.md:71-78) inlined verbatim`` | ``D40 regression (the `batch_document` panic, whole-branch-verdict.md Finding 1): the README's passthrough recipe (the YAML block under its "Pure passthrough: a profile with zero rules" heading) inlined verbatim`` | `batch_document`: opened `report/json.rs`, `pub fn batch_document(` is at `:44` at HEAD, so `:44` denotes that function - and the convention entry's own worked example prescribes this exact substitution. README anchor: `grep -n 'Pure passthrough'` -> `74:### Pure passthrough: a profile with zero rules` in the shipped file |
| 2 | `run_live.rs` (:315) | ``README.md:71-78, verbatim.`` | ``The YAML block under README.md's "Pure passthrough: a profile with zero rules" heading, verbatim.`` | Same heading, measured as above. The fence is `78-85` and its content `79-84` post-Task-4 (`sed -n '74,88p'`, pasted below in the README section); the span is deleted rather than updated, per constraint 2 |
| 3 | `run_live.rs` (:326-327) | ``(pre-fix: panics at report/json.rs:44 building the `Set` plan value, per README.md:91 "every command takes --json")`` | ``(pre-fix: panicked in `batch_document` while building the `Set` plan value; `--json` per README.md's "What you get" section, "Scriptable everything" bullet)`` | `batch_document` as in row 1. The QUOTATION is dropped, not re-anchored (cross-task constraint 1): `grep -c 'every command takes' README.md` -> `0`, fired control `grep -c 'Scriptable everything' README.md` -> `1`. Anchors: `grep -n 'What you get'` -> `89:## ✨ What you get`; the bullet at `:98` reads "**Scriptable everything**: `validate`, `dry-run`, `identify` and `run` each take `--json`", which is what the assertion under the comment (a `dry-run --json` exit-0 check) actually rests on |
| 4 | `run_live.rs` (:361) | ``but `run.rs`'s unconditional run-document build then panicked on the same `Set` value, run.rs:274-275`` | ``but the unconditional `run_document(batch_document(..))` build in `crates/muxsmith-cli/src/commands/run.rs`'s `run` then panicked on the same `Set` value`` | Both `run.rs` files opened. `src-tauri/src/run.rs` has no unconditional post-mux run-document build (its `run_document` calls sit in `plan_run`'s early-return arms and in `start_run`). `crates/muxsmith-cli/src/commands/run.rs` has exactly one, in `run` at `:219`, unconditional and after `run_batch`. `commands/run.rs:274-275` at HEAD is inside `create_logger`'s `MUXSMITH_RUNS_ROOT` test-seam comment - unrelated, as the ROADMAP already recorded. Ambiguity resolved by naming both the symbol and the full path |
| 5 | `crates/muxsmith-core/src/identify.rs` (:557) | ``parse_attachment's contract (identify.rs:224-225)`` | ``parse_attachment's contract (its own doc comment)`` | Opened `identify.rs`: `parse_attachment`'s doc comment is at `:228-229` today ("Parses one `-J` attachment entry. Required fields (`id`, `file_name`, `size`) missing or wrong-typed drop the entry"), i.e. exactly the content the citation meant, shifted four lines. The symbol is already named at the head of the sentence, so the citation collapses to naming which part of it - the doc comment |
| 6 | `crates/muxsmith-core/src/planner.rs` (:2226) | ``D40 (report/json.rs:44 panic fix, ...)`` | ``D40 (the `batch_document` panic fix, ...)`` | Same target as row 1, same verification |
| 7 | `crates/muxsmith-core/src/report/json.rs` (:161) | ``(D40, report/json.rs:44's fix)`` | ``(D40, `batch_document`'s panic fix)`` | Same-file citation. `:44` is `batch_document`'s own signature line, and `plan_value`'s doc already opens "Serializes `plan` for [`batch_document`]'s per-file `\"plan\"` field", so the symbol is confirmed by the sentence it sits in |
| 8 | `crates/muxsmith-core/tests/suggestions.rs` (:1015-1016) | ``(delta_for's two exact-bearing arms, planner.rs:1812, :1817)`` | ``(delta_for's two exact-bearing arms in planner.rs, the `AddExact` and `AddNotExact` ones)`` | Opened `planner.rs`: `fn delta_for` begins at `:1820`, so **neither cited line is inside it** - naming the symbol from the cited LINE would have named the wrong thing. Its two exact-bearing arms are `StructuredEdit::AddExact` at `:1823` (sets `m.exact`) and `StructuredEdit::AddNotExact` at `:1828` (sets `m.not[0].exact`), matching what `spliced_scalar` beneath the comment actually reads. Those are also the two lines the ROADMAP's stale-citation entry records for HEAD |
| 9 | `suggestions.rs` (:1035) | ``(matcher.rs:202-212 has no (Str, Bool) arm)`` | ``(`scalar_eq` in matcher.rs has no (Str, Bool) arm)`` | Opened `matcher.rs`: `fn scalar_eq` at `:202`, arms `:203-210`, closing brace `:212`. Its six arms are (Str,Str), (Bool,Bool), (Int,Int), (Int,Float), (Float,Float), (Float,Int) plus `_ => false` - no (Str, Bool) arm, so the claim holds against the symbol |
| 10 | `suggestions.rs` (:1071) | ``(generated.rs:42)`` | ``(its `MATCHABLE_PROPERTIES` entry in capability/generated.rs)`` | Opened `crates/muxsmith-core/src/capability/generated.rs`: `grep -n '("id", '` -> `42:    ("id", PropType::Integer),`, inside `pub static MATCHABLE_PROPERTIES`. Path widened to `capability/generated.rs`; the basename is already unique among tracked files (`git ls-files \| grep generated` -> one hit) |
| 11 | `crates/muxsmith-core/tests/profile_save.rs` (:95, B) | ``(design `:1517-1535`)`` | ``(design D48)`` | The plan's prescribed mapping. The comment already opens "D48 guard 2: schema-default honesty", so D48 is the identifier the comment itself supplies; nothing invented |
| 12 | `crates/muxsmith-core/tests/ts_export.rs` (:10, B) | ``(D44 `:592-611`)`` | ``(D44)`` | Prescribed mapping; the span drops and the D-entry the comment already carries survives |
| 13 | `e2e/smoke.spec.ts` (:1434) | ``(the `RunHistory.vue:168-173` house precedent, ...)`` | ``(the `jobs-history-run` button in `RunHistory.vue` is the house precedent, ...)`` | Opened `src/components/RunHistory.vue`: `:168-174` is the `<button type="button" data-testid="jobs-history-run" ... :aria-current="...">` inside `jobs-history-list`. The citing comment is about `:aria-current` on a native button rather than a hand-rolled `<tr>`, and that button carries `:aria-current` at `:172` - the precedent is the element, named by its stable `data-testid` |
| 14 | `src-tauri/src/lib.rs` (:730) | ``the same fixture shape as lib.rs:557-563`` | ``the same fixture shape as validate_profile_body_reports_load_failure_with_no_mkvmerge_key`` | Verified against git, not against HEAD, because the citation is stale at HEAD (`:557-563` is inside the Tauri `invoke_handler` list). At the PARENT of the introducing commit `997666a`, `lib.rs:557` is `fn validate_profile_body_reports_load_failure_with_no_mkvmerge_key()` and `:558-559` its `tempdir()` + `validate_profile_body(&dir.path().join("missing.yaml"))` - the same fixture shape as the `let missing = dir.path().join("missing.yaml"); // never written` line the comment sits above. See "For the controller" |
| 15 | `src/components/ResolutionTable.vue` (:32) | ``` `Assignment::track_kind` doc comment (`planner.rs:53`) ``` | ``` `Assignment::track_kind` doc comment (`planner.rs`) ``` | The symbol is already named. Opened `planner.rs`: `:53` is `/// `None` exactly when `track_id` is `None`.` and `:54` is `pub track_kind: Option<String>,` - the citation is accurate today, and only the number drops. Naming the file stays wanted |
| 16 | `src/editor/fieldSpec.ts` (:65) | ``(validate.rs:466-472 rejects Some(false); not a tri-state)`` | ``(`validate_locator` in profile/validate.rs rejects Some(false) for `match_to_source`; not a tri-state)`` | `validate.rs` is AMBIGUOUS in the repo (`crates/muxsmith-cli/src/commands/validate.rs` and `crates/muxsmith-core/src/profile/validate.rs`), so the surviving path is disambiguated. Opened the core one: the only `Some(false)` rejection in the file is `:472 if locator.match_to_source == Some(false) {`, inside `fn validate_locator` (`:454`). The widget it annotates is `optionalFlag`, whose sole registry user is `locatorFields.match_to_source` (`registries.ts:177`) - so the property name is measured, not guessed |
| 17 | `src/editor/registries.ts` (:12, B) | ``in D45 (design `:889-936`); do not re-derive them here.`` | ``in D45; do not re-derive them here.`` | Prescribed mapping's surviving identifier is `D45`, which the same sentence already carries ("the 43-row table in D45"). See judgment call 1 |
| 18 | `src/editor/widgets/FieldWidgetDispatcher.vue` (:6) | ``the existing house shape at `src/jobRowState.ts:44-55``` | ``the existing house shape in `jobStateKey` (`src/jobRowState.ts`)`` | Opened `src/jobRowState.ts`: `export function jobStateKey` at `:44`, its `switch (state)` with four `case` arms and no `default`, closing at `:55` - exactly the "plain control-flow exhaustiveness with no such arm" the comment contrasts against. Citation accurate today; the number drops |
| 19 | `src/editor/widgets/OptionalFlagWidget.vue` (:4) | ``(validate.rs:466-472 rejects `Some(false)`; model.rs's own doc says ...)`` | ``(`validate_locator` in profile/validate.rs rejects `Some(false)`; model.rs's own doc says ...)`` | Same target as row 16. `model.rs` is left as it stands: it is unambiguous among tracked files (one `model.rs`), it carries no line number, and its quoted doc string "the only valid value is `true`" was verified present on `Locator::match_to_source` (`profile/model.rs:363-364`) |
| 20 | `src/editor/widgets/PropertyMapWidget.vue` (:48) | ``` for (`registries.ts:185-189`) ``` | ``` for (`attachmentRuleFields` in `registries.ts`) ``` | Stale at HEAD: `:185-189` is the tail of `locatorFields` and the head of `attachmentsFields`. The comment is about the fields `editor-attachment-rule-add`/`-drop` caption; `grep -n 'editor-attachment-rule'` -> `:215`, `:219`, `:222`, all inside `export const attachmentRuleFields` (`:213-223`). Symbol named from what the comment MEANS |
| 21 | `src/views/EditorView.vue` (:83) | ``the `RunHistory.vue:168-173` house precedent`` | ``the `jobs-history-run` button in `RunHistory.vue` being the house precedent`` | Same target and verification as row 13 |

### Step 3, scope boundary

No file under `docs/` was touched: `git diff --stat` lists sixteen files, none
of them under `docs/`. Expression B's three design-document citations were
swept, not exempted, because the convention separates by the artifact DOING the
citing; all three live in source comments.

---

## The three README-citing sites, and the anchors used

All three are in `crates/muxsmith-cli/tests/run_live.rs`, and all three were
read against `README.md` **as Task 4 committed it** (working tree at
`44f1c8e`, unchanged from HEAD at the time of reading).

| Site | Old anchor | New anchor | Why |
|---|---|---|---|
| `:274` (doc comment) | `README.md:71-78` | the YAML block under the "Pure passthrough: a profile with zero rules" heading | The heading is stable and names the recipe. Post-Task-4 the fence is `78-85` and its content `79-84`; the pre-task number `71-78` was the FENCE span, so the two numbers in circulation were never the same unit. Constraint 2 rules the span deleted either way |
| `:315` (inline, above the literal) | `README.md:71-78` | same heading | Same. The literal it annotates is unchanged (the test writes the recipe byte-for-byte; `cargo test` green, see below) |
| `:327` (inline, in the `dry-run --json` block) | `README.md:91 "every command takes --json"` | README.md's "What you get" section, "Scriptable everything" bullet | **The quotation is dropped, not re-anchored** (constraint 1). Measured: `grep -c 'every command takes' README.md` -> `0`; fired control on a present phrase -> `1`. Line 91 today is the "A real dry-run" bullet. The bullet at `:98` is the claim that actually carries this test: "`validate`, `dry-run`, `identify` and `run` each take `--json`" |

The `:327` rewrite does NOT change what the comment asserts about the code under
it. The assertion below it is unchanged (`dry-run --json` exits 0 and prints one
valid JSON document); only the justification for exercising `--json` moves from
a deleted, false README sentence to the shipped, true one. No NEEDS_CONTEXT.

Supporting measurement, pasted (`sed -n '74,88p' README.md`, numbered):

```
     1	### Pure passthrough: a profile with zero rules
     2	
     3	A profile whose `tracks` block is `{ unmatched: keep, rules: [] }` is a legal pure-passthrough remux: ...
     4	
     5	```yaml
     6	profile_version: 1
     7	input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
     8	tracks:
     9	  unmatched: keep
    10	  rules: []
    11	title: { template: 'S{season}E{episode}' }
    12	```
```

(heading at README line 74, fence 78-85, content 79-84).

---

## Step 4: the two absence checks, with their fires

Both checks are the Step-1 expressions re-run unchanged on the end state, and
**each command's own Step-1 run on the pre-state IS its fire** - the same
process, same shell, same pattern, differing only in the tree.

### Check A

- **Fire (pre-state):** the Step-1 run above, **20 lines across 13 files**. A
  malformed pattern could not have produced those 20 lines.
- **End state:**

```
=== ABSENCE CHECK A (final end state) ===
[A produced no lines]
```

  (the grep pipeline emitted nothing; `xargs` returned 123, its "every command
  exited 1" code, i.e. no batch matched)

### Check B

- **Fire (pre-state):** the Step-1 run above, **4 lines across 4 files**. This
  is the check where the fire carries real weight: the naive one-pipeline form
  of B matches its own `file:line:` prefix and returns empty against exactly
  this four-hit tree. The prescribed per-file form fired.
- **End state:**

```
=== ABSENCE CHECK B (final end state) ===
[B produced no lines]
```

**Reachable green state, argued member by member:** every one of the 24 returned
lines sits inside one of the 21 comments this task rewrote; the rewrite unit was
the comment rather than the matched line, which is what makes
`suggestions.rs:1016`'s continuation disappear with `:1015`; and the Files list
is exhaustive and set-equal to the union. No member survives.

---

## Full gate, foreground, in BUILDING.md's order

Eleven parts as the file enumerates them (6 Rust, 4 frontend, 1
house-knowledge). Exit codes captured **directly**, not through
`${PIPESTATUS[0]}` (empty in zsh) and not from a pipeline's tail.

| # | Command | Exit | Evidence |
|---|---|---|---|
| 1 | `cargo fmt --all --check` | 0 | no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s`` |
| 3 | `cargo test --workspace` | 0 | `grep -c '^test result: ok\.'` over the run's log -> `39`; `0 failed` on every one |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 | ``Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files`` |
| 5 | `cargo deny check` | 0 | `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 | ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s`` |
| 7 | `pnpm lint` | 0 | `$ eslint .` and no findings |
| 8 | `pnpm build` | 0 | `✓ built in 153ms` |
| 9 | `pnpm check:i18n` | 0 | `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| 10 | `pnpm test:e2e` | 0 | `68 passed (3.0s)` |
| 11 | `python3 scripts/ledger-lint.py` | 0 | `ledger-lint: 541 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

**No `FAIL BUILDING.md: ...` line appeared** (part 11's only output is the ok
line above), so nothing wrote to that file outside its owner's Files list.

Working tree after the gate: exactly the sixteen `M` entries, nothing else -
no generated artifact, no `ts-rs` export, no snapshot churn.

**The gate's green run is meaningful evidence here** because the diff is comment
text only. As a targeted cross-check, the tests whose comments this task
rewrote all RAN (not skipped) and passed, pasted from the part-3 log:

```
test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok
test identify::tests::attachment_with_wrong_typed_id_is_dropped ... ok
test planner::tests::title_action_variants_serialize_to_the_expected_kind_tagged_shape ... ok
test apply_splices_the_simulated_scalar_for_a_bool_property ... ok
test apply_splices_the_simulated_scalar_for_an_int_property ... ok
test tests::load_profile_body_matches_validate_profile_diagnostics_and_adds_the_model ... ok
```

The first is `have_mkvmerge()`-gated and did NOT skip: `grep -c 'MKVMERGE'` over
the test log -> `0` skip markers, and `mkvmerge --version` on this machine ->
`mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit` at
`/home/linuxbrew/.linuxbrew/bin/mkvmerge`.

## `git diff --stat` (pre-commit)

```
 crates/muxsmith-cli/tests/run_live.rs        | 18 +++++++++++-------
 crates/muxsmith-core/src/identify.rs         |  2 +-
 crates/muxsmith-core/src/planner.rs          |  2 +-
 crates/muxsmith-core/src/report/json.rs      |  2 +-
 crates/muxsmith-core/tests/profile_save.rs   |  2 +-
 crates/muxsmith-core/tests/suggestions.rs    | 13 ++++++++-----
 crates/muxsmith-core/tests/ts_export.rs      |  4 ++--
 e2e/smoke.spec.ts                            |  7 ++++---
 src-tauri/src/lib.rs                         |  3 ++-
 src/components/ResolutionTable.vue           |  2 +-
 src/editor/fieldSpec.ts                      |  5 +++--
 src/editor/registries.ts                     |  3 +--
 src/editor/widgets/FieldWidgetDispatcher.vue |  8 ++++----
 src/editor/widgets/OptionalFlagWidget.vue    |  9 +++++----
 src/editor/widgets/PropertyMapWidget.vue     |  2 +-
 src/views/EditorView.vue                     |  9 +++++----
 16 files changed, 51 insertions(+), 40 deletions(-)
```

Exactly the sixteen files, set-equal to the Files list and to the `git add`
pathspec set. Nothing else.

---

## Divergences and judgment calls, each named

1. **`registries.ts`: the prescribed mapping collapses, and I dropped the
   parenthetical rather than writing a tautology.** Step 2's bare-span bullet
   gives three mappings; two are direct substitutions (`design :1517-1535` ->
   `design D48`, `D44 :592-611` -> `D44`), but the third's surviving identifier
   is `D45`, and the sentence already reads "straight from the 43-row table in
   D45 (design `:889-936`)". A literal substitution yields "in D45 (D45)".
   I wrote "straight from the 43-row table in D45; do not re-derive them here."
   The plan's own gloss supports this - it lists the third target as bare `D45`
   where it lists the first as `design D48`, and notes the surrounding prose
   already names the 43-row table. Reviewer can rule (concern 1).
2. **Comment blocks were re-wrapped where the substitution changed line
   lengths.** Measured from the commit's stat: six files show a `2 +-` swap
   (one line out, one in) and each of those carries exactly one site, so six
   comments changed by a pure token substitution -
   `identify.rs`, `planner.rs`, `report/json.rs`, `profile_save.rs`,
   `ResolutionTable.vue`, `PropertyMapWidget.vue`. The other fifteen comments
   re-wrapped so no line is left ragged. Still comment text only - no code,
   markup or test logic line appears in the diff, which the `git diff` output
   above shows directly.
3. **`identify.rs` and `ResolutionTable.vue`: the symbol was already in the
   sentence, so the citation reduces rather than gaining a new name.**
   `parse_attachment's contract (identify.rs:224-225)` became
   `parse_attachment's contract (its own doc comment)` - the cited lines ARE
   that doc comment, and repeating the symbol inside its own parenthetical would
   be noise. `(`planner.rs:53`)` became `(`planner.rs`)` for the same reason:
   `Assignment::track_kind` is named two words earlier. Naming the file stays
   normal and wanted per the ruling.
4. **Paths were widened where the surviving basename was ambiguous, and left
   alone where it was not.** `validate.rs` -> `profile/validate.rs` (two tracked
   `validate.rs`); `run.rs` -> `crates/muxsmith-cli/src/commands/run.rs` (two
   tracked `run.rs`); `generated.rs` -> `capability/generated.rs` (unambiguous
   already, widened for readability). `model.rs`, `matcher.rs`, `planner.rs`,
   `RunHistory.vue`, `registries.ts`, `jobRowState.ts` were left as written -
   each is unique among tracked files.

Nothing was re-argued: the corpus was measured with the two prescribed
expressions, expression B's four sites were treated as IN, historical citations
lost their numbers with no live-versus-historical classification made, and no
`docs/` file and no code line was touched.

---

## Numbered concerns a reviewer can rule on yes/no

1. **`registries.ts`: dropping the parenthetical instead of substituting.**
   Is "straight from the 43-row table in D45; do not re-derive them here."
   the correct execution of the mapping `design \`:889-936\`` -> `D45`, or
   should the parenthetical have survived as `(design D45)`? (judgment call 1)
2. **`run_live.rs:315`'s anchor names the heading, not the fence.** The comment
   annotates a byte-for-byte literal of the YAML. "The YAML block under
   README.md's 'Pure passthrough: a profile with zero rules' heading" is durable
   but slightly less precise than a fence pointer would be. Acceptable?
3. **`identify.rs`'s "(its own doc comment)" names a location, not a symbol.**
   The symbol `parse_attachment` is already the sentence's subject, so the
   parenthetical says which PART of it. Is that within the handle, or should it
   have read `(parse_attachment's doc comment)` in full?
4. **`run_live.rs:361` names a full repo path inside a comment.**
   `crates/muxsmith-cli/src/commands/run.rs` is verbose but is the only form
   that disambiguates the two `run.rs` files at a glance. Preferred over the
   shorter `commands/run.rs`?

---

## What I surface for the controller

1. **A citation that was already stale in the commit that introduced it.**
   `src-tauri/src/lib.rs:730`'s `lib.rs:557-563` pointed at
   `validate_profile_body_reports_load_failure_with_no_mkvmerge_key` in the
   PARENT of `997666a`; in `997666a` itself - the commit that added the citing
   comment - `:557-563` is already `pub fn run()`'s Tauri builder, because the
   author's own diff moved the target down by 137 lines before the comment was
   committed: the test's `fn` line is at `:557` in the parent and at `:694` in
   the commit itself, both grepped. Measured, not inferred: `git log -S` located
   the commit, and both `997666a:src-tauri/src/lib.rs` and
   `997666a^:src-tauri/src/lib.rs` were read at that span. This is the sharpest available evidence for the ruling's own
   ground - a line citation that never once pointed at its target in a committed
   tree - and it is ledger-harvest material beyond this sweep.
2. **Which citations were measurably stale at HEAD before the sweep**, since the
   ROADMAP disposition will want the fuller picture behind "the class". Nine of
   the twenty-four, enumerated rather than given as a fraction:
   `run_live.rs:274`, `:315`, `:327` (all three README spans, staleified by
   Task 4 inside this very package), `run_live.rs:361` (`run.rs:274-275` is
   `create_logger`'s test seam), `identify.rs:557` (off by four),
   `suggestions.rs:1015` and `:1016` (`delta_for` begins at `:1820`, arms at
   `:1823`/`:1828`), `lib.rs:730` (item 1 above), and
   `PropertyMapWidget.vue:48` (`registries.ts:185-189` is the tail of
   `locatorFields`, while `attachmentRuleFields` is `:213-223`). A tenth is
   partially stale at two sites: `validate.rs:466-472` (`fieldSpec.ts:65`,
   `OptionalFlagWidget.vue:4`) - the `Some(false)` check now begins at `:472`,
   the span's last line, with its body past the end. Three more
   (`design :1517-1535`, `:592-611`, `design :889-936`) could not be checked
   against the tree at all, since they point into design documents, which is
   itself the argument the plan makes for sweeping them.
3. **The corpus did not move.** Fourth consecutive unchanged measurement
   (20/13, 4/4, union 24/16). The ROADMAP's "Docs accuracy" entry records 17;
   the disposition's corrected figure of 24 lines across 16 files under two
   expressions is confirmed by this run.
4. **The convention's occurrence record could carry the union.** The Tier-2
   entry's single occurrence records "Corpus: 20 comment lines across 13 files",
   which is expression A only. The union that was actually swept is 24 across
   16. Not this task's to edit (no task edits house-knowledge YAML); flagged for
   the harvest.

---

## Commit

```
1a23283ab0bed61e80c81a7229d98fe15fa4c16e
comments: locate code by symbol, never by line number (owner ruling, whole corpus)

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

- Staged explicitly by the sixteen fenced pathspecs, never `git add -A`.
- Unsigned as SI-4 requires: `git log -1 --format='%G?'` -> `N`.
- Exactly one trailer: `git log -1 --format='%(trailers)'` -> the single
  `Co-Authored-By` line above. No `Claude-Session` line, no context-window
  suffix.
- No push (the plan close owns the single push).

`git show --stat`:

```
 crates/muxsmith-cli/tests/run_live.rs        | 18 +++++++++++-------
 crates/muxsmith-core/src/identify.rs         |  2 +-
 crates/muxsmith-core/src/planner.rs          |  2 +-
 crates/muxsmith-core/src/report/json.rs      |  2 +-
 crates/muxsmith-core/tests/profile_save.rs   |  2 +-
 crates/muxsmith-core/tests/suggestions.rs    | 13 ++++++++-----
 crates/muxsmith-core/tests/ts_export.rs      |  4 ++--
 e2e/smoke.spec.ts                            |  7 ++++---
 src-tauri/src/lib.rs                         |  3 ++-
 src/components/ResolutionTable.vue           |  2 +-
 src/editor/fieldSpec.ts                      |  5 +++--
 src/editor/registries.ts                     |  3 +--
 src/editor/widgets/FieldWidgetDispatcher.vue |  8 ++++----
 src/editor/widgets/OptionalFlagWidget.vue    |  9 +++++----
 src/editor/widgets/PropertyMapWidget.vue     |  2 +-
 src/views/EditorView.vue                     |  9 +++++----
 16 files changed, 51 insertions(+), 40 deletions(-)
```

Working tree after the commit: `git status --porcelain` prints nothing.
