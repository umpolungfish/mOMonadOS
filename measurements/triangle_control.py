import json, random

random.seed(20260826)

data = json.load(open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/triangle_data.json", encoding="utf-8"))
idxs = sorted(data.keys())
n = len(idxs)

def crystal(idx, field):
    return data[idx][field][0]

pairs = [("seed","privkey"), ("seed","pubkey"), ("privkey","pubkey")]

print(f"N={n} rows\n")
for a,b in pairs:
    real = sum(1 for i in idxs if crystal(i,a)==crystal(i,b))
    # per-field marginal distributions for the independence expectation
    from collections import Counter
    ca = Counter(crystal(i,a) for i in idxs)
    cb = Counter(crystal(i,b) for i in idxs)
    expected_indep = sum((ca[c]/n)*(cb[c]/n) for c in set(ca)|set(cb)) * n

    # empirical shuffle control: derange b's assignment across idx, K trials
    K = 2000
    shuf_counts = []
    b_vals = [crystal(i,b) for i in idxs]
    for _ in range(K):
        perm = b_vals[:]
        random.shuffle(perm)
        m = sum(1 for x,y in zip((crystal(i,a) for i in idxs), perm) if x==y)
        shuf_counts.append(m)
    mean_shuf = sum(shuf_counts)/K
    # empirical two-sided p-value: how often shuffled >= real (if real is high) or extreme
    ge = sum(1 for m in shuf_counts if m >= real)
    le = sum(1 for m in shuf_counts if m <= real)
    p_two_sided = 2*min(ge,le)/K
    p_two_sided = min(p_two_sided, 1.0)

    print(f"{a} == {b}:")
    print(f"  real matches            = {real}/{n}")
    print(f"  expected under independence (marginals) = {expected_indep:.2f}")
    print(f"  shuffle-null mean ({K} trials) = {mean_shuf:.2f}")
    print(f"  empirical two-sided p  = {p_two_sided:.4f}")
    print()
