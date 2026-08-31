// braid_protocol.rs — The IMASM braiding protocol
//
// The evaluator sphere braids. This module carries the twelve IMASM tokens onto
// the tangle generators, so an IMASM program can be read as a braid word
// and its invariants taken.
//
// Author: Lando⊗⊙perator
// Date: 2026-08-02

#![allow(dead_code)]

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use alloc::format;
use libm::atan;
use crate::tokens::Token;

// ===========================================================================
// 1. THE TOKEN TABLE
// ===========================================================================

// One table, not two. Token already knows its own name and its own glyph, and
// a second copy here answered "UNKNOWN" for the three-arity and rotation tokens
// the type has carried for some time. Delegate.
pub fn token_name(tok: &Token) -> &'static str { tok.name() }

/// The token's canonical glyph — ⋈ for CLINK, ∈ ∋ for the dyad, ⊤ ⊥ for the
/// evaluators, ⊡ for IFIX. Exposed here so a caller reaching for the braid
/// protocol does not have to know which module owns the alphabet.
pub fn token_glyph(tok: &Token) -> &'static str { tok.code() }

/// Parse a token from a name OR a glyph.
///
/// This used to carry its own table of the twelve ASCII names and reject every
/// glyph, so `⋈` parsed through `Token::parse` and failed here — two parsers for
/// one type, disagreeing about the alphabet. `Token::parse` is the one that
/// knows both, including the short forms and the δ/μ spellings, and it is
/// explicit that the retired marks ◇ ● ☊ ☋ are not tokens.
pub fn parse_token_name(name: &str) -> Option<Token> { Token::parse(name) }

pub fn stack_delta(tok: &Token) -> i32 {
    match tok {
        Token::Vinit | Token::Engagr | Token::Fsplit => 1,
        Token::Tanch | Token::Ifix | Token::Ffuse => -1,
        _ => 0,
    }
}

pub fn tangle_role(tok: &Token) -> &'static str {
    match tok {
        Token::Vinit => "unit eta - cup, strand birth",
        Token::Tanch => "counit epsilon - cap, strand death",
        Token::Fsplit => "comultiplication delta - trivalent split",
        Token::Ffuse => "multiplication mu - trivalent fuse, n-ary",
        Token::Afwd => "braid sigma_i - positive crossing",
        Token::Arev => "braid sigma_i^-1 - negative crossing",
        Token::Clink => "readout w_ij - winding/linking number, no extension",
        Token::Imscrib => "Markov closure - braid becomes link",
        Token::Evalt => "phase marker phi_1 - strand held",
        Token::Evalf => "phase marker phi_2 - strand held",
        Token::Engagr => "phase marker phi_3 + cup - strand held and one born",
        Token::Ifix => "framing twist - the ! promotion, seals a strand",
        _ => "unknown role",
    }
}

pub fn strand_footprint(tok: &Token) -> usize {
    let d = stack_delta(tok);
    if d > 0 {
        1 + d as usize
    } else {
        1
    }
}

pub fn evaluator_footprint() -> (usize, usize, usize) {
    (
        strand_footprint(&Token::Evalt),
        strand_footprint(&Token::Evalf),
        strand_footprint(&Token::Engagr),
    )
}

pub fn tilt_radians() -> f64 {
    let (t, f, i) = evaluator_footprint();
    atan(1.0 / (t + f + i) as f64)
}

// ===========================================================================
// 2. A PROGRAM AS A TANGLE
// ===========================================================================

#[derive(Clone, Debug)]
pub struct TangleReading {
    pub program: Vec<Token>,
    pub depth_profile: Vec<i32>,
    pub generators: Vec<i32>,
    pub strands: usize,
    pub closes: bool,
    pub is_markov_closed: bool,
    pub crossings: usize,
    pub writhe: i32,
}

impl TangleReading {
    pub fn to_string(&self) -> String {
        let mut lines = Vec::new();
        let mut prog_str = Vec::new();
        for t in &self.program {
            prog_str.push(String::from(token_name(t)));
        }
        lines.push(format!("  program   : {}", prog_str.join(" ")));
        let mut depth_str = Vec::new();
        for d in &self.depth_profile {
            depth_str.push(format!("{}", d));
        }
        lines.push(format!("  depth     : {}", depth_str.join(" ")));
        let mut gen_str = Vec::new();
        for g in &self.generators {
            gen_str.push(format!("{}", g));
        }
        lines.push(format!("  braid     : strands={} generators=[{}]", self.strands, gen_str.join(", ")));
        lines.push(format!("  crossings : {}   writhe: {:+}", self.crossings, self.writhe));
        lines.push(format!("  closes    : {}  (depth returns to its start)", self.closes));
        lines.push(format!("  Markov    : {}  (IMSCRIB bookends)", self.is_markov_closed));
        lines.join("\n")
    }
}

pub fn read_tangle(program: &[Token], n_strands: usize, start_depth: i32) -> Result<TangleReading, String> {
    let mut depth = start_depth;
    let mut profile = Vec::new();
    profile.push(depth);
    let mut generators = Vec::new();

    for tok in program {
        if *tok == Token::Afwd || *tok == Token::Arev {
            if depth < 2 {
                return Err(format!("{} at depth {}: nothing to cross", token_name(tok), depth));
            }
            let sign = if *tok == Token::Afwd { 1 } else { -1 };
            generators.push(sign * (depth - 1));
        }
        depth += stack_delta(tok);
        profile.push(depth);
    }

    let writhe: i32 = generators.iter().map(|g| g.signum()).sum();
    let markov = program.len() >= 2 && program[0] == Token::Imscrib && program[program.len() - 1] == Token::Imscrib;

    Ok(TangleReading {
        program: program.to_vec(),
        depth_profile: profile,
        generators,
        strands: n_strands,
        closes: depth == start_depth,
        is_markov_closed: markov,
        crossings: program.iter().filter(|&&t| t == Token::Afwd || t == Token::Arev).count(),
        writhe,
    })
}

// ===========================================================================
// 3. COMPILATION
// ===========================================================================

pub fn braid_to_imasm(generators: &[i32], start_depth: i32, close: bool) -> Vec<Token> {
    let mut prog = Vec::new();
    let mut depth = start_depth;
    for &g in generators {
        let want = g.abs() + 1;
        while depth < want {
            prog.push(Token::Fsplit);
            depth += 1;
        }
        while depth > want {
            prog.push(Token::Ffuse);
            depth -= 1;
        }
        prog.push(if g > 0 { Token::Afwd } else { Token::Arev });
    }
    if close {
        while depth > start_depth {
            prog.push(Token::Ffuse);
            depth -= 1;
        }
        while depth < start_depth {
            prog.push(Token::Fsplit);
            depth += 1;
        }
    }
    prog
}

// ===========================================================================
// 4. LINKING AND SEALS
// ===========================================================================

pub fn linking_matrix(generators: &[i32], n_strands: usize) -> (Vec<Vec<i32>>, Vec<usize>) {
    let mut pos: Vec<usize> = (0..n_strands).collect();
    let mut lk = vec![vec![0; n_strands]; n_strands];
    for &g in generators {
        let i = (g.abs() - 1) as usize;
        if i + 1 >= n_strands {
            continue;
        }
        let a = pos[i];
        let b = pos[i + 1];
        let half = if g > 0 { 1 } else { -1 };
        lk[a][b] += half;
        lk[b][a] += half;
        pos.swap(i, i + 1);
    }
    for r in 0..n_strands {
        for c in 0..n_strands {
            if lk[r][c] % 2 == 0 {
                lk[r][c] /= 2;
            }
        }
    }
    (lk, pos)
}

pub fn arrival(generators: &[i32], n_strands: usize) -> Vec<usize> {
    linking_matrix(generators, n_strands).1
}

pub fn is_pure(generators: &[i32], n_strands: usize) -> bool {
    let arr = arrival(generators, n_strands);
    let expected: Vec<usize> = (0..n_strands).collect();
    arr == expected
}

pub struct Seal {
    pub name: &'static str,
    pub paths: Vec<(&'static str, Vec<i32>)>,
    pub n_strands: usize,
}

impl Seal {
    pub fn is_seal(&self) -> bool {
        if self.paths.is_empty() { return false; }
        
        let first_arr = arrival(&self.paths[0].1, self.n_strands);
        let all_arrive_together = self.paths.iter().all(|(_, p)| arrival(p, self.n_strands) == first_arr);
        if !all_arrive_together { return false; }

        let mut unique_windings = Vec::new();
        for (_, path) in &self.paths {
            let (lk, _) = linking_matrix(path, self.n_strands);
            let mut w = Vec::new();
            for i in 0..self.n_strands {
                for j in (i+1)..self.n_strands {
                    w.push(lk[i][j]);
                }
            }
            if !unique_windings.contains(&w) {
                unique_windings.push(w);
            }
        }
        unique_windings.len() == self.paths.len()
    }
}

pub fn evaluator_seal() -> Seal {
    Seal {
        name: "the three evaluator arms as P_3 generators",
        paths: vec![
            ("EVALT  A_12", vec![1, 1]),
            ("EVALF  A_23", vec![2, 2]),
            ("ENGAGR A_13", vec![2, 1, 1, -2]),
        ],
        n_strands: 3,
    }
}

// ===========================================================================
// 5. SPHERE PROGRAMS
// ===========================================================================

pub fn evaluator_sphere_program(generation: usize) -> Vec<Token> {
    if generation == 0 {
        return Vec::new();
    }
    let mut n = 3usize;
    for _ in 1..generation {
        n *= 3;
    }
    let markers = n;
    let splits = markers - 1;
    
    let mut body = Vec::new();
    for i in 0..markers {
        body.push(match i % 3 {
            0 => Token::Evalt,
            1 => Token::Evalf,
            _ => Token::Engagr,
        });
    }

    let births = body.iter().filter(|&&t| t == Token::Engagr).count();
    let fuses = markers + births - 1;

    let mut prog = Vec::new();
    prog.push(Token::Imscrib);
    for _ in 0..splits {
        prog.push(Token::Fsplit);
    }
    prog.extend(body);
    for _ in 0..fuses {
        prog.push(Token::Ffuse);
    }
    prog.push(Token::Imscrib);
    prog
}

// ===========================================================================
// 6. REPORT
// ===========================================================================

pub fn report() -> String {
    let mut lines = Vec::new();
    lines.push(String::from("======================================================================"));
    lines.push(String::from("THE IMASM BRAIDING PROTOCOL"));
    lines.push(String::from("======================================================================"));
    lines.push(String::from(""));
    lines.push(String::from("Token map (arity from sequence.rs stack_delta):"));
    lines.push(String::from(""));
    lines.push(format!("  {:9} {:>2}  {:>4}  role", "token", "d", "foot"));
    lines.push(String::from("  ------------------------------------------------------------------"));
    
    let tokens = [
        Token::Vinit, Token::Tanch, Token::Afwd, Token::Arev, Token::Clink, Token::Imscrib,
        Token::Fsplit, Token::Ffuse, Token::Evalt, Token::Evalf, Token::Engagr, Token::Ifix
    ];
    for t in &tokens {
        lines.push(format!("  {:9} {:+2}  {:4}  {}", token_name(t), stack_delta(t), strand_footprint(t), tangle_role(t)));
    }
    lines.push(String::from(""));

    let (t, f, i) = evaluator_footprint();
    lines.push(String::from("The tilt, by two routes:"));
    lines.push(format!("  evaluator strand footprint T:F:I = {}:{}:{}", t, f, i));
    lines.push(String::from("  B4 popcount              T:F:I = 1:1:2"));
    let tilt = tilt_radians();
    lines.push(format!("  pitch = arctan(1/{}) = {:.4} deg", t + f + i, tilt * 180.0 / 3.141592653589793));
    lines.push(String::from(""));

    lines.push(String::from("The sphere read as a tangle, by generation:"));
    lines.push(format!("  {:>3} {:>8} {:>8} {:>11} {:>7} {:>7}", "gen", "markers", "strands", "per triple", "tokens", "closes"));
    
    for gen in 1..=4 {
        let prog = evaluator_sphere_program(gen);
        let mut width = 3usize;
        for _ in 1..gen { width *= 3; }
        let standing = width + width / 3;
        let triples = width / 3;
        let per_triple = standing as f64 / triples as f64;
        let r = read_tangle(&prog, width, 1).unwrap();
        lines.push(format!("  {:>3} {:>8} {:>8} {:>11.4} {:>7} {:>7}", gen, width, standing, per_triple, prog.len(), r.closes));
    }
    lines.push(String::from(""));

    lines.push(String::from("Generation 1 in full:"));
    let g1 = evaluator_sphere_program(1);
    let r1 = read_tangle(&g1, 3, 1).unwrap();
    lines.push(r1.to_string());
    lines.push(String::from(""));

    let seal = evaluator_seal();
    lines.push(format!("Seal verification ({}): is_seal = {}", seal.name, seal.is_seal()));
    lines.push(String::from("======================================================================"));
    lines.join("\n")
}
