/* momonados.ld — Linker script for mOMonadOS
 *
 * Load address 0x200000: matches the default x86_64-unknown-none VirtAddr
 * and is within 32-bit address space so multiboot1 can place us here.
 *
 * Section order:
 *   .multiboot   — multiboot1 magic (must be in first 8KB of first LOAD)
 *   .text.boot32 — 32-bit entry stub (sets up page tables + enters long mode)
 *   .text        — normal 64-bit kernel code
 *   .rodata      — read-only data
 *   .data        — read-write data
 *   .bss         — zero-initialized (includes boot page tables + kernel heap)
 */

ENTRY(_start)

SECTIONS {
    . = 0x200000;

    /* PVH ELF note — QEMU reads this to find the 32-bit entry for 64-bit ELFs */
    .note.Xen : {
        KEEP(*(.note.Xen))
    }

    /* 32-bit bootstrap: page tables + long-mode transition */
    .text.boot32 : {
        KEEP(*(.text.boot32))
    }

    /* 64-bit kernel code */
    .text : ALIGN(16) {
        *(.text .text.*)
    }

    /* Read-only data */
    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }

    /* Initialized writable data */
    .data : ALIGN(4096) {
        *(.data .data.*)
    }

    /* Zero-initialized: kernel BSS + boot page tables + heap/stack */
    .bss : ALIGN(4096) {
        *(.bss .bss.*)
        *(.bss.boot)
        *(COMMON)
    }

    /DISCARD/ : {
        *(.comment)
        *(.note.gnu*)
        *(.note.ABI-tag)
        *(.eh_frame*)
        *(.debug*)
    }
}
