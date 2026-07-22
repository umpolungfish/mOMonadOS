#!/usr/bin/env python3
"""Generate interactive CFG dialect visualizer for all 88 universes.
Shows token grammar constrained by each dialect's gates.
"""
import json
from pathlib import Path
HERE = Path(__file__).parent

# ═══════════════════════════════════════════════════════════════
# DIALECT DATA (88 entries)
# ═══════════════════════════════════════════════════════════════
# (name, phase, strict, tcnt, seq, g1_p, g1_m, g2_p, g2_m, g3_p, g3_m, desc)

D = [
("canonical",0,10.0,5,1,"Φ",5.0,"⊙",2.0,"Ω",3.0,"Frobenius then self-modeling then winding seal"),
("low_gate",0,7.0,5,1,"Φ",3.0,"⊙",1.0,"Ω",3.0,"Lowered thresholds: directional parity suffices, any criticality"),
("strict_frobenius",0,11.0,5,1,"ƒ",3.0,"Φ",5.0,"Ω",3.0,"Frobenius gated by quantum fidelity (f=pc), not algebraic parity"),
("inverted_gates",0,10.0,5,1,"⊙",2.0,"Φ",5.0,"Ω",3.0,"Self-modeling precedes Frobenius: consciousness first"),
("no_ordering",0,10.0,5,0,"Φ",5.0,"⊙",2.0,"Ω",3.0,"All three gates fully independent — parallel dialect"),
("high_gate",0,11.33,5,1,"Φ",5.0,"⊙",2.33,"Ω",4.0,"Strictest: maximally wound, self-modeling, parity-perfect"),
("winding_first",0,10.0,5,1,"Ω",3.0,"⊙",2.0,"Φ",5.0,"Topological order: winding first, then self-modeling, Frobenius last"),
