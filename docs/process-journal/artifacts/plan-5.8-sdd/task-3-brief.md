### Task 3: D39 - catalog selector, allowed-param removal, coupled-comment sweep

**Files:**
- Modify: `locales/en/diagnostics.ftl:43`, `locales/de/diagnostics.ftl:50`
- Modify: `crates/muxsmith-core/src/planner.rs:440-448` and `:811-824`
- Modify: `crates/muxsmith-core/src/report/mod.rs` (InvalidPropertyValue rustdoc line)
- Modify: `crates/muxsmith-cli/tests/catalog_completeness.rs` (fixture_args entry :63-67, doc comments :39-47 and :387-395, leak-test assertions)
- Modify: `src/diagnosticFluentParams.ts:19-27` (doc comment only)

**Interfaces:**
- Consumes: nothing from other tasks (independent stream).
- Produces: `invalid-property-value` emissions with `property=language` carry `property` + `value` only (no `allowed`) - wire-format change per D39.

- [ ] **Step 1: Turn the leak test into the failing spec of the new rendering**

In `crates/muxsmith-cli/tests/catalog_completeness.rs`, in `invalid_changes_language_diagnostic_renders_without_placeholder_leak` (:397), extend the final assertions: keep the no-`{$allowed}`-leak assertion, add that the rendered en message contains `must be a valid ISO 639 or BCP-47 language code` and does NOT contain `Allowed values include`.

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p muxsmith-cli --test catalog_completeness invalid_changes_language -- --nocapture`
Expected: FAIL - current message renders "Allowed values include: a valid ISO 639/BCP-47 language code."

- [ ] **Step 3: Replace both catalog messages with the selector**

`locales/en/diagnostics.ftl:43`, replace the `invalid-property-value` line with:

```
invalid-property-value = { $property ->
    [language] Value "{ $value }" is not valid for property "language"; it must be a valid ISO 639 or BCP-47 language code.
   *[other] Value "{ $value }" is not valid for property "{ $property }". Allowed values include: { $allowed }.
}
```

`locales/de/diagnostics.ftl:50`, replace with:

```
invalid-property-value = { $property ->
    [language] Der Wert "{ $value }" ist für die Eigenschaft "language" nicht gültig; er muss ein gültiger Sprachcode nach ISO 639 oder BCP-47 sein.
   *[other] Der Wert "{ $value }" ist für die Eigenschaft "{ $property }" nicht gültig. Zulässige Werte sind unter anderem: { $allowed }.
}
```

(Both validated pre-merge against @fluent/bundle during design review; variant indentation follows the catalog's existing select style.)

- [ ] **Step 4: Remove the prose param from both emitters and sweep the comments**

In `crates/muxsmith-core/src/planner.rs`:
- `walk_exact_languages` (:440-448): delete the line `.with("allowed", "a valid ISO 639/BCP-47 language code"),` (:447).
- `resolve_changes` (:811-824): delete the same `.with("allowed", ...)` line (:822) AND replace the comment at :819-821 with:

```rust
                        // `invalid-property-value` selects on `$property` in
                        // the catalog: the [language] arm renders registry
                        // wording and takes no `allowed` param (D39). Only
                        // the closed-domain emitters (validate.rs) pass one.
```

In `crates/muxsmith-core/src/report/mod.rs`, update the `InvalidPropertyValue` rustdoc line inside `diag_codes!` to:

```rust
    /// An `exact` condition value lies outside a closed value domain: `type`/`codec_kind` are checked at config time, `language` at plan time (spec 4.4). `property`/`value` params carry the offender; closed-domain emitters also pass `allowed` (a hint sample), language emissions do not - the catalog's language arm renders registry wording (D39).
```

- [ ] **Step 5: Switch the fixture to the list arm and sweep the two test doc comments**

In `catalog_completeness.rs`:
- `fixture_args` entry (:63-67) becomes:

```rust
        DiagCode::InvalidPropertyValue => vec![
            ("property", "type"),
            ("value", "text"),
            ("allowed", "video, audio, subtitles"),
        ],
```

- Rewrite the stale halves of the two narrations (comment-coupling sweep, D39):
  - :43-47 ("The one known instance -- `resolve_changes` emitting ... without `allowed` ... is fixed and pinned by ..."): now describes the selector split - the fixture exercises the `*[other]` list arm; the `[language]` arm is pinned by the site-level leak test rendering the real emitter output.
  - :387-395 ("Before the fix that added `.with(\"allowed\", ...)` ..."): now states the inverse - since D39 both language emitters deliberately carry no `allowed`; the test pins that the `[language]` arm renders complete registry wording with no placeholder leak.

- [ ] **Step 6: Fix the diagnosticFluentParams.ts strictness comment (ROADMAP trigger, routed-items item 7)**

In `src/diagnosticFluentParams.ts`, replace the two doc-comment sentences (:25-26)

```
 * Mirrors the Rust side's `parse::<usize>()` strictness: rejects negative
 * numbers, floats, empty strings, and scientific notation.
```

with

```
 * Close to the Rust side's `parse::<usize>()` strictness: rejects negative
 * numbers, floats and empty strings, but unlike Rust it accepts spellings
 * Number() normalizes to a non-negative integer (e.g. "1e3" -> 1000) -
 * acceptable because the wire values are Rust usize serializations and
 * arrive as plain digit strings.
```

Code stays unchanged (decided at plan level: the params originate from Rust `usize` serialization, the guard is defensive only, and the frontend has no unit-test vehicle for a stricter regex until Plan 6's GUI test-harness block).

- [ ] **Step 7: Run the affected layers**

Run: `cargo test -p muxsmith-cli --test catalog_completeness && cargo test -p muxsmith-core && pnpm check:i18n`
Expected: PASS (id parity between en/de holds - both locales changed in sync). If any insta snapshot in `crates/muxsmith-cli/tests/snapshots/` carries the old `invalid-property-value` wording, review the diff deliberately and update it (`cargo insta review` or manual edit), then note the snapshot update in your report.

- [ ] **Step 8: Commit**

```bash
git add locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-core/src/planner.rs crates/muxsmith-core/src/report/mod.rs crates/muxsmith-cli/tests/catalog_completeness.rs src/diagnosticFluentParams.ts
git -c commit.gpgsign=false commit -m "fix: language diagnostics render locale-pure via catalog selector, allowed param off the wire (D39)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Execution and close

- Stream A = Tasks 1-2 (sequential, one worktree), Stream B = Task 3 (own worktree), concurrent. Merge A then B (or completion order), nine-part gate after each merge; textual conflicts go back to the owning stream's implementer per doctrine.
- Expected merge friction: `report/mod.rs` (both streams touch the diag_codes! block ~20 lines apart), `locales/*/diagnostics.ftl` (lines 7/14 vs 43/50), `catalog_completeness.rs` (fixture match vs fixture entry + comments) - distinct hunks, auto-merge expected.
- Whole-branch review after both streams merge; then plan close per doctrine (funnel, salvage `.superpowers/sdd/plan-5.8/` including the design-phase four-eyes artifacts, ROADMAP resolve: mark the zero-rule-keep and mixed-language pre-1.0 gates DONE, annotate the item-7 trigger as consumed by this plan, journal entry, HANDOFF supersede + snapshot).
