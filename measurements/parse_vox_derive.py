import re, random, json
from collections import Counter
random.seed(20260826)

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_derive_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field = line.rstrip("\n").split("\t")
        labels.append((idx, field))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_derive_out.txt", encoding="utf-8").read()
crystals = re.findall(r"crystal: (\d+)", out)
assert len(crystals) == len(labels), (len(crystals), len(labels))

data = {}
for (idx, field), c in zip(labels, crystals):
    data.setdefault(idx, {})[field] = int(c)

freq = Counter()
for idx, d in data.items():
    for field, c in d.items():
        freq[c] += 1
print("distinct crystal addresses:", len(freq), "of 300 words")
print("top 10:", freq.most_common(10))

idxs = sorted(data.keys())
for a, b in [("seed","privkey"), ("seed","pubkey"), ("privkey","pubkey")]:
    real = sum(1 for i in idxs if data[i].get(a) == data[i].get(b))
    b_vals = [data[i][b] for i in idxs]
    K = 3000
    cnt = []
    for _ in range(K):
        perm = b_vals[:]
        random.shuffle(perm)
        m = sum(1 for x,y in zip((data[i][a] for i in idxs), perm) if x==y)
        cnt.append(m)
    mean_shuf = sum(cnt)/K
    ge = sum(1 for m in cnt if m>=real); le = sum(1 for m in cnt if m<=real)
    p = min(1.0, 2*min(ge,le)/K)
    print(f"{a} == {b}: real={real} shuffle-mean={mean_shuf:.2f} p={p:.4f}")

json.dump({i: d for i,d in data.items()}, open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_derive_data.json","w"))
