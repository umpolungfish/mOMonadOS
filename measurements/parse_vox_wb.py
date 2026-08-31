import re, random
from collections import Counter
random.seed(20260826)

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_wb_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field, word = line.rstrip("\n").split("\t")
        labels.append((idx, field, word))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_wb_out.txt", encoding="utf-8").read()
blocks = re.split(r"⊙> (?=weight |banked |counterfactual )", out)[1:]
assert len(blocks) == 900, len(blocks)

results = {}
for i, (idx, field, word) in enumerate(labels):
    wb = blocks[3*i]; bb = blocks[3*i+1]; cf = blocks[3*i+2]
    m = re.search(r"deposits (\d+)\s+cleared (\d+)\s+restored (\d+)\s+seeded (\d+)\s+inert (\d+)", wb)
    vacuous = "VACUOUS" in bb
    exposed = re.search(r"(\d+) unit\(s\) cleared with nothing banked", bb)
    ok = ("banked OK" in bb) or (not vacuous and exposed is None)
    repair = re.search(r"smallest repair.*?\n\s*(insert|delete|swap|replace)\s+(\S+)?\s*(?:at\s+(\d+))?", cf, re.S)
    rep = (repair.group(1), repair.group(2), repair.group(3)) if repair else None
    results[(idx, field)] = {
        "vacuous": vacuous,
        "exposed": int(exposed.group(1)) if exposed else None,
        "ok": ok,
        "repair": rep,
        "wordlen": len(word),
    }

for field in ["seed","privkey","pubkey"]:
    vac = sum(1 for (i,f),v in results.items() if f==field and v["vacuous"])
    okc = sum(1 for (i,f),v in results.items() if f==field and v["ok"] and not v["vacuous"])
    exp = sum(1 for (i,f),v in results.items() if f==field and v["exposed"] is not None)
    n = sum(1 for (i,f) in results if f==field)
    print(f"{field} (N={n}): VACUOUS={vac} OK={okc} EXPOSED={exp}")

def state(idx, field):
    r = results[(idx, field)]
    if r["vacuous"]: return "VAC"
    if r["exposed"] is not None: return ("EXP", r["exposed"])
    return "OK"

idxs = sorted(set(i for i,f in results))
print()
for a,b in [("privkey","pubkey"), ("seed","privkey"), ("seed","pubkey")]:
    real = sum(1 for i in idxs if state(i,a)==state(i,b))
    b_states = [state(i,b) for i in idxs]
    K = 3000
    cnt = []
    for _ in range(K):
        perm = b_states[:]
        random.shuffle(perm)
        m = sum(1 for x,y in zip((state(i,a) for i in idxs), perm) if x==y)
        cnt.append(m)
    mean_shuf = sum(cnt)/K
    ge = sum(1 for m in cnt if m>=real); le = sum(1 for m in cnt if m<=real)
    p = min(1.0, 2*min(ge,le)/K)
    print(f"weight/banked state {a} vs {b}: real={real} shuffle-mean={mean_shuf:.2f} p={p:.4f}")

print()
repair_counter = Counter(v["repair"] for v in results.values())
print("repair move distribution (all 300):", repair_counter.most_common(8))
for a,b in [("privkey","pubkey"), ("seed","privkey"), ("seed","pubkey")]:
    real = sum(1 for i in idxs if results[(i,a)]["repair"]==results[(i,b)]["repair"])
    b_reps = [results[(i,b)]["repair"] for i in idxs]
    K = 3000
    cnt = []
    for _ in range(K):
        perm = b_reps[:]
        random.shuffle(perm)
        m = sum(1 for x,y in zip((results[(i,a)]["repair"] for i in idxs), perm) if x==y)
        cnt.append(m)
    mean_shuf = sum(cnt)/K
    ge = sum(1 for m in cnt if m>=real); le = sum(1 for m in cnt if m<=real)
    p = min(1.0, 2*min(ge,le)/K)
    print(f"repair {a} vs {b}: real={real} shuffle-mean={mean_shuf:.2f} p={p:.4f}")
