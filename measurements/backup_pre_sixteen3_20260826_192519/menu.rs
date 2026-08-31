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
    MenuItem { name: "Quantum",  cmd: "quantum",  desc: "Quantum computation (fibqc, jones, braids, shor, shors_btc_2, btc_oneshot, qft, iuft, sic, d12, d2048)", example: "help quantum", submenu: Some(QUANTUM_MENU) },
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
    MenuItem { name: "fold",           cmd: "fold",                 desc: "The fold verdict of a word — closed form, surplus, the enclosure witness, the codon lane", example: "fold", submenu: None },
    MenuItem { name: "erdos",          cmd: "erdos",                desc: "Guided walks through the Erdős manuscripts — list | schutte | landau | lcm", example: "erdos schutte", submenu: None },
];

pub static PROOF_MENU: &[MenuItem] = &[
    MenuItem { name: "list",      cmd: "proof list",      desc: "List available guided proofs", example: "proof list", submenu: None },
    MenuItem { name: "bootstrap", cmd: "proof bootstrap", desc: "The Grammar verifying itself (7 steps, auto-play)", example: "proof bootstrap", submenu: None },
    MenuItem { name: "prooflift", cmd: "prooflift", desc: "Proof-lift report: undischarged claims and unrejoined forks as one object; `prooflift nest` runs the self-nest word, the proof of mu.delta=id itself (86065 glyphs, verdict T)", example: "prooflift nest", submenu: None },
];

pub static EXEC_MENU: &[MenuItem] = &[
    MenuItem { name: "tick",     cmd: "tick",     desc: "Run N manual ticks (default 1)", example: "tick 10", submenu: None },
    MenuItem { name: "run",      cmd: "run",      desc: "Run N ticks; no arg = continuous (ESC to stop)", example: "run 100", submenu: None },
    MenuItem { name: "watch",    cmd: "watch",    desc: "Live terminal HUD (ESC to stop)", example: "watch", submenu: None },
    MenuItem { name: "timer",    cmd: "timer",    desc: "Run N ticks, one per PIT interrupt", example: "timer 50", submenu: None },
    MenuItem { name: "boot",     cmd: "boot",     desc: "Load + run any program (I-XXVIII or decimal)", example: "boot VII", submenu: None },
    MenuItem { name: "load",     cmd: "load",     desc: "Load program by Roman numeral", example: "load XII", submenu: None },
];

// ─── Fixed-point instruments ──────────────────────────────
// Each names its own forms. `ctc` and `nesting` take a pairing and answer about
// that pairing; bare, they sweep. `carriers` and `substrate` read kernel state
// and take nothing, so their submenus say what they read rather than inventing
// arguments they do not have.

pub static CTC_MENU: &[MenuItem] = &[
    MenuItem { name: "sweep",    cmd: "ctc",          desc: "every value in every action, with the price each closure cost", example: "ctc", submenu: None },
    MenuItem { name: "not",      cmd: "ctc not",      desc: "Belnap negation — fixed at Neither and Both, a 2-cycle between True and False", example: "ctc not T", submenu: None },
    MenuItem { name: "next",     cmd: "ctc next",     desc: "temporal step — True and False swap, Neither and Both hold", example: "ctc next N", submenu: None },
    MenuItem { name: "collapse", cmd: "ctc collapse", desc: "everything to Both in one step — one fixed point, whole space in its basin", example: "ctc collapse T", submenu: None },
    MenuItem { name: "cycle",    cmd: "ctc cycle",    desc: "T→F→N→B→T — no fixed point at all, so closure must be manufactured", example: "ctc cycle T", submenu: None },
    MenuItem { name: "meet",     cmd: "ctc meet",     desc: "lattice meet against Both", example: "ctc meet T", submenu: None },
    MenuItem { name: "join",     cmd: "ctc join",     desc: "lattice join against Both", example: "ctc join F", submenu: None },
];

pub static NESTING_MENU: &[MenuItem] = &[
    MenuItem { name: "sweep",    cmd: "nesting",         desc: "the reference pairings, each predicted then run", example: "nesting", submenu: None },
    MenuItem { name: "halve",    cmd: "nesting halve",   desc: "halve the distance to 3 — settles from anywhere, q = 0.5", example: "nesting halve 203", submenu: None },
    MenuItem { name: "newton",   cmd: "nesting newton",  desc: "Newton on x³−2x−5 — settles fast in range, q well below 1", example: "nesting newton 2", submenu: None },
    MenuItem { name: "shift",    cmd: "nesting shift",   desc: "add one forever — never settles, and its gap never changes", example: "nesting shift 0", submenu: None },
    MenuItem { name: "rotate",   cmd: "nesting rotate",  desc: "turn a third of a circle — a closed orbit, needs x and y", example: "nesting rotate 1 0", submenu: None },
    MenuItem { name: "project",  cmd: "nesting project", desc: "flatten onto the first axis — settles in one step, needs x and y", example: "nesting project 1 5", submenu: None },
];

pub static CARRIERS_MENU: &[MenuItem] = &[
    MenuItem { name: "census",   cmd: "carriers", desc: "reads the catalog: every entry meeting the closure condition, the distance between each pair, and the classes they fall into", example: "carriers", submenu: None },
];

pub static SUBSTRATE_MENU: &[MenuItem] = &[
    MenuItem { name: "sweep",    cmd: "substrate", desc: "reads the sequence builder: return time and behaviour across the weight range, plus where a critical weight can exist at all", example: "substrate", submenu: None },
];

pub static STATUS_MENU: &[MenuItem] = &[
    MenuItem { name: "status",   cmd: "status",   desc: "Kernel status (tick, IP, stack, fork, frob)", example: "status", submenu: None },
    MenuItem { name: "program",  cmd: "program",  desc: "Show loaded program + fork depth", example: "program", submenu: None },
    MenuItem { name: "snapshot", cmd: "snapshot", desc: "Structural snapshot (sig, tier, period)", example: "snapshot", submenu: None },
    MenuItem { name: "graph",    cmd: "graph",    desc: "ASCII-art token graph with nesting", example: "graph", submenu: None },
    MenuItem { name: "heatmap",  cmd: "heatmap",  desc: "B4 memory heatmap", example: "heatmap", submenu: None },
    MenuItem { name: "memory",   cmd: "memory",   desc: "Dump B4 memory", example: "memory", submenu: None },
    MenuItem { name: "registers",cmd: "registers",desc: "Show R0-R7", example: "registers", submenu: None },
    MenuItem { name: "color",      cmd: "color",      desc: "Toggle terminal colour (alias colour)", example: "color on", submenu: None },
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
    MenuItem { name: "vita", cmd: "vita", desc: "one certified turn from the on-board vae_vita trunk", example: "vita", submenu: None },
    MenuItem { name: "whoami", cmd: "whoami", desc: "IG tuple under the active ruleset", example: "whoami --ruleset", submenu: None },
    MenuItem { name: "ruleset", cmd: "ruleset", desc: "show the active ruleset", example: "ruleset", submenu: None },
    MenuItem { name: "absorption", cmd: "absorption", desc: "list all absorption rules", example: "absorption", submenu: None },
    MenuItem { name: "replicative", cmd: "replicative", desc: "load the program targeting O_inf_dag (R2) deliberately", example: "replicative", submenu: None },
    MenuItem { name: "vox",        cmd: "vox",        desc: "Control-flow closure auditor: verdict <word> | evm <hex> | wasm <hex> | classify <mn> | run <file> [--argv a,b] — runs the file as a real process from its own entry, real argv/envp/auxv stack, real read/write/open/openat/close/mmap/brk syscalls. run <sym> <file> [--args a,b] calls one function directly instead: scalar int args, one int back, no process. Static binaries on direct syscalls run for real; dynamic linking and glibc's own TLS setup are further rungs, not yet built", example: "vox run ./mybinary --argv hello", submenu: None },
    MenuItem { name: "quit", cmd: "quit", desc: "halt the kernel (aliases exit, halt)", example: "quit", submenu: None },
];

pub static QUANTUM_MENU: &[MenuItem] = &[
    MenuItem { name: "fibqc",      cmd: "fibqc",      desc: "Fibonacci anyon QC: verify | compile | jones | knot | winding (see also qc, jp)", example: "fibqc verify", submenu: None },
    MenuItem { name: "qc", cmd: "qc", desc: "Compile a circuit over H T S X to a braid word; spaces optional; draw|svg|loop before the gates renders it, and two depths size the net and the recursion (aliases quantum_compile, fibqc compile)", example: "qc loop HTSX 10 3", submenu: None },
    MenuItem { name: "bi", cmd: "bi", desc: "Draw a braid word — strand diagram in the terminal, SVG with `svg`, the closed braid as a ring with `loop`; window with start:count, column height with /N (alias braid_image)", example: "bi loop 1 2 -1 -2 1 2", submenu: None },
    MenuItem { name: "jp", cmd: "jp", desc: "Jones polynomial at the 1/5 winding; signed Artin generators (alias jones_polynomial)", example: "jp 1 1 1", submenu: None },
    MenuItem { name: "bg",         cmd: "bg",         desc: "Braid word to grammar tuple (alias braid-grammar); winding is a closed form in the writhe", example: "bg tuple 1,2,1 3", submenu: None },
    MenuItem { name: "shor",       cmd: "shor",       desc: "Belnap Shor pipeline + dialetheic Fibonacci Shor (word ⊢∈≻⋈⊞∈⊤≻⊥≺∋⊙⋈⊡⊣); N=15,21", example: "shor dialetheic 15 7", submenu: None },
    MenuItem { name: "shors_btc_2", cmd: "shors_btc_2", desc: "Shor over secp256k1 ECDLP: recover a Bitcoin private key from a public key (x,y)", example: "shors_btc_2", submenu: None },
    MenuItem { name: "qft",        cmd: "qft",        desc: "Quantum Fourier Transform: circuit | phases | iqft | iqft braid | braid, on n qubits", example: "qft circuit 3", submenu: None },
    MenuItem { name: "btc_oneshot",  cmd: "btc_oneshot",  desc: "BTC Secret Key Oneshot Operator — structural verification & phase steps", example: "btc_oneshot verify", submenu: None },
    MenuItem { name: "winding",    cmd: "winding",    desc: "Period as a torus winding: order | factor | closure | factorgen (alias wperiod)", example: "winding order 2 101", submenu: None },
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
    MenuItem { name: "bip39",      cmd: "bip39",      desc: "BIP39-SIC-POVM: search | words | verify | map | gap", example: "bip39 sic verify", submenu: None },
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
    MenuItem { name: "ctc",      cmd: "ctc",      desc: "nest a value in an action; closure imposed where the action has none, priced by the width it smears", example: "ctc cycle T", submenu: Some(CTC_MENU) },
    MenuItem { name: "collatz",   cmd: "collatz",   desc: "the Collatz block nesting: blocks to one, the budget spectrum, and the records", example: "collatz 27", submenu: None },
    MenuItem { name: "straus",   cmd: "straus",   desc: "the Erdős–Straus ladder: which rung r closes 4/n, and the spectrum across a range", example: "straus 49", submenu: None },
    MenuItem { name: "nesting",  cmd: "nesting",  desc: "read a point against a map: q=r2/r1 splits attracted from never-arrives where one gap cannot", example: "nesting halve 203", submenu: Some(NESTING_MENU) },
    MenuItem { name: "carriers", cmd: "carriers", desc: "census of the mu-delta=id carriers by class: one fixed point seen many ways, or a family", example: "carriers", submenu: Some(CARRIERS_MENU) },
    MenuItem { name: "substrate", cmd: "substrate", desc: "closure constant, content bifurcating: the conservative substrate read on both observables", example: "substrate", submenu: Some(SUBSTRATE_MENU) },
    MenuItem { name: "stark", cmd: "stark", desc: "Stark unit extraction: formula,fibqc,tower,exponents,verify", example: "stark formula 2048", submenu: None },
    MenuItem { name: "riemann", cmd: "riemann", desc: "Riemann-SIC report; sub-actions available", example: "riemann", submenu: None },
    MenuItem { name: "distance", cmd: "distance", desc: "Hamming + weighted distance vs the ZFC baseline tuple (alias dist)", example: "distance", submenu: None },
    MenuItem { name: "join", cmd: "join", desc: "join of the active IG tuple with the ZFC baseline", example: "join", submenu: None },
    MenuItem { name: "sigma", cmd: "sigma", desc: "sigma <n> — analyze the Sigma(n) divisor ring", example: "sigma 5", submenu: None },
    MenuItem { name: "ringspec", cmd: "ringspec", desc: "ringspec <w1> <w2> <w3> — the spectrum of a ring, in integers: bond weights around a cycle, clean bond 1, cross-link its reaction centres; three is the minimum", example: "ringspec 1 2 2 1", submenu: None },
    MenuItem { name: "clay", cmd: "clay", desc: "Clay Millennium structural status (machine-checked)", example: "clay", submenu: None },
    MenuItem { name: "psm", cmd: "psm", desc: "dialetheic alignment + measurement tests", example: "psm test", submenu: None },
    MenuItem { name: "entropy", cmd: "entropy", desc: "entropy experiment: dS vs tier promotion", example: "entropy tier", submenu: None },
    MenuItem { name: "invariant", cmd: "invariant", desc: "Discover invariants under transformations: ROTAT, IMSCRIB, FSPLIT/FFUSE", example: "invariant catalog under ROTAT", submenu: None },
    MenuItem { name: "redteam", cmd: "redteam", desc: "Adversarial testing: analyze|stress|mutate, and audit <theory> for hidden assumptions", example: "redteam audit RH", submenu: None },
    MenuItem { name: "witness", cmd: "witness", desc: "Smallest executable object standing behind a claim", example: "witness bsd", submenu: None },
    MenuItem { name: "counterfactual", cmd: "counterfactual", desc: "Perturb one glyph: invariants held/broken, reversibility, smallest repair (alias cf)", example: "counterfactual \u{22a2}\u{2208}\u{22a4}\u{220b}\u{22a3} rotate 1", submenu: None },
    MenuItem { name: "basin", cmd: "basin", desc: "Fixed-point archaeology: orbit, attractor, transient depth, exact basin size", example: "basin \u{22a2}\u{2208}\u{22a4}\u{220b} --action REPAIR", submenu: None },
    MenuItem { name: "ouroboros-inverse", cmd: "ouroboros-inverse", desc: "Inverse grammar: shortest IMASM word imscribing a tuple, plus its braid (alias oinv)", example: "oinv", submenu: None },
    MenuItem { name: "frobenius-fuzzer", cmd: "frobenius-fuzzer", desc: "Mine the word space for programs the braid reproduces exactly (alias fuzz)", example: "fuzz --len 3", submenu: None },
    MenuItem { name: "oracle", cmd: "oracle", desc: "Adversarial: hunt the cheapest structural counterexample; surviving is not proof", example: "oracle rotat-register", submenu: None },
    MenuItem { name: "blackbox", cmd: "blackbox", desc: "Infer a law from integer observations, ranked by fit minus complexity", example: "blackbox 1 1 2 3 5 8 13", submenu: None },
    MenuItem { name: "dialetheic-compiler", cmd: "dialetheic-compiler", desc: "Lift a classical gate into Belnap FOUR; show where a row rests on a paradox", example: "dialetheic-compiler xor", submenu: None },
    MenuItem { name: "stark-geometer", cmd: "stark-geometer", desc: "SIC Stark arithmetic for dimension d: m_d, unit, ramified primes", example: "stark-geometer 12", submenu: None },
    MenuItem { name: "dialect-necromancer", cmd: "dialect-necromancer", desc: "Imscribe a fragment and recover its nearest catalog ghost", example: "dialect-necromancer the boundary imscribes the bulk", submenu: None },
    MenuItem { name: "braid-apocrypha", cmd: "braid-apocrypha", desc: "Search braid words for a target Jones magnitude; first hit is shortest", example: "braid-apocrypha --target 0.618034", submenu: None },
    MenuItem { name: "proof-braider", cmd: "proof-braider", desc: "Lift a claim to a braid and back; PASS iff Frobenius closure survives", example: "proof-braider roundtrip Imscribing.Frobenius", submenu: None },
    MenuItem { name: "universe-wormhole", cmd: "universe-wormhole", desc: "Minimum gate-space path between two hop frameworks, as a braid + Jones", example: "universe-wormhole hqe fibonacci", submenu: None },
    MenuItem { name: "vox-ce", cmd: "vox-ce", desc: "Lift EVM/WASM hex into an IMASM word and verdict its control-flow closure", example: "vox-ce evm 0x600160025b00", submenu: None },
    MenuItem { name: "consciousness-lath", cmd: "consciousness-lath", desc: "Single-axis mutation that most raises the C-score with both gates open", example: "consciousness-lath ⊢∈><⊤⋈⊙⊞∋⊡⊣", submenu: None },
    MenuItem { name: "paradox-engine", cmd: "paradox-engine", desc: "Hunt words that are dialetheias by four readings at once (B, price, gate1, C=0)", example: "paradox-engine --min-price 3", submenu: None },
    MenuItem { name: "key-dissolver", cmd: "key-dissolver", desc: "SIC-narrowed bounded window before a BSGS split; recovers no real key", example: "key-dissolver 03f01d 40", submenu: None },
    MenuItem { name: "compiler", cmd: "compiler", desc: "Compile a braid to imasm/jones/lean, or a token word back to a braid", example: "compiler braid 1 2 1 --to imasm", submenu: None },
    MenuItem { name: "catalogue", cmd: "catalogue", desc: "Synthesize candidate operators; rank by novelty against the catalog", example: "catalogue synthesize --top 5", submenu: None },
    MenuItem { name: "sk_forge", cmd: "sk_forge", desc: "Crystal Harvester: BIP39-SIC integrated structural gap analysis against O_∞ carriers; scalar is STRUCTURAL, never a key. Commands: forge, tuple, word, verify, carriers, bip39-sic, bip39-pipeline (alias sk-forge)", example: "sk_forge bip39-sic", submenu: None },
    MenuItem { name: "museum", cmd: "museum", desc: "The permanent collection of failed constructions — append-only negative knowledge", example: "museum open", submenu: None },
    MenuItem { name: "phase", cmd: "phase", desc: "Phase as an object: orbit spectrum, phase period, and two-word interference", example: "phase interference \u{22a2}\u{2208}\u{22a4}\u{220b} \u{22a2}\u{22a4}\u{2208}\u{220b}", submenu: None },
    MenuItem { name: "demonstrate", cmd: "demonstrate", desc: "Run a claim as an experiment: INPUT/OPERATION/OUTPUT/CHECK, computed live (alias demo)", example: "demonstrate mu-delta 1 2 -1", submenu: None },
    MenuItem { name: "loss", cmd: "loss", desc: "What a transformation destroys: entropy in/out, bits destroyed, irreversible transitions", example: "loss collapse", submenu: None },
    MenuItem { name: "shadow", cmd: "shadow", desc: "Ontological nearest-neighbour: shared structure and the measured critical difference", example: "shadow hsoa", submenu: None },
    MenuItem { name: "provenance", cmd: "provenance", desc: "Epistemic type check: dependency DAG graded by lattice MEET, not by best sibling (alias prov)", example: "prov RH", submenu: None },
    MenuItem { name: "ctc-loom", cmd: "ctc-loom", desc: "Sweep the six Belnap actions over the whole word space; rank closures by price (alias loom)", example: "ctc-loom --len 3", submenu: None },
    MenuItem { name: "cl9nk", cmd: "cl9nk", desc: "CLINK L9, the replicative lateral: d(L8,L9) and the ladder read from L9", example: "cl9nk chain", submenu: None },
    MenuItem { name: "crystal-scope", cmd: "crystal-scope", desc: "Substitution microscope: distance, tier, dS, gate jump, and the measured driver (alias cscope)", example: "cscope", submenu: None },
    MenuItem { name: "minimal", cmd: "minimal", desc: "Shortest word achieving a target property", example: "minimal reach O_inf", submenu: None },
    MenuItem { name: "repair", cmd: "repair", desc: "Ranked program/proof surgery with a proof-diff", example: "repair \u{22a2}\u{2208}", submenu: None },
    MenuItem { name: "mersearch", cmd: "mersearch", desc: "Mersenne search: run|ll. Composite exponents answer at once (alias msearch)", example: "msearch ll 2213", submenu: None },
    MenuItem { name: "pk2sk", cmd: "pk2sk", desc: "PK→SK recovery: bounded-range ECDLP on secp256k1 — recover the scalar in [lo, hi) from its compressed public key, curve-gated, imscribed", example: "pk2sk selftest", submenu: Some(PK2SK_MENU) },
];

pub static PK2SK_MENU: &[MenuItem] = &[
    MenuItem { name: "search", cmd: "pk2sk search", desc: "recover the private scalar from a compressed public key when the scalar lies in [lo, hi): BSGS meet-in-the-middle, gated by the curve itself", example: "pk2sk search 03f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f 12000 13000", submenu: None },
    MenuItem { name: "selftest", cmd: "pk2sk selftest", desc: "recover the fixed selftest key (SK = 0x1000000b8ef) from its public key alone", example: "pk2sk selftest", submenu: None },
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

    // Walk the whole menu tree — top level AND every submenu — so a tool listed
    // under a category (e.g. sk_forge under Grammar) is findable, not just the
    // category names. Matches on name, cmd, or description.
    let mut found = false;
    fn walk(items: &[MenuItem], kw: &str, found: &mut bool) {
        for item in items.iter() {
            if item.name.to_lowercase().contains(kw)
                || item.cmd.to_lowercase().contains(kw)
                || item.desc.to_lowercase().contains(kw)
            {
                crate::sprintln!("  {:18} — {}", item.cmd, item.desc);
                *found = true;
            }
            if let Some(sub) = item.submenu {
                walk(sub, kw, found);
            }
        }
    }
    walk(MAIN_MENU, &kw, &mut found);

    // Also match known sub-commands not carried as menu items
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

    let mut extra = false;
    for (name, desc) in known {
        if name.contains(&kw) || desc.to_lowercase().contains(&kw) {
            if !extra {
                crate::sprintln!("\nADDITIONAL COMMANDS:");
                extra = true;
            }
            crate::sprintln!("  {:18} — {}", name, desc);
        }
    }
    found |= extra;

    if !found {
        crate::sprintln!("No commands found matching '{}'.", keyword);
        crate::sprintln!("Type 'help' with no argument for the full list.");
    }
}
