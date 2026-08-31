import re, random, subprocess
random.seed(20260826)

rows = []
with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100.txt", encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit(): continue
        parts = [p.strip() for p in line.split("|")]
        idx, mn, seed, priv, pub = parts[0], parts[1], parts[2], parts[3], parts[4]
        rows.append((idx, priv, pub))

BIN = "/home/mrnob0dy666/imsgct/mOMonadOS/target/x86_64-unknown-linux-gnu/release/momonados"

def xor_hex(a, b):
    a = bytes.fromhex(a); b = bytes.fromhex(b)
    n = min(len(a), len(b))
    return bytes(x ^ y for x, y in zip(a[:n], b[:n])).hex()

privs = [r[1] for r in rows]
pubs_keep_parity = [r[2] for r in rows]   # keep leading parity byte, truncate trailing byte to match length

def batch_verdicts(hex_list):
    cmds = [f"vox-ce hex {h}\n" for h in hex_list] + ["quit\n"]
    out = subprocess.run([BIN], input="".join(cmds), capture_output=True, text=True, timeout=60).stdout
    verdicts = re.findall(r"verdict:\s*([A-Za-z]+)", out)
    return verdicts

# real pairing, parity-KEPT alignment
real_xor = [xor_hex(privs[i], pubs_keep_parity[i]) for i in range(100)]
real_v = batch_verdicts(real_xor)
from collections import Counter
print("REAL (priv XOR pub, parity byte KEPT, trailing byte dropped):", Counter(real_v))

# shuffled-pairing control, same alignment, 10 derangements
shuf_v = []
n = 100
for trial in range(10):
    perm = list(range(n))
    while True:
        random.shuffle(perm)
        if all(perm[i] != i for i in range(n)): break
    xs = [xor_hex(privs[i], pubs_keep_parity[perm[i]]) for i in range(n)]
    shuf_v.extend(batch_verdicts(xs))
print("SHUFFLED (1000 mismatched pairs, same alignment):", Counter(shuf_v))

real_B = real_v.count("B")
shuf_B = shuf_v.count("B")
import math
p1, n1 = real_B/100, 100
p2, n2 = shuf_B/1000, 1000
pooled = (real_B+shuf_B)/(n1+n2)
se = math.sqrt(pooled*(1-pooled)*(1/n1+1/n2))
z = (p1-p2)/se
print(f"\nreal B-share={p1:.3f} ({real_B}/100), shuffled B-share={p2:.3f} ({shuf_B}/1000), z={z:.3f}")
