#!/usr/bin/env python3
"""Structural integrity check for the four house-knowledge YAML files.

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

Exit 0 clean, 1 on any violation.

House-pattern note: scripts/ otherwise holds check-i18n.mjs (Node), which is
deliberately line-based because no Fluent parser exists. That reason does NOT
transfer to YAML - a real parser exists (PyYAML), and a linter whose whole job
is to be trusted must not itself be a fragile line parser. So this is Python,
a deliberate and recorded divergence, not an oversight. Requires PyYAML; run
with the project's mise-managed python (`python3 scripts/ledger-lint.py`).
CI wiring exists: the ci.yml `ledger-lint` job runs this script on every push
and pull request (Plan 8 rider, ROADMAP 'Ledger hygiene' ruling 2026-07-22).
"""

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

        loader = DuplicateKeyLoader(text)
        try:
            doc = loader.get_single_data()
        except yaml.YAMLError as exc:
            violations.append(f"{rel}: does not parse ({exc})")
            continue
        finally:
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

    total = len(seen_ids)
    if violations:
        for v in violations:
            print(f"FAIL {v}")
        print(f"\nledger-lint: {len(violations)} violation(s) across {total} entries")
        return 1

    print(f"ledger-lint: {total} entries across {len(FILES)} files, all invariants hold")
    return 0


if __name__ == "__main__":
    sys.exit(main())
