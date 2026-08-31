\\ Second distillation: the first cyclic-quartic unramified layer beyond the genus field,
\\ via Redei theory on the prime discriminants of m_d = (d+1)(d-3).
md = 4190205;
pd = [-3, 5, 409, -683];   \\ prime discriminants (product = m_d)
pp = [3, 5, 409, 683];
print("prime discriminants: ", pd, "   product = ", pd[1]*pd[2]*pd[3]*pd[4], " (= m_d ", md, ")");

\\ Redei matrix over F_2: R[i,j] = 0 if (pd[i]/p[j])=+1 else 1  (i!=j); diagonal makes rows sum 0.
R = matrix(4,4, i,j, if(i==j, 0, (1 - kronecker(pd[i], pp[j]))/2));
for(i=1,4, s=0; for(j=1,4, if(j!=i, s+=R[i,j])); R[i,i] = s % 2);
print("\nRedei matrix over F_2:");
for(i=1,4, print("  ", vector(4, j, R[i,j])));

Rm = R * Mod(1,2);
rk = 4 - #matker(Rm);
print("\nrank_F2(R) = ", rk, "   4-rank of class group = t-1-rank = ", 4-1-rk, "  (expect 1, matches [32,2])");

ker = matker(Rm);
print("\nkernel vectors (each = a subset S -> factorization m_d = D1*D2 giving a cyclic-quartic layer):");
for(c=1, #ker, v = lift(ker[,c]); S=[]; D1=1; for(i=1,4, if(v[i]==1, S=concat(S,pd[i]); D1*=pd[i])); print("  ker vec ", v~, " -> D1 = ", D1, "  D2 = ", md/D1, "   (S = ", S, ")"));
