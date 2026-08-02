\\ Eighth distillation: probe ramified ray class field at conductor (2048)*oo1*oo2.
\\ Hilbert (unramified) = deg 64/F done. Ray order = 2^27/F => ramified quotient 2^21/F.
default(parisize, 4000000000);
md = 4190205; d = 2048;
F = bnfinit(y^2 - md, 1);
bnr_h = bnrinit(F, 1);
bnr_r = bnrinit(F, [d, [1, 1]]);
print("=== TOWER STEP 8: ramified probe (2048)*oo ===");
print("Hilbert class group: ", bnr_h.cyc, " order ", bnr_h.no);
print("Ray (2048)*oo:       ", bnr_r.cyc, " order ", bnr_r.no);
print("ramified index over Hilbert: ", bnr_r.no / bnr_h.no, " (= 2^", log(bnr_r.no/bnr_h.no)/log(2), ")");
print("");
print("pro-2 tower factors in ray cyc: 4096=2^12, 512=2^9, 8=2^3, 4=2^2, 2=2^1  (sum=27)");
print("Hilbert consumed 6 of 27 binary steps; remaining ramified: 21");
print("");
\\ try first small quotient: kill the 2-factor (index 2^26) => degree-2 over ray? 
\\ simpler: quotient by order-2^26 subgroup => cyclic of order 2
t0 = getwalltime();
H2 = bnrclassfield(bnr_r, [2], 2);
print("bnrclassfield(ray, [2], 2) ms=", getwalltime()-t0);
C = H2[1]; if(type(C) != "t_POL", C = H2);
print("first ramified layer [2]: deg/Q=", poldegree(C), " deg/F=", poldegree(C)/2);
print("=== STEP 8 PROBE DONE ===");