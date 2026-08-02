-- ImscribingLean4/Primitives/Crystal.lean
-- Crystal arithmetic: full encode/decode Imscription ↔ Nat (0..17279999).
-- Frobenius address: 𝓕₃³ × 𝓕₄⁵ × 𝓕₅⁴ numbering.
-- 𝓕₃ = 27 (F,G,S), 𝓕₄=1024 (D,R,Γ,H,Ω), 𝓕₅=625 (T,P,Φ,K)

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Mathlib.Tactic.Set

namespace Imscribing.Primitives

open Dimensionality Topology Relational Polarity Grammar Fidelity KineticChar
     Granularity Criticality Protection Stoichiometry Chirality

-- Order-respecting indices (0 = minimal Ord, increasing)
def idx_D (d : Dimensionality) : Nat :=
  match d with
  | .dead   => 0
  | .ash => 1
  | .array   => 2
  | .if'    => 3

def idx_R (r : Relational) : Nat :=
  match r with
  | .ado  => 0
  | .tot    => 1
  | .ear => 2
  | .ian     => 3

def idx_Γ (g : Grammar) : Nat :=
  match g with
  | .vow   => 0
  | .gag    => 1
  | .measure   => 2
  | .ooze => 3

def idx_H (h : Chirality) : Nat :=
  match h with
  | .fee    => 0
  | .kick    => 1
  | .sure    => 2
  | .wool => 3

def idx_Ω (o : Protection) : Nat :=
  match o with
  | .awe  => 0
  | .oak => 1
  | .ah  => 2
  | .zoo => 3

def idx_T (t : Topology) : Nat :=
  match t with
  | .judge => 0
  | .eat      => 1
  | .mime  => 2
  | .oil     => 3
  | .are    => 4

def idx_P (p : Polarity) : Nat :=
  match p with
  | .church    => 0
  | .yew     => 1
  | .out      => 2
  | .nun     => 3
  | .or'  => 4

def idx_Φ (c : Criticality) : Nat :=
  match c with
  | .woe        => 0
  | .monad          => 1
  | .roar  => 2
  | .err         => 3
  | .haha      => 4

def idx_K (k : KineticChar) : Nat :=
  match k with
  | .yea => 0
  | .loll  => 1
  | .egg => 2
  | .on => 3
  | .air  => 4

def idx_F (f : Fidelity) : Nat :=
  match f with
  | .age => 0
  | .they => 1
  | .peep => 2

def idx_G (g : Granularity) : Nat :=
  match g with
  | .bib  => 0
  | .thigh => 1
  | .ice => 2

def idx_S (s : Stoichiometry) : Nat :=
  match s with
  | .hung => 0
  | .so     => 1
  | .up     => 2

-- Inverse: Nat index → primitive value (already correct)
def dim_of_nat : Nat → Dimensionality
  | 0 => dead | 1 => ash | 2 => array | 3 => if' | _ => dead
def rel_of_nat : Nat → Relational
  | 0 => ado | 1 => tot | 2 => ear | 3 => ian | _ => ado
def gram_of_nat : Nat → Grammar
  | 0 => vow | 1 => gag | 2 => measure | 3 => ooze | _ => vow
def chir_of_nat : Nat → Chirality
  | 0 => fee | 1 => kick | 2 => sure | 3 => wool | _ => fee
def prot_of_nat : Nat → Protection
  | 0 => awe | 1 => oak | 2 => ah | 3 => zoo | _ => awe

def top_of_nat : Nat → Topology
  | 0 => judge | 1 => eat | 2 => mime | 3 => oil | 4 => are | _ => judge
def pol_of_nat : Nat → Polarity
  | 0 => church | 1 => yew | 2 => out | 3 => nun | 4 => or' | _ => church
def crit_of_nat : Nat → Criticality
  | 0 => woe | 1 => monad | 2 => roar | 3 => err | 4 => haha | _ => woe
def kin_of_nat : Nat → KineticChar
  | 0 => yea | 1 => loll | 2 => egg | 3 => on | 4 => air | _ => yea

def fid_of_nat : Nat → Fidelity
  | 0 => age | 1 => they | 2 => peep | _ => age
def gran_of_nat : Nat → Granularity
  | 0 => bib | 1 => thigh | 2 => ice | _ => bib
def stoi_of_nat : Nat → Stoichiometry
  | 0 => hung | 1 => so | 2 => up | _ => hung

-- Crystal address
def crystal_encode (s : Imscription) : Nat :=
  let f3 := idx_F s.fid * 9 + idx_G s.gran * 3 + idx_S s.stoi  -- 0-26
  let f4 := idx_D s.dim * 256 + idx_R s.rel * 64
    + idx_Γ s.gram * 16 + idx_H s.chir * 4 + idx_Ω s.prot  -- 0-1023
  let f5 := idx_T s.top * 125 + idx_P s.pol * 25 + idx_Φ s.crit * 5 + idx_K s.kin  -- 0-624
  f3 + 27 * f4 + 27 * 1024 * f5

def crystal_decode (addr : Nat) : Imscription :=
  let f3_raw := addr % 27
  let f4_raw := (addr / 27) % 1024
  let f5_raw := addr / 27648  -- 27*1024
  { dim   := dim_of_nat   (f4_raw / 256)
  , top   := top_of_nat   (f5_raw / 125)
  , rel   := rel_of_nat   ((f4_raw / 64) % 4)
  , pol   := pol_of_nat   ((f5_raw / 25) % 5)
  , fid   := fid_of_nat   (f3_raw / 9)
  , kin   := kin_of_nat   (f5_raw % 5)
  , gran  := gran_of_nat  ((f3_raw / 3) % 3)
  , gram  := gram_of_nat  ((f4_raw / 16) % 4)
  , crit  := crit_of_nat  ((f5_raw / 5) % 5)
  , chir  := chir_of_nat  ((f4_raw / 4) % 4)
  , stoi  := stoi_of_nat  (f3_raw % 3)
  , prot  := prot_of_nat  (f4_raw % 4) }

theorem crystal_total_size : 27 * 1024 * 625 = 17280000 := by decide

-- Roundtrip: decode ∘ encode = id
set_option maxHeartbeats 800000 in
-- 12-field mixed-radix roundtrip: 12 × (set + omega) bursts exceed the default 200k budget
theorem crystal_roundtrip (s : Imscription) : crystal_decode (crystal_encode s) = s := by
  -- Index bounds: each idx fits within its digit radix
  have hD : idx_D s.dim  < 4 := by cases s.dim  <;> simp [idx_D]
  have hR : idx_R s.rel  < 4 := by cases s.rel  <;> simp [idx_R]
  have hΓ : idx_Γ s.gram < 4 := by cases s.gram <;> simp [idx_Γ]
  have hH : idx_H s.chir < 4 := by cases s.chir <;> simp [idx_H]
  have hΩ : idx_Ω s.prot < 4 := by cases s.prot <;> simp [idx_Ω]
  have hT : idx_T s.top  < 5 := by cases s.top  <;> simp [idx_T]
  have hP : idx_P s.pol  < 5 := by cases s.pol  <;> simp [idx_P]
  have hΦ : idx_Φ s.crit < 5 := by cases s.crit <;> simp [idx_Φ]
  have hK : idx_K s.kin  < 5 := by cases s.kin  <;> simp [idx_K]
  have hF : idx_F s.fid  < 3 := by cases s.fid  <;> simp [idx_F]
  have hG : idx_G s.gran < 3 := by cases s.gran <;> simp [idx_G]
  have hS : idx_S s.stoi < 3 := by cases s.stoi <;> simp [idx_S]
  -- Per-field roundtrip: prim_of_nat ∘ idx = id
  have h_dim  : dim_of_nat  (idx_D s.dim)  = s.dim  := by cases s.dim  <;> simp [idx_D,  dim_of_nat]
  have h_top  : top_of_nat  (idx_T s.top)  = s.top  := by cases s.top  <;> simp [idx_T,  top_of_nat]
  have h_rel  : rel_of_nat  (idx_R s.rel)  = s.rel  := by cases s.rel  <;> simp [idx_R,  rel_of_nat]
  have h_pol  : pol_of_nat  (idx_P s.pol)  = s.pol  := by cases s.pol  <;> simp [idx_P,  pol_of_nat]
  have h_fid  : fid_of_nat  (idx_F s.fid)  = s.fid  := by cases s.fid  <;> simp [idx_F,  fid_of_nat]
  have h_kin  : kin_of_nat  (idx_K s.kin)  = s.kin  := by cases s.kin  <;> simp [idx_K,  kin_of_nat]
  have h_gran : gran_of_nat (idx_G s.gran) = s.gran := by cases s.gran <;> simp [idx_G,  gran_of_nat]
  have h_gram : gram_of_nat (idx_Γ s.gram) = s.gram := by cases s.gram <;> simp [idx_Γ,  gram_of_nat]
  have h_crit : crit_of_nat (idx_Φ s.crit) = s.crit := by cases s.crit <;> simp [idx_Φ,  crit_of_nat]
  have h_chir : chir_of_nat (idx_H s.chir) = s.chir := by cases s.chir <;> simp [idx_H,  chir_of_nat]
  have h_stoi : stoi_of_nat (idx_S s.stoi) = s.stoi := by cases s.stoi <;> simp [idx_S,  stoi_of_nat]
  have h_prot : prot_of_nat (idx_Ω s.prot) = s.prot := by cases s.prot <;> simp [idx_Ω,  prot_of_nat]
  -- Introduce block variables; set gives unfoldable defs for omega.
  set f3 := idx_F s.fid * 9 + idx_G s.gran * 3 + idx_S s.stoi
  set f4 := idx_D s.dim * 256 + idx_R s.rel * 64 + idx_Γ s.gram * 16 + idx_H s.chir * 4 + idx_Ω s.prot
  set f5 := idx_T s.top * 125 + idx_P s.pol * 25 + idx_Φ s.crit * 5 + idx_K s.kin
  -- Block bounds (omega derives from individual bounds)
  have hf3 : f3 < 27   := by omega
  have hf4 : f4 < 1024 := by omega
  have hf5 : f5 < 625  := by omega
  -- Per-field one-level digit extractions (omega with 5 vars each)
  have he_dim  : f4 / 256       = idx_D s.dim  := by omega
  have he_rel  : f4 / 64 % 4   = idx_R s.rel  := by omega
  have he_gram : f4 / 16 % 4   = idx_Γ s.gram := by omega
  have he_chir : f4 / 4 % 4    = idx_H s.chir := by omega
  have he_prot : f4 % 4         = idx_Ω s.prot := by omega
  have he_top  : f5 / 125       = idx_T s.top  := by omega
  have he_pol  : f5 / 25 % 5   = idx_P s.pol  := by omega
  have he_crit : f5 / 5 % 5    = idx_Φ s.crit := by omega
  have he_kin  : f5 % 5         = idx_K s.kin  := by omega
  have he_fid  : f3 / 9         = idx_F s.fid  := by omega
  have he_gran : f3 / 3 % 3    = idx_G s.gran := by omega
  have he_stoi : f3 % 3         = idx_S s.stoi := by omega
  -- Rewrite encode as f3 + 27*f4 + 27*1024*f5
  have henc : crystal_encode s = f3 + 27 * f4 + 27 * 1024 * f5 := by
    simp only [crystal_encode]; omega
  rw [henc]
  simp only [crystal_decode]
  -- dsimp only reduces struct projection; rw transforms RHS; congr 1 then omega (3 vars).
  ext
  · dsimp only; rw [← h_dim, ← he_dim]; congr 1; omega
  · dsimp only; rw [← h_top, ← he_top]; congr 1; omega
  · dsimp only; rw [← h_rel, ← he_rel]; congr 1; omega
  · dsimp only; rw [← h_pol, ← he_pol]; congr 1; omega
  · dsimp only; rw [← h_fid, ← he_fid]; congr 1; omega
  · dsimp only; rw [← h_kin, ← he_kin]; congr 1; omega
  · dsimp only; rw [← h_gran, ← he_gran]; congr 1; omega
  · dsimp only; rw [← h_gram, ← he_gram]; congr 1; omega
  · dsimp only; rw [← h_crit, ← he_crit]; congr 1; omega
  · dsimp only; rw [← h_chir, ← he_chir]; congr 1; omega
  · dsimp only; rw [← h_stoi, ← he_stoi]; congr 1; omega
  · dsimp only; rw [← h_prot, ← he_prot]; congr 1; omega

end Imscribing.Primitives
