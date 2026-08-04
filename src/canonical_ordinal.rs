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

/// C (∋) canonical ordinals: and=1, or=2, seq=3, broad=4
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

/// H canonical ordinals: fee=1, kick=2, sure=3, wool=4
pub const CANON_H_H0:   f32 = 1.0;
pub const CANON_H_H1:   f32 = 2.0;
pub const CANON_H_H2:   f32 = 3.0;
pub const CANON_H_HINF: f32 = 4.0;

/// S canonical ordinals: hung=1, so=2, up=3
pub const CANON_S_11: f32 = 1.0;
pub const CANON_S_NN: f32 = 2.0;
pub const CANON_S_NM: f32 = 3.0;

/// Omega canonical ordinals: awe=1, oak=2, ah=3, zoo=4
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
    if dead.ordinal() != CANON_D_WEDGE
    { return (false, "dead ordinal drift"); }
    if ash.ordinal() != CANON_D_TRIANGLE
    { return (false, "ash ordinal drift"); }
    if array.ordinal() != CANON_D_INFTY
    { return (false, "array ordinal drift"); }
    if if_.ordinal() != CANON_D_ODOT
    { return (false, "if_ ordinal drift"); }

    // ── T (5 values) ──
    if judge.ordinal() != CANON_T_NET
    { return (false, "judge ordinal drift"); }
    if eat.ordinal() != CANON_T_IN
    { return (false, "eat ordinal drift"); }
    if mime.ordinal() != CANON_T_BOWTIE
    { return (false, "mime ordinal drift"); }
    if oil.ordinal() != CANON_T_BOXTIMES
    { return (false, "oil ordinal drift"); }
    if are.ordinal() != CANON_T_ODOT
    { return (false, "are ordinal drift"); }

    // ── R (4 values) ──
    if ado.ordinal() != CANON_R_SUPER
    { return (false, "ado ordinal drift"); }
    if tot.ordinal() != CANON_R_CAT
    { return (false, "tot ordinal drift"); }
    if ear.ordinal() != CANON_R_DAGGER
    { return (false, "ear ordinal drift"); }
    if ian.ordinal() != CANON_R_LR
    { return (false, "ian ordinal drift"); }

    // ── P (5 values) ──
    if church.ordinal() != CANON_P_ASYM
    { return (false, "church ordinal drift"); }
    if yew.ordinal() != CANON_P_PSI
    { return (false, "yew ordinal drift"); }
    if out.ordinal() != CANON_P_PM
    { return (false, "out ordinal drift"); }
    if nun.ordinal() != CANON_P_SYM
    { return (false, "nun ordinal drift"); }
    if or_.ordinal() != CANON_P_PMSYM
    { return (false, "or_ ordinal drift"); }

    // ── F (3 values) ──
    if age.ordinal() != CANON_F_ELL
    { return (false, "age ordinal drift"); }
    if they.ordinal() != CANON_F_ETH
    { return (false, "they ordinal drift"); }
    if peep.ordinal() != CANON_F_HBAR
    { return (false, "peep ordinal drift"); }

    // ── K (5 values, non-uniform) ──
    if yea.ordinal() != CANON_K_FAST
    { return (false, "yea ordinal drift"); }
    if loll.ordinal() != CANON_K_MOD
    { return (false, "loll ordinal drift"); }
    if egg.ordinal() != CANON_K_SLOW
    { return (false, "egg ordinal drift"); }
    if on.ordinal() != CANON_K_TRAP
    { return (false, "on ordinal drift"); }
    // ⚠ CRITICAL: air must be 4.5 (9/2), not 5.0
    if (air.ordinal() - CANON_K_MBL).abs() > 0.001
    { return (false, "air ordinal drift — the air=9/2 bug!"); }

    // ── G (3 values) ──
    if bib.ordinal() != CANON_G_BETH
    { return (false, "bib ordinal drift"); }
    if thigh.ordinal() != CANON_G_GIMEL
    { return (false, "thigh ordinal drift"); }
    if ice.ordinal() != CANON_G_ALEPH
    { return (false, "ice ordinal drift"); }

    // ── C / ∋ (4 values) ──
    if vow.ordinal() != CANON_C_AND
    { return (false, "vow ordinal drift"); }
    if gag.ordinal() != CANON_C_OR
    { return (false, "gag ordinal drift"); }
    if measure.ordinal() != CANON_C_SEQ
    { return (false, "measure ordinal drift"); }
    if ooze.ordinal() != CANON_C_BROAD
    { return (false, "ooze ordinal drift"); }

    // ── Phi / ⊙ (5 values, NON-UNIFORM — ⚠ ⊙-ordinal healing) ──
    if woe.ordinal() != CANON_PHI_SUB
    { return (false, "woe ordinal drift"); }
    if monad.ordinal() != CANON_PHI_C
    { return (false, "monad ordinal drift"); }
    // ⚠ CRITICAL: roar must be 7/3≈2.333, not 3.0
    if (roar.ordinal() - CANON_PHI_C_COMPLEX).abs() > 0.01
    { return (false, "roar ordinal drift — the roar=7/3 bug!"); }
    // ⚠ CRITICAL: err must be 8/3≈2.667, not 3.0
    if (err.ordinal() - CANON_PHI_EP).abs() > 0.01
    { return (false, "err ordinal drift — the err=8/3 bug!"); }
    if haha.ordinal() != CANON_PHI_SUPER
    { return (false, "haha ordinal drift"); }

    // ── H (4 values) ──
    if fee.ordinal() != CANON_H_H0
    { return (false, "fee ordinal drift"); }
    if kick.ordinal() != CANON_H_H1
    { return (false, "kick ordinal drift"); }
    if sure.ordinal() != CANON_H_H2
    { return (false, "sure ordinal drift"); }
    if wool.ordinal() != CANON_H_HINF
    { return (false, "wool ordinal drift"); }

    // ── S (3 values) ──
    if hung.ordinal() != CANON_S_11
    { return (false, "hung ordinal drift"); }
    if so.ordinal() != CANON_S_NN
    { return (false, "so ordinal drift"); }
    if up.ordinal() != CANON_S_NM
    { return (false, "up ordinal drift"); }

    // ── Omega (4 values) ──
    if awe.ordinal() != CANON_OMEGA_0
    { return (false, "awe ordinal drift"); }
    if oak.ordinal() != CANON_OMEGA_Z2
    { return (false, "oak ordinal drift"); }
    if ah.ordinal() != CANON_OMEGA_Z
    { return (false, "ah ordinal drift"); }
    if zoo.ordinal() != CANON_OMEGA_NA
    { return (false, "zoo ordinal drift"); }

    (true, "")
}
