### Task 14: D43 - one-click apply in the batch view

Sequenced **after Task 13b** (F4; amended 2026-07-16, detail-editor routing - Task 13b is the new last wave-3 task and also writes `e2e/smoke.spec.ts`). Its only file it shares with wave 3 is `e2e/smoke.spec.ts`, and running after Task 13b means no concurrent writer - so no new spec file is needed.

**Files (amended 2026-07-16, apply-wiring routing):**
- Modify: `src/components/SuggestionCard.vue` - renders the apply control and EMITS an `apply` event carrying its two opaque fields (`config_path`, `edit`), still never interpreted in the frontend; it does NOT orchestrate the round trip. Update the stale D22 comment at `:6-13`.
- Modify: `src/views/BatchView.vue` - the direct parent of both `SuggestionCard` and `DiagnosticsPanel` (siblings here since Plan 5, `:405`/`:429`), owner of the picked-profile FILE path (`selectedProfile`, `:27`) and of the existing IPC call sites. Handles the `apply` emit: `loadProfile(selectedProfile) -> applySuggestion(model, config_path, edit) -> saveProfile(selectedProfile, updated)`. Imports `loadProfile`/`applySuggestion`/`saveProfile` from `../ipc` (Task 8's surface; present at the wave-3 base).
- Modify: `src/components/DiagnosticsPanel.vue` - documentation-only comment; NOT in the apply chain. Records that the apply control lives in `SuggestionCard` (its sibling) handled by `BatchView` (their common parent), that the no-fix/partition diagnostic renders here unchanged with no apply control, and why none is added here. No template, prop, or emit change. (The uncommitted draft already carries a suitable comment; refit its "entirely inside SuggestionCard" phrasing to the sibling-emits/parent-handles split.)
- Modify: `locales/en/gui-batch.ftl`, `locales/de/gui-batch.ftl` (2 keys)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes (amended 2026-07-16, apply-wiring routing): Task 8's `load_profile`, `apply_suggestion` and `save_profile` commands, via `src/ipc.ts`'s `loadProfile`/`applySuggestion`/`saveProfile`.
- Produces: the apply control.

Binding points:
- The frontend **forwards two opaque fields it never interprets** - `config_path` and `edit`. Core does all the interpreting. Do not parse `config_path` in TypeScript, and do not read into `edit` (it stays `unknown` in `src/ipc.ts:132`; D49 confirms the echo is type-preserving).
- The two new keys go in the existing **`gui-batch.ftl`**, beside `SuggestionCard.vue`'s current copy-button keys (`batch-suggestion-copy` / `batch-suggestion-copy-tooltip`, `gui-batch.ftl:54-55`), because the apply control lives in the batch view. Only the editor's own surface justified the new `gui-editor.ftl`.
- **The no-fix case has no apply button and that is not a gap to close.** `core-109-two-required-no-fix` records that two required rules colliding on one track yield no suggestion at all, only the partition report; the diagnostics panel renders it as it does today.
- **Wiring (amended 2026-07-16, apply-wiring routing).** `SuggestionCard` renders the apply control and emits `apply` carrying `{ config_path, edit }`; `BatchView`, its direct parent, handles the emit against the profile FILE path it already owns (`selectedProfile`, `:27`): `loadProfile(selectedProfile)` -> guard `doc.profile === null` (a `ParseError` arising after the suggestion was computed; `load_profile` folds it into `config_diagnostics[0]` per D42 rather than throwing, surfaced through BatchView's existing shared alert line at `:373-378`) -> `applySuggestion(doc.profile, config_path, edit as StructuredEdit)` -> `saveProfile(selectedProfile, updated)`. **`config_path` is a config-field LOCATOR (`tracks[<N>].match`, parsed core-side by `rule_index_of`), never a file path**: it is forwarded opaque to `apply_suggestion` and is NOT the load/save path. The load and save path is always `selectedProfile`. `DiagnosticsPanel` is not in the chain. While the round trip is in flight BatchView reflects it on the clicked card's control via `aria-busy` and gates re-entry, mirroring its existing `dryRunning`/`busy` idiom.
- **Post-apply behavior (amended 2026-07-16, apply-wiring routing).** After a successful `saveProfile`, BatchView does **nothing beyond dropping its applying state** (and clearing any prior error); the just-applied suggestion card stays on screen until the user next runs the dry-run. It does **not** auto-re-run the dry-run or `validate_profile`. Design D43's post-apply validation (`docs/superpowers/specs/2026-07-15-plan-6-design.md:509-515`) runs through *the editor's* `validate_profile_model` round-trip, which the batch view does not have; `core-03`'s guarantee is that the applied edit "survives the next dry run" (design `:503-504`) - a correctness property observed when the user next runs one, not a mandate that apply triggers one. Auto-refreshing the batch view's suggestion list is a user-visible behavior the design does not record, so it is deferred (see the out-of-scope note), not built here.

**Out of scope, routed by the controller (not implementer TODOs):**
- **Apply-vs-editor concurrency.** `App.vue` keeps all views mounted (`v-show`, not `v-if`), so an `EditorView` holding an older in-memory model of the same file can, on its next Save, overwrite a fix applied here from the batch view - the load/apply/save round trip writes disk independently of the editor's model. Design D41 names shared-model ownership as the reason apply and the editor must reconcile, a property the as-built independent views do not provide for any caller. Whole-branch/ROADMAP candidate; the controller routes it, Task 14 does not solve it.
- **Auto-refresh after apply.** Whether the batch view should automatically re-run its dry-run after a successful apply (so the suggestion list reflects the new profile) is a user-visible behavior the design does not record. Deferred to the controller (ROADMAP candidate), not decided at the keyboard; Task 14 leaves the report as-is post-save (see the post-apply binding point).

- [ ] **Step 1: Write the failing e2e test (amended 2026-07-16, apply-wiring routing)**

Extend `e2e/smoke.spec.ts`. A suggestion card renders an apply button; clicking it drives the full `load_profile -> apply_suggestion -> save_profile` chain. The fixture must make the picked profile path and the suggestion's `config_path` **two different values** so a swap of locator and path cannot pass (the echo-mock semantics-blindness that hid the draft's locator-as-path misuse):
- picked profile path (the dialog mock) = the file path, e.g. `PROFILE_PATH = "/profiles/demo.yaml"`;
- the suggestion's `config_path` = a LOCATOR, e.g. `"tracks[0].match"`, distinct from `PROFILE_PATH`.

Assert, against the recorded IPC calls (not a UI echo):
- `apply_suggestion` is invoked exactly once with `configPath` **equal to the card's locator** (`"tracks[0].match"`) and `edit` **deep-equal to the fixture's `StructuredEdit`** (`toEqual`, JSON boundary);
- `load_profile` and `save_profile` are each invoked once with `path` **equal to the picked path** (`PROFILE_PATH`), not the locator.

Because the two values differ, using the locator as the save path or the path as the apply locator fails an assertion. Keep the no-fix paired-control assertion: in the SAME report a partition/no-fix diagnostic (`suggestion-partition`, which carries no `Suggestion`) renders **no** apply button - `getByRole("button", name("batch-suggestion-apply"))` scoped to the diagnostics region has count 0, on the identical selector the positive assertion just proved resolves (the present/absent pair is the falsifiability control; a typo'd selector cannot make the negative pass vacuously).

- [ ] **Step 2: Run to confirm it fails**

```bash
pnpm test:e2e
```
Expected: FAIL - no apply button.

- [ ] **Step 3: Implement, update the stale comment, and add the two bilingual keys (amended 2026-07-16, apply-wiring routing)**

Implement the wiring per the Wiring binding point above: `SuggestionCard.vue` renders the apply control and emits `apply` carrying `{ config_path, edit }`; `BatchView.vue` handles the emit and orchestrates `loadProfile(selectedProfile) -> applySuggestion(model, config_path, edit) -> saveProfile(selectedProfile, updated)`, forwarding `config_path` opaque and never treating it as a path. `DiagnosticsPanel.vue` gets only the documentation-only comment (Files list). Add the two bilingual keys to `gui-batch.ftl`. **Update the D22 comment at `SuggestionCard.vue:6-13`**: it currently states `edit` "is deliberately never read" and that suggestions are "never applied", which Task 14 falsifies. Record that apply now reads `edit` (it is emitted to `BatchView` and forwarded to `apply_suggestion`), and that D41 supersedes D22's stated reason (the comment-preservation premise D22 rested on is dead - a canonical save does not preserve comments, so the editor+apply pairing survives on shared model ownership, not on the old machinery).

- [ ] **Step 4: Run the suite**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/components/SuggestionCard.vue src/views/BatchView.vue src/components/DiagnosticsPanel.vue locales/en/gui-batch.ftl locales/de/gui-batch.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: one-click apply-suggestion in the batch view (D43, D49)"
```

---

## Triggers this plan creates (controller mirrors into ROADMAP at plan close)

Design section 7 (`:1895-1930`) names seven; they are the controller's to write, not a task's. Restated here only so the plan close has them in one place:

1. A profile-model field gains a `#[serde(default)]` -> it joins D48's 17-row table with **all three** attributes naming the same function. This is the one place in the plan where getting it wrong loses user data silently.
2. **D48's derivation exists in the tree -> re-examine guard 2.** Mutate one field's `extend` expression away from its `default` function and see whether guard 2 goes red. If it cannot be made to fail, it is measured redundant and removed **then**, with the measurement recorded. If it can, the design phase was wrong to think it a tautology and the guard stays for good. Either way the question gets settled by running it, which is exactly what the design phase could not do.
3. `tauri-specta` publishes a stable non-RC Tauri-2 release -> re-evaluate D44's rejection.
4. A second Muxsmith artifact needs TypeScript types -> extend the `ts` feature's export set rather than hand-mirroring again.
5. 1.0 is tagged, or a user asks for zero-config schema autocompletion -> re-evaluate SchemaStore publication.
6. A profile-model field is added or removed -> the D44 drift check and the D45 registry both fail by construction, naming the site. No tracker entry needed; the mechanism **is** the tracker.
7. A second generated artifact gains a CI drift check -> the committed-generated-plus-drift-check pattern reaches count 2 toward Tier-2 promotion.

**D49 adds one removal trigger** (D49 §"Removal trigger", `:1122-1137`): after D49 lands, change `delta_for`'s `AddExact` arm to `map.insert(property.clone(), Scalar::Str(scalar_display(value)))` and run the suite. If G1, G2 and G3 all fail, they are load-bearing and stay; if only G3 fails, G1/G2 are candidates for removal as localizers. They stay until that run happens (`proc-proposed-safeguard-stays`).

## Open, carried into the plan close, not assigned to a task

- **`gui-22` vs `exec-44-runlog-14day-autoprune` is a recorded-statement collision** in `product-boundaries.yaml`: `gui-22` (`:243-252`) says v1 keeps all run logs with pruning deferred to v1.x, while `exec-44` (`:15-23`) records D35 reversing exactly that to an automatic 14-day prune, which shipped. `gui-22` still carries `status: settled` with no supersession marker. Unrelated to Plan 6, found while reading Tier 2 for the design and independently confirmed by the design reviewer (design `:1947-1956`). Needs a controller action item; a recorded-statement collision is one of the observable contested criteria, so it routes to the owner as a spec question.
- **The D23 `runActive` reset re-check** stays listed under the ROADMAP's Plan-6 anchor although the design review established it is run-path only (`src/views/JobsView.vue:150-200`) and touches nothing this plan builds. It stays until an owner call re-points it, rather than being moved silently.
- **The `IpcError`-code i18n gap** (D49 `:654-659`): nothing gates `IpcError` codes against `gui-common.ftl`, and `IpcError` params are not number-promoted through Fluent. D49 sidesteps both for its three codes (labelled-value wording, no plural selector). The structural gap is a ROADMAP item with its own trigger, out of Plan 6's scope.
