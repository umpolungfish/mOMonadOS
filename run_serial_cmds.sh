#!/usr/bin/env bash
# Pipe REPL commands into mOMonadOS via -serial stdio.
# Usage: ./run_serial_cmds.sh "d2048 tower" "d2048 redei" ...
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${PROFILE:-release}"
ELF="target/x86_64-unknown-none/${PROFILE}/momonados"
[ -f "$ELF" ] || bash build_bootimage.sh "$PROFILE"

{
  sleep 2
  for cmd in "$@"; do
    echo "$cmd"
    sleep 0.5
  done
  echo quit
} | timeout 60 qemu-system-x86_64 \
  -kernel "$ELF" \
  -m 256M \
  -display none \
  -no-reboot \
  -device isa-debug-exit,iobase=0xf4,iosize=4 \
  -serial stdio 2>&1 | sed -n '/═══ d=2048/,/^⊙/p; /TOWER ASCENT/,/^⊙/p; /C16 LAYER/,/^⊙/p; /C32 = HILBERT/,/^⊙/p; /REDEI/,/^⊙/p; /NEXT EAGLE/,/^⊙/p'