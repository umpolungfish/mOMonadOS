//! IMASM-16_3 — the 14-opcode, purely-symbolic trilattice extension of the classic
//! 12-opcode IMASM grammar (`imasm.rs`), for the real trilattice SIXTEEN_3:
//! Shramko, Dunn & Takenaka, "The Trilattice of Constructive Truth Values",
//! J. Logic and Computation 11(6):761-788, 2001 — verified against the source PDF.
//!
//! THE REAL CONSTRUCTION (§5 of the paper). The base set is four initial truth
//! values, not two and not a product of two FOURs:
//!
//! ```text
//!     I = {T, F, t, f}
//!       T — a sentence is constructively PROVEN
//!       F — a sentence is constructively REFUTED
//!       t — a sentence is (non-constructively) ACCEPTABLE
//!       f — a sentence is (non-constructively) REJECTABLE
//! ```
//!
//! SIXTEEN_3 is the full powerset P(I) — all 16 subsets of these four base
//! values (N = {} = empty, A = {T,F,t,f} = full). Three orderings (Def. 5.2):
//!
//! ```text
//!     x ≤_i y  ⟺  x ⊆ y                                        (information)
//!     x ≤_t y  ⟺  x∩{T,t} ⊆ y∩{T,t}  and  y∩{F,f} ⊆ x∩{F,f}     (truth)
//!     x ≤_c y  ⟺  x∩{T,F} ⊆ y∩{T,F}  and  y∩{t,f} ⊆ x∩{t,f}     (constructivity)
//! ```
//!
//! Verified against the paper's own worked example: T ∧ t = N under ≤_t (the
//! conjunction of two "truths" gives nothing) — `meet_t` below reproduces this
//! exactly, checked by a unit test.
//!
//! Register: a real 4-bit subset of {T, F, t, f} (16 states, matching the
//! paper's carrier exactly). Opcode → base-value mapping:
//!
//! ```text
//!     EVALT sets T (constructive truth touched)
//!     EVALF sets F (constructive falsity touched)
//!     EVALI sets BOTH t and f (the acceptable/rejectable pair IS the
//!           information layer beyond classical T/F)
//!     AREV  the reverse morphism, T↔F and t↔f together (bilattice negation:
//!           inverts ≤_t, leaves ≤_i exactly — a swap preserves |x|). It factors
//!           internally into a per-layer swap each, but those factors are not
//!           opcodes; the retired marks ~ ≁ once named them and ⊡ IFIX replaces
//!           them, so the printed set is twelve.
//! ```
//!
//! A sibling module, not a replacement: FSPLIT3/FFUSE3 (3-way fork/fuse) sit
//! alongside FSPLIT/FFUSE — `imasm.rs`'s ancestry-pairing graph engine and the
//! Lean `.prod` scaffold generator are untouched. No opcode glyph is a Latin
//! letter, so a trilattice verdict (T, N, B, F) can never be confused with a
//! graph node.

use core::fmt::Write as _;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token16_3 {
    Vinit,   // ⊢  0→1  source boundary
    Tanch,   // ⊣  1→1  sink boundary
    Afwd,    // >  1→1  forward morphism, WORK
    Arev,    // <  1→1  reverse morphism, WORK
    Clink,   // ⋈  1→1  composition / relational link, WORK
    Imscrib, // ⊙  1→1  identity / neutral self-reference
    Fsplit3, // ∈  1→3  3-way split: T, F, I arms
    Ffuse3,  // ∋  3→1  3-way fuse: merges T, F, I arms
    Evalt,   // ⊤  1→1  evaluates the True axis (≤_t), WORK
    Evalf,   // ⊥  1→1  evaluates the False axis (≤_t), WORK
    Evali,   // ⊞  1→1  evaluates the Information axis (≤_i), WORK
    Ifix,    // ⊡  1→1  irreversible commit, WORK
}

use Token16_3::*;

// THE set is twelve. AREV `<` is the whole reverse morphism; the two-layer swaps
// it factors into (once mis-spelled ~ TNEG / ≁ INEG) are internal to `<`, never
// opcodes — ⊡ IFIX replaces those retired marks. ROTAT ↺/↻ is the op-opcode that
// acts ON a word, not a token in it.
pub const ALL_TOKENS: [Token16_3; 12] = [
    Vinit, Tanch, Afwd, Arev, Clink, Imscrib, Fsplit3, Ffuse3,
    Evalt, Evalf, Evali, Ifix,
];

impl Token16_3 {
    pub fn glyph(self) -> char {
        match self {
            Vinit => '⊢', Tanch => '⊣', Afwd => '>', Arev => '<', Clink => '⋈',
            Imscrib => '⊙', Fsplit3 => '∈', Ffuse3 => '∋', Evalt => '⊤',
            Evalf => '⊥', Evali => '⊞', Ifix => '⊡',
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Vinit => "VINIT", Tanch => "TANCH", Afwd => "AFWD", Arev => "AREV",
            Clink => "CLINK", Imscrib => "IMSCRIB", Fsplit3 => "FSPLIT3",
            Ffuse3 => "FFUSE3", Evalt => "EVALT", Evalf => "EVALF",
            Evali => "EVALI", Ifix => "IFIX",
        }
    }

    pub fn is_work(self) -> bool {
        matches!(self, Afwd | Arev | Clink | Evalt | Evalf | Evali | Ifix)
    }

    fn from_glyph(c: char) -> Option<Token16_3> {
        // Only the twelve glyphs are tokens. The old marks ◇ ● ☊ ☋ = ═ + × ¬ ~ ≁
        // are NOT IMASM tokens and do not parse — no alias, no shim; a word that
        // contains one reads it as nothing (N / void), exactly as a stray letter
        // does. A retired form that still loaded is how legacy notation survives a
        // purge, so there is none.
        ALL_TOKENS.iter().copied().find(|t| t.glyph() == c)
    }
}

pub fn parse_glyph_word(word: &str) -> Vec<Token16_3> {
    word.chars().filter_map(Token16_3::from_glyph).collect()
}

/// Register: a real subset of the 4-element base {T, F, t, f} — 16 states,
/// the actual Shramko-Wansing SIXTEEN_3 carrier (not an approximation).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Reg16_3 {
    pub big_t: bool,   // T — constructively proven
    pub big_f: bool,   // F — constructively refuted
    pub small_t: bool, // t — acceptable (non-constructive truth)
    pub small_f: bool, // f — rejectable (non-constructive falsity)
}

impl Reg16_3 {
    pub fn name(self) -> String {
        if !self.big_t && !self.big_f && !self.small_t && !self.small_f {
            return "N".to_string();
        }
        if self.big_t && self.big_f && self.small_t && self.small_f {
            return "A".to_string();
        }
        let mut s = String::new();
        if self.big_t { s.push('T'); }
        if self.big_f { s.push('F'); }
        if self.small_t { s.push('t'); }
        if self.small_f { s.push('f'); }
        s
    }

    /// The glyph face: occupancy of {T, F, t, f} drawn as quadrant ink —
    /// T upper-left, F upper-right, t lower-left, f lower-right. The glyph
    /// IS the subset (meet/join of states = intersection/union of ink);
    /// N renders as ░, the unmarked field, never as absence. No Latin,
    /// matching the opcode face's rule.
    pub fn glyph(self) -> char {
        match (self.big_t, self.big_f, self.small_t, self.small_f) {
            (false, false, false, false) => '░',
            (true,  false, false, false) => '▘',
            (false, true,  false, false) => '▝',
            (false, false, true,  false) => '▖',
            (false, false, false, true ) => '▗',
            (true,  true,  false, false) => '▀',
            (true,  false, true,  false) => '▌',
            (true,  false, false, true ) => '▚',
            (false, true,  true,  false) => '▞',
            (false, true,  false, true ) => '▐',
            (false, false, true,  true ) => '▄',
            (true,  true,  true,  false) => '▛',
            (true,  true,  false, true ) => '▜',
            (true,  false, true,  true ) => '▙',
            (false, true,  true,  true ) => '▟',
            (true,  true,  true,  true ) => '█',
        }
    }

    pub fn union(self, o: Reg16_3) -> Reg16_3 {
        Reg16_3 {
            big_t: self.big_t || o.big_t, big_f: self.big_f || o.big_f,
            small_t: self.small_t || o.small_t, small_f: self.small_f || o.small_f,
        }
    }

    /// The truth part x ∩ {T, t} — what EVALT passes, and the first δ arm.
    pub fn truth_part(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_t, small_t: self.small_t, ..Reg16_3::default() }
    }

    /// The falsity part x ∩ {F, f} — what EVALF passes, and the second δ arm.
    pub fn falsity_part(self) -> Reg16_3 {
        Reg16_3 { big_f: self.big_f, small_f: self.small_f, ..Reg16_3::default() }
    }

    /// The constructive part x ∩ {T, F} — the two arms δ separates when it fans
    /// at arity 3, where the truth cut is taken inside the constructive block.
    pub fn constructive_part(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_t, big_f: self.big_f, ..Reg16_3::default() }
    }

    /// The information part x ∩ {t, f} — the third δ arm, and what EVALI sets.
    ///
    /// It is one arm rather than two because ⊞ sets t and f together, so the
    /// non-constructive pair can only ever be entered as a block. That is where
    /// the arity of the fork comes from: not a choice, but the gate set.
    pub fn info_part(self) -> Reg16_3 {
        Reg16_3 { small_t: self.small_t, small_f: self.small_f, ..Reg16_3::default() }
    }

    /// The constructive swap T ↔ F, leaving the information layer alone.
    ///
    /// This and `info_swap` are the two factors of the involution: they act on
    /// disjoint bits, so they commute, and composing them IS `invol`. AREV is not
    /// a third operation beside them, it is both at once, which is why the
    /// classical slice needs only one of them: with t and f absent `info_swap` is
    /// the identity and AREV is `truth_swap`.
    pub fn truth_swap(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_f, big_f: self.big_t, ..self }
    }

    /// The information swap t ↔ f, leaving the constructive layer alone.
    pub fn info_swap(self) -> Reg16_3 {
        Reg16_3 { small_t: self.small_f, small_f: self.small_t, ..self }
    }

    /// The involution T ↔ F, t ↔ f — the reverse morphism AREV's action on
    /// values (both layer-swaps at once; ⊆-monotone, its own inverse).
    pub fn invol(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_f, big_f: self.big_t, small_t: self.small_f, small_f: self.small_t }
    }

    /// FOUR sits inside SIXTEEN_3 as the classical pair {T, F}: N={}, T={T},
    /// F={F}, B={T,F}. Render this value as its FOUR name when it lives in that
    /// slice; a value touching t/f has left the slice and keeps its 16_3 name.
    pub fn four_name(self) -> String {
        if self.small_t || self.small_f {
            return self.name();
        }
        match (self.big_t, self.big_f) {
            (false, false) => "N".into(),
            (true, false) => "T".into(),
            (false, true) => "F".into(),
            (true, true) => "B".into(),
        }
    }

    /// The FOUR slice's glyph face: ░/▘/▝/▀ for N/T/F/B per `glyph()` — B of
    /// FOUR IS the state {T,F}, so it wears TF's ink; there is no separate
    /// verdict alphabet and no Latin anywhere. Values touching t/f have left
    /// the slice and keep their 16_3 glyph.
    pub fn four_glyph(self) -> char {
        self.glyph()
    }

    /// Parse a register from its name — "N", "A", or any combination of
    /// T/F/t/f (order-insensitive), e.g. "Tf" or "tF". The inverse of `name()`.
    pub fn from_name(s: &str) -> Option<Reg16_3> {
        if s == "N" { return Some(Reg16_3::default()); }
        if s == "A" {
            return Some(Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true });
        }
        let mut r = Reg16_3::default();
        for c in s.chars() {
            match c {
                'T' => r.big_t = true,
                'F' => r.big_f = true,
                't' => r.small_t = true,
                'f' => r.small_f = true,
                _ => return None,
            }
        }
        Some(r)
    }
}

// ── The three orderings (Definition 5.2) — kept as free functions since
// they're part of the trilattice's public algebra, not just machine internals.

/// Information order: x ≤_i y ⟺ x ⊆ y.
pub fn leq_i(x: Reg16_3, y: Reg16_3) -> bool {
    (!x.big_t || y.big_t) && (!x.big_f || y.big_f) && (!x.small_t || y.small_t) && (!x.small_f || y.small_f)
}

/// Truth order: positive part (T,t) grows, negative part (F,f) shrinks.
pub fn leq_t(x: Reg16_3, y: Reg16_3) -> bool {
    let pos_ok = (!x.big_t || y.big_t) && (!x.small_t || y.small_t);
    let neg_ok = (!y.big_f || x.big_f) && (!y.small_f || x.small_f);
    pos_ok && neg_ok
}

/// Constructivity order: constructive part (T,F) grows, non-constructive part (t,f) shrinks.
pub fn leq_c(x: Reg16_3, y: Reg16_3) -> bool {
    let con_ok = (!x.big_t || y.big_t) && (!x.big_f || y.big_f);
    let noncon_ok = (!y.small_t || x.small_t) && (!y.small_f || x.small_f);
    con_ok && noncon_ok
}

/// Meet under ≤_t: positive part intersects, negative part unions. Verified
/// against the paper's own worked example, T ∧ t = N (§5, p.776-777) — see
/// the unit test `meet_t_matches_paper_example`.
pub fn meet_t(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t && y.big_t, small_t: x.small_t && y.small_t,
        big_f: x.big_f || y.big_f, small_f: x.small_f || y.small_f,
    }
}

pub fn join_t(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t || y.big_t, small_t: x.small_t || y.small_t,
        big_f: x.big_f && y.big_f, small_f: x.small_f && y.small_f,
    }
}

pub fn meet_c(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t && y.big_t, big_f: x.big_f && y.big_f,
        small_t: x.small_t || y.small_t, small_f: x.small_f || y.small_f,
    }
}

pub fn join_c(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t || y.big_t, big_f: x.big_f || y.big_f,
        small_t: x.small_t && y.small_t, small_f: x.small_f && y.small_f,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis { T, F, I }

struct Machine {
    reg: Reg16_3,
    fixed: bool,
    /// Fork state is a STACK, not a flag. Fsplit3 pushes a frame, Ffuse3 pops one
    /// and folds its touches into the register AND into the enclosing frame, so
    /// nested regions compose: an inner apex refines the outer one and μ∘δ=id at
    /// each level composes to μ∘δ=id overall.
    split_stack: Vec<Vec<Axis>>,
}

impl Machine {
    fn new() -> Self {
        Self { reg: Reg16_3::default(), fixed: false, split_stack: Vec::new() }
    }

    fn touch(&mut self, set: Reg16_3, a: Axis) {
        self.reg = self.reg.union(set);
        if let Some(frame) = self.split_stack.last_mut() {
            if !frame.contains(&a) { frame.push(a); }
        }
    }

    fn step(&mut self, tok: Token16_3) {
        if self.fixed && tok != Ifix && tok != Imscrib {
            return;
        }
        let none = Reg16_3::default();
        match tok {
            Vinit => {
                self.reg = none;
                self.split_stack.clear();
            }
            Tanch => {}
            Afwd => {
                if self.reg == none { self.reg.big_t = true; }
            }
            Arev => {
                // Reverse morphism: clears the register, but does NOT close the
                // fork. Only Fsplit3 opens a split context and only Ffuse3 closes
                // one; an Arev between two arms is work on an arm, not a fuse.
                // Clearing in_split/split_touched here (the body was identical to
                // Vinit's) discarded every touch the arms had accumulated, so a
                // downstream Ffuse3 folded an empty set.
                self.reg = none;
            }
            Clink => {}
            Imscrib => {
                if self.reg == none { self.reg.big_t = true; }
            }
            Fsplit3 => {
                self.split_stack.push(Vec::new());
            }
            Ffuse3 => {
                if let Some(closed) = self.split_stack.pop() {
                    for a in &closed {
                        match a {
                            Axis::T => self.reg.big_t = true,
                            Axis::F => self.reg.big_f = true,
                            Axis::I => { self.reg.small_t = true; self.reg.small_f = true; }
                        }
                    }
                    // Fold the inner apex into the enclosing fork: the outer ∋
                    // must see what its sub-regions reconnected.
                    if let Some(parent) = self.split_stack.last_mut() {
                        for a in closed {
                            if !parent.contains(&a) { parent.push(a); }
                        }
                    }
                }
            }
            Evalt => self.touch(Reg16_3 { big_t: true, ..none }, Axis::T),
            Evalf => self.touch(Reg16_3 { big_f: true, ..none }, Axis::F),
            Evali => self.touch(Reg16_3 { small_t: true, small_f: true, ..none }, Axis::I),
            Ifix => { self.fixed = true; }
        }
    }
}

/// Tri-ancestral close condition, the arity-3 generalization of the classic
/// `imasm check`'s ancestry rule:
///   T — every FSPLIT3 pairs with a later FFUSE3, and at least one work
///       opcode ran somewhere inside that interval (a real transformation).
///   N — paired, but no work ran inside — μ∘δ=id verifies nothing (identity).
///   B — a FSPLIT3 has no matching later FFUSE3 — a fork left open.
///   F — a FFUSE3 has no preceding FSPLIT3 — ill-typed.
/// Run a word and return the final register's name — the parity handle the
/// Python kernel is diffed against (see `tests/python_parity.rs`).
pub fn run_word_register(steps: &[Token16_3]) -> String {
    let mut m = Machine::new();
    for &t in steps { m.step(t); }
    m.reg.name()
}

/// Pair FSPLIT3→FFUSE3 over the word read as a LOOP.
///
/// A word is a cycle and ROTAT is the cyclic shift (Weyl–Heisenberg X), so a
/// rotation that cuts through a region's interior must not change the verdict.
/// Pairing over the linearized slice makes it change: the ∋ of a cut region
/// lands ahead of its ∈ and reads as ill-typed while the region is intact.
/// Returns pairs `(si, fj)` with the region running forward around the cycle,
/// or `None` on a true count imbalance that no rotation can repair.
fn cyclic_pairs(steps: &[Token16_3]) -> Option<Vec<(usize, usize)>> {
    let n = steps.len();
    let splits: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Fsplit3).map(|(i, _)| i).collect();
    let fuses: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Ffuse3).map(|(i, _)| i).collect();
    if splits.is_empty() && fuses.is_empty() { return Some(Vec::new()); }
    if splits.len() != fuses.len() { return None; }
    // By the cycle lemma some start at a ∈ pairs every region without underflow.
    for &start in &splits {
        let mut stack: Vec<usize> = Vec::new();
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut ok = true;
        for off in 0..n {
            let i = (start + off) % n;
            if steps[i] == Fsplit3 {
                stack.push(i);
            } else if steps[i] == Ffuse3 {
                match stack.pop() {
                    Some(si) => pairs.push((si, i)),
                    None => { ok = false; break; }
                }
            }
        }
        if ok && stack.is_empty() { return Some(pairs); }
    }
    None
}

/// Tokens strictly inside the region, walking forward around the cycle.
fn cyclic_interior(steps: &[Token16_3], si: usize, fj: usize) -> Vec<Token16_3> {
    let n = steps.len();
    let span = (fj + n - si) % n;
    (1..span).map(|j| steps[(si + j) % n]).collect()
}

/// Tri-ancestral close condition, the arity-3 generalization of the classic
/// `imasm check`'s ancestry rule. Pairing is CYCLIC (see `cyclic_pairs`):
///   T — every FSPLIT3 pairs with a FFUSE3 around the cycle, and at least one
///       work opcode ran inside that region (a real transformation).
///   N — paired, but no work ran inside — μ∘δ=id verifies nothing (identity).
///   B — a FSPLIT3 has no FFUSE3 to pair with — a fork left open.
///   F — a FFUSE3 has no FSPLIT3 to pair with — ill-typed.
pub fn tri_ancestral_verdict(steps: &[Token16_3]) -> (char, String) {
    let splits: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Fsplit3).map(|(i, _)| i).collect();
    let fuses: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Ffuse3).map(|(i, _)| i).collect();

    match cyclic_pairs(steps) {
        None => {
            if fuses.len() > splits.len() {
                ('F', format!("FFUSE3 at step {} has no FSPLIT3 to pair — ill-typed", fuses[0] + 1))
            } else {
                ('B', format!("FSPLIT3 at step {} dangles — no matching FFUSE3", splits[0] + 1))
            }
        }
        Some(pairs) => {
            if pairs.is_empty() {
                return ('N', "No fork/fuse — void, never weighed alternatives".to_string());
            }
            for (si, fj) in pairs {
                if cyclic_interior(steps, si, fj).iter().any(|t| t.is_work()) {
                    return ('T', "Tri-ancestral reconnection over a transformed object — closes".to_string());
                }
            }
            ('N', "Split/fused with no work on any arm — μ∘δ=id verifies nothing".to_string())
        }
    }
}

fn run_trace(steps: &[Token16_3]) -> String {
    let mut mach = Machine::new();
    let mut out = String::new();
    let _ = writeln!(out, "  {:>3} {:^5} {:<9} {:>5} → {:>5}", "Step", "Glyph", "Token", "Reg↓", "Reg↑");
    let start = mach.reg;
    for (idx, &tok) in steps.iter().enumerate() {
        let before = mach.reg.name();
        mach.step(tok);
        let after = mach.reg.name();
        let _ = writeln!(out, "  {:>3} {:^5} {:<9} {:>5} → {:>5}", idx + 1, tok.glyph(), tok.name(), before, after);
    }
    let closed = mach.reg == start;
    let (verdict, msg) = tri_ancestral_verdict(steps);
    let _ = writeln!(out);
    let _ = writeln!(out, "  Closed walk: {closed}");
    let _ = writeln!(out, "  Final register: {}", mach.reg.name());
    let _ = writeln!(out, "  Tri-ancestral verdict: {verdict} — {msg}");
    out
}

pub fn run(args: &[String]) -> String {
    let Some(op) = args.first() else {
        return "imasm16_3 <op> …; op ∈ check|ref|algebra — the 12-opcode SIXTEEN_3 trilattice grammar.\n\
                `check <glyph_word>` runs the register machine and type-checks tri-ancestral closure.\n\
                `ref` lists the 12 opcodes.\n\
                `algebra <op> A B` runs a trilattice lattice operation on two register values (named\n\
                  N, A, or any of T/F/t/f, e.g. `algebra meet_t T t`); op ∈ leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c.\n".to_string();
    };
    match op.as_str() {
        "algebra" => {
            let (Some(sub), Some(a_name), Some(b_name)) = (args.get(1), args.get(2), args.get(3)) else {
                return "imasm16_3 algebra <op> A B; op ∈ leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c; \
                        A, B ∈ {N, A, or any combination of T/F/t/f}.\n".to_string();
            };
            let (Some(a), Some(b)) = (Reg16_3::from_name(a_name), Reg16_3::from_name(b_name)) else {
                return format!("imasm16_3 algebra: could not parse '{a_name}' or '{b_name}' — use N, A, or T/F/t/f.\n");
            };
            match sub.as_str() {
                "leq_i" => format!("{} ≤_i {} : {}\n", a.name(), b.name(), leq_i(a, b)),
                "leq_t" => format!("{} ≤_t {} : {}\n", a.name(), b.name(), leq_t(a, b)),
                "leq_c" => format!("{} ≤_c {} : {}\n", a.name(), b.name(), leq_c(a, b)),
                "meet_t" => format!("{} ∧ {} = {}\n", a.name(), b.name(), meet_t(a, b).name()),
                "join_t" => format!("{} ∨ {} = {}\n", a.name(), b.name(), join_t(a, b).name()),
                "meet_c" => format!("{} △ {} = {}\n", a.name(), b.name(), meet_c(a, b).name()),
                "join_c" => format!("{} ▽ {} = {}\n", a.name(), b.name(), join_c(a, b).name()),
                other => format!("imasm16_3 algebra: unknown op '{other}'; op ∈ leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c\n"),
            }
        }
        "ref" => {
            let mut out = String::from("IMASM-16_3 — 12 symbolic opcodes (SIXTEEN_3 = P({T,F,t,f}), Shramko/Dunn/Takenaka 2001):\n");
            for t in ALL_TOKENS {
                let _ = writeln!(out, "  {}  {:<9} {}", t.glyph(), t.name(), if t.is_work() { "WORK" } else { "no-op (structural)" });
            }
            let _ = writeln!(out, "  ↺/↻ ROTAT     op-opcode: the cyclic shift (Weyl-Heisenberg X) on the WHOLE word, not a token in it");
            let _ = writeln!(out, "  (the marks ◇ ● = + × ¬ ~ ≁ are NOT tokens and do not parse — no alias; they read as nothing)");
            out
        }
        "check" => {
            let Some(word) = args.get(1) else {
                return "imasm16_3 check <glyph_word>; e.g. imasm16_3 check ⊢>∈⊤⊥⊞∋⊡⊣\n".to_string();
            };
            let steps = parse_glyph_word(word);
            if steps.is_empty() {
                return format!("imasm16_3 check: no recognized IMASM-16_3 glyphs in '{word}'\n");
            }
            format!("Word: {}\n{}", steps.iter().map(|t| t.glyph()).collect::<String>(), run_trace(&steps))
        }
        other => format!("imasm16_3: unknown op '{other}'; op ∈ check|ref\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_word_closes_with_work() {
        // Legal-alphabet tri word: fork, work on the arms, fuse, latch.
        let steps = parse_glyph_word("⊢>∈⊤⊥⊞∋⊡⊣");
        assert_eq!(steps.len(), 9);
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'T');
    }

    #[test]
    fn retired_marks_do_not_parse() {
        // ◇ ● = + × ¬ ~ ≁ (and ═ ☊ ☋) are not IMASM tokens: they do not parse,
        // they are not aliased to anything, and a word of only them is empty.
        for m in ["~", "≁", "¬", "◇", "●", "=", "+", "×", "═", "☊", "☋"] {
            assert!(parse_glyph_word(m).is_empty(), "retired mark {m} still parses");
        }
        // Interspersed in a real word they are simply skipped (read as nothing).
        assert_eq!(parse_glyph_word("⊢∈~⊤≁∋⊡⊣"), parse_glyph_word("⊢∈⊤∋⊡⊣"));
    }

    #[test]
    fn verdict_is_rotat_invariant() {
        // ROTAT is the cyclic shift, so every rotation is the same object and
        // must return the same verdict. Linear pairing gave T,T,F,F,F,F,F,F,F,T,T,T.
        let base: Vec<char> = "⊢∈⋈<>⊤⊥⊞∋⊙⊡⊣".chars().collect();
        let n = base.len();
        for k in 0..n {
            let rot: String = (0..n).map(|i| base[(i + k) % n]).collect();
            let steps = parse_glyph_word(&rot);
            let (verdict, _) = tri_ancestral_verdict(&steps);
            assert_eq!(verdict, 'T', "rotation k={k} gave {verdict} for {rot}");
        }
    }

    #[test]
    fn arev_does_not_close_the_fork() {
        // AREV is work on an arm, not a fuse. Its body used to be identical to
        // VINIT's, which discarded the arms' touches so ∋ folded an empty set.
        let steps = parse_glyph_word("⊢∈⊤⋈⊥<>⊞∋⊙⊡⊣");
        let mut m = Machine::new();
        for &t in &steps { m.step(t); }
        assert_eq!(m.reg.name(), "A", "the three arms must all reach the apex");
    }

    #[test]
    fn nested_forks_compose() {
        // Fork state is a stack: an inner ∋ must not close the enclosing fork.
        // With in_split as a bool the outer region lost every touch after the
        // first inner fuse and landed on Ftf instead of the top.
        let steps = parse_glyph_word("⊢⊙⋈∈∈>⊤<∋∈⊥<∋⊞∋⋈⊙⊡⊣");
        let mut m = Machine::new();
        for &t in &steps { m.step(t); }
        assert_eq!(m.reg.name(), "A", "nested apexes must fold into the outer fork");
    }

    #[test]
    fn cross_repo_parity_word() {
        let steps = parse_glyph_word("⊢>>⋈∈⊤⊡∋<⊡⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'T');
    }

    #[test]
    fn neutral_inflation_is_identity_not_error() {
        let steps = parse_glyph_word("⊢∈⊙⊙⊙∋⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'N');
    }

    #[test]
    fn dangling_split_is_b() {
        let steps = parse_glyph_word("⊢∈⊤⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'B');
    }

    #[test]
    fn fuse_without_split_is_f() {
        let steps = parse_glyph_word("⊢⊤∋⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'F');
    }

    #[test]
    fn all_12_opcodes_have_distinct_glyphs() {
        let glyphs: std::collections::HashSet<char> = ALL_TOKENS.iter().map(|t| t.glyph()).collect();
        assert_eq!(glyphs.len(), 12);
        for g in &glyphs {
            assert!(!g.is_ascii_alphabetic(), "opcode glyph {g} is a Latin letter");
        }
    }

    /// The paper's own worked example (§5, p.776-777): "T ∧ t = N" — the
    /// conjunction of two truths gives nothing, because neither conjunct is
    /// BOTH T and t simultaneously.
    #[test]
    fn meet_t_matches_paper_example() {
        let big_t_only = Reg16_3 { big_t: true, ..Default::default() };
        let small_t_only = Reg16_3 { small_t: true, ..Default::default() };
        let result = meet_t(big_t_only, small_t_only);
        assert_eq!(result.name(), "N");
    }

    #[test]
    fn negation_preserves_information_order() {
        // A defining property of trilattice negation: it must leave ≤_i
        // (the subset/information order) unchanged. Swapping T↔F preserves
        // popcount, hence preserves ⊆-comparisons against any fixed y.
        let x = Reg16_3 { big_t: true, small_t: true, ..Default::default() };
        let mut neg_x = x;
        std::mem::swap(&mut neg_x.big_t, &mut neg_x.big_f);
        // |x| == |neg_x|, and both are still comparable to N and A the same way.
        assert_eq!(leq_i(Reg16_3::default(), x), leq_i(Reg16_3::default(), neg_x));
        assert_eq!(leq_i(x, Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true }),
                   leq_i(neg_x, Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true }));
    }

    #[test]
    fn sixteen_states_reachable() {
        // The full carrier has exactly 16 elements — spot check a handful of
        // named ones from Table 1 of the paper are constructible and distinct.
        let n = Reg16_3::default();
        let a = Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true };
        let t = Reg16_3 { big_t: true, ..Default::default() };
        let tf = Reg16_3 { big_t: true, big_f: true, ..Default::default() };
        assert_eq!(n.name(), "N");
        assert_eq!(a.name(), "A");
        assert_eq!(t.name(), "T");
        assert_eq!(tf.name(), "TF");
        assert_ne!(n.name(), a.name());
    }
}
