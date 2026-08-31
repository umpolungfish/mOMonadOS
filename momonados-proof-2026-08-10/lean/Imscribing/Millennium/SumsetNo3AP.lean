import Mathlib.Data.Finset.Basic
import Mathlib.Tactic

/-!
# Sumset No-3-AP Theorem

Let S(n) be the set of subset sums of {3⁰, 3¹, …, 3ⁿ⁻¹}.
Theorem: S(n) contains no non-trivial 3-term arithmetic progression.

If x, y, z ∈ S n and x + z = 2y, then x = y = z.

Grammar tuple: ⟨𐑼𐑶𐑛𐑗𐑱𐑧𐑒𐑵⊙𐑒𐑙𐑷⟩
-/

namespace SumsetNo3AP

def S : ℕ → Finset ℕ
  | 0 => {0}
  | n+1 => S n ∪ ((S n).image (fun x => x + 3 ^ n))

lemma S_zero_eq : S 0 = {0} := rfl

lemma mem_S_succ (n x : ℕ) : x ∈ S (n+1) ↔ x ∈ S n ∨ ∃ y ∈ S n, x = y + 3 ^ n := by
  simp [S, Finset.mem_union, Finset.mem_image, eq_comm]

lemma mod_three_le_one (n x : ℕ) (hx : x ∈ S n) : x % 3 ≤ 1 := by
  revert x
  induction n with
  | zero =>
    intro x hx
    have hx0 : x = 0 := by simpa [S_zero_eq] using hx
    subst hx0; norm_num
  | succ n ih =>
    intro x hx
    rw [mem_S_succ] at hx
    rcases hx with (hx' | ⟨y, hy, hx_eq⟩)
    · exact ih x hx'
    · rw [hx_eq]
      by_cases hn : n = 0
      · subst hn
        have hy0 : y = 0 := by simpa [S_zero_eq] using hy
        subst hy0; norm_num
      · have hpos : 1 ≤ n := by omega
        have h3 : 3 ∣ (3 : ℕ) ^ n := Nat.pow_dvd_pow 3 hpos
        -- (y + 3^n) % 3 = (y%3 + 0) % 3 = y%3
        simpa [Nat.add_mod, Nat.mod_eq_zero_of_dvd h3, Nat.mod_mod] using ih y hy

lemma mod3_equality (x y z : ℕ) (hsum : x + z = 2 * y)
    (hx : x % 3 ≤ 1) (hy : y % 3 ≤ 1) (hz : z % 3 ≤ 1) :
    x % 3 = y % 3 ∧ y % 3 = z % 3 := by
  have h3pos : 0 < 3 := by norm_num
  have hx01 : x % 3 = 0 ∨ x % 3 = 1 := by
    have h := Nat.mod_lt x h3pos; omega
  have hy01 : y % 3 = 0 ∨ y % 3 = 1 := by
    have h := Nat.mod_lt y h3pos; omega
  have hz01 : z % 3 = 0 ∨ z % 3 = 1 := by
    have h := Nat.mod_lt z h3pos; omega
  have h_mod : (x + z) % 3 = (2 * y) % 3 := by rw [hsum]
  rw [Nat.add_mod, Nat.mul_mod] at h_mod
  rcases hx01 with (hx0|hx1) <;> rcases hy01 with (hy0|hy1) <;> rcases hz01 with (hz0|hz1)
  · rw [hx0, hy0, hz0] at h_mod ⊢; norm_num
  · rw [hx0, hy0, hz1] at h_mod; norm_num at h_mod
  · rw [hx0, hy1, hz0] at h_mod; norm_num at h_mod
  · rw [hx0, hy1, hz1] at h_mod; norm_num at h_mod
  · rw [hx1, hy0, hz0] at h_mod; norm_num at h_mod
  · rw [hx1, hy0, hz1] at h_mod; norm_num at h_mod
  · rw [hx1, hy1, hz0] at h_mod; norm_num at h_mod
  · rw [hx1, hy1, hz1] at h_mod ⊢; norm_num

lemma shift_down (n x : ℕ) (hx : x ∈ S (n+1)) : (x - x % 3) / 3 ∈ S n := by
  revert x
  induction n with
  | zero =>
    intro x hx
    rw [mem_S_succ] at hx
    rcases hx with (hx' | ⟨y, hy, hx_eq⟩)
    · have hx0 : x = 0 := by simpa [S_zero_eq] using hx'
      subst hx0; norm_num; simp [S_zero_eq]
    · have hy0 : y = 0 := by simpa [S_zero_eq] using hy
      subst hy0; rw [hx_eq]; norm_num; simp [S_zero_eq]
  | succ n ih =>
    intro x hx
    rw [mem_S_succ] at hx
    rcases hx with (hx' | ⟨y, hy, hx_eq⟩)
    · have hq := ih x hx'
      rw [mem_S_succ]; left; exact hq
    · rw [hx_eq]
      have hpos : 1 ≤ n+1 := by omega
      have h3 : 3 ∣ (3 : ℕ)^(n+1) := Nat.pow_dvd_pow 3 hpos
      have h3mod : ((3 : ℕ)^(n+1)) % 3 = 0 := Nat.mod_eq_zero_of_dvd h3
      -- (y + 3^{n+1}) % 3 = y % 3  (since 3^{n+1} ≡ 0 mod 3)
      have hxmod : (y + (3 : ℕ)^(n+1)) % 3 = y % 3 := by
        rw [Nat.add_mod, h3mod, add_zero, Nat.mod_mod]
      -- Decompose y = d + 3k where d = y%3, k = y/3
      let d := y % 3
      let k := y / 3
      have hy_eq : y = d + 3 * k := by dsimp [d, k]; omega
      -- k = (y - d)/3 ∈ S n  (by IH)
      have hk_mem : k ∈ S n := by
        have htemp := ih y hy  -- (y - y%3)/3 ∈ S n
        -- Since d = y%3, (y - d)/3 = (y - y%3)/3, and from hy_eq: k = (y-d)/3
        have hk_eq : k = (y - y % 3) / 3 := by dsimp [d, k]; omega
        rw [hk_eq]; exact htemp
      -- Compute: ((y + 3^{n+1}) - y%3) / 3 = k + 3^n
      have hcalc : ((y + (3 : ℕ)^(n+1)) - (y % 3)) / 3 = k + (3 : ℕ)^n := by
        rw [hy_eq]
        -- ((d + 3k + 3^{n+1}) - (d+3k)%3) / 3
        -- (d+3k)%3 = d (since 3k ≡ 0 mod 3 and d < 3)
        have hmod_simp : (d + 3 * k) % 3 = d := by
          dsimp [d]
          -- (y%3 + 3*k) % 3 = (y%3)%3 = y%3 = d
          rw [Nat.add_mod, Nat.mul_mod, show (3 : ℕ) % 3 = 0 by norm_num]
          simp
        rw [hmod_simp]
        -- ((d + 3k + 3^{n+1}) - d) / 3 = (3k + 3^{n+1}) / 3
        have hsub : d + 3 * k + (3 : ℕ)^(n+1) - d = 3 * k + (3 : ℕ)^(n+1) := by omega
        rw [hsub]
        -- (3k + 3^{n+1}) / 3 = k + 3^n
        rw [Nat.pow_succ]
        rw [show (3 : ℕ)^n * 3 = 3 * (3 : ℕ)^n by ring]
        -- 3k + 3*(3^n) = 3*(k + 3^n)
        have h_factor : 3 * k + 3 * ((3 : ℕ)^n) = 3 * (k + (3 : ℕ)^n) := by ring
        rw [h_factor, Nat.mul_div_cancel_left _ (by norm_num)]
      rw [hxmod, hcalc]
      rw [mem_S_succ]
      right; exact ⟨k, hk_mem, rfl⟩

theorem no_three_term_ap (n x y z : ℕ)
    (hx : x ∈ S n) (hy : y ∈ S n) (hz : z ∈ S n) (hsum : x + z = 2 * y) : x = y ∧ y = z := by
  revert x y z
  induction n with
  | zero =>
    intro x y z hx hy hz hsum
    have hx0 : x = 0 := by simpa [S_zero_eq] using hx
    have hy0 : y = 0 := by simpa [S_zero_eq] using hy
    have hz0 : z = 0 := by simpa [S_zero_eq] using hz
    rw [hx0, hy0, hz0]; exact ⟨rfl, rfl⟩
  | succ n ih =>
    intro x y z hx hy hz hsum
    have hx_mod : x % 3 ≤ 1 := mod_three_le_one (n+1) x hx
    have hy_mod : y % 3 ≤ 1 := mod_three_le_one (n+1) y hy
    have hz_mod : z % 3 ≤ 1 := mod_three_le_one (n+1) z hz
    rcases mod3_equality x y z hsum hx_mod hy_mod hz_mod with ⟨hxy_mod, hyz_mod⟩
    -- hxy_mod: x%3 = y%3, hyz_mod: y%3 = z%3
    let d := x % 3
    have hdy : y % 3 = d := hxy_mod.symm
    have hdz : z % 3 = d := by rw [← hdy, hyz_mod]
    -- Decomposition via shift_down
    have hx'_mem : (x - d) / 3 ∈ S n := by
      simpa [d] using shift_down n x hx
    have hy'_mem : (y - d) / 3 ∈ S n := by
      simpa [d, hdy] using shift_down n y hy
    have hz'_mem : (z - d) / 3 ∈ S n := by
      simpa [d, hdz] using shift_down n z hz
    let x' := (x - d) / 3
    let y' := (y - d) / 3
    let z' := (z - d) / 3
    have hx'_mem' : x' ∈ S n := hx'_mem
    have hy'_mem' : y' ∈ S n := hy'_mem
    have hz'_mem' : z' ∈ S n := hz'_mem
    -- x = d + 3*x' using Nat.div_add_mod
    have hx_eq : x = d + 3 * x' := by
      dsimp [x', d]; omega
    have hy_eq : y = d + 3 * y' := by
      have := Nat.div_add_mod y 3
      dsimp [y', d] at this ⊢
      rw [hdy] at this
      omega
    have hz_eq : z = d + 3 * z' := by
      have := Nat.div_add_mod z 3
      dsimp [z', d] at this ⊢
      rw [hdz] at this
      omega
    -- x+z = 2y → x'+z' = 2y'
    have hsum' : x' + z' = 2 * y' := by
      rw [hx_eq, hy_eq, hz_eq] at hsum; omega
    -- Induction hypothesis
    rcases ih x' y' z' hx'_mem' hy'_mem' hz'_mem' hsum' with ⟨hxy', hyz'⟩
    rw [hx_eq, hy_eq, hz_eq, hxy', hyz']
    exact ⟨rfl, rfl⟩

end SumsetNo3AP
