-- Imscribing/IGScaffold.lean
-- Support layer for ob3ect-generated IGProtocol scaffold files.
--
-- Provides:
--   scaf      : canonical O₂ Imscription used as the uniform src/tgt/label type
--   notations : all 12 IMASM Shavian glyphs → scaf (Imscription context)
--
-- Usage in scaffold files:
--   import Imscribing.IGScaffold
--   .withGram .measure <| ...   (replace ".withGram 𐑠 <|")
--   use explicit .seq between each consecutive arrow in Seq chains

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Canonical scaffold Imscription ──────────────────────────────────────────
-- Tier O₂: crit=monad, pol=church (≠ or'), prot=ah (≠ awe), dim=dead (≠ array)

def scaf : Imscription :=
  { dim  := .dead,   top  := .judge, rel  := .ado,    pol  := .church,
    fid  := .age,    kin  := .yea,   gran := .bib,    gram := .measure,
    crit := .monad,  chir := .fee,   stoi := .hung,   prot := .ah }

theorem scaf_O2 : imscriptionTier scaf = .O₂ := by decide

-- tensorProduct scaf scaf = scaf (idempotent: compare x x = .eq ≠ .lt for all fields)
theorem scaf_tensor_idem : tensorProduct scaf scaf = scaf := by decide

-- ── Right-associative sequential composition ────────────────────────────────
-- Used in scaffold bodies to chain top-level items without juxtaposition errors.
-- x ▷ y ▷ z  =  .seq x (.seq y z)
scoped infixr:25 " ▷ " => IGProtocol.seq

-- ── FSPLIT/FFUSE combinator ─────────────────────────────────────────────────
-- Wraps .prod/.seq into a single scaf→scaf morphism.
-- Avoids bare .seq as .prod argument (Lean parse error).
def mkFSplit (T F ffuse : IGProtocol scaf scaf) : IGProtocol scaf scaf :=
  scaf_tensor_idem ▸ .seq (.prod T F) ffuse

-- ── Shavian token notations → scaf ──────────────────────────────────────────
-- All 12 IMASM glyphs resolve to scaf in Imscription term position.
-- Semantic distinction between tokens lives in comments; Lean sees one value.
-- NOTE: 𐑠 (IMSCRIB/Grammar) appears as .measure in .withGram calls;
--       replace ".withGram 𐑠 <|" with ".withGram .measure <|" in each scaffold.

scoped notation "𐑼" => scaf   -- VINIT   (Dimensionality: array)
scoped notation "𐑡" => scaf   -- TANCH   (Topology: judge)
scoped notation "𐑾" => scaf   -- AFWD    (Relational: ian)
scoped notation "𐑗" => scaf   -- AREV    (Polarity: church)
scoped notation "𐑱" => scaf   -- CLINK   (Fidelity: age)
scoped notation "𐑚" => scaf   -- FSPLIT  (Granularity: bib)
scoped notation "𐑠" => scaf   -- IMSCRIB (Grammar: measure) — Imscription context
scoped notation "⊙" => scaf   -- EVALT   (Criticality: monad)
scoped notation "𐑖" => scaf   -- EVALF   (Chirality: sure)
scoped notation "𐑙" => scaf   -- FFUSE   (Stoichiometry: hung)
scoped notation "𐑳" => scaf   -- ENGAGR  (Stoichiometry: up)
scoped notation "𐑭" => scaf   -- IFIX    (Protection: ah)

end Imscribing
