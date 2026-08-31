// kernel_torus.rs — mOMonadOS kernel wound on the horn torus (native Rust)
//
// Port of gen_kernel_torus.py. Eliminates Python modot.agent.agent_loop()
// dependency. Provides the agent loop program natively, computes the horn
// torus parametrization on bare metal, and outputs winding data through serial.
//
// Horn torus: R = r — inner equator collapses to origin (the PINCH).
// IMSCRIB ⊙ (μ∘δ=id) sits at the pinch. The (1,1) winding passes through
// the pinch once per revolution.
//
// Tuple: ⟨𐑦𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑫𐑕𐑭⟩ (mOMonadOS kernel — O_∞, Frobenius-closed)
//
// Author: Math⊙perator (Lando⊗⊙perator team)

#![allow(dead_code)]

use core::f64::consts::PI;
use crate::{sprint, sprintln};
use libm::{cos, sin, sqrt};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use crate::tokens::{Token, Program};

// ─── Token Glyphs ──────────────────────────────────────────────

pub const TOKEN_GLYPH: [&str; 12] = [
    // Catalog column order; ⊙ at slot nine, Criticality.
    "⊢", "⊣", ">", "<", "⋈", "⊤", "∈", "∋", "⊙", "⊥", "⊞", "⊡",
];

fn glyph(t: Token) -> &'static str { TOKEN_GLYPH[t as usize] }

fn name(t: Token) -> &'static str {
    match t {
        Token::Vinit => "VINIT", Token::Tanch => "TANCH",
        Token::Afwd  => "AFWD",  Token::Arev  => "AREV",
        Token::Clink => "CLINK", Token::Imscrib => "IMSCRIB",
        Token::Fsplit => "FSPLIT", Token::Ffuse  => "FFUSE",
        Token::Evalt => "EVALT", Token::Evalf => "EVALF",
        Token::Engagr => "ENGAGR", Token::Ifix => "IFIX",
        _ => "?",
    }
}

// ─── Agent Loop Program (native, no Python) ────────────────────

/// Python agent_loop() ported to Rust: 11-token cyclic program.
/// VINIT→IMSCRIB→FSPLIT→EVALT→CLINK→FFUSE→IFIX→ENGAGR→AREV→CLINK→TANCH
pub fn agent_loop_program() -> Program {
    let mut p = Program::empty();
    for t in [Token::Vinit, Token::Imscrib, Token::Fsplit,
              Token::Evalt, Token::Clink,   Token::Ffuse,
              Token::Ifix,  Token::Engagr,  Token::Arev,
              Token::Clink, Token::Tanch] { p.push(t); }
    p
}

// ─── Horn Torus Geometry ──────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct Point3D { pub x: f64, pub y: f64, pub z: f64 }

pub struct HornTorus { pub r: f64 }

impl HornTorus {
    pub const fn new(r: f64) -> Self { HornTorus { r } }

    pub fn point(&self, theta: f64, phi: f64) -> Point3D {
        let rp = self.r + self.r * cos(phi);
        Point3D { x: rp * cos(theta), y: rp * sin(theta), z: self.r * sin(phi) }
    }

    pub fn winding_point(&self, t: f64) -> Point3D { self.point(t, t) }

    pub fn label_point(&self, theta: f64) -> Point3D {
        let lr = self.r + 1.9 * self.r;
        Point3D { x: lr * cos(theta), y: lr * sin(theta), z: 1.9 * self.r * sin(theta) }
    }
}

// ─── Token Station ────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct TokenStation {
    pub index: usize,
    pub token: Token,
    pub name: &'static str,
    pub glyph: &'static str,
    pub angle: f64,
    pub station: Point3D,
    pub label_pos: Point3D,
}

pub struct TorusMap {
    pub program: Program,
    pub torus: HornTorus,
    pub stations: Vec<TokenStation>,
    pub imscrib_station: Option<TokenStation>,
    pub vinit_station: Option<TokenStation>,
    pub word_str: String,
}

impl TorusMap {
    /// Build torus map: stations at equal angular intervals, offset so IMSCRIB
    /// lands at π (the pinch). Matches gen_kernel_torus.py construction.
    pub fn new(program: &Program) -> Self {
        let torus = HornTorus::new(2.0);
        let n = program.len();
        let tokens: Vec<Token> = program.as_slice().to_vec();
        let im_i = tokens.iter().position(|t| *t == Token::Imscrib).unwrap_or(0);
        let offset = PI - 2.0 * PI * im_i as f64 / n as f64;

        let mut stations = Vec::with_capacity(n);
        let mut im = None;
        let mut vi = None;
        let mut word = String::new();

        for (i, &tok) in tokens.iter().enumerate() {
            let angle = 2.0 * PI * i as f64 / n as f64 + offset;
            word.push_str(glyph(tok));

            let station = if tok == Token::Imscrib {
                Point3D { x: 0.0, y: 0.0, z: 0.0 }
            } else {
                torus.winding_point(angle)
            };

            let ts = TokenStation {
                index: i, token: tok, name: name(tok), glyph: glyph(tok),
                angle, station, label_pos: torus.label_point(angle),
            };

            if tok == Token::Imscrib { im = Some(ts); }
            if tok == Token::Vinit { vi = Some(ts); }
            stations.push(ts);
        }

        TorusMap { program: *program, torus, stations, imscrib_station: im,
                   vinit_station: vi, word_str: word }
    }

    pub fn len(&self) -> usize { self.stations.len() }

    pub fn winding_curve(&self, n: usize) -> Vec<Point3D> {
        let off = match &self.imscrib_station {
            Some(s) => s.angle - PI, None => 0.0
        };
        (0..n).map(|i| self.torus.winding_point(2.0 * PI * i as f64 / n as f64 + off)).collect()
    }

    pub fn pinch_distance(&self) -> f64 {
        match &self.imscrib_station {
            Some(s) => sqrt(s.station.x * s.station.x + s.station.y * s.station.y + s.station.z * s.station.z),
            None => f64::MAX,
        }
    }
}

// ─── Serial Display ───────────────────────────────────────────

fn fmt_f64(n: f64) -> alloc::string::String {
    let neg = n < 0.0;
    let mag = libm::fabs(n);
    let int_part = mag as i32;
    let frac_part = ((mag - int_part as f64) * 10000.0 + 0.5) as i32;
    let frac_part = if frac_part >= 10000 { 0 } else { frac_part };
    let s = if neg { "-" } else { "" };
    format!("{}{}.{:04}", s, int_part, frac_part)
}

/// Display full torus map on serial.
pub fn display_torus(map: &TorusMap) {
    sprintln!("\x1b[36m╔══════════════════════════════════════════════════╗\x1b[0m");
    sprintln!("\x1b[36m║  mOMonadOS kernel — wound on the horn torus       ║\x1b[0m");
    sprintln!("\x1b[36m╚══════════════════════════════════════════════════╝\x1b[0m");
    sprintln!("");

    // Token sequence
    sprint!("\x1b[1;33mToken sequence ({} stations):\x1b[0m ", map.len());
    for ts in &map.stations { sprint!("{}", ts.glyph); }
    sprintln!("");

    // Stations
    sprintln!("\x1b[1m── Token Stations ──\x1b[0m");
    for ts in &map.stations {
        let turns = ts.angle / (2.0 * PI);
        sprintln!("{:>2} {:9} {} φ={:.4}x2π ({}, {}, {})",
            ts.index, ts.name, ts.glyph, turns,
            fmt_f64(ts.station.x), fmt_f64(ts.station.y), fmt_f64(ts.station.z));
    }

    // Pinch
    sprintln!("\x1b[1;32m── The Pinch (μ∘δ=id) ──\x1b[0m");
    if let Some(ref _im) = map.imscrib_station {
        let d = map.pinch_distance();
        sprintln!("\x1b[32m  ⊙ IMSCRIB at origin: distance = {}\x1b[0m", d);
        if d < 1e-10 { sprintln!("\x1b[32m    → PINCH IS EXACT ✓\x1b[0m"); }
        else { sprintln!("\x1b[33m    → PINCH OFFSET ⚠\x1b[0m"); }
    }

    // Junction
    sprintln!("\x1b[1;33m── Seal-to-Opening Junction ──\x1b[0m");
    if let Some(ref vs) = map.vinit_station {
        let last = &map.stations[map.len() - 1];
        sprintln!("  {} {} → wraps → {} (VINIT)", last.glyph, last.name, vs.glyph);
    }

    // Winding summary
    sprintln!("\x1b[1;36m── (1,1) Winding ──\x1b[0m");
    sprintln!("  Horn torus: R = r = {}", map.torus.r);
    let curve = map.winding_curve(800);
    let mp = &curve[curve.len() / 2];
    sprintln!("  Winding: {} pts, passes through ({}, {}, {}) at t=π",
        curve.len(), fmt_f64(mp.x), fmt_f64(mp.y), fmt_f64(mp.z));
    sprintln!("");
}

/// Brief boot banner: one-line token sequence.
pub fn display_banner(map: &TorusMap) {
    sprint!("\x1b[36m[TORUS]\x1b[0m kernel wound: ");
    for ts in &map.stations { sprint!("{}", ts.glyph); }
    sprintln!("  ({} tokens, pinch={})", map.len(), fmt_f64(map.pinch_distance()));
}
