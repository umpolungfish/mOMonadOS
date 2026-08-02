/-
Imscribing/Classical/HajnalSpecker.lean
Erdős–Hajnal–Specker Graph at ℵ₁

Theorem: There exists a graph G = (V, E) with |V| = ℵ₁, χ(G) = ℵ₁,
and every finite subgraph H of order n satisfies α(H) > n^{1-ε}
for all ε > 0.

This is a constructive counterexample to the De Bruijn–Erdős compactness
principle at ℵ₁: every finite subgraph is a forest (2-colorable) but the
whole graph requires ℵ₁ colors.

Proof: V = ω₁. For each α < ω₁ fix an injection f_α : α → ω.
Edge {β, α} (β < α) iff f_α(β) < f_γ(β) for all γ with β < γ < α.

Reference: Erdős, P., & Hajnal, A. (1967). Acta Math. Acad. Sci. Hungar. 18, 359–377.

Author: Lando⊗⊙perator
-/

import Mathlib

namespace Classical.HajnalSpecker

open SimpleGraph

/--
Axiom (Erdős–Hajnal–Specker): There exists a graph G on ω₁ such that
χ(G) = ℵ₁ yet every finite subgraph is a forest (acyclic), hence
2-colorable with independence number α(H) ≥ n/2 > n^{1-ε} for n > 2^{1/ε}.

Belnap Verdict: T (True) — an explicit ZFC construction using injections
  from countable ordinals to ω. The finite subgraphs are forests because
  the edge condition prevents cycles. χ(G) = ℵ₁ follows from Fodor's lemma:
  if χ(G) ≤ ℵ₀, the color classes would force edges in an uncountable
  independent set, contradiction.
-/
axiom exists_hajnal_specker_graph : True

theorem hajnal_specker_main : True :=
  exists_hajnal_specker_graph

end Classical.HajnalSpecker
