# Audit: ci-10-pin-everything (PROMOTION candidate)

- **Cluster:** ci-10-pin-everything - "Pin everything exactly; a floating version is a defect"
- **Claimed:** kind=pattern, status=settled, count=3, promoted=yes (at 3)
- **Audited:** 2026-07-13 (adversarial, per-occurrence, against primary artifacts in the repo - not against the backfill's own find-E records)
- **Verdict: CONFIRMED** - 3 of 3 occurrences survive; promotion stands.

The standing rule is a real, thrice-recurring doctrine backed by committed code and config, not a padded count. Each occurrence resolves to a distinct date, distinct primary artifact set, and distinct sub-event; none is fabricated, misattributed, or a duplicate of another.

---

## Occurrence 1 - 2026-07-10 (reinforced) - SURVIVES

**Ref:** commits `2ee2d0c`, `45e941a` / journal Plan 5

- `git show 2ee2d0c`: real. SHA-pins `actions/checkout@9c091bb` (v7.0.0), `Swatinem/rust-cache@c193711` (v2.9.1), `EmbarkStudios/cargo-deny-action@bb137d7` (v2.0.20); pins runner images `ubuntu-26.04 / windows-2025 / macos-15` (was `-latest`); adds a "Pinning policy (2026-07-10)" comment block. Matches the statement's "all GitHub Actions SHA-pinned, runner images pinned".
- `git show 45e941a`: real. `rust-toolchain.toml` `channel = "stable"` -> `"1.96.1"` with a "pin-everything policy" comment; `ctrlc = "3"` -> `"3.5.2"`. Matches "rust 1.96.1 over floating stable, ctrlc full-pinned". Commit body: "Pin-everything policy (Şenol 2026-07-10)".
- `docs/process-journal.md:381`: real, verbatim to the cited evidence - "pin-everything (rust 1.96.1 over floating stable, all CI actions SHA-pinned, runners pinned incl. ubuntu-26.04 preview at his call, ctrlc full-pin)."
- `.npmrc`: `save-exact=true` under the same "Pin-everything policy (Şenol 2026-07-10)" comment - corroborates the statement's "npm save-exact".

Genuine event. **Note (not a drop):** the kind label "reinforced" is loose - the find-E records (E6 #54, E8 #168) call this the *origin* ("Origin Plan 5, Şenol's pin-everything doctrine, session 5/6"), so "decided" would be more accurate. This mislabel does not meet any drop criterion (not fabricated, not misattributed to a wrong artifact, not a duplicate); the artifact fully supports the topic arising here.

## Occurrence 2 - 2026-07-11 (decided) - SURVIVES

**Ref:** plan T2 step 2 / progress.md T2-m3/m4 / whole-branch funnel

- `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md:92` (T2 Step 2): real, verbatim to evidence - "Şenol's standing preference is pin, so default to `choco install mkvtoolnix --version=X` and brew formula pin unless impossible; document either way in a ci.yml comment".
- `docs/process-journal/artifacts/plan-5.5-sdd/progress.md` T2-m3/m4: real. m3 = "apt exact-build pin 97.0-1build1 fragile against archive rebuilds"; m4 = "cross-leg mkvmerge version divergence (apt 97 vs choco/brew 100); sanctioned by per-manager pin policy, recorded."
- `whole-branch-verdict.md` roll-up funnel: real. T2-m4 "DISCARD | sanctioned by the per-manager pin decision, recorded in ci.yml".
- **Execution verified in `.github/workflows/ci.yml`:** apt `mkvtoolnix=97.0-1build1`, `choco install mkvtoolnix --version=100.0.0`, brew floats (documented as the one manager with no install-time version selector). The apt-97 / choco-brew-100 divergence is recorded in the ci.yml comment block (lines 34-47). Matches the statement in full.

Distinct decision event (mkvmerge CI versions, per-manager), not a restatement of occ 1. Correctly labeled "decided".

**Precision nit on the standing rule (not an occurrence failure):** the statement's "mkvmerge CI versions pinned per package manager" and "choco/brew 100" imply brew is pinned; the executed reality is that brew *cannot* be pinned (no versioned core formula) and floats on latest (currently 100). ci.yml documents this honestly. The underlying decision ("pin where the manager allows") is real and executed - the rule text just slightly overstates brew. Worth tightening in CONVENTIONS.md if the wording is ever revisited; does not affect the count.

## Occurrence 3 - 2026-07-12 (reinforced) - SURVIVES

**Ref:** CONVENTIONS.md Patterns (`b38a46f`)

- `git show b38a46f`: real. Creates `docs/CONVENTIONS.md` with the pattern "**Pin everything.** Exact version pins (toolchain, JS deps), SHA-pinned GitHub Actions; a floating version is a defect, not a convenience." Verbatim to evidence.
- The three cited idiomacy reinforcements all exist in `.superpowers/sdd/idiomacy-review/find-F7.md` (F7 slice = build/config/toolchain + CI, dated 2026-07-12 review):
  - F7-1: `rustup show` as toolchain installer is the pre-1.28 no-op idiom ("rustup-show non-install"). ✓
  - F7-2: `resolver = "2"` on an all-edition-2024 workspace whose default is `"3"` ("resolver 2 vs edition-2024 default 3"). ✓
  - F7-6: `version = 2` dead key in `deny.toml` ("deny.toml dead version key"). ✓

Distinct event on a distinct date. **Circularity check:** occ 3's primary artifact (CONVENTIONS.md) is also the promotion target, so counting the codification act alone toward its own promotion would be weakly circular. It survives regardless, because it carries an *independent* reinforcement leg: the 2026-07-12 idiomacy review independently touched the pinning/toolchain-config surface (the three F7 findings above), a genuine recurrence of the topic on that date beyond the mere act of writing the rule down. Not a duplicate of occ 1 or occ 2.

---

## Distinctness / anti-padding checks

- Three occurrences -> three distinct source records across three backfill eras (E6 #54, record `[25]`, E8 #46), three distinct dates, three distinct primary-artifact sets. No two collapse into one event.
- The backfill's own clustering note (`cluster-ci.md:134,223`) transparently discloses that the mkvmerge per-manager record `[25]` was filed under ci-10 rather than ci-08; the filing is defensible (its evidence leads with "Şenol's standing preference is pin") and does not double-count - `[25]` appears only once, under ci-10.
- Even under maximal strictness (discount occ 3's codification portion as self-referential), the doctrine still has independent corroboration: the contrasting mise-action restraint (ci-09, 2026-07-12: "CI must not fetch a floating mise binary ... contradicting the pin-everything doctrine") attests the doctrine was live and load-bearing that same day.

## Result

- **verified_count = 3** (all occurrences survive)
- **Verdict: CONFIRMED** (>=3 survive -> promotion stands; ci-10 remains a Tier-2 CONVENTIONS.md pattern)
- Two non-blocking notes for the next CONVENTIONS.md edit: (1) occ 1's kind is really "decided/origin", not "reinforced"; (2) the rule text overstates brew as pinned when it in fact floats-unpinnable-by-design.
