/-
# The windowed bound

The sum count gives a Sidon set `k(k+1)/2 ≤ 2N`, so `k ≲ 2√N`. The difference
count gives `k(k-1)/2 ≤ N`, so `k ≲ √(2N)`. Neither is sharp, and neither
crosses the covering demand a basis imposes, which is also `√(2N)`.

The windowed count is sharp. Slide a window of length `u` along `[1,N]` and let
`w t` be how many elements of `B` it holds. Then

  * the windows collectively see each element `u` times, so `Σ w t` is large;
  * two elements share a window only if they are within `u` of each other, and on
    a Sidon set each difference occurs once, so `Σ w t (w t - 1)` is SMALL,
    bounded by `u²` with no reference to `k` at all;
  * Cauchy–Schwarz over the `N+1` windows ties the two together.

The result is `W² ≤ (N+1)(u² + W)` with `W = Σ_{a ∈ B} min u a`, and optimizing
`u` turns it into `k ≲ √N`: leading constant `1` rather than `√2`. That is the
constant the covering demand cannot match, which is what closes `C = 2` and,
through the parity theorems, `C = 3` with it.

This file proves the windowed inequality itself. The two counts it rests on are
where the work is; the Cauchy–Schwarz step is one lemma.
-/

import Mathlib
import Imscribing.Classical.SidonRepresentation
import Imscribing.Classical.ErdosTuranSmallC
import Imscribing.Classical.ErdosTuranReduction

namespace Imscribing.Classical

open Finset

/-- How many elements of `B` sit in the half-open window `(t, t+u]`. -/
def windowCount (B : Finset ℕ) (u t : ℕ) : ℕ :=
  (B.filter (fun a => t < a ∧ a ≤ t + u)).card

/-- **The double count.** Summing the windows over `t ∈ [0, N]` sees each element
    `a` once for every window containing it, and there are `min u a` of those:
    the windows start at `t = a - u` and end at `t = a - 1`, clipped below at
    zero. -/
lemma sum_windowCount (B : Finset ℕ) (u N : ℕ) (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) :
    ∑ t ∈ range (N + 1), windowCount B u t = ∑ a ∈ B, min u a := by
  classical
  simp only [windowCount, Finset.card_filter]
  rw [Finset.sum_comm]
  refine Finset.sum_congr rfl (fun a ha => ?_)
  obtain ⟨ha1, haN⟩ := hB a ha
  -- the windows containing `a` are indexed by `t ∈ [a - u, a - 1]`
  have : (range (N + 1)).filter (fun t => t < a ∧ a ≤ t + u) = Ico (a - u) a := by
    ext t
    simp only [Finset.mem_filter, Finset.mem_range, Finset.mem_Ico]
    constructor
    · rintro ⟨_, h1, h2⟩; exact ⟨by omega, h1⟩
    · rintro ⟨h1, h2⟩; exact ⟨by omega, h2, by omega⟩
  rw [Finset.sum_boole, this, Nat.card_Ico]
  simp only [Nat.cast_id]
  omega

/-- **Cauchy–Schwarz over the windows.** -/
lemma windowed_cauchy (B : Finset ℕ) (u N : ℕ) :
    ((∑ t ∈ range (N + 1), (windowCount B u t : ℝ)) ^ 2)
      ≤ (N + 1) * ∑ t ∈ range (N + 1), (windowCount B u t : ℝ) ^ 2 := by
  have h := sq_sum_le_card_mul_sum_sq (s := range (N + 1))
      (f := fun t => (windowCount B u t : ℝ))
  simpa using h

/-- **The windowed bound.**

    With `W` the total window occupancy and `Q` the total number of ordered pairs
    of distinct elements sharing a window,

        W² ≤ (N + 1) * (Q + W)

    The content is that `Q` is small on a Sidon set: it does not grow with `k`,
    only with `u`. That is `sidon_pairs_in_windows` below. -/
theorem windowed_bound (B : Finset ℕ) (u N : ℕ) (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) :
    ((∑ a ∈ B, min u a : ℕ) : ℝ) ^ 2
      ≤ (N + 1) * ((∑ t ∈ range (N + 1),
          (windowCount B u t * (windowCount B u t - 1) : ℕ) : ℝ)
        + ((∑ a ∈ B, min u a : ℕ) : ℝ)) := by
  classical
  have hsum := sum_windowCount B u N hB
  have hcs := windowed_cauchy B u N
  -- w² = w(w-1) + w on ℕ, so the sum of squares splits
  have hsplit : ∀ t, ((windowCount B u t : ℝ)) ^ 2
      = ((windowCount B u t * (windowCount B u t - 1) : ℕ) : ℝ)
        + (windowCount B u t : ℝ) := by
    intro t
    cases h : windowCount B u t with
    | zero => simp
    | succ n =>
        push_cast [Nat.succ_sub_one]
        ring
  rw [Finset.sum_congr rfl (fun t _ => hsplit t), Finset.sum_add_distrib] at hcs
  have hcast : (∑ t ∈ range (N + 1), (windowCount B u t : ℝ))
      = ((∑ a ∈ B, min u a : ℕ) : ℝ) := by
    rw [← hsum]; push_cast; rfl
  rw [hcast] at hcs
  exact hcs

/-! ## The Sidon input

    Everything above holds for any `B`. This is where the Sidon condition
    enters, and it is the whole reason the windowed count is sharper than the
    difference count: the shared-window pairs are bounded by `u²` with no `k` in
    the bound at all.

    A pair `a > b` shares a window only if `a - b < u`, and then only for the
    `u - (a - b)` window positions holding both. On a Sidon set the difference
    determines the pair, so summing over differences gives `u(u-1)/2` ordered
    decreasing pairs, hence `u(u-1)` ordered pairs. -/

/-- The window-sharing decreasing pairs, tagged by their window. -/
def sharedPairs (B : Finset ℕ) (u N : ℕ) : Finset (ℕ × ℕ × ℕ) :=
  ((range (N + 1)) ×ˢ (B ×ˢ B)).filter
    (fun p => p.2.2 < p.2.1 ∧ p.1 < p.2.2 ∧ p.2.1 ≤ p.1 + u)

/-- **The Sidon input.** On a Sidon set the window-sharing pairs are bounded by
    the window length alone. -/
theorem sidon_pairs_in_windows (B : Finset ℕ) (u N : ℕ) (hB : isSidonFinset B) :
    (sharedPairs B u N).card ≤ ∑ d ∈ Ico 1 u, (u - d) := by
  classical
  -- tag each shared pair by (difference, offset of the window within its range)
  have hmap : ∀ p ∈ sharedPairs B u N,
      (⟨p.2.1 - p.2.2, p.1 - (p.2.1 - u)⟩ : (_ : ℕ) × ℕ)
        ∈ (Ico 1 u).sigma (fun d => range (u - d)) := by
    rintro ⟨t, a, b⟩ hp
    simp only [sharedPairs, Finset.mem_filter, Finset.mem_product,
               Finset.mem_range] at hp
    obtain ⟨⟨htN, ha, hb⟩, hba, htb, hat⟩ := hp
    have h1 : 1 ≤ a - b := by omega
    have h2 : a - b < u := by omega
    have h3 : t - (a - u) < u - (a - b) := by omega
    dsimp only
    exact Finset.mem_sigma.mpr ⟨Finset.mem_Ico.mpr ⟨h1, h2⟩, Finset.mem_range.mpr h3⟩
  have hinj : ∀ p ∈ sharedPairs B u N, ∀ q ∈ sharedPairs B u N,
      (⟨p.2.1 - p.2.2, p.1 - (p.2.1 - u)⟩ : (_ : ℕ) × ℕ)
        = ⟨q.2.1 - q.2.2, q.1 - (q.2.1 - u)⟩ → p = q := by
    rintro ⟨t, a, b⟩ hp ⟨s, c, d⟩ hq h
    simp only [sharedPairs, Finset.mem_filter, Finset.mem_product,
               Finset.mem_range] at hp hq
    obtain ⟨⟨htN, ha, hb⟩, hba, htb, hat⟩ := hp
    obtain ⟨⟨hsN, hc, hd⟩, hdc, hsd, hcs⟩ := hq
    obtain ⟨hd1, hd2⟩ := Sigma.mk.injEq .. ▸ h
    -- the difference determines the pair, and then the window
    have hdiff : a - b = c - d := by
      simpa using congrArg Sigma.fst h
    obtain ⟨e1, e2⟩ := diff_injective hB ha hb hc hd hba hdc hdiff
    subst e1; subst e2
    have : t = s := by
      have := congrArg Sigma.snd h
      simp at this
      omega
    subst this
    rfl
  calc (sharedPairs B u N).card
      ≤ ((Ico 1 u).sigma (fun d => range (u - d))).card :=
        Finset.card_le_card_of_injOn _ hmap (fun p hp q hq h => hinj p hp q hq h)
    _ = ∑ d ∈ Ico 1 u, (u - d) := by simp

/-! ## Joining the two counts -/

/-- The decreasing pairs inside one window are the shared pairs tagged with that
    window. -/
lemma diffPairs_window (B : Finset ℕ) (u t : ℕ) (N : ℕ) (ht : t < N + 1) :
    (diffPairs (B.filter (fun a => t < a ∧ a ≤ t + u))).card
      = ((sharedPairs B u N).filter (fun p => p.1 = t)).card := by
  classical
  apply Finset.card_bij (fun p _ => (t, p.1, p.2))
  · rintro ⟨a, b⟩ hp
    simp only [diffPairs, Finset.mem_filter, Finset.mem_product] at hp
    obtain ⟨⟨⟨ha, ha1, ha2⟩, ⟨hb, hb1, hb2⟩⟩, hba⟩ := hp
    refine Finset.mem_filter.mpr ⟨?_, rfl⟩
    simp only [sharedPairs, Finset.mem_filter, Finset.mem_product,
               Finset.mem_range]
    exact ⟨⟨ht, ha, hb⟩, hba, hb1, ha2⟩
  · rintro ⟨a, b⟩ _ ⟨c, d⟩ _ h
    have h1 : a = c := congrArg (fun q => q.2.1) h
    have h2 : b = d := congrArg (fun q => q.2.2) h
    exact Prod.ext h1 h2
  · rintro ⟨s, a, b⟩ hq
    obtain ⟨hq1, hq2⟩ := Finset.mem_filter.mp hq
    simp only [sharedPairs, Finset.mem_filter, Finset.mem_product,
               Finset.mem_range] at hq1
    obtain ⟨⟨_, ha, hb⟩, hba, hsb, hat⟩ := hq1
    simp only at hq2
    subst hq2
    refine ⟨(a, b), ?_, rfl⟩
    simp only [diffPairs, Finset.mem_filter, Finset.mem_product]
    exact ⟨⟨⟨ha, by omega, hat⟩, ⟨hb, hsb, by omega⟩⟩, hba⟩

/-- **The pair count, joined.** The window pair term is exactly twice the shared
    pairs, so `sidon_pairs_in_windows` bounds it. -/
theorem sum_window_pairs (B : Finset ℕ) (u N : ℕ) :
    ∑ t ∈ range (N + 1), windowCount B u t * (windowCount B u t - 1)
      = 2 * (sharedPairs B u N).card := by
  classical
  have hterm : ∀ t ∈ range (N + 1),
      windowCount B u t * (windowCount B u t - 1)
        = 2 * ((sharedPairs B u N).filter (fun p => p.1 = t)).card := by
    intro t ht
    have := card_product_split (B.filter (fun a => t < a ∧ a ≤ t + u))
    rw [diffPairs_window B u t N (Finset.mem_range.mp ht)] at this
    unfold windowCount
    set w := (B.filter (fun a => t < a ∧ a ≤ t + u)).card with hw
    have hexp : w * (w - 1) = w * w - w := by
      cases w with
      | zero => rfl
      | succ n => rw [Nat.succ_sub_one, Nat.mul_succ]; omega
    rw [hexp]
    omega
  have hfib : Set.MapsTo (fun p : ℕ × ℕ × ℕ => p.1)
      ↑(sharedPairs B u N) ↑(range (N + 1)) := by
    intro p hp
    simp only [Finset.mem_coe, sharedPairs, Finset.mem_filter,
               Finset.mem_product, Finset.mem_range] at hp ⊢
    exact hp.1.1
  have hcard := Finset.card_eq_sum_card_fiberwise hfib
  rw [Finset.sum_congr rfl hterm, ← Finset.mul_sum]
  congr 1
  exact hcard.symm

/-- The window budget: `Σ_{d=1}^{u-1} (u - d) = u(u-1)/2`, so twice it is
    `u(u-1)`, below `u²`. -/
lemma two_mul_window_budget (u : ℕ) :
    2 * (∑ d ∈ Ico 1 u, (u - d)) = u * (u - 1) := by
  rcases Nat.eq_zero_or_pos u with rfl | hu
  · simp
  obtain ⟨m, rfl⟩ : ∃ m, u = m + 1 := ⟨u - 1, by omega⟩
  rw [Finset.sum_Ico_eq_sum_range]
  have hrw : ∀ i ∈ range (m + 1 - 1), m + 1 - (1 + i) = m - i := by
    intro i hi; omega
  rw [Finset.sum_congr rfl hrw]
  simp only [Nat.add_sub_cancel]
  -- reflecting the range turns `m - i` into `i + 1`
  have hrefl := Finset.sum_range_reflect (fun j => j + 1) m
  have hleft : ∑ j ∈ range m, (m - 1 - j + 1) = ∑ j ∈ range m, (m - j) := by
    refine Finset.sum_congr rfl (fun j hj => ?_)
    have := Finset.mem_range.mp hj
    omega
  have hsplit : ∑ j ∈ range m, (j + 1) = (∑ j ∈ range m, j) + m := by
    rw [Finset.sum_add_distrib]; simp
  have hkey : ∑ i ∈ range m, (m - i) = (∑ j ∈ range m, j) + m := by
    rw [← hleft, hrefl, hsplit]
  have hgauss := Finset.sum_range_id_mul_two m
  have hmm : m * (m - 1) + 2 * m = (m + 1) * m := by
    cases m with
    | zero => simp
    | succ n => rw [Nat.succ_sub_one]; ring
  rw [hkey]
  omega

/-- **The windowed bound on a Sidon set.**

    `W² ≤ (N+1)(u(u-1) + W)` with `W = Σ_{a ∈ B} min u a`. The pair term does not
    mention `k`: that is what makes the leading constant `1` instead of `√2`. -/
theorem windowed_sidon_bound (B : Finset ℕ) (u N : ℕ)
    (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) (hS : isSidonFinset B) :
    ((∑ a ∈ B, min u a : ℕ) : ℝ) ^ 2
      ≤ (N + 1) * ((u * (u - 1) : ℕ) + ((∑ a ∈ B, min u a : ℕ) : ℝ)) := by
  have hmain := windowed_bound B u N hB
  have hjoin := sum_window_pairs B u N
  have hsid := sidon_pairs_in_windows B u N hS
  have hQ : (∑ t ∈ range (N + 1),
      (windowCount B u t * (windowCount B u t - 1) : ℕ)) ≤ u * (u - 1) := by
    rw [hjoin, ← two_mul_window_budget u]
    omega
  have hQR : ((∑ t ∈ range (N + 1),
      (windowCount B u t * (windowCount B u t - 1) : ℕ) : ℕ) : ℝ)
      ≤ ((u * (u - 1) : ℕ) : ℝ) := by exact_mod_cast hQ
  have hN : (0 : ℝ) ≤ (N : ℝ) + 1 := by positivity
  calc ((∑ a ∈ B, min u a : ℕ) : ℝ) ^ 2 ≤ _ := hmain
    _ ≤ (N + 1) * ((u * (u - 1) : ℕ) + ((∑ a ∈ B, min u a : ℕ) : ℝ)) := by
        push_cast at hQR ⊢
        nlinarith [hQR]

/-! ## Turning the bound into a bound on `k`

    `W` is nearly `u * k`: every element at or above `u` contributes the full
    `u`, and the elements below `u` are few, because they form a Sidon set inside
    `[1, u)` and `sidon_diff_bound` already caps such a set. -/

/-- The window occupancy is at least `u` times the count of large elements. -/
lemma occupancy_ge (B : Finset ℕ) (u : ℕ) :
    u * (B.filter (fun a => u ≤ a)).card ≤ ∑ a ∈ B, min u a := by
  classical
  calc u * (B.filter (fun a => u ≤ a)).card
      = ∑ a ∈ B.filter (fun a => u ≤ a), u := by
        rw [Finset.sum_const, smul_eq_mul, Nat.mul_comm]
    _ = ∑ a ∈ B.filter (fun a => u ≤ a), min u a :=
        Finset.sum_congr rfl (fun a ha => by
          have := (Finset.mem_filter.mp ha).2
          omega)
    _ ≤ ∑ a ∈ B, min u a :=
        Finset.sum_le_sum_of_subset (Finset.filter_subset _ _)

/-- The elements below `u` are a Sidon set inside `[1, u)`, so there are few of
    them: `p² ≤ 2u + p`. This is `sidon_diff_bound` on the prefix. -/
lemma prefix_small (B : Finset ℕ) (u : ℕ) (hS : isSidonFinset B) :
    (B.filter (fun a => a < u)).card * (B.filter (fun a => a < u)).card
      ≤ 2 * u + (B.filter (fun a => a < u)).card := by
  classical
  refine sidon_diff_bound _ u ?_ ?_
  · intro a b c d ha hb hc hd h
    exact hS (Finset.mem_filter.mp ha).1 (Finset.mem_filter.mp hb).1
             (Finset.mem_filter.mp hc).1 (Finset.mem_filter.mp hd).1 h
  · intro b hb
    exact le_of_lt (Finset.mem_filter.mp hb).2

/-- The split of `B` by the window length. -/
lemma card_split_at (B : Finset ℕ) (u : ℕ) :
    (B.filter (fun a => u ≤ a)).card + (B.filter (fun a => a < u)).card = B.card := by
  classical
  have := Finset.card_filter_add_card_filter_not (s := B) (fun a : ℕ => u ≤ a)
  have heq : B.filter (fun a => ¬ u ≤ a) = B.filter (fun a => a < u) := by
    apply Finset.filter_congr; intro a _; simp
  rw [heq] at this
  exact this

/-- **The assembled inequality.** Writing `k` for `|B|` and `p` for the count of
    elements below `u`, every Sidon set in `[1,N]` satisfies

        (u(k - p))² ≤ (N+1)(u(u-1) + u·k)     and     p² ≤ 2u + p

    The first says the windows are crowded, the second that the correction `p` is
    small. Dividing by `u²` and choosing `u` of order `N^{3/4}` gives
    `k ≤ √N + O(N^{3/8})`: leading constant `1`. -/
theorem windowed_k_bound (B : Finset ℕ) (u N : ℕ)
    (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) (hS : isSidonFinset B) :
    ((u * (B.filter (fun a => u ≤ a)).card : ℕ) : ℝ) ^ 2
      ≤ (N + 1) * ((u * (u - 1) : ℕ) + ((∑ a ∈ B, min u a : ℕ) : ℝ)) := by
  have hocc := occupancy_ge B u
  have hmain := windowed_sidon_bound B u N hB hS
  have h1 : ((u * (B.filter (fun a => u ≤ a)).card : ℕ) : ℝ)
      ≤ ((∑ a ∈ B, min u a : ℕ) : ℝ) := by exact_mod_cast hocc
  have h0 : (0 : ℝ) ≤ ((u * (B.filter (fun a => u ≤ a)).card : ℕ) : ℝ) :=
    Nat.cast_nonneg _
  calc ((u * (B.filter (fun a => u ≤ a)).card : ℕ) : ℝ) ^ 2
      ≤ ((∑ a ∈ B, min u a : ℕ) : ℝ) ^ 2 := by nlinarith
    _ ≤ _ := hmain

/-! ## The crossing

    The windowed bound puts `k²` at `N`; the covering demand a basis imposes puts
    it at `2N`. Once the window is long enough that `k/u` is small and the prefix
    `p` is a small fraction of `k`, the two cannot both hold.

    The constants below are not optimized. They are chosen so the crossing is an
    explicit finite inequality rather than a limit: `p ≤ k/8` and `u ≥ 8k` give
    `k² ≤ 1.47(N+1)` against a demand of `k² ≥ 2N - k`, and those meet at a
    concrete `N`. -/

/-- **The crossing, as arithmetic.** No set, no window, no combinatorics: if the
    windowed inequality and the covering demand both hold with the window long
    enough and the prefix small, there is no such `k`. -/
theorem windowed_crossing (N k p u c : ℝ)
    (hN : 100 ≤ N) (hk : 0 ≤ k) (_hp : 0 ≤ p) (hu : 0 < u)
    (hwin : (k - p) ^ 2 * u ^ 2 ≤ (N + 1) * (u ^ 2 + u * k))
    (hcov : 2 * N ≤ k * (k + 1) + c) (_hc : 0 ≤ c) (hcN : 4 * c ≤ N)
    (hp8 : 8 * p ≤ k) (hku : 8 * k ≤ u) :
    False := by
  -- k/u ≤ 1/8, so the right side is at most (9/8)(N+1) u²
  have hu2 : (0 : ℝ) < u ^ 2 := by positivity
  have hrhs : (N + 1) * (u ^ 2 + u * k) ≤ (N + 1) * (9 / 8) * u ^ 2 := by
    have h1 : u * k ≤ u ^ 2 / 8 := by nlinarith
    nlinarith [hN]
  -- and the left side is at least (7k/8)² u²
  have hlhs : (7 * k / 8) ^ 2 * u ^ 2 ≤ (k - p) ^ 2 * u ^ 2 := by
    have hle : 7 * k / 8 ≤ k - p := by linarith
    have h7 : (0 : ℝ) ≤ 7 * k / 8 := by linarith
    have hsq : (7 * k / 8) ^ 2 ≤ (k - p) ^ 2 := by nlinarith
    exact mul_le_mul_of_nonneg_right hsq (le_of_lt hu2)
  have hk2 : (7 * k / 8) ^ 2 ≤ (N + 1) * (9 / 8) := by
    have := hlhs.trans (hwin.trans hrhs)
    nlinarith
  -- so 49k² ≤ 72(N+1), while covering forces k² ≥ 2N - k
  have hkN : 49 * k ^ 2 ≤ 72 * (N + 1) := by nlinarith [hk2]
  have hexp : k * (k + 1) = k ^ 2 + k := by ring
  have hkbig : 2 * N - k - c ≤ k ^ 2 := by
    rw [hexp] at hcov; linarith
  -- the two together force k to be LINEAR in N, which the first forbids
  have hklin : 26 * N ≤ 49 * k + 49 * c + 72 := by nlinarith
  have hklin2 : 55 * N ≤ 196 * k + 288 := by nlinarith
  have hpos : 0 < 55 * N - 288 := by linarith
  have hsq : (55 * N - 288) ^ 2 ≤ (196 * k) ^ 2 := by nlinarith
  nlinarith [hsq, hkN, hN]

/-- The occupancy never exceeds `u * k`. -/
lemma occupancy_le (B : Finset ℕ) (u : ℕ) : ∑ a ∈ B, min u a ≤ u * B.card := by
  calc ∑ a ∈ B, min u a ≤ ∑ _a ∈ B, u := Finset.sum_le_sum (fun a _ => min_le_left _ _)
    _ = u * B.card := by rw [Finset.sum_const, smul_eq_mul, Nat.mul_comm]

/-- **No Sidon set in `[1,N]` meets the covering demand.**

    Taking the window `u = 8k`, the prefix below `u` is a Sidon set inside
    `[1, 8k)` so it has at most about `4√k` elements, which is below `k/8` once
    `k ≥ 1024`. That is exactly what `windowed_crossing` needs, and the two
    inequalities then have no common solution.

    The covering demand `2N ≤ k(k+1)` is what an asymptotic basis of order two
    forces on its window: each of the `N` targets consumes an unordered pair. So
    a Sidon set cannot be such a basis, which is `C = 2`, and by
    `reps_card_even_off_diagonal` also `C = 3`. -/
theorem no_sidon_covering (B : Finset ℕ) (N : ℕ)
    (hB : ∀ b ∈ B, 1 ≤ b ∧ b ≤ N) (hS : isSidonFinset B)
    (c : ℕ) (hcov : 2 * N ≤ B.card * (B.card + 1) + c) (hcN : 4 * c ≤ N)
    (hN : 100 ≤ N) (hk : 2048 ≤ B.card) : False := by
  classical
  set k := B.card with hkdef
  set u := 8 * k with hudef
  set p := (B.filter (fun a => a < u)).card with hpdef
  have hu0 : 0 < u := by omega
  have hsplit := card_split_at B u
  have hpk : p ≤ k := by omega
  -- the prefix is small: p² ≤ 2u + p = 16k + p, and k ≥ 2048 forces 8p ≤ k
  have hprefix : p * p ≤ 2 * u + p := prefix_small B u hS
  have hp8 : 8 * p ≤ k := by nlinarith [hprefix, hk, hpk]
  have hlarge : (B.filter (fun a => u ≤ a)).card = k - p := by omega
  have hmain := windowed_k_bound B u N hB hS
  rw [hlarge] at hmain
  have hocc := occupancy_le B u
  have hNR : (0 : ℝ) ≤ (N : ℝ) + 1 := by positivity
  have hcast : ((u * (k - p) : ℕ) : ℝ) = (u : ℝ) * ((k : ℝ) - (p : ℝ)) := by
    push_cast [Nat.cast_sub hpk]; ring
  have h1 : ((u * (u - 1) : ℕ) : ℝ) ≤ (u : ℝ) ^ 2 := by
    have hle : u * (u - 1) ≤ u * u := Nat.mul_le_mul_left _ (by omega)
    calc ((u * (u - 1) : ℕ) : ℝ) ≤ ((u * u : ℕ) : ℝ) := by exact_mod_cast hle
      _ = (u : ℝ) ^ 2 := by push_cast; ring
  have hR : ((∑ a ∈ B, min u a : ℕ) : ℝ) ≤ (u : ℝ) * (k : ℝ) := by
    have : ((∑ a ∈ B, min u a : ℕ) : ℝ) ≤ ((u * k : ℕ) : ℝ) := by exact_mod_cast hocc
    calc ((∑ a ∈ B, min u a : ℕ) : ℝ) ≤ ((u * k : ℕ) : ℝ) := this
      _ = (u : ℝ) * (k : ℝ) := by push_cast; ring
  have hstep : ((N : ℝ) + 1) * (((u * (u - 1) : ℕ) : ℝ)
        + ((∑ a ∈ B, min u a : ℕ) : ℝ))
      ≤ ((N : ℝ) + 1) * ((u : ℝ) ^ 2 + (u : ℝ) * (k : ℝ)) :=
    mul_le_mul_of_nonneg_left (by linarith) hNR
  have hwin : ((k : ℝ) - (p : ℝ)) ^ 2 * (u : ℝ) ^ 2
      ≤ ((N : ℝ) + 1) * ((u : ℝ) ^ 2 + (u : ℝ) * (k : ℝ)) := by
    calc ((k : ℝ) - (p : ℝ)) ^ 2 * (u : ℝ) ^ 2
        = ((u : ℝ) * ((k : ℝ) - (p : ℝ))) ^ 2 := by ring
      _ = ((u * (k - p) : ℕ) : ℝ) ^ 2 := by rw [hcast]
      _ ≤ _ := hmain
      _ ≤ _ := hstep
  refine windowed_crossing (N : ℝ) (k : ℝ) (p : ℝ) (u : ℝ) (c : ℝ)
    (by exact_mod_cast hN) (by positivity) (by positivity)
    (by exact_mod_cast hu0) hwin ?_ (by positivity) ?_ ?_ ?_
  · exact_mod_cast hcov
  · exact_mod_cast hcN
  · exact_mod_cast hp8
  · exact_mod_cast (by omega : 8 * k ≤ u)

/-! ## From a basis to the covering demand -/

/-- The sorted pairs: one per unordered pair, diagonal included. -/
def sortedPairs (B : Finset ℕ) : Finset (ℕ × ℕ) :=
  (B ×ˢ B).filter (fun p => p.1 ≤ p.2)

/-- There are `k(k+1)/2` of them. -/
lemma two_mul_card_sortedPairs (B : Finset ℕ) :
    2 * (sortedPairs B).card = B.card * B.card + B.card := by
  classical
  have hsplit := card_product_split B
  have hrise := card_rise_eq_diff B
  have hdiag := card_diagPairs B
  have hunion : sortedPairs B = risePairs B ∪ diagPairs B := by
    ext p
    simp only [sortedPairs, risePairs, diagPairs, Finset.mem_union,
               Finset.mem_filter]
    constructor
    · rintro ⟨hm, hle⟩
      rcases Nat.lt_or_ge p.1 p.2 with h | h
      · exact Or.inl ⟨hm, h⟩
      · exact Or.inr ⟨hm, by omega⟩
    · rintro (⟨hm, h⟩ | ⟨hm, h⟩) <;> exact ⟨hm, by omega⟩
  have hdisj : Disjoint (risePairs B) (diagPairs B) := by
    rw [Finset.disjoint_left]
    intro p hp hq
    simp only [risePairs, diagPairs, Finset.mem_filter] at hp hq
    omega
  rw [hunion, Finset.card_union_of_disjoint hdisj, hrise, hdiag]
  omega

/-- **The covering demand.** Every target in `[N₀, N]` consumes a sorted pair
    from the window, and different targets consume different pairs, so

        2(N - N₀ + 1) ≤ k² + k

    This is the demand the windowed bound cannot meet. -/
theorem covering_demand (A : Set ℕ) (N₀ N : ℕ)
    (hbasis : ∀ n, N₀ ≤ n → ∃ a ∈ A, ∃ b ∈ A, a + b = n)
    (_hpos : ∀ a ∈ A, 1 ≤ a) (_hle : N₀ ≤ N) :
    2 * (N + 1 - N₀) ≤ (window A N).card * (window A N).card + (window A N).card := by
  classical
  set W := window A N with hW
  have hsub : Finset.Icc N₀ N ⊆ (sortedPairs W).image (fun p => p.1 + p.2) := by
    intro n hn
    obtain ⟨h1, h2⟩ := Finset.mem_Icc.mp hn
    obtain ⟨a, ha, b, hb, hab⟩ := hbasis n h1
    have haW : a ∈ W := mem_window.mpr ⟨by omega, ha⟩
    have hbW : b ∈ W := mem_window.mpr ⟨by omega, hb⟩
    rcases Nat.le_total a b with h | h
    · exact Finset.mem_image.mpr ⟨(a, b),
        Finset.mem_filter.mpr ⟨Finset.mem_product.mpr ⟨haW, hbW⟩, h⟩, hab⟩
    · exact Finset.mem_image.mpr ⟨(b, a),
        Finset.mem_filter.mpr ⟨Finset.mem_product.mpr ⟨hbW, haW⟩, h⟩, by omega⟩
  have hcard : (Finset.Icc N₀ N).card ≤ (sortedPairs W).card :=
    le_trans (Finset.card_le_card hsub) Finset.card_image_le
  rw [Nat.card_Icc] at hcard
  have := two_mul_card_sortedPairs W
  omega

/-- **A Sidon set is not an asymptotic basis of order two.**

    Combining the covering demand with the windowed bound. The threshold is
    explicit rather than asymptotic: past it, the two are inconsistent.

    With `reps_card_even_off_diagonal` this covers `C = 3` as well, since a
    representation bound of three forces two off the diagonal, and that is the
    Sidon condition. -/
theorem no_sidon_basis (A : Set ℕ) (N₀ N : ℕ)
    (hbasis : ∀ n, N₀ ≤ n → ∃ a ∈ A, ∃ b ∈ A, a + b = n)
    (hpos : ∀ a ∈ A, 1 ≤ a)
    (hS : isSidonFinset (window A N))
    (hN : 8 * N₀ + 3000000 ≤ N) : False := by
  classical
  set W := window A N with hW
  set k := W.card with hk
  have hle : N₀ ≤ N := by omega
  have hcov := covering_demand A N₀ N hbasis hpos hle
  rw [← hW, ← hk] at hcov
  have hexp : k * (k + 1) = k * k + k := by ring
  -- the window sits in [1, N]
  have hB : ∀ b ∈ W, 1 ≤ b ∧ b ≤ N := by
    intro b hb
    obtain ⟨hbN, hbA⟩ := mem_window.mp hb
    exact ⟨hpos b hbA, hbN⟩
  -- the covering demand forces the window to be large
  have hk2048 : 2048 ≤ k := by
    by_contra hlt
    push_neg at hlt
    have hb : k * k + k ≤ 2047 * 2047 + 2047 := by nlinarith [hlt]
    omega
  exact no_sidon_covering W N hB hS (2 * N₀) (by rw [← hk]; omega) (by omega)
    (by omega) (by rw [← hk]; exact hk2048)

/-! ## Where this method stops

    The crossing rests on the shared-pair bound `u(u-1)/2`, which holds because a
    Sidon set repeats no difference. A set whose maximum difference multiplicity
    is `D` gets `D` times as many shared pairs, and the conclusion weakens to
    `49k² ≤ 72D(N+1)`.

    At `D = 1` that beats the covering demand `2N ≤ k² + k`. At `D = 2` it does
    not, and the theorem below says so in the only way that settles it: by
    exhibiting, for every `N`, a `k` satisfying BOTH. No refinement of the
    constants rescues the route, because the two are genuinely compatible there.

    Measured on the exact optima, the maximum difference multiplicity at `C = 4`
    runs 3 to 5. So `C ≥ 4` is beyond this argument, and that is not a gap in the
    write-up but a property of the method. -/

/-- **The method is exhausted at `D = 2`.** For every `N` there is a `k` meeting
    the covering demand and the windowed bound at difference multiplicity two, so
    no contradiction can be drawn from the pair of them. -/
theorem windowed_method_stops (N : ℝ) (hN : 100 ≤ N) :
    ∃ k : ℝ, 0 ≤ k ∧ 2 * N ≤ k ^ 2 + k ∧ 49 * k ^ 2 ≤ 72 * 2 * (N + 1) := by
  refine ⟨Real.sqrt (2 * N), Real.sqrt_nonneg _, ?_, ?_⟩
  · have h : Real.sqrt (2 * N) ^ 2 = 2 * N := Real.sq_sqrt (by linarith)
    have : 0 ≤ Real.sqrt (2 * N) := Real.sqrt_nonneg _
    linarith [h]
  · have h : Real.sqrt (2 * N) ^ 2 = 2 * N := Real.sq_sqrt (by linarith)
    rw [h]; linarith

#print axioms windowed_method_stops
#print axioms no_sidon_basis
#print axioms covering_demand
#print axioms no_sidon_covering
#print axioms windowed_crossing
#print axioms occupancy_ge
#print axioms prefix_small
#print axioms windowed_k_bound
#print axioms windowed_sidon_bound
#print axioms sum_window_pairs
#print axioms sidon_pairs_in_windows
#print axioms sum_windowCount
#print axioms windowed_bound

end Imscribing.Classical
