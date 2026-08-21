// pk2sk.rs — PK→SK recovery instrument, native (bounded-range ECDLP on
// secp256k1). Port of imsgct/pk2sk.py to the kernel.
//
// Recovers the secp256k1 private scalar from a compressed public key when the
// scalar is known (or suspected) to lie in an interval [lo, hi). BSGS
// meet-in-the-middle, exact, ~sqrt(w) steps and memory. The gate is the curve
// itself: a candidate scalar is accepted only when recomputing its compressed
// public key reproduces the target hex exactly. The recovered scalar is then
// imscribed through the same 12-slot mapping the corpus uses (⊢ bitlen,
// ⊤ leadz, ⊙ quadrant, ⊥ top4), so the Grammar closes the loop in-kernel.
//
// Bound, stated plainly (as in the python): a full-width 256-bit scalar needs
// 2^128 group operations — no imscription, no Grammar transform, and no
// classical tool changes that. What this instrument recovers is any key whose
// scalar sits in a searchable window. The kernel adds one honest bound of its
// own: the baby table is a sorted Vec in the 48 MB bump heap, so windows
// beyond ~2^39 are refused by the heap before they are attempted.
//
// no_std: U256 is 4×u64 limbs; field multiplication folds 2^256 ≡ 2^32+977
// (mod P), the identity P = 2^256 − (2^32+977); no bigint crate, no HashMap —
// the baby walk lands in a sorted Vec and the giant walk binary-searches it,
// exactly as winding_period does for its torus walk.

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;

// ── secp256k1 constants ─────────────────────────────────────────
// P = 2^256 − C with C = 2^32 + 977, so 2^256 ≡ C (mod P). Limbs are
// little-endian u64; verified against the hex forms by python before
// hardcoding.
const C: u64 = 0x1_0000_03D1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct U256(pub [u64; 4]);

pub const P_LIMBS: [u64; 4] =
    [0xfffffffefffffc2f, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff];
pub const N_LIMBS: [u64; 4] =
    [0xbfd25e8cd0364141, 0xbaaedce6af48a03b, 0xfffffffffffffffe, 0xffffffffffffffff];
pub const GX_LIMBS: [u64; 4] =
    [0x59f2815b16f81798, 0x029bfcdb2dce28d9, 0x55a06295ce870b07, 0x79be667ef9dcbbac];
pub const GY_LIMBS: [u64; 4] =
    [0x9c47d08ffb10d4b8, 0xfd17b448a6855419, 0x5da4fbfc0e1108a8, 0x483ada7726a3c465];

impl Ord for U256 {
    fn cmp(&self, other: &U256) -> Ordering {
        for i in (0..4).rev() {
            match self.0[i].cmp(&other.0[i]) {
                Ordering::Equal => continue,
                o => return o,
            }
        }
        Ordering::Equal
    }
}
impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &U256) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl U256 {
    pub const fn from_u64(v: u64) -> U256 { U256([v, 0, 0, 0]) }
    pub const fn p() -> U256 { U256(P_LIMBS) }
    pub const fn n() -> U256 { U256(N_LIMBS) }
    pub const fn gx() -> U256 { U256(GX_LIMBS) }
    pub const fn gy() -> U256 { U256(GY_LIMBS) }

    /// Parse 1..=64 lowercase/uppercase hex chars.
    pub fn from_hex(s: &str) -> Option<U256> {
        let s = s.trim();
        if s.is_empty() || s.len() > 64 { return None; }
        let mut v = [0u64; 4];
        for ch in s.chars() {
            let d = ch.to_digit(16)? as u64;
            let mut carry = d;
            for limb in v.iter_mut() {
                let t = (*limb as u128) << 4 | carry as u128;
                *limb = t as u64;
                carry = (t >> 64) as u64;
            }
            if carry != 0 { return None; }
        }
        Some(U256(v))
    }

    /// Zero-padded 64-char lowercase hex.
    pub fn to_hex_64(&self) -> String {
        let mut s = String::with_capacity(64);
        for limb in self.0.iter().rev() {
            s.push_str(&alloc::format!("{:016x}", limb));
        }
        s
    }

    /// Minimal hex (like python's hex(sk)[2:]); "0" for zero.
    pub fn to_hex_min(&self) -> String {
        let mut s = String::new();
        let mut started = false;
        for limb in self.0.iter().rev() {
            if !started {
                if *limb == 0 { continue; }
                s.push_str(&alloc::format!("{:x}", limb));
                started = true;
            } else {
                s.push_str(&alloc::format!("{:016x}", limb));
            }
        }
        if !started { s.push('0'); }
        s
    }

    pub fn bit_length(&self) -> u32 {
        for i in (0..4).rev() {
            if self.0[i] != 0 {
                return (i as u32) * 64 + (64 - self.0[i].leading_zeros());
            }
        }
        0
    }

    /// Full 4-limb add; returns (sum mod 2^256, carry_out).
    fn add_overflow(&self, b: &U256) -> (U256, bool) {
        let mut carry = 0u128;
        let mut out = [0u64; 4];
        for i in 0..4 {
            let s = self.0[i] as u128 + b.0[i] as u128 + carry;
            out[i] = s as u64;
            carry = s >> 64;
        }
        (U256(out), carry != 0)
    }

    /// self − b, plain 256-bit; returns (difference mod 2^256, borrow).
    fn sub_plain(&self, b: &U256) -> (U256, bool) {
        let mut borrow = 0u128;
        let mut out = [0u64; 4];
        for i in 0..4 {
            let bi = b.0[i] as u128 + borrow;
            let si = self.0[i] as u128;
            if si >= bi {
                out[i] = (si - bi) as u64;
                borrow = 0;
            } else {
                out[i] = (si + (1u128 << 64) - bi) as u64;
                borrow = 1;
            }
        }
        (U256(out), borrow != 0)
    }

    /// self − b (mod P). Assumes self, b < P; result < P.
    pub fn sub_mod(&self, b: &U256) -> U256 {
        let (d, bor) = self.sub_plain(b);
        if bor {
            // self < b: answer = P − (b − self), NOT P − (self − b mod 2^256).
            // The old code computed P − (2^256 − (b−self)) = P + (b−self)
            // ≡ −(b−self), which corrupted every addition with a descending y.
            let (delta, _) = b.sub_plain(self); // b − self, no borrow
            U256::p().sub_plain(&delta).0        // P − delta, no borrow (delta < P)
        } else {
            d
        }
    }

    /// self + b (mod P). Assumes self, b < P; result < P.
    pub fn add_mod(&self, b: &U256) -> U256 {
        let (s, c) = self.add_overflow(b);
        if c {
            // s + 2^256 ≡ s + C (mod P); s < 2^256 so s + C < 2^257.
            let (s2, c2) = s.add_overflow(&U256::from_u64(C));
            if c2 {
                // s ≥ 2^256 − C → s2 = s + C − 2^256 < C; ≡ s2 + C (mod P),
                // and s2 + C < 2^34 < P.
                U256([s2.0[0] + C, s2.0[1], s2.0[2], s2.0[3]])
            } else {
                s2.cond_sub_p()
            }
        } else {
            s.cond_sub_p()
        }
    }

    /// self − P if self ≥ P (self < 2^256 → one subtraction is exact).
    fn cond_sub_p(&self) -> U256 {
        let p = U256::p();
        if *self >= p { self.sub_plain(&p).0 } else { *self }
    }

    /// self · b (mod P). 8-limb schoolbook product, then fold the top half
    /// with 2^256 ≡ C (mod P) until it fits in 4 limbs.
    pub fn mul_mod(&self, b: &U256) -> U256 {
        let mut v = [0u64; 8];
        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let t = self.0[i] as u128 * b.0[j] as u128 + v[i + j] as u128 + carry;
                v[i + j] = t as u64;
                carry = t >> 64;
            }
            v[i + 4] = carry as u64;
        }
        // Fold: v = lo + hi·2^256 ≡ lo + hi·C (mod P). After the first fold
        // hi shrinks to one limb < 2^34; after the second to ≤ 1; the last
        // two iterations hold it there, so 4 passes terminate with v[4] ∈ {0,1}.
        for _ in 0..4 {
            if v[4] | v[5] | v[6] | v[7] == 0 { break; }
            let mut t = [0u64; 5];
            let mut carry = 0u128;
            for k in 0..4 {
                let p = v[4 + k] as u128 * C as u128 + carry;
                t[k] = p as u64;
                carry = p >> 64;
            }
            t[4] = carry as u64;
            let mut carry = 0u128;
            let mut w = [0u64; 8];
            for k in 0..4 {
                let s = v[k] as u128 + t[k] as u128 + carry;
                w[k] = s as u64;
                carry = s >> 64;
            }
            // t[4] < 2^33 and carry ≤ 1, so w[4] < 2^34 and w[5] = 0.
            w[4] = (t[4] as u128 + carry) as u64;
            v = w;
        }
        let lo = U256([v[0], v[1], v[2], v[3]]);
        if v[4] == 1 {
            // value = 2^256 + lo ≡ C + lo (mod P).
            let (s, c) = lo.add_overflow(&U256::from_u64(C));
            if c {
                U256([s.0[0] + C, s.0[1], s.0[2], s.0[3]])
            } else {
                s.cond_sub_p()
            }
        } else {
            lo.cond_sub_p()
        }
    }

    /// a^e (mod P), MSB-first square-and-multiply: r = r² every bit, r = r·a
    /// when the bit is set. The previous version multiplied r by the ADVANCING
    /// b (b = b² per bit) without ever squaring r — for e = 3 it returned a⁶
    /// instead of a³, so every modinv (Fermat a^(P−2)) was garbage and every
    /// point addition with a division landed off-curve.
    pub fn powmod(&self, e: &U256) -> U256 {
        let mut r = U256::from_u64(1);
        let a = *self;
        for i in (0..4).rev() {
            for bit in (0..64).rev() {
                r = r.mul_mod(&r);
                if (e.0[i] >> bit) & 1 == 1 {
                    r = r.mul_mod(&a);
                }
            }
        }
        r
    }

    /// Multiplicative inverse mod P by Fermat: a^(P−2). den ≠ 0 mod P for
    /// every live secp256k1 affine coordinate, so this is never called on 0.
    pub fn modinv(&self) -> U256 {
        let pm2 = U256([0xfffffffefffffc2d, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]);
        self.powmod(&pm2)
    }

    /// self >> 2 (exact floor division by 4; used for the quadrant split of N).
    fn shr2(&self) -> U256 {
        U256([
            self.0[0] >> 2 | (self.0[1] & 0x3) << 62,
            self.0[1] >> 2 | (self.0[2] & 0x3) << 62,
            self.0[2] >> 2 | (self.0[3] & 0x3) << 62,
            self.0[3] >> 2,
        ])
    }
}

// ── secp256k1 point arithmetic (affine, Option = identity) ─────

type Point = Option<(U256, U256)>;

/// Affine point add, ported line-for-line from pk2sk.py. None = identity.
pub fn pt_add(p: Point, q: Point) -> Point {
    match (p, q) {
        (None, q) => q,
        (p, None) => p,
        (Some((x1, y1)), Some((x2, y2))) => {
            let zero = U256::from_u64(0);
            if x1 == x2 && y1.add_mod(&y2) == zero {
                return None; // p + (−p) = O
            }
            let lam = if x1 == x2 {
                // doubling: λ = 3x²/2y
                let num = x1.mul_mod(&x1).mul_mod(&U256::from_u64(3));
                num.mul_mod(&y1.add_mod(&y1).modinv())
            } else {
                // addition: λ = (y2 − y1)/(x2 − x1)
                y2.sub_mod(&y1).mul_mod(&x2.sub_mod(&x1).modinv())
            };
            let x3 = lam.mul_mod(&lam).sub_mod(&x1).sub_mod(&x2);
            let y3 = lam.mul_mod(&x1.sub_mod(&x3)).sub_mod(&y1);
            Some((x3, y3))
        }
    }
}

/// Double-and-add scalar multiply. k is a u64 (< 2^64 < N, the group order,
/// so no reduction of k mod N is ever needed for kernel command ranges).
pub fn pt_mul(k: u64, x: U256, y: U256) -> Point {
    let mut rx: Point = None;
    let (mut cx, mut cy) = (x, y);
    let mut kk = k;
    while kk > 0 {
        if kk & 1 == 1 {
            rx = pt_add(rx, Some((cx, cy)));
        }
        if let Some((dx, dy)) = pt_add(Some((cx, cy)), Some((cx, cy))) {
            cx = dx;
            cy = dy;
        }
        kk >>= 1;
    }
    rx
}

/// Integer square root (Newton), from winding_period.
fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

// ── The gate: the curve itself ─────────────────────────────────

/// A candidate scalar is accepted only when recomputing its compressed
/// public key reproduces the target exactly: x-coordinate equal and y
/// parity matching the 02/03 prefix. This is the same gate pk2sk.py runs
/// through coincurve — the kernel recomputes it with its own field math.
fn gate(cand: u64, target_x: &U256, target_even: bool) -> bool {
    if cand == 0 { return false; }
    match pt_mul(cand, U256::gx(), U256::gy()) {
        None => false,
        Some((gx, gy)) => {
            let even = gy.0[0] & 1 == 0;
            gx == *target_x && even == target_even
        }
    }
}

// ── BSGS: the bounded-range ECDLP walk ─────────────────────────

pub struct Hit {
    pub cand: u64,
    pub m: u64,
    pub giant: u64,
    pub baby: Option<u64>,
}

/// Baby-step giant-step over [lo, hi): table of x(j·G) for j in 1..m−1,
/// then walk P − (lo + i·m)·G down toward the table. A hit yields
/// cand = lo + i·m + j, curve-gated before acceptance. Mirrors the python
/// exactly, including the j=0 (identity) branch via the None case.
fn bsgs(px: U256, py: U256, lo: u64, hi: u64, target_x: &U256, target_even: bool) -> Option<Hit> {
    let w = hi - lo;
    let m = isqrt(w) + 1;
    if m > (1u64 << 22) {
        return None; // caller reports BOUND
    }
    // Kernel-specific bound: the baby table lives in the 48 MB bump heap.
    // (U256, u64) is 40 bytes; with_capacity rounds, so ask for the exact
    // table and refuse if the heap cannot hold it.
    let need = (m - 1) as usize * 40;
    let (used, total) = crate::heap_used();
    if need > total.saturating_sub(used) {
        return None;
    }
    let mut baby: Vec<(U256, u64)> = Vec::with_capacity((m - 1) as usize);
    let (mut cx, mut cy) = (U256::gx(), U256::gy());
    for j in 1..m {
        baby.push((cx, j));
        if let Some((nx, ny)) = pt_add(Some((cx, cy)), Some((U256::gx(), U256::gy()))) {
            cx = nx;
            cy = ny;
        }
    }
    baby.sort_unstable_by_key(|t| t.0);
    baby.dedup_by_key(|t| t.0);

    // Start of the giant walk: P − lo·G.
    let start: Point = match pt_mul(lo, U256::gx(), U256::gy()) {
        None => Some((px, py)),
        Some((bx, by)) => pt_add(Some((px, py)), Some((bx, U256::p().sub_mod(&by)))),
    };
    let step = pt_mul(m, U256::gx(), U256::gy());
    let neg_step: Point = match step {
        None => None,
        Some((sx, sy)) => Some((sx, U256::p().sub_mod(&sy))),
    };
    let mut giant = start;
    for i in 0..m {
        let mut hit_baby: Option<u64> = None;
        let mut cand_u128: Option<u128> = None;
        match giant {
            None => {
                // P − (lo + i·m)·G = O  →  j = 0
                cand_u128 = Some(lo as u128 + (i as u128) * (m as u128));
            }
            Some((gx, _)) => {
                if let Ok(k) = baby.binary_search_by_key(&gx, |t| t.0) {
                    let (_, j) = baby[k];
                    hit_baby = Some(j);
                    cand_u128 = Some(lo as u128 + (i as u128) * (m as u128) + j as u128);
                }
            }
        }
        if let Some(c) = cand_u128 {
            if c < hi as u128 {
                let cand = c as u64;
                if gate(cand, target_x, target_even) {
                    return Some(Hit { cand, m, giant: i, baby: hit_baby });
                }
            }
        }
        giant = pt_add(giant, neg_step);
    }
    None
}

// ── Grammar verification layer: the 12-slot mapping, ported ────

const DIM: [&str; 4] = ["𐑛", "𐑨", "𐑼", "𐑦"];   // bitlen: ≤250 ≤252 ≤254 else
const KIN: [&str; 5] = ["𐑘", "𐑤", "𐑧", "𐑪", "𐑺"]; // leadz: 0 1 2 3 4
const QUAD: [&str; 4] = ["𐑢", "⊙", "𐑮", "𐑻"];  // sk vs N/4, N/2, 3N/4
const CHIR: [&str; 4] = ["𐑓", "𐑒", "𐑖", "𐑫"]; // top4: ≤3 ≤7 ≤11 else

/// The prior session's 12-slot mapping, restricted to SK-derived slots —
/// identical thresholds to pk2sk.py::imscribe_slots.
fn imscribe_slots(sk: u64) -> String {
    let n = U256::n();
    let q = n.shr2(); // floor(N/4); N ≡ 1 (mod 4) so python's n//4 = floor
    let sku = U256::from_u64(sk);
    let bl = sku.bit_length();
    let leadz = core::cmp::min(4, 64 - ((bl as i64 + 3) / 4)) as usize;
    let dim = if bl <= 250 { DIM[0] } else if bl <= 252 { DIM[1] } else if bl <= 254 { DIM[2] } else { DIM[3] };
    let kin = KIN[leadz];
    let q2 = q.add_overflow(&q).0;
    let q3 = q2.add_overflow(&q).0;
    let quad = if sku < q { QUAD[0] } else if sku < q2 { QUAD[1] } else if sku < q3 { QUAD[2] } else { QUAD[3] };
    let top4 = (sku.0[3] >> 60) & 0xF;
    let chir = if top4 <= 3 { CHIR[0] } else if top4 <= 7 { CHIR[1] } else if top4 <= 11 { CHIR[2] } else { CHIR[3] };
    alloc::format!(
        "⊢={} ⊤={} ⊙={} ⊥={} (bitlen={}, leadz={})",
        dim, kin, quad, chir, bl, leadz
    )
}

// ── The instrument: run / selftest / help ──────────────────────

/// Parse a compressed public key hex (02|03 + 64 hex chars of x) into the
/// x-coordinate and the y-parity the prefix asserts. Same shape as the
/// python's coincurve parse: 02 = y even, 03 = y odd.
pub fn parse_pk(pk_hex: &str) -> Option<(U256, bool)> {
    let h = pk_hex.trim();
    let x = h.strip_prefix("02").or_else(|| h.strip_prefix("03"))?;
    if x.len() != 64 { return None; }
    let even = h.starts_with("02");
    Some((U256::from_hex(x)?, even))
}


/// Decompress x to (x, y): y² = x³ + 7, sqrt by Euler since P ≡ 3 (mod 4):
/// y = (y²)^((P+1)/4). The root whose parity matches the 02/03 prefix is
/// chosen, so the walk target is the true point — the same decompression
/// coincurve does for the python's uncompressed parse.
fn decompress(x: U256, want_even: bool) -> (U256, U256) {
    let y2 = x.mul_mod(&x).mul_mod(&x).add_mod(&U256::from_u64(7));
    // (P+1)/4 = 2^254 − 2^30 − 244 (limbs verified by python round-trip)
    let e = U256([0xffffffffbfffff0c, 0xffffffffffffffff, 0xffffffffffffffff, 0x3fffffffffffffff]);
    let mut y = y2.powmod(&e);
    let even = y.0[0] & 1 == 0;
    if even != want_even { y = U256::p().sub_mod(&y); }
    (x, y)
}

/// Recover the scalar in [lo, hi) whose compressed PK is pk_hex.
/// Output matches pk2sk.py's shape: PK / range / RESULT lines.
pub fn run(pk_hex: &str, lo: u64, hi: u64) -> String {
    let t0 = unsafe { core::arch::x86_64::_rdtsc() };
    let mut s = String::new();
    if hi <= lo {
        s.push_str(&alloc::format!("BOUND: empty range [{}, {}) — nothing to search.\n", lo, hi));
        return s;
    }
    let (tx, teven) = match parse_pk(pk_hex) {
        Some(v) => v,
        None => {
            s.push_str(&alloc::format!(
                "ERR: cannot parse '{}' as a compressed secp256k1 public key \
                 (expected 02 or 03 followed by 64 hex digits).\n", pk_hex));
            return s;
        }
    };
    let w = hi - lo;
    let wk = (64 - w.leading_zeros()) as i64 - 1; // w.bit_length() − 1
    s.push_str(&alloc::format!("PK   : {}\n", pk_hex.trim()));
    s.push_str(&alloc::format!("range: [{}, {})  w=2^{}  method=bsgs\n", lo, hi, wk));
    // Classical bound, ported from the python: beyond 2^44 group ops is
    // years on one core; a full 256-bit scalar is 2^128 — no imscription
    // changes that.
    if wk > 44 {
        s.push_str("BOUND: window exceeds feasible classical search (2^44 group ops ≈ \
                    years on one core). The discrete-log bound for a full 256-bit \
                    scalar is 2^128 — no imscription changes that. Narrow the window.\n");
        return s;
    }
    let m = isqrt(w) + 1;
    if m > (1u64 << 22) {
        s.push_str(&alloc::format!(
            "BOUND: BSGS window {} too wide (m = 2^{}); the kernel's baby table \
             cannot exceed the 2^22 BSGS limit.\n", w, m.ilog2()));
        return s;
    }
    // The kernel's own honest bound: the baby table is a sorted Vec in the
    // 48 MB bump heap, so a window whose table would not fit is refused
    // before it is attempted.
    let need = (m - 1) as usize * 40;
    let (used, total) = crate::heap_used();
    let avail = total.saturating_sub(used);
    if need > avail {
        s.push_str(&alloc::format!(
            "BOUND: baby table needs {} bytes for m = 2^{}, but the bump heap has \
             only {} of {} bytes free. Narrow the window so m ≤ {} (w ≲ 2^{}).\n",
            need, m.ilog2(), avail, total, avail / 40, (avail / 40).ilog2() * 2));
        return s;
    }
    // Decompress the target to the true point; the gate resolves any sign
    // ambiguity against the prefix, but the walk needs an on-curve y.
    let (px, py) = decompress(tx, teven);
    match bsgs(px, py, lo, hi, &tx, teven) {
        None => {
            let dt = unsafe { core::arch::x86_64::_rdtsc() } - t0;
            s.push_str(&alloc::format!(
                "RESULT: no hit in window after {} cycles (rdtsc) — the key's \
                 scalar is not in [{}, {}).\n", dt, lo, hi));
        }
        Some(hit) => {
            let dt = unsafe { core::arch::x86_64::_rdtsc() } - t0;
            s.push_str(&alloc::format!("RESULT: SK = 0x{}\n", U256::from_u64(hit.cand).to_hex_min()));
            let jj = match hit.baby { Some(j) => alloc::format!("{}", j), None => "0 (identity)".into() };
            s.push_str(&alloc::format!(
                "  recovered after m={} baby steps, giant step i={} (j={}), in {} cycles (rdtsc)\n",
                hit.m, hit.giant, jj, dt));
            s.push_str("  curve-verified: PK recomputed from the scalar and matched\n");
            s.push_str(&alloc::format!("  imscribed SK slots: {}\n", imscribe_slots(hit.cand)));
        }
    }
    s
}

/// Fixed-key selftest. The kernel has no RNG, so the selftest key is
/// hardcoded (SK = 2^40 + 0xb8ef, the same key the python selftest drew):
/// the recovery is still a real computation — the kernel must find the
/// scalar in [2^40, 2^40 + 2^18) from its public key alone.
pub fn selftest() -> String {
    const SK: u64 = (1u64 << 40) + 0xb8ef;
    const PK: &str = "02d44829137ab6f3460dd0fda59bdb95ac0c0141a77f90e9303aa37143cf104205";
    const LO: u64 = 1u64 << 40;
    const HI: u64 = (1u64 << 40) + (1u64 << 18);
    let mut s = String::new();
    s.push_str(&alloc::format!("SELFTEST: fixed key with scalar in [2^40, 2^40 + 2^18)\n"));
    s.push_str(&alloc::format!("  true SK = 0x{:x}\n", SK));
    s.push_str(&alloc::format!("  true PK = {}\n", PK));
    let out = run(PK, LO, HI);
    s.push_str(&out);
    let ok = out.contains(&alloc::format!("SK = 0x{:x}", SK));
    s.push_str(if ok {
        "SELFTEST: PASS — SK recovered from PK alone\n"
    } else {
        "SELFTEST: FAIL — recovered scalar did not match the true key\n"
    });
    s
}

pub fn help() -> String {
    let mut s = String::new();
    s.push_str("pk2sk — PK→SK recovery (bounded-range ECDLP on secp256k1)\n");
    s.push_str("  Recover the private scalar from a compressed public key when the\n");
    s.push_str("  scalar is known to lie in [lo, hi). BSGS meet-in-the-middle, exact,\n");
    s.push_str("  ~sqrt(w) steps and memory. The gate is the curve itself: a candidate\n");
    s.push_str("  is accepted only when recomputing its compressed PK reproduces the\n");
    s.push_str("  target hex. The recovered scalar is then imscribed through the same\n");
    s.push_str("  12-slot mapping the corpus uses (⊢ bitlen, ⊤ leadz, ⊙ quadrant, ⊥ top4).\n");
    s.push_str("  Bound: a full 256-bit scalar needs 2^128 group ops; this recovers\n");
    s.push_str("  scalars in searchable windows (≤ 2^44, and the kernel's 48 MB bump\n");
    s.push_str("  heap limits the baby table to windows of roughly 2^39 in practice).\n");
    s.push_str("forms:\n");
    s.push_str("  pk2sk help\n");
    s.push_str("  pk2sk selftest\n");
    s.push_str("  pk2sk search <pk_hex> <lo> <hi>\n");
    s.push_str("example:\n");
    s.push_str("  pk2sk search 03f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f 12000 13000\n");
    s.push_str("  (recovers SK = 0x3039, the scalar 12345)\n");
    s
}
