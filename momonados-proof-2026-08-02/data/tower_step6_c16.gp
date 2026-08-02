\\ Sixth distillation: C16 layer (deg 64/Q = 32/F). Skip polredabs — it hangs on deg 64.
default(parisize, 4000000000);
md = 4190205; n = 16;
F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("=== TOWER STEP 6: C16 layer ===");
print("class ", bnr.cyc);
t0 = getwalltime();
H = bnrclassfield(bnr, [n], 2);
print("bnrclassfield ms=", getwalltime() - t0);
C = H[1]; if(type(C) != "t_POL", C = H);
print("deg/Q=", poldegree(C), " deg/F=", poldegree(C)/2);
write("tower_C16.poly", C);
print("written tower_C16.poly (raw, no polredabs)");
print("=== STEP 6 DONE ===");