\\ Third distillation: explicit cyclic-Z/4 layer over F = Q(sqrt m_d).
\\ Redei split m_d = 409 * 10245 (tower_step2_redei.gp).
\\ Route: bnrclassfield(bnr,[4]) -- NOT monolithic quadhilbert.
default(parisize, 800000000);
md = 4190205; d = 2048; D1 = 409; D2 = 10245;
print("=== TOWER STEP 3: explicit Z/4 layer ===");
print("Redei: m_d = ", D1, " * ", D2, "   (409 | d-3 = ", d-3, ")");

\\ --- Redei norm equation 409*x^2 + 10245*y^2 = z^2 ---
\\ trivial: x=2,y=1 -> mu=11881=109^2 (rational, not the field generator)
\\ nontrivial: x=1,y=1 -> mu=10654 (not a square in Q)
print("norm equation solutions:");
print("  trivial: x=2 y=1 z=109  mu=409*4+10245=", 409*4+10245);
print("  nontrivial: x=1 y=1 mu=10654 (nonsquare in Q, kronecker(mu,m_d)=", kronecker(10654, md), ")");

F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("F class group ", bnr.cyc, "  order ", bnr.no);

\\ relative distillation tower (2 then 4 over F)
T = bnrclassfield(bnr, [4], 0)[1];
print("relative tower: ", #T, " steps, degrees ", vector(#T, i, poldegree(T[i])));
print("  step1 (quadratic over F): ", T[1]);
print("  step2 (quartic over step1): ", T[2]);

\\ absolute polynomial (degree 16 over Q = degree 8 over F)
H = bnrclassfield(bnr, [4], 2);
C4 = H[1];
if(type(C4) != "t_POL", C4 = H);
C4 = polredabs(C4);
write("tower_step3_C4.poly", C4);
K4 = nfinit(C4);
print("absolute C4 field:");
print("  deg/Q = ", poldegree(C4), "  deg/F = ", poldegree(C4)/2);
print("  disc = ", factor(abs(K4.disc)), "  (expect m_d^8 => unramified over F)");
print("  sqrt(5)? ", #nfroots(K4, a^2-5));
print("  sqrt(409)? ", #nfroots(K4, a^2-409));
print("  sqrt(2049)? ", #nfroots(K4, a^2-2049));
print("  sqrt(m_d)? ", #nfisincl(y^2 - md, C4) > 0);

\\ genus compositum check
Rgen = polcompositum(polcompositum(x^2-5, x^2-409)[1], x^2-2049)[1];
print("genus field deg/Q = 8; C4 deg/Q = 16; C4 extends genus? ", #nfisincl(Rgen, C4) > 0);

print("=== STEP 3 DONE ===");