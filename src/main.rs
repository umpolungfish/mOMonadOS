#![cfg_attr(not(feature = "hosted"), no_std)]
// The IG primitives are Shavian glyphs and the glyph is the name; spelling them
// in Latin would be a different notation, so the lint is answered rather than
// obeyed.
#![allow(uncommon_codepoints)]
#![cfg_attr(not(feature = "hosted"), no_main)]
#![cfg_attr(not(feature = "hosted"), feature(abi_x86_interrupt))]
#![cfg_attr(not(feature = "hosted"), feature(alloc_error_handler))]
#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::approx_constant)]
#![allow(clippy::eq_op)]

// Needed in both builds: the portable modules use alloc::vec::Vec. On a host
// this resolves against the std-provided alloc, which is why build-std must be
// off for the hosted target -- rebuilding alloc there duplicates its lang items.
extern crate alloc;
#[cfg(not(feature = "hosted"))]
use core::panic::PanicInfo;
#[cfg(not(feature = "hosted"))]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(feature = "hosted"))]
use core::alloc::Layout;

mod serial;
#[macro_use]
mod style;
mod belnap;
mod tokens;
mod crystal;
mod kernel;
#[cfg(feature = "vita")]
mod vita;
#[cfg(not(feature = "hosted"))]
mod interrupts;
#[cfg(feature = "hosted")]
#[path = "interrupts_hosted.rs"]
mod interrupts;
mod frob_verify;
mod imas_ig;
mod aleph;
mod manus;
mod parasm;
mod belnap_shor;
mod belnap_shor_factors;
mod fibonacci_shor;
mod belnap_ring_shor;
mod belnap_phase_shor;
mod para_rh;
mod para_ym;
mod para_temporal;
mod para_category;
mod algebra;
mod canonical_ig;
mod catalog;
mod cl8nk;
mod consciousness;
mod rebis;
mod dialect;
mod menu;
mod sequence;
// The multiboot1 stub and 32->64 bit trampoline. A hosted process is entered
// by the host loader, which has already done all of it.
#[cfg(not(feature = "hosted"))]
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
mod quadratic;
mod sic_moduli;
mod riemann_sic;
mod riemann_hilbert;
mod witness_vessel;
mod ask;
mod proof;
mod seals;
mod constant_closure;
mod repl;
mod fibonacci_qc;
mod winding_period;
mod lattice_flow;
mod triple_frame;
mod iuft_qc;
mod iuft_teichmuller;

// ── m3iosis tool ports (native Rust implementations) ────────
mod stark;
mod hqe;
mod dyson;
mod afdmc;
mod troq;
mod hop;
mod braid_grammar;
mod braid_render;
mod braid_protocol;
mod text;
mod manifold;
mod kernel_torus;
mod ovm;

use tokens::{canonical_count, continuous_count, novel_count, shunted_count};
use crystal::TOTAL;
use kernel::Kernel;
// ─── Bump allocator (no external crates) ─────────────────────

#[cfg(not(feature = "hosted"))]
#[repr(C, align(4096))]
struct HeapStorage([u8; 48 * 1024 * 1024]);
#[cfg(not(feature = "hosted"))]
static mut HEAP_STORAGE: HeapStorage = HeapStorage([0; 48 * 1024 * 1024]);

#[cfg(not(feature = "hosted"))]
struct BumpAllocator {
    next: AtomicUsize,
    end:  AtomicUsize,
}

#[cfg(not(feature = "hosted"))]
impl BumpAllocator {
    const fn new() -> Self {
        Self { next: AtomicUsize::new(0), end: AtomicUsize::new(0) }
    }
    fn init(&self, start: usize, size: usize) {
        self.next.store(start, Ordering::Relaxed);
        self.end.store(start + size, Ordering::Relaxed);
    }
}

#[cfg(not(feature = "hosted"))]
unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size  = layout.size();
        // A zero-size request must never fail. With the heap nearly full the
        // alignment round-up alone can push past `end`, and returning null for a
        // zero-byte allocation surfaced as "memory allocation of 0 bytes failed"
        // — a panic that named a size nobody had asked for. Hand back a dangling
        // but correctly aligned pointer, which is what a zero-size allocation is.
        if size == 0 {
            return align as *mut u8;
        }
        loop {
            let cur     = self.next.load(Ordering::Relaxed);
            let aligned = (cur + align - 1) & !(align - 1);
            let new     = aligned + size;
            // Null here reaches `alloc_error` below, which reports the real
            // layout. Nothing is printed at this point so a caller that
            // handles the null itself is not made to look like a crash.
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

#[cfg(not(feature = "hosted"))]
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

/// Mark/reset scope for transient heavy work (the vita turn): everything
/// allocated after `heap_mark()` is reclaimed by `heap_reset(mark)`. Only
/// sound when nothing allocated inside the scope outlives it.
#[cfg(not(feature = "hosted"))]
pub fn heap_mark() -> usize {
    ALLOCATOR.next.load(Ordering::Relaxed)
}
#[cfg(not(feature = "hosted"))]
pub fn heap_reset(mark: usize) {
    ALLOCATOR.next.store(mark, Ordering::Relaxed);
}
/// Bytes currently handed out, and the arena total. A bump allocator that has
/// run out returns null and the allocation error path takes the kernel down
/// without a message, so anything heavy should report this rather than let
/// exhaustion look like a hang.
#[cfg(not(feature = "hosted"))]
pub fn heap_used() -> (usize, usize) {
    let next = ALLOCATOR.next.load(Ordering::Relaxed);
    let end = ALLOCATOR.end.load(Ordering::Relaxed);
    let start = unsafe { core::ptr::addr_of!(HEAP_STORAGE.0) as usize };
    (next.saturating_sub(start), end.saturating_sub(start))
}

// ─── Kernel stack + bare-metal entry ─────────────────────────

#[cfg(not(feature = "hosted"))]
#[repr(C, align(16))]
struct KernelStack([u8; 128 * 1024]);
#[cfg(not(feature = "hosted"))]
static BOOT_STACK: KernelStack = KernelStack([0; 128 * 1024]);

#[cfg(not(feature = "hosted"))]
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

#[cfg(not(feature = "hosted"))]
extern "C" fn rust_start() -> ! {
    unsafe {
        ALLOCATOR.init(
            core::ptr::addr_of_mut!(HEAP_STORAGE.0) as usize,
            core::mem::size_of::<HeapStorage>(),
        );
    }
    kmain()
}

// ── Hosted runtime ───────────────────────────────────────────────────
// On a host the OS supplies what the bare-metal half builds by hand: an
// allocator, a stack, a panic path, and an entry point. So there is nothing to
// port here, only to step aside for.

// The host allocator is wrapped so its use is measured rather than assumed.
// An earlier version returned a constant 48 MB here on the grounds that it kept
// size decisions identical across builds. That is a hardcode standing in for a
// measurement, which is the thing this project exists to not do: every value
// should be computed. `used` below is now real, counted allocation by
// allocation, and works anywhere Rust runs.
#[cfg(feature = "hosted")]
mod hosted_heap {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use std::alloc::{GlobalAlloc, Layout, System};

    pub static USED: AtomicUsize = AtomicUsize::new(0);

    /// Not a measurement and not pretending to be. A host has no fixed arena,
    /// so there is no total to read; this is a declared ceiling that the
    /// heavy-job guards compare against, defaulting to the bare-metal arena so
    /// the two builds refuse the same work. Settable at runtime.
    pub static BUDGET: AtomicUsize = AtomicUsize::new(48 * 1024 * 1024);

    pub struct Counting;

    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 {
            let p = System.alloc(l);
            if !p.is_null() { USED.fetch_add(l.size(), Ordering::Relaxed); }
            p
        }
        unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
            System.dealloc(p, l);
            USED.fetch_sub(l.size(), Ordering::Relaxed);
        }
    }
}

#[cfg(feature = "hosted")]
#[global_allocator]
static ALLOCATOR: hosted_heap::Counting = hosted_heap::Counting;

/// A mark is a real position in the measured total, so callers can see what a
/// scope cost. Reset cannot roll a system allocator back and does not pretend
/// to -- the allocator reclaims on drop, which is the same outcome by another
/// route.
#[cfg(feature = "hosted")]
pub fn heap_mark() -> usize {
    hosted_heap::USED.load(core::sync::atomic::Ordering::Relaxed)
}
#[cfg(feature = "hosted")]
pub fn heap_reset(_mark: usize) {}

#[cfg(feature = "hosted")]
pub fn heap_used() -> (usize, usize) {
    use core::sync::atomic::Ordering;
    (hosted_heap::USED.load(Ordering::Relaxed),
     hosted_heap::BUDGET.load(Ordering::Relaxed))
}

#[cfg(feature = "hosted")]
fn main() {
    kmain()
}

fn kmain() -> ! {
    serial::init();

    // The banner used to claim a PIT and a PIC remap on both builds. Hosted,
    // neither happens: the host owns the IDT. Saying so is not decoration --
    // a boot line asserting hardware it never touched is how a wrong finding
    // gets sourced later.
    interrupts::init(100);
    #[cfg(not(feature = "hosted"))]
    sprintln!("{}[boot]{} Interrupts online — PIT 100Hz, PIC remapped", style::muted(), style::reset());
    #[cfg(feature = "hosted")]
    sprintln!("{}[boot]{} Interrupts: none — the host owns the IDT, no periodic slot", style::muted(), style::reset());

    {
        let (used, total) = heap_used();
        #[cfg(not(feature = "hosted"))]
        sprintln!("{}[boot]{} Heap: {}MB static BSS", style::muted(), style::reset(), total / (1024 * 1024));
        #[cfg(feature = "hosted")]
        sprintln!("{}[boot]{} Heap: host allocator, {} bytes counted, {}MB declared budget", style::muted(), style::reset(),
                  used, total / (1024 * 1024));
        let _ = used;
    }

    let mut k = Kernel::new();
    k.boot();
    catalog::catalog_init();
    sprintln!("{}[boot]{} IG Catalog: {} entries loaded", style::muted(), style::reset(), catalog::catalog_size());
    sprintln!("{}[boot]{} Kernel online — graph execution, token-arity driven", style::muted(), style::reset());
    // ── ⊙-ordinal faithfulness guard (Track B) ──
    sprintln!("{}[boot]{} Canonical ordinal check...", style::muted(), style::reset());
    match canonical_ordinal::verify_canonical_ordinals() {
        (true, _) => sprintln!("{}[boot]{} Ordinal faithfulness: {}all 44 values match Lean canonical{}",
            style::muted(), style::reset(), style::verdict_t(), style::reset()),
        (false, why) => {
            sprintln!("{}[boot]{} ⚠ ORDINAL DRIFT DETECTED: {}", style::muted(), style::reset(), why);
            sprintln!("{}[boot]{} Kernel will NOT proceed — ordinal drift is a structural integrity violation.", style::muted(), style::reset());
            sprintln!("{}[boot]{} Regenerate canonical_ordinal.rs from CanonicalOrdinalFaithfulness.lean", style::muted(), style::reset());
            loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }
        }
    }
    // ── Clay closure/resistance status (Track C) ──
    sprintln!("{}[boot]{} Clay Millennium status: {} closed, {} one-bump-short, {} unclosed", style::muted(), style::reset(),
        clay_status::clay_summary().0, clay_status::clay_summary().1, clay_status::clay_summary().2);
    // Every figure on this line is read from the constants that define it. It
    // used to be typed into the format string -- the d, the 49, the 7, the 144
    // -- so the banner could have gone on asserting a structure the code had
    // stopped having.
    sprintln!("{}[boot]{} SIC-POVM d={}: Crystal-forced (dual lattice), Shavian count {}={}², WH group |orbit|={}", style::muted(), style::reset(),
        sic_povm::TOTAL_PRIMS, sic_povm::SHAVIAN_COUNT,
        sic_povm::SHAVIAN_ROOT, sic_povm::WH_GROUP_ORDER);
    // ── Frobenius unification self-verification (Track E) ──
    sprintln!("{}[boot]{} Frobenius identity check...", style::muted(), style::reset());
    let (frob_ham, frob_dist) = frobenius_unify::boot_summary();
    if frob_ham == 0 {
        sprintln!("{}[boot]{} Frobenius identity: KERNEL IS FROBENIUS FIXED POINT — d=0 ✓", style::muted(), style::reset());
    } else {
        sprintln!("{}[boot]{} Frobenius identity: hamming={}, weighted={:.4} — kernel is grammar operationalized", style::muted(), style::reset(),
            frob_ham, frob_dist);
    }

    sprintln!("{}[boot]{} Bootstrap: IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB (cyclic)", style::muted(), style::reset());
    sprintln!("{}[boot]{} Fibonacci anyon QC: algebra verified = {}", style::muted(), style::reset(), fibonacci_qc::verify_all());
    // ── Kernel torus winding display (Track A) ──
    let torus_map = kernel_torus::TorusMap::new(&kernel_torus::agent_loop_program());
    kernel_torus::display_banner(&torus_map);
    sprintln!("{}[boot]{} Crystal FS: {} addresses", style::muted(), style::reset(), TOTAL);
    sprintln!("{}[boot]{} {} total programs (I–XXIX): 12 canonical + {} continuous + {} novel + {} shunted", style::muted(), style::reset(),
        canonical_count() + continuous_count() + novel_count() + shunted_count(),
        continuous_count(), novel_count(), shunted_count());
    sprintln!();

    print_banner();
    repl::repl(&mut k);

    // ── Shutdown: write to QEMU isa-debug-exit port (0xf4).
    // Value 0x10 → QEMU exits with status 0.
    // On real hardware or without the device, falls through to HLT.
    sprintln!("[SHUTDOWN] μ∘δ=id. Goodbye.");

    // `out` and `hlt` are privileged. On bare metal they are the QEMU
    // debug-exit device and the idle halt; in a userspace process they are a
    // general protection fault, which is what made the hosted build segfault on
    // quit. A host does not need a debug-exit device emulated at it -- it
    // exits.
    #[cfg(not(feature = "hosted"))]
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xf4_u16,
            in("eax") 0x10_u32,
            options(nomem, nostack, preserves_flags)
        );
    }
    #[cfg(not(feature = "hosted"))]
    loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }

    #[cfg(feature = "hosted")]
    std::process::exit(0);
}

fn print_banner() {
    // The mark, the name, then what it is. This is the one screen where the
    // reader has no context yet, so it leads with the object and not the
    // feature list.
    sprintln!();
    sprintln!("   {}⊙{}   {}mOMonadOS{}", style::glyph(), style::reset(),
              style::heading(), style::reset());
    #[cfg(not(feature = "hosted"))]
    sprintln!("   {}the self-imscribing bare-metal kernel{}", style::muted(), style::reset());
    #[cfg(feature = "hosted")]
    sprintln!("   {}hosted build, on the host's runtime{}", style::muted(), style::reset());
    sprintln!("   {}μ∘δ = id{}", style::accent(), style::reset());
    sprintln!();
    sprintln!("   {}Frobenius core · Belnap FOUR · crystal FS · graph execution{}",
              style::muted(), style::reset());
    sprintln!();
    sprintln!("   {}help{} for commands, {}?{} for the menu, Tab completes.",
              style::key(), style::reset(), style::key(), style::reset());
    sprintln!();
}

// ─── Panic ────────────────────────────────────────────────────

#[cfg(not(feature = "hosted"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial::write_str("\n[PANIC] ");
    sprint!("{}", info.message());
    sprintln!();
    loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }
}

/// Report heap exhaustion ourselves.
///
/// The default handler reaches the panic through `__rust_alloc_error_handler`,
/// an internal symbol that takes the size and align as loose integers. Under
/// this build — `build-std` with fat LTO — that call arrives with a size of
/// zero however large the failed request was: an allocation of 1264 bytes
/// against 552 free reported itself as "memory allocation of 0 bytes failed",
/// which sent every reader hunting for a zero-size bug that does not exist.
///
/// Taking the `Layout` here reads it directly instead of through that symbol,
/// and the writes below never allocate, which matters when the reason we are
/// here is that allocation just failed.
#[cfg(not(feature = "hosted"))]
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    let (used, total) = heap_used();
    serial::write_str("\n[PANIC] heap exhausted — wanted ");
    serial::write_dec(layout.size());
    serial::write_str(" bytes (align ");
    serial::write_dec(layout.align());
    serial::write_str("), ");
    serial::write_dec(total.saturating_sub(used));
    serial::write_str(" free of ");
    serial::write_dec(total);
    serial::write_str("\n");
    loop { unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); } }
}

