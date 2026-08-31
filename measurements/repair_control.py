import re, random
random.seed(20260826)

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

MARKS = list("⊢⊣≻≺⋈⊙∈∋⊤⊥⊞⊡")  # kernel MARKS order (counterfactual.rs), just for generating independent-random control words

cmds = []
labels = []
for idx, field, word in rows:
    cmds.append(f"counterfactual {word} rotate 1")
    labels.append((idx, field, len(word)))

# independent random-glyph control words: 100 at length 32, 100 at length 33, 100 at length 64
rand_labels = []
for L, tag in [(32,"rand32"), (33,"rand33"), (64,"rand64")]:
    for i in range(100):
        w = "".join(random.choice(MARKS) for _ in range(L))
        cmds.append(f"counterfactual {w} rotate 1")
        rand_labels.append((tag, i, L))

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/repair_cmds.txt","w",encoding="utf-8") as f:
    for c in cmds: f.write(c+"\n")
    f.write("quit\n")

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/repair_labels.txt","w",encoding="utf-8") as f:
    for l in labels: f.write(f"corpus\t{l[0]}\t{l[1]}\t{l[2]}\n")
    for l in rand_labels: f.write(f"random\t{l[0]}\t{l[1]}\t{l[2]}\n")

print("corpus:", len(labels), "random:", len(rand_labels), "total cmds:", len(cmds))
