#!/usr/bin/env bash
# run_hosted_cmds.sh — feed REPL commands to the HOSTED binary, no QEMU.
#
# The non-interactive sibling of run_hosted.sh, and the counterpart of
# run_serial_cmds.sh: same arguments, same transcript, without the boot. A QEMU
# boot dominates the cost of any short command, which is why run_serial_cmds.sh
# tells callers to batch. Here there is nothing to amortise.
#
# Usage: ./run_hosted_cmds.sh "sic d16" "weight ⊢∈⊤⊡⊣" ...
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${PROFILE:-release}"
BIN="target/x86_64-unknown-linux-gnu/${PROFILE}/momonados"

if [ ! -x "$BIN" ]; then
  PROFILE_FLAG=()
  [ "$PROFILE" = "release" ] && PROFILE_FLAG=(--release)
  cargo build "${PROFILE_FLAG[@]}" --target x86_64-unknown-linux-gnu \
    --features hosted >&2
fi

{ for c in "$@"; do printf '%s\n' "$c"; done; printf 'quit\n'; } | "$BIN"
