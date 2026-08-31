#![allow(dead_code)]
#![allow(static_mut_refs)]
// Interrupt subsystem: IDT, PIC remap, PIT timer.
// All x86_64 crate types replaced with hand-rolled equivalents + inline asm.

use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// ─── Global tick counter ──────────────────────────────────────

pub static TICK_COUNTER: AtomicU64 = AtomicU64::new(0);
pub static TIMER_FIRED:  AtomicU64 = AtomicU64::new(0);

pub fn pending_ticks() -> u64 {
    TIMER_FIRED.swap(0, Ordering::Relaxed)
}

pub fn timer_ready() -> bool {
    TIMER_FIRED.load(Ordering::Relaxed) > 0
}

// ─── Inline port I/O ─────────────────────────────────────────

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") val,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    val
}

#[inline(always)]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nomem, nostack, preserves_flags)
    );
}

// ─── IDT types ───────────────────────────────────────────────

/// 16-byte IDT gate descriptor.
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low:  u16,
    selector:    u16,
    ist_zero:    u8,
    type_attr:   u8,  // 0x8E = 64-bit interrupt gate, DPL=0, present
    offset_mid:  u16,
    offset_high: u32,
    reserved:    u32,
}

impl IdtEntry {
    const fn absent() -> Self {
        Self {
            offset_low: 0, selector: 0, ist_zero: 0,
            type_attr: 0, offset_mid: 0, offset_high: 0, reserved: 0,
        }
    }
    fn from_fn(handler: u64, cs: u16) -> Self {
        Self {
            offset_low:  (handler & 0xFFFF) as u16,
            selector:    cs,
            ist_zero:    0,
            type_attr:   0x8E,
            offset_mid:  ((handler >> 16) & 0xFFFF) as u16,
            offset_high: ((handler >> 32) & 0xFFFF_FFFF) as u32,
            reserved:    0,
        }
    }
}

/// The CPU-facing IDTR structure (limit + base).
#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base:  u64,
}

/// Interrupt frame pushed by the CPU on interrupt entry.
#[repr(C)]
pub struct InterruptFrame {
    pub ip:    u64,
    pub cs:    u64,
    pub flags: u64,
    pub sp:    u64,
    pub ss:    u64,
}

fn read_cs() -> u16 {
    let cs: u16;
    unsafe {
        core::arch::asm!("mov {:x}, cs", out(reg) cs, options(nomem, nostack, preserves_flags));
    }
    cs
}

// ─── IDT ────────────────────────────────────────────────────

static mut IDT: [IdtEntry; 256] = [IdtEntry::absent(); 256];

pub fn init_idt() {
    let cs = read_cs();
    unsafe {
        IDT[3]    = IdtEntry::from_fn(breakpoint_handler   as *const () as usize as u64, cs);
        IDT[8]    = IdtEntry::from_fn(double_fault_handler as *const () as usize as u64, cs);
        IDT[0x20] = IdtEntry::from_fn(timer_handler        as *const () as usize as u64, cs);
        IDT[0x21] = IdtEntry::from_fn(keyboard_handler     as *const () as usize as u64, cs);
        let idtr = Idtr {
            limit: (256 * core::mem::size_of::<IdtEntry>() - 1) as u16,
            base:  IDT.as_ptr() as u64,
        };
        core::arch::asm!(
            "lidt [{ptr}]",
            ptr = in(reg) &idtr as *const Idtr as usize,
            options(nostack, readonly)
        );
    }
}

// ─── Exception handlers ──────────────────────────────────────

extern "x86-interrupt" fn breakpoint_handler(_: InterruptFrame) {
    // silent — used for debugging
}

extern "x86-interrupt" fn double_fault_handler(_: InterruptFrame, _: u64) -> ! {
    let msg = b"\n[DOUBLE FAULT]\n";
    for &b in msg {
        unsafe {
            while inb(0x3F8 + 5) & 0x20 == 0 {}
            outb(0x3F8, b);
        }
    }
    loop {
        unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); }
    }
}

// ─── IRQ handlers ────────────────────────────────────────────

extern "x86-interrupt" fn timer_handler(_: InterruptFrame) {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed);
    TIMER_FIRED.fetch_add(1, Ordering::Relaxed);
    // Send EOI to master PIC
    unsafe { outb(0x20, 0x20); }
}

extern "x86-interrupt" fn keyboard_handler(_: InterruptFrame) {
    unsafe {
        let scancode = inb(0x60);
        if scancode == 0x01 {
            ESCAPE_SEEN.store(true, Ordering::Relaxed);
        }
        outb(0x20, 0x20);  // EOI
    }
}

// ─── Escape detection ────────────────────────────────────────

static ESCAPE_SEEN: AtomicBool = AtomicBool::new(false);

pub fn escape_pressed() -> bool {
    if ESCAPE_SEEN.swap(false, Ordering::Relaxed) {
        return true;
    }
    if crate::serial::rx_ready() {
        crate::serial::read_byte();
        return true;
    }
    false
}

// ─── PIC remap ───────────────────────────────────────────────

pub fn remap_pic() {
    unsafe {
        let mask1 = inb(0x21);
        let mask2 = inb(0xA1);

        // ICW1
        outb(0x20, 0x11);
        outb(0xA0, 0x11);
        // ICW2: offsets
        outb(0x21, 0x20);
        outb(0xA1, 0x28);
        // ICW3
        outb(0x21, 0x04);
        outb(0xA1, 0x02);
        // ICW4
        outb(0x21, 0x01);
        outb(0xA1, 0x01);
        // Restore masks: enable IRQ0+IRQ1, mask everything else
        let _ = mask1;
        let _ = mask2;
        outb(0x21, 0xFC);
        outb(0xA1, 0xFF);
    }
}

// ─── PIT ─────────────────────────────────────────────────────

pub fn init_pit(hz: u32) {
    let hz = hz.clamp(18, 1000);
    let divisor = (1_193_182u32 / hz) as u16;
    unsafe {
        outb(0x43, 0x34);
        outb(0x40, (divisor & 0xFF) as u8);
        outb(0x40, ((divisor >> 8) & 0xFF) as u8);
    }
}

// ─── Init ────────────────────────────────────────────────────

pub fn init(hz: u32) {
    remap_pic();
    init_idt();
    init_pit(hz);
    unsafe {
        core::arch::asm!("sti", options(nostack, nomem, preserves_flags));
    }
}
