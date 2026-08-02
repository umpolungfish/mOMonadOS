//! The classic close condition as a portable engine — Graph, ancestry
//! pairing, and ClosureState, MOVED from ask_native/src/imasm.rs so the gate
//! can run wherever the kernel runs (no_std + alloc). Presentation, spectral
//! analysis, and the tool registry remain in ask_native as extensions.

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::classic::Token;

/// μ∘δ closure state — the real close condition (δ arms reconnecting at μ),
/// independent of whether the graph merely has a cycle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClosureState {
    Closed(usize), // δ arms transformed AND rejoined at μ: a real type-check
    Identity,      // δ/μ reconnect but nothing transforms between — μ∘δ=id, no work
    Open,          // δ/μ present but a fork or fuse dangles unreconnected
    None,          // no δ/μ dyad at all — a bare line or cycle is NOT a closure
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Token>,
    pub edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn add(&mut self, t: Token) -> usize {
        self.nodes.push(t);
        self.nodes.len() - 1
    }

    pub fn connect(&mut self, a: usize, b: usize) {
        self.edges.push((a, b));
    }

    /// Add a fresh path of `toks`, wire head→tail, return the node ids.
    pub fn chain_of(&mut self, toks: &[Token]) -> Vec<usize> {
        let ids: Vec<usize> = toks.iter().map(|&t| self.add(t)).collect();
        for w in ids.windows(2) {
            self.connect(w[0], w[1]);
        }
        ids
    }

    pub fn out_degree(&self, n: usize) -> usize {
        self.edges.iter().filter(|&&(a, _)| a == n).count()
    }

    pub fn in_degree(&self, n: usize) -> usize {
        self.edges.iter().filter(|&&(_, b)| b == n).count()
    }

    pub fn successors(&self, n: usize) -> Vec<usize> {
        self.edges.iter().filter(|&&(a, _)| a == n).map(|&(_, b)| b).collect()
    }

    pub fn predecessors(&self, n: usize) -> Vec<usize> {
        self.edges.iter().filter(|&&(_, b)| b == n).map(|&(a, _)| a).collect()
    }

    // ── invariants ──────────────────────────────────────────────────────────

    /// Weakly-connected component count (edges read undirected).
    pub fn components(&self) -> usize {
        let n = self.nodes.len();
        if n == 0 {
            return 0;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for &(a, b) in &self.edges {
            let ra = find(&mut parent, a);
            let rb = find(&mut parent, b);
            if ra != rb {
                parent[ra] = rb;
            }
        }
        let mut roots: Vec<usize> = (0..n).map(|i| find(&mut parent, i)).collect();
        roots.sort_unstable();
        roots.dedup();
        roots.len()
    }

    /// First Betti number β = E − V + C: the number of independent loops.
    pub fn circuit_rank(&self) -> i64 {
        self.edges.len() as i64 - self.nodes.len() as i64 + self.components() as i64
    }

    /// For each node, which FSPLIT nodes can reach it going forward. `anc_or_self`
    /// also counts a node that IS an FSPLIT as its own ancestor, so an FSPLIT that
    /// feeds an FFUSE directly (an empty-branch arm) still counts as that arm.
    pub fn fsplit_ancestors(&self) -> Vec<Vec<usize>> {
        let n = self.nodes.len();
        let mut anc = vec![Vec::new(); n];
        for f in 0..n {
            if self.nodes[f] != Token::Fsplit {
                continue;
            }
            let mut seen = vec![false; n];
            let mut stack = vec![f];
            seen[f] = true;
            while let Some(x) = stack.pop() {
                for s in self.successors(x) {
                    if !seen[s] {
                        seen[s] = true;
                        anc[s].push(f);
                        stack.push(s);
                    }
                }
            }
        }
        anc
    }

    /// The REAL close condition: δ arms reconnecting at μ. A μ∘δ closure is an
    /// (FSPLIT, FFUSE) pair where two distinct in-arms of the FFUSE trace back to
    /// a common FSPLIT — the fork was undone by the fuse, however it routed. A bare
    /// cycle (β≥1 with no δ/μ) is NOT a closure. Returns (reconnection pairs,
    /// fully_closed) where fully_closed means every δ and every μ participates.
    pub fn frobenius_closures(&self) -> (Vec<(usize, usize)>, bool) {
        let anc = self.fsplit_ancestors();
        let anc_or_self = |node: usize| -> Vec<usize> {
            let mut v = anc[node].clone();
            if self.nodes[node] == Token::Fsplit && !v.contains(&node) {
                v.push(node);
            }
            v
        };
        let fsplits: Vec<usize> =
            (0..self.nodes.len()).filter(|&i| self.nodes[i] == Token::Fsplit).collect();
        let mut pairs = Vec::new();
        for j in 0..self.nodes.len() {
            if self.nodes[j] != Token::Ffuse {
                continue;
            }
            // in-edges WITH multiplicity — an FFUSE fed twice by the same δ (empty
            // arms) still reconnects. A δ whose arms feed ≥2 of j's in-ports closes.
            let in_edges: Vec<usize> =
                self.edges.iter().filter(|&&(_, b)| b == j).map(|&(a, _)| a).collect();
            // Every δ whose arms feed ≥2 of j's in-ports is a candidate. On a strand an
            // UPSTREAM δ reaches every later μ, so it qualifies at every one of them —
            // taking the first candidate would let it claim them all and starve the δ
            // that actually forked here, reporting Open for a correctly wired program.
            // Pair j with the INNERMOST candidate instead: the one no other candidate
            // descends from. A δ may close more than one μ; a μ closes with exactly one δ.
            let cands: Vec<usize> = fsplits
                .iter()
                .copied()
                .filter(|&f| in_edges.iter().filter(|&&p| anc_or_self(p).contains(&f)).count() >= 2)
                .collect();
            if let Some(&f) = cands
                .iter()
                .find(|&&f| !cands.iter().any(|&g| g != f && anc[g].contains(&f)))
            {
                pairs.push((f, j));
            }
        }
        let n_split = self.nodes.iter().filter(|&&t| t == Token::Fsplit).count();
        let n_fuse = self.nodes.iter().filter(|&&t| t == Token::Ffuse).count();
        let closed_splits: alloc::collections::BTreeSet<usize> = pairs.iter().map(|&(f, _)| f).collect();
        let closed_fuses: alloc::collections::BTreeSet<usize> = pairs.iter().map(|&(_, j)| j).collect();
        let fully = (n_split > 0 || n_fuse > 0)
            && closed_splits.len() == n_split
            && closed_fuses.len() == n_fuse;
        (pairs, fully)
    }

    /// Nodes strictly on a path from `f` forward to `j`: forward-reachable from f
    /// AND backward-reachable from j (endpoints excluded). These are the arms.
    pub fn between(&self, f: usize, j: usize) -> Vec<usize> {
        let n = self.nodes.len();
        let reach = |start: usize, fwd: bool| -> Vec<bool> {
            let mut seen = vec![false; n];
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(x) = stack.pop() {
                let nbrs: Vec<usize> = if fwd {
                    self.successors(x)
                } else {
                    self.predecessors(x)
                };
                for y in nbrs {
                    if !seen[y] {
                        seen[y] = true;
                        stack.push(y);
                    }
                }
            }
            seen
        };
        let fwd = reach(f, true);
        let bwd = reach(j, false);
        (0..n).filter(|&k| k != f && k != j && fwd[k] && bwd[k]).collect()
    }

    /// Does a reconnection (f→…→j) carry a TRANSFORMATION — i.e. do the arms
    /// between the split and the fuse do real work (a transforming opcode), rather
    /// than pass the object through unchanged? This is the crux: split→fuse with
    /// nothing between is μ∘δ=id (identity), which type-checks nothing.
    pub fn transforms_between(&self, f: usize, j: usize) -> bool {
        self.between(f, j).into_iter().any(|k| self.nodes[k].transforms())
    }

    /// The μ∘δ closure state. A real closure (type-check) needs BOTH: δ arms that
    /// reconnect at μ, AND a transformation carried on those arms. Reconnection with
    /// no transformation is Identity (μ∘δ=id, no work). No δ/μ dyad at all is None —
    /// a bare line or cycle is not a closure (β is not diagnostic).
    pub fn closure_state(&self) -> ClosureState {
        let n_split = self.nodes.iter().filter(|&&t| t == Token::Fsplit).count();
        let n_fuse = self.nodes.iter().filter(|&&t| t == Token::Ffuse).count();
        if n_split == 0 && n_fuse == 0 {
            return ClosureState::None;
        }
        let (pairs, fully) = self.frobenius_closures();
        if !fully {
            return ClosureState::Open;
        }
        if pairs.iter().any(|&(f, j)| self.transforms_between(f, j)) {
            ClosureState::Closed(pairs.len())
        } else {
            ClosureState::Identity
        }
    }

    pub fn branch_points(&self) -> Vec<usize> {
        (0..self.nodes.len()).filter(|&n| self.out_degree(n) > 1).collect()
    }

    pub fn merge_points(&self) -> Vec<usize> {
        (0..self.nodes.len()).filter(|&n| self.in_degree(n) > 1).collect()
    }

    pub fn sources(&self) -> usize {
        (0..self.nodes.len()).filter(|&n| self.in_degree(n) == 0).count()
    }

    pub fn sinks(&self) -> usize {
        (0..self.nodes.len()).filter(|&n| self.out_degree(n) == 0).count()
    }

    // ── validation ──────────────────────────────────────────────────────────

    /// Grammar check. Errors: a non-FSPLIT fanning out, a non-FFUSE merging in,
    /// or any over-valence. Open valences (living ends) are reported, not fatal.
    pub fn validate(&self) -> Vec<String> {
        let mut errs = Vec::new();
        for (n, &tok) in self.nodes.iter().enumerate() {
            let (ai, ao) = tok.arity();
            let od = self.out_degree(n);
            let idg = self.in_degree(n);
            if od > 1 && tok != Token::Fsplit {
                errs.push(format!(
                    "node {n} ({}) fans out to {od}; only FSPLIT (δ) may branch",
                    tok.name()
                ));
            }
            if idg > 1 && tok != Token::Ffuse {
                errs.push(format!(
                    "node {n} ({}) merges {idg} in; only FFUSE (μ) may fuse",
                    tok.name()
                ));
            }
            if od > ao {
                errs.push(format!("node {n} ({}) out-degree {od} > arity_out {ao}", tok.name()));
            }
            if idg > ai {
                errs.push(format!("node {n} ({}) in-degree {idg} > arity_in {ai}", tok.name()));
            }
        }
        errs
    }
}


/// Ancestry pairing over a linear word: innermost-first (stack) matching of
/// FSPLIT to FFUSE, exactly as the protocol builder wires them.
pub fn match_pairs(ops: &[Token]) -> Vec<(usize, usize)> {
    let mut stack = Vec::new();
    let mut pairs = Vec::new();
    for (i, &t) in ops.iter().enumerate() {
        if t == Token::Fsplit {
            stack.push(i);
        } else if t == Token::Ffuse {
            if let Some(fs) = stack.pop() {
                pairs.push((fs, i));
            }
        }
    }
    pairs
}

/// The protocol wiring: a chain with every matched δ arm reconnected to its μ.
pub fn from_sequence(ops: &[Token], pairs: &[(usize, usize)]) -> Graph {
    let mut g = Graph::new();
    let ids = g.chain_of(ops);
    for &(fs, ff) in pairs {
        if fs < ids.len() && ff < ids.len() {
            g.connect(ids[fs], ids[ff]);
        }
    }
    g
}

/// The whole gate in one call: word tokens → protocol wiring → grammar
/// validation → closure state → verdict letter, exactly `imasm check`:
/// F ill-typed; B open OR (closes AND carries ENGAGR — paradox held beats T);
/// T closes over a transformation; N identity / no dyad / void.
pub fn word_verdict(ops: &[Token]) -> (char, ClosureState) {
    if ops.is_empty() {
        return ('N', ClosureState::None);
    }
    let pairs = match_pairs(ops);
    let g = from_sequence(ops, &pairs);
    if !g.validate().is_empty() {
        return ('F', g.closure_state());
    }
    let state = g.closure_state();
    let has_engagr = ops.iter().any(|&t| t == Token::Engagr);
    let v = match state {
        ClosureState::Closed(_) if has_engagr => 'B',
        ClosureState::Closed(_) => 'T',
        ClosureState::Identity => 'N',
        ClosureState::Open => 'B',
        ClosureState::None => 'N',
    };
    (v, state)
}
