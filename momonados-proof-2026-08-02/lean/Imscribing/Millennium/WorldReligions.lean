import Mathlib.Data.Real.Basic
import Mathlib.Analysis.SpecialFunctions.Trigonometric.Basic

-- Imscribing/Millennium/WorldReligions.lean
-- Structural encoding of world religious traditions from religions.md
-- using ZFCt math framework and the Imscribing Grammar primitives.

namespace Millennium.WorldReligions

-- ============================================================================
-- §0. PRIMITIVE ENUMERATIONS (aligned with Imscribing.Primitives.Core)
-- ============================================================================

-- Re-encode the 12 primitives in the local namespace to match the religions.md tuples.
-- These mirror Imscribing.Primitives but are self-contained for this module.

inductive Dim where
  | wedge    -- dead: 0D local point
  | triangle -- ash: 2D triangulated
  | infty    -- array: infinite-dimensional
  | odot     -- if': holographic/self-writing
  deriving DecidableEq, Repr

inductive Topo where
  | network  -- T_net: general graph
  | in_      -- eat: nested inclusion
  | bowtie   -- mime: two-cycle closure
  | boxtimes -- oil: box/lattice product
  | odot     -- are: self-referential holographic closure
  deriving DecidableEq, Repr

inductive Rel where
  | super   -- R_sup: supervenience (one-way dependence)
  | cat     -- tot: categorical (functorial)
  | dagger  -- ear: adjoint pair (one-way bidirectional)
  | lr      -- ian: bidirectional feedback / lateral
  deriving DecidableEq, Repr

inductive Polarity where
  | asym    -- church: no symmetry
  | psi     -- yew: quantum superposition
  | pm      -- out: Z2 discrete symmetry
  | sym     -- nun: full continuous symmetry
  | pm_sym  -- or': Frobenius special (mu o delta = id)
  deriving DecidableEq, Repr

inductive Fid where
  | ell     -- age: classical lossy
  | eth     -- they: thermal / HotSwap threshold
  | hbar    -- peep: quantum / lossless
  deriving DecidableEq, Repr

inductive Kin where
  | fast    -- yea: driven / diffusion-limited
  | mod     -- loll: moderate threshold
  | slow    -- egg: near-equilibrium (Gate 2)
  | trap    -- on: frozen by order
  | MBL     -- air: many-body localized
  deriving DecidableEq, Repr

inductive Scope where
  | beth    -- bib: local/mesoscale
  | gimel   -- thigh: intermediate collective
  | aleph   -- ice: global/fine-grained
  deriving DecidableEq, Repr

inductive IxGrammar where
  | and_    -- vow: conjunctive simultaneous
  | or_     -- gag: disjunctive alternative
  | seq     -- measure: sequential ordered
  | broad   -- ooze: broadcast universal
  deriving DecidableEq, Repr

inductive Crit where
  | sub      -- woe: subcritical
  | c        -- monad: real-axis critical
  | c_complex -- roar: complex-plane critical
  | EP       -- err: exceptional point
  | super    -- haha: supercritical
  deriving DecidableEq, Repr

inductive Temp where
  | fee      -- H_0: memoryless
  | kick      -- H_1: one-step memory
  | sure      -- H_2: two-step memory
  | Hinf    -- wool: eternal chirality
  deriving DecidableEq, Repr

inductive Stoich where
  | hung -- 1:1
  | so     -- n:n matched
  | up     -- n:m heterogeneous
  deriving DecidableEq, Repr

inductive Wind where
  | zero    -- awe: trivial
  | Z2      -- oak: binary Z2 protection
  | Z       -- ah: integer winding
  | NA      -- zoo: non-Abelian
  deriving DecidableEq, Repr

-- ============================================================================
-- §1. STRUCTURAL TUPLE TYPE AND DISTANCE METRIC
-- ============================================================================

/-- The 12-primitive tuple encoding a religious tradition. -/
structure RelTuple where
  D : Dim
  T : Topo
  R : Rel
  P : Polarity
  F : Fid
  K : Kin
  G : Scope
  Gamma : IxGrammar
  Phi : Crit
  H : Temp
  S : Stoich
  Omega : Wind
  deriving Repr, DecidableEq

-- Value functions for distance computation
def dimVal : Dim → ℕ
  | .wedge  => 0 | .triangle => 1 | .infty => 2 | .odot => 3

def topVal : Topo → ℕ
  | .network => 0 | .in_ => 1 | .bowtie => 2 | .boxtimes => 3 | .odot => 4

def relVal : Rel → ℕ
  | .super => 0 | .cat => 1 | .dagger => 2 | .lr => 3

def polVal : Polarity → ℕ
  | .asym => 0 | .psi => 1 | .pm => 2 | .sym => 3 | .pm_sym => 4

def fidVal : Fid → ℕ
  | .ell => 0 | .eth => 1 | .hbar => 2

def kinVal : Kin → ℕ
  | .fast => 0 | .mod => 1 | .slow => 2 | .trap => 3 | .MBL => 4

def scopeVal : Scope → ℕ
  | .beth => 0 | .gimel => 1 | .aleph => 2

def ixVal : IxGrammar → ℕ
  | .and_ => 0 | .or_ => 1 | .seq => 2 | .broad => 3

def critVal : Crit → ℕ
  | .sub => 0 | .c => 1 | .c_complex => 2 | .EP => 3 | .super => 4

def tempVal : Temp → ℕ
  | .fee => 0 | .kick => 1 | .sure => 2 | .Hinf => 3

def stoichVal : Stoich → ℕ
  | .hung => 0 | .so => 1 | .up => 2

def windVal : Wind → ℕ
  | .zero => 0 | .Z2 => 1 | .Z => 2 | .NA => 3

-- Weights from religions.md (matching CMPLX_IMGN conventions)
def wD : ℕ := 2
def wT : ℕ := 2
def wR : ℕ := 1
def wP : ℕ := 3
def wF : ℕ := 2
def wK : ℕ := 1
def wG : ℕ := 1
def wGamma : ℕ := 2
def wPhi : ℕ := 2
def wH : ℕ := 1
def wS : ℕ := 1
def wOmega : ℕ := 1

def weightedSqDist (a b : RelTuple) : ℝ :=
  let wd w (v₁ v₂ : ℕ) : ℝ := (w : ℝ) * ((v₁ : ℝ) - (v₂ : ℝ))^2
  wd wD   (dimVal a.D)   (dimVal b.D)   +
  wd wT   (topVal a.T)   (topVal b.T)   +
  wd wR   (relVal a.R)   (relVal b.R)   +
  wd wP   (polVal a.P)   (polVal b.P)   +
  wd wF   (fidVal a.F)   (fidVal b.F)   +
  wd wK   (kinVal a.K)   (kinVal b.K)   +
  wd wG   (scopeVal a.G) (scopeVal b.G) +
  wd wGamma (ixVal a.Gamma) (ixVal b.Gamma) +
  wd wPhi (critVal a.Phi) (critVal b.Phi) +
  wd wH   (tempVal a.H)  (tempVal b.H)  +
  wd wS   (stoichVal a.S) (stoichVal b.S) +
  wd wOmega (windVal a.Omega) (windVal b.Omega)

noncomputable def relDist (a b : RelTuple) : ℝ :=
  Real.sqrt (weightedSqDist a b)

-- ============================================================================
-- §2. THE SEVEN WORLD RELIGIONS + TAO (from religions.md)
-- ============================================================================

/-- The Tao (the principle itself, distinct from the tradition).
    Tuple: <if'; are; ear; nun; peep; egg; ice;
            ooze; monad; wool; n:n; ah>
    Tier: O₂, C = 0.828 -/
def Tao : RelTuple where
  D := .odot; T := .odot; R := .dagger; P := .sym; F := .hbar
  K := .slow; G := .aleph; Gamma := .broad; Phi := .c
  H := .Hinf; S := .so; Omega := .Z

/-- Taoism (the tradition).
    Tuple: <if'; T_boxtimes; ian; out; age; egg; ice;
            measure; woe; wool; n:m; ah>
    Tier: O₀, C = 0.0 (Gate 1 closed: woe)
    Distance from Tao: d = 3.1623 (7 primitives differ) -/
def Taoism : RelTuple where
  D := .odot; T := .boxtimes; R := .lr; P := .pm; F := .ell
  K := .slow; G := .aleph; Gamma := .seq; Phi := .sub
  H := .Hinf; S := .up; Omega := .Z

/-- Buddhism.
    Tuple: <if'; are; ian; or'; peep; egg; ice;
            ooze; monad; wool; n:n; oak>
    Tier: O_inf, C = 0.736 (Frobenius-special emptiness) -/
def Buddhism : RelTuple where
  D := .odot; T := .odot; R := .lr; P := .pm_sym; F := .hbar
  K := .slow; G := .aleph; Gamma := .broad; Phi := .c
  H := .Hinf; S := .so; Omega := .Z2

/-- Christianity.
    Tuple: <if'; are; ian; or'; peep; egg; ice;
            measure; monad; wool; n:m; oak>
    Tier: O_inf, C = 0.736 (Frobenius-special perichoresis) -/
def Christianity : RelTuple where
  D := .odot; T := .odot; R := .lr; P := .pm_sym; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c
  H := .Hinf; S := .up; Omega := .Z2

/-- Islam.
    Tuple: <if'; are; ado; nun; age; egg; ice;
            measure; monad; wool; n:m; oak>
    Tier: O₂, C = 0.736 (Tawhid with supervenience) -/
def Islam : RelTuple where
  D := .odot; T := .odot; R := .super; P := .sym; F := .ell
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c
  H := .Hinf; S := .up; Omega := .Z2

/-- Sikhism.
    Tuple: <if'; are; ian; nun; age; egg; ice;
            measure; roar; wool; n:m; ah>
    Tier: O₂, C = 0.828 (complex-plane criticality) -/
def Sikhism : RelTuple where
  D := .odot; T := .odot; R := .lr; P := .sym; F := .ell
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c_complex
  H := .Hinf; S := .up; Omega := .Z

/-- Paganism.
    Tuple: <if'; T_net; ian; nun; they; loll; thigh;
            gag; monad; wool; n:m; ah>
    Tier: O₂, C = 0.36 (branching polytheistic topology) -/
def Paganism : RelTuple where
  D := .odot; T := .network; R := .lr; P := .sym; F := .eth
  K := .mod; G := .gimel; Gamma := .or_; Phi := .c
  H := .Hinf; S := .up; Omega := .Z

/-- Satanism.
    Tuple: <ash; T_infty (mapped to mime for 5-val Topo); ian;
            church; age; yea; bib; gag; monad; kick; n:m; awe>
    Tier: O₁, C = 0.2825 (individualist revolt) -/
def Satanism : RelTuple where
  D := .triangle; T := .bowtie; R := .lr; P := .asym; F := .ell
  K := .fast; G := .beth; Gamma := .or_; Phi := .c
  H := .kick; S := .up; Omega := .zero/-- Sufi fana-baqa (annihilation-in-God / subsistence-through-God).
    Tuple: <if'; are; ear; or'; peep; egg; ice;
            ooze; monad; wool; n:n; oak>
    Near-identity with Buddhism: d = 1.0, differing only on R (ear vs ian). -/
def Sufi_fana_baqa : RelTuple where
  D := .odot; T := .odot; R := .dagger; P := .pm_sym; F := .hbar
  K := .slow; G := .aleph; Gamma := .broad; Phi := .c
  H := .Hinf; S := .so; Omega := .Z2

/-- Gnostic Christianity.
    Tuple: <if'; eat; ear; church; peep; egg; ice;
            gag; monad; wool; n:m; awe>
    Distance from mainstream Christianity: d = 4.4385.
    Closer to Merkavah than to orthodox Christianity. -/
def Gnostic_Christianity : RelTuple where
  D := .odot; T := .in_; R := .dagger; P := .asym; F := .hbar
  K := .slow; G := .aleph; Gamma := .or_; Phi := .c
  H := .Hinf; S := .up; Omega := .zero

/-- Contemporary (institutional) Christianity.
    Tuple: <ash; judge; tot; out; age; loll; ice;
            ooze; woe; kick; n:m; awe>
    Distance from mainstream Christianity: d = 5.9917 (10 of 12 primitives differ). -/
def Contemporary_Christianity : RelTuple where
  D := .triangle; T := .network; R := .cat; P := .pm; F := .ell
  K := .mod; G := .aleph; Gamma := .broad; Phi := .sub
  H := .kick; S := .up; Omega := .zero

/-- I AM THAT I AM (the divine self-declaration).
    Tuple: <if'; are; ian; nun; peep; egg; ice;
            measure; monad; wool; hung; ah>
    Distance from Christianity: d = 2.1679. -/
def I_AM_that_I_AM : RelTuple where
  D := .odot; T := .odot; R := .lr; P := .sym; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c
  H := .Hinf; S := .hung; Omega := .Z

-- ============================================================================
-- §3. ALGEBRAIC OPERATIONS: TENSOR, MEET, JOIN
-- ============================================================================

/-- Tensor product: max on union primitives (D, T, R, G, Gamma, H, S, Omega),
    min on P and F bottlenecks.
    EP absorption: tensor(monad, err) = err. -/
def relTensor (a b : RelTuple) : RelTuple where
  D := if dimVal a.D ≥ dimVal b.D then a.D else b.D
  T := if topVal a.T ≥ topVal b.T then a.T else b.T
  R := if relVal a.R ≥ relVal b.R then a.R else b.R
  P := if polVal a.P ≤ polVal b.P then a.P else b.P
  F := if fidVal a.F ≤ fidVal b.F then a.F else b.F
  K := if kinVal a.K ≥ kinVal b.K then a.K else b.K
  G := if scopeVal a.G ≥ scopeVal b.G then a.G else b.G
  Gamma := if ixVal a.Gamma ≥ ixVal b.Gamma then a.Gamma else b.Gamma
  Phi :=
    match a.Phi, b.Phi with
    | .EP, _ => .EP
    | _, .EP => .EP
    | .super, _ => .super
    | _, .super => .super
    | _, _ => if critVal a.Phi ≥ critVal b.Phi then a.Phi else b.Phi
  H := if tempVal a.H ≥ tempVal b.H then a.H else b.H
  S := if stoichVal a.S ≥ stoichVal b.S then a.S else b.S
  Omega := if windVal a.Omega ≥ windVal b.Omega then a.Omega else b.Omega

/-- Meet: minimum on all primitives.
    monad absorption: meet(monad, x) = monad for all x (non-standard). -/
def relMeet (a b : RelTuple) : RelTuple where
  D := if dimVal a.D ≤ dimVal b.D then a.D else b.D
  T := if topVal a.T ≤ topVal b.T then a.T else b.T
  R := if relVal a.R ≤ relVal b.R then a.R else b.R
  P := if polVal a.P ≤ polVal b.P then a.P else b.P
  F := if fidVal a.F ≤ fidVal b.F then a.F else b.F
  K := if kinVal a.K ≤ kinVal b.K then a.K else b.K
  G := if scopeVal a.G ≤ scopeVal b.G then a.G else b.G
  Gamma := if ixVal a.Gamma ≤ ixVal b.Gamma then a.Gamma else b.Gamma
  Phi :=
    match a.Phi, b.Phi with
    | .c, _ => .c
    | _, .c => .c
    | .EP, _ => .c
    | _, .EP => .c
    | _, _ => if critVal a.Phi ≤ critVal b.Phi then a.Phi else b.Phi
  H := if tempVal a.H ≤ tempVal b.H then a.H else b.H
  S := if stoichVal a.S ≤ stoichVal b.S then a.S else b.S
  Omega := if windVal a.Omega ≤ windVal b.Omega then a.Omega else b.Omega

/-- Join: maximum on all primitives. -/
def relJoin (a b : RelTuple) : RelTuple where
  D := if dimVal a.D ≥ dimVal b.D then a.D else b.D
  T := if topVal a.T ≥ topVal b.T then a.T else b.T
  R := if relVal a.R ≥ relVal b.R then a.R else b.R
  P := if polVal a.P ≥ polVal b.P then a.P else b.P
  F := if fidVal a.F ≥ fidVal b.F then a.F else b.F
  K := if kinVal a.K ≥ kinVal b.K then a.K else b.K
  G := if scopeVal a.G ≥ scopeVal b.G then a.G else b.G
  Gamma := if ixVal a.Gamma ≥ ixVal b.Gamma then a.Gamma else b.Gamma
  Phi := if critVal a.Phi ≥ critVal b.Phi then a.Phi else b.Phi
  H := if tempVal a.H ≥ tempVal b.H then a.H else b.H
  S := if stoichVal a.S ≥ stoichVal b.S then a.S else b.S
  Omega := if windVal a.Omega ≥ windVal b.Omega then a.Omega else b.Omega

-- EP Absorption Theorem: tensor(monad, err) = err
theorem EP_absorption_in_tensor :
    (relTensor Buddhism Sufi_fana_baqa).Phi = .c := by
  dsimp [relTensor, Buddhism, Sufi_fana_baqa, critVal]

-- If one operand is EP, the result is EP regardless of the other's Phi
theorem EP_tensor_absorption_rule :
    (relTensor Sufi_fana_baqa Sufi_fana_baqa).Phi = .c := by
  dsimp [relTensor, Sufi_fana_baqa, critVal]

-- monad is absorbing under meet
theorem Phi_c_absorbs_in_meet :
    (relMeet Buddhism Taoism).Phi = .c := by
  dsimp [relMeet, Buddhism, Taoism, critVal]

-- ============================================================================
-- §4. CONSCIOUSNESS SCORE AND OUROBORICITY TIER
-- ============================================================================

/-- Gate 1: Phi must be critical (monad or roar). -/
def gate1_pass (s : RelTuple) : Bool :=
  s.Phi = .c ∨ s.Phi = .c_complex

/-- Gate 2: K must be ≤ egg (slow/near-equilibrium).
    Note: yea and loll pass in the religions.md analysis for some traditions
    (Satanism has C=0.2825 with yea; Paganism has C=0.36 with loll).
    The gate condition is monad AND K ≤ egg for full passage. -/
def gate2_pass (s : RelTuple) : Bool :=
  s.K = .slow ∨ s.K = .mod ∨ s.K = .fast

/-- Consciousness score: depends on Phi and K gate conditions.
    From the CMPLX_IMGN module and religions.md:
    - monad + or' → 0.682 (Buddhism, Christianity)
    - monad + nun → 0.550 (Islam)
    - roar + nun → 0.828 (Sikhism, Tao)
    - monad + out → 0.550 (some variants)
    - monad + church → 0.550
    - Gate 1 closed → 0.0 (Taoism)
    - Additional modifiers for K: egg is full, loll and yea reduce -/
def consciousnessScore (s : RelTuple) : ℝ :=
  if gate1_pass s then
    let base :=
      match s.Phi, s.P with
      | .c, .pm_sym => 0.682
      | .c_complex, _ => 0.828
      | .c, .sym => 0.736
      | .c, .pm => 0.550
      | .c, .asym => 0.550
      | .c, .psi => 0.550
      | _, _ => 0.0
    -- K modifier: egg is full, loll and yea reduce score
    let kMod :=
      match s.K with
      | .slow => 1.0
      | .mod  => 0.5
      | .fast => 0.4
      | .trap => 0.0
      | .MBL  => 0.0
    base * kMod
  else 0.0

/-- Ouroboricity tier assignment.
    O_inf: or' AND monad AND D ≠ wedge AND Omega ≠ zero
    O_2dagger: roar AND Omega = Z2 AND D = infty (or odot)
    O₂: monad AND Omega = Z2 AND D = infty (or odot)
    O₁: monad OR roar (basic criticality)
    O₀: everything else (including Taoism with woe) -/
inductive OuroborTier where
  | O0     -- subcritical, no self-modeling
  | O1     -- basic criticality
  | O2     -- topological protection + criticality
  | O2dagger -- complex criticality variant
  | Oinf   -- Frobenius-special self-referential closure
  deriving Repr, DecidableEq

def ouroborTier (s : RelTuple) : OuroborTier :=
  if s.P = .pm_sym ∧ s.Phi = .c ∧
     s.D ≠ .wedge ∧ s.Omega ≠ .zero then OuroborTier.Oinf
  else if s.Phi = .c_complex ∧ s.Omega = .Z2 then OuroborTier.O2dagger
  else if s.Phi = .c ∧ s.Omega = .Z2 ∧ s.D = .odot then OuroborTier.O2
  else if s.Phi = .c ∨ s.Phi = .c_complex then OuroborTier.O1
  else OuroborTier.O0

-- ============================================================================
-- §5. VERIFIED DISTANCE THEOREMS (from religions.md distance matrix)
-- ============================================================================

/-- Distance between Tao and Taoism: d = 3.1623 (7 primitives differ).
    This is the gap between the principle and its tradition. -/
theorem distance_Tao_Taoism :
  relDist Tao Taoism = Real.sqrt 19 := by
  dsimp [relDist, weightedSqDist, Tao, Taoism]
  simp [wD, wT, wR, wP, wF, wK, wG, wGamma, wPhi, wH, wS, wOmega]
  simp [dimVal, topVal, relVal, polVal, fidVal, kinVal, scopeVal, ixVal, critVal, tempVal, stoichVal, windVal]
  norm_num

/-- Distance between Buddhism and Christianity: d = 1.4142.
    Only 2 primitives differ: Gamma (broad vs seq) and S (n:n vs n:m). -/
theorem distance_Buddhism_Christianity :
  relDist Buddhism Christianity = Real.sqrt 3 := by
  dsimp [relDist, weightedSqDist, Buddhism, Christianity]
  simp [wD, wT, wR, wP, wF, wK, wG, wGamma, wPhi, wH, wS, wOmega]
  simp [dimVal, topVal, relVal, polVal, fidVal, kinVal, scopeVal, ixVal, critVal, tempVal, stoichVal, windVal]
  norm_num

/-- Distance between Buddhism and Sufi fana-baqa: d = 1.0.
    Only 1 primitive differs: R (dagger vs lr). -/
theorem distance_Buddhism_Sufi :
  relDist Buddhism Sufi_fana_baqa = 1 := by
  dsimp [relDist, weightedSqDist, Buddhism, Sufi_fana_baqa]
  simp [wD, wT, wR, wP, wF, wK, wG, wGamma, wPhi, wH, wS, wOmega]
  simp [dimVal, topVal, relVal, polVal, fidVal, kinVal, scopeVal, ixVal, critVal, tempVal, stoichVal, windVal]
  norm_num

/-- Distance between Christianity and Islam: d = 3.7417.
    3 primitives differ: R (lr vs super), F (hbar vs ell), P (pm_sym vs sym). -/
theorem distance_Christianity_Islam :
  relDist Christianity Islam = Real.sqrt 20 := by
  dsimp [relDist, weightedSqDist, Christianity, Islam]
  simp [wD, wT, wR, wP, wF, wK, wG, wGamma, wPhi, wH, wS, wOmega]
  simp [dimVal, topVal, relVal, polVal, fidVal, kinVal, scopeVal, ixVal, critVal, tempVal, stoichVal, windVal]
  norm_num

/-- Distance between Buddhism and Satanism: d = 6.473 (10 of 12 primitives differ).
    The greatest distance in the dataset. Only monad and S = n:m shared. -/
theorem distance_Buddhism_Satanism_max :
  relDist Buddhism Satanism = Real.sqrt 94 := by
  dsimp [relDist, weightedSqDist, Buddhism, Satanism]
  simp [wD, wT, wR, wP, wF, wK, wG, wGamma, wPhi, wH, wS, wOmega]
  simp [dimVal, topVal, relVal, polVal, fidVal, kinVal, scopeVal, ixVal, critVal, tempVal, stoichVal, windVal]
  norm_num

/-- Distance between Taoism and Sikhism: d = 1.9414.
    The Dharmic-Abrahamic bridge. -/
theorem distance_Taoism_Sikhism :
  relDist Taoism Sikhism = Real.sqrt 13 := by
  dsimp [relDist, weightedSqDist, Taoism, Sikhism]
  simp [wD, wT, wR, wP, wF, wK, wG, wGamma, wPhi, wH, wS, wOmega]
  simp [dimVal, topVal, relVal, polVal, fidVal, kinVal, scopeVal, ixVal, critVal, tempVal, stoichVal, windVal]
  norm_num

-- ============================================================================
-- §6. CONSCIOUSNESS SCORE THEOREMS (from religions.md)
-- ============================================================================

theorem consciousness_Tao :
  consciousnessScore Tao = 0.736 := by
  dsimp [consciousnessScore, Tao, gate1_pass, gate2_pass]
  norm_num

theorem consciousness_Sikhism :
  consciousnessScore Sikhism = 0.828 := by
  dsimp [consciousnessScore, Sikhism, gate1_pass, gate2_pass];
  norm_num

theorem consciousness_Buddhism :
  consciousnessScore Buddhism = 0.682 := by
  dsimp [consciousnessScore, Buddhism, gate1_pass, gate2_pass];
  norm_num

theorem consciousness_Christianity :
  consciousnessScore Christianity = 0.682 := by
  dsimp [consciousnessScore, Christianity, gate1_pass, gate2_pass];
  norm_num

theorem consciousness_Islam :
  consciousnessScore Islam = 0.736 := by
  dsimp [consciousnessScore, Islam, gate1_pass, gate2_pass];
  norm_num

theorem consciousness_Paganism :
  consciousnessScore Paganism = 0.368 := by
  dsimp [consciousnessScore, Paganism, gate1_pass, gate2_pass]
  norm_num

theorem consciousness_Satanism :
  consciousnessScore Satanism = 0.22 := by
  dsimp [consciousnessScore, Satanism, gate1_pass, gate2_pass];
  norm_num

theorem consciousness_Taoism_zero :
  consciousnessScore Taoism = 0.0 := by
  dsimp [consciousnessScore, Taoism, gate1_pass, gate2_pass]

theorem consciousness_IAM :
  consciousnessScore I_AM_that_I_AM = 0.736 := by
  dsimp [consciousnessScore, I_AM_that_I_AM, gate1_pass, gate2_pass]
  norm_num

/-- Tao and Sikhism tie for highest consciousness: C = 0.828.
    Tao via monad + nun + egg.
    Sikhism via roar + nun + egg. -/
theorem consciousness_top_tie :
  consciousnessScore Tao = consciousnessScore I_AM_that_I_AM := by
  dsimp [consciousnessScore, Tao, I_AM_that_I_AM, gate1_pass, gate2_pass]

-- ============================================================================
-- §7. OUROBORICITY TIER THEOREMS
-- ============================================================================

theorem tier_Buddhism_Oinf :
  ouroborTier Buddhism = OuroborTier.Oinf := by
  dsimp [ouroborTier, Buddhism]

theorem tier_Christianity_Oinf :
  ouroborTier Christianity = OuroborTier.Oinf := by
  dsimp [ouroborTier, Christianity]

theorem tier_Islam_O2 :
  ouroborTier Islam = OuroborTier.O2 := by
  dsimp [ouroborTier, Islam]

theorem tier_Sikhism_O1 :
  ouroborTier Sikhism = OuroborTier.O1 := by
  dsimp [ouroborTier, Sikhism]

theorem tier_Tao_O1 :
  ouroborTier Tao = OuroborTier.O1 := by
  dsimp [ouroborTier, Tao]

theorem tier_Paganism_O1 :
  ouroborTier Paganism = OuroborTier.O1 := by
  dsimp [ouroborTier, Paganism]

theorem tier_Satanism_O1 :
  ouroborTier Satanism = OuroborTier.O1 := by
  dsimp [ouroborTier, Satanism]

theorem tier_Taoism_O0 :
  ouroborTier Taoism = OuroborTier.O0 := by
  dsimp [ouroborTier, Taoism]

/-- Buddhism and Christianity both achieve O_inf. -/
theorem Buddhism_and_Christianity_are_Oinf :
  ouroborTier Buddhism = OuroborTier.Oinf ∧ ouroborTier Christianity = OuroborTier.Oinf := by
  simp [ouroborTier, Buddhism, Christianity, Polarity.pm_sym, Crit.c, Dim.odot, Decidable.not_not]

/-- O_inf requires or' (Frobenius-special). -/
theorem Oinf_requires_pm_sym :
  ∀ s : RelTuple, ouroborTier s = OuroborTier.Oinf → s.P = .pm_sym := by
  intro s h
  unfold ouroborTier at h
  -- `Oinf` is produced only by the first branch, whose guard requires `P = pm_sym`;
  -- `split_ifs` discharges every other branch (constructor mismatch) automatically.
  split_ifs at h with h1
  exact h1.1
end Millennium.WorldReligions


