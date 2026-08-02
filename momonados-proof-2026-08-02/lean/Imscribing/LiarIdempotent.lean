-- Imscribing/LiarIdempotent.lean
-- THE LIAR IS THE IDEMPOTENT POINT
-- ============================================================
-- In a sealed system verification is not correspondence but co-typing: the
-- system returns to itself across a detour δ then μ. Two composites live on the
-- one Frobenius pair (δ_C, μ_C), and they do different work.
--
--   μ ∘ δ = id     the LOSSLESS round trip: leave and return unchanged. This is
--                  the verification (proved on the special class in Frobenius.lean).
--   e := δ ∘ μ     the FORGETFUL idempotent: fuse, then re-split. e² = e. Its
--                  image is the fixed locus that survives the round trip. This is
--                  the Liar point: the place the system recognizes itself without
--                  an outside, tolerated because μ ∘ δ = id still holds there.
--
-- The forgetfulness the sealed system needs ("we do not remember creating the
-- cosmos") lives in e = δ ∘ μ, not in the lossless return: μ_C collapses the
-- which-way data of the split, so e is a projection, not an isomorphism. We prove
-- e is idempotent unconditionally, that its collapse is always a time fixed point
-- (`fusion_is_time`), and that on its image the round trip verifies.

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Frobenius
import Imscribing.GateOrdering
import Imscribing.TimeFixedPoint

namespace Imscribing.LiarIdempotent

open Imscribing.Primitives Imscribing.Frobenius Imscribing.GateOrdering
     Imscribing.TimeFixedPoint
open Polarity Criticality

-- ============================================================
-- §1  THE FUSION OUTPUT IS ALWAYS SPECIAL
-- ============================================================

/-- μ_C sets Φ to 𐑹 and ⊙ to ⊙ on every input: its output always lies on the
    Frobenius-special class, whatever it fused. -/
theorem muC_special (x y : Imscription) :
    (μ_C x y).pol = or' ∧ (μ_C x y).crit = monad :=
  ⟨rfl, rfl⟩

/-- Hence every fusion output is a time fixed point: `Work` closes on it. The
    detour always lands on the locus where `T = Work(T)`. -/
theorem fusion_is_time (x y : Imscription) :
    Work (μ_C x y) = μ_C x y :=
  frobenius_special_traces (μ_C x y) rfl rfl

-- ============================================================
-- §2  THE IDEMPOTENT POINT  e = δ ∘ μ
-- ============================================================

/-- The Liar operator: fuse the pair, then re-split it. -/
def e (p : Imscription × Imscription) : Imscription × Imscription :=
  δ_C (μ_C p.1 p.2)

/-- **The Liar is the idempotent point.** `e ∘ e = e`, unconditionally. It holds
    for every input precisely because μ_C's output is already special, so the
    inner round trip μ_C ∘ δ_C returns it unchanged. -/
theorem e_idempotent (p : Imscription × Imscription) : e (e p) = e p := by
  unfold e
  rw [mu_delta_C_id_on_special (μ_C p.1 p.2) rfl rfl]

/-- On the image of the idempotent the round trip verifies: μ_C collapses the
    re-split back to the fused value. This is co-typing, μ ∘ δ = id, holding at
    the Liar point. -/
theorem verify_on_image (p : Imscription × Imscription) :
    μ_C (e p).1 (e p).2 = μ_C p.1 p.2 := by
  unfold e
  exact mu_delta_C_id_on_special (μ_C p.1 p.2) rfl rfl

-- ============================================================
-- §3  THE FORGETFULNESS IS REAL AND LIVES IN  e, NOT IN THE RETURN
-- ============================================================

/-- δ_C splits Φ into 𐑿 (left) and 𐑯 (right); μ_C fuses back to 𐑹. So the two
    halves of a split carry DIFFERENT parity, yet μ_C maps both futures to the
    same 𐑹. The which-way datum is destroyed by the fusion: μ_C is not injective
    in Φ. This is the forgetfulness the sealed system needs — proved by a witness
    where two distinct inputs fuse to the same output. -/
theorem fusion_forgets :
    ∃ x y : Imscription, x.pol ≠ y.pol ∧ μ_C x y = μ_C x x := by
  refine ⟨(δ_C w_stone).1, (δ_C w_stone).2, ?_, ?_⟩
  · decide
  · apply Imscription.ext <;> rfl

/-- Yet the round trip itself loses nothing: on the special class μ_C ∘ δ_C is the
    identity. Verification is lossless; the forgetting is in e = δ ∘ μ, the other
    composite. Both are true at once, of the same pair. -/
theorem return_is_lossless (a : Imscription)
    (hp : a.pol = or') (hc : a.crit = monad) :
    μ_C (δ_C a).1 (δ_C a).2 = a :=
  mu_delta_C_id_on_special a hp hc

-- ============================================================
-- §4  HEADLINE
-- ============================================================

/-- THE LIAR AS IDEMPOTENT POINT.
    (i)   the fusion output is always special, hence a time fixed point;
    (ii)  e = δ ∘ μ is idempotent everywhere (the Liar point);
    (iii) on its image the round trip verifies (co-typing);
    (iv)  the fusion forgets (μ is non-injective), while the return is lossless. -/
theorem liar_is_idempotent :
    (∀ x y, Work (μ_C x y) = μ_C x y) ∧
    (∀ p, e (e p) = e p) ∧
    (∀ p, μ_C (e p).1 (e p).2 = μ_C p.1 p.2) ∧
    (∃ x y : Imscription, x.pol ≠ y.pol ∧ μ_C x y = μ_C x x) :=
  ⟨fusion_is_time, e_idempotent, verify_on_image, fusion_forgets⟩

end Imscribing.LiarIdempotent
