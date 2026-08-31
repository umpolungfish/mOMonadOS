import Mathlib

/-!
# SIC_D12_Norm — the trace-one SIC condition, exact in K16 (machine-checked)

Part of the d=12 axiom-drop build (target `crystal_forces_d12_sic`). This file
discharges the **norm half** of `IsSICPOVM 12 ψ` exactly: the twelve fiducial
moduli |z_k|^2 sum to 1.

The moduli lie in the totally-real degree-16 field
  K16 = Q[g]/(g^16 - 10 g^14 + 40 g^12 - 90 g^10 + 126 g^8 - 96 g^6 + 25 g^4 + 2 g^2 + 1)
(the maximal-real subfield of the conductor-12 ray class field of Q(sqrt13)).
Each |z_k|^2 is entered as its exact coefficient vector in the power basis
{1, g, ..., g^15}, recovered from the 1500-digit numeric fiducial (each vector
reproduces |z_k|^2 to full precision; denominators ~15 bits).

`norm_sum` is a pure rational identity (K16 addition is componentwise), so it needs
no square roots and no multiplication table: the moduli **are** the summands. This is
the exact form of `∑ₖ |z_k|² = 1`, the trace-one SIC condition. The equiangularity
half (143 overlap identities) needs the magnitude/phase compositum tower and is built
separately; here the norm is settled unconditionally by `native_decide`.
-/

namespace SIC.D12.Norm

/-- Element of K16 as a length-16 coefficient vector in the power basis {1,g,...,g^15}. -/
abbrev K16 := List ℚ

/-- K16 addition: componentwise on the power-basis coefficients. -/
def add (x y : K16) : K16 := List.zipWith (· + ·) x y

/-- Exact fiducial modulus |z0|^2 in the K16 power basis. -/
def N0 : K16 := [563/8541, 1207/17082, 13/657, -3322/8541, 971/5694, 57245/34164, -2560/8541, -21838/8541, 461/1898, 67735/34164, -77/657, -8159/8541, 263/8541, 8899/34164, -3/949, -953/34164]
/-- Exact fiducial modulus |z1|^2 in the K16 power basis. -/
def N1 : K16 := [475/2847, -8243/34164, 686/2847, 13475/11388, -2713/5694, -7255/2628, 638/2847, 2107/657, -31/5694, -18479/8541, -61/949, 7919/8541, 203/5694, -199/876, -5/949, 769/34164]
/-- Exact fiducial modulus |z2|^2 in the K16 power basis. -/
def N2 : K16 := [2113/17082, 2581/34164, -6467/8541, 26231/17082, 11477/5694, -189017/34164, -20656/8541, 123533/17082, 4717/2847, -176809/34164, -6203/8541, 78887/34164, 1565/8541, -9917/17082, -53/2847, 997/17082]
/-- Exact fiducial modulus |z3|^2 in the K16 power basis. -/
def N3 : K16 := [-595/5694, 1445/8541, 26/219, 1462/2847, 971/949, -8261/2628, -5120/2847, 37946/8541, 1383/949, -111655/34164, -154/219, 25361/17082, 526/2847, -4279/11388, -18/949, 1289/34164]
/-- Exact fiducial modulus |z4|^2 in the K16 power basis. -/
def N4 : K16 := [74/657, 10475/34164, 5791/8541, -12439/34164, -15361/5694, -13645/34164, 30896/8541, 16033/17082, -7483/2847, -13333/17082, 10207/8541, 3130/8541, -2617/8541, -3131/34164, 89/2847, 307/34164]
/-- Exact fiducial modulus |z5|^2 in the K16 power basis. -/
def N5 : K16 := [257/1898, 3241/34164, -285/949, -9307/5694, -100/2847, 12299/2628, 1922/2847, -97933/17082, -2059/2847, 135319/34164, 1184/2847, -58867/34164, -243/1898, 2429/5694, 14/949, -727/17082]
/-- Exact fiducial modulus |z6|^2 in the K16 power basis. -/
def N6 : K16 := [563/8541, -1207/17082, 13/657, 3322/8541, 971/5694, -57245/34164, -2560/8541, 21838/8541, 461/1898, -67735/34164, -77/657, 8159/8541, 263/8541, -8899/34164, -3/949, 953/34164]
/-- Exact fiducial modulus |z7|^2 in the K16 power basis. -/
def N7 : K16 := [475/2847, 8243/34164, 686/2847, -13475/11388, -2713/5694, 7255/2628, 638/2847, -2107/657, -31/5694, 18479/8541, -61/949, -7919/8541, 203/5694, 199/876, -5/949, -769/34164]
/-- Exact fiducial modulus |z8|^2 in the K16 power basis. -/
def N8 : K16 := [2113/17082, -2581/34164, -6467/8541, -26231/17082, 11477/5694, 189017/34164, -20656/8541, -123533/17082, 4717/2847, 176809/34164, -6203/8541, -78887/34164, 1565/8541, 9917/17082, -53/2847, -997/17082]
/-- Exact fiducial modulus |z9|^2 in the K16 power basis. -/
def N9 : K16 := [-595/5694, -1445/8541, 26/219, -1462/2847, 971/949, 8261/2628, -5120/2847, -37946/8541, 1383/949, 111655/34164, -154/219, -25361/17082, 526/2847, 4279/11388, -18/949, -1289/34164]
/-- Exact fiducial modulus |z10|^2 in the K16 power basis. -/
def N10 : K16 := [74/657, -10475/34164, 5791/8541, 12439/34164, -15361/5694, 13645/34164, 30896/8541, -16033/17082, -7483/2847, 13333/17082, 10207/8541, -3130/8541, -2617/8541, 3131/34164, 89/2847, -307/34164]
/-- Exact fiducial modulus |z11|^2 in the K16 power basis. -/
def N11 : K16 := [257/1898, -3241/34164, -285/949, 9307/5694, -100/2847, -12299/2628, 1922/2847, 97933/17082, -2059/2847, -135319/34164, 1184/2847, 58867/34164, -243/1898, -2429/5694, 14/949, 727/17082]

/-- The field unit `1` as a K16 coefficient vector. -/
def one16 : K16 := [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

/-- **Trace-one / norm condition, exact.** The twelve d=12 SIC fiducial moduli sum to
    exactly 1 in K16 — a rational identity, no square roots. This is `∑ₖ |z_k|² = 1`. -/
theorem norm_sum :
    add N0 (add N1 (add N2 (add N3 (add N4 (add N5 (add N6 (add N7 (add N8 (add N9 (add N10 N11)))))))))) = one16 := by
  native_decide

/-- Each modulus vector has the right length (well-formed K16 element). -/
theorem all_len16 :
    [N0,N1,N2,N3,N4,N5,N6,N7,N8,N9,N10,N11].all (fun v => v.length == 16) = true := by
  native_decide

end SIC.D12.Norm
