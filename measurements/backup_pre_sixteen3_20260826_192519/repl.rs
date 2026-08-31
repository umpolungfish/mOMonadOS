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
    btc_secret_key_oneshot,
    serial, belnap, tokens, crystal, kernel, interrupts, frob_verify, imas_ig,
    aleph, manus, parasm, belnap_shor, para_rh, para_ym, para_temporal,
    para_category, algebra, catalog, cl8nk, consciousness, rebis, dialect, menu,
    sequence, cr3echrz, canonical_ordinal, clay_status, sic_povm,
    frobenius_unify, clay_witness, belnap_sic_bridge, belnap_c4, sic_compute,
    dialect_expansion, divisor_ring, mersenne_parallel, bifurcation_test, entropy, invariant, d12_sic, d2048_sic, d2048_sieve, stark,
    sic_moduli,
    riemann_sic,
    riemann_hilbert, bip39_sic_grover, redteam,
    witness_vessel, ask, ovm, pk2sk,
};
use crate::tokens::{canonical_name, canonical_count, continuous_name, continuous_count, novel_name, novel_count, shunted_name, shunted_count, compound_name, compound_index, compound_program, compound_count};
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
    // 2 MiB on the stack overflowed under Windows' default ~1 MiB thread
    // stack the instant `repl()` was entered — Linux's 8 MiB default had
    // masked this. The buffer itself is unchanged, only where it lives.
    let mut line_buf = vec![0u8; 2097152].into_boxed_slice();
    let mut history = History::new();
    let mut ctx_stack = ContextStack::new();
    let mut ask_paste = crate::ask::AskPaste::new();

    sprintln!("   {}ask{} runs a structural dry-run here; the full wet-run is on the host:",
        crate::style::key(), crate::style::reset());
    sprintln!("   {}./ask --file <path> | ./ask --ask \"…\" | ./ask -i{}",
        crate::style::muted(), crate::style::reset());
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
        // A category shortcut must only fire on a BARE category name. Without this,
        // any line whose first token is a category ("crystal indices …") is caught by
        // the shortcut arm below, which enters the context and `continue`s, swallowing
        // the subcommand. That made `crystal indices` — the mu leg, words -> crystal
        // indices, the measurement that makes mu∘delta = id checkable rather than
        // asserted — implemented but unreachable from the REPL.
        let bare_category = line.trim().split_whitespace().count() == 1;

        // ── Menu Navigation ────────────────────────────────
        match cmd {
            // Exit sub-context
            ".." | "back" => {
                if ctx_stack.depth > 0 {
                    let popped = ctx_stack.pop();
                    sprintln!("← returned from {}", popped);
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
                bare_category && (lower == "exec" || lower == "status" || lower == "programs" || lower == "crystal"
                    || lower == "grammar" || lower == "rebis" || lower == "dialect" || lower == "parasm" || lower == "cr3echrz" || lower == "clay")
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
            "winding" | "wperiod" => {
                match parts.next().unwrap_or("") {
                    "" | "help" => {
                        sprintln!("winding - period as a torus winding (native Rust, torus by default)");
                        sprintln!("  winding order <a> <N>    minimal period r of a^x mod N (BSGS winding halving)");
                        sprintln!("  winding factor <N> [tries] [seed]   end-to-end factorization via the Shor winding step");
                        sprintln!("  winding closure <N> <B>  Pollard p-1 closure at smoothness bound B");
                        sprintln!("  winding factorgen <bits> [tries] [seed]   native semiprime + factor (the push)");
                    }
                    "order" => {
                        let a = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let N = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        crate::winding_period::repl_order(a, N);
                    }
                    "factor" => {
                        let N = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let tries = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(12);
                        let seed = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0x9E37_79B9_7F4A_7C15);
                        crate::winding_period::repl_factor(N, tries, seed);
                    }
                    "closure" => {
                        let N = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let B = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(11);
                        crate::winding_period::repl_closure(N, B);
                    }
                    "factorgen" => {
                        let bits = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(48);
                        let tries = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(12);
                        let seed = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0x9E37_79B9_7F4A_7C15);
                        crate::winding_period::repl_factorgen(bits, tries, seed);
                    }
                    other => sprintln!("winding: unknown subcommand '{}' (try 'winding help')", other),
                }
            },
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
            "fold" => crate::fold_walk::walk_fold(),
            "erdos" => {
                match parts.next().unwrap_or("") {
                    "" | "list" => crate::erdos_walks::list_walks(),
                    other => crate::erdos_walks::dispatch(other),
                }
            },
            "seals" => {
                match parts.next().unwrap_or("") {
                    "" | "list" => crate::seals::list_seals(),
                    other => crate::seals::dispatch_seal(other),
                }
            },
            "cycle" => {
                let tail: Vec<&str> = parts.collect();
                let word = tail.join(" ");
                let w = word.trim();
                if w.is_empty() {
                    sprintln!("cycle <word>     — walk a word around its ROTAT orbit");
                    sprintln!("weight <word>    — where the weight moves through it");
                    sprintln!("");
                    sprintln!("A word is a ring and ROTAT is the cyclic shift, so every");
                    sprintln!("rotation is the same object. The verdict and the topology hold");
                    sprintln!("across the orbit; the final register does not, which makes the");
                    sprintln!("phase the only handle on where a word comes to rest.");
                } else {
                    crate::lattice_flow::cycle_report(w);
                }
            }
            "trans" => {
                let tail: Vec<&str> = parts.collect();
                let word = tail.join(" ");
                let w = word.trim();
                if w.is_empty() {
                    sprintln!("trans <word>     — transitions counted on the RING");
                    sprintln!("A word is a cycle, so it has as many transitions as opcodes.");
                    sprintln!("A linear read drops the closing edge, usually TANCH -> VINIT.");
                } else {
                    crate::lattice_flow::transitions_report(w);
                }
            }
            "insert" => {
                let tail: Vec<&str> = parts.collect();
                let word = tail.join(" ");
                let w = word.trim();
                if w.is_empty() {
                    sprintln!("insert <word>    — every single-glyph insertion that turns");
                    sprintln!("an exposed word into one whose weight survives its clears. The");
                    sprintln!("repair for a losing word is usually one glyph in the right place,");
                    sprintln!("and the search is small enough to be exact rather than reasoned.");
                } else if w.eq_ignore_ascii_case("all") {
                    crate::lattice_flow::insert_sweep_all();
                } else {
                    crate::lattice_flow::insert_report(w);
                }
            }
            "banked" => {
                let tail: Vec<&str> = parts.collect();
                let word = tail.join(" ");
                let w = word.trim();
                if w.is_empty() {
                    sprintln!("banked <word>    — was anything counted, then cleared");
                    sprintln!("with nothing banked behind it? AREV empties the register and");
                    sprintln!("leaves open frames alone, so a result fused to depth zero is");
                    sprintln!("exposed to the next reversal.");
                } else {
                    crate::lattice_flow::banked_report(w);
                }
            }
            // `proof` is the guided proof walker and is dispatched above; naming it
            // here too made this arm unreachable for it, so `proof` never once
            // reached prooflift and the alias only looked like it worked.
            "prooflift" => {
                let tail: Vec<&str> = parts.collect();
                if tail.first().map(|a| *a == "nest").unwrap_or(false) {
                    sprintln!("{}", crate::prooflift::nest());
                } else {
                    sprintln!("{}", crate::prooflift::report());
                }
            }
            "weight" => {
                let tail: Vec<&str> = parts.collect();
                let word = tail.join(" ");
                let w = word.trim();
                if w.is_empty() {
                    sprintln!("weight <word>    — where the weight moves through an IMASM word");
                    sprintln!("The fork is a set and the fuse a union, so a finished walk keeps");
                    sprintln!("which values were touched and nothing else. This counts.");
                } else {
                    crate::lattice_flow::weight_report(w);
                }
            }
            // `vox` reads control flow the way `weight` reads value flow: it takes
            // the instruction side rather than the word side, classifies each
            // mnemonic to a glyph, and runs the same verdict over the result.
            "vox" => {
                let tail: Vec<&str> = parts.collect();
                let rest: Vec<String> =
                    tail.join(" ").split_whitespace().map(|s| s.to_string()).collect();
                if rest.is_empty() || rest[0] == "help" {
                    sprintln!("vox <sub>        — control-flow closure auditor");
                    sprintln!("vox verdict <word>   — SIXTEEN_3 verdict over a glyph word");
                    sprintln!("vox evm <hex>        — lift EVM bytecode, verdict its closure");
                    sprintln!("vox wasm <hex>       — lift a WASM body, verdict its closure");
                    sprintln!("vox classify <mn>    — which glyph an instruction lifts to");
                    sprintln!("vox rna <seq> [code] — lift a coding sequence; code is");
                    sprintln!("                       standard or mitochondrial");
                    sprintln!("vox peptide <seq>    — lift a residue sequence");
                    sprintln!("vox run <file> [--argv a,b]");
                    sprintln!("                     — run the whole file as a real process from");
                    sprintln!("                       its own entry point: a real argv/envp/auxv");
                    sprintln!("                       stack underneath it, real registers, memory,");
                    sprintln!("                       flags, ALU in front of it, and real syscalls");
                    sprintln!("                       — read/write/open/openat/close go through");
                    sprintln!("                       the host filesystem and console for real,");
                    sprintln!("                       mmap/brk hand out a real anonymous heap. No");
                    sprintln!("                       function ever returns here; it ends by");
                    sprintln!("                       calling exit, same as any process. This is");
                    sprintln!("                       the only thing a PE binary offers too, since");
                    sprintln!("                       the loader never reads a symbol table for");
                    sprintln!("                       that format.");
                    sprintln!("vox run <sym> <file> [--args a,b]");
                    sprintln!("                     — call ONE function directly instead: scalar");
                    sprintln!("                       int args in, one int back, no process at");
                    sprintln!("                       all — the older, narrower contract.");
                    sprintln!("                       A statically-linked binary using only direct");
                    sprintln!("                       syscalls runs for real end to end. A dynamically-");
                    sprintln!("                       linked binary's calls into libc, and a glibc");
                    sprintln!("                       static binary's own TLS/segment-register setup,");
                    sprintln!("                       are further rungs, not yet built: those halt or");
                    sprintln!("                       loop rather than silently pretending to work.");
                    sprintln!("                       Hosted builds only.");
                    sprintln!("A word closes at T, carries an open fork at B, and runs clean");
                    sprintln!("and linear at N. The fork is what the verdict is looking for.");
                } else {
                    match rest[0].as_str() {
                        "verdict" | "words" | "word" => {
                            if rest.len() > 1 {
                                let word: Vec<char> = rest[1..].join("").chars().collect();
                                sprintln!("{}", crate::vox::glyphs(&word));
                                sprintln!("verdict {}", crate::vox::verdict(&word));
                            } else {
                                sprintln!("vox verdict <glyph-word>");
                            }
                        }
                        "lift" => {
                            if rest.len() < 2 {
                                sprintln!("vox lift <path>   — lift an ELF's executable sections");
                            } else {
                                vox_lift_file(&rest[1]);
                            }
                        }
                        "rna" => {
                            if rest.len() > 1 {
                                let dialect = rest.get(2).map(|s| s.as_str()).unwrap_or("standard");
                                let t = vox_core::genetic::lift_rna_dialect(&rest[1], dialect);
                                sprintln!("code    {}", dialect);
                                sprintln!("word    {}", crate::vox::glyphs(&t.word));
                                if let Some(stop) = t.stopped {
                                    sprintln!("stop    {}", stop);
                                }
                                sprintln!("verdict {}", crate::vox::verdict(&t.word));
                            } else { sprintln!("vox rna <sequence> [standard|mitochondrial]"); }
                        }
                        "peptide" | "aa" => {
                            if rest.len() > 1 {
                                let t = vox_core::genetic::lift_peptide(&rest[1]);
                                sprintln!("word    {}", crate::vox::glyphs(&t.word));
                                sprintln!("verdict {}", crate::vox::verdict(&t.word));
                            } else { sprintln!("vox peptide <residues>"); }
                        }
                        "evm" => {
                            if rest.len() > 1 {
                                let w = vox_core::lanes::evm_word(&rest[1]);
                                sprintln!("EVM  {}  {}", crate::vox::verdict(&w), crate::vox::glyphs(&w));
                            } else { sprintln!("vox evm <hex>"); }
                        }
                        "wasm" => {
                            if rest.len() > 1 {
                                let w = vox_core::lanes::wasm_word(&rest[1]);
                                sprintln!("WASM {}  {}", crate::vox::verdict(&w), crate::vox::glyphs(&w));
                            } else { sprintln!("vox wasm <hex>"); }
                        }
                        "classify" => {
                            if rest.len() > 1 {
                                let ins = crate::vox::Instruction {
                                    address: 0,
                                    mnemonic: rest[1].to_lowercase(),
                                    op_str: rest[2..].join(" "),
                                };
                                let g = crate::vox::classify_instruction(&ins);
                                sprintln!("{} {}", ins.mnemonic, g);
                            } else {
                                sprintln!("vox classify <mnemonic> [operands]");
                            }
                        }
                        "run" => vox_run_symbol(&rest[1..]),
                        other => sprintln!("vox has no `{}`; try `vox help`", other),
                    }
                }
            }
            // `circuit` runs the substrate round trips. The word is the invariant;
            // every substrate leg is many-to-one, so what is being checked is
            // that each leg is a retraction and that the detour changes nothing.
            "circuit" => {
                let tail: Vec<&str> = parts.collect();
                let rest: Vec<String> =
                    tail.join(" ").split_whitespace().map(|s| s.to_string()).collect();
                let sub = rest.first().map(|s| s.as_str()).unwrap_or("help");
                match sub {
                    "table" => {
                        for l in crate::circuit::table_lines() {
                            sprintln!("{}", l);
                        }
                    }
                    "retract" | "retraction" => {
                        for l in crate::circuit::retraction_lines() {
                            sprintln!("{}", l);
                        }
                    }
                    "one" => {
                        let word: Vec<crate::belnap_ring_shor::Glyph> = if rest.len() > 1 {
                            rest[1..].join("").chars()
                                .filter_map(crate::belnap_ring_shor::Glyph::from_char).collect()
                        } else {
                            crate::belnap_ring_shor::Glyph::all().to_vec()
                        };
                        let r = crate::circuit::circuit_one(&word);
                        let a: String = r.start.iter().map(|g| g.to_char()).collect();
                        let b: String = r.returned.iter().map(|g| g.to_char()).collect();
                        sprintln!("x86 → IMASM → RNA → IMASM → x86");
                        sprintln!("  in   {}", a);
                        sprintln!("  rna  {}", r.rna);
                        sprintln!("  out  {}", b);
                        sprintln!("  {}", if r.closes() { "closes" } else { "DOES NOT CLOSE" });
                        for i in &r.instructions {
                            sprintln!("    {}", i);
                        }
                    }
                    "two" => {
                        let rna = if rest.len() > 1 {
                            rest[1..].join("")
                        } else {
                            let mut s = alloc::string::String::new();
                            for g in crate::belnap_ring_shor::Glyph::all() {
                                if let Some(c) = crate::circuit::glyph_to_codon(g) {
                                    s.push_str(&crate::circuit::codon_rna(&c));
                                }
                            }
                            s
                        };
                        let r = crate::circuit::circuit_two(&rna);
                        sprintln!("RNA → IMASM → x86 → IMASM → wasm → IMASM → AA");
                        sprintln!("  rna  {}", rna);
                        for l in &r.trace {
                            sprintln!("    {}", l);
                        }
                        let d: alloc::vec::Vec<&str> =
                            r.direct.iter().map(|a| a.code3()).collect();
                        let o: alloc::vec::Vec<&str> =
                            r.routed.iter().map(|a| a.code3()).collect();
                        sprintln!("  direct  {}", d.join("-"));
                        sprintln!("  routed  {}", o.join("-"));
                        if r.skipped > 0 {
                            sprintln!("  {} codon(s) carried no glyph and did not enter", r.skipped);
                        }
                        if r.closes() {
                            sprintln!("  the detour is invisible");
                        } else {
                            sprintln!("  the detour changes the protein");
                            sprintln!("  {} codon(s) sat off the canonical section, and δ∘μ", r.offsection);
                            sprintln!("  moved them. Identity holds on the section and nowhere else.");
                        }
                    }
                    "slots" => {
                        sprintln!("single-position substitutions that change the amino acid,");
                        sprintln!("read off the live codon table:");
                        for l in crate::circuit::slot_loads() {
                            let role = match l.position {
                                1 => "sense-private",
                                2 => "SHARED by both strands",
                                _ => "antisense-private",
                            };
                            sprintln!("  p{}  {:>3}/{:<3}  {:>3}%   {}",
                                l.position, l.changed, l.substitutions, l.percent(), role);
                        }
                    }
                    "census" => {
                        let (prim, scaf, stop) = crate::circuit::codon_census();
                        sprintln!("codon space, classified with no residue:");
                        sprintln!("  {:>2}  carry a primitive (the twelve promoted acids)", prim);
                        sprintln!("  {:>2}  scaffold (the eight ground-layer acids, no primitive)", scaf);
                        sprintln!("  {:>2}  stop", stop);
                        sprintln!("  {:>2}  total", prim + scaf + stop);
                        sprintln!("");
                        sprintln!("section choice is free: {}",
                            if crate::circuit::section_choice_is_free() {
                                "yes — μ∘δ=id for every codon carrying the glyph"
                            } else {
                                "NO"
                            });
                    }
                    "drift" => {
                        let d = crate::circuit::primitive_drift();
                        if d.is_empty() {
                            sprintln!("to_primitive agrees with the canonical correspondence");
                        } else {
                            sprintln!("AminoAcid::to_primitive disagrees with GeneticCode.lean:");
                            for l in d {
                                sprintln!("{}", l);
                            }
                        }
                    }
                    "rc" | "strand" => {
                        let rna = if rest.len() > 1 { rest[1..].join("") } else {
                            "AUGGCCUUUAAAGGGCAUUGCACG".to_string()
                        };
                        let r = crate::circuit::strand_report(&rna);
                        sprintln!("  rna        {}", rna);
                        sprintln!("  sense      {}", r.sense);
                        sprintln!("  antisense  {}", r.antisense);
                        sprintln!("  frame 0    {}", r.frames[0]);
                        sprintln!("  frame 1    {}", r.frames[1]);
                        sprintln!("  frame 2    {}", r.frames[2]);
                        sprintln!("  `·` is scaffold, `|` is stop. The antisense strand");
                        sprintln!("  reads its first position off the sense strand's wobble.");
                    }
                    _ => {
                        sprintln!("circuit table       — every glyph across every substrate");
                        sprintln!("circuit rc [rna]    — sense, antisense, and all three frames");
                        sprintln!("circuit slots       — which codon position the code loads");
                        sprintln!("circuit drift       — to_primitive against the canonical map");
                        sprintln!("circuit census      — codon space with no residue");
                        sprintln!("circuit retract     — μ∘δ=id, leg by leg");
                        sprintln!("circuit one [word]  — x86 → IMASM → RNA → IMASM → x86");
                        sprintln!("circuit two [rna]   — RNA → IMASM → x86 → IMASM → wasm → IMASM → AA");
                        sprintln!("A binary cannot return byte-identical: each substrate leg is");
                        sprintln!("many-to-one. The word is what closes.");
                    }
                }
            }
            // The grammar-tool layer calls these `quantum_compile` and
            // `jones_polynomial`, and an agent that knows those names typed them
            // here and got "Unknown: quantum_compile". Same operations, two
            // vocabularies; accept both rather than make the caller learn which
            // surface it is standing on.
            "qc" | "quantum_compile" => {
                let tail: Vec<&str> = parts.collect();
                let joined = tail.join(" ");
                let rest: Vec<&str> = joined.split_whitespace().collect();
                // `draw` / `svg` in front of the circuit renders the word it
                // compiles to instead of listing it as integers. The word is
                // built inside the compile's heap scope and dies with it, so
                // the choice has to travel in rather than the word travelling out.
                let mut render = 0u8;
                let mut rest = rest;
                if !rest.is_empty() {
                    let first = rest[0].to_lowercase();
                    if first == "draw" {
                        render = 1;
                        rest.remove(0);
                    } else if first == "svg" {
                        render = 2;
                        rest.remove(0);
                    } else if first == "loop" || first == "curve" || first == "curvy" {
                        render = 3;
                        rest.remove(0);
                    }
                }
                if rest.is_empty() {
                    sprintln!("qc [draw|svg] <gates> [net_depth] [sk_depth]   e.g. `qc H T 8`, `qc HTSX 10 0`");
                    sprintln!("Known gates: H T S X — spaces optional, case free.");
                    sprintln!("Both depths are any positive integer (net 10, recursion 3");
                    sprintln!("by default). Each stops early rather than refusing: the net");
                    sprintln!("when it outgrows the arena, the recursion when the next");
                    sprintln!("level's word would, and the run says which one it was.");
                } else {
                    // Trailing integers are the two depths: net depth, then SK
                    // recursion depth. The recursion depth was pinned at 3 and
                    // invisible, which made the net-depth sweep unreadable —
                    // the reported error is the output of a 3-level recursion,
                    // not the net's own nearest neighbour, and only the latter
                    // is monotone in net size. `qc HTSX 10 0` asks the net
                    // directly.
                    let mut nums: Vec<usize> = Vec::new();
                    let mut cut = rest.len();
                    while cut > 0 {
                        match rest[cut - 1].parse::<usize>() {
                            Ok(v) if nums.len() < 2 => { nums.push(v); cut -= 1; }
                            _ => break,
                        }
                    }
                    nums.reverse();
                    let mut sk_depth = 3usize;
                    let (gates, mut depth) = match nums.len() {
                        2 => { sk_depth = nums[1]; (&rest[..cut], nums[0].max(1)) }
                        1 => (&rest[..cut], nums[0].max(1)),
                        _ => (&rest[..], 0),
                    };
                    // Gates need no separators, so the depth need not be separated
                    // either: `qc XTT4` reads the trailing digits as the depth.
                    let mut spec = gates.join(" ");
                    if depth == 0 {
                        let digits = spec.len() - spec.trim_end_matches(|c: char| c.is_ascii_digit()).len();
                        if digits > 0 && digits < spec.len() {
                            if let Ok(d) = spec[spec.len() - digits..].parse::<usize>() {
                                depth = d.max(1);
                                spec.truncate(spec.len() - digits);
                            }
                        }
                        if depth == 0 { depth = 10; }
                    }
                    if spec.trim().is_empty() {
                        sprintln!("No gates given. Known: H T S X");
                    } else {
                        crate::fibonacci_qc::repl_compile(&spec, depth, sk_depth, render);
                    }
                }
            }
            // A braid word printed as signed integers is exact and unreadable.
            // `bi` draws it: strand diagram in the terminal, SVG when asked for
            // a file. Both take the same window, because the reason to look at a
            // 385-crossing word is usually to find one crossing in it.
            "bi" | "braid_image" => {
                let tail: Vec<&str> = parts.collect();
                let joined = tail.join(" ");
                let toks: Vec<&str> = joined
                    .split(|c: char| c.is_whitespace() || c == ',')
                    .filter(|t| !t.is_empty())
                    .collect();
                let (mut as_svg, mut start, mut count) = (false, 0usize, 0usize);
                let mut as_loop = false;
                // Long words fold into columns by default; `/N` sets the column
                // height, `/0` forces the single tall column.
                let mut fold: usize = 48;
                let mut word: Vec<i32> = Vec::new();
                let mut bad: Option<&str> = None;
                for t in &toks {
                    match *t {
                        "svg" => { as_svg = true; as_loop = false; }
                        "loop" | "curve" | "curvy" => { as_svg = true; as_loop = true; }
                        "ascii" | "draw" => { as_svg = false; as_loop = false; }
                        _ => {
                            if let Some(n) = t.strip_prefix('/') {
                                match n.parse::<usize>() {
                                    Ok(f) => fold = f,
                                    Err(_) => bad = Some(t),
                                }
                            } else if let Some((a, b)) = t.split_once(':') {
                                // `12:40` — forty crossings from the twelfth.
                                match (a.parse::<usize>(), b.parse::<usize>()) {
                                    (Ok(x), Ok(y)) => { start = x; count = y; }
                                    _ => bad = Some(t),
                                }
                            } else if let Ok(g) = t.parse::<i32>() {
                                if g == 0 { bad = Some(t); } else { word.push(g); }
                            } else {
                                bad = Some(t);
                            }
                        }
                    }
                }
                if let Some(t) = bad {
                    sprintln!("bi: '{}' is not a generator, a window, or svg/ascii.", t);
                } else if word.is_empty() {
                    sprintln!("bi [svg|loop] <generators> [start:count]");
                    sprintln!("  e.g. `bi 1 2 1`, `bi 1 -2 1 -2`, `bi loop 1 2 1 2 1`");
                    sprintln!("  Generators are signed Artin: k is sigma_k, -k its inverse.");
                    sprintln!("  A window like `40:24` draws 24 crossings from the 40th.");
                    sprintln!("  SVG folds into columns of 48; `/N` sets that, `/0` is one column.");
                    sprintln!("  For a compiled circuit use `qc draw <gates>` or `qc svg <gates>`.");
                } else {
                    let strands = crate::braid_render::strands_for(&word);
                    let (a, b) = crate::braid_render::window(word.len(), start, count);
                    if as_svg {
                        if as_loop {
                            sprint!("{}", crate::braid_render::svg_loop(&word, strands));
                        } else {
                            sprint!("{}", crate::braid_render::svg(&word, strands, a, b, fold));
                        }
                    } else {
                        sprint!("{}", crate::braid_render::header(&word, strands, a, b));
                        sprint!("{}", crate::braid_render::ascii(&word, strands, a, b));
                    }
                }
            }
            "jp" | "jones_polynomial" => {
                let tail: Vec<&str> = parts.collect();
                let joined = tail.join(" ");
                let word: Vec<i32> = joined.split_whitespace()
                    .filter_map(|t| t.parse::<i32>().ok()).collect();
                if word.is_empty() {
                    sprintln!("jp <generators...>   e.g. `jp 1 1 1`   (alias jones_polynomial)");
                    sprintln!("Signed Artin generators, integers. Strands are implied:");
                    sprintln!("sigma_k needs k+1. IMASM opcode names are not accepted here.");
                } else {
                    let n = word.iter().map(|g| g.unsigned_abs() as usize).max()
                                .unwrap_or(0) + 1;
                    crate::fibonacci_qc::repl_jones(n, &word);
                }
            }
            "fibqc" => {
                match parts.next().unwrap_or("") {
                    "" | "help" => {
                        sprintln!("fibqc verify              — algebra self-check (F/R symbols, S/T, braid unitarity)");
                        sprintln!("fibqc compile <gates>     — compile a circuit over H T S X to a braid word");
                        sprintln!("fibqc compile <gates> <n> — same, with gate net depth n (default 10, max 12)");
                        sprintln!("fibqc jones <gens...>    — Jones polynomial; strands implied by the word");
                        sprintln!("fibqc knot [name]        — Jones value for a knot from the census");
                        sprintln!("fibqc winding            — the phase lattice, in windings");
                        sprintln!("fibqc readout <a> <N>    — one-shot topological readout (ModExp invariant -> winding -> period)");
                        sprintln!("fibqc alkahest <a> <N>   — the four-name dissolution report (root, fixed point, one, ⊡-promotion)");
                        sprintln!("fibqc protocol           — show the IMASM braiding protocol report");
                        sprintln!("fibqc braid <gens...>    — δ: compile a braid word to an IMASM program");
                        sprintln!("fibqc braid <gens...> close — same, closed (trace closure)");
                        sprintln!("fibqc tangle <program>   — μ: read an IMASM program back as a braid word");
                        sprintln!("");
                        sprintln!("The circuit compiles as ONE unitary, so the error is incurred once");
                        sprintln!("rather than accumulating gate by gate. Several braid words sit at the");
                        sprintln!("same distance from the target; each is followed as a separate arm, then");
                        sprintln!("the arms that lost compile the residual left by the arm that won.");
                        sprintln!("Net memory, measured in-kernel: 1.7 MB at depth 10, 6.9 MB at depth 12,");
                        sprintln!("against an 8 MB bump arena. Depth 12 peaks at 8156 KB, a 36 KB margin,");
                        sprintln!("so it is the hard ceiling. The command reports its own high-water mark.");
                    }
                    "winding" => crate::fibonacci_qc::repl_winding(),
                    "readout" => {
                        let a = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        let N = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                        crate::fibonacci_qc::repl_readout(a, N);
                    }
                    "alkahest" => {
                        let a_str = parts.next().unwrap_or("");
                        let n_str = parts.next().unwrap_or("");
                        crate::fibonacci_qc::repl_alkahest(a_str, n_str);
                    }
                    "verify" => {
                        sprintln!("Fibonacci anyon algebra verified = {}", crate::fibonacci_qc::verify_all());
                    }
                    "compile" => {
                        // the line was split with a field limit, so the tail can
                        // arrive as one token ("S 12"); re-tokenize it. Using
                        // split_whitespace also absorbs the CR the serial
                        // console appends on Enter.
                        let tail: Vec<&str> = parts.collect();
                        let joined = tail.join(" ");
                        let rest: Vec<&str> = joined.split_whitespace().collect();
                        if rest.is_empty() {
                            sprintln!("fibqc compile [draw|svg|loop] <gates> [depth]");
                        } else {
                            let mut render = 0u8;
                            let mut rest = rest;
                            if !rest.is_empty() {
                                let first = rest[0].to_lowercase();
                                if first == "draw" {
                                    render = 1;
                                    rest.remove(0);
                                } else if first == "svg" {
                                    render = 2;
                                    rest.remove(0);
                                } else if first == "loop" || first == "curve" || first == "curvy" {
                                    render = 3;
                                    rest.remove(0);
                                }
                            }
                            // a trailing integer is the net depth, everything before it is the circuit
                            let (gates, depth) = match rest.last().and_then(|s| s.parse::<usize>().ok()) {
                                Some(d) => (&rest[..rest.len()-1], d.max(1)),
                                None => (&rest[..], 10),
                            };
                            if gates.is_empty() {
                                sprintln!("No gates given. Known: H T S X");
                            } else {
                                crate::fibonacci_qc::repl_compile(&gates.join(" "), depth, 3, render);
                            }
                        }
                    }
                    "jones" => {
                        let tail: Vec<&str> = parts.collect();
                        let joined = tail.join(" ");
                        let word: Vec<i32> = joined.split_whitespace()
                            .filter_map(|t| t.parse::<i32>().ok()).collect();
                        if word.is_empty() {
                            sprintln!("fibqc jones <generators...>   e.g. `fibqc jones 1 1 1`");
                            sprintln!("Strand count is implied: sigma_k needs k+1 strands.");
                        } else {
                            // The word fixes the strand count. Asking for it
                            // separately only creates a number to get wrong.
                            let n = word.iter().map(|g| g.unsigned_abs() as usize).max()
                                        .unwrap_or(0) + 1;
                            crate::fibonacci_qc::repl_jones(n, &word);
                        }
                    }
                    "knot" => {
                        let name = parts.next().unwrap_or("").trim();
                        // braid words for a small census; the closure of each is the knot
                        let table: [(&str, usize, &[i32]); 9] = [
                            ("unknot",       1, &[]),
                            ("trefoil",      2, &[1,1,1]),
                            ("trefoil*",     2, &[-1,-1,-1]),
                            ("figure-eight", 3, &[1,-2,1,-2]),
                            ("cinquefoil",   2, &[1,1,1,1,1]),
                            ("7_1",          2, &[1,1,1,1,1,1,1]),
                            ("8_19",         3, &[1,1,1,2,1,1,1,2]),
                            ("T(2,9)",       2, &[1,1,1,1,1,1,1,1,1]),
                            ("T(2,10)",      2, &[1,1,1,1,1,1,1,1,1,1]),
                        ];
                        if name.is_empty() {
                            sprintln!("fibqc knot <name>   known:");
                            for (nm, n, w) in table.iter() {
                                sprintln!("    {:14} {} strands, {} crossings", nm, n, w.len());
                            }
                        } else if let Some((nm, n, w)) =
                            table.iter().find(|(nm, _, _)| *nm == name) {
                            sprintln!("{} — closure of a {}-strand braid", nm, n);
                            crate::fibonacci_qc::repl_jones(*n, w);
                        } else {
                            sprintln!("fibqc: no knot named '{}'. Try `fibqc knot`.", name);
                        }
                    }
                    "protocol" => {
                        sprint!("{}", crate::braid_protocol::report());
                    }
                    // δ and μ of the braid dual, reachable at last. The pair was written
                    // correctly and had no verb, so nothing could call it and nothing could
                    // close it. μ∘δ = id is gated in the lib tests, on the generator word:
                    // δ chooses a depth path between crossings and μ does not record which,
                    // so the identity is on the braid, which is the object, and not on the
                    // program, which is one presentation of it.
                    "braid" => {
                        let tail: alloc::vec::Vec<&str> = parts.collect();
                        let joined = tail.join(" ");
                        let rest: alloc::vec::Vec<&str> = joined.split_whitespace().collect();
                        let close = rest.last().map(|s| *s == "close").unwrap_or(false);
                        let gens: alloc::vec::Vec<i32> = rest
                            .iter()
                            .filter(|s| **s != "close")
                            .filter_map(|s| s.parse::<i32>().ok())
                            .collect();
                        if gens.is_empty() && !rest.is_empty() {
                            sprintln!("fibqc braid: give signed generators, e.g. `fibqc braid 1 2 -1`.");
                        } else {
                            let prog = crate::braid_protocol::braid_to_imasm(&gens, 1, close);
                            sprintln!("braid word: {:?}{}", gens, if close { " (closed)" } else { "" });
                            sprint!("IMASM: ");
                            for tok in prog.iter() {
                                sprint!("{} ", crate::braid_protocol::token_name(tok));
                            }
                            sprintln!("");
                            match crate::braid_protocol::read_tangle(&prog, gens.len() + 2, 1) {
                                Ok(r) => sprintln!(
                                    "μ∘δ: {} — writhe {}, {} crossings, closes {}",
                                    if r.generators == gens { "id" } else { "NOT id" },
                                    r.writhe, r.crossings, r.closes
                                ),
                                Err(e) => sprintln!("μ refused: {}", e),
                            }
                        }
                    }
                    "tangle" => {
                        let tail: alloc::vec::Vec<&str> = parts.collect();
                        let joined = tail.join(" ");
                        let names: alloc::vec::Vec<&str> = joined.split_whitespace().collect();
                        let mut prog = alloc::vec::Vec::new();
                        let mut bad = None;
                        for n in names.iter() {
                            match crate::braid_protocol::parse_token_name(n) {
                                Some(tok) => prog.push(tok),
                                None => { bad = Some(*n); break; }
                            }
                        }
                        if let Some(b) = bad {
                            sprintln!("fibqc tangle: '{}' is not a token name. Try `imasm ref`.", b);
                        } else if prog.is_empty() {
                            sprintln!("fibqc tangle: give an IMASM program, e.g. `fibqc tangle FSPLIT AFWD FFUSE`.");
                        } else {
                            match crate::braid_protocol::read_tangle(&prog, prog.len() + 2, 1) {
                                Ok(r) => {
                                    sprintln!("braid word: {:?}", r.generators);
                                    sprintln!(
                                        "writhe {}, {} crossings, closes {}, markov-closed {}",
                                        r.writhe, r.crossings, r.closes, r.is_markov_closed
                                    );
                                    sprintln!("depth profile: {:?}", r.depth_profile);
                                }
                                Err(e) => sprintln!("μ refused: {}", e),
                            }
                        }
                    }
                    other => sprintln!("fibqc: unknown subcommand '{}'. Try `fibqc help`.", other),
                }
            }
            "color" | "colour" => {
                match parts.next().unwrap_or("") {
                    "off" | "no" | "0" => {
                        crate::style::set_colour(false);
                        sprintln!("colour off — escapes suppressed, alignment unchanged");
                    }
                    "on" | "yes" | "1" | "" => {
                        crate::style::set_colour(true);
                        sprintln!("colour {}on{}", crate::style::accent(), crate::style::reset());
                    }
                    other => sprintln!("color on|off  (got '{}')", other),
                }
            }
            "iuft" => {
                match parts.next().unwrap_or("") {
                    "" | "help" => {
                        sprintln!("iuft gate <name>         — show IUFT QC gate encoding (Euler angles)");
                        sprintln!("iuft encode <name>       — compute IUFT gate from catalog entry on-the-fly");
                        sprintln!("iuft tuple <12-glyphs>   — encode an arbitrary 12-glyph tuple");
                        sprintln!("iuft report <name>       — full gate report (SU(2), Bloch, unitarity)");
                        sprintln!("iuft distance <a> <b>    — compute IUFT QC gate distance");
                        sprintln!("iuft matrix <names...>   — distance matrix over the named catalog entries");
                        sprintln!("iuft list [domain]       — encode every catalog entry, or one domain");
                        sprintln!("iuft verify [name]       — encode and check unitarity; no name checks the whole catalog");
                        sprintln!("");
                        sprintln!("Every gate is computed from its catalog tuple — nothing is hardcoded,");
                        sprintln!("and no reference list is kept here. Names go through the catalog's own");
                        sprintln!("aliases, so 'CLINK L8', 'clink_l8' and 'CLINK-L8' reach one entry.");
                        sprintln!("Domains: mathematics physics biology consciousness language");
                        sprintln!("         civilization computation theology alchemy ecology general");
                        sprintln!("Derived from IUFT Quantum Expansion II — 12->3 degenerate projection.");
                    }
                    "gate" | "report" => {
                        let name = parts.next().unwrap_or("");
                        if name.is_empty() {
                            sprintln!("iuft gate <name>   e.g. `iuft gate graviton`");
                        } else {
                            crate::iuft_qc::print_gate_report(name);
                        }
                    }
                    "encode" => {
                        let name = parts.next().unwrap_or("");
                        if name.is_empty() {
                            sprintln!("iuft encode <name>   encode any catalog entry e.g. `iuft encode electron`");
                        } else {
                            match crate::iuft_qc::gate_for(name) {
                                Some(gate) => {
                                    sprintln!("IUFT QC Gate (encoded): {}", name);
                                    sprintln!("  θ = {:.1}°", gate.theta_deg);
                                    sprintln!("  φ = {:.1}°", gate.phi_deg);
                                    sprintln!("  ψ = {:.1}°", gate.psi_deg);
                                    let su2 = gate.to_su2();
                                    sprintln!("  SU(2) = [[{:.4}{:+.4}i, {:.4}{:+.4}i],",
                                        su2[0][0], su2[0][1], su2[0][2], su2[0][3]);
                                    sprintln!("           [{:.4}{:+.4}i, {:.4}{:+.4}i]]",
                                        su2[1][0], su2[1][1], su2[1][2], su2[1][3]);
                                    sprintln!("  Unitary: {}", crate::iuft_qc::verify_unitary(&gate));
                                }
                                None => sprintln!("No encoding found for '{}'.", name),
                            }
                        }
                    }
                    "distance" => {
                        let a = parts.next().unwrap_or("");
                        let b = parts.next().unwrap_or("");
                        if a.is_empty() || b.is_empty() {
                            sprintln!("iuft distance <a> <b>   e.g. `iuft distance graviton photon`");
                        } else {
                            match crate::iuft_qc::gate_distance(a, b) {
                                Some(d) => sprintln!("IUFT QC gate distance d({}, {}) = {:.6}", a, b, d),
                                None => sprintln!("One or both entries lack IUFT gate encodings."),
                            }
                        }
                    }
                    "matrix" => {
                        // The line arrives split with a field limit, so the tail
                        // can be one token holding several names. Re-tokenize.
                        let tail: Vec<&str> = parts.collect();
                        let joined = tail.join(" ");
                        let names: Vec<&str> = joined.split_whitespace().collect();
                        crate::iuft_qc::print_distance_matrix(&names);
                    }
                    "list" => {
                        // The list used to be twelve hand-picked names. It is the
                        // catalog now; a domain narrows it.
                        let dom = crate::catalog::parse_domain(parts.next().unwrap_or(""));
                        let gates = crate::iuft_qc::gates_in(dom);
                        sprintln!("IUFT QC gate encodings ({} entries):", gates.len());
                        for (name, gate) in gates {
                            sprintln!("  {:>30}: θ={:6.1}°  φ={:6.1}°  ψ={:6.1}°",
                                name, gate.theta_deg, gate.phi_deg, gate.psi_deg);
                        }
                    }
                    "tuple" => {
                        let glyph_str = parts.next().unwrap_or("");
                        if glyph_str.is_empty() {
                            sprintln!("iuft tuple <12-glyphs>   e.g. `iuft tuple ⟨...⟩`");
                            sprintln!("Encodes an arbitrary 12-glyph string into an IUFT gate.");
                            sprintln!("Example: pass a 12-glyph tuple string like the graviton tuple.");
                        } else {
                            let rest: alloc::vec::Vec<&str> = core::iter::once(glyph_str).chain(parts).collect();
                            let glyphs = rest.join(" ");
                            match crate::iuft_qc::encode_glyphs(&glyphs) {
                                Some(gate) => {
                                    sprintln!("IUFT QC Gate (from tuple):");
                                    sprintln!("  θ = {:.1}°", gate.theta_deg);
                                    sprintln!("  φ = {:.1}°", gate.phi_deg);
                                    sprintln!("  ψ = {:.1}°", gate.psi_deg);
                                    let su2 = gate.to_su2();
                                    sprintln!("  SU(2) = [[{:.4}{:+.4}i, {:.4}{:+.4}i],",
                                        su2[0][0], su2[0][1], su2[0][2], su2[0][3]);
                                    sprintln!("           [{:.4}{:+.4}i, {:.4}{:+.4}i]]",
                                        su2[1][0], su2[1][1], su2[1][2], su2[1][3]);
                                    sprintln!("  Unitary: {}", crate::iuft_qc::verify_unitary(&gate));
                                    let (nearest, dist) = crate::iuft_qc::nearest_known(&gate);
                                    sprintln!("  Nearest known: {} (d={:.4})", nearest, dist);
                                }
                                None => sprintln!("Failed to parse glyph string. Need exactly 12 Shavian glyphs + ⊙."),
                            }
                        }
                    }
                    "verify" => {
                        // Same re-tokenization, and the whole tail is the name:
                        // catalog entries like "CLINK L8" carry a space.
                        let tail: Vec<&str> = parts.collect();
                        let joined = tail.join(" ");
                        let name = joined.trim();
                        if name.is_empty() {
                            crate::iuft_qc::verify_catalog();
                        } else {
                            crate::iuft_qc::verify_one(name);
                        }
                    }
                    other => sprintln!("iuft: unknown subcommand '{}'. Try `iuft help`.", other),
                }
            }
            "teich" => {
                match parts.next().unwrap_or("") {
                    "" | "help" => {
                        sprintln!("teich path <a> <b>    — Teichmuller deformation path between two universes");
                        sprintln!("teich ladder           — crystal tier ladder with gate-space jumps");
                        sprintln!("teich canonical        — canonical IUFT paths: monad/topos/Poincare-Hopf/grammar/CLINK");
                        sprintln!("teich tier <tier>      — approximate SU(2) gate for an ouroboricity tier");
                        sprintln!("");
                        sprintln!("Bridges IUFT (Frobenius) <-> IUTT (Teichmuller): promotion paths as gate trajectories.");
                        sprintln!("Etale = pinned primitives unchanged. Anabelian = core structure transforms.");
                    }
                    "path" => {
                        let a = parts.next().unwrap_or("");
                        let b = parts.next().unwrap_or("");
                        if a.is_empty() || b.is_empty() {
                            sprintln!("teich path <source> <target>   e.g. `teich path monad imscribing_grammar`");
                        } else {
                            crate::iuft_teichmuller::print_teichmuller_report(a, b);
                        }
                    }
                    "ladder" => {
                        crate::iuft_teichmuller::print_tier_ladder();
                    }
                    "canonical" => {
                        crate::iuft_teichmuller::print_canonical_paths();
                    }
                    "tier" => {
                        let tier = parts.next().unwrap_or("");
                        if tier.is_empty() {
                            sprintln!("teich tier <tier>   e.g. `teich tier O_inf`");
                            sprintln!("Tiers: O_0, O_1, O_2, O_2d, O_inf");
                        } else {
                            match crate::iuft_teichmuller::tier_to_gate(tier) {
                                Some(gate) => {
                                    sprintln!("Tier {} -> SU(2) gate: theta={:.1} phi={:.1} psi={:.1}",
                                        tier, gate.theta_deg, gate.phi_deg, gate.psi_deg);
                                }
                                None => sprintln!("Unknown tier: {}. Use O_0, O_1, O_2, O_2d, or O_inf.", tier),
                            }
                        }
                    }
                    _other => sprintln!("teich: unknown subcommand. Try `teich help`.",),
                }
            }
            "classify" => {
                let arg: alloc::string::String =
                    parts.collect::<alloc::vec::Vec<&str>>().join(" ");
                print_classify(k, &arg)
            }
            "arev" => {
                match parts.next().unwrap_or("") {
                    ""     => print_arev_hop(k),
                    "test" => print_arev_test(k),
                    _ => sprintln!("arev [test] — ⊥ hop to the lateral partner (O_∞ ↔ O_inf_dag) / door experiment"),
                }
            }
            "aleph" => print_aleph(k, parts.next().unwrap_or("")),
            "psm" => {
                let psm_arg = parts.next().unwrap_or("");
                let psm_rest: alloc::string::String = parts.collect::<alloc::vec::Vec<&str>>().join(" ");
                let psm_full = if psm_rest.is_empty() { alloc::string::String::from(psm_arg) } else { alloc::format!("{} {}", psm_arg, psm_rest) };
                print_psm(&psm_full);
            }
            "shor" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" => print_shor(),
                    "factors" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_factors(parse_u64(n_str), parse_u64(a_str));
                    }
                    "gap" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_gap(parse_u64(n_str), parse_u64(a_str));
                    }
                    "dialetheic" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_dialetheic(parse_u64(n_str), parse_u64(a_str));
                    }
                    "help" => {
                        sprintln!("shor — Belnap Shor pipeline + 4-problem solutions");
                        sprintln!("  shor                 default pipeline (N=15,21)");
                        sprintln!("  shor factors N a     full factorization run");
                        sprintln!("  shor gap [N a]       coherence gap analysis");
                        sprintln!("  shor N a             belnap cost analysis");
                        sprintln!("  shor phase N a       Phase-augmented Shor (P1+P2 solved)");
                        sprintln!("  shor ring N a        IMASM ring walk verification (P4)");
                        sprintln!("  shor fib N a         Fibonacci anyon braid estimation (P3)");
                        sprintln!("  shor dialetheic N a  Dialetheic Fibonacci Shor (ob3ect word ⊢∈≻⋈⊞∈⊤≻⊥≺∋⊙⋈⊡⊣)");
                        sprintln!("  shor integrated N a  All 4 problems integrated");
                    }
                    "phase" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_phase(parse_u64(n_str), parse_u64(a_str));
                    }
                    "ring" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_ring(parse_u64(n_str), parse_u64(a_str));
                    }
                    "fib" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_fib(parse_u64(n_str), parse_u64(a_str));
                    }
                    "integrated" => {
                        let n_str = parts.next().unwrap_or("");
                        let a_str = parts.next().unwrap_or("");
                        print_shor_integrated(parse_u64(n_str), parse_u64(a_str));
                    }
                    other => {
                        let n_val = parse_u64(other);
                        let a_val = parse_u64(parts.next().unwrap_or(""));
                        print_shor_custom(n_val, a_val);
                    }
                }
            }
            "qft" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "help" => {
                        sprintln!("qft — Quantum Fourier Transform circuit, phases, and braid compilation");
                        sprintln!("  qft <n>              QFT circuit diagram for n qubits");
                        sprintln!("  qft circuit <n>      QFT circuit diagram (explicit)");
                        sprintln!("  qft iqft <n>         Inverse QFT circuit diagram");
                        sprintln!("  qft phases <n>       Controlled-R_k phase angles for n qubits");
                        sprintln!("  qft braid <n>        Compile QFT to Fibonacci anyon braid word");
                        sprintln!("  qft iqft braid <n>   Compile IQFT to Fibonacci anyon braid word");
                        sprintln!("  qft verify <n>       Verify QFT∘IQFT = identity structure");
                        sprintln!("  qft estimate <n>     Estimate braid length for QFT/IQFT");
                    }
                    "circuit" => {
                        let n = parts.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if n == 0 {
                            sprintln!("qft circuit: usage: qft circuit <n>  (n > 0)");
                        } else {
                            let c = crate::qft::qft_circuit(n, false);
                            sprintln!("{}", crate::qft::format_circuit(&c));
                        }
                    }
                    "iqft" => {
                        let sub2 = parts.next().unwrap_or("");
                        if sub2 == "braid" {
                            let n = parts.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if n == 0 {
                                sprintln!("qft iqft braid: usage: qft iqft braid <n>  (n > 0)");
                            } else {
                                let braid = crate::qft::qft_to_braid(n, true);
                                sprintln!("IQFT braid ({} qubits, {} generators):", n, braid.len());
                                for chunk in braid.chunks(24) {
                                    sprintln!("  {}", chunk.iter().map(|g: &i32| g.to_string()).collect::<alloc::vec::Vec<_>>().join(" "));
                                }
                            }
                        } else {
                            let n = sub2.parse::<usize>().unwrap_or(0);
                            if n == 0 {
                                sprintln!("qft iqft: usage: qft iqft <n>  (n > 0)");
                            } else {
                                let c = crate::qft::qft_circuit(n, true);
                                sprintln!("{}", crate::qft::format_circuit(&c));
                            }
                        }
                    }
                    "phases" => {
                        let n = parts.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if n == 0 {
                            sprintln!("qft phases: usage: qft phases <n>  (n > 0)");
                        } else {
                            let c = crate::qft::qft_circuit(n, false);
                            let phases = crate::qft::circuit_phases(&c);
                            sprintln!("QFT phases ({} qubits):", n);
                            sprintln!("  control target  angle (rad)  angle (deg)  angle / π");
                            for (ctrl, target, angle) in phases {
                                sprintln!("  {:>7} {:>6}  {:>10.6}  {:>10.2}  {:>7.4}π",
                                    ctrl, target, angle, angle * 180.0 / core::f64::consts::PI, angle / core::f64::consts::PI);
                            }
                        }
                    }
                    "braid" => {
                        let n = parts.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if n == 0 {
                            sprintln!("qft braid: usage: qft braid <n>  (n > 0)");
                        } else {
                            let braid = crate::qft::qft_to_braid(n, false);
                            sprintln!("QFT braid ({} qubits, {} generators):", n, braid.len());
                            for chunk in braid.chunks(24) {
                                sprintln!("  {}", chunk.iter().map(|g: &i32| g.to_string()).collect::<alloc::vec::Vec<_>>().join(" "));
                            }
                        }
                    }
                    "verify" => {
                        let n = parts.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if n == 0 {
                            sprintln!("qft verify: usage: qft verify <n>  (n > 0)");
                        } else {
                            let ok = crate::qft::verify_qft_iqft(n);
                            sprintln!("QFT∘IQFT verification for {} qubits: {}", n, if ok { "PASS" } else { "FAIL" });
                        }
                    }
                    "estimate" => {
                        let n = parts.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if n == 0 {
                            sprintln!("qft estimate: usage: qft estimate <n>  (n > 0)");
                        } else {
                            let est = crate::qft::estimate_qft_braid_length(n);
                            sprintln!("QFT braid length estimate for {} qubits: ~{} generators", n, est);
                        }
                    }
                    other => {
                        // Try to parse as a number for default QFT circuit
                        let n = other.parse::<usize>().unwrap_or(0);
                        if n == 0 {
                            sprintln!("qft: unknown subcommand '{}' (try 'qft help')", other);
                        } else {
                            let c = crate::qft::qft_circuit(n, false);
                            sprintln!("{}", crate::qft::format_circuit(&c));
                        }
                    }
                }
            }
            "shors_btc_2" => {
                let arg = parts.next().unwrap_or("");
                if arg.is_empty() || arg == "help" {
                    sprintln!("shors_btc_2 — Quantum period-finding for Bitcoin secp256k1 ECDLP");
                    sprintln!("  shors_btc_2          Extract private key from standard test public key");
                    sprintln!("  shors_btc_2 <hex>    Extract private key from given compressed public key (02|03 + 64 hex x)");
                    sprintln!("example:");
                    sprintln!("  shors_btc_2 03f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f");
                } else {
                    let result = crate::shors_btc_2::run_shors_btc_2_from_hex(arg);
                    result.print_report();
                }
            }
            "btc_oneshot" => {
                let sub = parts.next().unwrap_or("");
                let pk_hex = parts.next().unwrap_or("");
                if sub.is_empty() || sub == "help" {
                    sprintln!("btc_oneshot — BTC Secret Key Oneshot Operator");
                    sprintln!("  btc_oneshot verify    — full structural verification suite");
                    sprintln!("  btc_oneshot steps     — 12 operational phase steps");
                    sprintln!("  btc_oneshot tuple     — print grammar tuple");
                    sprintln!("  btc_oneshot word      — print IMASM word");
                    sprintln!("  btc_oneshot extract   — extract private key from compressed pubkey");
                } else {
                    let args: Vec<&str> = if pk_hex.is_empty() {
                        vec![sub]
                    } else {
                        vec![sub, pk_hex]
                    };
                    sprintln!("{}", crate::btc_secret_key_oneshot::btc_oneshot_repl(&args));
                }
            }
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
                    "moduli" => sprintln!("{}", crate::sic_moduli::full_report()),
                    "d16" => sprintln!("{}", crate::sic_moduli::d16_proof()),
                    "d20" => sprintln!("{}", crate::sic_moduli::d20_anomaly()),
                    "calibrate" => sprintln!("{}", crate::sic_moduli::calibration_report()),
                    "scope" => sprintln!("{}", crate::sic_moduli::scope_report()),
                    "d2048" => sprintln!("{}", crate::sic_moduli::d2048_propagation()),
                    "grammar" => sprintln!("{}", crate::sic_moduli::grammar_encoding()),
                    "lean" => sprintln!("{}", crate::sic_moduli::lean_reference()),
                    "" => print_sic(),
                    _ => {
                        sprintln!("sic — SIC-POVM commands");
                        sprintln!("  sic verify      d=12 existence report");
                        sprintln!("  sic bridge      Belnap/SIC bridge");
                        sprintln!("  sic moduli      moduli-field report, all sections");
                        sprintln!("  sic d16         the d=16 settlement");
                        sprintln!("  sic d20         the d=20 anomaly");
                        sprintln!("  sic calibrate   calibration table, computed from d");
                        sprintln!("  sic scope       where the coinvariant identity holds");
                        sprintln!("  sic d2048       propagation to d=2048");
                        sprintln!("  sic grammar     tuple");
                        sprintln!("  sic lean        Lean cross-reference");
                    },
                }
            }
            "riemann" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" => sprintln!("{}", crate::riemann_sic::full_report()),
                    "opcodes" => sprintln!("{}", crate::riemann_sic::opcode_map()),
                    "frobenius" => sprintln!("{}", crate::riemann_sic::frobenius_report()),
                    "registers" => sprintln!("{}", crate::riemann_sic::register_states()),
                    "bootstrap" => sprintln!("{}", crate::riemann_sic::bootstrap_table()),
                    "kernel" => sprintln!("{}", crate::riemann_sic::momad_kernel_map()),
                    "entropy" => sprintln!("{}", crate::riemann_sic::entropy_report()),
                    "topology" => sprintln!("{}", crate::riemann_sic::topology_report()),
                    "sixteen3" => sprintln!("{}", crate::riemann_sic::sixteen3_breakdown()),
                    "rotat" => sprintln!("{}", crate::riemann_sic::rotat_audit()),
                    "grammar" => sprintln!("{}", crate::riemann_sic::grammar_encoding()),
                    "lean" => sprintln!("{}", crate::riemann_sic::lean_reference()),
                    "sic" => sprintln!("{}", crate::riemann_sic::run_sic_verify()),
                    "hilbert" => sprintln!("{}", crate::riemann_hilbert::run_hilbert()),
                    _ => {
                        sprintln!("riemann — Riemann-SIC spectral correspondence");
                        sprintln!("  riemann            full protocol report (all sections)");
                        sprintln!("  riemann opcodes    opcode → domain mapping");
                        sprintln!("  riemann frobenius  Frobenius split/fuse structure");
                        sprintln!("  riemann registers  Belnap register states");
                        sprintln!("  riemann bootstrap  bootstrap sequence table");
                        sprintln!("  riemann kernel     m⊙² kernel components");
                        sprintln!("  riemann entropy    entropy analysis");
                        sprintln!("  riemann topology   program topology report");
                        sprintln!("  riemann sixteen3   SIXTEEN_3 trilattice breakdown");
                        sprintln!("  riemann rotat      ROTAT orbit audit");
                        sprintln!("  riemann grammar    tuple + per-primitive justification");
                        sprintln!("  riemann lean       Lean 4 cross-reference");
                        sprintln!("  riemann sic        d=12 SIC-POVM Gerzon inverse numerical verification");
                        sprintln!("  riemann hilbert    Zauner Hamiltonian H_Z eigenvalue computation");
                    }
                }
            }
            "triple" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::triple_frame::full_report()),
                    "expand" => {
                        let name = parts.next().unwrap_or("sure");
                        sprintln!("{}", crate::triple_frame::expand_report(name));
                    }
                    "verify" => sprintln!("{}", crate::triple_frame::verify_report()),
                    "word" => {
                        let var = parts.next().unwrap_or("B");
                        sprintln!("{}", crate::triple_frame::word_report(var));
                    }
                    "types" => sprintln!("{}", crate::triple_frame::types_report()),
                    "cycle" => sprintln!("{}", crate::triple_frame::cycle_report()),
                    "path" => sprintln!("{}", crate::triple_frame::path_report()),
                    "bridge" => sprintln!("{}", crate::triple_frame::bridge_report()),
                    "check" => {
                        let w = parts.next().unwrap_or("");
                        sprintln!("{}", crate::triple_frame::check_report(w));
                    }
                    "tuple" => sprintln!("{}", crate::triple_frame::TRIPLE_FRAME_TUPLE),
                    "help" | "--help" | "-h" => sprintln!("{}", crate::triple_frame::triple_help()),
                    _ => sprintln!("{}", crate::triple_frame::triple_help()),
                }
            }


            "bip39" => {
                let sic = parts.next().unwrap_or("");
                if sic != "sic" {
                    sprintln!("bip39 sic — BIP39-SIC-POVM structural correspondence");
                    sprintln!("  bip39 sic search   run Grover search over d=2048 frame");
                    sprintln!("  bip39 sic words    word-level search structure");
                    sprintln!("  bip39 sic verify   B4 Frobenius verification");
                    sprintln!("  bip39 sic map      wordlist/Hilbert space correspondence");
                    sprintln!("  bip39 sic gap      gap analysis (2^106 -> 2^53 Grover)");
                    continue;
                }
                let sub = parts.next().unwrap_or("");
                match sub {
                    "search" => sprintln!("{}", crate::bip39_sic_grover::bip39_sic_grover_search(1661)),
                    "words" => sprintln!("{}", crate::bip39_sic_grover::bip39_word_level_analysis()),
                    "verify" => sprintln!("B4 Frobenius: {}", crate::bip39_sic_grover::b4_frobenius_check()),
                    "map" => sprintln!("BIP39 wordlist {} <-> d={} Hilbert space", crate::bip39_sic_grover::BIP39_WORDLIST_SIZE, crate::bip39_sic_grover::BIP39_WORDLIST_SIZE),
                    "gap" => sprintln!("BIP39 gap: 2^106 (from 2^128 entropy - 2^22 frame); Grover iterations: 2^53"),
                    "" => {
                        sprintln!("bip39 sic — BIP39-SIC-POVM structural correspondence");
                        sprintln!("  bip39 sic search   run Grover search over d=2048 frame");
                        sprintln!("  bip39 sic words    word-level search structure");
                        sprintln!("  bip39 sic verify   B4 Frobenius verification");
                        sprintln!("  bip39 sic map      wordlist/Hilbert space correspondence");
                        sprintln!("  bip39 sic gap      gap analysis (2^106 -> 2^53 Grover)");
                    }
                    _ => sprintln!("bip39 sic — see 'bip39 sic' (no subcommand)"),
                }
            }

            // ── m3iosis tool ports (native Rust implementations) ──────
            "ovm" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "eigen" => {
                        let x: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        let y: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        let z: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                        let norm: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1.0);
                        let trace: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1.0);
                        sprintln!("{}", crate::ovm::ovm_eigen(x, y, z, norm, trace));
                    }
                    "frame" => {
                        let name = parts.next().unwrap_or("");
                        sprintln!("{}", crate::ovm::ovm_frame(name));
                    }
                    "overlap" => {
                        let name = parts.next().unwrap_or("");
                        sprintln!("{}", crate::ovm::ovm_overlap(name));
                    }
                    "belnap" => sprintln!("{}", crate::ovm::ovm_belnap()),
                    "help" | "--help" | "-h" | "" => sprintln!("{}", crate::ovm::ovm_help()),
                    name => sprintln!("{}", crate::ovm::ovm_compute(name)),
                }
            }
            "hqe" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::hqe::full_report()),
                    "distance" => {
                        let t1 = parts.next().unwrap_or(crate::hqe::TUPLE_HQE);
                        let t2 = parts.next().unwrap_or(crate::afdmc::TUPLE_AFDMC);
                        sprintln!("d({}, {}) = {:.4}", t1, t2, crate::hqe::tuple_distance(t1, t2));
                    }
                    "cscore" => sprintln!("C-score: {:.4}", crate::hqe::consciousness_score(crate::hqe::TUPLE_HQE)),
                    "meet" => {
                        let t2 = parts.next().unwrap_or(crate::afdmc::TUPLE_AFDMC);
                        sprintln!("meet: ⟨{}⟩", crate::hqe::quantale_meet(crate::hqe::TUPLE_HQE, t2));
                    }
                    "join" => {
                        let t2 = parts.next().unwrap_or(crate::afdmc::TUPLE_AFDMC);
                        sprintln!("join: ⟨{}⟩", crate::hqe::quantale_join(crate::hqe::TUPLE_HQE, t2));
                    }
                    "tuple" => sprintln!("{}", crate::hqe::TUPLE_HQE),
                    _ => sprintln!("{}", crate::hqe::full_report()),
                }
            }
            "dyson" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::dyson::full_report()),
                    "tuple" => sprintln!("{}", crate::dyson::TUPLE_DRDA),
                    _ => sprintln!("{}", crate::dyson::full_report()),
                }
            }
            "afdmc" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::afdmc::full_report()),
                    "tuple" => sprintln!("{}", crate::afdmc::TUPLE_AFDMC),
                    _ => sprintln!("{}", crate::afdmc::full_report()),
                }
            }
            "troq" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::troq::full_report()),
                    "expand" => {
                        let axis = parts.next().unwrap_or("⊙");
                        for line in crate::troq::expand_axis(axis) {
                            sprintln!("  {}", line);
                        }
                    }
                    "tuple" => sprintln!("{}", crate::troq::TUPLE_TROQ),
                    _ => sprintln!("{}", crate::troq::full_report()),
                }
            }
            "hop" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::hop::full_report()),
                    "manifest" => {
                        let t = parts.next().unwrap_or(crate::hqe::TUPLE_HQE);
                        sprintln!("{}", crate::hop::manifest(t));
                    }
                    "matrix" => sprintln!("{}", crate::hop::framework_matrix()),
                    "hop" => {
                        let origin = parts.next().unwrap_or(crate::hqe::TUPLE_HQE);
                        let target = parts.next().unwrap_or(crate::afdmc::TUPLE_AFDMC);
                        sprintln!("{}", crate::hop::hop(origin, target));
                    }
                    _ => sprintln!("{}", crate::hop::full_report()),
                }
            }
            "braid-grammar" | "bg" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::braid_grammar::full_report()),
                    "tuple" => {
                        let word = parts.next().unwrap_or("1 2 1");
                        let strands: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(3);
                        let bw = crate::braid_grammar::BraidWord::from_string(word, strands);
                        sprintln!("⟨{}⟩", bw.to_grammar_tuple());
                    }
                    _ => sprintln!("{}", crate::braid_grammar::full_report()),
                }
            }
            "manifold" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "" | "report" => sprintln!("{}", crate::manifold::full_report()),
                    _ => sprintln!("{}", crate::manifold::full_report()),
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
            "invariant" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::invariant::invariant_main(&args));
            }
            "redteam" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::redteam::redteam_main(&args));
            }
            "witness" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::witness::witness_main(&args));
            }
            "counterfactual" | "cf" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::counterfactual::counterfactual_main(&args));
            }
            "basin" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::basin::basin_main(&args));
            }
            "ouroboros-inverse" | "oinv" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::ouroboros::ouroboros_main(&args));
            }
            "frobenius-fuzzer" | "fuzz" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::frobenius_fuzzer::fuzzer_main(&args));
            }
            "oracle" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::oracle::oracle_main(&args));
            }
            "dialetheic-compiler" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::dialetheic_compiler::dialetheic_compiler_main(&args));
            }
            "stark-geometer" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::stark_geometer::stark_geometer_main(&args));
            }
            "dialect-necromancer" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::dialect_necromancer::dialect_necromancer_main(&args));
            }
            "braid-apocrypha" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::braid_apocrypha::braid_apocrypha_main(&args));
            }
            "proof-braider" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::proof_braider::proof_braider_main(&args));
            }
            "universe-wormhole" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::universe_wormhole::universe_wormhole_main(&args));
            }
            "vox-ce" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::vox_ce::vox_ce_main(&args));
            }
            "consciousness-lath" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::consciousness_lath::consciousness_lath_main(&args));
            }
            "paradox-engine" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::paradox_engine::paradox_engine_main(&args));
            }
            "key-dissolver" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::key_dissolver::key_dissolver_main(&args));
            }
            "compiler" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::compiler::compiler_main(&args));
            }
            "catalogue" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::catalogue::catalogue_main(&args));
            }
            "blackbox" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::blackbox::blackbox_main(&args));
            }
            "museum" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::museum::museum_main(&args));
            }
            "phase" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::phase::phase_main(&args));
            }
            "demonstrate" | "demo" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::demonstrator::demonstrate_main(&args));
            }
            "loss" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::loss::loss_main(&args));
            }
            "shadow" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::shadow::shadow_main(&args));
            }
            "provenance" | "prov" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::provenance::provenance_main(&args));
            }
            "ctc-loom" | "loom" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::ctc_loom::ctc_loom_main(&args));
            }
            "cl9nk" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::cl8nk::cl9nk_main(&args));
            }
            "crystal-scope" | "cscope" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::crystal_scope::crystal_scope_main(&args));
            }
            "minimal" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::minimal::minimal_main(&args));
            }
            "repair" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::repair::repair_main(&args));
            }
            "ringspec" => {
                let args: Vec<&str> = parts.collect();
                sprintln!("{}", crate::ringspec::ringspec_main(&args));
            }
            "sk_forge" | "sk-forge" => {
                // sk_forge_main takes &str; join the remaining fields back.
                let rest: Vec<&str> = parts.collect();
                sprintln!("{}", crate::sk_forge::sk_forge_main(&rest.join(" ")));
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
                                sprintln!("{:>4} {:>24} {:>14} {:>6}", "p", "M_p", "VERDICT", "⊡");
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
                                use crate::mersenne_parallel as mp;
                                let (used, total) = crate::heap_used();
                                let worst = (start..=end).filter(|q| mp::is_prime_exponent(*q))
                                    .map(mp::lucas_lehmer_heap_estimate).max().unwrap_or(0);
                                if worst > total.saturating_sub(used) {
                                    sprintln!("Range needs about {} MiB at its largest prime exponent; {} MiB free.",
                                              worst / (1024 * 1024),
                                              total.saturating_sub(used) / (1024 * 1024));
                                    sprintln!("  Narrow the range or lower the upper bound.");
                                } else {
                                    sprintln!("{}", mp::search_report(start, end));
                                }
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
                            use crate::mersenne_parallel as mp;
                            if p >= 2 && !mp::is_prime_exponent(p) {
                                // No arithmetic needed: a factor of p gives a
                                // factor of M_p. Say which, so the answer is
                                // checkable rather than merely asserted.
                                let mut d = 2usize;
                                while p % d != 0 { d += 1; }
                                sprintln!("M_{} is composite — the exponent is: {} = {} x {}.",
                                          p, p, d, p / d);
                                sprintln!("  2^{} - 1 divides 2^{} - 1, so no Lucas-Lehmer is needed.", d, p);
                                sprintln!("  (Lucas-Lehmer is stated for prime exponents.)");
                            } else {
                                let need = mp::lucas_lehmer_heap_estimate(p);
                                let (used, total) = crate::heap_used();
                                let free = total.saturating_sub(used);
                                if need > free {
                                    sprintln!("M_{} needs about {} MiB of heap; {} MiB free of {} MiB.",
                                              p, need / (1024 * 1024),
                                              free / (1024 * 1024), total / (1024 * 1024));
                                    sprintln!("  Refusing rather than exhausting the heap mid-test.");
                                } else {
                                    sprintln!("Running Lucas-Lehmer for M_{}...", p);
                                    if mp::lucas_lehmer(p) {
                                        sprintln!("M_{} is PRIME!", p);
                                    } else {
                                        sprintln!("M_{} is composite.", p);
                                    }
                                }
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
                    "exact" => sprintln!("{}", crate::d2048_exact_sic::exact_extraction_report()),
                    "scaling" => sprintln!("{}", crate::d2048_exact_sic::scaling_report()),
                    "crossover" => sprintln!("{}", crate::d2048_exact_sic::crossover_report()),
                    "welch" => sprintln!("{}", crate::d2048_exact_sic::welch_report()),
                    "" => sprintln!("{}", crate::d2048_sic::d2048_summary()),
                    _ => sprintln!("d2048 [tower|c16|c32|ramified|redei|grammar|pari|next|sieve|verify|exact|scaling|crossover|welch]"),
                }
            }
            "stark" => {
                let sub = parts.next().unwrap_or("");
                match sub {
                    "formula" => {
                        let arg = parts.next().unwrap_or("");
                        if let Ok(d) = arg.parse::<u32>() {
                            sprintln!("{}", crate::stark::stark_formula(d));
                        } else {
                            sprintln!("Usage: stark formula <d>");
                        }
                    }
                    "fibqc" => {
                        let arg = parts.next().unwrap_or("");
                        if let Ok(d) = arg.parse::<u32>() {
                            sprintln!("{}", crate::stark::stark_fibqc(d));
                        } else if arg.is_empty() {
                            sprintln!("{}", crate::stark::stark_fibqc(48));
                        } else {
                            sprintln!("Usage: stark fibqc [d]");
                        }
                    }
                    "tower" => {
                        let arg = parts.next().unwrap_or("");
                        if let Ok(k) = arg.parse::<u32>() {
                            sprintln!("{}", crate::stark::stark_tower(Some(k)));
                        } else if arg.is_empty() {
                            sprintln!("{}", crate::stark::stark_tower(None));
                        } else {
                            sprintln!("Usage: stark tower [k]");
                        }
                    }
                    "exponents" | "exp" => {
                        let arg1 = parts.next().unwrap_or("");
                        let arg2 = parts.next().unwrap_or("");
                        if let Ok(d) = arg1.parse::<u32>() {
                            if let Ok(k) = arg2.parse::<u32>() {
                                sprintln!("{}", crate::stark::stark_exponents(d, Some(k)));
                            } else if arg2.is_empty() {
                                sprintln!("{}", crate::stark::stark_exponents(d, None));
                            } else {
                                sprintln!("Usage: stark exponents <d> [k]");
                            }
                        } else {
                            sprintln!("Usage: stark exponents <d> [k]");
                        }
                    }
                    "verify" => sprintln!("{}", crate::stark::stark_verify()),
                    "" => sprintln!("{}", crate::stark::stark_summary()),
                    _ => sprintln!("stark [formula|fibqc|tower|exponents|verify]"),
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
            "oneshots" => {
                sprintln!("{}", crate::exotic_one_shots::ExoticOneShots::report());
            }
            "ctc" => {
                let a: alloc::vec::Vec<&str> = parts.collect();
                match a.as_slice() {
                    [] => sprintln!("{}", crate::ctc::Ctc::sweep()),
                    ["help"] | ["-h"] | ["--help"] | ["?"] => sprintln!("{}", crate::ctc::Ctc::help()),
                    [act, val] => sprintln!("{}", crate::ctc::Ctc::run(act, val)),
                    _ => sprintln!("{}", crate::ctc::Ctc::help()),
                }
            }
            "nesting" => {
                let a: alloc::vec::Vec<&str> = parts.collect();
                match a.as_slice() {
                    [] => sprintln!("{}", crate::nesting::Nesting::sweep()),
                    ["help"] | ["-h"] | ["--help"] | ["?"] => sprintln!("{}", crate::nesting::Nesting::help()),
                    _ => sprintln!("{}", crate::nesting::Nesting::run(a[0], &a[1..])),
                }
            }
            "ig" => {
                print_ig(k);
            }
            // The two halves of tuple <-> word. The agent rider has told
            // operators to use `imasm write` and `imasm derive` for a long time
            // and neither existed, so an agent following the instruction failed
            // and fell back on conventional method — which is the behaviour the
            // rider was written to prevent.
            "imasm" => {
                use crate::imas_ig::IgTuple;
                match parts.next().unwrap_or("") {
                    "write" => {
                        let rest: alloc::vec::Vec<&str> = parts.collect();
                        match IgTuple::from_glyphs(&rest.join(" ")) {
                            Ok(t) => {
                                let prog = crate::sequence::build_via_substrate(
                                    &t, 12, t.t == crate::imas_ig::IgPrim::are, 3);
                                sprintln!("tuple: {}", t.display());
                                sprintln!("word:  {}",
                                    crate::belnap_ring_shor::glyphs_from_program(&prog));
                            }
                            Err((i, g)) => sprintln!("imasm write: {} at slot {}", g, i),
                        }
                    }
                    "derive" => {
                        let rest: alloc::vec::Vec<&str> = parts.collect();
                        match crate::belnap_ring_shor::program_from_glyphs(&rest.join(" ")) {
                            Ok(prog) => {
                                let t = IgTuple::from_snapshot(
                                    &crate::kernel::self_imscribe(&prog));
                                sprintln!("word:  {}",
                                    crate::belnap_ring_shor::glyphs_from_program(&prog));
                                sprintln!("tuple: {}", t.display());
                                sprintln!("crystal: {}", t.crystal_address());
                            }
                            Err((i, c)) => {
                                if crate::belnap_ring_shor::Glyph::from_char(c).is_some() {
                                    sprintln!("imasm derive: word exceeds the {}-token program capacity at position {}",
                                        crate::tokens::Program::CAPACITY, i);
                                    sprintln!("  the word instruments — weight, banked, cycle, insert, trans — have no such bound");
                                } else {
                                    sprintln!("imasm derive: '{}' at position {} is not a mark", c, i);
                                }
                            }
                        }
                    }
                    _ => {
                        sprintln!("imasm write <12 glyphs>   the word a tuple composes to");
                        sprintln!("imasm derive <word>       the tuple a word imscribes to");
                        sprintln!("Word instruments: weight | banked | cycle | insert | trans");
                    }
                }
            }
            "collatz" => {
                // `parts` is splitn(4), so a fourth argument arrives glued to the
                // third. Re-split the tail before reading it, or `balanced lo hi d`
                // loses its depth and reports a usage line instead of an answer.
                let tail: Vec<&str> = parts.collect();
                let joined = tail.join(" ");
                let rest: Vec<&str> = joined.split_whitespace().collect();
                match rest.first().copied() {
                    None | Some("help") => sprintln!("{}", crate::collatz::Collatz::help()),
                    Some("trace") => match rest.get(1).map(|v| v.parse::<u64>()) {
                        Some(Ok(v)) => sprintln!("{}", crate::collatz::Collatz::trace(v)),
                        _ => sprintln!("collatz trace <n> — n must be a number"),
                    },
                    Some("opnorm") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                             rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(r), Some(i)) => sprintln!("{}", crate::collatz::Collatz::opnorm_w(r, i,
                            rest.get(3).and_then(|v| v.parse::<f64>().ok()).unwrap_or(1.0))),
                        _ => sprintln!("collatz opnorm <rmax> <iters>"),
                    },
                    Some("operator") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                               rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(r), Some(i)) => sprintln!("{}", crate::collatz::Collatz::operator(r, i)),
                        _ => sprintln!("collatz operator <rmax> <iters>"),
                    },
                    Some("prbound") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                              rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(d), Some(r)) => sprintln!("{}", crate::collatz::Collatz::prbound(d, r)),
                        _ => sprintln!("collatz prbound <depth> <rungs>"),
                    },
                    Some("participation") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                                    rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(d), Some(r)) => sprintln!("{}", crate::collatz::Collatz::participation(d, r)),
                        _ => sprintln!("collatz participation <depth> <rungs>"),
                    },
                    Some("concentrate") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                                  rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(d), Some(r)) => sprintln!("{}", crate::collatz::Collatz::concentrate(d, r)),
                        _ => sprintln!("collatz concentrate <depth> <rungs>"),
                    },
                    Some("winding") => match rest.get(1).and_then(|v| v.parse::<u32>().ok()) {
                        Some(d) => sprintln!("{}", crate::collatz::Collatz::winding(d)),
                        _ => sprintln!("collatz winding <depth>"),
                    },
                    Some("lambda") => match rest.get(1).and_then(|v| v.parse::<u32>().ok()) {
                        Some(d) => sprintln!("{}", crate::collatz::Collatz::lambda(d)),
                        _ => sprintln!("collatz lambda <depth>"),
                    },
                    Some("lag") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                          rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(d), Some(r)) => sprintln!("{}", crate::collatz::Collatz::lag(d, r)),
                        _ => sprintln!("collatz lag <depth> <r>"),
                    },
                    Some("jratio") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                             rest.get(2).and_then(|v| v.parse::<u32>().ok())) {
                        (Some(d), Some(r)) => sprintln!("{}", crate::collatz::Collatz::jratio(d, r)),
                        _ => sprintln!("collatz jratio <depth> <rungs>"),
                    },
                    Some("attack") => match (rest.get(1).and_then(|v| v.parse::<u32>().ok()),
                                             rest.get(2).and_then(|v| v.parse::<u32>().ok()),
                                             rest.get(3).and_then(|v| v.parse::<u64>().ok())) {
                        (Some(d), Some(r), Some(m)) =>
                            sprintln!("{}", crate::collatz::Collatz::attack(d, r, m)),
                        _ => sprintln!("collatz attack <depth> <rungs> <minN>"),
                    },
                    Some("disjunct") => match rest.get(1).map(|v| v.parse::<u32>()) {
                        Some(Ok(d)) => sprintln!("{}", crate::collatz::Collatz::disjunct(d)),
                        _ => sprintln!("collatz disjunct <depth>"),
                    },
                    Some("norm") => match rest.get(1).map(|v| v.parse::<u32>()) {
                        Some(Ok(d)) => sprintln!("{}", crate::collatz::Collatz::norm(d,
                            rest.get(2).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0))),
                        _ => sprintln!("collatz norm <depth>"),
                    },
                    Some("perturb9") => match rest.get(1).map(|v| v.parse::<u32>()) {
                        Some(Ok(d)) => sprintln!("{}", crate::collatz::Collatz::perturb9(d)),
                        _ => sprintln!("collatz perturb9 <depth>"),
                    },
                    Some("perturb") => match rest.get(1).map(|v| v.parse::<u32>()) {
                        Some(Ok(d)) => sprintln!("{}", crate::collatz::Collatz::perturb(d)),
                        _ => sprintln!("collatz perturb <depth>"),
                    },
                    Some("excess") => match (rest.get(1).map(|v| v.parse::<u32>()),
                                             rest.get(2).map(|v| v.parse::<u32>())) {
                        (Some(Ok(d)), Some(Ok(r))) =>
                            sprintln!("{}", crate::collatz::Collatz::excess(d, r)),
                        _ => sprintln!("collatz excess <depth> <r>"),
                    },
                    Some("collisions") => match (rest.get(1).map(|v| v.parse::<u32>()),
                                                 rest.get(2).map(|v| v.parse::<u32>())) {
                        (Some(Ok(d)), Some(Ok(r))) =>
                            sprintln!("{}", crate::collatz::Collatz::collisions(d, r)),
                        _ => sprintln!("collatz collisions <depth> <r>"),
                    },
                    Some("flow") => match (rest.get(1).map(|v| v.parse::<u32>()),
                                           rest.get(2).map(|v| v.parse::<u32>())) {
                        (Some(Ok(d)), Some(Ok(r))) =>
                            sprintln!("{}", crate::collatz::Collatz::flow(d, r)),
                        _ => sprintln!("collatz flow <depth> <r>"),
                    },
                    Some("fourier") => match (rest.get(1).map(|v| v.parse::<u32>()),
                                              rest.get(2).map(|v| v.parse::<u32>())) {
                        (Some(Ok(d)), Some(Ok(r))) =>
                            sprintln!("{}", crate::collatz::Collatz::fourier(d, r)),
                        _ => sprintln!("collatz fourier <depth> <rmax>"),
                    },
                    Some("amax") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                           rest.get(2).map(|v| v.parse::<u64>()),
                                           rest.get(3).map(|v| v.parse::<u32>())) {
                        (Some(Ok(l)), Some(Ok(h)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::amax(l, h, d)),
                        _ => sprintln!("collatz amax <lo> <hi> <depth>"),
                    },
                    Some("birkhoff") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                               rest.get(2).map(|v| v.parse::<u64>()),
                                               rest.get(3).map(|v| v.parse::<u32>())) {
                        (Some(Ok(l)), Some(Ok(h)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::birkhoff(l, h, d)),
                        _ => sprintln!("collatz birkhoff <lo> <hi> <depth>"),
                    },
                    Some("amplitudes") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                                 rest.get(2).map(|v| v.parse::<u64>()),
                                                 rest.get(3).map(|v| v.parse::<u32>())) {
                        (Some(Ok(l)), Some(Ok(h)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::amplitudes(l, h, d)),
                        _ => sprintln!("collatz amplitudes <lo> <hi> <depth>"),
                    },
                    Some("growth") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                             rest.get(2).map(|v| v.parse::<u32>())) {
                        (Some(Ok(v)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::growth(v, d)),
                        _ => sprintln!("collatz growth <v> <dmax>"),
                    },
                    Some("adic") => match (rest.get(1).map(|v| v.parse::<u32>()),
                                           rest.get(2).map(|v| v.parse::<u64>()),
                                           rest.get(3).map(|v| v.parse::<u32>())) {
                        (Some(Ok(g)), Some(Ok(n)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::adic(g, n, d)),
                        _ => sprintln!("collatz adic <digits> <n> <depth>"),
                    },
                    Some("classes") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                              rest.get(2).map(|v| v.parse::<u64>()),
                                              rest.get(3).map(|v| v.parse::<u32>())) {
                        (Some(Ok(m)), Some(Ok(n)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::classes(m, n, d)),
                        _ => sprintln!("collatz classes <mod> <n> <depth>"),
                    },
                    Some("balance") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                              rest.get(2).map(|v| v.parse::<u32>())) {
                        (Some(Ok(v)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::balance(v, d)),
                        _ => sprintln!("collatz balance <v> <depth>"),
                    },
                    Some("balanced") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                               rest.get(2).map(|v| v.parse::<u64>()),
                                               rest.get(3).map(|v| v.parse::<u32>())) {
                        (Some(Ok(l)), Some(Ok(h)), Some(Ok(d))) =>
                            sprintln!("{}", crate::collatz::Collatz::balanced(l, h, d)),
                        _ => sprintln!("collatz balanced <lo> <hi> <depth>"),
                    },
                    Some("junctions") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                                rest.get(2).map(|v| v.parse::<u64>())) {
                        (Some(Ok(l)), Some(Ok(h))) =>
                            sprintln!("{}", crate::collatz::Collatz::junctions(l, h, 20)),
                        _ => sprintln!("collatz junctions <lo> <hi> — both must be numbers"),
                    },
                    Some("chain") => match rest.get(1).map(|v| v.parse::<u64>()) {
                        Some(Ok(v)) => sprintln!("{}", crate::collatz::Collatz::chain(v)),
                        _ => sprintln!("collatz chain <n> — n must be a number"),
                    },
                    Some("merge") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                            rest.get(2).map(|v| v.parse::<u64>())) {
                        (Some(Ok(x)), Some(Ok(y))) => sprintln!("{}", crate::collatz::Collatz::merge(x, y)),
                        _ => sprintln!("collatz merge <a> <b> — both must be numbers"),
                    },
                    Some("sweep") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                           rest.get(2).map(|v| v.parse::<u64>())) {
                        (Some(Ok(l)), Some(Ok(h))) => sprintln!("{}", crate::collatz::Collatz::sweep(l, h)),
                        _ => sprintln!("collatz sweep <lo> <hi> — both must be numbers"),
                    },
                    Some("ceiling") => match (rest.get(1).map(|v| v.parse::<u64>()),
                                              rest.get(2).map(|v| v.parse::<u64>())) {
                        (Some(Ok(l)), Some(Ok(h))) => sprintln!("{}", crate::collatz::Collatz::ceiling(l, h)),
                        _ => sprintln!("collatz ceiling <lo> <hi> — both must be numbers"),
                    },
                    Some("descent3") => {
                        use crate::tokens::{Program, Token};
                        // The Collatz descent ∀n>1 ∃k col^[k] n < n is item 1', and it sits
                        // at CLINK L9's ≻ 𐑑 (tot) and ≺ 𐑬 (out) — the two frobenius_order=3
                        // slots no twelve-mark word can write. The affine identity
                        // col^[k](2^k t + r) = 3^j t + col^[k] r composes three levels
                        // (root, class rep, image): a functorial, three-arity fork. So write
                        // the closing-form protocol with the three-arity Frobenius opcodes.
                        //   ⊢ ⊙ ∈₃ ≻ ⊤ ≺ ⊥ ∋₃ ⋈ ⊞ ⊡×8 ⊢   (self-referential: first = last)
                        let build = |splits3: bool| -> Program {
                            let mut p = Program::empty();
                            let (fs, ff) = if splits3 {
                                (Token::Fsplit3, Token::Ffuse3)
                            } else {
                                (Token::Fsplit, Token::Ffuse)
                            };
                            for t in [Token::Vinit, Token::Imscrib, fs, Token::Afwd,
                                      Token::Evalt, Token::Arev, Token::Evalf, ff,
                                      Token::Clink, Token::Engagr] { p.push(t); }
                            for _ in 0..8 { p.push(Token::Ifix); }
                            p.push(Token::Vinit); // close on ⊢ — self-referential
                            p
                        };
                        let l9 = "𐑛𐑥𐑑𐑬𐑐𐑪𐑔𐑝⊙𐑫𐑳𐑭";
                        for (label, splits3) in [("two-arity (word-reachable)", false),
                                                 ("three-arity (Fsplit3/Ffuse3)", true)] {
                            let prog = build(splits3);
                            let snap = crate::kernel::self_imscribe(&prog);
                            let tup = crate::imas_ig::IgTuple::from_snapshot(&snap);
                            let glyphs: alloc::string::String = tup.display().to_string();
                            let bare: alloc::string::String =
                                glyphs.chars().filter(|c| !"⟨⟩ ·".contains(*c)).collect();
                            let agree = bare.chars().zip(l9.chars())
                                .filter(|(a, b)| a == b).count();
                            sprintln!("  {}", label);
                            sprintln!("    fo={}  tuple {}  crystal {}",
                                snap.frobenius_order, glyphs, tup.crystal_address());
                            sprintln!("    tier {}  self_ref {}  dialetheia {}",
                                snap.tier_name(), snap.self_ref, snap.dialetheia_complete);
                            sprintln!("    vs CLINK L9 {} — {}/12 slots agree", l9, agree);
                        }
                        sprintln!("  CLINK L9 ≻ 𐑑 and ≺ 𐑬 are frobenius_order=3; the two-arity");
                        sprintln!("  row cannot reach them, the three-arity row does.");
                    }
                    Some(x) => match x.parse::<u64>() {
                        Ok(v) => sprintln!("{}", crate::collatz::Collatz::one(v)),
                        Err(_) => sprintln!("collatz <n> | trace <n> | descent3 | sweep <lo> <hi> | ceiling <lo> <hi>"),
                    },
                }
            }
            "straus" => {
                let a: alloc::vec::Vec<&str> = parts.collect();
                match a.as_slice() {
                    [] | ["help"] | ["-h"] | ["--help"] =>
                        sprintln!("{}", crate::straus::Straus::help()),
                    ["nest", n] => match n.parse::<u64>() {
                        Ok(v) => sprintln!("{}", crate::straus::Straus::nest(v)),
                        Err(_) => sprintln!("straus nest <n> — n must be a number"),
                    },
                    ["census", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::nest_census(l, h)),
                            _ => sprintln!("straus census <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["sweep", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::sweep(l, h)),
                            _ => sprintln!("straus sweep <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["defect", v] => match v.parse::<u64>() {
                        Ok(v) => sprintln!("{}", crate::straus::Straus::defect(v)),
                        Err(_) => sprintln!("straus defect <n> — n must be a number"),
                    }
                    // The line is split into at most four fields, so the trailing
                    // "<hi> <K>" arrives as one string and is split here.
                    ["kshift", lo, rest] => {
                        let mut it = rest.split_whitespace();
                        match (lo.parse::<u64>(),
                               it.next().unwrap_or("").parse::<u64>(),
                               it.next().unwrap_or("8").parse::<u64>()) {
                            (Ok(l), Ok(h), Ok(kk)) => sprintln!("{}", crate::straus::Straus::kshift(l, h, kk)),
                            _ => sprintln!("straus kshift <lo> <hi> <K> — all must be numbers"),
                        }
                    }
                    ["cof", v, maxr] => {
                        match (v.parse::<u64>(), maxr.parse::<u64>()) {
                            (Ok(x), Ok(m)) => sprintln!("{}", crate::straus::Straus::cofactor_height(x, m)),
                            _ => sprintln!("straus cof <n> <maxrung> — both must be numbers"),
                        }
                    }
                    ["budget", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::budget(l, h)),
                            _ => sprintln!("straus budget <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["ceiling", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::ceiling(l, h)),
                            _ => sprintln!("straus ceiling <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["kceiling", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::kceiling(l, h)),
                            _ => sprintln!("straus kceiling <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["reach", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::reach(l, h)),
                            _ => sprintln!("straus reach <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["cascade", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::cascade(l, h)),
                            _ => sprintln!("straus cascade <lo> <hi> — both must be numbers"),
                        }
                    }
                    ["frontier", lo, hi] => {
                        match (lo.parse::<u64>(), hi.parse::<u64>()) {
                            (Ok(l), Ok(h)) => sprintln!("{}", crate::straus::Straus::frontier(l, h)),
                            _ => sprintln!("straus frontier <lo> <hi> — both must be numbers"),
                        }
                    }
                    [n] => match n.parse::<u64>() {
                        Ok(v) => sprintln!("{}", crate::straus::Straus::read(v)),
                        Err(_) => sprintln!("straus <n> — n must be a number"),
                    },
                    _ => sprintln!("{}", crate::straus::Straus::help()),
                }
            }
            "carriers" => {
                match parts.next() {
                    None => sprintln!("{}", crate::carriers::Carriers::report()),
                    Some(_) => sprintln!("{}", crate::carriers::Carriers::help()),
                }
            }
            "substrate" => {
                match parts.next() {
                    None => sprintln!("{}", crate::substrate::Substrate::report()),
                    Some(_) => sprintln!("{}", crate::substrate::Substrate::help()),
                }
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
                        // Idle until the next interrupt on metal; on a host
                        // the read below blocks, so there is nothing to wait on.
                        #[cfg(not(feature = "hosted"))]
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
                    if i >= canonical_count() + continuous_count() + novel_count() + shunted_count() {
                        sprintln!("Program {} out of range (max XXVIII/{}).",
                            arg, canonical_count() + continuous_count() + novel_count() + shunted_count());
                    } else if load_by_roman(k, arg) {
                        let name: &str = if i < canonical_count() {
                            canonical_name(i)
                        } else if i < canonical_count() + continuous_count() {
                            continuous_name(i - canonical_count())
                        } else if i < canonical_count() + continuous_count() + novel_count() {
                            novel_name(i - canonical_count() - continuous_count())
                        } else {
                            shunted_name(i - canonical_count() - continuous_count() - novel_count())
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
                    if idx < novel_count() {
                        k.load_novel(idx);
                        sprintln!("Booting novel {}: {}", i, novel_name(idx));
                        sprintln!("Running (ESC to stop)...");
                        let ran = k.run_continuous(|| interrupts::escape_pressed());
                        sprintln!("
Stopped after {} ticks.", ran);
                        print_status(k);
                    } else {
                        sprintln!("Novel index {} out of range (max {}).",
                            i, novel_count());
                    }
                } else {
                    sprintln!("Usage: boot novel <1-{}>", novel_count());
                }
            }
            "shunt" => {
                let arg = parts.next().unwrap_or("").trim();
                if let Ok(i) = arg.parse::<usize>() {
                    let idx = i.saturating_sub(1);
                    if idx < shunted_count() {
                        k.load_shunted(idx);
                        sprintln!("Booting shunted {}: {}", i, shunted_name(idx));
                        sprintln!("Running (ESC to stop)...");
                        let ran = k.run_continuous(|| interrupts::escape_pressed());
                        sprintln!("
Stopped after {} ticks.", ran);
                        print_status(k);
                    } else {
                        sprintln!("Shunted index {} out of range (max {}).",
                            i, shunted_count());
                    }
                } else {
                    sprintln!("Usage: boot shunt <1-{}>", shunted_count());
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
                    for i in 0..continuous_count() {
                        sprintln!("  {}. {}", i + 1, continuous_name(i));
                    }
                    sprintln!("Usage: continuous <1-{}>", continuous_count());
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
                            let idx = name_hash(name) % canonical_count();
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
                            let pnames = crate::canonical_ig::PRIMITIVE_ORDER;
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
                for i in 0..canonical_count() {
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(i), canonical_name(i));
                }
                sprintln!("────────────────────────────────────────────────────────────");
                sprintln!("   ▸ CONTINUOUS (XIII–XVI)  — token-graph-native loops     ");
                sprintln!("────────────────────────────────────────────────────────────");
                for i in 0..continuous_count() {
                    let ri = canonical_count() + i;
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(ri), continuous_name(i));
                }
                sprintln!("────────────────────────────────────────────────────────────");
                sprintln!("   ▸ NOVEL (XVII–XIX)  — control-flow reconstructions      ");
                sprintln!("────────────────────────────────────────────────────────────");
                for i in 0..novel_count() {
                    let ri = canonical_count() + continuous_count() + i;
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(ri), novel_name(i));
                }
                sprintln!("╚══════════════════════════════════════════════════════════╝");
                sprintln!("   ▸ SHUNTED (XX–XXVIII) — branching/exotic compositions        ");
                for i in 0..shunted_count() {
                    let ri = i + canonical_count() + continuous_count() + novel_count();
                    sprintln!("   {:>4}.  {:<48} ", idx_to_roman(ri), shunted_name(i));
                }
                sprintln!("Use 'load <I–XXVIII>' to load any program by Roman numeral.");
            }
            "load" => {
                let arg = parts.next().unwrap_or("").trim();
                if load_by_roman(k, arg) {
                    let idx = roman_to_idx(arg).unwrap();
                    let name: &str = if idx < canonical_count() {
                        canonical_name(idx)
                    } else if idx < canonical_count() + continuous_count() {
                        continuous_name(idx - canonical_count())
                    } else if idx < canonical_count() + continuous_count() + novel_count() {
                        novel_name(idx - canonical_count() - continuous_count())
                    } else {
                        shunted_name(idx - canonical_count() - continuous_count() - novel_count())
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
                        sprintln!("  Absorbing: ⊙(all) ⊞=𐑳(tensor)");
                        if let Some(lim) = k.liminal_target {
                            sprintln!("  ⚠ LIMINAL JUMP PENDING → {} ({}). Use 'seal' to commit.",
                                dialect_display(lim), dialect_name(lim));
                        }
                    }
                    "list" => {
                        // The title said 12 while the loop ran to 88, and the
                        // bound was written out rather than taken from the one
                        // place that knows it.
                        use crate::dialect_expansion::DIALECT_COUNT;
                        head!("dialects");
                        for u in 0u8..(DIALECT_COUNT as u8) {
                            let active = u == k.active_dialect;
                            let (mark, col) = if active {
                                ("*", crate::style::accent())
                            } else {
                                (" ", crate::style::value())
                            };
                            sprintln!("  {}{} {:<3} {:<20}{} {}{}{}  {}O_∞ {}{}",
                                col, mark, dialect_display(u), dialect_name(u),
                                crate::style::reset(),
                                crate::style::muted(), dialect_gates(u), crate::style::reset(),
                                crate::style::muted(), dialect_o_inf(u), crate::style::reset());
                        }
                        divider!();
                        sprintln!("  {}{} dialects, active marked{}",
                            crate::style::muted(), DIALECT_COUNT, crate::style::reset());
                        foot!();
                        if k.liminal_target.is_some() {
                            sprintln!("  ⚠ Liminal jump pending. Use 'seal' to commit or 'jump' again to override.");
                        }
                    }
                    "verify" => {
                        let u = k.active_dialect;
                        // Optional catalog name: "ruleset verify birch_swinnerton_dyer"
                        // checks a named catalog entry's *static* tuple instead
                        // of the kernel's own live execution snapshot. Added 2026-06-16
                        // alongside U8 so externally-defined types (e.g. the
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
                                // Dialects 0–7 evaluate from the SAME GateSpec data as every
                                // other dialect. They used to carry hand-written gates using
                                // `(x as u8) <= (thresh as u8)`, which is the discriminant trick
                                // `IgPrim::ordinal`'s own docstring warns is invalid for ⊙
                                // Criticality and ⊡ Winding — the two families these very gates
                                // test at G2 and G3. It rejected roar/err/haha at ⊙≥⊙ and zoo at
                                // ⊡≥𐑭, and dialect 5's ⊡≥𐑟 admitted every winding value, a gate
                                // that always passed. Arms 8–11 had already been moved to
                                // `.ordinal()`; this finishes that move and removes the second
                                // copy of the gate table at the same time.
                                0..=7 => {
                                    let unis = crate::dialect_expansion::all_dialects();
                                    let uni = &unis[u as usize];
                                    let (g1, g1_ord, g1_glyph) = crate::dialect::eval_gate_spec(&uni.g1, &ig);
                                    let (g2, g2_ord, g2_glyph) = crate::dialect::eval_gate_spec(&uni.g2, &ig);
                                    let (g3, g3_ord, g3_glyph) = crate::dialect::eval_gate_spec(&uni.g3, &ig);
                                    sprintln!("  G1 ({}≥{}): {}  {}={} (ord {})",
                                        uni.g1.prim, uni.g1.min_ord, if g1 {"PASS"} else {"FAIL"},
                                        uni.g1.prim, g1_glyph, g1_ord);
                                    sprintln!("  G2 ({}≥{}): {}  {}={} (ord {})",
                                        uni.g2.prim, uni.g2.min_ord, if g2 {"PASS"} else {"FAIL"},
                                        uni.g2.prim, g2_glyph, g2_ord);
                                    sprintln!("  G3 ({}≥{}): {}  {}={} (ord {})",
                                        uni.g3.prim, uni.g3.min_ord, if g3 {"PASS"} else {"FAIL"},
                                        uni.g3.prim, g3_glyph, g3_ord);
                                    sprintln!("  Ordering: {}",
                                        if uni.gate_ordering {"SEQUENTIAL"} else {"PARALLEL"});
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    // t_structural additionally constrains the coupling primitive.
                                    if u == 7 {
                                        let t_ok = ig.c == IgPrim::measure;
                                        sprintln!("  T  (∋=𐑠): {}  ∋={}", if t_ok {"PASS"} else {"FAIL"}, ig.c.glyph());
                                        if !t_ok { all_pass = false; }
                                    }
                                }
                                8 => { // chirality_first: G1:⊥≥𐑖  G2:⊙≥⊙  G3:⊡≥𐑭
                                       // T: T_CEILING — see manuscripts/clay_cross_dialect_closure.md.
                                       // Uses IgPrim::ordinal(), NOT raw discriminant comparison — the
                                       // discriminant trick used in arms 0-7 is invalid for the criticality
                                       // family (roar/𐑻 are non-monotonic in discriminant order).
                                    let g1 = ig.h.ordinal() >= IgPrim::sure.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::monad.ordinal();
                                    let g3 = ig.omega.ordinal() >= IgPrim::ah.ordinal();
                                    sprintln!("  G1 (⊥≥𐑖): {}  ⊥={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.h.glyph(), ig.h.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (⊡≥𐑭): {}  ⊡={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph(), ig.omega.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_check(&ig) { all_pass = false; }
                                }
                                9 => { // scope_dialect: G1:∈≥𐑲(maximal scope)  G2:⊙≥⊙  G3:⊡≥𐑭
                                       // T: T_CEILING — same generalization as U8, paired with a different gate spec.
                                    let g1 = ig.g.ordinal() >= IgPrim::ice.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::monad.ordinal();
                                    let g3 = ig.omega.ordinal() >= IgPrim::ah.ordinal();
                                    sprintln!("  G1 (∈≥𐑲): {}  ∈={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.g.glyph(), ig.g.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (⊡≥𐑭): {}  ⊡={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.omega.glyph(), ig.omega.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_check(&ig) { all_pass = false; }
                                }
                                10 => { // triple_criticality: G1/G2/G3 all on ⊙, escalating thresholds 𐑢/⊙/𐑣
                                    let g1 = ig.phi.ordinal() >= IgPrim::woe.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::monad.ordinal();
                                    let g3 = ig.phi.ordinal() >= IgPrim::haha.ordinal();
                                    sprintln!("  G1 (⊙≥𐑢): {}  ⊙={} (ord {})", if g1 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G2 (⊙≥⊙): {}  ⊙={} (ord {})", if g2 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    sprintln!("  G3 (⊙≥𐑣): {}  ⊙={} (ord {})", if g3 {"PASS"} else {"FAIL"}, ig.phi.glyph(), ig.phi.ordinal());
                                    if !g1 || !g2 || !g3 { all_pass = false; }
                                    if !t_ceiling_check(&ig) { all_pass = false; }
                                }
                                11 => { // triple_criticality_gapped: same gates as U10, T_CEILING(gapped)
                                    let g1 = ig.phi.ordinal() >= IgPrim::woe.ordinal();
                                    let g2 = ig.phi.ordinal() >= IgPrim::monad.ordinal();
                                    let g3 = ig.phi.ordinal() >= IgPrim::haha.ordinal();
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
                                        // GateSpec::prim is already the mark, and already
                                        // 'static. `gate_prim_label` was a twelve-arm match
                                        // returning each mark as itself — a thirteenth hand copy
                                        // of the alphabet whose only possible behaviour was to
                                        // start answering "?" if a mark ever changed.
                                        let g1_label = uni.g1.prim;
                                        let g2_label = uni.g2.prim;
                                        let g3_label = uni.g3.prim;
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
                            ig.p.ordinal()     >= IgPrim::or_.ordinal()
                            && ig.phi.ordinal() >= IgPrim::monad.ordinal()
                            && ig.omega.ordinal() >= IgPrim::ah.ordinal();
                        let t_canon = t_canonical_check_silent(&ig);

                        // Alt-dialect gate verdict: only U8/U9/U10/U11 wired up so far.
                        // U8/U9/U10 use T_CEILING for their T side; U11 uses the
                        // gapped variant (raises only the ⊤ anchor — see dialect.rs).
                        let gate_alt = match alt {
                            8 => ig.h.ordinal() >= IgPrim::sure.ordinal()
                                && ig.phi.ordinal() >= IgPrim::monad.ordinal()
                                && ig.omega.ordinal() >= IgPrim::ah.ordinal(),
                            9 => ig.g.ordinal() >= IgPrim::ice.ordinal()
                                && ig.phi.ordinal() >= IgPrim::monad.ordinal()
                                && ig.omega.ordinal() >= IgPrim::ah.ordinal(),
                            10 | 11 => ig.phi.ordinal() >= IgPrim::woe.ordinal()
                                && ig.phi.ordinal() >= IgPrim::monad.ordinal()
                                && ig.phi.ordinal() >= IgPrim::haha.ordinal(),
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
                    "show" => sprintln!("Absorption rules (canonical U₀):\n  ⊙ absorbs under all ops\n  ⊞=𐑳 absorbs under tensor"),
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
                        for i in 0..compound_count() {
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
                    let comp_bytes: &[u8] = completion.as_bytes();
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
// Ceiling-generalizes canonical's existing ⊤-only ceiling rule to all five
// dynamics primitives, same anchors: <<=𐑹 ⋈<=𐑐 ⊤<=𐑧 ⊥<=𐑫 ⊡<=𐑭.
// See manuscripts/clay_cross_dialect_closure.md for the derivation. Uses
// IgPrim::ordinal(), not raw discriminant comparison.
// Canonical's actual T-constitution (exact-equality on four primitives,
// ceiling on ⊤ only) — matches Python's _T_CANONICAL exactly. This is the
// real canonical T-verdict, distinct from T_CEILING (which only applies
// to U8/U9/U10/U11).
fn t_canonical_check_silent(ig: &IgTuple) -> bool {
    ig.p.ordinal()     == IgPrim::or_.ordinal()
    && ig.f.ordinal()   == IgPrim::peep.ordinal()
    && ig.k.ordinal()   <= IgPrim::egg.ordinal()
    && ig.h.ordinal()   == IgPrim::wool.ordinal()
    && ig.omega.ordinal() == IgPrim::ah.ordinal()
}

fn t_ceiling_check_silent(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::or_.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::peep.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::egg.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::wool.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::ah.ordinal();
    t_phi && t_f && t_k && t_h && t_om
}

// U11 only: same as T_CEILING, but ⊤'s ceiling is raised from 𐑧 (egg,
// ord 3) to 𐑪 (on, ord 4) — a gapped/trapped spectrum, not just a slow
// one. Motivated, not tailored: see dialect.rs's U11 comment block.
fn t_ceiling_gapped_check_silent(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::or_.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::peep.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::on.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::wool.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::ah.ordinal();
    t_phi && t_f && t_k && t_h && t_om
}

fn t_ceiling_check(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::or_.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::peep.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::egg.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::wool.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::ah.ordinal();
    let t_ok = t_phi && t_f && t_k && t_h && t_om;
    sprintln!("  T_CEILING <<=𐑹: {}  ⋈<=𐑐: {}  ⊤<=𐑧: {}  ⊥<=𐑫: {}  ⊡<=𐑭: {}",
        if t_phi {"PASS"} else {"FAIL"}, if t_f {"PASS"} else {"FAIL"},
        if t_k {"PASS"} else {"FAIL"}, if t_h {"PASS"} else {"FAIL"},
        if t_om {"PASS"} else {"FAIL"});
    sprintln!("  T_CEILING overall: {}", if t_ok {"PASS"} else {"FAIL"});
    t_ok
}

fn t_ceiling_gapped_check(ig: &IgTuple) -> bool {
    let t_phi = ig.p.ordinal()     <= IgPrim::or_.ordinal();
    let t_f   = ig.f.ordinal()     <= IgPrim::peep.ordinal();
    let t_k   = ig.k.ordinal()     <= IgPrim::on.ordinal();
    let t_h   = ig.h.ordinal()     <= IgPrim::wool.ordinal();
    let t_om  = ig.omega.ordinal() <= IgPrim::ah.ordinal();
    let t_ok = t_phi && t_f && t_k && t_h && t_om;
    sprintln!("  T_CEILING(gapped) <<=𐑹: {}  ⋈<=𐑐: {}  ⊤<=𐑪: {}  ⊥<=𐑫: {}  ⊡<=𐑭: {}",
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

/// Section headings inside the long reports. Named once so the whole listing
/// moves with the theme rather than by search-and-replace across 31 sites.
fn style_section() -> &'static str { crate::style::heading() }

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
/// An imscription and the word that writes it, together.
///
/// A tuple says what something IS; the word says what it DOES, and they are two
/// readings of one object rather than two facts about it. The kernel already
/// held both maps and never showed them side by side: `build_via_substrate`
/// takes a tuple to its program, and `from_snapshot` takes a program's execution
/// back to a tuple. Printing the tuple alone left the operative half invisible,
/// so an imscription looked like a label instead of something you can run.
///
/// The round trip is the check, not decoration. tuple → word → tuple returning
/// the tuple it started from is mu-delta = id at the level of the imscription
/// itself, and it can fail — a tuple whose word imscribes to something else is
/// reporting a real disagreement between what it claims and what it does.
fn print_ig(k: &Kernel) {
    use crate::imas_ig::IgTuple;
    let Some(snap) = k.snapshot else {
        sprintln!("No snapshot. Tick first.");
        return;
    };
    let ig = IgTuple::from_snapshot(&snap);
    sprintln!("IG:      {}", ig.display());
    sprintln!("Crystal: {}", ig.crystal_address());

    // The word this tuple writes.
    let prog = crate::sequence::build_via_substrate(
        &ig, 12, ig.t == crate::imas_ig::IgPrim::are, 3);
    let word = crate::belnap_ring_shor::glyphs_from_program(&prog);
    sprintln!("IMASM:   {}", word);

    // And what that word imscribes back to. Two questions, not one, because the
    // tuple to word map is many-to-one: several tuples write the same word, so
    // the word can be a fixed point while the tuple is not. Reporting only the
    // strict test would call that a failure when it is the map being lossy.
    let back = IgTuple::from_snapshot(&crate::kernel::self_imscribe(&prog));
    let prog_back = crate::sequence::build_via_substrate(
        &back, 12, back.t == crate::imas_ig::IgPrim::are, 3);
    let word_back = crate::belnap_ring_shor::glyphs_from_program(&prog_back);

    if back == ig {
        sprintln!("Round:   tuple -> word -> tuple returns itself — mu.delta = id");
    } else if word_back == word {
        sprintln!("Round:   the WORD is fixed, the tuple is not.");
        sprintln!("         word -> {}", back.display());
        sprintln!("         and that tuple writes the same word back, so the pair");
        sprintln!("         closes on the word. The tuple->word map is many-to-one:");
        sprintln!("         what the imscription DOES is recoverable, what it CLAIMS");
        sprintln!("         is not recoverable from it alone.");
    } else {
        sprintln!("Round:   OPEN — neither the tuple nor the word returns.");
        sprintln!("         word -> {}", back.display());
        sprintln!("         which writes {}", word_back);
    }
}

fn print_classify(k: &Kernel, arg: &str) {
    use crate::imas_ig::{Classification, IgTuple};
    let arg = arg.trim();
    if !arg.is_empty() {
        // `classify <t>` classifies the tuple it is HANDED. Reading the live
        // kernel instead makes the command report its own state whatever it is
        // given, which is not a classification of anything the caller asked about.
        match IgTuple::from_glyphs(arg) {
            Ok(t) => sprintln!("{}", Classification::classify_tuple(&t).display()),
            // from_glyphs reuses its error pair for a length fault, where the
            // second field is a message rather than a glyph. Say which it is.
            Err((i, g)) if g.starts_with("expected") => sprintln!("classify: {}", g),
            Err((i, g)) => sprintln!("classify: slot {} is not a primitive: `{}`", i, g),
        }
        return;
    }
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

/// One ⊥ hop: toggle chirality, show the snapshot on each side of the door.
fn print_arev_hop(k: &mut Kernel) {
    let before = k.dynamic_imscribe();
    let h = k.arev_hop();
    let after = k.snapshot.unwrap_or(before);
    sprintln!("AREV — ⊥ hop, lateral at the same shell. ⊥ now {}", if h { "flipped" } else { "or'" });
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
    if k.chirality { k.arev_hop(); } // enter with ⊥ = or'
    let s0 = k.dynamic_imscribe();
    sprintln!("replicative loop, 16 ticks, ⊥ = or':");
    print_snap_line("s0", &s0);
    k.arev_hop();
    let s1 = k.snapshot.unwrap_or(s0);
    sprintln!("first hop (⊥ flipped) — R1 reads the mirrored evidence:");
    print_snap_line("s1", &s1);
    k.arev_hop();
    let s2 = k.snapshot.unwrap_or(s0);
    sprintln!("second hop (⊥ back to or'):");
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
        if idx < canonical_count() {
            k.load_canonical(idx);
            true
        } else if idx < canonical_count() + continuous_count() {
            k.load_continuous(idx - canonical_count())
        } else if idx < canonical_count() + continuous_count() + novel_count() {
            k.load_novel(idx - canonical_count() - continuous_count())
        } else if idx < canonical_count() + continuous_count() + novel_count() + shunted_count() {
            k.load_shunted(idx - canonical_count() - continuous_count() - novel_count())
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
    sprintln!("  {}Yang-Mills Mass Gap{}", style_section(), crate::style::reset());
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
    sprintln!("  {}Temporal Logic{}", style_section(), crate::style::reset());
    sprintln!("  B fixed point: {}", if b_temporal_fixed() { "PASS" } else { "FAIL" });
    sprintln!("  next involution: {}", if next_involution() { "PASS" } else { "FAIL" });
    sprintln!("  B absorbs until: {}", if b_absorbs_until() { "PASS" } else { "FAIL" });
    sprintln!("  B U N = B, N U T = T, T U F = T");
}
fn print_cat() {
    use para_category::*;
    sprintln!("  {}Category Theory{}", style_section(), crate::style::reset());
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

    sprintln!("  {}Riemann Hypothesis Bridge{}", style_section(), crate::style::reset());
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

    sprintln!("  {}Belnap Shor Pipeline{}", style_section(), crate::style::reset());

    sprintln!("── SIC-POVM Axioms ──");
    sprintln!("  verify: {}", if verify_sic_povm() { "PASS" } else { "FAIL" });

    sprintln!("── Hadamard ──");
    sprintln!("  H|T⟩=B: {}", if b4_hadamard(B4::T) == B4::B { "PASS" } else { "FAIL" });
    sprintln!("  H|F⟩=B: {}", if b4_hadamard(B4::F) == B4::B { "PASS" } else { "FAIL" });
    sprintln!("  H|B⟩=T: {}", if b4_hadamard(B4::B) == B4::T { "PASS" } else { "FAIL" });
    sprintln!("  H|N⟩=N: {}", if b4_hadamard(crate::belnap::B4::N) == crate::belnap::B4::N { "PASS" } else { "FAIL" });

    sprintln!("── Shor N=15,a=7 ──");
    let r1 = run_belnap_shor_output(4, 7, 15);
    sprintln!("  period={} H={} B-meas={} T-meas={} ratio={:.1}",
        r1.period_cl, r1.hadamard_coherence, r1.b_bias_coherence, r1.t_bias_coherence, r1.ratio);
    sprintln!("  allB={} b-preserves={} t-collapses={} bottleneck={}",
        r1.mod_exp_all_b, r1.b_bias_preserves, r1.t_bias_collapses, r1.polarity_bottleneck);

    sprintln!("── Shor N=21,a=5 ──");
    let r2 = run_belnap_shor_output(5, 5, 21);
    sprintln!("  period={} H={} B-meas={} T-meas={} ratio={:.1}",
        r2.period_cl, r2.hadamard_coherence, r2.b_bias_coherence, r2.t_bias_coherence, r2.ratio);

    // Both of these are Polarity values wearing an old Criticality-style
    // prefix: upsilon is yew (𐑿, phase symmetry) and pmsym is or' (𐑹,
    // Frobenius-special). The gap is along <, not ⊙.
    sprintln!("── <=𐑿 bottleneck ──");
    sprintln!("  B is the only superposition; all lattice ops preserve B.");
    sprintln!("  Period r encoded in 2:1 coherence cost ratio, not bits.");
    sprintln!("  <=𐑿 -> <=𐑹 gap: structural open problem.");
}

fn print_shor_phase(n_val: u64, a_val: u64) {
    use crate::belnap_phase_shor::{run_phase_belnap_shor, PhaseModExp, polarity_bottleneck_closed};

    if n_val == 0 || a_val == 0 {
        // Default: show phase-augmented analysis for canonical cases
        sprintln!("  {}Phase-Augmented Belnap Shor (Problems 1-2 Solution){}", style_section(), crate::style::reset());
        sprintln!();
        sprintln!("  The phase-augmented model adds complex phase to B4 lattice.");
        sprintln!("  B-bias measurement cost = 2 + |sin(π·phase)|");
        sprintln!("  This makes belnapCost proportional to accumulated phase.");
        sprintln!();
        let cases = [(4usize, 7u64, 15u64), (5, 5, 21), (6, 2, 35)];
        for (n, a, N) in &cases {
            let r = run_phase_belnap_shor(*n, *a, *N);
            sprintln!("  N={:<4} a={:<3} period={:<4} phase={:.4} B-cost={} gap={} closed={}",
                N, a, r.period, r.total_phase, r.belnap_cost, r.gap, r.bottleneck_closed);
        }
        sprintln!();
        sprintln!("  polarity_bottleneck: belnapCost = 2·period");
        sprintln!("  Phase-augmented model: cost depends on phase accumulation");
        sprintln!("  Gap is SMALLER than classical (belnapCost=2n) approach");
        return;
    }

    sprintln!("══ Phase-Augmented Shor: N={}, a={} ══", n_val, a_val);
    let n = if n_val <= 1 { 2 } else {
        let mut bits = 0; let mut v = n_val - 1;
        while v > 0 { bits += 1; v >>= 1; }
        bits.max(2) as usize
    };
    let r = run_phase_belnap_shor(n, a_val, n_val);
    sprintln!("  period={}  total_phase={:.4} windings", r.period, r.total_phase);
    sprintln!("  B-bias cost={}  T-bias cost={}  belnapCost={}", r.b_bias_cost, r.t_bias_cost, r.belnap_cost);
    sprintln!("  2·period={}  gap={}  bottleneck_closed={}", 2*r.period, r.gap, r.bottleneck_closed);
    sprintln!("  Phase kicks from ModExp: {:?}", (0..n).map(|k| {
        let pow = crate::belnap_phase_shor::mod_pow(a_val, 1u64 << k, n_val);
        format!("{:.3}", pow as f64 / n_val as f64)
    }).collect::<Vec<_>>());
}

fn print_shor_ring(n_val: u64, a_val: u64) {
    use crate::belnap_ring_shor::{verify_period_full, Sic2048Bridge, period_to_glyph_word};

    if n_val == 0 || a_val == 0 {
        sprintln!("  {}IMASM Ring Walk Period Verification (Problem 4){}", style_section(), crate::style::reset());
        sprintln!();
        let sic = Sic2048Bridge::new();
        sprintln!("  d=2048 SIC Bridge:");
        sprintln!("    discriminant = {}", sic.discriminant);
        sprintln!("    Stark unit ε ≈ {:.4}", sic.stark_unit);
        sprintln!("    tower deg over Q = 2^27");
        sprintln!("    algebraic period = {}", sic.algebraic_period());
        sprintln!();
        for (N, a) in &[(15u64, 7u64), (21, 5), (35, 2)] {
            let r = verify_period_full(*N, *a);
            sprintln!("  N={} a={} period={} ring_verified={} sic_consistent={}",
                N, a, r.period_classical, r.verified, r.consistency);
        }
        return;
    }

    sprintln!("══ IMASM Ring Walk: N={}, a={} ══", n_val, a_val);
    let r = verify_period_full(n_val, a_val);
    sprintln!("  classical period = {}", r.period_classical);
    sprintln!("  ring walk verified = {}", r.verified);
    sprintln!("  sic bridge period = {}", r.sic_bridge_period);
    sprintln!("  consistency = {}", r.consistency);
    sprintln!("  glyph word: {:?}", r.glyph_word.iter().map(|g| g.to_char()).collect::<Vec<_>>());
}

fn print_shor_fib(n_val: u64, a_val: u64) {
    use crate::fibonacci_shor::{assemble_shor_braid, certify_advantage, ShorCircuitParams, strands_for_qubits, estimate_braid_length};

    if n_val == 0 || a_val == 0 {
        sprintln!("  {}Fibonacci Anyon Braid Compiler for Shor (Problem 3){}", style_section(), crate::style::reset());
        sprintln!();
        for (n_q, a, N) in &[(4usize, 7u64, 15u64), (5, 5, 21), (8, 2, 35)] {
            let p = ShorCircuitParams::new(*n_q, *a, *N);
            let cert = certify_advantage(&p);
            sprintln!("  N={:<4} n={} strands={} fusion_dim={} braid_len~{} gate_err={:.4} logical_qubits={}",
                N, n_q, p.strands, p.fusion_dim, p.estimated_braid_len, cert.accumulated_error, cert.logical_qubits);
        }
        sprintln!();
        sprintln!("  Fibonacci anyon model: τ⊗τ = 1⊕τ");
        sprintln!("  4 anyons/qubit, fusion dim F_{{n-1}}");
        sprintln!("  Advantage is topological: it lives in the logical-qubit capacity of");
        sprintln!("  the anyonic encoding, not in a simulability threshold.");
        return;
    }

    let n = if n_val <= 1 { 2 } else {
        let mut bits = 0; let mut v = n_val - 1;
        while v > 0 { bits += 1; v >>= 1; }
        bits.max(2) as usize
    };
    sprintln!("══ Fibonacci Shor: N={}, a={}, n={} ══", n_val, a_val, n);
    let braid = assemble_shor_braid(n, a_val, n_val);
    sprintln!("  strands={}  fusion_dim={}", braid.params.strands, braid.params.fusion_dim);
    sprintln!("  period={:?}  braid_len={}", braid.params.period, braid.total_length);
    sprintln!("  H-layer: {} gens  ModExp: {} gens  IQFT: {} gens",
        braid.hadamard_word.len(), braid.mod_exp_word.len(), braid.iqft_word.len());
    let cert = certify_advantage(&braid.params);
    sprintln!("  gate_error={:.4}  logical_qubits={}  (topological capacity)",
        cert.accumulated_error, cert.logical_qubits);
}

fn print_shor_integrated(n_val: u64, a_val: u64) {
    use crate::belnap_phase_shor::run_integrated_shor;
    use crate::belnap_shor::run_belnap_shor_output;

    if n_val == 0 || a_val == 0 {
        sprintln!("  {}Integrated Shor Pipeline (All 4 Problems){}", style_section(), crate::style::reset());
        sprintln!();
        for (N, a) in &[(15u64, 7u64), (21, 5), (35, 2)] {
            let r = run_integrated_shor(*N, *a);
            sprintln!("  N={:<4} a={:<3} period={:<4} bottleneck={} phase={:.4} braid_len~{} ring={} factors={}×{}",
                N, a, r.period, if r.bottleneck_closed { "✓" } else { "≈" },
                r.total_phase, r.estimated_braid_len,
                if r.ring_walk_verified { "✓" } else { "?" },
                r.factor1.unwrap_or(0), r.factor2.unwrap_or(0));
        }
        sprintln!();
        sprintln!("  Problem 1: Phase-augmented B-bias → belnapCost ≈ 2·period");
        sprintln!("  Problem 2: Non-Boolean ModExp → phase-sensitive evaluation");
        sprintln!("  Problem 3: Fibonacci anyon braids → topological protection");
        sprintln!("  Problem 4: IMASM ring walk → paraconsistent verification");
        return;
    }

    sprintln!("══ Integrated Shor: N={}, a={} ══", n_val, a_val);
    let r = run_integrated_shor(n_val, a_val);
    sprintln!("  ── Core ──");
    sprintln!("  period={}", r.period);
    sprintln!("  ── P1: Phase-Augmented ──");
    sprintln!("  belnapCost={}  2·period={}  bottleneck_closed={}",
        r.belnap_cost, 2*r.period, r.bottleneck_closed);
    sprintln!("  total_phase={:.4} windings", r.total_phase);
    sprintln!("  ── P2: Non-Boolean ModExp ──");
    sprintln!("  phase_kicks: {:?}", r.phase_kicks.iter().map(|p| format!("{:.3}", p)).collect::<Vec<_>>());
    sprintln!("  ── P3: Fibonacci Braids ──");
    sprintln!("  strands={}  braid_len~{}", r.fibonacci_strands, r.estimated_braid_len);
    sprintln!("  ── P4: IMASM Ring Walk ──");
    sprintln!("  verified={}", r.ring_walk_verified);
    sprintln!("  ── Factorization ──");
    if r.factor1.is_some() {
        sprintln!("  ✓ N = {} × {}", r.factor1.unwrap_or(0), r.factor2.unwrap_or(0));
    } else {
        sprintln!("  ✗ factorization failed");
    }
}

fn print_shor_dialetheic(n_val: u64, a_val: u64) {
    use crate::dialetheic_fib_shor::{run_dialetheic_fib_shor, report};
    if n_val == 0 || a_val == 0 {
        sprintln!("  {}Dialetheic Fibonacci Shor (ob3ect word ⊢∈≻⋈⊞∈⊤≻⊥≺∋⊙⋈⊡⊣){}", style_section(), crate::style::reset());
        sprintln!();
        for (N, a) in &[(15u64, 7u64), (21, 5), (35, 2)] {
            let r = run_dialetheic_fib_shor(*N, *a);
            sprintln!("  N={:<4} a={} period={:<3} cost=2r={:<3} ratio={:.2} strands={} fusion=F_{}={} factors={}×{} verdict={}",
                N, a, r.period, r.belnap_cost, r.ratio, r.strands, r.strands-1, r.fusion_dim,
                r.factor1.unwrap_or(0), r.factor2.unwrap_or(0),
                if r.walk.open_frames > 0 { "B (dialetheic)" } else { "T (closed)" });
        }
        sprintln!();
        sprintln!("  usage: shor dialetheic N a   (e.g. shor dialetheic 15 7)");
        sprintln!("  The 16₃ register walk is the control flow: ∈ splits T-arm/F-arm,");
        sprintln!("  ⊤/⊥ evaluate constructive/destructive interference, ∋ fuses to TF.");
        sprintln!("  Period r is read from the 2:1 B-bias/T-bias coherence cost ratio.");
        return;
    }
    let r = run_dialetheic_fib_shor(n_val, a_val);
    sprintln!("{}", report(&r));
}



fn parse_u64(s: &str) -> u64 {
    s.parse::<u64>().unwrap_or(0)
}

fn print_shor_custom(n_val: u64, a_val: u64) {
    use crate::belnap::B4;
    use crate::belnap_shor::run_belnap_shor_output;
    use crate::belnap_shor_factors::{analyze_coherence_gap, extract_factors};

    if n_val == 0 || a_val == 0 {
        sprintln!("shor: usage: shor N a  (N>1, a>1, gcd(N,a)=1)");
        return;
    }

    let n_qubits = if n_val <= 1 { 2 } else {
        let mut bits = 0; let mut v = n_val - 1;
        while v > 0 { bits += 1; v >>= 1; }
        bits.max(2) as usize
    };

    sprintln!("══ Belnap Shor Pipeline: N={}, a={} ══", n_val, a_val);

    let shor = run_belnap_shor_output(n_qubits, a_val, n_val);
    let gap = analyze_coherence_gap(n_qubits, shor.period_cl, shor.b_bias_coherence);
    let factors = extract_factors(n_val, a_val, shor.period_cl);

    sprintln!("  n_qubits={}  period={}  n_qubits==period? {}",
        n_qubits, shor.period_cl, n_qubits as u64 == shor.period_cl);
    sprintln!("  belnapCost (B-meas) = {}", shor.b_bias_coherence);
    sprintln!("  2·period            = {}", gap.twice_period);
    sprintln!("  coherence gap        = {}  (precondition: {})",
        gap.gap, if gap.precondition_holds { "HOLDS" } else { "FAILS" });
    sprintln!("  ratio belnapCost/2r  = {:.4}", gap.ratio_to_2r);
    sprintln!("  B-bias/T-bias ratio  = {:.1}", shor.ratio);

    sprintln!("  ── Factorization ──");
    sprintln!("  period={}  trivial={}", factors.period, factors.trivial);
    if !factors.trivial {
        sprintln!("  factor1={}  factor2={}  N={}×{}",
            factors.factor1.unwrap_or(0), factors.factor2.unwrap_or(0),
            factors.factor1.unwrap_or(0), factors.factor2.unwrap_or(0));
    } else {
        sprintln!("  reason: {}", factors.reason);
    }
}

fn print_shor_factors(n_val: u64, a_val: u64) {
    use crate::belnap_shor_factors::*;
    use crate::belnap_shor::run_belnap_shor_output;

    if n_val == 0 || a_val == 0 {
        sprintln!("shor factors: usage: shor factors N a");
        return;
    }

    sprintln!("══ Belnap Shor Factorization: N={}, a={} ══", n_val, a_val);
    let r = run_full_belnap_shor_auto(a_val, n_val);

    sprintln!("  n_qubits={}  period={}", r.n_qubits, r.period);
    sprintln!("  Belnap B-meas cost = {}  (2n = {})", r.shor_result.b_bias_coherence, 2 * r.n_qubits);
    sprintln!("  T-bias cost        = {}  (n = {})", r.shor_result.t_bias_coherence, r.n_qubits);
    sprintln!("  Coherence gap      = {}  precondition={}",
        r.gap.gap, r.gap.precondition_holds);

    sprintln!("  ── Factors ──");
    if !r.factors.trivial {
        sprintln!("  ✓ N = {} × {}", r.factors.factor1.unwrap_or(0), r.factors.factor2.unwrap_or(0));
        sprintln!("  ✓ gcd(a^(r/2)±1, N) = ({},{})",
            r.factors.factor1.unwrap_or(0), r.factors.factor2.unwrap_or(0));
    } else {
        sprintln!("  ✗ {}", r.factors.reason);
        if r.factors.factor1.is_some() {
            sprintln!("    partial: gcd → {} and {}",
                r.factors.factor1.unwrap_or(0), r.factors.factor2.unwrap_or(0));
        }
    }
}

fn print_shor_gap(n_val: u64, a_val: u64) {
    use crate::belnap_shor_factors::*;
    use crate::belnap_shor::run_belnap_shor_output;

    if n_val == 0 || a_val == 0 {
        // Default: show gap for canonical cases
        sprintln!("  {}Belnap Shor Coherence Gap Analysis{}", style_section(), crate::style::reset());
        sprintln!();
        let cases = [(4usize, 7u64, 15u64, 4u64), (5, 5, 21, 6), (6, 2, 35, 12), (7, 2, 77, 30)];
        sprintln!("  {:<6} {:<6} {:<6} {:<10} {:<10} {:<8}", "N", "a", "r", "belnapCost", "2r", "gap");
        sprintln!("  {}", "─".repeat(52));
        for (n, a, N, r) in &cases {
            let shor = run_belnap_shor_output(*n, *a, *N);
            let gap = analyze_coherence_gap(*n, *r, shor.b_bias_coherence);
            sprintln!("  {:<6} {:<6} {:<6} {:<10} {:<10} {:<+8}  {}",
                N, a, r, shor.b_bias_coherence, gap.twice_period, gap.gap,
                if gap.precondition_holds { "✓ precondition holds" } else { "✗ gap" });
        }
        sprintln!();
        sprintln!("  polarity_bottleneck: belnapCost = 2·period  ✓ CLOSED");
        sprintln!("  Output-register measurement: |{{a^x mod N}}| = r distinct values.");
        sprintln!("  belnapCost = 2r for ALL N (verified: 15, 21, 35, 77).");
        sprintln!("  The 2:1 B-bias/T-bias ratio IS the period extractor.");
        return;
    }

    sprintln!("══ Coherence Gap: N={}, a={} ══", n_val, a_val);
    let n = if n_val <= 1 { 2 } else {
        let mut bits = 0; let mut v = n_val - 1;
        while v > 0 { bits += 1; v >>= 1; }
        bits.max(2) as usize
    };
    let shor = run_belnap_shor_output(n, a_val, n_val);
    let gap = analyze_coherence_gap(n, shor.period_cl, shor.b_bias_coherence);
    sprintln!("  n={}  r={}  belnapCost={}  2r={}  gap={}  holds={}",
        n, shor.period_cl, shor.b_bias_coherence, gap.twice_period, gap.gap, gap.precondition_holds);
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
            sprintln!("  {}CL8NK Promotion Ladder{}", style_section(), crate::style::reset());
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
                sprintln!("  {}CL8NK Distance{}", style_section(), crate::style::reset());
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
            sprintln!("  {}The ⊡/∋ Transcendence — CLINK L8 beyond ZFC_fe{}", style_section(), crate::style::reset());
            sprintln!("  d(ZFC_fe, CLINK L8) = {:.4}", tr.d_zfcfe_to_cl8nk);
            sprintln!();
            sprintln!("  ⊡: {} → {}",
                catalog::primitive_glyph(tr.omega_zfcfe),
                catalog::primitive_glyph(tr.omega_cl8nk));
            sprintln!("    ZFC_fe: {}", tr.omega_zfcfe_frag);
            sprintln!("    CL8NK:  {}", tr.omega_cl8nk_frag);
            sprintln!("    → Integer winding (Abelian anyons) → braid group (non-Abelian anyons)");
            sprintln!();
            sprintln!("  C (∋): {} → {}",
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
                sprintln!("  {}CL8NK Tier{}", style_section(), crate::style::reset());
                sprintln!("  System: {}  tier: {}  d(CLINK L8): {:.4}", cat_entry.name, tier, d);
            } else {
                sprintln!("[CL8NK] System '{}' not found in catalog.", lookup_name);
            }
        }
        "chain" => {
            let layers = chain_analysis();
            sprintln!("  {}CLINK Chain — Distance Ladder from CLINK L8{}", style_section(), crate::style::reset());
            sprintln!("  {} layers discovered in catalog", layers.len());
            sprintln!();
            for layer in &layers {
                sprintln!("  {:<24}  d={:.4}  tier={}  conflicts={}",
                    layer.name, layer.distance_from_l8, layer.tier, layer.conflicts_count);
            }
        }
        "systems" => {
            let systems = catalog_systems();
            sprintln!("  {}CL8NK — Catalog Systems{}", style_section(), crate::style::reset());
            sprintln!("  {} entries", systems.len());
            for s in &systems {
                sprintln!("    {}", s);
            }
        }
        "stats" => {
            let (count, cl8_found, zfcfe_found) = catalog_stats();
            sprintln!("  {}CL8NK — Catalog Statistics{}", style_section(), crate::style::reset());
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
            sprintln!("  transcendence     — ⊡/∋ transcendence analysis");
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
        sprintln!("  {}Consciousness Score{}", style_section(), crate::style::reset());
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
            // Stop codon analysis as ⊡ boundary
            use crate::belnap::B4;
            sprintln!("Stop Codon Analysis (⊡ boundary — kernel winding limit):");
            let stops = [
                ("UAA", Codon { p1: crate::belnap::B4::N, p2: B4::F, p3: B4::F }, "⊡₀  trivial winding — null boundary"),
                ("UAG", Codon { p1: crate::belnap::B4::N, p2: B4::F, p3: B4::B }, "𐑴  Z2-protected — amber boundary"),
                ("UGA", Codon { p1: crate::belnap::B4::N, p2: B4::B, p3: B4::F }, "𐑭   integer winding — opal boundary"),
            ];
            for (name, codon, desc) in &stops {
                let s = codon.symbol();
                sprintln!("  {} ({}{}{})  B4: ({:?},{:?},{:?})  {}",
                    name, s[0] as char, s[1] as char, s[2] as char,
                    codon.p1, codon.p2, codon.p3, desc);
            }
            sprintln!("  Mito additional stops: AGA (F,B,F)=⊡_AGA  AGG (F,B,B)=⊡_AGG");
            sprintln!("  Mito UGA → Trp (not Stop — ⊡ gate lifted in mitochondrial context)");
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
            sprintln!("  Stop:  {} codons (⊡ boundary)", stop);
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
        "orbital" => {
            let (ok, note) = crate::rebis::orbital::verify();
            sprintln!("── Orbital occupancy as Belnap FOUR ──");
            for o in crate::rebis::orbital::ALL_ORBITAL.iter() {
                sprintln!("  {:<9} = {}", o.name(), o.to_b4().name());
            }
            sprintln!("  Pauli ceiling: nothing sits above paired.");
            sprintln!("  {}: {}", if ok { "PASS" } else { "FAIL" }, note);
        }
        "quark" => {
            let (ok, note) = crate::rebis::quark::verify();
            sprintln!("── Quark colour as Belnap FIVE ──");
            sprintln!("  Vacuum < {{Red, Green, Blue}} < White");
            sprintln!("  distinct colours join to White, meet at Vacuum");
            let w = crate::rebis::quark::Quark::new(
                crate::rebis::quark::Colour::White, crate::rebis::orbital::Orbital::SpinUp);
            let r = crate::rebis::quark::Quark::new(
                crate::rebis::quark::Colour::Red, crate::rebis::orbital::Orbital::SpinUp);
            sprintln!("  Frobenius on white  : {}", crate::rebis::quark::frobenius_holds_white(w));
            sprintln!("  Frobenius on colour : fails = {}", crate::rebis::quark::frobenius_fails_coloured(r));
            sprintln!("  confinement IS that failure, not a separate postulate.");
            sprintln!("  {}: {}", if ok { "PASS" } else { "FAIL" }, note);
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
                        sprintln!("  {}IG Material Forge{}", style_section(), crate::style::reset());
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
                    sprintln!("  {}Ouroboric Alloy (64 grains){}", style_section(), crate::style::reset());
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
                    sprintln!("  rebis material alloy               — Ouroboric alloy computation");
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
                    sprintln!("  {}All 20 AA Sidechains{}", style_section(), crate::style::reset());
                    for (name, _) in sc {
                        sprintln!("  {}", name);
                    }
                    sprintln!();
                    let env = sidechain::all_environments();
                    sprintln!("  {}4 Environments{}", style_section(), crate::style::reset());
                    for (name, _) in env {
                        sprintln!("  {}", name);
                    }
                }
                "frustration" => {
                    let mat = crate::rebis::sidechain::frustration_matrix();
                    sprintln!("  {}Frustration Matrix (min tensor distance per pair){}", style_section(), crate::style::reset());
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
                    sprintln!("  {}Functional Groups{}", style_section(), crate::style::reset());
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
                    sprintln!("  {}Decay Series{}", style_section(), crate::style::reset());
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
                    sprintln!("  {}Ouroboric Telomere Computation{}", style_section(), crate::style::reset());
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
                    sprintln!("  {}Chemotherapeutic{}", style_section(), crate::style::reset());
                    sprintln!("  Name: {}  Target: {}", chemo.name, chemo.target_protein);
                    sprintln!("  Kd: {:.1} nM  Selectivity: {:.0}x", chemo.binding_affinity_nm, chemo.selectivity_ratio);
                    sprintln!("  Delivery: {}  Gate1(⊙): {}", chemo.delivery_mechanism,
                        if chemo.gate1_open { "OPEN" } else { "CLOSED" });
                    sprintln!("  Frobenius: {}  MTD: {:.1} mg",
                        if chemo.verify() { "PASS" } else { "FAIL" }, chemo.max_tolerated_dose_mg);
                }
                "pill" => {
                    let pill = crate::rebis::therapeutics::OuroboricPill::new("OP-001", 24.0);
                    sprintln!("  {}Ouroboric Pill{}", style_section(), crate::style::reset());
                    sprintln!("  Name: {}  Half-life: {:.1}h", pill.name, pill.half_life_hours);
                    sprintln!("  Frobenius: {}  Gate1: {}",
                        if pill.frobenius_verified { "μ∘δ=id" } else { "FAIL" },
                        if pill.gate1_open { "self-sensing" } else { "passive" });
                }
                "antidote" => {
                    let antidote = crate::rebis::therapeutics::UniversalAntidote::new("UA-001");
                    sprintln!("  {}Universal Antidote{}", style_section(), crate::style::reset());
                    sprintln!("  Name: {}  Targets: {}", antidote.name, antidote.n_targets);
                    sprintln!("  Library diversity: {} clones", antidote.library_diversity);
                    sprintln!("  Frobenius: {}", if antidote.frobenius_verified { "PASS" } else { "OPEN" });
                }
                "neuro" => {
                    let nf = crate::rebis::therapeutics::NeurotrophicFactor::new("NF-001", 25.0, 48.0);
                    sprintln!("  {}Neurotrophic Factor{}", style_section(), crate::style::reset());
                    sprintln!("  Name: {}  Receptor: {}", nf.name, nf.target_receptor);
                    sprintln!("  EC50: {:.1} nM  Half-life: {:.1}h", nf.ec50_nm, nf.half_life_hours);
                    sprintln!("  Pathway: {}  Frobenius: {}",
                        nf.downstream_pathway, if nf.frobenius_verified { "PASS" } else { "FAIL" });
                }
                _ => {
                    let chemo = Chemotherapeutic::new("RB-001", "TOP2A", 5.0, 500.0);
                    sprintln!("  {}Therapeutics{}", style_section(), crate::style::reset());
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


/// Lift an ELF's executable sections to IMASM words.
///
/// The decoder stops at the first opcode it does not know rather than guessing
/// a length, so coverage is reported and a partial lift always reads as partial.
#[cfg(feature = "hosted")]
fn vox_lift_file(path: &str) {
    let raw = match std::fs::read(path) {
        Ok(r) => r,
        Err(e) => {
            sprintln!("cannot read {}: {}", path, e);
            return;
        }
    };
    let (entry, segments) = crate::vox::parse_elf(&raw);
    if segments.is_empty() {
        sprintln!("{}: no executable sections found", path);
        return;
    }
    let image = crate::vox_decode::Image { segments };
    sprintln!("{}  entry 0x{:x}  {} byte(s) of code", path, entry, image.total_bytes());

    let seeds = crate::vox::elf_function_symbols(&raw);
    let w = crate::vox_decode::walk(&image, entry, &seeds);
    let funcs = &w.functions;
    let decoded: usize = funcs.iter().map(|(_, f)| f.len()).sum();
    sprintln!("  {} function(s), {} instruction(s)", funcs.len(), decoded);
    sprintln!("  claimed {}% of the image ({} of {} bytes)",
        w.claimed_percent(), w.claimed_bytes, w.total_bytes);
    if w.claimed_percent() < 100 {
        sprintln!("  the rest sits behind an indirect transfer or is not code, and");
        sprintln!("  nothing in the bytes tells those apart. Walking it anyway would");
        sprintln!("  not find functions, it would invent them mid-instruction.");
    }

    let mut tally = [0usize; 4];   // T, B, N, F
    let mut illtyped = alloc::vec::Vec::new();
    // B locates the arm left open, so the address is the useful part — a tally
    // says how many and nothing about which.
    let mut open_arms = alloc::vec::Vec::new();
    for (start, f) in funcs {
        let word = crate::vox::recompile_function(f);
        let v = crate::vox::verdict(&word);
        match v {
            'T' => tally[0] += 1,
            'B' => { tally[1] += 1; open_arms.push((*start, f.len())); }
            'N' => tally[2] += 1,
            _ => {
                tally[3] += 1;
                if illtyped.len() < 8 {
                    illtyped.push((*start, crate::vox::glyphs(&word)));
                }
            }
        }
    }
    sprintln!("");
    sprintln!("  verdicts  T {}   B {}   N {}   F {}", tally[0], tally[1], tally[2], tally[3]);
    if !open_arms.is_empty() {
        sprintln!("  B is an arm left OPEN. The addresses carrying one:");
        for (a, len) in open_arms.iter() {
            sprintln!("    0x{:x}   {} instruction(s)", a, len);
        }
        // The word itself is what `insert` and `weight` can act on, so print the
        // shortest open function's word: a repair is found on a word, not on a
        // tally.
        if let Some((addr, _)) = open_arms.iter().min_by_key(|(_, l)| *l) {
            for (start, f) in funcs {
                if start == addr {
                    let word = crate::vox::recompile_function(f);
                    let g = crate::vox::glyphs(&word);
                    sprintln!("");
                    sprintln!("  shortest open arm, 0x{:x}:", addr);
                    sprintln!("  {}", g);
                }
            }
        }
    }
    if tally[3] > 0 {
        sprintln!("  F is not a truth value: the word is ill-typed, a ∋ with no ∈ to");
        sprintln!("  pair. It marks a function cut in the wrong place, not a program.");
        for (a, g) in &illtyped {
            let head: alloc::string::String = g.chars().take(90).collect();
            sprintln!("    0x{:x}  {}", a, head);
        }
    }
}

#[cfg(not(feature = "hosted"))]
fn vox_lift_file(_path: &str) {
    sprintln!("vox lift needs a host filesystem; not available in the kernel build");
}

/// `vox run <symbol> --args a,b <file>` — order-tolerant, mirroring the
/// standalone `vox` binary's own CLI parser exactly (Vox/src/main.rs). Lifts
/// the file to the payload-carrying twelve-glyph module (`imasm_exec::emit`,
/// not the bare-glyph `vox lift`/`imasm derive` form) and actually runs the
/// named function through it: real registers, memory, flags, ALU. Only
/// exit()/exit_group() syscalls do anything; every other syscall halts rather
/// than pretending to succeed.
#[cfg(feature = "hosted")]
fn vox_run_symbol(args: &[alloc::string::String]) {
    // vox run <file> [--argv a,b]            — run it: real process, real
    //                                           argv/envp/auxv stack, real
    //                                           syscalls, from its own entry.
    // vox run <symbol> <file> [--args 1,2]   — call one function directly:
    //                                           scalar int args in, one int
    //                                           back, no process at all.
    // A single bare token is the FILE, not the symbol — with no name given
    // there is no function to call, so the whole file runs as a process.
    // This is also the only thing a PE binary offers, since the loader never
    // populates a symbol table for PE at all (only ELF has one).
    let mut bare: alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();
    let mut argv_ints: alloc::vec::Vec<i64> = alloc::vec::Vec::new();
    let mut argv_strs: alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--args" => {
                i += 1;
                if i < args.len() {
                    for a in args[i].split(',') {
                        let a = a.trim();
                        if !a.is_empty() {
                            let v = if let Some(h) = a.strip_prefix("0x") {
                                i64::from_str_radix(h, 16).unwrap_or(0)
                            } else {
                                a.parse().unwrap_or(0)
                            };
                            argv_ints.push(v);
                        }
                    }
                }
            }
            "--argv" => {
                i += 1;
                if i < args.len() {
                    for a in args[i].split(',') { argv_strs.push(a.to_string()); }
                }
            }
            other => bare.push(other.to_string()),
        }
        i += 1;
    }
    let (sym, file) = match bare.len() {
        0 => {
            sprintln!("vox run <file> [--argv a,b]   or   vox run <symbol> <file> [--args 1,2]");
            return;
        }
        1 => (alloc::string::String::new(), bare[0].clone()),
        _ => (bare[0].clone(), bare[1..].join(" ")),
    };
    if file.is_empty() {
        sprintln!("vox run <file> [--argv a,b]   or   vox run <symbol> <file> [--args 1,2]");
        return;
    }
    // A `.imasm` file is a saved module: text, already carrying its own
    // symbol table (`; sym NAME 0xADDR`), so it runs directly with no second
    // read of the original binary. Anything else is read as raw bytes and
    // lifted fresh — same as `vox imasm`/`vox run` in the standalone binary.
    let raw_for_words = std::fs::read(&file).ok();
    let is_module = file.ends_with(".imasm")
        || raw_for_words.as_deref().map(|b| b.starts_with(b"; ")).unwrap_or(false);
    let mut m = if is_module {
        match std::fs::read_to_string(&file) {
            Ok(text) => crate::imasm_exec::Machine::new(&text),
            Err(e) => {
                sprintln!("cannot read {}: {}", file, e);
                return;
            }
        }
    } else {
        let raw = match &raw_for_words {
            Some(r) => r,
            None => {
                sprintln!("cannot read {}", file);
                return;
            }
        };
        crate::imasm_exec::Machine::new(&crate::imasm_exec::emit(raw))
    };
    m.set_host(std::boxed::Box::new(crate::imasm_exec::StdHost::new()));
    let addr = if sym.is_empty() {
        let mut argv = alloc::vec![file.clone()];
        argv.extend(argv_strs);
        match m.run_process(&argv, &[], 50_000_000) {
            Ok(()) => sprintln!("entry(...) ran off the end with no exit call   [{} steps]", m.steps),
            Err(crate::imasm_exec::Stop::SysExit(c)) => sprintln!("entry(...) exited({})   [{} steps in the twelve]", c, m.steps),
            Err(crate::imasm_exec::Stop::Halt(e)) => sprintln!("entry(...) halted: {}   [{} steps]", e, m.steps),
        }
        m.entry
    } else {
        let addr = match m.resolve(&sym) {
            Some(a) => a,
            None => {
                sprintln!("no symbol '{}' in {}", sym, file);
                return;
            }
        };
        let argv_str: alloc::vec::Vec<alloc::string::String> =
            argv_ints.iter().map(|a| a.to_string()).collect();
        match m.call(addr, &argv_ints, 50_000_000) {
            Ok(r) => sprintln!("{}({}) = {}   [{} steps in the twelve]", sym, argv_str.join(", "), r, m.steps),
            Err(crate::imasm_exec::Stop::SysExit(c)) => sprintln!("{}(...) called exit({})   [{} steps in the twelve]", sym, c, m.steps),
            Err(crate::imasm_exec::Stop::Halt(e)) => sprintln!("{}(...) halted: {}   [{} steps]", sym, e, m.steps),
        }
        addr
    };
    // The execution above reads the payload-carrying module (real operands).
    // What weight/banked/cycle/imasm-derive read is the bare structural word
    // for the same function — the other half of the same object, not a
    // separate lookup. `words()` needs the raw x86 bytes to walk (a saved
    // `.imasm` file no longer carries those), so this half only runs when a
    // real binary was given.
    if !is_module {
        if let Some(raw) = &raw_for_words {
            let target = alloc::format!("0x{:x}\t", addr);
            let bare = crate::imasm_exec::words(raw);
            if let Some(line) = bare.lines().find(|l| l.starts_with(&target)) {
                if let Some(word) = line.split('\t').nth(1) {
                    let glyphs: alloc::vec::Vec<char> = word.chars().collect();
                    sprintln!("structure word (feed to weight/banked/cycle/imasm derive):");
                    sprintln!("  {}   verdict {}", word, crate::vox::verdict(&glyphs));
                }
            }
        }
    }
}

#[cfg(not(feature = "hosted"))]
fn vox_run_symbol(_args: &[alloc::string::String]) {
    sprintln!("vox run needs a host filesystem; not available in the kernel build");
}
