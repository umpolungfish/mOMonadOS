\\ d=2048 moduli tower — single-level bnrclassfield distillation.
\\ Usage: gp -q tower_d2048_pari.gp <level>
\\   level = 4 | 8 | 16 | 32  (cyclic quotient; 32 = full Hilbert class field)
default(parisize, 2000000000);
default(parisizemax, 24000000000);
md = 4190205;
argv = Vecsmall(["4"]);
if(argc >= 2, argv[1] = eval(argv[1]));
n = argv[1];
print("[tower_d2048] level C", n, " over F = Q(sqrt ", md, ")");
F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("class group ", bnr.cyc, " order ", bnr.no);
t0 = getwalltime();
H = bnrclassfield(bnr, [n], 2);
dt = getwalltime() - t0;
C = H[1];
if(type(C) != "t_POL", C = H);
C = polredabs(C);
out = Strprintf("../d12_sic_build/tower_C%d.poly", n);
write(out, C);
print("[done] deg/Q=", poldegree(C), " deg/F=", poldegree(C)/2, " time=", dt, "ms");
print("disc=", factor(abs(nfinit(C).disc)));
if(n >= 8,
  Hp = bnrclassfield(bnr, [n/2], 2)[1];
  if(type(Hp) != "t_POL", Hp = Hp[1]);
  print("contains C", n/2, "? ", #nfisincl(polredabs(Hp), C) > 0)
);