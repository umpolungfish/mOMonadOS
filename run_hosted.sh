#!/usr/bin/env bash
# run_hosted.sh — build and run the REPL directly on this machine, no QEMU,
# no bare metal. The one command for "just get me to the ⊙> prompt."
#
# Plain `cargo build` targets x86_64-unknown-none (bare metal, see
# .cargo/config.toml) even for a quick local check, which is not runnable
# here — that confusion is exactly what this script exists to remove.
#
# Usage: ./run_hosted.sh [release|debug] [extra,cargo,features]
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${1:-release}"
FEATURES="hosted${2:+,$2}"

PROFILE_FLAG=()
[ "$PROFILE" = "release" ] && PROFILE_FLAG=(--release)

cargo build "${PROFILE_FLAG[@]}" --target x86_64-unknown-linux-gnu --features "$FEATURES"
exec "target/x86_64-unknown-linux-gnu/${PROFILE}/momonados"
