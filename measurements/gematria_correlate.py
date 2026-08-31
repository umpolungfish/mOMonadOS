import re, json
from scipy import stats

gem = json.load(open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/gematria.json"))

# re-parse weight numerics + insert n_hold + cycle n_landings from the vox batches
labels_wb = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_wb_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field, word = line.rstrip("\n").split("\t")
        labels_wb.append((idx, field, word))
out_wb = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_wb_out.txt", encoding="utf-8").read()
blocks_wb = re.split(r"⊙> (?=weight |banked |counterfactual )", out_wb)[1:]

labels_cit = []
with open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_cit_labels.txt", encoding="utf-8") as f:
    for line in f:
        idx, field, word = line.rstrip("\n").split("\t")
        labels_cit.append((idx, field, word))
out_cit = open("/home/mrnob0dy666/imsgct/mOMonadOS/measurements/vox_cit_out.txt", encoding="utf-8").read()
blocks_cit = re.split(r"⊙> (?=cycle |insert |trans )", out_cit)[1:]

numeric = {}
for i, (idx, field, word) in enumerate(labels_wb):
    wb = blocks_wb[3*i]
    m = re.search(r"deposits (\d+)\s+cleared (\d+)\s+restored (\d+)\s+seeded (\d+)\s+inert (\d+)", wb)
    numeric.setdefault((idx,field), {})
    if m:
        numeric[(idx,field)]["deposits"] = int(m.group(1))
        numeric[(idx,field)]["cleared"] = int(m.group(2))
        numeric[(idx,field)]["inert"] = int(m.group(5))

for i, (idx, field, word) in enumerate(labels_cit):
    cy = blocks_cit[3*i]; ins = blocks_cit[3*i+1]
    n_landings = len(re.findall(r"^\s+\S+\s+at k", cy, re.M))
    hold_m = re.search(r"(\d+) distinct word\(s\) hold, of (\d+) tried", ins)
    numeric.setdefault((idx,field), {})
    numeric[(idx,field)]["n_landings"] = n_landings
    numeric[(idx,field)]["n_hold"] = int(hold_m.group(1)) if hold_m else None

idxs = sorted(gem.keys())
gem_vars = ["standard", "posweighted", "poly_mod_1e9"]
readouts = ["deposits", "cleared", "inert", "n_landings", "n_hold"]

print(f"{'gematria':<12} {'field':<8} {'readout':<10} {'r':>7} {'p':>8}")
results = []
for gv in gem_vars:
    gvals = [gem[i][gv] for i in idxs]
    for field in ["seed","privkey","pubkey"]:
        for ro in readouts:
            yvals = [numeric[(i,field)].get(ro) for i in idxs]
            if any(v is None for v in yvals): continue
            r, p = stats.pearsonr(gvals, yvals)
            results.append((gv, field, ro, r, p))

results.sort(key=lambda x: x[4])
for gv, field, ro, r, p in results[:15]:
    print(f"{gv:<12} {field:<8} {ro:<10} {r:7.3f} {p:8.4f}")
print(f"\ntotal comparisons: {len(results)}")
sig = [x for x in results if x[4] < 0.05]
print(f"p<0.05: {len(sig)} (expected by chance at N={len(results)}: ~{len(results)*0.05:.1f})")
