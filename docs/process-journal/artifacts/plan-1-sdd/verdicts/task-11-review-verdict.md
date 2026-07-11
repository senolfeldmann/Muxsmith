<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-11  (round 1 of 1)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01Pf58SXsjX8oFBucF2axCnS
  agent_id:           a45aac9899af0b716
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a45aac9899af0b716.jsonl
  dispatch_desc:      Review Task 11 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-08T00:20:44.242Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

Full compliance, verified independently (not just by reading the diff):

- **Dependencies**: `clap` (derive), `schemars`, `serde_json` under `[dependencies]`; `assert_cmd`, `predicates` under `[dev-dependencies]` in `crates/muxsmith-cli/Cargo.toml`. Correct crate, correct sections. `schemars`/`serde_json` versions match `muxsmith-core`'s existing pins.
- **CLI surface**: `cli.rs` defines `Cli { command: Cmd }` with `Cmd::Validate { profile: PathBuf, json: bool, locale: Option<String> }` and `Cmd::Schema`, matching the brief verbatim.
- **`validate` stub**: built the binary and ran it directly (`muxsmith validate somefile.yaml`). Confirmed empty stdout, empty stderr, exit code 2. No hardcoded prose anywhere in that arm — just a source comment (not printed).
- **`schema`**: ran `muxsmith schema` directly; output is valid pretty JSON with `"title": "Profile"`, exit 0.
- **No-args behavior**: ran with no args; clap emits its own library-generated usage text to stderr (subcommand descriptions come from doc comments, which the brief explicitly permits as library-generated help text), exit code 2.
- **ASCII**: all new Rust source is ASCII-only.
- **Tests**: ran `cargo test -p muxsmith-cli --test cli_schema` standalone (2 passed) and `cargo test --workspace`. Summed the per-crate results myself: 0+2+34+6+14+10+0+0+2+0+0 = **68**, matching the controller's verified figure, not the implementer's claimed 78.

### Strengths

- Implementation is a faithful, unembellished translation of the brief; no scope creep, no invented behavior.
- Dev-only test dependencies correctly kept out of `[dependencies]`, avoiding bloating the release binary.
- The "print nothing on validate" constraint was actually the easiest place to slip in something like a TODO `eprintln!` — it wasn't; verified at the byte level via direct invocation, not just diff-reading.

### Issues

#### Minor

1. **Unused dev-dependency (`predicates`)** — added per the brief's own Step 1 (`cargo add ... --dev assert_cmd predicates`), but `tests/cli_schema.rs` only uses `assert_cmd`; `predicates` is dead weight. **Plan-mandated**, not an implementer deviation — flagging for whoever owns the brief in case Task 12 was expected to consume it and doesn't end up needing it either.
2. **Report test-count error** — the report claims "78 passed" for the full workspace; actual is 68 (confirmed independently by summing per-crate results, matching the controller's figure). Doesn't affect the code, but the report's own verification section is wrong on a number it explicitly cites as evidence.
3. **Schema test assertion is a substring match on stringified JSON**, not a structural check against `schema["properties"]` — mandated verbatim by the brief's Step 2 test code, so **plan-mandated**, not an implementer shortcut.

### Assessment

**Task quality:** Approved
**Reasoning:** Every binding constraint (CLI surface, no-hardcoded-prose stub, schema output, dependency placement) is met and independently verified by direct binary execution, not just diff inspection; the only findings are minor and either brief-mandated or cosmetic (report miscount).