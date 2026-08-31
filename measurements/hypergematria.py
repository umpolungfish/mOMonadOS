import re
from scipy import stats
import json

rows = []
with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100.txt", encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit(): continue
        parts = [p.strip() for p in line.split("|")]
        idx, mn = parts[0], parts[1]
        rows.append((idx, mn))

def gematria(mn):
    letters = [c for c in mn if c.isalpha()]
    standard = sum(ord(c.lower()) - ord('a') + 1 for c in letters)
    posw = sum((ord(c.lower()) - ord('a') + 1) * (i+1) for i, c in enumerate(letters))
    poly = 0
    for c in letters:
        poly = poly * 26 + (ord(c.lower()) - ord('a') + 1)
    return standard, posw, poly

gem = {}
for idx, mn in rows:
    s, p, poly = gematria(mn)
    gem[idx] = {"standard": s, "posweighted": p, "poly_mod_1e9": poly % 1_000_000_007}

json.dump(gem, open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/gematria.json","w"))
for idx in ["000","001","002"]:
    print(idx, rows[int(idx)][1], gem[idx])
print("standard gematria range:", min(g['standard'] for g in gem.values()), "-", max(g['standard'] for g in gem.values()))
