//! gpu_trilattice.rs — GPU-native residue sweep for `trilattice_factor
//! dialect-probe`, the rule that everything here is built trilattice-gpu
//! native, applied to the CPU path that command shipped with.
//!
//! Per residue a^k mod n, `word_to_tuple(native_numeral::encode(residue))`
//! only ever needs three facts about the residue: its bit length, whether it
//! is zero, and whether it is an all-ones number (2^B - 1). Every other
//! Snapshot field self_imscribe derives from encode()'s word is FIXED by the
//! word's own shape, checked directly against the real encode()/self_imscribe
//! source, not assumed:
//!   - encode() always opens on VINIT (⊢) and closes on TANCH (⊣), two
//!     different tokens, so self_ref (first==last) is always false, and with
//!     it bifurcation_revisited (which requires self_ref).
//!   - ENGAGR (⊞) never appears in a native-numeral word at all, so
//!     dialetheia_complete (which requires an ENGAGR present) is always false.
//!   - VINIT appears exactly once, at position 0, in every word; a period
//!     p < len would require another VINIT at position p, which cannot exist,
//!     so the minimal period is always the full word length: 4 for n=0
//!     (⊢⊙⊡⊣), 5*B+4 for n>=1 (⊢ then B repeats of ≻⋈∈{⊤|⊥}∋, then ⊙⊡⊣).
//!   - FSPLIT/FFUSE (∈/∋) appear exactly once per bit, so frobenius_order is
//!     0 for n=0 and 1 for n>=1 (the first ∈ in bit 0's block always precedes
//!     that block's ∋), and atomic_reentry (exactly one of each) holds only
//!     at B=1 (n=1).
//!   - IFIX (⊡) appears exactly once regardless of n, so sig.3 (the Linear
//!     family count) is always 1.
//!   - token_diversity is 4 at n=0; otherwise 8 fixed types (VINIT, AFWD,
//!     CLINK, FSPLIT, FFUSE, IMSCRIB, IFIX, TANCH) plus EVALF (always present:
//!     to_bits_low_first's last bit, the number's own MSB, is always 1) plus
//!     EVALT (present unless every bit is 1, i.e. n is all-ones) — 9 for
//!     all-ones n, 10 otherwise.
//! `tuple_from_bitlen` below builds the real `Snapshot` from exactly these
//! three facts and hands it to the unmodified `IgTuple::from_snapshot` — the
//! axis derivation itself is never re-implemented, only the cheap path to the
//! Snapshot that feeds it. `verify_tuple_from_bitlen` checks this equivalence
//! against the literal token-array path for real residues before any GPU
//! result is trusted.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions, Ptx};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::gpu_rho::n0_inv;
use crate::imas_ig::IgTuple;
use crate::kernel::Snapshot;

fn limbs_of(n: &BigUint, limbs: usize) -> Vec<u64> {
    let d = n.to_u64_digits();
    let mut v = alloc::vec![0u64; limbs];
    for i in 0..limbs.min(d.len()) {
        v[i] = d[i];
    }
    v
}

/// Same cap the other multi-limb kernels use: enough limbs for n, plus one
/// guard limb the CIOS montmul needs, min 2, max 32 (2048 bits).
fn pick_limbs(n: &BigUint) -> usize {
    let need = ((n.bits() as usize) + 63) / 64;
    (need + 1).max(2).min(32)
}

fn ptx_path(limbs: usize, src: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in src.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("target/gpu_trilattice_{limbs}_{h:016x}.ptx")
}

fn load_ptx_cached(limbs: usize) -> Result<Ptx, String> {
    let src = kernel_src(limbs);
    let path = ptx_path(limbs, &src);
    if std::path::Path::new(&path).is_file() {
        return Ok(Ptx::from_file(&path));
    }
    let opts = CompileOptions { options: alloc::vec!["--device-int128".into()], ..Default::default() };
    let ptx = compile_ptx_with_opts(src, opts).map_err(|e| format!("NVRTC: {e}"))?;
    let out = ptx.to_src();
    let _ = std::fs::write(&path, out.as_bytes());
    Ok(Ptx::from_src(out))
}

fn kernel_src(limbs: usize) -> String {
    format!(r#"
#define LIMBS {L}
typedef unsigned long long u64;
typedef unsigned __int128 u128;

__device__ __noinline__ void montmul(const u64* a, const u64* b, const u64* N, u64 n0, u64* r){{
    u64 t[LIMBS+2]; for (int i=0;i<LIMBS+2;i++) t[i]=0;
    for (int i=0;i<LIMBS;i++){{
        u128 C=0;
        for (int j=0;j<LIMBS;j++){{ u128 x=(u128)a[j]*(u128)b[i]+(u128)t[j]+C; t[j]=(u64)x; C=x>>64; }}
        u128 s=(u128)t[LIMBS]+C; t[LIMBS]=(u64)s; t[LIMBS+1]=(u64)(s>>64);
        u64 m=t[0]*n0;
        u128 c2; {{ u128 x=(u128)m*(u128)N[0]+(u128)t[0]; c2=x>>64; }}
        for (int j=1;j<LIMBS;j++){{ u128 x=(u128)m*(u128)N[j]+(u128)t[j]+c2; t[j-1]=(u64)x; c2=x>>64; }}
        u128 s2=(u128)t[LIMBS]+c2; t[LIMBS-1]=(u64)s2; c2=s2>>64; t[LIMBS]=t[LIMBS+1]+(u64)c2; t[LIMBS+1]=0;
    }}
    bool ge; if (t[LIMBS]!=0) ge=true; else {{ ge=true; for(int j=LIMBS-1;j>=0;j--){{ if(t[j]!=N[j]){{ ge=t[j]>N[j]; break; }} }} }}
    if (ge){{ u128 br=0; for(int j=0;j<LIMBS;j++){{ u128 x=(u128)t[j]-(u128)N[j]-br; r[j]=(u64)x; br=(x>>64)&1; }} }}
    else {{ for(int j=0;j<LIMBS;j++) r[j]=t[j]; }}
}}

// One thread per exponent k = k_base + gid: a^k mod n via Montgomery
// square-and-multiply, then read the residue's bit length and all-ones-ness
// straight off the LIMBS words — the only two facts the trilattice reading
// of the residue's word needs beyond whether it is zero.
extern "C" __global__ void trilattice_snapshot(
    const u64* A, const u64* N, const u64* R2, u64 n0, const u64* one_mont,
    u64 k_base, u64* out_bitlen, unsigned char* out_all_ones, unsigned char* out_is_zero,
    unsigned int count)
{{
    unsigned int gid = blockIdx.x*blockDim.x + threadIdx.x;
    if (gid >= count) return;
    u64 k = k_base + (u64)gid;

    u64 base[LIMBS]; montmul(A, R2, N, n0, base);   // A into Montgomery form
    u64 acc[LIMBS]; for (int j=0;j<LIMBS;j++) acc[j]=one_mont[j];
    while (k > 0) {{
        if (k & 1ULL) {{ u64 t[LIMBS]; montmul(acc, base, N, n0, t); for(int j=0;j<LIMBS;j++) acc[j]=t[j]; }}
        u64 t2[LIMBS]; montmul(base, base, N, n0, t2); for(int j=0;j<LIMBS;j++) base[j]=t2[j];
        k >>= 1;
    }}
    u64 one_plain[LIMBS]; for (int j=0;j<LIMBS;j++) one_plain[j]=0; one_plain[0]=1;
    u64 res[LIMBS]; montmul(acc, one_plain, N, n0, res);   // out of Montgomery form

    bool seen = false; int bitlen = 0;
    for (int limb = LIMBS-1; limb >= 0; limb--) {{
        if (!seen && res[limb] != 0) {{
            seen = true;
            int hb = 63; while (hb >= 0 && ((res[limb] >> hb) & 1ULL) == 0) hb--;
            bitlen = limb*64 + hb + 1;
        }}
    }}
    bool is_zero = !seen;
    bool all_ones = false;
    if (!is_zero) {{
        int pc = 0;
        for (int limb = 0; limb < LIMBS; limb++) pc += __popcll(res[limb]);
        all_ones = (pc == bitlen);
    }}
    out_bitlen[gid] = (u64)bitlen;
    out_all_ones[gid] = all_ones ? 1 : 0;
    out_is_zero[gid] = is_zero ? 1 : 0;
}}
"#, L = limbs)
}

/// The real Snapshot self_imscribe would produce for the native-numeral word
/// of a residue with this bit length, zero-ness, and all-ones-ness — see the
/// module doc comment for the derivation of each field. Handed unmodified to
/// `IgTuple::from_snapshot`, never a second copy of that derivation.
pub fn tuple_from_bitlen(bitlen: u64, is_zero: bool, all_ones: bool) -> IgTuple {
    let b = bitlen as usize;
    let (frobenius_order, period, sig, token_diversity) = if is_zero {
        (0u8, 4usize, (3usize, 0usize, 0usize, 1usize), 4usize)
    } else {
        (1u8, 5 * b + 4, (3 + 2 * b, 2 * b, b, 1), if all_ones { 9 } else { 10 })
    };
    let snap = Snapshot {
        frobenius_order,
        period,
        sig,
        token_diversity,
        self_ref: false,
        dialetheia_complete: false,
        tier: 0,
        b_live_ticks: 0,
        gate_discriminations: 0,
        value_period: 0,
        atomic_reentry: !is_zero && bitlen == 1,
        bifurcation_revisited: false,
        winding_count: 0,
    };
    IgTuple::from_snapshot(&snap)
}

/// Checked equivalence: for residues 0..bound, `tuple_from_bitlen` fed the
/// real (bit length, is_zero, all-ones) of the residue must match
/// `axis_values::word_to_tuple(native_numeral::encode(residue))` exactly —
/// the literal path, token array and all. Any mismatch is a real defect in
/// the derivation above, reported by index rather than assumed absent.
pub fn verify_tuple_from_bitlen(bound: u64) -> String {
    let mut mismatches: Vec<u64> = Vec::new();
    for r in 0..bound {
        let rb = BigUint::from(r);
        let bitlen = rb.bits();
        let is_zero = rb.is_zero();
        let all_ones = !is_zero && {
            let ones = rb.count_ones();
            ones == bitlen
        };
        let got = tuple_from_bitlen(bitlen, is_zero, all_ones);
        let want = crate::axis_values::word_to_tuple(&crate::native_numeral::encode(&r.to_string()));
        if got != want {
            mismatches.push(r);
        }
    }
    if mismatches.is_empty() {
        format!("gpu_trilattice verify_tuple_from_bitlen: 0..{} residues, tuple_from_bitlen matches word_to_tuple in every case", bound)
    } else {
        format!("gpu_trilattice verify_tuple_from_bitlen: 0..{} residues, {} mismatch(es) at {:?}", bound, mismatches.len(), mismatches)
    }
}

/// a^k mod n for k = 0..max_k-1, in parallel, one GPU thread per k — the
/// naturally-parallel part of `dialect-probe`'s sweep. n must be odd (the
/// Montgomery arithmetic here requires it, same constraint every other
/// multi-limb kernel in this codebase carries); the caller falls back to the
/// CPU path otherwise. Returns (bitlen, is_zero, all_ones) per k, ready for
/// `tuple_from_bitlen` and the existing `dialect_all_pass` sweep — the gate
/// logic itself stays exactly what it already was, unduplicated.
pub fn gpu_residue_facts(n: &BigUint, a: &BigUint, max_k: u64, device: usize) -> Result<Vec<(u64, bool, bool)>, String> {
    use crate::native_numeral::{modulo_via_word, modulo_small_via_word, multiply_via_word, pow2};
    if modulo_small_via_word(n, 2).unwrap() == 0 {
        return Err("gpu_trilattice: n must be odd for Montgomery arithmetic".into());
    }
    if max_k == 0 {
        return Ok(Vec::new());
    }
    let limbs = pick_limbs(n);
    let a_mod = modulo_via_word(a, n).unwrap();
    let two_pow = pow2(64 * limbs);
    let r2 = modulo_via_word(&multiply_via_word(&two_pow, &two_pow), n).unwrap();
    let one_mont = modulo_via_word(&two_pow, n).unwrap();
    let nl = limbs_of(n, limbs);
    let n0 = n0_inv(nl[0]);

    let ctx = CudaContext::new(device).map_err(|e| format!("gpu_trilattice: no ctx: {e}"))?;
    let stream = ctx.default_stream();
    let ptx = load_ptx_cached(limbs)?;
    let module = ctx.load_module(ptx).map_err(|e| format!("gpu_trilattice: module: {e}"))?;
    let func = module.load_function("trilattice_snapshot").map_err(|e| format!("gpu_trilattice: load: {e}"))?;

    let d_a = stream.clone_htod(&limbs_of(&a_mod, limbs)).map_err(|e| format!("{e}"))?;
    let d_n = stream.clone_htod(&nl).map_err(|e| format!("{e}"))?;
    let d_r2 = stream.clone_htod(&limbs_of(&r2, limbs)).map_err(|e| format!("{e}"))?;
    let d_om = stream.clone_htod(&limbs_of(&one_mont, limbs)).map_err(|e| format!("{e}"))?;

    let count = max_k as u32;
    let mut d_bitlen = stream.alloc_zeros::<u64>(count as usize).map_err(|e| format!("{e}"))?;
    let mut d_all_ones = stream.alloc_zeros::<u8>(count as usize).map_err(|e| format!("{e}"))?;
    let mut d_is_zero = stream.alloc_zeros::<u8>(count as usize).map_err(|e| format!("{e}"))?;

    let block: u32 = 128;
    let grid = (count + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid, 1, 1), block_dim: (block, 1, 1), shared_mem_bytes: 0 };
    let k_base: u64 = 0;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_a); b.arg(&d_n); b.arg(&d_r2); b.arg(&n0); b.arg(&d_om);
    b.arg(&k_base); b.arg(&mut d_bitlen); b.arg(&mut d_all_ones); b.arg(&mut d_is_zero); b.arg(&count);
    if let Err(e) = unsafe { b.launch(cfg) } { return Err(format!("gpu_trilattice: launch: {e}")); }

    let bitlens = stream.clone_dtoh(&d_bitlen).map_err(|e| format!("{e}"))?;
    let all_ones = stream.clone_dtoh(&d_all_ones).map_err(|e| format!("{e}"))?;
    let is_zero = stream.clone_dtoh(&d_is_zero).map_err(|e| format!("{e}"))?;

    Ok((0..max_k as usize).map(|i| (bitlens[i], is_zero[i] != 0, all_ones[i] != 0)).collect())
}
