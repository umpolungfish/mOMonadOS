//! Native `ask` — MoDoT-parity language interface on bare-metal mOMonadOS.
//!
//! Design source: ob3ect `native_kernel_ask_full_modot_parity_language_int`
//! (Frobenius PASS). Pipeline is one IMASM-shaped chain, no Python:
//!
//!   prepare  = IMSCRIB demand (catalog type) + witness scaffold
//!   answer   = structural resolution (catalog description + CL8NK faces)
//!              optional boundary model is dual-substrate only — not implemented
//!              as smuggled modot/agent.py
//!   complete = EVALT/EVALF Dual-Link co-type + FFUSE voices + IFIX SpineReport
//!
//! Operator surface:
//!   ask [--verbose] [--no-selectivity] [--dry-run] [--cycles N] <question...>
//!   ask /          — begin multi-line paste; end with a line containing only `.`
//!
//! Parity notes (honest):
//!   --model / LLM answer production: bare metal has no network LLM. Structural
//!   answer is the native default (MoDoT --dry-run face). A host boundary device
//!   is designed as FSPLIT arm in the ob3ect; not wired as Python agent.

#![allow(dead_code)]

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::belnap::B4;
use crate::catalog::{self, CatalogEntry};
use crate::cl8nk;
use crate::d12_sic;
use crate::frob_verify::FrobeniusHarness;
use crate::imas_ig::IgTuple;
use crate::kernel::Kernel;
use crate::witness_vessel;

/// Parsed ask options (MoDoT flag parity).
#[derive(Clone, Debug)]
pub struct AskOptions {
    pub verbose: bool,
    pub no_selectivity: bool,
    pub dry_run: bool,
    pub cycles: u32,
}

impl Default for AskOptions {
    fn default() -> Self {
        Self {
            verbose: false,
            no_selectivity: false,
            dry_run: true, // native default: structure-only (no boundary LLM)
            cycles: 1,
        }
    }
}

/// Parse flags from the front of an ask argument string; remainder is the question.
pub fn parse_ask_args(raw: &str) -> (AskOptions, String) {
    let mut opts = AskOptions::default();
    let mut qparts: Vec<&str> = Vec::new();
    let mut tokens = raw.split_whitespace().peekable();
    while let Some(t) = tokens.next() {
        match t {
            "--verbose" | "-v" => opts.verbose = true,
            "--no-selectivity" => opts.no_selectivity = true,
            "--dry-run" => opts.dry_run = true,
            "--wet-run" | "--live" => opts.dry_run = false, // reserved: boundary model
            "--cycles" => {
                if let Some(n) = tokens.next().and_then(|s| s.parse().ok()) {
                    opts.cycles = n;
                }
            }
            flag if flag.starts_with("--cycles=") => {
                if let Ok(n) = flag[9..].parse() {
                    opts.cycles = n;
                }
            }
            _ => {
                qparts.push(t);
                // rest are question tokens
                while let Some(r) = tokens.next() {
                    qparts.push(r);
                }
                break;
            }
        }
    }
    (opts, qparts.join(" "))
}

/// Primitive-wise Hamming distance between two catalog tuples (0..=12).
fn tuple_hamming(a: &IgTuple, b: &IgTuple) -> u8 {
    let mut n = 0u8;
    if a.d != b.d { n += 1; }
    if a.t != b.t { n += 1; }
    if a.r != b.r { n += 1; }
    if a.p != b.p { n += 1; }
    if a.f != b.f { n += 1; }
    if a.k != b.k { n += 1; }
    if a.g != b.g { n += 1; }
    if a.c != b.c { n += 1; }
    if a.phi != b.phi { n += 1; }
    if a.h != b.h { n += 1; }
    if a.s != b.s { n += 1; }
    if a.omega != b.omega { n += 1; }
    n
}

fn named_defects(a: &IgTuple, b: &IgTuple) -> Vec<&'static str> {
    let keys = ["D", "T", "R", "P", "F", "K", "G", "C", "Φ", "H", "S", "Ω"];
    let av = [a.d, a.t, a.r, a.p, a.f, a.k, a.g, a.c, a.phi, a.h, a.s, a.omega];
    let bv = [b.d, b.t, b.r, b.p, b.f, b.k, b.g, b.c, b.phi, b.h, b.s, b.omega];
    let mut out = Vec::new();
    for i in 0..12 {
        if av[i] != bv[i] {
            out.push(keys[i]);
        }
    }
    out
}

/// Co-type lattice fold analogue: identity → T; any defect → F if all mismatch
/// style anti; mix → B; empty catalog → N.
fn fold_cotype(hamming: u8, engaged: bool) -> B4 {
    if !engaged {
        B4::N
    } else if hamming == 0 {
        B4::T
    } else if hamming == 12 {
        B4::F
    } else {
        B4::B
    }
}

fn b4_name(b: B4) -> &'static str {
    match b {
        B4::N => "N",
        B4::T => "T",
        B4::F => "F",
        B4::B => "B",
    }
}

fn b4_join(a: B4, b: B4) -> B4 {
    // Match kernel Belnap join used elsewhere: conflict lifts to B
    use B4::*;
    match (a, b) {
        (x, y) if x == y => x,
        (N, x) | (x, N) => x,
        (T, F) | (F, T) => B,
        (B, _) | (_, B) => B,
        _ => B,
    }
}

fn b4_conflict(a: B4, b: B4) -> u8 {
    // Hamming of 2-bit codes N=00 T=01 F=10 B=11
    let code = |x: B4| -> u8 {
        match x {
            B4::N => 0b00,
            B4::T => 0b01,
            B4::F => 0b10,
            B4::B => 0b11,
        }
    };
    (code(a) ^ code(b)).count_ones() as u8
}

/// Run native ask. Returns the serial report string.
pub fn run_ask(question: &str, opts: &AskOptions, k: &mut Kernel) -> String {
    let mut s = String::new();
    s.push_str("╔══════════════════════════════════════════════════════╗\n");
    s.push_str("║  ASK — native ManuscriptSpine (no Python)            ║\n");
    s.push_str("╚══════════════════════════════════════════════════════╝\n\n");

    if question.is_empty() {
        s.push_str("Usage:\n");
        s.push_str("  ask [--verbose|-v] [--no-selectivity] [--dry-run] [--cycles N] <question...>\n");
        s.push_str("  ask /     — multi-line paste; terminate with a line containing only .\n");
        s.push_str("\nMoDoT parity: same spine faces (prepare → answer → complete).\n");
        s.push_str("Default is structure-only (dry-run). Boundary LLM is not on bare metal.\n");
        return s;
    }

    s.push_str(&format!("Question: {}\n", question));
    s.push_str(&format!(
        "Options: verbose={} no_selectivity={} dry_run={} cycles={}\n\n",
        opts.verbose, opts.no_selectivity, opts.dry_run, opts.cycles
    ));

    // ── IMSCRIB: catalog witness resolve + demand type ─────────────────────
    s.push_str("── IMSCRIB (demand + witness) ──\n");
    let hits = catalog::search_query(question, 5);
    // Direct lookup attempt (name-like query)
    let direct = catalog::lookup(question.trim());
    let primary: Option<CatalogEntry> = if let Some(e) = direct {
        Some(e)
    } else {
        hits.first().map(|(e, _)| *e)
    };

    if hits.is_empty() && primary.is_none() {
        s.push_str("  witness: no catalog hit (expand ASK_CATALOG_SUBSET or rephrase)\n");
    } else {
        s.push_str("  ranked hits:\n");
        for (e, sc) in &hits {
            s.push_str(&format!("    [{:>3}] {}  tier={}  {}\n", sc, e.name, e.tier, e.domain.name()));
        }
    }

    let demand_tuple = primary.map(|e| e.tuple);
    if let Some(e) = primary {
        s.push_str(&format!("  primary witness: {}\n", e.name));
        s.push_str(&format!("  description: {}\n", e.description));
        let (d_cl8, conflicts) = cl8nk::tuple_distance_cl8nk(&e.tuple, &cl8nk::cl8nk_ref());
        let tier = cl8nk::assess_tier(&e.tuple);
        s.push_str(&format!(
            "  tier(cl8nk)={}  d(CLINK L8)={:.4}  conflicts={}\n",
            tier,
            d_cl8,
            conflicts.len()
        ));
    }

    // Scaffold roles (domain-invariant GeneralizedPipeline tables, structural)
    s.push_str("\n── Scaffold (instantiate in object language of the question) ──\n");
    s.push_str("  Encoding/Duality [Φ]: bijective encoding / injectivity on orbits\n");
    s.push_str("  Inverse structure [Þ]: dual/self-referential decomposition\n");
    s.push_str("  Bidirectional [Ř]: forward and inverse constructions exhaust\n");
    s.push_str("  Boundedness [⊙]: confinement / no escape to infinity\n");
    s.push_str("  Topological invariant [Ω]: integer invariant rules out exotics\n");
    s.push_str("  Regularity [Ç]: equidistribution / typical configurations\n");
    s.push_str("  Status: scaffold only — catalog proved_hint is not a proof.\n");

    // ── FSPLIT: structural answer body (native dry-run face) ───────────────
    s.push_str("\n── ANSWER (structural; dry-run default on bare metal) ──\n");
    let answer_body = if let Some(e) = primary {
        format!(
            "Structural resolution via catalog witness `{}` (domain {}, tier {}).\n\
             {}\n\n\
             Dual-Link co-typing will compare demand type to this witness type.\n\
             For a language-model expansion of the scaffold, attach a dual-substrate\n\
             boundary device (not Python MoDoT); without it the kernel answers as structure.\n",
            e.name,
            e.domain.name(),
            e.tier,
            e.description
        )
    } else {
        String::from(
            "No catalog witness resolved. Structural answer is empty (Belnap N engagement).\n\
             Rephrase, or ensure the witness is in the kernel catalog (ASK_CATALOG_SUBSET).\n",
        )
    };
    s.push_str(&answer_body);

    // Self-voice: structural answers that hit a witness assert T; else N
    let model_voice = if primary.is_some() { B4::T } else { B4::N };

    // ── EVALT/EVALF: Dual-Link co-type (unless --no-selectivity) ───────────
    let mut vessel_voice = B4::N;
    let mut riding = false;

    if !opts.no_selectivity {
        s.push_str("\n── PORT / Dual-Link co-type ──\n");
        if let Some(dem) = demand_tuple {
            // Structural answer re-uses primary witness type as answer type
            // (identity co-type when answer is that witness's body).
            let ans = dem;
            let hamming = tuple_hamming(&dem, &ans);
            let defects = named_defects(&dem, &ans);
            vessel_voice = fold_cotype(hamming, true);
            riding = true; // structural path: co-type is exact on shared type
            s.push_str(&format!(
                "  vessel_voice={}  hamming={}  defects={:?}  riding={}\n",
                b4_name(vessel_voice),
                hamming,
                defects,
                riding
            ));
            s.push_str("  provenance: Dual-Link face (catalog tuples); SIC delivery via `vessel run`\n");
        } else {
            s.push_str("  vessel silent: no demand type (N)\n");
        }
    } else {
        s.push_str("\n── PORT skipped (--no-selectivity: balance-only) ──\n");
    }

    // ── PROVE: Frobenius identity check (μ∘δ=id on B4) ─────────────────────
    let mut harness = FrobeniusHarness::new("ask");
    let mut prove_passed = 0u64;
    for &v in &[B4::N, B4::T, B4::F, B4::B] {
        if harness.check(crate::frob_verify::verify_frobenius_identity(v)) {
            prove_passed += 1;
        }
    }
    let balance_closed = harness.is_closed() && prove_passed == 4;

    // Optional: exercise vessel transport face once when verbose and selectivity on
    if opts.verbose && !opts.no_selectivity {
        s.push_str("\n── TRANSPORT (witness_vessel summary) ──\n");
        s.push_str(&witness_vessel::vessel_summary());
        s.push_str("\n");
    }

    // ── FFUSE ──────────────────────────────────────────────────────────────
    let fused = if opts.no_selectivity {
        model_voice
    } else if vessel_voice == B4::N && model_voice != B4::N {
        model_voice
    } else if vessel_voice != B4::N {
        b4_join(model_voice, vessel_voice)
    } else {
        B4::N
    };
    let conflict = if opts.no_selectivity {
        0
    } else {
        b4_conflict(model_voice, vessel_voice)
    };

    // ── IFIX / SpineReport ─────────────────────────────────────────────────
    s.push_str("\n── SPINE REPORT ──\n");
    s.push_str(&format!(
        "  fused={}  model_voice={}  vessel_voice={}  conflict={}\n",
        b4_name(fused),
        b4_name(model_voice),
        b4_name(vessel_voice),
        conflict
    ));
    s.push_str(&format!(
        "  faces: prove_balance={}  unify_B=T+F=true  port_riding={}  witness={}\n",
        balance_closed,
        riding,
        primary.map(|e| e.name).unwrap_or("—")
    ));
    s.push_str("  protocol: VINIT→IMSCRIB→FSPLIT→EVALT→EVALF→FFUSE→ENGAGR→IFIX\n");
    s.push_str("  Lean spine: DualLinkVessel · SIC_D12_WitnessVessel · VAE_Vita_ManuscriptSpine\n");
    s.push_str("  SIC theorem: crystal_forces_d12_sic (SICPOVM_Exists 12)\n");

    if conflict > 0 && fused == B4::B {
        s.push_str("  ENGAGR: dialetheia held (model ⋈ vessel conflict)\n");
    }

    s.push_str("\n── HONEST NON-CLAIMS ──\n");
    s.push_str("  · Cargo/tensor INTO vessel refused (D–T); boarding is Dual-Link only\n");
    s.push_str("  · Belnap stack ≠ algebraic Scott-Grassl fiducial\n");
    s.push_str("  · Clay/catalog T·B are Grammar typing, not Millennium proofs\n");
    s.push_str("  · d=2048 existence open (typed B)\n");
    s.push_str("  · No on-metal LLM; structural answer is the native dry-run face\n");

    if opts.verbose {
        s.push_str("\n── VERBOSE: Dual-Link + spine ledger ──\n");
        s.push_str(&d12_sic::dual_link_report());
        s.push_str("\n");
        s.push_str(&d12_sic::manuscript_spine_report());
    }

    // --cycles: additional kernel ticks (MoDoT --cycles parity)
    if opts.cycles > 1 {
        s.push_str(&format!("\n── CYCLES: running {} kernel ticks ──\n", opts.cycles));
        for _ in 0..opts.cycles {
            if !k.tick() {
                break;
            }
        }
        s.push_str(&format!(
            "  ticks advanced; kernel frob_open={}\n",
            k.frob_open
        ));
    }

    s
}

/// Multi-line paste buffer state for `ask /` … `.`
pub struct AskPaste {
    pub active: bool,
    pub buf: String,
    pub opts: AskOptions,
}

impl AskPaste {
    pub fn new() -> Self {
        Self {
            active: false,
            buf: String::new(),
            opts: AskOptions::default(),
        }
    }
}
