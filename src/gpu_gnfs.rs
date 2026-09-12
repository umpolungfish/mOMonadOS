//! gpu_gnfs.rs — Grammar-native general number field sieve on the GPU.
//!
//! Blueprint: `grammar_native_general_number_field_sieve_ob3ect`
//! (`ob3ect/digital/grammar_native_general_number_field_sieve/`).
//! The glyph word IS the pipeline; stages are not free-floating helpers.
//!
//!   Word: ⊢⊙≻∈⊤⊥⊞⊡≺⋈≻∋⊤⊣   (period 14)
//!   Fork/fuse: ∈@3 → ∋@11
//!   Surface: polynomial_selection · smooth_relation ·
//!            linear_dependence · congruence_of_squares
//!
//! Submodules (siblings under src/):
//!   gpu_gnfs_poly   — ⊙ base-m (f, g)
//!   gpu_gnfs_sieve  — ≻∈⊤⊥⊞⊡≺ lattice sieve + relations
//!   gpu_gnfs_linalg — ⋈≻∋⊤⊣ matrix, dependency, φ-congruence, factor

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::gpu_gnfs_fb::build_factor_bases_gpu;
use crate::gpu_gnfs_linalg::format_dep;
use crate::gpu_gnfs_linalg_gpu::{congruence_factor_gpu, find_dependencies_gpu};
use crate::gpu_gnfs_poly::{choose_degree, select_polynomial, PolyPair};
use crate::gpu_gnfs_sieve::{
    collect_relations, sieve_status_line, soak, FactorBases, Relation, SieveParams, SieveReport,
};

/// The blueprint word — fourteen glyphs, one pipeline.
pub const WORD: &str = "⊢⊙≻∈⊤⊥⊞⊡≺⋈≻∋⊤⊣";

/// Blueprint stages in word order (phase_4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Init,
    Polynomial,
    Sieve,
    Split,
    SmoothRational,
    SmoothAlgebraic,
    Paradox,
    RelationLog,
    Backtrack,
    Matrix,
    LinearAlgebra,
    Congruence,
    FactorCheck,
    Done,
}

impl Stage {
    pub const ALL: [Stage; 14] = [
        Stage::Init,
        Stage::Polynomial,
        Stage::Sieve,
        Stage::Split,
        Stage::SmoothRational,
        Stage::SmoothAlgebraic,
        Stage::Paradox,
        Stage::RelationLog,
        Stage::Backtrack,
        Stage::Matrix,
        Stage::LinearAlgebra,
        Stage::Congruence,
        Stage::FactorCheck,
        Stage::Done,
    ];

    pub fn glyph(self) -> &'static str {
        match self {
            Stage::Init => "⊢",
            Stage::Polynomial => "⊙",
            Stage::Sieve => "≻",
            Stage::Split => "∈",
            Stage::SmoothRational => "⊤",
            Stage::SmoothAlgebraic => "⊥",
            Stage::Paradox => "⊞",
            Stage::RelationLog => "⊡",
            Stage::Backtrack => "≺",
            Stage::Matrix => "⋈",
            Stage::LinearAlgebra => "≻",
            Stage::Congruence => "∋",
            Stage::FactorCheck => "⊤",
            Stage::Done => "⊣",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Stage::Init => "init",
            Stage::Polynomial => "polynomial",
            Stage::Sieve => "sieve",
            Stage::Split => "split",
            Stage::SmoothRational => "smooth_rational",
            Stage::SmoothAlgebraic => "smooth_algebraic",
            Stage::Paradox => "paradox",
            Stage::RelationLog => "relation_log",
            Stage::Backtrack => "backtrack",
            Stage::Matrix => "matrix",
            Stage::LinearAlgebra => "linear_algebra",
            Stage::Congruence => "congruence",
            Stage::FactorCheck => "factor_check",
            Stage::Done => "done",
        }
    }

    pub fn element(self) -> &'static str {
        match self {
            Stage::Init => "Uninitialized Integer N",
            Stage::Polynomial => "Self-Referential Polynomial",
            Stage::Sieve => "Sieving Forward",
            Stage::Split => "Rational vs Algebraic Side",
            Stage::SmoothRational => "Smooth Relation Found",
            Stage::SmoothAlgebraic => "Non-Smooth Rejection",
            Stage::Paradox => "Paradoxical Smoothness",
            Stage::RelationLog => "Relation Log",
            Stage::Backtrack => "Backtracking on Non-Smooth",
            Stage::Matrix => "Matrix Assembly",
            Stage::LinearAlgebra => "Linear Dependence (Lanczos/Wiedemann)",
            Stage::Congruence => "Congruence of Squares",
            Stage::FactorCheck => "Non-Trivial Factor Verify",
            Stage::Done => "Factorization Completion",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageStatus {
    Host,
    Gpu,
    Stub,
}

pub fn stage_status(s: Stage) -> StageStatus {
    match s {
        Stage::Init | Stage::Polynomial => StageStatus::Host,
        Stage::Sieve
        | Stage::Split
        | Stage::SmoothRational
        | Stage::SmoothAlgebraic
        | Stage::Paradox
        | Stage::RelationLog
        | Stage::Backtrack => StageStatus::Gpu, // GPU assists; host verifies
        Stage::Matrix | Stage::LinearAlgebra | Stage::Congruence | Stage::FactorCheck | Stage::Done => {
            StageStatus::Host
        }
    }
}

fn status_tag(st: StageStatus) -> &'static str {
    match st {
        StageStatus::Host => "host",
        StageStatus::Gpu => "gpu",
        StageStatus::Stub => "stub",
    }
}

fn choose_b(bits: u64, b_arg: u64) -> u64 {
    if b_arg != 0 {
        return b_arg;
    }
    // Classical GNFS factor-base scale: ~ √L_N[1/3, (8/9)^{1/3}].
    let ln_n = (bits as f64) * core::f64::consts::LN_2;
    let ln_ln = ln_n.ln().max(2.0);
    let c = (8.0_f64 / 9.0).powf(1.0 / 3.0);
    let l13 = c * ln_n.powf(1.0 / 3.0) * ln_ln.powf(2.0 / 3.0);
    let b_asym = l13.exp().sqrt().ceil() as u64;
    let floor = if bits < 16 {
        40
    } else if bits < 24 {
        100
    } else if bits < 32 {
        200
    } else if bits < 48 {
        500
    } else if bits < 64 {
        2_000
    } else if bits < 96 {
        5_000
    } else if bits < 128 {
        10_000
    } else if bits < 256 {
        50_000
    } else if bits < 512 {
        200_000
    } else {
        500_000
    };
    b_asym.max(floor)
}

struct GnfsState {
    n: BigUint,
    b: u64,
    poly: Option<PolyPair>,
    fb: Option<FactorBases>,
    sieve: Option<SieveReport>,
    relations: Vec<Relation>,
    deps: Vec<Vec<usize>>,
    factor: Option<BigUint>,
}

impl GnfsState {
    fn init(n: BigUint, b: u64) -> Self {
        Self {
            n,
            b,
            poly: None,
            fb: None,
            sieve: None,
            relations: Vec::new(),
            deps: Vec::new(),
            factor: None,
        }
    }
}

pub fn blueprint() -> String {
    let mut lines = Vec::new();
    lines.push(format!("gpu_gnfs blueprint word: {WORD}"));
    lines.push(String::from(
        "  fork/fuse: ∈@3 → ∋@11  (rational|algebraic → congruence of squares)",
    ));
    lines.push(String::from("  stages (glyph  name  element  status):"));
    for (i, s) in Stage::ALL.iter().enumerate() {
        lines.push(format!(
            "  {:>2} {}  {:<16}  {:<32}  {}",
            i + 1,
            s.glyph(),
            s.name(),
            s.element(),
            status_tag(stage_status(*s))
        ));
    }
    lines.push(String::from(
        "  ExOS: poly-select(host) · lattice-sieve(gpu/host) · sparse-matrix(host) · φ-congruence(host) · B(alfs)",
    ));
    lines.join("\n")
}

/// Report only the ⊙ polynomial pair for N (no full pipeline).
pub fn run_poly(n_str: &str, degree: Option<u32>) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("gpu_gnfs: '{n_str}' not an integer"),
    };
    match select_polynomial(&n, degree) {
        Ok(p) => {
            let check = p.eval_at_m() == n;
            format!(
                "gpu_gnfs ⊙ poly N={} bits={}\n  degree {}\n  m {}\n  g(x) = {}\n  f(x) = {}\n  f(m)=N: {}\n  coeff_score {}",
                n,
                n.bits(),
                p.degree,
                p.m,
                p.format_g(),
                p.format_f(),
                check,
                p.coeff_score()
            )
        }
        Err(e) => e,
    }
}

fn run_pipeline(st: &mut GnfsState) -> Result<(), String> {
    // ⊢ already done in init
    // ⊙
    let pair = select_polynomial(&st.n, None)?;
    eprintln!(
        "gpu_gnfs ⊙: d={} m_bits={} coeff_score={} f(m)=N — g(x)={}",
        pair.degree,
        pair.m.bits(),
        pair.coeff_score(),
        pair.format_g()
    );
    st.poly = Some(pair.clone());

    // ≻∈⊤⊥⊞⊡≺ — factor bases (GPU) + sieve (GPU)
    let t_alfs = std::time::Instant::now();
    eprintln!("gpu_gnfs ≻ device: building factor bases (GPU)…");
    let (fb, fb_gpu, fb_ms) = build_factor_bases_gpu(&pair, st.b);
    eprintln!(
        "gpu_gnfs ⊢/alfs: B={} |rat|={} |alg|={} ncols={} ({} ms, gpu={})",
        st.b,
        fb.rational.len(),
        fb.algebraic.len(),
        fb.ncols(),
        fb_ms,
        fb_gpu
    );
    let _ = t_alfs;
    let params = SieveParams::from_b(st.b, st.n.bits(), fb.ncols());
    eprintln!("gpu_gnfs ≻ device: classical log-line sieve (GPU)…");
    let rep = collect_relations(&pair, &fb, &params);
    eprintln!("{}", sieve_status_line(&rep, &fb));
    if rep.relations.len() < fb.ncols() + 1 {
        return Err(format!(
            "gpu_gnfs ≺: only {} relations for {} columns — widen B or bounds",
            rep.relations.len(),
            fb.ncols()
        ));
    }
    st.relations = rep.relations.clone();
    st.sieve = Some(rep);
    st.fb = Some(fb);

    // ⋈≻ — matrix + dependency on device
    let (deps, dep_gpu, dep_ms) = find_dependencies_gpu(&st.relations, 64);
    eprintln!(
        "gpu_gnfs ⋈≻: {} relations → {} dependencies ({} ms, gpu={})",
        st.relations.len(),
        deps.len(),
        dep_ms,
        dep_gpu
    );
    if deps.is_empty() {
        return Err(String::from(
            "gpu_gnfs ⋈: no GF(2) dependency — need more relations",
        ));
    }
    st.deps = deps;

    // ∋⊤⊣ — congruence of squares on device (u64 and multi-limb)
    let (sq, cong_gpu, cong_ms) =
        congruence_factor_gpu(&st.n, &pair, &st.relations, &st.deps)?;
    if let Some(f) = sq.factor {
        eprintln!(
            "gpu_gnfs ∋⊤: factor {} via dep {} (x={} y={}) ({} ms, gpu={})",
            f,
            format_dep(&sq.dep),
            sq.x,
            sq.y,
            cong_ms,
            cong_gpu
        );
        st.factor = Some(f);
        Ok(())
    } else {
        Err(format!(
            "gpu_gnfs ∋: dependency {} gave no nontrivial factor",
            format_dep(&sq.dep)
        ))
    }
}

/// Run the full GNFS pipeline for N.
pub fn run_factor(n_str: &str, b: u64) -> String {
    let n: BigUint = match n_str.trim().parse() {
        Ok(v) => v,
        Err(_) => return format!("gpu_gnfs: '{n_str}' not an integer"),
    };
    if n.is_zero() || n.is_one() {
        return format!("gpu_gnfs: N={n} is not a composite target");
    }
    if n == BigUint::from(2u32) || n == BigUint::from(3u32) {
        return format!("gpu_gnfs: N={n} is prime — nothing to factor");
    }
    if crate::native_numeral::modulo_small_via_word(&n, 2).unwrap() == 0 {
        return format!("gpu_gnfs {n}: trivial factor 2  (divides: true)  [⊢ short-circuit]");
    }

    let b0 = choose_b(n.bits(), b);
    let mut last_err = String::new();
    // Auto-escalate B a few times when the sieve/congruence rung is short.
    let schedule: Vec<u64> = if b != 0 {
        alloc::vec![b0]
    } else {
        alloc::vec![b0, b0.saturating_mul(2).max(b0 + 40), b0.saturating_mul(4).max(b0 + 80)]
    };

    for &b_try in &schedule {
        let mut st = GnfsState::init(n.clone(), b_try);
        eprintln!(
            "gpu_gnfs: bits={} B={} word={WORD} d≈{} — ⊢ opening",
            st.n.bits(),
            st.b,
            choose_degree(st.n.bits())
        );
        match run_pipeline(&mut st) {
            Ok(()) => {
                let f = st.factor.expect("factor set on Ok");
                let cof = crate::native_numeral::divmod_via_word(&st.n, &f).unwrap().0;
                let poly_note = st
                    .poly
                    .as_ref()
                    .map(|p| format!(" d={} m={}", p.degree, p.m))
                    .unwrap_or_default();
                let nrel = st.relations.len();
                let ndep = st.deps.len();
                return format!(
                    "gpu_gnfs {}: factor {}  cofactor {}  (divides: true)  [⊣]{} relations={} deps={} B={} gpu={} device={} gpu_ms={}",
                    st.n,
                    f,
                    cof,
                    poly_note,
                    nrel,
                    ndep,
                    st.b,
                    st.sieve.as_ref().map(|s| s.used_gpu).unwrap_or(false),
                    st.sieve.as_ref().map(|s| s.device).unwrap_or(0),
                    st.sieve.as_ref().map(|s| s.gpu_ms).unwrap_or(0)
                );
            }
            Err(e) => {
                eprintln!("gpu_gnfs: B={b_try} failed — {e}");
                last_err = e;
            }
        }
    }
    last_err
}

pub fn help() -> String {
    String::from(
        "gpu_gnfs <n> [B] | poly <n> [d] | soak [secs] [B] | blueprint | help\n\
         Grammar-native GNFS on GPU — word ⊢⊙≻∈⊤⊥⊞⊡≺⋈≻∋⊤⊣ from the GNFS ob3ect.\n\
         B = factor-base bound (0 → auto from bit-length). GNFS_GPU=0|1 selects device.\n\
         soak = keep the device under load for secs (default 10) so nvidia-smi -l 1 sees it.\n\
         ⊢ init · ⊙ base-m poly · ≻∈⊤⊥⊞⊡≺ classical log-line sieve (GPU) ·\n\
         ⋈≻ GF(2) dependency (GPU) · ∋ φ-congruence (GPU, multi-limb) · ⊤ factor · ⊣ done.",
    )
}

pub fn run_soak(secs: u64, b: u64) -> String {
    soak(secs, b)
}
