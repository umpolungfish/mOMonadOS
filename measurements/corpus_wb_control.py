import re, random, json
random.seed(20260826)

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/corpus_wb_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field = line.rstrip("\n").split("\t")
        labels.append((idx, field))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/corpus_wb_out.txt", encoding="utf-8").read()
blocks = re.split(r"⊙> (?=weight |banked )", out)[1:]

results = {}
li = 0
i = 0
while i < len(blocks):
    wb = blocks[i]; bb = blocks[i+1]
    idx, field = labels[li]
    m = re.search(r"deposits (\d+)\s+cleared (\d+)\s+restored (\d+)\s+seeded (\d+)\s+inert (\d+)", wb)
    vacuous = "VACUOUS" in bb
    exposed = re.search(r"(\d+) unit\(s\) cleared with nothing banked", bb)
    results[(idx, field)] = {
        "cleared": int(m.group(2)) if m else None,
        "vacuous": vacuous,
        "exposed_cleared": int(exposed.group(1)) if exposed else None,
    }
    li += 1; i += 2

idxs = sorted(set(i for i,f in results))

def state(idx, field):
    r = results[(idx, field)]
    if r["vacuous"]: return "VAC"
    if r["exposed_cleared"] is not None: return ("EXP", r["exposed_cleared"])
    return "OK"

for a,b in [("privkey","pubkey"), ("seed","privkey"), ("seed","pubkey")]:
    # 1. categorical match (VAC/OK/EXP-as-category, ignoring cleared count)
    def cat(s):
        return s[0] if isinstance(s, tuple) else s
    real_cat = sum(1 for i in idxs if cat(state(i,a))==cat(state(i,b)))

    # 2. exact match including cleared-count for EXPOSED cases
    real_exact = sum(1 for i in idxs if state(i,a)==state(i,b))

    # shuffle control on exact match
    K = 3000
    b_states = [state(i,b) for i in idxs]
    cnt = []
    for _ in range(K):
        perm = b_states[:]
        random.shuffle(perm)
        m = sum(1 for x,y in zip((state(i,a) for i in idxs), perm) if x==y)
        cnt.append(m)
    mean_shuf = sum(cnt)/K
    ge = sum(1 for m in cnt if m>=real_exact); le = sum(1 for m in cnt if m<=real_exact)
    p = min(1.0, 2*min(ge,le)/K)

    print(f"{a} vs {b} (N={len(idxs)}):")
    print(f"  same category (VAC/OK/EXP) : {real_cat}")
    print(f"  exact state match (incl. cleared-count when EXPOSED): real={real_exact}  shuffle-mean={mean_shuf:.2f}  p={p:.4f}")
    print()
