md = 4190205; d = 2048;
K = bnfinit(x^2 - md, 1);
print("h(F) = ", K.no, "  => Hilbert class field degree ", K.no, " over F");
\\ narrow (ray) class group at the two infinite places
bnr_inf = bnrinit(K, [1, [1,1]]);
print("narrow class group (cond = oo1 oo2):  cyc = ", bnr_inf.cyc, "  order = ", bnr_inf.no);
\\ SIC ray class field conductor: (d) * oo1 * oo2  (Appleby SIC field)
bnr_d = bnrinit(K, [d, [1,1]]);
print("ray class group (cond = (", d, ") oo1 oo2): cyc = ", bnr_d.cyc, "  order = ", bnr_d.no);
print("  => ray class field degree ", bnr_d.no, " over F, ", bnr_d.no*2, " over Q");
