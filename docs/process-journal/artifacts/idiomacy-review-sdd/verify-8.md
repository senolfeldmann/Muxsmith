# Verify-8: planner.rs fully-qualified paths despite existing imports

**Finding (F1a, idiom):** `std::collections::BTreeSet/BTreeMap` fully qualified at planner.rs 1646, 1825, 1846, 1903, 1952 and `crate::identify::Track/PropValue` at 1653, 1665-1680, 1717, 1998-2003, despite imports at lines 7 and 15. Replacement: add `PropValue, Track` to the line-15 import, use bare names everywhere.

## Verdict: CONFIRMED

### (a) Code says what the finding claims - verified at HEAD

- Line 7: `use std::collections::{BTreeMap, BTreeSet};` - both types already imported.
- Line 15: `use crate::identify::{Attachment, Identification, Identify};` - the module is already imported from; `Track`/`PropValue` merely missing from the list.
- Qualified occurrences match the cited lines exactly: `std::collections::BTreeSet` at 1646-1647, `std::collections::BTreeMap` at 1825, 1846, 1903, 1952-1953; `crate::identify::Track` at 1653; `crate::identify::PropValue` at 1665, 1672, 1676, 1680, 1717, 1998, 2000-2003.
- The file is internally inconsistent: bare `BTreeMap`/`BTreeSet` are used in 10+ places (506, 778, 793, 1113, 1158, 1177, 1429, ...), including line 1491 `base_sig: &BTreeMap<String, usize>` - the exact same signature shape that 1825/1846/1903 write fully qualified.

### (b) Replacement is current idiom - verified

- No name collision blocks the bare names: grep finds no local `Track`, `PropValue`, or BTree type in planner.rs (the only capitalized near-hits are `DiagCode::MissingTrack` and `TrackRule`, both distinct identifiers), and no glob import besides the test module's `use super::*`, which would inherit the added names without conflict.
- Sibling file matcher.rs:12 already does exactly what the replacement proposes: `use crate::identify::{Attachment, PropValue, Track};` with bare usage. Repo-wide, every other core file imports `std::collections::BTreeMap` and uses it bare.
- Current external confirmation (not training memory): clippy ships the `absolute_paths` lint precisely because fully-qualified paths where a `use` statement fits are unidiomatic ([Clippy lints](https://rust-lang.github.io/rust-clippy/master/index.html)); the module-path idiom of grouped `use` statements is the documented convention since the path-clarity RFC ([RFC 2126](https://rust-lang.github.io/rfcs/2126-path-clarity.html)). Nothing in edition 2024 / Rust 1.96 reverses this; the 2024 style-edition changes concern import ordering/formatting, not qualification.

### (c) Duplication with load-bearing difference - n/a

Not a duplication finding.

### (d) yagni without concrete construct - n/a

Tag is `idiom`, and both construct and replacement are concrete.

### Decision guard - no hit

- `docs/superpowers/specs/*.md` (D-memos): no mention of qualified paths, `prop_value_as`, `diag_signature`, `no_regression`, or import style.
- `docs/ROADMAP.md`: cosmetic-cleanup sweep group K lists six items; the only planner.rs entry there is the eager chapters/attachments resolve at planner.rs:541ff - unrelated. Test-hygiene collection, deliberate-restraint and deferred entries: no import/qualification item.
- `docs/IDEAS.md`: no hit.

Not tracked anywhere, contradicts no recorded decision.

### Note on claimed metrics

`lines_cut: 2` is plausible: 1646-1647 collapse to one line once `std::collections::` drops, and 1952-1953 likewise (`diag_signature`'s return type plus `BTreeMap::new()`); the wrapped signatures at 1825/1846/1903 shorten to the shape line 1491 already has.
