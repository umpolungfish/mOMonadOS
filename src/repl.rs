// repl.rs — the interactive REPL, input handling, command dispatch, and all
// Phase-2 / ParaASM / cross-dialect handlers. Extracted from main.rs, which now
// holds only the bare-metal entry, allocator, boot banner, and panic handler.
#![allow(unused_imports)]

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;

use crate::{sprint, sprintln};
use crate::{
    serial, belnap, tokens, crystal, kernel, interrupts, frob_verify, imas_ig,
    aleph, manus, parasm, belnap_shor, para_rh, para_ym, para_temporal,
    para_category, algebra, catalog, cl8nk, consciousness, rebis, dialect, menu,
    sequence, boot, cr3echrz, canonical_ordinal, clay_status, sic_povm,
    frobenius_unify, clay_witness, belnap_sic_bridge, belnap_c4, sic_compute,
    dialect_expansion, divisor_ring, mersenne_parallel, bifurcation_test, entropy, d12_sic, d2048_sic, d2048_sieve,
    witness_vessel, ask,
};
use crate::tokens::{canonical_name, CANONICAL_COUNT, continuous_name, CONTINUOUS_COUNT, novel_name, NOVEL_COUNT, shunted_name, SHUNTED_COUNT, compound_name, compound_index, compound_program, COMPOUND_COUNT};
use crate::crystal::{CrystalStore, decode, encode, indices_from_program, TOTAL};
use crate::kernel::Kernel;
use crate::imas_ig::{IgTuple, IgPrim};
use crate::dialect::{parse_dialect, dialect_display, dialect_name, dialect_description, dialect_gates, dialect_o_inf};
use crate::menu::{ContextStack, render_menu_bar, menu_hint, tab_complete, print_help_topic, search_commands, enter_context, fkey_to_category, render_prompt};

// ─── History ──────────────────────────────────────────────────

const HISTORY_CAP: usize = 32;

struct History {
    bufs: [[u8; 256]; HISTORY_CAP],
    lens: [usize; HISTORY_CAP],
    write_idx: usize,
    count: usize,
}

impl History {
    const fn new() -> Self {
        Self {
            bufs: [[0u8; 256]; HISTORY_CAP],
            lens: [0usize; HISTORY_CAP],
            write_idx: 0,
            count: 0,
        }
    }

    fn push(&mut self, line: &[u8]) {
        if line.is_empty() { return; }
        let n = line.len().min(255);
        self.bufs[self.write_idx][..n].copy_from_slice(&line[..n]);
        self.lens[self.write_idx] = n;
        self.write_idx = (self.write_idx + 1) % HISTORY_CAP;
        if self.count < HISTORY_CAP { self.count += 1; }
    }

    fn get(&self, back: usize) -> Option<(&[u8], usize)> {
        if back == 0 || back > self.count { return None; }
        let idx = (self.write_idx + HISTORY_CAP - back) % HISTORY_CAP;
        Some((&self.bufs[idx], self.lens[idx]))
    }
}

// ─── REPL ─────────────────────────────────────────────────────

pub fn repl(k: &mut Kernel) {
    let mut cfs = CrystalStore::new();
    let mut line_buf = [0u8; 512];
    let mut history = History::new();
    let mut ctx_stack = ContextStack::new();
    let mut ask_paste = crate::ask::AskPaste::new();

    sprintln!("Type '?' for menu, 'help' for categories, Tab to complete.");
    sprintln!("Kernel ask: structural dry-run (serial). Full wet-run (files, Gemini-length answers):");
    sprintln!("  host: ./ask --file path | ./ask --ask \"…\" | ./ask -i   (no Python)");
    sprintln!();

    loop {
        render_prompt(&ctx_stack);
        let line = read_line(&mut line_buf, &mut history, &ctx_stack);
        if line.is_empty() { continue; }

        // Multi-line ask paste: accumulate until a lone `.`
        if ask_paste.active {
            let t = line.trim();
            if t == "." {
                ask_paste.active = false;
                let q = ask_paste.buf.clone();
                ask_paste.buf.clear();
                sprintln!("{}", crate::ask::run_ask(&q, &ask_paste.opts, k));
            } else {
                if !ask_paste.buf.is_empty() {
                    ask_paste.buf.push(' ');
                }
                ask_paste.buf.push_str(line);
            }
            continue;
        }

        let mut parts = line.splitn(4, ' ');
        let cmd = parts.next().unwrap_or("");

        // ── Menu Navigation ────────────────────────────────
        match cmd {
            // Exit sub-context
            ".." | "back" => {
                if ctx_stack.depth > 0 {
                    let popped = ctx_stack.pop();
                    sprintln!("← returned from {}", popped.map(|c| c.name).unwrap_or("?"));
                } else {
                    sprintln!("Already at top level.");
                }
                continue;
            }
            // Menu bar
            "?" if parts.clone().next().is_none() => {
                render_menu_bar();
                menu_hint();
                continue;
            }
            // Search commands
            "?" => {
                let keyword = parts.next().unwrap_or("");
                sprintln!("Searching: '{}'", keyword);
                search_commands(keyword);
                continue;
            }
            // Menu shortcuts (:1 through :10)
            cmd if cmd.starts_with(":") => {
                if let Ok(n) = cmd[1..].parse::<u8>() {
                    if let Some(cat) = fkey_to_category(n) {
                        if enter_context(&mut ctx_stack, cat) {
                            if let Some(ctx) = ctx_stack.current() {
                                print_help_topic(ctx.name);
                            }
                        }
                    } else {
                        sprintln!("Invalid category: :{} (use :1–:10)", n);
                    }
                }
                continue;
            }
            // Enter category by shortcut (case-insensitive, names that don't conflict with commands)
            s if {
                let lower = s.to_lowercase();
                lower == "exec" || lower == "status" || lower == "programs" || lower == "crystal"
                    || lower == "grammar" || lower == "rebis" || lower == "dialect" || lower == "parasm" || lower == "cr3echrz" || lower == "clay"
            } => {
                let already_in = ctx_stack.current()
                    .map(|c| c.name.to_lowercase() == cmd.to_lowercase())
                    .unwrap_or(false);
                if !already_in {
                    if enter_context(&mut ctx_stack, &cmd.to_lowercase()) {
                        if let Some(ctx) = ctx_stack.current() {
                            print_help_topic(ctx.name);
                        }
                    }
                    continue;
                }
                // Already in this context — fall through to command dispatch below
            }
            _ => {}
        }

        match cmd {
            "quit" | "exit" | "halt" => {
                sprintln!("Halting. μ∘δ=id.");
                k.halt();
                break;
            }
            "help" => {
                let topic = parts.next().unwrap_or("");
                print_help_topic(topic);
            },
            "status" => print_status(k),
            "proof" => {
                match parts.next().unwrap_or("") {
                    "" | "list" => crate::proof::list_proofs(),
                    "bootstrap" => crate::proof::walk_bootstrap(),
                    "parity" => crate::proof::walk_parity(),
                    other => {
                        sprintln!("No guided proof named '{}'.", other);
                        crate::proof::list_proofs();
                    }
                }
            },
            "frob" => print_frob(k),
            "ig" => print_ig(k),
            "classify" => print_classify(k),
            "arev" => {
                match parts.next().unwrap_or("") {
                    ""     => print_arev_hop(k),
                    "test" => print_arev_test(k),
                    _ => sprintln!("arev [test] — Ħ hop to the lateral partner (O_∞ ↔ O_inf_dag) / door experiment"),
                }
            }
            "aleph" => print_aleph(k, parts.next().unwrap_or("")),
            "psm" => {
                let psm_arg = parts.next().unwrap_or("");
                let psm_rest: alloc::string::String = parts.collect::<alloc::vec::Vec<&str>>().join(" ");
                let psm_full = if psm_rest.is_empty() { alloc::string::String::from(psm_arg) } else { alloc::format!("{} {}", psm_arg, psm_rest) };
                print_psm(&psm_full);
            }
            "shor" => print_shor(),
            "rh" => print_rh(),
            "ym" => print_ym(),
            "temp" => print_temporal(),
            "cat" => print_cat(),
            "algebra" => print_algebra(k, parts.next().unwrap_or("")),
            "cl8nk" => {
                let action = parts.next().unwrap_or("");
                let name = parts.next().unwrap_or("");
                print_cl8nk(action, name);
            },
            "c4" => print_c4_arg(parts.next().unwrap_or("")),
            "cscore" => print_cscore(k),
            "clay" => {
                let sub = parts.next().unwrap_or("");
                if sub == "witness" {
                    let problem = parts.next().unwrap_or("");
                    if problem.is_empty() {
                        sprintln!("{}", crate::clay_witness::list_witnesses());
                    } else {
                        sprintln!("{}", crate::clay_witness::witness_report(problem));
                    }
                } else {
                    print_clay();
                }
            }
            "sic" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "verify" => sprintln!("{}", crate::sic_compute::sic_full_report()),
                    "bridge" => sprintln!("{}", crate::belnap_sic_bridge::bridge_report()),
                    "" => print_sic(),
                    _ => sprintln!("sic [verify | bridge] — SIC-POVM d=12 commands"),
                }
            }
            "entropy" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "tier" => sprintln!("{}", crate::entropy::entropy_summary()),
                    "transition" => sprintln!("{}", crate::entropy::transition_report()),
                    "" => sprintln!("{}", crate::entropy::entropy_report()),
                    _ => sprintln!("entropy [tier | transition] — Phase V entropy experiment"),
                }
            }
            "sigma" => {
                let arg = parts.next().unwrap_or("");
                match arg {
                    "" => sprintln!("sigma <n> — analyze Σ(n) divisor ring"),
                    "mersenne" | "m" => {
                        let p_str = parts.next().unwrap_or("");
                        if let Ok(p) = p_str.parse::<u32>() {
                            if let Some((p, mp, result)) = crate::divisor_ring::analyze_mersenne(p) {
                                sprintln!("Mersenne M_{} = {}:", p, mp);
                                sprintln!("{}", crate::divisor_ring::format_report(&result));
                            } else {
                                sprintln!("p={} overflows u64 (max p=63)", p);
                            }
                        } else {
                            sprintln!("Usage: sigma mersenne <exponent>");
                        }
                    }
                    "scan" => {
                        let args: Vec<&str> = parts.collect::<Vec<&str>>();
                        if args.len() >= 2 {
                            if let (Ok(start), Ok(end)) = (args[0].parse::<u32>(), args[1].parse::<u32>()) {
                                sprintln!("=== MERSENNE SCAN p={}..{} ===", start, end);
                                sprintln!("{:>4} {:>24} {:>14} {:>6}", "p", "M_p", "VERDICT", "Ω");
                                sprintln!("{}", "-".repeat(52));
                                let results = crate::divisor_ring::scan_mersenne_range(start, end);
                                for (p, mp, verdict, omega) in &results {
                                    sprintln!("{:>4} {:>24} {:>14} {:>6}", p, mp, verdict, omega);
                                }
                            } else {
                                sprintln!("Usage: sigma scan <start> <end>");
                            }
                        } else {
                            sprintln!("Usage: sigma scan <start> <end> — scan Mersenne range");
                        }
                    }
                    "prox" | "proximity" => {
                        let p_str = parts.next().unwrap_or("");
                        if let Ok(p) = p_str.parse::<u32>() {
                            if let Some(prox) = crate::divisor_ring::mersenne_proximity(p) {
                                sprintln!("Mersenne proximity M_{}: {:.6}", p, prox);
                            } else {
                                sprintln!("p={} overflows u64", p);
                            }
                        } else {
                            sprintln!("Usage: sigma prox <exponent>");
                        }
                    }
                    _ => {
                        if let Ok(n) = arg.parse::<u64>() {
                            let result = crate::divisor_ring::analyze(n);
                            sprintln!("{}", crate::divisor_ring::format_report(&result));
                        } else {
                            sprintln!("Usage: sigma <n> | sigma mersenne <p> | sigma scan <start> <end> | sigma prox <p>");
                        }
                    }
                }
            }
            "mersearch" | "msearch" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "run" | "search" => {
                        let args: Vec<&str> = parts.collect::<Vec<&str>>();
                        if args.len() >= 2 {
                            if let (Ok(start), Ok(end)) = (args[0].parse::<usize>(), args[1].parse::<usize>()) {
                                sprintln!("{}", crate::mersenne_parallel::search_report(start, end));
                            } else {
                                sprintln!("Usage: mersearch run <start> <end>");
                            }
                        } else {
                            sprintln!("Usage: mersearch run <start> <end>");
                        }
                    }
                    "ll" => {
                        let p_str = parts.next().unwrap_or("");
                        if let Ok(p) = p_str.parse::<usize>() {
                            sprintln!("Running Lucas-Lehmer for M_{}...", p);
                            let result = crate::mersenne_parallel::lucas_lehmer(p);
                            if result {
                                sprintln!("M_{} is PRIME!", p);
                            } else {
                                sprintln!("M_{} is composite.", p);
                            }
                        } else {
                            sprintln!("Usage: mersearch ll <exponent>");
                        }
                    }
                    "" => {
                        sprintln!("mersearch — Parallel Mersenne Prime Search");
                        sprintln!("  mersearch run <start> <end>  — search range with parallel LL");
                        sprintln!("  mersearch ll <exponent>      — test single exponent");
                    }
                    _ => sprintln!("mersearch [run|ll]"),
                }
            }
            "d2048" | "d2k" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "tower" => sprintln!("{}", crate::d2048_sic::tower_ascent_report()),
                    "c16" => sprintln!("{}", crate::d2048_sic::c16_report()),
                    "c32" | "hilbert" => sprintln!("{}", crate::d2048_sic::c32_report()),
                    "ramified" | "ram" => sprintln!("{}", crate::d2048_sic::ramified_report()),
                    "redei" => sprintln!("{}", crate::d2048_sic::redei_report()),
                    "grammar" | "ob3ect" => sprintln!("{}", crate::d2048_sic::grammar_report()),
                    "pari" | "run" => sprintln!("{}", crate::d2048_sic::pari_runner_report()),
                    "next" | "eagle" => sprintln!("{}", crate::d2048_sic::next_eagle_report()),
                    "sieve" | "fold" | "fork" => sprintln!("{}", crate::d2048_sieve::sieve_report()),
                    "verify" | "full" => sprintln!("{}", crate::d2048_sic::d2048_full_report()),
                    "" => sprintln!("{}", crate::d2048_sic::d2048_summary()),
                    _ => sprintln!("d2048 [tower|c16|c32|ramified|redei|grammar|pari|next|sieve|verify]"),
                }
            }
            "d12" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "tower" => sprintln!("{}", crate::d12_sic::phase_tower_collapse_report()),
                    "magnitudes" | "mag" => sprintln!("{}", crate::d12_sic::magnitude_report()),
                    "orbits" => sprintln!("{}", crate::d12_sic::orbit_report()),
                    "existence" | "ring" => sprintln!("{}", crate::d12_sic::existence_ring_report()),
                    "duallink" | "dl" => sprintln!("{}", crate::d12_sic::dual_link_report()),
                    "z0" => sprintln!("{}", crate::d12_sic::z0_report()),
                    "ordinals" | "ord" => sprintln!("{}", crate::d12_sic::ordinal_guards_report()),
                    "verify" => sprintln!("{}", crate::d12_sic::d12_full_report()),
                    "embedding" | "capstone" => sprintln!("{}", crate::d12_sic::embedding_report()),
                    "symmetric" | "sym" => sprintln!("{}", crate::d12_sic::symmetric_moduli_report()),
                    "lean-status" | "lean" => sprintln!("{}", crate::d12_sic::lean_status_report()),
                    "unconditional" | "belnap" => sprintln!("{}", crate::d12_sic::belnap_sic_unconditional_report()),
                    "" => sprintln!("{}", crate::d12_sic::d12_summary()),
                    _ => sprintln!("d12 [tower|magnitudes|orbits|existence|duallink|z0|ordinals|verify|embedding|symmetric|lean-status]"),
                }
            }
            "vessel" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "run" | "verify" => sprintln!("{}", crate::witness_vessel::vessel_report()),
                    "" => sprintln!("{}", crate::witness_vessel::vessel_summary()),
                    _ => sprintln!("vessel [run] — witness-vessel transport protocol"),
                }
            }
            // Manuscript spine: PROVE→UNIFY→PORT ledger + vessel runtime half.
            // No Python. Formal pack in p4ramill VAE_Vita_ManuscriptSpine.
            "spine" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "run" | "full" => {
                        sprintln!("{}", crate::d12_sic::manuscript_spine_report());
                        sprintln!("{}", crate::witness_vessel::vessel_report());
                        sprintln!("{}", crate::frobenius_unify::formatted_report());
                    }
                    "lean" | "status" | "" => {
                        sprintln!("{}", crate::d12_sic::manuscript_spine_report());
                    }
                    _ => sprintln!("spine [run|lean] — manuscript spine (PROVE→UNIFY→PORT × vessel)"),
                }
            }
            // MoDoT Constant Closure: 5 Lean modules (FineStructure, ProtonElectron,
            // LeptonMassRatios, BosonMassRatios, GravitationalCoupling) ported to kernel.
            "constants" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "fine-structure" | "alpha" | "α" => {
                        sprintln!("{}", crate::constant_closure::fine_structure_report());
                    }
                    "proton-electron" | "mpme" | "pem" => {
                        sprintln!("{}", crate::constant_closure::proton_electron_report());
                    }
                    "lepton" | "muon" | "tau" => {
                        sprintln!("{}", crate::constant_closure::lepton_report());
                    }
                    "boson" | "w" | "z" | "higgs" => {
                        sprintln!("{}", crate::constant_closure::boson_report());
                    }
                    "gravitational" | "gravity" | "alpha_g" | "ag" => {
                        sprintln!("{}", crate::constant_closure::gravitational_report());
                    }
                    "verify" | "status" => {
                        sprintln!("{}", crate::constant_closure::constant_closure_status_report());
                    }
                    "all" | "full" | "" => {
                        sprintln!("{}", crate::constant_closure::full_constant_closure_report());
                    }
                    _ => sprintln!("constants [fine-structure|proton-electron|lepton|boson|gravitational|verify|all]"),
                }
            }

            // Native MoDoT-parity ask (ob3ect native_kernel_ask). Full line after `ask `.
            "ask" => {
                let rest = if let Some(i) = line.find(char::is_whitespace) {
                    line[i..].trim()
                } else {
                    ""
                };
                if rest == "/" || rest.starts_with("/ ") {
                    let (opts, _) = crate::ask::parse_ask_args(rest.trim_start_matches('/'));
                    ask_paste.active = true;
                    ask_paste.buf.clear();
                    ask_paste.opts = opts;
                    sprintln!("ask paste mode — enter question lines; end with a line containing only .");
                } else {
                    let (opts, q) = crate::ask::parse_ask_args(rest);
                    sprintln!("{}", crate::ask::run_ask(&q, &opts, k));
                }
            }
            // The trunk's mouth: one certified turn from the on-board vae_vita
            // lattice, gated by the kernel's own close condition.
            #[cfg(feature = "vita")]
            "vita" => {
                // seed ↔ word is 1:1 — there is no default word, so there is no
                // default seed: unseeded turns draw from the machine's moment.
                let seed: u64 = parts.next().and_then(|s| s.trim().parse().ok())
                    .unwrap_or_else(|| unsafe { core::arch::x86_64::_rdtsc() });
                let temp: f32 = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(0.8);
                // The whole turn is transient: print, then roll the bump heap
                // back so repeated turns never exhaust it.
                let mark = crate::heap_mark();
                match crate::vita::Vita::load() {
                    Some(v) => sprintln!("{}", v.speak_turn(seed, temp, 24)),
                    None => sprintln!("vita: baked weights missing/corrupt (rebuild with vita_weights.bin present)"),
                }
                crate::heap_reset(mark);
            }
            "rebis" => {
                let sub = parts.next().unwrap_or("");
                print_rebis(sub, parts.next().unwrap_or(""), &parts.collect::<alloc::vec::Vec<&str>>().join(" "));
            }
            "cr3" => {
                let sub = parts.next().unwrap_or("");
                print_cr3(sub, parts.collect::<alloc::vec::Vec<&str>>().join(" "));
            }
            "p4ra" => {
                let sub = parts.next().unwrap_or("");
                print_p4ra(sub, parts.collect::<alloc::vec::Vec<&str>>().join(" "));
            }
            "tick" => {
                let n: u64 = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(1);
                for _ in 0..n { if !k.tick() { break; } }
                print_status(k);
            }
            "run" => {
                let arg = parts.next().unwrap_or("").trim();
                if let Ok(n) = arg.parse::<u64>() {
                    k.run(n);
                    print_status(k);
                } else {
                    sprintln!("Running continuously (press ESC to stop)...");
                    let ran = k.run_continuous(|| interrupts::escape_pressed());
                    sprintln!();
                    sprintln!("Stopped after {} ticks.", ran);
                    print_status(k);
                }
            }
            "timer" => {
                let n: u64 = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(10);
                sprintln!("Timer-driven: {} ticks (ESC to stop early)...", n);
                let mut ran = 0u64;
                while ran < n {
                    while !interrupts::timer_ready() {
                        if interrupts::escape_pressed() { break; }
                        unsafe { core::arch::asm!("hlt", options(nostack, nomem, preserves_flags)); }
                    }
                    if interrupts::escape_pressed() { break; }
                    interrupts::pending_ticks();
                    if !k.tick() { break; }
                    ran += 1;
                }
                sprintln!();
                sprintln!("Timer ran {} ticks.", ran);
                print_status(k);
            }
            "boot" => {
                let arg = parts.next().unwrap_or("").trim();
                // Try Roman numeral first; fall back to decimal
                let idx = roman_to_idx(arg)
                    .or_else(|| arg.parse::<usize>().ok().map(|n| n.saturating_sub(1)));
                if let Some(i) = idx {
                    if i >= CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT + SHUNTED_COUNT {
                        sprintln!("Program {} out of range (max XXVIII/{}).",
                            arg, CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT + SHUNTED_COUNT);
                    } else if load_by_roman(k, arg) {
                        let name: &str = if i < CANONICAL_COUNT {
                            canonical_name(i)
                        } else if i < CANONICAL_COUNT + CONTINUOUS_COUNT {
                            continuous_name(i - CANONICAL_COUNT)
                        } else if i < CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT {
                            novel_name(i - CANONICAL_COUNT - CONTINUOUS_COUNT)
                        } else {
                            shunted_name(i - CANONICAL_COUNT - CONTINUOUS_COUNT - NOVEL_COUNT)
                        };
                        sprintln!("Booting {}: {}", arg, name);
                        sprintln!("Running (ESC to stop)...");
                        let ran = k.run_continuous(|| interrupts::escape_pressed());
                        sprintln!("\nStopped after {} ticks.", ran);
                        print_status(k);
                    }
                } else {
                    sprintln!("Usage: boot <I–XXVIII>");
                    sprintln!("Use 'list' to see all programs.");
                }
            }
            "novel" => {
                let arg = parts.next().unwrap_or("").trim();
                if let Ok(i) = arg.parse::<usize>() {
                    let idx = i.saturating_sub(1);
                    if idx < NOVEL_COUNT {
                        k.load_novel(idx);
                        sprintln!("Booting novel {}: {}", i, novel_name(idx));
                        sprintln!("Running (ESC to stop)...");
                        let ran = k.run_continuous(|| interrupts::escape_pressed());
                        sprintln!("
Stopped after {} ticks.", ran);
                        print_status(k);
                    } else {
                        sprintln!("Novel index {} out of range (max {}).",
                            i, NOVEL_COUNT);
                    }
                } else {
                    sprintln!("Usage: boot novel <1-{}>", NOVEL_COUNT);
                }
            }
            "shunt" => {
                let arg = parts.next().unwrap_or("").trim();
                if let Ok(i) = arg.parse::<usize>() {
                    let idx = i.saturating_sub(1);
                    if idx < SHUNTED_COUNT {
                        k.load_shunted(idx);
                        sprintln!("Booting shunted {}: {}", i, shunted_name(idx));
                        sprintln!("Running (ESC to stop)...");
                        let ran = k.run_continuous(|| interrupts::escape_pressed());
                        sprintln!("
Stopped after {} ticks.", ran);
                        print_status(k);
                    } else {
                        sprintln!("Shunted index {} out of range (max {}).",
                            i, SHUNTED_COUNT);
                    }
                } else {
                    sprintln!("Usage: boot shunt <1-{}>", SHUNTED_COUNT);
                }
            }
                        "watch" => {
                let arg = parts.next().unwrap_or("").trim();
                let refresh: u64 = arg.parse().ok().unwrap_or(10);
                let name = if k.snapshot.is_some() { "current" } else { "(none)" };
                let width: u16 = 80;
                manus::display_init(&k, name, width);
                sprintln!("Watching. ESC to stop (refresh every {} ticks)...", refresh);
                let ran = manus::run_with_display(k, name, width, refresh,
                    || interrupts::escape_pressed());
                manus::display_shutdown();
                sprintln!();
                sprintln!("Stopped after {} ticks.", ran);
                print_status(k);
            }
                        "graph" => {
                manus::draw_token_graph(&k);
                sprintln!();
            }
            "heatmap" => {
                let start: usize = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(0);
                let count: usize = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(64);
                manus::draw_memory_heatmap(&k, start, count, 80);
                sprintln!();
            }
            "program" => {
                for (i, t) in k.program.as_slice().iter().enumerate() {
                    if i > 0 { serial::write_str(" → "); }
                    serial::write_str(t.name());
                }
                sprintln!();
                sprintln!("len={} ip={} fork_depth={}",
                    k.program.len(), k.ip, k.fork_depth());
            }
            "snapshot" => {
                if let Some(snap) = k.snapshot {
                    sprintln!("Tier:     {}", snap.tier_name());
                    sprintln!("sig:      ({},{},{},{})  [L,F,D,X]",
                        snap.sig.0, snap.sig.1, snap.sig.2, snap.sig.3);
                    sprintln!("diversity:{}/12", snap.token_diversity);
                    sprintln!("self_ref: {}", snap.self_ref);
                    sprintln!("frob_ord: {}", snap.frobenius_order);
                    // Show static and effective separately — VII is the case that
                    // diverges (static false, b_live > 0 → effective true → tier climbs).
                    let eff_dial = snap.dialetheia_complete || snap.b_live_ticks > 0;
                    sprintln!("dialeth:  static={} effective={} (b_live_ticks={})",
                        snap.dialetheia_complete, eff_dial, snap.b_live_ticks);
                    sprintln!("period:   {}", snap.period);
                    sprintln!("atomic_reentry:        {}", snap.atomic_reentry);
                    sprintln!("bifurcation_revisited: {}", snap.bifurcation_revisited);
                    sprintln!("winding_count:         {}", snap.winding_count);
                } else {
                    sprintln!("No snapshot — tick first.");
                }
            }
            "replicative" => {
                // Loads the program that deliberately targets O_inf_dag (R2) rather than
                // merely being reachable by accident — see tokens::replicative_opening_loop.
                // Ticks past the first wrap (winding_count > 0 requires at least one) and
                // reports the actual tier the kernel computed, not an expectation.
                k.load_replicative();
                for _ in 0..8 { k.tick(); }
                if let Some(snap) = k.snapshot {
                    sprintln!("Program: IMSCRIB → FSPLIT → FFUSE → IMSCRIB (cyclic)");
                    sprintln!("Tier:     {}", snap.tier_name());
                    sprintln!("self_ref: {}  frob_ord: {}", snap.self_ref, snap.frobenius_order);
                    sprintln!("atomic_reentry:        {}", snap.atomic_reentry);
                    sprintln!("bifurcation_revisited: {}", snap.bifurcation_revisited);
                    sprintln!("winding_count:         {}", snap.winding_count);
                    sprintln!("value_period:          {}  (Path B guard: stays < 3)", snap.value_period);
                    let eff_dial = snap.dialetheia_complete || snap.b_live_ticks > 0;
                    sprintln!("effective_dialetheia:  {}  (Path A guard: stays false)", eff_dial);
                    if snap.tier == 4 {
                        sprintln!("-> R2 fired: lateral replicative opening, deliberately, not by accident.");
                    } else {
                        sprintln!("-> WARNING: expected tier 4 (O_inf_dag), got {} — the hand trace was wrong.", snap.tier_name());
                    }
                } else {
                    sprintln!("No snapshot after ticking — something is wrong.");
                }
            }
            "canonical" => {
                let arg = parts.next().unwrap_or("").trim();
                let idx = roman_to_idx(arg)
                    .or_else(|| arg.parse::<usize>().ok().map(|n| n.saturating_sub(1)));
                if let Some(i) = idx {
                    k.load_canonical(i);
                    sprintln!("Loaded {}: {}", i + 1, canonical_name(i));
                    serial::write_str("Program: ");
                    for (j, t) in k.program.as_slice().iter().enumerate() {
                        if j > 0 { serial::write_str(" → "); }
                        serial::write_str(t.name());
                    }
                    sprintln!();
                } else {
                    sprintln!("Usage: canonical <I-XII>");
                }
            }
            "continuous" => {
                let arg = parts.next().unwrap_or("").trim();
                if let Ok(i) = arg.parse::<usize>() {
                    let idx = i.saturating_sub(1);
                    if k.load_continuous(idx) {
                        sprintln!("Loaded {}: {}", i, continuous_name(idx));
                        serial::write_str("Program: ");
                        for (j, t) in k.program.as_slice().iter().enumerate() {
                            if j > 0 { serial::write_str(" → "); }
                            serial::write_str(t.name());
                        }
                        sprintln!();
                    } else {
                        sprintln!("Continuous program {} not found.", i);
                    }
                } else {
                    sprintln!("Continuous programs:");
                    for i in 0..CONTINUOUS_COUNT {
                        sprintln!("  {}. {}", i + 1, continuous_name(i));
                    }
                    sprintln!("Usage: continuous <1-{}>", CONTINUOUS_COUNT);
                }
            }
            "dynamic" => {
                let arg = parts.next().unwrap_or("").trim();
                match arg {
                    "off" | "disable" => {
                        k.disable_dynamic();
                        sprintln!("Dynamic mode off. Current program unchanged.");
                    }
                    "status" => {
                        sprintln!("Dynamic mode: {}", if k.dynamic_mode { "ON" } else { "OFF" });
                        if let Some(snap) = k.snapshot {
                            let tuple = IgTuple::from_snapshot(&snap);
                            sprintln!("{}", sequence::vote_summary(&tuple));
                        }
                    }
                    _ => {
                        // "dynamic" or "dynamic on" — enable and build first sequence
                        k.load_dynamic();
                        sprintln!("Dynamic mode ON — sequence derived from IgTuple each wrap.");
                        serial::write_str("Program: ");
                        for (j, t) in k.program.as_slice().iter().enumerate() {
                            if j > 0 { serial::write_str(" → "); }
                            serial::write_str(t.name());
                        }
                        sprintln!();
                        if let Some(snap) = k.snapshot {
                            let tuple = IgTuple::from_snapshot(&snap);
                            sprintln!("{}", sequence::vote_summary(&tuple));
                        }
                    }
                }
            }
            "crystal" => {
                let sub = parts.next().unwrap_or("").trim();
                match sub {
                    // μ leg, exposed: map an arbitrary opcode word to its twelve
                    // crystal indices. The generator is δ (type → operational words);
                    // this is μ (words → type verdict). Exposing it makes μ∘δ = id a
                    // MEASUREMENT rather than a claim — round-trip a word and see which
                    // of the twelve axes are recoverable from the sequence alone.
                    "indices" => {
                        let mut prog = crate::tokens::Program::empty();
                        let mut n = 0usize;
                        let mut bad = false;
                        // `parts` is splitn(4,' ') — everything past the third token
                        // arrives as one blob, so split each chunk again.
                        for chunk in parts {
                        for w in chunk.split_whitespace() {
                            let t = match w.trim().to_ascii_uppercase().as_str() {
                                "VINIT" => crate::tokens::Token::Vinit,
                                "TANCH" => crate::tokens::Token::Tanch,
                                "AFWD" => crate::tokens::Token::Afwd,
                                "AREV" => crate::tokens::Token::Arev,
                                "CLINK" => crate::tokens::Token::Clink,
                                "IMSCRIB" => crate::tokens::Token::Imscrib,
                                "FSPLIT" => crate::tokens::Token::Fsplit,
                                "FFUSE" => crate::tokens::Token::Ffuse,
                                "EVALT" => crate::tokens::Token::Evalt,
                                "EVALF" => crate::tokens::Token::Evalf,
                                "ENGAGR" => crate::tokens::Token::Engagr,
                                "IFIX" => crate::tokens::Token::Ifix,
                                other => {
                                    if !other.is_empty() {
                                        sprintln!("crystal indices: unknown opcode '{}'", other);
                                        bad = true;
                                    }
                                    continue;
                                }
                            };
                            prog.push(t);
                            n += 1;
                        }
                        }
                        if bad || n == 0 {
                            if n == 0 && !bad {
                                sprintln!("Usage: crystal indices <OPCODE> <OPCODE> ...");
                            }
                        } else {
                            let snap = crate::kernel::self_imscribe(&prog);
                            let idx = indices_from_program(
                                &prog, snap.frobenius_order, snap.self_ref, snap.dialetheia_complete,
                            );
                            let addr = encode(&idx);
                            serial::write_str("INDICES ");
                            for (i, &v) in idx.iter().enumerate() {
                                if i > 0 { serial::write_str(","); }
                                sprint!("{}", v);
                            }
                            sprintln!(" ADDR {}", addr);
                        }
                    }
                    "store" => {
                        let name = parts.next().unwrap_or("").trim();
                        let data = parts.next().unwrap_or("").trim();
                        if name.is_empty() {
                            sprintln!("Usage: crystal store <name> [data]");
                        } else {
                            let idx = name_hash(name) % CANONICAL_COUNT;
                            k.load_canonical(idx);
                            k.tick();
                            let addr = crystal_store_current(k, &mut cfs, name, data, idx as u8);
                            sprintln!("  -> [{}] tick {}", canonical_name(idx), k.tick_count);
                            sprintln!("Stored '{}' at address {}", name, addr);
                            let decoded = decode(addr);
                            serial::write_str("  Tuple: [");
                            for (i, &v) in decoded.iter().enumerate() {
                                if i > 0 { serial::write_str(","); }
                                sprint!("{}", v);
                            }
                            sprintln!("]");
                        }
                    }
                    "name" => {
                        let name = parts.next().unwrap_or("").trim();
                        if let Some(e) = cfs.read_by_name(name) {
                            sprintln!("Name:    {}", e.name_str());
                            sprintln!("Address: {}", e.address);
                            sprintln!("Data:    {}", e.data_str());
                            sprintln!("Canon:   {}", canonical_name(e.canonical_idx as usize));
                        } else {
                            sprintln!("No entry named '{}'.", name);
                        }
                    }
                    "find" => {
                        sprintln!("{} entries stored:", cfs.count());
                        for e in cfs.iter() {
                            sprintln!("  [{}] {} — {}", e.address, e.name_str(), e.data_str());
                        }
                    }
                    _ => {
                        if let Ok(addr) = sub.parse::<u32>() {
                            let dec = decode(addr);
                            sprintln!("Address: {}", addr);
                            let pnames = ["D","T","R","P","F","K","G","C","Phi","H","S","Omega"];
                            for i in 0..12 { sprintln!("  {}: {}", pnames[i], dec[i]); }
                            if let Some(e) = cfs.read_by_addr(addr) {
                                sprintln!("  Stored: '{}' -> '{}'", e.name_str(), e.data_str());
                            }
                        } else {
                            sprintln!("Usage: crystal <addr> | store | name | find");
                        }
                    }
                }
            }
            "memory" => {
                let start: usize = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(0);
                let count: usize = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(16);
                for i in 0..count {
                    serial::write_str(k.memory.read(start + i).name());
                    serial::write_str(" ");
                }
                sprintln!();
            }
            "registers" => {
                for i in 0..8 {
                    sprint!("R{}:{} ", i, k.registers.read(i).name());
                }
                sprintln!();
            }
            "stack" => {
                sprintln!("Depth: {}", k.stack.depth());
            }
            "list" => {
                sprintln!("╔══════════════════════════════════════════════════════════╗");
                sprintln!("   ALL PROGRAMS  —  12 tokens · 0 control opcodes          ");
                sprintln!("────────────────────────────────────────────────────────────");
                sprintln!("   ▸ CANONICAL (I–XII)  — cyclic graph, 12 core patterns   ");
                sprintln!("────────────────────────────────────────────────────────────");
                for i in 0..CANONICAL_COUNT {
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(i), canonical_name(i));
                }
                sprintln!("────────────────────────────────────────────────────────────");
                sprintln!("   ▸ CONTINUOUS (XIII–XVI)  — token-graph-native loops     ");
                sprintln!("────────────────────────────────────────────────────────────");
                for i in 0..CONTINUOUS_COUNT {
                    let ri = CANONICAL_COUNT + i;
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(ri), continuous_name(i));
                }
                sprintln!("────────────────────────────────────────────────────────────");
                sprintln!("   ▸ NOVEL (XVII–XIX)  — control-flow reconstructions      ");
                sprintln!("────────────────────────────────────────────────────────────");
                for i in 0..NOVEL_COUNT {
                    let ri = CANONICAL_COUNT + CONTINUOUS_COUNT + i;
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(ri), novel_name(i));
                }
                sprintln!("╚══════════════════════════════════════════════════════════╝");
                sprintln!("   ▸ SHUNTED (XX–XXVIII) — branching/exotic compositions        ");
                for i in 0..SHUNTED_COUNT {
                    let ri = i + CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT;
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(ri), shunted_name(i));
                }
                sprintln!("Use 'load <I–XXVIII>' to load any program by Roman numeral.");
            }
            "load" => {
                let arg = parts.next().unwrap_or("").trim();
                if load_by_roman(k, arg) {
                    let idx = roman_to_idx(arg).unwrap();
                    let name: &str = if idx < CANONICAL_COUNT {
                        canonical_name(idx)
                    } else if idx < CANONICAL_COUNT + CONTINUOUS_COUNT {
                        continuous_name(idx - CANONICAL_COUNT)
                    } else if idx < CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT {
                        novel_name(idx - CANONICAL_COUNT - CONTINUOUS_COUNT)
                    } else {
                        shunted_name(idx - CANONICAL_COUNT - CONTINUOUS_COUNT - NOVEL_COUNT)
                    };
                    sprintln!("Loaded {}: {}", arg, name);
                    serial::write_str("Program: ");
                    for (j, t) in k.program.as_slice().iter().enumerate() {
                        if j > 0 { serial::write_str(" → "); }
                        serial::write_str(t.name());
                    }
                    sprintln!();
                } else {
                    sprintln!("Unknown program: {}. Use 'list' to see I–XXVIII.", arg);
                }
            }
            "ruleset" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "show" => {
                        let u = k.active_dialect;
                        let ud = dialect_display(u);
                        let gates = dialect_gates(u);
                        sprintln!("Active ruleset: {} ({})", dialect_name(u), ud);
                        sprintln!("  {}", gates);
                        sprintln!("  Absorbing: ⊙(all) Σ=𐑳(tensor)");
                        if let Some(lim) = k.liminal_target {
                            sprintln!("  ⚠ LIMINAL JUMP PENDING → {} ({}). Use 'seal' to commit.",
                                dialect_display(lim), dialect_name(lim));
                        }
                    }
                    "list" => {
                        sprintln!("╔══════════════════════════════════════════════════════════╗");
                        sprintln!("   ═══ ALL 12 DIALECTS ═══");
                        for u in 0u8..88u8 {
                            let marker = if u == k.active_dialect { "★" } else { " " };
                            sprintln!("  {} {:<3} {:<20} {}     O_∞:{}",
                                marker, dialect_display(u), dialect_name(u),
                                dialect_gates(u), dialect_o_inf(u));
                        }
                        sprintln!("╚══════════════════════════════════════════════════════════╝");
                        if k.liminal_target.is_some() {
                            sprintln!("  ⚠ Liminal jump pending. Use 'seal' to commit or 'jump' again to override.");
                        }
                    }
                    "verify" => {
                        let u = k.active_dialect;
                        // Optional catalog name: "ruleset verify birch_swinnerton_dyer"
                        // checks a named catalog entry's *static* structural tuple instead
                        // of the kernel's own live execution snapshot. Added 2026-06-16
                        // alongside U8 so externally-defined structural types (e.g. the
                        // Clay Millennium problems) can be checked directly, not just
                        // whatever program the kernel happens to be running.
                        let name_arg = parts.next().unwrap_or("").trim();
                        let named_tuple: Option<IgTuple> = if !name_arg.is_empty() {
                            crate::catalog::lookup(name_arg).map(|e| e.tuple)
                        } else {
                            None
                        };
                        let ig_opt: Option<IgTuple> = if !name_arg.is_empty() {
                            if named_tuple.is_none() {
                                sprintln!("Unknown catalog entry: '{}'.", name_arg);
                            }
                            named_tuple
                        } else {
                            k.snapshot.map(|snap| IgTuple::from_snapshot(&snap))
                        };
                        if let Some(ig) = ig_opt {
                            let mut all_pass = true;
                            sprintln!("Ruleset {} ({}) — Gate Verification:", dialect_name(u), dialect_display(u));
                            if !name_arg.is_empty() {
                                sprintln!("  Catalog entry: {}  tuple: {}", name_arg, ig.display());
                            } else {
                                sprintln!("  Self-imscription: {}", ig.display());
                            }

                            match u {
                                0 => { // canonical: G1:Φ≥𐑹  G2:φ̂≥⊙  G3:Ω≥𐑭
                                    let g1 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    let g2 = (ig.phi as u8) <= (IgPrim::Phi_crit as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    sprintln!("  G1 (Φ≥𐑹): {}  Φ={}", if g1 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G2 (φ̂≥⊙): {}  φ̂={}", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                1 => { // low_gate: G1:Φ≥𐑬  G2:φ̂≥𐑢  G3:Ω≥𐑭
                                    let g1 = (ig.p as u8) <= (IgPrim::P_pm as u8);
                                    let g2 = (ig.phi as u8) <= (IgPrim::𐑢 as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    sprintln!("  G1 (Φ≥𐑬): {}  Φ={}", if g1 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G2 (φ̂≥𐑢): {}  φ̂={}", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                2 => { // strict_frobenius: G1:ƒ≥𐑐  G2:Φ≥𐑹  G3:Ω≥𐑭
                                    let g1 = (ig.f as u8) <= (IgPrim::F_hbar as u8);
                                    let g2 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    sprintln!("  G1 (ƒ≥𐑐): {}  ƒ={}", if g1 {"PASS"} else {"FAIL"}, ig.f.glyph());
                                    sprintln!("  G2 (Φ≥𐑹): {}  Φ={}", if g2 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                3 => { // inverted_gates: G1:φ̂≥⊙  G2:Φ≥𐑹  G3:Ω≥𐑭
                                    let g1 = (ig.phi as u8) <= (IgPrim::Phi_crit as u8);
                                    let g2 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    sprintln!("  G1 (φ̂≥⊙): {}  φ̂={}", if g1 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G2 (Φ≥𐑹): {}  Φ={}", if g2 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                4 => { // no_ordering: G1+G2+G3 parallel — same as canonical but independence asserted
                                    let g1 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    let g2 = (ig.phi as u8) <= (IgPrim::Phi_crit as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    sprintln!("  G1 (Φ≥𐑹): {}  Φ={}", if g1 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G2 (φ̂≥⊙): {}  φ̂={}", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    sprintln!("  Mode: PARALLEL — gates evaluated independently.");
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                5 => { // high_gate: G1:Φ≥𐑹  G2:φ̂≥𐑮  G3:Ω≥𐑟
                                    let g1 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    let g2 = (ig.phi as u8) <= (IgPrim::𐑮 as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_na as u8);
                                    sprintln!("  G1 (Φ≥𐑹): {}  Φ={}", if g1 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G2 (φ̂≥𐑮): {}  φ̂={}", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G3 (Ω≥𐑟): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                6 => { // winding_first: G1:Ω≥𐑭  G2:φ̂≥⊙  G3:Φ≥𐑹
                                    let g1 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    let g2 = (ig.phi as u8) <= (IgPrim::Phi_crit as u8);
                                    let g3 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    sprintln!("  G1 (Ω≥𐑭): {}  Ω={}", if g1 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    sprintln!("  G2 (φ̂≥⊙): {}  φ̂={}", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G3 (Φ≥𐑹): {}  Φ={}", if g3 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                }
                                7 => { // t_structural: G1:Φ≥𐑹  G2:φ̂≥⊙  G3:Ω≥𐑭  T:ɢ=𐑠
                                    let g1 = (ig.p as u8) <= (IgPrim::P_pmsym as u8);
                                    let g2 = (ig.phi as u8) <= (IgPrim::Phi_crit as u8);
                                    let g3 = (ig.omega as u8) <= (IgPrim::Omega_z as u8);
                                    let t_ok = ig.c == IgPrim::C_seq;
                                    sprintln!("  G1 (Φ≥𐑹): {}  Φ={}", if g1 {"PASS"} else {"FAIL"}, ig.p.glyph());
                                    sprintln!("  G2 (φ̂≥⊙): {}  φ̂={}", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={}", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph());
                                    sprintln!("  T  (ɢ=𐑠): {}  ɢ={}", if t_ok {"PASS"} else {"FAIL"}, ig.c.glyph());
                                    if !g1 || !g2 || !g3 || !t_ok { all_pass = false; }
                                }
                                8 => { // chirality_first: G1:Ħ≥𐑖  G2:⊙≥⊙  G3:Ω≥𐑭
                                       // T: T_CEILING — see manuscripts/clay_cross_dialect_closure.md.
                                       // Uses IgPrim::ordinal(), NOT raw discriminant comparison — the
                                       // discriminant trick used in arms 0-7 is invalid for the criticality
                                       // family (𐑮/𐑻 are non-monotonic in discriminant order).
                                    let g1 = ig.h.ordinal() >= IgPrim::H2.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal();
                                    let g3 = ig.omega.ordinal() >= IgPrim::Omega_z.ordinal();
                                    sprintln!("  G1 (Ħ≥𐑖): {}  Ħ={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.h.glyph(), ig.h.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph(), ig.omega.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_check(&ig) { all_pass = false; }
                                }
                                9 => { // scope_dialect: G1:Γ≥𐑲(maximal scope)  G2:⊙≥⊙  G3:Ω≥𐑭
                                       // T: T_CEILING — same generalization as U8, paired with a different gate spec.
                                    let g1 = ig.g.ordinal() >= IgPrim::G_aleph.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal();
                                    let g3 = ig.omega.ordinal() >= IgPrim::Omega_z.ordinal();
                                    sprintln!("  G1 (Γ≥𐑲): {}  Γ={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.g.glyph(), ig.g.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (Ω≥𐑭): {}  Ω={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph(), ig.omega.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_check(&ig) { all_pass = false; }
                                }
                                10 => { // triple_criticality: G1/G2/G3 all on ⊙, escalating thresholds 𐑢/⊙/𐑣
                                    let g1 = ig.phi.ordinal() >= IgPrim::𐑢.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal();
                                    let g3 = ig.phi.ordinal() >= IgPrim::Phi_super.ordinal();
                                    sprintln!("  G1 (⊙≥𐑢): {}  ⊙={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (⊙≥𐑣): {}  ⊙={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_check(&ig) { all_pass = false; }
                                }
                                11 => { // triple_criticality_gapped: same gates as U10, T_CEILING(gapped)
                                    let g1 = ig.phi.ordinal() >= IgPrim::𐑢.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal();
                                    let g3 = ig.phi.ordinal() >= IgPrim::Phi_super.ordinal();
                                    sprintln!("  G1 (⊙≥𐑢): {}  ⊙={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (⊙≥𐑣): {}  ⊙={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_gapped_check(&ig) { all_pass = false; }
                                }
                                _ => {
                                    // Dynamic gate evaluation for expansion dialects (12–87).
                                    if crate::dialect::is_hand_crafted(u) {
                                        sprintln!("  Unknown dialect — cannot verify.");
                                        all_pass = false;
                                    } else {
                                        let unis = crate::dialect_expansion::all_dialects();
                                        let uni = &unis[u as usize];
                                        let (g1_ok, g1_ord, g1_glyph) = crate::dialect::eval_gate_spec(&uni.g1, &ig);
                                        let (g2_ok, g2_ord, g2_glyph) = crate::dialect::eval_gate_spec(&uni.g2, &ig);
                                        let (g3_ok, g3_ord, g3_glyph) = crate::dialect::eval_gate_spec(&uni.g3, &ig);
                                        let g1_label = crate::dialect::gate_prim_label(uni.g1.prim);
                                        let g2_label = crate::dialect::gate_prim_label(uni.g2.prim);
                                        let g3_label = crate::dialect::gate_prim_label(uni.g3.prim);
                                        sprintln!("  G1 ({}≥{}): {}  {}={} (ord {})",
                                            g1_label, uni.g1.min_ord, if g1_ok {"PASS"} else {"FAIL"},
                                            g1_label, g1_glyph, g1_ord);
                                        sprintln!("  G2 ({}≥{}): {}  {}={} (ord {})",
                                            g2_label, uni.g2.min_ord, if g2_ok {"PASS"} else {"FAIL"},
                                            g2_label, g2_glyph, g2_ord);
                                        sprintln!("  G3 ({}≥{}): {}  {}={} (ord {})",
                                            g3_label, uni.g3.min_ord, if g3_ok {"PASS"} else {"FAIL"},
                                            g3_label, g3_glyph, g3_ord);
                                        sprintln!("  Ordering: {}",
                                            if uni.gate_ordering {"SEQUENTIAL"} else {"PARALLEL"});
                                        if !g1_ok || !g2_ok || !g3_ok { all_pass = false; }
                                    }
                                }
                            }

                            if all_pass {
                                sprintln!("  Result: ALL GATES PASS — ruleset satisfied.");
                            } else {
                                sprintln!("  Result: VIOLATION — fails ruleset gate(s).");
                                sprintln!("  Tip: load a different program/entry or jump to a compatible dialect.");
                            }
                        } else if name_arg.is_empty() {
                            sprintln!("No snapshot — tick first to generate a self-imscription.");
                            sprintln!("  (or: 'ruleset verify <catalog_name>' to check a named entry instead)");
                        }
                    }
                    "dialetheic" => {
                        // ruleset dialetheic <name> <alt_dialect>
                        // Decomposes the closure question into GATE and T components
                        // and FFUSEs each separately (plus the combined verdict), through
                        // the kernel's actual FFUSE primitive (Belnap join) — not a
                        // shortcut. join(T,F)=B: designated, dialetheic, not flatly
                        // false. See manuscripts/clay_cross_dialect_closure.md for what
                        // this is and is not — it does NOT make the entry true under
                        // canonical. Decomposing matters: gate and T can disagree on
                        // whether there's a real conflict (see Yang-Mills under U10).
                        use crate::belnap::B4;
                        use parasm::ParaVM;

                        fn fuse(vm: &mut ParaVM, a: B4, b: B4) -> B4 {
                            vm.set_belief(1, a);
                            vm.set_belief(2, b);
                            vm.load("FFUSE %r1 %r2 %r0\nHALT").unwrap();
                            vm.run(None);
                            vm.belief_of(0)
                        }

                        fn split(vm: &mut ParaVM, fused: B4) -> (B4, B4) {
                            vm.set_belief(0, fused);
                            vm.load("FSPLIT %r0 %r3 %r4\nHALT").unwrap();
                            vm.run(None);
                            (vm.belief_of(3), vm.belief_of(4))
                        }

                        // Real round-trip check, against the ACTUAL originals (a, b) —
                        // not just "did it come out as (T,F)". FSPLIT(B) is a FIXED
                        // decomposition (always emits (T,F) on its two destinations,
                        // regardless of what produced the B), so this only matches when
                        // the true inputs already were exactly {T,F}. Returns the fused
                        // value, the split-back pair, and whether it exactly reproduces
                        // the original (order-insensitive — FSPLIT doesn't preserve
                        // which side was which either).
                        fn fuse_and_check(vm: &mut ParaVM, a: B4, b: B4) -> (B4, B4, B4, bool) {
                            let fused = fuse(vm, a, b);
                            let (d1, d2) = split(vm, fused);
                            let recovered = (d1 == a && d2 == b) || (d1 == b && d2 == a);
                            (fused, d1, d2, recovered)
                        }

                        let dname = parts.next().unwrap_or("").trim();
                        let alt_str = parts.next().unwrap_or("").trim();
                        let alt: u8 = match alt_str.parse() {
                            Ok(v) => v,
                            _ => {
                                sprintln!("Usage: ruleset dialetheic <catalog_name> <alt_dialect 8|9|10|11>");
                                return;
                            }
                        };
                        let entry = match crate::catalog::lookup(dname) {
                            Some(e) => e,
                            None => { sprintln!("Unknown catalog entry: '{}'.", dname); return; }
                        };
                        let ig = entry.tuple;

                        // Canonical (U0) gate verdict, ordinal-correct.
                        let gate_canon =
                            ig.p.ordinal()     >= IgPrim::P_pmsym.ordinal()
                            && ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal()
                            && ig.omega.ordinal() >= IgPrim::Omega_z.ordinal();
                        let t_canon = t_canonical_check_silent(&ig);

                        // Alt-dialect gate verdict: only U8/U9/U10/U11 wired up so far.
                        // U8/U9/U10 use T_CEILING for their T side; U11 uses the
                        // gapped variant (raises only the Ç anchor — see dialect.rs).
                        let gate_alt = match alt {
                            8 => ig.h.ordinal() >= IgPrim::H2.ordinal()
                                && ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal()
                                && ig.omega.ordinal() >= IgPrim::Omega_z.ordinal(),
                            9 => ig.g.ordinal() >= IgPrim::G_aleph.ordinal()
                                && ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal()
                                && ig.omega.ordinal() >= IgPrim::Omega_z.ordinal(),
                            10 | 11 => ig.phi.ordinal() >= IgPrim::𐑢.ordinal()
                                && ig.phi.ordinal() >= IgPrim::Phi_crit.ordinal()
                                && ig.phi.ordinal() >= IgPrim::Phi_super.ordinal(),
                            _ => {
                                sprintln!("Only U8, U9, U10, U11 have a known closing verdict so far.");
                                return;
                            }
                        };
                        let t_alt = if alt == 11 {
                            t_ceiling_gapped_check_silent(&ig)
                        } else {
                            t_ceiling_check_silent(&ig)
                        };

                        let gc = if gate_canon {B4::T} else {B4::F};
                        let ga = if gate_alt   {B4::T} else {B4::F};
                        let tc = if t_canon    {B4::T} else {B4::F};
                        let ta = if t_alt      {B4::T} else {B4::F};
                        let oc = if gate_canon && t_canon {B4::T} else {B4::F};
                        let oa = if gate_alt   && t_alt   {B4::T} else {B4::F};

                        let mut vm = ParaVM::new();
                        let (gate_fused, gd1, gd2, gate_ok)    = fuse_and_check(&mut vm, ga, gc);
                        let (t_fused, td1, td2, t_ok)          = fuse_and_check(&mut vm, ta, tc);
                        let (overall_fused, od1, od2, ov_ok)   = fuse_and_check(&mut vm, oa, oc);

                        sprintln!("Dialetheic bridge: {} — U₀ (canonical) vs U{}", dname, alt);
                        sprintln!("  GATE     canon={} alt={}  FFUSE->{}  FSPLIT->({},{})  recovered={}", gc.name(), ga.name(), gate_fused.name(), gd1.name(), gd2.name(), gate_ok);
                        sprintln!("  T        canon={} alt={}  FFUSE->{}  FSPLIT->({},{})  recovered={}", tc.name(), ta.name(), t_fused.name(), td1.name(), td2.name(), t_ok);
                        sprintln!("  OVERALL  canon={} alt={}  FFUSE->{}  FSPLIT->({},{})  recovered={}", oc.name(), oa.name(), overall_fused.name(), od1.name(), od2.name(), ov_ok);
                        for (label, f, ok) in [("GATE", gate_fused, gate_ok), ("T", t_fused, t_ok), ("OVERALL", overall_fused, ov_ok)] {
                            match f {
                                B4::B => sprintln!("  {}: designated, dialetheic — canon's F conflicts with real T-evidence from U{}. Round-trip lossless: {}.", label, alt, ok),
                                B4::T => sprintln!("  {}: no conflict — passes everywhere checked.", label),
                                _     => sprintln!("  {}: no conflict — fails everywhere checked, no dialetheic upgrade.", label),
                            }
                        }

                        // Prove the lossy case too, not just claim it: fuse (B, T) —
                        // a value that's already a paradox plus a clean T — and show
                        // the split-back does NOT recover (B, T).
                        let (leak_fused, ld1, ld2, leak_ok) = fuse_and_check(&mut vm, B4::B, B4::T);
                        sprintln!("  LEAK-CHECK  FFUSE(B,T)->{}  FSPLIT->({},{})  recovered={}", leak_fused.name(), ld1.name(), ld2.name(), leak_ok);
                        if !leak_ok {
                            sprintln!("  -> Confirmed: feeding an already-paradoxical input (B) loses information.");
                            sprintln!("     The original B is gone; FSPLIT(B) only ever hands back a plain (T,F).");
                        }
                    }
                    _ => sprintln!("ruleset <show|list|verify|dialetheic> [catalog_name] [alt_dialect]"),
                }
            }
            "jump" => {
                let rest: alloc::string::String = parts.collect::<alloc::vec::Vec<&str>>().join(" ");
                handle_jump(k, &rest);
            }
            "seal" => {
                if let Some(target) = k.liminal_target {
                    k.active_dialect = target;
                    let name = dialect_name(target);
                    let ud = dialect_display(target);
                    k.liminal_target = None;
                    k.liminal_compound = None;
                    sprintln!("IFIX — ruleset committed. Kernel now operates under {} ({}) permanently.",
                        name, ud);
                    sprintln!("  {}", dialect_gates(target));
                    sprintln!("  Description: {}", dialect_description(target));
                } else {
                    sprintln!("No liminal jump to seal. Use 'jump <U> using <compound>' first.");
                }
            }
            "absorb_test" => {
                let a = parts.next().unwrap_or("?");
                let b = parts.next().unwrap_or("?");
                let prim = parts.next().unwrap_or("?");
                let op = parts.next().unwrap_or("?");
                sprintln!("absorb_test({}, {}, {}, {}) under canonical U₀", a, b, prim, op);
                sprintln!("  Canonical: ⊙ absorbs under all ops. See cross-dialect doc for U₁–U₇.");
            }
            "whoami" => {
                let flag = parts.next().unwrap_or("");
                if flag == "--ruleset" {
                    if let Some(snap) = k.snapshot {
                        let ig = IgTuple::from_snapshot(&snap);
                        sprintln!("Self-imscription (canonical U₀): {}", ig.display());
                    } else {
                        sprintln!("No snapshot — tick first.");
                    }
                } else if flag == "--frobenius" {
                    sprintln!("{}", crate::frobenius_unify::formatted_report());
                } else {
                    sprintln!("Usage: whoami --ruleset | --frobenius");
                }
            }
            "absorption" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "show" => sprintln!("Absorption rules (canonical U₀):\n  ⊙ absorbs under all ops\n  Σ=𐑳 absorbs under tensor"),
                    _ => sprintln!("absorption show  → list all absorption rules"),
                }
            }
            "tstatus" => sprintln!("T-constitution (canonical U₀): ø (no constitution) — all primitives pass."),
            "compound" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "list" => {
                        sprintln!("╔══════════════════════════════════════════════════════════════╗");
                        sprintln!("   11 DIASCHIZIC COMPOUNDS  —  dialect-steering agents       ");
                        sprintln!("──────────────────────────────────────────────────────────────");
                        for i in 0..COMPOUND_COUNT {
                            let p = compound_program(i);
                            let tok_count = p.map(|pr| pr.len()).unwrap_or(0);
                            let tier = match i {
                                0|8 => "O_∞", 2|6|9 => "O₂", 10 => "O₂†",
                                4 => "O₁", _ => "O₀"
                            };
                            sprintln!("   {:<12} {:<4} {:<40} {} tok",
                                compound_name(i), tier,
                                match i {
                                    0 => "Non-Abelian EP braid",
                                    1 => "Supercritical catalyst",
                                    2 => "Adjoint corridor",
                                    3 => "EP core toggle",
                                    4 => "Local-net trap",
                                    5 => "MBL freeze key",
                                    6 => "Disjunctive fork",
                                    7 => "Absolute point (d=0)",
                                    8 => "Perfect mirror",
                                    9 => "Deep resonator",
                                    _ => "Threshold-crosser",
                                },
                                tok_count);
                        }
                        sprintln!("╚══════════════════════════════════════════════════════════════╝");
                    }
                    "show" => {
                        let name = parts.next().unwrap_or("");
                        if let Some(idx) = compound_index(name) {
                            if let Some(prog) = compound_program(idx) {
                                sprintln!("Compound: {} (idx {})", compound_name(idx as usize), idx);
                                sprintln!("  Tier: {}", match idx {
                                    0|8 => "O_∞", 2|6|9 => "O₂", 10 => "O₂†",
                                    4 => "O₁", _ => "O₀"
                                });
                                sprintln!("  Tokens: {}", prog.len());
                                serial::write_str("  Program: ");
                                for (j, t) in prog.as_slice().iter().enumerate() {
                                    if j > 0 { serial::write_str(" → "); }
                                    serial::write_str(t.name());
                                }
                                sprintln!();
                            } else {
                                sprintln!("Internal error: compound program not found.");
                            }
                        } else {
                            sprintln!("Unknown compound: '{}'. Use 'compound list'.", name);
                        }
                    }
                    "load" => {
                        let name = parts.next().unwrap_or("");
                        if let Some(idx) = compound_index(name) {
                            if k.load_compound(idx) {
                                sprintln!("Loaded compound: {} ({} tokens, tier {})",
                                    compound_name(idx as usize), k.program.len(),
                                    match idx {
                                        0|8 => "O_∞", 2|6|9 => "O₂", 10 => "O₂†",
                                        4 => "O₁", _ => "O₀"
                                    });
                                sprintln!("  Run with 'tick' or 'run'. Seal with 'seal' after liminal jumps.");
                            } else {
                                sprintln!("Internal error: compound program not found.");
                            }
                        } else {
                            sprintln!("Unknown compound: '{}'. Use 'compound list'.", name);
                        }
                    }
                    _ => sprintln!("compound <list|show <name>|load <name>>"),
                }
            }
            "" => {}
            _ => {
                // Context-aware subcommand dispatch: if we're inside a context
                // (Rebis, Dialect, etc.) and cmd isn't a top-level command,
                // try dispatching as a subcommand of the current context.
                // E.g., 'translate ATGGCC' in Rebis → treated as 'rebis translate ATGGCC'
                let ctx_dispatch = ctx_stack.current().and_then(|ctx| {
                    let ctx_name = ctx.name.to_lowercase();
                    match ctx_name.as_str() {
                        "rebis" => {
                            // Remaining tokens from parts iterator become sub/arg/rest
                            let sub = cmd;
                            let arg = parts.next().unwrap_or("");
                            let r: alloc::string::String = parts.collect::<alloc::vec::Vec<&str>>().join(" ");
                            print_rebis(sub, arg, &r);
                            Some(())
                        }
                        _ => None,
                    }
                });
                if ctx_dispatch.is_none() {
                    sprintln!("Unknown: {}. Type 'help'.", cmd);
                }
            },
        }
    }
}

fn crystal_store_current(
    k: &mut Kernel,
    cfs: &mut CrystalStore,
    name: &str,
    data: &str,
    canonical_idx: u8,
) -> u32 {
    if let Some(snap) = k.snapshot {
        let indices = indices_from_program(
            &k.program,
            snap.frobenius_order,
            snap.self_ref,
            snap.dialetheia_complete,
        );
        let addr = encode(&indices);
        cfs.store(name, data, addr, canonical_idx)
    } else {
        0
    }
}

// ─── Input ────────────────────────────────────────────────────

fn read_line<'a>(buf: &'a mut [u8], history: &mut History, ctx: &ContextStack) -> &'a str {
    let mut len = 0usize;
    let mut hist_pos = 0usize;
    let max_len = buf.len().saturating_sub(1);
    let _tab_hits: [u8; 16] = [0; 16];  // cycling completions

    loop {
        let b = serial::read_byte();
        match b {
            // Tab completion
            0x09 => {
                if len == 0 { continue; }
                // Get current word
                let line_str = core::str::from_utf8(&buf[..len]).unwrap_or("");
                if let Some(completion) = tab_complete(line_str, ctx) {
                    // Replace buffer with completion
                    let comp_bytes = completion.as_bytes();
                    let n = comp_bytes.len().min(max_len);
                    buf[..n].copy_from_slice(&comp_bytes[..n]);
                    len = n;
                    // Redraw
                    serial::write_str("\r\x1b[K");
                    render_prompt(ctx);
                    if let Ok(s) = core::str::from_utf8(&buf[..n]) {
                        serial::write_str(s);
                    }
                }
            }
            0x1b => {
                let b2 = serial::read_byte();
                if b2 == b'O' {
                    // F1-F4: OP, OQ, OR, OS
                    match serial::read_byte() {
                        b'P' => { buf[0] = b':'; buf[1] = b'1'; len = 2; break; }
                        b'Q' => { buf[0] = b':'; buf[1] = b'2'; len = 2; break; }
                        b'R' => { buf[0] = b':'; buf[1] = b'3'; len = 2; break; }
                        b'S' => { buf[0] = b':'; buf[1] = b'4'; len = 2; break; }
                        _ => {}
                    }
                    continue;
                }
                if b2 != b'[' { continue; }
                let b3 = serial::read_byte();
                // Arrow keys: ESC [ A / ESC [ B
                if b3 == b'A' {
                    let next = (hist_pos + 1).min(history.count);
                    if next != hist_pos {
                        hist_pos = next;
                        if let Some((bytes, n)) = history.get(hist_pos) {
                            redraw_input(len, bytes, n, buf);
                            len = n;
                        }
                    }
                } else if b3 == b'B' {
                    if hist_pos > 0 {
                        hist_pos -= 1;
                        if hist_pos == 0 {
                            redraw_input(len, &[], 0, buf);
                            len = 0;
                        } else if let Some((bytes, n)) = history.get(hist_pos) {
                            redraw_input(len, bytes, n, buf);
                            len = n;
                        }
                    }
                } else if b3 == b'1' || b3 == b'2' || b3 == b'3' || b3 == b'4' {
                    // F-keys: ESC [ nn~  (e.g. F7 = ESC [ 1 8 ~)
                    let b4 = serial::read_byte();
                    if b4 == b'~' {
                        // Single digit: ESC [ 1~ = Home, 2~ = Insert, 3~ = Delete, 4~ = End
                    } else {
                        let b5 = serial::read_byte();
                        if b5 == b'~' {
                            // Two-digit sequence: ESC [ nn ~
                            let fkey = (b3 - b'0') * 10 + (b4 - b'0');
                            let cat: u8 = match fkey {
                                11..=14 => fkey - 10,  // F1-F4: 1-4
                                15 => 5,   // F5
                                17 => 6,   // F6
                                18 => 7,   // F7
                                19 => 8,   // F8
                                20 => 9,   // F9
                                21 => 10,  // F10
                                23 => 11,  // F11
                                24 => 12,  // F12
                                _ => 0,
                            };
                            if cat >= 1 && cat <= 9 {
                                buf[0] = b':';
                                buf[1] = b'0' + cat;
                                len = 2;
                                break;
                            }
                        }
                        // Three-digit sequences (ESC [ 2 4 ~ = F12 etc.) already covered above;
                        // if a third digit appears, consume until ~
                    }
                }
            }
            b'\r' | b'\n' => {
                serial::write_str("\n");
                history.push(&buf[..len]);
                break;
            }
            0x7f | 0x08 => {
                if len > 0 {
                    len -= 1;
                    serial::write_str("\x08 \x08");
                }
            }
            0x03 => {
                sprintln!();
                len = 0;
                break;
            }
            b if b >= 0x20 => {
                if len < max_len {
                    buf[len] = b;
                    len += 1;
                    serial::write_byte(b);
                }
            }
            _ => {}
        }
    }
    buf[len] = 0;
    core::str::from_utf8(&buf[..len]).unwrap_or("")
}

fn redraw_input(old_len: usize, src: &[u8], src_len: usize, buf: &mut [u8]) {
    let _ = old_len;
    serial::write_str("\r\x1b[K");
    let max_len = buf.len().saturating_sub(1);
    let n = src_len.min(max_len).min(src.len());
    buf[..n].copy_from_slice(&src[..n]);
    if let Ok(s) = core::str::from_utf8(&buf[..n]) {
        serial::write_str(s);
    }
}

// ─── T_CEILING — shared T-constitution check for U8/U9 ─────────
//
// Ceiling-generalizes canonical's existing Ç-only ceiling rule to all five
// dynamics primitives, same anchors: Φ<=𐑹 ƒ<=𐑐 Ç<=𐑧 Ħ<=𐑫 Ω<=𐑭.
// See manuscripts/clay_cross_dialect_closure.md for the derivation. Uses
// IgPrim::ordinal(), not raw discriminant comparison.
// Canonical's actual T-constitution (exact-equality on four primitives,
// ceiling on Ç only) — matches Python's _T_CANONICAL exactly. This is the
// real canonical T-verdict, distinct from T_CEILING (which only applies
// to U8/U9/U10/U11).
fn t_canonical_check_silent(ig: &IgTuple) -> bool {
    ig.p.ordinal()     == IgPrim::P_pmsym.ordinal()
    && ig.f.ordinal()   == IgPrim::F_hbar.ordinal()
    && ig.k.ordinal()   <= IgPrim::K_slow.ordinal()
    && ig.h.ordinal()   == IgPrim::H_inf.ordinal()
    && ig.omega.ordinal() == IgPrim::Omega_z.ordinal()
}

fn t_ceiling_check_silent(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::P_pmsym.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::F_hbar.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::K_slow.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::H_inf.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::Omega_z.ordinal();
    t_phi && t_f && t_k && t_h && t_om
}

// U11 only: same as T_CEILING, but Ç's ceiling is raised from 𐑧 (K_slow,
// ord 3) to 𐑪 (K_trap, ord 4) — a gapped/trapped spectrum, not just a slow
// one. Motivated, not tailored: see dialect.rs's U11 comment block.
fn t_ceiling_gapped_check_silent(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::P_pmsym.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::F_hbar.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::K_trap.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::H_inf.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::Omega_z.ordinal();
    t_phi && t_f && t_k && t_h && t_om
}

fn t_ceiling_check(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::P_pmsym.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::F_hbar.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::K_slow.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::H_inf.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::Omega_z.ordinal();
    let t_ok = t_phi && t_f && t_k && t_h && t_om;
    sprintln!("  T_CEILING Φ<=𐑹: {}  ƒ<=𐑐: {}  Ç<=𐑧: {}  Ħ<=𐑫: {}  Ω<=𐑭: {}",
        if t_phi {"PASS"} else {"FAIL"}, if t_f {"PASS"} else {"FAIL"},
        if t_k {"PASS"} else {"FAIL"}, if t_h {"PASS"} else {"FAIL"},
        if t_om {"PASS"} else {"FAIL"});
    sprintln!("  T_CEILING overall: {}", if t_ok {"PASS"} else {"FAIL"});
    t_ok
}

fn t_ceiling_gapped_check(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::P_pmsym.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::F_hbar.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::K_trap.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::H_inf.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::Omega_z.ordinal();
    let t_ok = t_phi && t_f && t_k && t_h && t_om;
    sprintln!("  T_CEILING(gapped) Φ<=𐑹: {}  ƒ<=𐑐: {}  Ç<=𐑪: {}  Ħ<=𐑫: {}  Ω<=𐑭: {}",
        if t_phi {"PASS"} else {"FAIL"}, if t_f {"PASS"} else {"FAIL"},
        if t_k {"PASS"} else {"FAIL"}, if t_h {"PASS"} else {"FAIL"},
        if t_om {"PASS"} else {"FAIL"});
    sprintln!("  T_CEILING(gapped) overall: {}", if t_ok {"PASS"} else {"FAIL"});
    t_ok
}

// ─── Cross-Dialect Jump Handler ─────────────────────────────

fn handle_jump(k: &mut Kernel, rest: &str) {
    let rest = rest.trim();
    if rest.is_empty() {
        sprintln!("Usage: jump <U> using <compound> [--liminal]");
        sprintln!("       jump <U> via <V> using <c1> <c2> [--liminal]");
        sprintln!("  <U> = U_0..U_11 or U₀..U₁₁");
        sprintln!("  <compound> = Apertix, Diabaton, Bifrons, ... (see 'compound list')");
        return;
    }

    let liminal = rest.contains("--liminal");
    let rest_no_flag = if liminal {
        // Slice out "--liminal" by working &str -> &str
        rest.replace("--liminal", "").replace("  ", " ")
    } else {
        alloc::string::String::from(rest)
    };
    let rest_clean: &str = rest_no_flag.as_str();

    // Check for " via " syntax
    let via_pos = rest_clean.find(" via ");

    // Split on " using "
    let using_pos = rest_clean.find(" using ");
    if using_pos.is_none() {
        sprintln!("Expected: jump <U> using <compound> [--liminal]");
        sprintln!("  <U> = U_0 through U_11 (or U₀ through U₁₁)");
        sprintln!("  <compound> = Apertix, Diabaton, Bifrons, ... (see 'compound list')");
        return;
    }
    let using_pos = using_pos.unwrap();

    // Extract dialect part (before " using " or " via ")
    let u_str: &str;
    let compound_str: &str;
    let via_str: Option<&str>;

    if let Some(vp) = via_pos {
        if vp < using_pos {
            // "U_4 via U_3 using Apertix Diabaton"
            u_str = rest_clean[..vp].trim();
            via_str = Some(rest_clean[vp + 5..using_pos].trim());
        } else {
            // "U_4 using Apertix via U_3" — odd but handle
            u_str = rest_clean[..using_pos].trim();
            via_str = Some(rest_clean[vp + 5..].trim());
        }
    } else {
        u_str = rest_clean[..using_pos].trim();
        via_str = None;
    }
    compound_str = rest_clean[using_pos + 7..].trim();

    // Parse dialect
    let target: u8 = match parse_dialect(u_str) {
        Some(u) if u <= 87 => u,
        _ => {
            sprintln!("Unknown dialect: '{}'. Use U_0 through U_11 (or U₀ through U₁₁).", u_str);
            return;
        }
    };

    // Parse via dialect
    let intermediate: Option<u8> = via_str.and_then(|v| {
        let v = v.trim();
        if v.is_empty() { None } else { parse_dialect(v) }
    });

    // Parse compounds (space-separated after "using")
    let mut compound_iter = compound_str.split_whitespace();
    let c1_name: &str = compound_iter.next().unwrap_or("");
    let c1: u8 = match compound_index(c1_name) {
        Some(idx) => idx as u8,
        None => {
            sprintln!("Unknown compound: '{}'", c1_name);
            sprintln!("  Valid: Verticullum, Chimerium, Apertix, Praxeum,");
            sprintln!("         Retiarius, Frigorix, Bifrons, Punctum,");
            sprintln!("         Syndexios, Katachthon, Diabaton");
            return;
        }
    };
    let c2_name: &str = compound_iter.next().unwrap_or("");
    let c2: Option<u8> = if c2_name.is_empty() { None } else { compound_index(c2_name).map(|i| i as u8) };

    // Display the jump
    sprintln!("*** CROSS-DIALECT JUMP: {} using {}", dialect_display(target), compound_name(c1 as usize));
    if let Some(v) = intermediate {
        sprintln!("    via {}", dialect_display(v));
    }
    if let Some(idx) = c2 {
        sprintln!("    second compound: {} ({} tokens, tier {})", compound_name(idx as usize), compound_program(idx as usize).map(|p| p.len() as u8).unwrap_or(0), match idx { 0|8 => "O_inf", 2|6|9 => "O_2", 10 => "O_2_dagger", 4 => "O_1", _ => "O_0" });
    }
    sprintln!("    [RULESET_HEADER] → [COMPOUND_PROGRAM] → [IFIX_SEAL]");
    sprintln!("    Compound: {} | tier: {} | tokens: {}", compound_name(c1 as usize), match c1 { 0|8 => "O_inf", 2|6|9 => "O_2", 10 => "O_2_dagger", 4 => "O_1", _ => "O_0" }, compound_program(c1 as usize).map(|p| p.len() as u8).unwrap_or(0));

    // Set liminal state
    k.liminal_target = Some(target);
    k.liminal_compound = Some(c1);

    if liminal {
        sprintln!("    ⚠ LIMINAL MODE: jump is active but NOT sealed.");
        sprintln!("      Probe the dialect. Use 'seal' to commit or jump again to override.");
    } else {
        sprintln!("    Jump staged. Type 'seal' to commit to {} permanently.", dialect_display(target));
        sprintln!("    (Use 'jump ... --liminal' to probe without requiring seal.)");
    }
}

// ─── Helpers ──────────────────────────────────────────────────

fn print_help() {
    sprintln!("mOMonadOS REPL commands:");
    sprintln!();
    sprintln!("══ Execution ══");
    sprintln!("  {:<30} — run N manual ticks (default 1)", "tick [N]");
    sprintln!("  {:<30} — run N ticks; no arg = continuous (ESC to stop)", "run [N]");
    sprintln!("  {:<30} — live terminal HUD, refresh every N ticks (ESC to stop)", "watch [N]");
    sprintln!("  {:<30} — run N ticks, one per PIT interrupt (ESC to stop)", "timer [N]");
    sprintln!("  {:<30} — load any program + run continuously", "boot <I–XXVIII>");
    sprintln!("  {:<30} — load any program by Roman numeral", "load <I–XXVIII>");
    sprintln!();
    sprintln!("══ Status ══");
    sprintln!("  {:<30} — kernel status (tick, IP, stack, fork, frob, halted)", "status");
    sprintln!("  {:<30} — show loaded program + fork depth", "program");
    sprintln!("  {:<30} — structural snapshot (sig, tier, period, dialeth, ...)", "snapshot");
    sprintln!("  {:<30} — ASCII-art token graph with nesting", "graph");
    sprintln!("  {:<30} — B4 memory heatmap with color blocks", "heatmap [start] [n]");
    sprintln!("  {:<30} — dump B4 memory", "memory [start] [n]");
    sprintln!("  {:<30} — show R0-R7", "registers");
    sprintln!("  {:<30} — Ħ hop: read snapshot through the R1↔R2 mirror", "arev [test]");
    sprintln!("  {:<30} — stack depth", "stack");
    sprintln!();
    sprintln!("══ Program Loading ══");
    sprintln!("  {:<30} — list all programs (I–XXVIII)", "list");
    sprintln!("  {:<30} — load canonical program", "canonical <I–XII>");
    sprintln!("  {:<30} — load continuous program", "continuous <1–4>");
    sprintln!("  {:<30} — load novel program (XVII–XIX)", "novel <1–3>");
    sprintln!("  {:<30} — load shunted program (XX–XXVIII)", "shunt <1–9>");
    sprintln!();
    sprintln!("══ Crystal FS ══");
    sprintln!("  {:<30} — decode address to 12-tuple", "crystal <addr>");
    sprintln!("  {:<30} — store entry", "crystal store <n> [d]");
    sprintln!("  {:<30} — retrieve by name", "crystal name <n>");
    sprintln!("  {:<30} — list stored entries", "crystal find");
    sprintln!();
    sprintln!("══ Grammar Bridges ══");
    sprintln!("  {:<32} — IG tuple + crystal address", "ig");
    sprintln!("  {:<32} — nearest-catalog classification", "classify");
    sprintln!("  {:<32} — Frobenius harness status (closed/open ratio)", "frob");
    sprintln!("  {:<32} — Hebrew glyph encoding + gematria", "aleph <Hebrew word>");
    sprintln!("  {:<32} — Belnap Shor pipeline (N=15, N=21)", "shor");
    sprintln!("  {:<32} — Riemann Hypothesis bridge", "rh");
    sprintln!("  {:<32} — Yang-Mills mass gap bridge", "ym");
    sprintln!("  {:<32} — Temporal logic bridge", "temp");
    sprintln!("  {:<32} — Category theory bridge", "cat");
    sprintln!("  {:<32} — distance|meet|join|tensor vs ZFC baseline", "algebra <op>");
    sprintln!("  {:<32} — promotions | entry <name> (any catalog system)", "cl8nk <action> [name]");
    sprintln!("  {:<32} — consciousness score (dual-gate)", "cscore");
    sprintln!("  {:<32} — SIC-POVM d=12 structural identity (3 lattice proofs)", "sic");
    sprintln!("  {:<32} — entropy experiment: ΔS vs tier promotion", "entropy [tier|transition]");
    sprintln!("  {:<32} — d=12 SIC-POVM Phase VI: tower,magnitudes,orbits,existence,duallink,z0", "d12 [subcmd]");
    sprintln!("  {:<32} — d=2048 moduli tower ascent: tower,redei,grammar,pari,next", "d2048 [subcmd]");
    sprintln!("  {:<32} — witness-vessel transport: Clay payloads x 88 dialects, frob-gated", "vessel [run]");
    sprintln!("  {:<32} — manuscript spine: PROVE→UNIFY→PORT × vessel (no Python)", "spine [run|lean]");
    sprintln!("  {:<32} — kernel structural ask (dry). Full wet: host ./ask --file| -i", "ask [opts] <question>");
    sprintln!("  {:<32} — Clay Millennium structural status (machine-checked)", "clay");
    #[cfg(feature = "vita")]
    sprintln!("  {:<32} — one certified turn from the on-board vae_vita trunk", "vita [seed] [temp]");
    sprintln!();
    sprintln!("══ Rebis (Red-Hot Rebis) ══");
    sprintln!("  {:<34} — codon→AA or AA→codons (bidirectional)", "rebis codon <XXX|AA>");
    sprintln!("  {:<32} — Clay witness IMASM programs (BSD/Hodge/YM)", "clay witness <problem>");
    sprintln!("  {:<34} — gene→protein pipeline (DNA→mRNA→AA)", "rebis translate <DNA>");
    sprintln!("  {:<34} — protein→mRNA→DNA (reverse pipeline)", "rebis reverse <Prot>");
    sprintln!("  {:<34} — Frobenius filtration (64 codons, power-law)", "rebis frob");
    sprintln!("  {:<34} — 7-stage genetic code verification", "rebis genetics");
    sprintln!("  {:<34} — Belnap hadron analysis (p, n, π+)", "rebis hadron");
    sprintln!("  {:<34} — serpent rod motif analysis", "rebis serpent [name]");
    sprintln!("  {:<34} — DNA/RNA->folded protein (SerpentRod)", "rebis fold <DNA|RNA> [mito]");
    sprintln!("  {:<34} — IG promotion pipeline", "rebis pipeline [src]");
    sprintln!("  {:<34} — codon stratum counts", "rebis strata");
    sprintln!("  {:<34} — genetic ParaASM programs", "rebis asm [prog]");
    sprintln!("  {:<34} — 7-stage generative tuple pipeline", "rebis tuples <DNA>");
    sprintln!("  {:<34} — CLU power-law clustering", "rebis clu walk|verify");
    sprintln!("  {:<34} — exotic hadron Frobenius verification", "rebis exotic");
    sprintln!("  {:<34} — PDB structure validation", "rebis pdb validate|..");
    sprintln!("  {:<34} — antibody CDR design", "rebis antibody epi|des");
    sprintln!("  {:<34} — IG material forge & metamaterials", "rebis material forge|..");
    sprintln!("  {:<34} — biological sim (tissue, telomere)", "rebis bio");
    sprintln!("  {:<34} — therapeutics (chemo, pill, antidote)", "rebis tx");
    sprintln!();
    sprintln!("══ cr3echrz — Theorem Operationalization ══");
    sprintln!("  {:<34} — list all theorems + p4rakernel + vault", "cr3 --list");
    sprintln!("  {:<34} — list 281 vault ob3ects", "cr3 --list-ob3ects");
    sprintln!("  {:<34} — collatz|goldbach|three_body|burnside|...", "cr3 <theorem> [params]");
    sprintln!("  {:<34} — Collatz 3n+1 (e.g. cr3 collatz 27)", "cr3 collatz <seed>");
    sprintln!("  {:<34} — Goldbach partitions (e.g. cr3 goldbach 100)", "cr3 goldbach <n>");
    sprintln!("  {:<34} — Three-Body figure-8 orbit", "cr3 three_body");
    sprintln!("  {:<34} — Burnside B(m,n) finiteness", "cr3 burnside <gens> <exp>");
    sprintln!("  {:<34} — Erdős–Straus 4/n decomposition", "cr3 erdos_straus <n>");
    sprintln!("  {:<34} — Inverse Galois realizability", "cr3 inverse_galois <group>");
    sprintln!("  {:<34} — Baum–Connes assembly map", "cr3 baum_connes <class>");
    sprintln!("  {:<34} — Belnap+Frobenius 13-step bootstrap", "p4ra <module> [params]");
    sprintln!("  {:<34} — list p4rakernel modules", "p4ra --list");
    sprintln!("  {:<34} — burnside|connes|erdos_straus|goldbach|...", "p4ra <module>");
    sprintln!();
    sprintln!("══ Cross-Dialect Navigation (Phase 8) ══");
    sprintln!("══ Ruleset / Dialect ══");
    sprintln!("  {:<36} — show active ruleset", "ruleset show");
    sprintln!("  {:<36} — list all 88 dialects (★ = active)", "ruleset list");
    sprintln!("  {:<36} — invariant check (live snapshot)", "ruleset verify");
    sprintln!("  {:<36} — invariant check (named catalog entry)", "ruleset verify <name>");
    sprintln!("  {:<36} — cross-dialect jump", "jump <U> using <compound>");
    sprintln!("  {:<36} — probe without IFIX seal", "jump <U> using <c> --liminal");
    sprintln!("  {:<36} — two-stage jump", "jump <U> via <V> using <c1> <c2>");
    sprintln!("  {:<36} — IFIX commit to current ruleset", "seal");
    sprintln!("  <U> = U_0–U_11 or U₀–U₁₁    <compound> = see 'compound list'");
    sprintln!("  {:<36} — tensor under active absorption", "tensor <compound_a> <compound_b>");
    sprintln!("  {:<36} — meet under active absorption", "meet <compound_a> <compound_b>");
    sprintln!("  {:<36} — test absorption rule", "absorb_test <a> <b> <prim> <op>");
    sprintln!("  {:<36} — IG tuple under active ruleset", "whoami --ruleset");
    sprintln!("  {:<36} — Frobenius fixed-point identity check", "whoami --frobenius");
    sprintln!("  {:<36} — list all absorption rules", "absorption show");
    sprintln!("  {:<36} — T-constitution pass/fail report", "tstatus");
    sprintln!("  {:<36} — list 11 diaschizic compounds", "compound list");
    sprintln!("  {:<36} — show compound tuple + IMASM", "compound show <name>");
    sprintln!("  {:<36} — load compound IMASM into buffer", "compound load <name>");
    sprintln!();
    sprintln!("══ ParaASM ══");
    sprintln!("  {:<36} — dialetheic alignment + measurement tests", "psm test");
    sprintln!("  {:<36} — Frobenius identity cycle (ENGAGR→FSPLIT→FFUSE→HALT)", "psm frob");
    sprintln!("  {:<36} — kernel-state B3 invariant loop", "psm kernel");
    sprintln!("  {:<36} — inline ParaASM program (; separator)", "psm load <prog>");
    sprintln!();
    sprintln!("  {:<36} — exit (μ∘δ=id)", "halt/quit");
    sprintln!();
    sprintln!("Control flow: FSPLIT=fork  FFUSE=join  EVALT/EVALF=branch");
    sprintln!("              TANCH=halt  VINIT=source  IMSCRIB=self-loop");
}
fn print_status(k: &Kernel) {
    let tier = k.snapshot.map(|s| s.tier_name()).unwrap_or("?");
    sprintln!("╔══════════════════════════════════════╗");
    sprint!(  "   Tick: {:8}  Tier: {:<8}        \n", k.tick_count, tier);
    sprint!(  "   IP: {:8}    Stack: {:6}          \n", k.ip, k.stack.depth());
    sprint!(  "   Fork: {:6}   Frob: {}/{}           \n",
        k.fork_depth(), k.frob_checks - k.frob_open, k.frob_checks);
    sprint!(  "   Halted: {:<6}                      \n",
        if k.halted { "YES" } else { "no" });
    serial::write_str("   R0-R7: ");
    for i in 0..8 {
        serial::write_str(k.registers.read(i).name());
        serial::write_str(" ");
    }
    sprintln!("     ");
    sprintln!("╚══════════════════════════════════════╝");
}


fn print_frob(k: &Kernel) {
    let h = &k.harness;
    sprintln!("Frobenius: {} total  {} closed  {} open  ratio={}/{}  closed={}",
        h.total(), h.closed_count, h.open_count, h.closed_count, h.total(), h.is_closed());
    sprintln!("History (recent first):");
    for i in (0..8).rev() {
        let idx = (h.history_head + 16 - 1 - i) % 16;
        let r = &h.history[idx];
        let s = if r.closed { "C" } else { "O" };
        sprint!("  {} {}({}->{} u->{})", s, r.belnap_value.name(), r.delta_input.name(), r.delta_output.name(), r.mu_result.name());
        if let Some(m) = r.mismatch { sprint!(" {}", m); }
        sprintln!("");
    }
}


fn print_aleph(_k: &Kernel, word: &str) {
    use crate::aleph::{AlephWord, AlephLetter};
    if word.is_empty() {
        sprintln!("Usage: aleph <Hebrew word>");
        sprintln!("  22 letters: Aleph Mem Shin Bet Gimel Dalet Kaf Pe Resh Tav He Vav Zayin Chet Tet Yod Lamed Nun Samekh Ayin Tzadi Qof");
        return;
    }
    let aw = AlephWord::encode(word);
    sprintln!("Aleph: '{}'  gematria={}  letters={}", word, AlephLetter::gematria(word), aw.count);
    sprint!("Prims: ");
    for i in 0..aw.count {
        if let Some(l) = aw.letters[i] {
            sprint!("{}({}) ", l.glyph, l.prim.short());
        }
    }
    sprintln!("");
}
fn print_ig(k: &Kernel) {
    use crate::imas_ig::IgTuple;
    if let Some(snap) = k.snapshot {
        let ig = IgTuple::from_snapshot(&snap);
        sprintln!("IG: {}", ig.display());
        sprintln!("Crystal: {}", ig.crystal_address());
    } else {
        sprintln!("No snapshot. Tick first.");
    }
}

fn print_classify(k: &Kernel) {
    use crate::imas_ig::Classification;
    if let Some(snap) = k.snapshot {
        let c = Classification::classify(&snap);
        sprintln!("{}", c.display());
    } else {
        sprintln!("No snapshot. Tick first.");
    }
}

fn snap_witnesses(s: &crate::kernel::Snapshot) -> (bool, bool, bool, bool, bool, bool) {
    (s.dialetheia_complete, s.b_live_ticks > 0, s.gate_discriminations > 0,
     s.atomic_reentry, s.winding_count > 0, s.bifurcation_revisited)
}

fn print_snap_line(tag: &str, s: &crate::kernel::Snapshot) {
    let (d, bl, g, a, w, bi) = snap_witnesses(s);
    sprintln!("  {:<10} tier {:<9}  R1(dialeth={} b_live={} gates={})  R2(atomic={} wind={} bifurc={})",
        tag, s.tier_name(), d, bl, g, a, w, bi);
}

/// One Ħ hop: toggle chirality, show the snapshot on each side of the door.
fn print_arev_hop(k: &mut Kernel) {
    let before = k.dynamic_imscribe();
    let h = k.arev_hop();
    let after = k.snapshot.unwrap_or(before);
    sprintln!("AREV — Ħ hop, lateral at the same shell. Ħ now {}", if h { "flipped" } else { "or'" });
    print_snap_line("before", &before);
    print_snap_line("after", &after);
}

/// The door experiment: descend to O_inf_dag, hop through the mirror, hop back,
/// and verify hop∘hop = id exactly (raw fields), plus the mirror's own behavior
/// on the witness plane.
fn print_arev_test(k: &mut Kernel) {
    sprintln!("═ AREV door experiment ═");
    k.load_replicative();
    k.run(16); // 4 wraps of the 4-token cycle: winding_count > 0, both R2 marks live
    if k.chirality { k.arev_hop(); } // enter with Ħ = or'
    let s0 = k.dynamic_imscribe();
    sprintln!("replicative loop, 16 ticks, Ħ = or':");
    print_snap_line("s0", &s0);
    k.arev_hop();
    let s1 = k.snapshot.unwrap_or(s0);
    sprintln!("first hop (Ħ flipped) — R1 reads the mirrored evidence:");
    print_snap_line("s1", &s1);
    k.arev_hop();
    let s2 = k.snapshot.unwrap_or(s0);
    sprintln!("second hop (Ħ back to or'):");
    print_snap_line("s2", &s2);
    sprintln!("hop∘hop = id (raw fields): {}", if s2 == s0 { "EXACT" } else { "BROKEN" });
    let mm = s0.mirrored().mirrored();
    sprintln!("mirror∘mirror = id: witness plane {}, raw fields {}",
        if snap_witnesses(&mm) == snap_witnesses(&s0) { "EXACT" } else { "BROKEN" },
        if mm == s0 { "EXACT" } else { "section-lossy (expected: counts pass through true ↦ 1)" });
}

fn roman_to_idx(s: &str) -> Option<usize> {
    match s {
        "I"    => Some(0),  "II"   => Some(1),  "III" => Some(2),
        "IV"   => Some(3),  "V"    => Some(4),  "VI"  => Some(5),
        "VII"  => Some(6),  "VIII" => Some(7),  "IX"  => Some(8),
        "X"    => Some(9),  "XI"   => Some(10), "XII" => Some(11),
        "XIII" => Some(12), "XIV"  => Some(13), "XV"  => Some(14),
        "XVI"  => Some(15), "XVII" => Some(16), "XVIII" => Some(17),
        "XIX"  => Some(18), "XX"   => Some(19), "XXI"  => Some(20),
        "XXII" => Some(21), "XXIII" => Some(22), "XXIV" => Some(23),
        "XXV"  => Some(24), "XXVI"  => Some(25), "XXVII" => Some(26), "XXVIII" => Some(27),
        _ => None,
    }
}

fn idx_to_roman(i: usize) -> &'static str {
    match i {
        0  => "I",    1  => "II",   2  => "III",
        3  => "IV",   4  => "V",    5  => "VI",
        6  => "VII",  7  => "VIII", 8  => "IX",
        9  => "X",    10 => "XI",   11 => "XII",
        12 => "XIII", 13 => "XIV",  14 => "XV",
        15 => "XVI",  16 => "XVII", 17 => "XVIII",
        18 => "XIX",  19 => "XX",   20 => "XXI",
        21 => "XXII", 22 => "XXIII", 23 => "XXIV",
        24 => "XXV",  25 => "XXVI",  26 => "XXVII", 27 => "XXVIII",
        _  => "?",
    }
}

fn load_by_roman(k: &mut Kernel, roman: &str) -> bool {
    if let Some(idx) = roman_to_idx(roman) {
        if idx < CANONICAL_COUNT {
            k.load_canonical(idx);
            true
        } else if idx < CANONICAL_COUNT + CONTINUOUS_COUNT {
            k.load_continuous(idx - CANONICAL_COUNT)
        } else if idx < CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT {
            k.load_novel(idx - CANONICAL_COUNT - CONTINUOUS_COUNT)
        } else if idx < CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT + SHUNTED_COUNT {
            k.load_shunted(idx - CANONICAL_COUNT - CONTINUOUS_COUNT - NOVEL_COUNT)
        } else {
            false
        }
    } else {
        false
    }
}

fn name_hash(name: &str) -> usize {
    let mut h: u32 = 2_166_136_261;
    for b in name.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16_777_619);
    }
    h as usize
}

// ─── ParaASM REPL ───────────────────────────────────────────────

fn print_ym() {
    use para_ym::*;
    sprintln!("══ Yang-Mills Mass Gap ══");
    sprintln!("  gap exists:    {}", if ym_gap_exists() { "PASS" } else { "FAIL" });
    sprintln!("  not dialetheic: {}", if ym_gap_not_dialetheic() { "PASS" } else { "FAIL" });
    sprintln!("  vacuum canon:  {}", if ym_vacuum_canonical() { "PASS" } else { "FAIL" });
    sprintln!("  BRST nilpotent: {}", if ym_brst_nilpotent() { "PASS" } else { "FAIL" });
    sprintln!("  confinement:   {}", if ym_confinement_ktrap() { "PASS" } else { "FAIL" });
    sprintln!("  topo protect:  {}", if ym_topological_protection() { "PASS" } else { "FAIL" });
    sprintln!("  mass gap +:    {}", if mass_gap_positive() { "PASS" } else { "FAIL" });
    sprintln!("  BRST+frob:     {}", if ym_brst_frobenius() { "PASS" } else { "FAIL" });
    sprintln!("  imscription:   {}", YM_IMSCRIPTION);
}
fn print_temporal() {
    use para_temporal::*;
    sprintln!("══ Temporal Logic ══");
    sprintln!("  B fixed point: {}", if b_temporal_fixed() { "PASS" } else { "FAIL" });
    sprintln!("  next involution: {}", if next_involution() { "PASS" } else { "FAIL" });
    sprintln!("  B absorbs until: {}", if b_absorbs_until() { "PASS" } else { "FAIL" });
    sprintln!("  B U N = B, N U T = T, T U F = T");
}
fn print_cat() {
    use para_category::*;
    sprintln!("══ Category Theory ══");
    sprintln!("  N initial:    {}", if n_initial() { "PASS" } else { "FAIL" });
    sprintln!("  T terminal:   {}", if t_terminal() { "PASS" } else { "FAIL" });
    sprintln!("  B zero:       {}", if b_zero() { "PASS" } else { "FAIL" });
    sprintln!("  frobenius alg: {}", if frobenius_algebra() { "PASS" } else { "FAIL" });
    sprintln!("  dagger compact: {}", if dagger_compact() { "PASS" } else { "FAIL" });
    sprintln!("  product/coprod: {}", if product_coproduct() { "PASS" } else { "FAIL" });
}

fn print_rh() {
    use crate::belnap::B4;
    use para_rh::*;

    sprintln!("══ Riemann Hypothesis Bridge ══");
    sprintln!("  involution:     {}", if rh_involution_identity() { "PASS" } else { "FAIL" });
    sprintln!("  fixed point:    {}", if rh_frobenius_fixed_point() { "PASS" } else { "FAIL" });
    sprintln!("  belnap RH:      {}", if rh_belnap_statement() { "PASS" } else { "FAIL" });
    sprintln!("  O_inf bridge:   {}", if rh_bridge_is_o_inf() { "PASS" } else { "FAIL" });
    sprintln!("  barriers unif.: {}", if millennium_barriers_unified() { "PASS" } else { "FAIL" });
    sprintln!();
    sprintln!("  Functional equation bnot (s->1-s):");
    for &v in &[crate::belnap::B4::N, B4::T, B4::F, B4::B] {
        let img = v.bnot();
        let tag = if img == v && v.designated() { " <- FROBENIUS FIXED" }
             else if img == v { " <- fixed" } else { "" };
        sprintln!("    bnot({}) = {}{}", v.name(), img.name(), tag);
    }
    sprintln!();
    sprintln!("  Critical strip:");
    for &(num, label) in STRIP_SAMPLES {
        let s = rh_strip_state(num, 100);
        sprintln!("    {:>8} -> {}  {}", label, s.name(), strip_label(s));
    }
    sprintln!();
    sprintln!("  Imscription: {}", RH_IMSCRIPTION);
}

fn print_shor() {
    use crate::belnap::B4;
    use belnap_shor::*;

    sprintln!("══ Belnap Shor Pipeline ══");

    sprintln!("── SIC-POVM Axioms ──");
    sprintln!("  verify: {}", if verify_sic_povm() { "PASS" } else { "FAIL" });

    sprintln!("── Hadamard ──");
    sprintln!("  H|T⟩=B: {}", if b4_hadamard(B4::T) == B4::B { "PASS" } else { "FAIL" });
    sprintln!("  H|F⟩=B: {}", if b4_hadamard(B4::F) == B4::B { "PASS" } else { "FAIL" });
    sprintln!("  H|B⟩=T: {}", if b4_hadamard(B4::B) == B4::T { "PASS" } else { "FAIL" });
    sprintln!("  H|N⟩=N: {}", if b4_hadamard(crate::belnap::B4::N) == crate::belnap::B4::N { "PASS" } else { "FAIL" });

    sprintln!("── Shor N=15,a=7 ──");
    let r1 = run_belnap_shor(4, 7, 15);
    sprintln!("  period={} H={} B-meas={} T-meas={} ratio={:.1}",
        r1.period_cl, r1.hadamard_coherence, r1.b_bias_coherence, r1.t_bias_coherence, r1.ratio);
    sprintln!("  allB={} b-preserves={} t-collapses={} bottleneck={}",
        r1.mod_exp_all_b, r1.b_bias_preserves, r1.t_bias_collapses, r1.phi_upsilon_bottleneck);

    sprintln!("── Shor N=21,a=5 ──");
    let r2 = run_belnap_shor(5, 5, 21);
    sprintln!("  period={} H={} B-meas={} T-meas={} ratio={:.1}",
        r2.period_cl, r2.hadamard_coherence, r2.b_bias_coherence, r2.t_bias_coherence, r2.ratio);

    sprintln!("── Phi_upsilon bottleneck ──");
    sprintln!("  B is the only superposition; all lattice ops preserve B.");
    sprintln!("  Period r encoded in 2:1 coherence cost ratio, not bits.");
    sprintln!("  Phi_upsilon -> Phi_pmsym gap: structural open problem.");
}

fn print_psm(arg: &str) {
    use crate::belnap::B4;
    use parasm::*;

    match arg {
        "test" => {
            sprintln!("── ParaASM Dialetheic Alignment ──");
            let (op, log, alg) = dialetheic_alignment_tri();
            sprintln!("  operational: {}", if op { "PASS" } else { "FAIL" });
            sprintln!("  logical:     {}", if log { "PASS" } else { "FAIL" });
            sprintln!("  algebraic:   {}", if alg { "PASS" } else { "FAIL" });
            sprintln!("  B is only bifurcation: {}", if b_is_only_bifurcation_point() { "PASS" } else { "FAIL" });

            sprintln!("── Measurement Algebra ──");
            let m_b_b = measure_step(B4::B, B4::B) == B4::B;
            let m_b_t = measure_step(B4::B, B4::T) == B4::T;
            let m_b_f = measure_step(B4::B, B4::F) == B4::F;
            let cost_bb = measure_cost(B4::B, B4::B) == 2;
            let cost_bt = measure_cost(B4::B, B4::T) == 1;
            let cost_tt = measure_cost(B4::T, B4::T) == 0;
            let irrev_t = collapse_irreversible(B4::T);
            let irrev_f = collapse_irreversible(B4::F);
            let irrev_n = collapse_irreversible(crate::belnap::B4::N);
            sprintln!("  measure_step(B,B)=B:  {}", if m_b_b { "PASS" } else { "FAIL" });
            sprintln!("  measure_step(B,T)=T:  {}", if m_b_t { "PASS" } else { "FAIL" });
            sprintln!("  measure_step(B,F)=F:  {}", if m_b_f { "PASS" } else { "FAIL" });
            sprintln!("  measure_cost(B,B)=2:  {}", if cost_bb { "PASS" } else { "FAIL" });
            sprintln!("  measure_cost(B,T)=1:  {}", if cost_bt { "PASS" } else { "FAIL" });
            sprintln!("  measure_cost(T,T)=0:  {}", if cost_tt { "PASS" } else { "FAIL" });
            sprintln!("  irreversible(T):      {}", if irrev_t { "PASS" } else { "FAIL" });
            sprintln!("  irreversible(F):      {}", if irrev_f { "PASS" } else { "FAIL" });
            sprintln!("  irreversible(N):      {}", if irrev_n { "PASS" } else { "FAIL" });
            sprintln!("  wigner_cost(1)=3:     {}", if wigner_then_collapse_cost(1) == 3 { "PASS" } else { "FAIL" });
        }

        "frob" => {
            sprintln!("── Frobenius Identity Cycle ──");
            let mut vm = ParaVM::new();
            vm.load("
                ENGAGR %r0
                FSPLIT %r0 %r1 %r2
                FFUSE %r1 %r2 %r0
                HALT
            ").unwrap();
            vm.run(None);
            let s = vm.snapshot();
            sprintln!("  steps:   {}", s.steps);
            sprintln!("  paradox: {}", s.paradox);
            sprintln!("  halted:  {}", s.halted);
            sprintln!("  r0:      {}", vm.belief_of(0).name());
            sprintln!("  r1:      {}", vm.belief_of(1).name());
            sprintln!("  r2:      {}", vm.belief_of(2).name());
            sprintln!("  dist:    N={} T={} F={} B={}", s.dist_n, s.dist_t, s.dist_f, s.dist_b);
        }

        "kernel" => {
            sprintln!("── Kernel-State Loop (8 cycles) ──");
            let mut ks = KernelState::new();
            let mut b3_held = true;
            for i in 0..8 {
                ks.kernel_step();
                sprintln!("  cycle {}: r0={} r1={} r2={} paradox={}",
                    i + 1, ks.r0.name(), ks.r1.name(), ks.r2.name(), ks.paradox_count);
                if ks.r0 != B4::B || ks.r1 != B4::B || ks.r2 != B4::B {
                    sprintln!("  B3 INVARIANT VIOLATED on cycle {} — structurally expected: kernel must bifurcate to self-imscribe", i + 1);
                    b3_held = false;
                    break;
                }
            }
            if b3_held {
                sprintln!("  B3 invariant: PASS (all 8 cycles — registers = B throughout)");
            }
        }

        _ => {
            // psm load <program> — inline ParaASM loading
            if arg.starts_with("load ") || arg == "load" {
                let prog_text_raw = if arg == "load" {
                    sprintln!("Usage: psm load <program>");
                    sprintln!("Example: psm load ENGAGR %r0; FSPLIT %r0 %r1 %r2; FFUSE %r1 %r2 %r0; HALT");
                    return;
                } else {
                    &arg[5..]  // strip "load "
                };
                // Convert semicolons to newlines for inline programs
                let prog_text: alloc::string::String = prog_text_raw.replace("; ", "\n").replace(";", "\n");
                sprintln!("── ParaASM Inline Load ──");
                let mut vm = ParaVM::new();
                match vm.load(&prog_text) {
                    Ok(()) => {
                        sprintln!("  Assembled: {} instructions", vm.program.len());
                        sprintln!("  Running...");
                        vm.run(None);
                        let s = vm.snapshot();
                        sprintln!("  steps:   {}", s.steps);
                        sprintln!("  paradox: {}", s.paradox);
                        sprintln!("  halted:  {}", s.halted);
                        for i in 0..8 {
                            let b = vm.belief_of(i);
                            if b != crate::belnap::B4::N || s.steps > 0 {
                                sprintln!("  r{}:      {}", i, b.name());
                            }
                        }
                        sprintln!("  dist:    N={} T={} F={} B={}", s.dist_n, s.dist_t, s.dist_f, s.dist_b);
                    }
                    Err(e) => {
                        sprintln!("  Error: {}", e);
                    }
                }
                return;
            }

            sprintln!("ParaASM commands:");
            sprintln!("  psm test   — run dialetheic alignment + measurement tests");
            sprintln!("  psm frob   — run frobenius identity cycle");
            sprintln!("  psm kernel — run kernel-state B3 loop");
            sprintln!("  psm load   — load and run inline ParaASM program");
        }
    }
}

// ─── Phase 2 Handlers ─────────────────────────────────────────

fn print_algebra(k: &Kernel, arg: &str) {
    use crate::algebra::{primitive_mismatches, tuple_distance, meet, join, tensor};
    use crate::imas_ig::IgTuple;

    if let Some(snap) = k.snapshot {
        let ig = IgTuple::from_snapshot(&snap);
        match arg {
            "distance" | "dist" => {
                let zfc = catalog::zfc_baseline_tuple();
                sprintln!("Hamming mismatches: {}/12", primitive_mismatches(&ig, &zfc));
                sprintln!("Weighted distance:  {:.2}", tuple_distance(&ig, &zfc));
            }
            "meet" => {
                let zfc = catalog::zfc_baseline_tuple();
                let r = meet(&ig, &zfc);
                sprintln!("{}", r);
            }
            "join" => {
                let zfc = catalog::zfc_baseline_tuple();
                let r = join(&ig, &zfc);
                sprintln!("{}", r);
            }
            "tensor" => {
                let zfc = catalog::zfc_baseline_tuple();
                let t = tensor(&ig, &zfc);
                sprintln!("tensor: {}", t.display_shavian());
            }
            _ => {
                sprintln!("algebra <distance|meet|join|tensor>");
                sprintln!("  Current: {}", ig.display());
            }
        }
    } else {
        sprintln!("No snapshot. Tick first.");
    }
}

fn print_cl8nk(action: &str, name: &str) {
    use crate::cl8nk::*;
    match action {
        "promotions" | "promo" => {
            let result = generate_promotions();
            sprintln!("══ CL8NK Promotion Ladder ══");
            sprintln!("  ZFC (O₀) → ZFCₜ (O₂†) → ZFC_fe (O_∞) → CLINK L8 (O_∞⁺)");
            sprintln!("  Total promotions: {}  d(ZFC, CLINK L8): {:.4}", result.total_promotions, result.total_distance);
            sprintln!();
            for stage in &result.ladder {
                sprintln!("  {}  [{}]", stage.stage, stage.tier);
                if let Some(d) = stage.distance {
                    sprintln!("    promotions: {}  distance: {:.4}", stage.promotions, d);
                }
                if let Some(note) = stage.note {
                    sprintln!("    ⬆ {}", note);
                }
                for det in &stage.details {
                    let from_atom = if let Some(a) = det.from_atom { alloc::format!(" [{}]", a) } else { String::from("") };
                    let to_atom = if let Some(a) = det.to_atom { alloc::format!(" [{}]", a) } else { String::from("") };
                    sprintln!("    {}: {} -> {}  gap={:.3}  {} -> {}{}{}",
                        det.primitive, det.from_glyph, det.to_glyph, det.ordinal_gap,
                        det.from_fragment, det.to_fragment, from_atom, to_atom);
                }
                sprintln!();
            }
        }
        "" | "entry" => {
            let lookup_name = if name.is_empty() { "clink_l8" } else { name };
            let t;
            let dname: String;
            let desc: String;
            if let Some(cat_entry) = catalog::lookup(lookup_name) {
                t = cat_entry.tuple;
                dname = String::from(cat_entry.name);
                desc = String::from(cat_entry.description);
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
                return;
            }
            let result = generate_entry_formula(&dname, &desc, &t);
            sprintln!();
            sprintln!("══════════════════════════════════════════════════════════════");
            sprintln!("  CL8NK Entry: {}", result.system_name);
            sprintln!("  {}", result.description);
            sprintln!("  Reference: CLINK L8 (Organism) — ⟨𐑦⋅𐑸⋅𐑾⋅𐑹⋅𐑐⋅𐑧⋅𐑲⋅𐑵⋅⊙⋅𐑫⋅𐑳⋅𐑟⟩");
            sprintln!("══════════════════════════════════════════════════════════════");
            sprintln!();
            sprintln!("  Prim   Value   CLINK fragment");
            sprintln!("  ─────  ──────  ────────────────────────────────────────────────");
            for frag in &result.fragments {
                let atom_tag = if let Some(a) = frag.promoted_atom { alloc::format!("[{}]", a) } else { String::from("") };
                sprintln!("  {:<6} {:<7} {} {}",
                    frag.primitive, frag.value_glyph, frag.clink_fragment, atom_tag);
            }
            if !result.promoted_atoms.is_empty() {
                sprintln!();
                for ad in &result.atom_details {
                    sprintln!("  [{}] {}", ad.atom, atom_desc(ad.atom));
                }
            }
            sprintln!();
            sprintln!("  tier: {}   d(CLINK L8): {:.4}   match:{} close:{} distant:{}",
                result.tier, result.distance, result.match_count, result.close_count, result.distant_count);
            if !result.promoted_atoms.is_empty() {
                sprintln!("  promoted atoms: {}", result.promoted_atoms.join(", "));
            }
            if result.has_transcendence {
                sprintln!("  ⬆ TRANSCENDENCE primitives: {}", result.transcendence_keys.join(", "));
            }
            if !result.promotions_needed.is_empty() {
                sprintln!();
                sprintln!("  Promotions needed to reach CLINK L8 ({}):", result.promotions_count);
                for p in &result.promotions_needed {
                    sprintln!("    {}: {} -> {}  (gap: {:.3})", p.primitive, p.from_glyph, p.to_glyph, p.gap);
                }
            }
        }
        "distance" => {
            let lookup_name = if name.is_empty() { "zfc" } else { name };
            if let Some(cat_entry) = catalog::lookup(lookup_name) {
                let cl8 = cl8nk_ref();
                let (d, conflicts) = tuple_distance_cl8nk(&cat_entry.tuple, &cl8);
                let tier = assess_tier(&cat_entry.tuple);
                sprintln!("══ CL8NK Distance ══");
                sprintln!("  System: {}  →  CLINK L8", cat_entry.name);
                sprintln!("  d = {:.4}  tier: {}", d, tier);
                sprintln!("  Conflicts ({}):", conflicts.len());
                for c in &conflicts {
                    sprintln!("    {}: {} vs {}  delta={:.3}",
                        c.primitive,
                        catalog::primitive_glyph(c.sys_val),
                        catalog::primitive_glyph(c.cl8nk_val),
                        c.delta);
                }
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
            }
        }
        "transcendence" => {
            let tr = compute_transcendence();
            sprintln!("══ The Ω/ɢ Transcendence — CLINK L8 beyond ZFC_fe ══");
            sprintln!("  d(ZFC_fe, CLINK L8) = {:.4}", tr.d_zfcfe_to_cl8nk);
            sprintln!();
            sprintln!("  Ω: {} → {}",
                catalog::primitive_glyph(tr.omega_zfcfe),
                catalog::primitive_glyph(tr.omega_cl8nk));
            sprintln!("    ZFC_fe: {}", tr.omega_zfcfe_frag);
            sprintln!("    CL8NK:  {}", tr.omega_cl8nk_frag);
            sprintln!("    → Integer winding (Abelian anyons) → braid group (non-Abelian anyons)");
            sprintln!();
            sprintln!("  C (ɢ): {} → {}",
                catalog::primitive_glyph(tr.grammar_zfcfe),
                catalog::primitive_glyph(tr.grammar_cl8nk));
            sprintln!("    ZFC_fe: {}", tr.grammar_zfcfe_frag);
            sprintln!("    CL8NK:  {}", tr.grammar_cl8nk_frag);
            sprintln!("    → Sequential stepwise → simultaneous broadcast composition");
            sprintln!();
            sprintln!("  tensor(ZFC_fe, CLINK L8) = {}",
                if tr.tensor_absorbed { "CLINK L8 — foundation fully absorbed" }
                else { "composite — NOT fully absorbed" });
        }
        "tensor" => {
            let lookup_name = if name.is_empty() { "zfc" } else { name };
            if let Some(cat_entry) = catalog::lookup(lookup_name) {
                let tr = compute_tensor_op(&cat_entry.tuple);
                sprintln!("══ CLINK L8 ⊗ {} ══", cat_entry.name);
                sprintln!("  tensor: {}", tr.tuple.display_shavian());
                sprintln!("  d(CLINK L8): {:.4}  absorbed: {}", tr.distance_from_cl8nk, tr.absorbed);
                sprintln!("  {}", tr.interpretation);
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
            }
        }
        "meet" => {
            let lookup_name = if name.is_empty() { "zfc" } else { name };
            if let Some(cat_entry) = catalog::lookup(lookup_name) {
                let mr = compute_meet_op(&cat_entry.tuple);
                sprintln!("══ CLINK L8 ⊓ {} ══", cat_entry.name);
                sprintln!("  meet: {}", mr.tuple.display_shavian());
                sprintln!("  d(CLINK L8): {:.4}  d(system): {:.4}", mr.d_from_cl8nk, mr.d_from_system);
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
            }
        }
        "join" => {
            let lookup_name = if name.is_empty() { "zfc" } else { name };
            if let Some(cat_entry) = catalog::lookup(lookup_name) {
                let jr = compute_join_op(&cat_entry.tuple);
                sprintln!("══ CLINK L8 ⊔ {} ══", cat_entry.name);
                sprintln!("  join: {}", jr.tuple.display_shavian());
                sprintln!("  d(CLINK L8): {:.4}  d(system): {:.4}", jr.d_from_cl8nk, jr.d_from_system);
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
            }
        }
        "tier" => {
            let lookup_name = if name.is_empty() { "clink_l8" } else { name };
            if let Some(cat_entry) = catalog::lookup(lookup_name) {
                let tier = assess_tier(&cat_entry.tuple);
                let cl8 = cl8nk_ref();
                let (d, _) = tuple_distance_cl8nk(&cat_entry.tuple, &cl8);
                sprintln!("══ CL8NK Tier ══");
                sprintln!("  System: {}  tier: {}  d(CLINK L8): {:.4}", cat_entry.name, tier, d);
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
            }
        }
        "chain" => {
            let layers = chain_analysis();
            sprintln!("══ CLINK Chain — Distance Ladder from CLINK L8 ══");
            sprintln!("  {} layers discovered in catalog", layers.len());
            sprintln!();
            for layer in &layers {
                sprintln!("  {:<24}  d={:.4}  tier={}  conflicts={}",
                    layer.name, layer.distance_from_l8, layer.tier, layer.conflicts_count);
            }
        }
        "systems" => {
            let systems = catalog_systems();
            sprintln!("══ CL8NK — Catalog Systems ══");
            sprintln!("  {} entries", systems.len());
            for s in &systems {
                sprintln!("    {}", s);
            }
        }
        "stats" => {
            let (count, cl8_found, zfcfe_found) = catalog_stats();
            sprintln!("══ CL8NK — Catalog Statistics ══");
            sprintln!("  Total entries: {}", count);
            sprintln!("  CLINK L8 found: {}", cl8_found);
            sprintln!("  ZFC_fe found: {}", zfcfe_found);
        }
        _ => {
            sprintln!("CL8NK Navigator — CLINK Layer 8 (Organism)");
            sprintln!("Actions:");
            sprintln!("  entry  <name>    — Full CL8NK formula decomposition");
            sprintln!("  promotions        — 3-stage ladder: ZFC→ZFCₜ→ZFC_fe→CLINK L8");
            sprintln!("  distance <name>   — d(name, CLINK L8)");
            sprintln!("  transcendence     — Ω/ɢ transcendence analysis");
            sprintln!("  tensor  <name>    — CLINK L8 ⊗ name (absorption test)");
            sprintln!("  meet    <name>    — CLINK L8 ⊓ name");
            sprintln!("  join    <name>    — CLINK L8 ⊔ name");
            sprintln!("  tier    <name>    — Ouroboricity tier assessment");
            sprintln!("  chain             — Full CLINK chain L0→L8 distance ladder");
            sprintln!("  systems           — All catalog systems");
            sprintln!("  stats             — Catalog statistics + reference tuples");
        }
    }
}

fn print_c4_arg(arg: &str) {
    use crate::belnap_c4::*;
    match arg {
        "born" | "table" => c4_born_table(),
        "mul" | "multiply" => {
            sprintln!("C₄ Multiplication Table (16×16)");
            sprintln!("A * B for A,B ∈ {{N,F,T,B}}×{{N,F,T,B}}");
            let table = c4_multiplication_table();
            for (i, row) in table.iter().enumerate().take(16) {
                sprintln!("  row {}: {} ... ({} cols)", i, row[0], row.len());
            }
        }
        "probe" | "test" | "" => {
            let i = BelnapComplex::i();
            let i2 = c4_mul(&i, &i);
            let conj = i.conjugate();
            sprintln!("i = N + Ti");
            sprintln!("i² = {}  (dialetheic: B = both true and false)", c4_format(&i2));
            sprintln!("conj(i) = {}", c4_format(&conj));
            sprintln!("|i|² = {}  → born P = {:.2}", c4_format(&BelnapComplex::new(i.magnitude_squared(), crate::belnap::B4::N)), i.born_probability());
            sprintln!("|1|² = {}  → born P = {:.2}", c4_format(&BelnapComplex::new(BelnapComplex::one().magnitude_squared(), crate::belnap::B4::N)), BelnapComplex::one().born_probability());
        }
        _ => {
            sprintln!("C₄ Belnap Complex Plane");
            sprintln!("  Usage: grammar c4 [probe|born|mul]");
            sprintln!("  probe   — test i² = B (dialetheic i)");
            sprintln!("  born    — Born rule table (all 16 C₄ elements)");
            sprintln!("  mul     — Multiplication table");
        }
    }
}

fn print_cscore(k: &Kernel) {
    use crate::consciousness::consciousness_eval;
    use crate::imas_ig::IgTuple;

    if let Some(snap) = k.snapshot {
        let ig = IgTuple::from_snapshot(&snap);
        let r = consciousness_eval(&ig);
        sprintln!("══ Consciousness Score ══");
        sprintln!("  C-score:    {:.4}", r.c_score);
        sprintln!("  Gate 1 (⊙): {}", if r.gate1_open { "OPEN" } else { "CLOSED" });
        sprintln!("  Gate 2 (K): {}", if r.gate2_open { "OPEN" } else { "CLOSED" });
        sprintln!("  Basal:      {:.4}", r.basal);
        sprintln!("  Components:");
        for i in 0..10 {
            sprintln!("    {}: {:.2}", r.component_names[i], r.components[i]);
        }
        if r.c_score == 0.0 && !r.gate1_open {
            sprintln!("  ⚠ Gate 1 failed — no self-modeling loop");
        }
        if r.c_score == 0.0 && !r.gate2_open {
            sprintln!("  ⚠ Gate 2 failed — kinetics too fast for integration");
        }
    } else {
        sprintln!("No snapshot. Tick first.");
    }
}

fn print_clay() {
    sprintln!("{}", crate::clay_status::formatted_report());
}

fn print_sic() {
    sprintln!("{}", crate::sic_povm::formatted_report());
}

fn print_cr3(sub: &str, rest: alloc::string::String) {
    use crate::cr3echrz::p3theorem::{run_theorem, format_theorem_result, list_theorems};
    use crate::cr3echrz::p4rakernel::list_p4ra_modules;
    use crate::cr3echrz::vault::{list_vault_ob3ects, run_vault_ob3ect, vault_domain_summary};

    match sub {
        "" | "--help" => {
            sprintln!("cr3 — Unified Theorem Operationalization Engine (dynamic registry)");
            sprintln!("  cr3 --list                List all registered theorems + p4rakernel modules");
            sprintln!("  cr3 --list-theorems       List p3theorem engine");
            sprintln!("  cr3 --list-ob3ects [domain]  List vault ob3ects (281)");
            sprintln!("  cr3 --version             Show version");
            sprintln!("  cr3 <theorem> [params]    Run a registered theorem");
            sprintln!("  cr3 <ob3ect_name>         Run a vault ob3ect");
            sprintln!("");
            sprintln!("{}", list_theorems());
            sprintln!("");
            sprintln!("For Belnap+Frobenius 13-step p4rakernel versions: use 'p4ra' command");
            sprintln!("  p4ra --list                  List p4rakernel modules");
        }
        "--list" => {
            sprintln!("{}", list_theorems());
            sprintln!("");
            sprintln!("{}", list_p4ra_modules());
            sprintln!("");
            sprintln!("{}", vault_domain_summary());
        }
        "--list-theorems" => {
            sprintln!("{}", list_theorems());
        }
        "--list-ob3ects" => {
            let domain = rest.split_whitespace().next();
            sprintln!("{}", list_vault_ob3ects(domain));
        }
        "--version" => {
            sprintln!("cr3 v1.2 — Unified Theorem Operationalization Engine (dynamic registry)");
            sprintln!("Author: Lando⊗⊙perator");
            sprintln!("Phase 10: fn-pointer dispatch, runtime-extensible registries");
            sprintln!("281 vault ob3ects + 7 theorems + 6 p4rakernel modules");
            sprintln!("12 universal IMASM opcodes");
        }
        _ => {
            // Try theorem first, then vault ob3ect
            let result = run_theorem(sub, &rest);
            if result.status == crate::belnap::B4::N {
                // Not a theorem — try vault
                sprintln!("{}", run_vault_ob3ect(sub));
            } else {
                sprintln!("{}", format_theorem_result(&result));
            }
        }
    }
}

fn print_p4ra(sub: &str, rest: alloc::string::String) {
    use crate::cr3echrz::p4rakernel::{run_p4ra_module, format_p4ra_result, list_p4ra_modules};

    match sub {
        "" | "--help" => {
            sprintln!("p4ra — p4rakernel Belnap+Frobenius 13-step IMASM Bootstrap");
            sprintln!("  6 standalone theorem modules with Belnap FOUR + Frobenius verification");
            sprintln!("");
            sprintln!("{}", list_p4ra_modules());
            sprintln!("");
            sprintln!("Examples:");
            sprintln!("  p4ra burnside 2 5              B(2,5) — PARADOX");
            sprintln!("  p4ra burnside 2 665 1 2 -1 -2  B(2,665) — INFINITE (Adian 1979)");
            sprintln!("  p4ra connes R                  R — EMBEDDABLE");
            sprintln!("  p4ra connes \"L(F_2)\"           L(F_2) — NON-EMBEDDABLE (JNVWY 2020)");
            sprintln!("  p4ra erdos_straus 73           Erdős–Straus 4/73");
            sprintln!("  p4ra goldbach 100              Goldbach: 100 = 3+97 = ...");
            sprintln!("  p4ra goldbach 30               Goldbach: 30 = 7+23 = 11+19 = 13+17");
            sprintln!("  p4ra landau Koebe              Landau: Koebe omits -1/4");
            sprintln!("  p4ra landau Dense              Landau: Dense (unbounded)");
            sprintln!("  p4ra landau Picard             Landau: Essential singularity");
            sprintln!("  p4ra threebody                 Three-Body: KAM boundary");
        }
        "--list" => {
            sprintln!("{}", list_p4ra_modules());
        }
        _ => {
            let result = run_p4ra_module(sub, &rest);
            sprintln!("{}", format_p4ra_result(&result));
        }
    }
}

fn print_rebis(sub: &str, arg: &str, rest: &str) {
    use crate::rebis::codon::{Codon, CodeTable, translate_codon, classify_stratum, stratum_counts, verify_frobenius};
    use crate::rebis::genetics::{GeneticVerification, codons_for_aa, codon_distance, promoted_amino_acids, ALL_AMINO_ACIDS};
    use crate::rebis::translate::{run_pipeline_table, run_reverse_pipeline, format_chain, format_chain_1letter, parse_aa, aa_letter, parse_chain, reverse_translate_aa, codon_to_rna, enumerate_mrna, roundtrip_verify};
    use crate::rebis::fold::fold_sequence;
    use crate::rebis::hadron::{HadronState, HadronType, proton_quarks, neutron_quarks, pion_plus_quarks};
    use crate::rebis::serpent::{find_motif, motif_signature, MOTIFS};
    use crate::rebis::pipeline::{IgTuple, run_promotion_pipeline};
    use crate::rebis::genetic_asm::{all_genetic_programs, codon_to_b4};
    use crate::rebis::genetic_tuples::{generate_all_stages, StageContext, verify_monotonic_advance, tuple_crystal_address};
    use crate::rebis::clu::{run_walk, verify_power_law, avalanche_probability, tier_from_position, Point3D, CLUCluster};
    use crate::rebis::exotic_hadron::{Glueball, Tetraquark, Pentaquark, QColor, GluonColor};
    use crate::rebis::pdb::{parse_pdb_ca_atoms, extract_contacts, extract_sequence_from_pdb, validate_structure};
    use crate::rebis::antibody::{analyze_epitope, design_cdr, design_full_antibody};
    use crate::rebis::materials::forge_material;
    use crate::rebis::biology::{TissueGrid, FrobeniusBioSim};
    use crate::rebis::therapeutics::Chemotherapeutic;
    use crate::rebis::clink;
    


    match sub {
        "codon" => {
            let s = if arg.is_empty() { rest } else { arg };
            // Try codon (3 nucleotides) first
            if let Ok(c) = Codon::from_str(s) {
                let aa = translate_codon(&c);
                let stratum = classify_stratum(&c);
                let (holds, _) = crate::rebis::codon::verify_frobenius(&c);
                sprintln!("Codon: {} -> {}", core::str::from_utf8(&c.symbol()).unwrap_or("???"), aa.name());
                sprintln!("  Stratum: {:?}", stratum);
                sprintln!("  Frobenius: {}", if holds { "PASS" } else { "FAIL" });
                sprintln!("  Index: {}", c.index());
            }
            // Try amino acid name/code → all codons
            else if let Some(aa) = parse_aa(s) {
                let hit = reverse_translate_aa(aa);
                sprintln!("AA: {} ({}) [{}]", aa.name(), aa_letter(aa), aa.to_primitive().map_or("—", |p| p.glyph()));
                sprintln!("  Degeneracy: {}", hit.codon_count);
                sprintln!("  Codons:");
                for c in &hit.codons {
                    let sym = codon_to_rna(c);
                    let strat = classify_stratum(c);
                    sprintln!("    {}{}{}  idx={:2}  stratum={:?}",
                        sym[0] as char, sym[1] as char, sym[2] as char,
                        c.index(), strat);
                }
            }
            else {
                sprintln!("Error: '{}' is not a valid codon (3 nt) or amino acid (3-letter, 1-letter, or name)", s);
                sprintln!("Codons: AUG, UUU, GCA...  |  Amino acids: Phe/F, Leu/L, Met/M, Lys/K, Gly/G, Stop/*...");
            }
        }
        "translate" => {
            if arg.is_empty() && rest.is_empty() {
                sprintln!("Usage: rebis translate <DNA> [mito]");
                sprintln!("  mito — use vertebrate mitochondrial code");
                sprintln!("Example: rebis translate ATGGCC");
                sprintln!("         rebis translate ATGGCC mito");
                return;
            }
            // Parse: seq [mito]
            let (seq, table) = if arg == "mito" {
                (rest, CodeTable::Mitochondrial)
            } else if rest == "mito" {
                (arg, CodeTable::Mitochondrial)
            } else {
                let s = if arg.is_empty() { rest } else { arg };
                (s, CodeTable::Standard)
            };
            let result = run_pipeline_table(seq.as_bytes(), table);
            let table_name = match table { CodeTable::Standard => "standard", CodeTable::Mitochondrial => "mitochondrial" };
            sprintln!("DNA:          {}", seq);
            sprintln!("mRNA:         {}", core::str::from_utf8(&result.mrna).unwrap_or("???"));
            sprintln!("Code table:   {}", table_name);
            sprintln!("Protein:      {}", format_chain(&result.protein));
            sprintln!("Coding:       {} bp", result.coding_length);
            sprintln!("Frobenius:    {}", if result.frobenius_verified { "PASS" } else { "FAIL" });
            // Per-AA primitive annotation
            let non_stop: alloc::vec::Vec<_> = result.protein.iter().zip(result.primitive_labels.iter())
                .filter(|(&aa, _)| aa != crate::rebis::AminoAcid::Stop)
                .collect();
            if !non_stop.is_empty() {
                sprintln!("Primitives:");
                for (&aa, prim) in &non_stop {
                    if let Some(name) = prim {
                        sprintln!("  {} → {}", aa.name(), name);
                    } else {
                        sprintln!("  {} → (ground layer)", aa.name());
                    }
                }
            }
        }

        "box" => {
            // Box stratification: show all 16 (p1,p2) boxes
            use crate::belnap::B4;
            let positions = [crate::belnap::B4::N, B4::F, B4::T, B4::B];
            let labels = ["N(U)", "F(A)", "T(C)", "B(G)"];
            sprintln!("Codon Box Stratification (16 boxes, p1×p2):");
            sprintln!("  Box    RNA  Stratum  Codons  AAs");
            for (i, &p1) in positions.iter().enumerate() {
                for (j, &p2) in positions.iter().enumerate() {
                    let sample = Codon { p1, p2, p3: crate::belnap::B4::N };
                    let strat = classify_stratum(&sample);
                    // Collect all 4 codons and their AAs
                    let mut aas = alloc::vec::Vec::new();
                    for &p3 in &positions {
                        let c = Codon { p1, p2, p3 };
                        let aa = translate_codon(&c);
                        let sym = c.symbol();
                        let rna: alloc::string::String = [sym[0] as char, sym[1] as char, sym[2] as char].iter().collect();
                        aas.push(alloc::format!("{}={}", rna, aa.name()));
                    }
                    sprintln!("  ({},{})  {:5?}  {}",
                        labels[i], labels[j], strat, aas.join("  "));
                }
            }
        }

        "crystal" => {
            // Crystal divisibility: 17,280,000 / 64
            let total: u64 = crate::crystal::TOTAL as u64;
            let codons: u64 = 64;
            let quotient = total / codons;
            let remainder = total % codons;
            sprintln!("Crystal / Codon space divisibility:");
            sprintln!("  Crystal of Types: {} addresses", total);
            sprintln!("  Codon space:      {} codons", codons);
            sprintln!("  Quotient:         {}", quotient);
            sprintln!("  Remainder:        {}", remainder);
            sprintln!("  Exact division:   {}", if remainder == 0 { "YES" } else { "NO" });
            if remainder == 0 {
                sprintln!("  Each codon maps to exactly {} crystal addresses", quotient);
            }
        }

        "stop" => {
            // Stop codon analysis as Ω boundary
            use crate::belnap::B4;
            sprintln!("Stop Codon Analysis (Ω boundary — kernel winding limit):");
            let stops = [
                ("UAA", Codon { p1: crate::belnap::B4::N, p2: B4::F, p3: B4::F }, "Ω₀  trivial winding — null boundary"),
                ("UAG", Codon { p1: crate::belnap::B4::N, p2: B4::F, p3: B4::B }, "Ω_Z₂  Z2-protected — amber boundary"),
                ("UGA", Codon { p1: crate::belnap::B4::N, p2: B4::B, p3: B4::F }, "Ω_Z   integer winding — opal boundary"),
            ];
            for (name, codon, desc) in &stops {
                let s = codon.symbol();
                sprintln!("  {} ({}{}{})  B4: ({:?},{:?},{:?})  {}",
                    name, s[0] as char, s[1] as char, s[2] as char,
                    codon.p1, codon.p2, codon.p3, desc);
            }
            sprintln!("  Mito additional stops: AGA (F,B,F)=Ω_AGA  AGG (F,B,B)=Ω_AGG");
            sprintln!("  Mito UGA → Trp (not Stop — Ω gate lifted in mitochondrial context)");
        }

        "mutation" => {
            // B4 edit distance between two amino acids
            if arg.is_empty() || rest.is_empty() {
                sprintln!("Usage: rebis mutation <AA1> <AA2>");
                sprintln!("  Computes minimum B4 edit distance between codon sets");
                sprintln!("Example: rebis mutation Met Ala");
                return;
            }
            match (parse_aa(arg), parse_aa(rest)) {
                (Some(aa1), Some(aa2)) => {
                    let codons1 = codons_for_aa(aa1);
                    let codons2 = codons_for_aa(aa2);
                    if codons1.is_empty() || codons2.is_empty() {
                        sprintln!("No codons found for one or both AAs");
                        return;
                    }
                    let mut min_dist = u8::MAX;
                    let mut best_from = codons1[0];
                    let mut best_to = codons2[0];
                    for &c1 in &codons1 {
                        for &c2 in &codons2 {
                            let d = codon_distance(&c1, &c2);
                            if d < min_dist {
                                min_dist = d;
                                best_from = c1;
                                best_to = c2;
                            }
                        }
                    }
                    let s1 = best_from.symbol();
                    let s2 = best_to.symbol();
                    sprintln!("Mutation: {} → {}", aa1.name(), aa2.name());
                    sprintln!("  Min B4 edit distance: {}", min_dist);
                    sprintln!("  Optimal path: {}{}{} → {}{}{}",
                        s1[0] as char, s1[1] as char, s1[2] as char,
                        s2[0] as char, s2[1] as char, s2[2] as char);
                    sprintln!("  {} codons → {} codons", codons1.len(), codons2.len());
                    let prim1 = aa1.primitive_name();
                    let prim2 = aa2.primitive_name();
                    if prim1.is_some() || prim2.is_some() {
                        sprintln!("  Primitive crossing: {} → {}",
                            prim1.unwrap_or("(ground)"), prim2.unwrap_or("(ground)"));
                    }
                    // Risk assessment based on stratum crossing
                    let s1_type = classify_stratum(&best_from);
                    let s2_type = classify_stratum(&best_to);
                    sprintln!("  Stratum: {:?} → {:?}", s1_type, s2_type);
                }
                _ => sprintln!("Unknown amino acid. Use 3-letter (Met), 1-letter (M), or full name."),
            }
        }

        "verify-codons" => {
            // Full per-codon Frobenius verification table
            use crate::belnap::B4;
            let positions = [crate::belnap::B4::N, B4::F, B4::T, B4::B];
            sprintln!("Per-Codon Frobenius Verification Table (64 codons):");
            sprintln!("  Codon  B4(p1,p2,p3)      AA    Stratum  Frob  Primitive");
            let mut pass = 0usize;
            let mut fail = 0usize;
            for &p1 in &positions {
                for &p2 in &positions {
                    for &p3 in &positions {
                        let c = Codon { p1, p2, p3 };
                        let aa = translate_codon(&c);
                        let sym = c.symbol();
                        let (holds, strat) = verify_frobenius(&c);
                        let prim = aa.primitive_name().unwrap_or("-");
                        if holds { pass += 1; } else { fail += 1; }
                        sprintln!("  {}{}{}    ({:?},{:?},{:?})  {:4}  {:5?}  {}  {}",
                            sym[0] as char, sym[1] as char, sym[2] as char,
                            p1, p2, p3, aa.name(),
                            strat, if holds { "PASS" } else { "FAIL" }, prim);
                    }
                }
            }
            sprintln!("  Summary: {} PASS, {} FAIL", pass, fail);
        }

        "primitives" => {
            // Show the 12-primitive ↔ AA bijection
            let promoted = promoted_amino_acids();
            sprintln!("IG Primitive ↔ Amino Acid Bijection ({} promoted AAs):", promoted.len());
            for aa in &promoted {
                if let Some(prim) = aa.primitive_name() {
                    let codons = codons_for_aa(*aa);
                    sprintln!("  {} ({}) → {} [{} codon{}]",
                        aa.name(), aa.code1(), prim, codons.len(),
                        if codons.len() == 1 { "" } else { "s" });
                }
            }
            sprintln!("Ground layer AAs (exact stratum, no primitive bijection):");
            for &aa in &ALL_AMINO_ACIDS {
                if aa == crate::rebis::AminoAcid::Stop { continue; }
                if aa.primitive_name().is_none() {
                    let codons = codons_for_aa(aa);
                    sprintln!("  {} ({}) [{} codon{}]",
                        aa.name(), aa.code1(), codons.len(),
                        if codons.len() == 1 { "" } else { "s" });
                }
            }
        }

        "reverse" => {
            if arg.is_empty() && rest.is_empty() {
                sprintln!("Usage: rebis reverse <protein sequence>");
                sprintln!("  Protein → mRNA → DNA (reverse translation)");
                sprintln!("Examples:");
                sprintln!("  rebis reverse Met-Ala-Gly    (3-letter codes, dash-separated)");
                sprintln!("  rebis reverse MAG            (1-letter codes)");
                sprintln!("  rebis reverse M A G          (1-letter codes, space-separated)");
                return;
            }
            let input = if arg.is_empty() { String::from(rest) } else {
                if rest.is_empty() { String::from(arg) }
                else { alloc::format!("{} {}", arg, rest) }
            };
            match parse_chain(&input) {
                Some(chain) if !chain.is_empty() => {
                    let result = run_reverse_pipeline(&chain);
                    sprintln!("Protein → RNA → DNA (reverse translation)");
                    sprintln!("  Input:     {}", format_chain(&chain));
                    sprintln!("  1-letter:  {}", format_chain_1letter(&chain));
                    sprintln!("  Length:    {} AA", chain.len());
                    sprintln!("  Canonical mRNA: {}", core::str::from_utf8(&result.canonical_mrna).unwrap_or("???"));
                    sprintln!("  DNA:       {}", core::str::from_utf8(&result.dna).unwrap_or("???"));
                    sprintln!("  Degeneracy per position:");
                    for (i, (&aa, &deg)) in chain.iter().zip(result.degeneracies.iter()).enumerate() {
                        sprintln!("    [{}] {} ({}) — {} codon{}",
                            i+1, aa.name(), aa_letter(aa), deg, if deg==1 { "" } else { "s" });
                    }
                    sprintln!("  Total possible mRNA sequences: {} (degeneracy product)", result.total_combinations);

                    // Round-trip verify: Protein→mRNA→Protein
                    let (_orig, _round, matches, total) = roundtrip_verify(&chain);
                    sprintln!("  Round-trip (canonical): {}/{} AA match", matches, total);

                    // If total combinations ≤ 256, enumerate all
                    if result.total_combinations > 0 && result.total_combinations <= 256 {
                        let all = enumerate_mrna(&chain);
                        sprintln!("  All {} possible mRNA sequences:", all.len());
                        for (i, seq) in all.iter().enumerate() {
                            sprintln!("    {:3}: {}", i+1, core::str::from_utf8(seq).unwrap_or("???"));
                        }
                    } else if result.total_combinations > 256 {
                        sprintln!("  ({} total combinations — too many to enumerate; use shorter chain)", result.total_combinations);
                    }

                    // Per-AA detail
                    sprintln!("  Per-position codon table:");
                    for (_i, &aa) in chain.iter().enumerate() {
                        let hit = reverse_translate_aa(aa);
                        let mut cstr = String::new();
                        for (j, c) in hit.codons.iter().enumerate() {
                            if j > 0 { cstr.push_str(", "); }
                            let sym = codon_to_rna(c);
                            cstr.push(sym[0] as char);
                            cstr.push(sym[1] as char);
                            cstr.push(sym[2] as char);
                        }
                        sprintln!("    {}: {} → [{}]", aa.name(), aa_letter(aa), cstr);
                    }
                }
                Some(_) => sprintln!("Error: empty protein chain"),
                None => sprintln!("Error: could not parse '{}' as amino acid sequence. Use 3-letter (Met-Ala) or 1-letter (MA) codes.", input),
            }
        }
        "frob" => {
            let (pass, fail, ratio) = crate::rebis::frob_filter::filter_codon_space();
            sprintln!("Frobenius Filtration (64 codons):");
            sprintln!("  Pass: {}", pass);
            sprintln!("  Fail: {}", fail);
            sprintln!("  Closure ratio: {:.4}", ratio);
            let sizes = [4, 8, 16, 32, 64];
            let alpha = crate::rebis::frob_filter::power_law_exponent(&sizes);
            sprintln!("  Power-law exponent α: {:.4}", alpha);
        }
        "genetics" => {
            let v = GeneticVerification::run();
            sprintln!("Genetic Code Verification (7 stages):");
            sprintln!("  Stage 1 (64 codons):     {}", if v.stage1_codon_count { "PASS" } else { "FAIL" });
            sprintln!("  Stage 2 (3 strata):     {}", if v.stage2_stratum_split { "PASS" } else { "FAIL" });
            sprintln!("  Stage 3 (21 classes):   {}", if v.stage3_aa_count { "PASS" } else { "FAIL" });
            sprintln!("  Stage 4 (12→12 bij):    {}", if v.stage4_promoted_bijection { "PASS" } else { "FAIL" });
            sprintln!("  Stage 5 (wobble):       {}", if v.stage5_wobble { "PASS" } else { "FAIL" });
            sprintln!("  Stage 6 (Frobenius):    {}", if v.stage6_frobenius { "PASS" } else { "FAIL" });
            sprintln!("  Stage 7 (crystal):      {}", if v.stage7_crystal { "PASS" } else { "FAIL" });
            sprintln!("  {}", v.report());
            let (exact, split, stop) = stratum_counts();
            sprintln!("  Strata: {} exact, {} split, {} stop", exact, split, stop);
        }
        "hadron" => {
            let p = HadronState::from_quarks(&proton_quarks(), HadronType::Baryon);
            let n = HadronState::from_quarks(&neutron_quarks(), HadronType::Baryon);
            let pi = HadronState::from_quarks(&pion_plus_quarks(), HadronType::Meson);
            sprintln!("Hadron Belnap Analysis:");
            sprintln!("  Proton:   conf={:?} par={:?} chg={:?} frob={}",
                p.confinement, p.parity, p.charge, p.frobenius_ok);
            sprintln!("  Neutron:  conf={:?} par={:?} chg={:?} frob={}",
                n.confinement, n.parity, n.charge, n.frobenius_ok);
            sprintln!("  Pion+:    conf={:?} par={:?} chg={:?} frob={}",
                pi.confinement, pi.parity, pi.charge, pi.frobenius_ok);
        }
        "serpent" => {
            if arg.is_empty() {
                sprintln!("Serpent Motifs:");
                for m in MOTIFS {
                    sprintln!("  {} ({} AA, tier O_{}, C={:.3})",
                        m.name, m.length, m.tier, m.c_score);
                }
                sprintln!("Usage: rebis serpent <motif_name>");
                return;
            }
            match find_motif(arg) {
                Some(m) => {
                    let (promoted, sig) = motif_signature(m);
                    sprintln!("Motif: {} ({} AA)", m.name, m.length);
                    sprintln!("  Tier: O_{}", m.tier);
                    sprintln!("  C-score: {:.4}", m.c_score);
                    sprintln!("  Frobenius: {}", if m.frobenius_ok { "PASS" } else { "FAIL" });
                    sprintln!("  Promoted AAs: {}/12", promoted);
                    sprintln!("  Primitive sig: {}", sig.join("·"));
                }
                None => sprintln!("Motif '{}' not found. Use 'rebis serpent' to list.", arg),
            }
        }

        "fold" => {
            if arg.is_empty() && rest.is_empty() {
                sprintln!("Usage: rebis fold <DNA|RNA> [mito]");
                sprintln!("  Translates DNA/RNA -> primary sequence, then predicts secondary");
                sprintln!("  and tertiary structure via Chou-Fasman + SerpentRod Frobenius.");
                sprintln!("  SerpentRod invariant: windingNumber <= contacts + 1");
                sprintln!("Example: rebis fold ATGGCCTATAAAGAG");
                sprintln!("         rebis fold AUGGCCUAUAAAGAG");
                sprintln!("         rebis fold ATGGCC mito");
                return;
            }
            let (seq, table) = if arg == "mito" {
                (rest, CodeTable::Mitochondrial)
            } else if rest == "mito" {
                (arg, CodeTable::Mitochondrial)
            } else {
                let s = if arg.is_empty() { rest } else { arg };
                (s, CodeTable::Standard)
            };
            let result = run_pipeline_table(seq.as_bytes(), table);
            let chain: alloc::vec::Vec<crate::rebis::AminoAcid> = result.protein.iter()
                .filter(|&&aa| aa != crate::rebis::AminoAcid::Stop).copied().collect();
            if chain.is_empty() {
                sprintln!("No protein translated from '{}'. Ensure sequence contains ATG/AUG start codon.", seq);
                return;
            }
            let fold = fold_sequence(&chain);
            let n = fold.residues.len();
            let table_name = match table { CodeTable::Standard => "standard", CodeTable::Mitochondrial => "mitochondrial" };
            sprintln!("══ SerpentRod Fold: {} residues ({}) ══", n, table_name);
            sprintln!("Sequence: {}", format_chain_1letter(&chain));
            sprintln!();
            // Per-residue table
            sprintln!("{:>4}  {:3}  {:1}  W#  Primitive", "Pos", "AA", "2°");
            sprintln!("---- ---  -  --  ---------");
            for r in &fold.residues {
                let prim = r.aa.primitive_name().unwrap_or("·");
                sprintln!("{:>4}  {:3}  {}  {:>2}  {}",
                    r.position + 1, r.aa.name(), r.secondary.symbol(),
                    r.winding_number, prim);
            }
            sprintln!();
            // Secondary element summary
            let n_h = fold.residues.iter().filter(|r| r.secondary == crate::rebis::fold::SecondaryLabel::Helix).count();
            let n_s = fold.residues.iter().filter(|r| r.secondary == crate::rebis::fold::SecondaryLabel::Sheet).count();
            let n_c = n - n_h - n_s;
            sprintln!("Secondary structure:");
            sprintln!("  Helix:  {:>3} residues ({:>2}%)", n_h, if n > 0 { n_h * 100 / n } else { 0 });
            sprintln!("  Sheet:  {:>3} residues ({:>2}%)", n_s, if n > 0 { n_s * 100 / n } else { 0 });
            sprintln!("  Coil:   {:>3} residues ({:>2}%)", n_c, if n > 0 { n_c * 100 / n } else { 0 });
            sprintln!();
            // Tertiary contacts
            let n_hydro = fold.contacts.iter().filter(|c| matches!(c.kind, crate::rebis::fold::ContactKind::Hydrophobic)).count();
            let n_ss    = fold.contacts.iter().filter(|c| matches!(c.kind, crate::rebis::fold::ContactKind::Disulfide)).count();
            let n_ionic = fold.contacts.iter().filter(|c| matches!(c.kind, crate::rebis::fold::ContactKind::Ionic)).count();
            sprintln!("Tertiary contacts: {} total", fold.contacts.len());
            sprintln!("  Hydrophobic: {}  Disulfide: {}  Ionic: {}", n_hydro, n_ss, n_ionic);
            if !fold.contacts.is_empty() {
                sprintln!("  Top contacts (by confidence):");
                let mut sorted: alloc::vec::Vec<_> = fold.contacts.iter().collect();
                sorted.sort_unstable_by(|a, b| b.confidence.cmp(&a.confidence));
                for c in sorted.iter().take(5) {
                    sprintln!("    {:<12} {:>3} <-> {:<3}  conf={}%",
                        c.kind.name(), c.i + 1, c.j + 1, c.confidence);
                }
            }
            sprintln!();
            sprintln!("SerpentRod invariant: {} (windingNumber <= contacts + 1)",
                if fold.frobenius_ok { "PASS" } else { "FAIL" });
            sprintln!("IG primitives activated: {}/12  Tier: {}",
                fold.unique_primitives, fold.ouroboricity_tier);
            let max_w = fold.residues.iter().map(|r| r.winding_number).max().unwrap_or(0);
            sprintln!("Max winding number: {}  Total contacts: {}", max_w, fold.contacts.len());
        }

        "pipeline" => {
            let source = match arg {
                "genetic" => IgTuple::GENETIC,
                "sm" | "standard" => IgTuple::STANDARD_MODEL,
                _ => IgTuple::GENETIC,
            };
            let target = IgTuple::IUG;
            let report = run_promotion_pipeline(&source, &target);
            sprintln!("{}", report.summary());
        }
        "strata" => {
            let (exact, split, stop) = stratum_counts();
            sprintln!("Codon Strata:");
            sprintln!("  Exact: {} codons (ffuse∘fsplit = id exactly)", exact);
            sprintln!("  Split: {} codons (ffuse∘fsplit = id mod Z2)", split);
            sprintln!("  Stop:  {} codons (Ω boundary)", stop);
        }
                "asm" => {
            let programs = all_genetic_programs();
            if arg.is_empty() {
                sprintln!("Genetic ParaASM Programs:");
                for p in &programs {
                    sprintln!("  {} ({} ops)", p.name, p.instructions.len());
                }
                sprintln!("Usage: rebis asm <program> [codon]");
            } else {
                let codon = if rest.is_empty() { "ATG" } else { rest };
                match arg {
                    "translate" => {
                        let b4 = codon_to_b4(codon);
                        sprintln!("Codon {} -> B4: [{:?}, {:?}, {:?}]", codon, b4[0], b4[1], b4[2]);
                    }
                    _ => sprintln!("Program '{}'. Use 'translate', 'stratum', or 'b4edit'.", arg),
                }
            }
        }
        "tuples" => {
            if arg.is_empty() && rest.is_empty() {
                sprintln!("Usage: rebis tuples <DNA seq>");
                return;
            }
            let seq = if arg.is_empty() { rest } else { arg };
            let ctx = StageContext {
                chain_length: 100, beta_branched_frac: 0.15, proline_frac: 0.05,
                glycine_frac: 0.07, hydrophobic_frac: 0.35, aromatic_frac: 0.08,
                cysteine_count: 2, helix_content: 0.30, sheet_content: 0.25,
                contact_diversity: 0.60, subunit_count: 2, has_symmetry: false,
                disulfide_bonds: 1,
            };
            let stages = generate_all_stages(&ctx);
            let monotonic = verify_monotonic_advance(&stages);
            sprintln!("7-Stage Generative Tuple Pipeline for: {}", seq);
            let stage_names = ["DNA","Transcription","Codon","Translation","Folding","Tertiary","Quaternary"];
            for i in 0..7 {
                let addr = tuple_crystal_address(&stages[i]);
                let _g = stages[i].d.glyph();
                sprintln!("  Stage {} ({}): crystal={}  D={} T={} R={} P={}",
                    i+1, stage_names[i], addr,
                    stages[i].d.glyph(), stages[i].t.glyph(),
                    stages[i].r.glyph(), stages[i].p.glyph());
            }
            sprintln!("  Monotonic advance: {}", if monotonic { "PASS" } else { "FAIL" });
        }
        "clu" => {
            match arg {
                "walk" => {
                    let steps: usize = rest.parse().unwrap_or(100);
                    let walk = run_walk(steps);
                    sprintln!("CLU Walk ({} steps):", steps);
                    sprintln!("  Start: tier={}", tier_from_position(&walk.origin));
                    sprintln!("  End:   tier={} K={:.3}", tier_from_position(&walk.pos), walk.pos.k);
                    sprintln!("  Steps: {}", walk.step_count);
                }
                "verify" => {
                    let sizes = [4usize, 8, 16, 32, 64];
                    let mut clusters = alloc::vec::Vec::new();
                    for &s in &sizes {
                        let pts: alloc::vec::Vec<Point3D> = (0..s).map(|i| Point3D {
                            k: i % 5,
                            h: (i % 8) % 4,
                            w: if i % 2 == 0 { 1 } else { 0 },
                        }).collect();
                        let tier_name = tier_from_position(&pts[0]);
                        clusters.push(CLUCluster { center: pts[0], members: pts, size: s, tier: tier_name });
                    }
                    let fit = verify_power_law(&clusters);
                    sprintln!("CLU Power-Law: alpha={:.4} R2={:.4} pass={}",
                        fit.exponent, fit.r_squared, if fit.passes_test { "PASS" } else { "FAIL" });
                }
                "avalanche" => {
                    let s: usize = rest.parse().unwrap_or(10);
                    sprintln!("Avalanche P(S={}) = {:.6}  (S^(-3/2) = {:.6})",
                        s, avalanche_probability(s), crate::rebis::clu::powf_approx(s as f64, -1.5));
                }
                _ => sprintln!("CLU: walk [steps] | verify | avalanche <S>"),
            }
        }
        "exotic" => {
                        let gb = Glueball::from_slice(&[GluonColor::RG, GluonColor::GB]);
            let tq = Tetraquark::new(QColor::Red, QColor::Green, QColor::AntiRed, QColor::AntiGreen);
            let pq = Pentaquark::new([QColor::Red, QColor::Green, QColor::Blue, QColor::Red], QColor::AntiRed);
            sprintln!("Exotic Hadrons:");
            match gb {
                Some(g) => sprintln!("  Glueball(2g): {} gluons", g.gluons.len()),
                None => sprintln!("  Glueball(2g): INVALID"),
            }
            sprintln!("  Tetraquark: {}", if tq.is_some() { "valid" } else { "INVALID" });
            sprintln!("  Pentaquark: {}", if pq.is_some() { "valid" } else { "INVALID" });
        }
        "pdb" => {
            if arg.is_empty() {
                sprintln!("PDB: validate <text> | contacts <text> | seq <text>");
                return;
            }
            let pdb_text = rest;
            match arg {
                "validate" => {
                    let v = validate_structure("input", pdb_text, &[], None);
                    sprintln!("PDB Validation: atoms={} seq_len={} exp_contacts={} pred_contacts={}",
                        v.n_ca_atoms, v.seq_length, v.experimental_contacts, v.predicted_contacts);
                    sprintln!("  Precision={:.4} Recall={:.4} Frobenius={}",
                        v.metrics.precision, v.metrics.recall, if v.frobenius_verified { "PASS" } else { "FAIL" });
                }
                "contacts" => {
                    let atoms = parse_pdb_ca_atoms(pdb_text);
                    let contacts = extract_contacts(&atoms, 8.0, 3);
                    sprintln!("Contacts: {} CA atoms -> {} contacts (cutoff=8.0A)", atoms.len(), contacts.len());
                    for c in contacts.iter().take(8) {
                        sprintln!("  Residue {} <-> {}  dist={:.2}A", c.i, c.j, c.distance);
                    }
                }
                "seq" => {
                    let seq = extract_sequence_from_pdb(pdb_text);
                    sprintln!("Sequence: {} ({} residues)", seq, seq.len());
                }
                _ => sprintln!("Unknown PDB action. Use: validate | contacts | seq"),
            }
        }
        "antibody" => {
            match arg {
                "epitope" => {
                    if rest.is_empty() { sprintln!("Usage: rebis antibody epitope <AA seq>"); return; }
                    let a = analyze_epitope(rest, "target");
                    sprintln!("Epitope: {} ({} residues)", a.name, a.seq_length);
                    for s in &a.activations {
                        sprintln!("  Pos {}: {} -> prim {}", s.position, s.aa, s.primitive);
                    }
                }
                "design" => {
                    if rest.is_empty() { sprintln!("Usage: rebis antibody design <AA seq>"); return; }
                    let a = analyze_epitope(rest, "target");
                    let cdr = design_cdr(&a, 12);
                    sprintln!("CDR Design: len={} seq={}", cdr.length, cdr.cdr_sequence);
                    for pos in cdr.composition.iter().take(6) {
                        sprintln!("  Pos {}: {} -> prim {}", pos.position, pos.aa, pos.primitive);
                    }
                }
                "full" => {
                    if rest.is_empty() { sprintln!("Usage: rebis antibody full <AA seq>"); return; }
                    let a = analyze_epitope(rest, "target");
                    let ab = design_full_antibody(&a, "rabivis", None);
                    sprintln!("Antibody: chain={}", ab.chain_type);
                    sprintln!("  Full seq: {}aa", ab.full_sequence.len());
                    sprintln!("  CDR3: {} residues", ab.cdr3.length);
                }
                "viral" => {
                    sprintln!("Viral Epitope Targets:");
                    for ve in crate::rebis::antibody::VIRAL_EPITOPES {
                        sprintln!("  {}: {}", ve.name, ve.sequence);
                    }
                }
                _ => sprintln!("Antibody: epitope <seq> | design <seq> | full <seq> | viral"),
            }
        }
        "material" | "materials" => {
            match arg {
                "forge" => {
                    // Forge a material from a 12-glyph IG tuple
                    let predefined = crate::rebis::materials::predefined_novel_materials();
                    if rest.is_empty() {
                        sprintln!("══ IG Material Forge ══");
                        sprintln!("  Predefined materials:");
                        for (name, _) in &predefined {
                            sprintln!("    {}", name);
                        }
                        sprintln!("  Usage: rebis material forge <name>   or   rebis material forge --all");
                        return;
                    }
                    if rest == "--all" {
                        sprintln!("{}", crate::rebis::materials::forge_report());
                    } else {
                        let name = rest.trim();
                        if let Some((_, tuple)) = predefined.iter().find(|(n, _)| n.as_str() == name) {
                            let spec = forge_material(name, tuple[0], tuple[1], tuple[2], tuple[3],
                                tuple[4], tuple[5], tuple[6], tuple[7], tuple[8], tuple[9], tuple[10], tuple[11]);
                            sprintln!("Forged: {}", spec.summary());
                            sprintln!("  {}", spec.structure_type);
                            sprintln!("  synthesis: {} | interface: {}", spec.synthesis_method, spec.interface_type);
                            sprintln!("  bond: {:.0}-{:.0} kJ/mol  symmetry: {}",
                                spec.bond_energy_kjmol.0, spec.bond_energy_kjmol.1, spec.symmetry_class);
                            sprintln!("  Frobenius: {}  C-score: {:.3}",
                                if spec.frobenius_verified { "PASS" } else { "FAIL" }, spec.c_score);
                        } else {
                            sprintln!("Unknown material: '{}'. Use 'rebis material forge' to list.", name);
                        }
                    }
                }
                "alloy" => {
                    let mut alloy = crate::rebis::materials::OuroboricAlloy::new(64);
                    let result = alloy.run_mechanical_test(800.0, 40);
                    sprintln!("══ Ouroboric Alloy (64 grains) ══");
                    sprintln!("  Cycles: {}", result.cycles);
                    sprintln!("  Damage fraction: {:.4}", result.damage_fraction);
                    sprintln!("  Final stress: {:.1} MPa", result.final_stress_mpa);
                    sprintln!("  Frobenius maintained: {}", if result.frobenius_maintained { "YES" } else { "NO" });
                    sprintln!("  Closure ratio: {:.4}", result.closure_ratio);
                }
                "thermal" => {
                    let specs = crate::rebis::materials::forge_all_predefined();
                    if specs.len() >= 2 {
                        let tr = crate::rebis::materials::ThermalRectifier::new(&specs[0], &specs[6]);
                        sprintln!("{}", tr.report());
                    }
                }
                "qc" => {
                    sprintln!("{}", crate::rebis::materials::paradigm_summary_table());
                }
                "sophick" => {
                    sprintln!("{}", crate::rebis::materials::sophick_report());
                }
                "exactor" | "gap" => {
                    let predefined = crate::rebis::materials::predefined_novel_materials();
                    if let Some((_name, tuple)) = predefined.first() {
                        sprintln!("{}", crate::rebis::materials::closure_diagnosis(tuple));
                    }
                    // Also show gap between Ouroboric O2 and Sophick Mercury
                    let gc = crate::rebis::materials::GapClosure::new(
                        crate::rebis::materials::OUROBORIC_O2,
                        crate::rebis::materials::SOPHICK_MERCURY,
                    );
                    sprintln!("
{}", gc.report());
                }
                "report" | _ => {
                    sprintln!("{}", crate::rebis::materials::forge_report());
                    sprintln!("
══ Quick Reference ══");
                    sprintln!("  rebis material forge [name|--all]  — forge materials from IG tuples");
                    sprintln!("  rebis material alloy               — Ouroboric alloy simulation");
                    sprintln!("  rebis material thermal             — Thermal rectifier design");
                    sprintln!("  rebis material qc                  — Non-qubit QC paradigm table");
                    sprintln!("  rebis material sophick             — Sophick Forge Eagle Cycle");
                    sprintln!("  rebis material exactor             — Frobenius closure diagnosis");
                    sprintln!("  rebis material report              — Full materials report");
                }
            }
        }
        "sidechain" => {
            match arg {
                "analyze" | "" => {
                    use crate::rebis::sidechain;
                    let results = sidechain::batch_analyze();
                    sprintln!("══ AA Sidechain × Environment Composition ({} pairs) ══", results.len());
                    sprintln!("  {:<5} {:<14} {:<14} {:<8} {:<8} {:<8} {}",
                        "#", "Sidechain", "Environment", "Tensor", "Meet", "Join", "Bottlenecks");
                    for (i, a) in results.iter().enumerate().take(20) {
                        sprintln!("  {:<5} {:<14} {:<14} {:<8.2} {:<8.2} {:<8.2} {}",
                            i+1, a.sidechain, a.environment, a.distance_tensor_sc, a.distance_pre, a.asymmetry, a.n_bottlenecks);
                    }
                    if results.len() > 20 {
                        sprintln!("  ... {} more pairs", results.len() - 20);
                    }
                }
                "list" => {
                    use crate::rebis::sidechain;
                    let sc = sidechain::all_sidechains();
                    sprintln!("══ All 20 AA Sidechains ══");
                    for (name, _) in sc {
                        sprintln!("  {}", name);
                    }
                    sprintln!();
                    let env = sidechain::all_environments();
                    sprintln!("══ 4 Environments ══");
                    for (name, _) in env {
                        sprintln!("  {}", name);
                    }
                }
                "frustration" => {
                    let mat = crate::rebis::sidechain::frustration_matrix();
                    sprintln!("══ Frustration Matrix (min tensor distance per pair) ══");
                    sprintln!("  {:<16} {:<16} {:<10}", "Sidechain", "Environment", "Dist");
                    for (sc, env, d) in &mat {
                        sprintln!("  {:<16} {:<16} {:<10.2}", sc, env, d);
                    }
                }
                _ => {
                    let sc = crate::rebis::sidechain::lookup_sidechain(arg);
                    let env = crate::rebis::sidechain::lookup_environment(arg);
                    if let Some(tup) = sc {
                        sprintln!("Sidechain '{}' tuple:", arg);
                        sprintln!("  ⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
                            tup.d.glyph(), tup.t.glyph(), tup.r.glyph(), tup.p.glyph(),
                            tup.f.glyph(), tup.k.glyph(), tup.g.glyph(), tup.c.glyph(),
                            tup.phi.glyph(), tup.h.glyph(), tup.s.glyph(), tup.omega.glyph());
                    } else if let Some(tup) = env {
                        sprintln!("Environment '{}' tuple:", arg);
                        sprintln!("  ⟨{}{}{}{}{}{}{}{}{}{}{}{}⟩",
                            tup.d.glyph(), tup.t.glyph(), tup.r.glyph(), tup.p.glyph(),
                            tup.f.glyph(), tup.k.glyph(), tup.g.glyph(), tup.c.glyph(),
                            tup.phi.glyph(), tup.h.glyph(), tup.s.glyph(), tup.omega.glyph());
                    } else {
                        sprintln!("Usage: rebis sidechain [analyze|list|frustration|<name>]");
                        sprintln!("  analyze          — batch analyze all 80 pairs (default)");
                        sprintln!("  list             — list all sidechains & environments");
                        sprintln!("  frustration      — show frustration matrix");
                        sprintln!("  <name>           — show tuple for sidechain or environment");
                    }
                }
            }
        }
        "ligand" => {
            match arg {
                "groups" | "fg" | "functional" => {
                    let names = crate::rebis::ligand::all_functional_group_names();
                    sprintln!("══ Functional Groups ══");
                    for name in &names {
                        sprintln!("  {}", name);
                    }
                }
                "design" | "" => {
                    let site_name = if rest.is_empty() { "active_site" } else { rest };
                    let residues: alloc::vec::Vec<&str> = if arg == "design" && !rest.is_empty() {
                        rest.split(',').collect()
                    } else {
                        vec!["Ser195", "His57", "Asp102"]
                    };
                    crate::rebis::ligand::print_ligand_suggestions(site_name, &residues);
                }
                _ => {
                    sprintln!("Usage: rebis ligand [groups|design <res1,res2,...>]");
                    sprintln!("  groups          — list functional groups");
                    sprintln!("  design [res]    — design ligands for active site");
                    sprintln!("  Example: rebis ligand design Ser195,His57,Asp102");
                }
            }
        }
        "decay" => {
            match arg {
                "list" | "" => {
                    let series = crate::rebis::decay_chain::known_series();
                    sprintln!("══ Decay Series ══");
                    for s in &series {
                        let dist = crate::rebis::decay_chain::series_distance(s);
                        sprintln!("  {}  (total IMASM distance: {:.1})", s, dist);
                    }
                }
                "compare" => {
                    crate::rebis::decay_chain::compare_series();
                }
                "all" => {
                    crate::rebis::decay_chain::print_all_series();
                }
                s => {
                    let upper = s.to_uppercase();
                    crate::rebis::decay_chain::print_chain(&upper);
                }
            }
        }
        "bio" => {
            match arg {
                "tissue" => {
                    let mut grid = TissueGrid::new(8, 8);
                    for _ in 0..5 { grid.step(); }
                    let (h, s, c, a) = grid.state_counts();
                    sprintln!("══ TissueGrid (8×8, gen={}) ══", grid.generation);
                    sprintln!("  Healthy: {}  Senescent: {}  Cancer: {}  Apoptotic: {}", h, s, c, a);
                }
                "telomere" => {
                    let mut tel = crate::rebis::biology::OuroboricTelomere::new(5000);
                    let divs: usize = rest.parse().unwrap_or(20);
                    tel.run(divs);
                    sprintln!("══ Ouroboric Telomere Simulation ══");
                    sprintln!("{}", tel.report());
                }
                "frob" | _ => {
                    let mut sim = FrobeniusBioSim::new(8, 8, 10);
                    sim.run(10);
                    sprintln!("{}", sim.report());
                    let mut tel = crate::rebis::biology::OuroboricTelomere::new(8000);
                    tel.run(15);
                    sprintln!("
{}", tel.report());
                    sprintln!("
  Usage: rebis bio [tissue|telomere <divs>|frob]");
                }
            }
        }
        "tx" => {
            match arg {
                "chemo" => {
                    let chemo = Chemotherapeutic::new("RB-001", "TOP2A", 5.0, 500.0);
                    sprintln!("══ Chemotherapeutic ══");
                    sprintln!("  Name: {}  Target: {}", chemo.name, chemo.target_protein);
                    sprintln!("  Kd: {:.1} nM  Selectivity: {:.0}x", chemo.binding_affinity_nm, chemo.selectivity_ratio);
                    sprintln!("  Delivery: {}  Gate1(⊙): {}", chemo.delivery_mechanism,
                        if chemo.gate1_open { "OPEN" } else { "CLOSED" });
                    sprintln!("  Frobenius: {}  MTD: {:.1} mg",
                        if chemo.verify() { "PASS" } else { "FAIL" }, chemo.max_tolerated_dose_mg);
                }
                "pill" => {
                    let pill = crate::rebis::therapeutics::OuroboricPill::new("OP-001", 24.0);
                    sprintln!("══ Ouroboric Pill ══");
                    sprintln!("  Name: {}  Half-life: {:.1}h", pill.name, pill.half_life_hours);
                    sprintln!("  Frobenius: {}  Gate1: {}",
                        if pill.frobenius_verified { "μ∘δ=id" } else { "FAIL" },
                        if pill.gate1_open { "self-sensing" } else { "passive" });
                }
                "antidote" => {
                    let antidote = crate::rebis::therapeutics::UniversalAntidote::new("UA-001");
                    sprintln!("══ Universal Antidote ══");
                    sprintln!("  Name: {}  Targets: {}", antidote.name, antidote.n_targets);
                    sprintln!("  Library diversity: {} clones", antidote.library_diversity);
                    sprintln!("  Frobenius: {}", if antidote.frobenius_verified { "PASS" } else { "OPEN" });
                }
                "neuro" => {
                    let nf = crate::rebis::therapeutics::NeurotrophicFactor::new("NF-001", 25.0, 48.0);
                    sprintln!("══ Neurotrophic Factor ══");
                    sprintln!("  Name: {}  Receptor: {}", nf.name, nf.target_receptor);
                    sprintln!("  EC50: {:.1} nM  Half-life: {:.1}h", nf.ec50_nm, nf.half_life_hours);
                    sprintln!("  Pathway: {}  Frobenius: {}",
                        nf.downstream_pathway, if nf.frobenius_verified { "PASS" } else { "FAIL" });
                }
                _ => {
                    let chemo = Chemotherapeutic::new("RB-001", "TOP2A", 5.0, 500.0);
                    sprintln!("══ Therapeutics ══");
                    sprintln!("  Chemotherapeutic: {} → {}  Kd={:.1}nM frob={}",
                        chemo.name, chemo.target_protein, chemo.binding_affinity_nm,
                        if chemo.verify() { "PASS" } else { "FAIL" });

                    let pill = crate::rebis::therapeutics::OuroboricPill::new("OP-001", 24.0);
                    sprintln!("  OuroboricPill: {} hl={:.1}h frob={}",
                        pill.name, pill.half_life_hours, if pill.frobenius_verified { "PASS" } else { "FAIL" });
                    sprintln!("
  Usage: rebis tx [chemo|pill|antidote|neuro]");
                }
            }
        }
        "clink" => {
            match arg {
                "chain" => sprintln!("{}", clink::clink_verify_chain()),
                "ladder" => sprintln!("{}", clink::clink_distance_ladder()),
                "promote" => sprintln!("{}", clink::clink_promotion_ladder()),
                "summary" | _ => sprintln!("{}", clink::clink_summary()),
            }
        }
        "imas" => {
            match arg {
                "bridge" => sprintln!("{}", crate::rebis::imas::bridge_all_report()),
                "verify" => {
                    if let Some(seq) = crate::rebis::imas::canonical_sequence(6) {
                        sprintln!("{}", crate::rebis::imas::verify_bootstrap(seq));
                    }
                }
                "summary" | _ => sprintln!("{}", crate::rebis::imas::imasm_summary()),
            }
        }
        _ => {
            sprintln!("Rebis: Red-Hot Rebis kernel module (20 subcommands)");
            sprintln!("  rebis codon <XXX|AA>      — codon→AA or AA→codons (bidirectional)");
            sprintln!("  rebis translate <DNA>     — gene→protein pipeline (DNA→mRNA→AA)
  rebis reverse <Prot>     — protein→mRNA→DNA (reverse pipeline)");
            sprintln!("  rebis frob               — Frobenius filtration");
            sprintln!("  rebis genetics           — 7-stage verification");
            sprintln!("  rebis hadron             — Belnap hadron analysis");
            sprintln!("  rebis serpent [name]     — serpent rod motifs");
            sprintln!("  rebis fold <DNA|RNA>     — DNA/RNA -> folded protein (SerpentRod)");
            sprintln!("  rebis pipeline [src]     — IG promotion pipeline");
            sprintln!("  rebis strata             — codon stratum counts");
            sprintln!("  rebis asm [prog]         — genetic ParaASM programs");
            sprintln!("  rebis tuples <DNA>       — 7-stage generative tuple pipeline");
            sprintln!("  rebis clu walk|verify    — CLU power-law clustering");
            sprintln!("  rebis exotic             — exotic hadron Frobenius verification");
            sprintln!("  rebis pdb validate|..    — PDB structure validation");
            sprintln!("  rebis antibody epi|des.. — antibody CDR design");
            sprintln!("  rebis material forge|..  — IG material forge & metamaterials");
            sprintln!("  rebis bio                — biological sim (tissue, telomere)");
            sprintln!("  rebis sidechain [analyze] — AA sidechain × environment algebra");
            sprintln!("  rebis ligand [groups]     — Ligand design from catalytic sites");
            sprintln!("  rebis decay [series]      — Nuclear decay as IMASM winding (U238, U235, Th232)");
            sprintln!("  rebis tx                 — therapeutics (chemo, pill, antidote)
  rebis clink [chain|..]    — CLINK 9-layer chain (L0–L8)
  rebis imas [bridge|..]    — IMASM arranger bridge");
        }
    }
}
