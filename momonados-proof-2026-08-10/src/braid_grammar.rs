// braid_grammar.rs — Braid Grammar Bridge (native mOMonadOS port)
// Map Fibonacci braid words → Imscribing Grammar tuples.
#![allow(dead_code)]
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use libm::{fabs, fmod, cos};

pub struct BraidWord {
    pub strands: usize,
    pub generators: Vec<i32>,
}

impl BraidWord {
    pub fn new(strands: usize, generators: &[i32]) -> Self {
        Self { strands, generators: generators.to_vec() }
    }
    pub fn from_string(s: &str, strands: usize) -> Self {
        let mut gens = Vec::new();
        for token in s.split_whitespace() {
            if let Ok(n) = token.parse::<i32>() { gens.push(n); }
        }
        Self { strands, generators: gens }
    }
    pub fn writhe(&self) -> i32 { self.generators.iter().map(|g| g.signum()).sum() }
    pub fn crossing_count(&self) -> usize { self.generators.len() }
    pub fn fusion_dim(&self) -> usize {
        if self.strands <= 1 { return 1; }
        let (mut a, mut b) = (1usize, 1usize);
        for _ in 2..self.strands { let c = a + b; a = b; b = c; }
        b
    }
    pub fn topological_spin(&self) -> f64 { fmod(self.writhe() as f64 * 2.0 / 5.0, 1.0) }
    pub fn eigenvalue_winding(&self) -> f64 { self.topological_spin() * self.fusion_dim() as f64 }

    fn gram_dim(&self) -> &'static str { match self.fusion_dim() { 0=>"𐑛",1=>"𐑨",2..=10=>"𐑼",_=>"𐑦" } }
    fn gram_top(&self) -> &'static str { match self.crossing_count() { 0=>"𐑡",1..=3=>"𐑰",4..=8=>"𐑥",9..=15=>"𐑶",_=>"𐑸" } }
    fn gram_coupling(&self) -> &'static str { match self.strands { 0..=2=>"𐑩",3..=4=>"𐑑",_=>"𐑽" } }
    fn gram_parity(&self) -> &'static str {
        let s = fabs(self.topological_spin());
        if s<0.01{"𐑗"}else if s<0.3{"𐑿"}else if s<0.5{"𐑬"}else{"𐑯"}
    }
    fn gram_fidelity(&self) -> &'static str { let j=fabs(cos(self.topological_spin()*2.0)); if j>0.9{"𐑱"}else if j>0.5{"𐑞"}else{"𐑐"} }
    fn gram_kin(&self) -> &'static str {
        let c = self.generators.len() as f64 / (self.strands*3) as f64;
        if c<0.3{"𐑺"}else if c<0.6{"𐑪"}else if c<1.0{"𐑧"}else{"𐑤"}
    }
    fn gram_card(&self) -> &'static str { match self.strands { 0..=3=>"𐑲",4..=8=>"𐑚",_=>"𐑔" } }
    fn gram_comp(&self) -> &'static str { match self.generators.len() { 0..=2=>"𐑝",3..=5=>"𐑠",_=>"𐑵" } }
    fn gram_crit(&self) -> &'static str {
        let w = self.topological_spin();
        if fabs(w)<0.01{"𐑢"}else if fabs(w-0.4)<0.05{"⊙"}else if fabs(w-0.5)<0.05{"𐑻"}else{"𐑣"}
    }
    fn gram_chir(&self) -> &'static str {
        let w = self.writhe();
        if w==0{"𐑓"}else if w.abs()<=2{"𐑒"}else if w.abs()<=5{"𐑖"}else{"𐑫"}
    }
    fn gram_stoi(&self) -> &'static str {
        match self.fusion_dim() { 0..=1=>"𐑙",2..=5=>"𐑕",_=>"𐑳" }
    }
    fn gram_wind(&self) -> &'static str {
        let w = fabs(self.eigenvalue_winding());
        if w<0.01{"𐑷"}else if fabs(w-0.5)<0.05{"𐑴"}else if w<2.0{"𐑭"}else{"𐑟"}
    }

    pub fn to_grammar_tuple(&self) -> String {
        format!("{}{}{}{}{}{}{}{}{}{}{}{}",
            self.gram_dim(),self.gram_top(),self.gram_coupling(),self.gram_parity(),
            self.gram_fidelity(),self.gram_kin(),self.gram_card(),self.gram_comp(),
            self.gram_crit(),self.gram_chir(),self.gram_stoi(),self.gram_wind())
    }
}

pub fn full_report() -> String {
    let mut s = String::new();
    s.push_str("Braid Grammar Bridge\n");
    for (name, strands, gens) in &[("σ₁σ₂σ₁",3,vec![1,2,1]), ("σ₁σ₂σ₁σ₂σ₁",3,vec![1,2,1,2,1])] {
        let bw = BraidWord::new(*strands, gens);
        s.push_str(&format!("{} ({}str): ⟨{}⟩\n", name, strands, bw.to_grammar_tuple()));
    }
    s
}
