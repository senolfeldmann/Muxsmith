#!/usr/bin/env bash
# Version-sync guard (Plan 8, D87). Usage:
#   scripts/check-version-sync.sh          # consistency only
#   scripts/check-version-sync.sh vX.Y.Z   # consistency + tag equality
# Asserts: Cargo workspace version == package.json version;
# tauri.conf.json declares NO version (it inherits Cargo's);
# with an argument: the tag is exactly v<version>.
set -euo pipefail

fail() { echo "version-sync: $*" >&2; exit 1; }

cargo_v="$(awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml)"
[ -n "$cargo_v" ] || fail "could not parse [workspace.package] version from Cargo.toml"

pkg_v="$(jq -r .version package.json)"
[ "$cargo_v" = "$pkg_v" ] || fail "Cargo.toml ($cargo_v) != package.json ($pkg_v)"

tauri_has_v="$(jq 'has("version")' src-tauri/tauri.conf.json)"
[ "$tauri_has_v" = "false" ] || fail "src-tauri/tauri.conf.json declares 'version'; it must inherit from Cargo.toml (D87)"

if [ "$#" -ge 1 ]; then
  [ "$1" = "v$cargo_v" ] || fail "tag $1 != v$cargo_v"
fi

echo "version-sync: OK ($cargo_v)"
