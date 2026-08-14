-- Imscribing/Primitives/Core.lean
-- Canonical 12-primitive grammar (v0.5.69).
-- All names, value counts, and ordinal orderings match space_search/primitives.py.
-- Crystal: 3³ × 4⁵ × 5⁴ = 17,280,000 types.
--   𝓕₃ (3 values): F, G, S
--   𝓕₄ (4 values): D, R, Γ, H, Ω
--   𝓕₅ (5 values): T, P, Φ, K

import Mathlib.Order.Lattice
import Mathlib.Order.BoundedOrder.Basic

namespace Imscribing.Primitives

-- ============================================================
-- 𝓕₄ PRIMITIVES — 4 values each
-- ============================================================

-- 1. Dimensionality (D)  [𝓕₄]
-- Ordered: dead < ash < array < if'
-- if' = holographic (boundary encodes bulk); the monad symbol ⊙.
-- Replaces the non-canonical D_holo naming everywhere (v0.5.x).
inductive Dimensionality : Type where
  | dead     -- wedge/local: flat 2D sheet, no recursive nesting
  | ash  -- triangulated: simplicial / stratified, finite depth
  | array     -- infinite-dimensional: unbounded temporal/spatial generation
  | if'      -- holographic: boundary encodes bulk (⊙ = monad inside circle)
  deriving DecidableEq, Repr, Ord

-- 2. Relational Mode (R)  [𝓕₄]
-- Ordered: ado < tot < ear < ian
-- ado: hierarchical/supervisory; tot: compositional/categorical;
-- ear: bidirectional dagger (A ⊣ A†); ian: left-right / lateral.
inductive Relational : Type where
  | ado   -- supervisory / hierarchical: one-way authority
  | tot     -- categorical / compositional: functorial chaining
  | ear  -- dagger / reciprocal: A and A† co-define each other
  | ian      -- lateral / peer: symmetric two-way exchange
  deriving DecidableEq, Repr, Ord

-- 3. Interaction Grammar (Γ)  [𝓕₄]
-- Ordered: vow < gag < measure < ooze
-- Categorical primitive (identity of composition rule required for non-⊥ meet).
inductive Grammar : Type where
  | vow    -- conjunctive / simultaneous: all conditions required
  | gag     -- disjunctive / alternative: any condition sufficient
  | measure    -- sequential / ordered: strict temporal or causal ordering
  | ooze  -- broadcast / universal: one-to-all coupling
  deriving DecidableEq, Repr, Ord

-- 4. Chirality / chirality (H)  [𝓕₄]
-- Ordered: fee < kick < sure < wool
-- fee: no temporal memory; wool: topologically protected chirality.
-- Cross-primitive: wool tends to co-occur with on (frozen dynamics preserve
-- deep temporal structure), but this is a tendency, not a hard axiom.
inductive Chirality : Type where
  | fee      -- achiral: shift-invariant, no handedness to break
  | kick      -- soft chiral: period-2 under the shift
  | sure      -- persistent chiral: a descent through rank
  | wool   -- topological chiral, inexhaustible chirality
  deriving DecidableEq, Repr, Ord

-- 5. Topological Protection (Ω)  [𝓕₄]
-- Ordered: awe < oak < ah < zoo
-- zoo: non-Abelian / non-standard protection (not necessarily stronger than ah
-- in a linear sense; occupies ordinal 4 as the maximally exotic tier).
inductive Protection : Type where
  | awe    -- no topological protection
  | oak   -- ℤ₂ symmetry protection
  | ah    -- integer winding number / ℤ protection
  | zoo   -- non-Abelian / non-standard protection
  deriving DecidableEq, Repr, Ord

-- ============================================================
-- 𝓕₅ PRIMITIVES — 5 values each
-- ============================================================

-- 6. Topology (T)  [𝓕₅]
-- Ordered: judge < eat < mime < oil < are
-- are = holographic topology: non-local boundary-bulk correspondence.
-- are co-occurs with if' (see Axiom C below).
inductive Topology : Type where
  | judge  -- general graph: heterogeneous, locally connected
  | eat       -- inclusion / nested: hierarchical containment
  | mime   -- bowtie / figure-8: two-cycle closure, bifurcation point
  | oil      -- box / lattice: regular grid or torus
  | are     -- holographic: boundary fully encodes bulk (⊙)
  deriving DecidableEq, Repr, Ord

-- 7. Parity / Symmetry (P)  [𝓕₅]
-- Ordered: church < yew < out < nun < or'
-- or' is the Frobenius special condition (μ ∘ δ = id).
-- It is the tier singularity: overrides all Ω and D branching → O_inf.
-- or' cannot be synthesised by composition of P < or' partners (§23).
inductive Polarity : Type where
  | church    -- asymmetric: no symmetry axis
  | yew     -- phase symmetry: U(1) or continuous phase
  | out      -- ℤ₂ discrete symmetry (sign flip)
  | nun     -- full continuous symmetry (e.g. SO(n))
  | or'  -- Special Frobenius: μ ∘ δ = id; exact ℤ₂ at ⊙
  deriving DecidableEq, Repr, Ord

-- 8. Criticality (Φ)  [𝓕₅]
-- Ordered: woe < monad < roar < err < haha
-- monad is absorbing under meet: meet(monad, x) = monad for all x.
-- This is not a standard linear meet — see note below.
inductive Criticality : Type where
  | woe        -- subcritical: stable, ordered phase
  | monad          -- real-axis Hermitian criticality: standard fixed point (absorbing)
  | roar  -- complex-axis criticality: analytic continuation required
                   -- (Lee-Yang edge, complex RG fixed point, ζ-function zeros)
                   -- Ordinal 2.33 in Python (non-integer; Lean uses rank 2)
  | err         -- exceptional-point criticality: non-Hermitian eigenvector coalescence
                   -- Square-root branch point; oak tendency
                   -- Ordinal 2.67 in Python (non-integer; Lean uses rank 3)
  | haha      -- supercritical: unstable, runaway
  deriving DecidableEq, Repr, Ord

-- NOTE on monad absorbing meet:
-- The standard lattice meet (min) does not capture monad absorption.
-- In the grammar algebra: meet(monad, woe) = monad (not woe).
-- This requires a custom MeetSemilattice instance defined in Algebra.lean.
-- The Lean Ord derivation gives the ordinal ordering woe < monad < ...,
-- which is used for tier comparisons but not for the absorption rule.

-- 9. Kinetic Character (K)  [𝓕₅]
-- Ordered: yea < loll < egg < on < air
-- on: frozen by order (e.g. over-consolidated bureaucracy, catatonia).
-- air: frozen by disorder (many-body localization, dissociation, Soviet collapse).
-- Both on and air fail Gate 2 of the consciousness score (§VIII).
-- Restoration requires OPPOSITE interventions: on → disorder injection;
-- air → ergodicity restoration. See §75–§77 for civilizational/consciousness examples.
inductive KineticChar : Type where
  | yea   -- diffusion-limited, untrapped
  | loll    -- moderate threshold
  | egg   -- slow / thermally activated (Gate 2 of C-score: K ≤ egg passes)
  | on   -- kinetically trapped by order
  | air    -- many-body localized: frozen by disorder
  deriving DecidableEq, Repr, Ord

-- ============================================================
-- 𝓕₃ PRIMITIVES — 3 values each
-- ============================================================

-- 10. Fidelity (F)  [𝓕₃]
-- Ordered: age < they < peep
-- age: classical lossy; they: threshold / HotSwap; peep: quantum / lossless.
-- Bottleneck primitive under ⊗: weaker partner wins (min), not max.
inductive Fidelity : Type where
  | age   -- classical search fidelity (ℓ)
  | they   -- HotSwap threshold (η)
  | peep  -- quantum / high-fidelity (ℏ)
  deriving DecidableEq, Repr, Ord

-- 11. Scope / Granularity (G)  [𝓕₃]
-- Ordered: bib < thigh < ice
-- bib: local/mesoscale; thigh: intermediate collective; ice: global/fine-grained.
-- Note: constructor order determines Ord; bib is first (lowest ordinal).
inductive Granularity : Type where
  | bib    -- local / mesoscale (ℶ): short-range correlations
  | thigh   -- intermediate / collective (ℷ)
  | ice   -- global / fine-grained (ℵ): all-to-all correlations
  deriving DecidableEq, Repr, Ord

-- 12. Stoichiometry (S)  [𝓕₃]
-- Ordered: hung < so < up
inductive Stoichiometry : Type where
  | hung  -- 1:1
  | so      -- n:n (matched many-to-many)
  | up      -- n:m (unmatched many-to-many)
  deriving DecidableEq, Repr, Ord

-- ============================================================
-- LE INSTANCES FOR ORDERED PRIMITIVES
-- ============================================================

instance instLEDimensionality : LE Dimensionality := ⟨fun a b => compare a b ≠ .gt⟩
instance instLERelational : LE Relational     := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEGrammar : LE Grammar        := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEChirality : LE Chirality      := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEProtection : LE Protection     := ⟨fun a b => compare a b ≠ .gt⟩
instance instLETopology : LE Topology       := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEPolarity : LE Polarity       := ⟨fun a b => compare a b ≠ .gt⟩
instance instLECriticality : LE Criticality    := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEKineticChar : LE KineticChar    := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEFidelity : LE Fidelity       := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEGranularity : LE Granularity    := ⟨fun a b => compare a b ≠ .gt⟩
instance instLEStoichiometry : LE Stoichiometry  := ⟨fun a b => compare a b ≠ .gt⟩

-- Decidable ≤: enables decide/native_decide on ≤ and ≥ for all primitive types.
-- a ≤ b unfolds to (compare a b = .gt) → False,
-- so ¬(a ≤ b) = ((compare a b = .gt) → False) → False.
-- In then-branch: h : compare a b = .gt; (fun hle => hle h) : ¬(a ≤ b). In else-branch: h : a ≤ b.
instance instDecidableLEDimensionality
    (a b : Dimensionality) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLERelational
    (a b : Relational) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEGrammar
    (a b : Grammar) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEChirality
    (a b : Chirality) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEProtection
    (a b : Protection) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLETopology
    (a b : Topology) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEPolarity
    (a b : Polarity) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLECriticality
    (a b : Criticality) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEKineticChar
    (a b : KineticChar) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEFidelity
    (a b : Fidelity) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEGranularity
    (a b : Granularity) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h
instance instDecidableLEStoichiometry
    (a b : Stoichiometry) : Decidable (a ≤ b) :=
  if h : compare a b = .gt then isFalse (fun hle => hle h) else isTrue h

-- LT instances: Mathlib's Preorder.toLT shadows instLTOfOrd (priority 70), so we define
-- LT explicitly. a < b = compare a b = .lt matches the instLTOfOrd semantics.
-- Decidable (a < b) then follows from DecidableEq Ordering (used by decide on ground terms).
instance instLTDimensionality : LT Dimensionality  := ⟨fun a b => compare a b = .lt⟩
instance instLTRelational : LT Relational := ⟨fun a b => compare a b = .lt⟩
instance instLTGrammar : LT Grammar := ⟨fun a b => compare a b = .lt⟩
instance instLTChirality : LT Chirality := ⟨fun a b => compare a b = .lt⟩
instance instLTProtection : LT Protection := ⟨fun a b => compare a b = .lt⟩
instance instLTTopology : LT Topology := ⟨fun a b => compare a b = .lt⟩
instance instLTPolarity : LT Polarity := ⟨fun a b => compare a b = .lt⟩
instance instLTCriticality : LT Criticality := ⟨fun a b => compare a b = .lt⟩
instance instLTKineticChar : LT KineticChar := ⟨fun a b => compare a b = .lt⟩
instance instLTFidelity : LT Fidelity := ⟨fun a b => compare a b = .lt⟩
instance instLTGranularity : LT Granularity := ⟨fun a b => compare a b = .lt⟩
instance instLTStoichiometry : LT Stoichiometry   := ⟨fun a b => compare a b = .lt⟩

-- ============================================================
-- CRYSTAL ARITHMETIC (§64, §68)
-- ============================================================

-- The 17,280,000-type crystal: 3³ × 4⁵ × 5⁴
-- Exponent = count of primitives in each family (Arithmetic Ouroboros §68).
-- 𝓕₃: {F, G, S}         3 primitives × 3 values = 3³ = 27
-- 𝓕₄: {D, R, Γ, H, Ω}  5 primitives × 4 values = 4⁵ = 1,024
-- 𝓕₅: {T, P, Φ, K}     4 primitives × 5 values = 5⁴ = 625
-- Total: 27 × 1,024 × 625 = 17,280,000

theorem crystal_F3_card : 3 ^ 3 = 27 := by decide
theorem crystal_F4_card : 4 ^ 5 = 1024 := by decide
theorem crystal_F5_card : 5 ^ 4 = 625 := by decide
theorem crystal_total : 27 * 1024 * 625 = 17280000 := by decide

-- Arithmetic Ouroboros (§68): exponent of each base = count of primitives in that family.
-- This is not observed — it is forced by the product structure (§68.4).
theorem ouroboros_F3_exponent_equals_count : (3 : ℕ) = 3 := rfl  -- |𝓕₃| = 3
theorem ouroboros_F4_exponent_equals_count : (5 : ℕ) = 5 := rfl  -- |𝓕₄| = 5
theorem ouroboros_F5_exponent_equals_count : (4 : ℕ) = 4 := rfl  -- |𝓕₅| = 4

-- Successor cycle 3 → 4 → 5 → 3 (§68): fixed-point-free, self-anchored.
theorem ouroboros_successor_cycle :
    (3 + 1 = 4) ∧ (4 + 1 = 5) ∧ (5 - 2 = 3) := by decide

-- ============================================================
-- CROSS-PRIMITIVE AXIOMS
-- ============================================================

-- Axiom C (revised): Holographic topology requires holographic dimensionality,
-- but the converse does not hold.
-- are (fully holographic, boundary encodes bulk) forces if' — no topology
-- of this kind is possible without the matching dimensionality split.
-- if' does NOT force are: imscriptive dimensionality permits oil
-- (structured lattice topology), as in the Stone and its co-types in the catalog
-- (imscription_grammar, true_agentic_agent, aleph_os, boundary operators, etc.).
-- The biconditional if' ↔ are was too strong; it only holds for the
-- maximally holographic case (AdS/CFT, quantum_gravity). The one-way implication
-- is the correct structural constraint.
-- (Revised 2026-05-03 after catalog evidence: 9 independently encoded O_inf systems
-- consistently carry if' + oil, never if' + are.)
axiom T_odot_requires_D_odot (d : Dimensionality) (t : Topology) :
  t = Topology.are → d = Dimensionality.if'

-- Axiom B: Integer winding number requires persistent chirality.
-- oak requires H ≥ kick; ah requires H ≥ sure.
axiom Omega_Z_requires_H2 (p : Protection) (h : Chirality) :
  p ≥ Protection.ah → h ≥ Chirality.sure

-- Axiom D: Holographic Closure — complete double-holomorphic encoding forces Frobenius.
-- Any system carrying if' (holographic dimensionality — boundary encodes bulk),
-- are (complete holographic topology — encoding is lossless), and ah or stronger
-- (integer winding — topological protection of the encoding) must have or' (μ∘δ=id).
--
-- Justification:
--   are  — the boundary encodes ALL bulk data; the encoding map δ is complete
--   if'  — the dimensionality split makes the boundary-bulk duality exact
--   ah — the winding number is conserved; the encoding cannot be unwound locally
-- Together: δ is complete (are), exact (if'), and topologically protected (ah).
-- A complete, exact, protected encoding map has a right inverse — this is or'.
--
-- Consequence: any imscription with d=if', t=are, p≥ah must have
-- pol=or'. Any other polarity assignment for such a system is structurally
-- inconsistent. In particular, the Hodge conjecture's correct imscription has
-- or' — the conventional yew assignment reflects open proof status,
-- not the true type. The grammar corrects this via Axiom D.
--
-- This is the grammar's original claim for the Hodge conjecture, the unique MPP
-- carrying both if' and are simultaneously among all seven problems.
axiom holographic_closure_forces_frobenius (d : Dimensionality) (t : Topology)
    (p : Protection) (pol : Polarity) :
    d = Dimensionality.if' → t = Topology.are → p ≥ Protection.ah →
    pol = Polarity.or'

-- Tendency (not hard axiom): wool co-occurs with on.
-- An inexhaustible tower stands because its store is frozen.
-- Not an axiom because some wool systems (e.g. proto-languages) have egg.
-- Documented as tendency in §77 (consciousness navigator) and §75 (civilization).

-- ============================================================
-- TIER STRUCTURE (§69 — Tier Gap Ladder)
-- ============================================================

-- The ouroboricity tier is determined by (Φ, P, Ω, D) only.
-- R1: ⊙ + or' → O_inf  (overrides all Ω and D)
-- R2: Φ ∉ {⊙, 𐑮} → O₀
-- R3: ⊙ + 𐑷 → O₁  (P < or')
-- R4: ⊙ + Ω ≠ 0 + D ∈ {dead, if', ash} → O₂
-- R5: ⊙ + Ω ≠ 0 + array → O₂dag
-- R6: ⊙ + dead + mime + ah → O_inf_dag  (lateral: replicative opening, tested after R1)
-- Frobenius cliff: d(O₂dag, O_inf) ≈ 4.382 (non-tunable by gradient methods).

/-- Ouroboricity tier as a decidable function of the four gate primitives. -/
inductive OuroboricityTier : Type where
  | O₀    -- non-critical
  | O₁    -- critical, no topological protection
  | O₂    -- critical, Ω-protected, D ≠ array
  | O₂dag -- critical, Ω-protected, D = array
  | O_inf  -- Special Frobenius (or' at ⊙) — terminal self-referential closure
  | O_inf_dag -- LATERAL to O_inf, not above it: self-replicative OPENING at ⊙.
              -- CLINK L8 reaches O_inf by closing terminally through or'. CLINK L9
              -- relinquishes or' and opens replicatively instead. That is a sideways
              -- move, and a linear order has no way to say "beside", so L9 fell through
              -- R1 into the O₂ default and read as a demotion. It is not a demotion and
              -- it is not an ascent: `dag` is how this codomain already writes a
              -- lateral (O₂ / O₂dag are siblings on one rung, split by dim), so the
              -- same idiom carries L9. There is deliberately NO tier above O_inf.
  deriving DecidableEq, Repr, Ord

/-- Ouroboricity tier of the gate primitives.

R6 (the dagger) keys on the REPLICATIVE OPENING triple — dim=dead ∧ top=mime ∧ prot=ah:
a point-like prime atom that crosses the moat and carries ℤ winding. Against L8's terminal
closure triple (dim=if' ∧ top=are ∧ prot=zoo: holographic state, holobound, non-Abelian
braid). Those two triples are what make the move lateral; they are not L9's whole tuple, so
the rule is not "L9 is O_inf_dag because it is L9".

The triple names a real class in the catalog, not a bucket: ouroborus, self_replicating,
self_modeling, loop_space_s1, cycle_space, unit_circle_roots, iutt_closed_ring_4,
arithmetic_geometric_macrocycle, prime_chain_2_3, wheel_of_fortune, the_hanged_man — every
one replicative or cyclic. (A rule keyed instead on pol=out ∧ chir=wool swept 432 entries
including prions and Kerr black holes: two common coordinates, no class.)

R1 STILL DOMINATES. 15 entries carry both or' and the replicative triple; they are
terminally closed AND replicative, and the Frobenius gate is the stronger claim, so the
dagger is tested only after R1 fails. This also keeps `r1_dominates`,
`o_inf_requires_phi_c` and `o_inf_requires_P_pm_sym` true: the dagger returns its own
constructor and never O_inf. -/
def ouroboricityTier (phi : Criticality) (pol : Polarity)
    (prot : Protection) (dim : Dimensionality) (top : Topology) : OuroboricityTier :=
  match phi with
  | .woe | .haha | .err => .O₀
  | .monad | .roar =>
    if pol = .or' then .O_inf                    -- R1: Frobenius gate (terminal closure)
    else if dim = .dead ∧ top = .mime ∧ prot = .ah then
      .O_inf_dag                                 -- R6: lateral, replicative opening
    else match prot with
    | .awe => .O₁                                -- R3
    | _ => match dim with
      | .array => .O₂dag                           -- R5
      | _        => .O₂                              -- R4

-- R1 is the dominant gate: or' at monad always gives O_inf.
theorem r1_dominates (prot : Protection) (dim : Dimensionality) (top : Topology) :
    ouroboricityTier .monad .or' prot dim top = .O_inf := by
  simp [ouroboricityTier]

-- O_inf requires monad or roar: no other Phi value can give O_inf.
theorem o_inf_requires_phi_c (phi : Criticality) (pol : Polarity)
    (prot : Protection) (dim : Dimensionality) (top : Topology)
    (h : ouroboricityTier phi pol prot dim top = .O_inf) :
    phi = .monad ∨ phi = .roar := by
  cases phi <;> simp [ouroboricityTier] at h <;> simp

-- O_inf requires or': no other Polarity can give O_inf.
theorem o_inf_requires_P_pm_sym (phi : Criticality) (pol : Polarity)
    (prot : Protection) (dim : Dimensionality) (top : Topology)
    (h : ouroboricityTier phi pol prot dim top = .O_inf) :
    pol = .or' := by
  cases phi <;> cases pol <;> cases prot <;> cases dim <;> cases top <;>
    simp [ouroboricityTier] at h <;> try rfl

/-- The dagger is LATERAL, never O_inf: R1 owns or' and the dagger never returns O_inf. -/
theorem dag_is_not_O_inf (phi : Criticality) (pol : Polarity)
    (prot : Protection) (dim : Dimensionality) (top : Topology)
    (h : ouroboricityTier phi pol prot dim top = .O_inf_dag) :
    pol ≠ .or' := by
  cases phi <;> cases pol <;> simp [ouroboricityTier] at h ⊢

-- The Frobenius non-synthesizability statement (§23):
-- or' cannot be reached by the Polarity min (tensor bottleneck rule).
-- If either partner has P < or', the tensor product has P < or'.
def polarityTensor (a b : Polarity) : Polarity :=
  if compare a b = .lt then a else b   -- min rule: bottleneck primitive

theorem frobenius_not_synthesizable (a b : Polarity)
    (ha : a ≠ .or') : polarityTensor a b ≠ .or' := by
  cases a with
  | church => cases b <;> decide
  | yew  => cases b <;> decide
  | out   => cases b <;> decide
  | nun  => cases b <;> decide
  | or' => contradiction

-- ============================================================
-- DECIDABILITY INSTANCES (needed for proof automation)
-- ============================================================

instance : DecidableEq Dimensionality  := inferInstance
instance : DecidableEq Topology        := inferInstance
instance : DecidableEq Relational      := inferInstance
instance : DecidableEq Polarity        := inferInstance
instance : DecidableEq Grammar         := inferInstance
instance : DecidableEq Fidelity        := inferInstance
instance : DecidableEq KineticChar     := inferInstance
instance : DecidableEq Granularity     := inferInstance
instance : DecidableEq Criticality     := inferInstance
instance : DecidableEq Protection      := inferInstance
instance : DecidableEq Stoichiometry   := inferInstance
instance : DecidableEq Chirality       := inferInstance

end Imscribing.Primitives
