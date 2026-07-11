<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-3  (round 1 of 1)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01VRrKVzuctDQErp3wMKtLhf
  agent_id:           ac9a3663ce96a76b9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-ac9a3663ce96a76b9.jsonl
  dispatch_desc:      Review Task 3 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-10T11:59:41.038Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

All five brief interfaces match the required signatures exactly: `Mkvmerge::detect(override_path: Option<&Path>) -> Result<Mkvmerge, RuntimeError>` (`runtime.rs:101`), private `platform_candidates() -> Vec<PathBuf>` tested via same-file seam (`runtime.rs:234`, test at `runtime.rs:454`), `version_pair(&self) -> Result<(u64, u64), RuntimeError>`, `pub const MIN_SUPPORTED: (u64, u64)` (`runtime.rs:63`), and `RuntimeError::TooOld { found: String, minimum: String }` (`runtime.rs:35`). Ladder order (override -> `locate()` -> platform candidates, each probed by actually running `--version`, never by `Path::exists()`) matches D28 exactly (`runtime.rs:101-120`). All 4 brief-mandated test scenarios are present in `tests/mkvmerge_runtime.rs` (lines 87, 101, 118) plus the required `version_pair` unit tests. CLI call sites (`dry_run.rs`, `run.rs`, `identify.rs`) are untouched per the diff's file list; only `identify.rs`'s `Display` match gained the `TooOld` arm, which is compiler-forced exhaustiveness, not new call-site behavior.

I independently re-derived the SI-3 empirical chain against `~/Downloads/mkvtoolnix` rather than trusting the report:
- `src/merge/id_result.h:37` — confirmed `ID_JSON_FORMAT_VERSION = 20`.
- `doc/json-schema/...-v19.json` vs `...-v20.json` — confirmed by direct `diff`: the only substantive change is exactly what the doc comment claims (five enumerated `tag_*` properties replaced by `additionalProperties: true` + `patternProperties: "^tag_"`).
- `NEWS.md` "Version 86.0" entry (line 499) — confirmed it describes precisely that change ("track statistics tags ... prefixed with `tag_`"), and no schema-affecting entry exists in v83.0/v84.0/v85.0 between the v82.0 explicit "bumped to 19" (line 633) and v86.0 — the inductive step ("v86.0 is therefore the release that moved 19->20") holds.
- Packaging citations all check out verbatim: `packaging/windows/installer/mkvtoolnix.nsi` (`PROGRAMFILES64`/`PROGRAMFILES` + `MKVToolNix`), `packaging/macos/config.sh` (`APP_BUNDLE_NAME="MKVToolNix.app"`, no version suffix — correctly contradicts the brief's glob guess), `packaging/macos/build.sh` (`README.macOS.txt` telling users to copy to `/usr/local/bin`), `packaging/debian/mkvtoolnix.install` and `packaging/centos-fedora-opensuse/mkvtoolnix.spec` (both `/usr/bin`). This is genuine primary-source verification, not restated memory.
- `crate::capability::PINNED_IDENTIFICATION_FORMAT_VERSION = 20` (`capability/mod.rs:13`), which the new doc comment intra-links to, is a real, existing const — the doc-link target is valid.

### Strengths
- SI-3 empirical work is exemplary: every claim in the `MIN_SUPPORTED` and `platform_candidates` doc comments traces to a specific, checkable file in the mkvtoolnix source, and every one I re-checked matched (see above). This is exactly the discipline the task demanded and is easy to get wrong by hand-waving; it wasn't.
- The override-authoritative design (`runtime.rs:101-104`) is a clean, single early-return, doesn't leak into the PATH/platform rungs, and the flagged rationale (config errors shouldn't be silently masked by an automatic fallthrough) is sound and matches the controller's provisional acceptance.
- `enforce_floor` (`runtime.rs:183`) is a well-factored shared chokepoint: every rung's version-query + floor-comparison logic lives in one place, and `TooOld` is propagated immediately rather than treated as "try the next candidate," which is the correct read of "real, actionable signal" vs. "not found here."
- The flake fix is exactly what the brief asked to scrutinize and holds up: it lives entirely in `tests/mkvmerge_runtime.rs:16` (`fake_mkvmerge`, a test-only helper), is bounded (`attempt < 50` at 2ms each, ~100ms ceiling, then a loud `panic!`, never a silent skip), and the reasoning for why a second execution of the same script clears the race is correct (the warm-up call's completion guarantees the write-count race window has closed before the code under test executes the same path).
- `RuntimeError::TooOld` itself is properly data-only (`found`/`minimum` as plain strings); the only prose-emitting code touched is the pre-existing `IdentifyError` `Display` impl, which already emitted English sentences for every other variant before this task — correctly recognized as unavoidable exhaustiveness upkeep, not a new departure from core's no-prose rule.
- No non-ASCII punctuation anywhere in the diff (verified by direct scan).

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **Homebrew Apple Silicon path dropped, likely a real-world gap for T7.** `runtime.rs:212-224` drops `/opt/homebrew/bin/mkvmerge` because no Homebrew formula exists in the `mkvtoolnix` checkout itself (it lives in the separate `homebrew-core` repo, outside SI-3's evidentiary scope). This is a faithful, transparently-documented application of the "verify against packaging/, never from memory" instruction, so it's not a task defect — but Homebrew is a very common real-world install path on Apple Silicon Macs (`/usr/local/bin` from the README's manual-copy instructions covers Intel-prefix Homebrew incidentally, `/opt/homebrew/bin` does not). Worth flagging forward to whoever scopes T7's GUI-facing candidate list, since it's a product-completeness question the mkvtoolnix source tree genuinely cannot answer.
- **Double `--version` spawn on the PATH rung**, self-disclosed by the implementer (`runtime.rs:103-109`): `Mkvmerge::locate()` spawns once, `enforce_floor` spawns again on the same candidate to get a parseable string. A few ms in the common case, correctly reasoned as unavoidable without duplicating `locate()`'s Spawn->NotFound mapping. Not worth restructuring at this scale.
- **No unit test for the ladder-exhaustion `NotFound` path.** Disclosed and justified (in-process `PATH` mutation would race other parallel tests in the same binary, unlike the CLI's subprocess-isolated tests). The `NotFound` arm itself (`runtime.rs:119`) is a single unconditional `Err` with no branching, so risk is low, but it is the one brief-adjacent scenario with zero direct coverage.
- **`detect_prefers_override_over_path` (`tests/mkvmerge_runtime.rs:87`) proves override *works*, not strictly that it's *preferred over* PATH**, since the test can't safely mutate PATH in-process. It uses an implausible version number (v123.4.5, not a real mkvtoolnix release) as an indirect discriminator rather than a structural one. Reasonable given the constraint, but slightly weaker than the brief's "ladder prefers override over PATH" framing implies.
- **`IdentifyError`'s `TooOld` message reads slightly redundantly** (`identify.rs:283`): "mkvmerge failed: version mkvmerge v50.0.0 ('Old') 64-bit is older than the minimum supported 86.0" repeats "mkvmerge" and prefixes the raw `--version` line with "version". Cosmetic, and this Display impl is pre-existing prose infrastructure this task only extended for exhaustiveness, not a deliverable of Task 3 itself.

### Assessment
**Task quality:** Approved
**Reasoning:** Every interface, ladder-ordering, and evidentiary requirement in the brief is met, and the SI-3 empirical claims independently re-verified against the actual `~/Downloads/mkvtoolnix` source rather than trusting the report. The two implementer-flagged risk areas (flake fix scope, override-hard-fail design) both check out on inspection; remaining findings are disclosed coverage gaps or cosmetic, none rising above Minor.