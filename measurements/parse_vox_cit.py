import re, random
from collections import Counter
random.seed(20260826)

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_cit_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field, word = line.rstrip("\n").split("\t")
        labels.append((idx, field, word))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_cit_out.txt", encoding="utf-8").read()
blocks = re.split(r"⊙> (?=cycle |insert |trans )", out)[1:]
assert len(blocks) == 900, len(blocks)

results = {}
for i, (idx, field, word) in enumerate(labels):
    cy = blocks[3*i]; ins = blocks[3*i+1]; tr = blocks[3*i+2]

    landings = re.search(r"landing register by cut:\s*\n((?:\s+\S+\s+at k = [\d, ]+\n)+)", cy)
    n_landings = len(re.findall(r"^\s+\S+\s+at k", cy, re.M))
    invariant = "INVARIANT under ROTAT" in cy
    phase_bearing = "PHASE-BEARING" in cy

    hold_m = re.search(r"(\d+) distinct word\(s\) hold, of (\d+) tried", ins)
    n_hold = int(hold_m.group(1)) if hold_m else None
    n_tried = int(hold_m.group(2)) if hold_m else None

    closing = re.search(r"closing edge\s*:\s*(\S+)\s*->\s*(\S+)", tr)
    close_edge = (closing.group(1), closing.group(2)) if closing else None
    ring_trans = re.search(r"ring transitions\s*:\s*(\d+)", tr)
    n_trans = int(ring_trans.group(1)) if ring_trans else None
    distinct_trans = len(re.findall(r"^\s+\S+ -> \S+\s+\d+", tr, re.M))

    results[(idx, field)] = {
        "n_landings": n_landings, "invariant": invariant, "phase_bearing": phase_bearing,
        "n_hold": n_hold, "n_tried": n_tried,
        "close_edge": close_edge, "n_trans": n_trans, "distinct_trans": distinct_trans,
        "wordlen": len(word),
    }

for field in ["seed","privkey","pubkey"]:
    inv = sum(1 for (i,f),v in results.items() if f==field and v["invariant"])
    n = sum(1 for (i,f) in results if f==field)
    avg_land = sum(v["n_landings"] for (i,f),v in results.items() if f==field)/n
    avg_hold = sum(v["n_hold"] for (i,f),v in results.items() if f==field and v["n_hold"] is not None)/n
    print(f"{field} (N={n}): ROTAT-invariant={inv}/{n}  mean landings={avg_land:.2f}  mean insert-holds={avg_hold:.2f}")

idxs = sorted(set(i for i,f in results))
print()
def control(key_fn, label):
    for a,b in [("privkey","pubkey"), ("seed","privkey"), ("seed","pubkey")]:
        real = sum(1 for i in idxs if key_fn(i,a)==key_fn(i,b))
        b_vals = [key_fn(i,b) for i in idxs]
        K = 3000
        cnt = []
        for _ in range(K):
            perm = b_vals[:]
            random.shuffle(perm)
            m = sum(1 for x,y in zip((key_fn(i,a) for i in idxs), perm) if x==y)
            cnt.append(m)
        mean_shuf = sum(cnt)/K
        ge = sum(1 for m in cnt if m>=real); le = sum(1 for m in cnt if m<=real)
        p = min(1.0, 2*min(ge,le)/K)
        print(f"  {label} {a} vs {b}: real={real} shuffle-mean={mean_shuf:.2f} p={p:.4f}")

print("cycle: n_landings exact match")
control(lambda i,f: results[(i,f)]["n_landings"], "n_landings")
print("cycle: invariant/phase-bearing category")
control(lambda i,f: results[(i,f)]["invariant"], "invariant-flag")
print("insert: (n_hold, n_tried) exact match")
control(lambda i,f: (results[(i,f)]["n_hold"], results[(i,f)]["n_tried"]), "insert-holds")
print("trans: closing edge exact match")
control(lambda i,f: results[(i,f)]["close_edge"], "closing-edge")
print("trans: n_trans exact match")
control(lambda i,f: results[(i,f)]["n_trans"], "n_trans")
