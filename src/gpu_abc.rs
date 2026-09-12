//! Batched ABC observable evaluation on CUDA.
//!
//! Triple enumeration and radical construction stay on the reference path;
//! this kernel evaluates the logarithmic observable and writes the native
//! trilattice `B` tag beside every sample. That separation keeps arithmetic
//! exactness in the host certificate while moving the dense floating-point
//! reading to the device.

use alloc::format;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::compile_ptx;

const KERNEL: &str = r#"
extern "C" __global__ void abc_observable(const unsigned long long* c,
    const unsigned long long* rad, double eps, double* out,
    double* packet_rad, double* packet_height, unsigned char* state, unsigned int n) {
  unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
  if (i >= n) return;
  double h = log((double)c[i]);
  double r = log((double)rad[i]);
  out[i] = h - (1.0 + eps) * r;
  packet_rad[i] = r;
  packet_height[i] = h / 4.0;
  state[i] = 3; // B4::B: both transition lanes are retained.
}
"#;

#[derive(Clone, Debug)]
pub struct IuttReading {
    pub discrepancy: f64,
    pub packet_radical: f64,
    pub packet_height: f64,
    pub weighted_height: f64,
    pub state: crate::belnap::B4,
}

/// Evaluate a batch of `(c, radical)` observables on one CUDA device.
pub fn observable_batch(c: &[u64], radical: &[u64], eps: f64, device: usize)
    -> Result<Vec<IuttReading>, String> {
    if c.len() != radical.len() {
        return Err("gpu_abc: c/radical length mismatch".into());
    }
    if c.is_empty() {
        return Ok(Vec::new());
    }
    let ctx = CudaContext::new(device).map_err(|e| format!("gpu_abc: context: {e}"))?;
    let stream = ctx.default_stream();
    let ptx = compile_ptx(KERNEL).map_err(|e| format!("gpu_abc: NVRTC: {e}"))?;
    let module = ctx.load_module(ptx).map_err(|e| format!("gpu_abc: module: {e}"))?;
    let func = module.load_function("abc_observable").map_err(|e| format!("gpu_abc: kernel: {e}"))?;
    let d_c = stream.clone_htod(c).map_err(|e| format!("gpu_abc: upload c: {e}"))?;
    let d_rad = stream.clone_htod(radical).map_err(|e| format!("gpu_abc: upload radical: {e}"))?;
    let mut d_out = stream.alloc_zeros::<f64>(c.len()).map_err(|e| format!("gpu_abc: alloc: {e}"))?;
    let mut d_packet_rad = stream.alloc_zeros::<f64>(c.len()).map_err(|e| format!("gpu_abc: alloc packet: {e}"))?;
    let mut d_packet_height = stream.alloc_zeros::<f64>(c.len()).map_err(|e| format!("gpu_abc: alloc packet: {e}"))?;
    let mut d_state = stream.alloc_zeros::<u8>(c.len()).map_err(|e| format!("gpu_abc: alloc state: {e}"))?;
    let n = c.len() as u32;
    let mut launch = stream.launch_builder(&func);
    launch.arg(&d_c); launch.arg(&d_rad); launch.arg(&eps);
    launch.arg(&mut d_out); launch.arg(&mut d_packet_rad); launch.arg(&mut d_packet_height); launch.arg(&mut d_state); launch.arg(&n);
    unsafe { launch.launch(LaunchConfig::for_num_elems(n)) }
        .map_err(|e| format!("gpu_abc: launch: {e}"))?;
    let out = stream.clone_dtoh(&d_out).map_err(|e| format!("gpu_abc: download: {e}"))?;
    let packet_rad = stream.clone_dtoh(&d_packet_rad).map_err(|e| format!("gpu_abc: download packet: {e}"))?;
    let packet_height = stream.clone_dtoh(&d_packet_height).map_err(|e| format!("gpu_abc: download packet: {e}"))?;
    let state: Vec<crate::belnap::B4> = stream.clone_dtoh(&d_state).map_err(|e| format!("gpu_abc: download state: {e}"))?
        .into_iter().map(crate::belnap::B4::from_u8).collect();
    Ok(out.into_iter().zip(packet_rad).zip(packet_height).zip(state)
        .map(|(((discrepancy, packet_radical), packet_height), state)| IuttReading {
            discrepancy, packet_radical, packet_height,
            weighted_height: packet_height * 4.0, state,
        }).collect())
}

/// Evaluate several epsilon readings over one uploaded batch.
pub fn observable_batch_many(c: &[u64], radical: &[u64], eps: &[f64], device: usize)
    -> Result<Vec<Vec<f64>>, String> {
    let mut all = Vec::with_capacity(eps.len());
    for &e in eps { all.push(observable_batch(c, radical, e, device)?.into_iter().map(|r| r.discrepancy).collect()); }
    Ok(all)
}
