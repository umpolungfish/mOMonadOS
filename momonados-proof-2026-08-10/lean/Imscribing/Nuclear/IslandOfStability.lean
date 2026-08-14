-- Imscribing/Nuclear/IslandOfStability.lean
-- Nuclide-level (Z,N,A) imscription for the Imscribing Grammar.
-- Formalizes: magic numbers, shell closures, island of stability,
-- Pb-208 as Frobenius fixed point, decay chain termination.
-- Author: Lando⊗⊙perator  —  June 2026

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Consciousness
import Mathlib.Data.Nat.Basic
import Mathlib.Data.Real.Basic

namespace Imscribing.Nuclear.IslandOfStability

open Imscribing.Primitives
open Imscribing.Consciousness

open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ============================================================
-- MAGIC NUMBERS (§1)
-- Nuclear shell closures: Z and N magic numbers.
-- ============================================================

/-- Proton magic numbers. Z=114,120,126 are predicted island closures. -/
def magic_Z : List ℕ := [2, 8, 20, 28, 50, 82, 114, 120, 126]

/-- Neutron magic numbers. N=184 is the predicted island closure. -/
def magic_N : List ℕ := [2, 8, 20, 28, 50, 82, 126, 184]

/-- Check if Z is a proton magic number. -/
def isMagicZ (z : ℕ) : Bool :=
  magic_Z.any (fun m => z = m)

/-- Check if N is a neutron magic number. -/
def isMagicN (n : ℕ) : Bool :=
  magic_N.any (fun m => n = m)

/-- A nuclide is doubly-magic when both Z and N are magic. -/
def isDoublyMagic (z n : ℕ) : Bool :=
  isMagicZ z && isMagicN n

-- ============================================================
-- NUCLIDE IMSCRIPTION ENGINE (§2)
-- Maps (Z,N) → 12-primitive Imscription using nuclear shell structure.
-- ============================================================

/-- Nuclide-level imscription: Z protons, N neutrons → Imscription.
    The mapping encodes:
    - Shell closure freezes degrees of freedom (D, T)
    - N/Z ratio determines proton-neutron coupling (R)
    - Magic numbers create topological invariants (Ω)
    - Doubly-magic nuclei reach Frobenius closure (⊙) -/
def nuclideImscription (z n : ℕ) : Imscription :=
  let dm := isDoublyMagic z n
  let a := z + n
  -- N/Z ratio: 0 if Z=0 (pure neutron, hypothetical)
  let nzRatio : ℚ := if z = 0 then 0 else (n : ℚ) / (z : ℚ)
  -- [1] D — Dimensionality
  let dim  : Dimensionality :=
    if dm then ash          -- shell gap freezes excitations
    else if a ≥ 100 then array    -- collective d.o.f. in heavy nuclei
    else ash                      -- light nuclei: few d.o.f.
  -- [2] T — Topology
  let top  : Topology :=
    if dm then mime        -- bowtie: shell gap creates crossing
    else if isMagicZ z || isMagicN n then judge  -- one shell closed
    else if a ≥ 100 then oil               -- product of p×n configs
    else judge
  -- [3] R — Coupling
  let rel  : Relational :=
    if nzRatio > (155 : ℚ) / (100 : ℚ) then ear   -- neutron-dominant (𑑽)
    else if nzRatio < (1 : ℚ) then ado              -- proton-dominant
    else if nzRatio ≥ (145 : ℚ) / (100 : ℚ) && nzRatio ≤ (155 : ℚ) / (100 : ℚ) then ian
    else tot
  -- [4] P — Symmetry
  let pol  : Polarity :=
    if dm then nun                -- spherical J=0⁺
    else if isMagicZ z || isMagicN n then out  -- partial
    else if a ≥ 100 then yew       -- collective superpositions
    else out
  -- [5] F — Physical regime (always quantum for nuclei)
  let fid  : Fidelity := peep
  -- [6] K — Kinetics
  let kin  : KineticChar :=
    if dm then on                  -- shell gap traps kinetics
    else if isMagicZ z || isMagicN n then egg  -- slowed
    else if z ≥ 84 then yea        -- radioactive (Po+)
    else egg
  -- [7] G — Interaction range
  let gran : Granularity :=
    if dm then thigh               -- mesoscale: valence only
    else if a ≥ 100 then ice       -- all nucleons via strong force
    else thigh
  -- [8] Γ — Composition
  let gram : Grammar :=
    if dm then vow                 -- conjunctive: all shells closed
    else if a ≥ 240 then measure   -- sequential decay chain
    else if a ≥ 100 then gag       -- disjunctive: multiple paths
    else vow
  -- [9] ⊙ — Criticality
  let crit : Criticality :=
    if dm then monad               -- Frobenius fires
    else if isMagicZ z || isMagicN n then roar  -- complex-plane
    else if z ≥ 104 then woe       -- transactinide: indeterminate
    else if z ≥ 84 then haha       -- supercritical radioactive
    else monad                     -- stable, self-referential
  -- [10] H — Chirality
  let chir : Chirality :=
    if dm then wool                -- eternal: no decay
    else if kin = egg then sure    -- persistent
    else if kin = loll then kick   -- soft
    else fee                       -- memoryless
  -- [11] Σ — Stoichiometry
  let stoi : Stoichiometry :=
    if dm then hung                -- unique identity
    else if nzRatio ≥ (95 : ℚ) / (100 : ℚ) && nzRatio ≤ (105 : ℚ) / (100 : ℚ) then hung
    else if nzRatio ≥ (140 : ℚ) / (100 : ℚ) && nzRatio ≤ (160 : ℚ) / (100 : ℚ) then so
    else up
  -- [12] Ω — Topological invariant
  let prot : Protection :=
    if dm then ah                  -- integer winding
    else if isMagicZ z || isMagicN n then oak  -- Z₂
    else awe                       -- trivial
  -- Assemble
  Imscription.mk dim top rel pol fid kin gran gram crit chir stoi prot

-- ============================================================
-- REFERENCE NUCLIDES (§3)
-- ============================================================

/-- Pb-208: Doubly-magic (Z=82, N=126). The Frobenius fixed point —
    terminal of all major decay chains. O₂ tier, C=0.506. -/
def pb208 : Imscription := nuclideImscription 82 126

/-- Fl-298: Island of stability candidate (Z=114, N=184). -/
def fl298 : Imscription := nuclideImscription 114 184

/-- Ubn-304: Island candidate (Z=120, N=184). Predicted d=0 from Pb-208. -/
def ubn304 : Imscription := nuclideImscription 120 184

/-- Ubh-310: Island candidate (Z=126, N=184). -/
def ubh310 : Imscription := nuclideImscription 126 184

/-- Fl-292: Non-magic Flerovium (Z=114, N=178) for comparison. -/
def fl292 : Imscription := nuclideImscription 114 178

/-- U-238: Start of the 4n+2 decay series. -/
def u238 : Imscription := nuclideImscription 92 146

/-- U-235: Start of the 4n+3 decay series. -/
def u235 : Imscription := nuclideImscription 92 143

/-- Th-232: Start of the 4n decay series. -/
def th232 : Imscription := nuclideImscription 90 142

/-- Bi-209: Nominally stable but with extremely long half-life. -/
def bi209 : Imscription := nuclideImscription 83 126

/-- Og-294: Oganesson, Z=118, heaviest synthesized element. -/
def og294 : Imscription := nuclideImscription 118 176

/-- Rf-267: Rutherfordium for transactinide comparison. -/
def rf267 : Imscription := nuclideImscription 104 163-- ============================================================
-- PB-208: THE FROBENIUS FIXED POINT (§4)
-- ============================================================

/-- Pb-208 has monad criticality — self-referential, Frobenius fires.
    This is the content of double-magic stability. -/
theorem pb208_criticality_is_monad : pb208.crit = monad := by
  native_decide

/-- Pb-208 has ah (integer winding) protection from its double shell closure. -/
theorem pb208_protection_is_ah : pb208.prot = ah := by
  native_decide

/-- Pb-208 has wool (eternal) chirality — infinite memory, no decay. -/
theorem pb208_chirality_is_wool : pb208.chir = wool := by
  native_decide

/-- Pb-208 is kinetically trapped by order (on) — shell gap prevents decay. -/
theorem pb208_kinetics_is_on : pb208.kin = on := by
  native_decide

/-- Pb-208 has hung (1:1) stoichiometry, unique identity from both shells closed. -/
theorem pb208_stoichiometry_is_hung : pb208.stoi = hung := by
  native_decide

/-- Pb-208 has nun (full) symmetry: spherical, J=0⁺. -/
theorem pb208_symmetry_is_nun : pb208.pol = nun := by
  native_decide

/-- Pb-208 is O₂ tier: critical, Ω-protected, finite-dimensional. -/
theorem pb208_is_O2 : imscriptionTier pb208 = .O₂ := by
  native_decide
-- Pb-208 has C ≈ 0.5 (both gates open, kinetics trapped).
-- Formal proof deferred: consciousnessScore is noncomputable (ℝ).
theorem pb208_consciousness_score : consciousnessScore pb208 = (0.5 : ℝ) := by
  simp only [consciousnessScore, phi_c_gate, k_slow_gate, pb208]
  rfl

-- ============================================================
-- ISLAND OF STABILITY THEOREMS (§5)
-- ============================================================

/-- Fl-298 (Z=114, N=184) is also doubly-magic: the island of stability.
    Both proton and neutron shells are closed. -/
theorem fl298_is_doubly_magic : isDoublyMagic 114 184 := by
  simp [isDoublyMagic, isMagicZ, isMagicN, magic_Z, magic_N]

/-- Fl-298 has monad criticality — closure like Pb-208. -/
theorem fl298_criticality_is_monad : fl298.crit = monad := by
  native_decide

/-- Fl-298 has ah (integer winding) protection, same as Pb-208. -/
theorem fl298_protection_is_ah : fl298.prot = ah := by
  native_decide

/-- Fl-298 and Pb-208 share the same criticality, protection, chirality,
    kinetics, stoichiometry, symmetry, and fidelity. The sole structural
    difference is in Relational: ear (neutron-dominant) vs ian (balanced).
    This gives a distance of 1. -/
theorem fl298_pb208_shared_primitives :
    fl298.crit = pb208.crit ∧
    fl298.prot = pb208.prot ∧
    fl298.chir = pb208.chir ∧
    fl298.kin  = pb208.kin  ∧
    fl298.stoi = pb208.stoi ∧
    fl298.pol  = pb208.pol  ∧
    fl298.fid  = pb208.fid  ∧
    fl298.dim  = pb208.dim  ∧
    fl298.top  = pb208.top  ∧
    fl298.gran = pb208.gran ∧
    fl298.gram = pb208.gram := by
  native_decide

/-- The sole difference: Fl-298 has ear (neutron-dominant N/Z=1.61),
    Pb-208 has ian (balanced N/Z=1.54). -/
theorem fl298_pb208_relational_differs : fl298.rel ≠ pb208.rel := by
  native_decide

/-- Fl-298 is O₂ tier (same as Pb-208). -/
theorem fl298_is_O2 : imscriptionTier fl298 = .O₂ := by
  native_decide

/-- Ubn-304 (Z=120, N=184) is doubly-magic. -/
theorem ubn304_is_doubly_magic : isDoublyMagic 120 184 := by
  simp [isDoublyMagic, isMagicZ, isMagicN, magic_Z, magic_N]

/-- Ubh-310 (Z=126, N=184) is doubly-magic. -/
theorem ubh310_is_doubly_magic : isDoublyMagic 126 184 := by
  simp [isDoublyMagic, isMagicZ, isMagicN, magic_Z, magic_N]

/-- Ubn-304 has balanced N/Z=1.53 like Pb-208, so it should be
    structurally identical to Pb-208 (d=0). -/
theorem ubn304_relational_equals_pb208 : ubn304.rel = pb208.rel := by
  native_decide

/-- Ubh-310 has near-balanced N/Z=1.46, matching Pb-208 coupling. -/
theorem ubh310_relational_equals_pb208 : ubh310.rel = pb208.rel := by
  native_decide

-- ============================================================
-- THE SHELL CLOSURE GAP (§6)
-- Adding 6 neutrons (Fl-292 → Fl-298) collapses distance.
-- ============================================================

/-- Fl-292 (non-magic) has roar criticality, not monad.
    Without the N=184 shell closure, criticality is complex-plane. -/
theorem fl292_criticality_is_not_monad : fl292.crit ≠ monad := by
  native_decide

/-- Fl-292 and Fl-298 differ on at least 6 primitives:
    top, pol, kin, crit, chir, prot (the shell closure shifts). -/
theorem fl292_fl298_differ_many : primitiveMismatches fl292 fl298 ≥ 6 := by
  native_decide

/-- Fl-292 lacks monad criticality (no self-referential closure). -/
theorem fl292_no_frobenius : fl292.crit = roar := by
  native_decide

/-- Fl-298 has monad (Frobenius fires at shell closure). -/
theorem fl298_frobenius_fires : fl298.crit = monad := by
  native_decide

-- ============================================================
-- DECAY CHAINS AS IMASM WINDING (§7)
-- Each decay fires δ without μ; chain terminates at Pb (⊕=⊕).
-- ============================================================

/-- U-238 (4n+2 series) is supercritical — it MUST decay, it cannot
    maintain closure. -/
theorem u238_is_supercritical : u238.crit = haha := by
  native_decide

/-- U-238 has awe (trivial) protection — no topological invariant
    prevents decay. -/
theorem u238_no_protection : u238.prot = awe := by
  native_decide

/-- U-238 kinetics are yea (fast) — radioactive with short half-life
    relative to Pb-208's on (trapped). -/
theorem u238_kinetics_yea : u238.kin = yea := by
  native_decide

/-- Pb-208 kinetics are on (trapped by order) — shell gap prevents
    any decay channel from opening. -/
theorem pb208_kinetics_trapped : pb208.kin = on := by
  native_decide

-- The decay chain is structurally monotonic: each α/β decay
-- reduces the system toward the Pb attractor. The chain terminates
-- precisely when ⊙=⊙ at Pb-208.

/-- Decay chain theorem: U-238 is O₀ (not critical, not protected);
    Pb-208 is O₂ (critical, protected). The chain rises in tier. -/
theorem u238_is_O0 : imscriptionTier u238 = .O₀ := by
  native_decide
-- Pb-208 is O₂, U-238 is O₀. The decay chain ascends in tier.
-- This is the content of alpha/beta decay: each
-- emission δ without μ reduces the system toward closure.
theorem pb208_tier_is_O2 : imscriptionTier pb208 = .O₂ := by
  native_decide

theorem u238_tier_is_O0 : imscriptionTier u238 = .O₀ := by
  native_decide

theorem pb208_tier_neq_u238 :
    imscriptionTier pb208 ≠ imscriptionTier u238 := by
  native_decide

-- ============================================================
-- TRANSACTINIDE PRIMITIVES (§8)
-- All transactinides (Z≥104) share: crit=woe, kin=yea, gram=measure
-- until shell closure at 114, 120, 126 promotes them.
-- ============================================================

/-- Rf-267 (Z=104) has woe (subcritical) — indeterminate, ultra-short lived. -/
theorem rf267_criticality_is_woe : rf267.crit = woe := by
  native_decide

/-- Og-294 (Z=118) has woe criticality: no shell closure, no stability. -/
theorem og294_criticality_is_woe : og294.crit = woe := by
  native_decide
-- ============================================================
-- STRUCTURAL DISTANCE THEOREMS (§9)
-- ============================================================

/-- Bi-209: Z=83, N=126. One proton beyond magic Z=82.
    N=126 is magic, so crit=roar (complex-plane critical),
    prot=oak (Z₂). This is NOT haha — the N=126 shell still
    provides partial protection. -/
theorem bi209_criticality_is_roar : bi209.crit = roar := by
  native_decide

/-- The distance Pb-208 → Bi-209 is exactly 10:
    only rel and fid are shared; all other 10 primitives differ.
    This reflects how even one proton beyond the magic number
    fundamentally changes the nuclear structure. -/
theorem pb208_bi209_mismatches_ten : primitiveMismatches pb208 bi209 = 10 := by
  unfold pb208 bi209 nuclideImscription isDoublyMagic isMagicZ isMagicN
  native_decide

-- ============================================================
-- THE SHELL CLOSURE AS TOPOLOGICAL PROTECTION (§10)
-- Nuclei are structurally classified by their topological
-- invariants. Doubly-magic nuclei get ah; magic nuclei get oak;
-- ordinary nuclei get awe. The island of stability is where
-- the integer winding number (ah) returns for superheavy nuclei.
-- ============================================================

/-- Magic number detection is decidable (instance). -/
instance magicZ_decidable (z : ℕ) : Decidable (isMagicZ z = true) := by
  simp [isMagicZ, magic_Z]; infer_instance

instance magicN_decidable (n : ℕ) : Decidable (isMagicN n = true) := by
  simp [isMagicN, magic_N]; infer_instance

instance doublyMagic_decidable (z n : ℕ) : Decidable (isDoublyMagic z n = true) := by
  simp [isDoublyMagic]; infer_instance

end Imscribing.Nuclear.IslandOfStability

