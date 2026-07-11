<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 1 of 1)
  session_uuid:       ffc7c915-ea2a-432b-9ffd-64ac313a2a6f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f.jsonl
  tool_use_id:        toolu_01CuFQzyqTRHdBabBwuny8G5
  agent_id:           a362a520427c54db7
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f/subagents/agent-a362a520427c54db7.jsonl
  dispatch_desc:      Whole-branch review Plan 3.5
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T18:02:41.330Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

I have everything needed. The controller already re-ran the full gate green at HEAD; my job was the cross-cutting view, and I verified the one load-bearing runtime claim (donor+keep ordering) against the real mkvmerge v100 rather than trusting the single-file test. Here is the review.

---

# Whole-branch review: Muxsmith Plan 3.5 (b04c4a2..2b08de4)

## Strengths

- **The six commits compose cleanly.** The three features touch disjoint stages of the pipeline and interact only through well-defined seams: D19 changes *which tracks match* (matcher/validation), D20 changes *how the matched plan renders* (command), D21 changes *whether resolution runs at all* (a pre-loop gate). I traced every pairwise interaction (see below) and found no composition defect.
- **D21 gate placement is exactly right.** `resolve_file` (planner.rs:365-374) fires `UnsupportedSource` *after* the skew warning and *before* the rule loop, mirroring the `UnidentifiableSource` early-return's `FileReport` shape. Crucially it uses the raw `!container_recognized || !container_supported` predicate, **not** `is_identifiable()` (identify.rs:144-145, which also tests `!tracks.is_empty()`). That distinction is the whole point of D21 decision #5, and the three regression tests (`unrecognized_container…`, `unsupported_container…`, `recognized_supported_zero_tracks_stays_missing_track…`) lock all three corners including the zero-track-stays-missing-track boundary. This is the kind of trap a per-task review passes over and it was handled precisely.
- **The BCP-47 accept/match asymmetry is sound and self-consistent.** Accept side (`is_valid_value`, runtime.rs:175) = ISO-index-hit OR `parse().is_ok()`; match side (`lang_eq`, matcher.rs:142) = ISO-canonical-key first, else BCP-47 `canonicalize()` compare, else raw. The two-arm ordering means bare ISO codes (`de`/`ger`) never leave the fast path and existing behavior is untouched, while region/script tags (`pt-BR`, which is not in mkvmerge's ISO table) route to the canonical path. I walked `exact: { language: pt-BR }` against a track carrying `language: por` + `language_ietf: pt-BR`: it correctly matches via the `language_ietf` field (both canonicalize to `pt-BR`) and correctly does *not* match via `por` (canonicalizes to `pt`). The `notalanguage` test-literal migration (from `zz`/`zzz`) is not cosmetic: `zz`/`zzz` are now well-formed 2-3 ALPHA tags that legitimately parse, so the tests needed a value that exceeds the 8-char primary-subtag limit to still fail validation. That was correctly reasoned through.
- **The `tracks` restructure is complete.** A full-repo sweep for `profile.tracks` bare-list accesses (`.tracks[`, `.tracks.len/is_empty/get/last/iter/first`, excluding `Identification.tracks`) returns zero un-migrated sites. The JSON schema is generated fresh from the model at runtime (`muxsmith schema`), so there is no stale checked-in golden to drift; `cli_schema.rs` regenerates and passes. `thorough_separation` applied correctly.
- **The gated live test is honest verification, not decoration.** It builds a real 3-track source with mkvmerge, sanity-checks the fixture's track-id assignment before relying on it, then asserts the exact output order. That is the SI-3 discipline the plan demanded.

## Issues

### Important

**1. Donor+`keep` track ordering is surprising for the marquee use case, and is neither documented nor tested across the input-file boundary.** (`command.rs:189`, `push_track_selection` / `push_track_order`)

The `keep` guard itself is correct in every case I checked, including with donors: it suppresses selection flags only for `source == plan.source` (command.rs:189), so donor groups still get their per-category selection, and matched primary tracks still get property options + `--track-order`. No data is lost; all tracks are kept. That part is solid.

The sharp edge is what happens to the *order*. Under `keep`, only matched tracks land in `--track-order`; mkvmerge appends the kept-unmatched primary tracks after all listed ones. The single-file live test confirms the intra-file case. I verified the cross-file case against mkvmerge v100 directly, because it is exactly the additive scenario the D20 memo motivates `keep` with ("add a German external subtitle to every file, keep everything else"):

```
mkvmerge -o out.mkv ( primary.mkv[PRIM0,PRIM1] ) --subtitle-tracks 0 ( donor.mkv[DONOR] ) --track-order 1:0
=> output order: [DONOR, PRIM0, PRIM1]
```

So a profile that writes *only* a donor rule under `keep` puts the added subtitle **first**, ahead of the primary's video/audio. This is internally consistent with the memo's accepted assumption ("unmatched-kept tracks placed after the explicitly-ordered matched tracks") and is fully recoverable by the user (add explicit primary rules for the tracks whose order matters, which then precede the appended remainder). It is therefore **not a correctness defect and not a merge blocker** — but it is a genuine usability trap on the one workflow `keep` exists to enable, and the cross-file ordering is currently unverified.

Fix (follow-up, not blocking): (a) add one sentence to spec 4.5 / the `keep` docs stating that added donor tracks and any explicitly-matched tracks are ordered first, with kept-unmatched primary tracks appended in source order — so the additive recipe shows an explicit `video`/`audio` rule ahead of the donor rule; (b) add a gated cross-file variant of `live_keep_unmatched_orders_only_listed_track` (primary + donor) so the multi-input ordering is a standing guard, not an inference from the single-file case.

### Minor

**2. `NoTrackRules` diagnostic points at `"tracks"`, not `"tracks.rules"`.** (`validate.rs:60`) The empty check now tests `.rules` but the `config_path` still says `"tracks"`. Because `rules` is a required field (no `#[serde(default)]`), the only way to reach this diagnostic is an explicit `rules: []`; `tracks: { unmatched: keep }` without `rules` is a *parse* error instead. Two different surfaces for "no rules," and the one diagnostic that does fire under-specifies its location. `"tracks.rules"` is the precise pointer. No test asserts the path, so this is free to tighten.

**3. `TracksCfg` + `drop_policy()` (model.rs:301-316) are inserted between the `AttachmentsCfg` struct (276) and its `impl Default for AttachmentsCfg` (318),** splitting a contiguous type from its impl. Pure locality; move the new block below `impl Default for AttachmentsCfg` when convenient.

**4. No golden/unit test exercises `keep` + a donor's selection flags.** Both `keep` tests (unit `keep_unmatched_suppresses_primary_selection_flags`, live `live_keep_unmatched_orders_only_listed_track`) use a primary-only plan. The donor-still-gets-`--subtitle-tracks` half of the guard's contract is correct by inspection but unguarded by a test. Covered naturally if you take item 1(b).

**5. `keep_unmatched: bool` now serializes into any `Plan` JSON.** Additive and documented, harmless for Plan 4's consumer, noted only so it is a conscious part of the plan wire format going forward.

## Recommendations

- Take item 1(a)+(b) as a small doc+test follow-up before `keep` reaches users; it is the highest-value residue of this branch. Everything else is optional cleanup.
- The "`exact` is typed value-equality, not string equality" principle now stated in spec 4.3 is genuinely load-bearing and easy to lose; the self-review already flags it for the 1.0 README pass. Keep that flag alive — a user who assumes `exact` is `strcmp` will be surprised by `de` == `ger` and `pt-Latn-BR` == `pt-BR` in the opposite direction from the surprises above.

## Triage of the five roll-up items

1. **`TracksCfg` placement locality** — **defer** (cosmetic move, item 3 above; do it on the next touch of model.rs).
2. **`NoTrackRules` config_path `"tracks"` vs `"tracks.rules"`** — **fix-now-cheap** (item 2; one-line, no test churn, and it is the user-facing pointer). If you would rather not reopen the branch for a cosmetic string, **defer** is acceptable — it is not wrong, only imprecise.
3. **keep golden lists `--audio-tracks` among suppressed flags** — **fine-as-is.** The roll-up undersells it: audio is the matched category, so under `drop` that flag would be present as `--audio-tracks 1`; asserting its absence therefore confirms `keep` suppresses even the matched category's select-flag, and the full four-flag set is exactly what a regressed guard would emit. It is a real guard, not a redundant one.
4. **`notalanguage` literal reused in two tests** — **fine-as-is** (a shared const would read marginally better, but the intent — a value that fails BCP-47 parse — is clear in context).
5. **`lang_eq_preserves_meaningful_distinctions` passes under pre-fix code** — **fine-as-is**, with the correct label: it is a forward regression guard, not a discriminator of this change. The actual discriminator is `lang_eq_canonical_forms_match` (fails under the old raw compare because `pt-BR` != `pt-Latn-BR` as strings), and it is present. The pair is adequate.

## Assessment

**Ready to merge: Yes.** The three fixes compose correctly, the D19 accept/match asymmetry and the D21 gate predicate are both implemented with the precision their design memos demanded, and the `tracks` restructure is complete with no orphaned consumers. The one item worth attention (donor+`keep` track ordering) is a documentation-and-coverage gap on correct-as-designed behavior, not a defect — best handled as a fast follow-up rather than a gate on this branch.