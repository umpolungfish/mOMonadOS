import re

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/triangle_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field = line.rstrip("\n").split("\t")
        labels.append((idx, field))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/triangle_out.txt", encoding="utf-8").read()

# Each call block: "⊙> imasm derive <word>\nword:  ...\ntuple: <...>\ncrystal: <N>"
blocks = re.findall(r"⊙> imasm derive .*?\ncrystal: (\d+)", out, re.S)
tuples  = re.findall(r"tuple: (⟨[^⟩]*⟩)", out)

assert len(blocks) == len(labels), (len(blocks), len(labels))
assert len(tuples) == len(labels), (len(tuples), len(labels))

data = {}  # idx -> {field: (crystal, tuple)}
for (idx, field), crystal, tup in zip(labels, blocks, tuples):
    data.setdefault(idx, {})[field] = (int(crystal), tup)

# 1. frequency table of crystal addresses across all 300
from collections import Counter
freq = Counter()
for idx, d in data.items():
    for field, (c, t) in d.items():
        freq[c] += 1

print("=== crystal address frequency (top 15 of %d distinct) ===" % len(freq))
for c, n in freq.most_common(15):
    print(f"  {c}: {n}")

# 2. same-row matches
same_sp = sum(1 for idx,d in data.items() if d.get('seed',(None,))[0]==d.get('privkey',(None,))[0])
same_su = sum(1 for idx,d in data.items() if d.get('seed',(None,))[0]==d.get('pubkey',(None,))[0])
same_pu = sum(1 for idx,d in data.items() if d.get('privkey',(None,))[0]==d.get('pubkey',(None,))[0])
n = len(data)
print(f"\n=== same-row exact crystal match (N={n} rows) ===")
print(f"  seed == privkey: {same_sp}")
print(f"  seed == pubkey : {same_su}")
print(f"  privkey==pubkey: {same_pu}")

import json
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/triangle_data.json","w",encoding="utf-8") as f:
    json.dump({idx: {k: v for k,v in d.items()} for idx,d in data.items()}, f, ensure_ascii=False)
