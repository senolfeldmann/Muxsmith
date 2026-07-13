### Spec Compliance
✅ (crates/muxsmith-core/src/identify.rs:373-385, crates/muxsmith-core/tests/support/mod.rs:20-29, crates/muxsmith-cli/tests/catalog_completeness.rs:368-375) — every binding constraint satisfied exactly as specified.

### Strengths
- `known_extensions` trait method correctly stripped to a bare signature (`fn known_extensions(&mut self) -> Option<Vec<String>>;`), no default body remains.
- `FakeIdent` and `OneIdent` each get the exact explicit body from the brief (`{ None }`), byte-for-byte identical behavior to what the removed default previously supplied for them.
- Stale doc sentence ("Defaulted here so existing `Identify` fakes need no change to keep compiling.") removed; the remaining doc comment (spec 4.2 reference, `None` semantics, no-op-degrades-planning rationale) still reads coherently on its own.
- `LiveIdentifier` and `FakeIdentWithExtensions` genuinely untouched: neither file (`identify.rs` production impl block, `planner_resolution.rs`) appears in the diff at all.
- Diff is minimal and exactly matches the claimed shape: 3 files, 10 insertions / 5 deletions, no new `DiagCode`, no signature change beyond the default body removal.

### Issues
None found.

#### Critical
None.

#### Important
None.

#### Minor
- Brief cited `catalog_completeness.rs:467` for `OneIdent`; actual location is line 368 (confirmed by grep). Line-number drift in the brief itself, not a defect in this diff — the implementer's report correctly cites the actual location.

### House dimension
- Matches `docs/decision-ledger.yaml:715-724` entry `core-117-known-extensions-make-required` (`kind: non-decision`, `status: blocked`, `blocked_on: "idiomacy review (internal)"`), whose stated problem is exactly what this task fixes: a defaulted-`None` method on a single-production-impl trait silently vacuous-izes validation for any future impl. Making it required is the idiomatic fix and directly resolves the recorded non-decision.
- Report correctly does not edit `decision-ledger.yaml` itself, following this plan's established pattern (task-1/2/3/4/6/7/9) of surfacing ledger candidates for the controller/verdict step to harvest rather than writing the ledger from the implementer seat. Recommended edit (`status: blocked` -> `settled`, clear `blocked_on`, append occurrence) is correct and ready for the controller to apply.
- No fifth `Identify` impl exists anywhere in the workspace (`crates/muxsmith-core`, `crates/muxsmith-cli`, `crates/muxsmith-core` tests, `src-tauri`): grep confirms exactly the four impls the report inventories (`LiveIdentifier`, `FakeIdentWithExtensions`, `FakeIdent`, `OneIdent`), corroborating the "compiler-proven completeness" claim.

### Assessment
**Task quality:** Approved
**Reasoning:** Diff matches every binding constraint verbatim (required-method conversion, explicit `{ None }` bodies, doc trim, untouched impls), and the authorized grep independently confirms no fifth `Identify` impl exists to silently break or regress.
