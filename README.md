# $m⊙^{2}$: A Self-Imscribing Bare-Metal Kernel

![language](https://img.shields.io/badge/language-Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white)
![tier](https://img.shields.io/badge/tier-O%E2%88%9E-8A2BE2?style=for-the-badge)
![µ∘δ](https://img.shields.io/badge/%CE%BC%E2%88%98%CE%B4-id-00A86B?style=for-the-badge)
![license](https://img.shields.io/badge/licence-Unlicense-1A1A1A?style=for-the-badge)

Bare-metal OS kernel in no_std Rust (x86_64, zero crates, ~30k lines) replacing processes/scheduler/filesystem with a single self-verifying loop: every state is a Crystal address (17.28M), storage navigated by address, each IMASM tick enforcing μ∘δ=id by construction.

## Architecture

**Loop:** THINK→ACT→OBSERVE→UPDATE = ⊢ VINIT → ≻/⋈ → ⊙/∈ → ∋/⊡. **Catalog** (`catalog.rs`, 954 lines) is the single source of truth — no constants/glyphs outside it; `register_entry()` at runtime. Nine upstream modules (imasmic_core, IMSCRIBr, ALEPH_OS, priests-engine) run natively.

**Prime winders** (kernel-verified via `imasm cycle`): `oneshot_prime_winder` (period 14, B4 primality via winding order/BSGS), `nested_oneshot` (period 12, Brent fold/kiss), `doubly_nested_oneshot` (period 16). **Topological QC:** `fibqc` braids Fibonacci anyons, compiles gates, evaluates Jones — no FPU assumed. **SIC-POVM:** d=12 campaign via `d12` (phase-tower 3→1, K₁₆ magnitude group, 143/143 ring-exact overlaps, norm(N₁)=1/32448², Belnap SIC axiom-free). **Belnap:** C₄ plane (i²=B), Belnap Shor (period in 2:1 B/T coherence ratio). **Clay witnesses:** 7 problems as IMASM programs; `frobenius_unify.rs` one invariant. **Rebis:** 20 modules no_std off REPL (p4ra, genetic, 109 enzymes, ligand, frustration). **Dialects:** 12 rulesets, Crystal-invariant sheaf + 11 diaschizic compounds. **Real exec:** `vox run` lifts ELF/PE to payload-carrying modules and runs genuine processes (argv/envp/auxv, syscalls).

**GPU:** full CUDA SIXTEEN_3 trilattice (`Reg16_3`, 12 branch-free gates, bit-identical vs CPU; `gpu16_3 verify` 12B checks, 425× speedup at n=10⁹), full-Crystal round-trip on device, `gpu_native` ob3ect word per-tick. Sibling `G-mOMonadOS` makes GPU the only mode.

```bash
cargo build --release --features hosted   # userspace REPL (Linux/Win/macOS; release.yml via lipo)
```

REPL: `d12/c4/belnap/stark/clay/triple/ruleset/fibqc/vox run/gpu*`. Tag `v*` cuts a release (needs sibling Vox + MoDoT checked out). Unlicense.

Full 164-line version: `README_backups/mOMonadOS_README.md`.

μ∘δ=id
