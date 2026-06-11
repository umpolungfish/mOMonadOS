// Interrupt subsystem: IDT, PIC remap, PIT timer.
// Drives continuous kernel execution with a programmable heartbeat.

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::instructions::port::Port;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

// ─── Global tick counter ──────────────────────────────────────

pub static TICK_COUNTER: AtomicU64 = AtomicU64::new(0);
pub static TIMER_FIRED: AtomicU64 = AtomicU64::new(0);

/// How many PIT interrupts have accumulated since last drain.
pub fn pending_ticks() -> u64 {
    TIMER_FIRED.swap(0, Ordering::Relaxed)
}

/// Non-blocking check: did the timer fire?
pub fn timer_ready() -> bool {
    TIMER_FIRED.load(Ordering::Relaxed) > 0
}

// ─── IDT ──────────────────────────────────────────────────────

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault.set_handler_fn(double_fault_handler);
        // Timer IRQ (IRQ0 → INT 0x20 after PIC remap)
        IDT[0x20].set_handler_fn(timer_handler);
        // Keyboard IRQ (IRQ1 → INT 0x21)
        IDT[0x21].set_handler_fn(keyboard_handler);
        IDT.load();
    }
}

// ─── Exception handlers ───────────────────────────────────────

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // silent — used for debugging
    let _ = _stack_frame;
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    // Can't use sprint! here (might deadlock) — write raw
    let msg = b"\n[DOUBLE FAULT]\n";
    for &b in msg {
        unsafe {
            while Port::<u8>::new(0x3F8 + 5).read() & 0x20 == 0 {}
            Port::<u8>::new(0x3F8).write(b);
        }
    }
    loop { x86_64::instructions::hlt(); }
}

// ─── IRQ handlers ─────────────────────────────────────────────

extern "x86-interrupt" fn timer_handler(__stack_frame: InterruptStackFrame) {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed);
    TIMER_FIRED.fetch_add(1, Ordering::Relaxed);

    // Send EOI to master PIC
    unsafe {
        Port::<u8>::new(0x20).write(0x20);
    }
}

extern "x86-interrupt" fn keyboard_handler(__stack_frame: InterruptStackFrame) {
    // Read scancode to clear the interrupt
    unsafe {
        let scancode: u8 = Port::new(0x60).read();

        // Sticky flag: ESC press (0x01) sets a flag that stays high until
        // escape_pressed() consumes it.  The release scancode (0x81) is
        // ignored so it can never overwrite the press.
        if scancode == 0x01 {
            ESCAPE_SEEN.store(true, Ordering::Relaxed);
        }
    }

    // Send EOI
    unsafe {
        Port::<u8>::new(0x20).write(0x20);
    }
}

// ─── Escape detection (for REPL escape from continuous mode) ──

static ESCAPE_SEEN: AtomicBool = AtomicBool::new(false);

/// Check if ESC was pressed (or any key in serial-console mode).
///
/// Checks two sources:
///   1. Keyboard ISR sticky flag (PS/2 port 0x60) — for graphical console.
///   2. Serial port RX (port 0x3F8) — for -nographic / -serial stdio setups
///      where keypresses arrive as UART bytes, not PS/2 scancodes.
/// Any pending serial byte is drained so the REPL does not see a stale key.
pub fn escape_pressed() -> bool {
    if ESCAPE_SEEN.swap(false, Ordering::Relaxed) {
        return true;
    }
    // Serial-console fallback: any keypress during continuous execution = stop
    if crate::serial::rx_ready() {
        crate::serial::read_byte(); // drain so REPL doesn't consume it
        return true;
    }
    false
}

// ─── PIC remap ────────────────────────────────────────────────

pub fn remap_pic() {
    unsafe {
        let mut cmd: Port<u8> = Port::new(0x20);
        let mut data: Port<u8> = Port::new(0x21);
        let mut slave_cmd: Port<u8> = Port::new(0xA0);
        let mut slave_data: Port<u8> = Port::new(0xA1);

        // Save masks
        let _mask1 = data.read();
        let _mask2 = slave_data.read();

        // ICW1: init, ICW4 needed
        cmd.write(0x11);
        slave_cmd.write(0x11);

        // ICW2: vector offsets — master at 0x20, slave at 0x28
        data.write(0x20);
        slave_data.write(0x28);

        // ICW3: master has slave on IRQ2 (bit 2), slave ID = 2
        data.write(0x04);
        slave_data.write(0x02);

        // ICW4: 8086 mode
        data.write(0x01);
        slave_data.write(0x01);

        // Restore masks — unmask only IRQ0 (timer) and IRQ1 (keyboard)
        data.write(0xFC);   // ~(1<<0 | 1<<1) — enable IRQ0, IRQ1
        slave_data.write(0xFF); // mask all slave IRQs
    }
}

// ─── PIT (Programmable Interval Timer) ──────────────────────

/// Initialize PIT channel 0 for periodic interrupts.
/// `hz` — desired frequency in Hz (clamped 18–1000).
pub fn init_pit(hz: u32) {
    let hz = hz.clamp(18, 1000);
    let divisor: u16 = (1_193_182u32 / hz) as u16;

    unsafe {
        let mut cmd: Port<u8> = Port::new(0x43);
        let mut ch0: Port<u8> = Port::new(0x40);

        // Channel 0, lobyte+hibyte, rate generator, binary
        cmd.write(0x34);

        // Divisor (low then high)
        ch0.write((divisor & 0xFF) as u8);
        ch0.write(((divisor >> 8) & 0xFF) as u8);
    }
}

// ─── Init everything ──────────────────────────────────────────

pub fn init(hz: u32) {
    remap_pic();
    init_idt();
    init_pit(hz);
    x86_64::instructions::interrupts::enable();
}