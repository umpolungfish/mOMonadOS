// manus.rs — Dynamic terminal displays for mOMonadOS
//
// Provides ANSI-terminal visual output for the bare-metal kernel:
//   - Live HUD during continuous execution
//   - Token trace / execution trail
//   - Structural panel (12-primitive snapshot)
//   - ASCII-art crystal address display
//
// All output goes through the serial UART. ANSI escape codes are used
// for cursor positioning, colors, and screen management.
// Works with QEMU -serial stdio and any ANSI-compatible terminal.

use crate::serial;
use crate::kernel::{Kernel, Snapshot};
use crate::tokens::Token;

// ─── ANSI Escape Helpers ──────────────────────────────────────

/// Hide terminal cursor.
pub fn cursor_hide() { serial::write_str("\x1b[?25l"); }

/// Show terminal cursor.
pub fn cursor_show() { serial::write_str("\x1b[?25h"); }

/// Move cursor to (row, col) — 1-indexed.
pub fn cursor_goto(row: u16, col: u16) {
    serial::write_str("\x1b[");
    crate::serial::SERIAL.lock().write_fmt(format_args!("{};{}H", row, col)).ok();
}

/// Clear entire screen.
pub fn cls() { serial::write_str("\x1b[2J"); }

/// Clear from cursor to end of line.
pub fn clear_line() { serial::write_str("\x1b[K"); }

/// Clear from cursor to end of screen.
pub fn clear_below() { serial::write_str("\x1b[J"); }

// ─── Color / style helpers ────────────────────────────────────

pub const RESET:     &str = "\x1b[0m";
pub const BOLD:      &str = "\x1b[1m";
pub const DIM:       &str = "\x1b[2m";
pub const RED:       &str = "\x1b[31m";
pub const GREEN:     &str = "\x1b[32m";
pub const YELLOW:    &str = "\x1b[33m";
pub const BLUE:      &str = "\x1b[34m";
pub const MAGENTA:   &str = "\x1b[35m";
pub const CYAN:      &str = "\x1b[36m";
pub const WHITE:     &str = "\x1b[37m";
pub const BOLD_RED:  &str = "\x1b[1;31m";
pub const BOLD_GREEN:&str = "\x1b[1;32m";
pub const BOLD_CYAN: &str = "\x1b[1;36m";
pub const BOLD_WHITE:&str = "\x1b[1;37m";

/// Write a styled string: style + text + reset.
pub fn styled(style: &str, text: &str) {
    serial::write_str(style);
    serial::write_str(text);
    serial::write_str(RESET);
}

/// Draw a horizontal rule across the current line.
pub fn hr(ch: char, width: u16) {
    for _ in 0..width { serial::write_byte(ch as u8); }
}

// ─── Tier → color ─────────────────────────────────────────────

fn tier_color(tier: u8) -> &'static str {
    match tier {
        0 => DIM,       // O_0 — grey
        1 => YELLOW,    // O_1 — yellow
        2 => BLUE,      // O_2 — blue
        3 => MAGENTA,   // O_∞ — magenta
        _ => WHITE,
    }
}

fn tier_label(tier: u8) -> &'static str {
    match tier {
        0 => "O_0", 1 => "O_1", 2 => "O_2", 3 => "O_inf", _ => "?"
    }
}

fn b4_color(v: u8) -> &'static str {
    match v {
        0 => DIM,       // N — grey
        1 => GREEN,     // T — green
        2 => RED,       // F — red
        3 => MAGENTA,   // B — magenta
        _ => WHITE,
    }
}

fn token_color(t: Token) -> &'static str {
    match t {
        Token::VINIT  => CYAN,
        Token::TANCH  => RED,
        Token::AFWD   => GREEN,
        Token::AREV   => YELLOW,
        Token::CLINK  => BLUE,
        Token::ISCRIB => MAGENTA,
        Token::FSPLIT => BOLD_CYAN,
        Token::FFUSE  => BOLD_CYAN,
        Token::EVALT  => BOLD_GREEN,
        Token::EVALF  => BOLD_RED,
        Token::ENGAGR => BOLD_WHITE,
        Token::IFIX   => DIM,
    }
}

// ─── HUD (Heads-Up Display) ───────────────────────────────────

/// Draw the full HUD at the top of the terminal.
/// Occupies rows 1–9. Content area starts at row 10.
pub const HUD_HEIGHT: u16 = 9;

pub fn draw_hud(k: &Kernel, program_name: &str, width: u16) {
    cursor_goto(1, 1);
    clear_below();

    // ── Title bar (row 1) ──
    styled(BOLD_WHITE, "╔");
    hr('═', width - 2);
    styled(BOLD_WHITE, "╗");

    cursor_goto(2, 1);
    styled(BOLD_WHITE, "║");
    serial::write_str("  ");
    styled(BOLD_CYAN, "mOMonadOS");
    serial::write_str("  ·  ");
    styled(BOLD_WHITE, program_name);
    // Right-justify tick + tier
    let tick_str = alloc::format!("{:>8}", k.tick_count);
    let tier = k.snapshot.map(|s| s.tier).unwrap_or(0);
    let right = alloc::format!("Tick: {}  Tier: ", tick_str);
    let right_pad = width as usize - 2 - 15 - program_name.len() - right.len() - 4;
    for _ in 0..right_pad { serial::write_byte(b' '); }
    serial::write_str(&right);
    styled(tier_color(tier), tier_label(tier));
    serial::write_str("  ");
    styled(BOLD_WHITE, "║");

    // ── Status row (row 3) ──
    cursor_goto(3, 1);
    styled(BOLD_WHITE, "║");
    serial::write_str("  ");
    styled(DIM, "Phase:");
    serial::write_str(" ");
    styled(BOLD_WHITE, phase_label(k));
    serial::write_str("  ");
    styled(DIM, "IP:");
    serial::write_str(" ");
    let ip_str = alloc::format!("{}/{}", k.ip, k.program.len());
    serial::write_str(&ip_str);
    // Current token
    if k.ip < k.program.len() {
        serial::write_str("  ");
        styled(DIM, "Token:");
        serial::write_str(" ");
        styled(token_color(k.program.get(k.ip).unwrap()), k.program.get(k.ip).unwrap().name());
    }
    // Right side: halted
    let halted = if k.halted { styled(BOLD_RED, "HALTED") } else { serial::write_str("") };
    let _ = halted;
    serial::write_str("  ");
    if k.halted { styled(BOLD_RED, "HALTED"); }
    styled(BOLD_WHITE, " ║");

    // ── Counters row (row 4) ──
    cursor_goto(4, 1);
    styled(BOLD_WHITE, "║");
    serial::write_str("  ");
    styled(DIM, "Frob:");
    serial::write_str(" ");
    let frob_str = alloc::format!("{}/{}", k.frob_checks - k.frob_open, k.frob_checks);
    serial::write_str(&frob_str);
    serial::write_str("  ");
    styled(DIM, "B-live:");
    serial::write_str(" ");
    let bl = k.snapshot.map(|s| s.b_live_ticks).unwrap_or(0);
    let bl_str = alloc::format!("{}", bl);
    serial::write_str(&bl_str);
    serial::write_str("  ");
    styled(DIM, "Gates:");
    serial::write_str(" ");
    let gd = k.snapshot.map(|s| s.gate_discriminations).unwrap_or(0);
    let gd_str = alloc::format!("{}", gd);
    serial::write_str(&gd_str);
    serial::write_str("  ");
    styled(DIM, "Value-p:");
    serial::write_str(" ");
    let vp = k.snapshot.map(|s| s.value_period).unwrap_or(0);
    let vp_str = alloc::format!("{}", vp);
    serial::write_str(&vp_str);
    styled(BOLD_WHITE, " ║");

    // ── Structural row (row 5) ──
    cursor_goto(5, 1);
    styled(BOLD_WHITE, "║");
    if let Some(snap) = k.snapshot {
        serial::write_str("  ");
        styled(DIM, "Sig:");
        serial::write_str(" ");
        let sig_str = alloc::format!("({},{},{},{})", snap.sig.0, snap.sig.1, snap.sig.2, snap.sig.3);
        serial::write_str(&sig_str);
        serial::write_str("  ");
        styled(DIM, "Div:");
        serial::write_str(" ");
        let div_str = alloc::format!("{}/12", snap.token_diversity);
        serial::write_str(&div_str);
        serial::write_str("  ");
        styled(DIM, "Self:");
        serial::write_str(" ");
        styled(if snap.self_ref { GREEN } else { DIM },
               if snap.self_ref { "T" } else { "F" });
        serial::write_str("  ");
        styled(DIM, "Frob-ord:");
        serial::write_str(" ");
        let fo_str = alloc::format!("{}", snap.frobenius_order);
        serial::write_str(&fo_str);
        serial::write_str("  ");
        styled(DIM, "Dialeth:");
        serial::write_str(" ");
        styled(if snap.dialetheia_complete { GREEN } else { DIM },
               if snap.dialetheia_complete { "YES" } else { "no" });
        serial::write_str("  ");
        styled(DIM, "Period:");
        serial::write_str(" ");
        let p_str = alloc::format!("{}", snap.period);
        serial::write_str(&p_str);
    }
    styled(BOLD_WHITE, " ║");

    // ── Stack + Fork row (row 6) ──
    cursor_goto(6, 1);
    styled(BOLD_WHITE, "║");
    serial::write_str("  ");
    styled(DIM, "Stack[");
    serial::write_str(&alloc::format!("{}", k.stack.depth()));
    serial::write_str("]: ");
    // Show top 8 stack values
    let depth = k.stack.depth();
    let show = if depth > 8 { 8 } else { depth };
    for i in 0..show {
        let idx = depth - show + i;
        let val = k.stack.peek_at(idx);
        styled(b4_color(val as u8), val.name());
        serial::write_str(" ");
    }
    if depth > 8 { serial::write_str("…"); }
    serial::write_str("  ");
    styled(DIM, "Fork:");
    serial::write_str(" ");
    let fd_str = alloc::format!("{}", k.fork_depth());
    serial::write_str(&fd_str);
    styled(BOLD_WHITE, " ║");

    // ── Registers row (row 7) ──
    cursor_goto(7, 1);
    styled(BOLD_WHITE, "║");
    serial::write_str("  ");
    styled(DIM, "R0-R7: ");
    for i in 0..8 {
        let v = k.registers.read(i);
        styled(b4_color(v as u8), v.name());
        serial::write_str(" ");
    }
    styled(BOLD_WHITE, "║");

    // ── Token trace row (row 8) ──
    cursor_goto(8, 1);
    styled(BOLD_WHITE, "║");
    serial::write_str("  ");
    styled(DIM, "Trace: ");
    // Show the program with current IP highlighted
    let n = k.program.len();
    let show_start = if k.ip > 6 { k.ip - 6 } else { 0 };
    let show_end = (show_start + 13).min(n);
    if show_start > 0 { serial::write_str("… "); }
    for i in show_start..show_end {
        let t = k.program.get(i).unwrap();
        if i == k.ip {
            styled(BOLD_WHITE, "▶");
            styled(token_color(t), t.name());
        } else {
            if i > show_start { serial::write_str("·"); }
            styled(DIM, t.name());
        }
        if i < show_end - 1 { serial::write_str(" "); }
    }
    if show_end < n { serial::write_str(" …"); }
    styled(BOLD_WHITE, " ║");

    // ── Bottom bar (row 9) ──
    cursor_goto(9, 1);
    styled(BOLD_WHITE, "╚");
    hr('═', width - 2);
    styled(BOLD_WHITE, "╝");

    // Move cursor below HUD
    cursor_goto(HUD_HEIGHT + 1, 1);
}

fn phase_label(k: &Kernel) -> &'static str {
    match k.phase {
        crate::kernel::Phase::Boot    => "BOOT",
        crate::kernel::Phase::Think   => "THINK",
        crate::kernel::Phase::Act     => "ACT",
        crate::kernel::Phase::Observe => "OBSERVE",
        crate::kernel::Phase::Update  => "UPDATE",
        crate::kernel::Phase::Halt    => "HALT",
    }
}

// ─── Full-screen display modes ────────────────────────────────

/// Initialize the display: clear screen, hide cursor, draw HUD.
pub fn display_init(k: &Kernel, program_name: &str, width: u16) {
    cls();
    cursor_hide();
    draw_hud(k, program_name, width);
}

/// Shutdown display: show cursor, move to clean area.
pub fn display_shutdown() {
    cursor_show();
    cursor_goto(HUD_HEIGHT + 2, 1);
}

/// Refresh HUD only (no full clear — faster).
pub fn display_refresh(k: &Kernel, program_name: &str, width: u16) {
    draw_hud(k, program_name, width);
}

// ─── Continuous execution with periodic display refresh ───────

/// Run kernel ticks continuously with periodic HUD refresh.
/// `refresh_every` — refresh display every N ticks.
/// `should_stop` — called each tick; return true to halt.
/// Returns total ticks run.
pub fn run_with_display<F: FnMut() -> bool>(
    k: &mut Kernel,
    program_name: &str,
    width: u16,
    refresh_every: u64,
    mut should_stop: F,
) -> u64 {
    let start = k.tick_count;
    let mut since_refresh = 0u64;

    while !k.halted && !should_stop() {
        k.tick();
        since_refresh += 1;
        if since_refresh >= refresh_every {
            display_refresh(k, program_name, width);
            since_refresh = 0;
        }
    }

    // Final refresh
    if since_refresh > 0 {
        display_refresh(k, program_name, width);
    }

    k.tick_count - start
}

// ─── Token graph visualization (ASCII art) ────────────────────

/// Draw an ASCII-art representation of the token graph.
/// Shows FSPLIT/FFUSE nesting and token flow.
pub fn draw_token_graph(k: &Kernel, width: u16) {
    let prog = &k.program;
    let n = prog.len();
    if n == 0 { return; }

    cursor_goto(HUD_HEIGHT + 1, 1);
    clear_below();
    styled(BOLD_WHITE, "═══ Token Graph ═══");
    serial::write_str("\r\n");

    // Track fork depth for indentation
    let mut fork_depth: usize = 0;
    let max_depth: usize = 8; // cap indentation

    for i in 0..n {
        let t = prog.get(i).unwrap();
        let marker = if i == k.ip { "▶" } else { " " };

        // Adjust fork depth
        if t == Token::FFUSE && fork_depth > 0 { fork_depth -= 1; }

        // Draw indentation
        let indent = fork_depth * 2;
        for _ in 0..indent.min(max_depth * 2) { serial::write_byte(b' '); }

        // Draw node
        let prefix: &str = if t == Token::FSPLIT { "├─" }
                    else if t == Token::FFUSE { "└─" }
                    else if fork_depth > 0 { "│ " }
                    else { "─ " };

        serial::write_str(prefix);
        serial::write_str(marker);
        styled(token_color(t), t.name());

        // For FSPLIT, show matching FFUSE position
        if t == Token::FSPLIT {
            let ffuse = k.find_matching_ffuse(i);
            serial::write_str(&alloc::format!("  →FFUSE@{}", ffuse));
        }

        serial::write_str("\r\n");

        if t == Token::FSPLIT { fork_depth += 1; }
    }
}

// ─── Belnap memory heatmap ────────────────────────────────────

/// Draw a compact visual representation of B4 memory contents.
pub fn draw_memory_heatmap(k: &Kernel, start: usize, count: usize, width: u16) {
    cursor_goto(HUD_HEIGHT + 1, 1);
    clear_below();
    styled(BOLD_WHITE, "═══ B4 Memory Heatmap  ═══");
    serial::write_str("\r\n");

    let per_row: usize = width as usize / 4; // each B4 value: " N "
    for row in 0..((count + per_row - 1) / per_row) {
        for col in 0..per_row {
            let idx = start + row * per_row + col;
            if idx < 4096 {
                let val = k.memory.read(idx);
                let bg = match val as u8 {
                    0 => "\x1b[40m",  // N — black bg
                    1 => "\x1b[42m",  // T — green bg
                    2 => "\x1b[41m",  // F — red bg
                    3 => "\x1b[45m",  // B — magenta bg
                    _ => "\x1b[40m",
                };
                serial::write_str(bg);
                serial::write_str(" ");
                serial::write_str(val.name());
                serial::write_str(" ");
                serial::write_str("\x1b[0m");
            }
        }
        serial::write_str("\r\n");
    }
}
