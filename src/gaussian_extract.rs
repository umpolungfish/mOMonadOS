//! gaussian_extract.rs — "prime extraction via imaginary numbers," built as a
//! real module from the ob3ect of that name rather than only read.
//!
//! The word's marks name a real, classical piece of number theory, not a
//! decoration: a prime p ≡ 1 (mod 4) factors in the Gaussian integers as
//! p = (a+bi)(a-bi), i.e. p = a² + b² (Fermat's two-square theorem). ⊢
//! (Gaussian integer seed) is p itself; ≻ (complex rotation) is adjoining i
//! by finding a real square root of -1 mod p; ∈ (real/imaginary
//! bifurcation) and ≺ (conjugate descent) together are Cornacchia's
//! algorithm — a genuine Euclidean-style descent, structurally the same
//! repeated-reduction shape as gcd, that runs on the root of -1 until it
//! bottoms out at a and b; ∋/⊡ (winding fusion / immutable record) is
//! checking and recording a² + b² = p exactly. ⊤/⊥ (prime confirmation /
//! composite rejection) is not decorative either: a prime p ≡ 3 (mod 4)
//! has NO two-square decomposition at all — that is Fermat's theorem's own
//! other half, not a limitation of this code, so this module reports that
//! case as a real, named rejection rather than a failure.
//!
//! What this is not: p ≡ 1 (mod 4) is necessary for p prime, but composite
//! N ≡ 1 (mod 4) can also decompose as a sum of two squares (any N whose
//! prime factors are all ≡ 1 mod 4, or appear to an even power), so a
//! successful decomposition alone does not prove primality — checked
//! directly below by cross-referencing the real Miller-Rabin test already
//! built in native_numeral.rs, not asserted.

extern crate alloc;
use alloc::format;
use alloc::string::String;
use num_bigint::BigUint;
use num_traits::One;

use crate::native_numeral::is_prime_miller_rabin;

/// Find r with r² ≡ -1 (mod p), for prime p ≡ 1 (mod 4). Standard method:
/// for a random a, b = a^((p-1)/4) mod p satisfies b² ≡ ±1 (mod p); about
/// half of all a give -1, so a handful of trials succeeds with overwhelming
/// probability. None if p is not ≡ 1 (mod 4) — -1 is never a quadratic
/// residue there, so no such r exists, not just none found.
pub fn sqrt_neg1_mod_p(p: &BigUint) -> Option<BigUint> {
    use crate::native_numeral::{
        modulo_small_via_word, modulo_via_word, mod_pow_walk, multiply_via_word,
        subtract_via_word, to_bits_low_first,
    };
    if modulo_small_via_word(p, 4).unwrap() != 1 {
        return None;
    }
    let p_minus_1 = subtract_via_word(p, &BigUint::one()).unwrap();
    let exp = divmod_small_by_four(&p_minus_1);
    let mut seed: u64 = 2;
    for _ in 0..64 {
        let a = modulo_via_word(&BigUint::from(seed), p).unwrap();
        if a < BigUint::from(2u32) {
            seed += 1;
            continue;
        }
        let b = mod_pow_walk(&a, &to_bits_low_first(&exp), p);
        let b_sq = modulo_via_word(&multiply_via_word(&b, &b), p).unwrap();
        if b_sq == p_minus_1 {
            return Some(b);
        }
        seed += 1;
    }
    None
}

/// (p-1)/4, exact since p ≡ 1 (mod 4) is already checked by the caller.
fn divmod_small_by_four(v: &BigUint) -> BigUint {
    crate::native_numeral::divmod_small_via_word(v, 4).unwrap().0
}

/// Cornacchia's algorithm: given r with r² ≡ -1 (mod n), descend r and n by
/// repeated remainder (the same shape as gcd — a real Euclidean descent,
/// not a metaphor for one) until the running value drops to or below
/// sqrt(n), then the remaining pair is (a, b) with a² + b² = n, checked
/// directly by the caller rather than assumed here.
pub fn cornacchia(r: &BigUint, n: &BigUint) -> Option<(BigUint, BigUint)> {
    use crate::native_numeral::{isqrt, modulo_via_word, multiply_via_word, subtract_via_word};
    let (mut a, mut b) = (n.clone(), r.clone());
    while multiply_via_word(&b, &b) > *n {
        let t = modulo_via_word(&a, &b).unwrap();
        a = b;
        b = t;
    }
    // b has descended to <= sqrt(n); the other square is whatever is left
    // of n once b's square is removed, checked as an exact square directly
    // rather than assumed from the descent alone.
    let b_sq = multiply_via_word(&b, &b);
    let c = subtract_via_word(n, &b_sq).unwrap();
    let d = isqrt(&c);
    if multiply_via_word(&d, &d) == c {
        Some((b, d))
    } else {
        None
    }
}

/// The full real report: seed, rotate (find i), descend (Cornacchia),
/// fuse and record (check a²+b²=N exactly), confirm or reject against the
/// real primality test — every claim checked against the actual numbers,
/// not asserted from the theorem alone.
pub fn extract_report(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("gaussian_extract: '{}' is not a valid non-negative integer", n_str),
    };
    if n < BigUint::from(2u32) {
        return format!("gaussian_extract {}: N < 2, nothing to extract", n_str);
    }
    let n_mod_4 = crate::native_numeral::modulo_small_via_word(&n, 4).unwrap();
    let (is_prime, _) = is_prime_miller_rabin(&n);

    if n_mod_4 != 1 {
        // ⊥ composite_rejection / the real other half of Fermat's theorem:
        // a prime ≡ 3 (mod 4) provably has no two-square decomposition at
        // all, checked against the real primality test, not asserted.
        return format!(
            "N = {}\nN mod 4 = {} — not 1, so no square root of -1 mod N exists and no two-square decomposition can.\nMiller-Rabin: N is {}.\nverdict: ⊥ composite_rejection (definitional when prime, N mod 4 != 1) — real primality is independent of this test either way.\n",
            n, n_mod_4, if is_prime { "prime" } else { "composite" }
        );
    }

    let r = match sqrt_neg1_mod_p(&n) {
        Some(v) => v,
        None => {
            return if is_prime {
                format!(
                    "N = {}\nN mod 4 = 1, N is prime, but no square root of -1 mod N was found in 64 trials.\nMiller-Rabin: N is prime.\nverdict: ⊞ paradice — a root exists by Euler's criterion (about half of all residues witness it directly), the trial budget just did not land on one; genuinely a budget question here, raising it will find one.\n",
                    n
                )
            } else {
                format!(
                    "N = {}\nN mod 4 = 1, N is composite, no square root of -1 mod N was found in 64 trials.\nMiller-Rabin: N is composite.\nverdict: ⊞ paradice — not a budget shortfall: computing a square root of -1 mod a composite N without already knowing N's prime factorization is exactly as hard as factoring N itself (Rabin's reduction, modular square-root extraction mod a composite is polynomial-time equivalent to factoring it). Raising the trial count will not reach a root that this method is not positioned to find; the factorization would have to come from elsewhere first.\n",
                    n
                )
            };
        }
    };

    match cornacchia(&r, &n) {
        Some((a, b)) => {
            use crate::native_numeral::{add_via_word, multiply_via_word};
            let check = add_via_word(&multiply_via_word(&a, &a), &multiply_via_word(&b, &b));
            format!(
                "N = {}\n≻ i found: {}² ≡ -1 (mod N)\n≺ Cornacchia descent: a = {}, b = {}\n∋ a² + b² = {}  (checked = N: {})\nMiller-Rabin: N is {}.\nverdict: {}\n",
                n, r, a, b, check, check == n, if is_prime { "prime" } else { "composite" },
                if is_prime { "⊤ prime_confirmation — a real Gaussian factorization N = (a+bi)(a-bi)" }
                else { "N decomposes as a sum of two squares but Miller-Rabin says composite — a successful decomposition alone never proved primality, checked here rather than assumed" }
            )
        }
        None => format!(
            "N = {}\n≻ i found: {}² ≡ -1 (mod N)\n≺ Cornacchia descent did not bottom out on an exact a² + b² = N.\nMiller-Rabin: N is {}.\nverdict: ⊞ paradice — N mod 4 = 1 and -1 has a root, but the descent itself did not close; genuinely open, not a rejection.\n",
            n, r, if is_prime { "prime" } else { "composite" }
        ),
    }
}
