#!/usr/bin/env bash
# Pipe REPL commands into mOMonadOS via -serial stdio.
# Usage: ./run_serial_cmds.sh "sic d16" "sic calibrate" ...
#        FILTER='/TOWER ASCENT/,/^⊙/p' ./run_serial_cmds.sh "d2048 tower"
# Output passes through whole unless FILTER is set to a sed program.
#
# Wrapped big integers inside one argv are rejoined by
# join_digit_continuations.awk before the serial feed. Each argv is joined
# alone so a line-broken N is one REPL command and holds never cross commands.
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${PROFILE:-release}"
ELF="target/x86_64-unknown-none/${PROFILE}/momonados"
JOIN_AWK="$(dirname "$0")/join_digit_continuations.awk"
[ -f "$ELF" ] || bash build_bootimage.sh "$PROFILE"

{
  sleep 2
  for cmd in "$@"; do
    printf '%s\n' "$cmd" | awk -f "$JOIN_AWK" | while IFS= read -r joined || [ -n "$joined" ]; do
      echo "$joined"
      sleep 0.5
    done
  done
  echo quit
# No timeout. A turn that takes minutes (vita bakes a neural turn on emulated
# bare metal) was being killed at 60s and read as a hang, which is the harness
# lying about the kernel. If a command genuinely wedges, Ctrl-C is the tool.
} | qemu-system-x86_64 \
    -kernel "$ELF" \
    -m 256M \
    -display none \
    -no-reboot \
    -device isa-debug-exit,iobase=0xf4,iosize=4 \
    -serial stdio 2>&1 | { if [ -n "${FILTER:-}" ]; then sed -n "$FILTER"; else cat; fi; }
