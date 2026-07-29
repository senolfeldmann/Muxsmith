#!/usr/bin/env python3
"""Structural integrity check for the four house-knowledge YAML files and BUILDING.md.

Enforces the invariants the ledger (doctrine section 7) rests on but that no
tool checked until now - they were held by controller care alone, which is
exactly the state that ends badly once: a `reinforced` occurrence was appended
to core-47-with-severity-builder on 2026-07-13 without bumping the derived
count, and it survived every hand-check until an ad-hoc validator found it on
2026-07-16 (the day this script was written). See the ledger-lint ROADMAP entry.

Checks, per the ROADMAP spec plus duplicate-id (silent shadowing):
  1. count == len(occurrences)        - the anti-fabrication core; count is
                                         DERIVED, so any mismatch is a defect.
  2. every occurrence carries a ref    - no empty/absent citation.
  3. status: blocked => blocked_on set - a blocked entry names its blocker.
  4. tier: 2 => promoted_at is not null - a promoted entry records its count.
  5. id is unique across all four files - a duplicate silently shadows.
  6. no duplicate key in any mapping  - YAML's later-key-wins swallows a
                                         doubled field (observed: a second
                                         `steelman:` inside one entry).
  7. BUILDING.md gate total == blocks - the canonical "The pre-push gate is N
                                         parts: N Rust, N frontend, N
                                         house-knowledge." sentence is compared
                                         against the commands the three marked
                                         gate blocks enumerate, per block and in
                                         total. The `<!-- gate-block: ... -->`
                                         and `<!-- gate-total; ... -->` markers
                                         are the anchor because heading prose is
                                         not stable: headings in that file get
                                         reworded, and a heading match would
                                         then break silently.

Exit 0 clean, 1 on any violation.

House-pattern note: scripts/ otherwise holds check-i18n.mjs (Node), which is
deliberately line-based because no Fluent parser exists. That reason does NOT
transfer to YAML - a real parser exists (PyYAML), and a linter whose whole job
is to be trusted must not itself be a fragile line parser. So this is Python,
a deliberate and recorded divergence, not an oversight. Requires PyYAML; run
with the project's mise-managed python (`python3 scripts/ledger-lint.py`).
CI wiring exists: the ci.yml `ledger-lint` job runs this script on every master
push, `v*` tag and pull request (Plan 8 rider, ROADMAP 'Ledger hygiene' ruling
2026-07-22).
"""

import re
import sys
from pathlib import Path

try:
    import yaml
except ModuleNotFoundError:
    sys.exit("ledger-lint: PyYAML is required (pip install pyyaml, or use the mise python)")

# Tier-2 nature files + the Tier-1 ledger. Paths are resolved from the repo
# root (this script's parent's parent) so the check runs from any cwd.
REPO = Path(__file__).resolve().parent.parent
FILES = [
    "docs/conventions.yaml",
    "docs/process-conventions.yaml",
    "docs/product-boundaries.yaml",
    "docs/decision-ledger.yaml",
]

# Check 7's anchors. HTML comments rather than headings: a heading in BUILDING.md
# gets reworded (the Rust gate's own heading was, by the change that added this
# check), and a heading match would break silently where a marker match cannot.
BUILDING = "BUILDING.md"
GATE_BLOCKS = [
    ("rust", "<!-- gate-block: rust; checked by scripts/ledger-lint.py -->"),
    ("frontend", "<!-- gate-block: frontend; checked by scripts/ledger-lint.py -->"),
    ("house", "<!-- gate-block: house; checked by scripts/ledger-lint.py -->"),
]
GATE_TOTAL_MARKER = "<!-- gate-total; checked by scripts/ledger-lint.py -->"
GATE_TOTAL_RE = re.compile(
    r"^The pre-push gate is (\d+) parts: (\d+) Rust, (\d+) frontend, (\d+) house-knowledge\."
)


class DuplicateKeyLoader(yaml.SafeLoader):
    """SafeLoader that records duplicate mapping keys instead of losing them.

    `construct_mapping` is PyYAML's documented extension point for this: the
    raw mapping node still carries every key node with its line mark, while
    the dict built from it keeps only the last value per key - which is
    exactly how a doubled field passes unnoticed. Duplicates are collected,
    not raised, so one run reports every duplicate in the file. Applies to
    every mapping at any depth, which is a superset of per-entry scoping and
    simpler than tracking entry boundaries in the loader.
    """

    def __init__(self, stream):
        super().__init__(stream)
        self.duplicate_keys = []  # (key, first line, duplicate line), 1-based

    def construct_mapping(self, node, deep=False):
        first_line = {}
        for key_node, _value_node in node.value:
            key = self.construct_object(key_node, deep=deep)
            try:
                previous = first_line.get(key)
            except TypeError:  # unhashable key (a mapping/sequence key)
                continue
            line = key_node.start_mark.line + 1
            if previous is None:
                first_line[key] = line
            else:
                self.duplicate_keys.append((key, previous, line))
        return super().construct_mapping(node, deep=deep)


def _next_non_empty(lines: list[str], start: int) -> int | None:
    """Index of the first line at or after `start` that is not blank, else None."""
    for i in range(start, len(lines)):
        if lines[i].strip():
            return i
    return None


def _count_block_commands(
    name: str, lines: list[str], marker_idx: int, violations: list[str]
) -> int | None:
    """Command lines of one marked gate block, or None when the count is not derivable.

    A counted command line is a non-blank line inside the fence that does not
    start with `#`. None is returned for every shape where counting is
    impossible - no opening ```bash fence, or a fence that is never closed - and
    the caller then skips this block's comparison AND the total, because a
    partial sum is not a total. The continuation guard is deliberately NOT one of
    those cases: a backslash-continued line is reported and the count is still
    returned, so the run shows both the guard and the miscount it prevents.
    """
    opener = _next_non_empty(lines, marker_idx + 1)
    if opener is None or lines[opener].strip() != "```bash":
        found = "end of file" if opener is None else repr(lines[opener].strip())
        violations.append(
            f"{BUILDING}: gate-block '{name}': marker is not followed by an "
            f"opening ```bash fence (found {found})"
        )
        return None

    commands = 0
    for i in range(opener + 1, len(lines)):
        stripped = lines[i].strip()
        if stripped == "```":
            return commands
        if stripped.endswith("\\"):
            violations.append(
                f"{BUILDING}: gate-block '{name}': line {i + 1} ends with a "
                "backslash; this check does not model shell continuations, so "
                "the block must enumerate one command per line"
            )
        if stripped and not stripped.startswith("#"):
            commands += 1

    violations.append(
        f"{BUILDING}: gate-block '{name}': the fence opened at line "
        f"{opener + 1} is never closed"
    )
    return None


def check_building_gate_total(violations: list[str]) -> None:
    """Check 7: BUILDING.md's canonical gate total against its enumerated blocks.

    The file is the gate's single authoritative enumeration and used to state a
    per-section count with no total, leaving every reader to assemble one. It now
    states the total once; this compares that sentence against the commands the
    three marked gate blocks actually list, per block and in sum, so the stated
    number cannot drift from the file's own enumeration.
    """
    try:
        lines = (REPO / BUILDING).read_text().splitlines()
    except FileNotFoundError:
        violations.append(f"{BUILDING}: file not found")
        return

    stripped = [line.strip() for line in lines]

    counted: dict[str, int | None] = {}
    for name, marker in GATE_BLOCKS:
        hits = [i for i, s in enumerate(stripped) if s == marker]
        if len(hits) != 1:
            violations.append(
                f"{BUILDING}: gate-block '{name}': expected exactly one marker "
                f"line '{marker}', found {len(hits)}"
            )
            counted[name] = None
            continue
        counted[name] = _count_block_commands(name, lines, hits[0], violations)

    # With no stated numbers there is nothing to compare against, so a missing or
    # unparseable canonical sentence is one violation that stands alone.
    total_hits = [i for i, s in enumerate(stripped) if s == GATE_TOTAL_MARKER]
    if len(total_hits) != 1:
        violations.append(
            f"{BUILDING}: expected exactly one gate-total marker line "
            f"'{GATE_TOTAL_MARKER}', found {len(total_hits)}"
        )
        return

    sentence = _next_non_empty(lines, total_hits[0] + 1)
    match = GATE_TOTAL_RE.match(stripped[sentence]) if sentence is not None else None
    if match is None:
        found = "end of file" if sentence is None else repr(stripped[sentence])
        violations.append(
            f"{BUILDING}: the gate-total marker is not followed by the canonical "
            "sentence 'The pre-push gate is N parts: N Rust, N frontend, "
            f"N house-knowledge.' (found {found})"
        )
        return

    stated_total = int(match.group(1))
    stated = {
        "rust": int(match.group(2)),
        "frontend": int(match.group(3)),
        "house": int(match.group(4)),
    }

    for name, _marker in GATE_BLOCKS:
        if counted[name] is not None and counted[name] != stated[name]:
            violations.append(
                f"{BUILDING}: gate-block '{name}' states {stated[name]} commands "
                f"but enumerates {counted[name]}"
            )

    # A block whose count is not derivable makes the counted total underivable
    # too; comparing the stated total against a partial sum would report a second
    # violation that does not name the cause.
    if any(counted[name] is None for name, _marker in GATE_BLOCKS):
        return
    summed = sum(counted[name] for name, _marker in GATE_BLOCKS)
    if stated_total != summed:
        violations.append(
            f"{BUILDING}: gate-total states {stated_total} parts but the three "
            f"gate blocks enumerate {summed}"
        )


def main() -> int:
    violations = []
    seen_ids = {}  # id -> file where first defined

    for rel in FILES:
        path = REPO / rel
        try:
            text = path.read_text()
        except FileNotFoundError:
            violations.append(f"{rel}: file not found")
            continue

        loader = None
        try:
            loader = DuplicateKeyLoader(text)
            doc = loader.get_single_data()
        except yaml.YAMLError as exc:
            violations.append(f"{rel}: does not parse ({exc})")
            continue
        finally:
            if loader is not None:
                loader.dispose()

        # 6. no duplicate key in any mapping
        for key, first, dup in loader.duplicate_keys:
            violations.append(f"{rel}: duplicate key '{key}' (lines {first} and {dup})")

        entries = (doc or {}).get("entries")
        if not isinstance(entries, list):
            violations.append(f"{rel}: no top-level 'entries' list")
            continue

        for idx, entry in enumerate(entries):
            eid = entry.get("id", f"<entry #{idx}, no id>")

            # 5. unique id across all files
            if eid in seen_ids:
                violations.append(f"{rel}: id '{eid}' also defined in {seen_ids[eid]}")
            else:
                seen_ids[eid] = rel

            # 1. count == len(occurrences)
            occ = entry.get("occurrences") or []
            count = entry.get("count")
            if count != len(occ):
                violations.append(
                    f"{rel}: {eid}: count is {count} but has {len(occ)} occurrences"
                )

            # 2. every occurrence carries a ref
            for j, o in enumerate(occ):
                if not (isinstance(o, dict) and o.get("ref")):
                    violations.append(f"{rel}: {eid}: occurrence #{j} has no ref")

            # 3. blocked => blocked_on
            if entry.get("status") == "blocked" and not entry.get("blocked_on"):
                violations.append(f"{rel}: {eid}: status blocked without a blocked_on")

            # 4. tier 2 => promoted_at set
            if entry.get("tier") == 2 and entry.get("promoted_at") is None:
                violations.append(f"{rel}: {eid}: tier 2 but promoted_at is null")

    check_building_gate_total(violations)

    total = len(seen_ids)
    if violations:
        for v in violations:
            print(f"FAIL {v}")
        print(f"\nledger-lint: {len(violations)} violation(s) across {total} entries")
        return 1

    print(
        f"ledger-lint: {total} entries across {len(FILES)} files plus "
        "BUILDING.md's gate enumeration, all invariants hold"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
