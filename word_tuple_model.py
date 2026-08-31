"""word -> IG tuple, faithful to kernel.rs::self_imscribe + imas_ig.rs::from_snapshot.

Validated against the live kernel's own `imasm derive` output. R2=False reproduces
the derivation before the R2 fields were read; R2=True is with them.
"""
MARKS = "⊢⊣≻≺⋈⊤∈∋⊙⊥⊞⊡"
FAM = {'⊢':'L','⊣':'L','≻':'L','≺':'L','⋈':'L','⊙':'L','∈':'F','∋':'F',
       '⊤':'D','⊥':'D','⊞':'D','⊡':'X'}
GLY = dict(zip(
  "dead ash array if_ are judge eat mime oil ian ear tot ado or_ nun out yew church "
  "peep age they on egg loll yea air ice bib thigh measure vow gag ooze monad roar err "
  "woe haha wool sure kick fee up so hung ah oak awe zoo".split(),
  "𐑛 𐑨 𐑼 𐑦 𐑸 𐑡 𐑰 𐑥 𐑶 𐑾 𐑽 𐑑 𐑩 𐑹 𐑯 𐑬 𐑿 𐑗 𐑐 𐑱 𐑞 𐑪 𐑧 𐑤 𐑘 𐑺 𐑲 𐑚 𐑔 𐑠 𐑝 𐑜 𐑵 ⊙ 𐑮 𐑻 𐑢 𐑣 𐑫 𐑖 𐑒 𐑓 𐑳 𐑕 𐑙 𐑭 𐑴 𐑷 𐑟".split()))

def features(w):
    w = [c for c in w if c in MARKS]
    n = len(w)
    sig = tuple(sum(1 for c in w if FAM[c]==f) for f in 'LFDX')
    diversity = len(set(w))
    self_ref = n > 0 and w[0] == w[-1]
    has_split, has_fuse = '∈' in w, '∋' in w
    if not has_split and not has_fuse: fo = 0
    elif has_split and not has_fuse:   fo = 1
    elif has_fuse and not has_split:   fo = 2
    else: fo = 1 if w.index('∈') < w.index('∋') else 2
    dc = False
    if '⊤' in w and '⊥' in w and '⊞' in w:
        dc = all(any(w[(i+o) % n] in '⊤⊥' for o in range(1, n))
                 for i, c in enumerate(w) if c == '⊞')
    p = n
    for q in range(1, n+1):
        if n % q == 0 and all(w[i] == w[i % q] for i in range(q, n)):
            p = q; break
    ar = w.count('∈') == 1 and w.count('∋') == 1
    return dict(d=diversity, p=p, fo=fo, sr=self_ref, dc=dc, sx=sig[3],
                nz=sum(1 for x in sig if x > 0), ar=ar, br=ar and self_ref)

def tuple_of(f, R2=True):
    d,p,fo,sr,dc,sx,nz,ar,br = (f[k] for k in "d p fo sr dc sx nz ar br".split())
    dv = 'dead' if (R2 and ar) else 'dead' if d<=2 else 'ash' if d<=5 else 'array' if d<=9 else 'if_'
    tv = 'mime' if (R2 and br) else 'are' if sr else 'judge' if p==1 else 'mime' if p==2 else 'oil' if fo>0 else 'eat'
    rv = {1:'ian',2:'ear',3:'tot'}.get(fo,'ado')
    pv = {1:'or_',2:'nun',3:'out'}.get(fo,'yew' if dc else 'church')
    fv = 'peep' if dc else 'age' if p==1 else 'they'
    kv = 'on' if sx==8 else 'air' if sx>8 else 'egg' if p==1 else 'loll' if p<=4 else 'yea'
    gv = 'ice' if d>=10 else 'thigh' if d>=4 else 'bib'
    cv = 'measure' if fo>0 else 'vow' if p==1 else 'gag' if p==2 else 'ooze'
    hv = {1:'fee',2:'kick',3:'sure'}.get(p,'wool')
    ph = 'monad' if (sr and dc) else 'roar' if sr else 'err' if dc else 'woe' if p==1 else 'haha'
    sv = 'hung' if nz==1 else 'so' if nz==2 else 'up'
    om = 'ah' if fo==1 else 'oak' if fo==2 else ('ah' if sr else 'oak' if p==2 else 'awe')
    return (dv,tv,rv,pv,fv,kv,gv,cv,ph,hv,sv,om)

def show(t): return ''.join(GLY[x] for x in t)

if __name__ == '__main__':
    KERNEL = {                       # live `imasm derive`, pre-R2 binary
      '⊙∈∋⊙':                          '𐑨𐑸𐑾𐑹𐑞𐑤𐑚𐑠𐑮𐑫𐑕𐑭',
      '⊢⊙∈≻⊤≺⊥∋⋈⊞⊡⊣':                  '𐑦𐑶𐑾𐑹𐑐𐑘𐑲𐑠𐑻𐑫𐑳𐑭',
      '⊢⊙∈≻⊤≺⊥∋⋈⊞⊡⊡⊡⊡⊡⊡⊡⊡⊢':           '𐑦𐑸𐑾𐑹𐑐𐑪𐑲𐑠⊙𐑫𐑳𐑭',
    }
    print("validation against the live kernel (R2 off):")
    ok = True
    for w, want in KERNEL.items():
        got = show(tuple_of(features(w), R2=False))
        ok &= got == want
        print(f"  {w:24s} {got}  {'ok' if got==want else 'MISMATCH want '+want}")
    print(f"  faithful: {ok}\n")
    print("with the R2 fields read:")
    for w in KERNEL:
        f = features(w)
        print(f"  {w:24s} {show(tuple_of(f, R2=False))} -> {show(tuple_of(f, R2=True))}"
              f"   (ar={f['ar']} br={f['br']})")
