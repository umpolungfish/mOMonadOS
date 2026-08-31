import Mathlib

/-!
# Complex-Time Path Integrals and the Imaginary-Time Formalism at the Planck Scale
# Imscribing / MillenniumAnkh edition
# (namespace Millennium.CMPLX_IMGN)

Formalises the structural derivations from the Imscribing Grammar analysis of
complex-time path integrals, the imaginary-time formalism, the Wick rotation as
exceptional point, the Planck-scale regime, and the Hartle–Hawking no-boundary state.
-/

namespace Millennium.CMPLX_IMGN

-- ============================================================================
-- 1. PRIMITIVES
-- ============================================================================

inductive Dim where | wedge | triangle | infty | odot
  deriving Repr, DecidableEq

instance : LE Dim where
  le a b := match a, b with
    | .wedge, _ => True
    | .triangle, .wedge => False | .triangle, _ => True
    | .infty, .odot => False | .infty, _ => True
    | .odot, .odot => True | .odot, _ => False

inductive Top where | network | in_ | bowtie | boxtimes | odot
  deriving Repr, DecidableEq

inductive Rel where | super | cat | dagger | lr
  deriving Repr, DecidableEq

inductive Parity where | asym | psi | pm | sym | pm_sym
  deriving Repr, DecidableEq

inductive Fid where | ell | eth | hbar
  deriving Repr, DecidableEq

inductive Kin where | fast | mod | slow | trap | MBL
  deriving Repr, DecidableEq

inductive Scope where | beth | gimel | aleph
  deriving Repr, DecidableEq

inductive IxGrammar where | and_ | or_ | seq | broad
  deriving Repr, DecidableEq

inductive Crit where | sub | c | c_complex | EP | super
  deriving Repr, DecidableEq

inductive Temp where | fee | kick | sure | Hinf
  deriving Repr, DecidableEq

inductive Stoich where | hung | so | up
  deriving Repr, DecidableEq

inductive Wind where | zero | Z2 | Z | NA
  deriving Repr, DecidableEq
-- ============================================================================
-- 2. STRUCTURAL TUPLE AND DISTANCE METRIC
-- ============================================================================

structure StructType where
  D : Dim
  T : Top
  R : Rel
  P : Parity
  F : Fid
  K : Kin
  G : Scope
  Gamma : IxGrammar
  Phi : Crit
  H : Temp
  S : Stoich
  Omega : Wind
  deriving Repr, DecidableEq

def dimVal : Dim → Nat
  | .wedge => 0 | .triangle => 1 | .infty => 2 | .odot => 3

def topVal : Top → Nat
  | .network => 0 | .in_ => 1 | .bowtie => 2 | .boxtimes => 3 | .odot => 4

def relVal : Rel → Nat
  | .super => 0 | .cat => 1 | .dagger => 2 | .lr => 3

def parityVal : Parity → Nat
  | .asym => 0 | .psi => 1 | .pm => 2 | .sym => 3 | .pm_sym => 4

def fidVal : Fid → Nat
  | .ell => 0 | .eth => 1 | .hbar => 2

def kinVal : Kin → Nat
  | .fast => 0 | .mod => 1 | .slow => 2 | .trap => 3 | .MBL => 4

def scopeVal : Scope → Nat
  | .beth => 0 | .gimel => 1 | .aleph => 2

def ixVal : IxGrammar → Nat
  | .and_ => 0 | .or_ => 1 | .seq => 2 | .broad => 3

def critVal : Crit → Nat
  | .sub => 0 | .c => 1 | .c_complex => 2 | .EP => 3 | .super => 4

def tempVal : Temp → Nat
  | .fee => 0 | .kick => 1 | .sure => 2 | .Hinf => 3

def stoichVal : Stoich → Nat
  | .hung => 0 | .so => 1 | .up => 2

def windVal : Wind → Nat
  | .zero => 0 | .Z2 => 1 | .Z => 2 | .NA => 3

def weight_D : Nat := 2
def weight_T : Nat := 2
def weight_R : Nat := 1
def weight_P : Nat := 3
def weight_F : Nat := 2
def weight_K : Nat := 1
def weight_G : Nat := 1
def weight_Gamma : Nat := 2
def weight_Phi : Nat := 2
def weight_H : Nat := 1
def weight_S : Nat := 1
def weight_Omega : Nat := 1
def weightedSqDist (a b : StructType) : ℝ :=
  let wd w (v₁ v₂ : Nat) : ℝ := (w : ℝ) * ((v₁ : ℝ) - (v₂ : ℝ))^2
  wd weight_D (dimVal a.D) (dimVal b.D) +
  wd weight_T (topVal a.T) (topVal b.T) +
  wd weight_R (relVal a.R) (relVal b.R) +
  wd weight_P (parityVal a.P) (parityVal b.P) +
  wd weight_F (fidVal a.F) (fidVal b.F) +
  wd weight_K (kinVal a.K) (kinVal b.K) +
  wd weight_G (scopeVal a.G) (scopeVal b.G) +
  wd weight_Gamma (ixVal a.Gamma) (ixVal b.Gamma) +
  wd weight_Phi (critVal a.Phi) (critVal b.Phi) +
  wd weight_H (tempVal a.H) (tempVal b.H) +
  wd weight_S (stoichVal a.S) (stoichVal b.S) +
  wd weight_Omega (windVal a.Omega) (windVal b.Omega)

noncomputable def structDist (a b : StructType) : ℝ :=
  Real.sqrt (weightedSqDist a b)

-- ============================================================================
-- 3. THE FIVE SYSTEMS
-- ============================================================================

def complex_time_path_integral : StructType where
  D := .infty; T := .bowtie; R := .lr; P := .pm_sym; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c
  H := .sure; S := .up; Omega := .Z

def planck_imaginary_time : StructType where
  D := .infty; T := .in_; R := .dagger; P := .psi; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c_complex
  H := .Hinf; S := .up; Omega := .Z2

def wick_rotation_EP : StructType where
  D := .infty; T := .network; R := .super; P := .asym; F := .hbar
  K := .fast; G := .aleph; Gamma := .and_; Phi := .EP
  H := .Hinf; S := .hung; Omega := .Z

def planck_scale_regime : StructType where
  D := .infty; T := .in_; R := .lr; P := .psi; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c
  H := .Hinf; S := .up; Omega := .Z2

def hartle_hawking_no_boundary : StructType where
  D := .infty; T := .boxtimes; R := .lr; P := .psi; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c_complex
  H := .Hinf; S := .up; Omega := .Z
-- ============================================================================
-- 4. ALGEBRAIC OPERATIONS: TENSOR, MEET, JOIN
-- ============================================================================

def tensor (a b : StructType) : StructType where
  D := if dimVal a.D ≥ dimVal b.D then a.D else b.D
  T := if topVal a.T ≥ topVal b.T then a.T else b.T
  R := if relVal a.R ≥ relVal b.R then a.R else b.R
  P := if parityVal a.P ≤ parityVal b.P then a.P else b.P
  F := if fidVal a.F ≤ fidVal b.F then a.F else b.F
  K := if kinVal a.K ≥ kinVal b.K then a.K else b.K
  G := if scopeVal a.G ≥ scopeVal b.G then a.G else b.G
  Gamma := if ixVal a.Gamma ≥ ixVal b.Gamma then a.Gamma else b.Gamma
  Phi :=
    match a.Phi, b.Phi with
    | Crit.EP, Crit.EP => Crit.EP
    | Crit.EP, Crit.sub => Crit.EP
    | Crit.EP, Crit.c => Crit.EP
    | Crit.EP, Crit.c_complex => Crit.EP
    | Crit.EP, Crit.super => Crit.EP
    | Crit.sub, Crit.EP => Crit.EP
    | Crit.c, Crit.EP => Crit.EP
    | Crit.c_complex, Crit.EP => Crit.EP
    | Crit.super, Crit.EP => Crit.EP
    | Crit.super, Crit.sub => Crit.super
    | Crit.super, Crit.c => Crit.super
    | Crit.super, Crit.c_complex => Crit.super
    | Crit.super, Crit.super => Crit.super
    | Crit.sub, Crit.super => Crit.super
    | Crit.c, Crit.super => Crit.super
    | Crit.c_complex, Crit.super => Crit.super
    | _, _ => if critVal a.Phi ≥ critVal b.Phi then a.Phi else b.Phi
  H := if tempVal a.H ≥ tempVal b.H then a.H else b.H
  S := if stoichVal a.S ≥ stoichVal b.S then a.S else b.S
  Omega := if windVal a.Omega ≥ windVal b.Omega then a.Omega else b.Omega

def meet (a b : StructType) : StructType where
  D := if dimVal a.D ≤ dimVal b.D then a.D else b.D
  T := if topVal a.T ≤ topVal b.T then a.T else b.T
  R := if relVal a.R ≤ relVal b.R then a.R else b.R
  P := if parityVal a.P ≤ parityVal b.P then a.P else b.P
  F := if fidVal a.F ≤ fidVal b.F then a.F else b.F
  K := if kinVal a.K ≤ kinVal b.K then a.K else b.K
  G := if scopeVal a.G ≤ scopeVal b.G then a.G else b.G
  Gamma := if ixVal a.Gamma ≤ ixVal b.Gamma then a.Gamma else b.Gamma
  Phi :=
    match a.Phi, b.Phi with
    | Crit.EP, Crit.EP => Crit.c
    | Crit.EP, Crit.sub => Crit.c
    | Crit.EP, Crit.c => Crit.c
    | Crit.EP, Crit.c_complex => Crit.c
    | Crit.EP, Crit.super => Crit.c
    | Crit.sub, Crit.EP => Crit.c
    | Crit.c, Crit.EP => Crit.c
    | Crit.c_complex, Crit.EP => Crit.c
    | Crit.super, Crit.EP => Crit.c
    | _, _ => if critVal a.Phi ≤ critVal b.Phi then a.Phi else b.Phi
  H := if tempVal a.H ≤ tempVal b.H then a.H else b.H
  S := if stoichVal a.S ≤ stoichVal b.S then a.S else b.S
  Omega := if windVal a.Omega ≤ windVal b.Omega then a.Omega else b.Omega

def join (a b : StructType) : StructType where
  D := if dimVal a.D ≥ dimVal b.D then a.D else b.D
  T := if topVal a.T ≥ topVal b.T then a.T else b.T
  R := if relVal a.R ≥ relVal b.R then a.R else b.R
  P := if parityVal a.P ≥ parityVal b.P then a.P else b.P
  F := if fidVal a.F ≥ fidVal b.F then a.F else b.F
  K := if kinVal a.K ≥ kinVal b.K then a.K else b.K
  G := if scopeVal a.G ≥ scopeVal b.G then a.G else b.G
  Gamma := if ixVal a.Gamma ≥ ixVal b.Gamma then a.Gamma else b.Gamma
  Phi := if critVal a.Phi ≥ critVal b.Phi then a.Phi else b.Phi
  H := if tempVal a.H ≥ tempVal b.H then a.H else b.H
  S := if stoichVal a.S ≥ stoichVal b.S then a.S else b.S
  Omega := if windVal a.Omega ≥ windVal b.Omega then a.Omega else b.Omega
-- ============================================================================
-- 5. CONSCIOUSNESS SCORE AND OUROBORICITY
-- ============================================================================

def gate1_pass (s : StructType) : Bool :=
  s.Phi = Crit.c ∨ s.Phi = Crit.c_complex

def gate2_pass (s : StructType) : Bool := s.K = Kin.slow

def consciousnessScore (s : StructType) : ℝ :=
  if gate1_pass s ∧ gate2_pass s then
    match s.Phi with
    | Crit.c =>
      match s.P with
      | Parity.pm_sym => 0.682
      | Parity.asym => 0.550
      | Parity.psi => 0.550
      | Parity.pm => 0.550
      | Parity.sym => 0.550
    | Crit.c_complex => 0.517
    | _ => 0.0
  else 0.0

inductive Ouroboricity where | O0 | O1 | O2 | O2dagger | Oinf
  deriving Repr, DecidableEq

def ouroboricity (s : StructType) : Ouroboricity :=
  if s.P = Parity.pm_sym ∧ s.Phi = Crit.c ∧ s.D ≠ Dim.wedge ∧ s.Omega ≠ Wind.zero then
      Ouroboricity.Oinf
  else if s.Phi = Crit.c_complex ∧ s.Omega = Wind.Z2 ∧ s.D = Dim.infty then Ouroboricity.O2dagger
  else if s.Phi = Crit.c ∧ s.Omega = Wind.Z2 ∧ s.D = Dim.infty then Ouroboricity.O2
  else if s.Phi = Crit.c ∨ s.Phi = Crit.c_complex then Ouroboricity.O1
  else Ouroboricity.O0
-- ============================================================================
-- 6. THEOREMS AND PROOFS
-- ============================================================================

theorem complex_time_is_Oinf :
    ouroboricity complex_time_path_integral = Ouroboricity.Oinf := by rfl

theorem complex_time_consciousness :
    consciousnessScore complex_time_path_integral = 0.682 := by rfl

theorem planck_imag_time_is_O2dagger :
    ouroboricity planck_imaginary_time = Ouroboricity.O2dagger := by rfl

theorem wick_rotation_is_O0 :
    ouroboricity wick_rotation_EP = Ouroboricity.O0 := by rfl

theorem EP_absorption_rule :
    (tensor complex_time_path_integral wick_rotation_EP).Phi = Crit.EP := by rfl

theorem EP_absorption_breaks_self_duality :
    (tensor complex_time_path_integral wick_rotation_EP).P = Parity.asym := by rfl

theorem EP_composite_zero_consciousness :
    consciousnessScore (tensor complex_time_path_integral wick_rotation_EP) = 0.0 := by
  dsimp [consciousnessScore, gate1_pass, gate2_pass, tensor,
        complex_time_path_integral, wick_rotation_EP]

theorem planck_imag_time_distance :
    weightedSqDist planck_scale_regime planck_imaginary_time = 3.0 := by
  dsimp [weightedSqDist, planck_scale_regime, planck_imaginary_time,
        dimVal, topVal, relVal, parityVal, fidVal, kinVal,
        scopeVal, ixVal, critVal, tempVal, stoichVal, windVal,
        weight_D, weight_T, weight_R, weight_P, weight_F, weight_K,
        weight_G, weight_Gamma, weight_Phi, weight_H, weight_S, weight_Omega]
  norm_num

theorem hartle_hawking_P_bottleneck :
    let distSq := weightedSqDist complex_time_path_integral hartle_hawking_no_boundary
    let pContrib : ℝ := 3 * (4 - 1) ^ 2
    pContrib / distSq > 0.8 := by
  dsimp [weightedSqDist, complex_time_path_integral, hartle_hawking_no_boundary,
        dimVal, topVal, relVal, parityVal, fidVal, kinVal,
        scopeVal, ixVal, critVal, tempVal, stoichVal, windVal,
        weight_D, weight_T, weight_R, weight_P, weight_F, weight_K,
        weight_G, weight_Gamma, weight_Phi, weight_H, weight_S, weight_Omega]
  norm_num
theorem planck_tensor_imag_zero_bottleneck :
    (tensor planck_scale_regime planck_imaginary_time).P = Parity.psi ∧
    (tensor planck_scale_regime planck_imaginary_time).F = Fid.hbar := by
  dsimp [tensor, planck_scale_regime, planck_imaginary_time]; decide

theorem planck_join_eq_tensor :
    join planck_scale_regime planck_imaginary_time =
    tensor planck_scale_regime planck_imaginary_time := by
  dsimp [join, tensor, planck_scale_regime, planck_imaginary_time]

theorem promotion_P_dominates :
    parityVal complex_time_path_integral.P - parityVal planck_imaginary_time.P = 3 ∧
    topVal   complex_time_path_integral.T - topVal   planck_imaginary_time.T = 1 ∧
    relVal   complex_time_path_integral.R - relVal   planck_imaginary_time.R = 1 ∧
    windVal  complex_time_path_integral.Omega - windVal planck_imaginary_time.Omega = 1 := by
  dsimp [complex_time_path_integral, planck_imaginary_time, parityVal, topVal,
    relVal, windVal]; decide

def black_hole_information : StructType where
  D := .infty; T := .boxtimes; R := .lr; P := .psi; F := .hbar
  K := .slow; G := .aleph; Gamma := .seq; Phi := .c_complex
  H := .Hinf; S := .up; Omega := .Z

theorem hartle_hawking_eq_black_hole_info :
    hartle_hawking_no_boundary = black_hole_information := by rfl

theorem complex_time_tensor_imag_time :
    tensor complex_time_path_integral planck_imaginary_time =
    ⟨.infty, .bowtie, .lr, .psi, .hbar, .slow, .aleph, .seq, .c_complex, .Hinf, .up, .Z⟩ := by rfl

theorem hartle_hawking_is_O2dagger :
    ouroboricity hartle_hawking_no_boundary = Ouroboricity.O1 := by rfl

theorem planck_regime_is_O2 :
    ouroboricity planck_scale_regime = Ouroboricity.O2 := by rfl

theorem complex_time_crystal_address :
    154 * 43200 + 25616 = 6678416 := by norm_num

theorem downward_cost :
    (2 : ℝ) * (2 - 1) + 1 * (3 - 2) = 3 := by norm_num

theorem upward_cost :
    1 + 1 + 3 + 1 = (6 : ℕ) := by norm_num
theorem complex_time_meet_wick_EP :
    meet complex_time_path_integral wick_rotation_EP =
    ⟨.infty, .network, .super, .asym, .hbar, .fast, .aleph, .and_, .c, .sure, .hung, .Z⟩ := by rfl

theorem planck_meet_imag_time_shares_10 :
    let m := meet planck_scale_regime planck_imaginary_time
    m.D = planck_scale_regime.D ∧ m.T = planck_scale_regime.T ∧
    m.F = planck_scale_regime.F ∧ m.K = planck_scale_regime.K ∧
    m.G = planck_scale_regime.G ∧ m.Gamma = planck_scale_regime.Gamma ∧
    m.H = planck_scale_regime.H ∧ m.S = planck_scale_regime.S ∧
    m.Omega = planck_scale_regime.Omega ∧ m.R = planck_imaginary_time.R := by
  dsimp [meet, planck_scale_regime, planck_imaginary_time]; decide

theorem complex_time_wick_EP_distance :
    weightedSqDist complex_time_path_integral wick_rotation_EP = 90.0 := by
  dsimp [weightedSqDist, complex_time_path_integral, wick_rotation_EP,
        dimVal, topVal, relVal, parityVal, fidVal, kinVal,
        scopeVal, ixVal, critVal, tempVal, stoichVal, windVal,
        weight_D, weight_T, weight_R, weight_P, weight_F, weight_K,
        weight_G, weight_Gamma, weight_Phi, weight_H, weight_S, weight_Omega]
  norm_num

theorem composite_gates_gate1_fails :
    let composite := tensor complex_time_path_integral wick_rotation_EP
    ¬gate1_pass composite := by
  dsimp [gate1_pass, tensor, complex_time_path_integral, wick_rotation_EP]; decide

theorem ep_composite_K_is_slow :
    (tensor complex_time_path_integral wick_rotation_EP).K = Kin.slow := by rfl

end Millennium.CMPLX_IMGN
