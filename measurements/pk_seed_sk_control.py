import re, random, hashlib

random.seed(20260826)

path = "/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100.txt"
rows = []
with open(path) as f:
    for line in f:
        if not line.startswith(tuple("0123456789")):
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) < 5:
            continue
        idx, mn, seed, priv, pub = parts[0], parts[1], parts[2], parts[3], parts[4]
        rows.append((idx, priv, pub))

assert len(rows) == 100, len(rows)

def xor_hex(a_hex, b_hex):
    a = bytes.fromhex(a_hex)
    b = bytes.fromhex(b_hex)
    n = min(len(a), len(b))
    return bytes(x ^ y for x, y in zip(a[:n], b[:n])).hex()

# real pairing: priv (32B) XOR pub[1:] (drop leading parity byte -> 32B)
real_cmds = []
real_labels = []
for idx, priv, pub in rows:
    pub_body = pub[2:]  # drop leading parity byte (1 hex-pair = 2 hex chars)
    x = xor_hex(priv, pub_body)
    real_cmds.append(f"vox-ce hex {x}")
    real_labels.append(("real", idx))

# shuffled-pairing control: derange the pub list against the priv list, repeat K times
K = 10
shuf_cmds = []
shuf_labels = []
privs = [r[1] for r in rows]
pubs_body = [r[2][2:] for r in rows]
n = len(rows)
for trial in range(K):
    perm = list(range(n))
    while True:
        random.shuffle(perm)
        if all(perm[i] != i for i in range(n)):
            break
    for i in range(n):
        x = xor_hex(privs[i], pubs_body[perm[i]])
        shuf_cmds.append(f"vox-ce hex {x}")
        shuf_labels.append(("shuf%d" % trial, i))

# independent-random control: fresh random 32B "privkey-like" and 32B "pubkey-body-like"
rand_cmds = []
rand_labels = []
R = 500
for i in range(R):
    a = random.randbytes(32).hex()
    b = random.randbytes(32).hex()
    x = xor_hex(a, b)
    rand_cmds.append(f"vox-ce hex {x}")
    rand_labels.append(("rand", i))

all_cmds = real_cmds + shuf_cmds + rand_cmds
all_labels = [("real",)]*0  # placeholder
labels = [("real", l) for l in [r[0] for r in rows]] + shuf_labels + rand_labels

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/pk_seed_sk_cmds.txt", "w") as f:
    for c in all_cmds:
        f.write(c + "\n")
    f.write("quit\n")

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/pk_seed_sk_labels.txt", "w") as f:
    for lab in labels:
        f.write(f"{lab[0]}\t{lab[1]}\n")

print("real:", len(real_cmds), "shuf:", len(shuf_cmds), "rand:", len(rand_cmds), "total:", len(all_cmds))
