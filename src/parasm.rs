#![allow(dead_code)]
//! ParaASM — Practical Paraconsistent Universal Engine
//! Port of priests-engine/para_vm.py + imscribing_grammar/para/para_vm.py
//! Belnap FOUR VM with 19-instruction ISA, assembler, dialetheic alignment,
//! measurement sequence algebra, and IG snapshot bridge.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::belnap::B4;

// ── ParaASM instruction set ─────────────────────────────────────────

/// The 19-instruction ParaASM ISA.
/// Frobenius core (4), register ops (2), control flow (8), stack (2), I/O (2).
#[derive(Clone, Debug, PartialEq)]
pub enum ParaAsm {
    // ── Frobenius core (mirrors 4 of 12 IMASM tokens) ──
    ENGAGR(u8),              // Engage Both — register → paradox
    FSPLIT(u8, u8, u8),      // Fork — src → (dst1, dst2); B bifurcates to T+F
    FFUSE(Vec<u8>, u8),      // Fuse — join many sources → dst
    IFIX(u8),                // Permanent brand — linear ! exponential

    // ── Register ops ──
    MOVE(u8, u8),            // Copy src → dst
    CLEAR(u8),               // Reset register to N

    // ── Control flow ──
    JMP(String),             // Unconditional jump to label
    JB(u8, String),          // Jump if B
    JT(u8, String),          // Jump if T
    JF(u8, String),          // Jump if F
    JN(u8, String),          // Jump if N
    CALL(String),            // Call subroutine (push return addr)
    RET,                     // Return from subroutine
    HALT,                    // Stop execution

    // ── Stack ──
    PUSH(u8),                // Push register to data stack
    POP(u8),                 // Pop data stack → register

    // ── I/O (noop in kernel mode) ──
    EMIT(u8),                // Emit register value
    READ(u8),                // Read input → register (defaults N)
}

impl ParaAsm {
    pub fn op_name(&self) -> &'static str {
        match self {
            ParaAsm::ENGAGR(_)  => "ENGAGR",
            ParaAsm::FSPLIT(..) => "FSPLIT",
            ParaAsm::FFUSE(..)  => "FFUSE",
            ParaAsm::IFIX(_)    => "IFIX",
            ParaAsm::MOVE(..)   => "MOVE",
            ParaAsm::CLEAR(_)   => "CLEAR",
            ParaAsm::JMP(_)     => "JMP",
            ParaAsm::JB(..)     => "JB",
            ParaAsm::JT(..)     => "JT",
            ParaAsm::JF(..)     => "JF",
            ParaAsm::JN(..)     => "JN",
            ParaAsm::CALL(_)    => "CALL",
            ParaAsm::RET        => "RET",
            ParaAsm::HALT       => "HALT",
            ParaAsm::PUSH(_)    => "PUSH",
            ParaAsm::POP(_)     => "POP",
            ParaAsm::EMIT(_)    => "EMIT",
            ParaAsm::READ(_)    => "READ",
        }
    }

    /// Arity classification for kernel verification.
    pub fn is_frobenius(&self) -> bool {
        matches!(self, ParaAsm::ENGAGR(_) | ParaAsm::FSPLIT(..) |
                      ParaAsm::FFUSE(..) | ParaAsm::IFIX(_))
    }
}

// ── ParaRegister ────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ParaRegister {
    pub flux: B4,
    pub value: Option<&'static str>, // Some("FIXED") or None
    pub paradox_count: u32,
}

impl ParaRegister {
    pub fn new() -> Self {
        Self { flux: B4::N, value: None, paradox_count: 0 }
    }

    pub fn engage(&mut self) {
        self.flux = B4::B;
        self.paradox_count += 1;
    }

    pub fn is_fixed(&self) -> bool { self.value == Some("FIXED") }

    pub fn is_active(&self) -> bool {
        self.flux != B4::N || self.value.is_some()
    }

    pub fn clear(&mut self) {
        self.flux = B4::N;
        self.value = None;
    }
}

// ── Assembler ───────────────────────────────────────────────────────

pub struct AssembledProgram {
    pub instructions: Vec<ParaAsm>,
    pub label_map: BTreeMap<String, usize>,
}

/// Parse a register argument: "%r0" → 0, "%r15" → 15
fn parse_reg(arg: &str) -> Result<u8, String> {
    if arg.starts_with("%r") {
        arg[2..].parse::<u8>()
            .map_err(|_| format!("bad register: {}", arg))
    } else {
        Err(format!("expected %rN, got: {}", arg))
    }
}

/// Assemble ParaASM source text → (instructions, label_map).
/// Labels: `.name:` at start of line. Comments: `;` to end of line.
pub fn assemble(text: &str) -> Result<AssembledProgram, String> {
    let mut instrs: Vec<ParaAsm> = Vec::new();
    let mut labels: BTreeMap<String, usize> = BTreeMap::new();

    for (lineno, raw) in text.lines().enumerate() {
        // Strip comment
        let line = match raw.split(';').next() {
            Some(s) => String::from(s.trim()),
            None => continue,
        };
        if line.is_empty() { continue; }

        // Check for label: .name: [instruction]
        let (label, rest): (Option<String>, String) = if line.starts_with('.') {
            if let Some(colon) = line.find(':') {
                let lbl = String::from(&line[..colon]);
                let after = String::from(line[colon + 1..].trim());
                (Some(lbl), after)
            } else {
                (None, line)
            }
        } else {
            (None, line)
        };

        if let Some(lbl) = label {
            labels.insert(lbl, instrs.len());
        }

        if rest.is_empty() { continue; }

        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.is_empty() { continue; }

        let op = parts[0].to_uppercase();
        let args = &parts[1..];

        let instr = match op.as_str() {
            "ENGAGR" => {
                if args.len() < 1 { return Err(format!("line {}: ENGAGR needs 1 arg", lineno)); }
                ParaAsm::ENGAGR(parse_reg(args[0])?)
            }
            "FSPLIT" => {
                if args.len() < 3 { return Err(format!("line {}: FSPLIT needs 3 args", lineno)); }
                ParaAsm::FSPLIT(parse_reg(args[0])?, parse_reg(args[1])?, parse_reg(args[2])?)
            }
            "FFUSE" => {
                if args.len() < 2 { return Err(format!("line {}: FFUSE needs ≥2 args", lineno)); }
                let sources: Result<Vec<u8>, _> = args[..args.len()-1].iter()
                    .map(|a| parse_reg(a)).collect();
                let dst = parse_reg(args[args.len()-1])?;
                ParaAsm::FFUSE(sources?, dst)
            }
            "IFIX" => {
                if args.len() < 1 { return Err(format!("line {}: IFIX needs 1 arg", lineno)); }
                ParaAsm::IFIX(parse_reg(args[0])?)
            }
            "MOVE" => {
                if args.len() < 2 { return Err(format!("line {}: MOVE needs 2 args", lineno)); }
                ParaAsm::MOVE(parse_reg(args[0])?, parse_reg(args[1])?)
            }
            "CLEAR" => {
                if args.len() < 1 { return Err(format!("line {}: CLEAR needs 1 arg", lineno)); }
                ParaAsm::CLEAR(parse_reg(args[0])?)
            }
            "JMP" => {
                if args.len() < 1 { return Err(format!("line {}: JMP needs label", lineno)); }
                ParaAsm::JMP(String::from(args[0]))
            }
            "JB" => {
                if args.len() < 2 { return Err(format!("line {}: JB needs reg + label", lineno)); }
                ParaAsm::JB(parse_reg(args[0])?, String::from(args[1]))
            }
            "JT" => {
                if args.len() < 2 { return Err(format!("line {}: JT needs reg + label", lineno)); }
                ParaAsm::JT(parse_reg(args[0])?, String::from(args[1]))
            }
            "JF" => {
                if args.len() < 2 { return Err(format!("line {}: JF needs reg + label", lineno)); }
                ParaAsm::JF(parse_reg(args[0])?, String::from(args[1]))
            }
            "JN" => {
                if args.len() < 2 { return Err(format!("line {}: JN needs reg + label", lineno)); }
                ParaAsm::JN(parse_reg(args[0])?, String::from(args[1]))
            }
            "CALL" => {
                if args.len() < 1 { return Err(format!("line {}: CALL needs label", lineno)); }
                ParaAsm::CALL(String::from(args[0]))
            }
            "RET"  => ParaAsm::RET,
            "HALT" => ParaAsm::HALT,
            "PUSH" => {
                if args.len() < 1 { return Err(format!("line {}: PUSH needs 1 arg", lineno)); }
                ParaAsm::PUSH(parse_reg(args[0])?)
            }
            "POP" => {
                if args.len() < 1 { return Err(format!("line {}: POP needs 1 arg", lineno)); }
                ParaAsm::POP(parse_reg(args[0])?)
            }
            "EMIT" => {
                if args.len() < 1 { return Err(format!("line {}: EMIT needs 1 arg", lineno)); }
                ParaAsm::EMIT(parse_reg(args[0])?)
            }
            "READ" => {
                if args.len() < 1 { return Err(format!("line {}: READ needs 1 arg", lineno)); }
                ParaAsm::READ(parse_reg(args[0])?)
            }
            _ => return Err(format!("line {}: unknown opcode '{}'", lineno, op)),
        };
        instrs.push(instr);
    }

    Ok(AssembledProgram { instructions: instrs, label_map: labels })
}

// ── ParaVM ──────────────────────────────────────────────────────────

/// Practical Paraconsistent Universal Engine VM.
/// Belnap foundation: src/belnap.rs.
pub struct ParaVM {
    pub registers: BTreeMap<u8, ParaRegister>,
    pub belief: BTreeMap<u8, B4>,
    pub program: Vec<ParaAsm>,
    pub label_map: BTreeMap<String, usize>,
    pub pc: usize,
    pub total_steps: u64,
    pub cycles: u64,
    pub call_stack: Vec<usize>,
    pub data_stack: Vec<B4>,
    pub halted: bool,
    /// I/O capture buffer (EMIT writes here, READ reads from here if set)
    pub emit_buffer: Vec<String>,
    pub read_buffer: Option<Vec<B4>>,
    pub read_pos: usize,
}

impl ParaVM {
    pub fn new() -> Self {
        Self {
            registers: BTreeMap::new(),
            belief: BTreeMap::new(),
            program: Vec::new(),
            label_map: BTreeMap::new(),
            pc: 0,
            total_steps: 0,
            cycles: 0,
            call_stack: Vec::new(),
            data_stack: Vec::new(),
            halted: false,
            emit_buffer: Vec::new(),
            read_buffer: None,
            read_pos: 0,
        }
    }

    /// Get or create a register, returning belief as B4.
    pub fn belief_of(&self, reg_id: u8) -> B4 {
        self.belief.get(&reg_id).copied().unwrap_or(B4::N)
    }

    /// Set belief and update register flux.
    pub fn set_belief(&mut self, reg_id: u8, val: B4) {
        self.belief.insert(reg_id, val);
        self.registers.entry(reg_id).or_insert_with(ParaRegister::new).flux = val;
    }

    /// Engage a register — set to Both, increment paradox counter.
    pub fn engage(&mut self, reg_id: u8) {
        self.registers.entry(reg_id).or_insert_with(ParaRegister::new).engage();
        self.belief.insert(reg_id, B4::B);
    }

    /// Resolve a label to a PC address.
    pub fn resolve(&self, label: &str) -> Result<usize, String> {
        self.label_map.get(label)
            .copied()
            .ok_or_else(|| format!("undefined label: {}", label))
    }

    /// Load assembled program.
    pub fn load_program(&mut self, prog: AssembledProgram) {
        self.program = prog.instructions;
        self.label_map = prog.label_map;
        self.pc = 0;
        self.halted = false;
        self.total_steps = 0;
        self.cycles = 0;
    }

    /// Assemble and load source text.
    pub fn load(&mut self, text: &str) -> Result<(), String> {
        let prog = assemble(text)?;
        self.load_program(prog);
        Ok(())
    }

    /// Pre-set read buffer (for deterministic testing).
    pub fn set_reads(&mut self, values: Vec<B4>) {
        self.read_buffer = Some(values);
        self.read_pos = 0;
    }
}

// ── Execution ───────────────────────────────────────────────────────

impl ParaVM {
    /// Execute one instruction (internal).
    fn _exec(&mut self, instr: &ParaAsm) {
        match instr {
            ParaAsm::ENGAGR(r) => {
                self.engage(*r);
            }
            ParaAsm::FSPLIT(src, d1, d2) => {
                let b = self.belief_of(*src);
                let reg = self.registers.entry(*src).or_insert_with(ParaRegister::new);
                let p = reg.paradox_count;
                if b == B4::B {
                    self.set_belief(*d1, B4::T);
                    self.set_belief(*d2, B4::F);
                    let bump = p + 1;
                    self.registers.entry(*d1).or_insert_with(ParaRegister::new).paradox_count = bump;
                    self.registers.entry(*d2).or_insert_with(ParaRegister::new).paradox_count = bump;
                } else {
                    self.set_belief(*d1, b);
                    self.set_belief(*d2, b);
                    self.registers.entry(*d1).or_insert_with(ParaRegister::new).paradox_count = p;
                    self.registers.entry(*d2).or_insert_with(ParaRegister::new).paradox_count = p;
                }
            }
            ParaAsm::FFUSE(sources, dst) => {
                let mut joined = self.belief_of(sources[0]);
                for src in &sources[1..] {
                    joined = joined.join(self.belief_of(*src));
                }
                self.set_belief(*dst, joined);
            }
            ParaAsm::IFIX(r) => {
                self.registers.entry(*r).or_insert_with(ParaRegister::new).value = Some("FIXED");
                self.set_belief(*r, B4::T);
            }
            ParaAsm::MOVE(src, dst) => {
                let v = self.belief_of(*src);
                self.set_belief(*dst, v);
            }
            ParaAsm::CLEAR(r) => {
                self.registers.entry(*r).or_insert_with(ParaRegister::new).clear();
                self.belief.insert(*r, B4::N);
            }
            ParaAsm::JMP(label) => {
                if let Ok(addr) = self.resolve(label) {
                    self.pc = addr;
                }
            }
            ParaAsm::JB(r, label) => {
                if self.belief_of(*r) == B4::B {
                    if let Ok(addr) = self.resolve(label) { self.pc = addr; }
                }
            }
            ParaAsm::JT(r, label) => {
                if self.belief_of(*r) == B4::T {
                    if let Ok(addr) = self.resolve(label) { self.pc = addr; }
                }
            }
            ParaAsm::JF(r, label) => {
                if self.belief_of(*r) == B4::F {
                    if let Ok(addr) = self.resolve(label) { self.pc = addr; }
                }
            }
            ParaAsm::JN(r, label) => {
                if self.belief_of(*r) == B4::N {
                    if let Ok(addr) = self.resolve(label) { self.pc = addr; }
                }
            }
            ParaAsm::CALL(label) => {
                if let Ok(addr) = self.resolve(label) {
                    self.call_stack.push(self.pc);
                    self.pc = addr;
                }
            }
            ParaAsm::RET => {
                if let Some(addr) = self.call_stack.pop() {
                    self.pc = addr;
                } else {
                    self.halted = true;
                }
            }
            ParaAsm::HALT => {
                self.halted = true;
            }
            ParaAsm::PUSH(r) => {
                self.data_stack.push(self.belief_of(*r));
            }
            ParaAsm::POP(r) => {
                let val = self.data_stack.pop().unwrap_or(B4::N);
                self.set_belief(*r, val);
            }
            ParaAsm::EMIT(r) => {
                let b = self.belief_of(*r);
                let fixed = if self.registers.get(r).map_or(false, |reg| reg.is_fixed()) {
                    " [FIXED]"
                } else { "" };
                self.emit_buffer.push(format!("%r{} = {}{}", r, b.name(), fixed));
            }
            ParaAsm::READ(r) => {
                let val = if let Some(ref buf) = self.read_buffer {
                    let v = buf.get(self.read_pos).copied().unwrap_or(B4::N);
                    self.read_pos += 1;
                    v
                } else {
                    B4::N // kernel-mode default
                };
                self.set_belief(*r, val);
            }
        }
    }

    /// Single step. Returns false if halted or program empty.
    pub fn step(&mut self) -> bool {
        if self.halted || self.program.is_empty() {
            return false;
        }
        if self.pc >= self.program.len() {
            self.pc = 0;
            self.cycles += 1;
        }
        let instr = self.program[self.pc].clone();
        self.pc += 1;
        self.total_steps += 1;
        self._exec(&instr);
        !self.halted
    }

    /// Run up to `steps` instructions (None = run until halt).
    pub fn run(&mut self, steps: Option<usize>) {
        let mut n = 0;
        while !self.halted {
            if let Some(max) = steps {
                if n >= max { break; }
            }
            if !self.step() { break; }
            n += 1;
        }
    }

    /// Execute a single instruction directly (no program needed).
    pub fn exec_one(&mut self, instr: &ParaAsm) {
        self.total_steps += 1;
        self._exec(instr);
    }

    /// Reset VM state.
    pub fn reset(&mut self) {
        *self = ParaVM::new();
    }
}

// ── Snapshot (for IG bridge) ────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ParaVmSnapshot {
    pub steps: u64,
    pub cycles: u64,
    pub pc: usize,
    pub active: usize,
    pub fixed: usize,
    pub paradox: u32,
    pub dist_n: u32,
    pub dist_t: u32,
    pub dist_f: u32,
    pub dist_b: u32,
    pub halted: bool,
    pub data_stack_depth: usize,
    pub call_stack_depth: usize,
}

impl ParaVM {
    pub fn snapshot(&self) -> ParaVmSnapshot {
        let (mut dist_n, mut dist_t, mut dist_f, mut dist_b) = (0u32, 0u32, 0u32, 0u32);
        let mut paradox = 0u32;
        let mut active = 0usize;
        let mut fixed = 0usize;

        // Collect all register IDs
        let mut ids: Vec<u8> = self.registers.keys().chain(self.belief.keys()).copied().collect();
        ids.sort();
        ids.dedup();

        for rid in ids {
            let b = self.belief_of(rid);
            match b {
                B4::N => dist_n += 1,
                B4::T => dist_t += 1,
                B4::F => dist_f += 1,
                B4::B => dist_b += 1,
            }
            if let Some(reg) = self.registers.get(&rid) {
                paradox += reg.paradox_count;
                if reg.is_active() { active += 1; }
                if reg.is_fixed() { fixed += 1; }
            }
        }

        ParaVmSnapshot {
            steps: self.total_steps,
            cycles: self.cycles,
            pc: self.pc,
            active,
            fixed,
            paradox,
            dist_n, dist_t, dist_f, dist_b,
            halted: self.halted,
            data_stack_depth: self.data_stack.len(),
            call_stack_depth: self.call_stack.len(),
        }
    }

    /// Active registers: (id, belief, paradox_count, is_fixed).
    pub fn active_regs(&self) -> Vec<(u8, B4, u32, bool)> {
        let mut ids: Vec<u8> = self.registers.keys().chain(self.belief.keys()).copied().collect();
        ids.sort();
        ids.dedup();
        ids.into_iter()
            .filter(|rid| {
                self.registers.get(rid).map_or(false, |r| r.is_active())
                || self.belief.contains_key(rid)
            })
            .map(|rid| {
                let reg = self.registers.get(&rid);
                (
                    rid,
                    self.belief_of(rid),
                    reg.map_or(0, |r| r.paradox_count),
                    reg.map_or(false, |r| r.is_fixed()),
                )
            })
            .collect()
    }
}

// ── Dialetheic Alignment ────────────────────────────────────────────

/// Dialetheic image: B → B, T/F → T, N → N.
pub fn dialetheic_image(r0: B4) -> B4 {
    match r0 {
        B4::B => B4::B,
        B4::T | B4::F => B4::T,
        B4::N => B4::N,
    }
}

/// B is the only bifurcation point under FSPLIT.
/// ∀r: if r=B then FSPLIT(r)→(T,F) with T≠F; if r≠B then both outputs equal.
pub fn b_is_only_bifurcation_point() -> bool {
    for &r in &[B4::N, B4::T, B4::F, B4::B] {
        // simulate fsplit
        let (d1, d2) = if r == B4::B { (B4::T, B4::F) } else { (r, r) };
        if r == B4::B && d1 == d2 { return false; }
        if r != B4::B && d1 != d2 { return false; }
    }
    true
}

/// Three-arm dialetheic alignment check.
/// Returns (operational, logical, algebraic).
pub fn dialetheic_alignment_tri() -> (bool, bool, bool) {
    // operational: ffuse∘fsplit(B) = B ∧ B is only bifurcation
    let op = {
        let (d1, d2) = (B4::T, B4::F); // fsplit(B)
        d1.join(d2) == B4::B // ffuse
            && b_is_only_bifurcation_point()
    };
    // logical: B is dialetheic, nothing else is
    let log = B4::B.dialetheic()
        && ![B4::N, B4::T, B4::F].iter().any(|x| x.dialetheic());
    // algebraic: N not designated, T∨F=B, designated(B∧¬B)
    let alg = !B4::N.designated()
        && B4::T.join(B4::F) == B4::B
        && B4::B.band(B4::B.bnot()).designated();
    (op, log, alg)
}

// ── Measurement Sequence Algebra ────────────────────────────────────

/// Cost of measuring q with bias.
/// B→B: cost 2, B→T/F: cost 1, non-B: cost 0.
pub fn measure_cost(q: B4, bias: B4) -> u8 {
    if q != B4::B { return 0; }
    if bias == B4::B { 2 } else { 1 }
}

/// Single measurement step: collapse B under bias.
pub fn measure_step(q: B4, bias: B4) -> B4 {
    if q == B4::B {
        if bias == B4::B { B4::B } else { bias }
    } else {
        q
    }
}

/// Is q irreversible once collapsed? (T/F/N cannot return to B via single ops.)
pub fn collapse_irreversible(q: B4) -> bool {
    if q == B4::B { return true; }
    // Check: none of bnot, join(q,q), meet(q,q), band(q,q), bor(q,q) returns B
    ![q.bnot(), q.join(q), q.meet(q), q.band(q), q.bor(q)].contains(&B4::B)
}

/// Wigner's-friend-then-collapse cost: B→B measurement then collapse = 3.
pub fn wigner_then_collapse_cost(n: u32) -> u32 { 3 * n }

// ── Kernel-state bridge (mirrors p4ramill_py.kernel) ────────────────

#[derive(Clone, Debug)]
pub struct KernelState {
    pub r0: B4,
    pub r1: B4,
    pub r2: B4,
    pub paradox_count: u32,
    pub cycle_count: u32,
}

impl KernelState {
    pub fn new() -> Self {
        Self { r0: B4::B, r1: B4::B, r2: B4::B, paradox_count: 0, cycle_count: 0 }
    }

    /// Apply one frobenius kernel step: fsplit(r0)→(r1,r2), ffuse(r1,r2)→r0.
    pub fn kernel_step(&mut self) {
        let (d1, d2) = if self.r0 == B4::B {
            self.paradox_count += 1;
            (B4::T, B4::F)
        } else {
            (self.r0, self.r0)
        };
        self.r1 = d1;
        self.r2 = d2;
        self.r0 = d1.join(d2); // ffuse
        self.cycle_count += 1;
    }

    /// Run n kernel steps.
    pub fn kernel_run(&mut self, n: u32) {
        for _ in 0..n { self.kernel_step(); }
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_simple() {
        let src = "
            .start:
            ENGAGR %r0
            FSPLIT %r0 %r1 %r2
            FFUSE %r1 %r2 %r0
            IFIX %r0
            HALT
        ";
        let prog = assemble(src).unwrap();
        assert_eq!(prog.instructions.len(), 5);
        assert_eq!(prog.label_map.get(".start"), Some(&0));
        assert_eq!(prog.instructions[0], ParaAsm::ENGAGR(0));
        assert_eq!(prog.instructions[4], ParaAsm::HALT);
    }

    #[test]
    fn test_vm_basic_cycle() {
        let mut vm = ParaVM::new();
        vm.load("
            ENGAGR %r0
            FSPLIT %r0 %r1 %r2
            FFUSE %r1 %r2 %r0
            HALT
        ").unwrap();
        vm.run(None);
        let snap = vm.snapshot();
        // After ENGAGR: r0=B, paradox=1
        // After FSPLIT: r1=T, r2=F, paradox=2
        // After FFUSE: r0=T∨F=B
        assert!(snap.halted);
        assert_eq!(snap.paradox, 2);
        assert_eq!(vm.belief_of(0), B4::B);
    }

    #[test]
    fn test_frobenius_identity() {
        // ffuse(fsplit(r)) == r for all r
        for &r in &[B4::N, B4::T, B4::F, B4::B] {
            let (d1, d2) = if r == B4::B { (B4::T, B4::F) } else { (r, r) };
            assert_eq!(d1.join(d2), r, "frobenius failed for {:?}", r);
        }
    }

    #[test]
    fn test_bifurcation_point() {
        assert!(b_is_only_bifurcation_point());
    }

    #[test]
    fn test_dialetheic_alignment() {
        let (op, log, alg) = dialetheic_alignment_tri();
        assert!(op, "operational arm failed ");
        assert!(log, "logical arm failed ");
        assert!(alg, "algebraic arm failed ");
    }

    #[test]
    fn test_measurement_algebra() {
        assert_eq!(measure_step(B4::B, B4::B), B4::B);
        assert_eq!(measure_step(B4::B, B4::T), B4::T);
        assert_eq!(measure_cost(B4::B, B4::B), 2);
        assert_eq!(measure_cost(B4::B, B4::T), 1);
        assert_eq!(measure_cost(B4::T, B4::T), 0);
        assert!(collapse_irreversible(B4::T));
        assert!(collapse_irreversible(B4::F));
        assert!(collapse_irreversible(B4::N));
        assert_eq!(wigner_then_collapse_cost(1), 3);
    }

    #[test]
    fn test_kernel_state_loop() {
        let mut ks = KernelState::new();
        for _ in 0..8 {
            ks.kernel_step();
            assert_eq!(ks.r0, B4::B);
            assert_eq!(ks.r1, B4::B);
            assert_eq!(ks.r2, B4::B);
        }
    }

    #[test]
    fn test_push_pop() {
        let mut vm = ParaVM::new();
        vm.set_belief(0, B4::T);
        vm.set_belief(1, B4::B);
        vm.exec_one(&ParaAsm::PUSH(0));
        vm.exec_one(&ParaAsm::PUSH(1));
        vm.exec_one(&ParaAsm::POP(2));
        assert_eq!(vm.belief_of(2), B4::B);
        vm.exec_one(&ParaAsm::POP(3));
        assert_eq!(vm.belief_of(3), B4::T);
    }

    #[test]
    fn test_call_ret() {
        let mut vm = ParaVM::new();
        vm.load("
            JMP .main
        .sub:
            IFIX %r1
            RET
        .main:
            MOVE %r0 %r1
            CALL .sub
            HALT
        ").unwrap();
        vm.set_belief(0, B4::T);
        vm.run(None);
        let snap = vm.snapshot();
        assert!(snap.halted);
        assert!(vm.registers.get(&1).map_or(false, |r| r.is_fixed()));
    }
}
