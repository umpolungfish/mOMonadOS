import re, subprocess, itertools
from collections import Counter

rows = []
with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100.txt", encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit(): continue
        parts = [p.strip() for p in line.split("|")]
        idx, mn, seed, priv, pub = parts[0], parts[1], parts[2], parts[3], parts[4]
        rows.append((mn, seed, priv, pub))

BIN = "/home/mrnob0dy666/imsgct/mOMonadOS/target/x86_64-unknown-linux-gnu/release/momonados"

def xor_hex(a, b):
    ab = bytes.fromhex(a); bb = bytes.fromhex(b)
    n = min(len(ab), len(bb))
    return bytes(x ^ y for x, y in zip(ab[:n], bb[:n])).hex()

def xor_ascii(a, b):
    ab = a.encode(); bb = b.encode()
    n = min(len(ab), len(bb))
    return bytes(x ^ y for x, y in zip(ab[:n], bb[:n])).hex()

def batch_verdicts(hex_list):
    cmds = [f"vox-ce hex {h}\n" for h in hex_list] + ["quit\n"]
    out = subprocess.run([BIN], input="".join(cmds), capture_output=True, text=True, timeout=180).stdout
    return re.findall(r"verdict:\s*([A-Za-z]+)", out)

mn = [r[0] for r in rows]; sd = [r[1] for r in rows]; pv = [r[2] for r in rows]; pb = [r[3] for r in rows]
n = 100

# Sample C(100,2)=4950 pairs is a lot of kernel calls (4 fields x 4950 = 19800);
# batch each field type in one process call, but subsample to keep it fast:
# every unique pair, but cap at ~1000 random pairs per field type for speed,
# matching the shuffle-control sample sizes already used this session.
import random
random.seed(20260826)
all_pairs = list(itertools.combinations(range(n), 2))
sample_pairs = random.sample(all_pairs, 1000)

for name, data, xorfn in [
    ("mnemonic-mnemonic", mn, xor_ascii),
    ("seed-seed", sd, xor_hex),
    ("privkey-privkey", pv, xor_hex),
    ("pubkey-pubkey", pb, xor_hex),
]:
    xs = [xorfn(data[i], data[j]) for i, j in sample_pairs]
    v = batch_verdicts(xs)
    c = Counter(v)
    total = sum(c.values())
    print(f"{name:20s} N={total:4d}  T={c.get('T',0):4d} ({100*c.get('T',0)/total:.1f}%)  "
          f"F={c.get('F',0):4d} ({100*c.get('F',0)/total:.1f}%)  "
          f"B={c.get('B',0):4d} ({100*c.get('B',0)/total:.1f}%)  N={c.get('N',0):4d}")
