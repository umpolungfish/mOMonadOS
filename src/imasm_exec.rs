// ─── imasm_exec.rs ──────────────────────────────────────────────────────────
// The payload-carrying x86-64 lift and its interpreter, both already built and
// proven end to end in vox_core (`vox run <symbol> --args a,b <file>`). This
// file only re-exports them, matching the pattern `vox.rs` and `vox_decode.rs`
// already use for the rest of the crate.
pub use vox_core::imasm_module::{emit, words};
pub use vox_core::imasm_vm::{Host, Machine, Stop};

/// Real file and console I/O for `Machine::run_process`, backed by actual
/// `std::fs`/stdio — only compiled into the hosted build, same gate as every
/// other filesystem-touching command. A guest that opens, reads, or writes a
/// file under this does so for real, under whatever permission this process
/// already has.
#[cfg(feature = "hosted")]
pub struct StdHost { files: std::collections::BTreeMap<i32, std::fs::File>, next_fd: i32 }

#[cfg(feature = "hosted")]
impl StdHost {
    pub fn new() -> Self { StdHost { files: std::collections::BTreeMap::new(), next_fd: 100 } }
}

#[cfg(feature = "hosted")]
impl Host for StdHost {
    fn open(&mut self, path: &str, flags: i32, mode: i32) -> i32 {
        let mut opts = std::fs::OpenOptions::new();
        let acc = flags & 0b11;
        opts.read(acc == 0 || acc == 2).write(acc == 1 || acc == 2);
        if flags & 0o100 != 0 { opts.create(true); }
        if flags & 0o1000 != 0 { opts.truncate(true); }
        if flags & 0o2000 != 0 { opts.append(true); }
        #[cfg(unix)] { use std::os::unix::fs::OpenOptionsExt; opts.mode(mode as u32); }
        #[cfg(not(unix))] { let _ = mode; }
        match opts.open(path) {
            Ok(f) => { let fd = self.next_fd; self.next_fd += 1; self.files.insert(fd, f); fd }
            Err(_) => -2, // ENOENT: a precise errno map is a further rung
        }
    }
    fn read(&mut self, fd: i32, buf: &mut [u8]) -> i64 {
        use std::io::Read;
        match fd {
            0 => std::io::stdin().read(buf).map(|n| n as i64).unwrap_or(-5),
            _ => match self.files.get_mut(&fd) { Some(f) => f.read(buf).map(|n| n as i64).unwrap_or(-5), None => -9 },
        }
    }
    fn write(&mut self, fd: i32, buf: &[u8]) -> i64 {
        use std::io::Write;
        match fd {
            1 => std::io::stdout().write_all(buf).map(|_| buf.len() as i64).unwrap_or(-5),
            2 => std::io::stderr().write_all(buf).map(|_| buf.len() as i64).unwrap_or(-5),
            _ => match self.files.get_mut(&fd) { Some(f) => f.write(buf).map(|n| n as i64).unwrap_or(-5), None => -9 },
        }
    }
    fn close(&mut self, fd: i32) -> i32 {
        if matches!(fd, 0|1|2) { return 0; }
        if self.files.remove(&fd).is_some() { 0 } else { -9 }
    }
}
