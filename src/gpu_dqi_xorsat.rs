//! gpu_dqi_xorsat.rs — GPU brute-force cross-check for the CPU
//! Gaussian-elimination XORSAT solver in dqi.rs.
//!
//! Follows gpu_sixteen3.rs's pattern: one CUDA kernel, JIT-compiled via
//! cudarc/NVRTC, launched over a fixed grid that grid-strides across the
//! full 2^m assignment space, each thread checking a share of the
//! assignments against every clause (packed as a u64 variable-subset
//! bitmask + rhs bit). This pushes brute-force verification past what
//! serial CPU brute force in dqi.rs can reach (capped at m=24 there, to
//! stay a benchmark rather than a multi-hour loop) and cross-checks
//! dqi::xorsat_solve's answer against ground truth at that larger scale --
//! the same "run both, compare, zero mismatches" discipline
//! gpu_sixteen3.rs::verify already established for the trilattice gates,
//! applied here to the DQI solver.
//!
//! Hosted only: no CUDA driver in the bare-metal build.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

// Grid-stride loop, not one thread per assignment: `total` (2^num_vars)
// is a u64 and is never narrowed to fit a launch-config dimension, so
// there is no size at which the grid silently under-covers the space.
// Earlier version launched `LaunchConfig::for_num_elems(total as u32)`,
// which truncates any `total` that doesn't fit in a u32 -- at num_vars=32,
// `total as u32` wraps to 0 and the kernel launches zero threads, so it
// would have silently reported UNSAT for every instance from there up
// instead of erroring. Caught by checking why the num_vars<=30 refusal
// was there at all, rather than just re-asserting a number chosen by
// caution the first time through.
const KERNEL_SRC: &str = r#"
extern "C" __global__ void xorsat_brute(
    unsigned long long *found_mask, int *found_flag,
    const unsigned long long *clause_masks, const unsigned char *clause_rhs,
    const int num_clauses, const unsigned long long total)
{
    unsigned long long stride = (unsigned long long)gridDim.x * blockDim.x;
    for (unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
         i < total; i += stride) {
        if (*found_flag) return; // racy early-out is fine: correctness comes from the atomic below
        bool ok = true;
        for (int c = 0; c < num_clauses; c++) {
            unsigned long long parity = __popcll(i & clause_masks[c]) & 1ULL;
            bool want = clause_rhs[c] != 0;
            if ((parity != 0) != want) { ok = false; break; }
        }
        if (ok) {
            int prev = atomicExch(found_flag, 1);
            if (prev == 0) {
                *found_mask = i;
            }
            return;
        }
    }
}
"#;

/// Brute-forces every one of the 2^num_vars assignments on GPU for the
/// given clause set (same (variable-index-subset, rhs) shape
/// `dqi::xorsat_solve` takes), then cross-checks the result directly
/// against the CPU elimination solver run on the identical instance.
/// Refuses num_vars outside 1..=40: below 1 there is nothing to brute
/// force; above 40 (2^40, over a trillion assignments) a forced-UNSAT
/// instance -- the case with no early exit -- runs long enough on this
/// GPU that the command stops being a benchmark and starts being a wait.
/// That ceiling is a measured runtime choice now, not a launch-config
/// size limit: the grid-stride kernel has no architectural wall.
pub fn verify_against_cpu(clauses: &[(Vec<usize>, bool)], num_vars: usize, device: usize) -> String {
    if num_vars == 0 || num_vars > 40 {
        return format!(
            "gpu_dqi_xorsat: refusing num_vars={} (supported: 1..=40, a measured runtime ceiling, not an architectural one)",
            num_vars
        );
    }
    let ctx = match CudaContext::new(device) {
        Ok(c) => c,
        Err(e) => return format!("gpu_dqi_xorsat: no CUDA context (device {device}): {e}"),
    };
    let stream = ctx.default_stream();
    let ptx = match compile_ptx(KERNEL_SRC) {
        Ok(p) => p,
        Err(e) => return format!("gpu_dqi_xorsat: NVRTC compile failed: {e}"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m,
        Err(e) => return format!("gpu_dqi_xorsat: module load failed: {e}"),
    };
    let f = match module.load_function("xorsat_brute") {
        Ok(f) => f,
        Err(e) => return format!("gpu_dqi_xorsat: load xorsat_brute failed: {e}"),
    };

    let clause_masks: Vec<u64> = clauses
        .iter()
        .map(|(vars, _)| vars.iter().fold(0u64, |acc, &v| acc | (1u64 << v)))
        .collect();
    let clause_rhs: Vec<u8> = clauses.iter().map(|(_, rhs)| *rhs as u8).collect();
    let num_clauses = clauses.len() as i32;
    let total: u64 = 1u64 << num_vars;

    let d_masks = match stream.clone_htod(&clause_masks) {
        Ok(d) => d,
        Err(e) => return format!("gpu_dqi_xorsat: htod masks failed: {e}"),
    };
    let d_rhs = match stream.clone_htod(&clause_rhs) {
        Ok(d) => d,
        Err(e) => return format!("gpu_dqi_xorsat: htod rhs failed: {e}"),
    };
    let mut d_found_mask = match stream.alloc_zeros::<u64>(1) {
        Ok(d) => d,
        Err(e) => return format!("gpu_dqi_xorsat: alloc found_mask failed: {e}"),
    };
    let mut d_found_flag = match stream.alloc_zeros::<i32>(1) {
        Ok(d) => d,
        Err(e) => return format!("gpu_dqi_xorsat: alloc found_flag failed: {e}"),
    };

    // Fixed grid, independent of `total`: the kernel's own stride loop
    // covers however many assignments there are, so the launch shape
    // doesn't need to scale with (or be limited by) num_vars at all.
    let cfg = LaunchConfig {
        grid_dim: (65536, 1, 1),
        block_dim: (256, 1, 1),
        shared_mem_bytes: 0,
    };
    let mut builder = stream.launch_builder(&f);
    builder.arg(&mut d_found_mask);
    builder.arg(&mut d_found_flag);
    builder.arg(&d_masks);
    builder.arg(&d_rhs);
    builder.arg(&num_clauses);
    builder.arg(&total);
    let t0 = std::time::Instant::now();
    if let Err(e) = unsafe { builder.launch(cfg) } {
        return format!("gpu_dqi_xorsat: launch failed: {e}");
    }
    if let Err(e) = stream.synchronize() {
        return format!("gpu_dqi_xorsat: synchronize failed: {e}");
    }
    let kernel_micros = t0.elapsed().as_micros();

    let found_flag: Vec<i32> = match stream.clone_dtoh(&d_found_flag) {
        Ok(v) => v,
        Err(e) => return format!("gpu_dqi_xorsat: dtoh flag failed: {e}"),
    };
    let found_mask: Vec<u64> = match stream.clone_dtoh(&d_found_mask) {
        Ok(v) => v,
        Err(e) => return format!("gpu_dqi_xorsat: dtoh mask failed: {e}"),
    };
    let gpu_sat = found_flag[0] != 0;

    // Cross-check against the CPU elimination solver on the identical instance.
    let cpu = crate::dqi::xorsat_solve(clauses, num_vars);
    let cpu_sat = cpu.is_some();

    let mut out = String::new();
    out.push_str(&format!(
        "gpu_dqi_xorsat: {} variables, {} clauses, {} assignments brute-forced on GPU\n",
        num_vars,
        clauses.len(),
        total
    ));
    out.push_str(&format!(
        "  GPU brute force: {}  ({} us kernel time)\n",
        if gpu_sat { "SAT" } else { "UNSAT" },
        kernel_micros
    ));
    out.push_str(&format!(
        "  CPU elimination: {}\n",
        if cpu_sat { "SAT" } else { "UNSAT" }
    ));
    if gpu_sat {
        let gpu_assignment: Vec<bool> = (0..num_vars)
            .map(|i| (found_mask[0] >> i) & 1 == 1)
            .collect();
        let gpu_ok = clauses.iter().all(|(vars, rhs)| {
            vars.iter().fold(false, |acc, &v| acc ^ gpu_assignment[v]) == *rhs
        });
        out.push_str(&format!(
            "  GPU-found assignment verified against every clause directly: {}\n",
            gpu_ok
        ));
    }
    let agree = gpu_sat == cpu_sat;
    out.push_str(&format!(
        "  agreement: {}\n",
        if agree { "MATCH" } else { "MISMATCH — real bug if this ever prints" }
    ));
    out
}
