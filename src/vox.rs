// ─── V⊙x — Control-flow Closure Auditor ────────────────────
// Native Rust implementation of the Belnap FOUR control-flow auditor.
// Lifts program CFGs to twelve-glyph IMASM words and runs SIXTEEN_3 verdict.
#![allow(dead_code)]

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;
use alloc::vec;

// 12 IMASM opcodes as chars
pub const VINIT: char = '⊢';
pub const TANCH: char = '⊣';
pub const AFWD: char = '>';
pub const AREV: char = '<';
pub const FSPLIT: char = '∈';
pub const FFUSE: char = '∋';
pub const IMSCRIB: char = '⊙';
pub const IFIX: char = '◻';
pub const CLINK: char = '⋈';
pub const EVALT: char = '⊤';
pub const EVALF: char = '⊥';
pub const ENGAGR: char = '⊞';

/// A decoded instruction from a binary.
#[derive(Clone, Debug)]
pub struct Instruction {
    pub address: u64,
    pub mnemonic: String,
    pub op_str: String,
}

/// Mnemonic prefixes that don't change classification
const MNEMONIC_PREFIXES: &[&str] = &["notrack ", "lock ", "bnd ", "rep ", "repe ", "repne ", "data16 "];

/// Strip mnemonic prefixes so classification is consistent
fn strip_prefix(mn: &str) -> &str {
    for p in MNEMONIC_PREFIXES {
        if mn.starts_with(p) {
            return &mn[p.len()..];
        }
    }
    mn
}

/// Parse a hex immediate from operand string
fn parse_imm(op_str: &str) -> Option<u64> {
    let s = op_str.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).ok()
    } else {
        None
    }
}

/// Check if operand string indicates an immediate (direct target)
fn is_direct(op_str: &str) -> bool {
    let s = op_str.trim();
    parse_imm(s).is_some()
}

/// Check if the operand indicates a memory write (destination ends with ']')
fn writes_memory(op_str: &str) -> bool {
    let dst = op_str.split(',').next().unwrap_or("").trim();
    dst.ends_with(']')
}

/// Move instructions (data movement between slots)
const MOVE_OPS: &[&str] = &["mov", "movzx", "movsx", "movsxd", "movabs",
    "movaps", "movdqa", "movdqu", "movups", "movd", "movq",
    "lea", "push", "pop", "xchg", "leave"];

/// Arithmetic/logic instructions (engagement)
const ENGAGE_OPS: &[&str] = &["add", "sub", "adc", "sbb", "imul", "mul", "idiv", "div",
    "and", "or", "xor", "not", "neg", "inc", "dec", "shl", "shr", "sar", "rol",
    "ror", "sal", "bt", "bsf", "bsr", "popcnt", "cdq", "cqo", "cwde",
    "pushfd", "pushfq", "popfd", "popfq", "lahf", "sahf"];

/// Terminal instructions
const TERMINAL_OPS: &[&str] = &["ret", "retf", "int3", "ud2", "hlt", "iret", "sysret", "sysretq"];

/// Truth-producing instructions
const TRUTH_OPS: &[&str] = &["cmp", "test", "ucomiss", "ucomisd", "ucomis", "comisd", "comiss"];

/// Classify an instruction into one of 12 IMASM glyphs
pub fn classify_instruction(ins: &Instruction) -> char {
    let mn = strip_prefix(&ins.mnemonic);
    let ops = ins.op_str.trim();

    // Terminal instructions
    if mn.starts_with("ret") || TERMINAL_OPS.contains(&mn) {
        return TANCH;
    }

    // call/jmp with direct vs indirect
    if mn == "call" {
        if is_direct(ops) {
            return AFWD;
        } else {
            return IMSCRIB;
        }
    }

    if mn == "jmp" {
        if is_direct(ops) {
            return AREV;
        } else {
            return IMSCRIB;
        }
    }

    // syscall
    if mn == "syscall" || (mn == "int" && ops == "0x80") {
        return IMSCRIB;
    }

    // Conditional branches — forks (jcc except jmp)
    if mn.starts_with('j') && mn != "jmp" {
        return FSPLIT;
    }

    // setcc — truth consumed
    if mn.starts_with("set") {
        return EVALF;
    }

    // cmovcc — truth consumed
    if mn.starts_with("cmov") {
        return EVALF;
    }

    // cmp/test
    if TRUTH_OPS.contains(&mn) {
        return EVALT;
    }

    // Memory writes — commit state
    if writes_memory(ops) && !MOVE_OPS.contains(&mn) {
        return IFIX;
    }

    // Data movement
    if MOVE_OPS.contains(&mn) {
        return CLINK;
    }

    // Arithmetic/logic — engagement
    if ENGAGE_OPS.contains(&mn) {
        return ENGAGR;
    }

    // Default: engagement (total mapping)
    ENGAGR
}

/// A simple set for no_std compatibility
struct SimpleSet {
    addrs: Vec<u64>,
}

impl SimpleSet {
    fn new() -> Self {
        SimpleSet { addrs: Vec::new() }
    }

    fn contains(&self, addr: u64) -> bool {
        self.addrs.iter().any(|&a| a == addr)
    }
}

/// Compute merge points: addresses with ≥2 predecessors
pub fn compute_merges(insns: &[Instruction]) -> Vec<u64> {
    let aset = SimpleSet { addrs: insns.iter().map(|i| i.address).collect() };

    let mut preds: Vec<(u64, u32)> = Vec::new();

    for idx in 0..insns.len() {
        let mn = strip_prefix(&insns[idx].mnemonic);
        let terminates = mn == "jmp" || mn.starts_with("ret");

        // Fall-through edge
        if !terminates && idx + 1 < insns.len() {
            let target = insns[idx + 1].address;
            increment_pred(&mut preds, target);
        }

        // Jump edge
        if mn.starts_with('j') {
            if let Some(t) = parse_imm(&insns[idx].op_str) {
                if aset.contains(t) {
                    increment_pred(&mut preds, t);
                }
            }
        }
    }

    preds.into_iter()
        .filter(|(_, c)| *c >= 2)
        .map(|(a, _)| a)
        .collect()
}

fn increment_pred(preds: &mut Vec<(u64, u32)>, addr: u64) {
    for entry in preds.iter_mut() {
        if entry.0 == addr {
            entry.1 += 1;
            return;
        }
    }
    preds.push((addr, 1));
}

/// Lift a function's instruction list to an IMASM word
pub fn recompile_function(insns: &[Instruction]) -> Vec<char> {
    let merges = compute_merges(insns);
    let mut tokens = vec![VINIT];

    for ins in insns {
        if merges.contains(&ins.address) {
            tokens.push(FFUSE);
        }
        tokens.push(classify_instruction(ins));
    }

    tokens
}

/// Lift multiple functions to words
pub fn recompile_module(functions: &[(String, u64, Vec<Instruction>)]) -> Vec<(String, u64, Vec<char>)> {
    let mut result = Vec::new();
    for (label, addr, insns) in functions {
        let word = recompile_function(insns);
        result.push((label.clone(), *addr, word));
    }
    result
}

/// Run SIXTEEN_3 verdict on an IMASM word
/// Verdict: T=closes, B=open-fork, N=clean-linear
pub fn verdict(word: &[char]) -> char {
    let mut depth: i32 = 0;
    let mut has_fork = false;
    let mut open_at_terminal = false;

    for &g in word {
        match g {
            FSPLIT => {
                depth += 1;
                has_fork = true;
            }
            FFUSE => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            TANCH => {
                if depth > 0 {
                    open_at_terminal = true;
                }
            }
            _ => {}
        }
    }

    if open_at_terminal {
        'B'
    } else if has_fork {
        'T'
    } else {
        'N'
    }
}

/// Emit word as glyph string
pub fn glyphs(word: &[char]) -> String {
    word.iter().collect()
}

/// Emit IMASM module text for a set of functions
pub fn emit_word(functions: &[(&str, u64, &[Instruction])]) -> String {
    let mut lines = vec![format!("; ⊙ vox native module")];
    let total: usize = functions.iter().map(|f| recompile_function(f.2).len()).sum();
    lines.push(format!("; {} words   {} glyphs", functions.len(), total));
    for (label, addr, insns) in functions {
        let word = recompile_function(insns);
        lines.push(format!("0x{:x}", addr));
        lines.push(glyphs(&word));
    }
    lines.join("\n") + "\n"
}

/// Check round-trip: lift(decompile(emit(binary))) == lift(recompile_module(binary))
pub fn roundtrip_check(functions: &[(String, u64, Vec<Instruction>)]) -> bool {
    let words1 = recompile_module(functions);
    let mut combined1 = String::new();
    for (_, _, w) in &words1 {
        for c in w {
            combined1.push(*c);
        }
    }

    // Re-emit and parse back
    let views: Vec<(&str, u64, &[Instruction])> = functions.iter()
        .map(|(l, a, ins)| (l.as_str(), *a, ins.as_slice()))
        .collect();
    let text = emit_word(&views);
    let mut combined2 = String::new();
    for line in text.lines() {
        if line.starts_with('0') || line.is_empty() || line.starts_with(';') {
            continue;
        }
        combined2.push_str(line);
    }

    combined1 == combined2
}

/// Parse ELF binary to extract executable sections
pub fn parse_elf(raw: &[u8]) -> (u64, Vec<(u64, Vec<u8>)>) {
    if raw.len() < 64 || &raw[0..4] != b"\x7fELF" {
        return (0, vec![]);
    }

    let is_64 = raw[4] == 2;
    let is_le = raw[5] == 1;

    // Helper closures for reading little/big endian
    let read_u64 = |off: usize| -> u64 {
        if is_le {
            u64::from_le_bytes(raw[off..off+8].try_into().unwrap())
        } else {
            u64::from_be_bytes(raw[off..off+8].try_into().unwrap())
        }
    };

    let read_u32 = |off: usize| -> u32 {
        if is_le {
            u32::from_le_bytes(raw[off..off+4].try_into().unwrap())
        } else {
            u32::from_be_bytes(raw[off..off+4].try_into().unwrap())
        }
    };

    let read_u16 = |off: usize| -> u16 {
        if is_le {
            u16::from_le_bytes(raw[off..off+2].try_into().unwrap())
        } else {
            u16::from_be_bytes(raw[off..off+2].try_into().unwrap())
        }
    };

    let (entry, shoff, shentsize, shnum) = if is_64 {
        (read_u64(24), read_u64(40), read_u16(58), read_u16(60))
    } else {
        (read_u32(24) as u64, read_u32(32) as u64, read_u16(46), read_u16(48))
    };

    let mut segments = vec![];

    for k in 0..shnum {
        let off = (shoff + (k as u64) * (shentsize as u64)) as usize;
        if off >= raw.len() { break; }

        if is_64 {
            // sh_type(4) at off+0, sh_flags(8) at off+8, sh_addr(8) at off+16,
            // sh_offset(8) at off+24, sh_size(8) at off+32
            let sh_type = read_u32(off);
            let sh_flags = read_u64(off + 8);
            let sh_addr = read_u64(off + 16);
            let sh_offset = read_u64(off + 24);
            let sh_size = read_u64(off + 32);

            // SHT_PROGBITS=1, SHF_EXECINSTR=0x4
            if sh_type == 1 && (sh_flags & 0x4) != 0 && sh_size > 0 {
                let end = (sh_offset + sh_size) as usize;
                if end <= raw.len() {
                    segments.push((sh_addr, raw[sh_offset as usize..end].to_vec()));
                }
            }
        } else {
            let sh_type = read_u32(off);
            let sh_flags = read_u32(off + 4) as u64;
            let sh_addr = read_u32(off + 8) as u64;
            let sh_offset = read_u32(off + 12) as u64;
            let sh_size = read_u32(off + 16) as u64;

            if sh_type == 1 && (sh_flags & 0x4) != 0 && sh_size > 0 {
                let end = (sh_offset + sh_size) as usize;
                if end <= raw.len() {
                    segments.push((sh_addr, raw[sh_offset as usize..end].to_vec()));
                }
            }
        }
    }

    (entry, segments)
}
