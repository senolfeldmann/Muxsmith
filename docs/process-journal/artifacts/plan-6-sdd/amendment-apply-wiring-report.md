# Amendment 4 report: Task 14 apply-wiring routing

**Status: DONE.** One file changed: `docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`.
No commit (held in the working tree for a delta review, as instructed). No code touched.

## The refuted premise

Task 14's original Step 3 wired the apply flow as
`SuggestionCard.vue -> DiagnosticsPanel.vue -> apply_suggestion`. The implementer
returned NEEDS_CONTEXT (held uncommitted) rather than build it, correctly:
the chain rests on a false topology premise. Controller verification against the
trees confirmed the refutation and additionally exposed a `config_path`
locator-as-path misuse in the uncommitted self-contained draft.

## Corrected wiring (tree evidence)

- **Sibling topology.** `src/views/BatchView.vue:405` renders
  `<DiagnosticsPanel :diagnostics="generalDiagnostics" />` and `:429` renders
  `<SuggestionCard v-for=...>` - two separate `<section>`s under BatchView, not
  parent/child. Nothing routes a card emit through the panel. (Verified by
  reading BatchView.vue in full.)
- **The picked FILE path lives in BatchView.** `selectedProfile = ref<string |
  null>(null)` (`:27`), set by the profile picker (`pickProfile ->
  selectProfile(picked)`, `:142`/`:128`). This is the profile file path to
  load/save by.
- **`config_path` is a config-field locator, never a file path.**
  `Suggestion.config_path` is `tracks[<N>].match`, parsed by
  `rule_index_of` (`crates/muxsmith-core/src/planner.rs:2117`; design D43
  `:480-483` states the same: "not a general path"). It is forwarded opaque to
  `apply_suggestion`; core does all the interpreting.
- **The draft's misuse.** The uncommitted draft (in
  `.worktrees/plan6-e`, base a91e56f) implemented apply *inside*
  `SuggestionCard.vue` and called `loadProfile(props.suggestion.config_path)` and
  `saveProfile(props.suggestion.config_path, updated)` - loading/saving a file
  whose path is `tracks[<N>].match`. Runtime-broken. It passed 27/27 e2e only
  because the fixture set the suggestion's `config_path = PROFILE_PATH =
  "/profiles/demo.yaml"` (draft `applyReport.suggestions[0].config_path`), so
  load/save-by-`config_path` accidentally equalled load/save-by-path and the
  echo mock, being semantics-blind, could not tell the two apart.
- **IPC surface exists at the wave-3 base.** `.worktrees/plan6-e/src/ipc.ts`
  exports `loadProfile(path) -> LoadProfileDocument` (`ReportDocument` + `profile:
  Profile | null`), `applySuggestion(profile, configPath, edit) -> Profile`
  (invoke `apply_suggestion { profile, configPath, edit }`), and
  `saveProfile(path, profile) -> void` (invoke `save_profile { path, profile }`).
  `Suggestion.edit` stays `unknown`; the cast `as StructuredEdit` happens at the
  applySuggestion call site (the draft's established pattern).

**Corrected chain:** `SuggestionCard` renders the apply control and emits
`apply({ config_path, edit })` - its two opaque fields, still never interpreted
in the frontend. `BatchView` (the direct parent, owner of `selectedProfile` and
the IPC call sites) handles the emit:

```
loadProfile(selectedProfile)
  -> guard doc.profile === null  (post-suggestion ParseError, folded into
     config_diagnostics[0] per D42; surfaced via BatchView's existing shared
     alert line at :373-378)
  -> applySuggestion(doc.profile, config_path, edit as StructuredEdit)
  -> saveProfile(selectedProfile, updated)
```

`config_path` -> `apply_suggestion` (opaque locator). Load/save path ->
`selectedProfile` (picked file). Two different values. `DiagnosticsPanel` is not
in the chain.

## DiagnosticsPanel disposition (explicit, one choice)

**Kept in the Files list, role = documentation-only comment.** The uncommitted
draft already carries a suitable comment there (recording that apply lives in
SuggestionCard, the no-fix/partition diagnostic renders unchanged, and why no
per-diagnostic apply control is added). Keeping it preserves the "why not here"
for a future reader at the sibling and keeps the resumed implementer's refit
minimal. The Files-list entry directs the implementer to refit the draft
comment's "entirely inside SuggestionCard" phrasing to the sibling-emits /
parent-handles split, since orchestration now lives in BatchView.

## Post-apply behavior (design citation)

**BatchView does nothing beyond dropping its applying state (and clearing any
prior error) after a successful `saveProfile`; the just-applied suggestion card
stays on screen until the user next runs the dry-run.**

Design support: D43 `:509-515` locates post-apply validation in *the editor's*
existing `validate_profile_model` round-trip - which the batch view does not
have (no model, no `watch`). `core-03` `:503-504` guarantees the applied edit
"survives the next dry run" - a correctness property observed when the user next
runs one, **not** a mandate that apply auto-triggers one. The batch view's
`runDryRun` (`:176-195`) runs only on explicit button click today.

Auto-re-running the dry-run after apply would therefore be a **new user-visible
behavior the design does not record** - a NEEDS_CONTEXT / owner call, not the
amendment's to specify. So it is recorded as a deferred out-of-scope item
(controller-routed, ROADMAP candidate), and Task 14 specifies the design-supported
minimum: leave the report as-is post-save. This is also compatible with the
draft (its `finally` only drops `applying`).

## Concurrency concern disposition

The implementer's report names an apply-vs-editor concurrency hazard (`App.vue`
keeps all views mounted with `v-show`; an EditorView holding an older in-memory
model can overwrite an applied fix on its next Save; the load/apply/save round
trip writes disk independently of the editor's model). Recorded in Task 14 as an
explicit **out-of-scope, controller-routed** note (whole-branch/ROADMAP
candidate), **not** an implementer TODO, per the instruction.

## Plan hunks (all in the one file)

1. **New "Amendment 4 (2026-07-16): Task 14 apply-wiring routing" block** after
   Amendment 3, before the Wave-1 `---`. Four sentences: refuted premise,
   verified facts, corrected wiring, marker convention.
2. **Task 14 Files list** rewritten (marker on the header): SuggestionCard
   (renders control + emits, does not orchestrate), **`src/views/BatchView.vue`
   added** (parent, handles emit, load/apply/save via `selectedProfile`, imports
   the three IPC fns), DiagnosticsPanel (documentation-only comment, not in
   chain), the two `.ftl` files, the e2e test.
3. **Interfaces "Consumes"** updated (marker): now names `load_profile`,
   `apply_suggestion`, `save_profile` via `loadProfile`/`applySuggestion`/
   `saveProfile`.
4. **Binding points**: the old one-line "Apply does not validate; the editor's
   existing round-trip does." replaced by two marked bullets - **Wiring** (the
   full corrected chain, config_path-is-a-locator-not-a-path, DiagnosticsPanel
   out, aria-busy on the clicked card) and **Post-apply behavior** (nothing
   beyond state refresh, with the D43 `:509-515` / core-03 `:503-504` citation).
5. **New "Out of scope, routed by the controller" block** under the binding
   points: apply-vs-editor concurrency, and auto-refresh-after-apply.
6. **Step 1** rewritten (marker): the e2e fixture must make the picked path and
   the suggestion's `config_path` two different values (`PROFILE_PATH =
   "/profiles/demo.yaml"` vs locator `"tracks[0].match"`); assert
   `apply_suggestion.configPath` equals the locator and `load_profile` /
   `save_profile` `path` equals the picked path, so a swap cannot pass. No-fix
   paired count-0 control kept (present/absent on the identical selector = the
   falsifiability control).
7. **Step 3** rewritten (marker): the through-DiagnosticsPanel wiring sentence
   replaced by the correct SuggestionCard-emits / BatchView-orchestrates wiring;
   D22-comment-update instruction kept and clarified (apply now reads `edit` =
   emitted and forwarded).
8. **Step 5 `git add`**: `src/views/BatchView.vue` added to the staged set.

## Premises verified

| Premise | Source | Result |
|---|---|---|
| DiagnosticsPanel/SuggestionCard are siblings, not parent/child | BatchView.vue:405/:429 | Confirmed |
| Picked FILE path lives in BatchView | BatchView.vue:27 `selectedProfile` | Confirmed |
| `config_path` = `tracks[<N>].match`, parsed by `rule_index_of` | planner.rs:2117; design D43 :480-483 | Confirmed |
| Draft loads/saves by `config_path` (runtime-broken) | worktree diff, SuggestionCard.vue draft | Confirmed |
| Draft e2e fixture sets suggestion.config_path = PROFILE_PATH | worktree e2e :382 / diff | Confirmed |
| loadProfile/applySuggestion/saveProfile/LoadProfileDocument exist at base | worktree src/ipc.ts:270-302 | Confirmed |
| Post-apply validation is editor-model-based (D43); core-03 = survives NEXT dry run | design :509-515, :503-504 | Confirmed |
| No plan text implies config_path is a file path | grep (plan :972, :1606 both opaque) | Confirmed (only correct usages) |
| Only wrong-topology plan text = the through-panel chain | grep (:1597, :1624 old) | Confirmed, both corrected |

## Ripple

Grepped the plan for both defect classes. (a) through-DiagnosticsPanel chain:
the only two sites were the Files-list line ("wire the emit through") and Step 3;
both corrected. Post-edit grep for `wire the emit through` and the
`SuggestionCard -> DiagnosticsPanel -> the command` chain returns nothing; the
sole surviving `SuggestionCard.vue -> DiagnosticsPanel.vue` string is the
quoted-refuted original inside Amendment 4 (intended). (b) config_path-as-file-path:
the plan's two `config_path` hits (:972 shell command, :1606 opaque-fields
binding) both already treat it correctly as an opaque locator - the misuse
existed only in the draft code and its e2e fixture, never in the plan, so no plan
text needed that correction. Both grep hits were inspected (the grep produced
output, so it is live, not malformed).

## Concerns

1. **Busy-state granularity** is left to the implementer as an ordinary
   idiomatic choice (per-card vs an app-wide `applying` flag), constrained only
   by the observable the e2e asserts: the clicked card's `batch-suggestion-apply`
   button carries `aria-busy` during the round trip and drops it after. Not a
   design fork; the house already gates BatchView buttons app-wide via `busy`.
2. **Line-number drift.** `rule_index_of` is at `planner.rs:2117` in the current
   tree (design D43 cites `:2032`; the ledger T14 entry cites `:1897`). The
   amendment cites the current `:2117`. The line has drifted across
   worktree/merge states; the function identity is what binds, not the number.
3. Nothing blocked. The refit is minimal for the resumed implementer: keep the
   button/testid, the two keys, the D22 comment (with the noted phrasing refit),
   and the no-fix paired control from the draft; move orchestration from
   SuggestionCard into BatchView; correct load/save to use `selectedProfile`
   and pass `config_path` only to `apply_suggestion`; make the e2e fixture's
   path and locator distinct.
