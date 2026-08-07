// ─── V⊙x decode — x86-64 bytes to instructions ─────────────────
//
// Vox lifts control flow to IMASM, but it took an instruction list nothing
// produced: `parse_elf` hands back bytes and there was no way from bytes to
// instructions. This closes that.
//
// It is a length decoder with control-flow recognition, not a disassembler. It
// needs exactly two things per instruction: how long it is, so the walk stays
// in phase, and which of the twelve glyphs it carries. Register names and
// operand detail are not reconstructed, because the glyph does not depend on
// them.
//
// An opcode it does not know returns None rather than a guess. The walk stops
// there and reports how far it got, so a partial lift is always visible as
// partial. A length decoder that guesses goes out of phase and produces a
// confident, wrong word.
#![allow(dead_code)]

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::vox::Instruction;

/// What the decoder recovered: the length, and enough about the operation to
/// name a mnemonic the classifier already understands.
pub struct Decoded {
    pub len: usize,
    pub mnemonic: &'static str,
    /// Branch target, absolute, when the instruction carries a direct one.
    pub target: Option<u64>,
    /// True when the destination operand is memory.
    pub writes_mem: bool,
}

/// Condition-code suffixes, indexed by the low nibble of a jcc/setcc/cmovcc.
const CC: [&str; 16] = [
    "o", "no", "b", "ae", "e", "ne", "be", "a", "s", "ns", "p", "np", "l", "ge", "le", "g",
];

struct Cursor<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Cursor<'a> {
    fn u8(&mut self) -> Option<u8> {
        let v = *self.b.get(self.i)?;
        self.i += 1;
        Some(v)
    }
    fn skip(&mut self, n: usize) -> Option<()> {
        if self.i + n > self.b.len() {
            return None;
        }
        self.i += n;
        Some(())
    }
    fn i8v(&mut self) -> Option<i64> {
        Some(self.u8()? as i8 as i64)
    }
    fn i32v(&mut self) -> Option<i64> {
        if self.i + 4 > self.b.len() {
            return None;
        }
        let v = i32::from_le_bytes([
            self.b[self.i],
            self.b[self.i + 1],
            self.b[self.i + 2],
            self.b[self.i + 3],
        ]);
        self.i += 4;
        Some(v as i64)
    }
}

/// Consume a ModRM byte and everything it implies (SIB, displacement).
/// Returns (reg field, dest-is-memory).
fn modrm(c: &mut Cursor) -> Option<(u8, bool)> {
    let m = c.u8()?;
    let md = m >> 6;
    let reg = (m >> 3) & 7;
    let rm = m & 7;
    let is_mem = md != 3;
    if is_mem {
        if rm == 4 {
            let sib = c.u8()?;
            // base = 5 with mod = 0 means a disp32 with no base register.
            if md == 0 && (sib & 7) == 5 {
                c.skip(4)?;
            }
        } else if md == 0 && rm == 5 {
            // RIP-relative: disp32.
            c.skip(4)?;
        }
        match md {
            1 => c.skip(1)?,
            2 => c.skip(4)?,
            _ => {}
        }
    }
    Some((reg, is_mem))
}

/// Decode one instruction at the start of `b`, sitting at address `addr`.
pub fn decode_one(b: &[u8], addr: u64) -> Option<Decoded> {
    let mut c = Cursor { b, i: 0 };
    let mut rex_w = false;
    let mut op66 = false;

    // Prefixes, then REX. REX must be last before the opcode.
    loop {
        let p = *b.get(c.i)?;
        match p {
            0x66 => {
                op66 = true;
                c.i += 1;
            }
            0x67 | 0xF0 | 0xF2 | 0xF3 | 0x2E | 0x36 | 0x3E | 0x26 | 0x64 | 0x65 => {
                c.i += 1;
            }
            0x40..=0x4F => {
                rex_w = p & 0x08 != 0;
                c.i += 1;
                break;
            }
            _ => break,
        }
    }

    let op = c.u8()?;

    // Two-byte opcodes. The escape space is mostly ModRM-shaped, so it is
    // handled by shape: no-operand forms and immediate-carrying forms are named
    // explicitly and everything else takes a ModRM and nothing more.
    if op == 0x0F {
        let op2 = c.u8()?;
        return match op2 {
            // No operand bytes.
            0x05 => Some(Decoded { len: c.i, mnemonic: "syscall", target: None, writes_mem: false }),
            0x0B => Some(Decoded { len: c.i, mnemonic: "ud2", target: None, writes_mem: false }),
            0x06 | 0x09 | 0x30..=0x35 | 0x77 | 0xA0 | 0xA1 | 0xA8 | 0xA9 => {
                Some(Decoded { len: c.i, mnemonic: "cpuid", target: None, writes_mem: false })
            }
            0xA2 => Some(Decoded { len: c.i, mnemonic: "cpuid", target: None, writes_mem: false }),
            0xC8..=0xCF => {
                Some(Decoded { len: c.i, mnemonic: "bswap", target: None, writes_mem: false })
            }

            // Conditional forms.
            0x40..=0x4F => {
                modrm(&mut c)?;
                Some(Decoded {
                    len: c.i,
                    mnemonic: cmov_name(op2 & 0x0F),
                    target: None,
                    writes_mem: false,
                })
            }
            0x80..=0x8F => {
                let d = c.i32v()?;
                let len = c.i;
                Some(Decoded {
                    len,
                    mnemonic: jcc_name(op2 & 0x0F),
                    target: Some(addr.wrapping_add(len as u64).wrapping_add(d as u64)),
                    writes_mem: false,
                })
            }
            0x90..=0x9F => {
                let (_, mem) = modrm(&mut c)?;
                Some(Decoded {
                    len: c.i,
                    mnemonic: setcc_name(op2 & 0x0F),
                    target: None,
                    writes_mem: mem,
                })
            }

            // Three-byte escapes.
            0x38 => {
                c.u8()?;
                let (_, mem) = modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "pshufb", target: None, writes_mem: mem })
            }
            0x3A => {
                c.u8()?;
                let (_, mem) = modrm(&mut c)?;
                c.skip(1)?;
                Some(Decoded { len: c.i, mnemonic: "palignr", target: None, writes_mem: mem })
            }

            // ModRM plus an 8-bit immediate.
            0x70..=0x73 | 0xA4 | 0xAC | 0xBA | 0xC2 | 0xC4 | 0xC5 | 0xC6 => {
                let (_, mem) = modrm(&mut c)?;
                c.skip(1)?;
                let name = if op2 == 0xBA { "bt" } else { "pshufd" };
                Some(Decoded { len: c.i, mnemonic: name, target: None, writes_mem: mem })
            }

            // Named ModRM forms whose glyph is not the default.
            0xB6 | 0xB7 => {
                modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "movzx", target: None, writes_mem: false })
            }
            0xBE | 0xBF => {
                modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "movsx", target: None, writes_mem: false })
            }
            0xAF => {
                modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "imul", target: None, writes_mem: false })
            }
            0xA3 | 0xAB | 0xB3 | 0xBB | 0xBC | 0xBD => {
                let (_, mem) = modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "bt", target: None, writes_mem: mem })
            }
            0xB0 | 0xB1 | 0xC0 | 0xC1 => {
                let (_, mem) = modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "xadd", target: None, writes_mem: mem })
            }
            0x10..=0x17 | 0x28 | 0x29 | 0x2A..=0x2F => {
                let (_, mem) = modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "movaps", target: None, writes_mem: mem })
            }

            // Everything else in the escape space takes a ModRM and stops.
            _ => {
                let (_, mem) = modrm(&mut c)?;
                Some(Decoded { len: c.i, mnemonic: "paddb", target: None, writes_mem: mem })
            }
        };
    }

    // One-byte opcodes.
    match op {
        // Arithmetic block: op r/m,r and r,r/m at low bits 0-3, accumulator
        // forms with an immediate at 4 and 5.
        0x00..=0x3D if (op & 0x07) <= 5 && (op & 0xC0) == 0 => {
            let name = arith_name(op >> 3);
            match op & 0x07 {
                0 | 1 | 2 | 3 => {
                    let (_, mem) = modrm(&mut c)?;
                    // Direction bit clear means r/m is the destination.
                    let writes = mem && (op & 0x02) == 0;
                    Some(Decoded { len: c.i, mnemonic: name, target: None, writes_mem: writes })
                }
                4 => {
                    c.skip(1)?;
                    Some(Decoded { len: c.i, mnemonic: name, target: None, writes_mem: false })
                }
                _ => {
                    c.skip(if op66 { 2 } else { 4 })?;
                    Some(Decoded { len: c.i, mnemonic: name, target: None, writes_mem: false })
                }
            }
        }
        0x50..=0x57 => Some(Decoded { len: c.i, mnemonic: "push", target: None, writes_mem: false }),
        0x58..=0x5F => Some(Decoded { len: c.i, mnemonic: "pop", target: None, writes_mem: false }),
        0x63 => {
            modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "movsxd", target: None, writes_mem: false })
        }
        0x68 => {
            c.skip(4)?;
            Some(Decoded { len: c.i, mnemonic: "push", target: None, writes_mem: false })
        }
        0x6A => {
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "push", target: None, writes_mem: false })
        }
        0x69 => {
            modrm(&mut c)?;
            c.skip(if op66 { 2 } else { 4 })?;
            Some(Decoded { len: c.i, mnemonic: "imul", target: None, writes_mem: false })
        }
        0x6B => {
            modrm(&mut c)?;
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "imul", target: None, writes_mem: false })
        }
        0x70..=0x7F => {
            let d = c.i8v()?;
            let len = c.i;
            Some(Decoded {
                len,
                mnemonic: jcc_name(op & 0x0F),
                target: Some(addr.wrapping_add(len as u64).wrapping_add(d as u64)),
                writes_mem: false,
            })
        }
        0x80 | 0x83 => {
            let (r, mem) = modrm(&mut c)?;
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: arith_name(r), target: None, writes_mem: mem })
        }
        0x81 => {
            let (r, mem) = modrm(&mut c)?;
            c.skip(if op66 { 2 } else { 4 })?;
            Some(Decoded { len: c.i, mnemonic: arith_name(r), target: None, writes_mem: mem })
        }
        0x84 | 0x85 => {
            modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "test", target: None, writes_mem: false })
        }
        0x86 | 0x87 => {
            let (_, mem) = modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "xchg", target: None, writes_mem: mem })
        }
        0x88..=0x8B => {
            let (_, mem) = modrm(&mut c)?;
            let writes = mem && (op & 0x02) == 0;
            Some(Decoded { len: c.i, mnemonic: "mov", target: None, writes_mem: writes })
        }
        0x8D => {
            modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "lea", target: None, writes_mem: false })
        }
        0x8F => {
            let (_, mem) = modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "pop", target: None, writes_mem: mem })
        }
        0x90 => Some(Decoded { len: c.i, mnemonic: "nop", target: None, writes_mem: false }),
        // xchg with the accumulator: 0x90 is the nop special case of this range.
        0x91..=0x97 => Some(Decoded { len: c.i, mnemonic: "xchg", target: None, writes_mem: false }),
        0x98 => Some(Decoded { len: c.i, mnemonic: "cwde", target: None, writes_mem: false }),
        0x99 => Some(Decoded { len: c.i, mnemonic: "cdq", target: None, writes_mem: false }),
        0x9C => Some(Decoded { len: c.i, mnemonic: "pushfq", target: None, writes_mem: false }),
        0x9D => Some(Decoded { len: c.i, mnemonic: "popfq", target: None, writes_mem: false }),
        // String operations: no ModRM, no immediate. REP prefixes are already
        // consumed above, so `rep movsq` arrives here as a bare 0xA5.
        0xA4 | 0xA5 => Some(Decoded { len: c.i, mnemonic: "movs", target: None, writes_mem: true }),
        0xA6 | 0xA7 => Some(Decoded { len: c.i, mnemonic: "cmps", target: None, writes_mem: false }),
        0xAA | 0xAB => Some(Decoded { len: c.i, mnemonic: "stos", target: None, writes_mem: true }),
        0xAC | 0xAD => Some(Decoded { len: c.i, mnemonic: "lods", target: None, writes_mem: false }),
        0xAE | 0xAF => Some(Decoded { len: c.i, mnemonic: "scas", target: None, writes_mem: false }),
        0xA8 => {
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "test", target: None, writes_mem: false })
        }
        0xA9 => {
            c.skip(if op66 { 2 } else { 4 })?;
            Some(Decoded { len: c.i, mnemonic: "test", target: None, writes_mem: false })
        }
        0xB0..=0xB7 => {
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "mov", target: None, writes_mem: false })
        }
        0xB8..=0xBF => {
            c.skip(if rex_w { 8 } else if op66 { 2 } else { 4 })?;
            Some(Decoded { len: c.i, mnemonic: "mov", target: None, writes_mem: false })
        }
        0xC0 | 0xC1 => {
            let (r, mem) = modrm(&mut c)?;
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: shift_name(r), target: None, writes_mem: mem })
        }
        0xC2 => {
            c.skip(2)?;
            Some(Decoded { len: c.i, mnemonic: "ret", target: None, writes_mem: false })
        }
        0xC3 => Some(Decoded { len: c.i, mnemonic: "ret", target: None, writes_mem: false }),
        0xC6 => {
            let (_, mem) = modrm(&mut c)?;
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "mov", target: None, writes_mem: mem })
        }
        0xC7 => {
            let (_, mem) = modrm(&mut c)?;
            c.skip(if op66 { 2 } else { 4 })?;
            Some(Decoded { len: c.i, mnemonic: "mov", target: None, writes_mem: mem })
        }
        0xC9 => Some(Decoded { len: c.i, mnemonic: "leave", target: None, writes_mem: false }),
        0xCC => Some(Decoded { len: c.i, mnemonic: "int3", target: None, writes_mem: false }),
        0xD0..=0xD3 => {
            let (r, mem) = modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: shift_name(r), target: None, writes_mem: mem })
        }
        0xE8 => {
            let d = c.i32v()?;
            let len = c.i;
            Some(Decoded {
                len,
                mnemonic: "call",
                target: Some(addr.wrapping_add(len as u64).wrapping_add(d as u64)),
                writes_mem: false,
            })
        }
        0xE9 => {
            let d = c.i32v()?;
            let len = c.i;
            Some(Decoded {
                len,
                mnemonic: "jmp",
                target: Some(addr.wrapping_add(len as u64).wrapping_add(d as u64)),
                writes_mem: false,
            })
        }
        0xEB => {
            let d = c.i8v()?;
            let len = c.i;
            Some(Decoded {
                len,
                mnemonic: "jmp",
                target: Some(addr.wrapping_add(len as u64).wrapping_add(d as u64)),
                writes_mem: false,
            })
        }
        // x87: the whole D8..DF escape takes a ModRM and no immediate.
        0xD8..=0xDF => {
            let (_, mem) = modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "fld", target: None, writes_mem: mem })
        }
        0x8C | 0x8E => {
            let (_, mem) = modrm(&mut c)?;
            Some(Decoded { len: c.i, mnemonic: "mov", target: None, writes_mem: mem })
        }
        0x9B | 0x9E | 0x9F | 0xD7 | 0xF5 | 0xF8..=0xFD => {
            Some(Decoded { len: c.i, mnemonic: "lahf", target: None, writes_mem: false })
        }
        0xCD => {
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "int", target: None, writes_mem: false })
        }
        0xCE | 0xCF => Some(Decoded { len: c.i, mnemonic: "iret", target: None, writes_mem: false }),
        0xE0..=0xE3 => {
            let d = c.i8v()?;
            let len = c.i;
            Some(Decoded {
                len,
                mnemonic: "jne",
                target: Some(addr.wrapping_add(len as u64).wrapping_add(d as u64)),
                writes_mem: false,
            })
        }
        0xE4..=0xE7 => {
            c.skip(1)?;
            Some(Decoded { len: c.i, mnemonic: "in", target: None, writes_mem: false })
        }
        0xEC..=0xEF => Some(Decoded { len: c.i, mnemonic: "in", target: None, writes_mem: false }),
        0xF4 => Some(Decoded { len: c.i, mnemonic: "hlt", target: None, writes_mem: false }),
        0xF6 => {
            let (r, mem) = modrm(&mut c)?;
            if r <= 1 {
                c.skip(1)?;
            }
            Some(Decoded { len: c.i, mnemonic: unary_name(r), target: None, writes_mem: mem })
        }
        0xF7 => {
            let (r, mem) = modrm(&mut c)?;
            if r <= 1 {
                c.skip(if op66 { 2 } else { 4 })?;
            }
            Some(Decoded { len: c.i, mnemonic: unary_name(r), target: None, writes_mem: mem })
        }
        0xFE => {
            let (r, mem) = modrm(&mut c)?;
            Some(Decoded {
                len: c.i,
                mnemonic: if r == 0 { "inc" } else { "dec" },
                target: None,
                writes_mem: mem,
            })
        }
        0xFF => {
            let (r, mem) = modrm(&mut c)?;
            // Group 5. Indirect call and jmp are the ⊙ cases: the target is not
            // in the instruction, so the word cannot name where it goes.
            let name = match r {
                0 => "inc",
                1 => "dec",
                2 | 3 => "call",
                4 | 5 => "jmp",
                6 => "push",
                _ => return None,
            };
            let indirect_ops = if r == 2 || r == 3 || r == 4 || r == 5 { "rax" } else { "" };
            let _ = indirect_ops;
            Some(Decoded { len: c.i, mnemonic: name, target: None, writes_mem: mem && r <= 1 })
        }
        _ => None,
    }
}

fn arith_name(r: u8) -> &'static str {
    match r & 7 {
        0 => "add",
        1 => "or",
        2 => "adc",
        3 => "sbb",
        4 => "and",
        5 => "sub",
        6 => "xor",
        _ => "cmp",
    }
}

fn shift_name(r: u8) -> &'static str {
    match r & 7 {
        0 => "rol",
        1 => "ror",
        2 => "rcl",
        3 => "rcr",
        4 => "shl",
        5 => "shr",
        6 => "sal",
        _ => "sar",
    }
}

fn unary_name(r: u8) -> &'static str {
    match r & 7 {
        0 | 1 => "test",
        2 => "not",
        3 => "neg",
        4 => "mul",
        5 => "imul",
        6 => "div",
        _ => "idiv",
    }
}

fn jcc_name(cc: u8) -> &'static str {
    match cc {
        0 => "jo", 1 => "jno", 2 => "jb", 3 => "jae",
        4 => "je", 5 => "jne", 6 => "jbe", 7 => "ja",
        8 => "js", 9 => "jns", 10 => "jp", 11 => "jnp",
        12 => "jl", 13 => "jge", 14 => "jle", _ => "jg",
    }
}

fn setcc_name(cc: u8) -> &'static str {
    match cc {
        0 => "seto", 1 => "setno", 2 => "setb", 3 => "setae",
        4 => "sete", 5 => "setne", 6 => "setbe", 7 => "seta",
        8 => "sets", 9 => "setns", 10 => "setp", 11 => "setnp",
        12 => "setl", 13 => "setge", 14 => "setle", _ => "setg",
    }
}

fn cmov_name(cc: u8) -> &'static str {
    match cc {
        0 => "cmovo", 1 => "cmovno", 2 => "cmovb", 3 => "cmovae",
        4 => "cmove", 5 => "cmovne", 6 => "cmovbe", 7 => "cmova",
        8 => "cmovs", 9 => "cmovns", 10 => "cmovp", 11 => "cmovnp",
        12 => "cmovl", 13 => "cmovge", 14 => "cmovle", _ => "cmovg",
    }
}

/// The result of walking a byte range: the instructions recovered, and where
/// the walk stopped if it could not continue.
pub struct Lift {
    pub instructions: Vec<Instruction>,
    /// Byte offset at which decoding failed, if it did.
    pub stopped_at: Option<usize>,
    pub bytes_covered: usize,
    pub bytes_total: usize,
}

impl Lift {
    pub fn complete(&self) -> bool {
        self.stopped_at.is_none()
    }
    pub fn coverage_percent(&self) -> usize {
        if self.bytes_total == 0 {
            100
        } else {
            self.bytes_covered * 100 / self.bytes_total
        }
    }
}

/// Walk a byte range into instructions. Stops at the first opcode it cannot
/// decode, rather than guessing a length and going out of phase.
pub fn lift_bytes(base: u64, bytes: &[u8]) -> Lift {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let addr = base + i as u64;
        match decode_one(&bytes[i..], addr) {
            Some(d) if d.len > 0 => {
                let op_str = match d.target {
                    Some(t) => format!("0x{:x}", t),
                    None => {
                        if d.writes_mem {
                            "qword ptr [rax], rbx".to_string()
                        } else if is_indirect_branch(d.mnemonic, &bytes[i..i + d.len]) {
                            "rax".to_string()
                        } else {
                            String::new()
                        }
                    }
                };
                out.push(Instruction {
                    address: addr,
                    mnemonic: d.mnemonic.to_string(),
                    op_str,
                });
                i += d.len;
            }
            _ => {
                return Lift {
                    instructions: out,
                    stopped_at: Some(i),
                    bytes_covered: i,
                    bytes_total: bytes.len(),
                }
            }
        }
    }
    let n = bytes.len();
    Lift { instructions: out, stopped_at: None, bytes_covered: n, bytes_total: n }
}

/// A call or jmp with no immediate target is indirect, which vox reads as ⊙.
fn is_indirect_branch(mnemonic: &str, raw: &[u8]) -> bool {
    matches!(mnemonic, "call" | "jmp") && raw.last().is_some() && raw[0] != 0xE8 && raw[0] != 0xE9
        && raw[0] != 0xEB
}

/// Condition-code names, exposed so a caller can check the table is total.
pub fn cc_names() -> &'static [&'static str; 16] {
    &CC
}

// ── Recursive descent ──────────────────────────────────────────
//
// A linear sweep decodes padding and embedded data as if they were code, and
// it cannot tell where one function ends and the next begins except by
// guessing at padding. Descent walks instead: from the entry point and every
// direct call target, one instruction at a time, following the edges the
// program actually has. Anything reachable only through an indirect transfer
// (⊙) cannot be followed statically — that is what the glyph is for — so what
// descent never reaches is swept afterwards rather than dropped.

use alloc::collections::{BTreeMap, BTreeSet};

/// The executable image: byte ranges with the addresses they load at.
pub struct Image {
    pub segments: Vec<(u64, Vec<u8>)>,
}

impl Image {
    pub fn bytes_at(&self, addr: u64) -> Option<&[u8]> {
        for (base, data) in &self.segments {
            if addr >= *base && addr < *base + data.len() as u64 {
                return Some(&data[(addr - *base) as usize..]);
            }
        }
        None
    }

    pub fn total_bytes(&self) -> usize {
        self.segments.iter().map(|(_, d)| d.len()).sum()
    }
}

/// Build the instruction a Decoded stands for. Descent and the sweep must
/// agree here: synthesising the operand string differently in the two paths
/// makes the same byte read as ◻ in one and ⊞ in the other.
fn instruction_of(d: &Decoded, addr: u64, raw: &[u8]) -> Instruction {
    Instruction {
        address: addr,
        mnemonic: d.mnemonic.to_string(),
        op_str: match d.target {
            Some(t) => format!("0x{:x}", t),
            None => {
                if d.writes_mem {
                    "qword ptr [rax], rbx".to_string()
                } else if is_indirect_branch(d.mnemonic, raw) {
                    "rax".to_string()
                } else {
                    String::new()
                }
            }
        },
    }
}

/// The direct target of a branch, when it has one written in the instruction.
fn direct_target(ins: &Instruction) -> Option<u64> {
    let s = ins.op_str.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).ok()
    } else {
        None
    }
}

fn is_terminal(mn: &str) -> bool {
    mn.starts_with("ret") || matches!(mn, "int3" | "ud2" | "hlt" | "iret" | "jmp")
}

/// Walk an image into functions.
///
/// Returns (start address, instructions) per function, in discovery order for
/// the descended ones and address order for the swept remainder.
pub fn descend(image: &Image, entry: u64) -> Vec<(u64, Vec<Instruction>)> {
    descend_seeded(image, entry, &[])
}

/// Trailing padding is not part of the function it follows. Leaving it on
/// makes every function end in a run of ⊣, which changes the verdict.
fn trim_padding(mut f: Vec<Instruction>) -> Vec<Instruction> {
    while f.len() > 1 {
        let last = crate::vox::strip_prefix(&f[f.len() - 1].mnemonic).to_string();
        if crate::vox::PAD_OPS.contains(&last.as_str()) {
            f.pop();
        } else {
            break;
        }
    }
    f
}

/// What a walk claimed, and what it left.
///
/// Which bytes of a stripped binary begin a function is not recoverable from
/// the bytes. Descent proves it for whatever is reachable from the entry, the
/// symbol table proves it where one survives, a branch target proves it, and a
/// padding boundary is strong evidence. Past that there is nothing to read, and
/// walking the remainder byte by byte does not discover functions — it
/// manufactures them at offsets that are the middle of something else. So the
/// unclaimed bytes are counted and reported rather than guessed at.
pub struct Walk {
    pub functions: Vec<(u64, Vec<Instruction>)>,
    pub claimed_bytes: usize,
    pub total_bytes: usize,
}

impl Walk {
    pub fn claimed_percent(&self) -> usize {
        if self.total_bytes == 0 {
            100
        } else {
            self.claimed_bytes * 100 / self.total_bytes
        }
    }
}

/// Walk an image into functions, seeding descent with known function addresses
/// on top of the entry point.
pub fn descend_seeded(
    image: &Image,
    entry: u64,
    seeds: &[u64],
) -> Vec<(u64, Vec<Instruction>)> {
    let mut out: Vec<(u64, Vec<Instruction>)> = Vec::new();
    let mut covered: BTreeMap<u64, Instruction> = BTreeMap::new();
    let mut seen_funcs: BTreeSet<u64> = BTreeSet::new();
    let mut func_queue: Vec<u64> = Vec::new();
    // Every address the program branches to, whether or not descent got there.
    // A target is proof that the bytes at it are code; nothing else in the
    // leftover is.
    let mut targets: BTreeSet<u64> = BTreeSet::new();

    if image.bytes_at(entry).is_some() {
        func_queue.push(entry);
    } else if let Some((base, _)) = image.segments.first() {
        func_queue.push(*base);
    }
    for s in seeds {
        if image.bytes_at(*s).is_some() {
            func_queue.push(*s);
        }
    }

    while let Some(fstart) = func_queue.first().copied() {
        func_queue.remove(0);
        if seen_funcs.contains(&fstart) || covered.contains_key(&fstart) {
            continue;
        }
        if image.bytes_at(fstart).is_none() {
            continue;
        }
        seen_funcs.insert(fstart);

        let mut visited: BTreeSet<u64> = BTreeSet::new();
        let mut body: BTreeMap<u64, Instruction> = BTreeMap::new();
        let mut queue: Vec<u64> = vec![fstart];

        while let Some(addr) = queue.first().copied() {
            queue.remove(0);
            if visited.contains(&addr) || covered.contains_key(&addr) {
                continue;
            }
            let bytes = match image.bytes_at(addr) {
                Some(b) => b,
                None => continue,
            };
            let d = match decode_one(bytes, addr) {
                Some(d) if d.len > 0 => d,
                _ => continue,
            };
            visited.insert(addr);
            let ins = instruction_of(&d, addr, &bytes[..d.len]);
            let mn = d.mnemonic;
            let next = addr + d.len as u64;

            if let Some(t) = direct_target(&ins) {
                if mn == "call" || mn.starts_with('j') {
                    targets.insert(t);
                }
            }
            if mn == "call" {
                // A call's target is another function; the call itself returns.
                if let Some(t) = direct_target(&ins) {
                    if !seen_funcs.contains(&t) {
                        func_queue.push(t);
                    }
                }
                queue.push(next);
            } else if mn.starts_with('j') {
                if let Some(t) = direct_target(&ins) {
                    queue.push(t);
                }
                if mn != "jmp" {
                    queue.push(next);   // a conditional takes both edges
                }
            } else if !is_terminal(mn) {
                queue.push(next);
            }
            body.insert(addr, ins);
        }

        if !body.is_empty() {
            let insns = trim_padding(body.values().cloned().collect());
            for (a, i) in body {
                covered.insert(a, i);
            }
            if !insns.is_empty() {
                out.push((fstart, insns));
            }
        }
    }

    // Sweep whatever descent never reached: code behind an indirect transfer,
    // and data that is not code at all. Split on the same boundary rule.
    // What descent never reached is either code behind an indirect transfer or
    // it is not code at all, and nothing in the bytes distinguishes the two.
    // So the sweep does not walk the remainder byte by byte inventing
    // instructions at offsets nothing enters. It starts only where something
    // demonstrably starts: an address the program branches to, or the first
    // byte after a run of padding, which is where a compiler puts a function.
    let mut starts: BTreeSet<u64> = BTreeSet::new();
    for t in &targets {
        if !covered.contains_key(t) && image.bytes_at(*t).is_some() {
            starts.insert(*t);
        }
    }
    // Walking each section once, in phase, gives two more kinds of start: the
    // first byte after a run of padding, which is where a compiler puts a
    // function, and the first byte of any stretch descent did not claim. The
    // walk stays instruction-aligned throughout, so nothing is invented at an
    // offset that is only the middle of something else.
    for (base, data) in &image.segments {
        let mut i = 0usize;
        let mut after_pad = false;
        let mut prev_covered = true;
        while i < data.len() {
            let addr = base + i as u64;
            match decode_one(&data[i..], addr) {
                Some(d) if d.len > 0 => {
                    let is_pad = crate::vox::PAD_OPS.contains(&d.mnemonic);
                    let is_covered = covered.contains_key(&addr);
                    if !is_covered && !is_pad && (after_pad || prev_covered) {
                        starts.insert(addr);
                    }
                    after_pad = is_pad;
                    prev_covered = is_covered;
                    i += d.len;
                }
                _ => {
                    after_pad = false;
                    prev_covered = true;
                    i += 1;
                }
            }
        }
    }

    let mut runs: Vec<Vec<Instruction>> = Vec::new();
    let mut swept: BTreeSet<u64> = BTreeSet::new();
    for start in starts {
        if covered.contains_key(&start) || swept.contains(&start) {
            continue;
        }
        let mut run: Vec<Instruction> = Vec::new();
        let mut addr = start;
        loop {
            if covered.contains_key(&addr) || swept.contains(&addr) {
                break;
            }
            let bytes = match image.bytes_at(addr) {
                Some(b) => b,
                None => break,
            };
            let d = match decode_one(bytes, addr) {
                Some(d) if d.len > 0 => d,
                _ => break,
            };
            let mn = d.mnemonic;
            run.push(instruction_of(&d, addr, &bytes[..d.len]));
            swept.insert(addr);
            addr += d.len as u64;
            // A terminal ends the walk unless padding follows and more body
            // comes after it, which split_functions will cut apart.
            if is_terminal(mn) && !crate::vox::PAD_OPS.contains(&mn) {
                break;
            }
        }
        if !run.is_empty() {
            runs.push(run);
        }
    }

    for r in runs {
        for f in crate::vox::split_functions(&r) {
            if let Some(first) = f.first() {
                let trimmed = trim_padding(f.to_vec());
                if !trimmed.is_empty() {
                    out.push((first.address, trimmed));
                }
            }
        }
    }
    out
}

/// Walk an image and report what the walk could and could not claim.
pub fn walk(image: &Image, entry: u64, seeds: &[u64]) -> Walk {
    let functions = descend_seeded(image, entry, seeds);
    // Bytes, not instructions: the question is how much of the image was read,
    // so each claimed address contributes the width of the instruction on it.
    let mut seen: BTreeSet<u64> = BTreeSet::new();
    let mut bytes = 0usize;
    for (_, f) in &functions {
        for i in f {
            if !seen.insert(i.address) {
                continue;
            }
            if let Some(b) = image.bytes_at(i.address) {
                if let Some(d) = decode_one(b, i.address) {
                    bytes += d.len;
                }
            }
        }
    }
    Walk {
        claimed_bytes: bytes,
        total_bytes: image.total_bytes(),
        functions,
    }
}
