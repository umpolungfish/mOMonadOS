// dyson.rs — Dyson Beta-Ensemble + Double-Ramified Cycle (enterprise-grade toolset)
// Tuple: ⟨𐑼𐑸𐑾𐑹𐑞𐑧𐑔𐑠⊙𐑖𐑳𐑭⟩
// Enterprise upgrade: full command dispatch, multiple report formats, catalog integration
#![allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use core::f64::consts::PI;
use libm::{sqrt, exp, log, fabs, pow};

pub const TUPLE_DRDA: &str = "𐑼𐑸𐑾𐑹𐑞𐑧𐑔𐑠⊙𐑖𐑳𐑭";
pub const NAME: &str = "DRDA";
pub const VERSION: &str = "2.0-enterprise";

// ═══════════════════════════════════════════════════════════
// Core Mathematics
// ═══════════════════════════════════════════════════════════

pub fn mean_gap_ratio(beta: u8) -> f64 {
    match beta { 1 => 0.5307, 2 => 0.5996, 4 => 0.6744, _ => 0.3863 }
}

pub fn wigner_surmise(s: f64, beta: u8) -> f64 {
    match beta {
        1 => (PI/2.0)*s*exp(-PI*s*s/4.0),
        2 => (32.0/(PI*PI))*s*s*exp(-4.0*s*s/PI),
        4 => (64.0/(9.0*PI*PI*PI))*pow(s,4.0)*exp(-4.0*s*s/PI),
        _ => exp(-s),
    }
}

pub fn spectral_form_factor(tau: f64, beta: u8, n: usize) -> f64 {
    let nf = n as f64;
    let x = tau / nf;
    match beta {
        1 => { let xx = 2.0*x; 2.0*xx - xx*log(1.0+2.0*xx) }
        2 => if x < 1.0 { x } else { 1.0 },
        4 => { let xx = 2.0*x; xx - xx/2.0*log(1.0+2.0*xx) }
        _ => 1.0,
    }
}

pub struct DoubleRamCycle { pub genus: usize, pub n_marks: usize }
impl DoubleRamCycle {
    pub fn new(genus: usize, n_marks: usize) -> Self { Self { genus, n_marks } }
    pub fn dimension(&self) -> isize { (3*self.genus) as isize - 3 + self.n_marks as isize }
    pub fn euler_char(&self) -> isize { 2 - 2*self.genus as isize }
    pub fn frobenius_deviation(&self) -> f64 {
        let d = self.dimension() as f64;
        if d <= 0.0 { return 1.0; }
        fabs(1.0/d - 1.0/(d*2.0)) * 0.5
    }
    pub fn moduli_dimension(&self) -> usize {
        if self.genus >= 2 { 3*self.genus - 3 + self.n_marks }
        else if self.genus == 1 { if self.n_marks > 0 { self.n_marks } else { 1 } }
        else { if self.n_marks > 3 { self.n_marks - 3 } else { 0 } }
    }
}

// ═══════════════════════════════════════════════════════════
// Report Formats
// ═══════════════════════════════════════════════════════════

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("=== Dyson Beta-Ensemble {} v{} ===\n", NAME, VERSION));
    s.push_str(&format!("Tuple: ⟨{}⟩\n", TUPLE_DRDA));
    s.push_str("──────────────────────────────────────\n");
    s.push_str("Ensemble      ⟨r⟩       K(0.5)       Wigner(1.0)\n");
    for beta in [1u8,2,4] {
        let l = match beta {1=>"GOE",2=>"GUE",4=>"GSE",_=>"?"};
        s.push_str(&format!("{:6}       {:.4}     {:.6}     {:.6}\n",
            l, mean_gap_ratio(beta), spectral_form_factor(0.5,beta,100), wigner_surmise(1.0,beta)));
    }
    s.push_str("──────────────────────────────────────\n");
    for (g,m) in [(0,4),(1,1),(2,4),(3,0)] {
        let dr = DoubleRamCycle::new(g, m);
        s.push_str(&format!("DRC(g={},m={}): dim={} χ={} μ∘δ_dev={:.6} M_dim={}\n",
            g, m, dr.dimension(), dr.euler_char(), dr.frobenius_deviation(), dr.moduli_dimension()));
    }
    s
}

pub fn summary_report() -> String {
    format!("DRDA v{}: ⟨{}⟩ | GOE ⟨r⟩={:.4} GUE ⟨r⟩={:.4} GSE ⟨r⟩={:.4}",
        VERSION, TUPLE_DRDA, mean_gap_ratio(1), mean_gap_ratio(2), mean_gap_ratio(4))
}

pub fn json_report() -> String {
    let mut s = String::new();
    s.push_str("{");
    s.push_str(&format!("\"name\":\"{}\",\"version\":\"{}\",\"tuple\":\"{}\",", NAME, VERSION, TUPLE_DRDA));
    s.push_str("\"ensembles\":{");
    for (i, beta) in [1u8,2,4].iter().enumerate() {
        if i>0 { s.push_str(","); }
        let l = match beta {1=>"GOE",2=>"GUE",4=>"GSE",_=>"?"};
        s.push_str(&format!("\"{}\":{{\"mean_gap_ratio\":{:.4},\"sff_k05\":{:.6}}}",
            l, mean_gap_ratio(*beta), spectral_form_factor(0.5,*beta,100)));
    }
    s.push_str("}}");
    s
}

// ═══════════════════════════════════════════════════════════
// Sub-commands
// ═══════════════════════════════════════════════════════════

pub fn report_wigner(s: f64, beta: u8) -> String {
    format!("Wigner({:.2}, β={}): P(s)={:.6}", s, beta, wigner_surmise(s, beta))
}

pub fn report_sff(tau: f64, beta: u8, n: usize) -> String {
    format!("SFF(τ={:.3}, β={}, N={}): K={:.6}", tau, beta, n, spectral_form_factor(tau, beta, n))
}

pub fn report_ramify(genus: usize, n_marks: usize) -> String {
    let dr = DoubleRamCycle::new(genus, n_marks);
    format!("DRC(g={},m={}): dim={} χ={} μ∘δ_dev={:.6} M_dim={} frob_pass={}",
        genus, n_marks, dr.dimension(), dr.euler_char(), dr.frobenius_deviation(),
        dr.moduli_dimension(), dr.frobenius_deviation() < 0.01)
}

pub fn help_text() -> &'static str {
    "DRDA — Dyson Beta-Ensemble + Double-Ramified Cycle\n\
     dyson              full report\n\
     dyson summary      one-line summary\n\
     dyson json         JSON structured output\n\
     dyson wigner <s> <β>  Wigner surmise at s (β=1,2,4)\n\
     dyson sff <τ> <β> <N>  spectral form factor\n\
     dyson ramify <g> <m>  double-ramified cycle analysis\n\
     dyson tuple        tuple constant"
}

// ═══════════════════════════════════════════════════════════
// Command Dispatch
// ═══════════════════════════════════════════════════════════

pub fn dispatch<'a>(sub: &str, mut args: impl Iterator<Item=&'a str>) -> String {
    match sub {
        "" | "report" | "full" => full_report(),
        "summary" => summary_report(),
        "json" => json_report(),
        "wigner" => {
            let s: f64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(1.0);
            let beta: u8 = args.next().and_then(|v| v.parse().ok()).unwrap_or(2);
            report_wigner(s, beta)
        }
        "sff" => {
            let tau: f64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(0.5);
            let beta: u8 = args.next().and_then(|v| v.parse().ok()).unwrap_or(2);
            let n: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(100);
            report_sff(tau, beta, n)
        }
        "ramify" => {
            let g: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(2);
            let m: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(4);
            report_ramify(g, m)
        }
        "tuple" => TUPLE_DRDA.to_string(),
        "help" | "--help" | "-h" => help_text().to_string(),
        _ => format!("DRDA: unknown sub-command '{}'. Try: dyson help", sub),
    }
}
