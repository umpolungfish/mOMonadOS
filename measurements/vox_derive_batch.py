import re

rows = []
with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100_vox_words.txt", encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit():
            continue
        parts = [p.strip() for p in line.split("|")]
        idx, field, word = parts[0], parts[1], parts[2]
        rows.append((idx, field, word))

print(len(rows), "rows")

cmds = []
labels = []
for idx, field, word in rows:
    cmds.append(f"imasm derive {word}")
    labels.append((idx, field))

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_derive_cmds.txt", "w", encoding="utf-8") as f:
    for c in cmds: f.write(c + "\n")
    f.write("quit\n")
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_derive_labels.txt", "w", encoding="utf-8") as f:
    for l in labels: f.write(f"{l[0]}\t{l[1]}\n")
