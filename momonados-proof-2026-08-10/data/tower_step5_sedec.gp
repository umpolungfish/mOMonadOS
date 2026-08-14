\\ Fifth distillation: Z/16 layer toward full Hilbert class field (deg 64/F).
default(parisize, 800000000);
md = 4190205;
F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("=== TOWER STEP 5: Z/16 layer ===");
print("target Hilbert class field: deg 64 over F = deg 128 over Q");

v = [16, 32];
for(i = 1, #v, n = v[i]; H = bnrclassfield(bnr, [n], 2); C = H[1]; if(type(C) != "t_POL", C = H); C = polredabs(C); print("C", n, ": deg/Q = ", poldegree(C), "  deg/F = ", poldegree(C)/2); if(n == 16, write("tower_step5_C16.poly", C)); if(n == 32, write("tower_step5_C32.poly", C)));

H4 = polredabs(bnrclassfield(bnr, [4], 2)[1]);
H8 = polredabs(bnrclassfield(bnr, [8], 2)[1]);
H16 = polredabs(bnrclassfield(bnr, [16], 2)[1]);
print("nested: C16 contains C8? ", #nfisincl(H8, H16) > 0);
print("nested: C8 contains C4? ", #nfisincl(H4, H8) > 0);

print("=== STEP 5 DONE ===");