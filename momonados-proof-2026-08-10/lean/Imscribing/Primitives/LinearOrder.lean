-- Imscribing/Primitives/LinearOrder.lean
-- LinearOrder instances for all 12 primitive enum types.
-- Required for min/max to be available (LinearOrder provides Min and Max).

import Imscribing.Primitives.Core
import Mathlib.Order.Lattice

namespace Imscribing.Primitives

-- ============================================================
-- DIMENSIONALITY
-- ============================================================
instance : LinearOrder Dimensionality where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEDimensionality

-- ============================================================
-- RELATIONAL
-- ============================================================
instance : LinearOrder Relational where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLERelational

-- ============================================================
-- GRAMMAR
-- ============================================================
instance : LinearOrder Grammar where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEGrammar

-- ============================================================
-- CHIRALITY
-- ============================================================
instance : LinearOrder Chirality where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEChirality

-- ============================================================
-- PROTECTION
-- ============================================================
instance : LinearOrder Protection where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEProtection

-- ============================================================
-- TOPOLOGY
-- ============================================================
instance : LinearOrder Topology where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLETopology

-- ============================================================
-- POLARITY
-- ============================================================
instance : LinearOrder Polarity where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEPolarity

-- ============================================================
-- CRITICALITY
-- ============================================================
instance : LinearOrder Criticality where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLECriticality

-- ============================================================
-- KINETICCHAR
-- ============================================================
instance : LinearOrder KineticChar where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEKineticChar

-- ============================================================
-- FIDELITY
-- ============================================================
instance : LinearOrder Fidelity where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEFidelity

-- ============================================================
-- GRANULARITY
-- ============================================================
instance : LinearOrder Granularity where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEGranularity

-- ============================================================
-- STOICHIOMETRY
-- ============================================================
instance : LinearOrder Stoichiometry where
  le_refl a := by cases a <;> decide
  le_trans a b c h1 h2 := by revert h1 h2; cases a <;> cases b <;> cases c <;> decide
  le_antisymm a b h1 h2 := by revert h1 h2; cases a <;> cases b <;> decide
  le_total a b := by cases a <;> cases b <;> decide
  lt_iff_le_not_ge a b := by cases a <;> cases b <;> decide
  toDecidableLE := instDecidableLEStoichiometry

end Imscribing.Primitives
