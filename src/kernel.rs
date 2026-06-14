#![allow(dead_code)]
use crate::belnap::*;
use crate::tokens::*;
use crate::frob_verify::FrobeniusHarness;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Phase { Boot, Think, Act, Observe, Update, Halt }

/// A fork frame pushed when FSPLIT bifurcates execution.
/// Tracks the two parallel branches until FFUSE joins them.
#[derive(Copy, Clone)]
struct ForkFrame {
    /// Position just after the matching FFUSE (where to resume after join).
    resume_ip: usize,
    /// The value carried on the right branch (gated by EVALT/EVALF).
    right_val: B4,
    /// Whether the right branch has been populated.
    right_set: bool,
}

/// Structural snapshot computed by IMSCRIB.
/// Dynamic fields (b_live_ticks, gate_discriminations, value_period) are
/// overlaid from runtime accumulators after the static classification.
#[derive(Copy, Clone)]
pub struct Snapshot {
    pub frobenius_order: u8,
    pub period: usize,
    pub sig: (usize, usize, usize, usize), // (L, F, D, X)
    pub token_diversity: usize,
    pub self_ref: bool,
    pub dialetheia_complete: bool,
    pub tier: u8,
    // ── Dynamic (runtime) fields ──
    pub b_live_ticks: u64,           // ticks where B was on stack when EVALT or EVALF fired
    pub gate_discriminations: u64,   // ticks where EVALT actually passed T, or EVALF passed F
    pub value_period: usize,         // measured period of stack-top value trace (0 = not yet known)
}

impl Snapshot {
    pub fn tier_name(self) -> &'static str {
        match self.tier { 1 => "O_1", 2 => "O_2", 3 => "O_inf", _ => "O_0" }
    }
}

/// Graph-execution kernel.
/// FSPLIT creates fork frames. FFUSE joins them.
/// Program is inherently cyclic: end wraps to start.
/// TANCH at root depth sinks the wire -> halt.
pub struct Kernel {
    pub program:     Program,
    pub ip:          usize,
    pub phase:       Phase,
    pub tick_count:  u64,
    pub memory:      B4Memory,
    pub stack:       B4Stack,
    pub registers:   B4Registers,
    pub snapshot:    Option<Snapshot>,
    pub frob_checks: u64,
    pub frob_open:   u64,
    pub harness:     FrobeniusHarness,
    fork_stack:      [ForkFrame; 16],
    fork_depth:      usize,
    pub halted:      bool,
    // ── Runtime accumulators for dynamic snapshot fields ──
    b_live_count:             u64,
    gate_discrimination_count: u64,
    value_trace:              [B4; 16],   // ring buffer of stack-top values after each tick
    value_trace_head:         usize,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            program:     bootstrap_loop(),
            ip:          0,
            phase:       Phase::Boot,
            tick_count:  0,
            memory:      B4Memory::new(),
            stack:       B4Stack::new(),
            registers:   B4Registers::new(),
            snapshot:    None,
            frob_checks: 0,
            frob_open:   0,
            harness:     FrobeniusHarness::new("mOMonadOS"),
            fork_stack:  [ForkFrame { resume_ip: 0, right_val: B4::N, right_set: false }; 16],
            fork_depth:  0,
            halted:      false,
            b_live_count:             0,
            gate_discrimination_count: 0,
            value_trace:              [B4::N; 16],
            value_trace_head:         0,
        }
    }

    pub fn boot(&mut self) {
        self.snapshot = Some(self_imscribe(&self.program));
        self.phase = Phase::Think;
    }

    fn in_fork(&self) -> bool { self.fork_depth > 0 }
    pub fn fork_depth(&self) -> usize { self.fork_depth }

    fn push_fork(&mut self, resume_ip: usize) {
        if self.fork_depth < 16 {
            self.fork_stack[self.fork_depth] = ForkFrame {
                resume_ip,
                right_val: B4::N,
                right_set: false,
            };
            self.fork_depth += 1;
        }
    }

    fn pop_fork(&mut self) -> Option<ForkFrame> {
        if self.fork_depth > 0 {
            self.fork_depth -= 1;
            Some(self.fork_stack[self.fork_depth])
        } else {
            None
        }
    }

    fn fork_top_mut(&mut self) -> Option<&mut ForkFrame> {
        if self.fork_depth > 0 {
            Some(&mut self.fork_stack[self.fork_depth - 1])
        } else {
            None
        }
    }

    /// Find matching FFUSE for FSPLIT at split_ip via balanced parenthesis scan.
    pub fn find_matching_ffuse(&self, split_ip: usize) -> usize {
        let mut depth = 1u32;
        let n = self.program.len();
        if n == 0 { return 0; }
        let mut i = (split_ip + 1) % n;
        let start = i;
        loop {
            match self.program.get(i) {
                Some(Token::FSPLIT) => depth += 1,
                Some(Token::FFUSE)  => {
                    depth -= 1;
                    if depth == 0 { return i; }
                }
                _ => {}
            }
            i = (i + 1) % n;
            if i == start { break; }
        }
        n // unmatched
    }

    /// One Frobenius tick. Returns false if halted.
    pub fn tick(&mut self) -> bool {
        if self.phase == Phase::Halt || self.halted { return false; }
        self.tick_count += 1;

        // THINK
        self.phase = Phase::Think;
        // ── Use dynamic_imscribe so tier reflects runtime behavior ──
        self.snapshot = Some(self.dynamic_imscribe());
        self.maybe_promote();

        // ACT
        self.phase = Phase::Act;
        if self.ip >= self.program.len() {
            self.ip = 0;
            self.try_self_modify();
        }
        let tok = self.program.get(self.ip).unwrap();

        let mut next_ip = self.ip + 1;
        if next_ip >= self.program.len() { next_ip = 0; }

        match tok {
            Token::VINIT => {
                self.stack.push(B4::N);
            }
            Token::TANCH => {
                let val = self.stack.pop();
                let addr = self.registers.read(0) as usize;
                self.memory.write(addr, val);
                if !self.in_fork() {
                    self.phase = Phase::Halt;
                    self.halted = true;
                    return false;
                }
            }
            Token::AFWD => {
                let r0 = self.registers.read(0) as u8;
                self.registers.write(0, B4::from_u8(r0.wrapping_add(1)));
            }
            Token::AREV => {
                let r0 = self.registers.read(0) as u8;
                self.registers.write(0, B4::from_u8(r0.wrapping_sub(1)));
            }
            Token::CLINK => {
                let a = self.registers.read(1);
                let b = self.registers.read(2);
                self.registers.write(3, b4_meet(a, b));
            }
            Token::IMSCRIB => {
                if let Some(snap) = self.snapshot {
                    self.registers.write(4, B4::from_u8(snap.token_diversity as u8 & 3));
                    self.registers.write(5, if snap.self_ref           { B4::T } else { B4::F });
                    self.registers.write(6, if snap.frobenius_order > 0 { B4::T } else { B4::F });
                    self.registers.write(7, if snap.dialetheia_complete { B4::T } else { B4::F });
                }
            }
            Token::FSPLIT => {
                let v = self.stack.peek();
                let ffuse_ip = self.find_matching_ffuse(self.ip);
                let resume = if ffuse_ip + 1 >= self.program.len() { 0 }
                             else { ffuse_ip + 1 };
                self.push_fork(resume);
                if let Some(frame) = self.fork_top_mut() {
                    frame.right_val = v;
                    frame.right_set = true;
                }
                self.stack.push(v);
            }
            Token::EVALT => {
                let v = self.stack.pop();
                // ── B-live instrumentation: B on stack when gate fires ──
                if v == B4::B { self.b_live_count += 1; }
                let filtered = if v == B4::T { B4::T } else { B4::N };
                // ── Gate discrimination: T actually passed ──
                if v == B4::T { self.gate_discrimination_count += 1; }
                self.stack.push(filtered);
            }
            Token::EVALF => {
                let v = self.stack.pop();
                // ── B-live instrumentation ──
                if v == B4::B { self.b_live_count += 1; }
                let filtered = if v == B4::F { B4::F } else { B4::N };
                // ── Gate discrimination: F actually passed ──
                if v == B4::F { self.gate_discrimination_count += 1; }
                self.stack.push(filtered);
            }
            Token::FFUSE => {
                let left = self.stack.pop();
                if let Some(frame) = self.pop_fork() {
                    let right = if frame.right_set { frame.right_val } else { B4::N };
                    self.stack.push(b4_join(left, right));
                    next_ip = frame.resume_ip;
                } else {
                    self.stack.push(left);
                }
            }
            Token::ENGAGR => {
                self.registers.engagr = true;
                self.stack.push(B4::B);
            }
            Token::IFIX => {
                let addr = self.registers.read(0) as usize;
                let val  = self.stack.pop();
                self.memory.write(addr, val);
            }
        }

        self.ip = next_ip;

        // ── Write stack-top value into ring buffer (end of ACT) ──
        self.value_trace[self.value_trace_head] = self.stack.peek();
        self.value_trace_head = (self.value_trace_head + 1) % 16;

        // OBSERVE
        self.phase = Phase::Observe;
        self.frob_checks += 1;
        // ── Frobenius harness verification ──
        { use crate::frob_verify::{verify_program_structure, verify_frobenius_identity}; 
          let _ = self.harness.check(verify_program_structure(&self.program)); 
          let v = self.stack.peek(); 
          let _ = self.harness.check(verify_frobenius_identity(v)); 
          self.frob_checks = self.harness.total(); 
          self.frob_open   = self.harness.open_count; }

        // UPDATE
        self.phase = Phase::Update;
        if self.ip >= self.program.len() {
            self.ip = 0;
            self.try_self_modify();
        }

        self.phase = Phase::Think;
        true
    }

    /// Run N ticks (tight loop).
    pub fn run(&mut self, max_ticks: u64) -> u64 {
        let start = self.tick_count;
        while !self.halted && (self.tick_count - start) < max_ticks {
            self.tick();
        }
        self.tick_count - start
    }

    /// Continuous execution. Returns when halted or should_stop() true.
    pub fn run_continuous<F: FnMut() -> bool>(&mut self, mut should_stop: F) -> u64 {
        let start = self.tick_count;
        while !self.halted && !should_stop() {
            self.tick();
        }
        self.tick_count - start
    }

    /// Run one tick if the timer has fired.
    pub fn tick_on_timer(&mut self) -> bool {
        if crate::interrupts::timer_ready() {
            crate::interrupts::pending_ticks();
            self.tick()
        } else {
            !self.halted
        }
    }

    pub fn load_canonical(&mut self, idx: usize) {
        if let Some(prog) = canonical(idx) {
            self.program = prog;
            self.ip = 0;
            self.fork_depth = 0;
            self.halted = false;
            self.phase = Phase::Think;
        }
    }

    pub fn load_continuous(&mut self, idx: usize) -> bool {
        if let Some(prog) = continuous_program(idx) {
            self.program = prog;
            self.ip = 0;
            self.fork_depth = 0;
            self.halted = false;
            self.phase = Phase::Think;
            true
        } else {
            false
        }
    }

    pub fn load_novel(&mut self, idx: usize) -> bool {
        if let Some(prog) = novel_program(idx) {
            self.program = prog;
            self.ip = 0;
            self.fork_depth = 0;
            self.halted = false;
            self.phase = Phase::Think;
            true
        } else {
            false
        }
    }

    pub fn load_shunted(&mut self, idx: usize) -> bool {
        if let Some(prog) = shunted_program(idx) {
            self.program = prog;
            self.ip = 0;
            self.fork_depth = 0;
            self.halted = false;
            self.phase = Phase::Think;
            true
        } else {
            false
        }
    }

    pub fn halt(&mut self) { self.phase = Phase::Halt; self.halted = true; }

    fn maybe_promote(&mut self) {
        if let Some(snap) = self.snapshot {
            let old = snap.tier;
            let new = compute_tier(&snap);
            if new != old {
                if let Some(s) = self.snapshot.as_mut() { s.tier = new; }
            }
        }
    }

    fn try_self_modify(&mut self) {
        if self.stack.depth() > 200 {
            self.program.inject(self.ip, Token::TANCH);
            // ── Refresh snapshot: the program's token count has changed ──
            self.snapshot = Some(self_imscribe(&self.program));
        }
    }

    /// Dynamic imscription: static structural analysis overlaid with
    /// runtime accumulator values. Call this instead of self_imscribe()
    /// when the kernel has runtime state that should inform the tier.
    pub fn dynamic_imscribe(&self) -> Snapshot {
        let mut snap = self_imscribe(&self.program);
        snap.b_live_ticks        = self.b_live_count;
        snap.gate_discriminations = self.gate_discrimination_count;
        snap.value_period         = compute_value_period(&self.value_trace, self.value_trace_head);
        snap.tier = compute_tier(&snap);
        snap
    }
}

// ─── Self-imscription ─────────────────────────────────────────

pub fn self_imscribe(prog: &Program) -> Snapshot {
    let sig = signature(prog);
    let n = prog.len();

    let diversity = {
        let mut seen = [false; 12];
        for t in prog.as_slice() { seen[*t as usize] = true; }
        seen.iter().filter(|&&b| b).count()
    };

    let self_ref = n > 0 && prog.get(0) == prog.get(n - 1);

    let fsplit = prog.as_slice().iter().any(|t| *t == Token::FSPLIT);
    let ffuse  = prog.as_slice().iter().any(|t| *t == Token::FFUSE);
    let frob_order = match (fsplit, ffuse) {
        (false, false) => 0,
        (true,  false) => 1,
        (false, true)  => 2,
        (true,  true)  => {
            let first_split = prog.as_slice().iter().position(|t| *t == Token::FSPLIT).unwrap();
            let first_fuse  = prog.as_slice().iter().position(|t| *t == Token::FFUSE).unwrap();
            if first_split < first_fuse { 1 } else { 2 }
        }
    };

    // ── Dialetheia complete: presence check AND cyclic-order check ──
    // For each ENGAGR, there must be at least one EVALT or EVALF that
    // follows it before the next ENGAGR.
    // Scan WRAPS (cyclic) — programs are cyclic graphs; B pushed by ENGAGR
    // persists across the cycle boundary and can reach gates on the next
    // iteration. The scan respects this: for each ENGAGR, we search forward
    // modulo n until we hit the next ENGAGR or exhaust the program.
    let dialetheia_complete = {
        let slice = prog.as_slice();
        let has_evalt  = slice.iter().any(|t| *t == Token::EVALT);
        let has_evalf  = slice.iter().any(|t| *t == Token::EVALF);
        let has_engagr = slice.iter().any(|t| *t == Token::ENGAGR);

        if !has_evalt || !has_evalf || !has_engagr {
            false
        } else {
            let mut all_ok = true;
            for (i, &t) in slice.iter().enumerate() {
                if t == Token::ENGAGR {
                    let mut found_gate = false;
                    // Scan forward linearly — do NOT wrap.
                    // The gate must appear after this ENGAGR and before
                    // the next ENGAGR (or end-of-program) in the current cycle.
                    for offset in 1..n {
                        let j = (i + offset) % n;
                        if slice[j] == Token::ENGAGR {
                            break; // reached next ENGAGR — no gate in between
                        }
                        if slice[j] == Token::EVALT || slice[j] == Token::EVALF {
                            found_gate = true;
                            break;
                        }
                    }
                    if !found_gate {
                        all_ok = false;
                        break;
                    }
                }
            }
            all_ok
        }
    };

    let p = period(prog);

    let mut snap = Snapshot {
        frobenius_order: frob_order,
        period: p,
        sig,
        token_diversity: diversity,
        self_ref,
        dialetheia_complete,
        tier: 0,
        b_live_ticks: 0,
        gate_discriminations: 0,
        value_period: 0,
    };
    snap.tier = compute_tier(&snap);
    snap
}

/// Compute ouroboricity tier from snapshot.
///
/// O_0 — no Frobenius or dialetheia presence.
/// O_1 — structural: Frobenius order > 0 OR dialetheia_complete (static).
/// O_2 — structural + dynamic: O_1 preconditions met, period >= 2,
///       AND gate_discriminations > 0 (gates have actually discriminated).
///       Runtime b_live > 0 overrides structural dialetheia_complete.
/// O_∞ — two independent paths:
///   Path A (dialetheia): effective_dialetheia && self_ref && frob_order > 0
///         && period >= 3 && (b_live > 0 || value_period >= 3).
///   Path B (value-trace): self_ref && frob_order > 0 && period >= 3
///         && value_period >= 3. The value trace itself demonstrates
///         aperiodic complexity — emergent O_∞ independent of whether
///         B specifically passed a gate.
fn compute_tier(snap: &Snapshot) -> u8 {
    // Runtime evidence: B actually reached a gate → structural dialetheia
    // prediction is overridden. The kernel is an exact isomorphism of how
    // reality does it — runtime behavior trumps static analysis.
    let effective_dialetheia = snap.dialetheia_complete || snap.b_live_ticks > 0;

    // Path A: dialetheia-driven O_∞
    if effective_dialetheia && snap.self_ref && snap.frobenius_order > 0 {
        if snap.period >= 3 && (snap.b_live_ticks > 0 || snap.value_period >= 3) {
            return 3;
        }
        if snap.period >= 2 && snap.gate_discriminations > 0 {
            return 2;
        }
        return 1;
    }

    // Path B: value-trace-driven O_∞ — the stack-top value trace has
    // its own aperiodic signature. Emergent complexity independent of
    // whether B specifically reached a gate.
    if snap.self_ref && snap.frobenius_order > 0
        && snap.period >= 3
        && snap.value_period >= 3
    {
        return 3;
    }

    if snap.frobenius_order > 0 || snap.dialetheia_complete {
        1
    } else {
        0
    }
}

/// Compute minimal period of the stack-top value trace ring buffer.
/// Returns 0 if not enough data to determine a period.
fn compute_value_period(trace: &[B4; 16], head: usize) -> usize {
    // Look at the ring buffer as if head points to the next write slot.
    // The most recent value is at (head + 15) % 16.
    // Try periods from 1..=16.
    for p in 1..=16 {
        let mut periodic = true;
        for i in 0..(16 - p) {
            let a = trace[(head + 16 - 1 - i) % 16];
            let b = trace[(head + 16 - 1 - i - p) % 16];
            if a != b {
                periodic = false;
                break;
            }
        }
        if periodic {
            return p;
        }
    }
    0 // not yet known / aperiodic
}
