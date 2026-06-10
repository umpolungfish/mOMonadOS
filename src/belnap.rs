/// Belnap FOUR truth values.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum B4 {
    N = 0, // None  — void, absence
    T = 1, // True  — affirmation
    F = 2, // False — negation
    B = 3, // Both  — paradox stabilized
}

impl B4 {
    pub fn name(self) -> &'static str {
        match self { B4::N => "N", B4::T => "T", B4::F => "F", B4::B => "B" }
    }

    pub fn from_u8(v: u8) -> Self {
        match v & 0b11 { 1 => B4::T, 2 => B4::F, 3 => B4::B, _ => B4::N }
    }
}

pub fn b4_meet(a: B4, b: B4) -> B4 {
    B4::from_u8((a as u8) & (b as u8))
}

pub fn b4_join(a: B4, b: B4) -> B4 {
    B4::from_u8((a as u8) | (b as u8))
}

/// 4096-cell B4 memory, 2 bits per cell packed into a byte array.
pub struct B4Memory {
    data: [u8; 1024], // 4096 cells × 2 bits = 1024 bytes
}

impl B4Memory {
    pub const fn new() -> Self {
        Self { data: [0u8; 1024] }
    }

    pub fn read(&self, addr: usize) -> B4 {
        let addr = addr & 0xFFF;
        let byte = self.data[addr / 4];
        let shift = (addr % 4) * 2;
        B4::from_u8((byte >> shift) & 0b11)
    }

    pub fn write(&mut self, addr: usize, val: B4) {
        let addr = addr & 0xFFF;
        let shift = (addr % 4) * 2;
        let mask = !(0b11u8 << shift);
        self.data[addr / 4] = (self.data[addr / 4] & mask) | ((val as u8) << shift);
    }
}

/// 256-deep B4 stack.
pub struct B4Stack {
    data: [B4; 256],
    top: usize,
}

impl B4Stack {
    pub const fn new() -> Self {
        Self { data: [B4::N; 256], top: 0 }
    }

    pub fn push(&mut self, v: B4) {
        if self.top < 256 {
            self.data[self.top] = v;
            self.top += 1;
        }
    }

    pub fn pop(&mut self) -> B4 {
        if self.top == 0 { return B4::N; }
        self.top -= 1;
        self.data[self.top]
    }

    pub fn peek(&self) -> B4 {
        if self.top == 0 { B4::N } else { self.data[self.top - 1] }
    }

    pub fn depth(&self) -> usize { self.top }
}

/// 8 × B4 register file.
pub struct B4Registers {
    regs: [B4; 8],
    pub engagr: bool,
}

impl B4Registers {
    pub const fn new() -> Self {
        Self { regs: [B4::N; 8], engagr: false }
    }

    pub fn read(&self, i: usize) -> B4 { self.regs[i & 7] }

    pub fn write(&mut self, i: usize, v: B4) { self.regs[i & 7] = v; }
}
