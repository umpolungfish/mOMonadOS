#![allow(dead_code)]
// manus.rs — Dynamic terminal displays for mOMonadOS
//
// Provides ANSI-terminal visual output for the bare-metal kernel:
//   - Live HUD during continuous execution
//   - Token trace / execution trail
//   - Structural panel (12-primitive snapshot)
//   - ASCII-art token graph visualization
//   - Belnap memory heatmap
//
// All output through serial UART. ANSI escape codes for cursor, color, screen.
// Works with QEMU -serial stdio and any ANSI-compatible terminal.

use crate::serial;
use crate::kernel::Kernel;
use crate::tokens::Token;
use crate::belnap::B4;

// ─── ANSI Escape Helpers ──────────────────────────────────────

pub fn cursor_hide() { serial::write_str("\x1b[?25l"); }
pub fn cursor_show() { serial::write_str("\x1b[?25h"); }

pub fn cursor_goto(row: u16, col: u16) {
    // Write escape sequence byte by byte to avoid format machinery
    serial::write_str("\x1b[");
    write_u16(row);
    serial::write_byte(b';');
    write_u16(col);
    serial::write_byte(b'H');
}

fn write_u16(n: u16) {
    if n >= 100 { serial::write_byte(b'0' + (n / 100) as u8); }
    if n >= 10  { serial::write_byte(b'0' + ((n / 10) % 10) as u8); }
    serial::write_byte(b'0' + (n % 10) as u8);
}

pub fn cls() { serial::write_str("\x1b[2J"); }
pub fn clear_line() { serial::write_str("\x1b[K"); }
pub fn clear_below() { serial::write_str("\x1b[J"); }

pub fn enter_alt_screen() { serial::write_str("\x1b[?1049h"); }
pub fn exit_alt_screen() { serial::write_str("\x1b[?1049l"); }


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

pub fn styled(style: &str, text: &str) {
    serial::write_str(style);
    serial::write_str(text);
    serial::write_str(RESET);
}

pub fn hr(ch: &str, width: u16) {
    for _ in 0..width { serial::write_str(ch); }
}

// ─── Display utilities ────────────────────────────────────────

fn write_u64(n: u64) {
    if n == 0 { serial::write_byte(b'0'); return; }
    let mut buf = [0u8; 20];
    let mut i = 20;
    let mut m = n;
    while m > 0 { i -= 1; buf[i] = b'0' + (m % 10) as u8; m /= 10; }
    for j in i..20 { serial::write_byte(buf[j]); }
}

fn write_usize(n: usize) { write_u64(n as u64); }

fn pad_right(text: &str, width: usize) {
    serial::write_str(text);
    let len = text.len();
    if len < width {
        for _ in 0..(width - len) { serial::write_byte(b' '); }
    }
}

fn pad_left(text: &str, width: usize) {
    let len = text.len();
    if len < width {
        for _ in 0..(width - len) { serial::write_byte(b' '); }
    }
    serial::write_str(text);
}

// ─── Tier → color ─────────────────────────────────────────────

fn tier_color(tier: u8) -> &'static str {
    // tier 4 (O_inf_dag) is lateral to tier 3, not above it — given its own
    // color (CYAN) rather than falling into the WHITE catch-all.
    match tier { 0 => DIM, 1 => YELLOW, 2 => BLUE, 3 => MAGENTA, 4 => CYAN, _ => WHITE }
}

fn tier_label(tier: u8) -> &'static str {
    match tier { 0 => "O_0", 1 => "O_1", 2 => "O_2", 3 => "O_inf", 4 => "O_inf_dag", _ => "?" }
}

fn b4_style(v: B4) -> &'static str {
    match v { B4::N => DIM, B4::T => GREEN, B4::F => RED, B4::B => MAGENTA }
}

fn token_style(t: Token) -> &'static str {
    match t {
        Token::Vinit => CYAN,  Token::Tanch => RED,    Token::Afwd => GREEN,
        Token::Arev => YELLOW, Token::Clink => BLUE,    Token::Imscrib => MAGENTA,
        Token::Fsplit => BOLD_CYAN, Token::Ffuse => BOLD_CYAN,
        Token::Evalt => BOLD_GREEN, Token::Evalf => BOLD_RED,
        Token::Engagr => BOLD_WHITE, Token::Ifix => DIM,
        Token::Fsplit3 => BOLD_CYAN, Token::Ffuse3 => BOLD_CYAN,
        Token::Evali => MAGENTA,
        Token::Rotat => BOLD_WHITE,
    }
}


// ─── HUD (Heads-Up Display) ───────────────────────────────────

pub const HUD_HEIGHT: u16 = 9;

/// Draw the full HUD. Occupies rows 1–9. Content starts at row 10.
pub fn draw_hud(_k: &Kernel, _program_name: &str, _width: u16) {
    // HUD fully disabled — no terminal rendering, no cursor moves
}

/// Initialize display: clear screen, hide cursor, draw HUD.
pub fn display_init(_k: &Kernel, _program_name: &str, _width: u16) {
    // HUD fully disabled — no alt-screen, no clear, no cursor moves
}

/// Shutdown display: exit alt screen, show cursor.
/// HUD fully disabled — no alt-screen to leave, no cursor to restore.
pub fn display_shutdown() {
    // no-op
}

/// Refresh HUD only (no full clear — faster).
pub fn display_refresh(_k: &Kernel, _program_name: &str, _width: u16) {
    // HUD fully disabled — no refresh
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
    let mut since_refresh = refresh_every.saturating_sub(1);

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
/// Shows FSPLIT/FFUSE nesting and token flow with fork depth indentation.
pub fn draw_token_graph(k: &Kernel) {
    let prog = &k.program;
    let n = prog.len();
    if n == 0 { return; }

    cursor_goto(HUD_HEIGHT + 1, 1);
    clear_below();
    styled(BOLD_WHITE, "═══ Token Graph ═══\r\n");

    let mut fork_depth: usize = 0;

    for i in 0..n {
        let t = prog.get(i).unwrap();
        let marker = if i == k.ip { "▶" } else { " " };

        // Pre-adjust: FFUSE reduces depth before drawing
        if t == Token::Ffuse && fork_depth > 0 { fork_depth -= 1; }

        // Indent
        let indent = fork_depth * 2;
        for _ in 0..indent { serial::write_byte(b' '); }

        // Draw connector
        let prefix = if t == Token::Fsplit { "├─" }
                else if t == Token::Ffuse { "└─" }
                else if fork_depth > 0 { "│ " }
                else { "─ " };
        serial::write_str(prefix);
        serial::write_str(marker);
        styled(token_style(t), t.name());

        // For FSPLIT, show matching FFUSE
        if t == Token::Fsplit {
            let ffuse = k.find_matching_ffuse(i);
            serial::write_str(" →FFUSE@");
            write_usize(ffuse);
        }

        serial::write_str("\r\n");

        // Post-adjust: FSPLIT increases depth
        if t == Token::Fsplit { fork_depth += 1; }
    }
}

// ─── Belnap memory heatmap ────────────────────────────────────

/// Draw a compact visual representation of B4 memory as colored blocks.
pub fn draw_memory_heatmap(k: &Kernel, start: usize, count: usize, width: u16) {
    cursor_goto(HUD_HEIGHT + 1, 1);
    clear_below();
    styled(BOLD_WHITE, "═══ B4 Memory [");
    write_usize(start);
    serial::write_str("..");
    write_usize(start + count);
    styled(BOLD_WHITE, "] ═══\r\n");

    let per_row: usize = (width as usize) / 4;
    for row in 0..((count + per_row - 1) / per_row) {
        for col in 0..per_row {
            let idx = start + row * per_row + col;
            if idx < 4096 {
                let val = k.memory.read(idx);
                let bg = match val {
                    B4::N => "\x1b[40m",  // black
                    B4::T => "\x1b[42m",  // green
                    B4::F => "\x1b[41m",  // red
                    B4::B => "\x1b[45m",  // magenta
                };
                serial::write_str(bg);
                serial::write_str(" ");
                serial::write_str(val.name());
                serial::write_str(" ");
                serial::write_str(RESET);
            }
        }
        serial::write_str("\r\n");
    }
}

// ─── Animated spinners ────────────────────────────────────────

/// Simple ASCII spinner for "waiting" indication.
static SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn spinner_char(tick: u64) -> &'static str {
    SPINNER[(tick as usize) % SPINNER.len()]
}

/// Draw a simple activity indicator at a position.
pub fn draw_activity(k: &Kernel, row: u16, col: u16) {
    cursor_goto(row, col);
    serial::write_str(spinner_char(k.tick_count));
}
