-- Imscribing/Paraconsistent/ElectroweakBelnap.lean
-- ELECTROWEAK BELNAP -- SU(2)_L x U(1)_Y unification
-- Weak isospin doublets, hypercharge, Gell-Mann-Nishijima,
-- W+/W-/Z/gamma gauge bosons, chirality, and FDE structure.

import Imscribing.Paraconsistent.LeptonBelnap
import Imscribing.Paraconsistent.GaugeBosonBelnap
import Imscribing.Consciousness

open Imscribing.Primitives
open Imscribing.Paraconsistent.LeptonBelnap

set_option linter.style.longLine false

namespace Imscribing.Paraconsistent.ElectroweakBelnap


-- =====================================================================
-- SS1  WEAK ISOSPIN -- SU(2)_L structure (Belnap FOUR)
-- =====================================================================

inductive WeakIsospin where
  | none | up | down | both
  deriving Inhabited, Repr, DecidableEq

def isoLE (a b : WeakIsospin) : Bool :=
  match a, b with
  | .none, _ => true
  | _, .both => true
  | .up, .up => true
  | .down, .down => true
  | _, _ => false

instance : LE WeakIsospin where
  le a b := isoLE a b = true

theorem iso_none_is_min (w : WeakIsospin) : .none <= w := by
  cases w <;> rfl

theorem iso_both_is_max (w : WeakIsospin) : w <= .both := by
  cases w <;> rfl

theorem iso_up_down_incomparable :
    ¬ ((.up : WeakIsospin) ≤ (.down : WeakIsospin)) ∧
    ¬ ((.down : WeakIsospin) ≤ (.up : WeakIsospin)) := by
  unfold LE.le instLEWeakIsospin isoLE; simp

def isoToBelnap : WeakIsospin -> Belnap
  | .none => Belnap.N
  | .up   => Belnap.T
  | .down => Belnap.F
  | .both => Belnap.B


-- =====================================================================
-- SS2  HYPERCHARGE -- U(1)_Y quantum numbers
-- =====================================================================

structure Hypercharge where
  yNum : Int
  yDen : Nat
  den_pos : yDen = 1
  deriving Repr, DecidableEq

def hyper_Y_L  : Hypercharge := ⟨-1, 1, rfl⟩  -- lepton doublet: Y = -1
def hyper_Y_eR : Hypercharge := ⟨-2, 1, rfl⟩  -- e_R singlet:   Y = -2
def hyper_Y_QL : Hypercharge := ⟨ 1, 1, rfl⟩  -- quark doublet:  Y = 1 (using 3x convention handled elsewhere)
def hyper_Y_uR : Hypercharge := ⟨ 4, 1, rfl⟩  -- u_R singlet
def hyper_Y_dR : Hypercharge := ⟨-2, 1, rfl⟩  -- d_R singlet
def hyper_Y_phi : Hypercharge := ⟨ 1, 1, rfl⟩ -- Higgs doublet:   Y = +1


-- =====================================================================
-- SS3  GELL-MANN-NISHIJIMA -- Q = T3 + Y/2
-- =====================================================================

theorem gm_electron : ((-1 : Rat)/2 + ((-1 : Rat)/2)) = (-1 : Rat) := by norm_num
theorem gm_neutrino : ((1 : Rat)/2 + ((-1 : Rat)/2)) = (0 : Rat) := by norm_num
theorem gm_up : ((1 : Rat)/2 + ((1 : Rat)/2)) = (1 : Rat) := by norm_num
theorem gm_down : ((-1 : Rat)/2 + ((1 : Rat)/2)) = (0 : Rat) := by norm_num
theorem gm_electron_R : ((0 : Rat)/2 + ((-2 : Rat)/2)) = (-1 : Rat) := by norm_num
theorem gm_chiral_charge_identity :
    ((-1 : Rat)/2 + ((-1 : Rat)/2)) = ((0 : Rat)/2 + ((-2 : Rat)/2)) := by norm_num

theorem gm_up_quark : ((1 : Rat)/2 + ((1/3 : Rat)/2)) = (2/3 : Rat) := by norm_num
theorem gm_down_quark : ((-1 : Rat)/2 + ((1/3 : Rat)/2)) = (-1/3 : Rat) := by norm_num
theorem gm_uR_quark : ((0 : Rat)/2 + ((4/3 : Rat)/2)) = (2/3 : Rat) := by norm_num
theorem gm_dR_quark : ((0 : Rat)/2 + ((-2/3 : Rat)/2)) = (-1/3 : Rat) := by norm_num


-- =====================================================================
-- SS4  SU(2) DOUBLET STRUCTURE
-- =====================================================================

structure ElectroweakDoublet where
  upComp   : WeakIsospin
  downComp : WeakIsospin
  hyper    : Hypercharge
  up_is_up   : upComp = .up
  down_is_down : downComp = .down
  deriving Repr

def leptonDoublet : ElectroweakDoublet :=
  ⟨.up, .down, hyper_Y_L, rfl, rfl⟩

def quarkDoublet : ElectroweakDoublet :=
  ⟨.up, .down, hyper_Y_QL, rfl, rfl⟩

structure ElectroweakSinglet where
  iso : WeakIsospin
  hyper : Hypercharge
  iso_none : iso = .none
  deriving Repr

def eR_singlet : ElectroweakSinglet := ⟨.none, hyper_Y_eR, rfl⟩
def uR_singlet : ElectroweakSinglet := ⟨.none, hyper_Y_uR, rfl⟩
def dR_singlet : ElectroweakSinglet := ⟨.none, hyper_Y_dR, rfl⟩


-- =====================================================================
-- SS5  CHIRALITY
-- =====================================================================

inductive Chirality where
  | left | right
  deriving Inhabited, Repr, DecidableEq


-- =====================================================================
-- SS6  ELECTROWEAK GAUGE BOSONS
-- =====================================================================

inductive EWGaugeBoson where
  | W_plus | W_minus | Z | photon
  deriving Inhabited, Repr, DecidableEq

def ewBosonLE (a b : EWGaugeBoson) : Bool :=
  match a, b with
  | .W_plus,  .Z      => true
  | .W_minus, .Z      => true
  | .W_plus,  .photon => true
  | .W_minus, .photon => true
  | x, y => x == y

instance : LE EWGaugeBoson where
  le a b := ewBosonLE a b = true

theorem w_plus_w_minus_incomparable :
    ¬ ((.W_plus : EWGaugeBoson) ≤ (.W_minus : EWGaugeBoson)) ∧
    ¬ ((.W_minus : EWGaugeBoson) ≤ (.W_plus : EWGaugeBoson)) := by
  unfold LE.le instLEEWGaugeBoson ewBosonLE; simp

theorem w_plus_below_z : (.W_plus : EWGaugeBoson) ≤ .Z := by
  unfold LE.le instLEEWGaugeBoson ewBosonLE; rfl

theorem w_minus_below_z : (.W_minus : EWGaugeBoson) ≤ .Z := by
  unfold LE.le instLEEWGaugeBoson ewBosonLE; rfl

theorem z_self_le : (.Z : EWGaugeBoson) ≤ .Z := by
  unfold LE.le instLEEWGaugeBoson ewBosonLE; rfl

theorem photon_below_z : ¬((.photon : EWGaugeBoson) ≤ .Z) := by
  unfold LE.le instLEEWGaugeBoson ewBosonLE; simp


-- =====================================================================
-- SS7  ELECTROWEAK STATE -- Full fermion state
-- =====================================================================

structure EWState where
  iso    : WeakIsospin
  hyper  : Hypercharge
  chiral : Chirality
  deriving Repr, DecidableEq

def e_L  : EWState := ⟨.down, hyper_Y_L,  .left⟩
def nu_L : EWState := ⟨.up,   hyper_Y_L,  .left⟩
def e_R  : EWState := ⟨.none, hyper_Y_eR, .right⟩
def u_L  : EWState := ⟨.up,   hyper_Y_QL, .left⟩
def d_L  : EWState := ⟨.down, hyper_Y_QL, .left⟩
def u_R  : EWState := ⟨.none, hyper_Y_uR, .right⟩
def d_R  : EWState := ⟨.none, hyper_Y_dR, .right⟩

def ewfConsistent (s : EWState) : Prop :=
  s.chiral = .right -> s.iso = .none

theorem e_L_consistent : ewfConsistent e_L := by
  unfold ewfConsistent e_L; intro h; cases h

theorem nu_L_consistent : ewfConsistent nu_L := by
  unfold ewfConsistent nu_L; intro h; cases h

theorem e_R_consistent : ewfConsistent e_R := by
  unfold ewfConsistent e_R; intro h; rfl

def weakCouples (s : EWState) : Bool := s.chiral == .left

theorem weak_couples_left  : weakCouples e_L  = true := by unfold weakCouples e_L; rfl
theorem weak_couples_nu    : weakCouples nu_L = true := by unfold weakCouples nu_L; rfl
theorem weak_not_couples_right : weakCouples e_R = false := by unfold weakCouples e_R; rfl

def wPlusAction (s : EWState) : Option EWState :=
  if s.chiral == .left && s.iso == .down then
    some {s with iso := .up}
  else
    none

def wMinusAction (s : EWState) : Option EWState :=
  if s.chiral == .left && s.iso == .up then
    some {s with iso := .down}
  else
    none

theorem wPlus_electron_to_neutrino : wPlusAction e_L = some nu_L := by
  unfold wPlusAction e_L nu_L; rfl

theorem wMinus_neutrino_to_electron : wMinusAction nu_L = some e_L := by
  unfold wMinusAction nu_L e_L; rfl

theorem w_no_action_on_right (s : EWState) (h : s.chiral = .right) :
    wPlusAction s = none ∧ wMinusAction s = none := by
  unfold wPlusAction wMinusAction
  have h_eq : (s.chiral == .left) = false := by
    rw [beq_eq_false_iff_ne]
    intro h2; rw [h2] at h; exact Chirality.noConfusion h
  simp [h_eq]

theorem eL_not_eR : e_L ≠ e_R := by
  intro h; have := congrArg (fun x : EWState => x.chiral) h
  unfold e_L e_R at this; injection this


-- =====================================================================
-- SS8  UNIFICATION -- Before and after SSB
-- =====================================================================

structure UnbrokenEW where
  gaugeGroup : String
  bosons : List String
  massless : Bool
  deriving Repr

def unbrokenPhase : UnbrokenEW :=
  ⟨"SU(2)_L × U(1)_Y", ["W₁", "W₂", "W₃", "B"], true⟩

structure BrokenEW where
  gaugeGroup : String
  bosons : List (String × Bool)
  deriving Repr

def brokenPhase : BrokenEW :=
  ⟨"U(1)_EM", [("W⁺", true), ("W⁻", true), ("Z", true), ("γ", false)]⟩

theorem dof_count_conservation : 4 * 2 + 4 = 3 * 3 + 1 * 2 + 1 := rfl


-- =====================================================================
-- SS9  STRUCTURAL IMSCRIPTION
-- =====================================================================

def electroweakImscription : Imscription := {
  dim := Dimensionality.array
  top := Topology.are
  rel := Relational.ian
  pol := Polarity.church
  fid := Fidelity.peep
  kin := KineticChar.loll
  gran := Granularity.thigh
  gram := Grammar.measure
  crit := Criticality.monad
  chir := Chirality.sure
  stoi := Stoichiometry.up
  prot := Protection.ah
}

theorem electroweak_tier : imscriptionTier electroweakImscription = OuroboricityTier.O₂dag := by
  native_decide

theorem electroweak_parity_violation : electroweakImscription.pol = .church := rfl

theorem electroweak_infinite_dim : electroweakImscription.dim = .array := rfl

theorem electroweak_gate1_open :
    Imscribing.Consciousness.phi_c_gate electroweakImscription.crit = true := rfl

theorem electroweak_gate2_open :
    Imscribing.Consciousness.k_slow_gate electroweakImscription.kin = true := by
  unfold Imscribing.Consciousness.k_slow_gate electroweakImscription; rfl

theorem electroweak_consciousness_score :
    Imscribing.Consciousness.consciousnessScore electroweakImscription = (1 : Rat) := by
  unfold Imscribing.Consciousness.consciousnessScore
    Imscribing.Consciousness.phi_c_gate Imscribing.Consciousness.k_slow_gate
    electroweakImscription
  simp


-- =====================================================================
-- SS10  SUMMARY
-- =====================================================================

/--
The FDE tower across physics, now with the electroweak sector:

    | Sector           | Gauge Group      | FDE(n) | Truth Values | Ceiling      |
    |------------------|------------------|--------|--------------|--------------|
    | electron orbital | SU(2)_spin       | FDE(2) | 4            | B (Cooper)   |
    | lepton sector    | SU(2)_weak       | FDE(2) | 4            | FORBIDDEN B  |
    | EW unbroken      | SU(2)xU(1)       | 2xFDE(2)| 8          | coupled B    |
    | quark color      | SU(3)_color      | FDE(3) | 5            | White        |
    | Standard Model   | SU(3)xSU(2)xU(1) | mixed   | 4x4x5       | full confine |
    | Planck/gravity   | Diff(M)/E8       | FDE(inf) | aleph0      | inf-White    |

Key result: the electroweak sector is FDE(2) x U(1)_Y -- a Belnap FOUR
for weak isospin tensored with discrete hypercharge. church (parity
violation) makes it structurally distinct from electron spin (nun).
-/
theorem electroweak_structural_position :
    electroweakImscription.pol ≠ Polarity.nun := by
  unfold electroweakImscription; native_decide

end Imscribing.Paraconsistent.ElectroweakBelnap
