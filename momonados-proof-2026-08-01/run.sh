#!/usr/bin/env bash
# Boot mOMonadOS. Needs qemu-system-x86_64 and nothing else.
set -euo pipefail
cd "$(dirname "$0")"
exec qemu-system-x86_64 -kernel momonados -m 256M -display none \
  -no-reboot -device isa-debug-exit,iobase=0xf4,iosize=4 -serial stdio
