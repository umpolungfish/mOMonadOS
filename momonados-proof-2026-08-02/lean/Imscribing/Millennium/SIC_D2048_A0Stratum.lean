import Mathlib

/-!
# SIC_D2048_A0Stratum — eleven a=0 overlap slots over F₂₀₄₈ (data-parametric)

**Layer 2 plank** (`sic_geometric_lift_hypothesis`): the exact-arithmetic engine and
obligation slots for the a = 0 stratum at d = 2048, mirroring d = 12 `mini_engine_a0.py`
with deliverable = this Lean module.

## Register separation (load-bearing)

| Register | Object | Status |
|----------|--------|--------|
| T-arm | `ZaunerTransportMap.transport_b_fiducial` | proved skeleton, NOT SIC |
| F-arm | modulus data `N : ℕ → F2048` (parameter, no value in-tree) | **this module** |
| Open | `SICPOVM_Exists 2048` | NOT claimed |

## No placeholders

This module carries **no modulus values at all**. Every overlap slot is a function of
the data `N : ℕ → F2048`; nothing is baked in. Campaign state at the time of writing
(`d12_sic_build/r2048_fiducial_manifest.json`):

- best genuine numerical witness residual ≈ 5.2e-3 (`coords_2048.py`) — not floor;
- stage3 143-rep restricted L-BFGS collapsed (0,b) overlaps to 0, not 1/2049 — wrong object;
- `deep_vanish_floor: false` — nothing is pinnable yet;
- `ig-docs/zauner_2048/coordinate_witness_1500.txt` is an exact algebraic ansatz
  z = sqrt((1+sqrt 2049)/4096)·exp(i·pi/2048) (minpoly of |z|² is 8192x² − 4x − 1,
  PSLQ-verified to 1500 digits), NOT a recovered fiducial coordinate; flagged
  mislabeled in the manifest and excluded here.

When `gen_lean_2048_a0.py` pins real S-unit moduli over F₂₀₄₈, it instantiates `N`
and promotes the slots to `native_decide` theorems (orbit-reduced grouping; a full
2048-term slot at compile time is expensive — see the d = 12 stratum collapse).

## The eleven obligations (a = 0 stratum)

For `b ∈ {1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024}` (orbit reps, 2-adic label
`v₂(a) = 999`, `orbit_table_2048.txt`):

```
O_{0,b}(N) = Σ_{k=0}^{2047} N_k · ω^{b·k}        (modulus-only; phases u_k deferred)
O_{0,b}(N) · Ō_{0,b}(N) = 1/2049
```

WH convention matches Lean `D_ah`: `(Dψ)_k = ω^{b(k−a)} ψ_{k−a}`.

Ring: `R = F₂₀₄₈[ω]/(ω^2048 − 1)` with `F₂₀₄₈ = ℚ(√4190205)`.
Elements are sparse assoc lists `(exponent, coeff)` with `coeff : F2048`.
-/

namespace SIC.D2048.A0Stratum

/-- Appleby discriminant  m_d = (2048 − 3)(2048 + 1) = 4190205. -/
def m2048 : ℚ := 4190205

/-- Element of F₂₀₄₈ = ℚ(√m2048): coeffs of `1` and `θ` (θ² = m2048). -/
abbrev F2048 := List ℚ

def fpad (v : F2048) : F2048 :=
  v ++ List.replicate (2 - v.length) (0 : ℚ)

def fadd (a b : F2048) : F2048 :=
  let a := fpad a; let b := fpad b
  [a.getD 0 0 + b.getD 0 0, a.getD 1 0 + b.getD 1 0]

def fscale (c : ℚ) (a : F2048) : F2048 :=
  let a := fpad a
  [c * a.getD 0 0, c * a.getD 1 0]

def fmul (a b : F2048) : F2048 :=
  let a := fpad a; let b := fpad b
  let a0 := a.getD 0 0; let a1 := a.getD 1 0
  let b0 := b.getD 0 0; let b1 := b.getD 1 0
  [a0 * b0 + m2048 * a1 * b1, a0 * b1 + a1 * b0]

def fzero : F2048 := [0, 0]
def fone : F2048 := [1, 0]

def fisZero (a : F2048) : Bool :=
  let a := fpad a
  (a.getD 0 0 == 0) && (a.getD 1 0 == 0)

/-- Sparse element of `F₂₀₄₈[ω]/(ω^2048−1)`: sorted `(exponent, coeff)` pairs. -/
abbrev RElt := List (ℕ × F2048)

def rinsert (acc : RElt) (e : ℕ) (c : F2048) : RElt :=
  if fisZero c then acc
  else
    let e := e % 2048
    match acc with
    | [] => [(e, c)]
    | (e', c') :: rest =>
      if e < e' then (e, c) :: (e', c') :: rest
      else if e == e' then
        let w := fadd c' c
        if fisZero w then rest else (e, w) :: rest
      else (e', c') :: rinsert rest e c

def radd (A B : RElt) : RElt := B.foldl (fun acc p => rinsert acc p.1 p.2) A

def rmulTerm (e1 : ℕ) (c1 : F2048) (e2 : ℕ) (c2 : F2048) : RElt :=
  rinsert [] ((e1 + e2) % 2048) (fmul c1 c2)

def rmul (A B : RElt) : RElt :=
  A.foldl (fun acc (e1, c1) =>
    B.foldl (fun acc2 (e2, c2) =>
      radd acc2 (rmulTerm e1 c1 e2 c2)) acc) []

def rneg (A : RElt) : RElt := A.map fun p => (p.1, fscale (-1) p.2)

/-- Field conjugation in `F₂₀₄₈`: `a₀ + a₁·θ ↦ a₀ − a₁·θ`. -/
def fconj (c : F2048) : F2048 :=
  let c := fpad c
  [c.getD 0 0, 0 - c.getD 1 0]

/-- Algebraic conjugation: `ω^e ↦ ω^{2048−e}`, `θ ↦ −θ`. -/
def rconj (A : RElt) : RElt :=
  A.foldl (fun acc (e, c) => rinsert acc ((2048 - e) % 2048) (fconj c)) []

def rK (c : F2048) : RElt := rinsert [] 0 c
def rzero : RElt := []
def rone : RElt := rK fone

def omegaPow (e : ℕ) : RElt := rinsert [] (e % 2048) fone

/-- Target overlap squared: `1/2049` embedded in `F₂₀₄₈`. -/
def rTarget : RElt := rK [1/2049, 0]

/-- The eleven coarse a=0 stratum shift values `b`. -/
def a0_b_values : List ℕ := [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]

/-! ## Overlap slots — parametric in the modulus data `N` (no values in-tree) -/

/-- Norm gate on modulus data: `Σ_k N_k = 1`. -/
def norm_gate (N : ℕ → F2048) : Bool :=
  (List.range 2048).foldl (fun acc k => fadd acc (N k)) fzero == fone

/-- Modulus-only a=0 overlap: `O_{0,b}(N) = Σ_k N_k · ω^{b·k}`. -/
def overlap_a0 (N : ℕ → F2048) (b : ℕ) : RElt :=
  (List.range 2048).foldl (fun acc k =>
    radd acc (rmul (rK (N k)) (omegaPow (b * k)))) rzero

/-- `O_{0,b}(N) · Ō_{0,b}(N)` in `R`. -/
def overlap_sq (N : ℕ → F2048) (b : ℕ) : RElt :=
  rmul (overlap_a0 N b) (rconj (overlap_a0 N b))

/-- **native_decide slot**: true iff the overlap squared equals `1/2049`. -/
def a0_overlap_sq_eq (N : ℕ → F2048) (b : ℕ) : Bool :=
  overlap_sq N b == rTarget

/-- Admission gate aggregate: all eleven slots pass for the supplied data. -/
def a0_slots_admitted (N : ℕ → F2048) : Bool :=
  a0_b_values.all (a0_overlap_sq_eq N)

/-! ## Field sanity (proved now, data-free) -/

theorem m2048_not_square : ¬ IsSquare (4190205 : ℤ) := by native_decide

theorem omega_pow_2048 :
    omegaPow 2048 == rone := by native_decide

-- After `gen_lean_2048_a0.py` pins real S-unit moduli, it emits `def modK_pinned`
-- plus `theorem a0_admitted : a0_slots_admitted modK_pinned = true := by native_decide`
-- (orbit-reduced). Until then no data exists in this module.

end SIC.D2048.A0Stratum
