//! Terminal styling for the kernel's serial surface.
//!
//! Colours are named for the ROLE they carry, never for the colour they are.
//! A palette named `CYAN`/`YELLOW` freezes one look into every call site and
//! makes a theme change a rewrite; a palette named `HEADING`/`VERDICT_T` moves
//! with the meaning. The only place a raw SGR code appears is this file.
//!
//! Everything degrades: `style::set_colour(false)` empties every escape and the
//! output stays aligned, because no width computation depends on a colour being
//! present. That matters on a serial line whose far end may be a log file.
//!
//! Author: Lando⊗⊙perator

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

/// Colour is on by default and can be turned off at runtime. A viewer piping
/// the serial line into a file wants the glyphs, not the escapes.
static COLOUR: AtomicBool = AtomicBool::new(true);

pub fn set_colour(on: bool) {
    COLOUR.store(on, Ordering::Relaxed);
}

pub fn colour_on() -> bool {
    COLOUR.load(Ordering::Relaxed)
}

// ── the palette, by role ────────────────────────────────────────────────────
// Kept to the 16-colour set: a bare-metal serial console is not guaranteed
// 256-colour, and a truecolour escape on a terminal that lacks it prints as
// literal garbage in the middle of a table.

macro_rules! sgr {
    ($name:ident, $code:expr) => {
        pub fn $name() -> &'static str {
            if colour_on() { concat!("\x1b[", $code, "m") } else { "" }
        }
    };
}

sgr!(reset, "0");
sgr!(bold, "1");
sgr!(dim, "2");

sgr!(frame, "36");       // rules, boxes, the furniture of a report
sgr!(heading, "1;37");   // section titles
sgr!(key, "1;34");       // the left column of a key/value row
sgr!(value, "0");        // the right column: plain, so numbers read as numbers
sgr!(accent, "1;33");    // the one thing on the screen that matters most
sgr!(muted, "2;37");     // provenance, units, counts — present but not loud
sgr!(glyph, "1;35");     // IMASM and primitive glyphs

// Belnap. These four are the kernel's verdicts and they get fixed colours,
// because a verdict that changes colour between reports is a verdict a reader
// has to re-learn on every screen.
sgr!(verdict_t, "1;32"); // T — closes
sgr!(verdict_f, "1;31"); // F — refuses
sgr!(verdict_b, "1;33"); // B — both, held without explosion
sgr!(verdict_n, "1;30"); // N — void, no watch spoke

/// The escape for a Belnap letter, so a verdict is never coloured by hand.
pub fn verdict(v: char) -> &'static str {
    match v {
        'T' => verdict_t(),
        'F' => verdict_f(),
        'B' => verdict_b(),
        _ => verdict_n(),
    }
}

/// What a Belnap letter means, in one word, for a legend or a footer.
pub fn verdict_word(v: char) -> &'static str {
    match v {
        'T' => "closes",
        'F' => "refuses",
        'B' => "both, held",
        _ => "void",
    }
}

// ── box drawing ─────────────────────────────────────────────────────────────
// One width for every report. A screen where each command picks its own box
// width reads as several programs sharing a terminal.

pub const W: usize = 66;

pub const TL: &str = "╭";
pub const TR: &str = "╮";
pub const BL: &str = "╰";
pub const BR: &str = "╯";
pub const H: &str = "─";
pub const V: &str = "│";
pub const TEE_L: &str = "├";
pub const TEE_R: &str = "┤";

/// Print `n` copies of a box-drawing piece. `str::repeat` needs an allocator
/// and this runs where there may not be one.
#[macro_export]
macro_rules! rule_n {
    ($piece:expr, $n:expr) => {{
        let mut i = 0usize;
        while i < $n {
            sprint!("{}", $piece);
            i += 1;
        }
    }};
}

/// A titled top rule: `╭── title ──────────╮`
#[macro_export]
macro_rules! head {
    ($title:expr) => {{
        let t = $title;
        sprint!("{}{}{}{} ", $crate::style::frame(), $crate::style::TL,
                $crate::style::H, $crate::style::H);
        sprint!("{}{}{} ", $crate::style::heading(), t, $crate::style::frame());
        // The title's own width is subtracted so every report ends in the same
        // column whatever it is called.
        let used = 4 + t.chars().count() + 1;
        let pad = if $crate::style::W > used { $crate::style::W - used } else { 0 };
        $crate::rule_n!($crate::style::H, pad);
        sprintln!("{}{}", $crate::style::TR, $crate::style::reset());
    }};
}

/// A plain bottom rule matching `head!`.
#[macro_export]
macro_rules! foot {
    () => {{
        sprint!("{}{}", $crate::style::frame(), $crate::style::BL);
        $crate::rule_n!($crate::style::H, $crate::style::W - 1);
        sprintln!("{}{}", $crate::style::BR, $crate::style::reset());
    }};
}

/// A divider inside a report.
#[macro_export]
macro_rules! divider {
    () => {{
        sprint!("{}{}", $crate::style::frame(), $crate::style::TEE_L);
        $crate::rule_n!($crate::style::H, $crate::style::W - 1);
        sprintln!("{}{}", $crate::style::TEE_R, $crate::style::reset());
    }};
}

/// A key/value row, key left-padded to a fixed column so a column of rows
/// forms a column rather than a ragged edge.
#[macro_export]
macro_rules! kv {
    ($k:expr, $($v:tt)*) => {{
        let k = $k;
        sprint!("  {}{}{}", $crate::style::key(), k, $crate::style::reset());
        let n = k.chars().count();
        let pad = if 22 > n { 22 - n } else { 1 };
        $crate::rule_n!(" ", pad);
        // The reset goes BEFORE the newline. Emitting it after put an escape at
        // the start of every following line, which is invisible on a colour
        // terminal and litters any captured log.
        sprint!("{}", $crate::style::value());
        sprint!($($v)*);
        sprintln!("{}", $crate::style::reset());
    }};
}

/// A Belnap verdict line, coloured by the letter and glossed once.
#[macro_export]
macro_rules! verdict_line {
    ($v:expr) => {{
        let v: char = $v;
        sprintln!("  {}VERDICT {}{}  {}{}{}",
            $crate::style::bold(), $crate::style::verdict(v), v,
            $crate::style::muted(), $crate::style::verdict_word(v),
            $crate::style::reset());
    }};
}
