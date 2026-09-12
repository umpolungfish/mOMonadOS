//! gpu_factor.rs — factoring on the GPU by parallel trial division, arbitrary
//! precision in N.
//!
//! N is sent to the device as its 32-bit limbs. Each thread tests one odd
//! candidate divisor d and computes N mod d by Horner folding over the limbs,
//! so N has no size limit; the smallest hit in a window is taken by an atomic
//! minimum. The host peels factor 2, walks windows upward for the least prime
//! factor, divides, and repeats. Candidate divisors run up to 2^32, the
//! practical reach of trial division; a cofactor with no factor there is
//! reported with its primality (the winding is_prime), and one whose least
//! factor is above the reach is named as Pollard-rho work.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;
use cudarc::driver::{CudaContext, CudaStream, CudaFunction, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use num_bigint::BigUint;
use num_traits::{Zero, One};

/// Trial division cannot go past here in reasonable time; the Horner fold also
/// needs d below 2^32 so the running remainder stays inside 64 bits.
const DIVISOR_REACH: u64 = 1 << 24;   // cheap small-factor sweep; ECM/rho take the rest

const KERNEL_SRC: &str = r#"
typedef unsigned long long u64;
// N as little-endian 32-bit limbs; each thread tests d = base + 2*i.
extern "C" __global__ void smallest_divisor_big(
    const unsigned int* limbs, unsigned int nlimbs,
    u64 base, u64 count, unsigned long long* result)
{
    u64 i = (u64)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= count) return;
    u64 d = base + 2ULL * i;
    if (d < 3) return;
    u64 r = 0;
    for (int k = (int)nlimbs - 1; k >= 0; k--) {
        r = ((r << 32) + (u64)limbs[k]) % d;   // d < 2^32, so r < 2^32, no overflow
    }
    if (r == 0) atomicMin(result, d);
}
"#;

struct Gpu { stream: Arc<CudaStream>, func: CudaFunction }

fn setup(device: usize) -> Result<Gpu, String> {
    let ctx = CudaContext::new(device).map_err(|e| format!("gpu_factor: no CUDA context: {e}"))?;
    let stream = ctx.default_stream();
    let ptx = compile_ptx(KERNEL_SRC).map_err(|e| format!("gpu_factor: NVRTC: {e}"))?;
    let module = ctx.load_module(ptx).map_err(|e| format!("gpu_factor: module: {e}"))?;
    let func = module.load_function("smallest_divisor_big").map_err(|e| format!("gpu_factor: load: {e}"))?;
    Ok(Gpu { stream, func })
}

/// Integer square root of a BigUint, capped at DIVISOR_REACH so the divisor
/// search stays inside the feasible, no-overflow range.
fn search_bound(n: &BigUint) -> u64 {
    // sqrt(n) via Newton on BigUint, then cap.
    use crate::native_numeral::{add_via_word, divmod_small_via_word, divmod_via_word};
    if n < &BigUint::from(4u32) { return 2; }
    let mut x = n.clone();
    let mut y = divmod_small_via_word(&add_via_word(&x, &BigUint::from(1u32)), 2).unwrap().0;
    while y < x {
        x = y.clone();
        let q = divmod_via_word(n, &x).unwrap().0;
        y = divmod_small_via_word(&add_via_word(&x, &q), 2).unwrap().0;
    }
    // x = floor(sqrt(n))
    let cap = BigUint::from(DIVISOR_REACH);
    let b = if x > cap { cap } else { x };
    // to u64
    b.iter_u64_digits().next().unwrap_or(0)
}

/// Smallest odd divisor of n in [3, bound], or None if none up to the bound.
fn smallest_odd_divisor(g: &Gpu, n: &BigUint, bound: u64) -> Result<Option<u64>, String> {
    if bound < 3 { return Ok(None); }
    let limbs: Vec<u32> = { let mut v = n.to_u32_digits(); if v.is_empty() { v.push(0); } v };
    let nlimbs = limbs.len() as u32;
    let d_limbs = g.stream.clone_htod(&limbs).map_err(|e| format!("gpu_factor: htod limbs: {e}"))?;
    const WIN: u64 = 1 << 22;
    let mut base: u64 = 3;
    while base <= bound {
        let span = (bound - base) / 2 + 1;
        let count = if span > WIN { WIN } else { span };
        let init: Vec<u64> = alloc::vec![u64::MAX];
        let mut d_res = g.stream.clone_htod(&init).map_err(|e| format!("gpu_factor: htod res: {e}"))?;
        let cfg = LaunchConfig::for_num_elems(count as u32);
        let mut b = g.stream.launch_builder(&g.func);
        b.arg(&d_limbs); b.arg(&nlimbs); b.arg(&base); b.arg(&count); b.arg(&mut d_res);
        unsafe { b.launch(cfg) }.map_err(|e| format!("gpu_factor: launch: {e}"))?;
        let res = g.stream.clone_dtoh(&d_res).map_err(|e| format!("gpu_factor: dtoh: {e}"))?;
        if res[0] != u64::MAX { return Ok(Some(res[0])); }
        base += 2 * count;
    }
    Ok(None)
}

fn is_prime_big(n: &BigUint) -> bool {
    matches!(crate::prime_winding::is_prime(&n.to_string()),
             crate::prime_winding::PrimeVerdict::Prime)
}

fn abs_diff(a: &BigUint, b: &BigUint) -> BigUint {
    use crate::native_numeral::subtract_via_word;
    if a >= b { subtract_via_word(a, b).unwrap() } else { subtract_via_word(b, a).unwrap() }
}

/// Pollard-Brent rho: a nontrivial factor of a composite odd n, arbitrary
/// precision, bounded so it always returns. `None` means no factor was found
/// within the step budget (the smallest factor is large enough to need fast
/// Montgomery rho or ECM, not this plain-bignum walk).
fn brent(n: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{add_via_word, modulo_via_word, multiply_via_word, modulo_small_via_word};
    let one = BigUint::one();
    let two = BigUint::from(2u32);
    if modulo_small_via_word(n, 2).unwrap() == 0 { return Some(two); }
    let budget: u64 = 8_000_000;   // total inner iterations across all constants
    let mut spent: u64 = 0;
    let mut c = BigUint::one();
    let step = |v: &BigUint, c: &BigUint| -> BigUint {
        modulo_via_word(&add_via_word(&multiply_via_word(v, v), c), n).unwrap()
    };
    while spent < budget {
        let mut y = two.clone();
        let mut r: u64 = 1;
        let mut q = BigUint::one();
        let mut g = BigUint::one();
        let mut x = y.clone();
        let mut ys = y.clone();
        while g == one && spent < budget {
            x = y.clone();
            for _ in 0..r { y = step(&y, &c); }
            let mut k: u64 = 0;
            while k < r && g == one {
                ys = y.clone();
                let m = core::cmp::min(128u64, r - k);
                for _ in 0..m {
                    y = step(&y, &c);
                    q = modulo_via_word(&multiply_via_word(&q, &abs_diff(&x, &y)), n).unwrap();
                }
                spent += m;
                g = crate::prime_winding::big_gcd(q.clone(), n.clone());
                k += m;
            }
            r *= 2;
        }
        if g == *n {
            loop {
                ys = step(&ys, &c);
                g = crate::prime_winding::big_gcd(abs_diff(&x, &ys), n.clone());
                spent += 1;
                if g > one || spent >= budget { break; }
            }
        }
        if g != *n && g > one { return Some(g); }
        c = add_via_word(&c, &BigUint::from(1u32));
    }
    None
}

/// Fully factor n into primes: GPU trial division first (small factors), then
/// Pollard-Brent rho on any cofactor whose factors exceed the trial reach.
fn split_full(g: &Gpu, n: BigUint, out: &mut Vec<BigUint>) -> Result<(), String> {
    if n <= BigUint::one() { return Ok(()); }
    if is_prime_big(&n) { out.push(n); return Ok(()); }
    let bound = search_bound(&n);
    if let Some(d) = smallest_odd_divisor(g, &n, bound)? {
        // d is the least divisor, hence prime
        out.push(BigUint::from(d));
        return split_full(g, crate::native_numeral::divmod_via_word(&n, &BigUint::from(d)).unwrap().0, out);
    }
    // No small factor: ECM (any width) finds medium and large factors, at
    // escalating smoothness bounds; rho is a fast fallback for the small sizes.
    for b1 in [50_000u64, 250_000] {
        if let Some(f) = crate::gpu_ecm::factor_once(&n, b1, 0) {
            let cof = crate::native_numeral::divmod_via_word(&n, &f).unwrap().0;
            split_full(g, f, out)?;
            return split_full(g, cof, out);
        }
    }
    // Deep stage 2: per-block BSGS covers a wide B2 per curve, reaching factors
    // the per-thread walk above misses at the same B1.
    for b1 in [250_000u64, 1_000_000] {
        if let Some(f) = crate::gpu_ecm::factor_once_bsgs(&n, b1, b1 * 100) {
            let cof = crate::native_numeral::divmod_via_word(&n, &f).unwrap().0;
            split_full(g, f, out)?;
            return split_full(g, cof, out);
        }
    }
    if n.bits() <= 256 {
        if let Some(f) = crate::gpu_rho::factor_once(&n, 0) {
            let cof = crate::native_numeral::divmod_via_word(&n, &f).unwrap().0;
            split_full(g, f, out)?;
            return split_full(g, cof, out);
        }
    }
    // Fall back to the bounded plain-bignum rho.
    match brent(&n) {
        Some(f) => {
            let cof = crate::native_numeral::divmod_via_word(&n, &f).unwrap().0;
            split_full(g, f, out)?;
            split_full(g, cof, out)
        }
        None => {
            // Beyond the plain-bignum rho budget; leave it whole and flagged.
            out.push(n);
            Ok(())
        }
    }
}

/// True when a reported factor is composite (the rho budget ran out on it).
fn is_composite(n: &BigUint) -> bool {
    *n > BigUint::one() && !is_prime_big(n)
}

pub fn run(n_str: &str, device: usize) -> String {
    let n_str = n_str.trim();
    let n0: BigUint = match n_str.parse() {
        Ok(v) => v,
        Err(_) => return format!("gpu_factor: '{}' is not a non-negative integer", n_str),
    };
    if n0 < BigUint::from(2u32) { return format!("gpu_factor {}: nothing to factor", n0); }

    let g = match setup(device) { Ok(g) => g, Err(e) => return e };

    let mut n = n0.clone();
    let two = BigUint::from(2u32);
    let mut factors: Vec<BigUint> = Vec::new();
    while (&n % &two).is_zero() { factors.push(two.clone()); n /= &two; }
    if let Err(e) = split_full(&g, n, &mut factors) { return e; }
    factors.sort();

    let mut prod = BigUint::one();
    let mut s = String::new();
    let mut unfactored: Option<BigUint> = None;
    for (i, f) in factors.iter().enumerate() {
        if i > 0 { s.push_str(" x "); }
        s.push_str(&f.to_string());
        if is_composite(f) { s.push_str("[composite]"); unfactored = Some(f.clone()); }
        prod *= f;
    }
    let note = match unfactored {
        Some(c) => format!("\n  {} is composite; its factors are past trial, rho, and ECM to B1=250000 here (a larger B1 or GNFS reaches them)", c),
        None => String::new(),
    };
    format!("gpu_factor {} = {}  (product == n: {}){}", n0, s, prod == n0, note)
}
