import Mathlib

/-!
# SIC_D12_NumberField — piece 3 of the d=12 axiom-drop build

A **computable number field** `ℚ[θ]/(m(θ))` for a monic `m` of degree `n`, as
coefficient vectors (`Array ℚ`, length `n`) with `native_decide`-able +, ×, and
reduction mod `m`. This is the engine the d=12 coordinate field needs: PARI found
the SIC coordinate field is `K = ℚ(θ)` of **degree 48**, so the whole construction
(coordinates, WH operators, the 143 Gram identities) lives in one such ring.

Validated here on the exact monic octic `x⁸ − 2x⁶ − 4x⁴ − 8x² + 16` (the modulus
field, already known). Reduction is driven by `redRule`, the reduced form of `θⁿ`:
for a monic `m = xⁿ + Σ mᵢ xⁱ` we have `θⁿ = −Σ mᵢ θⁱ`, so `redRule` holds those
`−mᵢ`.
-/

namespace SIC.D12.NumberField

/-- Field element: coefficient vector, index `i` = coeff of `θⁱ`, length `n`. -/
abbrev NF := Array ℚ

/-- Distribute the single top coefficient `c·θᵏ` (k = p.size−1 ≥ n) down via
    `θᵏ = θ^(k−n)·θⁿ = θ^(k−n)·redRule`, then drop it. Lowers degree by 1. -/
def reduceTop (n : ℕ) (red : NF) (p : NF) : NF :=
  let k := p.size - 1
  let c := p.getD k 0
  let base := k - n
  let p := p.pop
  (List.range n).foldl (fun acc i => acc.modify (base + i) (· + c * red.getD i 0)) p

/-- Reduce a product (length up to `2n−1`) to length `n`, `fuel` = max excess. -/
def reduceFuel (n : ℕ) (red : NF) : ℕ → NF → NF
  | 0,        p => p
  | fuel + 1, p => if p.size > n then reduceFuel n red fuel (reduceTop n red p) else p

/-- Pad/truncate to exactly length `n`. -/
def norm (n : ℕ) (p : NF) : NF :=
  (Array.range n).map (fun i => p.getD i 0)

def addNF (a b : NF) : NF := (Array.range (max a.size b.size)).map (fun i => a.getD i 0 + b.getD i 0)

/-- Schoolbook convolution (result length `a.size + b.size − 1`). -/
def convolve (a b : NF) : NF :=
  if a.size = 0 ∨ b.size = 0 then #[] else
  (Array.range (a.size + b.size - 1)).map (fun k =>
    (List.range (k + 1)).foldl (fun s i =>
      if i < a.size ∧ (k - i) < b.size then s + a.getD i 0 * b.getD (k - i) 0 else s) 0)

/-- Multiply in `ℚ[θ]/(m)`. -/
def mulNF (n : ℕ) (red : NF) (a b : NF) : NF :=
  norm n (reduceFuel n red (2 * n) (convolve a b))

def oneNF (n : ℕ) : NF := norm n #[1]
def powNF (n : ℕ) (red : NF) (a : NF) : ℕ → NF
  | 0     => oneNF n
  | k + 1 => mulNF n red a (powNF n red a k)

-- ── Validation on the exact monic octic x⁸ − 2x⁶ − 4x⁴ − 8x² + 16 ──
-- θ⁸ = 2θ⁶ + 4θ⁴ + 8θ² − 16, so redRule (coeffs θ⁰..θ⁷):
def octicRed : NF := #[-16, 0, 8, 0, 4, 0, 2, 0]
def octicTheta : NF := #[0, 1, 0, 0, 0, 0, 0, 0]

/-- θ⁸ reduces exactly to `2θ⁶ + 4θ⁴ + 8θ² − 16` — reduction mod `m` works. -/
theorem theta_pow8 : powNF 8 octicRed octicTheta 8 = octicRed := by native_decide

/-- θ is a root of the octic: θ⁸ − 2θ⁶ − 4θ⁴ − 8θ² + 16 = 0 in the ring. -/
theorem theta_is_root :
    addNF (powNF 8 octicRed octicTheta 8)
      (norm 8 #[16, 0, -8, 0, -4, 0, -2, 0]) = norm 8 #[0] := by native_decide

/-- Ring sanity: θ·θ⁷ = θ⁸ (associativity/consistency of mul + reduce). -/
theorem theta_mul_consistent :
    mulNF 8 octicRed octicTheta (powNF 8 octicRed octicTheta 7)
      = powNF 8 octicRed octicTheta 8 := by native_decide

end SIC.D12.NumberField
