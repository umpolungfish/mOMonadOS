import re, subprocess, os

path = "/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100.txt"
rows = []
with open(path, encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit():
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) < 5:
            continue
        idx, mn, seed, priv, pub = parts[0], parts[1], parts[2], parts[3], parts[4]
        rows.append((idx, seed, priv, pub))

assert len(rows) == 100, len(rows)

VOX = "/home/mrnob0dy666/imsgct/Vox/target/release/vox"
RAWDIR = "/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_raw"
os.makedirs(RAWDIR, exist_ok=True)

def vox_word(hexstr, tag):
    binpath = f"{RAWDIR}/{tag}.bin"
    with open(binpath, "wb") as f:
        f.write(bytes.fromhex(hexstr))
    out = subprocess.run([VOX, "word", binpath], capture_output=True, text=True, timeout=10).stdout
    words = []
    for line in out.strip().splitlines():
        parts = line.split("\t")
        if len(parts) == 2:
            words.append(parts[1])
    # also grab the summary line (raw/elf + coverage) from the bare `vox <file>` call
    summ = subprocess.run([VOX, binpath], capture_output=True, text=True, timeout=10).stdout
    first_line = summ.strip().splitlines()[0] if summ.strip() else ""
    kind = "raw" if " raw " in first_line else ("elf" if " elf " in first_line else "?")
    cov_line = [l for l in summ.strip().splitlines() if "covered" in l]
    coverage = cov_line[0].strip() if cov_line else ""
    return "".join(words), kind, coverage, len(words)

results = []
for idx, seed, priv, pub in rows:
    for field, h in [("seed", seed), ("privkey", priv), ("pubkey", pub)]:
        tag = f"{field}{idx}"
        word, kind, coverage, nfuncs = vox_word(h, tag)
        results.append((idx, field, word, kind, coverage, nfuncs))
        os.remove(f"{RAWDIR}/{tag}.bin")

with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100_vox_words.txt", "w", encoding="utf-8") as f:
    f.write("# 100 BIP39 keypairs, hex fields converted to IMASM via the real Vox disassembler (vox word)\n")
    f.write("# NOT the byte%12 residue lift (vox-ce) used in the earlier keypairs_100_voxce.txt audit.\n")
    f.write("# idx | field | vox-word (functions concatenated, address order) | kind | coverage | n-functions\n")
    f.write("-"*100 + "\n")
    for idx, field, word, kind, coverage, nfuncs in results:
        f.write(f"{idx} | {field} | {word} | {kind} | {coverage} | {nfuncs}\n")

print("done,", len(results), "rows written")
# quick sanity: kind distribution
from collections import Counter
print(Counter(k for *_, k, _, _ in [(r[0],r[1],r[2],r[3],r[4],r[5]) for r in results]))
kinds = Counter(r[3] for r in results)
print("kind distribution:", kinds)
lens = [len(r[2]) for r in results]
print("word length: min", min(lens), "max", max(lens), "mean", sum(lens)/len(lens))
