import Mathlib

namespace Imscribing.NumberTheory

mutual
  inductive PrattTree : ℕ → Type
    | leaf : (n : ℕ) → PrattTree n
    | node : (n : ℕ) → PrattForest → PrattTree n

  inductive PrattForest : Type
    | nil  : PrattForest
    | cons : (m : ℕ) → PrattTree m → PrattForest → PrattForest
end

inductive TreeMem {m : ℕ} (t : PrattTree m) : PrattForest → Prop
  | here  (rest : PrattForest) : TreeMem t (.cons m t rest)
  | there {k : ℕ} {t' : PrattTree k} {rest : PrattForest} :
      TreeMem t rest → TreeMem t (.cons k t' rest)

mutual
  def depth {n : ℕ} : PrattTree n → ℕ
    | .leaf _ => 1
    | .node _ f => 1 + depthForest f
  termination_by t => sizeOf t

  def depthForest : PrattForest → ℕ
    | .nil => 0
    | .cons _ t rest => max (depth t) (depthForest rest)
  termination_by f => sizeOf f
end

theorem pratt_stratification {n : ℕ} (f : PrattForest) (m : ℕ) (t : PrattTree m)
    (h : TreeMem t f) : depth t < depth (.node n f : PrattTree n) := by
  simp [depth]
  -- Goal: depth t < 1 + depthForest f
  have hle : depth t ≤ depthForest f := by
    induction h with
    | here rest =>
      unfold depthForest
      exact le_max_left _ _
    | there hmem ih =>
      unfold depthForest
      exact le_trans ih (le_max_right _ _)
  have hlt : depthForest f < 1 + depthForest f := by omega
  exact lt_of_le_of_lt hle hlt

theorem depth_positive {n : ℕ} (cert : PrattTree n) : 0 < depth cert := by
  match cert with
  | .leaf _ => simp [depth]
  | .node _ f => simp [depth]

end Imscribing.NumberTheory
