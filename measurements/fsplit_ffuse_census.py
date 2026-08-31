import re
from collections import Counter

path = "/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100_voxce.txt"
rows = []
with open(path, encoding="utf-8") as f:
    for line in f:
        line = line.rstrip("\n")
        if not re.match(r"^\d{3} \|", line):
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) < 7:
            continue
        idx, field, hexv, word = parts[0], parts[1], parts[2], parts[3]
        rows.append((idx, field, word))

for field in ["seed", "privkey", "pubkey"]:
    fs = []  # fsplit counts
    ff = []  # ffuse counts
    zero_fsplit = 0
    zero_ffuse = 0
    for idx, f_, word in rows:
        if f_ != field: continue
        c_fs = word.count("∈")
        c_ff = word.count("∋")
        fs.append(c_fs); ff.append(c_ff)
        if c_fs == 0: zero_fsplit += 1
        if c_ff == 0: zero_ffuse += 1
    n = len(fs)
    print(f"=== {field} (N={n}) ===")
    print(f"  FSPLIT ∈ count: mean={sum(fs)/n:.2f} min={min(fs)} max={max(fs)}  zero-FSPLIT rows: {zero_fsplit}")
    print(f"  FFUSE  ∋ count: mean={sum(ff)/n:.2f} min={min(ff)} max={max(ff)}  zero-FFUSE rows: {zero_ffuse}")
    print(f"  FSPLIT dist: {dict(Counter(fs))}")
    print(f"  FFUSE  dist: {dict(Counter(ff))}")
    print()
