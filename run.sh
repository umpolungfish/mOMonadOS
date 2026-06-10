#!/usr/bin/env bash
# run.sh — boot mOMonadOS in QEMU
set -euo pipefail
cd "$(dirname "$0")"

IMG="target/x86_64-unknown-none/release/momonados.img"

if [ ! -f "$IMG" ]; then
    echo "No boot image — building now..."
    bash build_bootimage.sh
fi

# Find OVMF firmware
OVMF_CODE=""
for p in \
    "/usr/share/OVMF/OVMF_CODE.fd" \
    "/usr/share/edk2-ovmf/x64/OVMF_CODE.fd" \
    "/usr/share/qemu/edk2-x86_64-code.fd" \
    "/usr/share/ovmf/OVMF.fd"
do
    [ -f "$p" ] && { OVMF_CODE="$p"; break; }
done
[ -z "$OVMF_CODE" ] && OVMF_CODE="$(find /usr/share -name 'OVMF_CODE*.fd' -o -name 'edk2-x86_64-code.fd' 2>/dev/null | head -1)"
[ -z "$OVMF_CODE" ] && {
    echo "ERROR: OVMF not found. Install with:"
    echo "  Ubuntu/Debian: sudo apt install ovmf"
    echo "  Arch:          sudo pacman -S edk2-ovmf"
    echo "  Fedora:        sudo dnf install edk2-ovmf"
    exit 1
}

# Find OVMF_VARS alongside OVMF_CODE
OVMF_VARS="${OVMF_CODE/CODE/VARS}"
[ ! -f "$OVMF_VARS" ] && OVMF_VARS="${OVMF_CODE}"

LOCAL_VARS="./.ovmf_vars.fd"
if [ ! -w "$OVMF_VARS" ] || [ "$OVMF_VARS" = "$OVMF_CODE" ]; then
    cp -f "$OVMF_VARS" "$LOCAL_VARS" 2>/dev/null || true
    OVMF_VARS="$LOCAL_VARS"
fi

SERIAL_MODE="${1:-}"

if [ "$SERIAL_MODE" = "--serial" ]; then
    echo "mOMonadOS booting — serial mode (Ctrl-A X to quit)"
    qemu-system-x86_64 \
        -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
        -drive if=pflash,format=raw,file="$OVMF_VARS" \
        -drive format=raw,file="$IMG" \
        -m 256M \
        -display none \
        -no-reboot \
        -serial stdio
else
    echo "mOMonadOS booting — serial on pty, display off (use --serial for stdio)"
    qemu-system-x86_64 \
        -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
        -drive if=pflash,format=raw,file="$OVMF_VARS" \
        -drive format=raw,file="$IMG" \
        -m 256M \
        -display none \
        -no-reboot \
        -serial stdio
fi
