#!/usr/bin/env bash
# build_bootimage.sh — build mOMonadOS kernel ELF (bare ELF, no UEFI)
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${1:-release}"
TARGET="x86_64-unknown-none"

echo "═══ mOMonadOS ELF Builder ═══"
cargo build --profile "$PROFILE" --target "$TARGET" 2>&1 | grep -E 'Compiling|Finished|error' || true

ELF="target/${TARGET}/${PROFILE}/momonados"
[ ! -f "$ELF" ] && { echo "ERROR: $ELF not found"; exit 1; }
echo "  ✓ $(stat -c%s "$ELF") bytes — $ELF"
echo "═══ Done — run with: ./run.sh $PROFILE ═══"
