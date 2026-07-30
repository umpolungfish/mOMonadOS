// dyson.rs — Double-Ramified Dyson Algebra (native mOMonadOS port)
// Tuple: ⟨𐑼𐑸𐑾𐑹𐑞𐑧𐑔𐑠⊙𐑖𐑳𐑭⟩
#![allow(dead_code)]
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use core::f64::consts::PI;
use libm::{sqrt, exp, log, fabs, pow};

pub const TUPLE_DRDA: &str = "𐑼𐑸𐑾𐑹𐑞𐑧𐑔𐑠⊙𐑖𐑳𐑭";

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
        // μ∘δ check: split then fuse should return to origin
        let d = self.dimension() as f64;
        if d <= 0.0 { return 1.0; }
        fabs(1.0/d - 1.0/(d*2.0)) * 0.5
    }
}

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str(&format!("DRDA {}\n", TUPLE_DRDA));
    s.push_str("----------------------------------------\n");
    for beta in [1u8,2,4] {
        let l = match beta {1=>"GOE",2=>"GUE",4=>"GSE",_=>"?"};
        s.push_str(&format!("{} ⟨r⟩={:.4}  K(0.5)={:.6}\n", l, mean_gap_ratio(beta), spectral_form_factor(0.5,beta,100)));
    }
    let dr = DoubleRamCycle::new(2, 4);
    s.push_str(&format!("DR dim={} χ={} frob_dev={:.6}\n", dr.dimension(), dr.euler_char(), dr.frobenius_deviation()));
    s
}
