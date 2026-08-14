\\ Seventh distillation: C32 = FULL Hilbert class field (deg 128/Q = 64/F).
\\ Skip polredabs — hangs on high degree. bnrclassfield alone is fast.
default(parisize, 4000000000);
md = 4190205; n = 32;
F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("=== TOWER STEP 7: C32 = Hilbert class field ===");
print("class ", bnr.cyc, " order ", bnr.no);
t0 = getwalltime();
H = bnrclassfield(bnr, [n], 2);
print("bnrclassfield ms=", getwalltime() - t0);
C = H[1]; if(type(C) != "t_POL", C = H);
print("deg/Q=", poldegree(C), " deg/F=", poldegree(C)/2);
write("tower_C32.poly", C);
print("written tower_C32.poly (raw, no polredabs)");
print("=== STEP 7 DONE — HILBERT CLASS FIELD REACHED ===");