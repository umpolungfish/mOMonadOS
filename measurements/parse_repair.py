import re
from collections import Counter

labels = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/repair_labels.txt", encoding="utf-8") as f:
    for line in f:
        src, a, b, c = line.rstrip("\n").split("\t")
        labels.append((src, a, b, c))

out = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/repair_out.txt", encoding="utf-8").read()
blocks = re.split(r"⊙> (?=counterfactual )", out)[1:]
assert len(blocks) == len(labels), (len(blocks), len(labels))

repairs = []
no_repair = 0
for lab, b in zip(labels, blocks):
    m = re.search(r"smallest repair.*?\n\s*(insert|delete|swap|replace)\s+(\S+)?\s*(?:at\s+(\d+))?", b)
    if not m:
        # maybe "no repair found" or unreached invariant
        repairs.append((lab, None))
        no_repair += 1
        continue
    kind, glyph, pos = m.group(1), m.group(2), m.group(3)
    repairs.append((lab, (kind, glyph, pos)))

print("total:", len(repairs), "no-repair-found:", no_repair)

corpus_repairs = Counter(r for (src,*_),r in repairs if src=="corpus")
random_repairs = Counter(r for (src,*_),r in repairs if src=="random")

print("\n=== corpus (N=300) top repairs ===")
for r,n in corpus_repairs.most_common(10):
    print(f"  {r}: {n}")

print("\n=== independent-random (N=300) top repairs ===")
for r,n in random_repairs.most_common(10):
    print(f"  {r}: {n}")

target = ("insert", "⊢", "0")
print(f"\n'insert ⊢ at 0' exactly: corpus {corpus_repairs.get(target,0)}/300, random {random_repairs.get(target,0)}/300")

# also: how many hold "insert <anything> at position 0" (any glyph, position 0)
def at0(r):
    return r is not None and r[0]=="insert" and r[2]=="0"
c_at0 = sum(1 for (src,*_),r in repairs if src=="corpus" and at0(r))
r_at0 = sum(1 for (src,*_),r in repairs if src=="random" and at0(r))
print(f"'insert <any glyph> at 0': corpus {c_at0}/300, random {r_at0}/300")
