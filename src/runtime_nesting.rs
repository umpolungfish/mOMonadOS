//! The runtime is one nested IMASM vessel.
//!
//! A hosted command is admitted at VINIT, runs at the AFWD leaf, and releases
//! its accumulated surface only when its enclosing TANCH drops.  The frame
//! stack is run-length encoded: a fixed point does not need one host allocation
//! or one host call per enclosing copy, but every mark retains the depth at
//! which it was carried.

pub const PROCESS_WORD: &str = "⊢∈≻⋈⊙⊤≻⋈⊥≺⋈⊞∋⊡⋈⊙⊣";
pub const DEFAULT_DEPTH: u32 = 64;

#[cfg(feature = "hosted")]
use std::{cell::RefCell, io::Write};

#[cfg(feature = "hosted")]
#[derive(Clone, Debug)]
pub struct Frame {
    pub mark: char,
    pub copies: u32,
}

#[cfg(feature = "hosted")]
#[derive(Debug)]
struct Active {
    command: String,
    depth: u32,
    frames: Vec<Frame>,
    output: Vec<u8>,
}

#[cfg(feature = "hosted")]
thread_local! {
    static ACTIVE: RefCell<Vec<Active>> = const { RefCell::new(Vec::new()) };
}

/// A command remains inside this guard until its terminal surface is emitted.
/// Drop covers every REPL exit path, including `continue`, `break`, and early
/// returns from a command arm.
pub struct CommandVessel {
    #[cfg(feature = "hosted")]
    active: bool,
}

impl CommandVessel {
    pub fn admit(command: &str) -> Self {
        #[cfg(feature = "hosted")]
        {
            let frames = PROCESS_WORD.chars().map(|mark| Frame {
                mark,
                copies: DEFAULT_DEPTH,
            }).collect();
            ACTIVE.with(|slot| {
                let mut slot = slot.borrow_mut();
                slot.push(Active {
                    command: command.to_string(),
                    depth: DEFAULT_DEPTH,
                    frames,
                    output: Vec::new(),
                });
            });
            return Self { active: true };
        }
        #[cfg(not(feature = "hosted"))]
        {
            let _ = command;
            Self {}
        }
    }
}

impl Drop for CommandVessel {
    fn drop(&mut self) {
        #[cfg(feature = "hosted")]
        if self.active {
            let finished = ACTIVE.with(|slot| slot.borrow_mut().pop());
            if let Some(active) = finished {
                debug_assert_eq!(active.frames.len(), PROCESS_WORD.chars().count());
                debug_assert!(active.frames.iter().all(|frame| frame.copies == active.depth));
                debug_assert!(!active.command.is_empty());
                let mut output = Some(active.output);
                ACTIVE.with(|slot| {
                    if let Some(parent) = slot.borrow_mut().last_mut() {
                        parent.output.extend_from_slice(output.take().unwrap().as_slice());
                    }
                });
                if let Some(output) = output {
                    // TANCH is the only external write.  An inner command
                    // contributes its surface to its enclosing vessel, so the
                    // process vessel alone reaches the host boundary.
                    let mut out = std::io::stdout().lock();
                    let _ = out.write_all(&output);
                    let _ = out.flush();
                }
            }
        }
    }
}

/// Returns true when a byte stayed inside the currently admitted command.
#[cfg(feature = "hosted")]
pub fn capture_byte(byte: u8) -> bool {
    ACTIVE.with(|slot| {
        if let Some(active) = slot.borrow_mut().last_mut() {
            active.output.push(byte);
            true
        } else {
            false
        }
    })
}

#[cfg(not(feature = "hosted"))]
pub fn capture_byte(_byte: u8) -> bool { false }

/// Guest-process stdout and stderr share the command's terminal surface when
/// an IMASM-lifted binary runs inside a command vessel.
#[cfg(feature = "hosted")]
pub fn capture_bytes(bytes: &[u8]) -> bool {
    ACTIVE.with(|slot| {
        if let Some(active) = slot.borrow_mut().last_mut() {
            active.output.extend_from_slice(bytes);
            true
        } else {
            false
        }
    })
}

#[cfg(not(feature = "hosted"))]
pub fn capture_bytes(_bytes: &[u8]) -> bool { false }

/// Send a terminal surface through the innermost vessel, or to the host when
/// no hosted vessel is active.
#[cfg(feature = "hosted")]
pub fn write_stdout(bytes: &[u8]) {
    if !capture_bytes(bytes) {
        let mut out = std::io::stdout().lock();
        let _ = out.write_all(bytes);
        let _ = out.flush();
    }
}

#[cfg(not(feature = "hosted"))]
pub fn write_stdout(_bytes: &[u8]) {}

/// Stderr belongs to the same terminal surface while a vessel is active.  A
/// GPU diagnostic therefore remains inside the command that caused it.
#[cfg(feature = "hosted")]
pub fn write_stderr(bytes: &[u8]) {
    if !capture_bytes(bytes) {
        let mut err = std::io::stderr().lock();
        let _ = err.write_all(bytes);
        let _ = err.flush();
    }
}

#[cfg(not(feature = "hosted"))]
pub fn write_stderr(_bytes: &[u8]) {}

#[macro_export]
macro_rules! nested_println {
    ($($arg:tt)*) => {{
        $crate::runtime_nesting::write_stdout(format!("{}\n", format_args!($($arg)*)).as_bytes());
    }};
}

#[macro_export]
macro_rules! nested_eprintln {
    ($($arg:tt)*) => {{
        $crate::runtime_nesting::write_stderr(format!("{}\n", format_args!($($arg)*)).as_bytes());
    }};
}

#[cfg(all(test, feature = "hosted"))]
mod tests {
    use super::*;

    #[test]
    fn a_command_carries_the_full_process_word_at_every_depth() {
        let vessel = CommandVessel::admit("nesting-control");
        ACTIVE.with(|slot| {
            let active = slot.borrow();
            let active = active.last().unwrap();
            assert_eq!(active.depth, DEFAULT_DEPTH);
            assert_eq!(active.frames.len(), PROCESS_WORD.chars().count());
            assert!(active.frames.iter().all(|frame| frame.copies == DEFAULT_DEPTH));
        });
        assert!(capture_bytes(b"leaf"));
        ACTIVE.with(|slot| assert_eq!(slot.borrow().last().unwrap().output, b"leaf"));
        drop(vessel);
    }

    #[test]
    fn an_inner_terminal_surface_stays_inside_its_enclosing_vessel() {
        let outer = CommandVessel::admit("process");
        assert!(capture_bytes(b"before:"));
        {
            let inner = CommandVessel::admit("command");
            assert!(capture_bytes(b"leaf"));
            drop(inner);
        }
        assert!(capture_bytes(b":after"));
        ACTIVE.with(|slot| assert_eq!(slot.borrow().last().unwrap().output, b"before:leaf:after"));
        drop(outer);
    }

    #[test]
    fn a_hosted_diagnostic_stays_inside_the_command_that_emitted_it() {
        let vessel = CommandVessel::admit("diagnostic-control");
        crate::nested_eprintln!("nested diagnostic");
        ACTIVE.with(|slot| assert_eq!(slot.borrow().last().unwrap().output, b"nested diagnostic\n"));
        drop(vessel);
    }
}
