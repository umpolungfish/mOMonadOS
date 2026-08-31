// ─── mOMonadOS Menu System ──────────────────────────────────
// Hierarchical menu navigation, sub-context REPLs, tab completion
// Enhanced: every command includes a usage example
#![allow(dead_code)]

extern crate alloc;

// ─── Menu Item ─────────────────────────────────────────────

pub struct MenuItem {
    pub name: &'static str,
    pub cmd: &'static str,
    pub desc: &'static str,
    pub example: &'static str,
    pub submenu: Option<&'static [MenuItem]>,
}

// ─── Top-level menu categories ────────────────────────────

pub static MAIN_MENU: &[MenuItem] = &[
    MenuItem { name: "Exec",     cmd: "exec",     desc: "Execution (run, tick, watch, timer, boot)", example: "", submenu: Some(EXEC_MENU) },
    MenuItem { name: "Status",   cmd: "status",   desc: "Status (program, snapshot, graph, heatmap, registers)", example: "", submenu: Some(STATUS_MENU) },
    MenuItem { name: "Programs", cmd: "programs",  desc: "Program loading (list, canonical, continuous, novel, shunt)", example: "", submenu: Some(PROGRAMS_MENU) },
    MenuItem { name: "Crystal",  cmd: "crystal",  desc: "Crystal FS (decode, store, find, name)", example: "", submenu: Some(CRYSTAL_MENU) },
    MenuItem { name: "Grammar",  cmd: "grammar",  desc: "Grammar bridges (ig, classify, frob, aleph, shor, rh, ym)", example: "", submenu: Some(GRAMMAR_MENU) },
    MenuItem { name: "Quantum",  cmd: "quantum",  desc: "Quantum computation (fibqc, jones, braids, shor, iuft, sic, d12, d2048)", example: "help quantum", submenu: Some(QUANTUM_MENU) },
    MenuItem { name: "IMASM",    cmd: "imasm",    desc: "IMASM word walks (cycle, weight, banked, insert, trans, arev)", example: "", submenu: Some(IMASM_MENU) },
    MenuItem { name: "Kernel",   cmd: "kernel",   desc: "Kernel utilities (ask, spine, vessel, vita, whoami, ruleset)", example: "", submenu: Some(KERNEL_MENU) },
    MenuItem { name: "Rebis",    cmd: "rebis",    desc: "Red-Hot Rebis (codon, translate, genetics, materials, bio, tx)", example: "", submenu: Some(REBIS_MENU) },
    MenuItem { name: "Dialect", cmd: "dialect",  desc: "Cross-dialect (ruleset, jump, seal, compound, whoami)", example: "", submenu: Some(DIALECT_MENU) },
    MenuItem { name: "ParaASM",  cmd: "parasm",   desc: "ParaASM (test, frob, kernel, load)", example: "", submenu: Some(PARASM_MENU) },
    MenuItem { name: "Cr3echrz", cmd: "cr3echrz", desc: "Theorem engine + p4rakernel (cr3, p4ra)", example: "", submenu: Some(CR3ECHRZ_MENU) },
    MenuItem { name: "Seals",    cmd: "seals",     desc: "Sealed proofs — walk constant closure proofs step by step", example: "", submenu: Some(SEALS_MENU) },

    MenuItem { name: "Proof",    cmd: "proof",    desc: "Guided proofs — walk a proof step by step on the kernel", example: "", submenu: Some(PROOF_MENU) },
    MenuItem { name: "Help",     cmd: "help",     desc: "Help system (help <topic> for details)", example: "help fibqc", submenu: None },
];

pub static SEALS_MENU: &[MenuItem] = &[
    MenuItem { name: "list",           cmd: "seals list",           desc: "List all 10 sealed proofs", example: "seals list", submenu: None },
    MenuItem { name: "fine-structure", cmd: "seals fine-structure", desc: "α⁻¹ = d²−7 + arctan(1/4)/(4√3) + α²·d (3 steps)", example: "seals fine-structure", submenu: None },
    MenuItem { name: "proton",         cmd: "seals proton",         desc: "m_p/m_e = d³ + d(d−3) + α-dressing (2 steps)", example: "seals proton", submenu: None },
    MenuItem { name: "lepton",         cmd: "seals lepton",         desc: "m_μ/m_e (exact rational), m_τ/m_e (2 steps)", example: "seals lepton", submenu: None },
    MenuItem { name: "boson",          cmd: "seals boson",          desc: "m_W, m_Z, m_H — π + ω forms (2 steps)", example: "seals boson", submenu: None },
    MenuItem { name: "gravity",        cmd: "seals gravity",        desc: "α_G = α¹⁸·√3 (1 step)", example: "seals gravity", submenu: None },
    MenuItem { name: "weinberg",       cmd: "seals weinberg",       desc: "sin²θ_W = 3/13 (exact rational, 1 step)", example: "seals weinberg", submenu: None },
    MenuItem { name: "cosmology",      cmd: "seals cosmology",      desc: "ρ_Λ/ρ_Pl = e^{-44ω}/744 (1 step)", example: "seals cosmology", submenu: None },
    MenuItem { name: "neutrino",       cmd: "seals neutrino",       desc: "m₁:m₂:m₃ = 1:4:16 (1 step)", example: "seals neutrino", submenu: None },
    MenuItem { name: "winding",        cmd: "seals winding",        desc: "ω = 2π — all angles in windings (1 step)", example: "seals winding", submenu: None },
    MenuItem { name: "residuals",      cmd: "seals residuals",      desc: "Where every remainders comes from (1 step)", example: "seals residuals", submenu: None },
    MenuItem { name: "all",            cmd: "seals all",            desc: "GRAND SEAL — walk through all 10", example: "seals all", submenu: None },
];

pub static PROOF_MENU: &[MenuItem] = &[
    MenuItem { name: "list",      cmd: "proof list",      desc: "List available guided proofs", example: "proof list", submenu: None },
    MenuItem { name: "bootstrap", cmd: "proof bootstrap", desc: "The Grammar verifying itself (7 steps, auto-play)", example: "proof bootstrap", submenu: None },
];

pub static EXEC_MENU: &[MenuItem] = &[
    MenuItem { name: "tick",     cmd: "tick",     desc: "Run N manual ticks (default 1)", example: "tick 10", submenu: None },
    MenuItem { name: "run",      cmd: "run",      desc: "Run N ticks; no arg = continuous (ESC to stop)", example: "run 100", submenu: None },
    MenuItem { name: "watch",    cmd: "watch",    desc: "Live terminal HUD (ESC to stop)", example: "watch", submenu: None },
    MenuItem { name: "timer",    cmd: "timer",    desc: "Run N ticks, one per PIT interrupt", example: "timer 50", submenu: None },
    MenuItem { name: "boot",     cmd: "boot",     desc: "Load + run any program (I-XXVIII or decimal)", example: "boot VII", submenu: None },
    MenuItem { name: "load",     cmd: "load",     desc: "Load program by Roman numeral", example: "load XII", submenu: None },
];

pub static STATUS_MENU: &[MenuItem] = &[
    MenuItem { name: "status",   cmd: "status",   desc: "Kernel status (tick, IP, stack, fork, frob)", example: "status", submenu: None },
    MenuItem { name: "program",  cmd: "program",  desc: "Show loaded program + fork depth", example: "program", submenu: None },
    MenuItem { name: "snapshot", cmd: "snapshot", desc: "Structural snapshot (sig, tier, period)", example: "snapshot", submenu: None },
    MenuItem { name: "graph",    cmd: "graph",    desc: "ASCII-art token graph with nesting", example: "graph", submenu: None },
    MenuItem { name: "heatmap",  cmd: "heatmap",  desc: "B4 memory heatmap", example: "heatmap", submenu: None },
    MenuItem { name: "memory",   cmd: "memory",   desc: "Dump B4 memory", example: "memory", submenu: None },
    MenuItem { name: "registers",cmd: "registers",desc: "Show R0-R7", example: "registers", submenu: None },
    MenuItem { name: "stack",    cmd: "stack",    desc: "Stack depth", example: "stack", submenu: None },
];

pub static PROGRAMS_MENU: &[MenuItem] = &[
    MenuItem { name: "list",     cmd: "list",     desc: "List all programs (I-XXVIII)", example: "list", submenu: None },
    MenuItem { name: "canonical",cmd: "canonical",desc: "Load canonical program I-XII", example: "canonical VII", submenu: None },
    MenuItem { name: "continuous",cmd: "continuous",desc: "Load continuous program 1-4", example: "continuous 3", submenu: None },
    MenuItem { name: "novel",    cmd: "novel",    desc: "Load novel program 1-3", example: "novel 1", submenu: None },
    MenuItem { name: "shunt",    cmd: "shunt",    desc: "Load shunted program 1-9", example: "shunt 5", submenu: None },
    MenuItem { name: "dynamic",  cmd: "dynamic",  desc: "Dynamic mode: rebuild sequence from IgTuple each wrap", example: "dynamic on", submenu: None },
];

pub static CRYSTAL_MENU: &[MenuItem] = &[
    MenuItem { name: "decode",   cmd: "crystal",  desc: "Decode address to 12-tuple: crystal <addr>", example: "crystal 42", submenu: None },
    MenuItem { name: "store",    cmd: "crystal store", desc: "Store entry: crystal store <n> [d]", example: "crystal store my_system 42", submenu: None },
    MenuItem { name: "name",     cmd: "crystal name",  desc: "Retrieve by name: crystal name <n>", example: "crystal name sic_povm", submenu: None },
    MenuItem { name: "find",     cmd: "crystal find",  desc: "List stored entries", example: "crystal find", submenu: None },
];

pub static IMASM_MENU: &[MenuItem] = &[
    MenuItem { name: "cycle", cmd: "cycle", desc: "walk an IMASM word around its ROTAT orbit (glyphs only)", example: "cycle ⊢⊙∈⊤⊥∋⋈⊡⊣", submenu: None },
    MenuItem { name: "weight", cmd: "weight", desc: "where the weight moves through an IMASM word", example: "weight ⊢⊙∈⊤⊥∋⋈⊡⊣", submenu: None },
    MenuItem { name: "banked", cmd: "banked", desc: "was a count cleared with nothing banked?", example: "banked ⊢⊙∈⊤⊥∋⋈⊡⊣", submenu: None },
    MenuItem { name: "insert", cmd: "insert", desc: "every one-glyph repair for an exposed word", example: "insert ⊢⊙∈⊤⊥⊞∋><⋈⊡⊣", submenu: None },
    MenuItem { name: "trans", cmd: "trans", desc: "transitions counted on the ring, closing edge included", example: "trans ⊢⊙∈⊤⊥∋⋈⊡⊣", submenu: None },
    MenuItem { name: "arev", cmd: "arev", desc: "H hop: read snapshot through the R1<->R2 mirror", example: "arev", submenu: None },
];

pub static KERNEL_MENU: &[MenuItem] = &[
    MenuItem { name: "ask", cmd: "ask", desc: "kernel structural ask (dry). Full wet: host ./ask --file | -i", example: "ask What is the distance to CLINK L8?", submenu: None },
    MenuItem { name: "spine", cmd: "spine", desc: "manuscript spine: PROVE->UNIFY->PORT x vessel (no Python)", example: "spine", submenu: None },
    MenuItem { name: "vessel", cmd: "vessel", desc: "witness-vessel transport: Clay payloads x 88 dialects, frob-gated", example: "vessel", submenu: None },
    MenuItem { name: "vita", cmd: "vita", desc: "one certified turn from the on-board vae_vita trunk (needs --features vita)", example: "vita", submenu: None },
    MenuItem { name: "whoami", cmd: "whoami", desc: "IG tuple under the active ruleset", example: "whoami --ruleset", submenu: None },
    MenuItem { name: "ruleset", cmd: "ruleset", desc: "show the active ruleset", example: "ruleset", submenu: None },
    MenuItem { name: "absorption", cmd: "absorption", desc: "list all absorption rules", example: "absorption", submenu: None },
    MenuItem { name: "replicative", cmd: "replicative", desc: "load the program targeting O_inf_dag (R2) deliberately", example: "replicative", submenu: None },
    MenuItem { name: "quit", cmd: "quit", desc: "halt the kernel (aliases exit, halt)", example: "quit", submenu: None },
];

pub static QUANTUM_MENU: &[MenuItem] = &[
    MenuItem { name: "fibqc",      cmd: "fibqc",      desc: "Fibonacci anyon QC: verify | compile | jones | knot | winding (see also qc, jp)", example: "fibqc verify", submenu: None },
    MenuItem { name: "qc", cmd: "qc", desc: "Compile a circuit over H T S X to a braid word; spaces optional; draw|svg|loop before the gates renders it, and two depths size the net and the recursion (aliases quantum_compile, fibqc compile)", example: "qc loop HTSX 10 3", submenu: None },
    MenuItem { name: "bi", cmd: "bi", desc: "Draw a braid word — strand diagram in the terminal, SVG with `svg`, the closed braid as a ring with `loop`; window with start:count, column height with /N", example: "bi loop 1 2 -1 -2 1 2", submenu: None },
    MenuItem { name: "jp", cmd: "jp", desc: "Jones polynomial at the 1/5 winding; signed Artin generators (alias jones_polynomial)", example: "jp 1 1 1", submenu: None },
    MenuItem { name: "bg",         cmd: "bg",         desc: "Braid word to grammar tuple (alias braid-grammar); winding is a closed form in the writhe", example: "bg tuple 1,2,1 3", submenu: None },
    MenuItem { name: "shor",       cmd: "shor",       desc: "Belnap Shor pipeline, N=15 and N=21", example: "shor", submenu: None },
    MenuItem { name: "iuft",       cmd: "iuft",       desc: "IUFT QC gates — the 12->3 Euler-angle SU(2) encoding of an IG tuple", example: "iuft list", submenu: None },
    MenuItem { name: "teich",      cmd: "teich",      desc: "IUFT <-> IUTT bridge: Teichmuller deformation paths as gate trajectories", example: "teich canonical", submenu: None },
    MenuItem { name: "hqe",        cmd: "hqe",        desc: "Holonomic quasi-ergodic quantale, MBL holonomy", example: "hqe report", submenu: None },
    MenuItem { name: "dyson",      cmd: "dyson",      desc: "Dyson beta-ensemble, double-ramified cycle", example: "dyson report", submenu: None },
    MenuItem { name: "troq",       cmd: "troq",       desc: "Triple-ramified ouroboric quantale", example: "troq report", submenu: None },
    MenuItem { name: "afdmc",      cmd: "afdmc",      desc: "Asymptotic frozen-disordered monadic cohomology", example: "afdmc report", submenu: None },
    MenuItem { name: "hop",        cmd: "hop",        desc: "Universe hopping, cross-framework transport", example: "hop report", submenu: None },
    MenuItem { name: "manifold",   cmd: "manifold",   desc: "Topological manifold operations", example: "manifold", submenu: None },
    MenuItem { name: "triple",     cmd: "triple",     desc: "Triple-frame von Neumann superoperator algebra", example: "triple report", submenu: None },
    MenuItem { name: "sic",        cmd: "sic",        desc: "SIC-POVM d=12 identity, three lattice proofs", example: "sic", submenu: None },
    MenuItem { name: "d12",        cmd: "d12",        desc: "d=12 SIC Phase VI: tower, magnitudes, orbits, existence, duallink, z0", example: "d12 tower", submenu: None },
    MenuItem { name: "d2048",      cmd: "d2048",      desc: "d=2048 moduli tower ascent (alias d2k)", example: "d2048 next", submenu: None },
];

pub static GRAMMAR_MENU: &[MenuItem] = &[
    MenuItem { name: "ig",       cmd: "ig",       desc: "IG tuple + crystal address", example: "ig", submenu: None },
    MenuItem { name: "classify", cmd: "classify", desc: "Nearest-catalog classification", example: "classify", submenu: None },
    MenuItem { name: "frob",     cmd: "frob",     desc: "Frobenius harness status", example: "frob", submenu: None },
    MenuItem { name: "aleph",    cmd: "aleph",    desc: "Hebrew glyph encoding: aleph <word>", example: "aleph שלום", submenu: None },
    MenuItem { name: "rh",       cmd: "rh",       desc: "Riemann Hypothesis bridge", example: "rh", submenu: None },
    MenuItem { name: "ym",       cmd: "ym",       desc: "Yang-Mills mass gap bridge", example: "ym", submenu: None },
    MenuItem { name: "temp",     cmd: "temp",     desc: "Temporal logic bridge", example: "temp", submenu: None },
    MenuItem { name: "cat",      cmd: "cat",      desc: "Category theory bridge", example: "cat", submenu: None },
    MenuItem { name: "algebra",  cmd: "algebra",  desc: "distance|meet|join|tensor vs ZFC", example: "algebra distance", submenu: None },
    MenuItem { name: "cl8nk",    cmd: "cl8nk",    desc: "CLINK Layer 8: cl8nk <action> [name]", example: "cl8nk entry sic_povm", submenu: None },
    MenuItem { name: "c4",       cmd: "c4",       desc: "Belnap C₄ complex plane (i²=B)", example: "c4", submenu: None },
    MenuItem { name: "cscore",   cmd: "cscore",   desc: "Consciousness score (dual-gate)", example: "cscore", submenu: None },
    MenuItem { name: "constants", cmd: "constants", desc: "MoDoT constant closure: fine-structure, proton-electron, lepton, boson, gravity", example: "constants", submenu: None },
    MenuItem { name: "ovm", cmd: "ovm", desc: "OVM Computation Tools", example: "ovm list", submenu: None },
    MenuItem { name: "oneshots", cmd: "oneshots", desc: "the 10 exotic fixed-point nestings: inner already at outer's fixed point", example: "oneshots", submenu: None },
    MenuItem { name: "ctc",      cmd: "ctc",      desc: "manufactured fixed points: closure imposed where the action has none, priced by the width it smears", example: "ctc", submenu: None },
    MenuItem { name: "nesting",  cmd: "nesting",  desc: "the two-step observable: q=r2/r1 splits attracted from never-arrives where the residual alone cannot", example: "nesting", submenu: None },
    MenuItem { name: "carriers", cmd: "carriers", desc: "census of the mu-delta=id carriers by class: one fixed point seen many ways, or a family", example: "carriers", submenu: None },
    MenuItem { name: "substrate", cmd: "substrate", desc: "closure constant, content bifurcating: the conservative substrate read on both observables", example: "substrate", submenu: None },
    MenuItem { name: "stark", cmd: "stark", desc: "Stark unit extraction: formula,fibqc,tower,exponents,verify", example: "stark formula 2048", submenu: None },
    MenuItem { name: "riemann", cmd: "riemann", desc: "Riemann-SIC report; sub-actions available", example: "riemann", submenu: None },
    MenuItem { name: "distance", cmd: "distance", desc: "Hamming + weighted distance vs the ZFC baseline tuple (alias dist)", example: "distance", submenu: None },
    MenuItem { name: "join", cmd: "join", desc: "join of the active IG tuple with the ZFC baseline", example: "join", submenu: None },
    MenuItem { name: "sigma", cmd: "sigma", desc: "sigma <n> — analyze the Sigma(n) divisor ring", example: "sigma 5", submenu: None },
    MenuItem { name: "clay", cmd: "clay", desc: "Clay Millennium structural status (machine-checked)", example: "clay", submenu: None },
    MenuItem { name: "psm", cmd: "psm", desc: "dialetheic alignment + measurement tests", example: "psm test", submenu: None },
    MenuItem { name: "entropy", cmd: "entropy", desc: "entropy experiment: dS vs tier promotion", example: "entropy tier", submenu: None },
    MenuItem { name: "mersearch", cmd: "mersearch", desc: "Mersenne search: run|ll. Composite exponents answer at once (alias msearch)", example: "msearch ll 2213", submenu: None },
];

pub static REBIS_MENU: &[MenuItem] = &[
    MenuItem { name: "codon",    cmd: "rebis codon",    desc: "Codon ↔ AA bidirectional", example: "rebis codon AUG", submenu: None },
    MenuItem { name: "translate",cmd: "rebis translate",desc: "Gene → protein pipeline", example: "rebis translate ATG...", submenu: None },
    MenuItem { name: "reverse",  cmd: "rebis reverse",  desc: "Protein → mRNA → DNA", example: "rebis reverse MKY...", submenu: None },
    MenuItem { name: "frob",     cmd: "rebis frob",     desc: "Frobenius filtration (64 codons)", example: "rebis frob", submenu: None },
    MenuItem { name: "genetics", cmd: "rebis genetics", desc: "7-stage genetic code verification", example: "rebis genetics", submenu: None },
    MenuItem { name: "hadron",   cmd: "rebis hadron",   desc: "Belnap hadron analysis", example: "rebis hadron", submenu: None },
    MenuItem { name: "serpent",  cmd: "rebis serpent",  desc: "Serpent rod motif analysis", example: "rebis serpent", submenu: None },
    MenuItem { name: "pipeline", cmd: "rebis pipeline", desc: "IG promotion pipeline", example: "rebis pipeline", submenu: None },
    MenuItem { name: "strata",   cmd: "rebis strata",   desc: "Codon stratum counts", example: "rebis strata", submenu: None },
    MenuItem { name: "asm",      cmd: "rebis asm",      desc: "Genetic ParaASM programs", example: "rebis asm", submenu: None },
    MenuItem { name: "tuples",   cmd: "rebis tuples",   desc: "7-stage generative tuple pipeline", example: "rebis tuples", submenu: None },
    MenuItem { name: "clu",      cmd: "rebis clu",      desc: "CLU power-law clustering", example: "rebis clu", submenu: None },
    MenuItem { name: "exotic",   cmd: "rebis exotic",   desc: "Exotic hadron Frobenius verification", example: "rebis exotic", submenu: None },
    MenuItem { name: "pdb",      cmd: "rebis pdb",      desc: "PDB structure validation", example: "rebis pdb 1CRN", submenu: None },
    MenuItem { name: "antibody", cmd: "rebis antibody", desc: "Antibody CDR design", example: "rebis antibody", submenu: None },
    MenuItem { name: "material", cmd: "rebis material", desc: "IG material forge & metamaterials", example: "rebis material", submenu: None },
    MenuItem { name: "sidechain",cmd: "rebis sidechain",desc: "AA sidechain × environment algebra (20×4)", example: "rebis sidechain", submenu: None },
    MenuItem { name: "ligand",   cmd: "rebis ligand",   desc: "Ligand design from catalytic sites", example: "rebis ligand", submenu: None },
    MenuItem { name: "decay",    cmd: "rebis decay",    desc: "Nuclear decay as IMASM winding", example: "rebis decay", submenu: None },
    MenuItem { name: "bio",      cmd: "rebis bio",      desc: "Biological computation", example: "rebis bio", submenu: None },
    MenuItem { name: "tx",       cmd: "rebis tx",       desc: "Therapeutics (chemo, pill, antidote)", example: "rebis tx", submenu: None },
];

pub static DIALECT_MENU: &[MenuItem] = &[
    MenuItem { name: "show",     cmd: "ruleset show",    desc: "Active ruleset display", example: "ruleset show", submenu: None },
    MenuItem { name: "list",     cmd: "ruleset list",    desc: "List all 88 dialects", example: "ruleset list", submenu: None },
    MenuItem { name: "verify",   cmd: "ruleset verify",  desc: "Invariant violation check", example: "ruleset verify", submenu: None },
    MenuItem { name: "jump",     cmd: "jump",            desc: "Cross-dialect jump: jump <U> using <c>", example: "jump 42 using clay", submenu: None },
    MenuItem { name: "seal",     cmd: "seal",            desc: "IFIX commit to current ruleset", example: "seal", submenu: None },
    MenuItem { name: "whoami",   cmd: "whoami --ruleset",desc: "IG tuple under active ruleset", example: "whoami --ruleset", submenu: None },
    MenuItem { name: "tensor",   cmd: "tensor",          desc: "Tensor under active absorption", example: "tensor", submenu: None },
    MenuItem { name: "meet",     cmd: "meet",            desc: "Meet under active absorption", example: "meet", submenu: None },
    MenuItem { name: "absorb",   cmd: "absorb_test",     desc: "Test absorption rule", example: "absorb_test", submenu: None },
    MenuItem { name: "abs-show", cmd: "absorption show", desc: "List absorption rules", example: "absorption show", submenu: None },
    MenuItem { name: "tstatus",  cmd: "tstatus",         desc: "T-constitution pass/fail", example: "tstatus", submenu: None },
    MenuItem { name: "compounds",cmd: "compound list",   desc: "List 11 diaschizic compounds", example: "compound list", submenu: None },
    MenuItem { name: "compound", cmd: "compound",        desc: "compound show|load <name>", example: "compound show I", submenu: None },
];

pub static PARASM_MENU: &[MenuItem] = &[
    MenuItem { name: "test",     cmd: "psm test",   desc: "Dialetheic alignment + measurement", example: "psm test", submenu: None },
    MenuItem { name: "frob",     cmd: "psm frob",   desc: "Frobenius identity cycle", example: "psm frob", submenu: None },
    MenuItem { name: "kernel",   cmd: "psm kernel", desc: "Kernel-state B3 invariant loop", example: "psm kernel 5", submenu: None },
    MenuItem { name: "load",     cmd: "psm load",   desc: "Inline ParaASM program (; separator)", example: "psm load ENGAGR %r0; FSPLIT %r0 %r1 %r2; FFUSE %r1 %r2 %r0; HALT", submenu: None },
];

pub static CR3ECHRZ_MENU: &[MenuItem] = &[
    MenuItem { name: "cr3",      cmd: "cr3",       desc: "Theorem engine (Collatz, Goldbach, Three-Body, Burnside, ...)", example: "cr3", submenu: None },
    MenuItem { name: "p4ra",     cmd: "p4ra",      desc: "p4rakernel Belnap+Frobenius 13-step bootstrap", example: "p4ra", submenu: None },
    MenuItem { name: "version",  cmd: "cr3 --version", desc: "cr3 version info", example: "cr3 --version", submenu: None },
    MenuItem { name: "list",     cmd: "cr3 --list", desc: "List theorems + p4rakernel modules", example: "cr3 --list", submenu: None },
];


// ═══════════════════════════════════════════════════════════════
// CONTEXT-STACK AND MENU-UI STUBS
//
// These provide the menu-UI surface that repl.rs imports.
// Previously extracted from repl.rs during refactoring; the
// full interactive menu bar / context stack was moved into its
// own module but the stubs were never landed. These minimal
// implementations satisfy the repl.rs call-sites.
// ═══════════════════════════════════════════════════════════════

/// A simple named context stack for tab-completion and prompt rendering.
/// Each entry has a name; `current()` returns the top.
pub struct ContextStack {
    pub name: &'static str,
    pub depth: usize,
    stack: [&'static str; 8],
}

impl ContextStack {
    pub fn new() -> Self {
        ContextStack { name: "root", depth: 0, stack: ["root"; 8] }
    }

    /// Return the current (topmost) context, if any.
    pub fn current(&self) -> Option<&Self> {
        if self.depth > 0 {
            // Return a reference to self — the name is set to the current top
            Some(self)
        } else {
            Some(self)  // root context always available
        }
    }

    /// Pop the current context. Returns the name that was popped.
    pub fn pop(&mut self) -> &'static str {
        if self.depth > 0 {
            self.depth -= 1;
            self.name = self.stack[self.depth]
        }
        self.name
    }
}

/// Render the REPL prompt, showing the current context.
pub fn render_prompt(ctx: &ContextStack) {
    use crate::serial;
    serial::write_str("⊙> ");
    let _ = ctx;
}

/// Render the top-level menu bar (F-key shortcuts).
pub fn render_menu_bar() {
    // Minimal stub — full menu bar rendering is historical.
}

/// Show the menu hint line below the prompt.
pub fn menu_hint() {
    // Minimal stub.
}

/// Tab-complete the current line against the context stack.
/// Returns a completion string if exactly one match exists.
pub fn tab_complete(_line: &str, _ctx: &ContextStack) -> Option<&'static str> {
    None
}

/// Enter a named context, pushing it onto the stack.
/// Returns true if the context was entered successfully.
pub fn enter_context(ctx: &mut ContextStack, name: &str) -> bool {
    let _ = name;
    let _ = ctx;
    false
}

/// Map an F-key number (1-12) to a menu category name.
pub fn fkey_to_category(_n: u8) -> Option<&'static str> {
    None
}

/// Print the help topic for a given keyword.
/// Called by the `help` command (repl.rs line 184).
pub fn print_help_topic(topic: &str) {
    let t = topic.trim().to_lowercase();

    if t.is_empty() {
        // ── General help — list all top-level commands ──
        use crate::style as S;
        head!("mOMonadOS — help");
        crate::sprintln!("  {}help <topic>{} for any one command in detail.",
            S::key(), S::reset());
        crate::sprintln!("");
        for item in MAIN_MENU.iter() {
            crate::sprintln!("  {}{:12}{} {}{}{}",
                S::key(), item.name, S::reset(), S::muted(), item.desc, S::reset());
        }
        for cat in MAIN_MENU.iter() {
            if let Some(sub) = cat.submenu {
                divider!();
                crate::sprintln!("  {}{}{}", S::heading(), cat.name, S::reset());
                for si in sub.iter() {
                    crate::sprintln!("    {}{:22}{} {}", S::accent(), si.cmd, S::reset(), si.desc);
                    if !si.example.is_empty() {
                        // The example is the part a new reader actually copies,
                        // so it is dimmed but never dropped.
                        crate::sprintln!("    {:22} {}e.g. {}{}", "", S::muted(), si.example, S::reset());
                    }
                }
            }
        }
        foot!();
        return;
    }

    // ── Topic-specific help ──
    match t.as_str() {
        "ovm" => crate::sprintln!("{}", crate::ovm::ovm_help()),
        "hqe" => {
            crate::sprintln!("═══ HQE — Hadron-Quark-Electron Formal Homology ═══\n");
            crate::sprintln!("hqe              — full HQE report");
            crate::sprintln!("hqe report       — same as hqe");
            crate::sprintln!("hqe distance [t2] — tuple distance vs AFDMC (default)");
            crate::sprintln!("hqe cscore       — consciousness score");
            crate::sprintln!("hqe meet [t2]    — quantale meet");
            crate::sprintln!("hqe join [t2]    — quantale join");
            crate::sprintln!("hqe tuple        — display HQE tuple");
        }
        "dyson" => {
            crate::sprintln!("═══ Dyson RD/A Formal Decomposition ═══\n");
            crate::sprintln!("dyson            — full Dyson report");
            crate::sprintln!("dyson report     — same as dyson");
            crate::sprintln!("dyson tuple      — display Dyson tuple");
        }
        "afdmc" => {
            crate::sprintln!("═══ AFDMC Nuclear Many-Body Theory ═══\n");
            crate::sprintln!("afdmc            — full AFDMC report");
            crate::sprintln!("afdmc report     — same as afdmc");
            crate::sprintln!("afdmc tuple      — display AFDMC tuple");
            crate::sprintln!("afdmc hqe        — HQE report");
            crate::sprintln!("afdmc dyson      — Dyson report");
        }
        "triple" | "triple-frame" => {
            crate::sprintln!("═══ Triple-Frame ═══\n");
            crate::sprintln!("triple           — triple-frame overview");
            crate::sprintln!("triple report    — full report");
            crate::sprintln!("triple tuple     — display triple-frame tuple");
            crate::sprintln!("triple check [w] — W-bootstrap check (default W=3,7,12)");
            crate::sprintln!("triple help      — this help");
        }
        "proof" => {
            crate::sprintln!("═══ Guided Proofs ═══\n");
            crate::sprintln!("proof            — list available proofs");
            crate::sprintln!("proof list       — same as proof");
            crate::sprintln!("proof bootstrap  — Grammar self-verification (7 steps)");
            crate::sprintln!("proof parity     — Parity walk");
        }
        "cycle" | "weight" | "banked" | "insert" | "trans" => {
            crate::sprintln!("═══ IMASM Word Walks ═══\n");
            crate::sprintln!("{} <word>   — run the {} walk on an IMASM word", t, t);
            crate::sprintln!("Without <word>, shows usage and explanation.");
        }
        "status" => {
            crate::sprintln!("═══ Kernel Status ═══\n");
            crate::sprintln!("status           — tick, IP, stack, fork, frob snapshot");
        }
        "ruleset" => {
            crate::sprintln!("═══ Ruleset / Dialect Management ═══\n");
            crate::sprintln!("ruleset show     — active ruleset display");
            crate::sprintln!("ruleset list     — list all 88 dialects");
            crate::sprintln!("ruleset verify   — invariant violation check");
        }
        "jump" => {
            crate::sprintln!("═══ Cross-Dialect Jump ═══\n");
            crate::sprintln!("jump <U> using <c> — jump to universe U using catalyst c");
        }
        "seal" => {
            crate::sprintln!("═══ IFIX Seal ═══\n");
            crate::sprintln!("seal             — IFIX commit to current ruleset");
        }
        "whoami" => {
            crate::sprintln!("═══ Who Am I ═══\n");
            crate::sprintln!("whoami           — active IG tuple");
            crate::sprintln!("whoami --ruleset — IG tuple under active ruleset");
        }
        "compound" => {
            crate::sprintln!("═══ Diaschizic Compounds ═══\n");
            crate::sprintln!("compound list    — list all 11 compounds");
            crate::sprintln!("compound show <n> — show compound details");
            crate::sprintln!("compound load <n> — load compound");
        }
        "sigma" => {
            crate::sprintln!("═══ Sigma(n) Divisor Ring ═══\n");
            crate::sprintln!("sigma <n>        — analyze Sigma(n) divisor ring");
        }
        "clay" => {
            crate::sprintln!("═══ Clay Millennium Problems ═══\n");
            crate::sprintln!("clay             — structural status (machine-checked)");
        }
        "entropy" => {
            crate::sprintln!("═══ Entropy Experiment ═══\n");
            crate::sprintln!("entropy tier     — dS vs tier promotion");
        }
        "kernel" => {
            crate::sprintln!("═══ Kernel Utilities ═══\n");
            crate::sprintln!("kernel ask <q>   — query the kernel");
            crate::sprintln!("kernel spine     — spine report");
            crate::sprintln!("kernel vessel    — vessel status");
            crate::sprintln!("kernel vita      — vita status");
            crate::sprintln!("kernel whoami    — identity report");
            crate::sprintln!("kernel ruleset   — ruleset management");
        }
        "exec" | "tick" | "run" | "watch" | "timer" | "boot" | "load" => {
            crate::sprintln!("═══ Execution ═══\n");
            crate::sprintln!("tick [n]         — run n manual ticks (default 1)");
            crate::sprintln!("run [n]          — run n ticks; no arg = continuous");
            crate::sprintln!("watch            — live terminal HUD");
            crate::sprintln!("timer [n]        — n ticks, one per PIT interrupt");
            crate::sprintln!("boot <N>         — load + run program I-XXVIII");
            crate::sprintln!("load <N>         — load program by Roman numeral");
        }
        "rebis" | "codon" | "translate" | "genetics" | "material" | "bio" | "tx" => {
            crate::sprintln!("═══ Red-Hot Rebis ═══\n");
            crate::sprintln!("rebis codon <X>  — codon ↔ AA");
            crate::sprintln!("rebis translate  — gene → protein");
            crate::sprintln!("rebis reverse    — protein → mRNA → DNA");
            crate::sprintln!("rebis frob       — Frobenius filtration (64 codons)");
            crate::sprintln!("rebis genetics   — 7-stage genetic code verification");
            crate::sprintln!("rebis hadron     — Belnap hadron analysis");
            crate::sprintln!("rebis material   — IG material forge");
            crate::sprintln!("rebis bio        — biological computation");
            crate::sprintln!("rebis tx         — therapeutics");
            crate::sprintln!("\nType 'help rebis' for the full 21-command Rebis menu.");
        }
        "grammar" | "classify" | "frob" | "aleph" | "shor" | "ym" | "rh" | "riemann" => {
            crate::sprintln!("═══ Grammar Bridges ═══\n");
            crate::sprintln!("ig               — active IG tuple + distance to ZFC baseline");
            crate::sprintln!("classify <t>     — classify a 12-glyph tuple");
            crate::sprintln!("frob <t>         — Frobenius check");
            crate::sprintln!("aleph <t>        — Aleph-encode a tuple");
            crate::sprintln!("shor             — Shor 1994 → IG");
            crate::sprintln!("ym               — Yang-Mills mass gap → IG");
            crate::sprintln!("rh               — Riemann Hypothesis bridge");
        }
        _ => {
            // A category name expands to its commands, each with its example.
            for cat in MAIN_MENU.iter() {
                if cat.cmd == t || cat.name.to_lowercase() == t {
                    if let Some(sub) = cat.submenu {
                        crate::sprintln!("═══ {} ═══", cat.name);
                        crate::sprintln!("{}\n", cat.desc);
                        for si in sub.iter() {
                            crate::sprintln!("  {:22} {}", si.cmd, si.desc);
                            if !si.example.is_empty() {
                                crate::sprintln!("  {:22}   e.g.  {}", "", si.example);
                            }
                        }
                        return;
                    }
                }
            }
            // Before searching, see whether the menu already knows this command.
            // Every entry carries an example and nothing was printing it.
            let mut found = false;
            for cat in MAIN_MENU.iter() {
                if let Some(sub) = cat.submenu {
                    for si in sub.iter() {
                        if si.cmd == t || si.name.to_lowercase() == t {
                            crate::sprintln!("═══ {} ═══\n", si.name);
                            crate::sprintln!("{}", si.desc);
                            if !si.example.is_empty() {
                                crate::sprintln!("\n  e.g.  {}", si.example);
                            }
                            crate::sprintln!("\n(in the {} category — `help {}` for the rest)",
                                             cat.name, cat.cmd);
                            found = true;
                        }
                    }
                }
            }
            if !found {
                search_commands(topic);
            }
        }
    }
}

/// Search commands by keyword and print matches.
pub fn search_commands(keyword: &str) {
    if keyword.is_empty() {
        crate::sprintln!("Usage: help <topic>");
        crate::sprintln!("Type 'help' with no argument for full command list.");
        return;
    }
    let kw = keyword.to_lowercase();
    crate::sprintln!("Searching for '{}'...\n", keyword);

    // Search top-level menu
    for item in MAIN_MENU.iter() {
        if item.name.to_lowercase().contains(&kw) || item.desc.to_lowercase().contains(&kw) {
            crate::sprintln!("  {:12} — {}", item.name, item.desc);
        }
    }

    // Also match known sub-commands
    let known: &[(&str, &str)] = &[
        ("ovm", "OVM Computation Tools (eigen, frame, overlap, belnap, help)"),
        ("hqe", "Hadron-Quark-Electron formal homology"),
        ("dyson", "Dyson RD/A formal decomposition"),
        ("afdmc", "AFDMC nuclear many-body theory"),
        ("triple", "Triple-frame (SIC-POVM/Navier-Stokes/Yang-Mills)"),
        ("cycle", "IMASM word cycle walk"),
        ("weight", "IMASM word weight trace"),
        ("banked", "IMASM banked-count report"),
        ("insert", "IMASM one-glyph repair sweep"),
        ("trans", "IMASM transition counter"),
        ("proof", "Guided proof walker"),
        ("status", "Kernel status snapshot"),
        ("ruleset", "Dialect ruleset management"),
        ("compound", "Diaschizic compound management"),
        ("sigma", "Sigma(n) divisor ring"),
        ("clay", "Clay Millennium structural status"),
        ("entropy", "dS vs tier promotion"),
        ("rebis", "Red-Hot Rebis (21 commands: codon, translate, genetics, materials, bio, tx)"),
        ("jump", "Cross-dialect jump"),
        ("seal", "IFIX commit"),
        ("whoami", "Active IG tuple"),
        ("tensor", "Tensor under absorption"),
        ("meet", "Meet under absorption"),
    ];

    let mut found = false;
    for (name, desc) in known {
        if name.contains(&kw) || desc.to_lowercase().contains(&kw) {
            if !found {
                crate::sprintln!("\nADDITIONAL COMMANDS:");
                found = true;
            }
            crate::sprintln!("  {:12} — {}", name, desc);
        }
    }

    if !found {
        crate::sprintln!("No commands found matching '{}'.", keyword);
        crate::sprintln!("Type 'help' with no argument for the full list.");
    }
}
