// canonical_ordinal.rs — Canonical ordinal faithfulness guard
//
// GENERATED from Imscribing.Millennium.CanonicalOrdinalFaithfulness.lean
// in p4rakernel/p4ramill. DO NOT EDIT BY HAND. Regenerate when the
// canonical ORDINALS table changes.
//
// Each constant embeds the exact ordinal value from the Lean machine-checked
// source of truth. The boot-time guard `verify_canonical_ordinals()` compares
// `IgPrim::ordinal()` against these constants and returns false if any drift
// has occurred.
//
// Author: Lando⊗⊙perator
// Date: 2026-07-02 (⊙-ordinal healing)

use crate::imas_ig::IgPrim;

// ═══════════════════════════════════════════════════════════════
// CANONICAL ORDINAL CONSTANTS
//
// Sourced from CanonicalOrdinalFaithfulness.lean (proved by native_decide).
// The Lean uses constructor names; we map to Rust IgPrim variants.
// All values are f32 for comparison with IgPrim::ordinal() output.
// ═══════════════════════════════════════════════════════════════

/// D canonical ordinals: wedge=1, triangle=2, infty=3, odot=4
pub const CANON_D_WEDGE:    f32 = 1.0;
pub const CANON_D_TRIANGLE: f32 = 2.0;
pub const CANON_D_INFTY:    f32 = 3.0;
pub const CANON_D_ODOT:     f32 = 4.0;

/// T canonical ordinals: net=1, in=2, bowtie=3, boxtimes=4, odot=5
pub const CANON_T_NET:       f32 = 1.0;
pub const CANON_T_IN:        f32 = 2.0;
pub const CANON_T_BOWTIE:    f32 = 3.0;
pub const CANON_T_BOXTIMES:  f32 = 4.0;
pub const CANON_T_ODOT:      f32 = 5.0;

/// R canonical ordinals: super=1, cat=2, dagger=3, lr=4
pub const CANON_R_SUPER:  f32 = 1.0;
pub const CANON_R_CAT:    f32 = 2.0;
pub const CANON_R_DAGGER: f32 = 3.0;
pub const CANON_R_LR:     f32 = 4.0;

/// P canonical ordinals: asym=1, psi=2, pm=3, sym=4, pmsym=5
pub const CANON_P_ASYM:  f32 = 1.0;
pub const CANON_P_PSI:   f32 = 2.0;
pub const CANON_P_PM:    f32 = 3.0;
pub const CANON_P_SYM:   f32 = 4.0;
pub const CANON_P_PMSYM: f32 = 5.0;

/// F canonical ordinals: ell=1, eth=2, hbar=3
pub const CANON_F_ELL:  f32 = 1.0;
pub const CANON_F_ETH:  f32 = 2.0;
pub const CANON_F_HBAR: f32 = 3.0;

/// K canonical ordinals — NON-UNIFORM (⊙-ordinal healing):
///   fast=1.0, mod=2.0, slow=3.0, trap=4.0, mbl=9/2=4.5
/// The 4.5 for mbl comes from the Lean: ordinalK KineticChar.air = 9/2.
pub const CANON_K_FAST: f32 = 1.0;
pub const CANON_K_MOD:  f32 = 2.0;
pub const CANON_K_SLOW: f32 = 3.0;
pub const CANON_K_TRAP: f32 = 4.0;
pub const CANON_K_MBL:  f32 = 9.0 / 2.0; // = 4.5

/// G canonical ordinals: beth=1, gimel=2, aleph=3
pub const CANON_G_BETH:  f32 = 1.0;
pub const CANON_G_GIMEL: f32 = 2.0;
pub const CANON_G_ALEPH: f32 = 3.0;

/// C (ɢ) canonical ordinals: and=1, or=2, seq=3, broad=4
pub const CANON_C_AND:   f32 = 1.0;
pub const CANON_C_OR:    f32 = 2.0;
pub const CANON_C_SEQ:   f32 = 3.0;
pub const CANON_C_BROAD: f32 = 4.0;

/// Phi (⊙) canonical ordinals — NON-UNIFORM (⊙-ordinal healing):
///   sub=1.0, c=2.0, c_complex=7/3≈2.333..., ep=8/3≈2.667..., super=3.0
/// From Lean: ordinalPhi Criticality.roar = 7/3, ordinalPhi Criticality.err = 8/3.
pub const CANON_PHI_SUB:        f32 = 1.0;
pub const CANON_PHI_C:          f32 = 2.0;
pub const CANON_PHI_C_COMPLEX:  f32 = 7.0 / 3.0; // ≈ 2.333...
pub const CANON_PHI_EP:         f32 = 8.0 / 3.0; // ≈ 2.667...
pub const CANON_PHI_SUPER:      f32 = 3.0;

/// H canonical ordinals: H0=1, H1=2, H2=3, H_inf=4
pub const CANON_H_H0:   f32 = 1.0;
pub const CANON_H_H1:   f32 = 2.0;
pub const CANON_H_H2:   f32 = 3.0;
pub const CANON_H_HINF: f32 = 4.0;

/// S canonical ordinals: S_11=1, S_nn=2, S_nm=3
pub const CANON_S_11: f32 = 1.0;
pub const CANON_S_NN: f32 = 2.0;
pub const CANON_S_NM: f32 = 3.0;

/// Omega canonical ordinals: Omega_0=1, Omega_z2=2, Omega_z=3, Omega_na=4
pub const CANON_OMEGA_0:  f32 = 1.0;
pub const CANON_OMEGA_Z2: f32 = 2.0;
pub const CANON_OMEGA_Z:  f32 = 3.0;
pub const CANON_OMEGA_NA: f32 = 4.0;

// ═══════════════════════════════════════════════════════════════
// BOOT-TIME VERIFICATION
// ═══════════════════════════════════════════════════════════════

/// Verify that every IgPrim variant's `ordinal()` matches the canonical
/// constants from the Lean machine-checked source of truth.
///
/// Returns (true, "") if all 44 primitive values match.
/// Returns (false, diagnostic) if any drift is detected.
///
/// This is the ⊙-ordinal drift guard: it catches the exact class of bug
/// that made RH incorrectly appear to close under triple_criticality
/// before the ordinalPhi healing (roar=7/3, not 3).
pub fn verify_canonical_ordinals() -> (bool, &'static str) {
    use IgPrim::*;

    // ── D (4 values) ──
    if D_wedge.ordinal() != CANON_D_WEDGE
    { return (false, "D_wedge ordinal drift"); }
    if D_triangle.ordinal() != CANON_D_TRIANGLE
    { return (false, "D_triangle ordinal drift"); }
    if D_infty.ordinal() != CANON_D_INFTY
    { return (false, "D_infty ordinal drift"); }
    if D_odot.ordinal() != CANON_D_ODOT
    { return (false, "D_odot ordinal drift"); }

    // ── T (5 values) ──
    if T_net.ordinal() != CANON_T_NET
    { return (false, "T_net ordinal drift"); }
    if T_in.ordinal() != CANON_T_IN
    { return (false, "T_in ordinal drift"); }
    if T_bowtie.ordinal() != CANON_T_BOWTIE
    { return (false, "T_bowtie ordinal drift"); }
    if T_boxtimes.ordinal() != CANON_T_BOXTIMES
    { return (false, "T_boxtimes ordinal drift"); }
    if T_odot.ordinal() != CANON_T_ODOT
    { return (false, "T_odot ordinal drift"); }

    // ── R (4 values) ──
    if R_super.ordinal() != CANON_R_SUPER
    { return (false, "R_super ordinal drift"); }
    if R_cat.ordinal() != CANON_R_CAT
    { return (false, "R_cat ordinal drift"); }
    if R_dagger.ordinal() != CANON_R_DAGGER
    { return (false, "R_dagger ordinal drift"); }
    if R_lr.ordinal() != CANON_R_LR
    { return (false, "R_lr ordinal drift"); }

    // ── P (5 values) ──
    if P_asym.ordinal() != CANON_P_ASYM
    { return (false, "P_asym ordinal drift"); }
    if P_psi.ordinal() != CANON_P_PSI
    { return (false, "P_psi ordinal drift"); }
    if P_pm.ordinal() != CANON_P_PM
    { return (false, "P_pm ordinal drift"); }
    if P_sym.ordinal() != CANON_P_SYM
    { return (false, "P_sym ordinal drift"); }
    if P_pmsym.ordinal() != CANON_P_PMSYM
    { return (false, "P_pmsym ordinal drift"); }

    // ── F (3 values) ──
    if F_ell.ordinal() != CANON_F_ELL
    { return (false, "F_ell ordinal drift"); }
    if F_eth.ordinal() != CANON_F_ETH
    { return (false, "F_eth ordinal drift"); }
    if F_hbar.ordinal() != CANON_F_HBAR
    { return (false, "F_hbar ordinal drift"); }

    // ── K (5 values, non-uniform) ──
    if K_fast.ordinal() != CANON_K_FAST
    { return (false, "K_fast ordinal drift"); }
    if K_mod.ordinal() != CANON_K_MOD
    { return (false, "K_mod ordinal drift"); }
    if K_slow.ordinal() != CANON_K_SLOW
    { return (false, "K_slow ordinal drift"); }
    if K_trap.ordinal() != CANON_K_TRAP
    { return (false, "K_trap ordinal drift"); }
    // ⚠ CRITICAL: K_mbl must be 4.5 (9/2), not 5.0
    if (K_mbl.ordinal() - CANON_K_MBL).abs() > 0.001
    { return (false, "K_mbl ordinal drift — the air=9/2 bug!"); }

    // ── G (3 values) ──
    if G_beth.ordinal() != CANON_G_BETH
    { return (false, "G_beth ordinal drift"); }
    if G_gimel.ordinal() != CANON_G_GIMEL
    { return (false, "G_gimel ordinal drift"); }
    if G_aleph.ordinal() != CANON_G_ALEPH
    { return (false, "G_aleph ordinal drift"); }

    // ── C / ɢ (4 values) ──
    if C_and.ordinal() != CANON_C_AND
    { return (false, "C_and ordinal drift"); }
    if C_or.ordinal() != CANON_C_OR
    { return (false, "C_or ordinal drift"); }
    if C_seq.ordinal() != CANON_C_SEQ
    { return (false, "C_seq ordinal drift"); }
    if C_broad.ordinal() != CANON_C_BROAD
    { return (false, "C_broad ordinal drift"); }

    // ── Phi / ⊙ (5 values, NON-UNIFORM — ⚠ ⊙-ordinal healing) ──
    if Phi_sub.ordinal() != CANON_PHI_SUB
    { return (false, "Phi_sub ordinal drift"); }
    if Phi_c.ordinal() != CANON_PHI_C
    { return (false, "Phi_c ordinal drift"); }
    // ⚠ CRITICAL: Phi_c_complex must be 7/3≈2.333, not 3.0
    if (Phi_c_complex.ordinal() - CANON_PHI_C_COMPLEX).abs() > 0.01
    { return (false, "Phi_c_complex ordinal drift — the roar=7/3 bug!"); }
    // ⚠ CRITICAL: Phi_ep must be 8/3≈2.667, not 3.0
    if (Phi_ep.ordinal() - CANON_PHI_EP).abs() > 0.01
    { return (false, "Phi_ep ordinal drift — the err=8/3 bug!"); }
    if Phi_super.ordinal() != CANON_PHI_SUPER
    { return (false, "Phi_super ordinal drift"); }

    // ── H (4 values) ──
    if H0.ordinal() != CANON_H_H0
    { return (false, "H0 ordinal drift"); }
    if H1.ordinal() != CANON_H_H1
    { return (false, "H1 ordinal drift"); }
    if H2.ordinal() != CANON_H_H2
    { return (false, "H2 ordinal drift"); }
    if H_inf.ordinal() != CANON_H_HINF
    { return (false, "H_inf ordinal drift"); }

    // ── S (3 values) ──
    if S_11.ordinal() != CANON_S_11
    { return (false, "S_11 ordinal drift"); }
    if S_nn.ordinal() != CANON_S_NN
    { return (false, "S_nn ordinal drift"); }
    if S_nm.ordinal() != CANON_S_NM
    { return (false, "S_nm ordinal drift"); }

    // ── Omega (4 values) ──
    if Omega_0.ordinal() != CANON_OMEGA_0
    { return (false, "Omega_0 ordinal drift"); }
    if Omega_z2.ordinal() != CANON_OMEGA_Z2
    { return (false, "Omega_z2 ordinal drift"); }
    if Omega_z.ordinal() != CANON_OMEGA_Z
    { return (false, "Omega_z ordinal drift"); }
    if Omega_na.ordinal() != CANON_OMEGA_NA
    { return (false, "Omega_na ordinal drift"); }

    (true, "")
}
