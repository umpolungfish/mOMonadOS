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

use tokens::{canonical_name, CANONICAL_COUNT};
use crystal::{CrystalStore, decode, encode, indices_from_snapshot, TOTAL};
use kernel::Kernel;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 0x40000; // 256 KB
    config
};

entry_point!(kmain, config = &BOOTLOADER_CONFIG);

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    serial::init();
    sprintln!("[BOOT] mOMonadOS — The Self-Imscribing Bare-Metal Kernel");

    // Heap
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
    sprintln!("[BOOT] Kernel online — μ∘δ=id");
    sprintln!("[BOOT] Bootstrap: ISCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→ISCRIB");
    sprintln!("[BOOT] Crystal FS: {} addresses", TOTAL);
    sprintln!();

    print_banner();
    repl(&mut k);

    loop { x86_64::instructions::hlt(); }
}

fn print_banner() {
    sprintln!("╔══════════════════════════════════════════════════╗");
    sprintln!("║            m O M o n a d O S                    ║");
    sprintln!("║    The Self-Imscribing Bare-Metal Kernel         ║");
    sprintln!("║    Frobenius Core · Belnap FOUR · Crystal FS     ║");
    sprintln!("╚══════════════════════════════════════════════════╝");
    sprintln!();
    sprintln!("Type 'help' for commands.");
    sprintln!();
}

// ─── REPL ─────────────────────────────────────────────────────

fn repl(k: &mut Kernel) {
    let mut cfs = CrystalStore::new();
    let mut line_buf = [0u8; 256];

    loop {
        serial::write_str("⊙> ");
        let line = read_line(&mut line_buf);
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
                let n: u64 = parts.next().and_then(|s| s.trim().parse().ok()).unwrap_or(1);
                k.run(n);
                print_status(k);
            }
            "program" => {
                for (i, t) in k.program.as_slice().iter().enumerate() {
                    if i > 0 { serial::write_str(" → "); }
                    serial::write_str(t.name());
                }
                sprintln!();
                sprintln!("len={} ip={}", k.program.len(), k.ip);
            }
            "snapshot" => {
                if let Some(snap) = k.snapshot {
                    sprintln!("Tier:     {}", snap.tier_name());
                    sprintln!("sig:      ({},{},{},{})", snap.sig.0, snap.sig.1, snap.sig.2, snap.sig.3);
                    sprintln!("diversity:{}/12", snap.token_diversity);
                    sprintln!("self_ref: {}", snap.self_ref);
                    sprintln!("frob_ord: {}", snap.frobenius_order);
                    sprintln!("dialeth:  {}", snap.dialetheia_complete);
                    sprintln!("period:   {}", snap.period);
                } else {
                    sprintln!("No snapshot — tick first.");
                }
            }
            "canonical" => {
                let arg = parts.next().unwrap_or("").trim();
                let idx = roman_to_idx(arg).or_else(|| arg.parse::<usize>().ok().map(|n| n.saturating_sub(1)));
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
            "crystal" => {
                let sub = parts.next().unwrap_or("").trim();
                match sub {
                    "store" => {
                        let name = parts.next().unwrap_or("").trim();
                        let data = parts.next().unwrap_or("").trim();
                        if name.is_empty() {
                            sprintln!("Usage: crystal store <name> [data]");
                        } else {
                            // hash name → canonical, load, tick, store
                            let idx = name_hash(name) % CANONICAL_COUNT;
                            k.load_canonical(idx);
                            k.tick();
                            let addr = crystal_store_current(k, &mut cfs, name, data, idx as u8);
                            sprintln!("  ↻ [{}] → tick {}", canonical_name(idx), k.tick_count);
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
                                sprintln!("  Stored: '{}' → '{}'", e.name_str(), e.data_str());
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

fn read_line<'a>(buf: &'a mut [u8; 256]) -> &'a str {
    let mut len = 0usize;
    loop {
        let b = serial::read_byte();
        match b {
            b'\r' | b'\n' => {
                serial::write_str("\n");
                break;
            }
            0x7f | 0x08 => { // backspace
                if len > 0 {
                    len -= 1;
                    serial::write_str("\x08 \x08");
                }
            }
            0x03 => { // Ctrl-C
                sprintln!();
                len = 0;
                break;
            }
            b if b >= 0x20 => {
                if len < buf.len() - 1 {
                    buf[len] = b;
                    len += 1;
                    serial::write_byte(b); // echo
                }
            }
            _ => {}
        }
    }
    buf[len] = 0;
    core::str::from_utf8(&buf[..len]).unwrap_or("")
}

// ─── Helpers ──────────────────────────────────────────────────

fn print_help() {
    sprintln!("mOMonadOS REPL commands:");
    sprintln!("  tick [N]              — run N ticks (default 1)");
    sprintln!("  run [N]               — run N more ticks");
    sprintln!("  status                — kernel status");
    sprintln!("  program               — show program");
    sprintln!("  snapshot              — structural snapshot");
    sprintln!("  canonical <I-XII>     — load canonical program");
    sprintln!("  crystal <addr>        — decode address");
    sprintln!("  crystal store <n> [d] — store entry (auto seq-swap+tick)");
    sprintln!("  crystal name <n>      — retrieve by name");
    sprintln!("  crystal find          — list all stored entries");
    sprintln!("  memory [start] [n]    — dump B4 memory");
    sprintln!("  registers             — show R0-R7");
    sprintln!("  stack                 — stack depth");
    sprintln!("  halt/quit             — exit");
}

fn print_status(k: &Kernel) {
    let tier = k.snapshot.map(|s| s.tier_name()).unwrap_or("?");
    sprintln!("╔══════════════════════════════════════╗");
    sprint!(  "║  Tick: {:8}  Cycle: {:8}    ║\n", k.tick_count, k.cycle_count);
    sprint!(  "║  Tier: {:<8}  IP: {:8}      ║\n", tier, k.ip);
    sprint!(  "║  Stack: {:6}   Frob: {}/{}        ║\n",
        k.stack.depth(), k.frob_checks - k.frob_open, k.frob_checks);
    serial::write_str("║  R0-R7: ");
    for i in 0..8 {
        serial::write_str(k.registers.read(i).name());
        serial::write_str(" ");
    }
    sprintln!("     ║");
    sprintln!("╚══════════════════════════════════════╝");
}

fn roman_to_idx(s: &str) -> Option<usize> {
    match s {
        "I"    => Some(0),  "II"   => Some(1),  "III" => Some(2),
        "IV"   => Some(3),  "V"    => Some(4),  "VI"  => Some(5),
        "VII"  => Some(6),  "VIII" => Some(7),  "IX"  => Some(8),
        "X"    => Some(9),  "XI"   => Some(10), "XII" => Some(11),
        _ => None,
    }
}

fn name_hash(name: &str) -> usize {
    // FNV-1a, good enough for canonical selection
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
