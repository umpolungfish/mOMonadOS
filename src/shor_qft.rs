//! shor_qft.rs — Shor's period-finding, run on a real complex-amplitude
//! statevector, not asserted from a formula.
//!
//! Context for why this file exists: `belnap_shor.rs`'s header claims
//! "period r is encoded in the 2:1 coherence cost ratio." Traced by hand
//! and checked against its own arithmetic: b_meas_only=2n, t_meas_only=n
//! for any n, so ratio=2n/n=2.0 always -- a constant, independent of a,
//! N, or r. The period in that file is computed by a separate classical
//! brute-force walk and reported alongside the (unrelated) ratio. That
//! is not quantum advantage, algebraic or otherwise; it is a classical
//! computation with a decorative label. `qft.rs` builds the QFT as a
//! gate-count structure with no amplitudes. `period_finding_ecdlp.rs`
//! takes the measured phase as a GIVEN input and does the (real,
//! correct) classical continued-fraction half of the algorithm on it,
//! but never derives that phase from a simulated quantum state.
//!
//! This file does the missing half: a real complex-amplitude simulation
//! of Shor's period-finding circuit (uniform superposition, modular
//! exponentiation as a basis-state permutation, a genuine measurement
//! collapse, the QFT as the exact unitary DFT on what's left), producing
//! an actual probability distribution whose peaks land at multiples of
//! 2^n/r -- checked, not asserted, by printing the distribution and
//! reading off the peaks -- then the real continued-fraction extraction
//! and gcd-based factoring on top of it.
//!
//! Why this is the genuine article, unlike DQI and unlike the
//! structureless YZ separation: the interference that produces those
//! peaks depends on the actual multiplicative/cyclic-group structure of
//! (Z/NZ)* -- exactly the ALGEBRAIC structure DQI's speedup does not need,
//! and YZ's separation is titled to prove it doesn't need either. Simulating it here costs O(M^2) in the register size M=2^n --
//! exponential in the qubit count -- which is exactly why a real quantum
//! device would have a genuine advantage at a scale this simulation
//! cannot reach; the small scale here is what makes the simulation
//! checkable at all, not a limitation being hidden.

#![allow(dead_code)]

use crate::fibonacci_qc::Complex;
use crate::sprintln;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

fn mod_pow_u64(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus <= 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 != 0 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}

fn gcd_u64(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// The true period of a mod n_val, by direct classical walk -- used only
/// as the ground truth to check the simulation's extracted period
/// against, never as part of the extraction itself.
fn true_period(a: u64, n_val: u64) -> u64 {
    let mut val = 1u64;
    for r in 1..=n_val {
        val = ((val as u128 * a as u128) % n_val as u128) as u64;
        if val == 1 {
            return r;
        }
    }
    0
}

/// The QFT, applied as the exact unitary DFT matrix on the register:
/// out[k] = (1/sqrt(M)) * sum_x state[x] * exp(2*pi*i*k*x/M).
/// This is not an approximation of what the QFT gate sequence does --
/// it is the same unitary, computed directly rather than via H/CR gates,
/// exact for the register sizes this module uses.
fn qft_forward(state: &[Complex]) -> Vec<Complex> {
    let m = state.len();
    let scale = 1.0 / libm::sqrt(m as f64);
    let mut out = alloc::vec![Complex::zero(); m];
    for k in 0..m {
        let mut acc = Complex::zero();
        for (x, &amp) in state.iter().enumerate() {
            if amp.re == 0.0 && amp.im == 0.0 {
                continue;
            }
            let angle = 2.0 * core::f64::consts::PI * (k as f64) * (x as f64) / (m as f64);
            acc = acc + amp * Complex::new(libm::cos(angle), libm::sin(angle));
        }
        out[k] = acc.scale(scale);
    }
    out
}

/// Continued-fraction expansion of k/m, returning every convergent
/// (numerator, denominator) -- the standard classical post-processing
/// step of Shor's algorithm, run on the frequency actually measured from
/// the simulated distribution below, not on a hypothetical.
fn convergents(mut k: u64, mut m: u64) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    // p_{-2}=0, p_{-1}=1, q_{-2}=1, q_{-1}=0 -- the standard convergent
    // recurrence's seed values. Checked directly against k=64, m=256
    // (period 4 case): with this seed the first two convergents come out
    // (0,1) then (1,4), matching k/m=1/4 exactly. The previous version
    // had p and q each seeded with their own two values swapped, which
    // silently produced every convergent's reciprocal instead.
    let (mut p_prev, mut p_curr) = (0u64, 1u64);
    let (mut q_prev, mut q_curr) = (1u64, 0u64);
    while m != 0 {
        let a = k / m;
        let p_next = a.wrapping_mul(p_curr).wrapping_add(p_prev);
        let q_next = a.wrapping_mul(q_curr).wrapping_add(q_prev);
        out.push((p_next, q_next));
        p_prev = p_curr;
        p_curr = p_next;
        q_prev = q_curr;
        q_curr = q_next;
        let rem = k % m;
        k = m;
        m = rem;
    }
    out
}

pub struct ShorSimResult {
    pub a: u64,
    pub n_val: u64,
    pub n_qubits: usize,
    pub register_size: usize,
    pub true_period: u64,
    pub top_peaks: Vec<(usize, f64)>,
    pub extracted_period: Option<u64>,
    pub factors: Option<(u64, u64)>,
}

/// The full pipeline: build the real post-measurement index-register
/// state (a periodic amplitude comb of period r, r = true_period(a,
/// n_val), collapsed onto the branch where the output register reads
/// f(0)=1 -- one genuine, equally-likely branch among r), run it through
/// the exact QFT, read the resulting probability distribution, extract
/// the strongest peaks, run continued fractions on the best one, and
/// attempt to factor n_val via gcd on the recovered period. `n_qubits`
/// is capped at 14 (register size 16384, O(M^2) direct DFT already
/// costs ~2.7*10^8 complex multiplies there) so this stays a
/// demonstration and not a multi-minute wait.
pub fn simulate_shor(a: u64, n_val: u64, n_qubits: usize) -> Result<ShorSimResult, String> {
    if n_qubits == 0 || n_qubits > 14 {
        return Err(format!(
            "refusing n_qubits={} (supported: 1..=14 -- direct O(M^2) DFT)",
            n_qubits
        ));
    }
    if n_val < 2 {
        return Err(format!("n_val={} is not a valid modulus (need ≥ 2)", n_val));
    }
    if gcd_u64(a, n_val) != 1 {
        return Err(format!(
            "gcd(a={}, N={}) = {} ≠ 1 -- a must be coprime to N for period-finding to apply",
            a, n_val, gcd_u64(a, n_val)
        ));
    }
    let m = 1usize << n_qubits;
    let r_true = true_period(a, n_val);

    // Step 1-2 (uniform superposition) + step 3 (ModExp as a permutation)
    // + step 4 (measure the output register, branch f(x)=1): computed
    // directly rather than gate-by-gate, since a full H-layer into a
    // permutation into a projective measurement onto one output value
    // has one exact closed form -- equal amplitude on every x with
    // a^x mod n_val landing on the observed value, zero elsewhere. This
    // is the real post-measurement state, not an approximation of it.
    let f_vals: Vec<u64> = (0..m).map(|x| mod_pow_u64(a, x as u64, n_val)).collect();
    let observed = f_vals[0]; // x=0 always maps to 1 -- a real, always-available branch
    let matching: Vec<usize> = (0..m).filter(|&x| f_vals[x] == observed).collect();
    let amp = 1.0 / libm::sqrt(matching.len() as f64);
    let mut state = alloc::vec![Complex::zero(); m];
    for &x in &matching {
        state[x] = Complex::new(amp, 0.0);
    }

    // Step 5: the QFT, exact.
    let spectrum = qft_forward(&state);
    let probs: Vec<f64> = spectrum.iter().map(|c| c.norm_sq()).collect();

    // Step 6: read off the strongest peaks -- what an actual measurement
    // would sample from, weighted by these same probabilities.
    let mut indexed: Vec<(usize, f64)> = probs.iter().cloned().enumerate().collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top_peaks: Vec<(usize, f64)> = indexed.into_iter().take(8).collect();

    // Step 7: continued fractions, tried against each top peak in turn,
    // not just the single strongest one. k=0 is always a peak (every
    // periodic comb has a zero-frequency component) and is always
    // uninformative -- its convergent is 0/1, certifying nothing -- so
    // whenever probabilities tie exactly (a real, common outcome when M
    // is a multiple of r, seen directly in this simulation's own
    // output), taking only the first-sorted peak can hand the extractor
    // the one peak with no period information in it. A real measurement
    // would simply land on a different sample if k=0 came up; trying
    // every peak a real measurement could have landed on matches that,
    // rather than stopping silently at the first.
    let mut extracted_period = None;
    for &(k, _) in &top_peaks {
        if k == 0 {
            continue;
        }
        let mut found = None;
        for (_, q) in convergents(k as u64, m as u64) {
            if q > 0 && q < n_val && mod_pow_u64(a, q, n_val) == 1 {
                found = Some(q);
                break;
            }
        }
        if found.is_some() {
            extracted_period = found;
            break;
        }
    }

    // Step 8: factor n_val via gcd, the standard closing step, only if
    // the extracted period is even and not a trivial square root of 1.
    let mut factors = None;
    if let Some(r) = extracted_period {
        if r % 2 == 0 {
            let half = mod_pow_u64(a, r / 2, n_val);
            if half != n_val - 1 {
                let f1 = gcd_u64(if half >= 1 { half - 1 } else { n_val - 1 }, n_val);
                let f2 = gcd_u64(half + 1, n_val);
                if f1 > 1 && f1 < n_val {
                    factors = Some((f1, n_val / f1));
                } else if f2 > 1 && f2 < n_val {
                    factors = Some((f2, n_val / f2));
                }
            }
        }
    }

    Ok(ShorSimResult {
        a,
        n_val,
        n_qubits,
        register_size: m,
        true_period: r_true,
        top_peaks,
        extracted_period,
        factors,
    })
}

pub fn report(result: &ShorSimResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "shor_qft: a={}, N={}, {} index qubits (register size {})\n",
        result.a, result.n_val, result.n_qubits, result.register_size
    ));
    out.push_str(&format!("  true period (classical ground truth): {}\n", result.true_period));
    out.push_str("  strongest peaks in the measured distribution (index, probability):\n");
    for &(k, p) in &result.top_peaks {
        let ratio = k as f64 * result.true_period as f64 / result.register_size as f64;
        out.push_str(&format!(
            "    k={:<6} p={:.5}  k/M*r={:.3} (should land near an integer if peaks are where theory predicts)\n",
            k, p, ratio
        ));
    }
    match result.extracted_period {
        Some(r) => {
            out.push_str(&format!(
                "  period extracted via continued fractions from a measured peak: {}  (matches true period: {})\n",
                r, r == result.true_period
            ));
        }
        None => out.push_str("  continued fractions did not certify a period from any of the top peaks\n"),
    }
    match result.factors {
        Some((f1, f2)) => {
            out.push_str(&format!(
                "  factors recovered via gcd: {} * {} = {}  (verified: {})\n",
                f1, f2, f1 * f2, f1 * f2 == result.n_val
            ));
        }
        None => out.push_str("  no factors recovered this run (even/odd or gcd degeneracy -- try a different a)\n"),
    }
    out
}

pub fn repl_shor_qft(args: &[&str]) {
    if args.is_empty() || args[0] == "help" {
        sprintln!("shor-qft — Shor's period-finding on a real complex-amplitude statevector simulation");
        sprintln!("  shor-qft run <a> <N> <qubits>   full pipeline: superposition, ModExp, measure, QFT, continued fractions, factor");
        sprintln!("  qubits capped at 14 (register size 16384) -- O(M^2) direct DFT, stays a demo not a wait");
        sprintln!("  classic worked examples: a=7 N=15, a=2 N=21, a=2 N=35, a=8 N=21");
        return;
    }
    match args[0] {
        "run" => {
            let a: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(7);
            let n_val: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(15);
            let qubits: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(8);
            match simulate_shor(a, n_val, qubits) {
                Ok(result) => sprintln!("{}", report(&result)),
                Err(e) => sprintln!("shor-qft: {}", e),
            }
        }
        other => sprintln!("shor-qft: unknown subcommand '{}' (try 'shor-qft help')", other),
    }
}
