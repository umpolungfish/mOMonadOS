//! gpu_dqi_xorsat.rs — GPU brute-force cross-check for the CPU
//! Gaussian-elimination XORSAT solver in dqi.rs.
//!
//! Follows gpu_sixteen3.rs's pattern: one CUDA kernel, JIT-compiled via
//! cudarc/NVRTC, launched over the full 2^m assignment space, each thread
//! checking one assignment against every clause (packed as a u64
//! variable-subset bitmask + rhs bit, m <= 30 here). This pushes
//! brute-force verification past what serial CPU brute force in dqi.rs
//! can reach (capped at m=24 there, to stay a benchmark rather than a
//! multi-hour loop) and cross-checks dqi::xorsat_solve's answer against
//! ground truth at that larger scale -- the same "run both, compare, zero
//! mismatches" discipline gpu_sixteen3.rs::verify already established for
//! the trilattice gates, applied here to the DQI solver.
//!
//! Hosted only: no CUDA driver in the bare-metal build.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

const KERNEL_SRC: &str = r#"
extern "C" __global__ void xorsat_brute(
    unsigned long long *found_mask, int *found_flag,
    const unsigned long long *clause_masks, const unsigned char *clause_rhs,
    const int num_clauses, const unsigned long long total)
{
    unsigned long long i = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= total) return;
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
    }
}
"#;

/// Brute-forces every one of the 2^num_vars assignments on GPU for the
/// given clause set (same (variable-index-subset, rhs) shape
/// `dqi::xorsat_solve` takes), then cross-checks the result directly
/// against the CPU elimination solver run on the identical instance.
/// Refuses num_vars outside 1..=30: above that a single kernel launch
/// stops being the right tool, and 0 has no assignments to brute force.
pub fn verify_against_cpu(clauses: &[(Vec<usize>, bool)], num_vars: usize, device: usize) -> String {
    if num_vars == 0 || num_vars > 30 {
        return format!(
            "gpu_dqi_xorsat: refusing num_vars={} (supported: 1..=30 — 2^30 assignments is already the practical ceiling for one launch)",
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

    // total <= 2^30 by the guard above, well inside u32 range: exact, not truncated.
    let cfg = LaunchConfig::for_num_elems(total as u32);
    let mut builder = stream.launch_builder(&f);
    builder.arg(&mut d_found_mask);
    builder.arg(&mut d_found_flag);
    builder.arg(&d_masks);
    builder.arg(&d_rhs);
    builder.arg(&num_clauses);
    builder.arg(&total);
    if let Err(e) = unsafe { builder.launch(cfg) } {
        return format!("gpu_dqi_xorsat: launch failed: {e}");
    }

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
        "  GPU brute force: {}\n",
        if gpu_sat { "SAT" } else { "UNSAT" }
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
