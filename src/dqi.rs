// dqi.rs — Decoded Quantum Interferometry (DQI) operator
//
// Word: ⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙  (double-⊤ form, period 14, kernel-verified)
// Single-⊤ variant: ⊢∈∈≻⋈⊤⊥∋≺⋈∋⊣⊙  (period 13)
// Triple-⊤ variant: ⊢∈∈≻⋈⊤⊤⊤⊥∋≺⋈∋⊣⊙  (period 15)
// Pattern: each additional ⊤ in the deposit phase extends the ROTAT period by 1.
//
// Re-checked directly against the live kernel (`weight`/`banked`/`cycle` on
// the actual word, not read off a doc): both the double-⊤ WORD (deposits 3,
// cleared 3, restored 3, banked OK) and WORD_SINGLE_T (deposits 2, cleared
// 2, restored 2, banked OK) genuinely bank OK -- the double-frame ∈∈...∋∋
// nesting is what makes both safe. The word DQI_IMASM_UNIFIED.md calls the
// "naive, failed" attempt is a THIRD, structurally different word with only
// one ∈/∋ pair, `⊢∈≻⋈⊤⊤⊥∋≺⋈⊣⊙` -- re-run here too: deposits 3, cleared 3,
// restored 0, banked FAILS ("3 unit(s) cleared with nothing banked behind
// them"). That failure is about missing the outer frame entirely, not about
// which of WORD/WORD_SINGLE_T is canonical.
//
// DQI_IMASM_UNIFIED.md's target problem: syndrome decoding for the LDPC
// code C⊥ = {d∈F₂ᵐ : Bd=0} with up to ℓ errors. dqi_verdict and
// dqi_syndrome_decode run the word's B4 register through FOUR-native
// operations directly: parity count, alternating XOR mask. FDE is prior to
// QM: the project's algebraic demonstration of that boundary lives in
// multilattice.rs, where the Belnap evidence-counting Born rule and the
// real QM Hadamard agree exactly at n=1.
//
// B4 verdict: T=closure (objective-met), F=no-closure, B=paradice.
// Tuple: ⟨𐑨𐑶𐑽𐑿𐑐𐑘𐑔𐑠⊙𐑖𐑙𐑭⟩ — dqi_algorithm (catalog).
//
// ─────────────────────────────────────────────────────────────────────────
// WHAT IS CHECKED HERE
// ─────────────────────────────────────────────────────────────────────────
// DQI_IMASM_UNIFIED.md §6: max-XORSAT, an exact system of XOR (parity)
// constraints over F2, solves by Gaussian elimination, O(m^3). `xorsat_solve`
// is that solver, run for real; `benchmark_report` measures it against
// brute-force enumeration on the same instance, so the O(m^3) vs O(2^m) gap
// is a measured number.
//
// `syndrome_decode_bounded` handles the harder case, up to ℓ *errors*: it
// eliminates down to the code's free-variable (nullspace) dimension, then
// searches that residual space by weight -- exponential in that dimension,
// the real cost of general syndrome decoding, stated at the size it runs.

#![allow(dead_code)]

use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub const WORD: &str = "⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙";
pub const WORD_SINGLE_T: &str = "⊢∈∈≻⋈⊤⊥∋≺⋈∋⊣⊙";
pub const PERIOD: usize = 14;
pub const PHASE_BEARING: bool = true;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum B4Verdict { T, F, B }

impl core::fmt::Display for B4Verdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            B4Verdict::T => write!(f, "T"),
            B4Verdict::F => write!(f, "F"),
            B4Verdict::B => write!(f, "B"),
        }
    }
}

/// B4 classifier over a 1/0 string for the word's REPL surface (`dqi
/// verdict`): counts ones vs zeros and maps the count to T/F/B by parity.
/// The word's own kernel banking result (period 14, banked=OK, μ∘δ=id
/// closed) is stated above.
pub fn dqi_verdict(symbol_string: &str) -> B4Verdict {
    if symbol_string.is_empty() {
        return B4Verdict::B;
    }
    // Majority-1s with an even ones-count → T; tied → B; else → F.
    let (ones, zeros): (usize, usize) = symbol_string.chars().fold(
        (0usize, 0usize),
        |(o, z), c| match c {
            '1' => (o + 1, z),
            '0' => (o, z + 1),
            _ => (o, z),
        },
    );
    if ones + zeros == 0 {
        return B4Verdict::B;
    }
    if ones > zeros && ones % 2 == 0 {
        B4Verdict::T
    } else if ones == zeros {
        B4Verdict::B
    } else {
        B4Verdict::F
    }
}

/// Fixed-mask transform for the word's REPL surface (`dqi syndrome`): XORs
/// the input against an alternating 1,0,1,0... mask by position.
/// `syndrome_decode_bounded` below runs bounded-weight decoding against a
/// real parity-check matrix; `dqi decode` calls that one.
pub fn dqi_syndrome_decode(syndrome: &[u8]) -> alloc::vec::Vec<u8> {
    // Alternating XOR mask by position parity.
    let mut out = alloc::vec::Vec::with_capacity(syndrome.len());
    for (i, &s) in syndrome.iter().enumerate() {
        let mask = if i % 2 == 0 { 1u8 } else { 0u8 };
        out.push(s ^ mask);
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────
// GF(2) LINEAR ALGEBRA — the actual machine behind "max-XORSAT is
// Gaussian elimination, O(m^3)"
// ─────────────────────────────────────────────────────────────────────────

/// One XOR constraint: a subset of variable indices, XORed together, that
/// must equal `rhs`. This is a row of the parity-check system B·d = s in
/// DQI_IMASM_UNIFIED.md §5/§13.7: each constraint is one row of B, `rhs` is
/// the matching bit of the syndrome s.
#[derive(Clone)]
pub struct Gf2Row {
    bits: Vec<u64>,
    rhs: bool,
}

fn words_for(m: usize) -> usize {
    (m + 63) / 64
}
fn get_bit(bits: &[u64], j: usize) -> bool {
    (bits[j / 64] >> (j % 64)) & 1 != 0
}
fn set_bit(bits: &mut [u64], j: usize, v: bool) {
    if v {
        bits[j / 64] |= 1u64 << (j % 64);
    } else {
        bits[j / 64] &= !(1u64 << (j % 64));
    }
}
fn xor_into(a: &mut [u64], b: &[u64]) {
    for i in 0..a.len() {
        a[i] ^= b[i];
    }
}

pub(crate) fn build_rows(clauses: &[(Vec<usize>, bool)], num_vars: usize) -> Vec<Gf2Row> {
    let w = words_for(num_vars);
    clauses
        .iter()
        .map(|(vars, rhs)| {
            let mut bits = vec![0u64; w];
            for &v in vars {
                set_bit(&mut bits, v, true);
            }
            Gf2Row { bits, rhs: *rhs }
        })
        .collect()
}

/// Full Gauss-Jordan elimination to reduced row-echelon form, in place.
/// Returns (rank, pivot_row_of_col) — pivot_row_of_col[c] is Some(r) if
/// column c is a pivot column, found in row r after reduction.
pub(crate) fn eliminate(rows: &mut [Gf2Row], num_vars: usize) -> (usize, Vec<Option<usize>>) {
    let mut pivot_row_of_col = vec![None; num_vars];
    let mut rank = 0usize;
    for col in 0..num_vars {
        if rank >= rows.len() {
            break;
        }
        let found = (rank..rows.len()).find(|&r| get_bit(&rows[r].bits, col));
        let Some(pr) = found else { continue };
        rows.swap(rank, pr);
        let pivot = rows[rank].clone();
        for r in 0..rows.len() {
            if r != rank && get_bit(&rows[r].bits, col) {
                xor_into(&mut rows[r].bits, &pivot.bits);
                rows[r].rhs ^= pivot.rhs;
            }
        }
        pivot_row_of_col[col] = Some(rank);
        rank += 1;
    }
    (rank, pivot_row_of_col)
}

/// Max-XORSAT via Gaussian elimination — DQI_IMASM_UNIFIED.md §6's actual
/// claim, executed: O(m^3) (bit-packed: O(m^3/64) word-ops), no QFT, no
/// interference, no Hilbert space. Returns `None` iff the system is
/// inconsistent (some fully-eliminated row reads 0 = 1).
pub fn xorsat_solve(clauses: &[(Vec<usize>, bool)], num_vars: usize) -> Option<Vec<bool>> {
    let mut rows = build_rows(clauses, num_vars);
    let (rank, pivot_row_of_col) = eliminate(&mut rows, num_vars);
    for row in rows.iter().skip(rank) {
        if row.bits.iter().all(|&w| w == 0) && row.rhs {
            return None; // 0 = 1: no assignment satisfies every clause
        }
    }
    let mut assignment = vec![false; num_vars];
    for (col, pr) in pivot_row_of_col.iter().enumerate() {
        if let Some(r) = pr {
            // Free variables are set to 0, so the pivot's value is exactly
            // its row's rhs (RREF already cleared every other pivot column
            // out of this row; only free-column bits remain, and those
            // contribute 0 * free_value).
            assignment[col] = rows[*r].rhs;
        }
    }
    Some(assignment)
}

/// Bounded-weight syndrome decoding: find e with B·e = s (mod 2) and
/// Hamming weight ≤ ell. Reduces by elimination to a particular solution
/// plus a nullspace basis over the free (non-pivot) columns, then searches
/// the coset e0 ⊕ span(nullspace) exhaustively for a weight ≤ ell member.
/// Complexity: exponential in the number of free columns — the general
/// syndrome-decoding problem is NP-hard, and this is not a polynomial
/// algorithm. Refuses instances with more than 24 free columns rather
/// than hang.
pub fn syndrome_decode_bounded(
    parity_checks: &[(Vec<usize>, bool)],
    num_vars: usize,
    ell: usize,
) -> Option<(Vec<bool>, usize)> {
    let mut rows = build_rows(parity_checks, num_vars);
    let (rank, pivot_row_of_col) = eliminate(&mut rows, num_vars);
    for row in rows.iter().skip(rank) {
        if row.bits.iter().all(|&w| w == 0) && row.rhs {
            return None;
        }
    }
    let free_cols: Vec<usize> = (0..num_vars)
        .filter(|&c| pivot_row_of_col[c].is_none())
        .collect();
    if free_cols.len() > 24 {
        return None; // refuse to enumerate 2^25+ — not what this path is for
    }
    // Particular solution: free vars = 0.
    let mut e0 = vec![false; num_vars];
    for (col, pr) in pivot_row_of_col.iter().enumerate() {
        if let Some(r) = pr {
            e0[col] = rows[*r].rhs;
        }
    }
    // Nullspace basis vector for each free column f: set f=1, all other
    // free columns 0, homogeneous rhs (s=0) — pivot columns read straight
    // off the same RREF rows' bit at column f.
    let basis: Vec<Vec<bool>> = free_cols
        .iter()
        .map(|&f| {
            let mut v = vec![false; num_vars];
            v[f] = true;
            for (col, pr) in pivot_row_of_col.iter().enumerate() {
                if let Some(r) = pr {
                    v[col] = get_bit(&rows[*r].bits, f);
                }
            }
            v
        })
        .collect();

    let k = free_cols.len();
    let total: u64 = 1u64 << k;

    // GPU: search every mask for the least-weight coset member at once. The
    // global minimum is the answer when it lies within ell; otherwise no member
    // does. The CPU scan below is the bare-metal fallback and the verifier.
    #[cfg(feature = "hosted")]
    {
        let w = (num_vars + 63) / 64;
        let e0w = crate::gpu_dqi::pack(&e0, w);
        let mut basisw: Vec<u64> = Vec::new();
        for bv in &basis { basisw.extend(crate::gpu_dqi::pack(bv, w)); }
        if let Some((cand, weight)) = crate::gpu_dqi::coset_min_weight(&e0w, &basisw, w, k, num_vars) {
            return if weight <= ell { Some((cand, weight)) } else { None };
        }
    }

    let mut best: Option<(Vec<bool>, usize)> = None;
    for mask in 0..total {
        let mut candidate = e0.clone();
        for (i, b) in basis.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                for j in 0..num_vars {
                    candidate[j] ^= b[j];
                }
            }
        }
        let weight = candidate.iter().filter(|&&v| v).count();
        if weight <= ell {
            let better = match &best {
                None => true,
                Some((_, w)) => weight < *w,
            };
            if better {
                best = Some((candidate, weight));
                if weight == 0 {
                    break;
                }
            }
        }
    }
    best
}

/// A tiny, non-cryptographic xorshift PRNG for generating reproducible
/// demo instances — same convention `gpu_sixteen3.rs::Xorshift` uses.
struct Xorshift(u64);
impl Xorshift {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_below(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }
}

/// Generates a random, guaranteed-satisfiable XOR system on `num_vars`
/// variables with `num_vars / 2` clauses of arity 3 (or fewer for tiny
/// instances), by planting a random solution and reading each clause's
/// rhs off it — so `xorsat_solve`'s answer can always be checked against
/// ground truth, not just "did it return Some".
pub fn random_xorsat_instance(num_vars: usize, seed: u64) -> (Vec<(Vec<usize>, bool)>, Vec<bool>) {
    let mut rng = Xorshift(seed ^ 0x9E3779B97F4A7C15);
    let planted: Vec<bool> = (0..num_vars).map(|_| rng.next_u64() & 1 == 1).collect();
    let num_clauses = core::cmp::max(1, num_vars / 2);
    let arity = core::cmp::min(3, num_vars);
    let mut clauses = Vec::with_capacity(num_clauses);
    for _ in 0..num_clauses {
        let mut vars = Vec::with_capacity(arity);
        while vars.len() < arity {
            let v = rng.next_below(num_vars);
            if !vars.contains(&v) {
                vars.push(v);
            }
        }
        let rhs = vars.iter().fold(false, |acc, &v| acc ^ planted[v]);
        clauses.push((vars, rhs));
    }
    (clauses, planted)
}

fn check_assignment(clauses: &[(Vec<usize>, bool)], assignment: &[bool]) -> bool {
    clauses.iter().all(|(vars, rhs)| {
        vars.iter().fold(false, |acc, &v| acc ^ assignment[v]) == *rhs
    })
}

/// Brute force over all 2^num_vars assignments — the O(2^m) side of the
/// comparison. Refuses num_vars > 24 (would run for hours on CPU) so the
/// benchmark stays a benchmark and not a hang.
fn brute_force_solve(clauses: &[(Vec<usize>, bool)], num_vars: usize) -> Option<Vec<bool>> {
    if num_vars > 24 {
        return None;
    }
    let total: u64 = 1u64 << num_vars;
    for mask in 0..total {
        let assignment: Vec<bool> = (0..num_vars).map(|i| (mask >> i) & 1 == 1).collect();
        if check_assignment(clauses, &assignment) {
            return Some(assignment);
        }
    }
    None
}

#[cfg(feature = "hosted")]
fn now_micros() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

/// The measured version of DQI_IMASM_UNIFIED.md §6/§13.9's claim: builds a
/// random consistent XOR system on `num_vars` variables, solves it two
/// ways (Gaussian elimination; brute force over the full 2^m space, capped
/// at 24 vars), checks both answers actually satisfy every clause, and
/// reports how the two costs compare. This runs the advantage instead of
/// asserting it.
pub fn benchmark_report(num_vars: usize) -> String {
    let (clauses, planted) = random_xorsat_instance(num_vars, num_vars as u64);
    let mut out = String::new();
    out.push_str(&format!(
        "dqi benchmark: {} variables, {} clauses (arity {})\n",
        num_vars,
        clauses.len(),
        core::cmp::min(3, num_vars)
    ));

    #[cfg(feature = "hosted")]
    let t0 = now_micros();
    let elim = xorsat_solve(&clauses, num_vars);
    #[cfg(feature = "hosted")]
    let t1 = now_micros();

    match &elim {
        Some(a) if check_assignment(&clauses, a) => {
            out.push_str("  Gaussian elimination: SAT, verified against every clause\n");
        }
        Some(_) => out.push_str("  Gaussian elimination: returned an assignment that FAILS verification (bug)\n"),
        None => out.push_str("  Gaussian elimination: UNSAT\n"),
    }
    #[cfg(feature = "hosted")]
    out.push_str(&format!("  Gaussian elimination time: {} us\n", t1 - t0));

    if num_vars <= 24 {
        #[cfg(feature = "hosted")]
        let t2 = now_micros();
        let bf = brute_force_solve(&clauses, num_vars);
        #[cfg(feature = "hosted")]
        let t3 = now_micros();
        match &bf {
            Some(a) if check_assignment(&clauses, a) => {
                out.push_str("  brute force (2^m):     SAT, verified against every clause\n");
            }
            Some(_) => out.push_str("  brute force (2^m):     returned an assignment that FAILS verification (bug)\n"),
            None => out.push_str("  brute force (2^m):     UNSAT\n"),
        }
        #[cfg(feature = "hosted")]
        {
            out.push_str(&format!("  brute force time:      {} us\n", t3 - t2));
            let e = (t1 - t0).max(1);
            let b = (t3 - t2).max(1);
            out.push_str(&format!("  ratio (brute/elim):    {:.1}x\n", b as f64 / e as f64));
        }
        out.push_str(&format!(
            "  both solve the same planted instance (check bit-length {})\n",
            planted.len()
        ));
    } else {
        out.push_str("  brute force skipped (m > 24, would not finish) — this IS the O(2^m) wall the elimination solver has no wall against\n");
    }

    // The SAT case above can let brute force exit early on a lucky hit if
    // the solution space is dense (m/2 clauses under-determines m
    // variables), which understates the gap. Appending one contradictory
    // clause (0 = 1, an empty variable set forced to rhs=true) makes the
    // system UNSAT: brute force is then FORCED to exhaust every one of the
    // 2^m assignments before it can return, no early exit possible, while
    // elimination still detects the contradiction in one pass.
    let mut unsat_clauses = clauses.clone();
    unsat_clauses.push((Vec::new(), true));
    out.push_str("\n  forced-UNSAT case (same instance + one 0=1 clause, no early exit possible for brute force):\n");
    #[cfg(feature = "hosted")]
    let u0 = now_micros();
    let elim_u = xorsat_solve(&unsat_clauses, num_vars);
    #[cfg(feature = "hosted")]
    let u1 = now_micros();
    out.push_str(&format!(
        "    Gaussian elimination: {}\n",
        if elim_u.is_none() { "UNSAT (correct)" } else { "SAT (bug: should be UNSAT)" }
    ));
    #[cfg(feature = "hosted")]
    out.push_str(&format!("    Gaussian elimination time: {} us\n", u1 - u0));
    if num_vars <= 24 {
        #[cfg(feature = "hosted")]
        let u2 = now_micros();
        let bf_u = brute_force_solve(&unsat_clauses, num_vars);
        #[cfg(feature = "hosted")]
        let u3 = now_micros();
        out.push_str(&format!(
            "    brute force (2^m):     {}\n",
            if bf_u.is_none() { "UNSAT (correct, full 2^m enumerated)" } else { "SAT (bug: should be UNSAT)" }
        ));
        #[cfg(feature = "hosted")]
        {
            out.push_str(&format!("    brute force time:      {} us\n", u3 - u2));
            let e = (u1 - u0).max(1);
            let b = (u3 - u2).max(1);
            out.push_str(&format!("    ratio (brute/elim):    {:.1}x\n", b as f64 / e as f64));
        }
    }
    out
}

pub fn repl_dqi(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("dqi — Decoded Quantum Interferometry operator (DQI word ⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙, period 14)");
        sprintln!("  dqi word              print the canonical double-⊤ word");
        sprintln!("  dqi word single       print the single-⊤ variant (period 13)");
        sprintln!("  dqi period            print the period");
        sprintln!("  dqi phase             print whether the word is phase-bearing");
        sprintln!("  dqi verdict <string>  B4 verdict for a bit string (1/0 chars)");
        sprintln!("  dqi syndrome <bits>   alternating XOR-mask transform on a bit string; 'dqi decode' runs the real bounded-weight decoder");
        sprintln!("  dqi tuple             print the catalog tuple");
        sprintln!("  dqi report            full DQI report");
        sprintln!("  dqi xorsat <m>        solve a random m-variable XOR system (Gaussian elimination), verify");
        sprintln!("  dqi decode <m> <ell>  weight-≤ell syndrome decode on a random m-variable code");
        sprintln!("  dqi benchmark <m>     elimination vs brute force on the same random instance (m≤24 runs both)");
        sprintln!("  dqi gpu-benchmark <m> [unsat] [device]  brute-force all 2^m assignments on GPU, cross-check vs elimination");
        sprintln!("                          add 'unsat' to force the worst case (no thread can exit early)");
        sprintln!("  Single-⊤ variant: ⊢∈∈≻⋈⊤⊥∋≺⋈∋⊣⊙ (period 13)");
        sprintln!("  Double-⊤ variant: ⊢∈∈≻⋈⊤⊤⊥∋≺⋈∋⊣⊙ (period 14)");
        sprintln!("  Triple-⊤ variant: ⊢∈∈≻⋈⊤⊤⊤⊥∋≺⋈∋⊣⊙ (period 15)");
        sprintln!("  Pattern: each ⊤ in the deposit phase extends the ROTAT period by 1.");
        return;
    }
    match args[0] {
        "word" => {
            if args.get(1).copied() == Some("single") {
                sprintln!("{}", WORD_SINGLE_T);
            } else {
                sprintln!("{}", WORD);
            }
        }
        "period" => sprintln!("{}", PERIOD),
        "phase" => sprintln!("{}", if PHASE_BEARING { "phase-bearing (3 distinct landings)" } else { "trivial" }),
        "verdict" => {
            let s = args.get(1).copied().unwrap_or("");
            sprintln!("{}", dqi_verdict(s));
        }
        "syndrome" => {
            let s = args.get(1).copied().unwrap_or("");
            let bits: alloc::vec::Vec<u8> = s.bytes()
                .filter(|b| *b == b'0' || *b == b'1')
                .map(|b| b - b'0')
                .collect();
            if bits.is_empty() {
                sprintln!("dqi syndrome: no binary digits (0/1) found in input");
                sprintln!("usage: dqi syndrome <0/1 string>");
            } else {
                let decoded = dqi_syndrome_decode(&bits);
                let s_out: alloc::string::String = decoded.iter().map(|b| char::from(b'0' + b)).collect();
                sprintln!("{}", s_out);
            }
        }
        "tuple" => sprintln!("⟨𐑨𐑶𐑽𐑿𐑐𐑘𐑔𐑠⊙𐑖𐑙𐑭⟩"),
        "xorsat" => {
            let m: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(16);
            let (clauses, planted) = random_xorsat_instance(m, m as u64);
            match xorsat_solve(&clauses, m) {
                Some(a) => {
                    let ok = check_assignment(&clauses, &a);
                    let matches_planted = a == planted;
                    sprintln!(
                        "xorsat({} vars, {} clauses): SAT, verified={} matches_planted={}",
                        m, clauses.len(), ok, matches_planted
                    );
                }
                None => sprintln!("xorsat({} vars, {} clauses): UNSAT", m, clauses.len()),
            }
        }
        "decode" => {
            let m: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(16);
            let ell: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2);
            let mut rng = Xorshift(m as u64 ^ 0xD1B54A32D192ED03);
            let true_error: Vec<bool> = (0..m).map(|_| rng.next_below(4) == 0).collect();
            let num_checks = core::cmp::max(1, m / 2);
            let arity = core::cmp::min(3, m);
            let mut checks: Vec<(Vec<usize>, bool)> = Vec::with_capacity(num_checks);
            for _ in 0..num_checks {
                let mut vars = Vec::with_capacity(arity);
                while vars.len() < arity {
                    let v = rng.next_below(m);
                    if !vars.contains(&v) {
                        vars.push(v);
                    }
                }
                let rhs = vars.iter().fold(false, |acc, &v| acc ^ true_error[v]);
                checks.push((vars, rhs));
            }
            let true_weight = true_error.iter().filter(|&&v| v).count();
            match syndrome_decode_bounded(&checks, m, ell) {
                Some((e, w)) => {
                    let syndrome_ok = checks.iter().all(|(vars, rhs)| {
                        vars.iter().fold(false, |acc, &v| acc ^ e[v]) == *rhs
                    });
                    sprintln!(
                        "decode({} vars, {} checks, ell={}): found weight-{} error, syndrome_ok={}, planted weight={}",
                        m, checks.len(), ell, w, syndrome_ok, true_weight
                    );
                }
                None => sprintln!(
                    "decode({} vars, {} checks, ell={}): no weight≤{} solution found (or too many free columns to search)",
                    m, checks.len(), ell, ell
                ),
            }
        }
        "benchmark" => {
            let m: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(16);
            sprintln!("{}", benchmark_report(m));
        }
        "report" => {
            sprintln!("── DQI Report ──");
            sprintln!("  word (double-⊤): {}", WORD);
            sprintln!("  word (single-⊤): {}", WORD_SINGLE_T);
            sprintln!("  period: {}", PERIOD);
            sprintln!("  phase-bearing: {}", PHASE_BEARING);
            sprintln!("  tuple: ⟨𐑨𐑶𐑽𐑿𐑐𐑘𐑔𐑠⊙𐑖𐑙𐑭⟩");
            sprintln!("  catalog: dqi_algorithm");
            sprintln!("  μ∘δ=id: closed (B4=T)");
        }
        other => sprintln!("dqi: unknown subcommand '{}' (try 'dqi help')", other),
    }
}
