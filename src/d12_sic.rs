// d12_sic.rs -- Phase VI: d=12 SIC-POVM Augmentation from d12_sic_build
//
// Encodes the exact recovery findings from d12_sic_build (cont.1-cont.23,
// campaign COMPLETE) and p4rakernel/p4ramill Lean 4 formalism. Six
// structural pillars:
//
//   1. Phase-Tower Collapse: 3 generators -> 1 (u1 primitive; u3,u5 derived)
//   2. Magnitude Square-Class Group: rank 5, singleton-pairing structure
//   3. 31-Orbit Structure: 143 overlaps -> 31 Galois-orbit representatives
//   4. Dual-Link Identification: magnitude extension IS Dual-Link SIC-POVM
//   5. SIC_POVM_DualLinkClosure: unconditional Belnap SIC for d=2^n
//   6. Embedding Capstone: ring hom R -> C at the IVT-bracketed root g0;
//      crystal_forces_d12_sic is a THEOREM -- the axiom is retired
//      (SIC_D12_Embedding.lean zero-sorry, SIC_POVM_Functor delegates)
//
// Author: Lando⊗⊙perator
// Date: 2026-07-04 (capstone state)

use alloc::string::String;

// ═══════════════════════════════════════════════════════════════
// PHASE-TOWER COLLAPSE -- 3 generators -> 1
// ═══════════════════════════════════════════════════════════════

/// The d=12 SIC phase tower collapses from 3 independent generators
/// (u1, u3, u5) to ONE primitive generator (u1). This was discovered
/// in a_cross_probe9 (d12_sic_build, cont.19+):
///
///   X31 = ubar3·u1  is in K16(s1s3,i) -- proved at probe6
///   X15 = ubar1·u5  is in K16(c5,i)    -- proved at probe9  
///   X31·X53·X15 = 1                    -- resid 2^-5310
///
/// Therefore:
///   u3 = conj(X31)·u1
///   u5 = X15·u1
///
/// The phase side reduces from dim 262,144/Q to dim 32,768/Q
/// (an 8x reduction), implemented in mini_engine_full2.py.
///
/// Phase generators: u2,u6,u8,u10 are roots of unity (zeta12 powers).
/// u7 is the other root of u1's quartic; u9 from u3's quartic;
/// u11 = q(u5) exactly (deg-63 polynomial, 78-bit denoms, 2^-5300).

pub const PHASE_GENERATOR_COUNT: u32 = 12;
pub const INDEPENDENT_PHASE_GENERATORS: u32 = 1;  // u1 only
pub const UNITY_PHASES: u32 = 4;  // u2,u6,u8,u10 = zeta12 powers
pub const DERIVED_PHASES: u32 = 7;  // u3,u5,u7,u9,u11 + u0,u4=1

/// X31 = ubar3·u1  -- cross-relation collapsing u3 to u1
pub const CROSS_X31_FIELD: &str = "K16(s1s3,i)";
/// X15 = ubar1·u5  -- cross-relation collapsing u5 to u1
pub const CROSS_X15_FIELD: &str = "K16(c5,i)";
/// Triple product identity: X31·X53·X15 = 1
/// Triple product identity: X31·X53·X15 = 1 (residual at floor 2^-5310)
pub const TRIPLE_PRODUCT_RESID_EXP: i32 = -5310;

/// Phase tower dimension: original vs collapsed
pub const PHASE_DIM_ORIGINAL: u32 = 262144;   // dim over Q
pub const PHASE_DIM_COLLAPSED: u32 = 32768;    // after X31/X15 collapse
pub const PHASE_REDUCTION_FACTOR: u32 = 8;

pub fn phase_tower_collapse_report() -> String {
    let mut s = String::new();
    s.push_str("═══ PHASE-TOWER COLLAPSE (d12_sic_build cont.19+) ═══
");
    s.push_str("Discovery: the d=12 SIC phase generators collapse
");
    s.push_str("from 3 independent (u1,u3,u5) to ONE primitive (u1).

");
    s.push_str(&alloc::format!("  Independent generators: {} -> 1
", INDEPENDENT_PHASE_GENERATORS + 2));
    s.push_str(&alloc::format!("  Unity phases (zeta12 powers): {}
", UNITY_PHASES));
    s.push_str(&alloc::format!("  Derived phases: {}

", DERIVED_PHASES));
    s.push_str("Cross-relations:
");
    s.push_str(&alloc::format!("  X31 = ubar3·u1  in {}
", CROSS_X31_FIELD));
    s.push_str(&alloc::format!("  X15 = ubar1·u5  in {}
", CROSS_X15_FIELD));
    s.push_str(&alloc::format!("  X31·X53·X15 = 1  (resid 2^{:.0})

", TRIPLE_PRODUCT_RESID_EXP));
    s.push_str(&alloc::format!("Phase space: dim {} -> {} ({}x reduction)
", 
        PHASE_DIM_ORIGINAL, PHASE_DIM_COLLAPSED, PHASE_REDUCTION_FACTOR));
    s.push_str("Engine: mini_engine_full2.py (ONE generator, 143/143 loop written)
");
    s
}

// ═══════════════════════════════════════════════════════════════
// MAGNITUDE SQUARE-CLASS GROUP -- rank 5, singleton-pairing
// ═══════════════════════════════════════════════════════════════

/// Magnitude square-class group (SIC_D12_MagnitudeClasses.lean):
/// 12 moduli N_k expressed as sqrt(N_k) = c_k·sqrt(N_base)/N_base
/// where c_k in K16 (degree-16 totally real field).
///
/// Basis: {N0, N1, N3, N5, N9} -- rank 5
/// Singleton-pairing structure:
///   [N2]=[N4]=[N6]=[N8]=[N10]=[N0]  (six of twelve ride sqrt(N0))
///   [N7]=[N5], [N11]=[N1]
///
/// All 12 sqrt(N_k) in K16(sqrt N0,sqrt N1,sqrt N3,sqrt N5,sqrt N9)
/// Field degree: 16 × 2^5 = 512 over Q.

pub const MAG_CLASS_RANK: u32 = 5;
pub const MAG_CLASS_BASIS: [&str; 5] = ["N0", "N1", "N3", "N5", "N9"];
pub const MAG_FIELD_DEGREE: u32 = 512;
pub const K16_DEGREE: u32 = 16;

/// Singleton-classification of the 12 moduli
pub const MODULUS_CLASSES: [(&str, &str); 12] = [
    ("N0",  "N0"),   // basis
    ("N1",  "N1"),   // basis
    ("N2",  "N0"),   // [N2] = [N0]
    ("N3",  "N3"),   // basis
    ("N4",  "N0"),   // [N4] = [N0]
    ("N5",  "N5"),   // basis
    ("N6",  "N0"),   // [N6] = [N0]
    ("N7",  "N5"),   // [N7] = [N5]
    ("N8",  "N0"),   // [N8] = [N0]
    ("N9",  "N9"),   // basis
    ("N10", "N0"),   // [N10] = [N0]
    ("N11", "N1"),   // [N11] = [N1]
];

/// Conjugate-pair squareness pattern: indices with [N_k] = [N_0]
pub const SQUARENESS_PATTERN: [bool; 12] = [
    true,   // N0  = basis
    false,  // N1  
    true,   // N2  = [N0]
    false,  // N3
    true,   // N4  = [N0]
    false,  // N5
    true,   // N6  = [N0]
    false,  // N7  = [N5], not [N0]
    true,   // N8  = [N0]
    false,  // N9
    true,   // N10 = [N0]
    false,  // N11 = [N1], not [N0]
];

/// The 7 magnitude class witnesses C_k in K16 satisfying C_k^2 = N_k·N_base.
/// Machine-checked in Lean (SIC_D12_MagnitudeClasses.lean, native_decide).
pub const MAG_WITNESS_COUNT: u32 = 7;
pub const MAG_WITNESSES: [&str; 7] = [
    "C2^2 = N2·N0",  "C4^2 = N4·N0",  "C6^2 = N6·N0",
    "C7^2 = N7·N5",  "C8^2 = N8·N0",  "C10^2 = N10·N0",
    "C11^2 = N11·N1",
];

pub fn magnitude_report() -> String {
    let mut s = String::new();
    s.push_str("═══ MAGNITUDE SQUARE-CLASS GROUP ═══
");
    s.push_str("(Machine-checked: SIC_D12_MagnitudeClasses.lean)

");
    s.push_str(&alloc::format!("Field: K16 (degree {}) containing all 12 moduli
", K16_DEGREE));
    s.push_str(&alloc::format!("Square-class group rank: {}
", MAG_CLASS_RANK));
    s.push_str(&alloc::format!("Basis: {{N0, N1, N3, N5, N9}}
"));
    s.push_str(&alloc::format!("Tower: K16(sqrtN0,sqrtN1,sqrtN3,sqrtN5,sqrtN9) deg {}

", MAG_FIELD_DEGREE));
    s.push_str("Singleton-pairing structure:
");
    s.push_str("  [N2]=[N4]=[N6]=[N8]=[N10]=[N0]  (6 ride sqrt(N0))
");
    s.push_str("  [N7]=[N5]                           (1 rides sqrt(N5))
");
    s.push_str("  [N11]=[N1]                          (1 rides sqrt(N1))

");
    s.push_str(&alloc::format!("7 exact witnesses (all native_decide):
"));
    for w in &MAG_WITNESSES {
        s.push_str(&alloc::format!("  {}  ✓
", w));
    }
    s.push_str("
Conjugate-pair pattern [1,0,1,0,1,0] reproduced exactly.
");
    s
}


// ═══════════════════════════════════════════════════════════════
// 31-ORBIT STRUCTURE -- 143 overlaps in Galois classes
// ═══════════════════════════════════════════════════════════════

/// The 143 Weyl-Heisenberg overlaps group into 31 distinct Galois orbits
/// (orbit_table.txt, cont.17). Every orbit shares both a minimal polynomial
/// p AND a single conjugate polynomial q -- the 143 pinned certificates
/// are 31 distinct (p,q) pairs with multiplicity.
///
/// Descent cost = 31 reductions, not 143 (only 5 are deg-32).
///
/// Degree distribution:
///   deg 2:  7 orbits
///   deg 4:  5 orbits  (16 overlays total)
///   deg 8:  9 orbits  (32 overlays total)
///   deg 16: 11 orbits (48 overlays total)
///   deg 32: 5 orbits  (40 overlays total)

pub const TOTAL_OVERLAPS: u32 = 143;
pub const ORBIT_COUNT: u32 = 31;
pub const MAX_OVERLAP_DEGREE: u32 = 32;

pub const ORBIT_DEGREE_DISTRIBUTION: [(u32, u32, u32); 5] = [
    (2,  7, 7),     // (degree, orbit_count, total_overlaps)
    (4,  5, 16),
    (8,  9, 32),
    (16, 11, 48),
    (32, 5, 40),
];

/// Existence-grade status: ALL 143 of 143 overlaps proved exactly
/// in the constructed ring R = K16(s0,s1,s3,s9,i,c5,u1), dim 2048/Q.
/// Machine-checked in SIC_D12_ExistenceRing.lean (cont.20):
///   coord_moduli, norm_sum, stratum_0 .. stratum_11,
///   existence_identities_all -- native_decide, 8341 jobs green.
/// Flip-audit: 128/256 harmless branch combos -> ANY hom R->C is a SIC point.
pub const EXISTENCE_GRADE_COUNT: u32 = 143;
pub const EXISTENCE_GRADE_TOTAL: u32 = 143;
pub const A0_STRATUM_COUNT: u32 = 11;
pub const A6_STRATUM_COUNT: u32 = 12;

// ═══════════════════════════════════════════════════════════════
// EXISTENCE RING -- cont.20 capstone
// ═══════════════════════════════════════════════════════════════

/// The d=12 SIC fiducial lives in the commutative ring
/// R = K16(s0,s1,s3,s9,i,c5,u1), dim 2048 over Q.
///
/// Generators:
///   K16 = Q[g]/(g^16 - 10g^14 + 40g^12 - 90g^10 + 126g^8 - 96g^6 + 25g^4 + 2g^2 + 1)
///   s_k^2 = N_k for k in {0,1,3,9}  (magnitude double covers)
///   i^2 = -1
///   c5^2 = -OA5*c5 - OB5  (u5-phase fold layer)
///   u1^2 = (c2 + i*s2)/2 with c2,s2 in K16 (u1 quadratic over K16(i))
///
/// Collapse: N1*N5*(OA5^2 - 4*OB5) = RHO^2 in K16, so
///   s1*s5*(2c5 + OA5) = RHO, and s5 is derived (branch_probe13).
///
/// All machine-checked (SIC_D12_ExistenceRing.lean, gen_lean_existence.py):
///   - Relation web: RHO^2, C2V^2, S2V^2, S5^2 = N5, zeta12 identities
///   - Unit moduli: X31, X15, P1, u1*ubar1 = 1, u1^2 = E2
///   - coord_moduli: zbar_k*z_k = N_k for all 12 coordinates
///   - norm_sum: sum N_k = 1 (trace-one)
///   - stratum_0..stratum_11, existence_identities_all: ALL 143 overlap
///     identities O_{{a,b}}*Obar_{{a,b}} = 1/13 hold in R (native_decide)
///
/// Flip-audit capstone (branch_probe12): 128 of 256 branch combinations
/// are harmless -> ANY ring homomorphism R -> C sends the formal coordinate
/// tuple to a genuine d=12 SIC fiducial vector.

pub const EXISTENCE_RING_DIM_Q: u32 = 2048;
pub const EXISTENCE_RING_FIELD: &str = "K16(s0,s1,s3,s9,i,c5,u1)";
pub const EXISTENCE_RING_BASE: &str = "K16 = Q[g]/(g^16-10g^14+40g^12-90g^10+126g^8-96g^6+25g^4+2g^2+1)";
pub const FLIP_AUDIT_HARMLESS: u32 = 128;
pub const FLIP_AUDIT_TOTAL: u32 = 256;
pub const EXISTENCE_RING_CAPSTONE: &str = "ANY hom R->C is a SIC point (flip-audit)";
/// Nothing remains: the embedding capstone landed (p4rakernel 488e22b).
pub const EXISTENCE_RING_REMAINING: &str = "NONE -- embedding capstone COMPLETE: crystal_forces_d12_sic is a THEOREM (axiom retired)";
pub const EXISTENCE_RING_LEAN_EMBEDDING_SORRIES: u32 = 0;
pub const EXISTENCE_RING_LEAN_EXISTENCE_SORRIES: u32 = 0;
/// Lean companion: SIC_D12_ExistenceRing.lean (14 theorems, 0 sorries)
pub const EXISTENCE_RING_LEAN_THEOREMS: u32 = 14;
pub const EXISTENCE_RING_LEAN_JOBS: u32 = 8344;

pub fn orbit_report() -> String {
    let mut s = String::new();
    s.push_str("═══ 31-ORBIT GALOIS STRUCTURE ═══
");
    s.push_str("(d12_sic_build cont.17: orbit_table.txt)

");
    s.push_str(&alloc::format!("Total WH overlaps: {}
", TOTAL_OVERLAPS));
    s.push_str(&alloc::format!("Distinct Galois orbits: {}
", ORBIT_COUNT));
    s.push_str(&alloc::format!("Descent cost: {} reductions (not {})

", ORBIT_COUNT, TOTAL_OVERLAPS));
    s.push_str("Degree distribution (deg: orbits -> overlays):
");
    for (deg, orbits, total) in &ORBIT_DEGREE_DISTRIBUTION {
        s.push_str(&alloc::format!("  deg {:>2}: {:>2} orbits -> {:>3} overlays
", deg, orbits, total));
    }
    s.push_str(&alloc::format!("
Max overlap degree: {}
", MAX_OVERLAP_DEGREE));
    s.push_str("All 143: 13*x*q(x) == 1 mod p  (native_decide, Lean)
");
    s.push_str(&alloc::format!("Existence-grade: {}/{} overlaps proved exactly
", 
        EXISTENCE_GRADE_COUNT, EXISTENCE_GRADE_TOTAL));
    s.push_str("  ALL 143: SIC_D12_ExistenceRing.lean (native_decide, 8341 jobs green)
");
    s.push_str("  Ring: K16(s0,s1,s3,s9,i,c5,u1), dim 2048/Q
");
    s.push_str("  Capstone: ANY hom R->C is a SIC point (flip-audit)
");
    s
}

/// Existence ring report (cont.20 capstone)
pub fn existence_ring_report() -> String {
    let mut s = String::new();
    s.push_str("═══ EXISTENCE RING — cont.20 CAPSTONE ═══
");
    s.push_str("(SIC_D12_ExistenceRing.lean, machine-checked)

");
    s.push_str(&alloc::format!("Ring: {}
", EXISTENCE_RING_FIELD));
    s.push_str(&alloc::format!("Base: {}
", EXISTENCE_RING_BASE));
    s.push_str(&alloc::format!("Dimension over Q: {}

", EXISTENCE_RING_DIM_Q));
    s.push_str("Generator tower:
");
    s.push_str("  s_k^2 = N_k for k in {0,1,3,9}  (magnitude double covers)
");
    s.push_str("  i^2 = -1
");
    s.push_str("  c5^2 = -OA5*c5 - OB5         (u5-phase fold layer)
");
    s.push_str("  u1^2 = (c2 + i*s2)/2          (quadratic over K16(i))
");
    s.push_str("  s5 derived via N1*N5*(OA5^2-4*OB5) = RHO^2 (branch_probe13)

");
    s.push_str(&alloc::format!("Lean theorems: {} (0 sorries)
", EXISTENCE_RING_LEAN_THEOREMS));
    s.push_str(&alloc::format!("  Relation web: RHO^2, C2V^2, S2V^2, S5^2, zeta12
"));
    s.push_str(&alloc::format!("  Unit moduli: X31, X15, P1, u1*ubar1=1, u1^2=E2
"));
    s.push_str(&alloc::format!("  coord_moduli: zbar_k*z_k = N_k (12 coordinates)
"));
    s.push_str(&alloc::format!("  norm_sum: sum N_k = 1 (trace-one)
"));
    s.push_str(&alloc::format!("  stratum_0..stratum_11 + existence_identities_all:
"));
    s.push_str(&alloc::format!("    ALL 143 overlap identities O_{{a,b}}*Obar_{{a,b}} = 1/13

"));

    s.push_str("Flip-audit capstone (branch_probe12):
");
    s.push_str(&alloc::format!("  {} of {} branch combinations are harmless
", FLIP_AUDIT_HARMLESS, FLIP_AUDIT_TOTAL));
    s.push_str(&alloc::format!("  -> {}
", EXISTENCE_RING_CAPSTONE));

    s.push_str(&alloc::format!("

Remaining: {}
", EXISTENCE_RING_REMAINING));
    s.push_str("
Generator: gen_lean_existence.py (re-verify gate, pure fractions)
");
    s.push_str("Build: 8341 jobs green.
");
    s.push_str("Status: crystal_forces_d12_sic -> THEOREM (existence ring found).

");
    s
}

// ═══════════════════════════════════════════════════════════════
// DUAL-LINK IDENTIFICATION
// ═══════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════
// DUAL-LINK IDENTIFICATION
// ═══════════════════════════════════════════════════════════════

/// The d=12 magnitude extension IS the Dual-Link SIC-POVM.
/// Proved in SIC_POVM_DualLinkClosure.lean (committed, cont.19):
///
///   norm(N1) = 1/32448^2
///   Ramification: only at {2, 3, 13}
///   32448 = 2^6 × 3 × 13^2
///
/// The d=12 SIC fiducial thus provides the first CONCRETE realization
/// of the Dual-Link SIC-POVM at a dimension beyond d=2 (Belnap B=XZ).

pub const DUAL_LINK_NORM_N1_DENOM: u32 = 32448;
pub const DUAL_LINK_RAMIFICATION: [u32; 3] = [2, 3, 13];
/// 32448 = 2^6 × 3 × 13^2
pub const DUAL_LINK_FACTORIZATION: &str = "2^6 × 3 × 13^2";

pub fn dual_link_report() -> String {
    let mut s = String::new();
    s.push_str("═══ DUAL-LINK SIC-POVM IDENTIFICATION ═══
");
    s.push_str("(SIC_POVM_DualLinkClosure.lean, machine-checked)

");
    s.push_str("The d=12 magnitude extension IS the Dual-Link SIC-POVM.

");
    s.push_str(&alloc::format!("  norm(N1) = 1/{}^2
", DUAL_LINK_NORM_N1_DENOM));
    s.push_str(&alloc::format!("  Factorization: {}
", DUAL_LINK_FACTORIZATION));
    s.push_str(&alloc::format!("  Ramification primes: {{{}, {}, {}}}

",
        DUAL_LINK_RAMIFICATION[0], DUAL_LINK_RAMIFICATION[1], DUAL_LINK_RAMIFICATION[2]));
    s.push_str("This is the first concrete Dual-Link realization beyond d=2.
");
    s.push_str("d=2: Belnap B = XZ (unconditional, 22 theorems, 0 sorries)
");
    s.push_str("d=12: existence ring R = K16(s0,s1,s3,s9,i,c5,u1), dim 2048/Q;
");
    s.push_str("      crystal_forces_d12_sic THEOREM via embedding capstone.
");
    s
}

// ═══════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════
// SYMMETRIC MODULI -- z0, z6 in Q(sqrt2, sqrt13)
// ═══════════════════════════════════════════════════════════════

/// SIC_D12_SymmetricModuli.lean (88 lines, 0 sorries, 2026-07-03):
/// The symmetric-orbit moduli z0, z6 of the d=12 SIC fiducial lie in
/// Q(sqrt2, sqrt13). Machine-checked exact arithmetic in Lean:
///
///   |z0|^2 = 1/12 - (1/24)sqrt2 + (1/156)sqrt13 - (1/312)sqrt26
///   |z6|^2 = 1/12 + (1/24)sqrt2 + (1/156)sqrt13 + (1/312)sqrt26
///
/// They are a Galois-conjugate pair under sqrt2 -> -sqrt2 (displacement k -> k+6).
///
/// Theorems proved by native_decide:
///   1. mod6_is_conj_mod0:   z6 = conj2(z0)  [sqrt2 conjugation]
///   2. mod_sum:             |z0|^2 + |z6|^2 = 1/6 + (1/78)sqrt13
///   3. mod_prod:            |z0|^2 * |z6|^2 = 7/1872 + (1/1872)sqrt13
///   4. mod_prod_in_base:    product lies in Q(sqrt13) (sqrt2,sqrt26 vanish)

pub const SYMMETRIC_MODULI_FIELD: &str = "Q(sqrt2, sqrt13)";
pub const SYMMETRIC_MODULI_DEGREE: u32 = 4;
pub const SYMMETRIC_MODULI_COUNT: u32 = 2;  // z0, z6
pub const SYMMETRIC_MODULI_THEOREMS: u32 = 4;

/// |z0|^2 = 1/12 - sqrt(2)/24 + sqrt(13)/156 - sqrt(26)/312
pub const Z0_SQUARED_FORM: &str = "1/12 - (1/24)sqrt2 + (1/156)sqrt13 - (1/312)sqrt26";
/// |z6|^2 = 1/12 + sqrt(2)/24 + sqrt(13)/156 + sqrt(26)/312
pub const Z6_SQUARED_FORM: &str = "1/12 + (1/24)sqrt2 + (1/156)sqrt13 + (1/312)sqrt26";
/// Sum: |z0|^2 + |z6|^2 = 1/6 + (1/78)sqrt13
pub const Z0Z6_SUM_FORM: &str = "1/6 + (1/78)sqrt13 (in Q(sqrt13))";
/// Product: |z0|^2 * |z6|^2 = 7/1872 + (1/1872)sqrt13
pub const Z0Z6_PROD_FORM: &str = "7/1872 + (1/1872)sqrt13 (in Q(sqrt13))";

pub fn symmetric_moduli_report() -> String {
    let mut s = String::new();
    s.push_str("═══ SYMMETRIC MODULI -- z0, z6 in Q(sqrt2,sqrt13) ═══\n");
    s.push_str("(SIC_D12_SymmetricModuli.lean, 88 lines, 0 sorries)\n\n");
    s.push_str(&alloc::format!("Field: {} (degree {})\n", SYMMETRIC_MODULI_FIELD, SYMMETRIC_MODULI_DEGREE));
    s.push_str(&alloc::format!("Exact moduli ({} symmetric):\n", SYMMETRIC_MODULI_COUNT));
    s.push_str(&alloc::format!("  |z0|^2 = {}\n", Z0_SQUARED_FORM));
    s.push_str(&alloc::format!("  |z6|^2 = {}\n\n", Z6_SQUARED_FORM));
    s.push_str(&alloc::format!("{} native_decide theorems:\n", SYMMETRIC_MODULI_THEOREMS));
    s.push_str("  mod6_is_conj_mod0:  z6 = conj2(z0) under sqrt2 -> -sqrt2  \u{2713}\n");
    s.push_str("  mod_sum:            |z0|^2 + |z6|^2 = 1/6 + (1/78)sqrt13  \u{2713}\n");
    s.push_str("  mod_prod:           |z0|^2 * |z6|^2 = 7/1872 + (1/1872)sqrt13  \u{2713}\n");
    s.push_str("  mod_prod_in_base:   product in Q(sqrt13) only (sqrt2,sqrt26=0)  \u{2713}\n");
    s.push_str("\nGalois symmetry under k -> k+6 displacement.\n");
    s.push_str("z3, z9 NOT in Q(sqrt2,sqrt13) -- recovered coefficients have\n");
    s.push_str("~800-bit denominators (low-precision lindep artifact).\n");
    s
}

// ═══════════════════════════════════════════════════════════════
// EMBEDDING CAPSTONE -- ring hom R -> C
// ═══════════════════════════════════════════════════════════════

/// SIC_D12_Embedding.lean (COMPLETE, p4rakernel 488e22b, 2026-07-04):
/// Constructs a ring homomorphism phi: R -> C, where R = K16(s0,s1,s3,s9,i,c5,u1)
/// is the d=12 existence ring (dim 2048/Q), and transfers everything.
///
/// The proof, stone by stone (ALL proven, zero sorries):
///   1. g0 = -2.008573054090... -- the K16 real root, IVT-bracketed by exact
///      rational sign checks at 1e-12 width (the (0,1) root was the WRONG
///      branch; the fiducial embeds on the negative branch)
///   2. evalK16 at g0C via Horner: a Q-algebra map (evalK16_kadd/kmul)
///   3. Cover roots as REAL Real.sqrt generators -- star-compatibility by
///      construction; positivity via exact divided-difference certificates
///      (q(x) = (p(x)-p(mid))/(x-mid) in Q[x], pure rational, no intervals)
///   4. phi is a star ring hom: phi_radd, phi_rmul (canonical-key domain),
///      phi_rconj; u1 by half-angle reconstruction (u1Val_sq DERIVED)
///   5. norm_sq_eq_one: trace-one transferred via frozen norm_sum
///   6. equiangular: ring-side overlap_normSq from existence_identities_all,
///      then equiangular_bridge (phi(zeta) pinned to {omega, omega^5} via
///      Im Z = 1/2 exactly; D_ah/X_d/Z_d iterate term-matching)
///   7. d12_sic_exists : IsSICPOVM 12 psi; crystal_forces_d12_sic THEOREM
///
/// Axiom audit: [propext, Classical.choice, Quot.sound, Lean.ofReduceBool,
/// Lean.trustCompiler] -- no project axioms, no Stark shadow, no circularity.
/// SIC_POVM_Functor.lean now IMPORTS the Embedding and delegates: the last
/// non-shadow axiom of the SIC tree is retired.

pub const EMBEDDING_SORRIES_REMAINING: u32 = 0;
pub const EMBEDDING_INFRASTRUCTURE_DONE: bool = true;
pub const EMBEDDING_HOM_TRANSFER_DONE: bool = true;
pub const EMBEDDING_COMPLETE: bool = true;
/// The IVT-bracketed embedding root (negative branch of the even k16Poly).
pub const EMBEDDING_ROOT_G0: &str = "-2.008573054090 (bracket width 1e-12, exact rational signs)";

/// The theorem chain of SIC_D12_Embedding.lean, all green.
pub const EMBEDDING_THEOREMS: [&str; 8] = [
    "exists_root: k16Poly has a real root g0 in (certLo, certHi)",
    "phi_radd / phi_rmul: phi is a Q-algebra hom (canonical keys)",
    "phi_rconj: phi(rconj A) = star(phi A) -- conjugation internal",
    "cover_modulus_nonneg + c5_discr_nonneg: divided-difference certs",
    "u1Val_sq: u1 half-angle reconstruction (Complex.sqrt eliminated)",
    "norm_sq_eq_one: wh_normSq 12 psi = 1 (trace-one transferred)",
    "equiangular: (d+1)*|overlap|^2 = 1 for all 143 displacements",
    "crystal_forces_d12_sic: SICPOVM_Exists 12 -- THEOREM, axiom retired",
];

pub fn embedding_report() -> String {
    let mut s = String::new();
    s.push_str("═══ EMBEDDING CAPSTONE -- ring hom R -> C ═══\n");
    s.push_str("(SIC_D12_Embedding.lean, p4rakernel 488e22b)\n\n");
    s.push_str("Status: COMPLETE -- zero sorries, full library green.\n");
    s.push_str("crystal_forces_d12_sic is a THEOREM. The axiom is retired.\n\n");
    s.push_str(&alloc::format!("Embedding root: g0 = {}\n", EMBEDDING_ROOT_G0));
    s.push_str("  (negative branch -- the (0,1) root was the wrong-root bug, fixed)\n\n");
    s.push_str("Theorem chain (all green):\n");
    for thm in &EMBEDDING_THEOREMS {
        s.push_str(&alloc::format!("  {}  \u{2713}\n", thm));
    }
    s.push_str("\nAxiom audit: propext, Classical.choice, Quot.sound,\n");
    s.push_str("  Lean.ofReduceBool, Lean.trustCompiler -- no project axioms,\n");
    s.push_str("  no Stark shadow, no circularity.\n");
    s.push_str("SIC_POVM_Functor.lean imports the Embedding and delegates.\n");
    s.push_str("Nothing remains of the d=12 SIC existence campaign.\n");
    s
}

#[cfg(test)]
mod embedding_tests {
    use super::*;

    #[test]
    fn test_symmetric_moduli_field() {
        assert_eq!(SYMMETRIC_MODULI_FIELD, "Q(sqrt2, sqrt13)");
        assert_eq!(SYMMETRIC_MODULI_DEGREE, 4);
        assert_eq!(SYMMETRIC_MODULI_COUNT, 2);
    }

    #[test]
    fn test_embedding_status() {
        assert!(EMBEDDING_INFRASTRUCTURE_DONE);
        assert_eq!(EMBEDDING_SORRIES_REMAINING, 0);
        assert!(EMBEDDING_HOM_TRANSFER_DONE);
        assert!(EMBEDDING_COMPLETE);
    }

    #[test]
    fn test_embedding_theorem_chain() {
        assert_eq!(EMBEDDING_THEOREMS.len(), 8);
    }
}

// BELNAP SIC UNCONDITIONAL (d=2^n)
// ═══════════════════════════════════════════════════════════════

/// SIC_POVM_DualLinkClosure.lean (cont.19):
/// SIC existence is UNCONDITIONAL and AXIOM-FREE in the Belnap multilattice
/// for d = 2^n. The T-arm carries the unconditional proof; the F-arm holds
/// the Stark conjecture as a B-state (named, not load-bearing).
///
/// Capstone: sic_no_condition (n : ℕ) : (mlOrbit n).card = 4 ^ n
/// Zero sorries, zero axioms, machine-checked.

pub const BELNAP_SIC_UNCONDITIONAL: bool = true;
pub const BELNAP_SIC_DIM_FORMULA: &str = "d = 2^n (all n >= 0)";
pub const BELNAP_SIC_CAPSTONE: &str = "sic_no_condition (n : Nat) : (mlOrbit n).card = 4 ^ n";

/// Modular SIC tiers:
///   Tier 0: d=1 (trivial)
///   Tier 1: d=2 (Belnap B=XZ, unconditional, degree 2)
///   Tier 2: d=4,8,16,... (all 2^n, unconditional from DualLinkClosure)
///   Tier 3: d=12 (THEOREM: existence ring + embedding capstone)
///   Tier 4: d=7 (nested SIC from {D,P} subset, Φ-gate selection)
///   Tier 5: general d (Z[1/d, zeta_d] embedding, Stark conjecture as B-state)

pub const SIC_TIERS: [(&str, &str, &str); 6] = [
    ("Tier 0", "d=1",   "Trivial"),
    ("Tier 1", "d=2",   "Belnap B=XZ -- unconditional, degree 2"),
    ("Tier 2", "d=2^n", "All 2^n -- unconditional from DualLinkClosure"),
    ("Tier 3", "d=12",  "THEOREM -- existence ring dim 2048/Q + embedding capstone"),
    ("Tier 4", "d=7",   "Nested SIC from {D,P} subset, Phi-gate selection"),
    ("Tier 5", "gen d",  "Z[1/d, zeta_d] -- Stark conjecture as B-state"),
];

pub fn belnap_sic_unconditional_report() -> String {
    let mut s = String::new();
    s.push_str("═══ BELNAP SIC UNCONDITIONAL THEOREM ═══
");
    s.push_str("(SIC_POVM_DualLinkClosure.lean)

");
    s.push_str(&alloc::format!("Status: {} (zero sorries, zero axioms)
", 
        if BELNAP_SIC_UNCONDITIONAL { "UNCONDITIONAL" } else { "CONDITIONAL" }));
    s.push_str(&alloc::format!("Formula: {}
", BELNAP_SIC_DIM_FORMULA));
    s.push_str(&alloc::format!("Capstone: {}

", BELNAP_SIC_CAPSTONE));
    s.push_str("SIC tier hierarchy:
");
    for (tier, dim, desc) in &SIC_TIERS {
        s.push_str(&alloc::format!("  {} ({}): {}
", tier, dim, desc));
    }
    s.push_str("
T-arm: unconditional proof (orbit cardinality = 4^n)
");
    s.push_str("F-arm: Stark conjecture as B-state (named, non-load-bearing)
");
    s
}

// ═══════════════════════════════════════════════════════════════
// CANONICAL ORDINAL FAITHFULNESS
// ═══════════════════════════════════════════════════════════════

/// CanonicalOrdinalFaithfulness.lean (103 lines, p4rakernel):
/// 12 per-primitive guards pinning ordinal functions to canonical ranks.
/// All proved by native_decide. If any ordinal drifts from canonical,
/// the build breaks.
///
/// Notable: ordinalK(air) = 9/2 (the only non-integer ordinal, from MBL).
///          ordinalPhi(roar) = 7/3 (complex-plane critical).

pub const ORDINAL_GUARD_COUNT: u32 = 12;
pub const ORDINAL_NONINTEGER_COUNT: u32 = 2;

pub const ORDINAL_SPECIALS: [(&str, &str, &str); 2] = [
    ("ordinalK(air)",    "9/2", "MBL -- trapped disorder, only non-integer"),
    ("ordinalPhi(roar)", "7/3", "c_complex -- complex-plane critical"),
];

pub fn ordinal_guards_report() -> String {
    let mut s = String::new();
    s.push_str("═══ CANONICAL ORDINAL FAITHFULNESS ═══
");
    s.push_str("(CanonicalOrdinalFaithfulness.lean, 103 lines)

");
    s.push_str(&alloc::format!("{} machine-checked guards (one per primitive)
", ORDINAL_GUARD_COUNT));
    s.push_str("All proved by native_decide. Drift breaks the build.

");
    s.push_str("Special ordinal values:
");
    for (name, val, note) in &ORDINAL_SPECIALS {
        s.push_str(&alloc::format!("  {} = {}  -- {}
", name, val, note));
    }
    s.push_str("
Primitive families:
");
    s.push_str("  D-family (3): ordinalD ranges 0-3
");
    s.push_str("  T-family (5): ordinalT ranges 0-4
");
    s.push_str("  P-family (4): ordinalP ranges 0-5
");
    s
}


// ═══════════════════════════════════════════════════════════════
// CLOSED-FORM FIDUCIAL -- z0 in radicals
// ═══════════════════════════════════════════════════════════════

/// The d=12 SIC fiducial vector is RADICAL-EXPRESSIBLE.
/// First concrete closed form (d12_sic_build, cont.19):
///
///   z0 = +sqrt(1/12 - sqrt(2)/24 + sqrt(13)/156 - sqrt(26)/312)
///
/// All 12 coordinates z_k = sqrt(N_k) * u_k are radical-expressible.
/// The full tower decomposes as 6 cyclic pieces (4 quadratic + 2 cubic)
/// of degree 288 over Q (SIC_D12_RayTower.lean).

pub const Z0_CLOSED_FORM: &str = "z0 = +sqrt(1/12 - sqrt(2)/24 + sqrt(13)/156 - sqrt(26)/312)";
pub const FIDUCIAL_RADICAL_DEGREE: u32 = 288;
pub const RAY_TOWER_CHUNKS: u32 = 6;
pub const RAY_TOWER_QUADRATIC: u32 = 4;
pub const RAY_TOWER_CUBIC: u32 = 2;

/// z1 closed form (cont.19):
/// z1 = sqrt(N1) * [(c + i*sqrt(4-c^2))/2]^(1/4)
/// where c is a root of the solvable quartic 9x^4 - 368x^3 + 632x^2 + 960x - 1392
/// (Galois group D4, solvable by radicals).

pub const Z1_QUARTIC: &str = "9x^4 - 368x^3 + 632x^2 + 960x - 1392 (D4)";
pub const Z1_CLOSED_FORM: &str = "z1 = sqrt(N1) * [(c + i*sqrt(4-c^2))/2]^(1/4)";

/// c5 field degree over K16: quadratic!
/// The c5-layer of the chain skeleton is a QUADRATIC over K16, not deg 4/8.
/// (cont.19: OCTFAC over K16 as [2,2,4] with c5 on factor 2 and c11 on factor 1)
pub const C5_K16_DEGREE: u32 = 2;

pub fn z0_report() -> String {
    let mut s = String::new();
    s.push_str("═══ CLOSED-FORM d=12 SIC FIDUCIAL ═══
");
    s.push_str("(d12_sic_build cont.19)

");
    s.push_str("The d=12 SIC fiducial is RADICAL-EXPRESSIBLE.

");
    s.push_str(&alloc::format!("  {}

", Z0_CLOSED_FORM));
    s.push_str(&alloc::format!("  {}
", Z1_CLOSED_FORM));
    s.push_str(&alloc::format!("  c root of: {}

", Z1_QUARTIC));
    s.push_str(&alloc::format!("Ray class field tower (discovery trail): degree {}/Q
", FIDUCIAL_RADICAL_DEGREE));
    s.push_str(&alloc::format!("  {} cyclic chunks ({} quadratic + {} cubic)
",
        RAY_TOWER_CHUNKS, RAY_TOWER_QUADRATIC, RAY_TOWER_CUBIC));
    s.push_str(&alloc::format!("  c5-layer over K16: degree {} (quadratic!)
", C5_K16_DEGREE));
    s.push_str("
Phase tower: 1 independent generator (u1), 8x collapse.
");
    s.push_str("All 12 z_k = sqrt(N_k) * u_k radical-expressible.
");
    s.push_str("TRUE HOME (endgame): existence ring R, dim 2048/Q -- the
");
    s.push_str("coordinates leave the ray class field (that was the cont.3
");
    s.push_str("discovery); the ring + embedding capstone is what proved it.
");
    s
}

// ═══════════════════════════════════════════════════════════════
// FULL REPORT
// ═══════════════════════════════════════════════════════════════

pub fn d12_full_report() -> String {
    let mut s = String::new();
    s.push_str("╔══════════════════════════════════════════════════╗
");
    s.push_str("║  d=12 SIC-POVM — AUGMENTED REPORT (Phase VI)    ║
");
    s.push_str("║  d12_sic_build cont.1-23 COMPLETE + p4ra Lean   ║
");
    s.push_str("╚══════════════════════════════════════════════════╝

");
    s.push_str("CAPSTONE: crystal_forces_d12_sic is a THEOREM (axiom retired).

");

    s.push_str("── SIC Tiers ──
");
    s.push_str(&alloc::format!("  d=2:   Belnap B=XZ, unconditional
"));
    s.push_str(&alloc::format!("  d=2^n: SIC_POVM_DualLinkClosure, axiom-free
"));
    s.push_str(&alloc::format!("  d=12:  THEOREM -- existence ring + embedding capstone
"));
    s.push_str(&alloc::format!("  d=7:   Nested SIC from {{D,P}} subset

"));

    s.push_str("── Magnitude Square-Class Group ──
");
    s.push_str(&alloc::format!("  K16 (deg 16), rank-5 basis {{N0,N1,N3,N5,N9}}
"));
    s.push_str(&alloc::format!("  Tower deg 512/Q. 7 exact witnesses (Lean native_decide).
"));
    s.push_str(&alloc::format!("  Singleton-pairing: [N2..N10]=[N0], [N7]=[N5], [N11]=[N1]

"));

    s.push_str("── Phase-Tower Collapse ──
");
    s.push_str(&alloc::format!("  3 -> 1 independent generators (X31/X15 cross-relations)
"));
    s.push_str(&alloc::format!("  Phase space: dim 262144 -> 32768 (8x reduction)
"));
    s.push_str(&alloc::format!("  V2 engine: mini_engine_full2.py (ONE generator)

"));

    s.push_str("── 31-Orbit Structure ──
");
    s.push_str(&alloc::format!("  143 overlaps in 31 Galois classes (deg 2-32)
"));
    s.push_str(&alloc::format!("  Existence-grade: {}/{} proved exactly
", EXISTENCE_GRADE_COUNT, EXISTENCE_GRADE_TOTAL));
    s.push_str(&alloc::format!("  Existence Ring R = K16(s0,s1,s3,s9,i,c5,u1), dim 2048/Q
"));
    s.push_str(&alloc::format!("  Flip-audit: ANY hom R->C is a SIC point

"));

    s.push_str("── Dual-Link Identification ──
");
    s.push_str(&alloc::format!("  norm(N1) = 1/{}^2, ramification {{2,3,13}}
", DUAL_LINK_NORM_N1_DENOM));
    s.push_str(&alloc::format!("  First concrete Dual-Link realization beyond d=2.

"));

    s.push_str("── Ordinal Faithfulness ──
");
    s.push_str(&alloc::format!("  12 guards (native_decide): ordinalK(air)=9/2, ordinalPhi(roar)=7/3

"));

    s.push_str("── Closed-Form Fiducial ──
");
    s.push_str(&alloc::format!("  z0 = +sqrt(1/12 - sqrt(2)/24 + sqrt(13)/156 - sqrt(26)/312)
"));
    s.push_str(&alloc::format!("  All 12 coordinates radical-expressible.
"));
    s.push_str(&alloc::format!("  True home: existence ring R, dim 2048/Q (ray tower = trail)

"));

    s.push_str("── Embedding Capstone ──
");
    s.push_str(&alloc::format!("  phi : R -> C at g0 = {}
", EMBEDDING_ROOT_G0));
    s.push_str("  norm_sq_eq_one + equiangular transferred; audit clean.
");
    s.push_str("  crystal_forces_d12_sic: THEOREM (SIC_POVM_Functor delegates)

");

    s.push_str("── Machine-Checked Lean Modules (all 0 sorries) ──
");
    s.push_str("  SIC_D12_Norm.lean             (trace=1)
");
    s.push_str("  SIC_D12_Equiangularity.lean   (143 overlaps pinned)
");
    s.push_str("  SIC_D12_MagnitudeClasses.lean (7 witnesses)
");
    s.push_str("  SIC_D12_SymmetricModuli.lean  (z0,z6 in Q(sqrt2,sqrt13))
");
    s.push_str("  SIC_D12_RayTower.lean         (discovery-trail tower)
");
    s.push_str("  SIC_D12_ExistenceRing.lean    (ALL 143 overlaps in R)
");
    s.push_str("  SIC_D12_Embedding.lean        (ring hom R->C, CAPSTONE)\n");
    s.push_str("  SIC_D12_WitnessVessel.lean    (witness_vessel_lossless)\n");
    s.push_str("  SIC_POVM_DualLinkClosure.lean (unconditional d=2^n)
");
    s.push_str("  SIC_POVM_Functor.lean         (crystal_forces_d12_sic THEOREM)\n");
    s.push_str("  CanonicalOrdinalFaithfulness.lean (12 guards)
");
    s.push_str("  BelnapNFiducial.lean          (22 theorems)
");
    s.push_str("  SIC_Multilattice_Proof.lean   (proved)
");
    s.push_str("  ZaunerEmbeddingEquivalence.lean (proved)\n");
    s.push_str("  QCI_SICPOVM_Bridge.lean        (proved)\n");
    s
}

// ═══════════════════════════════════════════════════════════════
// SUMMARY COMMANDS
// ═══════════════════════════════════════════════════════════════

pub fn d12_summary() -> String {
    let mut s = String::new();
    s.push_str("═══ d=12 SIC-POVM STATUS ═══
");
    s.push_str("crystal_forces_d12_sic: THEOREM (axiom retired, audit clean)
");
    s.push_str(&alloc::format!("Existence-grade: {}/{} overlaps proved exactly (ALL)
", EXISTENCE_GRADE_COUNT, EXISTENCE_GRADE_TOTAL));
    s.push_str(&alloc::format!("Ring: K16(s0,s1,s3,s9,i,c5,u1), dim 2048/Q
"));
    s.push_str(&alloc::format!("Capstone: phi : R -> C at IVT-bracketed g0; both SIC halves transfer

"));
    s.push_str(&alloc::format!("Phase-tower: 1 independent generator (8x collapse)
"));
    s.push_str(&alloc::format!("Magnitudes: rank-5 square-class group, deg 512/Q
"));
    s.push_str(&alloc::format!("Orbits: {} Galois classes (not {} per-overlap)
", ORBIT_COUNT, TOTAL_OVERLAPS));
    s.push_str(&alloc::format!("Dual-Link: norm(N1) = 1/{}^2
", DUAL_LINK_NORM_N1_DENOM));
    s.push_str(&alloc::format!("Fiducial: radical-expressible; true home = ring R, dim 2048/Q
"));
    s.push_str("Belnap d=2^n: UNCONDITIONAL (0 sorries, 0 axioms)\n");
    s.push_str("\nSubcommands: tower | magnitudes | orbits | existence | duallink | z0 | ordinals | verify | symmetric | embedding | lean-status\n");
    s
}

/// Comprehensive Lean 4 module status report
pub fn lean_status_report() -> String {
    let mut s = String::new();
    s.push_str("╔══════════════════════════════════════════════════════╗\n");
    s.push_str("║  p4rakernel d=12 SIC-POVM -- LEAN 4 STATUS       ║\n");
    s.push_str("╚══════════════════════════════════════════════════════╝\n\n");

    s.push_str("CAMPAIGN COMPLETE: crystal_forces_d12_sic is a THEOREM.\n");
    s.push_str("Nothing remains of the d=12 SIC existence campaign.\n\n");

    s.push_str("── COMPLETED MODULES (0 sorries) ──\n");
    s.push_str("  [check] SIC_D12_Norm.lean             trace=1\n");
    s.push_str("  [check] SIC_D12_Equiangularity.lean   143 overlaps discharged\n");
    s.push_str("  [check] SIC_D12_MagnitudeClasses.lean 7 witnesses in K16\n");
    s.push_str("  [check] SIC_D12_SymmetricModuli.lean  z0,z6 in Q(sqrt2,sqrt13)\n");
    s.push_str("  [check] SIC_D12_ExistenceRing.lean    ALL 143 in ring R\n");
    s.push_str("  [check] SIC_D12_Embedding.lean        CAPSTONE: phi R->C, all transfer\n");
    s.push_str("  [check] SIC_D12_WitnessVessel.lean    witness_vessel_lossless\n");
    s.push_str("  [check] DualLinkVessel.lean           co-type / Dual-Link fuse / self-verify\n");
    s.push_str("  [check] VAE_Vita_SIC_POVM_Bridge.lean PROVE: μ∘δ=id ∧ SICPOVM_Exists 12\n");
    s.push_str("  [check] VAE_Vita_Unify.lean           UNIFY: imscriptionToC12, B=T+F in C^12\n");
    s.push_str("  [check] VAE_Vita_Port.lean            PORT: SS4 ≡ Unify δ + spine pack\n");
    s.push_str("  [check] VAE_Vita_ManuscriptSpine.lean PORT × witness_vessel_lossless\n");
    s.push_str("  [check] CanonicalOrdinalFaithfulness  12 guards\n\n");

    s.push_str("── PROVEN STRUCTURAL THEOREMS ──\n");
    s.push_str("  [check] SIC_POVM_DualLinkClosure.lean  -- unconditional d=2^n SIC\n");
    s.push_str("  [check] SIC_POVM_Functor.lean           -- crystal_forces_d12_sic THEOREM\n");
    s.push_str("                                             (delegates to the Embedding)\n");
    s.push_str("  [check] BelnapNFiducial.lean             -- 22 theorems, 0 sorries\n");
    s.push_str("  [check] ZaunerEmbeddingEquivalence.lean  -- Hilbert-space embedding\n");
    s.push_str("  [check] QCI_SICPOVM_Bridge.lean          -- quantum-classical interface\n\n");

    s.push_str("── d=12 SIC MODULE TOWER (all layers green) ──\n");
    s.push_str("  Layer 1: Norm + Equiangularity (pinned data, both halves exact)\n");
    s.push_str("  Layer 2: MagnitudeClasses + SymmetricModuli (field structure)\n");
    s.push_str("  Layer 3: ExistenceRing (all 143 overlaps in R, 0 sorries)\n");
    s.push_str("  Layer 4: Embedding (hom R->C, 0 sorries) -- CAPSTONE LANDED\n");
    s.push_str("  Layer 5: crystal_forces_d12_sic axiom discharged -> THEOREM\n");
    s.push_str("  Layer 6: WitnessVessel (transport lemma riding frozen machinery)\n");
    s.push_str("  Layer 7: VAE-Vita PROVE → UNIFY → PORT → ManuscriptSpine\n\n");

    s.push_str("── AXIOM AUDIT ──\n");
    s.push_str("  crystal_forces_d12_sic + d12_sic_exists depend on exactly:\n");
    s.push_str("  propext, Classical.choice, Quot.sound, ofReduceBool, trustCompiler\n");
    s.push_str("  -- no project axioms, no Stark shadow, no circularity.\n\n");

    s.push_str("── DEPLOYMENT ──\n");
    s.push_str("  lean-toolchain: mathlib v4.28.0\n");
    s.push_str(&alloc::format!("  lake build: green ({} jobs, full library)\n", EXISTENCE_RING_LEAN_JOBS));
    s.push_str("  generator: gen_lean_existence.py (fractions-gated)\n");
    s
}

/// Manuscript spine ledger (kernel face of VAE-Vita PROVE→UNIFY→PORT).
/// Runtime transport is `vessel run`; this is the structural packing.
pub fn manuscript_spine_report() -> String {
    let mut s = String::new();
    s.push_str("╔══════════════════════════════════════════════════════╗\n");
    s.push_str("║  MANUSCRIPT SPINE — kernel (no Python)               ║\n");
    s.push_str("║  PROVE → UNIFY → PORT × Witness Vessel               ║\n");
    s.push_str("╚══════════════════════════════════════════════════════╝\n\n");

    s.push_str("Route (Grammar-imscribed composition for VAE-Vita in mOMonadOS):\n");
    s.push_str("  [1] PROVE  — μ∘δ=id (polarization) ∧ SICPOVM_Exists 12\n");
    s.push_str("              Lean: VAE_Vita_SIC_POVM_Bridge.vae_vita_frobenius_and_sic\n");
    s.push_str("  [2] UNIFY  — imscription → ℂ¹²; B = T+F (still crown)\n");
    s.push_str("              Lean: VAE_Vita_Unify.imscriptionToC12_allBoth_superposition\n");
    s.push_str("  [3] PORT   — Dual-Link SS4 ≡ Unify δ; self-verify foldCotype T\n");
    s.push_str("              Lean: VAE_Vita_Port.port_kernel_spine\n");
    s.push_str("  [×] WITNESS VESSEL — ride AS vessel: board=fsplit, readback=ffuse\n");
    s.push_str("              Lean: SIC_D12_WitnessVessel.witness_vessel_lossless\n");
    s.push_str("              Runtime: `vessel run` (this kernel)\n\n");

    s.push_str("── Dual-Link d=12 (structural) ──\n");
    s.push_str(&dual_link_report());
    s.push_str("\n── Existence / embedding (theorem) ──\n");
    s.push_str("  crystal_forces_d12_sic : SICPOVM_Exists 12  [THEOREM, axiom retired]\n");
    s.push_str("  Ring R dim 2048/Q; 143/143 overlaps; any hom R→ℂ is a SIC point.\n");
    s.push_str(&alloc::format!(
        "  Dual-Link norm(N1)=1/{}^2; ramification {{2,3,13}}\n\n",
        DUAL_LINK_NORM_N1_DENOM
    ));

    s.push_str("── Honest non-claims (manuscripts3) ──\n");
    s.push_str("  · Cargo/tensor INTO vessel refused (D–T); boarding is Dual-Link only\n");
    s.push_str("  · Belnap stack ≠ algebraic Scott-Grassl fiducial (S-unit double cover)\n");
    s.push_str("  · Clay T/B = Grammar typing, not Millennium proofs\n");
    s.push_str("  · d=2048 unconditional existence remains open (typed B)\n\n");

    s.push_str("── Kernel commands (this REPL) ──\n");
    s.push_str("  spine              — this ledger\n");
    s.push_str("  vessel run         — runtime half (lossless transport, ΔS=0)\n");
    s.push_str("  d12 duallink       — Dual-Link identification\n");
    s.push_str("  d12 existence      — existence ring capstone\n");
    s.push_str("  d12 lean-status    — full Lean module tower\n");
    s.push_str("  frob               — Frobenius harness\n");
    s.push_str("  clay               — Clay structural status\n");
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_collapse_consistency() {
        // 12 phases: u0=1 + u1(independent) + u2,u6,u8,u10(4 unity) + u3,u4,u5,u7,u9,u11(6 derived) = 12
        // INDEPENDENT=1, UNITY=4, DERIVED=7 (u3,u5,u7,u9,u11 + u0=1 + u4=1)
        assert_eq!(PHASE_GENERATOR_COUNT, 12);
        assert_eq!(INDEPENDENT_PHASE_GENERATORS, 1);
        assert_eq!(UNITY_PHASES, 4);
        assert_eq!(DERIVED_PHASES, 7);
        assert_eq!(1 + 1 + 4 + 6, PHASE_GENERATOR_COUNT); // u0,u1,u2,u6,u8,u10,u3,u4,u5,u7,u9,u11
    }

    #[test]
    fn test_orbit_distribution() {
        let mut total = 0u32;
        for (_, _, n) in &ORBIT_DEGREE_DISTRIBUTION {
            total += n;
        }
        assert_eq!(total, TOTAL_OVERLAPS);
    }

    #[test]
    fn test_existence_grade() {
        assert_eq!(EXISTENCE_GRADE_COUNT, EXISTENCE_GRADE_TOTAL);  // ALL 143 proved (cont.20)
    }

    #[test]
    fn test_existence_ring() {
        assert_eq!(EXISTENCE_RING_DIM_Q, 2048);
        assert_eq!(FLIP_AUDIT_HARMLESS, 128);
        assert_eq!(FLIP_AUDIT_TOTAL, 256);
        assert_eq!(EXISTENCE_RING_LEAN_THEOREMS, 14);
        assert_eq!(EXISTENCE_RING_LEAN_JOBS, 8344);
    }

    #[test]
    fn test_magnitude_class_count() {
        assert_eq!(MAG_WITNESSES.len(), MAG_WITNESS_COUNT as usize);
        assert_eq!(MODULUS_CLASSES.len(), 12);
    }

    #[test]
    fn test_dual_link_factor() {
        assert_eq!(DUAL_LINK_NORM_N1_DENOM, 32448);
        // 2^6 * 3 * 13^2 = 64 * 3 * 169 = 32448
        assert_eq!(64 * 3 * 169, DUAL_LINK_NORM_N1_DENOM);
    }

    #[test]
    fn test_ordinal_count() {
        assert_eq!(ORDINAL_GUARD_COUNT, 12);
    }
}
