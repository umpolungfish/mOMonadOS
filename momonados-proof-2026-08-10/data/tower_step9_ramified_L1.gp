\\ First ramified distillation above Hilbert: halve the 4096-factor of ray cyc.
default(parisize, 4000000000);
md = 4190205; d = 2048;
F = bnfinit(y^2 - md, 1);
bnr_r = bnrinit(F, [d, [1, 1]]);
cyc = bnr_r.cyc;
print("=== TOWER STEP 9: ramified L1 (kill /2 on 4096-factor) ===");
print("ray cyc=", cyc);
\\ quotient: first factor halved, rest trivial
sub = [cyc[1]/2, 1, 1, 1, 1];
print("subgroup vector=", sub);
t0 = getwalltime();
H = bnrclassfield(bnr_r, sub, 2);
dt = getwalltime() - t0;
print("bnrclassfield ms=", dt);
C = H[1]; if(type(C) != "t_POL", C = H);
print("deg/Q=", poldegree(C), " deg/F=", poldegree(C)/2);
write("tower_ramified_L1.poly", C);
print("written tower_ramified_L1.poly");
print("=== STEP 9 DONE ===");