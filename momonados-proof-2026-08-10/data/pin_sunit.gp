default(realprecision, 400);
system("rm -f pin_sunit.txt");
read("coords_uhi.gp");
W(s) = write("pin_sunit.txt", s);
p = algdep(abs(z1)^2, 8);
L = bnfinit(p, 1);
S = concat(concat(idealprimedec(L,2), idealprimedec(L,3)), idealprimedec(L,13));
su = bnfsunit(L, S);
emb = nfisincl(p, L.pol);
n1 = Mod(emb[1], L.pol);
\\ express N1 in the S-unit basis: exponent vector on [fundamental S-units, then torsion]
ex = bnfissunit(L, su, n1);
W(Strprintf("L.pol (octic) = %s ,  h=%d ,  #fund S-units = %d", L.pol, L.no, #su[1]));
W(Strprintf("N1 as S-unit: exponent vector (on fund S-units + units + torsion):"));
W(Strprintf("   %s", ex~));
\\ square-class: exponents mod 2 => the exact ramifying S-unit sqrt(N1) adjoins
W(Strprintf("N1 square-class (exponents mod 2, the sqrt-N1 ramifier): %s", (lift(ex) % 2)~));
\\ sanity: reconstruct N1 from the S-units and the exponent vector, compare numerically
W("DONE");
