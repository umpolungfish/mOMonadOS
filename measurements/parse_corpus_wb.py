import re

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/corpus_wb_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field = line.rstrip("\n").split("\t")
        labels.append((idx, field))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/corpus_wb_out.txt", encoding="utf-8").read()

# split into per-command blocks starting at "⊙> weight" or "⊙> banked"
blocks = re.split(r"⊙> (?=weight |banked )", out)[1:]

results = {}  # (idx, field) -> dict
li = 0
i = 0
while i < len(blocks):
    wb = blocks[i]
    bb = blocks[i+1]
    idx, field = labels[li]

    # weight block: deposits N cleared N restored N seeded N inert N ; final: X ; VACUOUS? no, weight never says vacuous
    m = re.search(r"deposits (\d+)\s+cleared (\d+)\s+restored (\d+)\s+seeded (\d+)\s+inert (\d+)", wb)
    stranded = re.search(r"stranded in frames never fused: (\d+)", wb)
    finalm = re.search(r"final\s*:\s*(\S+)", wb)

    # banked block
    vacuous = "VACUOUS" in bb
    exposed = re.search(r"(\d+) unit\(s\) cleared with nothing banked", bb)
    ok = "banked OK" in bb or ("VACUOUS" not in bb and exposed is None and "cleared" not in bb)

    results[(idx, field)] = {
        "deposits": int(m.group(1)) if m else None,
        "cleared": int(m.group(2)) if m else None,
        "restored": int(m.group(3)) if m else None,
        "seeded": int(m.group(4)) if m else None,
        "inert": int(m.group(5)) if m else None,
        "stranded": int(stranded.group(1)) if stranded else 0,
        "final": finalm.group(1) if finalm else None,
        "vacuous": vacuous,
        "exposed_cleared": int(exposed.group(1)) if exposed else None,
    }
    li += 1
    i += 2

# Tabulate by field
from collections import Counter
for field in ["seed", "privkey", "pubkey"]:
    vac = sum(1 for (idx,f),v in results.items() if f==field and v["vacuous"])
    exp = Counter(v["exposed_cleared"] for (idx,f),v in results.items() if f==field and v["exposed_cleared"] is not None)
    okc = sum(1 for (idx,f),v in results.items() if f==field and not v["vacuous"] and v["exposed_cleared"] is None)
    n = sum(1 for (idx,f) in results if f==field)
    print(f"=== {field} (N={n}) ===")
    print(f"  VACUOUS: {vac}")
    print(f"  banked-OK (neither vacuous nor exposed): {okc}")
    print(f"  EXPOSED, by cleared-count: {dict(exp)}")
    print()
