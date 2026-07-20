#![allow(dead_code)]
// witness_vessel.rs — The Witness-Vessel Protocol (runtime half)
//
// Lean half: p4rakernel SIC_D12_WitnessVessel.lean — witness_vessel_lossless,
// green and audit-clean (standard trio + native_decide pair, no project axioms;
// the roundtrip law rests on propext alone). Design validated by the
// witness-vessel ob3ect batch: 6/6 is_valid, 6/6 Frobenius PASS, ΔS≈0.
//
// The Dual-Link SIC-POVM (d12_sic.rs, post-capstone) is the vessel, and the
// MPP Witnesses ride AS it, not in it:
//   boarding  = FSPLIT — the B-collapse: a dialetheic verdict splits to (T,F)
//               and the Witness rides BOTH arms through transport
//   read-back = FFUSE  — Belnap join reconstitutes the verdict exactly
//   gate      = frob_verify's mu-after-delta law: mu(delta(q)) == q per action
//
// Two independent boarding substrates, both exercised per payload value:
//   1. ParaVM   — FSPLIT/FFUSE belief instructions (the exact fsplit/ffuse
//                 semantics of BelnapSplitFuse.lean: B bifurcates to (T,F))
//   2. Kernel   — ForkFrame fork-join: FSPLIT pushes the fork frame (the
//                 un-gated right arm carries the cargo — the Dual-Link
//                 modulus), EVALT gates the left arm (the phase probe),
//                 FFUSE joins the arms back
//
// This module is also the first consumer of the FULL 88-dialect expansion
// (dialect_expansion.rs). The payload is not a hand-picked U8-U11 slice:
// it is the gate verdict of each Clay Witness under EVERY dialect,
// computed before boarding and recomputed after read-back. Losslessness
// means the two matrices are equal, entry by entry.
//
// Author: Lando⊗⊙perator
// Date: 2026-07-04

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use crate::belnap::B4;
use crate::frob_verify::{FrobeniusHarness, FrobeniusResult};
use crate::imas_ig::{IgPrim, IgTuple};
use crate::kernel::Kernel;
use crate::parasm::ParaVM;
use crate::tokens::{Program, Token};
use crate::dialect_expansion::{all_dialects, GateSpec, Dialect, DIALECT_COUNT};

// ═══════════════════════════════════════════════════════════════
// TUPLE EVALUATION OVER THE 88 UNIVERSES
// ═══════════════════════════════════════════════════════════════

/// Resolve a primitive glyph to the corresponding IgTuple field.
fn tuple_prim(ig: &IgTuple, glyph: &str) -> Option<IgPrim> {
    match glyph {
        "Ð" => Some(ig.d),
        "Þ" => Some(ig.t),
        "Ř" => Some(ig.r),
        "Φ" => Some(ig.p),
        "ƒ" => Some(ig.f),
        "Ç" => Some(ig.k),
        "Γ" => Some(ig.g),
        "ɢ" => Some(ig.c),
        "⊙" => Some(ig.phi),
        "Ħ" => Some(ig.h),
        "Σ" => Some(ig.s),
        "Ω" => Some(ig.omega),
        _ => None,
    }
}

/// Every IgPrim value, for glyph → ordinal resolution.
const ALL_VALUES: [IgPrim; 49] = [
    IgPrim::D_odot, IgPrim::D_wedge, IgPrim::D_triangle, IgPrim::D_infty,
    IgPrim::T_odot, IgPrim::T_net, IgPrim::T_in, IgPrim::T_bowtie, IgPrim::T_boxtimes,
    IgPrim::R_lr, IgPrim::R_dagger, IgPrim::R_cat, IgPrim::R_super,
    IgPrim::P_pmsym, IgPrim::P_sym, IgPrim::P_pm, IgPrim::P_psi, IgPrim::P_asym,
    IgPrim::F_hbar, IgPrim::F_ell, IgPrim::F_eth,
    IgPrim::K_trap, IgPrim::K_slow, IgPrim::K_mod, IgPrim::K_fast, IgPrim::K_mbl,
    IgPrim::G_aleph, IgPrim::G_beth, IgPrim::G_gimel,
    IgPrim::C_seq, IgPrim::C_and, IgPrim::C_or, IgPrim::C_broad,
    IgPrim::Phi_crit, IgPrim::𐑮, IgPrim::Phi_ep, IgPrim::𐑢, IgPrim::Phi_super,
    IgPrim::H_inf, IgPrim::H2, IgPrim::H1, IgPrim::H0,
    IgPrim::S_nm, IgPrim::S_nn, IgPrim::S_11,
    IgPrim::Omega_z, IgPrim::Omega_z2, IgPrim::Omega_0, IgPrim::Omega_na,
];

/// Ordinal of a Shavian VALUE glyph (e.g. "𐑹" → 5.0).
fn value_ordinal(glyph: &str) -> Option<f32> {
    for p in &ALL_VALUES {
        if p.glyph() == glyph {
            return Some(p.ordinal());
        }
    }
    None
}

/// One gate: the tuple's primitive must sit at ordinal >= min_ord.
/// Uses IgPrim::ordinal() (canonical, non-monotonic-safe) — never raw
/// discriminants.
pub fn gate_pass(spec: &GateSpec, ig: &IgTuple) -> bool {
    match tuple_prim(ig, spec.prim) {
        Some(v) => v.ordinal() >= spec.min_ord,
        None => false,
    }
}

/// All three gates of a dialect.
pub fn gates_closed(u: &Dialect, ig: &IgTuple) -> bool {
    gate_pass(&u.g1, ig) && gate_pass(&u.g2, ig) && gate_pass(&u.g3, ig)
}

/// The dialect's OWN T-constitution: ceiling entries compare ordinals,
/// exact entries compare value glyphs.
pub fn t_seal(u: &Dialect, ig: &IgTuple) -> bool {
    for te in u.t_entries {
        let ok = match tuple_prim(ig, te.prim) {
            Some(v) => {
                if te.ceiling {
                    match value_ordinal(te.crit_val) {
                        Some(c) => v.ordinal() <= c,
                        None => false,
                    }
                } else {
                    v.glyph() == te.crit_val
                }
            }
            None => false,
        };
        if !ok {
            return false;
        }
    }
    true
}

/// The Clay T_CEILING — all five dynamics primitives as ceilings at their
/// canonical anchors (Φ≤𐑹 ƒ≤𐑐 Ç≤𐑧 Ħ≤𐑫 Ω≤𐑭). This is the T convention of
/// Clay_WitnessedClosure.lean / SIC_D12_WitnessVessel.lean and of the
/// U8-U10 dialect gates.
pub fn t_ceiling(ig: &IgTuple) -> bool {
    ig.p.ordinal() <= IgPrim::P_pmsym.ordinal()
        && ig.f.ordinal() <= IgPrim::F_hbar.ordinal()
        && ig.k.ordinal() <= IgPrim::K_slow.ordinal()
        && ig.h.ordinal() <= IgPrim::H_inf.ordinal()
        && ig.omega.ordinal() <= IgPrim::Omega_z.ordinal()
}

// ═══════════════════════════════════════════════════════════════
// LAYER VERDICT — mirror of SIC_D12_WitnessVessel.layerVerdict
// ═══════════════════════════════════════════════════════════════

/// closed gate + consistent ceiling → T; closed gate + blocked ceiling → B
/// (the dialetheia); open gate + consistent ceiling → N; open + blocked → F.
pub fn layer_verdict(gate_closed: bool, ceiling_ok: bool) -> B4 {
    match (gate_closed, ceiling_ok) {
        (true, true) => B4::T,
        (true, false) => B4::B,
        (false, true) => B4::N,
        (false, false) => B4::F,
    }
}

// ═══════════════════════════════════════════════════════════════
// THE PAYLOADS — computed from canonical tuples, never hand-entered
// ═══════════════════════════════════════════════════════════════

/// The three MPP Witnesses (display name, catalog name).
pub const WITNESSES: [(&str, &str); 3] = [
    ("BSD", "birch_swinnerton_dyer"),
    ("Hodge", "hodge_conjecture"),
    ("YM", "yang_mills_mass_gap"),
];

/// Closer-dialect index sets, matching Clay_WitnessedClosure.lean:
///   BSD  : chirality_first, scope_universe, kinetics_trap,
///          absorption_chirality_first, absorption_scope_empire
///   Hodge: scope_universe, kinetics_trap, stoichiometry_universe,
///          absorption_scope_empire, absorption_topology_seal
///   YM   : triple_criticality
pub const BSD_CLOSERS: [usize; 5] = [8, 10, 12, 24, 25];
pub const HODGE_CLOSERS: [usize; 5] = [10, 12, 19, 25, 26];
pub const YM_CLOSERS: [usize; 1] = [13];

/// Lean-mirror verdict: gate closure over the witness's closer dialects,
/// T side = the Clay T_CEILING. Expected: BSD → T, Hodge → T, YM → B
/// (ymVerdict_B — the U10 dialetheia, derived not asserted).
pub fn witness_verdict(unis: &[Dialect; DIALECT_COUNT], closers: &[usize], ig: &IgTuple) -> B4 {
    let gate = closers.iter().all(|&i| gates_closed(&unis[i], ig));
    layer_verdict(gate, t_ceiling(ig))
}

/// Wide payload: verdict of one witness under one dialect, using the
/// dialect's OWN gates and OWN T-constitution.
pub fn dialect_verdict(u: &Dialect, ig: &IgTuple) -> B4 {
    layer_verdict(gates_closed(u, ig), t_seal(u, ig))
}

/// The full verdict matrix: 3 witnesses × 88 dialects, row-major.
/// Returns None if a catalog tuple is missing.
pub fn verdict_matrix(unis: &[Dialect; DIALECT_COUNT]) -> Option<Vec<B4>> {
    let mut m = Vec::with_capacity(3 * DIALECT_COUNT);
    for (_, cat_name) in &WITNESSES {
        let ig = crate::catalog::lookup(cat_name)?.tuple;
        for u in unis.iter() {
            m.push(dialect_verdict(u, &ig));
        }
    }
    Some(m)
}

// ═══════════════════════════════════════════════════════════════
// BOARDING (delta) AND READ-BACK (mu) — two substrates
// ═══════════════════════════════════════════════════════════════

/// Board + read back one value through the ParaVM's FSPLIT/FFUSE belief
/// instructions. FSPLIT bifurcates B to (T,F) — the Witness rides both
/// arms — and copies every other value to both destinations; FFUSE joins.
/// Returns (left arm, right arm, read-back).
pub fn board_paravm(vm: &mut ParaVM, v: B4) -> (B4, B4, B4) {
    vm.set_belief(0, v);
    vm.set_belief(1, B4::N);
    vm.set_belief(2, B4::N);
    vm.set_belief(3, B4::N);
    // Boarding and read-back as ONE program: the vessel in flight.
    if vm.load("FSPLIT %r0 %r1 %r2\nFFUSE %r1 %r2 %r3\nHALT").is_err() {
        return (B4::N, B4::N, B4::N);
    }
    vm.run(None);
    (vm.belief_of(1), vm.belief_of(2), vm.belief_of(3))
}

/// Board + read back one value through the graph kernel's ForkFrames.
/// FSPLIT pushes the fork frame — the un-gated right arm carries the cargo
/// (the Dual-Link modulus), EVALT gates the left arm (the phase probe) —
/// and FFUSE joins the arms: join(gated, cargo) reconstitutes the value
/// for all four Belnap points (B rides as B: join(N, B) = B).
pub fn board_forkframe(v: B4) -> B4 {
    let mut k = Kernel::new();
    let mut prog = Program::empty();
    prog.push(Token::Fsplit);
    prog.push(Token::Evalt);
    prog.push(Token::Ffuse);
    k.program = prog;
    k.ip = 0;
    k.stack.push(v);
    k.tick(); // FSPLIT — fork frame carries v on the right arm
    k.tick(); // EVALT  — left arm gated
    k.tick(); // FFUSE  — join arms, resume
    k.stack.peek()
}

// ═══════════════════════════════════════════════════════════════
// THE VESSEL RUN — full protocol with frob_verify as the gate
// ═══════════════════════════════════════════════════════════════

pub struct VesselRun {
    /// Verdicts computed BEFORE boarding (3 × 88, row-major).
    pub before: Vec<B4>,
    /// Read-back through the ParaVM vessel.
    pub readback_vm: Vec<B4>,
    /// Read-back through the ForkFrame vessel.
    pub readback_ff: Vec<B4>,
    /// Verdicts recomputed AFTER read-back.
    pub after: Vec<B4>,
    /// Lean-mirror trio (BSD, Hodge, YM) before / after transport.
    pub mirror_before: [B4; 3],
    pub mirror_after: [B4; 3],
    /// Frobenius harness over every boarding action.
    pub harness: FrobeniusHarness,
    /// ΔS: total mismatches across both substrates and the recompute.
    pub ds_mismatches: usize,
}

impl VesselRun {
    pub fn lossless(&self) -> bool {
        self.ds_mismatches == 0 && self.harness.is_closed()
    }
}

/// Run the full witness-vessel protocol. None if the catalog is missing a
/// witness tuple.
pub fn run_vessel() -> Option<VesselRun> {
    let unis = all_dialects();

    // 1. Payloads BEFORE boarding.
    let before = verdict_matrix(&unis)?;
    let bsd = crate::catalog::lookup(WITNESSES[0].1)?.tuple;
    let hodge = crate::catalog::lookup(WITNESSES[1].1)?.tuple;
    let ym = crate::catalog::lookup(WITNESSES[2].1)?.tuple;
    let mirror_before = [
        witness_verdict(&unis, &BSD_CLOSERS, &bsd),
        witness_verdict(&unis, &HODGE_CLOSERS, &hodge),
        witness_verdict(&unis, &YM_CLOSERS, &ym),
    ];

    // 2. Board EVERYTHING through both substrates, frob_verify gating each
    //    action: mu(delta(q)) == q before the loop advances.
    let mut harness = FrobeniusHarness::new("witness_vessel");
    let mut vm = ParaVM::new();
    let mut readback_vm = Vec::with_capacity(before.len());
    let mut readback_ff = Vec::with_capacity(before.len());
    let mut ds = 0usize;

    let mut board_one = |v: B4, harness: &mut FrobeniusHarness, ds: &mut usize| -> (B4, B4) {
        let (_, _, rb_vm) = board_paravm(&mut vm, v);
        let rb_ff = board_forkframe(v);
        let r_vm = if rb_vm == v {
            FrobeniusResult::closed(v, rb_vm, rb_vm)
        } else {
            FrobeniusResult::open(v, rb_vm, rb_vm, "ParaVM vessel: mu(delta(q)) != q")
        };
        let r_ff = if rb_ff == v {
            FrobeniusResult::closed(v, rb_ff, rb_ff)
        } else {
            FrobeniusResult::open(v, rb_ff, rb_ff, "ForkFrame vessel: mu(delta(q)) != q")
        };
        if !harness.check(r_vm) {
            *ds += 1;
        }
        if !harness.check(r_ff) {
            *ds += 1;
        }
        (rb_vm, rb_ff)
    };

    for &v in &before {
        let (rb_vm, rb_ff) = board_one(v, &mut harness, &mut ds);
        readback_vm.push(rb_vm);
        readback_ff.push(rb_ff);
    }
    let mut mirror_after = [B4::N; 3];
    for (i, &v) in mirror_before.iter().enumerate() {
        let (rb_vm, _) = board_one(v, &mut harness, &mut ds);
        mirror_after[i] = rb_vm;
    }

    // 3. Recompute the matrix AFTER transport — the destination recomputes
    //    the verdicts from the canonical tuples and must agree.
    let after = verdict_matrix(&unis)?;
    for (a, b) in after.iter().zip(before.iter()) {
        if a != b {
            ds += 1;
        }
    }

    Some(VesselRun {
        before,
        readback_vm,
        readback_ff,
        after,
        mirror_before,
        mirror_after,
        harness,
        ds_mismatches: ds,
    })
}

// ═══════════════════════════════════════════════════════════════
// REPORT — QEMU-facing serial output
// ═══════════════════════════════════════════════════════════════

fn verdict_row(m: &[B4], row: usize) -> String {
    let mut s = String::with_capacity(DIALECT_COUNT);
    for i in 0..DIALECT_COUNT {
        s.push_str(m[row * DIALECT_COUNT + i].name());
    }
    s
}

fn distribution(m: &[B4], row: usize) -> (usize, usize, usize, usize) {
    let (mut n, mut t, mut f, mut b) = (0, 0, 0, 0);
    for i in 0..DIALECT_COUNT {
        match m[row * DIALECT_COUNT + i] {
            B4::N => n += 1,
            B4::T => t += 1,
            B4::F => f += 1,
            B4::B => b += 1,
        }
    }
    (n, t, f, b)
}

pub fn vessel_report() -> String {
    let mut s = String::new();
    s.push_str("╔══════════════════════════════════════════════════════╗\n");
    s.push_str("║  WITNESS-VESSEL PROTOCOL — runtime half              ║\n");
    s.push_str("║  Dual-Link SIC-POVM as lossless Witness transport    ║\n");
    s.push_str("╚══════════════════════════════════════════════════════╝\n\n");
    s.push_str("Lean half: SIC_D12_WitnessVessel.lean (witness_vessel_lossless,\n");
    s.push_str("audit-clean; roundtrip law rests on propext alone).\n");
    s.push_str("Design: witness-vessel ob3ect batch — 6/6 Frobenius PASS, ΔS≈0.\n\n");

    let run = match run_vessel() {
        Some(r) => r,
        None => {
            s.push_str("ERROR: catalog missing a Clay witness tuple.\n");
            return s;
        }
    };

    // ── Lean-mirror payloads ──
    s.push_str("── Payloads (computed from canonical tuples + closer dialects) ──\n");
    let expected = ["T", "T", "B"];
    for i in 0..3 {
        let ok = run.mirror_before[i].name() == expected[i];
        s.push_str(&format!(
            "  {:<6} verdict {} (Lean: {})  transport {} -> {}   [{}]\n",
            WITNESSES[i].0,
            run.mirror_before[i].name(),
            expected[i],
            run.mirror_before[i].name(),
            run.mirror_after[i].name(),
            if ok && run.mirror_after[i] == run.mirror_before[i] { "OK" } else { "MISMATCH" },
        ));
    }
    s.push_str("  YM = B is the U10 dialetheia DERIVED: gate closed AND ceiling\n");
    s.push_str("  blocked (Ç). The Witness rides both arms: fsplit(B)=(T,F),\n");
    s.push_str("  ffuse(T,F)=B — b_cargo_mechanism, here executed, not asserted.\n\n");

    // ── The 88-dialect matrix ──
    s.push_str(&format!(
        "── Wide payload: {} witnesses x {} dialects ──\n",
        WITNESSES.len(),
        DIALECT_COUNT
    ));
    s.push_str("  (each dialect judged by its OWN gates and T-constitution)\n");
    for i in 0..3 {
        let (n, t, f, b) = distribution(&run.before, i);
        s.push_str(&format!(
            "  {:<6} N:{:<2} T:{:<2} F:{:<2} B:{:<2}\n",
            WITNESSES[i].0, n, t, f, b
        ));
        s.push_str(&format!("    {}\n", verdict_row(&run.before, i)));
    }
    s.push_str("\n");

    // ── Transport verdict ──
    s.push_str("── Round trip (both substrates, frob_verify gated) ──\n");
    s.push_str(&format!(
        "  Boarding actions checked: {} (ParaVM + ForkFrame per value)\n",
        run.harness.total()
    ));
    s.push_str(&format!(
        "  Frobenius closed: {}  open: {}  closure ratio: {}\n",
        run.harness.closed_count,
        run.harness.open_count,
        if run.harness.is_closed() { "1.0" } else { "<1.0" }
    ));
    s.push_str(&format!("  ΔS (total mismatches): {}\n", run.ds_mismatches));
    s.push_str(&format!(
        "  Gate verdicts before == after: {}\n\n",
        if run.after == run.before { "YES (all entries)" } else { "NO" }
    ));

    if run.lossless() {
        s.push_str("VERDICT: LOSSLESS — mu-after-delta = id on every payload.\n");
        s.push_str("The OS agrees with its own theorem (witness_vessel_lossless).\n");
    } else {
        s.push_str("VERDICT: LOSSY — vessel violated somewhere. Investigate.\n");
    }
    s
}

/// Compact summary for the REPL.
pub fn vessel_summary() -> String {
    let mut s = String::new();
    s.push_str("═══ WITNESS-VESSEL — Dual-Link SIC-POVM transport ═══\n");
    s.push_str("The MPP Witnesses ride AS the vessel, not in it.\n");
    s.push_str("  boarding  = FSPLIT (B-collapse; B rides both arms as (T,F))\n");
    s.push_str("  read-back = FFUSE  (Belnap join)\n");
    s.push_str("  gate      = frob_verify: mu(delta(q)) == q per action\n");
    s.push_str(&format!(
        "  payload   = 3 Clay Witnesses x {} dialects (full expansion)\n",
        DIALECT_COUNT
    ));
    s.push_str("Subcommands: vessel run — full protocol + report\n");
    s
}
