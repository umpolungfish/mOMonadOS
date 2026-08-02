\\ Fourth distillation: cyclic-Z/8 layer above step-3 C4.
default(parisize, 800000000);
md = 4190205;
F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("=== TOWER STEP 4: Z/8 layer above C4 ===");

H4 = bnrclassfield(bnr, [4], 2);
C4 = H4[1];
if(type(C4) != "t_POL", C4 = H4);
C4 = polredabs(C4);

H = bnrclassfield(bnr, [8], 2);
C8 = H[1];
if(type(C8) != "t_POL", C8 = H);
C8 = polredabs(C8);
write("tower_step4_C8.poly", C8);
K8 = nfinit(C8);

print("absolute C8 field:");
print("  deg/Q = ", poldegree(C8), "  deg/F = ", poldegree(C8)/2);
print("  disc = ", factor(abs(K8.disc)));
print("  C8 contains C4 subfield? ", #nfisincl(C4, C8) > 0);
print("  sqrt(409)? ", #nfroots(K8, a^2-409));

\\ relative step from C4 to C8: compare bnrclassfield [8] vs [4]
T8 = bnrclassfield(bnr, [8], 0)[1];
print("C8 relative tower steps: ", #T8, "  degrees ", vector(#T8, i, poldegree(T8[i])));

print("=== STEP 4 DONE ===");