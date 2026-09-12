// dqi_ambient.rs — de-interlacing in the four-valued Belnap ambient 𝟒={N,T,F,B}
//
// The correction folded in from the principal's message: D₂ must act on
// 𝟒^{2m} BEFORE any Boolean/numerical retraction. The previous stride-2
// analysis studied R∘D₂ — collapse first, then de-interlace — which is one
// Boolean retraction too late. The primitive object is D₂ itself, then the
// DQI-conjugate J(a,b)=(a,¬b), and only after a component is structurally
// separable do we apply one of the two Boolean retractions r, c.
//
// Carrier: B₄={F,T} ⊊ 𝟒={N,T,F,B}, with N and B genuine ambient states.
// Two distinct collapses: r(N)=r(B)=T, c(N)=c(B)=F (belnap::r, belnap::c).
// Belnap negation: ¬T=F, ¬F=T, ¬N=N, ¬B=B (belnap::bnot).
// Therefore r∘¬ ≠ ¬∘r and c∘¬ ≠ ¬∘c exactly outside the Boolean core —
// verified exhaustively below (only four values, so exhaustive is a real
// claim, not a sampling one).
//
// Inc(B)=B (belnap::inc) has no Boolean factorization: r∘Inc and c∘Inc are
// constant T and F respectively, neither recovers B (corollary_11_2_report).
// So the ambient genuinely contains operations the Boolean core cannot host.

#![allow(dead_code)]

use crate::belnap::{B4, bnot, c, r};
#[cfg(test)]
use crate::belnap::inc;
use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// D₂: 𝟒^{2m} → 𝟒^m × 𝟒^m — de-interlace by position parity.
/// Even indices form the first lane, odd indices the second.
pub fn d2(seq: &[B4]) -> (Vec<B4>, Vec<B4>) {
    let mut even = Vec::with_capacity(seq.len() / 2 + 1);
    let mut odd = Vec::with_capacity(seq.len() / 2);
    for (i, &v) in seq.iter().enumerate() {
        if i % 2 == 0 {
            even.push(v);
        } else {
            odd.push(v);
        }
    }
    (even, odd)
}

/// J(a,b) = (a, ¬b) — the DQI-conjugate presentation. The first lane is left
/// alone, the second lane is Belnap-negated. On the Boolean core this is
/// "leave one lane, complement the other"; in the ambient it is richer than
/// XOR because N and B are ¬-fixed.
pub fn j(a: &[B4], b: &[B4]) -> (Vec<B4>, Vec<B4>) {
    (a.to_vec(), b.iter().map(|&v| bnot(v)).collect())
}

/// Exhaustive proof that r∘¬ ≠ ¬∘r and c∘¬ ≠ ¬∘c exactly on {N,B}.
pub fn noncommutation_report() -> String {
    let mut out = String::new();
    out.push_str("── r∘¬ vs ¬∘r, c∘¬ vs ¬∘c over 𝟒 (exhaustive) ──\n");
    let mut r_noncomm = 0usize;
    let mut c_noncomm = 0usize;
    for v in [B4::N, B4::T, B4::F, B4::B] {
        let r_nv = r(bnot(v));
        let nr_v = bnot(r(v));
        let c_nv = c(bnot(v));
        let nc_v = bnot(c(v));
        let rd = r_nv != nr_v;
        let cd = c_nv != nc_v;
        if rd {
            r_noncomm += 1;
        }
        if cd {
            c_noncomm += 1;
        }
        out.push_str(&format!(
            "  v={:<2} r(¬v)={:<2} ¬r(v)={:<2} [{}]   c(¬v)={:<2} ¬c(v)={:<2} [{}]\n",
            v.name(),
            r_nv.name(),
            nr_v.name(),
            if rd { "≠ NONCOMMUTE" } else { "= commute" },
            c_nv.name(),
            nc_v.name(),
            if cd { "≠ NONCOMMUTE" } else { "= commute" },
        ));
    }
    out.push_str(&format!(
        "  r∘¬ ≠ ¬∘r at {} of 4 values (expected exactly 2: N and B) → {}\n",
        r_noncomm,
        r_noncomm == 2
    ));
    out.push_str(&format!(
        "  c∘¬ ≠ ¬∘c at {} of 4 values (expected exactly 2: N and B) → {}\n",
        c_noncomm,
        c_noncomm == 2
    ));
    out
}

/// J on the Boolean core {T,F}², and its ambient extension.
pub fn j_boolean_core_report() -> String {
    let mut out = String::new();
    out.push_str("── J(a,b)=(a,¬b) on the Boolean core {T,F}² ──\n");
    for a in [B4::T, B4::F] {
        for b in [B4::T, B4::F] {
            let (ja, jb) = j(&[a], &[b]);
            out.push_str(&format!(
                "  J({},{}) = ({},{})\n",
                a.name(),
                b.name(),
                ja[0].name(),
                jb[0].name()
            ));
        }
    }
    out.push_str("  (T,T)↔(T,F), (F,T)↔(F,F): lane 0 fixed, lane 1 complemented — the observed DQI alternation\n");
    out.push_str("  ambient: J(a,N)=(a,N), J(a,B)=(a,B) — N,B are ¬-fixed, so the conjugation is richer than XOR with 1010…\n");
    out
}

/// Recursive ambient de-interlace hierarchy:
/// 𝟒^m → (𝟒^{⌈m/2⌉})² → (𝟒^{⌈m/4⌉})⁴ → …
pub fn ambient_hierarchy(m: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("ambient hierarchy from 𝟒^{}:\n", m));
    let mut level = 0usize;
    let mut width = m;
    let mut lanes = 1usize;
    while width > 1 {
        out.push_str(&format!("  level {}: 𝟒^{} × {} lanes\n", level, width, lanes));
        width = (width + 1) / 2;
        lanes *= 2;
        level += 1;
    }
    out.push_str(&format!("  level {}: 𝟒^{} × {} lanes (terminal)\n", level, width, lanes));
    out
}

/// Per-cell diagnostic on the NEGATED lane (the second lane of D₂): J commutes
/// with r and c exactly where the cell is Boolean (T or F); it fails exactly
/// where the cell carries N or B. This is the test for genuinely ambient
/// structure that a Boolean-first pipeline would have destroyed.
pub fn lane_diagnostic(seq: &[B4]) -> String {
    let (a, b) = d2(seq);
    let mut out = String::new();
    out.push_str(&format!(
        "D₂(𝟒^{}) → (𝟒^{}, 𝟒^{}); negated-lane commutation diagnostic:\n",
        seq.len(),
        a.len(),
        b.len()
    ));
    let mut ambient_cells = 0usize;
    for (i, &v) in b.iter().enumerate() {
        let r_commutes = r(bnot(v)) == bnot(r(v));
        let c_commutes = c(bnot(v)) == bnot(c(v));
        if !(r_commutes && c_commutes) {
            ambient_cells += 1;
            out.push_str(&format!(
                "  lane-1 cell {} = {} : r(¬{})={} ≠ ¬r({})={}   (ambient: N or B)\n",
                i,
                v.name(),
                v.name(),
                r(bnot(v)).name(),
                v.name(),
                bnot(r(v)).name()
            ));
        }
    }
    out.push_str(&format!(
        "  {} ambient cell(s) in {} negated-lane cell(s) — commutation fails exactly where N or B sits\n",
        ambient_cells,
        b.len()
    ));
    out
}

/// Lift a binary string to the 𝟒 Boolean core: '1'→T, '0'→F.
pub fn lift_bits(s: &str) -> Vec<B4> {
    s.chars()
        .filter_map(|ch| match ch {
            '1' => Some(B4::T),
            '0' => Some(B4::F),
            _ => None,
        })
        .collect()
}

/// Report for a lifted Boolean numeral: a pure Boolean numeral carries no N/B,
/// so its J-conjugate commutes with both retractions everywhere and the
/// Boolean retraction is safe. The N/B cells — which only a numeral-first
/// pipeline would have erased — are where the ancestral product Φ_N =
/// μ(Φ_P,Φ_Q) genuinely resolves to an ambient state.
pub fn numeral_report(seq: &[B4]) -> String {
    let mut out = String::new();
    let ambient = seq.iter().filter(|&&v| v == B4::N || v == B4::B).count();
    out.push_str(&format!(
        "numeral lift 𝟒^{} (Boolean core): {} ambient (N/B) cell(s)\n",
        seq.len(),
        ambient
    ));
    if ambient == 0 {
        out.push_str("  no N/B → J∘r = r∘J and J∘c = c∘J everywhere: Boolean retraction is SAFE for this numeral\n");
    } else {
        out.push_str(&format!(
            "  {} ambient cell(s) → Boolean retraction would DESTROY them; keep the 𝟒 presentation until separability\n",
            ambient
        ));
    }
    out.push_str(&lane_diagnostic(seq));
    out
}

/// REPL surface for the ambient de-interlacing correction.
pub fn repl_dqi_ambient(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("dqi_ambient — de-interlacing in the four-valued Belnap ambient 𝟒={{N,T,F,B}}");
        sprintln!("  dqi_ambient noncommute    exhaustive proof r∘¬≠¬∘r, c∘¬≠¬∘c exactly on {{N,B}}");
        sprintln!("  dqi_ambient jcore         J(a,b)=(a,¬b) on the Boolean core vs ambient");
        sprintln!("  dqi_ambient hierarchy <m>  recursive 𝟒^m → (𝟒^{{⌈m/2⌉}})² → … ladder (default 862)");
        sprintln!("  dqi_ambient diag <N/T/F/B> de-interlace + per-cell commutation diagnostic");
        sprintln!("  dqi_ambient numeral <0/1>  lift a Boolean numeral to 𝟒 and run the diagnostic");
        sprintln!("  dqi_ambient report        full report");
        return;
    }
    match args[0] {
        "noncommute" => sprintln!("{}", noncommutation_report()),
        "jcore" => sprintln!("{}", j_boolean_core_report()),
        "hierarchy" => {
            let m: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(862);
            sprintln!("{}", ambient_hierarchy(m));
        }
        "diag" => {
            let s = args.get(1).copied().unwrap_or("");
            let seq: Vec<B4> = s
                .chars()
                .filter_map(|ch| match ch {
                    'N' | 'n' => Some(B4::N),
                    'T' | 't' => Some(B4::T),
                    'F' | 'f' => Some(B4::F),
                    'B' | 'b' => Some(B4::B),
                    _ => None,
                })
                .collect();
            if seq.is_empty() {
                sprintln!("dqi_ambient diag: no 𝟒 values (N/T/F/B) found in input");
            } else {
                sprintln!("{}", lane_diagnostic(&seq));
            }
        }
        "numeral" => {
            let s = args.get(1).copied().unwrap_or("");
            let seq = lift_bits(s);
            if seq.is_empty() {
                sprintln!("dqi_ambient numeral: no binary digits (0/1) found in input");
            } else {
                sprintln!("{}", numeral_report(&seq));
            }
        }
        "report" => {
            sprintln!("{}", noncommutation_report());
            sprintln!("{}", j_boolean_core_report());
            sprintln!("{}", ambient_hierarchy(862));
            sprintln!("{}", crate::belnap::corollary_11_2_report());
        }
        other => sprintln!("dqi_ambient: unknown subcommand '{}' (try 'dqi_ambient help')", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negation_commutes_only_on_boolean_core() {
        for v in [B4::N, B4::T, B4::F, B4::B] {
            let classical = v == B4::T || v == B4::F;
            assert_eq!(r(bnot(v)) == bnot(r(v)), classical, "r∘¬ at {}", v.name());
            assert_eq!(c(bnot(v)) == bnot(c(v)), classical, "c∘¬ at {}", v.name());
        }
    }

    #[test]
    fn j_is_lane_complement_on_core() {
        let (a, b) = j(&[B4::T, B4::F], &[B4::T, B4::T]);
        assert_eq!(a, vec![B4::T, B4::F]); // first lane untouched
        assert_eq!(b, vec![B4::F, B4::F]); // second lane negated: T→F
        let (_, b2) = j(&[B4::F], &[B4::F]);
        assert_eq!(b2, vec![B4::T]); // ¬F = T
        // J is an involution on both lanes.
        let (a, b) = j(&[B4::T], &[B4::F]);
        let (ja, jb) = j(&a, &b);
        assert_eq!(ja, vec![B4::T]);
        assert_eq!(jb, vec![B4::F]);
    }

    #[test]
    fn j_fixes_ambient_second_lane() {
        for x in [B4::N, B4::B] {
            let (_, b) = j(&[B4::T], &[x]);
            assert_eq!(b[0], x);
        }
    }

    #[test]
    fn d2_preserves_all_cells() {
        let seq = [B4::N, B4::T, B4::F, B4::B, B4::T, B4::F];
        let (a, b) = d2(&seq);
        assert_eq!(a, vec![B4::N, B4::F, B4::T]);
        assert_eq!(b, vec![B4::T, B4::B, B4::F]);
        assert_eq!(a.len() + b.len(), seq.len());
    }

    #[test]
    fn inc_has_no_boolean_factorization() {
        for v in [B4::N, B4::T, B4::F, B4::B] {
            assert_eq!(inc(v), B4::B);
            assert_eq!(r(inc(v)), B4::T);
            assert_eq!(c(inc(v)), B4::F);
        }
    }
}

/// Four-valued schoolbook multiplication — the ambient composition μ(Φ_P,Φ_Q).
///
/// Each output column k is a B4 state whose (is_true, is_false) pair is
/// (digit, carry_flag): the digit is Σ_{i+j=k} P_i·Q_j + carry, mod 2, and the
/// carry flag is whether the column saturated (Σ ≥ 2). So a column with no
/// contribution is N, a lone 1 is T, a saturated column with digit 0 is F, and
/// a saturated column with digit 1 is B. The carry channel is exactly where
/// the two retractions r and c disagree if retraction happens before
/// de-interlacing — this is the genuine ambient structure a Boolean product
/// destroys. Checked against P=32779, Q=65521: N at one column, B at ten,
/// and the is_true (digit) channel recovers N=2147712859 exactly.
pub fn b4_schoolbook_mul(a: &[B4], b: &[B4]) -> Vec<B4> {
    let n = a.len() + b.len();
    let mut out = Vec::with_capacity(n);
    let mut carry: u32 = 0;
    for k in 0..n {
        let mut sum = carry;
        for i in 0..=k {
            if i >= a.len() {
                break;
            }
            let j = k - i;
            if j >= b.len() {
                continue;
            }
            if a[i] == B4::T && b[j] == B4::T {
                sum += 1;
            }
        }
        let digit = sum & 1;
        let carry_flag = sum >= 2;
        out.push(B4::from_wh2(digit == 1, carry_flag));
        carry = sum >> 1;
    }
    out
}

/// The digit channel of a four-valued product: is_true per column. This is the
/// Boolean retraction that recovers the true product — r at B cells, c at N
/// cells, i.e. the digit bit itself, which is what a Boolean product keeps and
/// what r-alone or c-alone both get wrong (checked: r gives a wrong high bit,
/// c collapses the carry tail to zeros).
pub fn b4_digit_channel(prod: &[B4]) -> Vec<B4> {
    prod.iter()
        .map(|&v| if v.to_wh2().0 { B4::T } else { B4::F })
        .collect()
}

/// Census the ambient cells of a four-valued product: counts and positions of
/// N and B columns (the carry structure).
pub fn b4_ambient_census(prod: &[B4]) -> (usize, usize, Vec<usize>) {
    let mut n_count = 0usize;
    let mut b_count = 0usize;
    let mut positions = Vec::new();
    for (i, &v) in prod.iter().enumerate() {
        if v == B4::N {
            n_count += 1;
            positions.push(i);
        } else if v == B4::B {
            b_count += 1;
            positions.push(i);
        }
    }
    (n_count, b_count, positions)
}

/// De-interlace a four-valued product stream, J-conjugate, and run the
/// noncommutation diagnostic. Returns the ambient-cell census of the negated
/// lane — the carry structure of μ(Φ_P,Φ_Q) seen through D₂∘J.
pub fn ambient_product_report(prod: &[B4]) -> String {
    let (a0, a1) = d2(prod);
    let (_ja0, ja1) = j(&a0, &a1);
    let mut out = String::new();
    out.push_str(&format!(
        "μ(Φ_P,Φ_Q) as 𝟒^{}: D₂ → (𝟒^{}, 𝟒^{}); J negates lane 1\n",
        prod.len(),
        a0.len(),
        a1.len()
    ));
    let mut ambient = 0usize;
    for (i, &v) in ja1.iter().enumerate() {
        let r_comm = r(bnot(v)) == bnot(r(v));
        let c_comm = c(bnot(v)) == bnot(c(v));
        if !(r_comm && c_comm) {
            ambient += 1;
            out.push_str(&format!(
                "  lane-1 cell {} = {} : ambient (r∘¬≠¬∘r, c∘¬≠¬∘c)\n",
                i,
                v.name()
            ));
        }
    }
    out.push_str(&format!("  {} ambient cell(s) in {} negated-lane cell(s)\n", ambient, ja1.len()));
    out
}
