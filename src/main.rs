#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use alloc::string::String;
use bootloader_api::{entry_point, BootInfo, config::{BootloaderConfig, Mapping}};
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

mod serial;
mod belnap;
mod tokens;
mod crystal;
mod kernel;
mod interrupts;
mod frob_verify;
mod imas_ig;
mod aleph;
mod manus;
mod parasm;
mod belnap_shor;
mod para_rh;
mod para_ym;
mod para_temporal;
mod para_category;
mod algebra;
mod catalog;
mod cl8nk;
mod consciousness;

use tokens::{canonical_name, CANONICAL_COUNT, continuous_name, CONTINUOUS_COUNT, novel_name, NOVEL_COUNT};
use crystal::{CrystalStore, decode, encode, indices_from_snapshot, TOTAL};
use kernel::Kernel;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 0x40000;
    config
};

entry_point!(kmain, config = &BOOTLOADER_CONFIG);

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    sprintln!("[BOOT] mOMonadOS — The Self-Imscribing Bare-Metal Kernel");

    interrupts::init(100);
    sprintln!("[BOOT] Interrupts online — PIT 100Hz, PIC remapped");

    if let Some(phys_offset) = boot_info.physical_memory_offset.into_option() {
        if let Some(region) = boot_info.memory_regions.iter().find(|r| {
            matches!(r.kind, bootloader_api::info::MemoryRegionKind::Usable)
                && r.end.saturating_sub(r.start) >= 4 * 1024 * 1024
        }) {
            let heap_phys = region.start + 0x100_0000;
            let heap_start = (phys_offset + heap_phys) as *mut u8;
            let heap_size = 4 * 1024 * 1024usize;
            unsafe { ALLOCATOR.lock().init(heap_start, heap_size); }
            sprintln!("[BOOT] Heap: 4MB @ {:#x}", phys_offset + heap_phys);
        }
    }

    let mut k = Kernel::new();
    k.boot();
    catalog::catalog_init();
    sprintln!("[BOOT] IG Catalog: {} entries loaded", catalog::catalog_size());
    sprintln!("[BOOT] Kernel online — graph execution, token-arity driven");
    sprintln!("[BOOT] Bootstrap: IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB (cyclic)");
    sprintln!("[BOOT] Crystal FS: {} addresses", TOTAL);
    sprintln!("[BOOT] {} total programs (I–XIX): 12 canonical + {} continuous + {} novel",
        CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT,
        CONTINUOUS_COUNT, NOVEL_COUNT);
    sprintln!();

    print_banner();
    repl(&mut k);

    // ── Shutdown: write to QEMU isa-debug-exit port (0xf4).
    // Value 0x10 → QEMU exits with status 0.
    // On real hardware or without the device, falls through to HLT.
    sprintln!("[SHUTDOWN] μ∘δ=id. Goodbye.");
    unsafe {
        x86_64::instructions::port::PortWrite::write_to_port(0xf4, 0x10u32);
    }
    loop { x86_64::instructions::hlt(); }
}

fn print_banner() {
    sprintln!("╔══════════════════════════════════════════════════╗");
    sprintln!("             m O M o n a d O S                    ");
    sprintln!("     The Self-Imscribing Bare-Metal Kernel         ");
    sprintln!("     Frobenius Core · Belnap FOUR · Crystal FS     ");
    sprintln!("     Graph Execution — Token Arity as Topology     ");
    sprintln!("╚══════════════════════════════════════════════════╝");
    sprintln!();
    sprintln!("Type 'help' for commands.");
    sprintln!();
}

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

fn repl(k: &mut Kernel) {
    let mut cfs = CrystalStore::new();
    let mut line_buf = [0u8; 256];
    let mut history = History::new();

    loop {
        serial::write_str("⊙> ");
        let line = read_line(&mut line_buf, &mut history);
        if line.is_empty() { continue; }

        let mut parts = line.splitn(4, ' ');
        let cmd = parts.next().unwrap_or("");

        match cmd {
            "quit" | "exit" | "halt" => {
                sprintln!("Halting. μ∘δ=id.");
                k.halt();
                break;
            }
            "help" => print_help(),
            "status" => print_status(k),
            "frob" => print_frob(k),
            "ig" => print_ig(k),
            "classify" => print_classify(k),
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
            "cscore" => print_cscore(k),
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
                        x86_64::instructions::hlt();
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
                    if i >= CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT {
                        sprintln!("Program {} out of range (max XIX/{}).",
                            arg, CANONICAL_COUNT + CONTINUOUS_COUNT + NOVEL_COUNT);
                    } else if load_by_roman(k, arg) {
                        let name: &str = if i < CANONICAL_COUNT {
                            canonical_name(i)
                        } else if i < CANONICAL_COUNT + CONTINUOUS_COUNT {
                            continuous_name(i - CANONICAL_COUNT)
                        } else {
                            novel_name(i - CANONICAL_COUNT - CONTINUOUS_COUNT)
                        };
                        sprintln!("Booting {}: {}", arg, name);
                        sprintln!("Running (ESC to stop)...");
                        let ran = k.run_continuous(|| interrupts::escape_pressed());
                        sprintln!("\nStopped after {} ticks.", ran);
                        print_status(k);
                    }
                } else {
                    sprintln!("Usage: boot <I–XIX>");
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
                    let eff_dial = snap.dialetheia_complete || snap.b_live_ticks > 0;
                    sprintln!("dialeth:  {} (b_live_ticks={})", eff_dial, snap.b_live_ticks);
                    sprintln!("period:   {}", snap.period);
                } else {
                    sprintln!("No snapshot — tick first.");
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
            "crystal" => {
                let sub = parts.next().unwrap_or("").trim();
                match sub {
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
                sprintln!("Use 'load <I–XIX>' to load any program by Roman numeral.");
            }
            "load" => {
                let arg = parts.next().unwrap_or("").trim();
                if load_by_roman(k, arg) {
                    let idx = roman_to_idx(arg).unwrap();
                    let name: &str = if idx < CANONICAL_COUNT {
                        canonical_name(idx)
                    } else if idx < CANONICAL_COUNT + CONTINUOUS_COUNT {
                        continuous_name(idx - CANONICAL_COUNT)
                    } else {
                        novel_name(idx - CANONICAL_COUNT - CONTINUOUS_COUNT)
                    };
                    sprintln!("Loaded {}: {}", arg, name);
                    serial::write_str("Program: ");
                    for (j, t) in k.program.as_slice().iter().enumerate() {
                        if j > 0 { serial::write_str(" → "); }
                        serial::write_str(t.name());
                    }
                    sprintln!();
                } else {
                    sprintln!("Unknown program: {}. Use 'list' to see I–XIX.", arg);
                }
            }
            "" => {}
            _ => sprintln!("Unknown: {}. Type 'help'.", cmd),
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
        let indices = indices_from_snapshot(
            snap.frobenius_order,
            snap.period,
            snap.sig,
            snap.token_diversity,
            snap.self_ref,
            snap.dialetheia_complete,
            snap.tier,
            k.program.len(),
        );
        let addr = encode(&indices);
        cfs.store(name, data, addr, canonical_idx)
    } else {
        0
    }
}

// ─── Input ────────────────────────────────────────────────────

fn read_line<'a>(buf: &'a mut [u8; 256], history: &mut History) -> &'a str {
    let mut len = 0usize;
    let mut hist_pos = 0usize;

    loop {
        let b = serial::read_byte();
        match b {
            0x1b => {
                if serial::read_byte() != b'[' { continue; }
                match serial::read_byte() {
                    b'A' => {
                        let next = (hist_pos + 1).min(history.count);
                        if next != hist_pos {
                            hist_pos = next;
                            if let Some((bytes, n)) = history.get(hist_pos) {
                                redraw_input(len, bytes, n, buf);
                                len = n;
                            }
                        }
                    }
                    b'B' => {
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
                    }
                    _ => {}
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
                if len < buf.len() - 1 {
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

fn redraw_input(old_len: usize, src: &[u8], src_len: usize, buf: &mut [u8; 256]) {
    let _ = old_len;
    serial::write_str("\r\x1b[K⊙> ");
    let n = src_len.min(255);
    buf[..n].copy_from_slice(&src[..n]);
    if let Ok(s) = core::str::from_utf8(&buf[..n]) {
        serial::write_str(s);
    }
}

// ─── Helpers ──────────────────────────────────────────────────

fn print_help() {
    sprintln!("mOMonadOS REPL commands:");
    sprintln!();
    sprintln!("══ Execution ══");
    sprintln!("  tick [N]              — run N manual ticks (default 1)");
    sprintln!("  run [N]               — run N ticks; no arg = continuous (ESC to stop)");
    sprintln!("  watch [N]             — live terminal HUD, refresh every N ticks (ESC to stop)");
    sprintln!("  timer [N]             — run N ticks, one per PIT interrupt (ESC to stop)");
    sprintln!("  boot <I–XIX>          — load any program + run continuously");
    sprintln!("  load <I–XIX>          — load any program by Roman numeral");
    sprintln!();
    sprintln!("══ Status ══");
    sprintln!("  status                — kernel status (tick, IP, stack, fork, frob, halted)");
    sprintln!("  program               — show loaded program + fork depth");
    sprintln!("  snapshot              — structural snapshot (sig, tier, period, dialeth, ...)");
    sprintln!("  graph                 — ASCII-art token graph with nesting");
    sprintln!("  heatmap [start] [n]   — B4 memory heatmap with color blocks");
    sprintln!("  memory [start] [n]    — dump B4 memory");
    sprintln!("  registers             — show R0-R7");
    sprintln!("  stack                 — stack depth");
    sprintln!();
    sprintln!("══ Program Loading ══");
    sprintln!("  list                  — list all programs (I–XIX)");
    sprintln!("  canonical <I–XII>     — load canonical program");
    sprintln!("  continuous <1–4>      — load continuous program");
    sprintln!("  novel <1–3>           — load novel program (XVII–XIX)");
    sprintln!();
    sprintln!("══ Crystal FS ══");
    sprintln!("  crystal <addr>        — decode address to 12-tuple");
    sprintln!("  crystal store <n> [d] — store entry");
    sprintln!("  crystal name <n>      — retrieve by name");
    sprintln!("  crystal find          — list stored entries");
    sprintln!();
    sprintln!("══ Grammar Bridges ══");
    sprintln!("  ig                    — IG tuple + crystal address");
    sprintln!("  classify              — nearest-catalog classification");
    sprintln!("  frob                  — Frobenius harness status (closed/open ratio)");
    sprintln!("  aleph <Hebrew word>   — Hebrew glyph encoding + gematria");
    sprintln!("  shor                  — Belnap Shor pipeline (N=15, N=21)");
    sprintln!("  rh                    — Riemann Hypothesis bridge");
    sprintln!("  ym                    — Yang-Mills mass gap bridge");
    sprintln!("  temp                  — Temporal logic bridge");
    sprintln!("  cat                   — Category theory bridge");
    sprintln!("  algebra <op>          — distance|meet|join|tensor vs ZFC baseline");
    sprintln!("  cl8nk <action> [name]   — promotions | entry <name> (any catalog system)");
    sprintln!("  cscore                — consciousness score (dual-gate)");
    sprintln!();
    sprintln!("══ ParaASM ══");
    sprintln!("  psm test              — dialetheic alignment + measurement tests");
    sprintln!("  psm frob              — Frobenius identity cycle (ENGAGR→FSPLIT→FFUSE→HALT)");
    sprintln!("  psm kernel            — kernel-state B3 invariant loop");
    sprintln!("  psm load <prog>       — inline ParaASM program (; separator)");
    sprintln!();
    sprintln!("  halt/quit             — exit (μ∘δ=id)");
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

fn roman_to_idx(s: &str) -> Option<usize> {
    match s {
        "I"    => Some(0),  "II"   => Some(1),  "III" => Some(2),
        "IV"   => Some(3),  "V"    => Some(4),  "VI"  => Some(5),
        "VII"  => Some(6),  "VIII" => Some(7),  "IX"  => Some(8),
        "X"    => Some(9),  "XI"   => Some(10), "XII" => Some(11),
        "XIII" => Some(12), "XIV"  => Some(13), "XV"  => Some(14),
        "XVI"  => Some(15), "XVII" => Some(16), "XVIII" => Some(17),
        "XIX"  => Some(18),
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
        18 => "XIX",
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

// ─── Panic ────────────────────────────────────────────────────

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial::write_str("\n[PANIC] ");
    sprint!("{}", info.message());
    sprintln!();
    loop { x86_64::instructions::hlt(); }
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
    for &v in &[B4::N, B4::T, B4::F, B4::B] {
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
    sprintln!("  H|N⟩=N: {}", if b4_hadamard(B4::N) == B4::N { "PASS" } else { "FAIL" });

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
            let irrev_n = collapse_irreversible(B4::N);
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
                            if b != B4::N || s.steps > 0 {
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
                sprintln!("  Current: {}", ig.display_shavian());
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
