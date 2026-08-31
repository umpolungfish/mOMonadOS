"""The image of imas_ig.rs::from_snapshot.

The derivation reads twelve axes off six snapshot scalars (token_diversity,
period, frobenius_order, self_ref, dialetheia, IFIX count, non-zero signature
count). This enumerates every feature combination and reports which tuples the
map can write, which catalog entries lie outside its image, and why.
"""
import json, itertools, sys

G = {  # primitive -> glyph, by axis, in canonical slot order
 'if_':'\U00010466','dead':'\U0001045b','ash':'\U00010468','array':'\U0001047c',
 'are':'\U00010478','judge':'\U00010461','eat':'\U00010470','mime':'\U00010465','oil':'\U00010476',
 'ian':'\U0001047e','ear':'\U0001047d','tot':'\U00010451','ado':'\U00010469',
 'or_':'\U00010479','nun':'\U0001046f','out':'\U0001046c','yew':'\U0001047f','church':'\U00010457',
 'peep':'\U00010450','age':'\U00010471','they':'\U0001045e',
 'on':'\U0001046a','egg':'\U00010467','loll':'\U00010464','yea':'\U00010458','air':'\U0001047a',
 'ice':'\U00010472','bib':'\U0001045a','thigh':'\U00010454',
 'measure':'\U00010460','vow':'\U0001045d','gag':'\U0001045c','ooze':'\U00010475',
 'monad':'⊙','roar':'\U0001046e','err':'\U0001047b','woe':'\U00010462','haha':'\U00010463',
 'wool':'\U0001046b','sure':'\U00010456','kick':'\U00010452','fee':'\U00010453',
 'up':'\U00010473','so':'\U00010455','hung':'\U00010459',
 'ah':'\U0001046d','oak':'\U00010474','awe':'\U00010477','zoo':'\U0001045f',
}

AXES = [
 ('⊢','D',['if_','dead','ash','array']),
 ('⊣','T',['are','judge','eat','mime','oil']),
 ('≻','R',['ian','ear','tot','ado']),
 ('≺','P',['or_','nun','out','yew','church']),
 ('⋈','F',['peep','age','they']),
 ('⊤','K',['on','egg','loll','yea','air']),
 ('∈','G',['ice','bib','thigh']),
 ('∋','C',['measure','vow','gag','ooze']),
 ('⊙','Phi',['monad','roar','err','woe','haha']),
 ('⊥','H',['wool','sure','kick','fee']),
 ('⊞','S',['up','so','hung']),
 ('⊡','Om',['ah','oak','awe','zoo']),
]

def derive(d, p, fo, sr, dc, sx, nz):
    dv   = 'dead' if d<=2 else 'ash' if d<=5 else 'array' if d<=9 else 'if_'
    tv   = 'are' if sr else 'judge' if p==1 else 'mime' if p==2 else 'oil' if fo>0 else 'eat'
    rv   = {1:'ian',2:'ear',3:'tot'}.get(fo,'ado')
    pv   = {1:'or_',2:'nun',3:'out'}.get(fo, 'yew' if dc else 'church')
    fv   = 'peep' if dc else 'age' if p==1 else 'they'
    kv   = 'on' if sx==8 else 'air' if sx>8 else 'egg' if p==1 else 'loll' if p<=4 else 'yea'
    gv   = 'ice' if d>=10 else 'thigh' if d>=4 else 'bib'
    cv   = 'measure' if fo>0 else 'vow' if p==1 else 'gag' if p==2 else 'ooze'
    phiv = 'monad' if (sr and dc) else 'roar' if sr else 'err' if dc else 'woe' if p==1 else 'haha'
    hv   = {1:'fee',2:'kick',3:'sure'}.get(p,'wool')
    sv   = 'hung' if nz==1 else 'so' if nz==2 else 'up'
    omv  = 'ah' if fo==1 else 'oak' if fo==2 else ('ah' if sr else 'oak' if p==2 else 'awe')
    return (dv,tv,rv,pv,fv,kv,gv,cv,phiv,hv,sv,omv)

image = set()
for d in range(0,15):
  for p in range(0,9):
    for fo in range(0,6):
      for sr in (0,1):
        for dc in (0,1):
          for sx in range(0,13):
            for nz in range(1,5):
              image.add(derive(d,p,fo,sr,dc,sx,nz))

full = 1
for _,_,vals in AXES: full *= len(vals)
print(f"crystal (all axis combinations): {full}")
print(f"image of from_snapshot:          {len(image)}   ({100*len(image)/full:.4f}%)")

# per-axis reachability
for i,(mark,name,vals) in enumerate(AXES):
    seen = {t[i] for t in image}
    miss = [v for v in vals if v not in seen]
    if miss: print(f"  {mark} {name}: never emits {', '.join(miss)}")

gl = {v:k for k,v in G.items()}
def parse(tup):
    out=[]
    for ch in tup:
        if ch in gl: out.append(gl[ch])
    return tuple(out)

L9 = '\U0001045b\U00010465\U00010451\U0001046c\U00010450\U0001046a\U00010454\U0001045d⊙\U0001046b\U00010473\U0001046d'
l9 = parse(L9)
print(f"\nCLINK L9 = {[f'{m}{v}' for (m,_,_),v in zip(AXES,l9)]}")
print(f"CLINK L9 in image: {l9 in image}")

cat = json.load(open('/home/mrnob0dy666/imsgct/imscribing_grammar/IG_catalog.json'))
marks = [a[0] for a in AXES]
inside = outside = skipped = 0
outnames=[]
for e in cat:
    try:
        t = tuple(gl[e[m]] for m in marks)
    except KeyError:
        skipped += 1; continue
    if t in image: inside += 1
    else:
        outside += 1
        if len(outnames)<12: outnames.append(e.get('name'))
print(f"\ncatalog: {inside} inside the image, {outside} outside, {skipped} unparsed")
print("outside, first few:", ", ".join(outnames))
