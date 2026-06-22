// boot.rs — Multiboot1 header + 32→64 bootstrap (Intel syntax)
//
// QEMU `-kernel` with multiboot1: CPU enters at `_start` in 32-bit
// protected mode with a flat GDT already loaded.  This stub sets up
// minimal identity-mapped page tables, enables PAE + long mode, loads
// a 64-bit GDT, far-jumps to `_start64`, reloads segments, then calls
// `_rust_start` (our Rust naked function that sets up RSP and heap).
//
// Intel syntax is required — Rust `global_asm!` uses LLVM's integrated
// assembler in Intel mode on x86_64.

use core::arch::global_asm;

global_asm!(r#"
/* ── PVH ELF note (XEN_ELFNOTE_PHYS32_ENTRY = 18) ────────────── */
/* QEMU's `-kernel` for 64-bit ELFs requires SHT_NOTE type.        */
/* Switch to AT&T syntax for `.section` so `@note` is recognized.  */
    .att_syntax prefix
    .section .note.Xen, "a", @note
    .intel_syntax noprefix
    .align 4
    .long   4               /* namesz: "Xen\0" */
    .long   4               /* descsz: one 32-bit address */
    .long   18              /* type: XEN_ELFNOTE_PHYS32_ENTRY */
    .ascii  "Xen\0"
    .long   _start          /* 32-bit entry point */

/* ── 32-bit protected-mode entry ─────────────────────────────── */
    .section .text.boot32, "ax"
    .code32
    .global _start
_start:
    cli

    /* PML4[0] → PDPT  (present + writable) */
    mov     eax, offset _boot_pdpt
    or      eax, 3
    mov     DWORD PTR [_boot_pml4], eax

    /* PDPT[0] → PD */
    mov     eax, offset _boot_pd
    or      eax, 3
    mov     DWORD PTR [_boot_pdpt], eax

    /* PD entries: four 2MB identity-mapped huge pages (covers 0–8MB)  */
    /* PS=bit2, RW=bit1, P=bit0; bit2=PS for huge pages in PDE         */
    mov     DWORD PTR [_boot_pd + 0x00], 0x000083
    mov     DWORD PTR [_boot_pd + 0x08], 0x200083
    mov     DWORD PTR [_boot_pd + 0x10], 0x400083
    mov     DWORD PTR [_boot_pd + 0x18], 0x600083

    /* CR3 = PML4 */
    mov     eax, offset _boot_pml4
    mov     cr3, eax

    /* Enable PAE (CR4 bit 5) */
    mov     eax, cr4
    or      eax, 0x20
    mov     cr4, eax

    /* Enable long mode in EFER MSR (0xC0000080 bit 8) */
    mov     ecx, 0xC0000080
    rdmsr
    or      eax, 0x100
    wrmsr

    /* Enable paging (CR0 bit 31); protected mode already active */
    mov     eax, cr0
    or      eax, 0x80000000
    mov     cr0, eax

    /* Load 64-bit GDT then far-jump to switch CS to the 64-bit selector */
    lgdt    [_boot_gdt64_ptr]
    /* ljmp $0x08, $addr — encoded as opcode 0xEA + 32-bit offset + 16-bit selector */
    .byte   0xEA
    .long   _start64
    .word   0x08

/* ── 64-bit GDT ─────────────────────────────────────────────── */
    .align 16
_boot_gdt64:
    .quad 0x0000000000000000     /* null descriptor */
    .quad 0x00AF9A000000FFFF     /* 64-bit code, DPL=0, L=1 */
    .quad 0x00AF92000000FFFF     /* 64-bit data, DPL=0 */
_boot_gdt64_ptr:
    .word (_boot_gdt64_ptr - _boot_gdt64 - 1)
    .long _boot_gdt64

/* ── 64-bit continuation ─────────────────────────────────────── */
    .code64
    .global _start64
_start64:
    /* Reload data segments with 64-bit data selector */
    mov     ax, 0x10
    mov     ds, ax
    mov     es, ax
    mov     ss, ax
    xor     eax, eax
    mov     fs, ax
    mov     gs, ax

    /* Hand off to the Rust naked entry that sets up RSP + heap */
    call    _rust_start
2:
    hlt
    jmp     2b

/* ── Bootstrap page tables (zeroed by multiboot loader) ──────── */
    .section .bss.boot, "aw", @nobits
    .align 0x1000
_boot_pml4: .space 0x1000
_boot_pdpt: .space 0x1000
_boot_pd:   .space 0x1000
"#);
