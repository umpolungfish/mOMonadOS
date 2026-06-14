#![allow(dead_code)]
/// Crystal of Types — 17,280,000-address structural type space.
///
/// Address = Σᵢ (primitive_index[i] × STRIDE[i])
/// Strides: [5184000, 1728000, 576000, 144000, 48000, 12000, 4000, 800, 200, 50, 10, 1]
/// Cardinalities (D,T,R,P,F,K,G,C,Phi,H,S,Omega): [4,5,4,5,3,5,3,4,5,4,3,4]

/// Total number of distinct structural types in the crystal.
/// Computed from the product of all primitive family cardinalities.
/// CARDS = [4,5,4,5,3,5,3,4,5,4,3,4]; product = 17_280_000.
pub const TOTAL: u32 = CARDS[0] * CARDS[1] * CARDS[2] * CARDS[3]
                      * CARDS[4] * CARDS[5] * CARDS[6] * CARDS[7]
                      * CARDS[8] * CARDS[9] * CARDS[10] * CARDS[11];

const CARDS: [u32; 12] = [4, 5, 4, 5, 3, 5, 3, 4, 5, 4, 3, 4];

const STRIDES: [u32; 12] = {
    let mut s = [1u32; 12];
    let mut i = 11usize;
    loop {
        if i == 0 { break; }
        s[i - 1] = s[i] * CARDS[i];
        i -= 1;
    }
    s
};

/// Encode a 12-tuple of primitive indices (0-based, each < cardinality) to address.
pub fn encode(indices: &[u8; 12]) -> u32 {
    let mut addr = 0u32;
    for i in 0..12 {
        addr += indices[i] as u32 * STRIDES[i];
    }
    addr
}

/// Decode address to 12 primitive indices.
pub fn decode(mut addr: u32) -> [u8; 12] {
    let mut idx = [0u8; 12];
    for i in 0..12 {
        idx[i] = (addr / STRIDES[i]) as u8;
        addr %= STRIDES[i];
    }
    idx
}

/// Derive crystal indices from kernel structural snapshot.
///
/// Mapping:
///   D(4)     ← frobenius_order
///   T(5)     ← period % 5
///   R(4)     ← sig.0 (Logical) % 4
///   P(5)     ← sig.1 (Frobenius) % 5
///   F(3)     ← sig.2 (Dialetheia) % 3
///   K(5)     ← sig.3 (Linear) % 5
///   G(3)     ← token_diversity % 3
///   C(4)     ← (self_ref<<1 | dialetheia_complete) % 4
///   Phi(5)   ← tier
///   H(4)     ← program_len % 4
///   S(3)     ← sig_sum % 3
///   Omega(4) ← (period + frobenius_order) % 4
pub fn indices_from_snapshot(
    frobenius_order: u8,
    period: usize,
    sig: (usize, usize, usize, usize), // (L, F, D, X)
    token_diversity: usize,
    self_ref: bool,
    dialetheia_complete: bool,
    tier: u8,
    program_len: usize,
) -> [u8; 12] {
    let sig_sum = (sig.0 + sig.1 + sig.2 + sig.3) as u8;
    [
        frobenius_order & 3,
        (period as u8) % 5,
        (sig.0 as u8) % 4,
        (sig.1 as u8) % 5,
        (sig.2 as u8) % 3,
        (sig.3 as u8) % 5,
        (token_diversity as u8) % 3,
        ((self_ref as u8) << 1 | (dialetheia_complete as u8)) & 3,
        tier % 5,
        (program_len as u8) % 4,
        sig_sum % 3,
        ((period as u8).wrapping_add(frobenius_order)) & 3,
    ]
}

/// 64-entry crystal store (in-memory, fixed capacity for bare-metal).
pub struct CrystalStore {
    entries: [Option<CrystalEntry>; 64],
    count: usize,
}

#[derive(Clone, Copy)]
pub struct CrystalEntry {
    pub address: u32,
    pub name: [u8; 32],
    pub data: [u8; 64],
    pub canonical_idx: u8,
}

impl CrystalEntry {
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..end]).unwrap_or("")
    }

    pub fn data_str(&self) -> &str {
        let end = self.data.iter().position(|&b| b == 0).unwrap_or(64);
        core::str::from_utf8(&self.data[..end]).unwrap_or("")
    }
}

impl CrystalStore {
    pub const fn new() -> Self {
        Self { entries: [None; 64], count: 0 }
    }

    pub fn store(&mut self, name: &str, data: &str, address: u32, canonical_idx: u8) -> u32 {
        for slot in self.entries.iter_mut() {
            if let Some(e) = slot {
                if e.address == address {
                    let mut ne = *e;
                    Self::fill_str(&mut ne.name, name);
                    Self::fill_str(&mut ne.data, data);
                    ne.canonical_idx = canonical_idx;
                    *slot = Some(ne);
                    return address;
                }
            }
        }
        if self.count < 64 {
            let mut entry = CrystalEntry {
                address,
                name: [0u8; 32],
                data: [0u8; 64],
                canonical_idx,
            };
            Self::fill_str(&mut entry.name, name);
            Self::fill_str(&mut entry.data, data);
            self.entries[self.count] = Some(entry);
            self.count += 1;
        }
        address
    }

    pub fn read_by_addr(&self, addr: u32) -> Option<&CrystalEntry> {
        self.entries.iter().filter_map(|s| s.as_ref()).find(|e| e.address == addr)
    }

    pub fn read_by_name(&self, name: &str) -> Option<&CrystalEntry> {
        self.entries.iter().filter_map(|s| s.as_ref()).find(|e| e.name_str() == name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CrystalEntry> {
        self.entries.iter().filter_map(|s| s.as_ref())
    }

    pub fn count(&self) -> usize { self.count }

    fn fill_str(buf: &mut [u8], s: &str) {
        let bytes = s.as_bytes();
        let n = bytes.len().min(buf.len() - 1);
        buf[..n].copy_from_slice(&bytes[..n]);
        buf[n] = 0;
    }
}
