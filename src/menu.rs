// ─── mOMonadOS Menu System ──────────────────────────────────
// Hierarchical menu navigation, sub-context REPLs, tab completion
#![allow(dead_code)]

extern crate alloc;
use crate::serial;
use crate::sprintln;

// ─── Menu Item ─────────────────────────────────────────────

pub struct MenuItem {
    pub name: &'static str,
    pub cmd: &'static str,
    pub desc: &'static str,
    pub submenu: Option<&'static [MenuItem]>,
}

// ─── Top-level menu categories ────────────────────────────

pub static MAIN_MENU: &[MenuItem] = &[
    MenuItem { name: "Exec",     cmd: "exec",     desc: "Execution (run, tick, watch, timer, boot)", submenu: Some(EXEC_MENU) },
    MenuItem { name: "Status",   cmd: "status",   desc: "Status (program, snapshot, graph, heatmap, registers)", submenu: Some(STATUS_MENU) },
    MenuItem { name: "Programs", cmd: "programs",  desc: "Program loading (list, canonical, continuous, novel, shunt)", submenu: Some(PROGRAMS_MENU) },
    MenuItem { name: "Crystal",  cmd: "crystal",  desc: "Crystal FS (decode, store, find, name)", submenu: Some(CRYSTAL_MENU) },
    MenuItem { name: "Grammar",  cmd: "grammar",  desc: "Grammar bridges (ig, classify, frob, aleph, shor, rh, ym)", submenu: Some(GRAMMAR_MENU) },
    MenuItem { name: "Rebis",    cmd: "rebis",    desc: "Red-Hot Rebis (codon, translate, genetics, materials, bio, tx)", submenu: Some(REBIS_MENU) },
    MenuItem { name: "Dialect", cmd: "dialect",  desc: "Cross-dialect (ruleset, jump, seal, compound, whoami)", submenu: Some(DIALECT_MENU) },
    MenuItem { name: "ParaASM",  cmd: "parasm",   desc: "ParaASM (test, frob, kernel, load)", submenu: Some(PARASM_MENU) },
    MenuItem { name: "Cr3echrz", cmd: "cr3echrz", desc: "Theorem engine + p4rakernel (cr3, p4ra)", submenu: Some(CR3ECHRZ_MENU) },
    MenuItem { name: "Help",     cmd: "help",     desc: "Help system (help <topic> for details)", submenu: None },
];

pub static EXEC_MENU: &[MenuItem] = &[
    MenuItem { name: "tick",     cmd: "tick",     desc: "Run N manual ticks (default 1)", submenu: None },
    MenuItem { name: "run",      cmd: "run",      desc: "Run N ticks; no arg = continuous (ESC to stop)", submenu: None },
    MenuItem { name: "watch",    cmd: "watch",    desc: "Live terminal HUD (ESC to stop)", submenu: None },
    MenuItem { name: "timer",    cmd: "timer",    desc: "Run N ticks, one per PIT interrupt", submenu: None },
    MenuItem { name: "boot",     cmd: "boot",     desc: "Load + run any program (I-XXVIII or decimal)", submenu: None },
    MenuItem { name: "load",     cmd: "load",     desc: "Load program by Roman numeral", submenu: None },
];

pub static STATUS_MENU: &[MenuItem] = &[
    MenuItem { name: "status",   cmd: "status",   desc: "Kernel status (tick, IP, stack, fork, frob)", submenu: None },
    MenuItem { name: "program",  cmd: "program",  desc: "Show loaded program + fork depth", submenu: None },
    MenuItem { name: "snapshot", cmd: "snapshot", desc: "Structural snapshot (sig, tier, period)", submenu: None },
    MenuItem { name: "graph",    cmd: "graph",    desc: "ASCII-art token graph with nesting", submenu: None },
    MenuItem { name: "heatmap",  cmd: "heatmap",  desc: "B4 memory heatmap", submenu: None },
    MenuItem { name: "memory",   cmd: "memory",   desc: "Dump B4 memory", submenu: None },
    MenuItem { name: "registers",cmd: "registers",desc: "Show R0-R7", submenu: None },
    MenuItem { name: "stack",    cmd: "stack",    desc: "Stack depth", submenu: None },
];

pub static PROGRAMS_MENU: &[MenuItem] = &[
    MenuItem { name: "list",     cmd: "list",     desc: "List all programs (I-XXVIII)", submenu: None },
    MenuItem { name: "canonical",cmd: "canonical",desc: "Load canonical program I-XII", submenu: None },
    MenuItem { name: "continuous",cmd: "continuous",desc: "Load continuous program 1-4", submenu: None },
    MenuItem { name: "novel",    cmd: "novel",    desc: "Load novel program 1-3", submenu: None },
    MenuItem { name: "shunt",    cmd: "shunt",    desc: "Load shunted program 1-9", submenu: None },
    MenuItem { name: "dynamic",  cmd: "dynamic",  desc: "Dynamic mode: rebuild sequence from IgTuple each wrap", submenu: None },
];

pub static CRYSTAL_MENU: &[MenuItem] = &[
    MenuItem { name: "decode",   cmd: "crystal",  desc: "Decode address to 12-tuple: crystal <addr>", submenu: None },
    MenuItem { name: "store",    cmd: "crystal store", desc: "Store entry: crystal store <n> [d]", submenu: None },
    MenuItem { name: "name",     cmd: "crystal name",  desc: "Retrieve by name: crystal name <n>", submenu: None },
    MenuItem { name: "find",     cmd: "crystal find",  desc: "List stored entries", submenu: None },
];

pub static GRAMMAR_MENU: &[MenuItem] = &[
    MenuItem { name: "ig",       cmd: "ig",       desc: "IG tuple + crystal address", submenu: None },
    MenuItem { name: "classify", cmd: "classify", desc: "Nearest-catalog classification", submenu: None },
    MenuItem { name: "frob",     cmd: "frob",     desc: "Frobenius harness status", submenu: None },
    MenuItem { name: "aleph",    cmd: "aleph",    desc: "Hebrew glyph encoding: aleph <word>", submenu: None },
    MenuItem { name: "shor",     cmd: "shor",     desc: "Belnap Shor pipeline (N=15,21)", submenu: None },
    MenuItem { name: "rh",       cmd: "rh",       desc: "Riemann Hypothesis bridge", submenu: None },
    MenuItem { name: "ym",       cmd: "ym",       desc: "Yang-Mills mass gap bridge", submenu: None },
    MenuItem { name: "temp",     cmd: "temp",     desc: "Temporal logic bridge", submenu: None },
    MenuItem { name: "cat",      cmd: "cat",      desc: "Category theory bridge", submenu: None },
    MenuItem { name: "algebra",  cmd: "algebra",  desc: "distance|meet|join|tensor vs ZFC", submenu: None },
    MenuItem { name: "cl8nk",    cmd: "cl8nk",    desc: "CLINK Layer 8: cl8nk <action> [name]", submenu: None },
        MenuItem { name: "c4",       cmd: "c4",       desc: "Belnap C₄ complex plane (i²=B)", submenu: None },
    MenuItem { name: "cscore",   cmd: "cscore",   desc: "Consciousness score (dual-gate)", submenu: None },
];

pub static REBIS_MENU: &[MenuItem] = &[
    MenuItem { name: "codon",    cmd: "rebis codon",    desc: "Codon ↔ AA bidirectional", submenu: None },
    MenuItem { name: "translate",cmd: "rebis translate",desc: "Gene → protein pipeline", submenu: None },
    MenuItem { name: "reverse",  cmd: "rebis reverse",  desc: "Protein → mRNA → DNA", submenu: None },
    MenuItem { name: "frob",     cmd: "rebis frob",     desc: "Frobenius filtration (64 codons)", submenu: None },
    MenuItem { name: "genetics", cmd: "rebis genetics", desc: "7-stage genetic code verification", submenu: None },
    MenuItem { name: "hadron",   cmd: "rebis hadron",   desc: "Belnap hadron analysis", submenu: None },
    MenuItem { name: "serpent",  cmd: "rebis serpent",  desc: "Serpent rod motif analysis", submenu: None },
    MenuItem { name: "pipeline", cmd: "rebis pipeline", desc: "IG promotion pipeline", submenu: None },
    MenuItem { name: "strata",   cmd: "rebis strata",   desc: "Codon stratum counts", submenu: None },
    MenuItem { name: "asm",      cmd: "rebis asm",      desc: "Genetic ParaASM programs", submenu: None },
    MenuItem { name: "tuples",   cmd: "rebis tuples",   desc: "7-stage generative tuple pipeline", submenu: None },
    MenuItem { name: "clu",      cmd: "rebis clu",      desc: "CLU power-law clustering", submenu: None },
    MenuItem { name: "exotic",   cmd: "rebis exotic",   desc: "Exotic hadron Frobenius verification", submenu: None },
    MenuItem { name: "pdb",      cmd: "rebis pdb",      desc: "PDB structure validation", submenu: None },
    MenuItem { name: "antibody", cmd: "rebis antibody", desc: "Antibody CDR design", submenu: None },
    MenuItem { name: "material", cmd: "rebis material", desc: "IG material forge & metamaterials", submenu: None },
    MenuItem { name: "sidechain",cmd: "rebis sidechain",desc: "AA sidechain × environment algebra (20×4)", submenu: None },
    MenuItem { name: "ligand",   cmd: "rebis ligand",   desc: "Ligand design from catalytic sites", submenu: None },
    MenuItem { name: "decay",    cmd: "rebis decay",    desc: "Nuclear decay as IMASM winding", submenu: None },
    MenuItem { name: "bio",      cmd: "rebis bio",      desc: "Biological simulation", submenu: None },
    MenuItem { name: "tx",       cmd: "rebis tx",       desc: "Therapeutics (chemo, pill, antidote)", submenu: None },
];

pub static DIALECT_MENU: &[MenuItem] = &[
    MenuItem { name: "show",     cmd: "ruleset show",    desc: "Active ruleset display", submenu: None },
    MenuItem { name: "list",     cmd: "ruleset list",    desc: "List all 8 dialects", submenu: None },
    MenuItem { name: "verify",   cmd: "ruleset verify",  desc: "Invariant violation check", submenu: None },
    MenuItem { name: "jump",     cmd: "jump",            desc: "Cross-dialect jump: jump <U> using <c>", submenu: None },
    MenuItem { name: "seal",     cmd: "seal",            desc: "IFIX commit to current ruleset", submenu: None },
    MenuItem { name: "whoami",   cmd: "whoami --ruleset",desc: "IG tuple under active ruleset", submenu: None },
    MenuItem { name: "tensor",   cmd: "tensor",          desc: "Tensor under active absorption", submenu: None },
    MenuItem { name: "meet",     cmd: "meet",            desc: "Meet under active absorption", submenu: None },
    MenuItem { name: "absorb",   cmd: "absorb_test",     desc: "Test absorption rule", submenu: None },
    MenuItem { name: "abs-show", cmd: "absorption show", desc: "List absorption rules", submenu: None },
    MenuItem { name: "tstatus",  cmd: "tstatus",         desc: "T-constitution pass/fail", submenu: None },
    MenuItem { name: "compounds",cmd: "compound list",   desc: "List 11 diaschizic compounds", submenu: None },
    MenuItem { name: "compound", cmd: "compound",        desc: "compound show|load <name>", submenu: None },
];

pub static PARASM_MENU: &[MenuItem] = &[
    MenuItem { name: "test",     cmd: "psm test",   desc: "Dialetheic alignment + measurement", submenu: None },
    MenuItem { name: "frob",     cmd: "psm frob",   desc: "Frobenius identity cycle", submenu: None },
    MenuItem { name: "kernel",   cmd: "psm kernel", desc: "Kernel-state B3 invariant loop", submenu: None },
    MenuItem { name: "load",     cmd: "psm load",   desc: "Inline ParaASM program (; separator)", submenu: None },
];
pub static CR3ECHRZ_MENU: &[MenuItem] = &[
    MenuItem { name: "cr3",      cmd: "cr3",       desc: "Theorem engine (Collatz, Goldbach, Three-Body, Burnside, ...)", submenu: None },
    MenuItem { name: "p4ra",     cmd: "p4ra",      desc: "p4rakernel Belnap+Frobenius 13-step bootstrap", submenu: None },
    MenuItem { name: "version",  cmd: "cr3 --version", desc: "cr3 version info", submenu: None },
    MenuItem { name: "list",     cmd: "cr3 --list", desc: "List theorems + p4rakernel modules", submenu: None },
];



// ─── Menu Bar ──────────────────────────────────────────────

pub fn render_menu_bar() {
    serial::write_str("\n┌");
    for (i, item) in MAIN_MENU.iter().enumerate() {
        if i > 0 { serial::write_str("┬"); }
        let label = alloc::format!("[F{}]{}", i + 1, item.name);
        for _ in 0..(label.len().min(12)) { serial::write_str("─"); }
        serial::write_str("─");
    }
    serial::write_str("┐\n│");
    for (i, item) in MAIN_MENU.iter().enumerate() {
        if i > 0 { serial::write_str("│"); }
        let label = alloc::format!("F{}:{}", i + 1, item.name);
        serial::write_str(&label);
        // pad
        let width = 13;
        for _ in label.len()..width { serial::write_str(" "); }
    }
    serial::write_str("│\n└");
    for (i, _) in MAIN_MENU.iter().enumerate() {
        if i > 0 { serial::write_str("┴"); }
        serial::write_str("─────────────");
    }
    serial::write_str("┘\n");
}

/// Show compact menu hint (one line)
pub fn menu_hint() {
    serial::write_str("[F1]Exec [F2]Status [F3]Progs [F4]Crystal [F5]Grammar [F6]Rebis [F7]Dialect [F8]ParaASM [F9]Cr3 [F10]Help  [?]Menu\n");
}

// ─── Sub-context ───────────────────────────────────────────

/// Context stack entry
#[derive(Clone, Copy)]
pub struct Context {
    pub name: &'static str,
    pub menu: &'static [MenuItem],
}

pub static CONTEXT_STACK_CAP: usize = 4;

pub struct ContextStack {
    pub stack: [Option<Context>; CONTEXT_STACK_CAP],
    pub depth: usize,
}

impl ContextStack {
    pub const fn new() -> Self {
        Self { stack: [None; CONTEXT_STACK_CAP], depth: 0 }
    }

    pub fn push(&mut self, ctx: Context) -> bool {
        if self.depth >= CONTEXT_STACK_CAP { return false; }
        self.stack[self.depth] = Some(ctx);
        self.depth += 1;
        true
    }

    pub fn pop(&mut self) -> Option<Context> {
        if self.depth == 0 { return None; }
        self.depth -= 1;
        self.stack[self.depth].take()
    }

    pub fn current(&self) -> Option<&Context> {
        if self.depth == 0 { None }
        else { self.stack[self.depth - 1].as_ref() }
    }

    pub fn prompt(&self) -> &str {
        match self.current() {
            Some(ctx) => {
                // We'll return the name for the prompt builder to use
                ctx.name
            }
            None => "",
        }
    }
}

/// Render prompt with context breadcrumb
pub fn render_prompt(ctx: &ContextStack) {
    if ctx.depth > 0 {
        serial::write_str("⊙[");
        for i in 0..ctx.depth {
            if i > 0 { serial::write_str("/"); }
            if let Some(c) = &ctx.stack[i] {
                serial::write_str(c.name);
            }
        }
        serial::write_str("]> ");
    } else {
        serial::write_str("⊙> ");
    }
}

// ─── Hierarchical Help ─────────────────────────────────────

/// Show help for a topic ("" = top-level)
pub fn print_help_topic(topic: &str) {
    if topic.is_empty() {
        print_top_help();
        return;
    }

    // Search all menus for matching category
    for item in MAIN_MENU {
        if item.cmd == topic || item.name.to_lowercase() == topic.to_lowercase() {
            if let Some(sub) = item.submenu {
                sprintln!("══ {} Commands ══", item.name);
                sprintln!("  {}", item.desc);
                sprintln!();
                for si in sub {
                    sprintln!("  {:<24} — {}", si.cmd, si.desc);
                }
                sprintln!();
                sprintln!("Type '..' or 'back' to return to top-level.");
                sprintln!("Type 'help {} <subcmd>' for details on a sub-command.", item.cmd);
                return;
            } else {
                sprintln!("  {} — {}", item.cmd, item.desc);
                return;
            }
        }

        // Search sub-menus
        if let Some(sub) = item.submenu {
            for si in sub {
                if si.cmd.starts_with(topic) || si.name == topic {
                    // Show sub-menu this item belongs to
                    sprintln!("══ {} → {} ══", item.name, si.name);
                    sprintln!("  {}", si.desc);
                    sprintln!();
                    sprintln!("Full command: {}", si.cmd);
                    return;
                }
            }
        }
    }

    // Partial search
    sprintln!("No exact match for '{}'. Similar commands:", topic);
    search_commands(topic);
}

fn print_top_help() {
    sprintln!("mOMonadOS — Menu Navigation Help");
    sprintln!();
    sprintln!("══ Categories ══ (type name or F1-F10 to enter)");
    sprintln!();
    for (i, item) in MAIN_MENU.iter().enumerate() {
        sprintln!("  [F{}] {:<12} — {}", i + 1, item.name, item.desc);
    }
    sprintln!();
    sprintln!("══ Quick Reference ══");
    sprintln!("  F1-F10       — jump to category");
    sprintln!("  ?            — show menu bar");
    sprintln!("  Tab          — autocomplete command");
    sprintln!("  Up/Down      — command history");
    sprintln!("  .. or back   — exit sub-context");
    sprintln!("  help <topic> — detailed help for a command or category");
    sprintln!("  ? <keyword>  — search all commands");
    sprintln!();
}

/// Tab completion: given partial input, find matching command
pub fn tab_complete(input: &str, ctx: &ContextStack) -> Option<&'static str> {
    if input.is_empty() { return None; }

    let menu = if let Some(c) = ctx.current() {
        c.menu
    } else {
        // Top-level: search MAIN_MENU names + all submenu commands
        return tab_complete_top(input);
    };

    for item in menu {
        if item.cmd.starts_with(input) {
            return Some(item.cmd);
        }
    }
    None
}

fn tab_complete_top(input: &str) -> Option<&'static str> {
    // Check category names first
    for item in MAIN_MENU {
        if item.cmd.starts_with(input) {
            return Some(item.cmd);
        }
    }
    // Then check all sub-commands
    for item in MAIN_MENU {
        if let Some(sub) = item.submenu {
            for si in sub {
                if si.cmd.starts_with(input) {
                    return Some(si.cmd);
                }
            }
        }
    }
    None
}

/// Search all commands matching keyword
pub fn search_commands(keyword: &str) {
    let kw = keyword.to_lowercase();
    let mut found = 0u32;
    for item in MAIN_MENU {
        if item.name.to_lowercase().contains(&kw) || item.desc.to_lowercase().contains(&kw) {
            sprintln!("  [{}] {:<12} — {}", item.name, item.cmd, item.desc);
            found += 1;
        }
        if let Some(sub) = item.submenu {
            for si in sub {
                if si.name.to_lowercase().contains(&kw) || si.desc.to_lowercase().contains(&kw) || si.cmd.to_lowercase().contains(&kw) {
                    sprintln!("  {:<24} — {}", si.cmd, si.desc);
                    found += 1;
                }
            }
        }
    }
    if found == 0 {
        sprintln!("  No commands match '{}'", keyword);
    } else {
        sprintln!("  ({} match{})", found, if found == 1 { "" } else { "es" });
    }
}

/// Enter a sub-context by category name
pub fn enter_context(ctx: &mut ContextStack, name: &str) -> bool {
    for item in MAIN_MENU {
        if item.cmd.eq_ignore_ascii_case(name) || item.name.eq_ignore_ascii_case(name) {
            if let Some(sub) = item.submenu {
                ctx.push(Context { name: item.name, menu: sub });
                sprintln!("Entered: {} ({} commands). Type '..' to return.", item.name, sub.len());
                return true;
            }
        }
    }
    false
}

/// Translate F-key (1-10) to category
pub fn fkey_to_category(fkey: u8) -> Option<&'static str> {
    if fkey >= 1 && fkey <= 10 {
        Some(MAIN_MENU[(fkey - 1) as usize].cmd)
    } else {
        None
    }
}
