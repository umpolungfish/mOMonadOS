#!/usr/bin/env bash
# run.sh — boot mOMonadOS in QEMU directly from ELF (no OVMF, no disk image)
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${1:-release}"
ELF="target/x86_64-unknown-none/${PROFILE}/momonados"

if [ ! -f "$ELF" ]; then
    echo "No kernel ELF — building now..."
    bash build_bootimage.sh "$PROFILE"
fi

echo "mOMonadOS booting — serial on stdio (type 'quit' to exit cleanly)"
qemu-system-x86_64 \
    -kernel "$ELF" \
    -m 256M \
    -display none \
    -no-reboot \
    -device isa-debug-exit,iobase=0xf4,iosize=4 \
    -serial stdio
