//! gpu_sixteen3.rs — the SIXTEEN_3 trilattice's register gates, batched on GPU.
//!
//! ob3ect: sixteen3_gpu_tensor_kernel
//! (ob3ect/digital/sixteen3_gpu_tensor_kernel/sixteen3_gpu_tensor_kernel_ob3ect.json)
//!
//! `imasm_core::imasm16_3::Reg16_3` is a plain 4-bool tuple over the lanes
//! [T, F, t, f], and every lattice op on it (`union`, `meet_t`, `join_t`,
//! `meet_c`, `join_c`, `leq_i`, `leq_t`, `leq_c`, `truth_swap`, `info_swap`,
//! `invol`) is a fixed, data-independent per-lane bitwise op or permutation
//! -- never a branch on the value's content. That is a fixed-width
//! elementwise/permute kernel over an (N,4) boolean tensor, which is what
//! this module actually runs on GPU, verified bit-for-bit against the CPU
//! implementation it is a batched port of, not a reimplementation with its
//! own independent logic.
//!
//! Hosted only: no_std has no CUDA driver to link against. The bare-metal
//! build never sees this module at all (see main.rs's tree).

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use imasm_core::imasm16_3::{
    join_c, join_t, leq_c, leq_i, leq_t, meet_c, meet_t, Reg16_3,
};

/// One CUDA source, three kernels: binary lattice ops, unary swaps, and the
/// leq_* order checks. All three read/write a register packed as one byte,
/// bit 0..3 = T,F,t,f -- the same lane order `Reg16_3` uses, just packed.
const KERNEL_SRC: &str = r#"
extern "C" __global__ void sixteen3_binary(
    unsigned char *out, const unsigned char *x, const unsigned char *y,
    const int op, const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    unsigned char xv = x[i], yv = y[i];
    unsigned char xt=(xv>>0)&1, xf=(xv>>1)&1, xtt=(xv>>2)&1, xff=(xv>>3)&1;
    unsigned char yt=(yv>>0)&1, yf=(yv>>1)&1, ytt=(yv>>2)&1, yff=(yv>>3)&1;
    unsigned char rt, rf, rtt, rff;
    if (op == 0) { // union: every lane OR
        rt = xt|yt; rf = xf|yf; rtt = xtt|ytt; rff = xff|yff;
    } else if (op == 1) { // meet_t: T,t AND; F,f OR
        rt = xt&yt; rtt = xtt&ytt; rf = xf|yf; rff = xff|yff;
    } else if (op == 2) { // join_t: T,t OR; F,f AND
        rt = xt|yt; rtt = xtt|ytt; rf = xf&yf; rff = xff&yff;
    } else if (op == 3) { // meet_c: T,F AND; t,f OR
        rt = xt&yt; rf = xf&yf; rtt = xtt|ytt; rff = xff|yff;
    } else { // op == 4, join_c: T,F OR; t,f AND
        rt = xt|yt; rf = xf|yf; rtt = xtt&ytt; rff = xff&yff;
    }
    out[i] = rt | (rf<<1) | (rtt<<2) | (rff<<3);
}

extern "C" __global__ void sixteen3_unary(
    unsigned char *out, const unsigned char *x, const int op, const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    unsigned char xv = x[i];
    unsigned char xt=(xv>>0)&1, xf=(xv>>1)&1, xtt=(xv>>2)&1, xff=(xv>>3)&1;
    unsigned char rt, rf, rtt, rff;
    if (op == 0) { rt=xf; rf=xt; rtt=xtt; rff=xff; }       // truth_swap
    else if (op == 1) { rt=xt; rf=xf; rtt=xff; rff=xtt; }  // info_swap
    else { rt=xf; rf=xt; rtt=xff; rff=xtt; }                // invol
    out[i] = rt | (rf<<1) | (rtt<<2) | (rff<<3);
}

extern "C" __global__ void sixteen3_leq(
    unsigned char *out, const unsigned char *x, const unsigned char *y,
    const int op, const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    unsigned char xv = x[i], yv = y[i];
    unsigned char xt=(xv>>0)&1, xf=(xv>>1)&1, xtt=(xv>>2)&1, xff=(xv>>3)&1;
    unsigned char yt=(yv>>0)&1, yf=(yv>>1)&1, ytt=(yv>>2)&1, yff=(yv>>3)&1;
    unsigned char r;
    if (op == 0) { // leq_i: subset on all four lanes
        r = (!xt||yt) && (!xf||yf) && (!xtt||ytt) && (!xff||yff);
    } else if (op == 1) { // leq_t: T,t grow; F,f shrink
        unsigned char pos_ok = (!xt||yt) && (!xtt||ytt);
        unsigned char neg_ok = (!yf||xf) && (!yff||xff);
        r = pos_ok && neg_ok;
    } else { // leq_c: T,F grow; t,f shrink
        unsigned char con_ok = (!xt||yt) && (!xf||yf);
        unsigned char noncon_ok = (!ytt||xtt) && (!yff||xff);
        r = con_ok && noncon_ok;
    }
    out[i] = r;
}
"#;

fn pack(r: Reg16_3) -> u8 {
    (r.big_t as u8) | ((r.big_f as u8) << 1) | ((r.small_t as u8) << 2) | ((r.small_f as u8) << 3)
}

fn unpack(b: u8) -> Reg16_3 {
    Reg16_3 {
        big_t: b & 1 != 0,
        big_f: b & 2 != 0,
        small_t: b & 4 != 0,
        small_f: b & 8 != 0,
    }
}

/// A tiny xorshift PRNG -- no external `rand` dependency needed for a
/// verification sweep whose only requirement is "cover the space", not
/// cryptographic quality.
struct Xorshift(u64);
impl Xorshift {
    fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x & 0xF) as u8
    }
}

/// Runs all eleven Reg16_3 gates (5 binary, 3 unary, 3 leq) on `n` random
/// register pairs, once on the GPU (one kernel launch per gate, the whole
/// batch at once) and once on the CPU via `imasm_core::imasm16_3`'s own
/// scalar functions, and reports where -- if anywhere -- they disagree.
/// This is the check, not a demonstration: a batched port that has not
/// been run against the implementation it claims to match is a claim, not
/// a result.
pub fn verify(n: usize, device: usize) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c,
        Err(e) => return format!("gpu16_3 verify: no CUDA context (device {device}): {e}"),
    };
    let stream = ctx.default_stream();

    let ptx = match compile_ptx(KERNEL_SRC) {
        Ok(p) => p,
        Err(e) => return format!("gpu16_3 verify: NVRTC compile failed: {e}"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("gpu16_3 verify: module load failed: {e}"),
    };
    let f_bin = match module.load_function("sixteen3_binary") {
        Ok(f) => f,
        Err(e) => return format!("gpu16_3 verify: load sixteen3_binary failed: {e}"),
    };
    let f_un = match module.load_function("sixteen3_unary") {
        Ok(f) => f,
        Err(e) => return format!("gpu16_3 verify: load sixteen3_unary failed: {e}"),
    };
    let f_leq = match module.load_function("sixteen3_leq") {
        Ok(f) => f,
        Err(e) => return format!("gpu16_3 verify: load sixteen3_leq failed: {e}"),
    };

    let mut rng = Xorshift(0x9E3779B97F4A7C15 ^ (n as u64).wrapping_mul(2654435761));
    let xs_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let ys_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let xs: Vec<Reg16_3> = xs_packed.iter().map(|&b| unpack(b)).collect();
    let ys: Vec<Reg16_3> = ys_packed.iter().map(|&b| unpack(b)).collect();

    let d_xs = match stream.clone_htod(&xs_packed) {
        Ok(d) => d,
        Err(e) => return format!("gpu16_3 verify: htod x failed: {e}"),
    };
    let d_ys = match stream.clone_htod(&ys_packed) {
        Ok(d) => d,
        Err(e) => return format!("gpu16_3 verify: htod y failed: {e}"),
    };
    let cfg = LaunchConfig::for_num_elems(n as u32);
    let n_u64 = n as u64;

    let mut report = String::new();
    let mut total_mismatches: usize = 0;
    let mut total_checks: usize = 0;

    // Binary gates: op 0..4 = union, meet_t, join_t, meet_c, join_c.
    let binary_names = ["union", "meet_t", "join_t", "meet_c", "join_c"];
    for (op, name) in binary_names.iter().enumerate() {
        let mut d_out = match stream.alloc_zeros::<u8>(n) {
            Ok(d) => d,
            Err(e) => return format!("gpu16_3 verify: alloc out ({name}) failed: {e}"),
        };
        let op_i32 = op as i32;
        let mut builder = stream.launch_builder(&f_bin);
        builder.arg(&mut d_out);
        builder.arg(&d_xs);
        builder.arg(&d_ys);
        builder.arg(&op_i32);
        builder.arg(&n_u64);
        if let Err(e) = unsafe { builder.launch(cfg) } {
            return format!("gpu16_3 verify: launch {name} failed: {e}");
        }
        let gpu_out: Vec<u8> = match stream.clone_dtoh(&d_out) {
            Ok(v) => v,
            Err(e) => return format!("gpu16_3 verify: dtoh {name} failed: {e}"),
        };
        let mismatches = (0..n)
            .filter(|&i| {
                let cpu = match *name {
                    "union" => xs[i].union(ys[i]),
                    "meet_t" => meet_t(xs[i], ys[i]),
                    "join_t" => join_t(xs[i], ys[i]),
                    "meet_c" => meet_c(xs[i], ys[i]),
                    "join_c" => join_c(xs[i], ys[i]),
                    _ => unreachable!(),
                };
                pack(cpu) != gpu_out[i]
            })
            .count();
        total_checks += n;
        total_mismatches += mismatches;
        report.push_str(&format!("  {name:<8} {n} pairs, {mismatches} mismatch(es)\n"));
    }

    // Unary swaps: op 0..2 = truth_swap, info_swap, invol.
    let unary_names = ["truth_swap", "info_swap", "invol"];
    for (op, name) in unary_names.iter().enumerate() {
        let mut d_out = match stream.alloc_zeros::<u8>(n) {
            Ok(d) => d,
            Err(e) => return format!("gpu16_3 verify: alloc out ({name}) failed: {e}"),
        };
        let op_i32 = op as i32;
        let mut builder = stream.launch_builder(&f_un);
        builder.arg(&mut d_out);
        builder.arg(&d_xs);
        builder.arg(&op_i32);
        builder.arg(&n_u64);
        if let Err(e) = unsafe { builder.launch(cfg) } {
            return format!("gpu16_3 verify: launch {name} failed: {e}");
        }
        let gpu_out: Vec<u8> = match stream.clone_dtoh(&d_out) {
            Ok(v) => v,
            Err(e) => return format!("gpu16_3 verify: dtoh {name} failed: {e}"),
        };
        let mismatches = (0..n)
            .filter(|&i| {
                let cpu = match *name {
                    "truth_swap" => xs[i].truth_swap(),
                    "info_swap" => xs[i].info_swap(),
                    "invol" => xs[i].invol(),
                    _ => unreachable!(),
                };
                pack(cpu) != gpu_out[i]
            })
            .count();
        total_checks += n;
        total_mismatches += mismatches;
        report.push_str(&format!("  {name:<8} {n} values, {mismatches} mismatch(es)\n"));
    }

    // Order checks: op 0..2 = leq_i, leq_t, leq_c.
    let leq_names = ["leq_i", "leq_t", "leq_c"];
    for (op, name) in leq_names.iter().enumerate() {
        let mut d_out = match stream.alloc_zeros::<u8>(n) {
            Ok(d) => d,
            Err(e) => return format!("gpu16_3 verify: alloc out ({name}) failed: {e}"),
        };
        let op_i32 = op as i32;
        let mut builder = stream.launch_builder(&f_leq);
        builder.arg(&mut d_out);
        builder.arg(&d_xs);
        builder.arg(&d_ys);
        builder.arg(&op_i32);
        builder.arg(&n_u64);
        if let Err(e) = unsafe { builder.launch(cfg) } {
            return format!("gpu16_3 verify: launch {name} failed: {e}");
        }
        let gpu_out: Vec<u8> = match stream.clone_dtoh(&d_out) {
            Ok(v) => v,
            Err(e) => return format!("gpu16_3 verify: dtoh {name} failed: {e}"),
        };
        let mismatches = (0..n)
            .filter(|&i| {
                let cpu = match *name {
                    "leq_i" => leq_i(xs[i], ys[i]),
                    "leq_t" => leq_t(xs[i], ys[i]),
                    "leq_c" => leq_c(xs[i], ys[i]),
                    _ => unreachable!(),
                };
                (cpu as u8) != gpu_out[i]
            })
            .count();
        total_checks += n;
        total_mismatches += mismatches;
        report.push_str(&format!("  {name:<8} {n} pairs, {mismatches} mismatch(es)\n"));
    }

    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    format!(
        "gpu16_3 verify: device {device_name}, {n} registers/pair, 11 gates\n{report}\
         total: {total_checks} checks, {total_mismatches} mismatch(es) -- {}\n",
        if total_mismatches == 0 { "GPU batch matches CPU scalar exactly" } else { "MISMATCH FOUND" }
    )
}
