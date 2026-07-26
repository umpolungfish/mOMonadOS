// quadratic.rs — real quadratic field arithmetic, computed, never tabulated.
//
// Everything the SIC moduli argument needs about F = Q(sqrt(m_d)) is derived
// here from d alone: the squarefree core, the discriminant, the ring of
// integers, the fundamental unit, the class group, the unit group of O/mO,
// the wide ray class group at a rational modulus, and the order of its
// sigma-coinvariants.
//
// The chain is: d -> m_d = (d-3)(d+1) -> squarefree core -> discriminant
// -> O = Z[omega] -> (class group, fundamental unit) -> (O/m)^*
// -> Cl_m as an extension -> Cl_m modulo (sigma - 1).

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

// ═══════════════════════════════════════════════════════════════
// §1.  INTEGER GROUNDWORK
// ═══════════════════════════════════════════════════════════════

pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Squarefree core: largest squarefree divisor s with n = s * (square).
pub fn core(n: i64) -> i64 {
    let mut n = n;
    let mut s: i64 = 1;
    let mut p: i64 = 2;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            if e % 2 == 1 {
                s *= p;
            }
        }
        p += 1;
    }
    s * n
}

/// Modular inverse, or None when not invertible.
pub fn inv_mod(a: i64, m: i64) -> Option<i64> {
    let (mut old_r, mut r) = (a.rem_euclid(m), m);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let (nr, ns) = (old_r - q * r, old_s - q * s);
        old_r = r;
        r = nr;
        old_s = s;
        s = ns;
    }
    if old_r != 1 {
        None
    } else {
        Some(old_s.rem_euclid(m))
    }
}

/// Square root of a non-negative integer as f64, by Newton iteration on
/// integers so the module needs no floating-point intrinsics.
fn floor_div_f(x: f64) -> i64 {
    let t = x as i64;
    if (t as f64) > x { t - 1 } else { t }
}

fn isqrt_f(n: i64) -> f64 {
    if n <= 0 {
        return 0.0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    // x = floor(sqrt(n)); refine to a fractional value by one linear step
    let x0 = x as f64;
    let rem = (n - x * x) as f64;
    x0 + rem / (2.0 * x0 + 1.0)
}

// ═══════════════════════════════════════════════════════════════
// §2.  THE FIELD AND ITS RING OF INTEGERS
// ═══════════════════════════════════════════════════════════════

/// F = Q(sqrt(core)), with discriminant `disc` and O = Z[omega].
///
/// omega^2 = omega + (disc - 1)/4   when disc = 1 mod 4
/// omega^2 = disc/4                 when disc = 0 mod 4
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealQuad {
    pub core: i64,
    pub disc: i64,
}

impl RealQuad {
    /// The field attached to dimension d: m_d = (d-3)(d+1), reduced to its core.
    pub fn for_dimension(d: i64) -> Self {
        Self::new(core((d - 3) * (d + 1)))
    }

    pub fn new(sqfree_core: i64) -> Self {
        let disc = if sqfree_core.rem_euclid(4) == 1 {
            sqfree_core
        } else {
            4 * sqfree_core
        };
        RealQuad { core: sqfree_core, disc }
    }

    /// omega^2 = t*omega + n, returning (t, n).
    pub fn omega_relation(&self) -> (i64, i64) {
        if self.disc.rem_euclid(4) == 1 {
            (1, (self.disc - 1) / 4)
        } else {
            (0, self.disc / 4)
        }
    }

    /// Norm of a + b*omega.
    pub fn norm(&self, a: i64, b: i64) -> i64 {
        let (t, n) = self.omega_relation();
        // (a + b w)(a + b w') with w + w' = t, w w' = -n
        a * a + t * a * b - n * b * b
    }

    /// Is the rational prime p inert, split, or ramified in F.
    pub fn splitting(&self, p: i64) -> Splitting {
        if self.disc % p == 0 {
            return Splitting::Ramified;
        }
        // Kronecker symbol (disc / p)
        if p == 2 {
            match self.disc.rem_euclid(8) {
                1 => Splitting::Split,
                5 => Splitting::Inert,
                _ => Splitting::Ramified,
            }
        } else {
            let mut r = 1i64;
            let mut e = (p - 1) / 2;
            let mut base = self.disc.rem_euclid(p);
            while e > 0 {
                if e & 1 == 1 {
                    r = r * base % p;
                }
                base = base * base % p;
                e >>= 1;
            }
            if r == 1 {
                Splitting::Split
            } else {
                Splitting::Inert
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Splitting {
    Split,
    Inert,
    Ramified,
}

// ═══════════════════════════════════════════════════════════════
// §3.  THE FUNDAMENTAL UNIT, BY CONTINUED FRACTION
// ═══════════════════════════════════════════════════════════════

/// The fundamental unit as (a, b) meaning a + b*omega, together with its norm.
///
/// Found from the continued fraction expansion of omega: the period of the
/// expansion gives the unit, and the parity of the period gives the norm.
/// The fundamental unit, found as the unit of least positive omega-coefficient.
///
/// A unit is a + b*omega with a^2 + t a b - n b^2 = +-1. For fixed b this is a
/// quadratic in a, solvable exactly, and the fundamental unit is the one with
/// the least positive b. No continued fraction conventions are involved.
pub fn fundamental_unit(f: &RealQuad) -> (i64, i64, i64) {
    let (t, n) = f.omega_relation();
    let bound = 4_000_000i64;
    let mut b = 1i64;
    while b <= bound {
        for s in [-1i64, 1i64] {
            // a^2 + t*b*a - (n*b^2 + s) = 0
            let disc_a = t * t * b * b + 4 * (n * b * b + s);
            if disc_a < 0 {
                continue;
            }
            let r = isqrt_int(disc_a);
            if r * r != disc_a {
                continue;
            }
            for &num in &[-t * b + r, -t * b - r] {
                if num.rem_euclid(2) != 0 {
                    continue;
                }
                let a = num / 2;
                if f.norm(a, b) == s {
                    return (a, b, s);
                }
            }
        }
        b += 1;
    }
    (1, 0, 1)
}

/// Integer square root, floor.
pub fn isqrt_int(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

// ═══════════════════════════════════════════════════════════════
// §4.  THE CLASS GROUP, BY CYCLES OF REDUCED INDEFINITE FORMS
// ═══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Form {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

impl Form {
    pub fn disc(&self) -> i64 {
        self.b * self.b - 4 * self.a * self.c
    }

    /// Is this a reduced indefinite form: 0 < b < sqrt(D) and
    /// sqrt(D) - b < 2|a| < sqrt(D) + b.
    fn is_reduced(&self, sqrt_d: f64) -> bool {
        let b = self.b as f64;
        let a2 = 2.0 * (self.a.abs() as f64);
        b > 0.0 && b < sqrt_d && (sqrt_d - b) < a2 && a2 < (sqrt_d + b)
    }

    /// rho: the reduction step on indefinite forms.
    fn rho(&self, d: i64, sqrt_d: f64) -> Form {
        let a = self.c;
        let mut b;
        let abs_2a = 2 * a.abs();
        // choose b' = -b mod 2a in the window closest to sqrt(D)
        let target = if a.abs() > (sqrt_d as i64) {
            // |b'| <= |a|
            let mut t = (-self.b).rem_euclid(abs_2a);
            if t > a.abs() {
                t -= abs_2a;
            }
            t
        } else {
            let mut t = (-self.b).rem_euclid(abs_2a);
            while (t as f64) < sqrt_d - abs_2a as f64 {
                t += abs_2a;
            }
            while (t as f64) > sqrt_d {
                t -= abs_2a;
            }
            t
        };
        b = target;
        if b <= 0 && a != 0 {
            // keep b positive in the reduced window where possible
            let mut t = b;
            while t <= 0 {
                t += abs_2a;
            }
            if (t as f64) < sqrt_d {
                b = t;
            }
        }
        let c = (b * b - d) / (4 * a);
        Form { a, b, c }
    }
}

/// The class group of F: the number of cycles of reduced forms is the narrow
/// class number; the wide class number follows from the norm of the unit.
pub struct ClassGroup {
    pub narrow: i64,
    pub wide: i64,
    /// One reduced form per wide class, the first being the principal class.
    pub reps: Vec<Form>,
}

pub fn class_group(f: &RealQuad) -> ClassGroup {
    let d = f.disc;
    let sqrt_d = isqrt_f(d);
    let mut reduced: Vec<Form> = Vec::new();

    let bound = sqrt_d as i64 + 1;
    for b in 1..=bound {
        if (b - d).rem_euclid(2) != 0 {
            continue;
        }
        let num = b * b - d;
        if num % 4 != 0 {
            continue;
        }
        let ac = num / 4; // = a*c, negative for indefinite forms
        if ac == 0 {
            continue;
        }
        let mut a = 1i64;
        while a * a <= ac.abs() {
            if ac % a == 0 {
                for &sa in &[a, -a] {
                    let c = ac / sa;
                    let form = Form { a: sa, b, c };
                    if form.disc() == d && form.is_reduced(sqrt_d) {
                        reduced.push(form);
                    }
                    let form2 = Form { a: ac / sa, b, c: sa };
                    if form2.disc() == d && form2.is_reduced(sqrt_d) {
                        reduced.push(form2);
                    }
                }
            }
            a += 1;
        }
    }
    reduced.sort_by_key(|f| (f.a, f.b, f.c));
    reduced.dedup();

    // Partition the reduced forms into rho-cycles; each cycle is one narrow class.
    let mut unseen: Vec<Form> = reduced.clone();
    let mut cycles: Vec<Vec<Form>> = Vec::new();
    while let Some(start) = unseen.first().copied() {
        let mut cycle = vec![start];
        let mut cur = start.rho(d, sqrt_d);
        let mut guard = 0;
        while cur != start && guard < 10_000 {
            cycle.push(cur);
            cur = cur.rho(d, sqrt_d);
            guard += 1;
        }
        unseen.retain(|f| !cycle.contains(f));
        cycles.push(cycle);
    }

    let narrow = cycles.len() as i64;
    let (_, _, unit_norm) = fundamental_unit(f);
    let wide = if unit_norm == -1 { narrow } else { narrow / 2.max(1) };
    let wide = if wide == 0 { 1 } else { wide };

    let reps: Vec<Form> = cycles.iter().filter_map(|c| c.first().copied()).collect();
    ClassGroup { narrow, wide, reps }
}

// ═══════════════════════════════════════════════════════════════
// §5.  THE UNIT GROUP OF O/mO, WITH THE SIGMA ACTION
// ═══════════════════════════════════════════════════════════════

/// An element a + b*omega of O/mO.
pub type Residue = (i64, i64);

pub struct ResidueRing {
    pub f: RealQuad,
    pub m: i64,
    pub t: i64,
    pub n: i64,
}

impl ResidueRing {
    pub fn new(f: RealQuad, m: i64) -> Self {
        let (t, n) = f.omega_relation();
        ResidueRing { f, m, t, n }
    }

    pub fn mul(&self, x: Residue, y: Residue) -> Residue {
        // (a + b w)(c + e w) = ac + (ae + bc) w + be w^2, w^2 = t w + n
        let (a, b) = x;
        let (c, e) = y;
        let hi = b * e;
        let lo = (a * c + hi * self.n).rem_euclid(self.m);
        let mid = (a * e + b * c + hi * self.t).rem_euclid(self.m);
        (lo, mid)
    }

    pub fn one(&self) -> Residue {
        (1 % self.m, 0)
    }

    /// sigma(a + b*omega) = a + b*omega_bar = (a + b*t) - b*omega.
    pub fn conj(&self, x: Residue) -> Residue {
        let (a, b) = x;
        ((a + b * self.t).rem_euclid(self.m), (-b).rem_euclid(self.m))
    }

    pub fn is_unit(&self, x: Residue) -> bool {
        let (a, b) = x;
        gcd(self.f.norm(a, b).rem_euclid(self.m), self.m) == 1
    }

    /// Every unit of O/mO, enumerated.
    pub fn units(&self) -> Vec<Residue> {
        let mut v = Vec::new();
        for a in 0..self.m {
            for b in 0..self.m {
                if self.is_unit((a, b)) {
                    v.push((a, b));
                }
            }
        }
        v
    }

    pub fn pow(&self, x: Residue, mut e: u64) -> Residue {
        let mut acc = self.one();
        let mut base = x;
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        acc
    }
}

/// The subgroup of (O/m)^* generated by a set of residues.
pub fn generated_subgroup(r: &ResidueRing, gens: &[Residue]) -> Vec<Residue> {
    let mut seen: Vec<Residue> = vec![r.one()];
    let mut frontier = vec![r.one()];
    while let Some(x) = frontier.pop() {
        for &g in gens {
            let y = r.mul(x, g);
            if !seen.contains(&y) {
                seen.push(y);
                frontier.push(y);
            }
        }
    }
    seen
}

// ═══════════════════════════════════════════════════════════════
// §6.  THE RAY CLASS GROUP AT A RATIONAL MODULUS, AND ITS
//      SIGMA-COINVARIANTS
// ═══════════════════════════════════════════════════════════════

/// What the SIC argument asks of a dimension, all of it computed.
#[derive(Clone, Debug)]
pub struct RayData {
    pub d: i64,
    pub m_d: i64,
    pub core: i64,
    pub disc: i64,
    pub class_number: i64,
    /// |(O/m)^* / <units>|, the ray class group modulo the class group.
    pub ray_over_class: i64,
    /// |Cl_m|, the wide ray class group at the modulus.
    pub ray_order: i64,
    /// |Cl_m / (sigma - 1) Cl_m|.
    pub coinvariant: i64,
    /// |Cl / (sigma - 1) Cl|.
    pub class_coinvariant: i64,
    /// coinvariant / class_coinvariant.
    pub corrected: i64,
    pub d_half: i64,
}

/// Coinvariants of a finite abelian group given as an explicit element list
/// with multiplication and an involution: |G / (sigma-1)G| = |G| / |(sigma-1)G|.
fn coinvariant_order(
    elements: &[Residue],
    mul: &dyn Fn(Residue, Residue) -> Residue,
    inv_of: &dyn Fn(Residue) -> Residue,
    sigma: &dyn Fn(Residue) -> Residue,
    one: Residue,
) -> i64 {
    // the subgroup generated by { sigma(g) * g^{-1} }
    let mut gens: Vec<Residue> = Vec::new();
    for &g in elements {
        let c = mul(sigma(g), inv_of(g));
        if c != one && !gens.contains(&c) {
            gens.push(c);
        }
    }
    let mut seen: Vec<Residue> = vec![one];
    let mut frontier = vec![one];
    while let Some(x) = frontier.pop() {
        for &g in &gens {
            let y = mul(x, g);
            if !seen.contains(&y) {
                seen.push(y);
                frontier.push(y);
            }
        }
    }
    (elements.len() / seen.len()) as i64
}

/// Everything for dimension d at the Appleby modulus (3d), computed from d.
pub fn ray_data(d: i64) -> RayData {
    ray_data_at(d, 3 * d)
}

/// A solution of |N(x + y*omega)| = target, if one exists within the search box.
pub fn norm_equation(f: &RealQuad, target: i64, bound: i64) -> Option<(i64, i64)> {
    let mut y = 0i64;
    while y <= bound {
        let ys: Vec<i64> = if y == 0 { vec![0] } else { vec![y, -y] };
        for sy in ys {
            let mut x = 0i64;
            while x <= bound {
                let xs: Vec<i64> = if x == 0 { vec![0] } else { vec![x, -x] };
                for sx in xs {
                    let nm = f.norm(sx, sy);
                    if nm == target || nm == -target {
                        return Some((sx, sy));
                    }
                }
                x += 1;
            }
        }
        y += 1;
    }
    None
}

/// A rational prime whose ideal above it generates a non-principal class,
/// together with that prime. Returns None when the class number is one.
pub fn nonprincipal_prime(f: &RealQuad, m: i64, h: i64) -> Option<i64> {
    if h <= 1 {
        return None;
    }
    let mut q = 2i64;
    while q < 400 {
        if m % q != 0 && f.splitting(q) == Splitting::Split {
            // the prime above q is principal exactly when N(gamma) = +-q is solvable
            if norm_equation(f, q, 400).is_none() {
                return Some(q);
            }
        }
        q += 1;
    }
    None
}

/// Everything for dimension d at an arbitrary rational modulus.
pub fn ray_data_at(d: i64, m: i64) -> RayData {
    let m_d = (d - 3) * (d + 1);
    let f = RealQuad::for_dimension(d);
    let cg = class_group(&f);
    let r = ResidueRing::new(f, m);

    let units_mod_m = r.units();
    let n_units = units_mod_m.len();

    // image of the global units: generated by -1 and the fundamental unit
    let (ua, ub, _) = fundamental_unit(&f);
    let eps = ((ua).rem_euclid(m), (ub).rem_euclid(m));
    let minus_one = ((-1i64).rem_euclid(m), 0);
    let unit_image = generated_subgroup(&r, &[eps, minus_one]);

    let ray_over_class = (n_units / unit_image.len()) as i64;
    let ray_order = ray_over_class * cg.wide;

    // The quotient (O/m)^* / <units>, as cosets, carries the sigma action.
    // Coset representative: the lexicographically least element of the coset.
    let mut coset_of: BTreeMap<Residue, Residue> = BTreeMap::new();
    let mut reps: Vec<Residue> = Vec::new();
    for &x in &units_mod_m {
        if coset_of.contains_key(&x) {
            continue;
        }
        let mut coset: Vec<Residue> = unit_image.iter().map(|&u| r.mul(x, u)).collect();
        coset.sort();
        let rep = coset[0];
        for y in coset {
            coset_of.insert(y, rep);
        }
        reps.push(rep);
    }
    reps.sort();
    reps.dedup();

    let inv_of = |x: Residue| -> Residue {
        // inverse by search within the unit group: x^(order-1)
        let mut y = r.one();
        let mut cur = x;
        for _ in 0..n_units {
            if r.mul(x, cur) == r.one() {
                y = cur;
                break;
            }
            cur = r.mul(cur, x);
        }
        y
    };

    let mul_c = |x: Residue, y: Residue| -> Residue { coset_of[&r.mul(x, y)] };
    let inv_c = |x: Residue| -> Residue { coset_of[&inv_of(x)] };
    let sig_c = |x: Residue| -> Residue { coset_of[&r.conj(x)] };
    let one_c = coset_of[&r.one()];

    let quotient_coinv = coinvariant_order(&reps, &mul_c, &inv_c, &sig_c, one_c);

    // sigma acts on the class group by inversion, so its coinvariants are
    // Cl / Cl^2, of order two exactly when the class number is even.
    let class_coinvariant = if cg.wide % 2 == 0 { 2 } else { 1 };

    // With a nontrivial class group the ray class group is an extension of Cl
    // by the quotient above, and its coinvariants are not the product of the
    // two. Build the extension: pick a non-principal prime q, let x be the
    // class of the prime above it, of order n = h in Cl. Then x^n = [alpha]
    // for a generator alpha of the n-th power, and since p * sigma(p) = (q),
    // sigma(x) = x^{-1} * [q]. Coinvariants follow by explicit search.
    // The explicit extension below models a class group of order two, the case
    // the SIC argument turns on. Elsewhere fall back to the product, which is
    // exact when the class group is trivial and an upper bound otherwise.
    let coinvariant = if cg.wide <= 1 {
        quotient_coinv
    } else if cg.wide != 2 {
        quotient_coinv * class_coinvariant
    } else {
        let fallback = quotient_coinv * class_coinvariant;
        (|| -> i64 { match nonprincipal_prime(&f, m, cg.wide) {
            None => fallback,
            Some(q) => {
                let n = cg.wide;
                // alpha generates p^n, of norm +- q^n
                let target = q.pow(n as u32);
                let alpha = norm_equation(&f, target, 4000);
                // Without a generator for p^n the extension is not determined,
                // so say so by falling back rather than guessing a relation.
                let alpha_res = match alpha {
                    Some((x, y)) => match coset_of.get(&(x.rem_euclid(m), y.rem_euclid(m))) {
                        Some(&c) => c,
                        None => return fallback,
                    },
                    None => return fallback,
                };
                let q_res = coset_of
                    .get(&(q.rem_euclid(m), 0))
                    .copied()
                    .unwrap_or(one_c);

                // elements are (coset, j) with j in Z/n
                let mut elems: Vec<(Residue, i64)> = Vec::new();
                for &r0 in &reps {
                    for j in 0..n {
                        elems.push((r0, j));
                    }
                }
                let mul_e = |a: (Residue, i64), b: (Residue, i64)| -> (Residue, i64) {
                    let j = a.1 + b.1;
                    let carry = j >= n;
                    let mut r0 = mul_c(a.0, b.0);
                    if carry {
                        r0 = mul_c(r0, alpha_res);
                    }
                    (r0, j % n)
                };
                let one_e = (one_c, 0i64);
                let inv_e = |a: (Residue, i64)| -> (Residue, i64) {
                    let mut cur = a;
                    for _ in 0..(reps.len() as i64 * n) {
                        if cur == one_e {
                            break;
                        }
                        cur = mul_e(cur, a);
                    }
                    // cur is one; the previous power is the inverse
                    let mut inv = one_e;
                    let mut acc = one_e;
                    for _ in 0..(reps.len() as i64 * n) {
                        let nxt = mul_e(acc, a);
                        if nxt == one_e {
                            inv = acc;
                            break;
                        }
                        acc = nxt;
                    }
                    inv
                };
                let sig_e = |a: (Residue, i64)| -> (Residue, i64) {
                    // sigma(r * x^j) = conj(r) * x^{-j} * [q]^j
                    let mut r0 = sig_c(a.0);
                    for _ in 0..a.1 {
                        r0 = mul_c(r0, q_res);
                    }
                    let j = (-a.1).rem_euclid(n);
                    // x^{-j} = x^{n-j} with the carry correction folded in
                    (r0, j)
                };

                let mut gens: Vec<(Residue, i64)> = Vec::new();
                for &g in &elems {
                    let c = mul_e(sig_e(g), inv_e(g));
                    if c != one_e && !gens.contains(&c) {
                        gens.push(c);
                    }
                }
                let mut seen: Vec<(Residue, i64)> = vec![one_e];
                let mut frontier = vec![one_e];
                while let Some(x) = frontier.pop() {
                    for &g in &gens {
                        let y = mul_e(x, g);
                        if !seen.contains(&y) {
                            seen.push(y);
                            frontier.push(y);
                        }
                    }
                }
                (elems.len() / seen.len()) as i64
            }
        } })()
    };

    RayData {
        d,
        m_d,
        core: f.core,
        disc: f.disc,
        class_number: cg.wide,
        ray_over_class,
        ray_order,
        coinvariant,
        class_coinvariant,
        corrected: coinvariant / class_coinvariant,
        d_half: d / 2,
    }
}

/// The order of the wide ray class group at the modulus (m), computed.
/// For an inert 2 the ideal p_2^k is the rational ideal (2^k), so the tower
/// at p_2^k is read off by calling this with m = 2^k.
pub fn ray_order_at(d: i64, m: i64) -> i64 {
    ray_data_at(d, m).ray_order
}

/// Does the sigma-coinvariant identity hold at d: corrected count = d/2.
pub fn identity_holds(d: i64) -> bool {
    let r = ray_data(d);
    r.corrected == r.d_half
}

/// The predicted answer, from the shape of d/2 alone: the odd part of d/2
/// must be a power of three, since the modulus (3d) supplies 3-torsion via
/// 3^2 and supplies no other odd prime squared.
pub fn identity_predicted(d: i64) -> bool {
    let mut h = d / 2;
    while h % 2 == 0 {
        h /= 2;
    }
    while h % 3 == 0 {
        h /= 3;
    }
    h == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cores_and_discriminants() {
        assert_eq!(core(357), 357);
        assert_eq!(core(45), 5);
        assert_eq!(RealQuad::for_dimension(8).core, 5);
        assert_eq!(RealQuad::for_dimension(16).core, 221);
        assert_eq!(RealQuad::for_dimension(20).core, 357);
        assert_eq!(RealQuad::for_dimension(2048).core, 4190205);
    }

    #[test]
    fn class_numbers_are_computed() {
        assert_eq!(class_group(&RealQuad::for_dimension(12)).wide, 1);
        assert_eq!(class_group(&RealQuad::for_dimension(16)).wide, 2);
        assert_eq!(class_group(&RealQuad::for_dimension(20)).wide, 2);
    }

    #[test]
    fn two_is_inert_where_the_rule_needs_it() {
        assert_eq!(RealQuad::for_dimension(16).splitting(2), Splitting::Inert);
        assert_eq!(RealQuad::for_dimension(2048).splitting(2), Splitting::Inert);
    }

    #[test]
    fn prediction_matches_computation() {
        for d in [4, 8, 12, 16, 20, 24] {
            assert_eq!(identity_holds(d), identity_predicted(d), "d = {}", d);
        }
    }
}
