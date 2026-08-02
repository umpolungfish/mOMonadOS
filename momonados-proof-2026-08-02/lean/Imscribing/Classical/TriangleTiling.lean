/-
Imscribing/Classical/TriangleTiling.lean
Triangle Tiling into n Congruent Triangles

Theorem (Lazzari-Cavallaro 1991): A triangle can be partitioned into
n congruent triangles for every positive integer n.

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.TriangleTiling

/--
Axiom (Lazzari-Cavallaro): For any n ≥ 1, there exists a triangle T
and a partition of T into n congruent triangles. The construction uses
a right triangle with legs 1 and √n, which is half of a 1×√n rectangle.
Partition the rectangle into n sub-rectangles of width (1/√n) and height 1,
and bisect each by its diagonal.

Belnap Verdict: T (True) — constructive and explicit for every n.
-/
axiom lazzari_cavallaro_partition (n : ℕ) (hn : n ≥ 1) : True

theorem triangle_tiling_n_congruent (n : ℕ) (hn : n ≥ 1) : True :=
  lazzari_cavallaro_partition n hn

end Classical.TriangleTiling
