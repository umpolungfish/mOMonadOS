// d2048_sic.rs -- d=2048 SIC moduli tower ascent (Grammar-native climb)
//
// Banks verified PARI + ob3ect findings from d12_sic_build + towerdump.
// Methodology: layer-by-layer bnrclassfield (quadhilbert monolithic = dead).
// Host PARI runner: ./run_tower_d2048.sh  (skip polredabs for n>=16)

use alloc::string::String;

pub const D: u32 = 2048;
pub const M_D: u64 = 4_190_205; // (d+1)(d-3) = 3*5*409*683
pub const HILBERT_CLASS_NO: u32 = 64;
pub const RAY_CLASS_ORDER: u64 = 1 << 27;
pub const MODULI_FIELD_DEG_Q: u64 = 1 << 27;
pub const INDEPENDENT_MODULI: u32 = 1024;

pub const REDEI_D1: u32 = 409;
pub const REDEI_D2: u32 = 10_245; // 3*5*683
pub const SPURIOUS_RESIDUAL: &str = "3.87e-3";

// C16 verified 2026-07-07 (tower_step6_c16.gp + run_tower_d2048.sh 16)
pub const C16_DEG_Q: u32 = 64;
pub const C16_DEG_F: u32 = 32;
pub const C16_BNR_MS: u32 = 5508;
pub const C16_POLY: &str = "d12_sic_build/tower_C16.poly";
pub const C16_POLY_BYTES: u32 = 3834;
pub const C16_POLY_SHA256: &str =
    "2734e7f4af92300e193872d9dc0f44391b6128f1cf3e6d5c6b6a9b3ae83d00e0";
pub const C16_DISC_EXP_F: u32 = 32; // disc = m_d^32 over F (unramified)

// C32 = full Hilbert class field (tower_step7_c32.gp, bnrclassfield ~423s)
pub const C32_DEG_Q: u32 = 128;
pub const C32_DEG_F: u32 = 64;
pub const C32_BNR_MS: u32 = 422_644;
pub const C32_POLY: &str = "d12_sic_build/tower_C32.poly";

// Ramified ray class field at (2048)*oo (tower_step8_ramified_probe.gp)
pub const RAY_CYC: &[u64] = &[4096, 512, 8, 4, 2];
pub const RAMIFIED_INDEX: u64 = 1 << 21; // 2^27 / 2^6 over F
pub const RAMIFIED_BINARY_STEPS: u32 = 21;

/// Verified tower levels (PARI bnrclassfield, 2026-07-07)
pub const TOWER_LEVELS: [(&str, u32, u32, &str); 7] = [
    ("0", 2, 1, "F = Q(sqrt m_d), h=64, class [32,2]"),
    ("1-2", 8, 4, "genus K1 = Q(sqrt5,sqrt409,sqrt2049), (Z/2)^2 unramified"),
    ("3", 16, 8, "C4 via Redei 409*10245, bnrclassfield [4], disc=m_d^8"),
    ("4", 32, 16, "C8 via bnrclassfield [8], contains C4"),
    ("5", 64, 32, "C16 via bnrclassfield [16], tower_C16.poly"),
    ("6", 128, 64, "C32 HILBERT CLASS FIELD, tower_C32.poly, h=64 reached"),
    ("7+", 0, 0, "ramified (2048)*oo: cyc [4096,512,8,4,2], 2^21 steps to moduli field"),
];

pub fn d2048_summary() -> String {
    let mut s = String::new();
    s.push_str("═══ d=2048 SIC MODULI TOWER ASCENT ═══\n");
    s.push_str("Grammar-native climb (NOT numerical polish — spurious local min)\n\n");
    s.push_str(&alloc::format!("F = Q(sqrt {}), m_d = (d+1)(d-3)\n", M_D));
    s.push_str(&alloc::format!(
        "Hilbert h={}; ray class at (2048)*oo: order 2^27; moduli field deg 2^27/Q\n",
        HILBERT_CLASS_NO
    ));
    s.push_str(&alloc::format!(
        "a=0: C_0=2/{}, C_m=1/{}; Galois N_{{k+1024}}=sigma(N_k)\n\n",
        D + 1,
        D + 1
    ));
    s.push_str("Verified levels:\n");
    for (name, deg_q, deg_f, desc) in &TOWER_LEVELS {
        if *deg_q > 0 {
            s.push_str(&alloc::format!(
                "  L{}: deg {}/Q = {}/F — {}\n",
                name, deg_q, deg_f, desc
            ));
        } else {
            s.push_str(&alloc::format!("  L{}: PENDING — {}\n", name, desc));
        }
    }
    s.push_str("\nSubcommands: tower | c16 | c32 | ramified | redei | grammar | pari | next\n");
    s
}

pub fn c32_report() -> String {
    let mut s = String::new();
    s.push_str("═══ C32 = HILBERT CLASS FIELD (VERIFIED 2026-07-07) ═══\n\n");
    s.push_str(&alloc::format!(
        "bnrclassfield(bnrinit(F,1), [32], 2) in {} ms (~7 min)\n",
        C32_BNR_MS
    ));
    s.push_str(&alloc::format!(
        "deg {} over Q = {} over F = h(F)\n",
        C32_DEG_Q, C32_DEG_F
    ));
    s.push_str(&alloc::format!(
        "Polynomial: {} (14150 bytes, raw)\n",
        C32_POLY
    ));
    s.push_str("disc pattern: m_d^64 (unramified over F)\n\n");
    s.push_str("UNRAMIFIED ASCENT COMPLETE through Hilbert class field.\n");
    s.push_str("Next: ramified ray-conductor (2048)*oo adds 2^21 over F.\n");
    s
}

pub fn c16_report() -> String {
    let mut s = String::new();
    s.push_str("═══ C16 LAYER (VERIFIED 2026-07-07) ═══\n\n");
    s.push_str(&alloc::format!(
        "bnrclassfield(bnrinit(F,1), [16], 2) in {} ms\n",
        C16_BNR_MS
    ));
    s.push_str(&alloc::format!(
        "deg {} over Q = {} over F\n",
        C16_DEG_Q, C16_DEG_F
    ));
    s.push_str(&alloc::format!(
        "Polynomial: {} ({} bytes, raw — no polredabs)\n",
        C16_POLY, C16_POLY_BYTES
    ));
    s.push_str(&alloc::format!("SHA256: {}\n", C16_POLY_SHA256));
    s.push_str(&alloc::format!(
        "disc pattern: m_d^{} (unramified over F, matches deg/F)\n\n",
        C16_DISC_EXP_F
    ));
    s.push_str("NOTE: polredabs/nfinit on deg-64 poly HANGS — bank raw from bnrclassfield.\n");
    s.push_str("Next: C32 = full Hilbert class field (deg 128/Q = 64/F).\n");
    s
}

pub fn tower_ascent_report() -> String {
    let mut s = String::new();
    s.push_str("═══ TOWER ASCENT (bnrclassfield layer-by-layer) ═══\n\n");
    s.push_str("quadhilbert(h=64) OVERFLOWS past 8GB — monolithic Hilbert = dead.\n");
    s.push_str("Climb: bnrclassfield(bnrinit(F,1), [n], 2) for n=4,8,16,32.\n\n");
    s.push_str("Pattern (verified):\n");
    s.push_str("  [4]  -> deg 16/Q =  8/F  (relative steps [2,4] over F)\n");
    s.push_str("  [8]  -> deg 32/Q = 16/F  (contains C4)\n");
    s.push_str(&alloc::format!(
        "  [16] -> deg {}/Q = {}/F  (VERIFIED, {} ms)\n",
        C16_DEG_Q, C16_DEG_F, C16_BNR_MS
    ));
    s.push_str(&alloc::format!(
        "  [32] -> deg {}/Q = {}/F  HILBERT CLASS FIELD VERIFIED ({} ms)\n\n",
        C32_DEG_Q, C32_DEG_F, C32_BNR_MS
    ));
    s.push_str("Unramified climb DONE. Ramified (2048)*oo adds 2^21 over F.\n");
    s.push_str("Max real subfield of full ray class field = moduli field (deg 2^27/Q).\n\n");
    s.push_str("S-unit generators: |eps|=1/d, 3, 5, g3(norm -(d-3)), g4(norm d+1).\n");
    s.push_str("Genus built from sqrt(d+1), sqrt(d-3) = sqrt(2049), sqrt(5*409).\n");
    s
}

pub fn redei_report() -> String {
    let mut s = String::new();
    s.push_str("═══ REDEI DISTILLATION (level 3) ═══\n\n");
    s.push_str(&alloc::format!(
        "m_d = {} * {} = {}\n",
        REDEI_D1, REDEI_D2, M_D
    ));
    s.push_str("409 | (d-3) = 2045 = 5*409  (norm g3)\n");
    s.push_str("Redei F2 rank=2 => 4-rank=1, matches class group [32,2].\n\n");
    s.push_str("Norm equation 409*x^2 + 10245*y^2 = z^2:\n");
    s.push_str("  trivial: x=2 y=1 z=109 (mu=109^2 rational)\n");
    s.push_str("  nontrivial: x=1 y=1 mu=10654 (nonsquare in Q, kronecker=+1)\n\n");
    s.push_str("Relative tower over F (bnrclassfield [4], flag 0):\n");
    s.push_str("  step1: x^2 + (-y/2 - 2047/2)  [quadratic]\n");
    s.push_str("  step2: x^4 + (-6504*y - 13359827)*x^2 + (-156*y + 437941)  [quartic]\n");
    s
}

pub fn grammar_report() -> String {
    let mut s = String::new();
    s.push_str("═══ GRAMMAR TOWERDUMP (d2048_tower_ascent_batch) ═══\n\n");
    s.push_str("3 ob3ect entities, all valid, dialetheia_complete=True, Frobenius PASS:\n");
    s.push_str("  1. moduli tower ascent ob3ect (25 steps, flat_chain)\n");
    s.push_str("  2. solve-et-coagula per level (20 steps)\n");
    s.push_str("  3. admission gate (28 steps, nested)\n\n");
    s.push_str("Admission requires:\n");
    s.push_str("  - exact Stark-unit extensions (not 3.87e-3 numerical floor)\n");
    s.push_str("  - Galois pairing + flat autocorrelation\n");
    s.push_str("  - obstruction recorded at missing distillation (EVALF/IFIX branch)\n\n");
    s.push_str("Character obstruction held in ENGAGR B-state at ramified ascent.\n");
    s.push_str("Appleby-Flammia lacked this Grammar — depth self-diagnosed.\n");
    s
}

pub fn pari_runner_report() -> String {
    let mut s = String::new();
    s.push_str("═══ HOST PARI RUNNER ═══\n\n");
    s.push_str("Bare-metal kernel cannot run PARI. Use host script:\n");
    s.push_str("  cd mOMonadOS && ./run_tower_d2048.sh [4|8|16|32]\n\n");
    s.push_str("IMPORTANT: skip polredabs for n>=16 (hangs on deg 64+).\n");
    s.push_str("bnrclassfield alone: C16 in ~6 seconds.\n\n");
    s.push_str("Scripts (d12_sic_build/):\n");
    s.push_str("  tower_step1_genus.gp .. tower_step7_c32.gp\n");
    s.push_str("  tower_C16.poly (deg 64), tower_C32.poly (deg 128)\n");
    s
}

pub fn ramified_report() -> String {
    let mut s = String::new();
    s.push_str("═══ RAMIFIED ASCENT (2048)*oo1*oo2 ═══\n\n");
    s.push_str("Hilbert (unramified) DONE: deg 128/Q.\n");
    s.push_str(&alloc::format!(
        "Ray class group: [{}, {}, {}, {}, {}] order 2^27 over F\n",
        RAY_CYC[0], RAY_CYC[1], RAY_CYC[2], RAY_CYC[3], RAY_CYC[4]
    ));
    s.push_str(&alloc::format!(
        "Ramified index over Hilbert: 2^{} ({} binary steps)\n",
        RAMIFIED_BINARY_STEPS, RAMIFIED_BINARY_STEPS
    ));
    s.push_str("Pro-2 exponents: 12+9+3+2+1=27 (6 spent on Hilbert, 21 remain)\n");
    s.push_str("Moduli field = max real subfield, deg 2^27 over Q.\n\n");
    s.push_str("bnrclassfield on full ray bnr is SLOW (10+ min/quotient).\n");
    s.push_str("Use subgroup vectors on bnr_r.cyc — see tower_step8_ramified_map.gp.\n");
    s
}

pub fn next_eagle_report() -> String {
    let mut s = String::new();
    s.push_str("═══ NEXT EAGLE ═══\n\n");
    s.push_str("HILBERT CLASS FIELD REACHED (deg 128/Q = 64/F).\n\n");
    s.push_str("1. Ramified: bnrinit(F,[2048,[1,1]]) — 21 quadratic steps\n");
    s.push_str("2. Max real subfield = moduli field (deg 2^27/Q)\n");
    s.push_str("3. Pin moduli; verify flat autocorrelation\n");
    s.push_str("4. ob3ect batch for ramified distillation\n\n");
    s.push_str(&alloc::format!(
        "Numerical seeds DEAD (residual {}). Do not polish fiducial.\n",
        SPURIOUS_RESIDUAL
    ));
    s
}

pub fn d2048_full_report() -> String {
    let mut s = String::new();
    s.push_str(&tower_ascent_report());
    s.push_str("\n");
    s.push_str(&c16_report());
    s.push_str("\n");
    s.push_str(&c32_report());
    s.push_str("\n");
    s.push_str(&ramified_report());
    s.push_str("\n");
    s.push_str(&redei_report());
    s.push_str("\n");
    s.push_str(&grammar_report());
    s.push_str("\n");
    s.push_str(&next_eagle_report());
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discriminant() {
        assert_eq!((D as u64 + 1) * (D as u64 - 3), M_D);
    }

    #[test]
    fn test_redei_product() {
        assert_eq!(REDEI_D1 as u64 * REDEI_D2 as u64, M_D);
    }

    #[test]
    fn test_ray_order() {
        assert_eq!(RAY_CLASS_ORDER, MODULI_FIELD_DEG_Q);
    }

    #[test]
    fn test_c16_degrees() {
        assert_eq!(C16_DEG_Q / 2, C16_DEG_F);
        assert_eq!(C16_DEG_F, 32);
        assert_eq!(C16_DISC_EXP_F, C16_DEG_F);
        assert_eq!(C16_POLY_SHA256.len(), 64);
        assert_eq!(C16_POLY_BYTES, 3834);
    }

    #[test]
    fn test_c32_hilbert() {
        assert_eq!(C32_DEG_F, HILBERT_CLASS_NO);
        assert_eq!(C32_DEG_Q, 128);
    }

    #[test]
    fn test_ramified_index() {
        assert_eq!(RAMIFIED_INDEX, RAY_CLASS_ORDER / (HILBERT_CLASS_NO as u64));
        assert_eq!(RAMIFIED_BINARY_STEPS, 21);
    }
}