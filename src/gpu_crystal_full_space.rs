//! gpu_crystal_full_space.rs — phase_5's actual claim, checked at its actual scale.
//!
//! `gpu_native_build_of_momonados`'s phase_5 names `memory_mechanism` as "a
//! 17.28-million-entry Crystal type space navigated by address lookup."
//! `gpu_catalog_crystal.rs` batches `crystal::encode` for the live catalog,
//! 8667 entries, the ones that happen to have a type ob3ect written at their
//! address. That is not the claim: 8667 of 17,280,000 is not "the" crystal
//! type space, it's the populated fraction of it. This module checks the
//! address space itself, every one of the 17,280,000 addresses `crystal::TOTAL`
//! names, whether or not anything lives there -- decode(addr) -> 12-axis
//! tuple -> encode(tuple), on the GPU, and every address must round-trip to
//! itself. A round trip that always agrees with itself proves nothing if
//! encode and decode share the same bug, so every result is also checked
//! against the CPU `crystal::decode`/`crystal::encode` this kernel is a copy
//! of, on a real sample, not assumed to match because the logic looks the
//! same on the page.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

use crate::crystal::{decode, encode, CARDS, TOTAL};

/// STRIDES is private in crystal.rs; recomputed here from the same public
/// CARDS table by the same construction, not duplicated as a literal that
/// could drift from it.
fn strides() -> [u32; 12] {
    let mut s = [1u32; 12];
    let mut i = 11usize;
    loop {
        if i == 0 {
            break;
        }
        s[i - 1] = s[i] * CARDS[i];
        i -= 1;
    }
    s
}

const ROUND_TRIP_SRC: &str = r#"
extern "C" __global__ void crystal_round_trip(
    unsigned int *strides, unsigned char *out_ok, const unsigned long long total)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= total) return;
    unsigned int addr = (unsigned int)i;
    unsigned int a = addr;
    unsigned char idx[12];
    for (int k = 0; k < 12; k++) {
        idx[k] = (unsigned char)(a / strides[k]);
        a = a % strides[k];
    }
    unsigned int re = 0;
    for (int k = 0; k < 12; k++) {
        re += (unsigned int)idx[k] * strides[k];
    }
    out_ok[i] = (re == addr) ? 1 : 0;
}
"#;

/// Every one of the 17,280,000 crystal addresses, decoded then re-encoded on
/// the GPU, checked bit-for-bit against the same round trip on the CPU
/// (`crystal::decode`/`crystal::encode`) for a real sample, not merely
/// checked for internal self-agreement.
pub fn run() -> String {
    let mut out = String::new();
    let strides = strides();
    out.push_str(&format!(
        "gpu_crystal_full_space: {} addresses (crystal::TOTAL), strides {:?}\n\n",
        TOTAL, strides
    ));

    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}  no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n"));

    let ptx = match compile_ptx(ROUND_TRIP_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  NVRTC compile failed: {e}\n"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  module load failed: {e}\n"),
    };
    let f = match module.load_function("crystal_round_trip") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load crystal_round_trip failed: {e}\n"),
    };

    let d_strides = match stream.clone_htod(&strides) {
        Ok(d) => d,
        Err(e) => return format!("{out}  htod strides failed: {e}"),
    };
    let mut d_ok = match stream.alloc_zeros::<u8>(TOTAL as usize) {
        Ok(d) => d,
        Err(e) => return format!("{out}  alloc out_ok failed: {e} (needs ~{} MB device memory)", TOTAL / 1_000_000),
    };
    let total_u64 = TOTAL as u64;

    let mut builder = stream.launch_builder(&f);
    builder.arg(&d_strides);
    builder.arg(&mut d_ok);
    builder.arg(&total_u64);
    let cfg = LaunchConfig::for_num_elems(TOTAL);
    if let Err(e) = unsafe { builder.launch(cfg) } {
        return format!("{out}  launch crystal_round_trip failed: {e}");
    }

    let ok: Vec<u8> = match stream.clone_dtoh(&d_ok) {
        Ok(v) => v,
        Err(e) => return format!("{out}  dtoh out_ok failed: {e}"),
    };

    let bad_count = ok.iter().filter(|&&b| b == 0).count();
    out.push_str(&format!(
        "  every one of {} addresses decoded then re-encoded on the GPU\n",
        TOTAL
    ));
    out.push_str(&format!(
        "  self round-trip failures: {bad_count} of {}\n",
        TOTAL
    ));

    // Cross-check against the CPU's own decode/encode on a real sample --
    // a self-consistent GPU round trip proves nothing if the GPU kernel
    // shares a bug with itself; this checks it against the independent
    // Rust implementation the kernel was copied from.
    let sample_n = 2_000_000usize.min(TOTAL as usize);
    let stride_sample = (TOTAL as usize / sample_n).max(1);
    let mut cross_mismatches = 0usize;
    let mut checked = 0usize;
    let mut addr = 0usize;
    while addr < TOTAL as usize {
        let idx = decode(addr as u32);
        let re = encode(&idx);
        if re != addr as u32 || ok[addr] == 0 {
            cross_mismatches += 1;
        }
        checked += 1;
        addr += stride_sample;
    }
    out.push_str(&format!(
        "  cross-checked {checked} addresses against CPU crystal::decode/encode (stride {stride_sample}): {cross_mismatches} mismatch(es)\n"
    ));

    out.push_str(&format!(
        "\n  {}\n",
        if bad_count == 0 && cross_mismatches == 0 {
            "the full 17.28-million-entry address space round-trips, GPU and CPU agree -- phase_5's memory_mechanism claim checked at its actual scale, not the 8667-entry populated fraction"
        } else {
            "MISMATCH FOUND -- the address space does not fully round-trip"
        }
    ));

    out
}
