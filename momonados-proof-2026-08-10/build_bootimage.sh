#!/usr/bin/env bash
# build_bootimage.sh — build mOMonadOS kernel ELF (bare ELF, no UEFI)
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${1:-release}"
TARGET="x86_64-unknown-none"

echo "═══ mOMonadOS ELF Builder ═══"
# The exit status has to come from cargo, not from grep. Piping and then
# appending `|| true` discarded it, so a failed compile printed its errors and
# the script still said Done — over whatever stale ELF was already on disk.
set -o pipefail
cargo build --profile "$PROFILE" --target "$TARGET" \
  -Z build-std=core,compiler_builtins,alloc \
  -Z build-std-features=compiler-builtins-mem \
  2>&1 | grep -E 'Compiling|Finished|error'
status=${PIPESTATUS[0]}
if [ "$status" -ne 0 ]; then
  echo "ERROR: cargo build failed (status $status) — the ELF on disk is stale"
  exit "$status"
fi

ELF="target/${TARGET}/${PROFILE}/momonados"
[ ! -f "$ELF" ] && { echo "ERROR: $ELF not found"; exit 1; }
echo "  ✓ $(stat -c%s "$ELF") bytes — $ELF"
echo "═══ Done — run with: ./run.sh $PROFILE ═══"
