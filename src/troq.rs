// troq.rs — Triple-Ramified Ouroboric Quantale (enterprise-grade toolset)
// Tuple: ⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭⟩
// Enterprise upgrade: full dispatch, triple-axis expansion, ouroboric convergence tests
#![allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use core::f64::consts::PI;
use libm::{sqrt, fabs, cos, floor};

pub const TUPLE_TROQ: &str = "𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭";
pub const NAME: &str = "TROQ";
pub const VERSION: &str = "2.0-enterprise";

fn frac(x: f64) -> f64 { x - floor(x) }

// ═══════════════════════════════════════════════════════════
// Axis Expansions
// ═══════════════════════════════════════════════════════════

pub fn expand_axis(slot: &str) -> Vec<&'static str> {
    match slot {
        "⊙" => vec!["woe (sub)", "⊙ (critical)", "roar (c_complex)", "𐑻 (EP)", "𐑣 (super)"],
        "≺" => vec!["𐑗 (asym)", "𐑿 (psi)", "𐑬 (pm)", "𐑯 (sym)", "𐑹 (pm_sym/Frobenius)"],
        "⊡" => vec!["𐑷 (0)", "𐑴 (Z2)", "𐑭 (Z)", "𐑟 (NA)"],
        "⊢" => vec!["𐑛 (wedge/0D)", "𐑨 (ash/2D)", "𐑼 (array/∞D)", "𐑦 (if'/imscriptive)"],
        "⊣" => vec!["𐑡 (network)", "𐑰 (inclusion)", "𐑥 (bowtie)", "𐑶 (box)", "𐑸 (imscriptive)"],
        "≻" => vec!["𐑩 (supervenience)", "𐑑 (functorial)", "𐑽 (adjoint)", "𐑾 (bidirectional)"],
        "⋈" => vec!["𐑱 (classical)", "𐑞 (thermal)", "𐑐 (quantum)"],
        "⊤" => vec!["𐑘 (driven)", "𐑤 (trapped)", "𐑧 (slow)", "𐑪 (moderate)", "𐑺 (fast-MBL)"],
        "∈" => vec!["𐑲 (local)", "𐑚 (mesoscale)", "𐑔 (aleph/maximal)"],
        "∋" => vec!["𐑝 (conjunctive)", "𐑜 (disjunctive)", "𐑠 (sequential)", "𐑵 (broadcast)"],
        "⊥" => vec!["𐑓 (memoryless)", "𐑒 (one-step)", "𐑖 (two-step)", "𐑫 (eternal)"],
        "⊞" => vec!["𐑙 (1:1)", "𐑕 (n:n)", "𐑳 (n:m)"],
        _ => vec!["no expansion for this slot"],
    }
}

// ═══════════════════════════════════════════════════════════
// Ouroboric Deviations
// ═══════════════════════════════════════════════════════════

pub fn triangular_deviation(seed: f64) -> f64 {
    let a = frac(seed * 12.0) * PI;
    let b = frac(seed * 7.0) * PI;
    let c = frac(seed * 3.0) * PI;
    let composed = cos(a + b + c);
    let original = cos(a);
    fabs(composed - original)
}

pub fn ouroboric_deviation(seed: f64) -> f64 {
    let q: f64 = (0..12).map(|i| frac(seed * (i+1) as f64)).sum();
    let end_q: f64 = (0..12).map(|i| {
        let v = frac(seed * (i+1) as f64);
        v * v
    }).sum();
    fabs(q - sqrt(end_q))
}

pub fn triple_ramification_deviation(seed: f64) -> f64 {
    let a = triangular_deviation(seed);
    let b = ouroboric_deviation(seed);
    let c = fabs(frac(seed * 5.0) - frac(seed * 11.0));
    sqrt(a*a + b*b + c*c) / 3.0
}

pub fn quantale_convergence(n_iter: usize, seed: f64) -> Vec<f64> {
    let mut vals = Vec::new();
    let mut s = seed;
    for _ in 0..n_iter {
        vals.push(triple_ramification_deviation(s));
        s = frac(s * 1.618033988749895); // φ step
    }
    vals
}

// ═══════════════════════════════════════════════════════════
// Reports
// ═══════════════════════════════════════════════════════════

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("=== TROQ {} v{} ===\n", NAME, VERSION));
    s.push_str(&format!("Tuple: ⟨{}⟩\n", TUPLE_TROQ));
    s.push_str("──────────────────────────────────────\n");
    let td = triangular_deviation(0.618);
    s.push_str(&format!("Triangular γ∘β∘α=id deviation: {:.6} {}\n", td, if td < 0.01 { "✓" } else { "✗" }));
    let od = ouroboric_deviation(0.618);
    s.push_str(&format!("Ouroboric Q≅End(Q) deviation: {:.6} {}\n", od, if od < 0.1 { "✓" } else { "✗" }));
    let rd = triple_ramification_deviation(0.618);
    s.push_str(&format!("Triple-ramification norm:     {:.6} {}\n", rd, if rd < 0.05 { "✓" } else { "✗" }));
    s.push_str("──────────────────────────────────────\n");
    s.push_str("Axis expansions:\n");
    for ax in &["⊙", "≺", "⊡"] {
        let exps = expand_axis(ax);
        s.push_str(&format!("  {}: ", ax));
        for (i, e) in exps.iter().enumerate() {
            if i>0 { s.push_str(", "); }
            s.push_str(e);
        }
        s.push_str("\n");
    }
    s.push_str("──────────────────────────────────────\n");
    s.push_str("Convergence (φ-step, 8 iter): ");
    let conv = quantale_convergence(8, 0.618);
    for v in &conv { s.push_str(&format!("{:.4} ", v)); }
    s.push_str("\n");
    s
}

pub fn summary_report() -> String {
    format!("TROQ v{}: ⟨{}⟩ | Δ_tri={:.4} Δ_ouro={:.4} Δ_ram={:.4}",
        VERSION, TUPLE_TROQ,
        triangular_deviation(0.618), ouroboric_deviation(0.618), triple_ramification_deviation(0.618))
}

pub fn json_report() -> String {
    format!("{{\"name\":\"{}\",\"version\":\"{}\",\"tuple\":\"{}\",\
        \"triangular_deviation\":{:.6},\"ouroboric_deviation\":{:.6},\"triple_ramification_deviation\":{:.6}}}",
        NAME, VERSION, TUPLE_TROQ,
        triangular_deviation(0.618), ouroboric_deviation(0.618), triple_ramification_deviation(0.618))
}

pub fn report_axis(slot: &str) -> String {
    let exps = expand_axis(slot);
    let mut s = format!("{} axis ({} values): ", slot, exps.len());
    for (i, e) in exps.iter().enumerate() {
        if i>0 { s.push_str(", "); }
        s.push_str(e);
    }
    s
}

pub fn report_all_axes() -> String {
    let mut s = String::new();
    for ax in crate::canonical_ig::PRIMITIVE_ORDER.iter() {
        s.push_str(&report_axis(ax));
        s.push_str("\n");
    }
    s
}

pub fn report_convergence(n: usize, seed: f64) -> String {
    let conv = quantale_convergence(n, seed);
    let mut s = format!("TROQ convergence (φ-step, seed={}, {} iter): ", seed, n);
    for (i, v) in conv.iter().enumerate() {
        if i>0 { s.push_str(", "); }
        s.push_str(&format!("{:.4}", v));
    }
    let min = conv.iter().cloned().fold(f64::MAX, f64::min);
    s.push_str(&format!("\n  min={:.6} converging={}", min, min < 0.01));
    s
}

pub fn help_text() -> &'static str {
    "TROQ — Triple-Ramified Ouroboric Quantale\n\
     troq              full report\n\
     troq summary      one-line summary\n\
     troq json         JSON structured output\n\
     troq axis <slot>  expand a primitive axis (⊙, <, ⊡, ⊢, ...)\n\
     troq axes         all 12 axes expanded\n\
     troq converge <n> <seed>  ouroboric convergence trace\n\
     troq tuple        tuple constant"
}

// ═══════════════════════════════════════════════════════════
// Command Dispatch
// ═══════════════════════════════════════════════════════════

pub fn dispatch<'a>(sub: &str, mut args: impl Iterator<Item=&'a str>) -> String {
    match sub {
        "" | "report" | "full" => full_report(),
        "summary" => summary_report(),
        "json" => json_report(),
        "axis" | "expand" => {
            let slot = args.next().unwrap_or("⊙");
            report_axis(slot)
        }
        "axes" => report_all_axes(),
        "converge" => {
            let n: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(8);
            let seed: f64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(0.618);
            report_convergence(n, seed)
        }
        "tuple" => TUPLE_TROQ.to_string(),
        "help" | "--help" | "-h" => help_text().to_string(),
        _ => format!("TROQ: unknown sub-command '{}'. Try: troq help", sub),
    }
}
