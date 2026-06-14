#![allow(dead_code)]
use x86_64::instructions::port::Port;
use core::fmt;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        Port::<u8>::new(COM1 + 1).write(0x00); // disable interrupts
        Port::<u8>::new(COM1 + 3).write(0x80); // enable DLAB
        Port::<u8>::new(COM1 + 0).write(0x01); // baud divisor lo = 1 → 115200
        Port::<u8>::new(COM1 + 1).write(0x00); // baud divisor hi
        Port::<u8>::new(COM1 + 3).write(0x03); // 8N1, DLAB off
        Port::<u8>::new(COM1 + 2).write(0xC7); // FIFO on, clear, 14-byte threshold
        Port::<u8>::new(COM1 + 4).write(0x0B); // RTS/DSR
    }
}

#[inline]
fn tx_ready() -> bool {
    unsafe { Port::<u8>::new(COM1 + 5).read() & 0x20 != 0 }
}

#[inline]
pub fn rx_ready() -> bool {
    unsafe { Port::<u8>::new(COM1 + 5).read() & 0x01 != 0 }
}

/// Write a single byte, spin-waiting for TX ready.
pub fn write_byte(b: u8) {
    while !tx_ready() {}
    unsafe { Port::<u8>::new(COM1).write(b); }
}

pub fn read_byte() -> u8 {
    while !rx_ready() {}
    unsafe { Port::<u8>::new(COM1).read() }
}

/// FIFO-burst write: fill the 14-byte FIFO before re-checking TX ready.
/// Dramatically reduces spin-wait overhead for display output.
/// 14-byte threshold set in init() — we batch up to 14 bytes per burst.
pub fn write_str(s: &str) {
    let mut buf: [u8; 14] = [0; 14];
    let mut fill: usize = 0;

    let flush = |buf: &mut [u8; 14], fill: &mut usize| {
        if *fill == 0 { return; }
        // Wait once for TX ready, then burst the batch
        while !tx_ready() {}
        unsafe {
            let mut port = Port::<u8>::new(COM1);
            for i in 0..*fill {
                port.write(buf[i]);
            }
        }
        *fill = 0;
    };

    for b in s.bytes() {
        if b == b'\n' {
            flush(&mut buf, &mut fill);
            write_byte(b'\r');
            write_byte(b'\n');
            continue;
        }
        buf[fill] = b;
        fill += 1;
        if fill >= 14 {
            flush(&mut buf, &mut fill);
        }
    }
    flush(&mut buf, &mut fill);
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
