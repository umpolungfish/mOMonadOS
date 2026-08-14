// ─── blackbox.rs ───────────────────────────────────────────────────────
// Infer a law from observations (build.txt §449).
//
// Give it numbers rather than equations. It searches a small space of exact
// laws — constant, polynomial, geometric, second-order linear recurrence,
// pure period — and returns the COMPETING hypotheses ranked by
//
//     score = fit - lambda * complexity
//
// so a law that fits by having as many parameters as data points loses to a
// simpler law that fits as well. Every candidate is checked against every
// observation by integer arithmetic; `fit` is the exact fraction reproduced,
// never a correlation. A law that reproduces the data is reported as fitting
// the data and nothing more — extrapolation is offered as a prediction to be
// tested, not as a property of the sequence.
//
// This is the one tool here that works on data from outside the kernel's own
// vocabulary, which is the point: it takes a list of integers.
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Weight on complexity. At 0.05, one extra parameter must buy 5% of fit.
pub const LAMBDA: f64 = 0.05;

pub struct Law {
    pub name: String,
    pub form: String,
    /// Fraction of observations the law reproduces exactly.
    pub fit: f64,
    /// Free parameters. A law with as many parameters as data explains nothing.
    pub complexity: usize,
    /// The next term, when the law licenses one.
    pub next: Option<i64>,
}

impl Law {
    pub fn score(&self) -> f64 {
        self.fit - LAMBDA * self.complexity as f64
    }
}

fn diffs(v: &[i64]) -> Vec<i64> {
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

/// Exact polynomial law by finite differences: the k-th difference of a degree-k
/// polynomial is constant. Integer arithmetic throughout, so a "fit" here is an
/// identity rather than a regression.
fn try_polynomial(v: &[i64]) -> Option<Law> {
    let mut level = v.to_vec();
    for degree in 0..=4usize {
        if level.is_empty() {
            return None;
        }
        if level.iter().all(|x| *x == level[0]) {
            // Rebuild the next term by summing the difference table back up.
            let mut tail: Vec<i64> = Vec::new();
            let mut cur = v.to_vec();
            for _ in 0..degree {
                tail.push(*cur.last()?);
                cur = diffs(&cur);
            }
            let mut next = level[0];
            for t in tail.iter().rev() {
                next += *t;
            }
            let form = match degree {
                0 => format!("a_n = {}", level[0]),
                1 => format!("a_n = a_0 + {}n   (arithmetic)", level[0]),
                d => format!(
                    "polynomial of degree {} ({} difference constant = {})",
                    d,
                    match d { 2 => "2nd", 3 => "3rd", _ => "4th" },
                    level[0]
                ),
            };
            return Some(Law {
                name: if degree <= 1 {
                    if degree == 0 { "constant".to_string() } else { "arithmetic".to_string() }
                } else {
                    format!("polynomial deg {}", degree)
                },
                form,
                fit: 1.0,
                complexity: degree + 1,
                next: Some(next),
            });
        }
        if level.len() < 2 {
            return None;
        }
        level = diffs(&level);
    }
    None
}

fn try_geometric(v: &[i64]) -> Option<Law> {
    if v.len() < 3 || v[0] == 0 {
        return None;
    }
    if v[1] % v[0] != 0 {
        return None;
    }
    let r = v[1] / v[0];
    if r == 0 || r == 1 {
        return None; // r=1 is the constant law, already covered
    }
    let ok = v.windows(2).all(|w| w[0] != 0 && w[1] == w[0] * r);
    if !ok {
        return None;
    }
    Some(Law {
        name: "geometric".to_string(),
        form: format!("a_n = {} * {}^n", v[0], r),
        fit: 1.0,
        complexity: 2,
        next: v.last().map(|x| x * r),
    })
}

/// a_n = p*a_{n-1} + q*a_{n-2}, solved from two equations and then CHECKED
/// against every remaining term. Solving alone proves nothing — two equations
/// always have a solution; the check is what makes it a law.
fn try_recurrence(v: &[i64]) -> Option<Law> {
    if v.len() < 5 {
        return None;
    }
    let (a0, a1, a2, a3) = (v[0], v[1], v[2], v[3]);
    let det = a1 * a1 - a2 * a0;
    if det == 0 {
        return None;
    }
    let pn = a2 * a1 - a3 * a0;
    let qn = a3 * a1 - a2 * a2;
    if pn % det != 0 || qn % det != 0 {
        return None;
    }
    let (p, q) = (pn / det, qn / det);
    let mut hits = 0usize;
    let mut total = 0usize;
    for i in 2..v.len() {
        total += 1;
        if v[i] == p * v[i - 1] + q * v[i - 2] {
            hits += 1;
        }
    }
    if total == 0 || hits != total {
        return None;
    }
    let n = v.len();
    Some(Law {
        name: "linear recurrence".to_string(),
        form: format!("a_n = {}*a_(n-1) + {}*a_(n-2)", p, q),
        fit: 1.0,
        complexity: 2,
        next: Some(p * v[n - 1] + q * v[n - 2]),
    })
}

/// Pure repetition: the sequence is a block repeated. Cheap, and it catches the
/// case every other law would fit badly with many parameters.
fn try_period(v: &[i64]) -> Option<Law> {
    let n = v.len();
    if n < 4 {
        return None;
    }
    for p in 1..=(n / 2) {
        if (0..n).all(|i| v[i] == v[i % p]) {
            return Some(Law {
                name: "periodic".to_string(),
                form: format!("period {} block repeated", p),
                fit: 1.0,
                complexity: p,
                next: Some(v[n % p]),
            });
        }
    }
    None
}

/// A law that fits only part of the data is still evidence, so a partial
/// polynomial fit is reported with its true fit rather than discarded.
fn partial_polynomial(v: &[i64]) -> Option<Law> {
    // Longest prefix that IS a polynomial of degree <= 2.
    for cut in (4..=v.len()).rev() {
        if let Some(mut l) = try_polynomial(&v[..cut]) {
            if cut < v.len() {
                l.fit = cut as f64 / v.len() as f64;
                l.name = format!("{} (first {} terms only)", l.name, cut);
                l.next = None;
                return Some(l);
            }
        }
    }
    None
}

pub fn infer(v: &[i64]) -> Vec<Law> {
    let mut out: Vec<Law> = Vec::new();
    if let Some(l) = try_polynomial(v) {
        out.push(l);
    }
    if let Some(l) = try_geometric(v) {
        out.push(l);
    }
    if let Some(l) = try_recurrence(v) {
        out.push(l);
    }
    if let Some(l) = try_period(v) {
        out.push(l);
    }
    if out.is_empty() {
        if let Some(l) = partial_polynomial(v) {
            out.push(l);
        }
    }
    // A law with as many free parameters as observations is interpolation, not
    // inference: a degree n-1 polynomial fits ANY n points exactly, so its
    // fit of 1.000 carries no information. The complexity penalty alone does
    // not catch this — when no simpler law exists, the vacuous one wins by
    // default. It is kept and marked rather than hidden, because "the only
    // thing that fits is interpolation" is itself the finding, but it licenses
    // no prediction.
    for l in out.iter_mut() {
        if l.complexity >= v.len() {
            l.name = format!("{} — INTERPOLATION", l.name);
            l.form = format!(
                "{}   (as many parameters as observations: explains nothing)",
                l.form
            );
            l.next = None;
        }
    }

    out.sort_by(|a, b| {
        b.score()
            .partial_cmp(&a.score())
            .unwrap_or(core::cmp::Ordering::Equal)
    });
    out
}

pub fn format_inference(v: &[i64], laws: &[Law]) -> String {
    let mut out = String::new();
    out.push_str("BLACKBOX\n========\n\n");
    out.push_str("observations: ");
    for (i, x) in v.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{}", x));
    }
    out.push_str(&format!("   ({} terms)\n\n", v.len()));

    if laws.is_empty() {
        out.push_str(
            "No law in the searched space reproduces these observations.\n\n\
             The space is: constant, polynomial to degree 4, geometric,\n\
             second-order linear recurrence, pure period. Absence here means\n\
             the search found nothing — not that no law exists.\n",
        );
        return out;
    }

    out.push_str(&format!(
        "hypotheses, ranked by fit - {:.2} * complexity:\n\n",
        LAMBDA
    ));
    out.push_str("      score   fit   cx  law\n");
    for l in laws {
        out.push_str(&format!(
            "    {:>6.3}  {:>5.3}  {:>3}  {:<22} {}\n",
            l.score(),
            l.fit,
            l.complexity,
            l.name,
            l.form
        ));
    }

    let best = &laws[0];
    out.push_str(&format!("\nbest: {}\n", best.form));
    match best.next {
        Some(n) => out.push_str(&format!(
            "prediction: the next term is {} — a claim to TEST, not a property\n\
             \x20           of the sequence\n",
            n
        )),
        None => {
            // Two different silences: a fit that covers only part of the data,
            // and a fit that covers all of it while explaining none of it.
            if best.complexity >= v.len() {
                out.push_str(
                    "prediction: none. The fit is exact but vacuous — it has as many\n\
                     \x20           parameters as observations, so it interpolates rather\n\
                     \x20           than infers. More observations would decide it.\n",
                );
            } else {
                out.push_str("prediction: none licensed by a partial fit\n");
            }
        }
    }

    if laws.len() > 1 && (laws[0].score() - laws[1].score()).abs() < 0.001 {
        out.push_str(
            "\nThe top two score equally. The data does not choose between them;\n\
             more observations would.\n",
        );
    }
    out
}

pub fn blackbox_main(args: &[&str]) -> String {
    // The REPL splits a line with `splitn(4, ' ')`, so everything past the third
    // field arrives as ONE argument still carrying its spaces: "9 16 25". Parsing
    // the arguments as given silently dropped every term after the third and then
    // reported the usage text, which looks like the command was mistyped rather
    // than truncated. Split again here.
    let nums: Vec<i64> = args
        .iter()
        .flat_map(|s| s.split_whitespace())
        .filter_map(|s| s.trim_matches(|c: char| c == ',' || c == ';').parse::<i64>().ok())
        .collect();

    if nums.len() < 3 {
        return "blackbox <n1> <n2> <n3> ...\n\
                \n\
                Infer a law from observations. Searches constant, polynomial to\n\
                degree 4, geometric, second-order linear recurrence and pure\n\
                period; ranks the survivors by fit - 0.05 * complexity.\n\
                \n\
                Every candidate is checked against every observation in integer\n\
                arithmetic, so a fit of 1.000 is an identity, not a correlation.\n\
                \n\
                Try:  blackbox 1 4 9 16 25\n\
                      blackbox 1 1 2 3 5 8 13\n\
                      blackbox 3 6 12 24 48\n"
            .to_string();
    }

    format_inference(&nums, &infer(&nums))
}
