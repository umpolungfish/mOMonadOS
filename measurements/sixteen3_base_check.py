import itertools

def rust_orders(x):
    bt, bf, st, sf = x
    return bt, bf, st, sf

def le_i_bits(x, y):
    return all((not x[i]) or y[i] for i in range(4))

def rust_le_t(x, y):
    bt_x, bf_x, st_x, sf_x = x
    bt_y, bf_y, st_y, sf_y = y
    return ((not bt_x or bt_y) and (not st_x or st_y) and
            (not bf_y or bf_x) and (not sf_y or sf_x))

def rust_le_c(x, y):
    bt_x, bf_x, st_x, sf_x = x
    bt_y, bf_y, st_y, sf_y = y
    return ((not bt_x or bt_y) and (not bf_x or bf_y) and
            (not st_y or st_x) and (not sf_y or sf_x))

def lean_asserts_true(x):
    n, t, f, b = x
    return t or b

def lean_asserts_false(x):
    n, t, f, b = x
    return f or b

def lean_le_t(x, y):
    return ((not lean_asserts_true(x)) or lean_asserts_true(y)) and \
           ((not lean_asserts_false(y)) or lean_asserts_false(x))

def lean_le_f(x, y):
    return ((not lean_asserts_false(x)) or lean_asserts_false(y)) and \
           ((not lean_asserts_true(y)) or lean_asserts_true(x))

bits16 = list(itertools.product([0,1], repeat=4))

found = []
for perm in itertools.permutations(range(4)):
    def relabel(x, perm=perm):
        # x is a rust tuple (bt,bf,st,sf); perm tells which rust-index goes to which lean-slot
        out = [0,0,0,0]
        for rust_idx, lean_slot in enumerate(perm):
            out[lean_slot] = x[rust_idx]
        return tuple(out)

    ok_tc = True   # rust le_t <-> lean le_t, rust le_c <-> lean le_f
    ok_ct = True   # rust le_t <-> lean le_f, rust le_c <-> lean le_t
    for x in bits16:
        for y in bits16:
            lx, ly = relabel(x), relabel(y)
            rt = rust_le_t(x,y); rc = rust_le_c(x,y)
            lt = lean_le_t(lx,ly); lf = lean_le_f(lx,ly)
            if rt != lt or rc != lf:
                ok_tc = False
            if rt != lf or rc != lt:
                ok_ct = False
        if not ok_tc and not ok_ct:
            break
    if ok_tc or ok_ct:
        found.append((perm, ok_tc, ok_ct))

print("permutations tried:", 24)
print("matches found:", found)

print("--- widening to permutation + independent bit-flips (384 total) ---")
found2 = []
for perm in itertools.permutations(range(4)):
    for flips in itertools.product([0,1], repeat=4):
        def relabel(x, perm=perm, flips=flips):
            out = [0,0,0,0]
            for rust_idx, lean_slot in enumerate(perm):
                v = x[rust_idx] ^ flips[rust_idx]
                out[lean_slot] = v
            return tuple(out)
        ok_tc = True
        ok_ct = True
        for x in bits16:
            for y in bits16:
                lx, ly = relabel(x), relabel(y)
                rt = rust_le_t(x,y); rc = rust_le_c(x,y)
                lt = lean_le_t(lx,ly); lf = lean_le_f(lx,ly)
                if rt != lt or rc != lf:
                    ok_tc = False
                if rt != lf or rc != lt:
                    ok_ct = False
            if not ok_tc and not ok_ct:
                break
        if ok_tc or ok_ct:
            found2.append((perm, flips, ok_tc, ok_ct))
print("matches found:", found2[:5], "... total:", len(found2))
