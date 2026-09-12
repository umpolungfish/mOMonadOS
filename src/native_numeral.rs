//! Native IMASM-Numeral Mapping — ob3ect-backed kernel tool.
//!
//! The artifact "Native IMASM-Numeral Mapping" is a verified ob3ect whose
//! glyph word `⊢⊣≻⋈∈⊤⊥⊞∋⊙≺⊡⋈⊣` maps a number into the Grammar through its
//! parity grade. Its boundary is the Z2 topological invariant, parity, the
//! one bit that cannot deform to zero without crossing a phase boundary. The
//! marks are numeral roles:
//!
//!   ⊢ VINIT   void_numeral          the uninitialized value
//!   ⊣ TANCH   topological_boundary  the closed numeral system
//!   ≻ AFWD    increment_magnitude   one step up in magnitude
//!   ⋈ CLINK   digit_composition     chain a digit onto the numeral
//!   ∈ FSPLIT  parity_branch         split on parity, even arm and odd arm
//!   ⊤ EVALT   even_parity           the digit is even (0)
//!   ⊥ EVALF   odd_parity            the digit is odd (1)
//!   ⊞ ENGAGR  chiral_superposition  both parities held
//!   ∋ FFUSE   stoichiometric_rejoin the arms rejoin
//!   ⊙ IMSCRIB critical_state        the value recognizes itself
//!   ≺ AREV    decrement_magnitude   one step down
//!   ⊡ IFIX    immutable_record      the Z2 fixation, the value made permanent
//!
//! Why this and not the hex-nibble encoding. prime_winding maps a number by
//! looking each hex digit up in a fixed table, and its own header records that
//! this reads only a bounded local pattern in the digits: it went T the moment
//! any nibble was 8 or higher, tracking no global fact. This mapping is built
//! from the Z2 parity grade instead: a number is encoded bit by bit, each bit
//! its own parity branch, even on the ⊤ arm and odd on the ⊥ arm, so the whole
//! binary expansion is carried as a chain of closed parity branches. The
//! encoding is exact and reversible, and distinct numbers give distinct words.
//!
//! The raw blueprint word clears its parity superposition at the ≺ decrement
//! with nothing banked, which the ob3ect's own banked check flags. The
//! per-number encoding here is built to hold instead: each bit's branch opens,
//! works, and fuses before the next, so no count is left in the open.
//!
//! Subcommands:
//!   native_numeral word         the canonical glyph word and its marks
//!   native_numeral encode <n>   map n to its native parity-graded word
//!   native_numeral help         list subcommands

use alloc::string::String;
use alloc::format;
use alloc::vec::Vec;
use num_bigint::{BigUint, BigInt};
use num_traits::Zero;
use imasm_core::lattice_flow::{tri_ancestral_word_verdict, cycle_landings};

/// The ob3ect's own fixed reference word.
pub const WORD: &str = "⊢⊣≻⋈∈⊤⊥⊞∋⊙≺⊡⋈⊣";
pub const PERIOD: usize = 14;
pub const ARTIFACT: &str = "Native IMASM-Numeral Mapping";

/// Map n to its native parity-graded IMASM word. Bits are read low to high;
/// each bit is one closed parity branch, ∈ then its parity arm then ∋, so the
/// branch works and fuses before the next bit opens, and nothing is left in
/// the open for a later clear. The magnitude step ≻ precedes each branch, the
/// value is recognized at ⊙ and fixed at ⊡, and the whole is bounded by ⊢ and
/// ⊣. n = 0 is the void numeral, ⊢⊙⊡⊣, a fixed empty magnitude.
pub fn encode(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    encode_biguint(&n)
}

/// The same word encode produces, taking the BigUint directly rather than
/// its decimal string. encode itself already does its real work this way,
/// parsing once and calling straight into to_bits_low_first -- this just
/// gives that path its own name so a caller who already has the BigUint
/// (every word-native operation in this file does) never has to pay for
/// converting it to decimal and re-parsing it just to read its own bits.
fn encode_biguint(n: &BigUint) -> String {
    let mut w = String::new();
    w.push('⊢');
    if n.is_zero() {
        w.push('⊙');
        w.push('⊡');
        w.push('⊣');
        return w;
    }
    let bits = to_bits_low_first(n);
    for b in bits.iter() {
        w.push('≻');            // increment_magnitude: advance one position
        w.push('⋈');            // digit_composition: chain this digit on
        w.push('∈');            // parity_branch
        w.push(if *b { '⊥' } else { '⊤' }); // odd → ⊥, even → ⊤
        w.push('∋');            // stoichiometric_rejoin
    }
    w.push('⊙');                // critical_state: recognize the value
    w.push('⊡');                // immutable_record: the Z2 fixation
    w.push('⊣');                // topological_boundary
    w
}

/// The exact inverse of `encode`: reads a native-numeral word back to its
/// integer. None for anything that isn't a well-formed encode() output —
/// callers that also accept plain decimal strings should try that first and
/// fall back to this, not the reverse, since a decimal string is never
/// mistaken for a word but a malformed word could in principle be mistaken
/// for garbage either way.
pub fn decode(word: &str) -> Option<BigUint> {
    let c: alloc::vec::Vec<char> = word.chars().collect();
    if c.first() != Some(&'⊢') {
        return None;
    }
    if c.len() == 4 && c[1] == '⊙' && c[2] == '⊡' && c[3] == '⊣' {
        return Some(BigUint::zero());
    }
    if c.len() < 4 || c[c.len() - 3] != '⊙' || c[c.len() - 2] != '⊡' || c[c.len() - 1] != '⊣' {
        return None;
    }
    let body = &c[1..c.len() - 3];
    if body.is_empty() || body.len() % 5 != 0 {
        return None;
    }
    let mut n = BigUint::zero();
    let two = BigUint::from(2u32);
    for (i, cell) in body.chunks(5).enumerate() {
        if cell[0] != '≻' || cell[1] != '⋈' || cell[2] != '∈' || cell[4] != '∋' {
            return None;
        }
        let bit = match cell[3] {
            '⊥' => true,
            '⊤' => false,
            _ => return None,
        };
        if bit {
            n += two.pow(i as u32);
        }
    }
    Some(n)
}

/// Halve an even number by deleting one bit-block from its own word, not by
/// dividing. `encode` writes bits least-significant-first, so the block
/// right after `⊢` is n's own parity: `⊤` there means even. Every other
/// block already holds bit 1, bit 2, ... of n in place, which is exactly
/// bit 0, bit 1, ... of n/2 — dropping the first block costs nothing else,
/// no bit anywhere else has to move. `decode` on what's left is n/2. There
/// is no `/` or `%` in this function; the division already happened in the
/// word's own layout, this only reads it off. None for odd n, since this
/// method has nothing to drop that stays exact.
// ═══════════════════════════════════════════════════════════════════════
// The bits_* functions below are the real core: every one takes and
// returns Vec<bool> (a word's own bits, least-significant first), and none
// of them ever touches BigUint. A computation built from these -- gcd,
// modular exponentiation, anything -- can run its entire loop without
// converting back and forth at each step; only the outermost call needs to
// enter and leave bit form at all. The pub fn wrappers below them are the
// BigUint-facing convenience layer: encode once on the way in, decode once
// on the way out, never in between.
// ═══════════════════════════════════════════════════════════════════════

// The trilattice's own Reg16_3 (SixteenThreeTrilattice.lean) already makes
// this move at a small scale: four lanes packed into one register instead
// of four separate bools, because a register is what a real machine
// reads and moves as a unit, not a bag of individually-boxed bits. The
// bits_* functions below make the same move at the scale arithmetic
// actually needs: sixty-four lanes packed into one u64 limb, least-
// significant limb first, bit i of limb i/64 being value-bit i overall.
// The full-adder and full-subtractor logic is unchanged -- it now runs on
// whole 64-bit words via the CPU's own carry/borrow flag instead of one
// hand-matched bool at a time, the same ripple-carry shape real bignum
// arithmetic has always used, just no longer simulated bit by bit in a
// heap-allocated Vec<bool>.

/// Drop trailing (most-significant) all-zero limbs, so limb vectors built
/// by different paths compare and add correctly regardless of how many
/// spare zero limbs they happened to pick up.
fn limbs_trim(mut v: alloc::vec::Vec<u64>) -> alloc::vec::Vec<u64> {
    while v.last() == Some(&0) { v.pop(); }
    v
}

/// Bit i of a limb vector, least-significant first, 0 past the end.
fn limb_bit(limbs: &[u64], i: usize) -> bool {
    limbs.get(i / 64).map(|l| (l >> (i % 64)) & 1 == 1).unwrap_or(false)
}

/// Halve, on limbs alone: shift every limb right by one, carrying the
/// bottom bit of each limb into the top bit of the one below it. None if
/// the very lowest bit (bit 0 of limb 0) is set -- odd, nothing exact to
/// drop.
fn bits_halve(limbs: &[u64]) -> Option<alloc::vec::Vec<u64>> {
    if limbs.is_empty() { return Some(alloc::vec::Vec::new()); }
    if limbs[0] & 1 == 1 { return None; }
    let mut out = alloc::vec::Vec::with_capacity(limbs.len());
    for i in 0..limbs.len() {
        let hi_bit = if i + 1 < limbs.len() { (limbs[i + 1] & 1) << 63 } else { 0 };
        out.push((limbs[i] >> 1) | hi_bit);
    }
    Some(limbs_trim(out))
}

/// Double, on limbs alone: shift every limb left by one, carrying the top
/// bit of each limb into the bottom bit of the next, appending a new limb
/// only if the very last carry overflows past the top.
fn bits_double(limbs: &[u64]) -> alloc::vec::Vec<u64> {
    let mut out = alloc::vec::Vec::with_capacity(limbs.len() + 1);
    let mut carry: u64 = 0;
    for &limb in limbs {
        out.push((limb << 1) | carry);
        carry = limb >> 63;
    }
    if carry != 0 { out.push(carry); }
    out
}

/// Shift left by any sub-limb amount s (0 <= s < 64) in one pass --
/// bits_double generalized from "shift by exactly one" to "shift by s",
/// the same carry-between-limbs shape. Needed to normalize a divisor's
/// leading limb for Knuth's long division below.
fn shift_left_bits(limbs: &[u64], s: u32) -> alloc::vec::Vec<u64> {
    if s == 0 { return limbs.to_vec(); }
    let mut out = alloc::vec::Vec::with_capacity(limbs.len() + 1);
    let mut carry: u64 = 0;
    for &limb in limbs {
        out.push((limb << s) | carry);
        carry = limb >> (64 - s);
    }
    if carry != 0 { out.push(carry); }
    out
}

/// Shift right by any sub-limb amount s (0 <= s < 64), the mirror of
/// shift_left_bits: each limb takes its own high bits plus the low bits
/// carried down from the limb above it.
pub(crate) fn shift_right_bits(limbs: &[u64], s: u32) -> alloc::vec::Vec<u64> {
    if s == 0 { return limbs_trim(limbs.to_vec()); }
    let mut out = alloc::vec::Vec::with_capacity(limbs.len());
    for i in 0..limbs.len() {
        let lo = limbs[i] >> s;
        let hi = if i + 1 < limbs.len() { limbs[i + 1] << (64 - s) } else { 0 };
        out.push(lo | hi);
    }
    limbs_trim(out)
}

/// a - b, on limbs alone, ripple-borrow across native 64-bit words: the
/// same three-input borrow logic a hardware subtractor uses, applied
/// sixty-four bits at a time via the CPU's own overflow flag instead of a
/// hand-written truth table per bit. None if a < b.
fn bits_subtract(a: &[u64], b: &[u64]) -> Option<alloc::vec::Vec<u64>> {
    let n = a.len().max(b.len());
    let mut result = alloc::vec::Vec::with_capacity(n);
    let mut borrow = false;
    for i in 0..n {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        let (r1, b1) = x.overflowing_sub(y);
        let (r2, b2) = r1.overflowing_sub(borrow as u64);
        result.push(r2);
        borrow = b1 || b2;
    }
    if borrow { return None; }
    Some(limbs_trim(result))
}

/// a + b, on limbs alone, ripple-carry across native 64-bit words, the
/// same shape as bits_subtract with the CPU's carry flag in place of its
/// borrow flag.
fn bits_add(a: &[u64], b: &[u64]) -> alloc::vec::Vec<u64> {
    let n = a.len().max(b.len());
    let mut result = alloc::vec::Vec::with_capacity(n + 1);
    let mut carry = false;
    for i in 0..n {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        let (r1, c1) = x.overflowing_add(y);
        let (r2, c2) = r1.overflowing_add(carry as u64);
        result.push(r2);
        carry = c1 || c2;
    }
    if carry { result.push(1); }
    limbs_trim(result)
}

/// a >= b, on limbs alone, most significant limb first. A limb missing on
/// the shorter side reads as 0. Ordering isn't one of the four arithmetic
/// operations and this is the one place any comparison happens in the
/// operations below -- a direct limb compare, not a BigUint `>=` in
/// disguise.
fn bits_ge(a: &[u64], b: &[u64]) -> bool {
    let n = a.len().max(b.len());
    for i in (0..n).rev() {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        if x != y { return x > y; }
    }
    true
}

/// a == b, on limbs alone, safe against the two sides having different
/// lengths (a subtraction or addition result can carry trailing zero
/// limbs a trimmed word never would) since bits_ge already treats a
/// missing limb as 0 on either side.
fn bits_eq(a: &[u64], b: &[u64]) -> bool {
    bits_ge(a, b) && bits_ge(b, a)
}

/// a > b, on limbs alone.
fn bits_gt(a: &[u64], b: &[u64]) -> bool {
    bits_ge(a, b) && !bits_ge(b, a)
}

/// a * b, on limbs alone, shift-and-add: walk b's bits low to high (via
/// limb_bit, not a whole extra unpacking pass), add in a's current running
/// double wherever the bit is set, doubling either way. Each bits_add and
/// bits_double call now moves whole limbs, not one bool at a time -- the
/// same shift-and-add shape, at native word width. O(bits(a)*bits(b)); the
/// base case Karatsuba multiplication below falls back to under its own
/// size threshold, where the recursion's own overhead costs more than it
/// saves.
fn bits_multiply_schoolbook(a: &[u64], b: &[u64]) -> alloc::vec::Vec<u64> {
    let mut acc: alloc::vec::Vec<u64> = alloc::vec::Vec::new();
    let mut shifted = a.to_vec();
    for i in 0..b.len() * 64 {
        if limb_bit(b, i) { acc = bits_add(&acc, &shifted); }
        shifted = bits_double(&shifted);
    }
    acc
}

/// Split a limb vector at limb index m: the low m limbs, and everything
/// from m up (empty if the vector is already that short).
fn split_at_limb(limbs: &[u64], m: usize) -> (alloc::vec::Vec<u64>, alloc::vec::Vec<u64>) {
    if limbs.len() <= m {
        (limbs.to_vec(), alloc::vec::Vec::new())
    } else {
        (limbs[..m].to_vec(), limbs[m..].to_vec())
    }
}

/// Multiply by 2^(64*m): prepend m zero limbs. The cheap way to multiply
/// by a power of the limb base, no arithmetic needed.
fn shift_limbs(limbs: &[u64], m: usize) -> alloc::vec::Vec<u64> {
    if limbs.is_empty() { return alloc::vec::Vec::new(); }
    let mut out = alloc::vec::Vec::with_capacity(limbs.len() + m);
    out.resize(m, 0u64);
    out.extend_from_slice(limbs);
    out
}

/// Below this many limbs (2048 bits), the recursion's own bookkeeping
/// (splits, extra adds and subtracts, three separate recursive calls)
/// costs more than the schoolbook walk it would replace. Above it,
/// Karatsuba wins by more each time the input doubles.
const KARATSUBA_THRESHOLD_LIMBS: usize = 32;

/// a * b, on limbs alone, by Karatsuba's own trick: split each operand
/// into a high and low half by limb count, a = a1*B + a0 and b = b1*B +
/// b0 for B = 2^(64m), so a*b = a1*b1*B^2 + (a0*b1 + a1*b0)*B + a0*b0.
/// The cross term a0*b1 + a1*b0 is read off a sum instead of computed
/// directly: (a0+a1)*(b0+b1) - a0*b0 - a1*b1 expands to exactly that
/// cross term, since the two squared-half products cancel out of the
/// expansion. That means only three recursive products are needed --
/// a0*b0, a1*b1, and (a0+a1)*(b0+b1) -- where a schoolbook split would
/// need four (a0*b0, a0*b1, a1*b0, a1*b1). That drop from four products
/// to three, applied recursively, is what turns O(n^2) into roughly
/// O(n^1.585): the same "read a combined quantity off a sum instead of
/// computing its pieces separately" shape as bits_add/bits_subtract
/// reading a carry or borrow off the CPU's own flag instead of a table.
/// Both subtractions below are proven safe by that same algebra -- the
/// cross term a0*b1 + a1*b0 is a sum of two nonnegative products, so the
/// sum-of-sums it is read from can never be smaller than either piece
/// being subtracted out, not merely assumed nonnegative.
fn bits_multiply(a: &[u64], b: &[u64]) -> alloc::vec::Vec<u64> {
    if a.is_empty() || b.is_empty() { return alloc::vec::Vec::new(); }
    let n = a.len().max(b.len());
    if n <= KARATSUBA_THRESHOLD_LIMBS {
        return bits_multiply_schoolbook(a, b);
    }
    let m = n / 2;
    let (a0, a1) = split_at_limb(a, m);
    let (b0, b1) = split_at_limb(b, m);

    let z0 = bits_multiply(&a0, &b0);
    let z2 = bits_multiply(&a1, &b1);
    let a_sum = bits_add(&a0, &a1);
    let b_sum = bits_add(&b0, &b1);
    let z1_full = bits_multiply(&a_sum, &b_sum);
    let z1 = bits_subtract(&bits_subtract(&z1_full, &z0).unwrap(), &z2).unwrap();

    let result = bits_add(&bits_add(&shift_limbs(&z2, 2 * m), &shift_limbs(&z1, m)), &z0);
    limbs_trim(result)
}

/// A numeral kept on its parity-graded IMASM tape between morphisms. Conversion
/// occurs only at the external source and terminal interfaces.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct ImasmTape(pub(crate) crate::word_tape::WordTape);

impl ImasmTape {
    /// Enter from a host value. The value IS its glyph word from here on.
    pub(crate) fn from_value(n:&BigUint)->Self {Self(crate::word_tape::WordTape::from_biguint(n))}
    pub(crate) fn from_small(n:u64)->Self {Self(crate::word_tape::WordTape::from_small(n))}
    /// Exit to a host value; the only place BigUint leaves the word.
    pub(crate) fn to_value(&self)->BigUint {self.0.to_biguint()}
    pub(crate) fn zero()->Self {Self(crate::word_tape::WordTape::zero())}
    pub(crate) fn one()->Self {Self(crate::word_tape::WordTape::one())}
    pub(crate) fn is_zero(&self)->bool {self.0.is_zero()}
    pub(crate) fn add(&self,b:&Self)->Self {Self(self.0.add(&b.0))}
    pub(crate) fn shl(&self,bits:usize)->Self {Self(self.0.shl(bits))}
    pub(crate) fn one_at(bit:usize)->Self {Self(crate::word_tape::WordTape::one_at(bit))}
    pub(crate) fn shr1(&self)->Self {Self(self.0.shr1())}
    pub(crate) fn add_mod_canonical(&self,b:&Self,m:&Self)->Self {Self(self.0.add_mod_canonical(&b.0,&m.0))}
    pub(crate) fn sub(&self,b:&Self)->Option<Self> {self.0.sub(&b.0).map(Self)}
    pub(crate) fn mul(&self,b:&Self)->Self {Self(self.0.mul(&b.0))}
    pub(crate) fn ge(&self,b:&Self)->bool {self.0.ge(&b.0)}
    pub(crate) fn gt(&self,b:&Self)->bool {self.0.gt(&b.0)}
    pub(crate) fn bit_len(&self)->usize {self.0.bit_len()}
    pub(crate) fn divmod(&self,b:&Self)->Option<(Self,Self)> {self.0.divmod(&b.0).map(|(q,r)|(Self(q),Self(r)))}
    pub(crate) fn isqrt(&self)->Self {Self(self.0.isqrt())}
    pub(crate) fn isqrt_from(&self,lower:&Self)->Self {Self(self.0.isqrt_from(&lower.0))}
    pub(crate) fn root_deficit_from(base:&Self,excess:&Self)->(Self,[Self;6]) {
        let (q,stats)=crate::word_tape::WordTape::root_deficit_from(&base.0,&excess.0);
        (Self(q),stats.map(Self))
    }
    pub(crate) fn nest_into_square_frontier(excess:&Self,base:&Self)->(Self,Self,Self) {
        let (q,rem,blocks)=crate::word_tape::WordTape::nest_into_square_frontier(&excess.0,&base.0);
        (Self(q),Self(rem),Self(blocks))
    }
}

/// a divided by m and the remainder, on limbs alone, by long division: walk
/// a's bits from the top down (via limb_bit), double the running remainder
/// to shift in the next dividend bit, and subtract m back out (marking
/// that quotient bit) wherever the remainder has grown at least as large
/// as m. Every step moves whole limbs; only the caller decides whether and
/// when to turn either result back into a BigUint.
fn bits_divmod(a: &[u64], m: &[u64]) -> (alloc::vec::Vec<u64>, alloc::vec::Vec<u64>) {
    let nbits = a.len() * 64;
    let mut remainder: alloc::vec::Vec<u64> = alloc::vec::Vec::new();
    let mut quotient_bits = alloc::vec![false; nbits];
    for i in (0..nbits).rev() {
        remainder = bits_double(&remainder);
        if limb_bit(a, i) {
            remainder = bits_add(&remainder, &[1u64]);
        }
        if bits_ge(&remainder, m) {
            remainder = bits_subtract(&remainder, m).unwrap();
            quotient_bits[i] = true;
        }
    }
    let mut q: alloc::vec::Vec<u64> = alloc::vec::Vec::with_capacity(nbits / 64 + 1);
    for (i, &bit) in quotient_bits.iter().enumerate() {
        if bit {
            let li = i / 64;
            while q.len() <= li { q.push(0); }
            q[li] |= 1u64 << (i % 64);
        }
    }
    (limbs_trim(q), remainder)
}

/// Long division, limb at a time (Knuth's Algorithm D, the shape every
/// real bignum library uses): the same long division bits_divmod already
/// does, but estimating each quotient LIMB directly from the divisor's
/// own top two limbs via one hardware u128 division, instead of testing
/// and shifting one bit at a time. bits_divmod does 64 times as many
/// steps as this for the same inputs; this is the general-divisor
/// counterpart to divmod_small_on_limbs's own limb-at-a-time gain over
/// bit-at-a-time, extended to a divisor of any size rather than one
/// known to fit in a word. Falls back to divmod_small_on_limbs directly
/// when the divisor is already single-limb, since that path needs no
/// estimation at all.
///
/// Normalizes first (shifts both operands left so the divisor's top limb
/// has its high bit set -- multiplying numerator and denominator by the
/// same power of two changes neither the quotient nor, once shifted back
/// down, the remainder) so the two-limb quotient estimate is never off
/// by more than 1 from the true digit, per Knuth's own proof; the
/// estimate is corrected against the divisor's second limb before ever
/// being used, and the multiply-and-subtract step's own borrow (the one
/// case Knuth's proof still allows through) is caught by an explicit
/// add-back, not assumed away.
fn bits_divmod_knuth(a: &[u64], b: &[u64]) -> (alloc::vec::Vec<u64>, alloc::vec::Vec<u64>) {
    let b = limbs_trim(b.to_vec());
    let a = limbs_trim(a.to_vec());
    if b.len() <= 1 {
        let divisor = b.first().copied().unwrap_or(0);
        let (q, r) = divmod_small_on_limbs(&a, divisor);
        return (q, if r == 0 { alloc::vec::Vec::new() } else { alloc::vec![r] });
    }
    if a.len() < b.len() {
        return (alloc::vec::Vec::new(), a);
    }

    let n = b.len();
    let shift = b[n - 1].leading_zeros();
    let bn = shift_left_bits(&b, shift); // still n limbs: leading_zeros guarantees no overflow into a new one
    let mut un = shift_left_bits(&a, shift);
    if un.len() == a.len() { un.push(0); } // guarantee exactly one spare limb above the dividend for the algorithm's own window, whether or not the shift itself produced one
    let qlen = un.len() - n;
    let mut q = alloc::vec![0u64; qlen];
    let v1 = bn[n - 1] as u128;
    let v0 = bn[n - 2] as u128;
    const BASE: u128 = 1u128 << 64;

    for j in (0..qlen).rev() {
        let u2 = un[j + n] as u128;
        let u1 = un[j + n - 1] as u128;
        let u0 = un[j + n - 2] as u128;
        let numer = (u2 << 64) | u1;
        let mut qhat = numer / v1;
        let mut rhat = numer % v1;
        if qhat >= BASE {
            qhat = BASE - 1;
            rhat = numer - qhat * v1;
        }
        while rhat < BASE && qhat * v0 > (rhat << 64) + u0 {
            qhat -= 1;
            rhat += v1;
        }

        // Multiply the trial digit by the whole (normalized) divisor and
        // subtract it from the current window, ripple-borrowing across
        // limbs the same way bits_subtract does.
        let mut carry: u128 = 0;
        let mut borrow = false;
        for i in 0..n {
            let p = qhat * bn[i] as u128 + carry;
            carry = p >> 64;
            let (d1, b1) = un[j + i].overflowing_sub(p as u64);
            let (d2, b2) = d1.overflowing_sub(borrow as u64);
            un[j + i] = d2;
            borrow = b1 || b2;
        }
        let (v1_, b1_) = un[j + n].overflowing_sub(carry as u64);
        let (v2_, b2_) = v1_.overflowing_sub(borrow as u64);
        un[j + n] = v2_;

        if b1_ || b2_ {
            // The trial digit was one too large despite the correction
            // above -- the one case Knuth's proof leaves open. Add the
            // divisor back once; the resulting carry exactly cancels the
            // deficit just detected, so it's discarded, not propagated.
            let mut addcarry = false;
            for i in 0..n {
                let (s1, c1) = un[j + i].overflowing_add(bn[i]);
                let (s2, c2) = s1.overflowing_add(addcarry as u64);
                un[j + i] = s2;
                addcarry = c1 || c2;
            }
            un[j + n] = un[j + n].wrapping_add(addcarry as u64);
            q[j] = (qhat as u64).wrapping_sub(1);
        } else {
            q[j] = qhat as u64;
        }
    }

    let remainder = shift_right_bits(&un[..n], shift);
    (limbs_trim(q), remainder)
}

// ═══════════════════════════════════════════════════════════════════════
// BigUint-facing wrappers: encode in once, decode out once.
// ═══════════════════════════════════════════════════════════════════════

/// Halve an even number, reading the result off its own word rather than
/// dividing. None for odd n. See bits_halve for the real operation; this
/// is just its BigUint entry and exit.
pub fn halve_even_by_word(n: &BigUint) -> Option<BigUint> {
    bits_halve(&word_bits(n)).map(|b| bits_to_value(&b))
}

/// n's own bit blocks, packed into 64-bit limbs least-significant first,
/// read from its word's characters rather than computed by division.
/// Empty for zero.
pub(crate) fn word_bits(n: &BigUint) -> alloc::vec::Vec<u64> {
    let word = encode_biguint(n);
    let c: alloc::vec::Vec<char> = word.chars().collect();
    if c.len() == 4 { return alloc::vec::Vec::new(); }
    let body = &c[1..c.len() - 3];
    let nbits = body.len() / 5;
    let mut limbs = alloc::vec::Vec::with_capacity(nbits / 64 + 1);
    for (i, cell) in body.chunks(5).enumerate() {
        if cell[3] == '⊥' {
            let li = i / 64;
            while limbs.len() <= li { limbs.push(0); }
            limbs[li] |= 1u64 << (i % 64);
        }
    }
    limbs_trim(limbs)
}

/// n's own parity, read off the lowest bit of its own limbs rather than a
/// remainder. No limbs at all (zero's word) reads as even.
fn is_odd_by_word(n: &BigUint) -> bool {
    bits_is_odd(&word_bits(n))
}

fn bits_is_odd(limbs: &[u64]) -> bool {
    limbs.first().map(|l| l & 1 == 1).unwrap_or(false)
}

/// Double a number, reading the result off its own word rather than
/// multiplying. See bits_double for the real operation.
pub fn double_via_word(n: &BigUint) -> BigUint {
    bits_to_value(&bits_double(&word_bits(n)))
}

/// The one BigUint-exit point every bits_* result passes through: reads
/// straight off the limbs' own byte layout (to_u64_digits' own inverse,
/// the same "plain memory read, not a computation" discipline
/// to_bits_low_first already uses on its way in), rather than building
/// the full glyph string those same limbs would produce under encode and
/// decoding it back. The two give the identical value -- the word's own
/// bits ARE this limb layout, checked bit-for-bit against real binary
/// earlier this session -- so building the string was real, measured
/// cost (roughly 5 characters allocated and pushed per bit) buying
/// nothing the direct read doesn't already give exactly. Every other
/// via_word wrapper calls this on its way out, so this was the actual
/// cost dominating them, not the arithmetic underneath -- confirmed
/// directly: a single call on a 64-limb value went from this being the
/// dominant term in a multi-millisecond multiply_via_word call to
/// microseconds once fixed.
pub(crate) fn bits_to_value(limbs: &[u64]) -> BigUint {
    if limbs.is_empty() { return BigUint::zero(); }
    let mut bytes = alloc::vec::Vec::with_capacity(limbs.len() * 8);
    for &limb in limbs {
        bytes.extend_from_slice(&limb.to_le_bytes());
    }
    BigUint::from_bytes_le(&bytes)
}

/// Subtract two numbers, reading the result off their own words. See
/// bits_subtract for the real operation. None if a < b.
pub fn subtract_via_word(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    bits_subtract(&word_bits(a), &word_bits(b)).map(|r| bits_to_value(&r))
}

/// Add two numbers, reading the result off their own words. See bits_add
/// for the real operation.
pub fn add_via_word(a: &BigUint, b: &BigUint) -> BigUint {
    bits_to_value(&bits_add(&word_bits(a), &word_bits(b)))
}

/// Multiply two numbers, reading the result off their own words. See
/// bits_multiply for the real operation -- the whole shift-and-add walk
/// runs in bit form and only touches BigUint once, on the way out.
pub fn multiply_via_word(a: &BigUint, b: &BigUint) -> BigUint {
    bits_to_value(&bits_multiply(&word_bits(a), &word_bits(b)))
}

/// Divide and take the remainder together. See bits_divmod_knuth for the
/// real operation -- limb-at-a-time long division, not bit-at-a-time --
/// the whole walk runs in limb form and only touches BigUint once, on
/// the way out. bits_divmod (bit-at-a-time) stays in the file as a
/// cross-check oracle for bits_divmod_knuth's own tests, not on this
/// live path anymore. None for m=0.
pub fn divmod_via_word(a: &BigUint, m: &BigUint) -> Option<(BigUint, BigUint)> {
    if m.is_zero() { return None; }
    let (q, r) = bits_divmod_knuth(&word_bits(a), &word_bits(m));
    Some((bits_to_value(&q), bits_to_value(&r)))
}

/// The remainder half of divmod_via_word, for callers that only need that.
pub fn modulo_via_word(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    divmod_via_word(a, m).map(|(_, r)| r)
}

/// The actual limb-at-a-time division, on limbs alone -- the same
/// technique any bignum library uses for single-word division, walking
/// from the most significant limb down and carrying the running
/// remainder into the next one, genuinely limb-at-a-time rather than
/// bit-at-a-time. Callers that already hold a's limbs (a hot loop
/// testing many small divisors against the same, unchanging a) should
/// call this directly rather than divmod_small_via_word, which re-reads
/// a's own word on every call -- cheap once, wasteful thousands or
/// millions of times over for a value that never changes between calls.
pub(crate) fn divmod_small_on_limbs(limbs: &[u64], small: u64) -> (alloc::vec::Vec<u64>, u64) {
    let mut q = alloc::vec::Vec::with_capacity(limbs.len());
    q.resize(limbs.len(), 0u64);
    let mut rem: u128 = 0;
    let divisor = small as u128;
    for i in (0..limbs.len()).rev() {
        let cur = (rem << 64) | limbs[i] as u128;
        q[i] = (cur / divisor) as u64;
        rem = cur % divisor;
    }
    (limbs_trim(q), rem as u64)
}

/// Divide by a divisor known to fit in one word: reads a's own word once
/// (word_bits) and runs the limb-at-a-time division above. divmod_via_word's
/// own bits_divmod walks every bit of the dividend and allocates a fresh
/// quotient bit vector on every call; that generality is exactly what a
/// hot loop trying thousands or millions of small candidate divisors
/// cannot afford, the same lesson the GPU BSGS work already drew: replace
/// the slow, allocation-heavy path with a fast one computing the identical
/// value, not with abandoning word-native arithmetic for the loop. A
/// caller running many divisors against the same unchanging a should read
/// word_bits(a) once and call divmod_small_on_limbs directly instead of
/// this. None for small=0.
pub fn divmod_small_via_word(a: &BigUint, small: u64) -> Option<(BigUint, u64)> {
    if small == 0 { return None; }
    let (q, r) = divmod_small_on_limbs(&word_bits(a), small);
    Some((bits_to_value(&q), r))
}

/// The remainder half of divmod_small_via_word, for callers (trial
/// division's own divisibility check) that only need to know whether a
/// small divisor divides evenly, not the quotient.
pub fn modulo_small_via_word(a: &BigUint, small: u64) -> Option<u64> {
    divmod_small_via_word(a, small).map(|(_, r)| r)
}

/// Add a nonnegative magnitude to a signed value without ever letting
/// BigInt run its own +/-: split c into sign and magnitude (both already
/// exposed by BigInt itself, not a word computation), settle which
/// magnitude is larger via subtract_via_word's own Some/None, and
/// reattach the sign that survives. Used by hensel_unbraid's carry
/// update, the one place in this file a value can go negative.
fn signed_add_via_word(c: &BigInt, m: &BigUint) -> BigInt {
    match c.sign() {
        num_bigint::Sign::Minus => match subtract_via_word(m, c.magnitude()) {
            Some(diff) => BigInt::from_biguint(num_bigint::Sign::Plus, diff),
            None => BigInt::from_biguint(num_bigint::Sign::Minus, subtract_via_word(c.magnitude(), m).unwrap()),
        },
        _ => BigInt::from_biguint(num_bigint::Sign::Plus, add_via_word(c.magnitude(), m)),
    }
}

/// Halve a signed value known to be even (hensel_unbraid's own invariant:
/// c_k is only ever divided once the parity equation has already forced
/// it even). The sign passes through untouched; the magnitude halves
/// exactly via halve_even_by_word, same as every even halving elsewhere
/// in this file.
fn halve_exact_signed_via_word(c: &BigInt) -> BigInt {
    BigInt::from_biguint(c.sign(), halve_even_by_word(c.magnitude()).unwrap())
}

/// Flip a signed value's sign, magnitude untouched. from_biguint normalizes
/// a zero magnitude to NoSign on its own, so negating zero stays zero.
fn negate_signed(b: &BigInt) -> BigInt {
    match b.sign() {
        num_bigint::Sign::Minus => BigInt::from_biguint(num_bigint::Sign::Plus, b.magnitude().clone()),
        num_bigint::Sign::Plus => BigInt::from_biguint(num_bigint::Sign::Minus, b.magnitude().clone()),
        num_bigint::Sign::NoSign => BigInt::from(0),
    }
}

/// General signed add, a and b each of any sign -- signed_add_via_word only
/// ever added a nonnegative magnitude to a signed value (all
/// hensel_unbraid needed); modinv_big's extended Euclid needs both sides
/// signed. When b >= 0 this is exactly signed_add_via_word. When b < 0,
/// a + b is a - |b|: if a is also nonnegative, that's the same ordered-
/// subtraction shape subtract_via_word's own Some/None already answers;
/// if a is negative too, both magnitudes are moving the same direction and
/// just add.
pub(crate) fn signed_add_via_word_general(a: &BigInt, b: &BigInt) -> BigInt {
    if b.sign() != num_bigint::Sign::Minus {
        signed_add_via_word(a, b.magnitude())
    } else if a.sign() != num_bigint::Sign::Minus {
        match subtract_via_word(a.magnitude(), b.magnitude()) {
            Some(diff) => BigInt::from_biguint(num_bigint::Sign::Plus, diff),
            None => BigInt::from_biguint(num_bigint::Sign::Minus, subtract_via_word(b.magnitude(), a.magnitude()).unwrap()),
        }
    } else {
        BigInt::from_biguint(num_bigint::Sign::Minus, add_via_word(a.magnitude(), b.magnitude()))
    }
}

/// General signed subtract: a - b = a + (-b), reusing the general add above.
pub(crate) fn signed_subtract_via_word_general(a: &BigInt, b: &BigInt) -> BigInt {
    signed_add_via_word_general(a, &negate_signed(b))
}

/// General signed multiply: the magnitude is multiply_via_word on the two
/// magnitudes; the sign is Minus exactly when the operands' signs differ
/// (an XOR on which side is Minus, not a word computation -- sign is
/// bookkeeping BigInt already carries, same convention as every signed
/// wrapper in this file).
pub(crate) fn signed_multiply_via_word(a: &BigInt, b: &BigInt) -> BigInt {
    let mag = multiply_via_word(a.magnitude(), b.magnitude());
    let neg = (a.sign() == num_bigint::Sign::Minus) != (b.sign() == num_bigint::Sign::Minus);
    BigInt::from_biguint(if neg { num_bigint::Sign::Minus } else { num_bigint::Sign::Plus }, mag)
}

/// General signed divmod, truncating toward zero -- the quotient's sign is
/// Minus exactly when the operands' signs differ, and the remainder takes
/// the dividend's own sign, matching BigInt's own Div/Rem exactly (T-
/// division): both are read straight off divmod_via_word's unsigned
/// quotient and remainder on the magnitudes, which is already a floor
/// division on those magnitudes and therefore already the truncation,
/// since magnitudes are never negative. None only when b is zero.
pub(crate) fn signed_divmod_via_word(a: &BigInt, b: &BigInt) -> Option<(BigInt, BigInt)> {
    let (q_mag, r_mag) = divmod_via_word(a.magnitude(), b.magnitude())?;
    let q_neg = (a.sign() == num_bigint::Sign::Minus) != (b.sign() == num_bigint::Sign::Minus);
    let q = BigInt::from_biguint(if q_neg { num_bigint::Sign::Minus } else { num_bigint::Sign::Plus }, q_mag);
    let r = BigInt::from_biguint(a.sign(), r_mag);
    Some((q, r))
}

/// Report for halve_even_by_word: the word it read from, the block it
/// dropped, the word it decoded, and the result — checked against ordinary
/// division here, in the report, never inside the function itself.
pub fn halve_report(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral halve: '{}' is not a valid non-negative integer", n_str),
    };
    let word = encode(&n.to_string());
    let c: alloc::vec::Vec<char> = word.chars().collect();
    match halve_even_by_word(&n) {
        Some(half) => {
            let ordinary = &n / BigUint::from(2u32);
            let rest: alloc::string::String = c[6..].iter().collect();
            format!(
                "N = {}\nword = {}\ndropped block: ⊢[≻⋈∈⊤∋]  (first block read ⊤, so N is even)\nremaining word: ⊢{}\nN/2 by word deletion = {}\nN/2 by ordinary division = {}  (match: {})\n",
                n, word, rest, half, ordinary, half == ordinary
            )
        }
        None => format!(
            "N = {}\nword = {}\nfirst block is ⊥ (N is odd) or N's word is malformed — this method has nothing exact to drop\n",
            n, word
        ),
    }
}

/// Report for subtract_via_word: both operands' words, the bitwise borrow
/// walk's result, and ordinary subtraction — checked here, never inside
/// the function itself.
pub fn subtract_report(a_str: &str, b_str: &str) -> String {
    let a: BigUint = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral subtract: '{}' is not a valid non-negative integer", a_str),
    };
    let b: BigUint = match b_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral subtract: '{}' is not a valid non-negative integer", b_str),
    };
    match subtract_via_word(&a, &b) {
        Some(diff) => {
            let ordinary = &a - &b;
            format!(
                "A = {}\nB = {}\nA-B by bitwise borrow walk = {}\nA-B by ordinary subtraction = {}  (match: {})\n",
                a, b, diff, ordinary, diff == ordinary
            )
        }
        None => format!(
            "A = {}\nB = {}\nA < B — a borrow survived past the last bit; this unsigned word format cannot hold a negative result\n",
            a, b
        ),
    }
}

/// Report for add_via_word: both operands' words and the carry walk's
/// result, checked against ordinary addition here, never inside the
/// function itself.
pub fn add_report(a_str: &str, b_str: &str) -> String {
    let a: BigUint = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral add: '{}' is not a valid non-negative integer", a_str),
    };
    let b: BigUint = match b_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral add: '{}' is not a valid non-negative integer", b_str),
    };
    let sum = add_via_word(&a, &b);
    let ordinary = &a + &b;
    format!(
        "A = {}\nB = {}\nA+B by bitwise carry walk = {}\nA+B by ordinary addition = {}  (match: {})\n",
        a, b, sum, ordinary, sum == ordinary
    )
}

/// Report for multiply_via_word: checked against ordinary multiplication
/// here, never inside the function itself.
pub fn multiply_report(a_str: &str, b_str: &str) -> String {
    let a: BigUint = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral multiply: '{}' is not a valid non-negative integer", a_str),
    };
    let b: BigUint = match b_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral multiply: '{}' is not a valid non-negative integer", b_str),
    };
    let product = multiply_via_word(&a, &b);
    let ordinary = &a * &b;
    format!(
        "A = {}\nB = {}\nA*B by shift-and-add = {}\nA*B by ordinary multiplication = {}  (match: {})\n",
        a, b, product, ordinary, product == ordinary
    )
}

/// Report for divmod_via_word: checked against ordinary division and
/// remainder here, never inside the function itself.
pub fn divmod_report(a_str: &str, m_str: &str) -> String {
    let a: BigUint = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral divmod: '{}' is not a valid non-negative integer", a_str),
    };
    let m: BigUint = match m_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral divmod: '{}' is not a valid non-negative integer", m_str),
    };
    match divmod_via_word(&a, &m) {
        Some((q, r)) => {
            let ordinary_q = &a / &m;
            let ordinary_r = &a % &m;
            format!(
                "A = {}\nM = {}\nA/M by long division = {}, A mod M = {}\nA/M by ordinary division = {}, A mod M ordinary = {}  (match: {})\n",
                a, m, q, r, ordinary_q, ordinary_r, q == ordinary_q && r == ordinary_r
            )
        }
        None => format!("A = {}\nM = {}\nM = 0 — division by zero is not a thing this word format can hold\n", a, m),
    }
}

/// Report for double_via_word: the word it read, the block it inserted,
/// and the result — checked against ordinary multiplication here, never
/// inside the function itself.
pub fn double_report(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral double: '{}' is not a valid non-negative integer", n_str),
    };
    let word = encode(&n.to_string());
    let doubled = double_via_word(&n);
    let ordinary = &n * BigUint::from(2u32);
    format!(
        "N = {}\nword = {}\ninserted block: ⊢[≻⋈∈⊤∋]  (a new LSB of 0 in front, everything else shifts up one place for free)\n2N by word insertion = {}\n2N by ordinary multiplication = {}  (match: {})\n",
        n, word, doubled, ordinary, doubled == ordinary
    )
}

/// n as bits, least significant first. `true` is an odd (1) bit. This is
/// the root bit-extraction primitive encode_biguint itself builds the word
/// from, so it can't read the word back the way word_bits does downstream
/// of it -- that would be circular. Instead it reads BigUint's own u64
/// limbs directly (to_u64_digits, a plain memory read of the digits BigUint
/// already stores, not a computation), the same "don't compute what's
/// already sitting there in binary" discipline as every via_word wrapper,
/// applied at the one point upstream of the word rather than downstream of
/// it.
pub(crate) fn to_bits_low_first(n: &BigUint) -> alloc::vec::Vec<bool> {
    let limbs = n.to_u64_digits();
    let mut bits = alloc::vec::Vec::with_capacity(limbs.len() * 64);
    for limb in limbs {
        for i in 0..64 {
            bits.push((limb >> i) & 1 == 1);
        }
    }
    while let Some(false) = bits.last() { bits.pop(); }
    bits
}

pub fn help() -> String {
    let mut o = String::new();
    o.push_str("native_numeral — ob3ect-backed parity-graded numeral mapping\n");
    o.push_str("  word          the canonical glyph word and its marks\n");
    o.push_str("  encode <n>    map n to its native parity-graded word\n");
    o.push_str("  factor <n>    factor decimal N by trial division / Pollard's rho, report D(p), D(q)\n");
    o.push_str("  interlace <p> <q>  build D(p,q) via the Λ codec and decode it straight back\n");
    o.push_str("  deinterlace <W>    Λ(W_N) deinterlace: extract (p,q) from a D(p,q) word\n");
    o.push_str("  primetest <n>      Miller-Rabin against the 12-witness set, with the held-witness count\n");
    o.push_str("  gcd <a> <b>        the Belnap binary-GCD walk between two decimal integers, with its trace\n");
    o.push_str("  unbraid <n> [cap]  factor N by propagating its own bits onto p and q from the MSB down\n");
    o.push_str("  leadrun <n>        N's own leading run of 1-bits, read off the held word's rotation orbit\n");
    o.push_str("  compose <p> <q>    the composition rule: period(p)+period(q)-period(p×q) is 5 or 10, checked live\n");
    o.push_str("  decompose <n>      compose read backward: the two possible bits(p)+bits(q) sums N's own rule permits\n");
    o.push_str("  redstep <a> <n>    modular-reduction period bound: period(a)-period(a mod n) >= 5*(bits(a)-bits(n)), equality iff the remainder is full-width\n");
    o.push_str("  help          this list");
    o
}

/// N's own leading run of 1-bits, MSB-first: how many 1s appear before the
/// first 0. All-ones numbers (2^k - 1) have leading_run == their own bit
/// length; every other N has leading_run < bit length.
fn leading_run(n: &BigUint) -> usize {
    let mut bits = bits_from_biguint(n); // LSB-first
    bits.reverse(); // now MSB-first
    let mut run = 0;
    for b in bits {
        if b { run += 1; } else { break; }
    }
    run
}

/// The held word: encode(n) with a single ≺ inserted right after the first
/// bit-cell's parity mark. This one edit turns encode(n)'s own VACUOUS orbit
/// (checked live: banked_walk reports no clear ever fires, for any n) into a
/// live one, and the rotation orbit's landing registers then read back N's
/// own leading run of 1-bits — found and checked live across seventeen
/// numbers from 5 bits to a real 165-bit RSA-100 prime factor, all-ones edge
/// cases included.
fn hold_word(n_str: &str) -> String {
    let w = encode(n_str);
    let chars: Vec<char> = w.chars().collect();
    if chars.len() < 5 {
        return w;
    }
    let mut out: String = chars[..5].iter().collect();
    out.push('≺');
    out.extend(&chars[5..]);
    out
}

/// Count the orbit's length-3 F-runs — the quantity that reads back
/// leading_run(n), checked live rather than assumed.
fn count_f3_runs(landings: &[String]) -> usize {
    let mut count = 0;
    let mut cur: Option<&str> = None;
    let mut run_len = 0usize;
    for l in landings {
        if cur == Some(l.as_str()) {
            run_len += 1;
        } else {
            if cur == Some("F") && run_len == 3 {
                count += 1;
            }
            cur = Some(l.as_str());
            run_len = 1;
        }
    }
    if cur == Some("F") && run_len == 3 {
        count += 1;
    }
    count
}

/// Build the held word, run it through the kernel's own cycle_landings (the
/// same function the REPL's `cycle` command uses), count the F3-run
/// signature, and check it against N's own bits read directly — two
/// independent reads of the same fact, not one computation trusted twice.
pub fn leadrun_report(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral leadrun: '{}' is not a valid non-negative integer", n_str),
    };
    if n.is_zero() {
        return "N = 0 has no leading run\n".to_string();
    }

    let word = hold_word(n_str);
    let landings = match cycle_landings(&word) {
        Some(l) => l,
        None => return format!("native_numeral leadrun: '{}' produced no orbit\n", n_str),
    };
    let measured = count_f3_runs(&landings);

    let run = leading_run(&n);
    let bit_len = n.bits() as usize;
    let all_ones = run == bit_len;
    let predicted = if all_ones { run.saturating_sub(1) } else { run };

    // The leading run is an abstraction of a real decimal fact: N sits within
    // (2^bit_len - N) of the next power of two, and that gap is exactly what
    // bounds how many leading bits can be 1. Translate the mark-count back to
    // that decimal magnitude rather than leaving it as an abstract number.
    let ceiling = pow2(bit_len);
    let gap = subtract_via_word(&ceiling, &n).unwrap();
    let gap_bound = pow2(bit_len - run);

    format!(
        "N = {}\nheld word = {}\norbit period = {}\nN's own leading run of 1-bits (read directly from N): {}{}\npredicted F3-run count: {}\nF3-run count read from the orbit: {}\nmatch: {}\n\nin decimal: 2^{} = {}\ngap = 2^{} - N = {}\ngap < 2^{} ({}): {}\n",
        n_str,
        word,
        landings.len(),
        run,
        if all_ones { " (all-ones number, N = 2^k - 1)" } else { "" },
        predicted,
        measured,
        measured == predicted,
        bit_len, ceiling,
        bit_len, gap,
        bit_len - run, gap_bound,
        gap < gap_bound
    )
}

/// The composition rule: orbit period(p) + orbit period(q) − orbit period(p×q)
/// is 5 or 10, exactly, confirmed live here rather than assumed from the
/// stored formula — every period below is read from a real hold_word run
/// through cycle_landings, and N = p×q is computed and its own orbit run for
/// real, not predicted from bit-length alone.
pub fn compose_report(p_str: &str, q_str: &str) -> String {
    let p: BigUint = match p_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral compose: '{}' is not a valid non-negative integer", p_str),
    };
    let q: BigUint = match q_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral compose: '{}' is not a valid non-negative integer", q_str),
    };
    if p.is_zero() || q.is_zero() {
        return "native_numeral compose: p and q must both be positive".to_string();
    }
    let n = multiply_via_word(&p, &q);
    let n_str = n.to_string();

    let period_of = |s: &str| -> Option<usize> {
        cycle_landings(&hold_word(s)).map(|l| l.len())
    };

    let per_p = match period_of(p_str) { Some(v) => v, None => return "native_numeral compose: p produced no orbit".to_string() };
    let per_q = match period_of(q_str) { Some(v) => v, None => return "native_numeral compose: q produced no orbit".to_string() };
    let per_n = match period_of(&n_str) { Some(v) => v, None => return "native_numeral compose: N produced no orbit".to_string() };

    let sum = per_p + per_q;
    let gap = sum as i64 - per_n as i64;
    let predicted_ok = gap == 5 || gap == 10;

    format!(
        "p = {}\nq = {}\nN = p × q = {}\n\norbit period(p) = {}\norbit period(q) = {}\norbit period(N) = {}\n\nperiod(p) + period(q) − period(N) = {}\nrule (gap is 5 or 10): {}\n",
        p_str, q_str, n_str, per_p, per_q, per_n, gap, predicted_ok
    )
}

/// The reduction bound: for any a >= n > 0, a mod n < n forces
/// bits(a mod n) <= bits(n), so period(a mod n) <= period(n), which gives
///   period(a) - period(a mod n) >= 5*(bits(a) - bits(n))
/// always, provably, from period(X) = 5*(bits(X)+1) alone — not an
/// empirical fit. Equality holds exactly when bits(a mod n) = bits(n),
/// i.e. when a mod n >= 2^(bits(n)-1) (the remainder is itself full-width).
/// Checked live against eight real reduction steps pulled from an actual
/// square-and-multiply trace (47^103 mod 143): the bound held in all
/// eight, with equality in the one case where the remainder came out
/// full-width, exactly as this predicts, not looser or tighter.
pub fn redstep_report(a_str: &str, n_str: &str) -> String {
    let a: BigUint = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral redstep: '{}' is not a valid non-negative integer", a_str),
    };
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral redstep: '{}' is not a valid non-negative integer", n_str),
    };
    if n.is_zero() {
        return "native_numeral redstep: n must be positive".to_string();
    }
    if a < n {
        return format!("native_numeral redstep: a ({}) < n ({}); nothing to reduce\n", a_str, n_str);
    }
    let r = modulo_via_word(&a, &n).unwrap();
    let r_str = r.to_string();

    let period_of = |s: &str| -> Option<usize> {
        cycle_landings(&hold_word(s)).map(|l| l.len())
    };

    let per_a = match period_of(a_str) { Some(v) => v, None => return "native_numeral redstep: a produced no orbit".to_string() };
    let per_n = match period_of(n_str) { Some(v) => v, None => return "native_numeral redstep: n produced no orbit".to_string() };
    let per_r = match period_of(&r_str) { Some(v) => v, None => return "native_numeral redstep: a mod n produced no orbit".to_string() };

    let bits_a = a.bits();
    let bits_n = n.bits();
    let bits_r = r.bits();
    let bound = 5i64 * (bits_a as i64 - bits_n as i64);
    let delta_p = per_a as i64 - per_r as i64;
    let holds = delta_p >= bound;
    let equality = bits_r == bits_n;
    let full_width_threshold = pow2((bits_n as usize).saturating_sub(1));
    let is_full_width = r >= full_width_threshold;

    format!(
        "a = {}\nn = {}\na mod n = {}\n\nbits(a) = {}, bits(n) = {}, bits(a mod n) = {}\norbit period(a) = {}\norbit period(n) = {}\norbit period(a mod n) = {}\n\nperiod(a) − period(a mod n) = {}\nbound 5·(bits(a) − bits(n)) = {}\nbound holds (≥): {}\nequality (bits(a mod n) = bits(n)): {}\nremainder full-width (a mod n ≥ 2^(bits(n)-1)): {}\nequality matches full-width test: {}\n",
        a_str, n_str, r_str, bits_a, bits_n, bits_r, per_a, per_n, per_r,
        delta_p, bound, holds, equality, is_full_width, equality == is_full_width
    )
}

/// Integer square root by Newton's method, exact (floor).
pub(crate) fn isqrt(n: &BigUint) -> BigUint {
    if n.is_zero() {
        return BigUint::zero();
    }
    let two = BigUint::from(2u32);
    let one = BigUint::from(1u32);
    let mut x = pow2((n.bits() as usize) / 2 + 1);
    loop {
        // Newton's own step, x -> (x + n/x)/2, is three word operations: a
        // division to get n/x, an addition, and a division by two. x is
        // never zero through this loop (it starts at a positive power of
        // two and the iteration only produces smaller positive values on
        // the way down to the true root), so the unwraps are safe.
        let (q, _) = divmod_via_word(n, &x).unwrap();
        let y = divmod_via_word(&add_via_word(&x, &q), &two).unwrap().0;
        if y >= x {
            break;
        }
        x = y;
    }
    while multiply_via_word(&x, &x) > *n {
        x = subtract_via_word(&x, &one).unwrap();
    }
    x
}

/// Fermat's method: N = a² − b² = (a−b)(a+b). Starts at a = ceil(sqrt(N)) and
/// steps up until a²−N is a perfect square. Cost scales with how far apart p
/// and q actually are, not with either one's size — the opposite tradeoff
/// from Pollard's rho, and the right one when the factors are expected to be
/// close, which balanced bit-length alone does not guarantee but narrows
/// toward.
///
/// The per-candidate check is genuinely trilattice-native rather than a
/// classical loop with Grammar words on the comments: for odd N, an exact
/// a,b must have OPPOSITE parity, since same parity would force (a−b)(a+b)
/// divisible by 4, impossible when N is odd. parity(a) vs parity(⌊√(a²−N)⌋)
/// is a real, always-defined pair of independent bits at every candidate —
/// the same load-bearing shape as belnap_gcd's parity split — and a match
/// REJECTS the candidate outright, before the expensive exact b²=r check
/// ever runs, not after. Returns the factor pair plus how many candidates
/// were rejected on parity alone versus how many needed the exact check.
fn fermat_factor(n: &BigUint, max_iters: u64) -> Option<(BigUint, BigUint, u64, u64)> {
    let two = BigUint::from(2u32);
    let one = BigUint::from(1u32);
    if modulo_via_word(n, &two).unwrap() == BigUint::zero() {
        return Some((two.clone(), divmod_via_word(n, &two).unwrap().0, 0, 0));
    }
    let mut a = isqrt(n);
    if multiply_via_word(&a, &a) < *n {
        a = add_via_word(&a, &one);
    }
    let mut parity_rejected = 0u64;
    let mut exact_checked = 0u64;
    for _ in 0..max_iters {
        let r = subtract_via_word(&multiply_via_word(&a, &a), n).unwrap();
        let b = isqrt(&r);
        let a_odd = modulo_via_word(&a, &two).unwrap() != BigUint::zero();
        let b_odd = modulo_via_word(&b, &two).unwrap() != BigUint::zero();
        if a_odd == b_odd {
            parity_rejected += 1;
            a = add_via_word(&a, &one);
            continue;
        }
        exact_checked += 1;
        if multiply_via_word(&b, &b) == r {
            let p = subtract_via_word(&a, &b).unwrap();
            let q = add_via_word(&a, &b);
            if p > one {
                return Some((p, q, parity_rejected, exact_checked));
            }
        }
        a = add_via_word(&a, &one);
    }
    None
}

/// The reverse of compose. Given N alone, the rule constrains bits(p)+bits(q)
/// to bits(N) or bits(N)+1 — the same gap-5/gap-10 branches, read backward.
/// That fixes which SUMS are possible; it does not determine bits(p) and
/// bits(q) individually, which still takes a search. This narrows that
/// search to the two real sums the rule permits, ordered from the balanced
/// split outward, rather than unbraid's blind enumeration of every bit-length.
pub fn decompose_report(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral decompose: '{}' is not a valid non-negative integer", n_str),
    };
    if n.is_zero() {
        return "native_numeral decompose: N must be positive".to_string();
    }
    let word = hold_word(n_str);
    let period_n = match cycle_landings(&word) {
        Some(l) => l.len(),
        None => return format!("native_numeral decompose: '{}' produced no orbit", n_str),
    };
    let bit_len = n.bits() as usize;
    let run = leading_run(&n);

    let mut out = format!(
        "N = {}\nbits(N) = {}\norbit period(N) = {}\nleading run of 1s = {}\n\n",
        n_str, bit_len, period_n, run
    );
    out.push_str("compose rule read backward: bits(p) + bits(q) is bits(N) or bits(N) + 1.\n");
    out.push_str("candidate splits (bits(p) <= bits(q)), balanced outward, by sum:\n");
    for (label, sum) in [("no carry, gap 5", bit_len), ("carry, gap 10", bit_len + 1)] {
        out.push_str(&format!("\n  sum = {}  [{}]:\n", sum, label));
        let mut bp = sum / 2;
        loop {
            if bp < 2 {
                break;
            }
            let bq = sum - bp;
            out.push_str(&format!("    bits(p) = {}, bits(q) = {}\n", bp, bq));
            if sum / 2 - bp >= 5 {
                break;
            }
            if bp == 2 {
                break;
            }
            bp -= 1;
        }
    }
    // Fermat's reach scales with N's own size: cost is ~|p-q|^2 / sqrt(N)
    // steps, so the window is set relative to bit_len, not a flat constant.
    // The exponent is capped so the budget stays bounded for very large N.
    let fermat_iters: u64 = 1u64 << (bit_len / 6).min(28);
    // bits(gap) reachable at that budget: b^2 ~ 2*sqrt(N)*iters, gap=2b.
    let reachable_gap_bits = (1.0 + 0.5 * (1.0 + 0.5 * bit_len as f64 + (fermat_iters as f64).log2())) as usize;
    let typical_rsa_gap_bits = bit_len / 2; // measured directly off the real corpus: bits(|p-q|) sits within a few bits of bit_len/2 in every one of the 24 real cases

    out.push_str(&format!(
        "\nFermat window scaled to bits(N): {} iterations (2^{}), reaching a gap of\nroughly {} bits. Real RSA-generated pairs run |p-q| near {} bits (measured\nacross the 24-number corpus) — this window does not close that gap, and no\nbit-length-relative window does, since the required iteration count grows\nexponentially faster than the affordable one as bits(N) grows.\n\n",
        fermat_iters, (bit_len / 6).min(28), reachable_gap_bits, typical_rsa_gap_bits
    ));
    out.push_str("split alone doesn't fix p or q, so it runs the search: small-prime peel,\nFermat's method, GPU Montgomery rho (N <= 256 bits), then CPU Pollard's rho.\n\n");

    let (is_prime, _held) = is_prime_miller_rabin(&n);
    if is_prime {
        out.push_str("N is prime (Miller-Rabin, 12 witnesses): no two-factor decomposition.\n");
        return out;
    }
    if n == BigUint::from(1u32) {
        out.push_str("N = 1 is a unit: no two-factor decomposition.\n");
        return out;
    }

    let mut found: Option<(BigUint, BigUint)> = None;
    let mut mechanism = "";

    for sp in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].iter() {
        let spb = BigUint::from(*sp);
        if modulo_via_word(&n, &spb).unwrap() == BigUint::zero() {
            found = Some((spb.clone(), divmod_via_word(&n, &spb).unwrap().0));
            mechanism = "small-prime peel";
            break;
        }
    }
    if found.is_none() {
        if let Some((p, q, rejected, checked)) = fermat_factor(&n, fermat_iters) {
            found = Some((p, q));
            mechanism = "Fermat's method (parity(a) vs parity(b) rejects most candidates before the exact check)";
            out.push_str(&format!(
                "Fermat candidates: {} rejected on parity alone, {} needed the exact perfect-square check\n",
                rejected, checked
            ));
        }
    }
    if found.is_none() && n.bits() <= 2048 {
        if let Some(p) = crate::gpu_rho_ml::factor_once_ml(&n, 0) {
            let q = divmod_via_word(&n, &p).unwrap().0;
            found = Some((p, q));
            mechanism = "GPU-parallel Montgomery Pollard's rho (multi-limb)";
        }
    }
    if found.is_none() {
        if let Some((p, _trace)) = pollard_rho_belnap(&n) {
            let q = divmod_via_word(&n, &p).unwrap().0;
            found = Some((p, q));
            mechanism = "CPU Pollard's rho, Belnap binary-GCD";
        }
    }

    match found {
        Some((p, q)) => {
            let (p, q) = if p <= q { (p, q) } else { (q, p) };
            out.push_str(&format!(
                "found via {}:\np = {}\nq = {}\np × q = N: {}\n",
                mechanism, p, q, multiply_via_word(&p, &q) == n
            ));
        }
        None => {
            out.push_str("no factor found within the search budget.\n");
        }
    }

    out
}

pub fn word() -> String {
    format!(
        "{}\n  word   : {}   period {}\n  boundary: the Z2 parity invariant\n  parity branch ∈ (4) / rejoin ∋ (8): even on ⊤, odd on ⊥",
        ARTIFACT, WORD, PERIOD
    )
}

/// Encode n and read the result back through the instruments: its length, its
/// tri-ancestral verdict as a loop, and whether every cut holds the same
/// verdict. This is the number's native word plus what the Grammar says of it.
pub fn encode_report(n: &str) -> String {
    let w = encode(n);
    if w.is_empty() {
        return format!("native_numeral encode {}: not a valid non-negative integer", n);
    }
    let mut o = format!("native_numeral encode {}:\n  word   : {}\n", n, w);
    match tri_ancestral_word_verdict(&w) {
        Some(v) => o.push_str(&format!("  tri-ancestral verdict (as a loop): {}\n", v)),
        None => o.push_str("  no IMASM glyphs\n"),
    }
    match cycle_landings(&w) {
        Some(l) => {
            let mut distinct: alloc::vec::Vec<&String> = alloc::vec::Vec::new();
            for r in l.iter() { if !distinct.iter().any(|s| *s == r) { distinct.push(r); } }
            o.push_str(&format!(
                "  period {}, {} distinct landing register(s) over the orbit",
                l.len(), distinct.len()
            ));
        }
        None => o.push_str("  no orbit"),
    }
    o
}

/// Payload codec Λ (Lambda) for a two-factor interlaced word D(p,q), distinct
/// from the single-number encode() above. A digram over {≻, ⋈} carries one
/// bit of each factor at once:
///   D(p,q) = ⊢⊣ W_N ∈⊥⊞∋⊙≺⊡⋈⊣  (or ∈⊤⊞∋⊙≺⊡⋈⊣, N=p×q's own parity)
///   W_N    = body over {≻, ⋈} encoding both factors via γ operator
///
/// γ: (p_bit, q_bit) → digram
///   γ(0,0) = ≻≻
///   γ(0,1) = ≻⋈
///   γ(1,0) = ⋈≻
///   γ(1,1) = ⋈⋈
///
/// encode_pq is the forward half: both factors go in MSB-first, the shorter
/// zero-padded at its high end to match the longer, one digram per position.
/// lambda_operator/factor_report are the reverse: split the digram stream back
/// into the two bit lanes and read off p and q.

const PREFIX: &str = "⊢⊣";
const SUFFIX_EVEN: &str = "∈⊤⊞∋⊙≺⊡⋈⊣";
const SUFFIX_ODD: &str = "∈⊥⊞∋⊙≺⊡⋈⊣";

/// γ operator: (p_bit, q_bit) → digram
fn gamma(p: u8, q: u8) -> &'static str {
    match (p, q) {
        (0, 0) => "≻≻",
        (0, 1) => "≻⋈",
        (1, 0) => "⋈≻",
        (1, 1) => "⋈⋈",
        _ => "≻≻",
    }
}

/// γ⁻¹: digram → (p_bit, q_bit)
fn gamma_inv(digram: &str) -> Option<(u8, u8)> {
    match digram {
        "≻≻" => Some((0, 0)),
        "≻⋈" => Some((0, 1)),
        "⋈≻" => Some((1, 0)),
        "⋈⋈" => Some((1, 1)),
        _ => None,
    }
}

/// Extract W_N from the full numeral word D(N).
fn extract_payload_body(word: &str) -> Result<&str, &'static str> {
    if !word.starts_with(PREFIX) {
        return Err("Word must start with ⊢⊣");
    }
    let body_start = PREFIX.len();
    
    let suffix = if word.ends_with(SUFFIX_EVEN) {
        SUFFIX_EVEN
    } else if word.ends_with(SUFFIX_ODD) {
        SUFFIX_ODD
    } else {
        return Err("Word must end with ∈⊤⊞∋⊙≺⊡⋈⊣ or ∈⊥⊞∋⊙≺⊡⋈⊣");
    };
    
    let body_end = word.len() - suffix.len();
    if body_end <= body_start {
        return Err("Empty body");
    }
    
    Ok(&word[body_start..body_end])
}

/// Λ(W_N) = extract factor payload via γ-deinterlacing.
/// Returns (bin_p, bin_q) as binary strings.
pub fn lambda_operator(body: &str) -> (String, String) {
    // Filter to only ≻ and ⋈
    let payload_glyphs: Vec<char> = body.chars().filter(|&c| c == '≻' || c == '⋈').collect();
    
    if payload_glyphs.is_empty() {
        return (String::new(), String::new());
    }
    
    let mut bin_p = String::new();
    let mut bin_q = String::new();
    
    // Split into digrams (pairs of glyphs)
    for i in (0..payload_glyphs.len()).step_by(2) {
        if i + 1 >= payload_glyphs.len() {
            break; // odd length, ignore trailing glyph
        }
        let digram: String = payload_glyphs[i..=i+1].iter().collect();
        if let Some((p_bit, q_bit)) = gamma_inv(&digram) {
            bin_p.push(char::from(b'0' + p_bit));
            bin_q.push(char::from(b'0' + q_bit));
        }
    }
    
    (bin_p, bin_q)
}

/// Reconstruct body from binary lanes using γ.
pub fn reconstruct_body(bin_p: &str, bin_q: &str) -> String {
    let max_len = bin_p.len().max(bin_q.len());
    let mut body = String::new();
    
    for i in 0..max_len {
        let p_bit = bin_p.chars().nth(i).unwrap_or('0') as u8 - b'0';
        let q_bit = bin_q.chars().nth(i).unwrap_or('0') as u8 - b'0';
        body.push_str(gamma(p_bit, q_bit));
    }
    body
}

/// Build D(p,q): the forward half of the Λ codec.
pub fn encode_pq(p: &BigUint, q: &BigUint) -> String {
    let mut bp = bits_from_biguint(p);
    let mut bq = bits_from_biguint(q);
    bp.reverse(); // MSB-first
    bq.reverse();
    let width = bp.len().max(bq.len());
    while bp.len() < width { bp.insert(0, false); }
    while bq.len() < width { bq.insert(0, false); }

    let bin_p: String = bp.iter().map(|&b| if b { '1' } else { '0' }).collect();
    let bin_q: String = bq.iter().map(|&b| if b { '1' } else { '0' }).collect();
    let body = reconstruct_body(&bin_p, &bin_q);

    let n = multiply_via_word(p, q);
    let two = BigUint::from(2u32);
    let suffix = if modulo_via_word(&n, &two).unwrap() == BigUint::zero() { SUFFIX_EVEN } else { SUFFIX_ODD };
    format!("{}{}{}", PREFIX, body, suffix)
}

/// Decode report for a direct D(p,q) word: the reverse half of the Λ codec.
pub fn factor_report(word: &str) -> String {
    let body = match extract_payload_body(word) {
        Ok(b) => b,
        Err(e) => return format!("Error: {}", e),
    };

    let (bin_p, bin_q) = lambda_operator(body);
    let p = BigUint::parse_bytes(bin_p.as_bytes(), 2).unwrap_or_else(BigUint::zero);
    let q = BigUint::parse_bytes(bin_q.as_bytes(), 2).unwrap_or_else(BigUint::zero);
    let product = multiply_via_word(&p, &q);

    // Reconstruct body via γ and check it matches (round trip through the codec)
    let reconstructed = reconstruct_body(&bin_p, &bin_q);
    let body_filtered: String = body.chars().filter(|&c| c == '≻' || c == '⋈').collect();
    let gamma_matches = reconstructed == body_filtered;

    let mut report = String::new();
    report.push_str(&format!("D(p,q) = {}\n", word));
    report.push_str(&format!("W_N    = {} (len={})\n", body, body.len()));
    report.push_str(&format!("Λ(W_N) = bin(p)={}, bin(q)={}\n", bin_p, bin_q));
    report.push_str(&format!("p = {}, q = {}\n", p, q));
    report.push_str(&format!("p × q = {}\n", product));
    report.push_str(&format!("γ-reconstructed = {}\n", reconstructed));
    report.push_str(&format!("γ matches body = {}\n", gamma_matches));

    report
}

/// Build D(p,q), then decode it straight back through Λ, in one call: encode_pq
/// is the forward half, factor_report the reverse, and this runs the round trip
/// live rather than assuming the two agree.
pub fn interlace_report(p_str: &str, q_str: &str) -> String {
    let p: BigUint = match p_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral interlace: '{}' is not a valid non-negative integer", p_str),
    };
    let q: BigUint = match q_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral interlace: '{}' is not a valid non-negative integer", q_str),
    };
    if p.is_zero() || q.is_zero() {
        return "native_numeral interlace: p and q must both be positive".to_string();
    }

    let word = encode_pq(&p, &q);
    let verdict = tri_ancestral_word_verdict(&word).map(|c| c.to_string()).unwrap_or("?".to_string());

    let body = extract_payload_body(&word).unwrap_or("");
    let (bin_p, bin_q) = lambda_operator(body);
    let p2 = BigUint::parse_bytes(bin_p.as_bytes(), 2).unwrap_or_else(BigUint::zero);
    let q2 = BigUint::parse_bytes(bin_q.as_bytes(), 2).unwrap_or_else(BigUint::zero);

    let mut report = format!("D({},{}) = {}\n", p, q, word);
    report.push_str(&format!("word tri-ancestral verdict: {}\n\n", verdict));
    report.push_str(&factor_report(&word));
    report.push_str(&format!("\nround trip matches (p,q): {}\n", p2 == p && q2 == q));
    report
}

/// Modular exponentiation, base^exp mod modulus, walking exp's own bits
/// LSB-first via to_bits_low_first — the same bit vector encode() builds its
/// word from. Square every step; multiply the base in only where the bit is
/// set, exactly the ⊥/odd branch encode() already marks per cell.
pub(crate) fn mod_pow_walk(base: &BigUint, exp_bits: &[bool], modulus: &BigUint) -> BigUint {
    // The whole square-and-multiply walk runs in bit form now, not just
    // each individual step: base and modulus are read into bits once at
    // the top, every multiply and reduce inside the loop is bits_multiply
    // and bits_divmod directly, and the only BigUint conversion left is
    // bits_to_value on the way out. A loop that ran for hundreds of
    // exponent bits used to decode and re-encode at every one of them;
    // now it doesn't touch BigUint until it's actually done.
    let m_bits = word_bits(modulus);
    let mut result: alloc::vec::Vec<u64> = alloc::vec![1u64]; // 1, low-first
    let (_, mut b) = bits_divmod(&word_bits(base), &m_bits);
    for &bit in exp_bits {
        if bit {
            let (_, r) = bits_divmod(&bits_multiply(&result, &b), &m_bits);
            result = r;
        }
        let (_, r2) = bits_divmod(&bits_multiply(&b, &b), &m_bits);
        b = r2;
    }
    bits_to_value(&result)
}

/// Miller-Rabin against the fixed witness set {2,3,5,...,37}, deterministic
/// (not probabilistic) for every n below 3.3×10^24 — far past anything this
/// module will be handed. Each witness's test is mod_pow_walk over n-1's own
/// bits, so primality is read off N's own word representation, not a library
/// call.
///
/// The ob3ect "the dialetheic computation that will find any prime and
/// factor any number" (grounded full, lean_verified, Frobenius T, banked OK)
/// names the shape this loop actually has: ∈ bifurcates into a prime arm and
/// a composite arm, ⊞ holds both live (paradice) until every witness has
/// cleared, ⊤/⊥ only fire at the ends — a single refuting witness collapses
/// straight to composite (⊥), while primality only fires (⊤) once nothing is
/// left held. Returns (is_prime, how many witnesses were held-both before the
/// loop closed one way or the other).
pub(crate) fn is_prime_miller_rabin(n: &BigUint) -> (bool, u32) {
    let zero = BigUint::zero();
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);
    if *n < two { return (false, 0); }
    if *n == two || *n == three { return (true, 0); }
    if modulo_via_word(n, &two).unwrap() == zero { return (false, 0); }

    // Trial-divide by the same twelve small primes used as witnesses below,
    // before paying for a single modular exponentiation. winding_period's
    // u64 is_prime_mr already does exactly this; this function never did,
    // found by comparing the two independently once vox located the same
    // witness constant in both. Roughly 84% of random odd composites carry
    // a factor at or below 37, so this turns the common case into a handful
    // of cheap word-native remainders instead of twelve full modpows.
    for &p_val in &[3u64, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let p = BigUint::from(p_val);
        if modulo_via_word(n, &p).unwrap() == zero { return (*n == p, 0); }
    }

    let n_minus_1 = subtract_via_word(n, &one).unwrap();
    let mut d = n_minus_1.clone();
    let mut r = 0u32;
    while modulo_via_word(&d, &two).unwrap() == zero {
        d = halve_even_by_word(&d).unwrap();
        r += 1;
    }

    let witnesses: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut held = 0u32;
    'witness: for &a_val in witnesses.iter() {
        let a = BigUint::from(a_val);
        if a >= *n { continue; }
        let exp_bits = to_bits_low_first(&d);
        let mut x = mod_pow_walk(&a, &exp_bits, n);
        if x == one || x == n_minus_1 { held += 1; continue; }
        for _ in 0..r.saturating_sub(1) {
            x = modulo_via_word(&multiply_via_word(&x, &x), n).unwrap();
            if x == n_minus_1 { held += 1; continue 'witness; }
        }
        return (false, held); // this witness refutes primality outright
    }
    (true, held) // every witness held; nothing left both-live, collapses to prime
}

/// Stein's binary GCD, with every step's branch read as a Belnap value from
/// two independent parity bits — a odd is positive evidence, b odd is
/// negative evidence, the same way T/F/B/N arise from two independent
/// evidence bits in Belnap's own construction:
///   N  both even   — no odd evidence yet; the shared factor of 2 comes off here
///   T  a odd alone — shift b
///   F  b odd alone — shift a
///   B  both odd    — the hard case, real subtraction work
/// Returns gcd(a,b) and the branch trace, so the computation's own dialetheic
/// character is visible rather than asserted.
fn belnap_gcd(a: &BigUint, b: &BigUint) -> (BigUint, Vec<char>) {
    let mut trace = Vec::new();
    if a.is_zero() { return (b.clone(), trace); }
    if b.is_zero() { return (a.clone(), trace); }

    // The whole descent runs in bit form: a and b are read into bits once
    // here, and nothing gets decoded back to BigUint until the loop is
    // actually done, the same discipline mod_pow_walk and divmod now use.
    let mut bits_a = word_bits(a);
    let mut bits_b = word_bits(b);
    let mut shift = 0u32;

    loop {
        if bits_eq(&bits_a, &bits_b) { break; }
        let a_odd = bits_is_odd(&bits_a);
        let b_odd = bits_is_odd(&bits_b);
        match (a_odd, b_odd) {
            // Every halving below reads its result off the operand's own
            // bits (bits_halve), not a division — the branch's own parity
            // check already proved the operand is even, so the unwrap is
            // a proven invariant, not a guess.
            (false, false) => {
                trace.push('N');
                bits_a = bits_halve(&bits_a).unwrap();
                bits_b = bits_halve(&bits_b).unwrap();
                shift += 1;
            }
            (true, false) => { trace.push('T'); bits_b = bits_halve(&bits_b).unwrap(); }
            (false, true) => { trace.push('F'); bits_a = bits_halve(&bits_a).unwrap(); }
            (true, true) => {
                trace.push('B');
                // Both the subtraction and the halving read off bits
                // directly: bits_subtract walks both operands' bits with
                // a carried borrow, and the result of two odds subtracted
                // is even by construction, so the halving that follows is
                // the same proven case as above.
                if bits_gt(&bits_a, &bits_b) {
                    let diff = bits_subtract(&bits_a, &bits_b).unwrap();
                    bits_a = bits_halve(&diff).unwrap();
                } else {
                    let diff = bits_subtract(&bits_b, &bits_a).unwrap();
                    let na = bits_halve(&diff).unwrap();
                    bits_b = bits_a.clone();
                    bits_a = na;
                }
            }
        }
    }
    for _ in 0..shift { bits_a = bits_double(&bits_a); }
    (bits_to_value(&bits_a), trace)
}

/// Pollard's rho, Floyd cycle-finding, where the periodic gcd check is
/// belnap_gcd rather than a library call: the moment a factor surfaces is a
/// real branch trace, not a hidden gcd. Returns a nontrivial factor and the
/// trace of the winning gcd call.
fn pollard_rho_belnap(n: &BigUint) -> Option<(BigUint, Vec<char>)> {
    let zero = BigUint::zero();
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    if modulo_via_word(n, &two).unwrap() == zero { return Some((two, vec!['N'])); }

    let mut c = BigUint::from(1u32);
    for _attempt in 0..20 {
        // Every step of Pollard's own map reads off the two operands' words:
        // multiply_via_word and add_via_word for x*x + c, modulo_via_word for
        // the reduction. n is never zero here (n >= 2 is checked before this
        // function runs), so the unwrap is a proven invariant, the same
        // convention every word operation in this file uses.
        let f = |x: &BigUint| -> BigUint {
            modulo_via_word(&add_via_word(&multiply_via_word(x, x), &c), n).unwrap()
        };

        let mut x = BigUint::from(2u32);
        let mut y = BigUint::from(2u32);
        let mut d = BigUint::from(1u32);
        let mut trace: Vec<char> = Vec::new();
        let mut iters = 0u64;

        while d == one {
            x = f(&x);
            y = f(&f(&y));
            // The comparison deciding which order to subtract in is the
            // one place ordering, not one of the four proven operations,
            // enters -- the same boundary belnap_gcd's own `a > b` draws.
            let diff = if x > y { subtract_via_word(&x, &y).unwrap() } else { subtract_via_word(&y, &x).unwrap() };
            if diff.is_zero() { break; }
            let (g, tr) = belnap_gcd(&diff, n);
            d = g;
            trace = tr;
            iters += 1;
            if iters > 1_000_000 { break; }
        }

        if d > one && d < *n {
            return Some((d, trace));
        }
        c = add_via_word(&c, &one);
    }
    None
}

/// Factor a decimal N. Miller-Rabin (mod_pow_walk over N-1's own bits)
/// decides primality outright rather than reporting a search giving up.
/// Composite N: small primes are peeled directly, then Pollard's rho pulls a
/// factor via belnap_gcd — the branch trace of the winning gcd call is
/// printed, so the dialetheic content is read, not claimed.
pub fn factor_decimal(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral factor: '{}' is not a valid non-negative integer", n_str),
    };

    if n.is_zero() {
        return "N = 0\nD(0) = ⊢⊙⊡⊣\np = 0, q = 0\np × q = 0\n".to_string();
    }
    if n == BigUint::from(1u32) {
        return format!("N = 1\nD(1) = {}\n1 is a unit; it has no prime factorization\n", encode("1"));
    }

    let word = encode(n_str);

    let (is_prime, held) = is_prime_miller_rabin(&n);
    if is_prime {
        return format!(
            "N = {}\nD(N) = {}\n\nMiller-Rabin, walking N-1's own bits through mod_pow_walk against 12 fixed witnesses (deterministic below 3.3×10^24).\n{} witness(es) held both prime and composite live (paradice) before the last one cleared; nothing was left held, so the state collapses to prime.\nN is prime.\n",
            n_str, word, held
        );
    }

    for sp in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].iter() {
        let spb = BigUint::from(*sp);
        if modulo_via_word(&n, &spb).unwrap() == BigUint::zero() {
            let q = divmod_via_word(&n, &spb).unwrap().0;
            return format!(
                "N = {}\nD(N) = {}\n\nfactor (small-prime peel):\np = {}\nD(p) = {}\nq = {}\nD(q) = {}\n\nVerification:\n  p × q = N: {}\n",
                n_str, word, spb, encode(&spb.to_string()), q, encode(&q.to_string()), multiply_via_word(&spb, &q) == n
            );
        }
    }

    if n.bits() <= 2048 {
        if let Some(p) = crate::gpu_rho_ml::factor_once_ml(&n, 0) {
            let q = divmod_via_word(&n, &p).unwrap().0;
            let (p, q) = if p <= q { (p, q) } else { (q, p) };
            return format!(
                "N = {}\nD(N) = {}\n\nfactor (GPU-parallel Montgomery Pollard's rho, gpu_rho_ml.rs, multi-limb CIOS arithmetic verified against BigUint):\np = {}\nD(p) = {}\nq = {}\nD(q) = {}\n\nVerification:\n  p × q = N: {}\n",
                n_str, word, p, encode(&p.to_string()), q, encode(&q.to_string()), multiply_via_word(&p, &q) == n
            );
        }
    }

    match pollard_rho_belnap(&n) {
        Some((p, trace)) => {
            let q = divmod_via_word(&n, &p).unwrap().0;
            let (p, q) = if p <= q { (p, q) } else { (q, p) };
            let trace_str: String = trace.iter().collect();
            let mut report = String::new();
            report.push_str(&format!("N = {}\n", n_str));
            report.push_str(&format!("D(N) = {}\n", word));
            report.push_str("\nfactor (Pollard's rho, gcd via the Belnap binary-GCD walk):\n");
            report.push_str(&format!("p = {}\n", p));
            report.push_str(&format!("D(p) = {}\n", encode(&p.to_string())));
            report.push_str(&format!("q = {}\n", q));
            report.push_str(&format!("D(q) = {}\n", encode(&q.to_string())));
            report.push_str(&format!(
                "\nthe winning gcd's own branch trace ({} steps, N=neither-odd, T=a-odd, F=b-odd, B=both-odd):\n  {}\n",
                trace_str.chars().count(),
                trace_str
            ));
            report.push_str("\nVerification:\n");
            report.push_str(&format!("  p × q = N: {}\n", multiply_via_word(&p, &q) == n));
            report
        }
        None => format!(
            "N = {}\nD(N) = {}\n\nMiller-Rabin said composite. GPU Montgomery rho (if N ≤ 256 bits) and CPU Pollard's rho (20 constants, Belnap gcd) both found no split.\nN needs a larger rho budget.\n",
            n_str, word
        ),
    }
}

/// Report for is_prime_miller_rabin on its own, so it can be run directly
/// against a given decimal integer rather than only from inside factor_decimal.
pub fn primetest_report(n_str: &str) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral primetest: '{}' is not a valid non-negative integer", n_str),
    };
    let (is_prime, held) = is_prime_miller_rabin(&n);
    format!(
        "N = {}\nMiller-Rabin, 12 fixed witnesses: {} held both prime and composite live before the loop closed.\nverdict: {}\n",
        n_str, held, if is_prime { "prime" } else { "composite" }
    )
}

/// Report for belnap_gcd on its own, so the branch trace can be read for any
/// pair of decimal integers directly, not only the one call rho happens to win on.
pub fn gcd_report(a_str: &str, b_str: &str) -> String {
    let a: BigUint = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral gcd: '{}' is not a valid non-negative integer", a_str),
    };
    let b: BigUint = match b_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("native_numeral gcd: '{}' is not a valid non-negative integer", b_str),
    };
    let (g, trace) = belnap_gcd(&a, &b);
    let trace_str: String = trace.iter().collect();
    let mut counts = [0u32; 4];
    for &c in trace.iter() {
        match c {
            'N' => counts[0] += 1,
            'T' => counts[1] += 1,
            'F' => counts[2] += 1,
            'B' => counts[3] += 1,
            _ => {}
        }
    }
    format!(
        "gcd({}, {}) = {}\nbranch trace ({} steps, N=neither-odd T=a-odd F=b-odd B=both-odd): {}\ncounts: N={} T={} F={} B={}\n",
        a_str, b_str, g, trace_str.chars().count(), trace_str, counts[0], counts[1], counts[2], counts[3]
    )
}

pub(crate) fn pow2(k: usize) -> BigUint {
    let mut r = BigUint::from(1u32);
    for _ in 0..k { r = double_via_word(&r); }
    r
}

struct UnbraidBudget {
    nodes: u64,
    cap: u64,
}

/// The other arm: build p and q from the TOP bit down instead of the bottom
/// bit up. Fixing the leading bits of p and q down to position `pos+1`
/// bounds the true p in [p_fixed, p_fixed+2^(pos+1)) and q likewise — both
/// bounds exact, product monotone in each factor, so p*q is bounded exactly
/// in [p_fixed*q_fixed, (p_fixed+2^(pos+1)-1)*(q_fixed+2^(pos+1)-1)]. N
/// outside that interval means this prefix is impossible: pruned before any
/// lower bit is ever guessed, unlike the bottom-up arm, which only checks
/// c==0 once every bit is already fixed. `pos` walks from high to low; both
/// leading bits and both bit-0s (odd) are forced, never branched.
fn range_prune_unbraid(
    n: &BigUint,
    p_bits: usize,
    q_bits: usize,
    pos: i64,
    p_fixed: BigUint,
    q_fixed: BigUint,
    budget: &mut UnbraidBudget,
    stop: &core::sync::atomic::AtomicBool,
) -> Option<(BigUint, BigUint)> {
    budget.nodes += 1;
    if budget.nodes > budget.cap || stop.load(core::sync::atomic::Ordering::Relaxed) {
        return None;
    }
    if pos < 0 {
        return if multiply_via_word(&p_fixed, &q_fixed) == *n { Some((p_fixed, q_fixed)) } else { None };
    }
    let k = (pos + 1) as usize;
    let span = subtract_via_word(&pow2(k), &BigUint::from(1u32)).unwrap();
    let p_max = add_via_word(&p_fixed, &span);
    let q_max = add_via_word(&q_fixed, &span);
    let lo = multiply_via_word(&p_fixed, &q_fixed);
    let hi = multiply_via_word(&p_max, &q_max);
    if *n < lo || *n > hi {
        return None;
    }
    let p_has_free_bit = (pos as usize) < p_bits.saturating_sub(1) && pos > 0;
    let q_has_free_bit = (pos as usize) < q_bits.saturating_sub(1) && pos > 0;
    let p_choices: &[u32] = if p_has_free_bit { &[0, 1] } else if pos == 0 { &[1] } else { &[0] };
    let q_choices: &[u32] = if q_has_free_bit { &[0, 1] } else if pos == 0 { &[1] } else { &[0] };
    let step = pow2(pos as usize);
    for &pb in p_choices {
        let new_p = if pb == 1 { add_via_word(&p_fixed, &step) } else { p_fixed.clone() };
        for &qb in q_choices {
            let new_q = if qb == 1 { add_via_word(&q_fixed, &step) } else { q_fixed.clone() };
            if let Some(r) = range_prune_unbraid(n, p_bits, q_bits, pos - 1, new_p.clone(), new_q, budget, stop) {
                return Some(r);
            }
            if budget.nodes > budget.cap || stop.load(core::sync::atomic::Ordering::Relaxed) { return None; }
        }
    }
    None
}

/// The real square-root cut on the search above. p_k*q_k ≡ N (mod 2^k) is an
/// invariant Hensel lifts one bit at a time: track c_k = (p_k*q_k − N)/2^k
/// exactly (both odd below 2^k, so it's always an integer once the invariant
/// holds). Extending by one bit needs p_bit·q_k + q_bit·p_k ≡ −c_k (mod 2);
/// since p_k, q_k are both odd once k≥1, that reduces to p_bit XOR q_bit
/// fixed by c_k's own parity — CHOOSE p_bit (2-way) and q_bit is FORCED, not
/// guessed. So the two unknowns cost one bit of branching, not two: the
/// search is 2^min(p_bits,q_bits), the square root of the naive 2^(p_bits+q_bits)
/// space the range search above pays for guessing both sides independently.
/// Once the shorter factor's bits run out it stays fixed, and every remaining
/// bit of the longer factor is forced by the same parity equation — no
/// branching left at all, which is exactly "p = N / q" arriving one bit at a
/// time instead of by division.
fn hensel_unbraid(
    n: &BigInt,
    p_bits: usize,
    q_bits: usize,
    k: usize,
    p_val: BigUint,
    q_val: BigUint,
    c: BigInt,
    budget: &mut UnbraidBudget,
    stop: &core::sync::atomic::AtomicBool,
) -> Option<(BigUint, BigUint)> {
    budget.nodes += 1;
    if budget.nodes > budget.cap || stop.load(core::sync::atomic::Ordering::Relaxed) {
        return None;
    }
    let short = p_bits.min(q_bits);
    let long = p_bits.max(q_bits);
    if k == long {
        // c==0 is proven to mean p*q==N exactly whenever the recursion's
        // own exactness invariant held at every step -- but that proof
        // assumes N odd, and the N=100 false positive showed what happens
        // when a caller hands this an N where the seed carry was never a
        // real integer to begin with: the recursion still runs to
        // completion and can still land on c=0 by accident. Checking the
        // product directly here, the same defense range_prune_unbraid
        // already has in its own base case, means a caller mistake never
        // turns into a wrong answer, only a missed one.
        return if c == BigInt::from(0) && multiply_via_word(&p_val, &q_val) == *n.magnitude() {
            Some((p_val, q_val))
        } else {
            None
        };
    }

    // c's sign never changes whether it's even or odd -- 2 divides c iff 2
    // divides |c| -- so parity reads straight off the magnitude's own
    // lowest bit via the same is_odd_by_word every other parity check in
    // this file uses, rather than a signed remainder.
    let c_parity: u32 = if is_odd_by_word(c.magnitude()) { 1 } else { 0 };
    let p_still_open = k < p_bits;
    let q_still_open = k < q_bits;

    if k < short {
        // both sides still open — branch p_bit, derive q_bit from parity
        let step = pow2(k);
        for &p_bit in &[0u32, 1u32] {
            let q_bit = (c_parity + p_bit) % 2;
            let new_p = if p_bit == 1 { add_via_word(&p_val, &step) } else { p_val.clone() };
            let new_q = if q_bit == 1 { add_via_word(&q_val, &step) } else { q_val.clone() };
            // p_bit·q_val + q_bit·p_val + p_bit·q_bit·2^k, but p_bit and
            // q_bit are each 0 or 1, so every term is either the named
            // magnitude or nothing -- read directly off the bits instead
            // of multiplying by 0 or 1 through BigInt.
            let mut magnitude = BigUint::zero();
            if p_bit == 1 { magnitude = add_via_word(&magnitude, &q_val); }
            if q_bit == 1 { magnitude = add_via_word(&magnitude, &p_val); }
            if p_bit == 1 && q_bit == 1 { magnitude = add_via_word(&magnitude, &step); }
            let numerator = signed_add_via_word(&c, &magnitude);
            let new_c = halve_exact_signed_via_word(&numerator);
            if let Some(r) = hensel_unbraid(n, p_bits, q_bits, k + 1, new_p, new_q, new_c, budget, stop) {
                return Some(r);
            }
            if budget.nodes > budget.cap || stop.load(core::sync::atomic::Ordering::Relaxed) { return None; }
        }
        None
    } else {
        // one side is fully fixed now — the other's next bit is FORCED, no branch
        let step = pow2(k);
        let (bit, new_p, new_q) = if p_still_open {
            let bit = c_parity; // q's contribution is fixed (q_bit=0 beyond q_bits), so p_bit ≡ c_parity
            (bit, if bit == 1 { add_via_word(&p_val, &step) } else { p_val.clone() }, q_val.clone())
        } else if q_still_open {
            let bit = c_parity;
            (bit, p_val.clone(), if bit == 1 { add_via_word(&q_val, &step) } else { q_val.clone() })
        } else {
            return if c == BigInt::from(0) { Some((p_val, q_val)) } else { None };
        };
        let numerator = if bit == 1 {
            let contribution = if p_still_open { &q_val } else { &p_val };
            signed_add_via_word(&c, contribution)
        } else {
            c.clone()
        };
        let new_c = halve_exact_signed_via_word(&numerator);
        hensel_unbraid(n, p_bits, q_bits, k + 1, new_p, new_q, new_c, budget, stop)
    }
}

/// Try every (p_bits, q_bits) split consistent with N's own bit length, real
/// unbraiding each via the Hensel lift above. Reports the factor pair found,
/// or the exact node count spent if the budget ran out first — a measured
/// count, not an estimate of how close it got.
pub fn unbraid_report(n_str: &str, node_cap: u64) -> String {
    // Accepts encode()'s own output too, not only a plain decimal string —
    // the braiding search should take what the numeral mapping actually
    // produces, not force a caller to keep the decimal string around
    // separately just to hand N to unbraid.
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => match decode(n_str.trim()) {
            Some(v) => v,
            None => return format!(
                "native_numeral unbraid: '{}' is not a valid non-negative integer or native-numeral word",
                n_str
            ),
        },
    };
    // Every report line below prints n (the decoded value), never n_str —
    // so the output reads the same real number whichever format it arrived
    // in, instead of echoing a raw glyph word back where a decimal belongs.
    if n.is_zero() || n == BigUint::from(1u32) {
        return format!("N = {} has no two-factor unbraid\n", n);
    }
    // Both braid arms below force bit 0 to 1 on p and q by construction
    // (hensel_unbraid starts both at 1, range_prune_unbraid forces the
    // lowest bit to 1 the same way) -- the ansatz is odd x odd, always.
    // Odd x odd is always odd, so an even N has no witness anywhere in
    // that search space, not merely a hard-to-reach one. Handing an even
    // N to hensel_unbraid anyway breaks its own invariant at the first
    // step (c0 = (1-N)/2 needs 1-N even, which needs N odd) and the
    // recursion can still land on c=0 by the accident of a corrupted
    // computation, reporting a factor pair that doesn't actually multiply
    // to N. Peel the factor of 2 up front instead of letting the braid
    // answer a question outside its own domain.
    let two = BigUint::from(2u32);
    if modulo_via_word(&n, &two).unwrap() == BigUint::zero() {
        let q = divmod_via_word(&n, &two).unwrap().0;
        if q == BigUint::from(1u32) {
            // N=2 is the one number where "even" and "prime" coincide: q=1
            // is a unit, not a factor, so this isn't a real two-factor
            // split any more than N=97 has one -- say so the same way the
            // odd-prime fallback below does, not as a found pair.
            return format!("N = 2\nN is prime and even (the only such N) -- no two-factor unbraid exists, definitionally, not merely unfound.\n");
        }
        return format!(
            "N = {}\nN is even: both braid arms force p and q odd by construction, so neither can ever reach a factor of 2 -- peeled directly instead.\np = 2\nq = {}\np × q = N: {}\n",
            n, q, multiply_via_word(&two, &q) == n
        );
    }
    let n_signed = BigInt::from(n.clone());
    let total_bits = n.bits() as usize;
    let mut total_nodes = 0u64;
    // Balanced splits first: a real semiprime's factors sit near bits(N)/2
    // each, so start at the middle and converge downward through
    // increasingly unbalanced splits, rather than starting at p_bits=2 —
    // splits small-prime peel already covers — and burning the budget
    // before ever reaching the realistic case.
    for p_bits in (2..=(total_bits / 2 + 1)).rev() {
        let q_bits_candidates = [total_bits + 1 - p_bits, total_bits - p_bits];
        for &q_bits in q_bits_candidates.iter() {
            if q_bits < p_bits || q_bits == 0 {
                continue;
            }
            let share = node_cap.saturating_sub(total_nodes);
            if share == 0 {
                return format!(
                    "N = {}\nunbraid exhausted the node budget ({}) before finishing every split.\n",
                    n, node_cap
                );
            }
            // Both arms of the same identity, run as real concurrent threads,
            // not one after the other: μ (build p*q up from the bottom bit
            // by 2-adic Hensel lift, the δ-split-outward arm already there)
            // and its mirror (narrow p and q down from the top bit by exact
            // interval pruning, the μ-fuse-inward arm just added). Each gets
            // its own full share of the budget rather than splitting it, so
            // running both costs wall-time parallelism, not search coverage.
            // Shared stop flag: whichever arm lands first sets it, so the
            // other actually cuts short instead of burning its own full
            // budget after the answer is already known. Without this the
            // two threads only shared a budget NUMBER, never a live signal —
            // a fast win on one side left the other running to completion
            // pointlessly.
            let stop = std::sync::Arc::new(core::sync::atomic::AtomicBool::new(false));
            let (bottom_result, bottom_nodes, top_result, top_nodes) = std::thread::scope(|scope| {
                let n_signed_ref = &n_signed;
                let stop_b = stop.clone();
                let bottom = scope.spawn(move || {
                    let mut b = UnbraidBudget { nodes: 0, cap: share };
                    let p0 = BigUint::from(1u32);
                    let q0 = BigUint::from(1u32);
                    // c0 = (1 - N) / 2, truncating toward zero same as BigInt's
                    // own Div: N >= 2 here (checked above), so 1 - N is always
                    // negative or zero, and its magnitude is exactly N - 1 --
                    // subtract_via_word gives that directly, divmod_via_word's
                    // quotient gives the floor-divide-by-2 on that nonnegative
                    // magnitude, which is truncation since magnitudes are
                    // never negative. from_biguint normalizes a zero magnitude
                    // to NoSign on its own, matching N=2 giving c0=0 exactly.
                    let n_minus_1 = subtract_via_word(n_signed_ref.magnitude(), &BigUint::from(1u32)).unwrap();
                    let c0 = BigInt::from_biguint(num_bigint::Sign::Minus, divmod_via_word(&n_minus_1, &BigUint::from(2u32)).unwrap().0);
                    let r = hensel_unbraid(n_signed_ref, p_bits, q_bits, 1, p0, q0, c0, &mut b, &stop_b);
                    if r.is_some() { stop_b.store(true, core::sync::atomic::Ordering::Relaxed); }
                    (r, b.nodes)
                });
                let n_ref = &n;
                let stop_t = stop.clone();
                let top = scope.spawn(move || {
                    let mut b = UnbraidBudget { nodes: 0, cap: share };
                    let start_pos = (p_bits.max(q_bits) as i64) - 2;
                    let p_hi = pow2(p_bits - 1);
                    let q_hi = pow2(q_bits - 1);
                    let r = range_prune_unbraid(n_ref, p_bits, q_bits, start_pos, p_hi, q_hi, &mut b, &stop_t);
                    if r.is_some() { stop_t.store(true, core::sync::atomic::Ordering::Relaxed); }
                    (r, b.nodes)
                });
                let (br, bn) = bottom.join().unwrap();
                let (tr, tn) = top.join().unwrap();
                (br, bn, tr, tn)
            });
            total_nodes += bottom_nodes + top_nodes;
            if let Some((p, q)) = bottom_result {
                return format!(
                    "N = {}\nunbraid found p={} bits, q={} bits after {} node(s) (bottom-up arm, {} bottom / {} top):\np = {}\nq = {}\np × q = N: {}\n",
                    n, p_bits, q_bits, total_nodes, bottom_nodes, top_nodes, p, q, multiply_via_word(&p, &q) == n
                );
            }
            if let Some((p, q)) = top_result {
                return format!(
                    "N = {}\nunbraid found p={} bits, q={} bits after {} node(s) (top-down arm, {} bottom / {} top):\np = {}\nq = {}\np × q = N: {}\n",
                    n, p_bits, q_bits, total_nodes, bottom_nodes, top_nodes, p, q, multiply_via_word(&p, &q) == n
                );
            }
        }
    }
    // "No factor found" is not one claim, it's two, and this search alone
    // can't tell them apart: N could be provably prime (no two-factor
    // unbraid EXISTS, definitionally) or genuinely composite with a factor
    // this budget-limited search just never reached. Miller-Rabin (already
    // built, 12 fixed witnesses, deterministic below 3.3e24) resolves which
    // one actually happened instead of leaving both readings open.
    let (is_prime, held) = is_prime_miller_rabin(&n);
    let det_bound = BigUint::parse_bytes(b"3300000000000000000000000", 10).unwrap();
    if is_prime {
        format!(
            "N = {}\nunbraid exhausted every (p_bits, q_bits) split within {} node(s) total, no factor found.\nMiller-Rabin, 12 fixed witnesses ({} held both live before closing): N is prime{} — no two-factor unbraid exists, definitionally, not merely unfound.\n",
            n, total_nodes, held,
            if n < det_bound { " (deterministic below 3.3e24)" } else { " (probabilistic at this size)" }
        )
    } else {
        format!(
            "N = {}\nunbraid exhausted every (p_bits, q_bits) split within {} node(s) total, no factor found.\nMiller-Rabin, 12 fixed witnesses ({} held both live before closing): N is composite — a real factor pair exists, this search just did not reach it within the node budget.\n",
            n, total_nodes, held
        )
    }
}

fn bits_from_biguint(n: &BigUint) -> Vec<bool> {
    let mut bits = to_bits_low_first(n);
    if bits.is_empty() { bits.push(false); }
    bits
}

#[cfg(test)]
mod halve_via_word_tests {
    use super::*;

    #[test]
    fn zero_halves_to_zero() {
        assert_eq!(halve_even_by_word(&BigUint::zero()), Some(BigUint::zero()));
    }

    #[test]
    fn odd_numbers_are_rejected() {
        for n in [1u32, 3, 7, 91, 12345] {
            assert_eq!(halve_even_by_word(&BigUint::from(n)), None, "n={}", n);
        }
    }

    #[test]
    fn small_even_numbers_match_ordinary_division() {
        for n in [2u32, 4, 6, 8, 10, 12, 100, 1000, 999998] {
            let n = BigUint::from(n);
            let expected = &n / BigUint::from(2u32); // ordinary arithmetic, verification only
            assert_eq!(halve_even_by_word(&n), Some(expected), "n={}", n);
        }
    }

    #[test]
    fn large_even_number_matches_ordinary_division() {
        let n: BigUint = "246913578024691357802469135780246913578024691357802"
            .parse().unwrap();
        assert!((&n % BigUint::from(2u32)).is_zero());
        let expected = &n / BigUint::from(2u32);
        assert_eq!(halve_even_by_word(&n), Some(expected));
    }

    #[test]
    fn repeated_halving_walks_all_the_way_down_a_power_of_two() {
        let mut n = BigUint::from(2u32).pow(20);
        loop {
            let expected = &n / BigUint::from(2u32);
            let got = halve_even_by_word(&n);
            assert_eq!(got, Some(expected.clone()));
            n = expected;
            if n.is_zero() { break; }
            if (&n % BigUint::from(2u32)) != BigUint::zero() { break; }
        }
    }
}

#[cfg(test)]
mod subtract_via_word_tests {
    use super::*;

    #[test]
    fn a_less_than_b_is_none() {
        assert_eq!(subtract_via_word(&BigUint::from(3u32), &BigUint::from(9u32)), None);
    }

    #[test]
    fn equal_operands_give_zero() {
        let n = BigUint::from(12345u32);
        assert_eq!(subtract_via_word(&n, &n), Some(BigUint::zero()));
    }

    #[test]
    fn zero_minus_zero_is_zero() {
        assert_eq!(subtract_via_word(&BigUint::zero(), &BigUint::zero()), Some(BigUint::zero()));
    }

    #[test]
    fn small_pairs_match_ordinary_subtraction() {
        let pairs = [(9u32, 3u32), (17, 5), (1071, 462), (100, 1), (128, 127), (256, 1)];
        for (a, b) in pairs {
            let (a, b) = (BigUint::from(a), BigUint::from(b));
            let expected = &a - &b;
            assert_eq!(subtract_via_word(&a, &b), Some(expected), "a={} b={}", a, b);
        }
    }

    #[test]
    fn a_single_borrow_propagates_through_a_long_run_of_zero_bits() {
        // 2^40 - 1: every one of the low 40 bits has to borrow from the
        // single set bit at the top, the longest possible borrow chain at
        // this width.
        let a = BigUint::from(2u32).pow(40);
        let b = BigUint::from(1u32);
        let expected = &a - &b;
        assert_eq!(subtract_via_word(&a, &b), Some(expected));
    }

    #[test]
    fn large_pair_matches_ordinary_subtraction() {
        let a: BigUint = "987654321098765432109876543210".parse().unwrap();
        let b: BigUint = "123456789012345678901234567890".parse().unwrap();
        let expected = &a - &b;
        assert_eq!(subtract_via_word(&a, &b), Some(expected));
    }
}

#[cfg(test)]
mod double_via_word_tests {
    use super::*;

    #[test]
    fn zero_doubles_to_zero() {
        assert_eq!(double_via_word(&BigUint::zero()), BigUint::zero());
    }

    #[test]
    fn small_numbers_match_ordinary_multiplication() {
        for n in [1u32, 2, 3, 7, 100, 999999] {
            let n = BigUint::from(n);
            let expected = &n * BigUint::from(2u32);
            assert_eq!(double_via_word(&n), expected, "n={}", n);
        }
    }

    #[test]
    fn large_number_matches_ordinary_multiplication() {
        let n: BigUint = "123456789012345678901234567890123456789".parse().unwrap();
        let expected = &n * BigUint::from(2u32);
        assert_eq!(double_via_word(&n), expected);
    }

    #[test]
    fn doubling_and_halving_are_inverse() {
        for n in [0u32, 1, 2, 41, 999999] {
            let n = BigUint::from(n);
            let doubled = double_via_word(&n);
            assert_eq!(halve_even_by_word(&doubled), Some(n));
        }
    }

    #[test]
    fn repeated_doubling_matches_powers_of_two() {
        let mut n = BigUint::from(1u32);
        for k in 1..30u32 {
            n = double_via_word(&n);
            assert_eq!(n, BigUint::from(2u32).pow(k));
        }
    }
}

#[cfg(test)]
mod is_odd_by_word_tests {
    use super::*;

    #[test]
    fn matches_ordinary_parity() {
        for n in [0u32, 1, 2, 3, 4, 100, 101, 999999, 999998] {
            let n = BigUint::from(n);
            let expected = (&n % BigUint::from(2u32)) != BigUint::zero();
            assert_eq!(is_odd_by_word(&n), expected, "n={}", n);
        }
    }
}

#[cfg(test)]
mod add_via_word_tests {
    use super::*;

    #[test]
    fn zero_plus_zero_is_zero() {
        assert_eq!(add_via_word(&BigUint::zero(), &BigUint::zero()), BigUint::zero());
    }

    #[test]
    fn small_pairs_match_ordinary_addition() {
        let pairs = [(9u32, 3u32), (17, 5), (1071, 462), (100, 1), (127, 1), (255, 255)];
        for (a, b) in pairs {
            let (a, b) = (BigUint::from(a), BigUint::from(b));
            let expected = &a + &b;
            assert_eq!(add_via_word(&a, &b), expected, "a={} b={}", a, b);
        }
    }

    #[test]
    fn a_carry_propagates_through_a_long_run_of_one_bits() {
        // (2^40 - 1) + 1: every one of the low 40 bits carries into the
        // next, the longest possible carry chain at this width, and it
        // has to produce one new leading bit past the original width.
        let a = &BigUint::from(2u32).pow(40) - &BigUint::from(1u32);
        let b = BigUint::from(1u32);
        let expected = &a + &b;
        assert_eq!(add_via_word(&a, &b), expected);
    }

    #[test]
    fn large_pair_matches_ordinary_addition() {
        let a: BigUint = "123456789012345678901234567890".parse().unwrap();
        let b: BigUint = "987654321098765432109876543210".parse().unwrap();
        let expected = &a + &b;
        assert_eq!(add_via_word(&a, &b), expected);
    }

    #[test]
    fn addition_and_subtraction_are_inverse() {
        for (a, b) in [(9u32, 3u32), (100, 1), (999999, 1)] {
            let (a, b) = (BigUint::from(a), BigUint::from(b));
            let sum = add_via_word(&a, &b);
            assert_eq!(subtract_via_word(&sum, &b), Some(a));
        }
    }
}

#[cfg(test)]
mod multiply_via_word_tests {
    use super::*;

    #[test]
    fn anything_times_zero_is_zero() {
        assert_eq!(multiply_via_word(&BigUint::from(12345u32), &BigUint::zero()), BigUint::zero());
        assert_eq!(multiply_via_word(&BigUint::zero(), &BigUint::from(12345u32)), BigUint::zero());
    }

    #[test]
    fn small_pairs_match_ordinary_multiplication() {
        let pairs = [(6u32, 7u32), (17, 5), (1071, 462), (100, 100), (255, 255), (1, 999999)];
        for (a, b) in pairs {
            let (a, b) = (BigUint::from(a), BigUint::from(b));
            let expected = &a * &b;
            assert_eq!(multiply_via_word(&a, &b), expected, "a={} b={}", a, b);
        }
    }

    #[test]
    fn matches_doubling_for_multiply_by_two() {
        for n in [0u32, 1, 41, 999999] {
            let n = BigUint::from(n);
            assert_eq!(multiply_via_word(&n, &BigUint::from(2u32)), double_via_word(&n));
        }
    }

    #[test]
    fn large_pair_matches_ordinary_multiplication() {
        let a: BigUint = "123456789012345678901234567890".parse().unwrap();
        let b: BigUint = "987654321098765432109876543210".parse().unwrap();
        let expected = &a * &b;
        assert_eq!(multiply_via_word(&a, &b), expected);
    }

    #[test]
    fn multiplication_is_commutative_here_too() {
        let a = BigUint::from(123456u32);
        let b = BigUint::from(789u32);
        assert_eq!(multiply_via_word(&a, &b), multiply_via_word(&b, &a));
    }
}

#[cfg(test)]
mod bits_multiply_karatsuba_tests {
    // The existing multiply_via_word tests above all stay well under
    // KARATSUBA_THRESHOLD_LIMBS (32 limbs, 2048 bits), so none of them
    // ever actually exercise the recursive Karatsuba branch, only the
    // schoolbook fallback it calls below its own threshold. These
    // deliberately span, straddle, and exceed that threshold, checked
    // against BigUint's own real multiplication -- ground truth
    // independent of anything this file computes.
    use super::*;

    fn xorshift(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    fn random_biguint(limbs: usize, seed: &mut u64) -> BigUint {
        let mut v = alloc::vec::Vec::with_capacity(limbs);
        for _ in 0..limbs { v.push(xorshift(seed)); }
        bits_to_value(&limbs_trim(v))
    }

    #[test]
    fn matches_ordinary_multiplication_at_every_limb_count_around_the_threshold() {
        let mut seed: u64 = 0xDEAD_BEEF_CAFE_F00D;
        // 1 limb up through 4x the threshold, so this covers well below,
        // exactly at, and well above KARATSUBA_THRESHOLD_LIMBS on both
        // operands independently.
        for a_limbs in [1, 2, 8, 16, 31, 32, 33, 48, 64, 96, 128] {
            for b_limbs in [1, 16, 32, 33, 64, 128] {
                let a = random_biguint(a_limbs, &mut seed);
                let b = random_biguint(b_limbs, &mut seed);
                let expected = &a * &b;
                let got = multiply_via_word(&a, &b);
                assert_eq!(got, expected, "a_limbs={} b_limbs={} a={} b={}", a_limbs, b_limbs, a, b);
            }
        }
    }

    #[test]
    fn matches_ordinary_multiplication_on_many_random_large_pairs() {
        let mut seed: u64 = 0x1234_5678_9ABC_DEF0;
        for _ in 0..30 {
            let a_limbs = 40 + (xorshift(&mut seed) % 100) as usize;
            let b_limbs = 40 + (xorshift(&mut seed) % 100) as usize;
            let a = random_biguint(a_limbs, &mut seed);
            let b = random_biguint(b_limbs, &mut seed);
            let expected = &a * &b;
            let got = multiply_via_word(&a, &b);
            assert_eq!(got, expected, "a_limbs={} b_limbs={} a={} b={}", a_limbs, b_limbs, a, b);
        }
    }

    #[test]
    fn matches_schoolbook_directly_above_threshold() {
        // Cross-check the Karatsuba path against the schoolbook path it
        // replaced, not only against BigUint's own operator -- two
        // independently-shaped algorithms agreeing is a stronger check
        // than either alone against a single oracle.
        let mut seed: u64 = 0x0BAD_F00D_1234_5678;
        for _ in 0..15 {
            let a_limbs = 33 + (xorshift(&mut seed) % 80) as usize;
            let b_limbs = 33 + (xorshift(&mut seed) % 80) as usize;
            let mut av = alloc::vec::Vec::with_capacity(a_limbs);
            for _ in 0..a_limbs { av.push(xorshift(&mut seed)); }
            let mut bv = alloc::vec::Vec::with_capacity(b_limbs);
            for _ in 0..b_limbs { bv.push(xorshift(&mut seed)); }
            let av = limbs_trim(av);
            let bv = limbs_trim(bv);
            let karatsuba = bits_multiply(&av, &bv);
            let schoolbook = bits_multiply_schoolbook(&av, &bv);
            assert_eq!(karatsuba, schoolbook, "a_limbs={} b_limbs={}", a_limbs, b_limbs);
        }
    }

    #[test]
    fn handles_empty_and_single_limb_edge_cases() {
        assert_eq!(bits_multiply(&[], &[5]), alloc::vec::Vec::<u64>::new());
        assert_eq!(bits_multiply(&[5], &[]), alloc::vec::Vec::<u64>::new());
        assert_eq!(bits_multiply(&[], &[]), alloc::vec::Vec::<u64>::new());
        assert_eq!(bits_multiply(&[3], &[4]), vec![12]);
    }

    #[test]
    fn karatsuba_is_actually_faster_than_schoolbook_at_real_scale() {
        // Direct evidence, not an assumption: multiply two ~64000-bit
        // numbers (1000 limbs, well past the threshold) 20 times each way
        // and compare wall-clock time. Measured in isolation (single-
        // threaded `cargo test ... -- --test-threads=1`) this ran about
        // 2.85x faster (1.11s vs 3.17s); the assertion here only checks
        // strictly faster, not by how much, since this suite's own many
        // other tests running in parallel share the same CPU and can
        // compress both times toward each other without either algorithm
        // actually changing speed -- a hard multiplier here would be
        // measuring test-suite contention, not Karatsuba.
        let mut seed: u64 = 0x5EED_5EED_5EED_5EED;
        let a_limbs = 1000;
        let b_limbs = 1000;
        let a = random_biguint(a_limbs, &mut seed);
        let b = random_biguint(b_limbs, &mut seed);
        let a_bits = word_bits(&a);
        let b_bits = word_bits(&b);

        let t0 = std::time::Instant::now();
        for _ in 0..20 { core::hint::black_box(bits_multiply(&a_bits, &b_bits)); }
        let karatsuba_time = t0.elapsed();

        let t1 = std::time::Instant::now();
        for _ in 0..20 { core::hint::black_box(bits_multiply_schoolbook(&a_bits, &b_bits)); }
        let schoolbook_time = t1.elapsed();

        crate::nested_eprintln!("karatsuba={:?} schoolbook={:?} (1000 limbs, 20 reps each)", karatsuba_time, schoolbook_time);
        assert!(
            karatsuba_time < schoolbook_time,
            "Karatsuba ({:?}) should be faster than schoolbook ({:?}) at 1000 limbs",
            karatsuba_time, schoolbook_time
        );
    }
}

#[cfg(test)]
mod bits_divmod_knuth_tests {
    // Standalone, not yet wired into divmod_via_word or anything else --
    // exhaustive testing here decides whether it ever gets wired in at
    // all. Every case checks BOTH quotient and remainder, and re-derives
    // a == q*b + r directly from the result rather than trusting a single
    // BigUint operator to be the only oracle.
    use super::*;

    fn xorshift(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    fn random_limbs(n: usize, seed: &mut u64) -> alloc::vec::Vec<u64> {
        (0..n).map(|_| xorshift(seed)).collect()
    }

    fn check(a: &BigUint, b: &BigUint) {
        let a_limbs = word_bits(a);
        let b_limbs = word_bits(b);
        let (q, r) = bits_divmod_knuth(&a_limbs, &b_limbs);
        let qv = bits_to_value(&q);
        let rv = bits_to_value(&r);
        assert_eq!(&qv * b + &rv, *a, "a={} b={} q={} r={} (reconstruction failed)", a, b, qv, rv);
        assert!(rv < *b, "a={} b={} r={} >= b (remainder not reduced)", a, b, rv);
        let expected_q = a / b;
        let expected_r = a % b;
        assert_eq!(qv, expected_q, "a={} b={} quotient mismatch", a, b);
        assert_eq!(rv, expected_r, "a={} b={} remainder mismatch", a, b);
    }

    #[test]
    fn matches_ordinary_division_on_small_known_cases() {
        for (a, b) in [(100u64, 7u64), (91, 7), (999999999989, 3), (1071, 462), (u64::MAX, 3)] {
            check(&BigUint::from(a), &BigUint::from(b));
        }
    }

    #[test]
    fn matches_ordinary_division_with_multi_limb_divisors() {
        let a: BigUint = "123456789012345678901234567890123456789012345678901234567890".parse().unwrap();
        let b: BigUint = "987654321098765432109876543210".parse().unwrap();
        check(&a, &b); // a < b here, exercises the a.len() < b.len() early return
        check(&b, &a);
        let c: BigUint = "999999999999999999999999999999999999999999999999999999".parse().unwrap();
        check(&a, &c);
        check(&c, &b);
    }

    #[test]
    fn matches_on_many_random_pairs_at_varying_limb_counts() {
        let mut seed: u64 = 0xF00D_CAFE_BEEF_1234;
        for _ in 0..200 {
            let a_limbs = 1 + (xorshift(&mut seed) % 12) as usize;
            let b_limbs = 1 + (xorshift(&mut seed) % 8) as usize;
            let a = bits_to_value(&limbs_trim(random_limbs(a_limbs, &mut seed)));
            let mut b = bits_to_value(&limbs_trim(random_limbs(b_limbs, &mut seed)));
            if b.is_zero() { b = BigUint::from(1u32); }
            check(&a, &b);
        }
    }

    #[test]
    fn stresses_the_quotient_correction_with_extreme_limb_values() {
        // Divisor limbs at or near u64::MAX specifically bias the two-limb
        // quotient estimate toward needing its own correction step (the
        // `while rhat < BASE && qhat*v0 > ...` loop), and toward the rarer
        // add-back path when the estimate is still one too high after
        // that. Extreme values make both far more likely to actually fire
        // than uniformly random ones would.
        let mut seed: u64 = 0xABCD_1234_EF00_9999;
        let extremes: [u64; 6] = [u64::MAX, u64::MAX - 1, 1u64 << 63, (1u64 << 63) + 1, 0, 1];
        for &v1 in &extremes {
            for &v0 in &extremes {
                for _ in 0..10 {
                    let b_limbs = alloc::vec![v0, v1.max(1)]; // top limb nonzero so this is genuinely 2-limb
                    let mut a_limbs = random_limbs(4, &mut seed);
                    // Bias the dividend's own top two limbs toward the
                    // divisor's, the classic trigger for qhat overshoot.
                    a_limbs[3] = v1;
                    a_limbs[2] = v0;
                    let a = bits_to_value(&limbs_trim(a_limbs));
                    let b = bits_to_value(&limbs_trim(b_limbs));
                    if b.is_zero() { continue; }
                    check(&a, &b);
                }
            }
        }
    }

    #[test]
    fn matches_on_many_random_pairs_at_large_limb_counts() {
        let mut seed: u64 = 0x1357_9BDF_2468_ACE0;
        for _ in 0..30 {
            let a_limbs = 20 + (xorshift(&mut seed) % 60) as usize;
            let b_limbs = 2 + (xorshift(&mut seed) % 20) as usize;
            let (a_limbs, b_limbs) = if a_limbs < b_limbs { (b_limbs, a_limbs) } else { (a_limbs, b_limbs) };
            let a = bits_to_value(&limbs_trim(random_limbs(a_limbs, &mut seed)));
            let mut b = bits_to_value(&limbs_trim(random_limbs(b_limbs, &mut seed)));
            if b.is_zero() { b = BigUint::from(1u32); }
            check(&a, &b);
        }
    }

    #[test]
    fn falls_back_correctly_for_single_limb_divisors() {
        let mut seed: u64 = 0x0F0F_0F0F_0F0F_0F0F;
        for _ in 0..20 {
            let a_limbs = 1 + (xorshift(&mut seed) % 10) as usize;
            let a = bits_to_value(&limbs_trim(random_limbs(a_limbs, &mut seed)));
            let mut divisor = xorshift(&mut seed);
            if divisor == 0 { divisor = 1; }
            check(&a, &BigUint::from(divisor));
        }
    }

    #[test]
    fn a_less_than_b_gives_zero_quotient_and_a_as_remainder() {
        check(&BigUint::from(5u32), &BigUint::from(1000000007u32));
        let big: BigUint = "999999999999999999999999999999".parse().unwrap();
        check(&BigUint::from(3u32), &big);
    }

    #[test]
    fn a_equal_to_b_gives_quotient_one_and_zero_remainder() {
        let n: BigUint = "123456789012345678901234567890".parse().unwrap();
        check(&n, &n);
    }

    #[test]
    fn bits_to_value_stays_a_direct_reconstruction_not_a_glyph_round_trip() {
        // A real regression guard, not just a correctness check: an
        // earlier version of bits_to_value built the full glyph string
        // those limbs would produce under encode and decoded it back,
        // which cost roughly 100+ seconds across the ~1290 calls a single
        // gpu_gnfs_poly::select_polynomial invocation makes at this
        // scale, dwarfing the arithmetic underneath it entirely. This
        // asserts the fast, direct path stays fast: the same 1290 calls
        // on a 64-limb value should complete in well under a second, not
        // the two-plus minutes the glyph round trip would take.
        let mut seed: u64 = 0xC0FF_EE00_C0FF_EE00;
        let limbs = limbs_trim(random_limbs(64, &mut seed));
        let t0 = std::time::Instant::now();
        for _ in 0..1290 { core::hint::black_box(bits_to_value(&limbs)); }
        let elapsed = t0.elapsed();
        assert!(
            elapsed < std::time::Duration::from_secs(1),
            "bits_to_value took {:?} for 1290 calls on a 64-limb value -- should be well under 1s; the old glyph-string round trip took well over a minute here",
            elapsed
        );
    }

    #[test]
    fn matches_on_a_large_random_sweep() {
        // A much larger pass than the others above, before this function
        // is trusted for anything beyond its own test module: thousands
        // of pairs, every limb count from 1 to 24 on both sides.
        let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
        for a_limbs in 1..=24usize {
            for b_limbs in 1..=a_limbs.max(1) {
                for _ in 0..15 {
                    let a = bits_to_value(&limbs_trim(random_limbs(a_limbs, &mut seed)));
                    let mut b = bits_to_value(&limbs_trim(random_limbs(b_limbs, &mut seed)));
                    if b.is_zero() { b = BigUint::from(1u32); }
                    check(&a, &b);
                }
            }
        }
    }
}

#[cfg(test)]
mod divmod_via_word_tests {
    use super::*;

    #[test]
    fn division_by_zero_is_none() {
        assert_eq!(divmod_via_word(&BigUint::from(10u32), &BigUint::zero()), None);
    }

    #[test]
    fn zero_dividend_gives_zero_and_zero() {
        assert_eq!(
            divmod_via_word(&BigUint::zero(), &BigUint::from(7u32)),
            Some((BigUint::zero(), BigUint::zero()))
        );
    }

    #[test]
    fn dividend_smaller_than_modulus_is_all_remainder() {
        assert_eq!(
            divmod_via_word(&BigUint::from(3u32), &BigUint::from(9u32)),
            Some((BigUint::zero(), BigUint::from(3u32)))
        );
    }

    #[test]
    fn exact_division_leaves_no_remainder() {
        assert_eq!(
            divmod_via_word(&BigUint::from(91u32), &BigUint::from(7u32)),
            Some((BigUint::from(13u32), BigUint::zero()))
        );
    }

    #[test]
    fn small_pairs_match_ordinary_division_and_remainder() {
        let pairs = [(17u32, 5u32), (1071, 462), (100, 7), (255, 16), (999983, 37)];
        for (a, m) in pairs {
            let (a, m) = (BigUint::from(a), BigUint::from(m));
            let expected = (&a / &m, &a % &m);
            assert_eq!(divmod_via_word(&a, &m), Some(expected), "a={} m={}", a, m);
        }
    }

    #[test]
    fn large_dividend_small_modulus_matches_ordinary_arithmetic() {
        // exactly the shape mod_pow_walk needs: a huge number reduced
        // against a small witness-sized modulus, not two similarly-sized
        // operands.
        let a: BigUint = "123456789012345678901234567890123456789012345678901234567890"
            .parse().unwrap();
        let m = BigUint::from(37u32);
        let expected = (&a / &m, &a % &m);
        assert_eq!(divmod_via_word(&a, &m), Some(expected));
    }

    #[test]
    fn large_pair_matches_ordinary_arithmetic() {
        let a: BigUint = "987654321098765432109876543210".parse().unwrap();
        let m: BigUint = "123456789012345678901234567890".parse().unwrap();
        let expected = (&a / &m, &a % &m);
        assert_eq!(divmod_via_word(&a, &m), Some(expected));
    }

    #[test]
    fn modulo_via_word_matches_ordinary_remainder() {
        for (a, m) in [(999999999989u64, 3u64), (100, 7), (91, 7)] {
            let (a, m) = (BigUint::from(a), BigUint::from(m));
            let expected = &a % &m;
            assert_eq!(modulo_via_word(&a, &m), Some(expected), "a={} m={}", a, m);
        }
    }

    #[test]
    fn divmod_small_via_word_matches_ordinary_arithmetic() {
        for (a, small) in [
            (999999999989u64, 3u64), (100, 7), (91, 7), (0, 5), (5, 5),
            (u64::MAX, 999983), (1, u64::MAX),
        ] {
            let ab = BigUint::from(a);
            let expected_q = &ab / small;
            let expected_r = a % small;
            assert_eq!(
                divmod_small_via_word(&ab, small), Some((expected_q, expected_r)),
                "a={} small={}", a, small
            );
        }
    }

    #[test]
    fn divmod_small_via_word_matches_on_a_number_past_one_limb() {
        let a: BigUint = "123456789012345678901234567890123456789012345678901234567890".parse().unwrap();
        let small = 999999937u64; // a real prime, comfortably one word
        let expected_q = &a / small;
        let expected_r: u64 = (&a % small).try_into().unwrap();
        assert_eq!(divmod_small_via_word(&a, small), Some((expected_q, expected_r)));
    }

    #[test]
    fn divmod_small_via_word_is_none_for_zero_divisor() {
        assert_eq!(divmod_small_via_word(&BigUint::from(5u32), 0), None);
    }

    #[test]
    fn modulo_small_via_word_matches_ordinary_remainder() {
        for (a, small) in [(999999999989u64, 3u64), (100, 7), (91, 7)] {
            assert_eq!(modulo_small_via_word(&BigUint::from(a), small), Some(a % small), "a={} small={}", a, small);
        }
    }
}

#[cfg(test)]
mod signed_general_via_word_tests {
    // The general signed primitives (built for modinv_big's extended Euclid,
    // which needs both operands able to be negative, unlike
    // signed_add_via_word's nonnegative-magnitude-only case) checked against
    // BigInt's own +, -, *, / and % directly -- the same "ordinary" oracle
    // convention every other via_word test in this file uses.
    use super::*;

    fn values() -> Vec<i64> {
        vec![0, 1, -1, 2, -2, 5, -5, 17, -17, 100, -100, 12345, -12345, 999983, -999983]
    }

    #[test]
    fn signed_add_matches_ordinary_addition() {
        for &x in &values() {
            for &y in &values() {
                let a = BigInt::from(x);
                let b = BigInt::from(y);
                assert_eq!(signed_add_via_word_general(&a, &b), &a + &b, "x={} y={}", x, y);
            }
        }
    }

    #[test]
    fn signed_subtract_matches_ordinary_subtraction() {
        for &x in &values() {
            for &y in &values() {
                let a = BigInt::from(x);
                let b = BigInt::from(y);
                assert_eq!(signed_subtract_via_word_general(&a, &b), &a - &b, "x={} y={}", x, y);
            }
        }
    }

    #[test]
    fn signed_multiply_matches_ordinary_multiplication() {
        for &x in &values() {
            for &y in &values() {
                let a = BigInt::from(x);
                let b = BigInt::from(y);
                assert_eq!(signed_multiply_via_word(&a, &b), &a * &b, "x={} y={}", x, y);
            }
        }
    }

    #[test]
    fn signed_divmod_matches_ordinary_truncating_division() {
        for &x in &values() {
            for &y in &values() {
                if y == 0 { continue; }
                let a = BigInt::from(x);
                let b = BigInt::from(y);
                let expected = (&a / &b, &a % &b);
                assert_eq!(signed_divmod_via_word(&a, &b), Some(expected), "x={} y={}", x, y);
            }
        }
    }

    #[test]
    fn signed_divmod_is_none_for_zero_divisor() {
        assert_eq!(signed_divmod_via_word(&BigInt::from(5), &BigInt::from(0)), None);
    }

    #[test]
    fn signed_general_ops_match_on_large_values() {
        let a: BigInt = "-123456789012345678901234567890".parse().unwrap();
        let b: BigInt = "987654321098765432109876543210".parse().unwrap();
        assert_eq!(signed_add_via_word_general(&a, &b), &a + &b);
        assert_eq!(signed_subtract_via_word_general(&a, &b), &a - &b);
        assert_eq!(signed_multiply_via_word(&a, &b), &a * &b);
        assert_eq!(signed_divmod_via_word(&b, &a), Some((&b / &a, &b % &a)));
    }
}

#[cfg(test)]
mod unbraid_arm_tests {
    // Direct, unraced calls into each search arm, since unbraid_report runs
    // both arms concurrently and the bottom-up (hensel_unbraid) arm has won
    // every live race tried so far -- these are the only checks that ever
    // actually exercise range_prune_unbraid's own converted arithmetic
    // end to end.
    use super::*;

    #[test]
    fn range_prune_unbraid_finds_a_known_close_pair() {
        // 1009 x 1013 = 1022117, both 10 bits, both odd, top bits both 1.
        let n = BigUint::from(1022117u32);
        let p_bits = 10usize;
        let q_bits = 10usize;
        let p_hi = pow2(p_bits - 1);
        let q_hi = pow2(q_bits - 1);
        let start_pos = (p_bits.max(q_bits) as i64) - 2;
        let mut budget = UnbraidBudget { nodes: 0, cap: 1_000_000 };
        let stop = core::sync::atomic::AtomicBool::new(false);
        let (p, q) = range_prune_unbraid(&n, p_bits, q_bits, start_pos, p_hi, q_hi, &mut budget, &stop)
            .expect("range_prune_unbraid should find 1009 x 1013");
        let (lo, hi) = if p <= q { (p, q) } else { (q, p) };
        assert_eq!(lo, BigUint::from(1009u32));
        assert_eq!(hi, BigUint::from(1013u32));
    }

    #[test]
    fn range_prune_unbraid_finds_an_unbalanced_pair() {
        // 97 x 8191 = 794527: 7 bits vs 13 bits, the far-apart-factor shape
        // the top-down interval bound has to prune hard on rather than the
        // close-pair shape above.
        let n = BigUint::from(794527u32);
        let p_bits = 7usize;
        let q_bits = 13usize;
        let p_hi = pow2(p_bits - 1);
        let q_hi = pow2(q_bits - 1);
        let start_pos = (p_bits.max(q_bits) as i64) - 2;
        let mut budget = UnbraidBudget { nodes: 0, cap: 1_000_000 };
        let stop = core::sync::atomic::AtomicBool::new(false);
        let (p, q) = range_prune_unbraid(&n, p_bits, q_bits, start_pos, p_hi, q_hi, &mut budget, &stop)
            .expect("range_prune_unbraid should find 97 x 8191");
        let (lo, hi) = if p <= q { (p, q) } else { (q, p) };
        assert_eq!(lo, BigUint::from(97u32));
        assert_eq!(hi, BigUint::from(8191u32));
    }

    #[test]
    fn hensel_unbraid_finds_the_same_close_pair() {
        let n = BigUint::from(1022117u32);
        let n_signed = BigInt::from(n.clone());
        let p_bits = 10usize;
        let q_bits = 10usize;
        let p0 = BigUint::from(1u32);
        let q0 = BigUint::from(1u32);
        let c0 = (BigInt::from(1) - &n_signed) / BigInt::from(2);
        let mut budget = UnbraidBudget { nodes: 0, cap: 1_000_000 };
        let stop = core::sync::atomic::AtomicBool::new(false);
        let (p, q) = hensel_unbraid(&n_signed, p_bits, q_bits, 1, p0, q0, c0, &mut budget, &stop)
            .expect("hensel_unbraid should find 1009 x 1013");
        let (lo, hi) = if p <= q { (p, q) } else { (q, p) };
        assert_eq!(lo, BigUint::from(1009u32));
        assert_eq!(hi, BigUint::from(1013u32));
    }

    #[test]
    fn hensel_unbraid_finds_a_deliberately_unbalanced_odd_pair() {
        // 97 x 8191 = 794527: 7 bits vs 13 bits, long=13 while p_bits+q_bits=20 --
        // the exact shape where p*q ≡ N (mod 2^long) is a much weaker
        // congruence than p*q == N, so a false positive here (or a spurious
        // panic) would mean the recursion's exactness proof doesn't actually
        // hold once the split is this unbalanced.
        let n = BigUint::from(794527u32);
        let n_signed = BigInt::from(n.clone());
        let p_bits = 7usize;
        let q_bits = 13usize;
        let p0 = BigUint::from(1u32);
        let q0 = BigUint::from(1u32);
        let c0 = (BigInt::from(1) - &n_signed) / BigInt::from(2);
        let mut budget = UnbraidBudget { nodes: 0, cap: 1_000_000 };
        let stop = core::sync::atomic::AtomicBool::new(false);
        let (p, q) = hensel_unbraid(&n_signed, p_bits, q_bits, 1, p0, q0, c0, &mut budget, &stop)
            .expect("hensel_unbraid should find 97 x 8191");
        let (lo, hi) = if p <= q { (p, q) } else { (q, p) };
        assert_eq!(lo, BigUint::from(97u32));
        assert_eq!(hi, BigUint::from(8191u32));
    }

    #[test]
    fn hensel_unbraid_never_reports_a_false_positive_across_every_split() {
        // Sweep every (p_bits, q_bits) split unbraid_report itself would try
        // -- not just the one matching each N's real factorization -- for
        // several odd composites of different shapes (balanced, unbalanced,
        // a Carmichael number, a product of three odds). Whenever the arm
        // claims c=0 (a "found"), that claim must survive an exact
        // multiply_via_word check against N, not just the arm's own
        // internal congruence test. This is the direct empirical answer to
        // whether the N=100 false positive was specific to even N or a
        // wider gap in the exactness proof.
        let cases: [u64; 6] = [561, 8051, 1022117, 794527, 999983 * 3, 15];
        for &n_u64 in cases.iter() {
            let n = BigUint::from(n_u64);
            if modulo_via_word(&n, &BigUint::from(2u32)).unwrap() == BigUint::zero() {
                continue; // even N is out of this arm's domain entirely, covered elsewhere
            }
            let n_signed = BigInt::from(n.clone());
            let total_bits = n.bits() as usize;
            for p_bits in 2..=(total_bits / 2 + 1) {
                for &q_bits in &[total_bits + 1 - p_bits, total_bits.wrapping_sub(p_bits)] {
                    if q_bits < p_bits || q_bits == 0 || q_bits > total_bits {
                        continue;
                    }
                    let p0 = BigUint::from(1u32);
                    let q0 = BigUint::from(1u32);
                    let c0 = (BigInt::from(1) - &n_signed) / BigInt::from(2);
                    let mut budget = UnbraidBudget { nodes: 0, cap: 200_000 };
                    let stop = core::sync::atomic::AtomicBool::new(false);
                    if let Some((p, q)) =
                        hensel_unbraid(&n_signed, p_bits, q_bits, 1, p0, q0, c0, &mut budget, &stop)
                    {
                        assert_eq!(
                            multiply_via_word(&p, &q), n,
                            "false positive: N={} p_bits={} q_bits={} p={} q={} p*q={}",
                            n, p_bits, q_bits, p, q, multiply_via_word(&p, &q)
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod isqrt_and_fermat_tests {
    // fermat_factor's own success check (multiply_via_word(&b,&b) == r) is a
    // direct, single-step verification, not a distant invariant like
    // hensel_unbraid's -- so a false positive there is already structurally
    // impossible, given correct multiply/subtract primitives (which have
    // their own dedicated test modules). What fermat_factor actually
    // depends on without re-checking is isqrt being an exact floor sqrt:
    // if isqrt ever overshot low, a*a could fall below n and the
    // subtract_via_word(&multiply_via_word(&a,&a), n).unwrap() inside the
    // loop would panic rather than silently misbehave, the same
    // unwrap-backed-by-a-distant-proof shape as the hensel_unbraid bug,
    // just crashing instead of lying. This sweeps isqrt's own exactness
    // directly, then fermat_factor's absence of any panic or false
    // positive across a spread of gap sizes.
    use super::*;

    fn assert_isqrt_exact(n: &BigUint) {
        let x = isqrt(n);
        let x_sq = multiply_via_word(&x, &x);
        assert!(x_sq <= *n, "isqrt({}) = {} overshot: {}^2 = {} > n", n, x, x, x_sq);
        let x_plus_1 = add_via_word(&x, &BigUint::from(1u32));
        let x_plus_1_sq = multiply_via_word(&x_plus_1, &x_plus_1);
        assert!(x_plus_1_sq > *n, "isqrt({}) = {} undershot: ({}+1)^2 = {} <= n", n, x, x, x_plus_1_sq);
    }

    #[test]
    fn isqrt_is_exact_on_small_values_and_perfect_squares() {
        for k in 0u64..2000 {
            assert_isqrt_exact(&BigUint::from(k));
        }
        for k in 0u64..200 {
            assert_isqrt_exact(&BigUint::from(k * k));
        }
    }

    #[test]
    fn isqrt_is_exact_straddling_power_of_two_boundaries() {
        // Newton's own starting point is pow2(bits(n)/2 + 1), so the values
        // right around each power of two are exactly where a starting-point
        // or trailing-correction mistake would show up first.
        for shift in 1u32..80 {
            let p = pow2(shift as usize);
            for delta in [0i64, 1, -1, 2, -2] {
                let n = if delta >= 0 {
                    add_via_word(&p, &BigUint::from(delta as u64))
                } else {
                    subtract_via_word(&p, &BigUint::from((-delta) as u64)).unwrap()
                };
                if n.is_zero() { continue; }
                assert_isqrt_exact(&n);
            }
        }
    }

    #[test]
    fn isqrt_is_exact_on_large_numbers() {
        // Numbers well past anything the hensel_unbraid sweep exercised --
        // this is specifically stressing Newton's convergence and the
        // trailing correction loop at a scale where an off-by-a-few-
        // iterations bug would still show up as a wrong answer, not just
        // a slow one.
        let bases: [&str; 4] = [
            "123456789012345678901234567890123456789012345678901234567890",
            "999999999999999999999999999999999999999999999999999999999999999999",
            "100000000000000000000000000000000000000000000000000000000000000001",
            "31415926535897932384626433832795028841971693993751058209749445923078",
        ];
        for s in bases {
            let n: BigUint = s.parse().unwrap();
            assert_isqrt_exact(&n);
        }
    }

    #[test]
    fn fermat_factor_never_panics_or_false_positives_across_many_gaps() {
        // Odd composites with gaps between their real factors ranging from
        // adjacent (the cheapest case for Fermat) out to gaps Fermat's own
        // iteration budget should exhaust before finding anything -- both
        // outcomes (Some and None) must be self-consistent, and Some must
        // never be wrong.
        let cases: [(u64, u64); 8] = [
            (1009, 1013),   // gap 4, close
            (97, 103),      // gap 6
            (997, 1009),    // gap 12
            (7, 11),        // gap 4, small
            (3, 5),         // gap 2, smallest odd primes
            (10007, 10009), // gap 2, larger
            (65537, 65539), // gap 2, still larger
            (11, 9973),     // wide gap -- expect None within a small budget, not a panic
        ];
        for (p, q) in cases {
            let n = BigUint::from(p) * BigUint::from(q);
            for max_iters in [1u64, 5, 100, 5000] {
                if let Some((fp, fq, _rejected, _checked)) = fermat_factor(&n, max_iters) {
                    assert_eq!(
                        multiply_via_word(&fp, &fq), n,
                        "false positive: N={} (p={} q={}) max_iters={} got fp={} fq={}",
                        n, p, q, max_iters, fp, fq
                    );
                }
                // None is always an acceptable answer here (ran out of
                // budget); only Some is ever checked for correctness.
            }
        }
    }
}
