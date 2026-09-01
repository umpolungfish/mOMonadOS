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
const TENSOR_SRC: &str = r#"
extern "C" __global__ void tensor_chain(
    unsigned char *out_stage1, unsigned char *out_stage2, unsigned char *out_stage3,
    unsigned char *both_flags,
    const unsigned char *x, const unsigned char *y, const unsigned char *z,
    const unsigned long long n)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;

    // ⊢ void state for this thread's slot -- nothing read yet.
    unsigned char xv = x[i], yv = y[i], zv = z[i];

    // ── stage 1: union(x, y) ──
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
    out_stage1[i] = stage1;
    // ⊞ hold the B state where this result genuinely has both T and F.
    unsigned char stage1_both = ((stage1 & 1) && ((stage1>>1)&1)) ? 1 : 0;

    // ⋈ chain stage 1's output into stage 2's input.
    // ── stage 2: meet_t(stage1, z) ──
    unsigned char s1v_t = (stage1>>0)&1, s1v_f = (stage1>>1)&1, s1v_tt = (stage1>>2)&1, s1v_ff = (stage1>>3)&1;
    unsigned char z_t = (zv>>0)&1, z_f = (zv>>1)&1, z_tt = (zv>>2)&1, z_ff = (zv>>3)&1;
    unsigned char s2_t_arm = (s1v_t & z_t) | ((s1v_tt & z_tt) << 2);
    unsigned char s2_f_arm = (s1v_f | z_f) | ((s1v_ff | z_ff) << 2);
    unsigned char stage2 = (s2_t_arm & 0x5) | ((s2_f_arm & 0x5) << 1);
    out_stage2[i] = stage2;
    unsigned char stage2_both = ((stage2 & 1) && ((stage2>>1)&1)) ? 1 : 0;

    // ⋈ chain stage 2's output into stage 3's input.
    // ── stage 3: truth_swap(stage2) -- unary, a REAL lane permutation. ──
    unsigned char s2v_t = (stage2>>0)&1, s2v_f = (stage2>>1)&1, s2v_tt = (stage2>>2)&1, s2v_ff = (stage2>>3)&1;
    // ⊤/≻ T-arm: truth_swap's T lane comes from F, unchanged t lane.
    unsigned char s3_t_arm = s2v_f | (s2v_tt << 2);
    // ⊥/≺ F-arm: truth_swap's F lane comes from T -- the permutation this
    // step exists to name, not a no-op like stages 1 and 2.
    unsigned char s3_f_arm = s2v_t | (s2v_ff << 2);
    unsigned char stage3 = (s3_t_arm & 0x5) | ((s3_f_arm & 0x5) << 1);
    out_stage3[i] = stage3;
    unsigned char stage3_both = ((stage3 & 1) && ((stage3>>1)&1)) ? 1 : 0;

    both_flags[i] = stage1_both | (stage2_both << 1) | (stage3_both << 2);
    // ⊡/⊣ commit and close happen host-side on readback + stream sync.
}
"#;

fn pack(r: Reg16_3) -> u8 {
    (r.big_t as u8) | ((r.big_f as u8) << 1) | ((r.small_t as u8) << 2) | ((r.small_f as u8) << 3)
}
fn unpack(b: u8) -> Reg16_3 {
    Reg16_3 { big_t: b & 1 != 0, big_f: b & 2 != 0, small_t: b & 4 != 0, small_f: b & 8 != 0 }
}
struct Xorshift(u64);
impl Xorshift {
    fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.0 = x;
        (x & 0xF) as u8
    }
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
    let f = match module.load_function("tensor_chain") {
        Ok(f) => f,
        Err(e) => return format!("{out}  load tensor_chain failed: {e}\n"),
    };

    let t_gen = Instant::now();
    let mut rng = Xorshift(0xC2B2AE3D27D4EB4F ^ n.wrapping_mul(0x165667B19E3779F9));
    let xs_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let ys_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let zs_packed: Vec<u8> = (0..n).map(|_| rng.next_u8()).collect();
    let gen_elapsed = t_gen.elapsed();

    let t_htod = Instant::now();
    let d_x = match stream.clone_htod(&xs_packed) { Ok(d) => d, Err(e) => return format!("{out}  htod x: {e}") };
    let d_y = match stream.clone_htod(&ys_packed) { Ok(d) => d, Err(e) => return format!("{out}  htod y: {e}") };
    let d_z = match stream.clone_htod(&zs_packed) { Ok(d) => d, Err(e) => return format!("{out}  htod z: {e}") };
    let mut d_s1 = match stream.alloc_zeros::<u8>(n as usize) { Ok(d) => d, Err(e) => return format!("{out}  alloc s1: {e}") };
    let mut d_s2 = match stream.alloc_zeros::<u8>(n as usize) { Ok(d) => d, Err(e) => return format!("{out}  alloc s2: {e}") };
    let mut d_s3 = match stream.alloc_zeros::<u8>(n as usize) { Ok(d) => d, Err(e) => return format!("{out}  alloc s3: {e}") };
    let mut d_both = match stream.alloc_zeros::<u8>(n as usize) { Ok(d) => d, Err(e) => return format!("{out}  alloc both: {e}") };

    {
        let mut builder = stream.launch_builder(&f);
        builder.arg(&mut d_s1);
        builder.arg(&mut d_s2);
        builder.arg(&mut d_s3);
        builder.arg(&mut d_both);
        builder.arg(&d_x);
        builder.arg(&d_y);
        builder.arg(&d_z);
        builder.arg(&n);
        let cfg = LaunchConfig::for_num_elems(n as u32);
        if let Err(e) = unsafe { builder.launch(cfg) } {
            return format!("{out}  launch tensor_chain failed: {e}");
        }
    }
    // ⊣ close the kernel boundary, synchronize before reading anything back.
    if let Err(e) = stream.synchronize() {
        return format!("{out}  synchronize failed: {e}");
    }
    let htod_launch_elapsed = t_htod.elapsed();

    let t_dtoh = Instant::now();
    let gpu_s1: Vec<u8> = match stream.clone_dtoh(&d_s1) { Ok(v) => v, Err(e) => return format!("{out}  dtoh s1: {e}") };
    let gpu_s2: Vec<u8> = match stream.clone_dtoh(&d_s2) { Ok(v) => v, Err(e) => return format!("{out}  dtoh s2: {e}") };
    let gpu_s3: Vec<u8> = match stream.clone_dtoh(&d_s3) { Ok(v) => v, Err(e) => return format!("{out}  dtoh s3: {e}") };
    let gpu_both: Vec<u8> = match stream.clone_dtoh(&d_both) { Ok(v) => v, Err(e) => return format!("{out}  dtoh both: {e}") };
    let dtoh_elapsed = t_dtoh.elapsed();
    // ⊡ commit: the readback above IS the immutable record for this run.

    out.push_str("  ⊢∈⊤≻⊥≺∋⊞ per gate, ⋈ chaining, three stages, checked against the CPU at each stage\n\n");

    let t_verify = Instant::now();
    let mut mismatch_s1 = 0u64;
    let mut mismatch_s2 = 0u64;
    let mut mismatch_s3 = 0u64;
    let mut both_count = [0u64; 3];
    let mut self_ref_fail = 0u64;
    for i in 0..n as usize {
        let x = unpack(xs_packed[i]);
        let y = unpack(ys_packed[i]);
        let z = unpack(zs_packed[i]);

        let cpu_s1 = x.union(y);
        if pack(cpu_s1) != gpu_s1[i] { mismatch_s1 += 1; }

        let cpu_s2 = meet_t(cpu_s1, z);
        if pack(cpu_s2) != gpu_s2[i] { mismatch_s2 += 1; }

        let cpu_s3 = cpu_s2.truth_swap();
        if pack(cpu_s3) != gpu_s3[i] { mismatch_s3 += 1; }

        for stage in 0..3 {
            if (gpu_both[i] >> stage) & 1 == 1 { both_count[stage] += 1; }
        }

        // ⊙ self-reference check: truth_swap is its own inverse -- applying
        // it again to the GPU's stage 3 output must recover stage 2 exactly.
        let un_swapped = unpack(gpu_s3[i]).truth_swap();
        if pack(un_swapped) != gpu_s2[i] { self_ref_fail += 1; }
    }

    out.push_str(&format!(
        "  stage 1 union(x,y):        {n} checked, {mismatch_s1} mismatch(es) vs CPU\n"
    ));
    out.push_str(&format!(
        "  stage 2 meet_t(stage1,z):  {n} checked, {mismatch_s2} mismatch(es) vs CPU\n"
    ));
    out.push_str(&format!(
        "  stage 3 truth_swap(stage2): {n} checked, {mismatch_s3} mismatch(es) vs CPU\n"
    ));
    out.push_str(&format!(
        "  ⊞ B-state held (T and F both set): stage1 {} of {n}, stage2 {} of {n}, stage3 {} of {n}\n",
        both_count[0], both_count[1], both_count[2]
    ));
    out.push_str(&format!(
        "  ⊙ self-reference check (truth_swap is its own inverse): {self_ref_fail} of {n} failed to recover stage 2\n"
    ));

    let verify_elapsed = t_verify.elapsed();

    let total_mismatch = mismatch_s1 + mismatch_s2 + mismatch_s3 + self_ref_fail;
    out.push_str(&format!(
        "\n  {}\n",
        if total_mismatch == 0 {
            "all three chained stages match the CPU exactly, the self-reference check holds -- the split/rejoin/chain shape this ob3ect names, built and verified, not the flat expression already covered elsewhere"
        } else {
            "MISMATCH FOUND"
        }
    ));

    out.push_str(&format!(
        "\n  TIMING n={n} gen_ms={:.3} htod_launch_sync_ms={:.3} dtoh_ms={:.3} cpu_verify_ms={:.3} total_ms={:.3}\n",
        gen_elapsed.as_secs_f64() * 1000.0,
        htod_launch_elapsed.as_secs_f64() * 1000.0,
        dtoh_elapsed.as_secs_f64() * 1000.0,
        verify_elapsed.as_secs_f64() * 1000.0,
        (gen_elapsed + htod_launch_elapsed + dtoh_elapsed + verify_elapsed).as_secs_f64() * 1000.0,
    ));

    out
}
