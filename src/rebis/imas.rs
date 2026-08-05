//! imas.rs — IMASM Arranger Bridge
//! Port of imas/arranger.py + ig_bridge.py + clink_bridge.py + frobenius_hunter.py

use crate::rebis::pipeline::IgTuple;
use crate::imas_ig::IgPrim;

pub const VINIT:   u8 = 0;
pub const TANCH:   u8 = 1;
pub const AFWD:    u8 = 2;
pub const AREV:    u8 = 3;
pub const CLINK_T: u8 = 4;
pub const IMSCRIB: u8 = 5;
pub const FSPLIT:  u8 = 6;
pub const FFUSE:   u8 = 7;
pub const EVALT:   u8 = 8;
pub const EVALF:   u8 = 9;
pub const ENGAGR:  u8 = 10;
pub const IFIX:    u8 = 11;

pub fn token_name(t: u8) -> &'static str {
    match t { 0=>"VINIT",1=>"TANCH",2=>"AFWD",3=>"AREV",4=>"CLINK",5=>"IMSCRIB",
              6=>"FSPLIT",7=>"FFUSE",8=>"EVALT",9=>"EVALF",10=>"ENGAGR",11=>"IFIX", _=>"???" }
}

pub fn token_family(t: u8) -> u8 {
    match t { 0..=5=>0, 6|7=>1, 8..=10=>2, 11=>3, _=>255 }
}

pub fn signature(arr: &[u8]) -> (u8, u8, u8, u8) {
    let (mut l,mut f,mut d,mut x) = (0,0,0,0);
    for &t in arr { match token_family(t) { 0=>l+=1,1=>f+=1,2=>d+=1,3=>x+=1,_=>{} } }
    (l,f,d,x)
}

#[derive(Clone,Debug)]
pub struct StructFingerprint {
    pub length: usize,
    pub sig_l: u8, pub sig_f: u8, pub sig_d: u8, pub sig_x: u8,
    pub start_token: u8, pub end_token: u8,
    pub self_ref: bool, pub frobenius_order: u8,
    pub dialetheia_complete: bool, pub period: u8, pub token_mask: u16,
}

impl StructFingerprint {
    pub fn token_diversity(&self) -> u32 { self.token_mask.count_ones() }
}

pub fn fingerprint(arr: &[u8]) -> StructFingerprint {
    let n = arr.len();
    let sig = signature(arr);
    let start = if n>0 {arr[0]} else {0};
    let end = if n>0 {arr[n-1]} else {0};
    let mut frob = 0u8;
    let mut hs=false; let mut hf=false;
    for i in 0..n {
        if arr[i]==FSPLIT {hs=true;} else if arr[i]==FFUSE {hf=true;}
        if i+1<n && arr[i]==FSPLIT && arr[i+1]==FFUSE {frob=1;}
        if i+1<n && arr[i]==FFUSE && arr[i+1]==FSPLIT {frob=2;}
    }
    if hs&&hf&&frob==0 {frob=3;}
    let mut ht=false; let mut he=false; let mut hg=false;
    for &t in arr { if t==EVALT {ht=true;} else if t==EVALF {he=true;} else if t==ENGAGR {hg=true;} }
    let dial = ht&&he&&hg;
    let period = if n<=1 {n as u8} else {
        let mut p=1u8;
        'outer: while p<=(n as u8)/2 {
            for i in (p as usize)..n { if arr[i]!=arr[i-p as usize] {p+=1; continue 'outer;} }
            break;
        }
        p
    };
    let mut mask: u16=0;
    for &t in arr { mask|=1u16<<(t as u16); }
    StructFingerprint{length:n,sig_l:sig.0,sig_f:sig.1,sig_d:sig.2,sig_x:sig.3,
        start_token:start,end_token:end,self_ref:start==end,frobenius_order:frob,
        dialetheia_complete:dial,period,token_mask:mask}
}

/// Fingerprint → IG Tuple
pub fn fingerprint_to_ig(fp: &StructFingerprint) -> IgTuple {
    use IgPrim::*;
    let td = fp.token_diversity();
    let d = if td<=2 {dead} else if td<=5 {ash} else if td<=9 {array} else {if_};
    let t = if fp.self_ref {are} else if fp.period==1 {judge}
        else if fp.period==2 {mime} else if fp.frobenius_order>0 {oil} else {eat};
    let r = match fp.frobenius_order {1=>ian,2=>ear,3=>tot,_=>ado};
    let p = if fp.frobenius_order==1 {or_} else if fp.frobenius_order==2 {nun}
        else if fp.frobenius_order==3 {out} else if fp.dialetheia_complete {yew} else {church};
    let f = if fp.dialetheia_complete {peep} else if fp.period==1 {age} else {they};
    let k = if fp.sig_x==8 {loll} else if fp.period==1 {egg}
        else if fp.period<=4 {on} else {air};
    let g = if fp.sig_x>=3 {bib} else if fp.sig_x>=1 {ice}
        else if td<=3 {thigh} else {ice};
    let c = if fp.frobenius_order>0 {measure} else if fp.period==1 {vow}
        else if fp.period==2 {gag} else {ooze};
    let phi = if fp.self_ref&&fp.dialetheia_complete {monad} else if fp.self_ref {roar}
        else if fp.dialetheia_complete {err} else if fp.period==1 {woe} else {haha};
    let h = match fp.period {1=>fee,2=>kick,3=>sure,_=>wool};
    let nz = (fp.sig_l>0)as u8+(fp.sig_f>0)as u8+(fp.sig_d>0)as u8+(fp.sig_x>0)as u8;
    let s = if nz==1 {hung} else if nz==2 {so} else {up};
    let omega = if fp.frobenius_order==1 {ah} else if fp.frobenius_order==2 {oak}
        else if fp.self_ref {ah} else if fp.period==2 {oak} else {awe};
    IgTuple{d,t,r,p,f,k,g,c,phi,h,s,omega}
}

pub const CANONICAL_NAMES: [&str;12] = [
    "I_Dialetheic_Bootstrap","II_Void_Genesis","III_Anchor_Protocol",
    "IV_Dual_Bootstrap","V_Linear_Chain","VI_Empty_Bootstrap",
    "VII_Parakernel","VIII_Frobenius_Kernel","IX_Chiral_Pairs",
    "X_Truth_Machine","XI_Eternal_Return","XII_ROM_Burn"];

pub fn canonical_sequence(idx: usize) -> Option<&'static [u8]> {
    match idx {
        0=>Some(&[VINIT,TANCH,AFWD,AREV,CLINK_T,IMSCRIB,EVALT,EVALF]),
        1=>Some(&[VINIT,TANCH,IFIX]),
        2=>Some(&[VINIT,TANCH,AFWD,CLINK_T,IMSCRIB,EVALT,ENGAGR,IFIX]),
        3=>Some(&[AFWD,AREV,CLINK_T,IMSCRIB,VINIT,TANCH,EVALT,EVALF]),
        4=>Some(&[VINIT,TANCH,AFWD,AREV,CLINK_T,IMSCRIB,IFIX,IFIX]),
        5=>Some(&[]),
        6=>Some(&[VINIT,TANCH,AFWD,AREV,FSPLIT,FFUSE,CLINK_T,IMSCRIB]),
        7=>Some(&[FSPLIT,AFWD,FFUSE,AREV,FSPLIT,CLINK_T,FFUSE,IMSCRIB]),
        8=>Some(&[AFWD,AREV,CLINK_T,IMSCRIB,AREV,AFWD,IMSCRIB,CLINK_T]),
        9=>Some(&[EVALT,EVALF,ENGAGR,IFIX,EVALT,EVALF,ENGAGR,IFIX]),
        10=>Some(&[VINIT,TANCH,AFWD,AREV,CLINK_T,IMSCRIB,VINIT,TANCH]),
        11=>Some(&[VINIT,TANCH,IFIX,IFIX,IFIX,IFIX,IFIX,IFIX]),
        _=>None,
    }
}

pub fn has_frobenius_pair(arr: &[u8]) -> bool {
    let mut saw=false;
    for &t in arr { if t==FSPLIT {saw=true;} else if t==FFUSE&&saw {return true;} }
    false
}

pub fn imasm_to_clink(arr: &[u8]) -> (usize, &'static str, f64, &'static str) {
    let ig = fingerprint_to_ig(&fingerprint(arr));
    let (idx,dist) = crate::rebis::clink::nearest_clink_layer(&ig);
    (idx, crate::rebis::clink::CLINK_NAMES[idx], dist, crate::rebis::clink::CLINK_TIERS[idx])
}

pub fn bridge_all_report() -> alloc::string::String {
    let mut s = alloc::string::String::from("══ IMASM→CLINK Bridge (All 12 Canonicals) ══\n");
    for i in 0..12 {
        if let Some(seq) = canonical_sequence(i) {
            let (idx,name,dist,tier) = imasm_to_clink(seq);
            let toks: alloc::vec::Vec<&str> = seq.iter().map(|&t| token_name(t)).collect();
            s.push_str(&alloc::format!("  {:2} {:24} → L{} {:20} d={:.2} {}\n    {}\n",
                i, CANONICAL_NAMES[i], idx, name, dist, tier, toks.join(" ")));
        }
    }
    s
}

pub fn verify_bootstrap(arr: &[u8]) -> alloc::string::String {
    let _fp = fingerprint(arr);
    let pair = has_frobenius_pair(arr);
    let imsc = arr.iter().any(|&t| t==IMSCRIB);
    let cl = arr.iter().any(|&t| t==CLINK_T);
    let fix = arr.iter().any(|&t| t==IFIX);
    let ok = pair&&imsc&&cl&&fix;
    alloc::format!("══ IMASM Bootstrap Verify ══\n  Frobenius: {}\n  IMSCRIB: {}\n  CLINK: {}\n  IFIX: {}\n  VERDICT: {}",
        if pair {"PASS"} else {"FAIL"}, if imsc {"✓"} else {"✗"},
        if cl {"✓"} else {"✗"}, if fix {"✓"} else {"✗"},
        if ok {"FROBENIUS-COMPLETE"} else {"OPEN"})
}

pub fn imasm_summary() -> alloc::string::String {
    use crate::style as S;
    // The token count is the length of the canonical table, not a number
    // written beside it: the two used to be able to disagree.
    let n = CANONICAL_NAMES.len();
    let mut s = alloc::string::String::new();
    s.push_str(&alloc::format!("  {}IMASM arranger{}  {}{} canonicals{}\n\n",
        S::heading(), S::reset(), S::muted(), n, S::reset()));
    s.push_str(&alloc::format!("  {}{:2} {:22} {:>4}  {:^4} {:^4} {:^4} {:>4}{}\n",
        S::muted(), "#", "name", "len", "frob", "self", "dial", "per", S::reset()));
    for i in 0..n {
        if let Some(seq) = canonical_sequence(i) {
            let fp = fingerprint(seq);
            // A mark that carries a verdict is coloured by that verdict, so the
            // column reads at a glance instead of tick by tick.
            let mark = |b: bool| if b {
                alloc::format!("{}✓{}", S::verdict_t(), S::reset())
            } else {
                alloc::format!("{}·{}", S::verdict_n(), S::reset())
            };
            s.push_str(&alloc::format!("  {}{:2}{} {}{:22}{} {:>4}  {:^4} {:^4} {:^4} {:>4}\n",
                S::muted(), i, S::reset(),
                S::key(), CANONICAL_NAMES[i], S::reset(),
                fp.length,
                mark(has_frobenius_pair(seq)), mark(fp.self_ref),
                mark(fp.dialetheia_complete), fp.period));
        }
    }
    s
}
