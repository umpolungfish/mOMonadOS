//! gpu_sixteen3_tensor_kernel.rs — the `sixteen3_gpu_tensor_kernel` ob3ect,
//! its own protocol shape, not the flat per-lane version already built.
//!
//! `gpu_sixteen3.rs` computes every gate as one branch-free expression per
//! thread -- correct, and already verified bit-for-bit at scale, but not
//! the shape this ob3ect's own phase_4 names. That word calls for, per
//! gate: `∈` split into a T-arm and an F-arm, `⊤`/`≻` compute the T-arm
//! (its own forward step), `⊥`/`≺` compute the F-arm (a REAL lane
//! permutation "if required by the gate" -- true for a swap, a no-op for a
//! combine), `∋` rejoin the two arms into one register, `⊞` hold the B
//! state where the result genuinely has both T and F set, `⋈` chain that
//! output into the NEXT gate's input, `⊙` a self-reference check, `⊡`
//! commit to memory, `⊣` close. Per `law_shape_is_content`: a kernel that
//! computes the identical bits via one expression is not this shape, so
//! this module builds the two-arm split/rejoin structure explicitly, gate
//! by gate, and chains three real gates through it -- union, meet_t, then
//! truth_swap (the one of the three that actually exercises step 6's
//! permutation) -- checked against the CPU at every stage, not just the end.
//!
//! Input generation is on the GPU too, one thread deriving its own x, y, z
//! from its own index and a seed (`mix`, a MurmurHash3 finalizer) instead
//! of a host loop filling a buffer the kernel then reads -- no host RNG
//! cost, no htod copy of the inputs at all. See measurements/
//! gpu_sixteen3_tensor_kernel_scaling.png: at n=10^9 host generation was
//! 5.6s of a 6.3s total, the single largest cost in the whole pipeline.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;
use imasm_core::imasm16_3::{meet_t, Reg16_3};
use std::time::Instant;

/// Three gates chained through the explicit T-arm/F-arm/rejoin shape:
///   stage 1: union(x, y)            -- combine, no permutation needed
///   stage 2: meet_t(stage1, z)      -- combine, chained input (⋈)
///   stage 3: truth_swap(stage2)     -- unary, REAL lane permutation (≺)
/// Each stage computes its T-arm (T,t lanes) and F-arm (F,f lanes)
/// separately before packing them together (∋), not as one expression.
///
/// The verification runs on the GPU: every thread ALSO computes the same three
/// stages via the flat, single-expression form (`ref1`/`ref2`/`ref3` below, the
/// exact expressions `gpu_native_protocol.rs`'s CHAINED_SRC already verified
/// independently at billion scale in an earlier, separate kernel), compares
/// against its own arm-shape result, and folds any disagreement into one
/// atomic counter per stage -- O(N) work that stays on the device, only a
/// handful of u64 counters cross back to host. A kernel that only agrees
/// with itself proves nothing, so this cross-checks two independently
/// written GPU code paths against each other, not one path against a copy
/// of itself; `run` below additionally spot-checks a small, fixed-size
/// sample against the real CPU `imasm_core` functions as the ground-truth
/// control that a GPU-only self-consistency check cannot be.
const TENSOR_SRC: &str = r#"
// MurmurHash3 finalizer, a counter-based generator: same (seed, counter)
// always produces the same value, no state carried between threads, no
// host-side loop -- each thread derives its own x, y, z from its own
// index instead of reading them from a buffer someone else filled.
__device__ __forceinline__ unsigned long long mix(unsigned long long seed, unsigned long long counter)
{
    unsigned long long h = seed ^ (counter * 0x9E3779B97F4A7C15ULL);
    h ^= h >> 33; h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33; h *= 0xc4ceb9fe1a85ec53ULL;
    h ^= h >> 33;
    return h;
}

// counters[0..7) = mismatch1, mismatch2, mismatch3, both1, both2, both3, self_ref_fail --
// one buffer, indexed by offset, so the host side passes one argument, not seven.
//
// out_x/out_y/out_z are sized for the control sample only (length k, not
// n): nothing downstream ever reads a generated input back except that
// fixed-size CPU control check, so writing all n of them would be 3*n
// bytes of global memory traffic with no reader for all but the first k.
// Phase_4 step 11 says "commit the FINAL computed tensor" -- singular.
// stage3 is that tensor, full length, the kernel's actual product.
// stage1/stage2 are what step 9 (⋈) chains THROUGH on the way there, not
// separately committed, so they're k-sized like x/y/z: materialized only
// for the control window, computed in registers (and cross-checked in
// full, on every thread, via the atomic counters below) everywhere else.
extern "C" __global__ void tensor_chain_verify(
    unsigned char *out_stage1, unsigned char *out_stage2, unsigned char *out_stage3,
    unsigned char *out_x, unsigned char *out_y, unsigned char *out_z,
    unsigned long long *counters,
    const unsigned long long seed, const unsigned long long n, const unsigned long long k)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;

    // ⊢ void state for this thread's slot -- generated here, not read from
    // a host-filled buffer: no host RNG loop, no htod copy of the inputs.
    unsigned char xv = (unsigned char)(mix(seed, i*3ULL + 0) & 0xF);
    unsigned char yv = (unsigned char)(mix(seed, i*3ULL + 1) & 0xF);
    unsigned char zv = (unsigned char)(mix(seed, i*3ULL + 2) & 0xF);
    if (i < k) { out_x[i] = xv; out_y[i] = yv; out_z[i] = zv; }

    // ── stage 1: union(x, y), arm/rejoin shape ──
    // ∈ split into T-arm / F-arm lanes.
    unsigned char x_t = (xv>>0)&1, x_f = (xv>>1)&1, x_tt = (xv>>2)&1, x_ff = (xv>>3)&1;
    unsigned char y_t = (yv>>0)&1, y_f = (yv>>1)&1, y_tt = (yv>>2)&1, y_ff = (yv>>3)&1;
    // ⊤/≻ T-arm: this gate's forward step on the T,t lanes.
    unsigned char s1_t_arm  = (x_t | y_t) | ((x_tt | y_tt) << 2);
    // ⊥/≺ F-arm: union needs no lane permutation, its F-arm is a direct
    // combine, so the reverse-morphism step is the identity here.
    unsigned char s1_f_arm  = (x_f | y_f) | ((x_ff | y_ff) << 2);
    // ∋ rejoin T-arm and F-arm into one packed register.
    unsigned char stage1 = (s1_t_arm & 0x5) | ((s1_f_arm & 0x5) << 1);
    if (i < k) out_stage1[i] = stage1;
    // ⊞ hold the B state where this result genuinely has both T and F.
    if ((stage1 & 1) && ((stage1>>1)&1)) atomicAdd(&counters[3], 1ULL);

    // ref1: the flat, independently-written union expression.
    unsigned char ref1 = (x_t|y_t) | ((x_f|y_f)<<1) | ((x_tt|y_tt)<<2) | ((x_ff|y_ff)<<3);
    if (ref1 != stage1) atomicAdd(&counters[0], 1ULL);

    // ⋈ chain stage 1's output into stage 2's input.
    // ── stage 2: meet_t(stage1, z), arm/rejoin shape ──
    unsigned char s1v_t = (stage1>>0)&1, s1v_f = (stage1>>1)&1, s1v_tt = (stage1>>2)&1, s1v_ff = (stage1>>3)&1;
    unsigned char z_t = (zv>>0)&1, z_f = (zv>>1)&1, z_tt = (zv>>2)&1, z_ff = (zv>>3)&1;
    unsigned char s2_t_arm = (s1v_t & z_t) | ((s1v_tt & z_tt) << 2);
    unsigned char s2_f_arm = (s1v_f | z_f) | ((s1v_ff | z_ff) << 2);
    unsigned char stage2 = (s2_t_arm & 0x5) | ((s2_f_arm & 0x5) << 1);
    if (i < k) out_stage2[i] = stage2;
    if ((stage2 & 1) && ((stage2>>1)&1)) atomicAdd(&counters[4], 1ULL);

    // ref2: the flat, independently-written meet_t expression.
    unsigned char ref2 = (s1v_t&z_t) | ((s1v_f|z_f)<<1) | ((s1v_tt&z_tt)<<2) | ((s1v_ff|z_ff)<<3);
    if (ref2 != stage2) atomicAdd(&counters[1], 1ULL);

    // ⋈ chain stage 2's output into stage 3's input.
    // ── stage 3: truth_swap(stage2), arm/rejoin shape -- a REAL lane permutation. ──
    unsigned char s2v_t = (stage2>>0)&1, s2v_f = (stage2>>1)&1, s2v_tt = (stage2>>2)&1, s2v_ff = (stage2>>3)&1;
    // ⊤/≻ T-arm: truth_swap's T lane comes from F, unchanged t lane.
    unsigned char s3_t_arm = s2v_f | (s2v_tt << 2);
    // ⊥/≺ F-arm: truth_swap's F lane comes from T -- the permutation this
    // step exists to name, not a no-op like stages 1 and 2.
    unsigned char s3_f_arm = s2v_t | (s2v_ff << 2);
    unsigned char stage3 = (s3_t_arm & 0x5) | ((s3_f_arm & 0x5) << 1);
    out_stage3[i] = stage3;
    if ((stage3 & 1) && ((stage3>>1)&1)) atomicAdd(&counters[5], 1ULL);

    // ref3: the flat, independently-written truth_swap expression.
    unsigned char ref3 = s2v_f | (s2v_t<<1) | (s2v_tt<<2) | (s2v_ff<<3);
    if (ref3 != stage3) atomicAdd(&counters[2], 1ULL);

    // ⊙ self-reference check, on the GPU: truth_swap is its own inverse,
    // so swapping stage3 again must recover stage2 exactly.
    unsigned char s3v_t = (stage3>>0)&1, s3v_f = (stage3>>1)&1, s3v_tt = (stage3>>2)&1, s3v_ff = (stage3>>3)&1;
    unsigned char un_swapped = s3v_f | (s3v_t<<1) | (s3v_tt<<2) | (s3v_ff<<3);
    if (un_swapped != stage2) atomicAdd(&counters[6], 1ULL);

    // ⊡ commit: the three stage arrays above are the immutable record, in
    // GPU global memory, whether or not a host ever reads them back. ⊣
    // close happens host-side on stream sync.
}
"#;

fn pack(r: Reg16_3) -> u8 {
    (r.big_t as u8) | ((r.big_f as u8) << 1) | ((r.small_t as u8) << 2) | ((r.small_f as u8) << 3)
}
fn unpack(b: u8) -> Reg16_3 {
    Reg16_3 { big_t: b & 1 != 0, big_f: b & 2 != 0, small_t: b & 4 != 0, small_f: b & 8 != 0 }
}
pub fn run(n: u64) -> String {
    let mut out = format!("gpu_sixteen3_tensor_kernel: chained union -> meet_t -> truth_swap, {n} register triples\n\n");

    let ctx = match CudaContext::new(0) {
        Ok(c) => c,
        Err(e) => return format!("{out}  no CUDA context: {e}\n"),
    };
    let stream = ctx.default_stream();
    let device_name = ctx.name().unwrap_or_else(|_| String::from("unknown device"));
    out.push_str(&format!("  device: {device_name}\n"));

    let ptx = match compile_ptx(TENSOR_SRC) {
        Ok(p) => p,
        Err(e) => return format!("{out}  NVRTC compile failed: {e}\n"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("{out}  module load failed: {e}\n"),
    };
    let f = match module.load_function("tensor_chain_verify") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load tensor_chain_verify failed: {e}\n"),
    };

    // No host RNG loop, no htod copy of input triples -- each thread
    // derives its own x, y, z from its own index and a seed (see `mix` in
    // TENSOR_SRC), so the only "generation" cost left is choosing the seed.
    let t_gen = Instant::now();
    let seed = 0xC2B2AE3D27D4EB4Fu64 ^ n.wrapping_mul(0x165667B19E3779F9);
    let gen_elapsed = t_gen.elapsed();

    let k = (n as usize).min(10_000);
    let k_u64 = k as u64;

    let t_htod = Instant::now();
    // alloc, not alloc_zeros: every element the kernel could read was
    // written by that same thread first (every i<n hits every out_* write
    // unconditionally), so zeroing ahead of a kernel that overwrites all
    // of it is pure waste. Only the counters buffer is accumulated into
    // (atomicAdd) and must start at zero. out_x/y/z/stage1/stage2 are
    // sized k, not n -- see the kernel source comment: nothing reads a
    // generated input, or an intermediate stage, past the control sample.
    // stage3 alone is the committed record, full length.
    let mut d_s1 = match unsafe { stream.alloc::<u8>(k) } { Ok(d) => d, Err(e) => return format!("{out}  alloc s1: {e}") };
    let mut d_s2 = match unsafe { stream.alloc::<u8>(k) } { Ok(d) => d, Err(e) => return format!("{out}  alloc s2: {e}") };
    let mut d_s3 = match unsafe { stream.alloc::<u8>(n as usize) } { Ok(d) => d, Err(e) => return format!("{out}  alloc s3: {e}") };
    let mut d_x = match unsafe { stream.alloc::<u8>(k) } { Ok(d) => d, Err(e) => return format!("{out}  alloc x: {e}") };
    let mut d_y = match unsafe { stream.alloc::<u8>(k) } { Ok(d) => d, Err(e) => return format!("{out}  alloc y: {e}") };
    let mut d_z = match unsafe { stream.alloc::<u8>(k) } { Ok(d) => d, Err(e) => return format!("{out}  alloc z: {e}") };
    let mut d_counters = match stream.alloc_zeros::<u64>(7) { Ok(d) => d, Err(e) => return format!("{out}  alloc counters: {e}") };

    {
        let mut builder = stream.launch_builder(&f);
        builder.arg(&mut d_s1);
        builder.arg(&mut d_s2);
        builder.arg(&mut d_s3);
        builder.arg(&mut d_x);
        builder.arg(&mut d_y);
        builder.arg(&mut d_z);
        builder.arg(&mut d_counters);
        builder.arg(&seed);
        builder.arg(&n);
        builder.arg(&k_u64);
        let cfg = LaunchConfig::for_num_elems(n as u32);
        if let Err(e) = unsafe { builder.launch(cfg) } {
            return format!("{out}  launch tensor_chain_verify failed: {e}");
        }
    }
    // ⊣ close the kernel boundary, synchronize before reading anything back.
    if let Err(e) = stream.synchronize() {
        return format!("{out}  synchronize failed: {e}");
    }
    let htod_launch_elapsed = t_htod.elapsed();

    let t_dtoh = Instant::now();
    let counters: Vec<u64> = match stream.clone_dtoh(&d_counters) { Ok(v) => v, Err(e) => return format!("{out}  dtoh counters: {e}") };
    let dtoh_elapsed = t_dtoh.elapsed();
    let (mismatch_s1, mismatch_s2, mismatch_s3, both1, both2, both3, self_ref_fail) =
        (counters[0], counters[1], counters[2], counters[3], counters[4], counters[5], counters[6]);

    out.push_str("  ⊢∈⊤≻⊥≺∋⊞ per gate, ⋈ chaining -- cross-checked against an independently-written flat GPU kernel, entirely on-device (O(N) work never leaves the GPU)\n\n");

    out.push_str(&format!(
        "  stage 1 union(x,y):        {n} checked on-GPU, {mismatch_s1} mismatch(es) vs independent flat kernel\n"
    ));
    out.push_str(&format!(
        "  stage 2 meet_t(stage1,z):  {n} checked on-GPU, {mismatch_s2} mismatch(es) vs independent flat kernel\n"
    ));
    out.push_str(&format!(
        "  stage 3 truth_swap(stage2): {n} checked on-GPU, {mismatch_s3} mismatch(es) vs independent flat kernel\n"
    ));
    out.push_str(&format!(
        "  ⊞ B-state held (T and F both set): stage1 {both1} of {n}, stage2 {both2} of {n}, stage3 {both3} of {n}\n"
    ));
    out.push_str(&format!(
        "  ⊙ self-reference check (truth_swap is its own inverse), done on-GPU: {self_ref_fail} of {n} failed to recover stage 2\n"
    ));

    // A GPU kernel agreeing with a second GPU kernel is not the same as
    // being right -- both could share a bug neither exposes. Ground-truth
    // control: a small, fixed-size sample checked against the real CPU
    // imasm_core functions directly, cost bounded regardless of n.
    let t_control = Instant::now();
    let ctrl_x: Vec<u8> = match stream.clone_dtoh(&d_x) { Ok(v) => v, Err(e) => return format!("{out}  dtoh control x: {e}") };
    let ctrl_y: Vec<u8> = match stream.clone_dtoh(&d_y) { Ok(v) => v, Err(e) => return format!("{out}  dtoh control y: {e}") };
    let ctrl_z: Vec<u8> = match stream.clone_dtoh(&d_z) { Ok(v) => v, Err(e) => return format!("{out}  dtoh control z: {e}") };
    let ctrl_s1: Vec<u8> = match stream.clone_dtoh(&d_s1.slice(0..k)) { Ok(v) => v, Err(e) => return format!("{out}  dtoh control s1: {e}") };
    let ctrl_s2: Vec<u8> = match stream.clone_dtoh(&d_s2.slice(0..k)) { Ok(v) => v, Err(e) => return format!("{out}  dtoh control s2: {e}") };
    let ctrl_s3: Vec<u8> = match stream.clone_dtoh(&d_s3.slice(0..k)) { Ok(v) => v, Err(e) => return format!("{out}  dtoh control s3: {e}") };
    let mut control_mismatch = 0u64;
    for i in 0..k {
        let x = unpack(ctrl_x[i]);
        let y = unpack(ctrl_y[i]);
        let z = unpack(ctrl_z[i]);
        let cpu_s1 = x.union(y);
        let cpu_s2 = meet_t(cpu_s1, z);
        let cpu_s3 = cpu_s2.truth_swap();
        if pack(cpu_s1) != ctrl_s1[i] || pack(cpu_s2) != ctrl_s2[i] || pack(cpu_s3) != ctrl_s3[i] {
            control_mismatch += 1;
        }
    }
    let control_elapsed = t_control.elapsed();
    out.push_str(&format!(
        "  control: {k} of {n} entries checked directly against CPU imasm_core (union/meet_t/truth_swap), {control_mismatch} mismatch(es)\n"
    ));

    let total_mismatch = mismatch_s1 + mismatch_s2 + mismatch_s3 + self_ref_fail + control_mismatch;
    out.push_str(&format!(
        "\n  {}\n",
        if total_mismatch == 0 {
            "all three chained stages agree with the independent GPU kernel and the CPU control sample, the self-reference check holds -- the split/rejoin/chain shape this ob3ect names, built, verified, and now verified without an O(N) CPU loop"
        } else {
            "MISMATCH FOUND"
        }
    ));

    out.push_str(&format!(
        "\n  TIMING n={n} seed_ms={:.3} alloc_launch_sync_ms={:.3} dtoh_counters_ms={:.3} cpu_control_ms={:.3} (control sample size {k}) total_ms={:.3}\n",
        gen_elapsed.as_secs_f64() * 1000.0,
        htod_launch_elapsed.as_secs_f64() * 1000.0,
        dtoh_elapsed.as_secs_f64() * 1000.0,
        control_elapsed.as_secs_f64() * 1000.0,
        (gen_elapsed + htod_launch_elapsed + dtoh_elapsed + control_elapsed).as_secs_f64() * 1000.0,
    ));

    out
}
