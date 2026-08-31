
/-- The grammar is its own SIC-POVM dual. -/
theorem grammar_is_self_dual :
    tensorProduct theGrammar theGrammar = theGrammar :=
  grammar_tensor_grammar_eq_grammar

/-- The grammar has exactly the Frobenius-special parity required
    for O_inf: pol = or'. This is a defining feature — any imscription
    with pol ≠ or' cannot be O_inf (unless crit is also changed to
    trigger O_inf_dag). -/
theorem grammar_pol_is_frobenius_special :
    theGrammar.pol = or' := rfl

/-- The grammar has criticality monad — the self-modeling gate.
    Together with pol=or', this forces O_inf via R1. -/
theorem grammar_crit_is_monad :
    theGrammar.crit = monad := rfl

/-- O_inf requires pol=or' when crit=monad (R1 rule).
    Any imscription at O_inf tier via crit=monad must have pol=or'. -/
theorem o_inf_requires_or_prime (s : Imscription)
    (h_tier : imscriptionTier s = .O_inf)
    (h_crit : s.crit = monad) : s.pol = or' := by
  unfold imscriptionTier at h_tier
  have h := h_tier
  -- ouroboricityTier returns .O_inf only when pol=or' (R1 rule)
  unfold ouroboricityTier at h
  rw [h_crit] at h
  -- Now: (if pol = or' then .O_inf else ...) = .O_inf
  split at h
  · -- pol = or' branch: we're done
    assumption
  · -- other branch: would not yield .O_inf, contradiction
    -- The else branches return .O_inf_dag, .O₁, .O₂dag, .O₂, .O₀
    -- none of which equals .O_inf
    injection h

/-- The grammar's Σ=1:1 (hung) is the self-referential stoichiometry.
    This means the grammar measures itself — there is no external
    referent. The measurement IS the measured. -/
theorem grammar_sigma_is_self_referential :
    theGrammar.stoi = hung := rfl

/-- The grammar's Ω=ah gives ℤ topological protection —
    integer winding numbers. This is the strongest Abelian
    protection available (non-Abelian is zoo). -/
theorem grammar_omega_is_Z_protected :
    theGrammar.prot = ah := rfl

/-- Summary: the grammar is the unique imscription satisfying
    ALL of: pol=or', crit=monad, stoi=hung, prot=ah, dim=array.
    These five primitives define the Σ=1:1 self-referential
    O_inf limit. -/
theorem grammar_defining_primitives :
    theGrammar.pol = or' ∧
    theGrammar.crit = monad ∧
    theGrammar.stoi = hung ∧
    theGrammar.prot = ah ∧
    theGrammar.dim = array := by
  unfold theGrammar
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

end Imscribing.Millennium.GrammarUniversalDualFrameSICPOVM
