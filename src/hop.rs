// hop.rs — Universe Hopping Engine (native mOMonadOS port)
#![allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use libm::{sqrt, fabs};

pub const FRAMEWORKS: &[(&str, &str, &str)] = &[
    ("hqe", "𐑦𐑸𐑽𐑹𐑐𐑘𐑔𐑝⊙𐑫𐑕𐑟", "Holonomic Quasi-Ergodic Quantale"),
    ("fibonacci", "𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑜⊙𐑫𐑕𐑭", "Fibonacci Anyon Braid Algebra"),
    ("berry", "𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑫𐑕𐑭", "Non-Abelian Berry Holonomy U(n)"),
    ("mbl", "𐑼𐑸𐑽𐑹𐑐𐑘𐑔𐑝⊙𐑖𐑳𐑭", "Many-Body Localization Phase Diagram"),
    ("triple", "𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑕𐑭", "Triple Frame Von Neumann Algebra"),
    ("afdmc", "𐑼𐑸𐑽𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑳𐑭", "AFDMC Cohomology"),
    ("dyson", "𐑼𐑸𐑾𐑹𐑞𐑧𐑔𐑠⊙𐑖𐑳𐑭", "Dyson β-ensemble DR cycle"),
    ("troq", "𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑖𐑕𐑭", "Triple-Ramified Ouroboric Quantale"),
];

fn glyph_val(c: &str) -> f64 {
    match c {"𐑛"=>1.0,"𐑨"=>2.0,"𐑼"=>3.0,"𐑦"=>4.0,"𐑡"=>1.0,"𐑰"=>2.0,"𐑥"=>3.0,"𐑶"=>4.0,"𐑸"=>5.0,
             "𐑩"=>1.0,"𐑑"=>2.0,"𐑽"=>3.0,"𐑾"=>4.0,"𐑗"=>1.0,"𐑿"=>2.0,"𐑬"=>3.0,"𐑯"=>4.0,"𐑹"=>5.0,
             "𐑱"=>1.0,"𐑞"=>2.0,"𐑐"=>3.0,"𐑘"=>1.0,"𐑤"=>2.0,"𐑧"=>3.0,"𐑪"=>4.0,"𐑺"=>5.0,
             "𐑲"=>1.0,"𐑚"=>2.0,"𐑔"=>3.0,"𐑝"=>1.0,"𐑜"=>2.0,"𐑠"=>3.0,"𐑵"=>4.0,
             "𐑢"=>1.0,"⊙"=>2.0,"𐑮"=>3.0,"𐑻"=>4.0,"𐑣"=>5.0,
             "𐑓"=>1.0,"𐑒"=>2.0,"𐑖"=>3.0,"𐑫"=>4.0,"𐑙"=>1.0,"𐑕"=>2.0,"𐑳"=>3.0,
             "𐑷"=>1.0,"𐑴"=>2.0,"𐑭"=>3.0,"𐑟"=>4.0,_=>0.0}
}

fn vec_from(s: &str) -> [f64;12] {
    let c = s.trim().trim_matches(|c| c=='⟨'||c=='⟩');
    let mut v=[0.0;12];
    for i in 0..12 { v[i]=glyph_val(&c[i..=i]); }
    v
}

pub fn distance(t1: &str, t2: &str) -> f64 {
    let v1=vec_from(t1); let v2=vec_from(t2);
    let mut tot=0.0; for i in 0..12 { let d=fabs(v1[i]-v2[i]); tot+=d*d; } sqrt(tot)
}

pub fn manifest(t: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!("Manifest ⟨{}⟩:\n", t.trim().trim_matches(|c| c=='⟨'||c=='⟩')));
    let mut best=""; let mut bd=f64::MAX;
    for (nm, tu, _) in FRAMEWORKS {
        let d=distance(t,tu);
        s.push_str(&format!("  {:10} ⟨{}⟩ d={:.4}\n", nm, tu, d));
        if d<bd { bd=d; best=nm; }
    }
    s.push_str(&format!("Closest: {} (d={:.4})\n", best, bd));
    s
}

pub fn framework_matrix() -> String {
    let mut s = String::new();
    for (nm,tu,_) in FRAMEWORKS {
        s.push_str(&format!("{:10}", nm));
        for (_,tu2,_) in FRAMEWORKS { s.push_str(&format!(" {:6.2}", distance(tu, tu2))); }
        s.push_str("\n");
    }
    s
}

pub fn hop(origin: &str, target: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!("Hop: ⟨{}⟩ → ⟨{}⟩  d={:.4}\n", origin, target, distance(origin, target)));
    s
}

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("Hop Engine: {} frameworks\n", FRAMEWORKS.len()));
    for (nm, tu, desc) in FRAMEWORKS { s.push_str(&format!("  {:10} ⟨{}⟩  — {}\n", nm, tu, desc)); }
    s.push_str("\nMatrix:\n"); s.push_str(&framework_matrix());
    s.push_str("\n"); s.push_str(&hop(FRAMEWORKS[0].1, FRAMEWORKS[1].1));
    s
}
