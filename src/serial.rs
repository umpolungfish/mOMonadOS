#![allow(dead_code)]
use core::fmt;

const COM1: u16 = 0x3F8;

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

pub fn init() {
    unsafe {
        outb(COM1 + 1, 0x00); // disable interrupts
        outb(COM1 + 3, 0x80); // enable DLAB
        outb(COM1 + 0, 0x01); // baud divisor lo = 1 → 115200
        outb(COM1 + 1, 0x00); // baud divisor hi
        outb(COM1 + 3, 0x03); // 8N1, DLAB off
        outb(COM1 + 2, 0xC7); // FIFO on, clear, 14-byte threshold
        outb(COM1 + 4, 0x0B); // RTS/DSR
    }
}

#[inline]
fn tx_ready() -> bool {
    unsafe { inb(COM1 + 5) & 0x20 != 0 }
}

#[inline]
pub fn rx_ready() -> bool {
    unsafe { inb(COM1 + 5) & 0x01 != 0 }
}

pub fn write_byte(b: u8) {
    while !tx_ready() {}
    unsafe { outb(COM1, b); }
}

pub fn read_byte() -> u8 {
    while !rx_ready() {}
    unsafe { inb(COM1) }
}

/// FIFO-burst write: fill the 14-byte FIFO before re-checking TX ready.
/// Closures can't call `unsafe fn` directly; use a standalone flush_buf().
fn flush_buf(buf: &[u8; 14], fill: usize) {
    if fill == 0 { return; }
    while !tx_ready() {}
    unsafe {
        for i in 0..fill { outb(COM1, buf[i]); }
    }
}

pub fn write_str(s: &str) {
    let mut buf: [u8; 14] = [0; 14];
    let mut fill: usize = 0;
    for b in s.bytes() {
        if b == b'\n' {
            flush_buf(&buf, fill);
            fill = 0;
            write_byte(b'\r');
            write_byte(b'\n');
            continue;
        }
        buf[fill] = b;
        fill += 1;
        if fill >= 14 {
            flush_buf(&buf, fill);
            fill = 0;
        }
    }
    flush_buf(&buf, fill);
}

pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::serial::Writer, $($arg)*);
    }};
}

#[macro_export]
macro_rules! sprintln {
    () => { $crate::sprint!("\n") };
    ($($arg:tt)*) => { $crate::sprint!("{}\n", format_args!($($arg)*)) };
}
