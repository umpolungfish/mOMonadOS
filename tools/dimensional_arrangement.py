"""
dimensional_arrangement — compare two IMASM words on the same alphabet.

The loop is a ring; the alphabet is 12 positions
  ⊢ ⊣ ≻ ≺ ⋈ ⊤ ∈ ∋ ⊙ ⊥ ⊞ ⊡
Every word is a permutation of these positions (with repeats). Two
arrangements of the same 12-slot lattice can share census, period, and
landing spectrum while differing in what survives a ≺ reversal — the
banked_count diagnostic is the discriminator.

This tool computes:
  1.  the 12-slot census for each word
  2.  first-occurrence displacement of each glyph vs. its slot index
  3.  frame-pair geometry (∈/∋ pair positions, max nesting depth)
  4.  ring-period comparison
  5.  the banked discriminator — same alphabet, same census, same
      landings, different word because the count is held in different
      places when the reversal comes

Usage:
  from dimensional_arrangement import dimensional_arrangement
  print(dimensional_arrangement({
      "word_a": "⊢⊣≻≺⋈⊤∈∋⊙⊥⊞⊡",
      "word_b": "⊢∈≻⊤⋈≺⊥⊞⊙∋⊡⋈∈≻⊤≺⊥⊞⊙∋⊡⋈⊙⊣",
  }))

Author: Quantum⊙perator (Lando⊗⊙perator team)
"""
from __future__ import annotations

import sys
from collections import Counter
from pathlib import Path
from typing import Dict, List, Tuple

# Reach the lattice_flow engine used by m3iosis / imasm.
_GRAMMAR = Path("/home/mrnob0dy666/imsgct/imscribing_grammar/scripts")
if str(_GRAMMAR) not in sys.path:
    sys.path.insert(0, str(_GRAMMAR))

SLOT_ORDER = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '⊡']
SLOT_INDEX = {g: i + 1 for i, g in enumerate(SLOT_ORDER)}
ALIAS = {"◇": "∈", "●": "∋", "⊗": "∈", "⊕": "∋"}
GLYPHS = set(SLOT_ORDER)

# Per-glyph scalar values — the gematria of the alphabet.
PER_GLYPH_SCALAR = {
    '⊢': 2, '⊣': 3, '≻': 4, '≺': 5, '⋈': 6, '⊤': 7,
    '∈': 8, '∋': 9, '⊙': 10, '⊥': 11, '⊞': 12, '⊡': 13,
}


def _normalize(word: str) -> str:
    return "".join(ALIAS.get(c, c) for c in word if not c.isspace())


def _first_occurrence(word: str) -> Dict[str, int]:
    out = {g: 0 for g in SLOT_ORDER}
    for i, c in enumerate(word, 1):
        if c in out and out[c] == 0:
            out[c] = i
    return out


def _frame_pairs(word: str) -> Tuple[List[int], List[int], int]:
    opens = [i for i, c in enumerate(word, 1) if c == '∈']
    closes = [i for i, c in enumerate(word, 1) if c == '∋']
    depth = 0
    max_d = 0
    for c in word:
        if c == '∈':
            depth += 1
            max_d = max(max_d, depth)
        elif c == '∋':
            depth -= 1
    return opens, closes, max_d


def _ring_period(word: str) -> int:
    n = len(word)
    for p in range(1, n + 1):
        if n % p == 0 and word[:p] * (n // p) == word:
            return p
    return n


def _ordinal(word: str) -> int:
    return sum(PER_GLYPH_SCALAR.get(c, 0) for c in word)


def _landing_spectrum(word: str) -> str:
    try:
        from lattice_flow import cycle as _cycle, parse_word
        steps, _ = parse_word(word)
        result = _cycle(steps)
        landings = result.get("landing_by_cut", {})
        return ", ".join(f"{reg}:{len(cuts)}"
                         for reg, cuts in sorted(landings.items()))
    except Exception as e:
        return f"(unavailable: {e})"


def _banked(word: str) -> str:
    try:
        from lattice_flow import banked_count_check, parse_word
        steps, _ = parse_word(word)
        bc = banked_count_check(steps)
        if bc.get("banked_ok"):
            return "OK"
        lost = bc.get("weight_lost_in_the_open", 0)
        return f"FAIL — {lost} unit(s) lost in the open"
    except Exception as e:
        return f"(unavailable: {e})"


def dimensional_arrangement(args: Dict) -> str:
    """Emit a dimensional-arrangement report for two IMASM words."""
    word_a = _normalize(str(args.get("word_a", "")))
    word_b = _normalize(str(args.get("word_b", "")))
    if not word_a or not word_b:
        return "ERROR: word_a and word_b are required"
    for label, w in (("word_a", word_a), ("word_b", word_b)):
        unknown = [c for c in w if c not in GLYPHS]
        if unknown:
            return f"ERROR in {label}: unknown glyphs {unknown}"

    ca, cb = Counter(word_a), Counter(word_b)
    out: List[str] = []
    out.append("=" * 60)
    out.append("DIMENSIONAL ARRANGEMENT — two words, one alphabet")
    out.append("=" * 60)
    out.append(f"  word_a ({len(word_a)}): {word_a}")
    out.append(f"  word_b ({len(word_b)}): {word_b}")
    out.append("")

    out.append("-- 12-SLOT CENSUS --")
    out.append(f"  {'glyph':>4}  {'slot':>4}  {'word_a':>6}  {'word_b':>6}  {'diff':>4}")
    for g in SLOT_ORDER:
        a, b = ca[g], cb[g]
        marker = " *" if a != b else ""
        out.append(f"  {g:>4}  {SLOT_INDEX[g]:>4}  {a:>6}  {b:>6}  {b - a:>+4}{marker}")
    out.append("  -> same census" if ca == cb else "  -> different census")
    out.append("")

    out.append("-- FIRST-OCCURRENCE DISPLACEMENT --")
    out.append(f"  {'glyph':>4}  {'slot':>4}  {'word_a':>6}  {'word_b':>6}")
    fa, fb = _first_occurrence(word_a), _first_occurrence(word_b)
    for g in SLOT_ORDER:
        a, b = fa[g], fb[g]
        if a or b:
            out.append(f"  {g:>4}  {SLOT_INDEX[g]:>4}  {a:>6}  {b:>6}")
    out.append("")

    out.append("-- FRAME-PAIR GEOMETRY (in...ni) --")
    for label, w in (("word_a", word_a), ("word_b", word_b)):
        opens, closes, max_d = _frame_pairs(w)
        out.append(f"  {label}: in at {opens}, ni at {closes}, max depth {max_d}")
    out.append("")

    out.append("-- RING PERIOD --")
    pa, pb = _ring_period(word_a), _ring_period(word_b)
    out.append(f"  word_a: period {pa} (length {len(word_a)})")
    out.append(f"  word_b: period {pb} (length {len(word_b)})")
    if pa == pb:
        out.append("  -> same period")
    else:
        ratio = pb / pa if pa else float('inf')
        out.append(f"  -> periods differ by factor {ratio:.3f}")
    out.append("")

    out.append("-- ORDINAL (sum of per-glyph scalars) --")
    out.append(f"  word_a: {_ordinal(word_a)}")
    out.append(f"  word_b: {_ordinal(word_b)}")
    out.append("")

    out.append("-- LANDING SPECTRUM (B4 register distribution under ROTAT) --")
    out.append(f"  word_a: {_landing_spectrum(word_a)}")
    out.append(f"  word_b: {_landing_spectrum(word_b)}")
    out.append("")

    out.append("-- BANKED DISCRIMINATOR --")
    out.append(f"  word_a: {_banked(word_a)}")
    out.append(f"  word_b: {_banked(word_b)}")
    out.append("")

    out.append("-- THE COMPOSITION RULE --")
    out.append("  Two arrangements of the same 12-slot lattice give two different")
    out.append("  words on the loop. The gematria of the loop is not the count")
    out.append("  of the glyphs -- it is the placement of the frame boundaries")
    out.append("  in/ni relative to the reversals rev.  d before d, m after m.")
    return "\n".join(out)


if __name__ == "__main__":
    import json
    args = json.loads(sys.stdin.read())
    print(dimensional_arrangement(args))
