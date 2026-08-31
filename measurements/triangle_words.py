import re

path = "/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100_voxce.txt"
rows = []  # (idx, field, word)
with open(path, encoding="utf-8") as f:
    for line in f:
        line = line.rstrip("\n")
        if not re.match(r"^\d{3} \|", line):
            continue
        parts = [p.strip() for p in line.split("|")]
        # idx | field | hex | word | register | verdict | closed_walk
        if len(parts) < 7:
            continue
        idx, field, hexv, word = parts[0], parts[1], parts[2], parts[3]
        rows.append((idx, field, word))

print("total rows:", len(rows))

cmds = []
labels = []
for idx, field, word in rows:
    cmds.append(f"imasm derive {word}")
    labels.append((idx, field))

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/triangle_cmds.txt", "w", encoding="utf-8") as f:
    for c in cmds:
        f.write(c + "\n")
    f.write("quit\n")

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/triangle_labels.txt", "w", encoding="utf-8") as f:
    for idx, field in labels:
        f.write(f"{idx}\t{field}\n")

print("commands:", len(cmds))
