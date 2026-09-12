//! baryon_asymmetry.rs — the baryon-asymmetry ob3ect as a live readout.
//!
//! Codifies the ob3ect at `ob3ect/digital/baryon_asymmetry/` and reads its
//! word through the live Grammar instruments rather than echoing the ob3ect's
//! recorded status. The physical claim it carries: the matter surplus of the
//! universe is a banked survival, one unit of matter carried across annihilation
//! because a frame protected it, not a conversion of antimatter into matter.
//! The banked instrument settles this on the word directly; see
//! `ig-docs/baryon_asymmetry_banked_survival.tex` for the full reading.

#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::String;

/// The 15-glyph word the ob3ect minted for baryon asymmetry.
pub const GLYPH_WORD: &str = "⊢⋈⋈⋈∈≻⊤≺⊥⊞∋⊡⊙⋈⊣";

/// Each distinct mark to its baryon-asymmetry element, from the ob3ect JSON.
pub const PHASE_1_MAPPING: [(&str, &str); 12] = [
    ("⊢", "early_universe"),        // VINIT  — uninitialised state
    ("≻", "particle_creation"),     // AFWD   — forward walk
    ("⊤", "matter_affirmed"),       // EVALT  — matter deposit
    ("≺", "annihilating_step"),     // AREV   — reversal, the clear
    ("⊥", "antimatter"),            // EVALF  — the negative arm
    ("∈", "cp_violating_branch"),   // FSPLIT — even/odd arms
    ("∋", "collision_rejoin"),      // FFUSE  — fuse at collision
    ("⊞", "dual_state_collision"),  // ENGAGR — both counts held
    ("⊡", "sphaleron_winding"),     // IFIX   — integer winding, protection
    ("⊙", "phase_transition"),      // IMSCRIB— criticality
    ("⋈", "chain_reaction"),        // CLINK  — chained sequence
    ("⊣", "net_baryon_number"),     // TANCH  — settled surplus
];

/// The banked-survival reading in one paragraph.
pub fn reading() -> String {
    let mut s = String::new();
    s.push_str("Baryon asymmetry as a banked survival:\n");
    s.push_str("  Matter and antimatter both survive; the final register holds T and F together.\n");
    s.push_str("  Annihilation runs: the reversal ≺ is a clear, and it destroys a unit.\n");
    s.push_str("  The asymmetry is the one unit the clear did NOT destroy, banked in a frame\n");
    s.push_str("  opened by the CP-violating branch ∈ and made integer by the winding ⊡.\n");
    s.push_str("  The fuse ∋ restores it. That restored unit is the net baryon number.\n");
    s.push_str("  Sakharov, off the marks: B-violation = the banking, CP = ∈, out-of-equilibrium = ≺ at ⊙.\n");
    s
}

/// The twelve mark-to-physics mappings.
pub fn mapping() -> String {
    let mut s = String::new();
    s.push_str("phase-1 mapping (mark → baryon-asymmetry element):\n");
    for (g, e) in PHASE_1_MAPPING.iter() {
        s.push_str(&format!("  {}  {}\n", g, e));
    }
    s
}

/// Live readout: the word run through the weight and banked instruments, plus
/// its executed crystal address. The banked instrument settles the survival.
pub fn report() -> String {
    let mut s = String::new();
    s.push_str("baryon asymmetry — 15-glyph ob3ect word\n");
    s.push_str(&format!("word: {}\n\n", GLYPH_WORD));
    s.push_str(&mapping());
    s.push('\n');
    s.push_str(&reading());
    s.push_str("\n┌─ weight (live) ─────────────────────────────────────────────\n");
    for line in imasm_core::lattice_flow::weight_report(GLYPH_WORD).lines() {
        s.push_str("│ "); s.push_str(line); s.push('\n');
    }
    s.push_str("├─ banked (live) ─────────────────────────────────────────────\n");
    for line in imasm_core::lattice_flow::banked_report(GLYPH_WORD).lines() {
        s.push_str("│ "); s.push_str(line); s.push('\n');
    }
    s.push_str("└─────────────────────────────────────────────────────────────\n");
    if let Ok(prog) = crate::belnap_ring_shor::program_from_glyphs(GLYPH_WORD) {
        let snap = crate::kernel::self_imscribe(&prog);
        let tup = crate::imas_ig::IgTuple::from_snapshot(&snap);
        s.push_str(&format!("executed crystal address: {}\n", tup.crystal_address()));
    }
    s
}
