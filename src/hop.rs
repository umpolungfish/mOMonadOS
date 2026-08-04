// hop.rs — Universe Hopping Engine (enterprise-grade toolset)
// Enterprise upgrade: full dispatch, named-framework hopping, distance matrix, catalog bridge
#![allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
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

pub const NAME: &str = "HOP";
pub const VERSION: &str = "2.0-enterprise";

// ═══════════════════════════════════════════════════════════
// Tuple Operations
// ═══════════════════════════════════════════════════════════

fn glyph_val(c: &str) -> f64 {
    match c {"𐑛"=>1.0,"𐑨"=>2.0,"𐑼"=>3.0,"𐑦"=>4.0,"𐑡"=>1.0,"𐑰"=>2.0,"𐑥"=>3.0,"𐑶"=>4.0,"𐑸"=>5.0,
             "𐑩"=>1.0,"𐑑"=>2.0,"𐑽"=>3.0,"𐑾"=>4.0,"𐑗"=>1.0,"𐑿"=>2.0,"𐑬"=>3.0,"𐑯"=>4.0,"𐑹"=>5.0,
             "𐑱"=>1.0,"𐑞"=>2.0,"𐑐"=>3.0,"𐑘"=>1.0,"𐑤"=>2.0,"𐑧"=>3.0,"𐑪"=>4.0,"𐑺"=>5.0,
             "𐑲"=>1.0,"𐑚"=>2.0,"𐑔"=>3.0,"𐑝"=>1.0,"𐑜"=>2.0,"𐑠"=>3.0,"𐑵"=>4.0,
             "woe"=>1.0,"⊙"=>2.0,"roar"=>3.0,"𐑻"=>4.0,"𐑣"=>5.0,
             "𐑓"=>1.0,"𐑒"=>2.0,"𐑖"=>3.0,"𐑫"=>4.0,"𐑙"=>1.0,"𐑕"=>2.0,"𐑳"=>3.0,
             "𐑷"=>1.0,"𐑴"=>2.0,"𐑭"=>3.0,"𐑟"=>4.0,_=>0.0}
}

fn vec_from(s: &str) -> [f64;12] {
    // Slot i is the i-th CHARACTER. `&c[i..=i]` is a byte slice, and every Shavian
    // glyph is four bytes, so it panics on the second slot.
    let c = s.trim().trim_matches(|c| c=='⟨'||c=='⟩');
    let mut v=[0.0;12];
    let mut buf=[0u8;4];
    for (i, ch) in c.chars().take(12).enumerate() { v[i]=glyph_val(ch.encode_utf8(&mut buf)); }
    v
}

pub fn distance(t1: &str, t2: &str) -> f64 {
    let v1=vec_from(t1); let v2=vec_from(t2);
    let mut tot=0.0; for i in 0..12 { let d=fabs(v1[i]-v2[i]); tot+=d*d; } sqrt(tot)
}

// ═══════════════════════════════════════════════════════════
// Framework Operations
// ═══════════════════════════════════════════════════════════

pub fn find_framework(name: &str) -> Option<(&'static str, &'static str)> {
    for (nm, tu, _) in FRAMEWORKS {
        if *nm == name { return Some((nm, tu)); }
    }
    None
}

pub fn manifest(t: &str) -> String {
    let s_clean = t.trim().trim_matches(|c| c=='⟨'||c=='⟩');
    let mut s = String::new();
    s.push_str(&format!("Manifest ⟨{}⟩:\n", s_clean));
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
    s.push_str(&format!("{:>10}", ""));
    for (nm,_,_) in FRAMEWORKS { s.push_str(&format!(" {:>6}", nm)); }
    s.push_str("\n");
    for (nm,tu,_) in FRAMEWORKS {
        s.push_str(&format!("{:10}", nm));
        for (_,tu2,_) in FRAMEWORKS { s.push_str(&format!(" {:6.2}", distance(tu, tu2))); }
        s.push_str("\n");
    }
    s
}

pub fn hop(origin: &str, target: &str) -> String {
    let mut s = String::new();
    let orig_name = if origin.len() > 4 { "custom" } else { origin };
    let targ_name = if target.len() > 4 { "custom" } else { target };
    let d = distance(origin, target);
    s.push_str(&format!("Hop: {} → {}  d={:.4}\n", orig_name, targ_name, d));
    s.push_str(&format!("  origin: ⟨{}⟩\n", origin.trim().trim_matches(|c| c=='⟨'||c=='⟩')));
    s.push_str(&format!("  target: ⟨{}⟩\n", target.trim().trim_matches(|c| c=='⟨'||c=='⟩')));
    s
}

pub fn hop_named(origin_name: &str, target_name: &str) -> String {
    match (find_framework(origin_name), find_framework(target_name)) {
        (Some((_on, ot)), Some((_tn, tt))) => hop(ot, tt),
        (None, _) => format!("Unknown origin framework: '{}'. Try: hop list", origin_name),
        (_, None) => format!("Unknown target framework: '{}'. Try: hop list", target_name),
    }
}

// ═══════════════════════════════════════════════════════════
// Reports
// ═══════════════════════════════════════════════════════════

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("=== Universe Hopping Engine {} v{} ===\n", NAME, VERSION));
    s.push_str(&format!("Frameworks: {}\n", FRAMEWORKS.len()));
    s.push_str("──────────────────────────────────────\n");
    for (nm, tu, desc) in FRAMEWORKS {
        s.push_str(&format!("  {:10} ⟨{}⟩\n           {}\n", nm, tu, desc));
    }
    s.push_str("──────────────────────────────────────\n");
    s.push_str(&format!("Min inter-framework distance: {:.2}\n", min_inter_framework_distance()));
    s.push_str(&format!("Max inter-framework distance: {:.2}\n", max_inter_framework_distance()));
    s
}

pub fn summary_report() -> String {
    format!("HOP v{}: {} frameworks | min_d={:.2} max_d={:.2}",
        VERSION, FRAMEWORKS.len(), min_inter_framework_distance(), max_inter_framework_distance())
}

pub fn json_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("{{\"name\":\"{}\",\"version\":\"{}\",\"n_frameworks\":{},\"frameworks\":[",
        NAME, VERSION, FRAMEWORKS.len()));
    for (i, (nm, tu, desc)) in FRAMEWORKS.iter().enumerate() {
        if i>0 { s.push_str(","); }
        s.push_str(&format!("{{\"name\":\"{}\",\"tuple\":\"{}\",\"description\":\"{}\"}}", nm, tu, desc));
    }
    s.push_str("]}");
    s
}

pub fn min_inter_framework_distance() -> f64 {
    let mut min_d = f64::MAX;
    for (_, t1, _) in FRAMEWORKS {
        for (_, t2, _) in FRAMEWORKS {
            if t1 != t2 {
                let d = distance(t1, t2);
                if d < min_d { min_d = d; }
            }
        }
    }
    min_d
}

pub fn max_inter_framework_distance() -> f64 {
    let mut max_d = 0.0;
    for (_, t1, _) in FRAMEWORKS {
        for (_, t2, _) in FRAMEWORKS {
            let d = distance(t1, t2);
            if d > max_d { max_d = d; }
        }
    }
    max_d
}

pub fn find_closest(t: &str) -> String {
    let mut best_name = "";
    let mut best_dist = f64::MAX;
    for (nm, tu, _) in FRAMEWORKS {
        let d = distance(t, tu);
        if d < best_dist { best_dist = d; best_name = nm; }
    }
    format!("Closest to custom tuple: {} (d={:.4})", best_name, best_dist)
}

pub fn help_text() -> &'static str {
    "HOP — Universe Hopping Engine\n\
     hop                  full framework report\n\
     hop summary          one-line summary\n\
     hop json             JSON structured output\n\
     hop list             list all framework names\n\
     hop matrix           inter-framework distance matrix\n\
     hop manifest <tuple>  find closest framework to a tuple\n\
     hop hop <from> <to>   distance between two named frameworks\n\
     hop dist <t1> <t2>    distance between two tuples\n\
     hop closest <tuple>   find nearest framework"
}

// ═══════════════════════════════════════════════════════════
// Command Dispatch
// ═══════════════════════════════════════════════════════════

pub fn dispatch<'a>(sub: &str, mut args: impl Iterator<Item=&'a str>) -> String {
    match sub {
        "" | "report" | "full" => full_report(),
        "summary" => summary_report(),
        "json" => json_report(),
        "list" => {
            let mut s = format!("{} frameworks: ", FRAMEWORKS.len());
            for (i, (nm, _, _)) in FRAMEWORKS.iter().enumerate() {
                if i>0 { s.push_str(", "); }
                s.push_str(nm);
            }
            s
        }
        "matrix" => framework_matrix(),
        "manifest" => {
            let t = args.next().unwrap_or("𐑦𐑸𐑽𐑹𐑐𐑘𐑔𐑝⊙𐑫𐑕𐑟");
            manifest(t)
        }
        "hop" => {
            let origin = args.next().unwrap_or("hqe");
            let target = args.next().unwrap_or("afdmc");
            hop_named(origin, target)
        }
        "dist" | "distance" => {
            let t1 = args.next().unwrap_or("𐑦𐑸𐑽𐑹𐑐𐑘𐑔𐑝⊙𐑫𐑕𐑟");
            let t2 = args.next().unwrap_or("𐑼𐑸𐑽𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑳𐑭");
            format!("d={:.4}", distance(t1, t2))
        }
        "closest" => {
            let t = args.next().unwrap_or("𐑦𐑸𐑽𐑹𐑐𐑘𐑔𐑝⊙𐑫𐑕𐑟");
            find_closest(t)
        }
        "help" | "--help" | "-h" => help_text().to_string(),
        _ => format!("HOP: unknown sub-command '{}'. Try: hop help", sub),
    }
}
