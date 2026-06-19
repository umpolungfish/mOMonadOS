//! imas.rs — IMASM Arranger Bridge
//! Port of imas/arranger.py + imas/ig_bridge.py + imas/clink_bridge.py + imas/frobenius_hunter.py
//!
//! Bridges IMASM token space (12 tokens, 430M length-8 arrangements) to
//! the IG crystal (17.28M types) and the CLINK biological chain (L0–L8).

use crate::rebis::pipeline::IgTuple;

// ═══════════════════════════════════════════════════════════════
// IMASM Token Definitions
// ═══════════════════════════════════════════════════════════════

pub const VINIT:   u8 = 0;
pub const TANCH:   u8 = 1;
pub const AFWD:    u8 = 2;
pub const AREV:    u8 = 3;
pub const CLINK:   u8 = 4;
pub const IMSCRIB: u8 = 5;
pub const FSPLIT:  u8 = 6;
pub const FFUSE:   u8 = 7;
pub const EVALT:   u8 = 8;
pub const EVALF:   u8 = 9;
pub const ENGAGR:  u8 = 10;
pub const IFIX:    u8 = 11;

pub const TOKEN_COUNT: usize = 12;

pub fn token_name(t: u8) -> &'static str {
    match t {
        0 => "VINIT", 1 => "TANCH", 2 => "AFWD", 3 => "AREV",
        4 => "CLINK", 5 => "IMSCRIB", 6 => "FSPLIT", 7 => "FFUSE",
        8 => "EVALT", 9 => "EVALF", 10 => "ENGAGR", 11 => "IFIX",
        _ => "???",
    }
}

pub fn token_family(t: u8) -> u8 {
    match t {
        0..=5 => 0,   // Logical
        6|7 => 1,     // Frobenius
        8..=10 => 2,  // Dialetheia
        11 => 3,      // Linear
        _ => 255,
    }
}

pub const FAMILY_NAMES: [&str; 4] = ["Logical", "Frobenius", "Dialetheia", "Linear"];

/// (logical, frobenius, dialetheia, linear) counts
pub fn signature(arr: &[u8]) -> (u8, u8, u8, u8) {
    let (mut l, mut f, mut d, mut x) = (0u8, 0u8, 0u8, 0u8);
    for &t in arr {
        match token_family(t) {
            0 => l += 1,
            1 => f += 1,
            2 => d += 1,
            3 => x += 1,
            _ => {},
        }
    }
    (l, f, d, x)
}

// ═══════════════════════════════════════════════════════════════
// Structural Fingerprint
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct StructFingerprint {
    pub length: usize,
    pub sig_l: u8,
    pub sig_f: u8,
    pub sig_d: u8,
    pub sig_x: u8,
    pub start_token: u8,
    pub end_token: u8,
    pub self_ref: bool,
    pub frobenius_order: u8,    // 0=none, 1=split→fuse, 2=fuse→split, 3=both
    pub dialetheia_complete: bool,
    pub period: u8,
    pub token_mask: u16,
}

impl StructFingerprint {
    pub fn token_diversity(&self) -> u32 {
        self.token_mask.count_ones()
    }

    pub fn signature(&self) -> (u8, u8, u8, u8) {
        (self.sig_l, self.sig_f, self.sig_d, self.sig_x)
    }
}

/// Compute fingerprint from an arrangement of tokens.
pub fn fingerprint(arr: &[u8]) -> StructFingerprint {
    let n = arr.len();
    let sig = signature(arr);
    let start = if n > 0 { arr[0] } else { 0 };
    let end = if n > 0 { arr[n - 1] } else { 0 };

    // Frobenius order
    let mut frob_order = 0u8;
    let mut has_split = false;
    let mut has_fuse = false;
    for i in 0..n {
        if arr[i] == FSPLIT { has_split = true; }
        if arr[i] == FFUSE { has_fuse = true; }
        if i + 1 < n && arr[i] == FSPLIT && arr[i + 1] == FFUSE { frob_order = 1; }
        if i + 1 < n && arr[i] == FFUSE && arr[i + 1] == FSPLIT { frob_order = 2; }
    }
    if has_split && has_fuse && frob_order == 0 { frob_order = 3; }

    // Dialetheia complete: all three present
    let mut has_evalt = false; let mut has_evalf = false; let mut has_engagr = false;
    for &t in arr {
        if t == EVALT { has_evalt = true; }
        if t == EVALF { has_evalf = true; }
        if t == ENGAGR { has_engagr = true; }
    }
    let dial_complete = has_evalt && has_evalf && has_engagr;

    // Period
    let period = if n <= 1 { n as u8 } else {
        let mut p = 1u8;
        'outer: while p <= (n as u8) / 2 {
            for i in (p as usize)..n {
                if arr[i] != arr[i - p as usize] { p += 1; continue 'outer; }
            }
            break;
        }
        p
    };

    // Token mask
    let mut mask: u16 = 0;
    for &t in arr { mask |= 1u16 << (t as u16); }

    StructFingerprint {
        length: n, sig_l: sig.0, sig_f: sig.1, sig_d: sig.2, sig_x: sig.3,
        start_token: start, end_token: end, self_ref: start == end,
        frobenius_order: frob_order, dialetheia_complete: dial_complete,
        period, token_mask: mask,
    }
}

// ═══════════════════════════════════════════════════════════════
// Fingerprint → IG Tuple Mapping
// ═══════════════════════════════════════════════════════════════

/// Map a StructuralFingerprint to an IG tuple.
/// Uses the deterministic mapping from ig_bridge.py.
pub fn fingerprint_to_ig(fp: &StructFingerprint) -> IgTuple {
    use crate::rebis::pipeline::*;

    let td = fp.token_diversity();
    let d = if td <= 2 { Dim::Wedge }
        else if td <= 5 { Dim::Triangle }
        else if td <= 9 { Dim::Infty }
        else { Dim::Odot };

    let t = if fp.self_ref { Top::Odot }
        else if fp.period == 1 { Top::Net }
        else if fp.period == 2 { Top::Bowtie }
        else if fp.frobenius_order > 0 { Top::Boxtimes }
        else { Top::In };

    let r = match fp.frobenius_order {
        1 => Coup::Lr,
        2 => Coup::Dagger,
        3 => Coup::Cat,
        _ => Coup::Super,
    };

    let p = if fp.frobenius_order == 1 { Par::PmSym }
        else if fp.frobenius_order == 2 { Par::Sym }
        else if fp.frobenius_order == 3 { Par::Pm }
        else if fp.dialetheia_complete { Par::Psi }
        else { Par::Asym };

    let f = if fp.dialetheia_complete { Fid::Hbar }
        else if fp.period == 1 { Fid::Ell }
        else { Fid::Eth };

    let sx = fp.sig_x;
    let k = if sx == 8 { Kin::Mod }
        else if fp.period == 1 { Kin::Slow }
        else if fp.period <= 4 { Kin::Trap }
        else { Kin::Mbl };

    let g = if sx >= 3 { Car::Beth }
        else if sx >= 1 { Car::Aleph }
        else if td <= 3 { Car::Gimel }
        else { Car::Aleph };

    let gm = if fp.frobenius_order > 0 { Comp::Seq }
        else if fp.period == 1 { Comp::And }
        else if fp.period == 2 { Comp::Or }
        else { Comp::Broad };

    let ph = if fp.self_ref && fp.dialetheia_complete { Cri::C }
        else if fp.self_ref { Cri::CComplex }
        else if fp.dialetheia_complete { Cri::Ep }
        else if fp.period == 1 { Cri::Sub }
        else { Cri::Super };

    let h = match fp.period {
        1 => Chi::H0,
        2 => Chi::H1,
        3 => Chi::H2,
        _ => Chi::HInf,
    };

    let nz = (fp.sig_l > 0) as u8 + (fp.sig_f > 0) as u8 + (fp.sig_d > 0) as u8 + (fp.sig_x > 0) as u8;
    let s = if nz == 1 { Sto::One }
        else if nz == 2 { Sto::Many }
        else { Sto::Het };

    let w = if fp.frobenius_order == 1 { Win::Z }
        else if fp.frobenius_order == 2 { Win::Z2 }
        else if fp.self_ref { Win::Z }
        else if fp.period == 2 { Win::Z2 }
        else { Win::Zero };

    IgTuple { d, t, r, p, f, k, g, gm, ph, h, s, w }
}

// ═══════════════════════════════════════════════════════════════
// 12 Canonical IMASM Arrangements
// ═══════════════════════════════════════════════════════════════

pub const CANONICAL_NAMES: [&str; 12] = [
    "I_Dialetheic_Bootstrap", "II_Void_Genesis", "III_Anchor_Protocol",
    "IV_Dual_Bootstrap", "V_Linear_Chain", "VI_Empty_Bootstrap",
    "VII_Parakernel", "VIII_Frobenius_Kernel", "IX_Chiral_Pairs",
    "X_Truth_Machine", "XI_Eternal_Return", "XII_ROM_Burn",
];

/// 12 canonical IMASM token sequences
pub fn canonical_sequence(idx: usize) -> Option<&'static [u8]> {
    match idx {
        0 => Some(&[VINIT, TANCH, AFWD, AREV, CLINK, IMSCRIB, EVALT, EVALF]),
        1 => Some(&[VINIT, TANCH, IFIX]),
        2 => Some(&[VINIT, TANCH, AFWD, CLINK, IMSCRIB, EVALT, ENGAGR, IFIX]),
        3 => Some(&[AFWD, AREV, CLINK, IMSCRIB, VINIT, TANCH, EVALT, EVALF]),
        4 => Some(&[VINIT, TANCH, AFWD, AREV, CLINK, IMSCRIB, IFIX, IFIX]),
        5 => Some(&[]),
        6 => Some(&[VINIT, TANCH, AFWD, AREV, FSPLIT, FFUSE, CLINK, IMSCRIB]),
        7 => Some(&[FSPLIT, AFWD, FFUSE, AREV, FSPLIT, CLINK, FFUSE, IMSCRIB]),
        8 => Some(&[AFWD, AREV, CLINK, IMSCRIB, AREV, AFWD, IMSCRIB, CLINK]),
        9 => Some(&[EVALT, EVALF, ENGAGR, IFIX, EVALT, EVALF, ENGAGR, IFIX]),
        10 => Some(&[VINIT, TANCH, AFWD, AREV, CLINK, IMSCRIB, VINIT, TANCH]),
        11 => Some(&[VINIT, TANCH, IFIX, IFIX, IFIX, IFIX, IFIX, IFIX]),
        _ => None,
    }
}

/// Get the IG tuple for a canonical arrangement.
pub fn canonical_ig(idx: usize) -> Option<IgTuple> {
    canonical_sequence(idx).map(|seq| {
        let fp = fingerprint(seq);
        fingerprint_to_ig(&fp)
    })
}

// ═══════════════════════════════════════════════════════════════
// CLINK Bridge — IMASM arrangement → biological layer
// ═══════════════════════════════════════════════════════════════

/// Bridge an IMASM arrangement to the nearest CLINK biological layer.
pub fn imasm_to_clink(arr: &[u8]) -> (usize, &'static str, f64, &'static str) {
    let fp = fingerprint(arr);
    let ig = fingerprint_to_ig(&fp);
    // Find nearest CLINK layer
    let (idx, dist) = crate::rebis::clink::nearest_clink_layer(&ig);
    let name = crate::rebis::clink::CLINK_NAMES[idx];
    let tier = crate::rebis::clink::CLINK_TIERS[idx];
    (idx, name, dist, tier)
}

/// Full IMASM→CLINK bridge report for a canonical arrangement.
pub fn bridge_report(idx: usize) -> alloc::string::String {
    let name = if idx < 12 { CANONICAL_NAMES[idx] } else { "Unknown" };
    match canonical_sequence(idx) {
        Some(seq) => {
            let (cl_idx, cl_name, dist, tier) = imasm_to_clink(seq);
            let fp = fingerprint(seq);
            let ig = fingerprint_to_ig(&fp);
            alloc::format!(
                "══ IMASM→CLINK Bridge: {} ══\n  Tokens: {}\n  IG: {}\n  Nearest: L{} {}\n  Distance: {:.3}  Tier: {}",
                name,
                seq.iter().map(|&t| token_name(t)).collect::<alloc::vec::Vec<_>>().join(" "),
                crate::rebis::clink::format_tuple_glyphs(&ig),
                cl_idx, cl_name, dist, tier,
            )
        }
        None => alloc::format!("Canonical {} not found.", idx),
    }
}

/// Bridge all 12 canonicals to CLINK.
pub fn bridge_all_report() -> alloc::string::String {
    let mut s = alloc::string::String::new();
    s.push_str("══ IMASM→CLINK Bridge (All 12 Canonicals) ══\n");
    for i in 0..12 {
        if let Some(seq) = canonical_sequence(i) {
            let (cl_idx, cl_name, dist, tier) = imasm_to_clink(seq);
            let tok_str = seq.iter().map(|&t| token_name(t)).collect::<alloc::vec::Vec<_>>().join(" ");
            s.push_str(&alloc::format!(
                "  {:2} {:24} → L{} {:20}  d={:.2} {}\n  {}\n",
                i, CANONICAL_NAMES[i], cl_idx, cl_name, dist, tier, tok_str,
            ));
        }
    }
    s
}

// ═══════════════════════════════════════════════════════════════
// Frobenius Hunter
// ═══════════════════════════════════════════════════════════════

/// Check if a token sequence has FSPLIT followed by FFUSE.
pub fn has_frobenius_pair(arr: &[u8]) -> bool {
    let mut saw_split = false;
    for &t in arr {
        if t == FSPLIT { saw_split = true; }
        if t == FFUSE && saw_split { return true; }
    }
    false
}

/// Frobenius closure check for an arrangement.
pub fn frobenius_check(arr: &[u8]) -> bool {
    has_frobenius_pair(arr)
}

/// Verify IMASM bootstrap sequence is Frobenius-complete.
pub fn verify_bootstrap(arr: &[u8]) -> alloc::string::String {
    let fp = fingerprint(arr);
    let has_pair = has_frobenius_pair(arr);
    let has_imsc = arr.iter().any(|&t| t == IMSCRIB);
    let has_clink = arr.iter().any(|&t| t == CLINK);
    let has_ifix = arr.iter().any(|&t| t == IFIX);
    let all_ok = has_pair && has_imsc && has_clink && has_ifix;
    alloc::format!(
        "══ IMASM Bootstrap Verification ══\n  Frobenius pair: {}\n  IMSCRIB: {}\n  CLINK: {}\n  IFIX: {}\n  VERDICT: {}",
        if has_pair { "PASS" } else { "FAIL" },
        if has_imsc { "✓" } else { "✗" },
        if has_clink { "✓" } else { "✗" },
        if has_ifix { "✓" } else { "✗" },
        if all_ok { "FROBENIUS-COMPLETE" } else { "OPEN" },
    )
}

/// IMASM summary with fingerprint table.
pub fn imasm_summary() -> alloc::string::String {
    let mut s = alloc::string::String::new();
    s.push_str("══ IMASM Arranger ══\n");
    s.push_str("  Tokens: 12 (6 Logical, 2 Frobenius, 3 Dialetheia, 1 Linear)\n");
    s.push_str("  Arrangement space: 430,981,696 length-8 sequences\n");
    s.push_str("  12 canonical classes (I–XII)\n");
    s.push_str("\n  Canonical Fingerprints:\n");
    for i in 0..12 {
        if let Some(seq) = canonical_sequence(i) {
            let fp = fingerprint(seq);
            let frob = has_frobenius_pair(seq);
            s.push_str(&alloc::format!(
                "  {:2} {}  len={} frob={} sf={} dc={} per={}\n",
                i, CANONICAL_NAMES[i], fp.length,
                if frob { "✓" } else { " " },
                if fp.self_ref { "✓" } else { " " },
                if fp.dialetheia_complete { "✓" } else { " " },
                fp.period,
            ));
        }
    }
    s
}
