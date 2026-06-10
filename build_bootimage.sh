#!/usr/bin/env bash
# build_bootimage.sh — build mOMonadOS UEFI boot image
set -euo pipefail
cd "$(dirname "$0")"

PROFILE="${1:-release}"
TARGET="x86_64-unknown-none"
KERNEL_NAME="momonados"
OUT_DIR="target/${TARGET}/${PROFILE}"
KERNEL_ELF="${OUT_DIR}/${KERNEL_NAME}"
BOOT_DIR="target/uefi-boot"
ESP_DIR="${BOOT_DIR}/esp"
EFI_DIR="${ESP_DIR}/EFI/BOOT"

echo "═══ mOMonadOS UEFI Bootimage Builder ═══"
echo ""

# ── 1: Build kernel ELF ──────────────────────────────────────────
echo "[1/3] Building kernel (${PROFILE})..."
cargo build --profile "$PROFILE" --target "$TARGET" 2>&1 | grep -E 'Compiling|Finished|error' || true
[ ! -f "$KERNEL_ELF" ] && { echo "ERROR: $KERNEL_ELF not found"; exit 1; }
echo "  ✓ $(stat -c%s "$KERNEL_ELF") bytes"

# ── 2: Build UEFI bootloader (embeds kernel) ──────────────────────
echo "[2/3] Building UEFI bootloader..."
BL_SRC=""
for f in ~/.cargo/registry/src/*/bootloader-x86_64-uefi-0.11.*/Cargo.toml; do
    [ -f "$f" ] && { BL_SRC="$(dirname "$f")"; break; }
done
[ -z "$BL_SRC" ] && { echo "ERROR: bootloader-x86_64-uefi not found. Run 'cargo build' first."; exit 1; }

rm -rf "${BOOT_DIR}/bootloader-build"
mkdir -p "${BOOT_DIR}/bootloader-build"

KERNEL="$(pwd)/$KERNEL_ELF" \
KERNEL_MANIFEST="$(pwd)/Cargo.toml" \
KERNEL_DIRECTORY="$(pwd)" \
cargo build \
    --manifest-path "$BL_SRC/Cargo.toml" \
    --release \
    --target x86_64-unknown-uefi \
    --target-dir "$(pwd)/${BOOT_DIR}/bootloader-build" \
    2>&1 | grep -E 'Compiling|Finished|error' || true

BOOT_ELF="${BOOT_DIR}/bootloader-build/x86_64-unknown-uefi/release/bootloader-x86_64-uefi.efi"
[ ! -f "$BOOT_ELF" ] && { echo "ERROR: $BOOT_ELF not found"; exit 1; }
echo "  ✓ $(stat -c%s "$BOOT_ELF") bytes"

# ── 3: Create FAT32 ESP image ─────────────────────────────────────
echo "[3/3] Creating EFI system partition image..."
mkdir -p "$EFI_DIR"
cp "$BOOT_ELF" "${EFI_DIR}/BOOTX64.EFI"
cp "$KERNEL_ELF" "${ESP_DIR}/kernel-x86_64"

IMG="${OUT_DIR}/momonados.img"
dd if=/dev/zero of="$IMG" bs=1M count=64 2>/dev/null
mkfs.vfat -F 32 "$IMG" >/dev/null 2>&1 || {
    echo "  WARNING: mkfs.vfat failed — copy ${BOOT_ELF} to USB manually"
}

if command -v mcopy &>/dev/null; then
    mcopy -i "$IMG" -s "$ESP_DIR/EFI" "::EFI" 2>/dev/null || true
    mcopy -i "$IMG" -o "${ESP_DIR}/kernel-x86_64" "::" 2>/dev/null || true
    echo "  ✓ ESP image: $IMG"
else
    echo "  WARNING: mcopy not found (install mtools) — raw EFI binary at ${EFI_DIR}/BOOTX64.EFI"
fi

echo ""
echo "═══ Done ═══"
echo "  Disk image:  $IMG"
echo "  Run:         ./run.sh"
echo "  Serial only: ./run.sh --serial"
echo ""
