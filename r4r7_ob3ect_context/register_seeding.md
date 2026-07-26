# Register seeding in the MiniKernel, after the correction

```
R0 = tuple_to_b4(D, Phi)      Dimension x Criticality
R1 = tuple_to_b4(T, Omega)    Topology x Winding
R2 = tuple_to_b4(K, f)        Kinetics x Fidelity
R3 = tuple_to_b4(H, P)        Chirality x Parity
R4 = tuple_to_b4(R, S)        Adjoint x One-to-one   (recovered slots)
R5 = tuple_to_b4(G, C)        Maximal x C            (recovered slots)
R6 = N                        written by IMSCRIB
R7 = N                        written by IMSCRIB
```

`tuple_to_b4(a, b)` sums the two ordinals modulo four and reads the result as a
Belnap value: 0 -> N, 1 -> T, 2 -> F, 3 -> B.

On IMSCRIB, mirroring the kernel exactly:

```
R4 = B4::from_u8(token_diversity & 3)
R5 = T if self_ref else F
R6 = T if frobenius_order > 0 else F
R7 = T if dialetheia_complete else F
```

Note the collision: R4 and R5 carry recovered tuple slots until IMSCRIB
overwrites them with witnesses. Whether the seeding and the witnesses should
occupy the same four registers, or whether the recovered slots belong elsewhere,
is part of what the two statements decide.
