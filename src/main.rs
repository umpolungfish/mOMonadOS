#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo, config::{BootloaderConfig, Mapping}};
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

mod serial;
mod belnap;
mod tokens;
mod crystal;
mod kernel;
mod interrupts;
mod manus;

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
    sprintln!("mOMonadOS REPL commands (graph execution — 12 tokens, 0 control opcodes):");
    sprintln!("  tick [N]              — run N manual ticks (default 1)");
    sprintln!("  run [N]               — run N ticks; no arg = continuous (ESC to stop)");
    sprintln!("  watch [N]             — live terminal HUD, refresh every N ticks (ESC to stop)");
    sprintln!("  graph                 — ASCII-art token graph with nesting"); 
    sprintln!("  heatmap [start] [n]   — B4 memory heatmap with color blocks");
    sprintln!("  timer [N]             — run N ticks, one per PIT interrupt (ESC to stop)");
    sprintln!("  boot <I–XIX>           — load any program + run continuously");
    sprintln!("  load <I–XIX>           — load any program by Roman numeral");
    sprintln!("  status                — kernel status");
    sprintln!("  program               — show loaded program + fork depth");
    sprintln!("  snapshot              — structural snapshot (sig, tier, period, ...)");
    sprintln!("  canonical <I–XII>      — load canonical program");
    sprintln!("  continuous <1–4>       — load continuous program");
    sprintln!("  novel <1–3>            — load novel program (XVII–XIX)");
    sprintln!("  list                  — list all programs");
    sprintln!("  crystal <addr>        — decode address");
    sprintln!("  crystal store <n> [d] — store entry");
    sprintln!("  crystal name <n>      — retrieve by name");
    sprintln!("  crystal find          — list stored entries");
    sprintln!("  memory [start] [n]    — dump B4 memory");
    sprintln!("  registers             — show R0-R7");
    sprintln!("  stack                 — stack depth");
    sprintln!("  halt/quit             — exit");
    sprintln!();
    sprintln!("Control flow is token-graph-native:");
    sprintln!("  FSPLIT (1->2) = fork     FFUSE (2->1) = join");
    sprintln!("  EVALT/EVALF (1->1) = branch gates");
    sprintln!("  TANCH (1->0) = halt       VINIT (0->1) = source");
    sprintln!("  IMSCRIB (1->1) = self-loop  End-of-program = cycle");
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
