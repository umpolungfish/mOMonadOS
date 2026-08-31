#![allow(dead_code)]
use core::fmt;

const COM1: u16 = 0x3F8;

#[inline(always)]
#[cfg(not(feature = "hosted"))]
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
#[cfg(not(feature = "hosted"))]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nomem, nostack, preserves_flags)
    );
}

#[cfg(not(feature = "hosted"))]
/// The byte the host handed the UART before the guest configured it.
///
/// `qemu -serial stdio` delivers into the receive register immediately, so a
/// piped or fast-typed first character is already waiting when `init` runs --
/// and enabling the FIFO discards it, which made every first command arrive a
/// letter short (`banked ...` as `anked ...`). Reading it out before touching
/// FCR preserves it; `read_byte` hands it back before going to the port, so
/// the character reaches the line reader in its proper place.
///
/// The high bit distinguishes a stored NUL from an empty slot.
static PENDING: core::sync::atomic::AtomicU16 =
    core::sync::atomic::AtomicU16::new(0);

#[cfg(not(feature = "hosted"))]
pub fn init() {
    unsafe {
        // Probe before configuring: did the host hand us a byte already?
        if inb(COM1 + 5) & 0x01 != 0 {
            let b = inb(COM1);
            PENDING.store(0x100 | b as u16, core::sync::atomic::Ordering::Relaxed);
        }
        outb(COM1 + 1, 0x00); // disable interrupts
        outb(COM1 + 3, 0x80); // enable DLAB
        outb(COM1 + 0, 0x01); // baud divisor lo = 1 -> 115200
        outb(COM1 + 1, 0x00); // baud divisor hi
        outb(COM1 + 3, 0x03); // 8N1, DLAB off
        outb(COM1 + 2, 0xC7); // FIFO on, clear, 14-byte threshold
        outb(COM1 + 4, 0x0B); // RTS/DSR
    }
}

#[inline]
#[cfg(not(feature = "hosted"))]
fn tx_ready() -> bool {
    unsafe { inb(COM1 + 5) & 0x20 != 0 }
}

#[inline]
#[cfg(not(feature = "hosted"))]
pub fn rx_ready() -> bool {
    if PENDING.load(core::sync::atomic::Ordering::Relaxed) != 0 { return true; }
    unsafe { inb(COM1 + 5) & 0x01 != 0 }
}

#[cfg(not(feature = "hosted"))]
pub fn write_byte(b: u8) {
    while !tx_ready() {}
    unsafe { outb(COM1, b); }
}

#[cfg(not(feature = "hosted"))]
pub fn read_byte() -> u8 {
    let held = PENDING.swap(0, core::sync::atomic::Ordering::Relaxed);
    if held != 0 { return (held & 0xff) as u8; }
    while !rx_ready() {}
    unsafe { inb(COM1) }
}

/// FIFO-burst write: fill the 14-byte FIFO before re-checking TX ready.
/// Closures can't call `unsafe fn` directly; use a standalone flush_buf().
#[cfg(not(feature = "hosted"))]
fn flush_buf(buf: &[u8; 14], fill: usize) {
    if fill == 0 { return; }
    while !tx_ready() {}
    unsafe {
        for i in 0..fill { outb(COM1, buf[i]); }
    }
}

/// Hosted has no FIFO to burst into; stdout does its own buffering.
#[cfg(feature = "hosted")]
fn flush_buf(buf: &[u8; 14], fill: usize) {
    for i in 0..fill { write_byte(buf[i]); }
}

/// Decimal on the stack. The heap-exhaustion path cannot use `format!` -- that
/// allocates, and allocating is precisely what has just failed.
pub fn write_dec(mut n: usize) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    if n == 0 {
        write_byte(b'0');
        return;
    }
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for b in &buf[i..] { write_byte(*b); }
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

// ── Hosted I/O ───────────────────────────────────────────────────────
// The same four entry points over stdio. write_dec and write_str sit above
// write_byte and need no variant.

#[cfg(feature = "hosted")]
pub fn init() {}

/// Hosted reads block, so a byte is always considered available.
#[cfg(feature = "hosted")]
pub fn rx_ready() -> bool { true }

#[cfg(feature = "hosted")]
pub fn write_byte(b: u8) {
    use std::io::Write;
    let out = std::io::stdout();
    let mut h = out.lock();
    let _ = h.write_all(&[b]);
    if b == b'\n' { let _ = h.flush(); }
}

#[cfg(feature = "hosted")]
pub fn read_byte() -> u8 {
    use std::io::Read;
    let mut b = [0u8; 1];
    match std::io::stdin().read_exact(&mut b) {
        Ok(()) => b[0],
        Err(_) => b'\n',   // EOF reads as a newline so the REPL sees a blank line
    }
}

// ── SVG output (hosted-only) ─────────────────────────────────────────
// Saves SVG content to a file in the kernel's output directory.
// In hosted mode, files go to <project_root>/ob3ects/.
// In bare-metal mode (unimplemented), this would be a no-op.

#[cfg(feature = "hosted")]
pub mod svg_out {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    /// Saves SVG content to a file in the kernel's output directory.
    pub fn save_svg(content: &str, name_hint: &str) -> std::io::Result<PathBuf> {
        let dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let out_dir = dir.join("ob3ects");
        std::fs::create_dir_all(&out_dir)?;

        // Sanitize the name hint into a filename
        let sanitized: String = name_hint
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect();
        let filename = format!("braid_{}.svg", sanitized);
        let path = out_dir.join(&filename);

        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        Ok(path)
    }
}

#[cfg(not(feature = "hosted"))]
pub mod svg_out {
    /// No-op in bare-metal mode: SVG output stays on serial only.
    pub fn save_svg(content: &str, name_hint: &str) -> Result<(), ()> {
        let _ = (content, name_hint);
        Err(())
    }
}
