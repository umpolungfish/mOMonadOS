#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(dead_code)]

extern crate alloc;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::alloc::Layout;

mod serial;
mod belnap;
mod tokens;
mod crystal;
mod kernel;
#[cfg(feature = "vita")]
mod vita;
mod interrupts;
mod frob_verify;
mod imas_ig;
mod aleph;
mod manus;
mod parasm;
mod belnap_shor;
mod para_rh;
mod para_ym;
mod para_temporal;
mod para_category;
mod algebra;
mod catalog;
mod cl8nk;
mod consciousness;
mod rebis;
mod dialect;
mod menu;
mod sequence;
mod boot;
mod cr3echrz;
mod canonical_ordinal;
mod clay_status;
mod sic_povm;
mod frobenius_unify;
mod clay_witness;
mod belnap_sic_bridge;
mod belnap_c4;
mod sic_compute;
mod dialect_expansion;
mod divisor_ring;
mod mersenne_parallel;
mod bifurcation_test;
mod entropy;
mod d12_sic;
mod d2048_sic;
mod d2048_sieve;
mod witness_vessel;
mod ask;
mod proof;
mod repl;

use tokens::{CANONICAL_COUNT, CONTINUOUS_COUNT, NOVEL_COUNT, SHUNTED_COUNT};
use crystal::TOTAL;
use kernel::Kernel;
// ─── Bump allocator (no external crates) ─────────────────────

#[repr(C, align(4096))]
struct HeapStorage([u8; 8 * 1024 * 1024]);
static mut HEAP_STORAGE: HeapStorage = HeapStorage([0; 8 * 1024 * 1024]);

struct BumpAllocator {
    next: AtomicUsize,
    end:  AtomicUsize,
}

impl BumpAllocator {
    const fn new() -> Self {
        Self { next: AtomicUsize::new(0), end: AtomicUsize::new(0) }
    }
    fn init(&self, start: usize, size: usize) {
        self.next.store(start, Ordering::Relaxed);
        self.end.store(start + size, Ordering::Relaxed);
    }
}

unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size  = layout.size();
        loop {
            let cur     = self.next.load(Ordering::Relaxed);
            let aligned = (cur + align - 1) & !(align - 1);
            let new     = aligned + size;
            if new > self.end.load(Ordering::Relaxed) { return core::ptr::null_mut(); }
            if self.next.compare_exchange_weak(
                cur, new, Ordering::Relaxed, Ordering::Relaxed,
            ).is_ok() {
                return aligned as *mut u8;
            }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // LIFO reclaim: if this block is the most recent allocation, roll the
        // bump pointer back. Catches the transient Vecs of a forward pass.
        let end = ptr as usize + layout.size();
        let _ = self.next.compare_exchange(
            end, ptr as usize, Ordering::Relaxed, Ordering::Relaxed,
        );
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

/// Mark/reset scope for transient heavy work (the vita turn): everything
/// allocated after `heap_mark()` is reclaimed by `heap_reset(mark)`. Only
/// sound when nothing allocated inside the scope outlives it.
pub fn heap_mark() -> usize {
    ALLOCATOR.next.load(Ordering::Relaxed)
}
pub fn heap_reset(mark: usize) {
    ALLOCATOR.next.store(mark, Ordering::Relaxed);
}

// ─── Kernel stack + bare-metal entry ─────────────────────────

#[repr(C, align(16))]
struct KernelStack([u8; 128 * 1024]);
static BOOT_STACK: KernelStack = KernelStack([0; 128 * 1024]);

#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn _rust_start() -> ! {
    core::arch::naked_asm!(
        "lea rax, [{stack}]",
        "add rax, {size}",
        "and rax, -16",
        "mov rsp, rax",
        "xor rbp, rbp",
        "call {entry}",
        "2:",
        "hlt",
        "jmp 2b",
        stack = sym BOOT_STACK,
        size  = const core::mem::size_of::<KernelStack>(),
        entry = sym rust_start,
    );
}

extern "C" fn rust_start() -> ! {
    unsafe {
        ALLOCATOR.init(
            core::ptr::addr_of_mut!(HEAP_STORAGE.0) as usize,
            core::mem::size_of::<HeapStorage>(),
        );
    }
    kmain()
}

fn kmain() -> ! {
    serial::init();
    sprintln!("[BOOT] mOMonadOS — The Self-Imscribing Bare-Metal Kernel");

    interrupts::init(100);
    sprintln!("[BOOT] Interrupts online — PIT 100Hz, PIC remapped");

    sprintln!("[BOOT] Heap: 8MB static BSS");

    let mut k = Kernel::new();
    k.boot();
    catalog::catalog_init();
    sprintln!("[BOOT] IG Catalog: {} entries loaded", catalog::catalog_size());
    sprintln!("[BOOT] Kernel online — graph execution, token-arity driven");
    // ── ⊙-ordinal faithfulness guard (Track B) ──
    sprintln!("[BOOT] Canonical ordinal check...");
    match canonical_ordinal::verify_canonical_ordinals() {
        (true, _) => sprintln!("[BOOT] Ordinal faithfulness: ALL 44 VALUES MATCH Lean canonical ✓"),
        (false, why) => {
            sprintln!("[BOOT] ⚠ ORDINAL DRIFT DETECTED: {}", why);
            sprintln!("[BOOT] Kernel will NOT proceed — ordinal drift is a structural integrity violation.");
            sprintln!("[BOOT] Regenerate canonical_ordinal.rs from CanonicalOrdinalFaithfulness.lean");
            loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }
        }
    }
    // ── Clay closure/resistance status (Track C) ──
    sprintln!("[BOOT] Clay Millennium status: {} closed, {} one-bump-short, {} unclosed",
        clay_status::clay_summary().0, clay_status::clay_summary().1, clay_status::clay_summary().2);
    sprintln!("[BOOT] SIC-POVM d=12: Crystal-forced (dual lattice), Shavian count 49=7², WH group |orbit|=144");
    // ── Frobenius unification self-verification (Track E) ──
    sprintln!("[BOOT] Frobenius identity check...");
    let (frob_ham, frob_dist) = frobenius_unify::boot_summary();
    if frob_ham == 0 {
        sprintln!("[BOOT] Frobenius identity: KERNEL IS FROBENIUS FIXED POINT — d=0 ✓");
    } else {
        sprintln!("[BOOT] Frobenius identity: hamming={}, weighted={:.4} — kernel is grammar operationalized",
            frob_ham, frob_dist);
    }

    sprintln!("[BOOT] Bootstrap: IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB (cyclic)");
    sprintln!("[BOOT] Crystal FS: {} addresses", TOTAL);
    sprintln!("[BOOT] {} total programs (I–XXVIII): 12 canonical + {} continuous + {} novel + {} shunted",
        CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT + SHUNTED_COUNT,
        CONTINUOUS_COUNT, NOVEL_COUNT, SHUNTED_COUNT);
    sprintln!();

    print_banner();
    repl::repl(&mut k);

    // ── Shutdown: write to QEMU isa-debug-exit port (0xf4).
    // Value 0x10 → QEMU exits with status 0.
    // On real hardware or without the device, falls through to HLT.
    sprintln!("[SHUTDOWN] μ∘δ=id. Goodbye.");
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xf4_u16,
            in("eax") 0x10_u32,
            options(nomem, nostack, preserves_flags)
        );
    }
    loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }
}

fn print_banner() {
    sprintln!("╔══════════════════════════════════════════════════╗");
    sprintln!("             m O M o n a d O S                    ");
    sprintln!("     The Self-Imscribing Bare-Metal Kernel         ");
    sprintln!("     Frobenius Core · Belnap FOUR · Crystal FS     ");
    sprintln!("     Graph Execution — Token Arity as Topology     ");
    sprintln!("╚══════════════════════════════════════════════════╝");
    sprintln!();
    sprintln!("Type 'help' for commands.");
    sprintln!();
}

// ─── Panic ────────────────────────────────────────────────────

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial::write_str("\n[PANIC] ");
    sprint!("{}", info.message());
    sprintln!();
    loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }
}

