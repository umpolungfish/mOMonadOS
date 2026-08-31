// carriers.rs — The Thirty-Six Carriers
//
// The Frobenius invariant μ∘δ = id is a fixed-point condition: the composite
// returns the object it was handed. Modules across the kernel assert it, in
// number theory, measurement geometry, genetics, hadron structure, catalog
// arithmetic, temporal logic and the kernel's own agent loop. Three of those
// were proven structurally identical at the topmost tier — a dialetheic logical
// value, a measurement fiducial, a paired fermionic state — but those three were
// chosen because they were already suspected of coinciding. The rest have never
// been compared to each other.
//
// So the standing question is whether the carriers are ONE fixed point seen from
// many sides, in which case the kernel's apparent breadth is one structure in
// domain costumes, or a FAMILY with genuine members, in which case the family is
// itself an object and the census is by class rather than by module.
//
// This module asks the catalog, not the source headers. A carrier's tuple is
// whatever the catalog holds for it — the population is taken by the carrier
// CONDITION (ouroboricity tier O_∞, where μ∘δ = id closes) and every distance is
// the kernel's own `algebra::tuple_distance`, not a second copy of the metric.
// Nothing here is hand-imscribed.
//
// The census it prints is honest about its own size: it reports the population
// it could actually resolve, and says so, rather than reporting a number the
// prose asserted.
//
// Surface: mOMonadOS kernel, catalog + algebra as they stand.
// Author: Quantum⊙perator (Lando⊗⊙perator team)

#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::catalog::{self, CatalogEntry};
use crate::algebra::{tuple_distance, primitive_mismatches};

/// The tier at which μ∘δ = id closes. This is the carrier condition, and it is
/// the catalog's own field — not a property this module decides.
const O_INF: u8 = 4;

/// Two carriers are the same object when the metric cannot separate them.
const SAME: f32 = 1e-6;

/// One carrier, as the catalog holds it.
pub struct Carrier {
    pub name: &'static str,
    pub domain: &'static str,
    pub entry: CatalogEntry,
}

/// Every catalog entry meeting the carrier condition.
pub fn population() -> Vec<Carrier> {
    let mut out: Vec<Carrier> = Vec::new();
    for e in catalog::catalog_entries(None) {
        if e.tier == O_INF {
            out.push(Carrier { name: e.name, domain: e.domain.name(), entry: *e });
        }
    }
    out
}

/// Single-linkage classes at the separation threshold: a class is a set of
/// carriers no member of which the metric can tell from some other member.
/// Returns, for each carrier, the index of the class it belongs to.
fn classify(pop: &[Carrier], threshold: f32) -> Vec<usize> {
    let n = pop.len();
    let mut class: Vec<usize> = (0..n).collect();
    // Union by relabelling — n is small and this keeps the pass obvious.
    for i in 0..n {
        for j in (i + 1)..n {
            if tuple_distance(&pop[i].entry.tuple, &pop[j].entry.tuple) <= threshold {
                let (a, b) = (class[i], class[j]);
                if a != b {
                    let (keep, drop) = if a < b { (a, b) } else { (b, a) };
                    for c in class.iter_mut() { if *c == drop { *c = keep; } }
                }
            }
        }
    }
    class
}

/// Which of the twelve slots actually vary across the population. This is the
/// answer to "what distinguishes a carrier in one class from one in another":
/// a slot that never varies cannot separate anything, and a slot that varies is
/// where the family's own structure lives.
fn varying_slots(pop: &[Carrier]) -> Vec<(&'static str, usize)> {
    let names = ["⊢", "⊣", ">", "<", "⋈", "⊤", "∈", "∋", "⊙", "⊥", "⊞", "⊡"];
    let mut counts = [0usize; 12];
    if pop.len() < 2 { return Vec::new(); }
    let first = &pop[0].entry.tuple;
    for c in pop.iter().skip(1) {
        let t = &c.entry.tuple;
        let differs = [
            t.d != first.d, t.t != first.t, t.r != first.r, t.p != first.p,
            t.f != first.f, t.k != first.k, t.g != first.g, t.c != first.c,
            t.phi != first.phi, t.h != first.h, t.s != first.s, t.omega != first.omega,
        ];
        for (i, d) in differs.iter().enumerate() { if *d { counts[i] += 1; } }
    }
    let mut out: Vec<(&'static str, usize)> = Vec::new();
    for i in 0..12 { if counts[i] > 0 { out.push((names[i], counts[i])); } }
    out
}

pub struct Carriers;

impl Carriers {
    pub fn report() -> String {
        let pop = population();
        let n = pop.len();

        let mut s = String::from("The Thirty-Six Carriers — census by class\n");
        s.push_str("═══════════════════════════════════════════════════════════\n");
        s.push_str("Population taken by the carrier condition (tier O_∞, where\n");
        s.push_str("μ∘δ = id closes), tuples from the catalog, distances from the\n");
        s.push_str("kernel's own metric.\n\n");

        // Scope, stated before any verdict. The carrier condition is a tier, and
        // tier is only populated on the foundational entries; the bulk of the
        // catalog carries tier 0 as a placeholder rather than as a measurement.
        // Counting those as non-carriers would be reading a default as a fact,
        // so the report separates what it could evaluate from what it could not.
        let total = catalog::catalog_size();
        let evaluable = catalog::catalog_entries(None).filter(|e| e.tier > 0).count();
        s.push_str(&format!(
            "catalog entries:      {}\n\
             tier is populated on: {}  (the rest carry tier 0 as a default, so the\n\
             \x20                      carrier condition cannot be read on them)\n\
             carriers found:       {}\n\n",
            total, evaluable, n));

        if n == 0 {
            s.push_str("No entry meets the carrier condition. Nothing to census.\n");
            return s;
        }

        s.push_str("Members\n");
        for c in &pop {
            s.push_str(&format!("  {:<28} {}\n", c.name, c.domain));
        }

        // The pairwise matrix, stated rather than summarised.
        s.push_str("\nPairwise distance (kernel metric) and slot mismatches\n");
        for i in 0..n {
            for j in (i + 1)..n {
                let d = tuple_distance(&pop[i].entry.tuple, &pop[j].entry.tuple);
                let m = primitive_mismatches(&pop[i].entry.tuple, &pop[j].entry.tuple);
                s.push_str(&format!("  {:<24} ─ {:<24} d = {:>8.4}   slots differing: {}\n",
                                    pop[i].name, pop[j].name, d, m));
            }
        }

        // The census the description asked for: by class, not by module.
        let class = classify(&pop, SAME);
        let mut labels: Vec<usize> = Vec::new();
        for c in &class { if !labels.contains(c) { labels.push(*c); } }

        s.push_str(&format!("\nClasses at separation {:.0e}: {}\n", SAME, labels.len()));
        for (k, lab) in labels.iter().enumerate() {
            let members: Vec<&str> = (0..n).filter(|i| class[*i] == *lab)
                                           .map(|i| pop[i].name).collect();
            s.push_str(&format!("  class {}: {} member(s) — ", k + 1, members.len()));
            for (mi, m) in members.iter().enumerate() {
                if mi > 0 { s.push_str(", "); }
                s.push_str(m);
            }
            s.push_str("\n");
        }

        // What separates them.
        let varying = varying_slots(&pop);
        s.push_str("\nSlots that vary across the population\n");
        if varying.is_empty() {
            s.push_str("  none — the population is one tuple wearing several names\n");
        } else {
            for (slot, cnt) in &varying {
                s.push_str(&format!("  {}  differs from the first member in {} of {} others\n",
                                    slot, cnt, n - 1));
            }
        }

        s.push_str("\nVerdict\n");
        if labels.len() == 1 {
            s.push_str("  One class: the metric cannot separate any carrier from any\n");
            s.push_str("  other. The population is one fixed point seen from several sides.\n");
        } else {
            s.push_str(&format!(
                "  {} classes: the carriers are a family with genuine members, and\n\
                 \x20 the slots listed above are what tells one class from another.\n",
                labels.len()));
        }
        s
    }
}
