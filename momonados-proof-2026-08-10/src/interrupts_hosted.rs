// interrupts_hosted.rs — the interrupt surface on a host that has an OS.
//
// Nothing here is emulated. A hosted process does not own the IDT, does not
// remap the PIC, and has no business programming the PIT; the host kernel is
// already doing all three. What the rest of the kernel actually asks of this
// module is four questions, and on a host the honest answers are constant.
//
// The timer answers are `false`/`0` rather than a simulated tick because the
// callers use them to decide whether a periodic slot is due. On bare metal that
// is a real 100Hz interrupt. Hosted, there is no such slot, and inventing one
// would make the loop behave differently rather than identically-but-elsewhere.
#![allow(dead_code)]

/// No PIT, so no ticks have accrued.
pub fn pending_ticks() -> u64 { 0 }

/// No periodic slot is ever due.
pub fn timer_ready() -> bool { false }

/// Escape is a keyboard-controller read on bare metal. Hosted, the terminal
/// delivers keys through stdin and the REPL sees them there instead.
pub fn escape_pressed() -> bool { false }

pub fn init_idt() {}
pub fn remap_pic() {}
pub fn init_pit(_hz: u32) {}

/// Boot calls this once. On a host it is the whole of what needs doing.
pub fn init(_hz: u32) {}
