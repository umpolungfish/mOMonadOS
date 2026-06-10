use crate::belnap::*;
use crate::tokens::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Phase { Boot, Think, Act, Observe, Update, Halt }

/// Structural snapshot computed by ISCRIB.
#[derive(Copy, Clone)]
pub struct Snapshot {
    pub frobenius_order: u8,   // 0=none 1=split→fuse 2=fuse→split 3=both
    pub period: usize,
    pub sig: (usize, usize, usize, usize), // (L, F, D, X)
    pub token_diversity: usize,
    pub self_ref: bool,
    pub dialetheia_complete: bool,
    pub tier: u8,              // 0=O_0 1=O_1 2=O_2 3=O_inf
}

impl Snapshot {
    pub fn tier_name(self) -> &'static str {
        match self.tier { 1 => "O_1", 2 => "O_2", 3 => "O_inf", _ => "O_0" }
    }
}

pub struct Kernel {
    pub program:     Program,
    pub ip:          usize,
    pub phase:       Phase,
    pub tick_count:  u64,
    pub cycle_count: u64,
    pub memory:      B4Memory,
    pub stack:       B4Stack,
    pub registers:   B4Registers,
    pub snapshot:    Option<Snapshot>,
    pub frob_checks: u64,
    pub frob_open:   u64,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            program:     bootstrap_loop(),
            ip:          0,
            phase:       Phase::Boot,
            tick_count:  0,
            cycle_count: 0,
            memory:      B4Memory::new(),
            stack:       B4Stack::new(),
            registers:   B4Registers::new(),
            snapshot:    None,
            frob_checks: 0,
            frob_open:   0,
        }
    }

    pub fn boot(&mut self) {
        self.snapshot = Some(self_imscribe(&self.program));
        self.phase = Phase::Think;
    }

    /// One Frobenius tick: THINK → ACT → OBSERVE → UPDATE.
    pub fn tick(&mut self) -> bool {
        if self.phase == Phase::Halt { return false; }
        self.tick_count += 1;

        // THINK
        self.phase = Phase::Think;
        self.snapshot = Some(self_imscribe(&self.program));
        self.maybe_promote();

        // ACT
        self.phase = Phase::Act;
        if self.ip >= self.program.len() {
            self.phase = Phase::Halt;
            return false;
        }
        let tok = self.program.get(self.ip).unwrap();
        self.dispatch(tok);
        self.ip += 1;

        // OBSERVE — simplified Frobenius check
        self.phase = Phase::Observe;
        self.frob_checks += 1;

        // UPDATE
        self.phase = Phase::Update;
        if self.ip >= self.program.len() {
            self.ip = 0;
            self.cycle_count += 1;
            self.try_self_modify();
        }

        self.phase = Phase::Think;
        true
    }

    pub fn run(&mut self, max_ticks: u64) {
        let start = self.tick_count;
        while self.phase != Phase::Halt && (self.tick_count - start) < max_ticks {
            self.tick();
        }
    }

    pub fn load_canonical(&mut self, idx: usize) {
        if let Some(prog) = canonical(idx) {
            self.program = prog;
            self.ip = 0;
        }
    }

    pub fn halt(&mut self) { self.phase = Phase::Halt; }

    fn dispatch(&mut self, tok: Token) {
        match tok {
            Token::VINIT  => self.stack.push(B4::N),
            Token::TANCH  => {
                let addr = self.registers.read(0) as usize;
                let val  = self.stack.pop();
                self.memory.write(addr, val);
            }
            Token::AFWD   => {
                let r0 = self.registers.read(0) as u8;
                self.registers.write(0, B4::from_u8(r0.wrapping_add(1)));
            }
            Token::AREV   => {
                let r0 = self.registers.read(0) as u8;
                self.registers.write(0, B4::from_u8(r0.wrapping_sub(1)));
            }
            Token::CLINK  => {
                let a = self.registers.read(1);
                let b = self.registers.read(2);
                self.registers.write(3, b4_meet(a, b));
            }
            Token::ISCRIB => {
                if let Some(snap) = self.snapshot {
                    self.registers.write(4, B4::from_u8(snap.token_diversity as u8 & 3));
                    self.registers.write(5, if snap.self_ref           { B4::T } else { B4::F });
                    self.registers.write(6, if snap.frobenius_order > 0 { B4::T } else { B4::F });
                    self.registers.write(7, if snap.dialetheia_complete { B4::T } else { B4::F });
                }
            }
            Token::FSPLIT => {
                let v = self.stack.peek();
                self.stack.push(v);
            }
            Token::FFUSE  => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(b4_join(a, b));
            }
            Token::EVALT  => self.stack.push(B4::T),
            Token::EVALF  => self.stack.push(B4::F),
            Token::ENGAGR => {
                self.registers.engagr = true;
                self.stack.push(B4::B);
            }
            Token::IFIX   => {
                let addr = self.registers.read(0) as usize;
                let val  = self.stack.pop();
                self.memory.write(addr, val);
            }
        }
    }

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
        // Stack overflow guard
        if self.stack.depth() > 200 {
            self.program.inject(self.ip, Token::TANCH);
        }
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

    let dialetheia_complete =
        prog.as_slice().iter().any(|t| *t == Token::EVALT) &&
        prog.as_slice().iter().any(|t| *t == Token::EVALF) &&
        prog.as_slice().iter().any(|t| *t == Token::ENGAGR);

    let p = period(prog);

    let mut snap = Snapshot {
        frobenius_order: frob_order,
        period: p,
        sig,
        token_diversity: diversity,
        self_ref,
        dialetheia_complete,
        tier: 0,
    };
    snap.tier = compute_tier(&snap);
    snap
}

fn compute_tier(snap: &Snapshot) -> u8 {
    if snap.dialetheia_complete && snap.self_ref && snap.frobenius_order > 0 {
        if snap.period >= 3 { 3 } // O_inf
        else if snap.period == 2 { 2 } // O_2
        else { 1 } // O_1
    } else if snap.frobenius_order > 0 || snap.dialetheia_complete {
        1 // O_1
    } else {
        0 // O_0
    }
}
