#![allow(dead_code)]
use crate::tokens::{period as tok_period, signature, Program, Token};
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

/// Imscribe a running program into the crystal — the operational witness of each
/// of the 12 cosmic primitives, NOT a modular projection onto them.
///
/// Each axis reads the actual token-graph structure and returns the index of the
/// Lean constructor (0-based, in Core.lean order) that the program instantiates.
/// The axes marked EXACT are structurally forced identities; the others are
/// grounded structural readings of the meaning the Lean names.
///
///   D  Dimensionality [dead<ash<array<if']  ← recursive nesting depth (self_ref ⇒ if')
///   T  Topology       [judge<eat<mime<oil<are] ← fork/loop wiring (self_ref ⇒ are)
///   R  Relational     [ado<tot<ear<ian]     ← AFWD/AREV/CLINK morphism mode
///   P  Polarity       [church<yew<out<nun<or'] ← EXACT: μ∘δ=id closure ⇒ or'
///   F  Fidelity       [age<they<peep]       ← IFIX (!) lossless brand ⇒ peep
///   K  Kinetic        [yea<loll<egg<on<air] ← trapping: halt vs diversity vs mixing
///   G  Granularity    [bib<thigh<ice]       ← token diversity (scope)
///   Γ  Grammar        [vow<gag<measure<ooze] ← fork resolution / sequence / broadcast
///   Φ  Criticality    [woe<monad<roar<err<haha] ← self_ref ⇒ monad (⊙ fixed point)
///   H  Chirality      [fee<kick<sure<wool]  ← EXACT: ROTAT period (chirality under shift)
///   S  Stoichiometry  [hung<so<up]          ← EXACT: FSPLIT/FFUSE (δ/μ) balance
///   Ω  Protection     [awe<oak<ah<zoo]      ← winding: rotational period + fork order
pub fn indices_from_program(
    p: &Program,
    frobenius_order: u8,
    self_ref: bool,
    dialetheia_complete: bool,
) -> [u8; 12] {
    let len = p.len();
    let per = tok_period(p);
    let (l, f, d, x) = signature(p);

    // Per-token census (Token is #[repr(u8)] 0x0..0xB).
    let mut c = [0u32; 12];
    for &t in p.as_slice() { c[t as usize] += 1; }
    let n_tanch  = c[Token::TANCH  as usize];
    let n_afwd   = c[Token::AFWD   as usize];
    let n_arev   = c[Token::AREV   as usize];
    let n_clink  = c[Token::CLINK  as usize];
    let n_fsplit = c[Token::FSPLIT as usize];
    let n_ffuse  = c[Token::FFUSE  as usize];
    let n_evalt  = c[Token::EVALT  as usize];
    let n_evalf  = c[Token::EVALF  as usize];
    let n_engagr = c[Token::ENGAGR as usize];
    let n_ifix   = c[Token::IFIX   as usize];

    // ── Forced structural predicates only (no free numeric threshold) ──
    let halts    = n_tanch > 0;                       // TANCH sinks the wire
    let forked   = n_fsplit > 0 || n_ffuse > 0;       // δ/μ present
    let balanced = n_fsplit == n_ffuse;               // δ/μ conserved
    // m = rotational symmetry multiplicity = how many times the ring repeats.
    // Exact integer: period always divides len (period is the minimal divisor).
    let m = if per > 0 { (len / per) as u32 } else { 1 };
    // fams = number of the 4 families present. A completeness predicate, not a size.
    let fams = (l > 0) as u32 + (f > 0) as u32 + (d > 0) as u32 + (x > 0) as u32;

    // D — nesting depth. if' = holographic (boundary encodes bulk, self-imscription).
    let dim = if self_ref { 3 } else if frobenius_order >= 2 { 2 }
              else if frobenius_order == 1 { 1 } else { 0 };

    // T — wiring topology. are ⇔ self_ref (Axiom C); eat = nested; mime = fork bifurcation;
    // oil = fork-free ring with rotational symmetry (regular lattice/torus).
    let top = if self_ref { 4 }
              else if frobenius_order >= 2 { 1 }
              else if forked { 2 }
              else if m >= 2 { 3 } else { 0 };

    // R — relational mode by morphism-token presence (forced).
    let rel = if n_afwd > 0 && n_arev > 0 { 3 }   // ian: lateral both-way
              else if n_arev > 0 { 2 }             // ear: dagger reciprocal A⊣A†
              else if n_clink > 0 { 1 }            // tot: categorical composition
              else { 0 };                          // ado: hierarchical one-way

    // P — or' ⇔ μ∘δ=id closure (tier singularity); ENGAGR ⇒ out (ℤ₂ at ⊙);
    // both gates ⇒ nun; one gate ⇒ yew.
    let pol = if dialetheia_complete && self_ref { 4 }
              else if n_engagr > 0 { 2 }
              else if n_evalt > 0 && n_evalf > 0 { 3 }
              else if n_evalt > 0 || n_evalf > 0 { 1 } else { 0 };

    // F — peep ⇔ IFIX (linear ! lossless brand); they ⇔ dialetheia threshold.
    let fid = if n_ifix > 0 { 2 }
              else if n_engagr > 0 || dialetheia_complete { 1 } else { 0 };

    // K — kinetic trapping (forced by halt, period=1, multiplicity, family-completeness):
    // on = trapped by order (halt + crystalline); egg = halt through structured steps;
    // yea = ergodic (aperiodic, all families); air = MBL (aperiodic, families missing);
    // loll = periodic non-halting (moderate).
    let kin = if halts && per <= 1 { 3 }
              else if halts { 2 }
              else if m == 1 && fams == 4 { 0 }
              else if m == 1 { 4 }
              else { 1 };

    // G — scope by family-completeness (forced): all-to-all ⇒ ice.
    let gran = if fams >= 4 { 2 } else if fams >= 2 { 1 } else { 0 };

    // Γ — composition rule. ooze = broadcast (self_ref); measure = fork-free sequence;
    // vow = conjunctive (fork balanced+resolved); gag = disjunctive (fork open).
    let gram = if self_ref { 3 }
               else if !forked { 2 }
               else if balanced && dialetheia_complete { 0 } else { 1 };

    // Φ — criticality (forced): monad ⇔ self-modeling ⊙; woe = halts stable;
    // roar = complex/dialetheic (ENGAGR); err = exceptional point (unbalanced fork
    // coalescence); haha = supercritical runaway (aperiodic, no halt, no closure).
    let crit = if self_ref { 1 }
               else if halts { 0 }
               else if n_engagr > 0 { 2 }
               else if forked && !balanced { 3 } else { 4 };

    // H — EXACT: chirality under the shift IS the ROTAT period class.
    let chir = if per <= 1 { 0 } else if per == 2 { 1 }
               else if per < len { 2 } else { 3 };

    // S — EXACT: stoichiometry is the δ/μ (FSPLIT/FFUSE) conservation balance.
    let stoi = if !forked { 0 } else if balanced { 1 } else { 2 };

    // Ω — protection = winding, dual to H: H is the period class, Ω the multiplicity m.
    // zoo = non-Abelian (nested forks); ah = ℤ winding (m≥3); oak = ℤ₂ (m=2); awe = none.
    let prot = if frobenius_order >= 2 { 3 }
               else if m >= 3 { 2 }
               else if m == 2 { 1 } else { 0 };

    [dim, top, rel, pol, fid, kin, gran, gram, crit, chir, stoi, prot]
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
