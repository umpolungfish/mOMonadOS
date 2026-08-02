\\ Map ramified ray-class tower quotients (fixed PARI syntax).
default(parisize, 4000000000);
md = 4190205; d = 2048;
F = bnfinit(y^2 - md, 1);
bnr_h = bnrinit(F, 1);
bnr_r = bnrinit(F, [d, [1, 1]]);
cyc = bnr_r.cyc; n = #cyc;
print("=== RAMIFIED TOWER MAP ===");
print("Hilbert=", bnr_h.no, " Ray=", bnr_r.no, " index=2^21");
print("Ray cyc=", cyc);
quo(i, half)={
  my(v = vector(n, j, if(j==i, cyc[i]/half, 1)));
  my(H = bnrclassfield(bnr_r, v, 2), C = H[1]);
  if(type(C) != "t_POL", C = H);
  print("  factor[", i, "] kill /", half, ": deg/Q=", poldegree(C))
};
for(i=1,5, quo(i,2));
for(i=1,5, quo(i,4));
print("full ray field (test [2] cyclic):");
H2 = bnrclassfield(bnr_r, [2], 2)[1];
if(type(H2) != "t_POL", H2 = H2);
print("  bnrclassfield(ray,[2]): deg/Q=", poldegree(H2));
print("=== DONE ===");