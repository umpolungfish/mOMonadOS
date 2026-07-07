#!/usr/bin/env bash
# run.sh — boot mOMonadOS in QEMU directly from ELF (no OVMF, no disk image)
# Usage: ./run.sh [release|debug]          — serial on stdio (default)
#        ./run.sh --serial [release|debug] — explicit serial mode (Makefile target)
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="release"
if [ "${1:-}" = "--serial" ]; then
  PROFILE="${2:-release}"
elif [ "${1:-}" = "release" ] || [ "${1:-}" = "debug" ]; then
  PROFILE="$1"
fi

ELF="target/x86_64-unknown-none/${PROFILE}/momonados"

if [ ! -f "$ELF" ]; then
  echo "No kernel ELF — building now..."
  bash build_bootimage.sh "$PROFILE"
fi

echo "mOMonadOS booting — serial on stdio (type 'quit' to exit cleanly)"
exec qemu-system-x86_64 \
  -kernel "$ELF" \
  -m 256M \
  -display none \
  -no-reboot \
  -device isa-debug-exit,iobase=0xf4,iosize=4 \
  -serial stdio