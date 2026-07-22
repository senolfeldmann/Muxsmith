# Task 6 (D60) verdict - independent SDD review

**VERDICT: APPROVED**

Commit `2422a58` on `plan7-b` (parent `3fab82f` = approved Task 5). D60:
`resolvedTrackLabel` punctuation moved into the new `batch-resolved-track`
Fluent key (en+de identical), `ResolutionTable.vue` re-pointed via `useFluent`,
the `"-"` placeholder and the untranslated `track_kind` kept code-side, comment
block updated. 3-file diff (+18/-2), full nine-part-relevant GUI gate green.

The implementer shipped DONE_WITH_CONCERNS with one flagged deviation (the null
guard extended past the plan's literal). That deviation is the central
adjudication below; I rule it a **grant-covered mechanical repair of a
non-compiling plan line**, correctly executed and disclosed - not a skipped
NEEDS_CONTEXT. No fixes required.

Every load-bearing claim was re-derived from the tree, not borrowed from the
commit message or the implementer's self-report. The plan-literal type error was
reproduced by running `vue-tsc` against it; absence checks were fire-verified
against a control; all probe state restored byte-identically.

---

## Findings

### 1. Catalog keys - COMPLETE, en+de identical, per plan - PASS
- `batch-resolved-track = { $id } ({ $kind })` added to both `locales/en/gui-batch.ftl:50`
  and `locales/de/gui-batch.ftl:46`, byte-identical value (the design's point: catalog
  control, not divergence; D60, plan Step 1). Diff numstat: `+1/-0` each catalog.
- Placeables match across locales (`{ $id }`, `{ $kind }`), so de and en render the
  same composition. `pnpm check:i18n` id-parity passes; no selector/plural involved.
- gui-batch reaches its final **28/28** ids (en and de), the count the plan ties to
  Task 6 (`grep -cE "^[a-z][a-zA-Z0-9-]* =" -> 28` each; pattern control-verified to
  match `batch-resolved-track`). No other id touched.

### 2. Component re-point - PASS
- `import { useFluent } from "fluent-vue";` added; `const fluent = useFluent();` added
  (the component held none). `$t`-in-template sites elsewhere in the file are the
  component-global `$t`; the function needs the composition-API handle, correct.
- The `"-"` stays a code-side literal; `track_kind` stays an untranslated mkvmerge
  passthrough (no `$t` wrapping it) - both recorded prior decisions respected exactly,
  per D60 and plan Step 2.
- Comment block updated honestly: it now states the `id (kind)` parenthesization is
  catalog-controlled via `batch-resolved-track` (D60) while the `"-"` and the
  `track_kind` value stay code-side. Accurate, matches the shipped code, no drift
  from the pre-existing spec-7 / spec-8.4 rationale it extends.

### 3. Scope and commit discipline - PASS
- Exactly the 3 files the plan enumerates changed; `e2e/smoke.spec.ts` correctly NOT
  touched - the plan gates that edit on "an assertion renders the old literal
  composition", and no e2e assertion pins the composition text (see Finding 5), so the
  conditional did not fire. Nothing extra.
- Commit message matches plan Step 4 verbatim; `Co-Authored-By: Claude Fable 5` trailer
  present; unsigned. No `git add -A` residue (only the 3 files in the tree).

### 4. Gate - GREEN (run foreground, this reviewer, exit codes captured)
- `pnpm lint` -> exit 0.
- `pnpm check:i18n` -> exit 0 ("ok, 208 catalog ids, 17 unused warning(s)"; the unused
  set is the pre-existing IpcError-code catalog ids, non-fatal, unrelated to this task).
- `pnpm build` (`vue-tsc --noEmit && vite build`) -> exit 0. This is the decisive gate:
  the implementer's actual code type-checks; the plan's literal would not (Finding 6).
- `pnpm test:e2e` -> exit 0, **32 passed** (catalogs 1 + editor-tooltips 1 + smoke 30).

### 5. e2e coverage of the label - render path exercised, composition not text-asserted
- `smoke.spec.ts:199` ("dry-run document renders the resolution table") drives a fixture
  with `{track_id:0,track_kind:"video"}` and `{track_id:1,track_kind:"audio"}`
  (`:173-174`), and asserts the table renders with 3 rows (header + 2 assignments,
  `:233`). `resolvedTrackLabel` runs for both rows, so the `fluent.$t` render path is
  exercised end to end (the regenerated mount-harness embeds `batch-resolved-track` in
  both catalogs; verified present in `e2e/.generated/mount-harness.js`, which is
  gitignored - correctly not committed).
- The test does not text-assert `"0 (video)"`/`"1 (audio)"`; a fluent fallback to the
  bare key would still yield 3 rows. This is a **pre-existing** coverage trait, not
  introduced or regressed by Task 6, and not in scope for this task. Noted, no finding.

### 6. THE ADJUDICATION - the extended null guard

**The deviation.** Plan Step 2 hands over this literal:
```ts
return a.track_id === null
  ? "-"
  : fluent.$t("batch-resolved-track", { id: a.track_id, kind: a.track_kind });
```
The implementer shipped instead:
```ts
return a.track_id === null || a.track_kind === null
  ? "-"
  : fluent.$t("batch-resolved-track", { id: a.track_id, kind: a.track_kind });
```
with a source comment citing core's `planner.rs:53` invariant and a DONE_WITH_CONCERNS
flag.

**Reproduction (this reviewer, not borrowed).** I applied the plan's literal to
`ResolutionTable.vue` in-tree and ran `pnpm exec vue-tsc --noEmit`:
```
src/components/ResolutionTable.vue(38,59): error TS2322: Type 'string | null' is not
assignable to type 'FluentVariable'.
  Type 'null' is not assignable to type 'FluentVariable'.
vue-tsc exit: 2
```
Then restored the file (`command cp -f`; `cmp` byte-identical; sha `7fa30bde...`; tree
clean). The type facts behind it, all read from the tree:
- `PlanAssignment.track_id: number | null`, `track_kind: string | null` (`src/ipc.ts:106-107`),
  mirroring `Assignment.track_id: Option<u64>`, `track_kind: Option<String>`.
- `useFluent()` returns `TranslationContext`; its `$t` params type is
  `Record<string, FluentVariable | CustomVariableTypes>`
  (`fluent-vue/dist/TranslationContext-*.d.mts:118`).
- `FluentVariable = FluentType<unknown> | string | TemporalObject | string | number | Date`
  (`@fluent/bundle/esm/types.d.ts:11`) - **no `null`**; `CustomVariableTypes = never`
  (no project `TypesConfig` augmentation, `grep` over `src/` empty).
- `tsconfig.json` `strict: true`, and `pnpm build` runs `vue-tsc --noEmit`.

So the plan's literal is genuinely non-compiling: after narrowing only `track_id`, the
`kind: a.track_kind` argument is still `string | null`, rejected by the param type, and
this would break the mandatory `pnpm build` gate. Transcribing the plan verbatim is not
an option.

**The cited invariant - verified, and stronger than cited.** `planner.rs:53` reads
`/// None exactly when track_id is None.` on `pub track_kind: Option<String>` - the
citation is exact (also mirrored in `ipc.ts:99`). I audited every `Assignment`
constructor in core: exactly two exist - `unmatched()` (`:67`, both fields `None`) and
the matched push (`:669`, `track_id: Some(tid)`, `track_kind: Some(tkind)`). Core sets
the two fields as a pair in both arms, so it **structurally cannot emit**
`{track_id: Some, track_kind: None}`. The implementer's extra disjunct is therefore
provably unreachable for real core data: zero outward behavioral change on any state
core produces.

**Argue both directions.**

*Grant-covered mechanical repair (the implementer's position, and mine):* The governing
rule is `brief-drafts-verified-against-tree` - an SDD brief's literal draft (code,
premises, line spans) is verified against the tree and local idiom, and a divergence is
**adapted and surfaced, never transcribed**. The plan-6 T9 occurrence is nearly
isomorphic: a design literal that could not compile against the actual types, adapted to
the one type-correct form, disclosed with a source comment, reviewer-confirmed the
original cannot compile - adjudicated correct keyboard-level resolution. Here the
implementer met that obligation in full: reproducible compile failure, a repair that
(a) reuses the placeholder D60 already blessed for the no-id/no-kind case rather than
inventing a sentinel, (b) is defended by a verified (and structurally enforced)
invariant, and (c) is disclosed both in the report (DONE_WITH_CONCERNS) and at the site.
Among the type-correct repairs it is the most conservative and the most honest: a
non-null assertion (`a.track_kind!`) or a cast would *suppress* the invariant the type
error surfaces (a symptom-hiding move), whereas narrowing to `"-"` degrades gracefully to
D60's own "nothing resolvable to show" rendering if the invariant were ever violated.

*Owed a NEEDS_CONTEXT round-trip (the fork position):* The plan's Global Constraints are
maximally strict - "Every fork in this plan is closed... A fork discovered on code
contact returns as NEEDS_CONTEXT... never decided at the keyboard." A compile failure is
a discovery on code contact. The `zero-content-structural-forks` grant's four-part test
has a bright-line item 4 ("nothing user-visible"), and the fix *does* alter user-visible
output for the wire state `{id: non-null, kind: null}`: the wire format (JSON over IPC)
can represent it - `string | null` is the proof the boundary admits it - even though core
never sends it. D60 also *enumerated* the `"-"` trigger as `track_id === null`; widening
it to a second condition is a semantic choice, and "the grant fills silence only - an
explicit enumeration always wins." The recorded plan-6 T13 shape (self-classifying a
user-visible-touching change as grant-covered, ruled a skipped NEEDS_CONTEXT) is the
cautionary parallel.

**Ruling.** Grant-covered mechanical repair; **not** a skipped fork. The fork position's
decisive weakness: *every* type-correct repair of a non-compiling literal makes *some*
choice about the `{id, kind:null}` state, because the plan's literal has no behavior at
all (it does not compile), and no repair exists in TypeScript that both compiles and
preserves the plan's (non-existent) behavior - the two Options cannot be narrowed as a
pair without a discriminated-union change at the wire boundary, which would itself be an
out-of-scope data-format change. So "route it because any fix changes behavior on the
null-kind state" proves too much: it would make a compile-blocking defect in the plan's
own literal un-repairable at the keyboard under any circumstance, contradicting the
settled `brief-drafts-verified-against-tree` doctrine that non-compiling drafts are
adapted-and-surfaced. The correct partition: a *fork* is an unenumerated design-latitude
question; this is a *defect in the plan's own transcribed literal*, which
`brief-drafts-verified-against-tree` assigns to keyboard-adaptation-plus-disclosure. The
implementer identified the correct side of that partition and discharged it well. The
DONE_WITH_CONCERNS flag is appropriate honesty, not an admission of overreach.

---

## Fire-verification of my own absence checks

- My first invariant grep, `grep "None exactly when" planner.rs`, returned empty. That is
  a malformed-pattern false negative: a backtick sits between `None` and `exactly` in the
  source (`` `None` exactly when ``). Corrected `grep "exactly when"` found it at
  `planner.rs:53`; a control token known-absent (`ZZZ_NOT_PRESENT_TOKEN`) returned exit 1
  with no output, confirming grep does fire on true absence and the earlier empty was the
  pattern, not the fact.
- The `CustomVariableTypes`/no-augmentation absence (`grep customVariableTypes src/` ->
  none) is corroborated positively: `CustomVariableTypes` resolves to `never` only when
  no `TypesConfig` augmentation exists, and the `$t` call type-checks under the shipped
  code, which it could not if a stricter custom type were injected.
- Restore discipline: the first `cp -f` restore silently no-op'd (a `cp -i` shell alias
  intercepted it; `cmp` caught the still-modified file), fixed with `command cp -f`;
  final `cmp` byte-identical and `git status` clean.

---

## HARVEST

**H1 - plan defect for the record (class relation).** The plan carried a fenced code
block presented as the implementation ("the function becomes:") that does not compile
against the repo's own types. It reached execution because the plan's self-review checks
type *consistency between tasks* (does Task 6 use the field names/types Task 5
established?), not *compilation* of the snippet against the actual `FluentVariable` param
type. Class relation: **plan-carried code is a claim - specifically that it compiles and
behaves as described - and a transcription task is the first place that claim is tested,
at compile time in the real tree.** This is the code sibling of `proc-57-briefs-not-
ground-truth` (a brief's load-bearing premise is verified at source) and a direct
instance of `brief-drafts-verified-against-tree`. The durable wrinkle worth recording:
compilation is a verification the plan/design phase *structurally cannot perform* on a
prose snippet - only transcription in the real tree surfaces it - so a plan's own review
green-lighting a fenced block is never evidence the block compiles. Candidate: reinforce
`brief-drafts-verified-against-tree` with this occurrence (plan-7 T6: plan-literal
`resolvedTrackLabel` fails `vue-tsc` TS2322 against `FluentVariable`; adapted with an
invariant-backed null narrowing, disclosed, reviewer-reproduced).

**H2 - over-restriction watch (zero-content grant boundary calibration).** This is
calibration data either way, per the boundary's own watch clause. The extended guard
touches user-visible output for the wire-representable-but-core-impossible state
`{id: non-null, kind: null}`, which the four-part test's item 4 arguably does not clear
for the *full* wire domain. Yet routing it as a fork would force a NEEDS_CONTEXT
round-trip for a compile-repair of the plan's own defective literal - resurrecting
exactly the "transcribe non-compiling code" failure mode `brief-drafts-verified-against-
tree` exists to prevent. **Recommendation: do NOT tighten the boundary to force routing
of compile-forced repairs.** The right discriminator is not "does the fix touch a
user-visible surface for some wire-representable state" but "is this a design-latitude
choice, or a repair of a non-compiling transcribed literal?" A compile-forced repair that
(i) reuses an already-enumerated placeholder, (ii) is backed by a verified/structurally-
enforced core invariant, and (iii) is disclosed, is inside `brief-drafts-verified-
against-tree`, not a fork. This occurrence is evidence the boundary is drawn correctly:
the zero-content grant governs discretionary pattern extensions, and a separate rule
governs draft-repair; conflating them would over-restrict.

**H3 - reusable GUI-i18n hazard (for the rest of stream B and wave 3).** Any `$t`/`$ta`
call that passes an `Option<T>`-derived field (`T | null` on the TS side) as a placeable
argument needs a null-narrowing before the call: `FluentVariable` rejects `null`
(`@fluent/bundle`), and `check-i18n` does **not** catch this - only `vue-tsc` does. T7
(D56) and any later site rendering a nullable IPC field through Fluent inherit the same
constraint; the narrowing target should reuse the surface's already-decided empty/placeholder
token, not a new sentinel.
