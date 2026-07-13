# Verify-23: golden argv tests' `.into_iter().map(String::from).collect::<Vec<_>>()` tails

**Verdict: CONFIRMED**

Finding: `crates/muxsmith-core/tests/command.rs`, 10 golden argv sites (asserts at lines 48, 398, 467, 565, 605, 635, 664, 702, 749, 798) compare `command(&plan)` against `vec![&str, ...].into_iter().map(String::from).collect::<Vec<_>>()`; replacement is direct comparison against the array literal.

## (a) Code says what the finding claims — yes

- `grep -n 'map(String::from)'` finds exactly 10 conversion tails (lines 66, 432, 487, 592, 622, 651, 682, 729, 778, 827), mapping 1:1 to the 10 claimed assert sites. Spot-read at 48-70, 398-404, 798-830 confirms the identical pattern at each: `assert_eq!(muxsmith_core::command::command(&plan), vec![...].into_iter().map(String::from).collect::<Vec<_>>())`.
- The finding is correctly scoped: the file's three other `assert_eq!` sites (157, 291, 342) compare an extracted `track_order: Option<&str>` — a different construct, properly excluded.
- `pub fn command(plan: &Plan) -> Vec<String>` (src/command.rs:55) confirms the LHS type.
- 10 sites x 3-line tail = 30 lines cut, matching `lines_cut: 30`.

## (b) Replacement is current idiom on the pinned toolchain — yes, empirically verified

Verified against the actual pinned toolchain (`rust-toolchain.toml` pins 1.96.1; system `rustc 1.96.1 (31fca3adb 2026-06-26)`; workspace `edition = "2024"`), not training memory:

- Compiled and ran a snippet with `fn command() -> Vec<String>` asserted directly against `["--output", "/out/e.mkv"]` via `rustc --edition 2024`: compiles, passes.
- Adversarial edge (largest site at line 798 has ~28 argv entries; the pre-const-generics std impls capped array PartialEq at 32): compiled and ran `Vec<String> == [&str; 40]` — passes. The impl is `impl<T: PartialEq<U>, U, const N: usize> PartialEq<[U; N]> for Vec<T>`, generic over N, so every site's length is covered.
- No inference obstacle: the array literal of `&'static str` on the `assert_eq!` RHS infers cleanly; both sides are `Debug`.

## (c) Duplication with load-bearing difference — n/a

Not a duplication finding; all 10 sites are the same mechanical conversion tail with no per-site variation beyond the argv contents, which stay untouched.

## (d) yagni without concrete construct — n/a

Tag is `idiom`, and both construct and replacement are concrete anyway.

## Decision guard — no conflict, not tracked

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `command.rs`, `golden argv`, `String::from`, `map(String`, `argv test`: no hits. Read the ROADMAP cosmetic-cleanup group K and the test-hygiene collection in full: group K's only command-test entry is the stale "Two tests:" module doc in `command_integration.rs` (different file, different issue); the B-minors list nothing about argv comparison style. No design memo records the conversion tail as deliberate.
