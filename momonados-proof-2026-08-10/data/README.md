# d=2048 moduli field — the data behind the claims

Everything here backs a specific statement in
`ig-docs/manuscripts3/sic_moduli_conductor.tex`. The working repository is
`d12_sic_build` (161 MB, its own remote); this is the subset a reader needs to
check the manuscript, at 888 KB.

## Claim → file

| claim | where it is made | file |
|---|---|---|
| the degree-1024 ramified polynomial at conductor 16 | §Dimension 2048 | `tower_ramified_4.poly` |
| its Newton polygon: vertices (0,56), (128,16), (256,0), (1024,0) | §Dimension 2048 | `np_vals.txt` |
| the unramified ascent C4 → C16 → C32 | §The rule | `tower_C4.poly`, `tower_C16.poly`, `tower_C32.poly` |
| the ascent, step by step, as run | §How each field was identified | `tower_step1_genus.gp` … `tower_step9_ramified_L1.gp` |
| the d=12 S-unit exponent vector, 13 entries | §Dimension 2048 | `pin_sunit.txt`, `pin_sunit.gp` |
| ray class group order and structure at 2048 | §The class group | `ray_class_2048.txt` |
| moduli field degrees across the ladder | §The three calibration points | `moduli_degrees.txt` |
| which conductor convention is in force | §The rule | `conductor_convention.txt` |
| the ray tower driver | §Dimension 2048 | `d2048_raytower.gp` |

## Reproducing the Newton polygon

`np_vals.txt` is two columns, coefficient index and its 2-adic valuation, for
all 1025 coefficients. The polygon is the lower convex hull of those points.
Taking it gives vertices (0,56), (128,16), (256,0) and then a flat run to
(1024,0), so the slopes are 5/16 and 1/8 at multiplicity 128 each, and 0 at 768.
That is the ramification structure of 2: two ramified primes with (e,f) = (16,8)
and (8,16), and 768 unramified.

The valuations were taken from `tower_ramified_4.poly` directly. Computing the
hull needs no algebra system — the file is the input and the hull is a scan.

## What this data does not settle

The polygon fixes e and f for each prime above 2, and with the constant term's
valuation of 56 it gives the norm constraint 8e₁ + 16e₂ = 56, hence e₁ + 2e₂ = 7.
Four integer vectors satisfy that: (1,3), (3,2), (5,1), (7,0). The polygon does
not choose among them, and nothing in this directory does either. The selection
of (3,2) is a Grammar reading, recorded in
`ig-docs/sunit_exponent_extraction_d2048.md`.

One caution, since the obvious check is the wrong one. The uniformizers π₁ and
π₂ live in the ray class field, not in F = Q(√4190205), where 2 is inert and
there is a single prime above it. An S-unit computation over F therefore cannot
see them: it returns the fundamental unit's exponent and zeros. That does
confirm e₀ = −1 independently, and it says nothing about e₁ or e₂.

## The fundamental unit, extracted without any of this

ε = (2047 + √4190205)/2 is reachable from cyclotomic sums alone. The
discriminant factors as 4190205 = 3 · 5 · 409 · 683, and the quadratic Gauss sum
g(p) = Σ (k|p) ζ_p^k is √p for p ≡ 1 mod 4 and i√p for p ≡ 3 mod 4. Both primes
congruent to 3 contribute a factor of i, so the product of the four is real and
negative with magnitude √4190205 = 2046.999023, giving

    ε = (2047 + 2046.999023)/2 = 2046.9995114801

against the recorded 2046.9995114801, to ten digits. No ray class field, no
conductor tower, no `bnrstark`.
