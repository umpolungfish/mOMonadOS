# BTC Key Deriver - FDE Lattice Private Key Derivation

## Overview

The BTC Key Deriver module implements private key derivation using the FDE
(First-Depth Entanglement) lattice structure discovered in btclattice.txt.

## FDE Lattice Structure

The SIXTEEN_3 lattice = {T,F} x {t,f} as tensor product:

- Truth layer: {T, F} - classical truth values
- Information layer: {t, f} - informational acceptance/rejection
- Combined: 16 possible states

## Derivation Chain

  Seed(A = TFtf) -> Privkey(T, verdict B) -> Pubkey(T, verdict F)

### Born Rule

Formula: |psi intersect {T}| / |psi| (cardinality ratio in FDE native form)

### State Probabilities

| State     | p_T | p_F | tau | Verdict |
|-----------|-----|-----|-----|---------|
| Seed(A)   | 1/4 | 1/4 | 1/2 | F       |
| Privkey(T)| 1   | 0   | 1   | B       |
| Pubkey(T) | 1   | 0   | 1   | F       |

## Canonical Words

### Seed Word (64 glyphs)
⊞∋∋⊙⊣∈⊣⊥⊤⊞⊡≺≻⊤⊙⊞≻⊞≻⊙⊤⊙⊢≺∈⋈⋈≺≺≻∋⊞⊥⊙≻⊣⊥⊢∋⊥⊡∋⋈⊙⊡⊞⊙⊢⊞∈⊙∋⊙∈≻≻⊥⊞⋈⋈⊢⊞⊣⊡

Final register: A (TFtf)
Phase-bearing: 4 distinct landings (A, Ftf, tf, T)

### Privkey Word (32 glyphs)
⊞∈≻⋈⊢⊢⊢∈≺⊢⊡⊡≻⊙⊙∈⋈⊡⊡⋈≺≻⊞⊡⊣⊥⊡∋⊣⊢⊙⊢

Final register: T (pure truth)
Verdict: B (Both)

### Pubkey Word (33 glyphs)
≺≻⊞≺⊡⊙⊥⊥⋈≺≻⊤⊥⊢∋⊞≺∋⊢⊤⊡≺⋈≻≺⊙⊤⊡⊡≻⋈⊡⋈

Final register: T (pure truth)
Verdict: F (False)

## Key Insight

The seed's arbitrary-complex state (A=TFtf) collapses to pure truth (T)
through key derivation. This is the Born rule in action.

The public key carries the private key's hidden structure in its verdict F
- the verdict flip preserves the truth layer while changing the informational
structure.

## Verification

The canonical IMASM word ⊢∈≻⊤⋈⊙≺⊥⊞∋⊡⊣ produces:

- Final register: A (TFtf)
- Phase-bearing: 4 distinct landings
- mu-circ-delta=id verified

This confirms the seed state is correctly represented in the IMASM lattice.

## Module Usage

  use momonados::btc_key_deriver::{BtcKeyDeriver, LatticeState};

  // Verify the derivation chain
  let reports = BtcKeyDeriver::verify();

  // Check Born rule for seed
  let seed = LatticeState::A;
  assert_eq!(seed.born_rule_p_T(), 0.25);
  assert_eq!(seed.born_rule_p_F(), 0.25);

## References

- btclattice.txt: FDE lattice analysis of Bitcoin keypairs
- seedkeypair2.txt: Vox-CE audit of 100 BIP39/BIP32/BIP44 keypairs
