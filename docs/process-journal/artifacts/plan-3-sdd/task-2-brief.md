### Task 2: generalize the matcher over a `Matchable` trait

**Files:**
- Modify: `crates/muxsmith-core/src/matcher.rs`

**Interfaces:**
- Consumes: `identify::{Track, PropValue}`, `Track::get`.
- Produces: `pub trait Matchable { fn get(&self, prop: &str) -> Option<PropValue>; }` (impl'd for `Track`); `pub fn matches<M: Matchable>(expr: &MatchExpr, item: &M, lang: &LanguageIndex) -> bool` (signature widened from `&Track` to `&M`). No behavior change for tracks: all existing `matcher` tests must still pass unchanged.

- [ ] **Step 1: Write the failing test.** Add a test that pins the trait exists and `matches` is generic by calling it through a generic helper:

```rust
#[test]
fn matches_is_generic_over_matchable() {
    fn check<M: Matchable>(m: &M) -> bool {
        matches(&expr("exact: { type: audio }"), m, &lang())
    }
    let t = track("audio", &[]);
    assert!(check(&t));
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core matcher` -> FAIL (`Matchable` undefined).

- [ ] **Step 3: Implement.** Define the trait, impl it for `Track` (delegating to the existing `Track::get`), and make `matches`, `exact_matches`, and the `track_str` helper generic over `M: Matchable`. Rename `track_str` to `item_str<M: Matchable>(prop, item)`. The language and `codec_kind` special cases in `exact_matches` stay: they consult `item.get(...)` and are simply never triggered for property sets that lack those names. The boolean-absent-false branch keeps consulting `matchable_type` (a track-schema fact); for a non-track item whose property is absent and not in the track table it yields `false`, which is correct.

```rust
pub trait Matchable {
    /// The value of a match property, or `None` if absent.
    fn get(&self, prop: &str) -> Option<PropValue>;
}

impl Matchable for Track {
    fn get(&self, prop: &str) -> Option<PropValue> { Track::get(self, prop) }
}
```

- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> all matcher + planner tests PASS (planner calls `matcher::matches` with `&Track`, still resolves via type inference).
- [ ] **Step 5: Gate + commit.** `refactor(matcher): generalize matches over a Matchable trait`.

---

