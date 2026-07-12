### Task 16: UnknownPropertySkew forward-compat path (#4, gated on D32)

**Files:**
- Modify: `crates/muxsmith-core/src/profile/validate.rs:345` (UnknownProperty hard-reject becomes D32's shape), `crates/muxsmith-core/src/matcher.rs` (untyped comparison path), catalog messages
- Test: validate + matcher tests per D32's decided shape

- [ ] Failing tests per D32's acceptance criteria (written into D32 itself at decision time - the memo must contain concrete test cases as part of the design round). Implement; spec §9.2 gets amended to match the decided semantics exactly; spec self-contradiction sweep after amending (doctrine §1).
- [ ] Full gate; commit `feat(core): reachable forward-compat path for unknown properties (D32, #4)`.

