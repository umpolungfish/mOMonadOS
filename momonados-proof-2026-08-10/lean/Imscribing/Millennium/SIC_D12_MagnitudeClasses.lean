import Imscribing.Millennium.SIC_D12_Norm
import Imscribing.Millennium.SIC_D12_Equiangularity

/-!
# SIC_D12_MagnitudeClasses — the magnitude field is K16(√N₀,√N₁,√N₃,√N₅,√N₉) (machine-checked)

Part of the d=12 existence construction (target `crystal_forces_d12_sic`; route
sanctioned by the d12-existence ob3ect batch, all 7 Frobenius PASS). The twelve
fiducial magnitudes are √N₀,…,√N₁₁ with N_k the exact K16 moduli of `SIC_D12_Norm`.
The magnitude square-class group of the fiducial has rank 5 with independent set
{N₀, N₁, N₃, N₅, N₉}, and — the singleton-pairing structure found 2026-07-03 —
every dependent modulus pairs with exactly ONE basis modulus:

  [N₂] = [N₄] = [N₆] = [N₈] = [N₁₀] = [N₀],  [N₇] = [N₅],  [N₁₁] = [N₁].

This file machine-checks the seven witnesses: for each dependent k, an explicit
C_k ∈ K16 with C_k² = N_k·N_i exactly (i its basis partner). Consequently
√N_k = C_k·√N_i / N_i, so ALL TWELVE magnitude roots lie in the five-cover tower
K16(√N₀,√N₁,√N₃,√N₅,√N₉) of degree 512 = 16·2⁵ over ℚ — the magnitude layer of
the coordinate tower, now exact in Lean, no floors.

K16 multiplication is flat polynomial arithmetic on power-basis vectors modulo
pr = x¹⁶−10x¹⁴+40x¹²−90x¹⁰+126x⁸−96x⁶+25x⁴+2x²+1 (monic, totally real), reusing
the `Poly` engine of `SIC_D12_Equiangularity`. Data generated procedurally by
gen_lean_magnitude_classes.py from magnitude_classes_data.txt (each C_k recovered
by nfroots in PARI — an exact field computation, not a floor — and re-verified in
Python fractions before emission).
-/

namespace SIC.D12.MagnitudeClasses

open SIC.D12.Norm (K16 N0 N1 N2 N3 N4 N5 N6 N7 N8 N9 N10 N11)
open SIC.D12.Equiangularity (pmul pmod)

/-- The K16 defining polynomial pr, monic tail highest-first after the leading 1. -/
def prTail : List ℚ := [0, -10, 0, 40, 0, -90, 0, 126, 0, -96, 0, 25, 0, 2, 0, 1]

/-- K16 multiplication: flat polynomial product reduced mod pr. -/
def kmul (x y : K16) : K16 := pmod (pmul x y) prTail 16

/-- Exact square-class witness: C2² = N2·N0 in K16 (branch positive at the witness embedding). -/
def C2 : K16 := [0, -10/657, 0, 32693/34164, 0, -20632/8541, 0, 92687/34164, 0, -15242/8541, 0, 25267/34164, 0, -455/2628, 0, 281/17082]
/-- Exact square-class witness: C4² = N4·N0 in K16 (branch positive at the witness embedding). -/
def C4 : K16 := [-862/8541, -1240/8541, -1889/8541, 4730/8541, 614/949, -35599/34164, -4474/8541, 34961/34164, 707/2847, -5360/8541, -452/8541, 2116/8541, -83/17082, -1889/34164, 2/949, 43/8541]
/-- Exact square-class witness: C6² = N6·N0 in K16 (branch positive at the witness embedding). -/
def C6 : K16 := [0, -1207/17082, 0, 3322/8541, 0, -57245/34164, 0, 21838/8541, 0, -67735/34164, 0, 8159/8541, 0, -8899/34164, 0, 953/34164]
/-- Exact square-class witness: C7² = N7·N5 in K16 (branch positive at the witness embedding). -/
def C7 : K16 := [791/5694, 1370/8541, -275/2847, -51613/34164, -779/949, 118127/34164, 3878/2847, -31912/8541, -2953/2847, 20602/8541, 2713/5694, -33731/34164, -114/949, 1951/8541, 34/2847, -367/17082]
/-- Exact square-class witness: C8² = N8·N0 in K16 (branch positive at the witness embedding). -/
def C8 : K16 := [1187/17082, 10/657, -2734/8541, -32693/34164, -1171/5694, 20632/8541, 8326/8541, -92687/34164, -5501/5694, 15242/8541, 4553/8541, -25267/34164, -2713/17082, 455/2628, 17/949, -281/17082]
/-- Exact square-class witness: C10² = N10·N0 in K16 (branch positive at the witness embedding). -/
def C10 : K16 := [0, 1240/8541, 0, -4730/8541, 0, 35599/34164, 0, -34961/34164, 0, 5360/8541, 0, -2116/8541, 0, 1889/34164, 0, -43/8541]
/-- Exact square-class witness: C11² = N11·N1 in K16 (branch positive at the witness embedding). -/
def C11 : K16 := [791/5694, -1370/8541, -275/2847, 51613/34164, -779/949, -118127/34164, 3878/2847, 31912/8541, -2953/2847, -20602/8541, 2713/5694, 33731/34164, -114/949, -1951/8541, 34/2847, 367/17082]

/-- N2 lies in the square class of N0: C2² = N2·N0 exactly in K16. -/
theorem N2_class : kmul C2 C2 = kmul N2 N0 := by native_decide

/-- N4 lies in the square class of N0: C4² = N4·N0 exactly in K16. -/
theorem N4_class : kmul C4 C4 = kmul N4 N0 := by native_decide

/-- N6 lies in the square class of N0: C6² = N6·N0 exactly in K16. -/
theorem N6_class : kmul C6 C6 = kmul N6 N0 := by native_decide

/-- N7 lies in the square class of N5: C7² = N7·N5 exactly in K16. -/
theorem N7_class : kmul C7 C7 = kmul N7 N5 := by native_decide

/-- N8 lies in the square class of N0: C8² = N8·N0 exactly in K16. -/
theorem N8_class : kmul C8 C8 = kmul N8 N0 := by native_decide

/-- N10 lies in the square class of N0: C10² = N10·N0 exactly in K16. -/
theorem N10_class : kmul C10 C10 = kmul N10 N0 := by native_decide

/-- N11 lies in the square class of N1: C11² = N11·N1 exactly in K16. -/
theorem N11_class : kmul C11 C11 = kmul N11 N1 := by native_decide

/-- The five-cover structure, stated once: every dependent modulus has an exact
    K16 witness pairing it to its unique basis partner among {N₀,N₁,N₃,N₅,N₉}. -/
theorem magnitude_field_five_covers :
    kmul C2 C2 = kmul N2 N0 ∧ kmul C4 C4 = kmul N4 N0 ∧ kmul C6 C6 = kmul N6 N0 ∧
    kmul C7 C7 = kmul N7 N5 ∧ kmul C8 C8 = kmul N8 N0 ∧ kmul C10 C10 = kmul N10 N0 ∧
    kmul C11 C11 = kmul N11 N1 :=
  ⟨N2_class, N4_class, N6_class, N7_class, N8_class, N10_class, N11_class⟩

end SIC.D12.MagnitudeClasses
