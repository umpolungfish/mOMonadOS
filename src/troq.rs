// troq.rs — Triple-Ramified Ouroboric Quantale (native mOMonadOS port)
// Tuple: ⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭⟩
#![allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use core::f64::consts::PI;
use libm::{sqrt, fabs, sin, cos, floor};

pub const TUPLE_TROQ: &str = "𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭";

fn frac(x: f64) -> f64 { x - floor(x) }

pub fn expand_axis(slot: &str) -> Vec<&'static str> {
    match slot {
        "⊙" => vec!["𐑢 (sub)", "⊙ (critical)", "𐑮 (c_complex)", "𐑻 (EP)", "𐑣 (super)"],
        "Φ" => vec!["𐑗 (asym)", "𐑿 (psi)", "𐑬 (pm)", "𐑯 (sym)", "𐑹 (pm_sym/Frobenius)"],
        "Ω" => vec!["𐑷 (0)", "𐑴 (Z2)", "𐑭 (Z)", "𐑟 (NA)"],
        _ => vec!["no expansion"],
    }
}

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

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("TROQ {}\n", TUPLE_TROQ));
    s.push_str("----------------------------------------\n");
    let td = triangular_deviation(0.618);
    s.push_str(&format!("Triangular γ∘β∘α=id deviation: {:.6} {}\n", td, if td < 0.01 { "✓" } else { "✗" }));
    let od = ouroboric_deviation(0.618);
    s.push_str(&format!("Ouroboric Q≅End(Q) deviation: {:.6} {}\n", od, if od < 0.1 { "✓" } else { "✗" }));
    for ax in &["⊙", "Φ", "Ω"] {
        s.push_str(&format!("{}: {}\n", ax, expand_axis(ax).join(", ")));
    }
    s
}
