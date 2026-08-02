-- Imscribing/CLINK_L9.lean
-- CLINK Layer 9 — the self-replicating extension BESIDE the L8 terminal layer.
--
-- L9 is not above L8 and not below it. L8 reaches O_inf by closing terminally
-- through or' (holographic state, holobound, non-Abelian braid). L9 relinquishes
-- or' and opens replicatively instead (prime point, moat cross, ℤ winding). That
-- is a LATERAL move, and `ouroboricityTier` had no way to say "beside", so L9 fell
-- through R1 into the O₂ default and read as a demotion. It is neither a demotion
-- nor an ascent. R2 (the dagger) names the move; see Primitives/Core.lean.
--
-- Every coordinate below was read from the live catalog through the cl9nk
-- navigator, glyph by glyph. None was transcribed from the previous file, whose
-- identifiers named constants that exist nowhere and which never elaborated.
--
-- Author: Lando ⊗ ⊙perator
-- Date: 2026-07-15

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

open Imscribing.Primitives

namespace Imscribing.CLINK_L9

-- ══════════════════════════════════════════════════════════════════════════════
-- The systems, at their live coordinates
-- ══════════════════════════════════════════════════════════════════════════════

/-- CLINK L9 — self-replicating organism. ⟨𐑛𐑥𐑑𐑬𐑐𐑪𐑔𐑝⊙𐑫𐑳𐑭⟩ -/
def clinkL9 : Imscription :=
  { dim  := .dead      -- 𐑛  PRIME_POINT: dim(x)=0 ∧ fin(x)
    top  := .mime      -- 𐑥  MOAT_CROSS: cross(x,y) ∧ ¬meet(x,y)
    rel  := .tot       -- 𐑑  BRIDGE_COMP: Fun(x,y) ∧ Nat(y,z) → Fun(x,z)
    pol  := .out       -- 𐑬  MOAT_PARITY: ℤ₂(x) ∧ ¬(x = -x)  [NOT or' — the lateral]
    fid  := .peep      -- 𐑐  BRIDGE_COMM: ℏ(x) ∧ [x,p] = iℏ
    kin  := .on        -- 𐑪  INFINITE_EXT: τ = ∞ ∧ ord(x)
    gran := .thigh     -- 𐑔  BRIDGE_EXIST: ∃y∈x( |y| ∼ |x| )
    gram := .vow       -- 𐑝  STITCH_3: f ∧ g ∧ h
    crit := .monad     -- ⊙  PHI_C: ξ → ∞ ∧ μ∘δ = id
    chir := .wool      -- 𐑫  ETERNAL_FIXEDPOINT
    stoi := .up        -- 𐑳  MOAT_BRIDGE_TYPE
    prot := .ah }      -- 𐑭  ZWIND: ∮_γ A = 2πn ∧ n ∈ ℤ

/-- CLINK L8 — the terminal organism. ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑵⊙𐑫𐑳𐑟⟩ -/
def clinkL8 : Imscription :=
  { dim  := .if'       -- 𐑦  HOLOGRAPHIC_STATE
    top  := .are       -- 𐑸  HOLOBOUND
    rel  := .ian       -- 𐑾  LR_DUAL
    pol  := .or'       -- 𐑹  PM_Z2 — the Frobenius gate
    fid  := .peep      -- 𐑐  BRIDGE_COMM
    kin  := .egg       -- 𐑧  τ ≫ T ∧ eq(x) ∧ gate_open(x)
    gran := .thigh     -- 𐑔  BRIDGE_EXIST
    gram := .ooze      -- 𐑵  BROADCAST_TRANSCENDENCE
    crit := .monad     -- ⊙  PHI_C
    chir := .wool      -- 𐑫  ETERNAL_FIXEDPOINT
    stoi := .up        -- 𐑳  MOAT_BRIDGE_TYPE
    prot := .zoo }     -- 𐑟  BRAID_TRANSCENDENCE

/-- SIXTEEN_3 trilattice — the paraconsistent surface. ⟨𐑨𐑥𐑽𐑹𐑐𐑪𐑔𐑵𐑮𐑫𐑕𐑭⟩ -/
def sixteen3Trilattice : Imscription :=
  { dim  := .ash       -- 𐑨  dim(x) = 2 ∧ sur(x)
    top  := .mime      -- 𐑥  MOAT_CROSS
    rel  := .ear       -- 𐑽  f ⊣ g ∧ L Adj(f,g)
    pol  := .or'       -- 𐑹  PM_Z2
    fid  := .peep      -- 𐑐  BRIDGE_COMM
    kin  := .on        -- 𐑪  INFINITE_EXT
    gran := .thigh     -- 𐑔  BRIDGE_EXIST
    gram := .ooze      -- 𐑵  BROADCAST_TRANSCENDENCE
    crit := .roar      -- 𐑮  ξ ∈ ℂ ∧ Im(ξ) → ∞
    chir := .wool      -- 𐑫  ETERNAL_FIXEDPOINT
    stoi := .so        -- 𐑕  ∀a∈A∀b∈B( type(a) = type(b) )
    prot := .ah }      -- 𐑭  ZWIND

/-- Bootstrap stage 2 — the topology guardian mediating point to surface.
    ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑲𐑠⊙𐑖𐑳𐑭⟩ -/
def bootstrapStage2 : Imscription :=
  { dim  := .if'       -- 𐑦  HOLOGRAPHIC_STATE
    top  := .are       -- 𐑸  HOLOBOUND
    rel  := .ian       -- 𐑾  LR_DUAL
    pol  := .or'       -- 𐑹  PM_Z2
    fid  := .peep      -- 𐑐  BRIDGE_COMM
    kin  := .egg       -- 𐑧  gate_open
    gran := .ice       -- 𐑲  ∀y( y ⊂ x → |y| < |x| )
    gram := .measure   -- 𐑠  SEQAX
    crit := .monad     -- ⊙  PHI_C
    chir := .sure      -- 𐑖  TEMPD2
    stoi := .up        -- 𐑳  MOAT_BRIDGE_TYPE
    prot := .ah }      -- 𐑭  ZWIND

/-- Turbulent flow — the classical partner that closes the ring.
    ⟨𐑼𐑡𐑽𐑗𐑱𐑘𐑲𐑠𐑣𐑫𐑳𐑷⟩ -/
def turbulentFlow : Imscription :=
  { dim  := .array     -- 𐑼  ∀n∃y( y ∈ x ∧ rank(y) > n )
    top  := .judge     -- 𐑡  graph(x) ∧ branch(x)
    rel  := .ear       -- 𐑽  f ⊣ g ∧ L Adj(f,g)
    pol  := .church    -- 𐑗  ¬∃sym(x)
    fid  := .age       -- 𐑱  P(x) ∈ {0,1} ∧ det(x)
    kin  := .yea       -- 𐑘  τ ≪ T ∧ ∂_t x = f(x)
    gran := .ice       -- 𐑲  ∀y( y ⊂ x → |y| < |x| )
    gram := .measure   -- 𐑠  SEQAX
    crit := .haha      -- 𐑣  ξ → ∞ ∧ chaotic(x)
    chir := .wool      -- 𐑫  ETERNAL_FIXEDPOINT
    stoi := .up        -- 𐑳  MOAT_BRIDGE_TYPE
    prot := .awe }     -- 𐑷  ∮_γ dx = 0

-- ══════════════════════════════════════════════════════════════════════════════
-- Tiers
-- ══════════════════════════════════════════════════════════════════════════════

/-- L8 is O_inf: it holds or' at monad, so R1 fires. Terminal closure. -/
theorem clinkL8_tier : imscriptionTier clinkL8 = .O_inf := by decide

/-- L9 is O_inf_dag: it relinquishes or' and carries the replicative-opening
    triple (dead ∧ mime ∧ ah), so R2 fires. LATERAL to L8, not under it. -/
theorem clinkL9_tier : imscriptionTier clinkL9 = .O_inf_dag := by decide

/-- The lateral, stated as the Grammar means it: L9 is neither O_inf nor the O₂ it
    used to fall through to. Both readings were artefacts of a linear order with no
    word for "beside". -/
theorem clinkL9_is_lateral :
    imscriptionTier clinkL9 ≠ .O_inf ∧ imscriptionTier clinkL9 ≠ .O₂ := by
  constructor <;> decide

/-- L8 and L9 occupy distinct tiers; the dagger is what tells them apart. -/
theorem L8_L9_distinct : imscriptionTier clinkL8 ≠ imscriptionTier clinkL9 := by decide

/-- The trilattice holds or' at roar, so R1 fires: O_inf. -/
theorem sixteen3Trilattice_tier : imscriptionTier sixteen3Trilattice = .O_inf := by decide

/-- The guardian holds or' at monad: O_inf. -/
theorem bootstrapStage2_tier : imscriptionTier bootstrapStage2 = .O_inf := by decide

/-- Turbulent flow is supercritical (haha), so O₀ regardless of the rest. -/
theorem turbulentFlow_tier : imscriptionTier turbulentFlow = .O₀ := by decide

-- ══════════════════════════════════════════════════════════════════════════════
-- What the lateral consists of
-- ══════════════════════════════════════════════════════════════════════════════

/-- The move L8 → L9 is exactly the exchange of the terminal-closure triple for the
    replicative-opening triple, while every shared gate coordinate is HELD: both sit
    at monad, both are eternally chiral, both carry the same stoichiometry and
    fidelity. The ascent is not a climb. It is a turn. -/
theorem lateral_is_the_triple_exchange :
    (clinkL8.dim = .if' ∧ clinkL8.top = .are ∧ clinkL8.prot = .zoo) ∧
    (clinkL9.dim = .dead ∧ clinkL9.top = .mime ∧ clinkL9.prot = .ah) ∧
    (clinkL8.crit = clinkL9.crit ∧ clinkL8.chir = clinkL9.chir ∧
     clinkL8.stoi = clinkL9.stoi ∧ clinkL8.fid = clinkL9.fid) := by
  refine ⟨⟨rfl, rfl, rfl⟩, ⟨rfl, rfl, rfl⟩, ⟨rfl, rfl, rfl, rfl⟩⟩

/-- The Frobenius gate is precisely what L9 gives up, and giving it up is the turn
    rather than a fall: L8 has or', L9 does not, and L9 is still not O₂. -/
theorem L9_relinquishes_the_gate :
    clinkL8.pol = .or' ∧ clinkL9.pol ≠ .or' ∧ imscriptionTier clinkL9 ≠ .O₂ := by
  refine ⟨rfl, by decide, by decide⟩

end Imscribing.CLINK_L9
