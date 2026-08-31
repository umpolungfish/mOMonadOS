\\ First distillation up the d=2048 moduli tower: the genus field of F = Q(sqrt m_d).
\\ m_d = (d+1)(d-3) = 3*5*409*683. Genus theory: the first unramified layer is generated
\\ by the prime-discriminant square roots. We want the REAL layer (moduli are real).
default(parisize, 800000000);
md = 4190205; d = 2048;
print("m_d factor: ", factor(md), "   = (d+1)(d-3) = ", (d+1)*(d-3));
\\ prime discriminants p* = (-1)^((p-1)/2) p
ps = [3,5,409,683];
print("prime discriminants:");
pdprod = 1;
for(i=1,4, p=ps[i]; ps_star = if((p%4)==1, p, -p); pdprod *= ps_star; print("  ", p, "* = ", ps_star, "   (p mod 4 = ", p%4, ")"));
print("product of p* = ", pdprod, "   (should equal m_d = ", md, ")");

F = bnfinit(y^2 - md, 1);
print("");
print("class group of F = ", F.clgp.cyc, "  h = ", F.no, " = 2^6");

\\ the REAL genus layer: sqrt of the real prime-power combos tied to d+1 and d-3
\\ d+1 = 2049 = 3*683,  d-3 = 2045 = 5*409
print("");
print("d+1 = ", d+1, " = 3*683   |   d-3 = ", d-3, " = 5*409");
print("norm(g4) = d+1 = ", d+1, "  norm(g3) = -(d-3) = ", -(d-3));
\\ real multiquadratic containing sqrt(m_d): Q(sqrt5, sqrt409, sqrt2049)
R = polcompositum(polcompositum(x^2-5, x^2-409)[1], x^2-2049)[1];
KR = nfinit(R);
print("real genus field Q(sqrt5,sqrt409,sqrt2049): deg over Q = ", poldegree(R), "  deg over F = ", poldegree(R)/2);
print("  contains sqrt(m_d)? ", #nfisincl(y^2-md, R) > 0);
print("  disc factorization: ", factor(abs(KR.disc)));
