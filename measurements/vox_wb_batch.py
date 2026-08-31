rows = []
with open("/home/mrnob0dy666/imsgct/ig-docs/keypairs/keypairs_100_vox_words.txt", encoding="utf-8") as f:
    for line in f:
        if not line[:3].isdigit(): continue
        parts = [p.strip() for p in line.split("|")]
        idx, field, word = parts[0], parts[1], parts[2]
        rows.append((idx, field, word))

cmds, labels = [], []
for idx, field, word in rows:
    cmds.append(f"weight {word}")
    cmds.append(f"banked {word}")
    cmds.append(f"counterfactual {word} rotate 1")
    labels.append((idx, field, word))

with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_wb_cmds.txt","w",encoding="utf-8") as f:
    for c in cmds: f.write(c+"\n")
    f.write("quit\n")
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_wb_labels.txt","w",encoding="utf-8") as f:
    for l in labels: f.write(f"{l[0]}\t{l[1]}\t{l[2]}\n")
print(len(rows), "rows,", len(cmds), "cmds")
