#!/usr/bin/env bash
# Usage: packaging/homebrew/update-cask.sh 0.1.0 mcgarrahjohn/quill
# Downloads the two published macOS installers, then writes their version and SHA-256 checksums into quill.rb.
set -euo pipefail
VERSION="${1:?version, for example 0.1.0}"; REPO="${2:?GitHub repo, for example you/quill}"
HERE="$(cd "$(dirname "$0")" && pwd)"; TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT
BASE="https://github.com/${REPO}/releases/download/v${VERSION}"
for A in aarch64 x64; do curl -fL --silent --show-error -o "$TMP/$A.dmg" "$BASE/Quill_${VERSION}_${A}.dmg"; done
ARM="$(shasum -a 256 "$TMP/aarch64.dmg" | cut -d' ' -f1)"; X64="$(shasum -a 256 "$TMP/x64.dmg" | cut -d' ' -f1)"
sed -i '' -E "s|^  version \".*\"|  version \"${VERSION}\"|; s|arm:   \".*\",|arm:   \"${ARM}\",|; s|intel: \".*\"|intel: \"${X64}\"|; s|github.com/[^/]+/[^/]+/releases|github.com/${REPO}/releases|; s|homepage \".*\"|homepage \"https://github.com/${REPO}\"|" "$HERE/quill.rb"
echo "Updated $HERE/quill.rb for v${VERSION}"; echo "arm64: $ARM"; echo "x64:   $X64"
