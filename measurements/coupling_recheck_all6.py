import re, random, subprocess, math
from collections import Counter
random.seed(20260826)

rows = []
with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100.txt", encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit(): continue
        parts = [p.strip() for p in line.split("|")]
        idx, mn, seed, priv, pub = parts[0], parts[1], parts[2], parts[3], parts[4]
        rows.append((idx, mn, seed, priv, pub))

BIN = "/home/mrnob0dy666/imsgct/mOMonadOS/target/x86_64-unknown-linux-gnu/release/momonados"

def xor_hex(a, b):
    ab = bytes.fromhex(a); bb = bytes.fromhex(b)
    n = min(len(ab), len(bb))
    return bytes(x ^ y for x, y in zip(ab[:n], bb[:n])).hex()

def xor_ascii_hex(mn, other_hex):
    mn_bytes = mn.encode()
    other_bytes = bytes.fromhex(other_hex)
    n = min(len(mn_bytes), len(other_bytes))
    return bytes(x ^ y for x, y in zip(mn_bytes[:n], other_bytes[:n])).hex()

def batch_verdicts(hex_list):
    cmds = [f"vox-ce hex {h}\n" for h in hex_list] + ["quit\n"]
    out = subprocess.run([BIN], input="".join(cmds), capture_output=True, text=True, timeout=90).stdout
    return re.findall(r"verdict:\s*([A-Za-z]+)", out)

mn = [r[1] for r in rows]; sd = [r[2] for r in rows]; pv = [r[3] for r in rows]; pb = [r[4] for r in rows]

pairs = {
    "mnemonic-seed":   (lambda i,j: xor_ascii_hex(mn[i], sd[j])),
    "mnemonic-privkey":(lambda i,j: xor_ascii_hex(mn[i], pv[j])),
    "mnemonic-pubkey": (lambda i,j: xor_ascii_hex(mn[i], pb[j])),
    "seed-privkey":    (lambda i,j: xor_hex(sd[i], pv[j])),
    "seed-pubkey":     (lambda i,j: xor_hex(sd[i], pb[j])),
    "privkey-pubkey":  (lambda i,j: xor_hex(pv[i], pb[j])),
}

n = 100
results = {}
for name, fn in pairs.items():
    real_xor = [fn(i,i) for i in range(n)]
    real_v = batch_verdicts(real_xor)
    real_B = real_v.count("B")

    shuf_B_total = 0
    K = 10
    for trial in range(K):
        perm = list(range(n))
        while True:
            random.shuffle(perm)
            if all(perm[i] != i for i in range(n)): break
        xs = [fn(i, perm[i]) for i in range(n)]
        v = batch_verdicts(xs)
        shuf_B_total += v.count("B")
    shuf_B = shuf_B_total
    p1, n1 = real_B/n, n
    p2, n2 = shuf_B/(n*K), n*K
    pooled = (real_B+shuf_B)/(n1+n2)
    se = math.sqrt(pooled*(1-pooled)*(1/n1+1/n2)) if pooled not in (0,1) else 1e-9
    z = (p1-p2)/se
    results[name] = (real_B, shuf_B, n1, n2, z)
    print(f"{name:20s} real_B={real_B:3d}/{n1}  shuffled_B={shuf_B:4d}/{n2}  z={z:6.3f}")

print()
zs = sorted(results.items(), key=lambda kv: -abs(kv[1][4]))
print("ranked by |z|:", [(k, round(v[4],3)) for k,v in zs])
bonf = 0.05/6
from scipy.stats import norm
crit = abs(norm.ppf(bonf/2))
print(f"\nBonferroni-corrected critical |z| for 6 comparisons at alpha=0.05: {crit:.3f}")
