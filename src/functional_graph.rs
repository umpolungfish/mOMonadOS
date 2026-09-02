//! Grammar-native representation of functional-graph dynamics.
//!
//! `cycle`, `combo2` and every rotation-orbit reading in this codebase read
//! one specific kind of orbit: a word's own, where ROTAT is a bijection and
//! every element therefore has tail length zero -- the graph is a disjoint
//! union of pure cycles, nothing feeding in from outside. An arbitrary map
//! f: S -> S need not be injective or surjective, so most elements have more
//! than one predecessor and some have none; iterating from any element walks
//! a rho-shaped path, a tail of unique length ending on a cycle. That
//! asymmetry between forward step and backward step is the actual content
//! here, and no existing instrument in this project reads it, because all of
//! them assume invertibility. See ig-docs/functional_graph_reference.md,
//! whose spec this implements.
//!
//! Two algorithms, both textbook, both here so the constant-memory one
//! (Brent) can be checked against the two-pointer baseline (Floyd) on the
//! same map before either is trusted: `prime_winding.rs` already carries a
//! private Brent's-variant walk, hardwired to x^2+c mod N for factoring
//! alone. This is the general form -- any f: u64 -> u64 -- with a third
//! capability neither point-reading gives: `decompose` walks the WHOLE
//! finite domain once and returns every rho-component, not just the one
//! reached from a chosen start.

/// Floyd's tortoise-and-hare: the two-pointer baseline. Returns
/// (tail_length, cycle_length) = (mu, lambda) for the orbit of x0 under f.
/// O(mu + lambda) steps, two pointers of state.
pub fn floyd<F: Fn(u64) -> u64>(f: F, x0: u64) -> (u64, u64) {
    let mut tortoise = f(x0);
    let mut hare = f(f(x0));
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(f(hare));
    }

    // tortoise now sits at some point on the cycle; find where the tail
    // from x0 first touches it by walking both from a common distance.
    let mut mu = 0u64;
    let mut tortoise = x0;
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1;
    }

    let mut lambda = 1u64;
    let mut hare = f(tortoise);
    while tortoise != hare {
        hare = f(hare);
        lambda += 1;
    }

    (mu, lambda)
}

/// Brent's variant: power-of-two bursts against a fixed checkpoint reset at
/// the start of each burst, per ig-docs/functional_graph_reference.md.
/// Same (tail_length, cycle_length) answer as `floyd`, same asymptotic step
/// count, fewer function evaluations in practice since it never re-walks
/// the tortoise every step during cycle-length discovery.
pub fn brent<F: Fn(u64) -> u64>(f: F, x0: u64) -> (u64, u64) {
    let mut power = 1u64;
    let mut lambda = 1u64;
    let mut tortoise = x0;
    let mut hare = f(x0);
    while tortoise != hare {
        if power == lambda {
            tortoise = hare;
            power *= 2;
            lambda = 0;
        }
        hare = f(hare);
        lambda += 1;
    }

    // mu: walk a hare lambda steps ahead of x0, then close the gap in lockstep.
    let mut tortoise = x0;
    let mut hare = x0;
    for _ in 0..lambda {
        hare = f(hare);
    }
    let mut mu = 0u64;
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1;
    }

    (mu, lambda)
}

/// One rho-shaped component: a cycle, and every tail vertex that feeds into
/// it paired with its exact distance to the cycle.
pub struct RhoComponent {
    pub cycle: Vec<u64>,
    pub tails: Vec<(u64, u64)>, // (vertex, tail_length), tail_length >= 1
}

/// The full functional graph of f restricted to {0, ..., domain-1} (f's
/// output is taken mod domain, so f always stays inside the set). One pass,
/// O(domain) total work: walk each unvisited vertex's path, colour it
/// in-progress, and stop either at a vertex already in-progress (a new
/// cycle, found where the path first revisits itself) or at a vertex
/// already finished (this path's tail lands on an already-known component).
pub fn decompose<F: Fn(u64) -> u64>(f: F, domain: u64) -> Vec<RhoComponent> {
    const UNVISITED: u8 = 0;
    const IN_PROGRESS: u8 = 1;
    const DONE: u8 = 2;

    let n = domain as usize;
    let mut state = vec![UNVISITED; n];
    let mut tail_len = vec![0u64; n];
    let mut component_of = vec![usize::MAX; n];
    let mut components: Vec<RhoComponent> = Vec::new();

    for start in 0..n as u64 {
        if state[start as usize] != UNVISITED {
            continue;
        }
        let mut path: Vec<u64> = Vec::new();
        let mut x = start;
        while state[x as usize] == UNVISITED {
            state[x as usize] = IN_PROGRESS;
            path.push(x);
            x = f(x) % domain;
        }

        if state[x as usize] == IN_PROGRESS {
            // x sits on this very path: the path from its first occurrence
            // to the end is the new cycle.
            let idx = path.iter().position(|&v| v == x).unwrap();
            let cycle: Vec<u64> = path[idx..].to_vec();
            let cid = components.len();
            for &v in &cycle {
                component_of[v as usize] = cid;
                tail_len[v as usize] = 0;
            }
            for (i, &v) in path[..idx].iter().enumerate() {
                let dist = (idx - i) as u64;
                tail_len[v as usize] = dist;
                component_of[v as usize] = cid;
            }
            components.push(RhoComponent { cycle, tails: Vec::new() });
        } else {
            // x is DONE: this whole path is tail feeding into its component.
            let cid = component_of[x as usize];
            let base = tail_len[x as usize];
            for (i, &v) in path.iter().enumerate().rev() {
                let dist = base + (path.len() - i) as u64;
                tail_len[v as usize] = dist;
                component_of[v as usize] = cid;
            }
        }

        for &v in &path {
            state[v as usize] = DONE;
        }
    }

    for v in 0..n as u64 {
        let dist = tail_len[v as usize];
        if dist > 0 {
            let cid = component_of[v as usize];
            components[cid].tails.push((v, dist));
        }
    }

    components
}

/// The doc's own contrast, made checkable: a bijection's functional graph
/// has tail length zero everywhere. True iff every vertex sits directly on
/// a cycle -- iff `decompose` found no tails at all.
pub fn is_bijection_on<F: Fn(u64) -> u64>(f: F, domain: u64) -> bool {
    decompose(f, domain).iter().all(|c| c.tails.is_empty())
}

fn quadratic(n: u64, c: u64) -> impl Fn(u64) -> u64 {
    move |x| {
        let x = x as u128;
        let c = c as u128;
        let n = n as u128;
        ((x * x + c) % n) as u64
    }
}

pub fn help() -> String {
    let mut out = String::new();
    out.push_str("fgraph -- Grammar-native functional-graph dynamics (ig-docs/functional_graph_reference.md)\n");
    out.push_str("  fgraph point <n> <c> <x0>      tail/cycle length of x0 under x -> x^2+c mod n, Floyd and Brent cross-checked\n");
    out.push_str("  fgraph decompose <n> <c>       the whole functional graph on {0..n-1} under x -> x^2+c mod n: every rho-component\n");
    out.push_str("  fgraph bijection <n> <k>       control: x -> x+k mod n is a bijection -- verifies every tail length is zero\n");
    out
}

pub fn point(n_str: &str, c_str: &str, x0_str: &str) -> String {
    let n: u64 = match n_str.parse() { Ok(v) if v > 0 => v, _ => return "fgraph point: n must be a positive integer".to_string() };
    let c: u64 = match c_str.parse() { Ok(v) => v, _ => return "fgraph point: c must be a non-negative integer".to_string() };
    let x0: u64 = match x0_str.parse::<u64>() { Ok(v) => v % n, _ => return "fgraph point: x0 must be a non-negative integer".to_string() };

    let f = quadratic(n, c);
    let (mu_f, lam_f) = floyd(&f, x0);
    let (mu_b, lam_b) = brent(&f, x0);

    let mut out = String::new();
    out.push_str(&format!("f(x) = x^2 + {} mod {}, x0 = {}\n", c, n, x0));
    out.push_str(&format!("  Floyd : tail = {}, cycle = {}\n", mu_f, lam_f));
    out.push_str(&format!("  Brent : tail = {}, cycle = {}\n", mu_b, lam_b));
    if mu_f == mu_b && lam_f == lam_b {
        out.push_str("  agree -- two different instruments answering the same question, same answer\n");
    } else {
        out.push_str("  DISAGREE -- do not trust either reading until this is resolved\n");
    }
    out
}

pub fn decompose_report(n_str: &str, c_str: &str) -> String {
    let n: u64 = match n_str.parse() { Ok(v) if v > 0 => v, _ => return "fgraph decompose: n must be a positive integer".to_string() };
    let c: u64 = match c_str.parse() { Ok(v) => v, _ => return "fgraph decompose: c must be a non-negative integer".to_string() };

    let f = quadratic(n, c);
    let components = decompose(&f, n);

    let mut out = String::new();
    out.push_str(&format!("f(x) = x^2 + {} mod {}: {} rho-component(s)\n", c, n, components.len()));
    let mut total_tail = 0u64;
    let mut max_tail = 0u64;
    for (i, comp) in components.iter().enumerate() {
        out.push_str(&format!(
            "  component {}: cycle length {} ({:?}), {} tail vertice(s)\n",
            i, comp.cycle.len(), comp.cycle, comp.tails.len()
        ));
        for &(_, t) in &comp.tails {
            total_tail += t;
            if t > max_tail { max_tail = t; }
        }
    }
    let mean_tail = if n > 0 { total_tail as f64 / n as f64 } else { 0.0 };
    out.push_str(&format!("  mean tail length {:.4}, max tail length {}\n", mean_tail, max_tail));
    out
}

pub fn bijection_report(n_str: &str, k_str: &str) -> String {
    let n: u64 = match n_str.parse() { Ok(v) if v > 0 => v, _ => return "fgraph bijection: n must be a positive integer".to_string() };
    let k: u64 = match k_str.parse() { Ok(v) => v, _ => return "fgraph bijection: k must be a non-negative integer".to_string() };

    let f = move |x: u64| (x + k) % n;
    let components = decompose(&f, n);
    let bijective = components.iter().all(|c| c.tails.is_empty());

    let mut out = String::new();
    out.push_str(&format!("control: x -> x + {} mod {}\n", k, n));
    out.push_str(&format!("  {} rho-component(s), every one a pure cycle: {}\n", components.len(), bijective));
    if bijective {
        let lens: Vec<usize> = components.iter().map(|c| c.cycle.len()).collect();
        out.push_str(&format!("  cycle lengths {:?} -- tail length zero everywhere, exactly the doc's own contrast\n", lens));
    } else {
        out.push_str("  NOT bijective on this domain -- a real tail exists where the doc says there should be none\n");
    }
    out
}

pub fn fgraph_main(args: &[&str]) -> String {
    match args {
        [] | ["help"] => help(),
        ["point", n, c, x0] => point(n, c, x0),
        ["decompose", n, c] => decompose_report(n, c),
        ["bijection", n, k] => bijection_report(n, k),
        _ => format!("fgraph: unrecognized arguments\n{}", help()),
    }
}
